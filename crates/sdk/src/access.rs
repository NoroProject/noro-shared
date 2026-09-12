//! Access to servers and builds.
//!
//! Underneath these are ordinary permissions on generated nodes, which is why
//! [`crate::permissions`] could express the same thing. They exist separately
//! because the node has to be spelled exactly right — `noro.server.<uuid>.join`
//! — and a module that builds that string itself will one day build it wrong
//! and silently grant nothing.
//!
//! ```ignore
//! access::allow_join(player, server_id)?;
//! access::allow_build(player, builds::published(server_id)?.id)?;
//! ```

use noro_module_abi::error::ModuleError;
use noro_module_abi::ops::{BuildAccess, ServerAccess};
use noro_module_abi::player::IntoPlayerRef;
use uuid::Uuid;

/// Lets a player into a server build.
///
/// Requires `access = ["grant"]`.
pub fn allow_join(who: impl IntoPlayerRef, server_id: Uuid) -> Result<(), ModuleError> {
    crate::host::access_join_call(ServerAccess {
        player: who.into_player_ref(),
        server_id,
    })
}

/// Takes that away again.
///
/// Requires `access = ["grant"]`.
pub fn revoke_join(who: impl IntoPlayerRef, server_id: Uuid) -> Result<(), ModuleError> {
    crate::host::access_join_revoke_call(ServerAccess {
        player: who.into_player_ref(),
        server_id,
    })
}

/// Lets a player download one client build.
///
/// The server the build belongs to is looked up by the master, so a build id
/// from another server cannot be pointed at the wrong one.
///
/// Requires `access = ["grant"]`.
pub fn allow_build(who: impl IntoPlayerRef, build_id: Uuid) -> Result<(), ModuleError> {
    crate::host::access_build_call(BuildAccess {
        player: who.into_player_ref(),
        build_id,
    })
}

/// Takes build access away.
///
/// Requires `access = ["grant"]`.
pub fn revoke_build(who: impl IntoPlayerRef, build_id: Uuid) -> Result<(), ModuleError> {
    crate::host::access_build_revoke_call(BuildAccess {
        player: who.into_player_ref(),
        build_id,
    })
}
