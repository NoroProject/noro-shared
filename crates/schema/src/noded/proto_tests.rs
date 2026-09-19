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
