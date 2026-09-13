//! A face of your own to write from.
//!
//! Without one, a module that wants to message a player has to name a living
//! person as the sender — and the recipient sees someone who wrote nothing and
//! answers them. A bot has its own name, stands in the conversation list beside
//! their friends, and can be answered: replies arrive as `dm.sent` with your bot
//! as the recipient.
//!
//! ```ignore
//! let shop = bots::ensure("Shop")?;
//! shop.dm().send_to(&buyer, "Your order is ready.")?;
//! ```
//!
//! A bot is an ordinary [`Player`] as far as everything else is concerned, so
//! every method you already know works on it. What it cannot do is sign in or
//! join a game — the master will not let it, and neither will its database.
//!
//! Owners are of two kinds, module and player; today only modules can create
//! one. When players can, nothing here changes for you.

use noro_module_abi::error::ModuleError;
use noro_module_abi::player::Player;

/// Your bot by that name, created on first use.
///
/// Calling it again with the same name hands back the same bot — otherwise a
/// restart of the master would leave the player with a pile of identical
/// correspondents.
///
/// Refused when the name is taken by a person or by somebody else's bot. Names
/// are unique across every account for a reason: in a list of conversations a
/// second "Shop" means the reader does not know who they are answering.
///
/// Requires `bots = ["create"]`.
pub fn ensure(name: &str) -> Result<Player, ModuleError> {
    crate::host::bot_ensure_call(serde_json::json!({ "name": name }))
}

/// Every bot your module has.
///
/// Requires `bots = ["create"]`.
pub fn list() -> Result<Vec<Player>, ModuleError> {
    crate::host::bot_list_call(serde_json::json!({}))
}
