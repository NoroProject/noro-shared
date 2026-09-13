//! Things done to a player in the running game.
//!
//! A message, an announcement, a kick — one frame each to the agent on the game
//! server, landing immediately. Named after the capability that gates them,
//! `agent`, because that is what they have in common: they need somebody to be
//! in game, and nothing of them survives the player leaving.
//!
//! ```ignore
//! if !agent::tell(player, "Thanks! Your rank is now VIP.")? {
//!     // not in game right now — say it in the cabinet instead
//! }
//! ```
//!
//! The text is finished text, not a key. Your locale catalog is built for the
//! panel and is not installed in the master's own `i18n`, so a key here would
//! reach the player as the key itself — `mod-shop-paid` instead of a sentence.
//!
//! # Why a kick is here and not in `punish`
//!
//! Because a kick is not a punishment, and putting it there would promise
//! things that do not exist. A punishment has a row, a duration, an appeal and
//! a `revoke`; a kick has a disconnect and a line in the audit log. The player
//! can reconnect a second later, and there is nothing to lift.
//!
//! If you want a kick that is on the record and can be argued with, that is
//! [`crate::punish::ban`] with a short duration.

use noro_module_abi::error::ModuleError;
use noro_module_abi::ops::{Announcement, PlayerMessage};
use noro_module_abi::player::IntoPlayerRef;
use uuid::Uuid;

/// Sends a private message to a player.
///
/// `false` means they are not in game, so there was nobody to show it to. That
/// is an ordinary outcome, not an error: telling a player who has just left is
/// exactly what a "player left" handler does.
///
/// Requires `agent = ["tell"]`.
pub fn tell(who: impl IntoPlayerRef, message: &str) -> Result<bool, ModuleError> {
    crate::host::chat_tell_call(PlayerMessage {
        player: who.into_player_ref(),
        message: message.to_string(),
    })
}

/// Announces something to everyone in game.
///
/// Requires `agent = ["announce"]`.
pub fn announce(message: &str) -> Result<(), ModuleError> {
    crate::host::chat_announce_call(Announcement {
        message: message.to_string(),
        server_id: None,
    })
}

/// The same, to one server build only.
///
/// Requires `agent = ["announce"]`.
pub fn announce_on(server_id: Uuid, message: &str) -> Result<(), ModuleError> {
    crate::host::chat_announce_call(Announcement {
        message: message.to_string(),
        server_id: Some(server_id),
    })
}

/// Throws a player out of the game with a reason they will see.
///
/// `false` means they were not in game. A kick is written to the audit log —
/// unlike a message, it is something done *to* a player, and they will ask
/// about it.
///
/// Requires `agent = ["kick"]`.
pub fn kick(who: impl IntoPlayerRef, reason: &str) -> Result<bool, ModuleError> {
    crate::host::chat_kick_call(PlayerMessage {
        player: who.into_player_ref(),
        message: reason.to_string(),
    })
}
