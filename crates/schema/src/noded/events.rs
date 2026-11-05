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

/// What is running out of room.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum PressureKind {
    /// A server against its own disk quota.
    ServerDisk,
    /// The node's data disk against its reserve: what is left before every
    /// server on the machine starts getting refusals.
    NodeDisk,
}

/// Share of a limit at which a warning goes out, in percent.
pub const PRESSURE_WARN_PERCENT: u8 = 90;

/// Share below which a raised warning is withdrawn. The gap is the
/// hysteresis: usage hovering around the line would otherwise warn on every
/// other sample.
pub const PRESSURE_CLEAR_PERCENT: u8 = 85;

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
        /// Сервер был остановлен вручную (Stop / Kill из панели), а не упал сам.
        #[serde(default)]
        stopped_by_user: bool,
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
    /// A limit is close: sent once when usage crosses
    /// [`PRESSURE_WARN_PERCENT`] and once when it falls back under
    /// [`PRESSURE_CLEAR_PERCENT`], never on every sample.
    DiskPressure {
        kind: PressureKind,
        /// The server it is about; absent for [`PressureKind::NodeDisk`].
        #[serde(default)]
        server: Option<uuid::Uuid>,
        used_mb: i64,
        limit_mb: i64,
        /// `false` withdraws the warning.
        active: bool,
    },
    /// A `noro-*` container with no row behind it. Reported, never deleted on
    /// its own: automatic cleanup here is how somebody's world disappears.
    Orphan {
        container: String,
        name: String,
    },
}
