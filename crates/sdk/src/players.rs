//! Players.

use noro_module_abi::error::ModuleError;
use noro_module_abi::ops::{
    BanRequest, Cape, CapeRequest, PresetRef, RenameRequest, SavePreset, SkinPreset, SkinRequest,
};
use noro_module_abi::player::{IntoPlayerRef, Player};
use uuid::Uuid;

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

/// Changes a player's Minecraft username.
///
/// Refused if the name is taken: names are unique, and the game addresses
/// players by them.
///
/// Requires `players = ["read", "rename"]`.
pub fn rename(who: impl IntoPlayerRef, username: &str) -> Result<(), ModuleError> {
    crate::host::player_rename_call(RenameRequest {
        player: who.into_player_ref(),
        username: username.to_string(),
    })
}

/// Sets a player's skin. `url = None` clears it back to the default.
///
/// `slim` is the Alex model. It goes with the file rather than being kept
/// separately: a slim texture on classic arms reads as broken, and both halves
/// are one decision.
///
/// Requires `players = ["read", "skin"]`.
pub fn set_skin(who: impl IntoPlayerRef, url: Option<&str>, slim: bool) -> Result<(), ModuleError> {
    crate::host::player_skin_call(SkinRequest {
        player: who.into_player_ref(),
        url: url.map(str::to_string),
        slim,
    })
}

/// Puts a cape from the instance's set on a player. `None` takes it off.
///
/// Requires `players = ["read", "skin"]`.
pub fn set_cape(who: impl IntoPlayerRef, cape_id: Option<Uuid>) -> Result<(), ModuleError> {
    crate::host::player_cape_call(CapeRequest {
        player: who.into_player_ref(),
        cape_id,
    })
}

/// Every cape the instance has.
///
/// Requires `players = ["read"]`.
pub fn capes() -> Result<Vec<Cape>, ModuleError> {
    crate::host::capes_list_call(())
}

/// The skins a player has saved, newest first.
///
/// Requires `players = ["read"]`.
pub fn presets(who: impl IntoPlayerRef) -> Result<Vec<SkinPreset>, ModuleError> {
    crate::host::presets_list_call(who.into_player_ref())
}

/// Saves a skin under a name. `slim = None` takes the geometry they wear now.
///
/// Requires `players = ["read", "skin"]`.
pub fn save_preset(
    who: impl IntoPlayerRef,
    name: &str,
    skin_url: &str,
    slim: Option<bool>,
) -> Result<SkinPreset, ModuleError> {
    crate::host::preset_save_call(SavePreset {
        player: who.into_player_ref(),
        name: name.to_string(),
        skin_url: skin_url.to_string(),
        slim,
    })
}

/// Deletes a saved skin. `false` — it was not there, or not theirs.
///
/// Requires `players = ["read", "skin"]`.
pub fn delete_preset(who: impl IntoPlayerRef, preset_id: Uuid) -> Result<bool, ModuleError> {
    crate::host::preset_delete_call(PresetRef {
        player: who.into_player_ref(),
        preset_id,
    })
}
