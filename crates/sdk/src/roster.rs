//! Who is in game right now.
//!
//! This is a snapshot held in the master's memory, not history: it is what the
//! agents last reported. Playtime and sessions live in the database, and
//! `players` is where you ask about those.
//!
//! ```ignore
//! for p in roster::online()? {
//!     chat::tell(p.player.id, "Restart in 10 minutes.")?;
//! }
//! ```

use noro_module_abi::error::ModuleError;
use noro_module_abi::ops::OnlinePlayer;
use noro_module_abi::player::IntoPlayerRef;
use uuid::Uuid;

/// Everyone in game, across every server.
///
/// Requires `roster = ["read"]`.
pub fn online() -> Result<Vec<OnlinePlayer>, ModuleError> {
    crate::host::roster_online_call(())
}

/// Which game server a player is on, if any.
///
/// `None` means they are not in game — which is an ordinary answer, not a
/// failure.
///
/// Requires `roster = ["read"]`.
pub fn locate(who: impl IntoPlayerRef) -> Result<Option<Uuid>, ModuleError> {
    crate::host::roster_where_call(who.into_player_ref())
}

/// Whether a player is in game at all.
///
/// Requires `roster = ["read"]`.
pub fn is_online(who: impl IntoPlayerRef) -> Result<bool, ModuleError> {
    Ok(locate(who)?.is_some())
}
