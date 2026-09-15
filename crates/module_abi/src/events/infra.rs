//! Events about servers, builds, game servers and the instance itself.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

use crate::context::EventCtx;
use crate::entity::{Build, GameServer, Server};
use crate::events::impl_event;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerCreated {
    pub ctx: EventCtx,
    pub server: Server,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerUpdated {
    pub ctx: EventCtx,
    pub server: Server,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerDeleted {
    pub ctx: EventCtx,
    pub server_id: Uuid,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildCreated {
    pub ctx: EventCtx,
    pub build: Build,
}

/// A build is about to be published.
///
/// Cancellable: the place for checks before a rollout — for instance,
/// forbidding publication until the file integrity check has passed.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildPrePublish {
    pub ctx: EventCtx,
    pub build: Build,
    #[serde(default, flatten)]
    pub cancel: crate::events::Cancel,
}

impl BuildPrePublish {
    pub fn cancel(&mut self, reason_key: impl Into<String>) {
        self.cancel.cancel(reason_key);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildPublished {
    pub ctx: EventCtx,
    pub build: Build,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildDeleted {
    pub ctx: EventCtx,
    pub build_id: Uuid,
    pub server_id: Uuid,
}

/// A game server's agent connected.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameServerOnline {
    pub ctx: EventCtx,
    pub game_server: GameServer,
}

/// The agent dropped, or the server was shut down.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameServerOffline {
    pub ctx: EventCtx,
    pub game_server: GameServer,
}

/// Maintenance mode was turned on or off.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameServerMaintenance {
    pub ctx: EventCtx,
    pub game_server: GameServer,
    pub enabled: bool,
}

/// An instance setting changed.
///
/// The value arrives as is, unparsed: the set of settings grows, and typing it
/// in the public ABI would mean freezing an internal list.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstanceSettingChanged {
    pub ctx: EventCtx,
    pub key: String,
    #[serde(default)]
    pub value: Option<Value>,
}

/// Another module was enabled.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleEnabled {
    pub ctx: EventCtx,
    pub module_id: String,
    pub version: String,
}

/// Another module was disabled — by hand, or automatically after failures.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleDisabled {
    pub ctx: EventCtx,
    pub module_id: String,
    /// Disabled by the automation rather than by a person.
    pub automatic: bool,
}

impl_event!(ServerCreated, super::EV_SERVER_CREATED, Post);
impl_event!(ServerUpdated, super::EV_SERVER_UPDATED, Post);
impl_event!(ServerDeleted, super::EV_SERVER_DELETED, Post);
impl_event!(BuildCreated, super::EV_BUILD_CREATED, Post);
impl_event!(BuildPrePublish, super::EV_BUILD_PRE_PUBLISH, Pre);
impl_event!(BuildPublished, super::EV_BUILD_PUBLISHED, Post);
impl_event!(BuildDeleted, super::EV_BUILD_DELETED, Post);
impl_event!(GameServerOnline, super::EV_GAMESERVER_ONLINE, Post);
impl_event!(GameServerOffline, super::EV_GAMESERVER_OFFLINE, Post);
impl_event!(
    GameServerMaintenance,
    super::EV_GAMESERVER_MAINTENANCE,
    Post
);
impl_event!(InstanceSettingChanged, super::EV_INSTANCE_SETTING, Post);
impl_event!(ModuleEnabled, super::EV_MODULE_ENABLED, Post);
impl_event!(ModuleDisabled, super::EV_MODULE_DISABLED, Post);
