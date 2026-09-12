//! Реестр событий и механика отмены.
//!
//! Приём тот же, что у `nodes!` в правах: события заводятся одним макросом,
//! который порождает и константу, и запись в каталоге. Параллельного списка,
//! который разъезжается с кодом, не существует — админка показывает подписки
//! модуля, беря названия отсюда.

use serde::{Deserialize, Serialize};

pub mod access;
pub mod economy;
pub mod infra;
pub mod moderation;
pub mod player;

pub use access::*;
pub use economy::*;
pub use infra::*;
pub use moderation::*;
pub use player::*;

/// Когда модуль видит событие.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EventKind {
    /// До действия. Модуль может отменить его и поправить поля.
    ///
    /// Мастер ждёт ответа, поэтому обработчик стоит на пути живого запроса:
    /// затянешь — затормозится вход игрока или его перевод.
    Pre,
    /// После того, как всё случилось и записалось. Отменить нечего.
    Post,
}

/// Запись каталога.
pub struct EventMeta {
    pub name: &'static str,
    pub kind: EventKind,
    /// Группа для админки: `player`, `access`, `infra`, `moderation`, `economy`.
    pub group: &'static str,
    /// Ключ Fluent с человеческим названием.
    pub title: &'static str,
}

impl EventMeta {
    pub fn cancellable(&self) -> bool {
        matches!(self.kind, EventKind::Pre)
    }
}

/// Связь структуры события с его именем в каталоге.
///
/// Нужен, чтобы `#[event]` не требовал повторять имя строкой: тип обработчика
/// уже всё говорит, а строка в двух местах однажды разойдётся.
pub trait Event: Serialize + for<'de> Deserialize<'de> {
    const NAME: &'static str;
    const KIND: EventKind;

    /// Обстоятельства события.
    ///
    /// В трейте, а не только в поле: шина отбирает подписчиков по сборке ещё
    /// до того, как узнает конкретный тип события.
    fn ctx(&self) -> &crate::context::EventCtx;
}

/// Решение модуля по отменяемому событию.
///
/// Причина — ключ Fluent, а не готовый текст: сообщение увидит игрок, а язык
/// известен только на стороне мастера.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Cancel {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    reason: Option<String>,
}

impl Cancel {
    /// Запретить действие. Повторный вызов перетирает причину: побеждает
    /// последний обработчик, а порядок задают приоритеты подписок.
    pub fn cancel(&mut self, reason_key: impl Into<String>) {
        self.reason = Some(reason_key.into());
    }

    pub fn cancelled(&self) -> bool {
        self.reason.is_some()
    }

    pub fn reason(&self) -> Option<&str> {
        self.reason.as_deref()
    }
}

/// Привязывает структуру события к записи каталога.
///
/// Только `impl`, сами структуры пишутся руками: их читает автор модуля в
/// документации, а сгенерированные макросом поля там не видны.
macro_rules! impl_event {
    ($t:ty, $konst:path, $kind:ident) => {
        impl $crate::events::Event for $t {
            const NAME: &'static str = $konst;
            const KIND: $crate::events::EventKind = $crate::events::EventKind::$kind;

            fn ctx(&self) -> &$crate::context::EventCtx {
                &self.ctx
            }
        }
    };
}

pub(crate) use impl_event;

macro_rules! events {
    ($($konst:ident = $name:literal, $kind:ident, $group:literal, $title:literal;)*) => {
        $(pub const $konst: &str = $name;)*

        /// Все события — для подсказок в админке и проверки манифеста.
        pub const ALL_EVENTS: &[&EventMeta] = &[$(&EventMeta {
            name: $name,
            kind: EventKind::$kind,
            group: $group,
            title: $title,
        }),*];
    };
}

events! {
    // --- Игроки и вход --------------------------------------------------------
    EV_PLAYER_PRE_JOIN       = "player.pre_join",        Pre,  "player", "ev-player-pre-join";
    EV_PLAYER_JOINED         = "player.joined",          Post, "player", "ev-player-joined";
    EV_PLAYER_LEFT           = "player.left",            Post, "player", "ev-player-left";
    EV_USER_REGISTERED       = "user.registered",        Post, "player", "ev-user-registered";
    EV_USER_PRE_LOGIN        = "user.pre_login",         Pre,  "player", "ev-user-pre-login";
    EV_USER_LOGGED_IN        = "user.logged_in",         Post, "player", "ev-user-logged-in";
    EV_USER_BANNED           = "user.banned",            Post, "player", "ev-user-banned";
    EV_USER_UNBANNED         = "user.unbanned",          Post, "player", "ev-user-unbanned";
    EV_USER_PRE_RENAME       = "user.pre_rename",        Pre,  "player", "ev-user-pre-rename";
    EV_USER_RENAMED          = "user.renamed",           Post, "player", "ev-user-renamed";
    EV_USER_SKIN_CHANGED     = "user.skin_changed",      Post, "player", "ev-user-skin-changed";
    EV_IDENTITY_LINKED       = "user.identity_linked",   Post, "player", "ev-identity-linked";
    EV_IDENTITY_UNLINKED     = "user.identity_unlinked", Post, "player", "ev-identity-unlinked";

    // --- Роли, права, доступы -------------------------------------------------
    EV_ROLE_CREATED          = "role.created",           Post, "access", "ev-role-created";
    EV_ROLE_UPDATED          = "role.updated",           Post, "access", "ev-role-updated";
    EV_ROLE_DELETED          = "role.deleted",           Post, "access", "ev-role-deleted";
    EV_ROLE_PRE_GRANT        = "user.pre_role_granted",  Pre,  "access", "ev-role-pre-grant";
    EV_ROLE_GRANTED          = "user.role_granted",      Post, "access", "ev-role-granted";
    EV_ROLE_REVOKED          = "user.role_revoked",      Post, "access", "ev-role-revoked";
    EV_PERM_GRANTED          = "user.permission_granted", Post, "access", "ev-perm-granted";
    EV_PERM_REVOKED          = "user.permission_revoked", Post, "access", "ev-perm-revoked";

    // --- Сборки, билды, игровые серверы ---------------------------------------
    EV_SERVER_CREATED        = "server.created",         Post, "infra", "ev-server-created";
    EV_SERVER_UPDATED        = "server.updated",         Post, "infra", "ev-server-updated";
    EV_SERVER_DELETED        = "server.deleted",         Post, "infra", "ev-server-deleted";
    EV_BUILD_CREATED         = "build.created",          Post, "infra", "ev-build-created";
    EV_BUILD_PRE_PUBLISH     = "build.pre_publish",      Pre,  "infra", "ev-build-pre-publish";
    EV_BUILD_PUBLISHED       = "build.published",        Post, "infra", "ev-build-published";
    EV_BUILD_DELETED         = "build.deleted",          Post, "infra", "ev-build-deleted";
    EV_GAMESERVER_ONLINE     = "gameserver.online",      Post, "infra", "ev-gameserver-online";
    EV_GAMESERVER_OFFLINE    = "gameserver.offline",     Post, "infra", "ev-gameserver-offline";
    EV_GAMESERVER_MAINTENANCE = "gameserver.maintenance", Post, "infra", "ev-gameserver-maintenance";
    EV_INSTANCE_SETTING      = "instance.setting_changed", Post, "infra", "ev-instance-setting";
    EV_MODULE_ENABLED        = "module.enabled",         Post, "infra", "ev-module-enabled";
    EV_MODULE_DISABLED       = "module.disabled",        Post, "infra", "ev-module-disabled";

    // --- Модерация ------------------------------------------------------------
    EV_PUNISH_PRE_ISSUE      = "punishment.pre_issue",   Pre,  "moderation", "ev-punish-pre-issue";
    EV_PUNISH_ISSUED         = "punishment.issued",      Post, "moderation", "ev-punish-issued";
    EV_PUNISH_REVOKED        = "punishment.revoked",     Post, "moderation", "ev-punish-revoked";
    EV_PUNISH_EXPIRED        = "punishment.expired",     Post, "moderation", "ev-punish-expired";
    EV_REPORT_CREATED        = "report.created",         Post, "moderation", "ev-report-created";
    EV_CASE_CREATED          = "case.created",           Post, "moderation", "ev-case-created";
    EV_CASE_RESOLVED         = "case.resolved",          Post, "moderation", "ev-case-resolved";

    // --- Экономика и подсайт --------------------------------------------------
    EV_BANK_PRE_TRANSFER     = "bank.pre_transfer",      Pre,  "economy", "ev-bank-pre-transfer";
    EV_BANK_TRANSFERRED      = "bank.transferred",       Post, "economy", "ev-bank-transferred";
    EV_BANK_ACCOUNT_OPENED   = "bank.account_opened",    Post, "economy", "ev-bank-account-opened";
    EV_HUB_PRE_POST          = "hub.pre_post",           Pre,  "economy", "ev-hub-pre-post";
    EV_HUB_POST_CREATED      = "hub.post_created",       Post, "economy", "ev-hub-post-created";
    EV_HUB_MEMBER_JOINED     = "hub.member_joined",      Post, "economy", "ev-hub-member-joined";
    EV_TOWN_FOUNDED          = "town.founded",           Post, "economy", "ev-town-founded";
    EV_MARKET_LOT_LISTED     = "market.lot_listed",      Post, "economy", "ev-market-lot-listed";
    EV_MARKET_LOT_SOLD       = "market.lot_sold",        Post, "economy", "ev-market-lot-sold";
    EV_FINE_ISSUED           = "fine.issued",            Post, "economy", "ev-fine-issued";
}

/// Найти событие в каталоге. Мастер проверяет этим подписки из манифеста.
pub fn find(name: &str) -> Option<&'static EventMeta> {
    ALL_EVENTS.iter().copied().find(|e| e.name == name)
}
