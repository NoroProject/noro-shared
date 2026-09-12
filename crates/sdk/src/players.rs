//! Players.

use noro_module_abi::error::ModuleError;
use noro_module_abi::ops::BanRequest;
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

/// Bans an account outright.
///
/// This is the account flag, not a punishment: it carries no duration and keeps
/// the player out of the launcher and the panel as well as the game. For timed
/// sanctions with a reason a player can appeal, use `punish`.
///
/// The ban is written to the audit log under the same action an operator's ban
/// would use, signed with your module, so it appears in the list staff already
/// read rather than in a feed of its own.
///
/// Requires `players = ["read", "ban"]`.
pub fn ban(who: impl IntoPlayerRef, reason: Option<&str>) -> Result<(), ModuleError> {
    crate::host::player_ban_call(BanRequest {
        player: who.into_player_ref(),
        banned: true,
        reason: reason.map(str::to_string),
    })
}

/// Lifts an account ban. Unbanning someone who is not banned is not an error.
///
/// Requires `players = ["read", "ban"]`.
pub fn unban(who: impl IntoPlayerRef) -> Result<(), ModuleError> {
    crate::host::player_ban_call(BanRequest {
        player: who.into_player_ref(),
        banned: false,
        reason: None,
    })
}
