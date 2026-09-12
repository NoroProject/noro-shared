//! События про роли, права и доступы.

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

/// Игроку собираются выдать роль.
///
/// Отменяемое: так держат правило «эту роль выдаёт только модуль», не полагаясь
/// на то, что никто не нажмёт кнопку в админке.
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

/// Игроку выдали личное право.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionGranted {
    pub ctx: EventCtx,
    pub player: Player,
    pub permission: String,
    /// Сборка, в контексте которой выдано. `None` — везде.
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
