//! Events about players and signing in.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::context::EventCtx;
use crate::events::impl_event;
use crate::player::Player;

/// A player is trying to join a game server.
///
/// Cancellable: `cancel` keeps the player out and shows them the reason. The
/// handler sits in the path of a live connection — if the module drags it out,
/// the player waits.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerPreJoin {
    pub ctx: EventCtx,
    pub player: Player,
    /// What they are joining.
    pub game_server_id: Uuid,
    #[serde(default, flatten)]
    pub cancel: crate::events::Cancel,
}

impl PlayerPreJoin {
    /// Keep the player out. `reason_key` is a Fluent key; the player sees it.
    pub fn cancel(&mut self, reason_key: impl Into<String>) {
        self.cancel.cancel(reason_key);
    }
}

/// A player joined the game.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerJoined {
    pub ctx: EventCtx,
    pub player: Player,
    pub game_server_id: Uuid,
    /// Whether this is the player's first ever join to this instance.
    pub first_join: bool,
}

/// A player left.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerLeft {
    pub ctx: EventCtx,
    pub player: Player,
    pub game_server_id: Uuid,
    /// How long the session lasted. Zero when the server restarted and lost count.
    pub session_secs: i64,
}

/// An account was created.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRegistered {
    pub ctx: EventCtx,
    pub player: Player,
    /// What they registered through: `discord`, `twitch`, `local`.
    pub provider: String,
}

/// A player is signing in to the cabinet, the admin panel or the launcher.
///
/// Cancellable: a module can close sign-in without touching any ban. That is
/// how staff-only maintenance is done without revoking anyone's access.
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

/// The sign-in went through.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserLoggedIn {
    pub ctx: EventCtx,
    pub player: Player,
    pub provider: String,
}

/// A player was banned by the account flag (not by a punishment).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserBanned {
    pub ctx: EventCtx,
    pub player: Player,
    #[serde(default)]
    pub reason: Option<String>,
}

/// The account ban was lifted.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserUnbanned {
    pub ctx: EventCtx,
    pub player: Player,
}

/// A player is changing their username.
///
/// Cancellable and mutable: a module can forbid a taken name or force it to
/// its own casing by rewriting `new_name`.
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

/// The username changed.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRenamed {
    pub ctx: EventCtx,
    pub player: Player,
    #[serde(default)]
    pub old_name: Option<String>,
    pub new_name: String,
}

/// The skin or the cape changed.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSkinChanged {
    pub ctx: EventCtx,
    pub player: Player,
    /// `skin` or `cape`.
    pub what: String,
}

/// A login method was linked.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityLinked {
    pub ctx: EventCtx,
    pub player: Player,
    pub provider: String,
    pub external_id: String,
}

/// A login method was unlinked.
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
