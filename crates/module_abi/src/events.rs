//! The event registry and the cancellation mechanics.
//!
//! The technique is the same as `nodes!` in permissions: events are introduced
//! by a single macro that produces both the constant and the catalog entry.
//! There is no parallel list to drift from the code — the admin panel shows a
//! module's subscriptions taking the names from here.

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

/// When a module sees an event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EventKind {
    /// Before the action. The module can cancel it and adjust the fields.
    ///
    /// The master waits for the answer, so the handler sits in the path of a
    /// live request: drag it out and you delay a player's join or their
    /// transfer.
    Pre,
    /// After everything has happened and been written. There is nothing to cancel.
    Post,
}

/// A catalog entry.
pub struct EventMeta {
    pub name: &'static str,
    pub kind: EventKind,
    /// The group for the admin panel: `player`, `access`, `infra`, `moderation`, `economy`.
    pub group: &'static str,
    /// The Fluent key holding the human-readable name.
    pub title: &'static str,
    /// The struct a handler for this event accepts.
    ///
    /// Named here rather than derived from `name`, because the two do not line
    /// up: `user.role_granted` is carried by `RoleGranted`, and
    /// `gameserver.online` by `GameServerOnline`. Deriving it produced nine
    /// wrong answers out of fifty-one. `impl_event!` turns a wrong name here
    /// into a compile error.
    pub payload: &'static str,
}

impl EventMeta {
    pub fn cancellable(&self) -> bool {
        matches!(self.kind, EventKind::Pre)
    }
}

/// Byte-wise string comparison, usable in a `const`.
///
/// `==` on `&str` is not a const operation, and the check below has to run at
/// compile time to be worth anything.
const fn str_eq(a: &str, b: &str) -> bool {
    let (a, b) = (a.as_bytes(), b.as_bytes());
    if a.len() != b.len() {
        return false;
    }
    let mut i = 0;
    while i < a.len() {
        if a[i] != b[i] {
            return false;
        }
        i += 1;
    }
    true
}

/// Whether the catalog says this event is carried by this struct.
///
/// Used by `impl_event!` in a `const` assertion, so a catalog entry naming the
/// wrong struct stops the build instead of producing a dead link in the
/// generated documentation.
pub const fn declares_payload(name: &str, payload: &str) -> bool {
    let mut i = 0;
    while i < ALL_EVENTS.len() {
        if str_eq(ALL_EVENTS[i].name, name) {
            return str_eq(ALL_EVENTS[i].payload, payload);
        }
        i += 1;
    }
    false
}

/// The link between an event's struct and its name in the catalog.
///
/// It exists so `#[event]` never asks you to repeat the name as a string: the
/// handler's type already says everything, and a string in two places
/// eventually diverges.
pub trait Event: Serialize + for<'de> Deserialize<'de> {
    const NAME: &'static str;
    const KIND: EventKind;

    /// The circumstances of the event.
    ///
    /// On the trait, not only in a field: the bus selects subscribers by server
    /// before it knows the event's concrete type.
    fn ctx(&self) -> &crate::context::EventCtx;
}

/// A module's decision on a cancellable event.
///
/// The reason is a Fluent key rather than finished text: a player will see the
/// message, and the language is known only on the master's side.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Cancel {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    reason: Option<String>,
}

impl Cancel {
    /// Forbid the action. Calling it again overwrites the reason: the last
    /// handler wins, and the subscription priorities set the order.
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

/// Ties an event's struct to its catalog entry.
///
/// The `impl` only — the structs themselves are written by hand: a module
/// author reads them in the documentation, and macro-generated fields are not
/// visible there.
macro_rules! impl_event {
    ($t:ty, $konst:path, $kind:ident) => {
        impl $crate::events::Event for $t {
            const NAME: &'static str = $konst;
            const KIND: $crate::events::EventKind = $crate::events::EventKind::$kind;

            fn ctx(&self) -> &$crate::context::EventCtx {
                &self.ctx
            }
        }

        // The catalog has to name this very struct. Without this the mismatch
        // is invisible: the code keeps working and only the documentation ends
        // up pointing at a type that does not exist.
        const _: () = assert!($crate::events::declares_payload(
            <$t as $crate::events::Event>::NAME,
            stringify!($t)
        ));
    };
}

pub(crate) use impl_event;

macro_rules! events {
    ($($konst:ident = $name:literal, $kind:ident, $group:literal, $title:literal, $payload:literal;)*) => {
        $(pub const $konst: &str = $name;)*

        /// Every event — for hints in the admin panel and manifest validation.
        pub const ALL_EVENTS: &[&EventMeta] = &[$(&EventMeta {
            name: $name,
            kind: EventKind::$kind,
            group: $group,
            title: $title,
            payload: $payload,
        }),*];
    };
}

events! {
    // --- Players and signing in -----------------------------------------------
    EV_PLAYER_PRE_JOIN        = "player.pre_join",          Pre,  "player",     "ev-player-pre-join",        "PlayerPreJoin";
    EV_PLAYER_JOINED          = "player.joined",            Post, "player",     "ev-player-joined",          "PlayerJoined";
    EV_PLAYER_LEFT            = "player.left",              Post, "player",     "ev-player-left",            "PlayerLeft";
    EV_USER_REGISTERED        = "user.registered",          Post, "player",     "ev-user-registered",        "UserRegistered";
    EV_USER_PRE_LOGIN         = "user.pre_login",           Pre,  "player",     "ev-user-pre-login",         "UserPreLogin";
    EV_USER_LOGGED_IN         = "user.logged_in",           Post, "player",     "ev-user-logged-in",         "UserLoggedIn";
    EV_USER_BANNED            = "user.banned",              Post, "player",     "ev-user-banned",            "UserBanned";
    EV_USER_UNBANNED          = "user.unbanned",            Post, "player",     "ev-user-unbanned",          "UserUnbanned";
    EV_USER_PRE_RENAME        = "user.pre_rename",          Pre,  "player",     "ev-user-pre-rename",        "UserPreRename";
    EV_USER_RENAMED           = "user.renamed",             Post, "player",     "ev-user-renamed",           "UserRenamed";
    EV_USER_SKIN_CHANGED      = "user.skin_changed",        Post, "player",     "ev-user-skin-changed",      "UserSkinChanged";
    EV_IDENTITY_LINKED        = "user.identity_linked",     Post, "player",     "ev-identity-linked",        "IdentityLinked";
    EV_IDENTITY_UNLINKED      = "user.identity_unlinked",   Post, "player",     "ev-identity-unlinked",      "IdentityUnlinked";

    // --- Roles, permissions, access -------------------------------------------
    EV_ROLE_CREATED           = "role.created",             Post, "access",     "ev-role-created",           "RoleCreated";
    EV_ROLE_UPDATED           = "role.updated",             Post, "access",     "ev-role-updated",           "RoleUpdated";
    EV_ROLE_DELETED           = "role.deleted",             Post, "access",     "ev-role-deleted",           "RoleDeleted";
    EV_ROLE_PRE_GRANT         = "user.pre_role_granted",    Pre,  "access",     "ev-role-pre-grant",         "RolePreGrant";
    EV_ROLE_GRANTED           = "user.role_granted",        Post, "access",     "ev-role-granted",           "RoleGranted";
    EV_ROLE_REVOKED           = "user.role_revoked",        Post, "access",     "ev-role-revoked",           "RoleRevoked";
    EV_PERM_GRANTED           = "user.permission_granted",  Post, "access",     "ev-perm-granted",           "PermissionGranted";
    EV_PERM_REVOKED           = "user.permission_revoked",  Post, "access",     "ev-perm-revoked",           "PermissionRevoked";

    // --- Servers, builds, game servers ----------------------------------------
    EV_SERVER_CREATED         = "server.created",           Post, "infra",      "ev-server-created",         "ServerCreated";
    EV_SERVER_UPDATED         = "server.updated",           Post, "infra",      "ev-server-updated",         "ServerUpdated";
    EV_SERVER_DELETED         = "server.deleted",           Post, "infra",      "ev-server-deleted",         "ServerDeleted";
    EV_BUILD_CREATED          = "build.created",            Post, "infra",      "ev-build-created",          "BuildCreated";
    EV_BUILD_PRE_PUBLISH      = "build.pre_publish",        Pre,  "infra",      "ev-build-pre-publish",      "BuildPrePublish";
    EV_BUILD_PUBLISHED        = "build.published",          Post, "infra",      "ev-build-published",        "BuildPublished";
    EV_BUILD_DELETED          = "build.deleted",            Post, "infra",      "ev-build-deleted",          "BuildDeleted";
    EV_GAMESERVER_ONLINE      = "gameserver.online",        Post, "infra",      "ev-gameserver-online",      "GameServerOnline";
    EV_GAMESERVER_OFFLINE     = "gameserver.offline",       Post, "infra",      "ev-gameserver-offline",     "GameServerOffline";
    EV_GAMESERVER_MAINTENANCE = "gameserver.maintenance",   Post, "infra",      "ev-gameserver-maintenance", "GameServerMaintenance";
    EV_INSTANCE_SETTING       = "instance.setting_changed", Post, "infra",      "ev-instance-setting",       "InstanceSettingChanged";
    EV_MODULE_ENABLED         = "module.enabled",           Post, "infra",      "ev-module-enabled",         "ModuleEnabled";
    EV_MODULE_DISABLED        = "module.disabled",          Post, "infra",      "ev-module-disabled",        "ModuleDisabled";

    // --- Moderation -----------------------------------------------------------
    EV_PUNISH_PRE_ISSUE       = "punishment.pre_issue",     Pre,  "moderation", "ev-punish-pre-issue",       "PunishmentPreIssue";
    EV_PUNISH_ISSUED          = "punishment.issued",        Post, "moderation", "ev-punish-issued",          "PunishmentIssued";
    EV_PUNISH_REVOKED         = "punishment.revoked",       Post, "moderation", "ev-punish-revoked",         "PunishmentRevoked";
    EV_PUNISH_EXPIRED         = "punishment.expired",       Post, "moderation", "ev-punish-expired",         "PunishmentExpired";
    EV_REPORT_CREATED         = "report.created",           Post, "moderation", "ev-report-created",         "ReportCreated";
    EV_CASE_CREATED           = "case.created",             Post, "moderation", "ev-case-created",           "CaseCreated";
    EV_CASE_RESOLVED          = "case.resolved",            Post, "moderation", "ev-case-resolved",          "CaseResolved";

    // --- Economy and the hub --------------------------------------------------
    EV_BANK_PRE_TRANSFER      = "bank.pre_transfer",        Pre,  "economy",    "ev-bank-pre-transfer",      "BankPreTransfer";
    EV_BANK_TRANSFERRED       = "bank.transferred",         Post, "economy",    "ev-bank-transferred",       "BankTransferred";
    EV_BANK_ACCOUNT_OPENED    = "bank.account_opened",      Post, "economy",    "ev-bank-account-opened",    "BankAccountOpened";
    EV_HUB_PRE_POST           = "hub.pre_post",             Pre,  "economy",    "ev-hub-pre-post",           "HubPrePost";
    EV_HUB_POST_CREATED       = "hub.post_created",         Post, "economy",    "ev-hub-post-created",       "HubPostCreated";
    EV_HUB_MEMBER_JOINED      = "hub.member_joined",        Post, "economy",    "ev-hub-member-joined",      "HubMemberJoined";
    EV_TOWN_FOUNDED           = "town.founded",             Post, "economy",    "ev-town-founded",           "TownFounded";
    EV_MARKET_LOT_LISTED      = "market.lot_listed",        Post, "economy",    "ev-market-lot-listed",      "MarketLotListed";
    EV_MARKET_LOT_SOLD        = "market.lot_sold",          Post, "economy",    "ev-market-lot-sold",        "MarketLotSold";
    EV_FINE_ISSUED            = "fine.issued",              Post, "economy",    "ev-fine-issued",            "FineIssued";
}

/// Find an event in the catalog. The master validates declared subscriptions with this.
pub fn find(name: &str) -> Option<&'static EventMeta> {
    ALL_EVENTS.iter().copied().find(|e| e.name == name)
}
