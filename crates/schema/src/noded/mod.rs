//! Wire protocol between the master and a node daemon (`noded`).
//!
//! Frames are JSON tagged by `type`. The master sends requests, the node sends
//! replies and events; a request carries `id` and the reply quotes it, or
//! parallel operations would swap answers.
//!
//! Two things differ from the older wrapper channel on purpose. One socket
//! carries the **whole node**, so every operation names its server — a node
//! with forty servers cannot hold forty sockets. And the node never issues a
//! request over this socket: when it needs something from the master (SFTP
//! auth, storage credentials) it makes a plain HTTPS call with its own token,
//! which keeps the state machine one-directional.
//!
//! Not re-exported at the crate root: `FileEntry`, `Op` and `Status` would
//! collide with `schema::build` and `schema::launcher`. Callers write
//! `schema::noded::NodeOp`.

pub mod events;
pub mod ops;
pub mod replies;

pub use events::{NodeEvent, NodeStats, PowerState, ServerStats, SftpActionKind};
pub use ops::{
    AgentSpec, BackupUpload, CoreSource, InstallSpec, NodeOp, PortMapping, PortProtocol,
    PowerAction, ServerSpec, SyncPolicy, TicketMode,
};
pub use replies::{
    BackupInfo, DirEntry, DirListing, FileContent, InstallResult, SyncReport, TicketGrant,
};

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::time::Duration;

/// Node secrets carry this prefix, the way agent secrets carry `noroagent_`.
/// It makes a leaked string identifiable at a glance in a log or a paste.
pub const TOKEN_PREFIX: &str = "noronode_";

/// How often the master pings. TCP staying open answers "is the socket alive",
/// not "is this node healthy", and a half-open socket used to swallow commands
/// silently.
pub const PING_EVERY: Duration = Duration::from_secs(15);

/// No pong for this long and the node counts as offline — the panel greys out
/// every server on it rather than pretending they are fine.
pub const OFFLINE_AFTER: Duration = Duration::from_secs(45);

/// Optional abilities. The panel hides what a node cannot honour instead of
/// failing at the click.
pub mod caps {
    pub const SFTP: &str = "sftp";
    /// Reachable from a browser, so uploads can skip the master.
    pub const DIRECT_TRANSFER: &str = "direct-transfer";
    pub const BACKUP_S3: &str = "backup-s3";
    pub const BACKUP_WEBDAV: &str = "backup-webdav";
    pub const IMAGE_PULL: &str = "image-pull";
    /// Real per-server disk quotas, which need overlay2 on xfs with pquota.
    pub const DISK_QUOTA: &str = "disk-quota";
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ToNode {
    Request {
        id: u64,
        #[serde(flatten)]
        op: NodeOp,
    },
    /// Stop a long operation. Syncing four hundred mods or packing forty
    /// gigabytes has to be interruptible.
    Cancel {
        id: u64,
    },
    Ping {
        seq: u64,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum FromNode {
    /// Must be the first frame; anything else drops the session.
    Hello {
        #[serde(flatten)]
        info: NodeHello,
    },
    Pong {
        seq: u64,
    },
    Reply {
        id: u64,
        ok: bool,
        #[serde(default)]
        data: Value,
        #[serde(default)]
        error: Option<OpError>,
    },
    Event {
        #[serde(flatten)]
        event: NodeEvent,
    },
}

/// Structured so the master can map a node's refusal onto its own error code
/// instead of forwarding an opaque sentence.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpError {
    pub code: String,
    pub message: String,
}

impl OpError {
    pub fn new(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            message: message.into(),
        }
    }
}

/// Who connected, and what it already has running.
///
/// The snapshot matters as much as the identity: a container removed by hand
/// and a server created while the node was offline both have to be noticed, and
/// this is the only moment both sides can compare notes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeHello {
    pub node_version: String,
    pub docker_version: String,
    pub os: String,
    pub arch: String,
    pub cpus: u32,
    pub total_memory_mb: i64,
    pub total_disk_mb: i64,
    #[serde(default)]
    pub capabilities: Vec<String>,
    /// Base URL a browser can reach, when the node has one. Empty means every
    /// byte goes through the master.
    #[serde(default)]
    pub public_url: Option<String>,
    #[serde(default)]
    pub sftp_port: Option<u16>,
    /// `SHA256:…` of the SFTP host key, shown in the panel so the user verifies
    /// it instead of typing `yes` blind.
    #[serde(default)]
    pub sftp_fingerprint: Option<String>,
    #[serde(default)]
    pub servers: Vec<ServerSnapshot>,
}

impl NodeHello {
    pub fn has_cap(&self, cap: &str) -> bool {
        self.capabilities.iter().any(|c| c == cap)
    }
}

#[cfg(test)]
#[path = "proto_tests.rs"]
mod tests;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerSnapshot {
    pub server: uuid::Uuid,
    pub state: PowerState,
    #[serde(default)]
    pub ready: bool,
    #[serde(default)]
    pub uptime_secs: u64,
    /// The container exists on disk even if it is stopped.
    #[serde(default)]
    pub container: bool,
}
