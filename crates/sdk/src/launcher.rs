//! Talking to the launcher.
//!
//! The launcher is open source and people change it. A change almost always
//! needs a half on the master — something counted, something stored — and the
//! master is not open. A module is that half: your fork calls its endpoint,
//! it answers, and where the fork asked nothing, the module sends a frame of
//! its own.
//!
//! ```ignore
//! // A fork asked for something and waits for an answer: an endpoint.
//! #[route(GET, "/my-fork/state")]
//! fn state(req: HttpRequest) -> Result<Value> {
//!     let id = req.require_user()?;
//!     Ok(json!({ "points": store::user(id).get_or::<i64>("points")? }))
//! }
//!
//! // Something happened and the fork did not ask: a frame.
//! #[event]
//! fn on_paid(e: BankTransferred) -> Result<()> {
//!     launcher::send(e.to, json!({ "kind": "paid", "amount": e.amount }))?;
//!     Ok(())
//! }
//!
//! // The fork sent a frame of its own.
//! #[event]
//! fn on_frame(e: LauncherMessage) -> Result<()> {
//!     log::info(format!("{} says {}", e.player.label(), e.payload));
//!     Ok(())
//! }
//! ```
//!
//! # Which direction to use
//!
//! An endpoint has a reply, a status code and a body; use it whenever the fork
//! is asking a question. A frame has none of those and is not acknowledged —
//! it exists for the case the endpoint cannot cover, which is the master
//! speaking first.
//!
//! Frames are addressed to **your** module by name, set by the master rather
//! than by the sender, so one module cannot send in another's name and a
//! neighbour does not receive what was meant for you.
//!
//! # The stock launcher ignores all of this
//!
//! An unknown frame is dropped where it is parsed, without an error and
//! without dropping the connection. That is deliberate: it is what lets you
//! roll a fork out to some machines and not others.

use noro_module_abi::error::ModuleError;
use noro_module_abi::ops::LauncherFrame;
use noro_module_abi::player::{IntoPlayerRef, PlayerRef};
use serde::Serialize;

/// Sends a frame to one player's launcher.
///
/// `false` means their launcher is not connected — an ordinary state, not a
/// failure. There is no queue: a frame nobody was there to hear is gone.
///
/// Requires `launcher = ["notify"]`.
pub fn send(who: impl IntoPlayerRef, payload: impl Serialize) -> Result<bool, ModuleError> {
    crate::host::launcher_send_call(LauncherFrame {
        player: Some(who.into_player_ref()),
        payload: to_value(payload)?,
    })
}

/// Sends a frame to every connected launcher.
///
/// Only to players who are signed in — an anonymous socket has no one to
/// address and receives nothing.
///
/// Requires `launcher = ["notify"]`.
pub fn broadcast(payload: impl Serialize) -> Result<bool, ModuleError> {
    crate::host::launcher_send_call(LauncherFrame {
        player: None,
        payload: to_value(payload)?,
    })
}

/// Whether this player has a launcher open right now.
///
/// Requires `launcher = ["read"]`.
pub fn is_online(who: impl IntoPlayerRef) -> Result<bool, ModuleError> {
    let who: PlayerRef = who.into_player_ref();
    crate::host::launcher_online_call(who)
}

/// How many signed-in launchers are connected.
///
/// Requires `launcher = ["read"]`.
pub fn connected() -> Result<i64, ModuleError> {
    crate::host::launcher_count_call(serde_json::Value::Null)
}

fn to_value(payload: impl Serialize) -> Result<serde_json::Value, ModuleError> {
    serde_json::to_value(payload)
        .map_err(|e| ModuleError::invalid(format!("кадр не сериализуется: {e}")))
}
