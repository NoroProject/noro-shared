//! Restart schedules of game servers.
//!
//! ```ignore
//! restarts::add(ScheduleDraft {
//!     game_server_id: gs.id,
//!     at_times: vec!["04:00".into()],
//!     notice_minutes: 10,
//!     ..Default::default()
//! })?;
//! ```

use noro_module_abi::error::ModuleError;
use noro_module_abi::ops::{RestartSchedule, ScheduleDraft};
use uuid::Uuid;

/// The schedules of one game server.
///
/// Requires `restarts = ["read"]`.
pub fn of(game_server_id: Uuid) -> Result<Vec<RestartSchedule>, ModuleError> {
    crate::host::restarts_of_call(game_server_id)
}

/// Adds a schedule and returns it, with the next run already computed.
///
/// Exactly one of `cron`, `at_times` and `interval_minutes` may say when. The
/// master refuses a draft naming none or several: "every two hours and also at
/// 04:00" has no single answer, and picking one silently would be a guess.
///
/// Requires `restarts = ["read", "manage"]`.
pub fn add(draft: ScheduleDraft) -> Result<RestartSchedule, ModuleError> {
    crate::host::restart_add_call(draft)
}

/// Removes a schedule. `false` — it was not there.
///
/// Requires `restarts = ["read", "manage"]`.
pub fn remove(id: Uuid) -> Result<bool, ModuleError> {
    crate::host::restart_remove_call(id)
}
