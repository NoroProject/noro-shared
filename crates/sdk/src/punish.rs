//! Sanctions.
//!
//! A punishment is not the account ban flag. It has a kind, a reason the player
//! reads, a duration, and it reaches the game the moment it is issued: a ban
//! throws the player out, a mute stops them mid-sentence. The flag —
//! [`crate::players::ban`] — is the blunt version with none of that.
//!
//! ```ignore
//! punish::mute(player, "spam", Some(600))?;
//! for p in punish::active(player)? {
//!     log::info(format!("{} until {:?}", p.kind, p.expires_at));
//! }
//! ```

use noro_module_abi::entity::Punishment;
use noro_module_abi::error::ModuleError;
use noro_module_abi::ops::{PunishKind, PunishRequest};
use noro_module_abi::player::IntoPlayerRef;
use uuid::Uuid;

/// Bans a player. `seconds = None` means until it is lifted.
///
/// Requires `punish = ["issue"]`.
pub fn ban(
    who: impl IntoPlayerRef,
    reason: &str,
    seconds: Option<i64>,
) -> Result<Punishment, ModuleError> {
    issue(who, PunishKind::Ban, reason, seconds, None)
}

/// Stops a player from speaking.
///
/// Requires `punish = ["issue"]`.
pub fn mute(
    who: impl IntoPlayerRef,
    reason: &str,
    seconds: Option<i64>,
) -> Result<Punishment, ModuleError> {
    issue(who, PunishKind::Mute, reason, seconds, None)
}

/// Leaves a warning the player has to acknowledge.
///
/// Requires `punish = ["issue"]`.
pub fn warn(who: impl IntoPlayerRef, reason: &str) -> Result<Punishment, ModuleError> {
    issue(who, PunishKind::Warn, reason, None, None)
}

/// Keeps a player out of one server build.
///
/// Requires `punish = ["issue"]`.
pub fn server_ban(
    who: impl IntoPlayerRef,
    server_id: Uuid,
    reason: &str,
    seconds: Option<i64>,
) -> Result<Punishment, ModuleError> {
    issue(who, PunishKind::ServerBan, reason, seconds, Some(server_id))
}

/// The general form, for when the kind is computed rather than written.
///
/// Requires `punish = ["issue"]`.
pub fn issue(
    who: impl IntoPlayerRef,
    kind: PunishKind,
    reason: &str,
    seconds: Option<i64>,
    server_id: Option<Uuid>,
) -> Result<Punishment, ModuleError> {
    crate::host::punish_issue_call(PunishRequest {
        player: who.into_player_ref(),
        kind,
        reason: reason.to_string(),
        seconds,
        server_id,
    })
}

/// Lifts a sanction. `false` — it was already lifted or never existed.
///
/// Requires `punish = ["revoke"]`.
pub fn revoke(id: Uuid) -> Result<bool, ModuleError> {
    crate::host::punish_revoke_call(id)
}

/// The sanctions in force on a player right now.
///
/// Requires `punish = ["read"]`.
pub fn active(who: impl IntoPlayerRef) -> Result<Vec<Punishment>, ModuleError> {
    crate::host::punish_active_call(who.into_player_ref())
}

/// Everything ever issued to a player, lifted and expired included.
///
/// Requires `punish = ["read"]`.
pub fn history(who: impl IntoPlayerRef) -> Result<Vec<Punishment>, ModuleError> {
    crate::host::punish_list_call(who.into_player_ref())
}
