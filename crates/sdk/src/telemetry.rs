//! How a game server is holding up.
//!
//! The agent reports this with its heartbeat, every thirty seconds. What you
//! get is the latest measurement, so a module can decide things like "do not
//! schedule a restart on a server that is already struggling".
//!
//! ```ignore
//! if let Some(t) = telemetry::of(gs.id)? {
//!     if t.mspt.unwrap_or(0.0) > 45.0 {
//!         chat::announce("The server is lagging, a restart is coming.")?;
//!     }
//! }
//! ```

use noro_module_abi::error::ModuleError;
use noro_module_abi::ops::Telemetry;
use uuid::Uuid;

/// The latest measurement, or `None` when the agent has never reported.
///
/// `mspt` — milliseconds per tick — is the number to watch: it rises while
/// `tps` still looks fine, so it notices trouble first.
///
/// Requires `telemetry = ["read"]`.
pub fn of(game_server_id: Uuid) -> Result<Option<Telemetry>, ModuleError> {
    crate::host::telemetry_of_call(game_server_id)
}
