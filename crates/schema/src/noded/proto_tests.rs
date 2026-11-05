//! Frame shape is the contract. Two implementations read it — the master and
//! the daemon — and a rename here is a silent outage there.

use super::*;
use serde_json::json;

fn power(server: uuid::Uuid) -> NodeOp {
    NodeOp::Power {
        server,
        action: PowerAction::Restart,
    }
}

/// `flatten` over an adjacently tagged enum is the kind of thing that compiles
/// and then fails at runtime, so it is pinned rather than assumed.
#[test]
fn a_request_flattens_the_operation_beside_its_id() {
    let server = uuid::Uuid::nil();
    let frame = ToNode::Request {
        id: 7,
        op: power(server),
    };

    let value = serde_json::to_value(&frame).expect("serialize");
    assert_eq!(
        value,
        json!({
            "type": "request",
            "id": 7,
            "op": "power",
            "args": { "server": "00000000-0000-0000-0000-000000000000", "action": "restart" }
        })
    );

    match serde_json::from_value::<ToNode>(value).expect("deserialize") {
        ToNode::Request { id, op } => {
            assert_eq!(id, 7);
            assert_eq!(op.server(), Some(server));
        }
        other => panic!("expected a request, got {other:?}"),
    }
}

#[test]
fn an_event_flattens_its_payload() {
    let frame = FromNode::Event {
        event: NodeEvent::Console {
            server: uuid::Uuid::nil(),
            lines: vec!["Done (12.3s)!".into()],
            skipped: 0,
        },
    };

    let value = serde_json::to_value(&frame).expect("serialize");
    assert_eq!(value["type"], "event");
    assert_eq!(value["event"], "console");
    assert_eq!(value["lines"][0], "Done (12.3s)!");

    let back: FromNode = serde_json::from_value(value).expect("deserialize");
    match back {
        FromNode::Event {
            event: NodeEvent::Console { lines, .. },
        } => assert_eq!(lines, vec!["Done (12.3s)!".to_string()]),
        other => panic!("expected a console event, got {other:?}"),
    }
}

/// A node built against an older schema must still parse a hello it did not
/// write every field of — and vice versa.
#[test]
fn hello_survives_missing_optional_fields() {
    let minimal = json!({
        "type": "hello",
        "node_version": "0.1.0",
        "docker_version": "27.0.0",
        "os": "linux",
        "arch": "x86_64",
        "cpus": 8,
        "total_memory_mb": 32768,
        "total_disk_mb": 512000
    });

    match serde_json::from_value::<FromNode>(minimal).expect("deserialize") {
        FromNode::Hello { info } => {
            assert!(info.servers.is_empty());
            assert!(info.public_url.is_none());
            assert!(!info.has_cap(caps::SFTP));
        }
        other => panic!("expected hello, got {other:?}"),
    }
}

#[test]
fn an_error_reply_carries_a_code() {
    let frame = FromNode::Reply {
        id: 3,
        ok: false,
        data: serde_json::Value::Null,
        error: Some(OpError::new("docker_failed", "no such image")),
    };
    let text = serde_json::to_string(&frame).expect("serialize");
    let back: FromNode = serde_json::from_str(&text).expect("deserialize");
    match back {
        FromNode::Reply { ok, error, .. } => {
            assert!(!ok);
            assert_eq!(error.expect("error").code, "docker_failed");
        }
        other => panic!("expected a reply, got {other:?}"),
    }
}

/// Node-wide operations have no server, and routing must not invent one.
#[test]
fn node_wide_operations_name_no_server() {
    assert!(NodeOp::NodeInfo.server().is_none());
    assert!(NodeOp::SftpReload.server().is_none());
    assert_eq!(power(uuid::Uuid::nil()).server(), Some(uuid::Uuid::nil()));
}

/// Packing a world takes minutes; a shared short timeout would cut it in half.
#[test]
fn slow_operations_get_room() {
    let install = NodeOp::Install {
        server: uuid::Uuid::nil(),
        install: Box::new(InstallSpec {
            core: CoreSource {
                url: "https://example.invalid/paper.jar".into(),
                sha1: None,
                sha256: None,
                installer: false,
                installer_args: vec![],
            },
            spec: ServerSpec {
                image: "eclipse-temurin:21-jre".into(),
                memory_mb: 4096,
                swap_mb: None,
                disk_mb: 20480,
                cpu_percent: 200,
                pids_limit: 512,
                jvm_args: vec!["-Xmx3584M".into()],
                jar: "server.jar".into(),
                server_args: vec!["nogui".into()],
                ports: vec![PortMapping {
                    port: 25565,
                    protocol: PortProtocol::Both,
                    primary: true,
                }],
                env: Default::default(),
                authlib_url: None,
                agent: None,
            },
            properties: Default::default(),
            server_pack: None,
        }),
    };

    assert!(install.timeout() > power(uuid::Uuid::nil()).timeout());
    assert!(
        NodeOp::FsMkdir {
            server: uuid::Uuid::nil(),
            path: "plugins".into(),
        }
        .timeout()
            < power(uuid::Uuid::nil()).timeout()
    );
}

/// The two sides of a server pack in the install roll out at different times:
/// a master's install without one and a node's reply without the flag both
/// have to keep reading.
#[test]
fn installs_from_before_server_packs_still_read() {
    let spec: InstallSpec = serde_json::from_value(json!({
        "core": { "url": "https://example.invalid/forge-installer.jar", "installer": true },
        "spec": {
            "image": "eclipse-temurin:17-jre", "memory_mb": 6144, "disk_mb": 20480,
            "cpu_percent": 200, "pids_limit": 512, "jvm_args": [], "jar": "server.jar",
            "server_args": [], "ports": [], "env": {}
        }
    }))
    .unwrap();
    assert!(spec.server_pack.is_none());

    let reply: InstallResult =
        serde_json::from_value(json!({ "jar": "forge-universal.jar", "bytes": 1 })).unwrap();
    assert!(!reply.server_pack);
}

/// A master from before backup quotas sends no `quota_mb`, and the node must
/// read that as "no limit" rather than refuse every backup.
#[test]
fn a_backup_without_a_quota_has_no_limit() {
    let op: NodeOp = serde_json::from_value(json!({
        "op": "backup_create",
        "args": {
            "server": "00000000-0000-0000-0000-000000000000",
            "backup": "00000000-0000-0000-0000-000000000000",
            "name": "nightly"
        }
    }))
    .unwrap();
    match op {
        NodeOp::BackupCreate { quota_mb, .. } => assert_eq!(quota_mb, 0),
        other => panic!("expected a backup, got {other:?}"),
    }
}

/// An old node answers a backup with bare `bytes`/`sha256`: that is a full
/// archive, and nothing about the space the server's backups take.
#[test]
fn an_old_backup_reply_reads_as_a_full_archive() {
    let info: BackupInfo = serde_json::from_value(json!({
        "backup": "00000000-0000-0000-0000-000000000000",
        "bytes": 10,
        "sha256": "ab",
    }))
    .unwrap();
    assert_eq!(info.format, BackupFormat::Tar);
    assert_eq!(info.backups_bytes, None);
    assert_eq!(info.source_bytes, None);

    let chunks: BackupInfo = serde_json::from_value(json!({
        "backup": "00000000-0000-0000-0000-000000000000",
        "bytes": 10,
        "sha256": "ab",
        "format": "chunks",
        "backups_bytes": 30,
    }))
    .unwrap();
    assert_eq!(chunks.format, BackupFormat::Chunks);
    assert_eq!(chunks.backups_bytes, Some(30));
}

/// A whole restore leaves `paths` off the wire: an old node never sees a
/// field it would silently ignore, and a new one reads its absence as "all".
#[test]
fn a_whole_restore_carries_no_paths() {
    let op = NodeOp::BackupRestore {
        server: uuid::Uuid::nil(),
        backup: uuid::Uuid::nil(),
        download: None,
        paths: vec![],
    };
    let value = serde_json::to_value(&op).unwrap();
    assert!(value["args"].get("paths").is_none(), "{value}");

    let old: NodeOp = serde_json::from_value(json!({
        "op": "backup_restore",
        "args": {
            "server": "00000000-0000-0000-0000-000000000000",
            "backup": "00000000-0000-0000-0000-000000000000"
        }
    }))
    .unwrap();
    match old {
        NodeOp::BackupRestore { paths, .. } => assert!(paths.is_empty()),
        other => panic!("expected a restore, got {other:?}"),
    }
}

/// The rebuild waits for a stop that may take a minute; the master giving up
/// first leaves the panel saying "failed" about a rebuild that went through.
#[test]
fn a_rebuild_outlives_the_stop_inside_it() {
    let update = NodeOp::ServerUpdate {
        server: uuid::Uuid::nil(),
        spec: Box::new(ServerSpec {
            image: "eclipse-temurin:21-jre".into(),
            memory_mb: 4096,
            swap_mb: None,
            disk_mb: 10240,
            cpu_percent: 0,
            pids_limit: 0,
            jvm_args: vec![],
            jar: "server.jar".into(),
            server_args: vec![],
            ports: vec![],
            env: Default::default(),
            authlib_url: None,
            agent: None,
        }),
    };
    assert!(update.timeout() >= std::time::Duration::from_secs(120));
}

#[test]
fn a_pressure_event_names_its_server_only_when_it_has_one() {
    let frame = FromNode::Event {
        event: NodeEvent::DiskPressure {
            kind: PressureKind::NodeDisk,
            server: None,
            used_mb: 9300,
            limit_mb: 10000,
            active: true,
        },
    };
    let value = serde_json::to_value(&frame).unwrap();
    assert_eq!(value["event"], "disk_pressure");
    assert_eq!(value["kind"], "node_disk");

    let back: FromNode = serde_json::from_value(json!({
        "type": "event", "event": "disk_pressure", "kind": "server_disk",
        "server": "00000000-0000-0000-0000-000000000000",
        "used_mb": 1, "limit_mb": 2, "active": false
    }))
    .unwrap();
    assert!(matches!(
        back,
        FromNode::Event {
            event: NodeEvent::DiskPressure {
                kind: PressureKind::ServerDisk,
                server: Some(_),
                active: false,
                ..
            }
        }
    ));
}

/// The summary is flattened into the full report, so one id reads the same in
/// the list and in the report beside it.
#[test]
fn a_crash_report_carries_its_summary_inline() {
    let reports = CrashReports {
        crashes: vec![CrashSummary {
            id: "1700000000000".into(),
            at: 1_700_000_000,
            state: PowerState::OomKilled,
            exit_code: Some(137),
        }],
        report: Some(CrashReport {
            summary: CrashSummary {
                id: "1700000000000".into(),
                at: 1_700_000_000,
                state: PowerState::OomKilled,
                exit_code: Some(137),
            },
            console: vec!["java.lang.OutOfMemoryError".into()],
            crash_file: None,
        }),
    };
    let value = serde_json::to_value(&reports).unwrap();
    assert_eq!(value["report"]["id"], "1700000000000");
    assert_eq!(value["report"]["state"], "oom_killed");
    let back: CrashReports = serde_json::from_value(value).unwrap();
    assert_eq!(back.report.unwrap().console.len(), 1);
}

#[test]
fn node_diagnostics_read_without_servers() {
    let diag: NodeDiag = serde_json::from_value(json!({
        "version": "0.9.0", "uptime_secs": 5,
        "master": { "connected": true, "last_contact": 1700000000, "disconnects": 0 },
        "events_queued": 0, "console_lines_dropped": 0,
        "sftp": { "enabled": false }
    }))
    .unwrap();
    assert!(diag.master.connected);
    assert!(diag.servers.is_empty());
    assert!(
        diag.network.is_none(),
        "a daemon older than isolation says nothing"
    );
    assert!(diag.disk_quota.is_none());
}

/// Both ops are new: an older daemon does not parse them, and the master only
/// sends them to a node that announced the capability.
#[test]
fn isolation_and_limit_ops_keep_their_shape() {
    let group = uuid::Uuid::from_u128(1);
    let member = uuid::Uuid::from_u128(2);
    let op = NodeOp::NetworkGroups {
        groups: vec![NetworkGroup {
            id: group,
            servers: vec![group, member],
        }],
    };
    let value = serde_json::to_value(&op).unwrap();
    assert_eq!(value["op"], "network_groups");
    assert_eq!(value["args"]["groups"][0]["servers"][1], member.to_string());
    assert_eq!(op.server(), None, "groups concern the whole node");

    let op = NodeOp::ServerLimits {
        server: member,
        disk_mb: 4096,
    };
    let value = serde_json::to_value(&op).unwrap();
    assert_eq!(value["op"], "server_limits");
    assert_eq!(value["args"]["disk_mb"], 4096);
    assert_eq!(op.server(), Some(member));
}
