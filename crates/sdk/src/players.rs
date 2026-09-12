//! Players.

use noro_module_abi::error::ModuleError;
use noro_module_abi::player::{IntoPlayerRef, Player};

/// Finds a player by any of the ways they can be named.
///
/// ```ignore
/// players::get(uuid)?;                                 // internal id
/// players::get("Dalynkaa")?;                           // Minecraft username
/// players::get(PlayerRef::mc_uuid(u))?;                // game account uuid
/// players::get(PlayerRef::discord("123456789"))?;      // Discord
/// players::get(PlayerRef::identity("twitch", "42"))?;  // any linked login
/// ```
///
/// A username is also looked up among linked logins: the player may have
/// renamed themselves in Minecraft while the operator still calls them by the
/// old name.
///
/// Requires the `players = ["read"]` capability.
pub fn get(who: impl IntoPlayerRef) -> Result<Option<Player>, ModuleError> {
    crate::host::player_get_call(who.into_player_ref())
}

/// The same lookup, except a missing player is an error.
///
/// For cases where the player must exist: they have just joined the game, or
/// they arrived with an event.
pub fn require(who: impl IntoPlayerRef) -> Result<Player, ModuleError> {
    get(who)?.ok_or_else(|| ModuleError::not_found("player not found"))
}
