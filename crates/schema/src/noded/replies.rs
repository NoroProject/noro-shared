//! Payloads a node returns in `Reply.data`.
//!
//! Typed rather than free-form JSON because both sides read them: the master
//! renders these straight into the panel, and an untyped `data` would let a
//! renamed field travel all the way to a blank column in the UI.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirEntry {
    pub name: String,
    /// Relative to the server root, with `/` separators on every platform.
    pub path: String,
    pub dir: bool,
    pub size: u64,
    /// Unix seconds; absent when the filesystem does not say.
    #[serde(default)]
    pub modified: Option<i64>,
    /// Shown in the file manager, but never followed — see the node's path
    /// resolver.
    #[serde(default)]
    pub symlink: bool,
    /// Unix mode bits, for the `+x` a launcher script needs.
    #[serde(default)]
    pub mode: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirListing {
    pub path: String,
    pub entries: Vec<DirEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileContent {
    pub path: String,
    pub content: String,
    pub size: u64,
    /// The file was longer than the caller allowed and `content` is cut. The
    /// panel has to say so rather than let someone save a truncated config
    /// over a working one.
    #[serde(default)]
    pub truncated: bool,
    /// The file is not text. `content` is empty: a jar shown as mojibake is
    /// worse than nothing, because saving it back destroys the file.
    #[serde(default)]
    pub binary: bool,
}

/// What a build rollout actually did.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SyncReport {
    pub downloaded: u32,
    pub unchanged: u32,
    pub deleted: u32,
    /// Files sitting in a managed directory that the build does not know about
    /// and that were left alone. Shown to the owner instead of being removed.
    #[serde(default)]
    pub extra: Vec<String>,
    #[serde(default)]
    pub failed: Vec<String>,
    pub bytes: u64,
}

/// How a backup is kept on the node.
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum BackupFormat {
    /// One self-contained `.tar.gz`. Every node before incremental backups
    /// made only these, so it is what a reply without the field means.
    #[default]
    Tar,
    /// A snapshot in the server's chunk store: deduplicated against every
    /// other snapshot of the same server.
    Chunks,
}

/// Reply to `BackupCreate`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupInfo {
    pub backup: uuid::Uuid,
    /// Bytes this backup added to the node's disk: the archive itself, or the
    /// chunks no earlier snapshot had.
    pub bytes: u64,
    /// sha256 of the archive, or of the snapshot manifest.
    pub sha256: String,
    /// Set when the node pushed it to a storage as well.
    #[serde(default)]
    pub storage_path: Option<String>,
    #[serde(default)]
    pub format: BackupFormat,
    /// Size of the files the backup holds, before compression and dedup.
    /// Absent for archives, where only the packed size is known.
    #[serde(default)]
    pub source_bytes: Option<u64>,
    /// Everything this server's backups take on the node after the operation.
    /// With deduplication the sum of `bytes` says nothing about it.
    #[serde(default)]
    pub backups_bytes: Option<u64>,
}

/// Reply to `BackupDelete`. Older nodes answer `null`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BackupDeleted {
    /// What the server's backups take on the node once this one and the
    /// chunks only it used are gone.
    #[serde(default)]
    pub backups_bytes: Option<u64>,
}

/// One-shot URL for a direct upload or download against the node.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TicketGrant {
    pub ticket: String,
    /// Absolute URL when the node is reachable; `None` means the master has to
    /// proxy the bytes itself.
    #[serde(default)]
    pub url: Option<String>,
    pub expires_at: i64,
}

/// Installed server layout, reported once the install finishes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstallResult {
    /// What actually has to be launched — the installer may have produced a
    /// different name than the core that was downloaded (NeoForge args file).
    pub jar: String,
    pub bytes: u64,
    /// The node laid down the server pack itself. A node that predates packs
    /// in the install says nothing, and the master has to lay it down after.
    #[serde(default)]
    pub server_pack: bool,
}

/// One crash the node remembers, without its contents.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrashSummary {
    /// Opaque to the master; handed back to ask for this crash in full.
    pub id: String,
    /// Unix seconds.
    pub at: i64,
    /// `crashed` or `oom_killed` — the owner goes down a different road for each.
    pub state: super::PowerState,
    #[serde(default)]
    pub exit_code: Option<i32>,
}

/// What a crash left behind, kept on the node outside the server directory so
/// the next run's log rotation cannot erase it.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrashReport {
    #[serde(flatten)]
    pub summary: CrashSummary,
    /// The last lines of the console before the process died.
    pub console: Vec<String>,
    /// The newest `crash-reports/*.txt` written during that run, if any.
    #[serde(default)]
    pub crash_file: Option<CrashFile>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrashFile {
    pub name: String,
    pub content: String,
    /// The file was longer than the node keeps and `content` is its start.
    #[serde(default)]
    pub truncated: bool,
}

/// Reply to `CrashReport`: every kept crash, newest first, and one in full.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CrashReports {
    pub crashes: Vec<CrashSummary>,
    #[serde(default)]
    pub report: Option<CrashReport>,
}

/// The daemon's own state — reply to `NodeInfo`, and what `GET /status` on the
/// node shows its operator. Typed because three readers render it: the admin
/// page, `noro-admin node status` and a person with `curl`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeDiag {
    pub version: String,
    pub uptime_secs: u64,
    pub master: MasterLink,
    /// Events waiting for the master. A queue that keeps growing means the
    /// link is up but the master is not reading.
    pub events_queued: u64,
    /// Console lines thrown away because that queue was full.
    pub console_lines_dropped: u64,
    pub sftp: SftpDiag,
    #[serde(default)]
    pub servers: Vec<ServerDiag>,
    /// Absent from a daemon older than network isolation.
    #[serde(default)]
    pub network: Option<NetworkDiag>,
    #[serde(default)]
    pub disk_quota: Option<DiskQuotaDiag>,
}

/// Whether servers are fenced off from the host, the cloud metadata service
/// and each other.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkDiag {
    /// Turned on in the daemon's config.
    pub enabled: bool,
    /// The firewall rules are in place right now. `false` with `enabled` means
    /// servers run on their own networks, but nothing stops them reaching the
    /// host — `error` says why.
    pub enforced: bool,
    #[serde(default)]
    pub error: Option<String>,
    /// Groups of servers allowed to reach each other.
    #[serde(default)]
    pub groups: u32,
}

/// How a server's disk limit is held.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiskQuotaDiag {
    /// `project` — the filesystem refuses writes past the limit; `walk` — the
    /// daemon measures the directory and refuses only its own writes.
    pub mode: String,
    /// Why project quota is not in use, when it is not.
    #[serde(default)]
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MasterLink {
    pub connected: bool,
    /// Unix seconds of the last frame from the master.
    #[serde(default)]
    pub last_contact: Option<i64>,
    /// How many times the link dropped since the daemon started.
    pub disconnects: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SftpDiag {
    pub enabled: bool,
    #[serde(default)]
    pub fingerprint: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerDiag {
    pub id: uuid::Uuid,
    pub power: super::PowerState,
    pub ready: bool,
    pub stopped_on_purpose: bool,
    /// Automatic restarts in a row after crashes.
    pub restarts: u32,
    /// The daemon holds the server's stdin.
    pub console: bool,
    /// The heavy operation running on it, in the daemon's words.
    #[serde(default)]
    pub busy: Option<String>,
    /// The daemon holds the port of the sleeping server.
    pub sleeping: bool,
}
