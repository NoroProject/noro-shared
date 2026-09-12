//! Events about roles, permissions and access.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::context::EventCtx;
use crate::entity::Role;
use crate::events::impl_event;
use crate::player::Player;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleCreated {
    pub ctx: EventCtx,
    pub role: Role,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleUpdated {
    pub ctx: EventCtx,
    pub role: Role,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleDeleted {
    pub ctx: EventCtx,
    pub role_id: Uuid,
    pub name: String,
}

/// A role is about to be granted to a player.
///
/// Cancellable: this is how the rule "only the module grants this role" is
/// held, without relying on nobody pressing the button in the admin panel.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RolePreGrant {
    pub ctx: EventCtx,
    pub player: Player,
    pub role: Role,
    #[serde(default, flatten)]
    pub cancel: crate::events::Cancel,
}

impl RolePreGrant {
    pub fn cancel(&mut self, reason_key: impl Into<String>) {
        self.cancel.cancel(reason_key);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleGranted {
    pub ctx: EventCtx,
    pub player: Player,
    pub role: Role,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleRevoked {
    pub ctx: EventCtx,
    pub player: Player,
    pub role: Role,
}

/// A personal permission was granted to a player.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionGranted {
    pub ctx: EventCtx,
    pub player: Player,
    pub permission: String,
    /// The server it was granted in the context of. `None` means everywhere.
    #[serde(default)]
    pub server_id: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionRevoked {
    pub ctx: EventCtx,
    pub player: Player,
    pub permission: String,
    #[serde(default)]
    pub server_id: Option<Uuid>,
}

impl_event!(RoleCreated, super::EV_ROLE_CREATED, Post);
impl_event!(RoleUpdated, super::EV_ROLE_UPDATED, Post);
impl_event!(RoleDeleted, super::EV_ROLE_DELETED, Post);
impl_event!(RolePreGrant, super::EV_ROLE_PRE_GRANT, Pre);
impl_event!(RoleGranted, super::EV_ROLE_GRANTED, Post);
impl_event!(RoleRevoked, super::EV_ROLE_REVOKED, Post);
impl_event!(PermissionGranted, super::EV_PERM_GRANTED, Post);
impl_event!(PermissionRevoked, super::EV_PERM_REVOKED, Post);
