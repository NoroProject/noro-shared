//! A player's sessions — the launchers and browsers they are signed in from.
//!
//! ```ignore
//! for s in sessions::of(player)? {
//!     log::info(format!("{} from {:?}", s.id, s.ip));
//! }
//! sessions::revoke_all(player)?;   // after a password change, say
//! ```

use noro_module_abi::error::ModuleError;
use noro_module_abi::ops::{Session, SessionRef};
use noro_module_abi::player::IntoPlayerRef;
use uuid::Uuid;

/// Every session a player has open.
///
/// Requires `sessions = ["read"]`.
pub fn of(who: impl IntoPlayerRef) -> Result<Vec<Session>, ModuleError> {
    crate::host::sessions_of_call(who.into_player_ref())
}

/// Closes one session. `false` — it was already gone.
///
/// Requires `sessions = ["read", "revoke"]`.
pub fn revoke(who: impl IntoPlayerRef, session_id: Uuid) -> Result<bool, ModuleError> {
    crate::host::session_revoke_call(SessionRef {
        player: who.into_player_ref(),
        session_id: Some(session_id),
    })
}

/// Closes all of them and returns how many were closed.
///
/// The player is signed out everywhere — launcher included. Worth saying so in
/// a message, or it reads as a fault.
///
/// Requires `sessions = ["read", "revoke"]`.
pub fn revoke_all(who: impl IntoPlayerRef) -> Result<u64, ModuleError> {
    crate::host::sessions_revoke_all_call(SessionRef {
        player: who.into_player_ref(),
        session_id: None,
    })
}
