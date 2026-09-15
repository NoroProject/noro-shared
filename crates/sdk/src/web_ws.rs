//! Talking to the player's web browser tab.
//!
//! A module's Vue 3 micro-frontend runs in the browser. When an action happens
//! on the server (e.g. auction outbid, quest completed, new notification),
//! the module can push a realtime frame to the player's open website tab.
//!
//! ```ignore
//! // Send a frame to one player's browser tab.
//! web_ws::send(user_id, json!({ "event": "quest_done", "reward": 100 }))?;
//!
//! // Broadcast to all players with an open website tab.
//! web_ws::broadcast(json!({ "event": "lot_sold", "winner": player.username }))?;
//!
//! // Receive a frame from the player's web tab.
//! #[event]
//! fn on_web_msg(e: WebMessage) -> Result<()> {
//!     log::info(format!("{} sent {}", e.player.label(), e.payload));
//!     Ok(())
//! }
//! ```

use noro_module_abi::error::ModuleError;
use noro_module_abi::ops::WebFrame;
use noro_module_abi::player::{IntoPlayerRef, PlayerRef};
use serde::Serialize;

/// Sends a frame to one player's browser tab over WebSocket.
///
/// `false` means the player has no website tab open right now — an ordinary state.
///
/// Requires `web_ws = ["notify"]`.
pub fn send(who: impl IntoPlayerRef, payload: impl Serialize) -> Result<bool, ModuleError> {
    crate::host::web_ws_send_call(WebFrame {
        player: Some(who.into_player_ref()),
        payload: to_value(payload)?,
    })
}

/// Sends a frame to every player who currently has a website tab open.
///
/// Requires `web_ws = ["notify"]`.
pub fn broadcast(payload: impl Serialize) -> Result<bool, ModuleError> {
    crate::host::web_ws_send_call(WebFrame {
        player: None,
        payload: to_value(payload)?,
    })
}

/// Whether this player has a website tab open right now.
///
/// Requires `web_ws = ["read"]`.
pub fn is_online(who: impl IntoPlayerRef) -> Result<bool, ModuleError> {
    let who: PlayerRef = who.into_player_ref();
    crate::host::web_ws_online_call(who)
}

/// How many players have a website tab open right now.
///
/// Requires `web_ws = ["read"]`.
pub fn connected() -> Result<i64, ModuleError> {
    crate::host::web_ws_count_call(serde_json::Value::Null)
}

fn to_value(payload: impl Serialize) -> Result<serde_json::Value, ModuleError> {
    serde_json::to_value(payload)
        .map_err(|e| ModuleError::invalid(format!("кадр не сериализуется: {e}")))
}
