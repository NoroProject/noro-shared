//! What a node reports without being asked.

use serde::{Deserialize, Serialize};

/// Power state of one server. Volatile on purpose: it lives in the master's
/// memory, not in Postgres, because a transition per second per server is a
/// write per second per server for nothing.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "snake_case")]
pub enum PowerState {
    #[default]
    Offline,
    Starting,
    Running,
    Stopping,
    /// The process died on its own.
    Crashed,
    /// The kernel killed it against the container's memory limit. Kept apart
    /// from `Crashed` deliberately — "out of memory" and "it crashed" send the
    /// owner down completely different roads.
    OomKilled,
}

impl PowerState {
    pub fn is_up(&self) -> bool {
        matches!(self, PowerState::Starting | PowerState::Running)
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub struct ServerStats {
    /// Percent of one core, so 250.0 is two and a half cores.
    pub cpu_percent: f64,
    pub memory_mb: i64,
    pub memory_limit_mb: i64,
    pub disk_mb: i64,
    pub net_rx_bytes: u64,
    pub net_tx_bytes: u64,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub struct NodeStats {
    pub cpu_percent: f64,
    pub memory_used_mb: i64,
    pub memory_total_mb: i64,
    pub disk_used_mb: i64,
    pub disk_total_mb: i64,
    pub servers_running: u32,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SftpActionKind {
    Write,
    Delete,
    Rename,
    Mkdir,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "event", rename_all = "snake_case")]
pub enum NodeEvent {
    ServerState {
        server: uuid::Uuid,
        state: PowerState,
        /// The server printed `Done (` — from here on it takes players. Same
        /// signal the Java wrapper uses.
        #[serde(default)]
        ready: bool,
        #[serde(default)]
        uptime_secs: u64,
        #[serde(default)]
        exit_code: Option<i32>,
    },
    /// Batched on a short window: a boot-time stack trace is hundreds of lines,
    /// and one frame per line is hundreds of frames.
    Console {
        server: uuid::Uuid,
        lines: Vec<String>,
        /// Dropped because the producer outran the window.
        #[serde(default)]
        skipped: u32,
    },
    /// Progress of an install, on the same channel as the console so the panel
    /// shows it live instead of staring at a spinner for ten minutes.
    InstallLog {
        server: uuid::Uuid,
        lines: Vec<String>,
    },
    Stats {
        server: uuid::Uuid,
        stats: ServerStats,
    },
    NodeStats {
        stats: NodeStats,
    },
    /// Long operation moving along; `id` is the request it belongs to.
    TaskProgress {
        id: u64,
        phase: String,
        #[serde(default)]
        percent: Option<u8>,
        #[serde(default)]
        detail: Option<String>,
    },
    /// Somebody changed something over SFTP. The owner has to see who deleted
    /// `world/`, and they are not staff — this ends up in the server's own
    /// activity log, not in the staff audit trail.
    SftpAction {
        server: uuid::Uuid,
        user_id: uuid::Uuid,
        action: SftpActionKind,
        path: String,
    },
    /// A `noro-*` container with no row behind it. Reported, never deleted on
    /// its own: automatic cleanup here is how somebody's world disappears.
    Orphan {
        container: String,
        name: String,
    },
}
