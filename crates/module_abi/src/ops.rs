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

#[cfg(test)]
#[path = "ops_tests.rs"]
mod tests;
