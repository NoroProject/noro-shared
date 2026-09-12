//! Events about punishments, reports and cases.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::context::EventCtx;
use crate::entity::Punishment;
use crate::events::impl_event;
use crate::player::Player;

/// A punishment is about to be issued.
///
/// Cancellable and mutable: `duration_secs` and `reason` can be rewritten.
/// That is how an automatic ladder of durations is built — an hour instead of
/// ten minutes for a third mute in a row, without editing the rules in the
/// admin panel.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PunishmentPreIssue {
    pub ctx: EventCtx,
    pub target: Player,
    /// `ban`, `mute`, `warn`, `kick`.
    pub kind: String,
    pub reason: String,
    /// The duration in seconds. `None` means forever.
    #[serde(default)]
    pub duration_secs: Option<i64>,
    /// The server it applies to. `None` means all of them.
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

/// The punishment ran out and the master lifted it by itself.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PunishmentExpired {
    pub ctx: EventCtx,
    pub target: Player,
    pub punishment: Punishment,
}

/// A player reported a player.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportCreated {
    pub ctx: EventCtx,
    pub report_id: Uuid,
    pub reporter: Player,
    pub target: Player,
    pub reason: String,
}

/// A case was opened.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaseCreated {
    pub ctx: EventCtx,
    pub case_id: Uuid,
    pub target: Player,
}

/// A case was closed.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaseResolved {
    pub ctx: EventCtx,
    pub case_id: Uuid,
    pub target: Player,
    /// `resolved` or `rejected`.
    pub status: String,
}

impl_event!(PunishmentPreIssue, super::EV_PUNISH_PRE_ISSUE, Pre);
impl_event!(PunishmentIssued, super::EV_PUNISH_ISSUED, Post);
impl_event!(PunishmentRevoked, super::EV_PUNISH_REVOKED, Post);
impl_event!(PunishmentExpired, super::EV_PUNISH_EXPIRED, Post);
impl_event!(ReportCreated, super::EV_REPORT_CREATED, Post);
impl_event!(CaseCreated, super::EV_CASE_CREATED, Post);
impl_event!(CaseResolved, super::EV_CASE_RESOLVED, Post);
