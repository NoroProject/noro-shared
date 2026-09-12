//! События про наказания, репорты и дела.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::context::EventCtx;
use crate::entity::Punishment;
use crate::events::impl_event;
use crate::player::Player;

/// Наказание собираются выдать.
///
/// Отменяемое и изменяемое: `duration_secs` и `reason` можно переписать. Так
/// делается автоматическая лестница сроков — за третий мут подряд час вместо
/// десяти минут, без правки правил в админке.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PunishmentPreIssue {
    pub ctx: EventCtx,
    pub target: Player,
    /// `ban`, `mute`, `warn`, `kick`.
    pub kind: String,
    pub reason: String,
    /// Срок в секундах. `None` — навсегда.
    #[serde(default)]
    pub duration_secs: Option<i64>,
    /// Сборка, на которой действует. `None` — на всех.
    #[serde(default)]
    pub server_id: Option<Uuid>,
    #[serde(default, flatten)]
    pub cancel: crate::events::Cancel,
}

impl PunishmentPreIssue {
    pub fn cancel(&mut self, reason_key: impl Into<String>) {
        self.cancel.cancel(reason_key);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PunishmentIssued {
    pub ctx: EventCtx,
    pub target: Player,
    pub punishment: Punishment,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PunishmentRevoked {
    pub ctx: EventCtx,
    pub target: Player,
    pub punishment: Punishment,
}

/// Срок наказания вышел, мастер снял его сам.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PunishmentExpired {
    pub ctx: EventCtx,
    pub target: Player,
    pub punishment: Punishment,
}

/// Игрок пожаловался на игрока.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportCreated {
    pub ctx: EventCtx,
    pub report_id: Uuid,
    pub reporter: Player,
    pub target: Player,
    pub reason: String,
}

/// Заведено дело.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaseCreated {
    pub ctx: EventCtx,
    pub case_id: Uuid,
    pub target: Player,
}

/// Дело закрыто.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaseResolved {
    pub ctx: EventCtx,
    pub case_id: Uuid,
    pub target: Player,
    /// `resolved` или `rejected`.
    pub status: String,
}

impl_event!(PunishmentPreIssue, super::EV_PUNISH_PRE_ISSUE, Pre);
impl_event!(PunishmentIssued, super::EV_PUNISH_ISSUED, Post);
impl_event!(PunishmentRevoked, super::EV_PUNISH_REVOKED, Post);
impl_event!(PunishmentExpired, super::EV_PUNISH_EXPIRED, Post);
impl_event!(ReportCreated, super::EV_REPORT_CREATED, Post);
impl_event!(CaseCreated, super::EV_CASE_CREATED, Post);
impl_event!(CaseResolved, super::EV_CASE_RESOLVED, Post);
