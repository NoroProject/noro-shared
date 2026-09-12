//! Linked logins: Discord, Twitch, Google, anything else a player signs in with.
//!
//! ```ignore
//! #[event]
//! fn on_link(e: IdentityLinked) -> Result<()> {
//!     if e.provider == "discord" {
//!         roles::grant(e.player.id, "verified")?;
//!     }
//!     Ok(())
//! }
//! ```

use noro_module_abi::error::ModuleError;
use noro_module_abi::ops::{Identity, LinkRequest, ProviderQuery};
use noro_module_abi::player::{IntoPlayerRef, Player};

/// Every login linked to a player.
///
/// Requires `identities = ["read"]`.
pub fn of(who: impl IntoPlayerRef) -> Result<Vec<Identity>, ModuleError> {
    crate::host::identities_of_call(who.into_player_ref())
}

/// The player behind an identifier on some platform, if any.
///
/// The same lookup [`crate::players::get`] does with `PlayerRef::identity`;
/// this form reads better when the provider is what you have in hand.
///
/// Requires `identities = ["read"]`.
pub fn find(provider: &str, provider_user_id: &str) -> Result<Option<Player>, ModuleError> {
    crate::players::get(noro_module_abi::player::PlayerRef::identity(
        provider,
        provider_user_id,
    ))
}

/// Links a login to a player. `false` — that identifier is already taken.
///
/// Taken means taken by somebody: the platform identifier is unique across the
/// instance, and moving one between accounts is not something a module does
/// silently.
///
/// Requires `identities = ["read", "link"]`.
pub fn link(
    who: impl IntoPlayerRef,
    provider: &str,
    provider_user_id: &str,
    username: Option<&str>,
) -> Result<bool, ModuleError> {
    crate::host::identity_link_call(LinkRequest {
        player: who.into_player_ref(),
        provider: provider.to_string(),
        provider_user_id: provider_user_id.to_string(),
        username: username.map(str::to_string),
    })
}

/// Unlinks a platform. `false` — it was not linked.
///
/// The platform a player registered through cannot be unlinked at all: their
/// Minecraft UUID is derived from it, and with it their inventory, their
/// progress and their permissions everywhere. That attempt is an error, not a
/// `false`.
///
/// Requires `identities = ["read", "link"]`.
pub fn unlink(who: impl IntoPlayerRef, provider: &str) -> Result<bool, ModuleError> {
    crate::host::identity_unlink_call(ProviderQuery {
        player: who.into_player_ref(),
        provider: provider.to_string(),
    })
}
