//! Игроки.

use noro_module_abi::error::ModuleError;
use noro_module_abi::player::{IntoPlayerRef, Player};

/// Находит игрока любым из способов, которыми его можно назвать.
///
/// ```ignore
/// players::get(uuid)?;                                 // внутренний id
/// players::get("Dalynkaa")?;                           // ник Minecraft
/// players::get(PlayerRef::mc_uuid(u))?;                // uuid аккаунта игры
/// players::get(PlayerRef::discord("123456789"))?;      // Discord
/// players::get(PlayerRef::identity("twitch", "42"))?;  // любая привязка входа
/// ```
///
/// Ник ищется и среди привязок входа: игрок мог сменить имя в Minecraft, а
/// оператор всё ещё зовёт его прежним.
///
/// Требует возможности `players = ["read"]`.
pub fn get(who: impl IntoPlayerRef) -> Result<Option<Player>, ModuleError> {
    crate::host::player_get_call(who.into_player_ref())
}

/// Тот же поиск, но отсутствие игрока — ошибка.
///
/// Для случаев, где игрок обязан существовать: он только что вошёл в игру или
/// пришёл из события.
pub fn require(who: impl IntoPlayerRef) -> Result<Player, ModuleError> {
    get(who)?.ok_or_else(|| ModuleError::not_found("игрок не найден"))
}
