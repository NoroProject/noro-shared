//! Velocity proxy operations: moving players, evacuating servers, and titles.
//!
//! Subservers sit behind a Velocity proxy. When a game server is restarting,
//! or a module needs to route a player to an arena or lobby, these calls
//! instruct the proxy to perform the connection request.

use noro_module_abi::entity::GameServer;
use noro_module_abi::error::ModuleError;
use noro_module_abi::ops::{ProxyEvacuate, ProxyTitle, ProxyTransfer};
use noro_module_abi::player::IntoPlayerRef;
use uuid::Uuid;

/// Transfers a player to another subserver by its game server ID.
///
/// Requires `proxy = ["transfer"]`.
pub fn transfer(who: impl IntoPlayerRef, target_server_id: Uuid) -> Result<bool, ModuleError> {
    crate::host::proxy_transfer_call(ProxyTransfer {
        player: who.into_player_ref(),
        target_server_id: Some(target_server_id),
        target_server_name: None,
        reason: None,
    })
}

/// Transfers a player to another subserver by its registered name.
///
/// Requires `proxy = ["transfer"]`.
pub fn transfer_named(
    who: impl IntoPlayerRef,
    target_server_name: &str,
) -> Result<bool, ModuleError> {
    crate::host::proxy_transfer_call(ProxyTransfer {
        player: who.into_player_ref(),
        target_server_id: None,
        target_server_name: Some(target_server_name.to_string()),
        reason: None,
    })
}

/// Evacuates all players from a subserver to another (e.g. Limbo / Lobby).
///
/// Returns the number of players evacuated.
/// Requires `proxy = ["manage"]`.
pub fn evacuate(
    from_server_id: Uuid,
    to_server_id: Option<Uuid>,
    title: Option<&str>,
) -> Result<u32, ModuleError> {
    crate::host::proxy_evacuate_call(ProxyEvacuate {
        from_server_id,
        to_server_id,
        title: title.map(ToString::to_string),
    })
}

/// Displays an in-game Title to a specific player or all players via proxy.
///
/// Requires `proxy = ["title"]`.
pub fn title(
    who: Option<impl IntoPlayerRef>,
    title_text: &str,
    subtitle_text: Option<&str>,
) -> Result<(), ModuleError> {
    crate::host::proxy_title_call(ProxyTitle {
        player: who.map(|p| p.into_player_ref()),
        title: title_text.to_string(),
        subtitle: subtitle_text.map(ToString::to_string),
        fade_in_ms: 500,
        stay_ms: 3000,
        fade_out_ms: 500,
    })
}

/// Returns the subservers registered under this proxy.
///
/// Requires `proxy = ["read"]`.
pub fn subservers(proxy_id: Uuid) -> Result<Vec<GameServer>, ModuleError> {
    crate::host::proxy_subservers_call(proxy_id)
}
