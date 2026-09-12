//! Permissions.
//!
//! A permission reaches a player two ways: through a role, or granted to them
//! personally. [`has`] and [`effective`] answer about both together — that is
//! what the game and the panel act on. [`grant`] and [`revoke`] touch only the
//! personal ones; changing what a role carries is a decision about everyone who
//! holds it, and modules do not get to make it.

use noro_module_abi::error::ModuleError;
use noro_module_abi::ops::{PermissionOn, PermissionQuery};
use noro_module_abi::player::IntoPlayerRef;
use uuid::Uuid;

/// Whether a player has a permission, counting their roles.
///
/// Requires `permissions = ["read"]`.
pub fn has(who: impl IntoPlayerRef, node: &str) -> Result<bool, ModuleError> {
    crate::host::perm_has_call(PermissionOn {
        player: who.into_player_ref(),
        node: node.to_string(),
        server_id: None,
    })
}

/// The same question, on one server build.
///
/// Requires `permissions = ["read"]`.
pub fn has_on(who: impl IntoPlayerRef, node: &str, server_id: Uuid) -> Result<bool, ModuleError> {
    crate::host::perm_has_call(PermissionOn {
        player: who.into_player_ref(),
        node: node.to_string(),
        server_id: Some(server_id),
    })
}

/// Every permission a player effectively has, roles included.
///
/// Requires `permissions = ["read"]`.
pub fn effective(who: impl IntoPlayerRef) -> Result<Vec<String>, ModuleError> {
    crate::host::perm_effective_call(PermissionQuery {
        player: who.into_player_ref(),
        server_id: None,
    })
}

/// The same list, as it stands on one server build.
///
/// Requires `permissions = ["read"]`.
pub fn effective_on(who: impl IntoPlayerRef, server_id: Uuid) -> Result<Vec<String>, ModuleError> {
    crate::host::perm_effective_call(PermissionQuery {
        player: who.into_player_ref(),
        server_id: Some(server_id),
    })
}

/// Grants a permission to a player personally, everywhere.
///
/// Requires `permissions = ["read", "grant"]`.
pub fn grant(who: impl IntoPlayerRef, node: &str) -> Result<(), ModuleError> {
    crate::host::perm_grant_call(PermissionOn {
        player: who.into_player_ref(),
        node: node.to_string(),
        server_id: None,
    })
}

/// Grants it on one server build only.
///
/// Requires `permissions = ["read", "grant"]`.
pub fn grant_on(who: impl IntoPlayerRef, node: &str, server_id: Uuid) -> Result<(), ModuleError> {
    crate::host::perm_grant_call(PermissionOn {
        player: who.into_player_ref(),
        node: node.to_string(),
        server_id: Some(server_id),
    })
}

/// Takes a personal permission away.
///
/// Exactly one context: without a server it removes only the global grant.
/// Otherwise revoking "on this build" would silently drop the global one too.
///
/// Requires `permissions = ["read", "grant"]`.
pub fn revoke(who: impl IntoPlayerRef, node: &str) -> Result<(), ModuleError> {
    crate::host::perm_revoke_call(PermissionOn {
        player: who.into_player_ref(),
        node: node.to_string(),
        server_id: None,
    })
}

/// Takes it away on one server build.
///
/// Requires `permissions = ["read", "grant"]`.
pub fn revoke_on(who: impl IntoPlayerRef, node: &str, server_id: Uuid) -> Result<(), ModuleError> {
    crate::host::perm_revoke_call(PermissionOn {
        player: who.into_player_ref(),
        node: node.to_string(),
        server_id: Some(server_id),
    })
}
