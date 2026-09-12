//! Calling out to the internet.
//!
//! ```ignore
//! let reply = http::send(HttpCall::post("https://discord.com/api/webhooks/…", body)
//!     .header("content-type", "application/json"))?;
//! if !reply.ok() {
//!     log::warn(format!("webhook answered {}", reply.status));
//! }
//! ```
//!
//! # What is checked before the request leaves
//!
//! The host must be in your manifest's `http` allow-list, and the scheme must
//! be `https`. The master then resolves the name and refuses addresses inside
//! the machine or the private network — otherwise an allow-listed name pointed
//! at `127.0.0.1` would turn your module into a way to reach the master's own
//! services. The connection then goes to the address that was checked, not to
//! whatever the name resolves to a moment later.
//!
//! # What it costs
//!
//! The call blocks your handler until the answer arrives or the deadline
//! passes. A `Post` event handler has five seconds in total, so a slow endpoint
//! is a handler that times out. A big answer is refused rather than loaded:
//! it would come into the sandbox's memory.

use noro_module_abi::error::ModuleError;
use noro_module_abi::http_out::{HttpCall, HttpReply};

/// Sends a request and waits for the answer.
///
/// A non-2xx status comes back as a normal [`HttpReply`] — check
/// [`HttpReply::ok`]. Only a refusal to send at all, a timeout, or a broken
/// connection is an error.
///
/// Requires the host in `http = [...]`.
pub fn send(call: HttpCall) -> Result<HttpReply, ModuleError> {
    crate::host::http_send_call(call)
}

/// A `GET` whose answer is parsed as JSON.
///
/// Requires the host in `http = [...]`.
pub fn get_json<T: for<'de> serde::Deserialize<'de>>(url: &str) -> Result<T, ModuleError> {
    let reply = send(HttpCall::get(url))?;
    if !reply.ok() {
        return Err(ModuleError::conflict(format!(
            "{url} answered {}",
            reply.status
        )));
    }
    reply.json()
}
