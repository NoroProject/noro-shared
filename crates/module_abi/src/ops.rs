//! The shapes of host calls that take more than one argument.
//!
//! These live in the ABI because both sides have to agree on them: the SDK
//! serializes, the master parses. A struct with named fields rather than a
//! tuple, because a call carrying two `Uuid`s in a row is one worth being
//! unable to get backwards.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::player::PlayerRef;

/// How to name a role in a call.
///
/// Machine name or identifier, the same two ways the panel names one. Struct
/// variants rather than tuples: on an internally tagged enum serde requires the
/// content to be an object, and a newtype variant fails only at runtime.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "by", rename_all = "snake_case")]
pub enum RoleRef {
    /// `vip`, `moderator` — what you write in code.
    Name {
        name: String,
    },
    Id {
        id: Uuid,
    },
}

impl RoleRef {
    pub fn name(name: impl Into<String>) -> Self {
        RoleRef::Name { name: name.into() }
    }

    pub fn id(id: Uuid) -> Self {
        RoleRef::Id { id }
    }
}

/// Everything a role can be named by, so `roles::get("vip")` works.
pub trait IntoRoleRef {
    fn into_role_ref(self) -> RoleRef;
}

impl IntoRoleRef for RoleRef {
    fn into_role_ref(self) -> RoleRef {
        self
    }
}

impl IntoRoleRef for &str {
    fn into_role_ref(self) -> RoleRef {
        RoleRef::name(self)
    }
}

impl IntoRoleRef for String {
    fn into_role_ref(self) -> RoleRef {
        RoleRef::Name { name: self }
    }
}

impl IntoRoleRef for Uuid {
    fn into_role_ref(self) -> RoleRef {
        RoleRef::Id { id: self }
    }
}

/// Granting or revoking a role.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleGrant {
    pub player: PlayerRef,
    pub role: RoleRef,
}

/// A permission on a player, in one context.
///
/// `server_id = None` means everywhere. It is not a filter but part of the
/// identity of the grant: revoking "on this server" must not quietly take away
/// the global one, which is why the master matches it exactly.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionOn {
    pub player: PlayerRef,
    pub node: String,
    #[serde(default)]
    pub server_id: Option<Uuid>,
}

/// Which player, and where to look at their permissions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionQuery {
    pub player: PlayerRef,
    #[serde(default)]
    pub server_id: Option<Uuid>,
}

/// Access to a server build — the right to join it at all.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerAccess {
    pub player: PlayerRef,
    pub server_id: Uuid,
}

/// Access to one client build of a server.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildAccess {
    pub player: PlayerRef,
    pub build_id: Uuid,
}

/// Turning maintenance mode on or off for a game server.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Maintenance {
    pub game_server_id: Uuid,
    pub enabled: bool,
}

/// Banning a player by the account flag.
///
/// This is the flag, not a punishment: it keeps the account out entirely and
/// carries no duration. Timed sanctions are `punish`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BanRequest {
    pub player: PlayerRef,
    pub banned: bool,
    #[serde(default)]
    pub reason: Option<String>,
}

/// A message to one player, in game.
///
/// The text is finished text, not a Fluent key: the module's catalog is built
/// for the web and is not installed in the master's own `i18n`, so resolving a
/// key here would quietly hand the player the key itself.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerMessage {
    pub player: PlayerRef,
    pub message: String,
}

/// A message to everyone, or to everyone on one server build.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Announcement {
    pub message: String,
    /// `None` — the whole instance.
    #[serde(default)]
    pub server_id: Option<Uuid>,
}

/// What kind of sanction to issue.
///
/// An enum rather than a string: the master stores it as text, but a typo in
/// `"mute"` would arrive as a warning and nobody would notice for a week.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PunishKind {
    /// Out of the game and the launcher both.
    Ban,
    /// Out of one server build only.
    ServerBan,
    /// Cannot speak.
    Mute,
    /// A note the player has to acknowledge.
    Warn,
}

impl PunishKind {
    /// How the master stores it.
    pub fn as_str(self) -> &'static str {
        match self {
            PunishKind::Ban => "ban",
            PunishKind::ServerBan => "server_ban",
            PunishKind::Mute => "mute",
            PunishKind::Warn => "warn",
        }
    }
}

/// A sanction to issue.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PunishRequest {
    pub player: PlayerRef,
    pub kind: PunishKind,
    /// What the player will read. Not a key — see [`PlayerMessage`].
    pub reason: String,
    /// How long it lasts. `None` — until it is lifted by hand.
    #[serde(default)]
    pub seconds: Option<i64>,
    /// Which server build it applies to. `None` — all of them.
    #[serde(default)]
    pub server_id: Option<Uuid>,
}

/// Which account, on which server.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountQuery {
    pub server_id: Uuid,
    pub player: PlayerRef,
}

/// Moving money between two accounts.
///
/// Accounts, not players: a player can hold several, and "their money" is not a
/// well-defined place to take it from. Take the account you mean from
/// [`AccountQuery`] or from the treasury.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transfer {
    pub server_id: Uuid,
    pub from: Uuid,
    pub to: Uuid,
    /// In the smallest unit. Must be above zero — a transfer of nothing is a
    /// mistake, and a negative one is a transfer the other way written wrong.
    pub amount: i64,
    /// Shown in the bank's ledger to both sides.
    pub comment: String,
    /// Repeat protection. Send the same key again and the master returns the
    /// transfer already made instead of making a second one — which is what you
    /// want when your handler ran twice.
    #[serde(default)]
    pub idempotency_key: Option<String>,
}

/// A linked login: how a player signs in besides the game itself.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Identity {
    /// `discord`, `twitch`, `google` — the machine name of the platform.
    pub provider: String,
    /// Their identifier on that platform.
    pub provider_user_id: String,
    #[serde(default)]
    pub username: Option<String>,
    /// The platform the player registered through. Their Minecraft UUID is
    /// derived from it, so this one cannot be unlinked.
    #[serde(default)]
    pub primary: bool,
}

/// Linking a login to a player.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkRequest {
    pub player: PlayerRef,
    pub provider: String,
    pub provider_user_id: String,
    /// The name on that platform, for staff to recognise.
    #[serde(default)]
    pub username: Option<String>,
}

/// Which platform to look at, on which player.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderQuery {
    pub player: PlayerRef,
    pub provider: String,
}

/// Renaming a player.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenameRequest {
    pub player: PlayerRef,
    /// The new Minecraft username.
    pub username: String,
}

/// Setting a skin.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkinRequest {
    pub player: PlayerRef,
    /// The file. `None` clears the skin back to the default.
    #[serde(default)]
    pub url: Option<String>,
    /// The slim model — Alex arms. Part of the same decision as the file: a
    /// slim texture on classic arms reads as broken.
    #[serde(default)]
    pub slim: bool,
}

/// Setting a cape.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapeRequest {
    pub player: PlayerRef,
    /// A cape from the instance's set. `None` takes the cape off.
    #[serde(default)]
    pub cape_id: Option<Uuid>,
}

/// A saved skin, under a name the player gave it.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkinPreset {
    pub id: Uuid,
    pub name: String,
    pub skin_url: String,
    pub slim: bool,
}

/// Saving a skin under a name.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavePreset {
    pub player: PlayerRef,
    pub name: String,
    pub skin_url: String,
    /// `None` — take the geometry the player is wearing now.
    #[serde(default)]
    pub slim: Option<bool>,
}

/// Which preset of which player.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PresetRef {
    pub player: PlayerRef,
    pub preset_id: Uuid,
}

/// Publishing or unpublishing a client build.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublishRequest {
    pub build_id: Uuid,
    pub published: bool,
}

/// A cape available on the instance.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cape {
    pub id: Uuid,
    pub name: String,
    pub url: String,
}

#[cfg(test)]
#[path = "ops_tests.rs"]
mod tests;
