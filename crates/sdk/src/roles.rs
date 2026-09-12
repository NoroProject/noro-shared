//! Roles.
//!
//! ```ignore
//! if !players::require(id)?.has_role("vip") {
//!     roles::grant(id, "vip")?;
//! }
//! ```

use noro_module_abi::entity::Role;
use noro_module_abi::error::ModuleError;
use noro_module_abi::ops::{IntoRoleRef, RoleDraft, RoleGrant};
use noro_module_abi::player::IntoPlayerRef;

/// Every role on the instance.
///
/// Requires `roles = ["read"]`.
pub fn list() -> Result<Vec<Role>, ModuleError> {
    crate::host::roles_list_call(())
}

/// One role, by machine name or identifier.
///
/// Requires `roles = ["read"]`.
pub fn get(role: impl IntoRoleRef) -> Result<Option<Role>, ModuleError> {
    crate::host::role_get_call(role.into_role_ref())
}

/// The roles a player holds.
///
/// Requires `roles = ["read"]`.
pub fn of(who: impl IntoPlayerRef) -> Result<Vec<Role>, ModuleError> {
    crate::host::roles_of_call(who.into_player_ref())
}

/// Gives a player a role. Doing it twice is not an error.
///
/// The grant is written to the audit log under the same action an operator's
/// grant would use, signed with your module — so it shows up in the same place
/// staff already look, not in a separate feed.
///
/// Requires `roles = ["read", "grant"]`.
pub fn grant(who: impl IntoPlayerRef, role: impl IntoRoleRef) -> Result<(), ModuleError> {
    crate::host::role_grant_call(RoleGrant {
        player: who.into_player_ref(),
        role: role.into_role_ref(),
    })
}

/// Takes a role away. Revoking one the player does not have is not an error.
///
/// Requires `roles = ["read", "grant"]`.
pub fn revoke(who: impl IntoPlayerRef, role: impl IntoRoleRef) -> Result<(), ModuleError> {
    crate::host::role_revoke_call(RoleGrant {
        player: who.into_player_ref(),
        role: role.into_role_ref(),
    })
}

/// Creates a role and returns its identifier.
///
/// Separate from granting on purpose: `grant` hands an existing role to a
/// player, while this decides what roles the instance has at all. An operator
/// can reasonably trust a module with the first and not the second.
///
/// Requires `roles = ["read", "manage"]`.
pub fn create(draft: RoleDraft) -> Result<uuid::Uuid, ModuleError> {
    crate::host::role_save_call(RoleDraft { id: None, ..draft })
}

/// Rewrites a role. Every field is replaced, not merged.
///
/// Requires `roles = ["read", "manage"]`.
pub fn update(id: uuid::Uuid, draft: RoleDraft) -> Result<(), ModuleError> {
    crate::host::role_save_call(RoleDraft {
        id: Some(id),
        ..draft
    })
    .map(|_: uuid::Uuid| ())
}

/// Deletes a role. Players holding it simply stop holding it.
///
/// Requires `roles = ["read", "manage"]`.
pub fn delete(id: uuid::Uuid) -> Result<(), ModuleError> {
    crate::host::role_delete_call(id)
}
