//! Events of your own, for other modules to handle.
//!
//! The master's events tell you what happened on the platform. These are the
//! other direction: your module announcing something, so that a second module
//! can react without either of them knowing the other exists.
//!
//! ```ignore
//! events::emit("purchase", serde_json::json!({
//!     "player": player.to_string(),
//!     "item": "vip",
//!     "paid": 500,
//! }))?;
//! ```
//!
//! And on the other side, in a different module:
//!
//! ```ignore
//! #[event("mod.shop.purchase")]
//! fn on_purchase(e: serde_json::Value) -> Result<()> {
//!     log::info(format!("{e:?}"));
//!     Ok(())
//! }
//! ```
//!
//! # Names
//!
//! Your event is published as `mod.<your-id>.<name>`; you pass only the last
//! part. The prefix is added by the master rather than trusted from you, for
//! the same reason locale keys carry one: without it a module could publish
//! `player.banned` and every handler of the real event would believe it.
//!
//! # Loops
//!
//! Module A announces something, B reacts and announces something else, A
//! reacts to that. Left alone this spins forever, so every event carries how
//! deeply it is nested and the master stops publishing past a small ceiling.
//! What you get then is an error rather than silence — a module that has
//! accidentally built a loop should find out.
//!
//! The other half is yours: check `ctx.caused_by(…)` before reacting to
//! something your own module set off.

use noro_module_abi::error::ModuleError;
use noro_module_abi::ops::ModuleEvent;
use serde::Serialize;
use uuid::Uuid;

/// Announces something. Handlers run after your call returns, not during it.
///
/// Requires `events = ["emit"]`.
pub fn emit<T: Serialize>(name: &str, payload: T) -> Result<(), ModuleError> {
    send(name, payload, None)
}

/// The same, about one server build.
///
/// Modules scoped to a server only hear events carrying theirs, so this is what
/// makes an announcement reach them.
///
/// Requires `events = ["emit"]`.
pub fn emit_on<T: Serialize>(name: &str, server_id: Uuid, payload: T) -> Result<(), ModuleError> {
    send(name, payload, Some(server_id))
}

fn send<T: Serialize>(name: &str, payload: T, server_id: Option<Uuid>) -> Result<(), ModuleError> {
    let payload = serde_json::to_value(payload)
        .map_err(|e| ModuleError::invalid(format!("payload was not serialized: {e}")))?;
    crate::host::event_emit_call(ModuleEvent {
        name: name.to_string(),
        payload,
        server_id,
    })
}
