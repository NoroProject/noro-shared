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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupInfo {
    pub backup: uuid::Uuid,
    pub bytes: u64,
    pub sha256: String,
    /// Set when the node pushed it to a storage as well.
    #[serde(default)]
    pub storage_path: Option<String>,
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
}
