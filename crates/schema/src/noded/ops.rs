//! What the master asks a node to do.
//!
//! Every variant names the server it is about: one socket carries the whole
//! node, unlike the wrapper channel where the socket *was* the server.

use crate::build::FileEntry;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::time::Duration;

/// Everything a node needs to build the container. Assembled by the master, so
/// the heap rule and the startup line have one home and show up in the panel.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerSpec {
    pub image: String,
    pub memory_mb: i64,
    #[serde(default)]
    pub swap_mb: Option<i64>,
    pub disk_mb: i64,
    /// Percent of one core; 200 means two cores. 0 is unlimited.
    #[serde(default)]
    pub cpu_percent: i64,
    #[serde(default)]
    pub pids_limit: i64,
    pub jvm_args: Vec<String>,
    /// Server jar relative to the server root, or `@libraries/…/unix_args.txt`
    /// for NeoForge — passed through untouched, exactly as the wrapper does.
    pub jar: String,
    #[serde(default)]
    pub server_args: Vec<String>,
    pub ports: Vec<PortMapping>,
    #[serde(default)]
    pub env: BTreeMap<String, String>,
    /// Yggdrasil root for `-javaagent`. Absent means vanilla auth.
    #[serde(default)]
    pub authlib_url: Option<String>,
    /// Game agent to install into `mods/`/`plugins/`; the node verifies the
    /// ed25519 descriptor itself before writing anything.
    #[serde(default)]
    pub agent: Option<AgentSpec>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct PortMapping {
    pub port: u16,
    #[serde(default)]
    pub protocol: PortProtocol,
    /// The one the players connect to; it also becomes `server-port`.
    #[serde(default)]
    pub primary: bool,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "lowercase")]
pub enum PortProtocol {
    #[default]
    Tcp,
    Udp,
    Both,
}

/// The game agent jar to install alongside the server.
///
/// No signature here, unlike the descriptor the wrapper fetches over HTTP: this
/// one arrives over the node's authenticated socket, and `sha1` — checked while
/// downloading — already pins the contents.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentSpec {
    pub url: String,
    pub sha1: String,
    pub size: u64,
    /// `mods` or `plugins`, decided by the platform.
    pub dir: String,
}

/// Where the server core comes from. Public cores are fetched by the node;
/// only a custom upload is served from the master's store.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreSource {
    pub url: String,
    #[serde(default)]
    pub sha1: Option<String>,
    #[serde(default)]
    pub sha256: Option<String>,
    /// Forge and NeoForge ship an installer rather than a runnable jar. The node
    /// runs it inside a throwaway container of the target image — an installer
    /// is arbitrary code and has no business running on the host.
    #[serde(default)]
    pub installer: bool,
    #[serde(default)]
    pub installer_args: Vec<String>,
}

/// Which directories the build owns. Outside them nothing is ever deleted: a
/// world lost to a sync is not recoverable by apologising.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncPolicy {
    pub managed_roots: Vec<String>,
    /// Never touched even inside a managed root.
    pub keep: Vec<String>,
    pub prune: bool,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum PowerAction {
    Start,
    Stop,
    Restart,
    Kill,
}

/// Everything the install needs, in one box.
///
/// Boxed as a whole rather than inlined: `ServerSpec` alone is several hundred
/// bytes, and an enum is as large as its biggest variant — a `Power` frame
/// would otherwise carry that weight through every queue it passes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstallSpec {
    pub core: CoreSource,
    pub spec: ServerSpec,
    /// Seeded into `server.properties` before the first start.
    #[serde(default)]
    pub properties: BTreeMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "op", content = "args", rename_all = "snake_case")]
pub enum NodeOp {
    // --- lifecycle ---------------------------------------------------------
    ServerCreate {
        server: uuid::Uuid,
        spec: Box<ServerSpec>,
    },
    ServerUpdate {
        server: uuid::Uuid,
        spec: Box<ServerSpec>,
    },
    ServerDelete {
        server: uuid::Uuid,
        #[serde(default)]
        keep_files: bool,
    },
    Install {
        server: uuid::Uuid,
        install: Box<InstallSpec>,
    },

    // --- power and console -------------------------------------------------
    Power {
        server: uuid::Uuid,
        action: PowerAction,
    },
    Command {
        server: uuid::Uuid,
        line: String,
    },
    ConsoleAttach {
        server: uuid::Uuid,
    },
    ConsoleDetach {
        server: uuid::Uuid,
    },

    // --- files -------------------------------------------------------------
    FsList {
        server: uuid::Uuid,
        path: String,
    },
    FsRead {
        server: uuid::Uuid,
        path: String,
        max_bytes: u64,
    },
    FsWrite {
        server: uuid::Uuid,
        path: String,
        content: String,
    },
    FsDelete {
        server: uuid::Uuid,
        paths: Vec<String>,
    },
    FsMkdir {
        server: uuid::Uuid,
        path: String,
    },
    FsRename {
        server: uuid::Uuid,
        from: String,
        to: String,
    },
    /// Fetch a file by URL into the server directory, verifying the hash.
    FsPull {
        server: uuid::Uuid,
        url: String,
        sha1: String,
        dest: String,
    },
    FsArchive {
        server: uuid::Uuid,
        paths: Vec<String>,
        dest: String,
    },
    FsUnarchive {
        server: uuid::Uuid,
        path: String,
        dest_dir: String,
    },
    /// One-shot ticket for a direct upload or download against the node.
    FsTicket {
        server: uuid::Uuid,
        path: String,
        mode: TicketMode,
        ttl_secs: u64,
    },

    // --- build rollout -----------------------------------------------------
    BuildSync {
        server: uuid::Uuid,
        /// Server-side files of the build, reusing the launcher's entry type.
        files: Vec<FileEntry>,
        policy: SyncPolicy,
    },

    // --- backups -----------------------------------------------------------
    BackupCreate {
        server: uuid::Uuid,
        backup: uuid::Uuid,
        name: String,
        #[serde(default)]
        ignore: Vec<String>,
        /// Where to push it after packing. Absent means it stays on the node.
        #[serde(default)]
        upload: Option<Box<BackupUpload>>,
    },
    BackupRestore {
        server: uuid::Uuid,
        backup: uuid::Uuid,
        #[serde(default)]
        download: Option<Box<BackupUpload>>,
    },
    BackupDelete {
        server: uuid::Uuid,
        backup: uuid::Uuid,
    },
    BackupTicket {
        server: uuid::Uuid,
        backup: uuid::Uuid,
        ttl_secs: u64,
    },

    // --- node itself -------------------------------------------------------
    NodeInfo,
    ImagePull {
        image: String,
    },
    SftpReload,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum TicketMode {
    Upload,
    Download,
}

/// Credentials for the node to talk to a backup storage directly. Handed out
/// per operation — a node never holds a standing copy of them.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupUpload {
    pub storage: uuid::Uuid,
    pub kind: String,
    pub config: serde_json::Value,
    pub path: String,
}

impl NodeOp {
    /// How long to wait for the reply. A world archive packs for minutes while
    /// `stop` waits for a save — one shared timeout would cut both in half.
    pub fn timeout(&self) -> Duration {
        match self {
            NodeOp::Install { .. } => Duration::from_secs(1800),
            NodeOp::BuildSync { .. }
            | NodeOp::BackupCreate { .. }
            | NodeOp::BackupRestore { .. }
            | NodeOp::FsUnarchive { .. }
            | NodeOp::FsArchive { .. } => Duration::from_secs(900),
            NodeOp::ImagePull { .. } | NodeOp::ServerCreate { .. } => Duration::from_secs(600),
            NodeOp::Power { .. } | NodeOp::ServerDelete { .. } => Duration::from_secs(180),
            NodeOp::FsPull { .. } => Duration::from_secs(300),
            _ => Duration::from_secs(30),
        }
    }

    /// Which server it concerns, for routing replies and refusing a server that
    /// lives on another node.
    pub fn server(&self) -> Option<uuid::Uuid> {
        match self {
            NodeOp::ServerCreate { server, .. }
            | NodeOp::ServerUpdate { server, .. }
            | NodeOp::ServerDelete { server, .. }
            | NodeOp::Install { server, .. }
            | NodeOp::Power { server, .. }
            | NodeOp::Command { server, .. }
            | NodeOp::ConsoleAttach { server }
            | NodeOp::ConsoleDetach { server }
            | NodeOp::FsList { server, .. }
            | NodeOp::FsRead { server, .. }
            | NodeOp::FsWrite { server, .. }
            | NodeOp::FsDelete { server, .. }
            | NodeOp::FsMkdir { server, .. }
            | NodeOp::FsRename { server, .. }
            | NodeOp::FsPull { server, .. }
            | NodeOp::FsArchive { server, .. }
            | NodeOp::FsUnarchive { server, .. }
            | NodeOp::FsTicket { server, .. }
            | NodeOp::BuildSync { server, .. }
            | NodeOp::BackupCreate { server, .. }
            | NodeOp::BackupRestore { server, .. }
            | NodeOp::BackupDelete { server, .. }
            | NodeOp::BackupTicket { server, .. } => Some(*server),
            NodeOp::NodeInfo | NodeOp::ImagePull { .. } | NodeOp::SftpReload => None,
        }
    }
}
