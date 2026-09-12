//! Moderation cases.
//!
//! A case is what a report turns into: a target, a timeline, and eventually a
//! verdict. A module can read them, assign them and close them — which is what
//! automation around moderation is usually for.
//!
//! ```ignore
//! let case = cases::get(id)?.expect("case");
//! if case.claimed_by.is_none() {
//!     cases::claim(id, on_duty_moderator)?;
//! }
//! ```

use noro_module_abi::error::ModuleError;
use noro_module_abi::ops::{Case, CaseEvent, CaseVerdict};
use noro_module_abi::player::IntoPlayerRef;
use uuid::Uuid;

/// One case.
///
/// Requires `cases = ["read"]`.
pub fn get(id: Uuid) -> Result<Option<Case>, ModuleError> {
    crate::host::case_get_call(id)
}

/// Its timeline: claims, punishments, notes, chat that was kept.
///
/// Requires `cases = ["read"]`.
pub fn events(id: Uuid) -> Result<Vec<CaseEvent>, ModuleError> {
    crate::host::case_events_call(id)
}

/// The open case on a player, if there is one.
///
/// Requires `cases = ["read"]`.
pub fn open_on(who: impl IntoPlayerRef) -> Result<Option<Case>, ModuleError> {
    crate::host::case_open_on_call(who.into_player_ref())
}

/// Assigns a case to a moderator. `false` — somebody already has it.
///
/// A moderator has to be named: a case belongs to a person who will answer for
/// it, and a module holding one itself would be a case nobody is working on.
///
/// Requires `cases = ["read", "claim"]`.
pub fn claim(id: Uuid, moderator: impl IntoPlayerRef) -> Result<bool, ModuleError> {
    crate::host::case_claim_call(serde_json::json!({
        "case_id": id,
        "player": moderator.into_player_ref(),
    }))
}

/// Closes a case. `verdict = "confirmed"` means the report held up.
///
/// The reports behind it are closed with the same outcome: a resolved case with
/// its reports still open is the state that makes staff do the work twice.
///
/// Requires `cases = ["read", "resolve"]`.
pub fn resolve(
    id: Uuid,
    verdict: &str,
    resolution: &str,
    rule_code: Option<&str>,
) -> Result<bool, ModuleError> {
    crate::host::case_resolve_call(CaseVerdict {
        case_id: id,
        verdict: verdict.to_string(),
        resolution: resolution.to_string(),
        rule_code: rule_code.map(str::to_string),
    })
}
