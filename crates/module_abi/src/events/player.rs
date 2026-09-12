//! События про игроков и вход.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::context::EventCtx;
use crate::events::impl_event;
use crate::player::Player;

/// Игрок пытается зайти на игровой сервер.
///
/// Отменяемое: `cancel` не пустит игрока и покажет ему причину. Обработчик
/// стоит на пути живого подключения — если модуль затянет, игрок будет ждать.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerPreJoin {
    pub ctx: EventCtx,
    pub player: Player,
    /// Куда заходит.
    pub game_server_id: Uuid,
    #[serde(default, flatten)]
    pub cancel: crate::events::Cancel,
}

impl PlayerPreJoin {
    /// Не пустить игрока. `reason_key` — ключ Fluent, его увидит игрок.
    pub fn cancel(&mut self, reason_key: impl Into<String>) {
        self.cancel.cancel(reason_key);
    }
}

/// Игрок вошёл в игру.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerJoined {
    pub ctx: EventCtx,
    pub player: Player,
    pub game_server_id: Uuid,
    /// Первый ли это вход игрока на этот инстанс за всё время.
    pub first_join: bool,
}

/// Игрок вышел.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerLeft {
    pub ctx: EventCtx,
    pub player: Player,
    pub game_server_id: Uuid,
    /// Сколько длилась сессия. Ноль, если сервер перезапустился и потерял счёт.
    pub session_secs: i64,
}

/// Аккаунт создан.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRegistered {
    pub ctx: EventCtx,
    pub player: Player,
    /// Через что зарегистрировался: `discord`, `twitch`, `local`.
    pub provider: String,
}

/// Игрок входит в кабинет, админку или лаунчер.
///
/// Отменяемое: модуль может закрыть вход, не трогая бан. Так делается
/// техобслуживание «только для персонала» без снятия доступа у остальных.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPreLogin {
    pub ctx: EventCtx,
    pub player: Player,
    pub provider: String,
    #[serde(default, flatten)]
    pub cancel: crate::events::Cancel,
}

impl UserPreLogin {
    pub fn cancel(&mut self, reason_key: impl Into<String>) {
        self.cancel.cancel(reason_key);
    }
}

/// Вход состоялся.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserLoggedIn {
    pub ctx: EventCtx,
    pub player: Player,
    pub provider: String,
}

/// Игроку выдали бан флагом аккаунта (не наказанием).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserBanned {
    pub ctx: EventCtx,
    pub player: Player,
    #[serde(default)]
    pub reason: Option<String>,
}

/// Бан аккаунта снят.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserUnbanned {
    pub ctx: EventCtx,
    pub player: Player,
}

/// Игрок меняет ник.
///
/// Отменяемое и изменяемое: модуль может запретить занятое имя или привести
/// его к своему регистру, переписав `new_name`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPreRename {
    pub ctx: EventCtx,
    pub player: Player,
    #[serde(default)]
    pub old_name: Option<String>,
    pub new_name: String,
    #[serde(default, flatten)]
    pub cancel: crate::events::Cancel,
}

impl UserPreRename {
    pub fn cancel(&mut self, reason_key: impl Into<String>) {
        self.cancel.cancel(reason_key);
    }
}

/// Ник сменился.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRenamed {
    pub ctx: EventCtx,
    pub player: Player,
    #[serde(default)]
    pub old_name: Option<String>,
    pub new_name: String,
}

/// Скин или плащ сменились.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSkinChanged {
    pub ctx: EventCtx,
    pub player: Player,
    /// `skin` или `cape`.
    pub what: String,
}

/// Привязан способ входа.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityLinked {
    pub ctx: EventCtx,
    pub player: Player,
    pub provider: String,
    pub external_id: String,
}

/// Способ входа отвязан.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityUnlinked {
    pub ctx: EventCtx,
    pub player: Player,
    pub provider: String,
}

impl_event!(PlayerPreJoin, super::EV_PLAYER_PRE_JOIN, Pre);
impl_event!(PlayerJoined, super::EV_PLAYER_JOINED, Post);
impl_event!(PlayerLeft, super::EV_PLAYER_LEFT, Post);
impl_event!(UserRegistered, super::EV_USER_REGISTERED, Post);
impl_event!(UserPreLogin, super::EV_USER_PRE_LOGIN, Pre);
impl_event!(UserLoggedIn, super::EV_USER_LOGGED_IN, Post);
impl_event!(UserBanned, super::EV_USER_BANNED, Post);
impl_event!(UserUnbanned, super::EV_USER_UNBANNED, Post);
impl_event!(UserPreRename, super::EV_USER_PRE_RENAME, Pre);
impl_event!(UserRenamed, super::EV_USER_RENAMED, Post);
impl_event!(UserSkinChanged, super::EV_USER_SKIN_CHANGED, Post);
impl_event!(IdentityLinked, super::EV_IDENTITY_LINKED, Post);
impl_event!(IdentityUnlinked, super::EV_IDENTITY_UNLINKED, Post);
