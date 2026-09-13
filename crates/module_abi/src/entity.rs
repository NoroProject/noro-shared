//! The master's entities, projected for a module.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A server build. Called a "server" in the interface; in the data it is `servers`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Server {
    pub id: Uuid,
    pub name: String,
    /// The hub slug. Empty when no hub has been set up.
    #[serde(default)]
    pub slug: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
}

/// A game server: the process the agent connects to.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameServer {
    pub id: Uuid,
    pub server_id: Uuid,
    pub name: String,
    /// `proxy` or `server`.
    pub kind: String,
    pub online: bool,
    #[serde(default)]
    pub players_online: i32,
    #[serde(default)]
    pub max_players: Option<i32>,
    #[serde(default)]
    pub version: Option<String>,
}

/// A client build: what the launcher downloads for a player.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Build {
    pub id: Uuid,
    pub server_id: Uuid,
    pub name: String,
    pub published: bool,
    pub created_at: DateTime<Utc>,
    #[serde(default)]
    pub minecraft_version: Option<String>,
}

/// A file inside a build, exactly as the launcher will download it.
///
/// `sha1` is the store's address, not a checksum you compute: two builds
/// carrying the same mod carry the same `sha1` and the file is stored once.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildFile {
    pub id: Uuid,
    pub build_id: Uuid,
    /// Where it lands in the game directory: `mods/sodium.jar`, `config/x.toml`.
    pub path: String,
    pub sha1: String,
    pub size: i64,
    /// `client`, `server` or `both`.
    pub side: String,
    /// What it is: `mod`, `config`, `resource`, and the rest the panel shows.
    pub kind: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Role {
    pub id: Uuid,
    /// The machine name: `vip`, `moderator`.
    pub name: String,
    pub display_name: String,
    #[serde(default)]
    pub color: Option<String>,
    pub is_default: bool,
    /// The server the role belongs to. `None` means the role applies everywhere.
    #[serde(default)]
    pub server_id: Option<Uuid>,
}

/// An account in the hub's bank.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub id: Uuid,
    pub server_id: Uuid,
    /// The owner. `None` on system accounts: the treasury, the tax account, the fines account.
    #[serde(default)]
    pub owner_id: Option<Uuid>,
    /// The system account's machine code, when it is a system account.
    #[serde(default)]
    pub code: Option<String>,
    pub balance: i64,
    #[serde(default)]
    pub frozen: bool,
}

/// A punishment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Punishment {
    pub id: Uuid,
    pub user_id: Uuid,
    /// `ban`, `server_ban`, `mute`, `warn` — the same set as [`crate::ops::PunishKind`].
    ///
    /// A kick is not among them and never was: it records nothing and there is
    /// nothing to lift. It lives in the `agent` domain of the SDK.
    pub kind: String,
    pub reason: String,
    pub issued_at: DateTime<Utc>,
    /// When it expires. `None` means never.
    #[serde(default)]
    pub expires_at: Option<DateTime<Utc>>,
    #[serde(default)]
    pub revoked: bool,
    /// The server it applies to. `None` means all of them.
    #[serde(default)]
    pub server_id: Option<Uuid>,
}

impl Punishment {
    /// Whether the punishment is in force at the given moment.
    ///
    /// The time is passed in rather than read from the system clock: wasm has
    /// none, and `Utc::now()` inside a module would return the start of the
    /// epoch. The current time comes from `noro_sdk::now()`, which gets it from
    /// the master.
    pub fn active_at(&self, now: DateTime<Utc>) -> bool {
        !self.revoked && self.expires_at.is_none_or(|e| e > now)
    }
}
