//! Events about private messages between players.
//!
//! Handling these means reading what two people wrote to each other, and they
//! did not write it to you. Subscribe when the job needs it — filtering,
//! auto-replies, answering machines — and leave the rest alone. The operator
//! sees `dm = ["read"]` in the manifest and decides whether your module gets
//! that; the privacy policy of the instance says the same in plain words.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::context::EventCtx;
use crate::events::impl_event;
use crate::player::Player;

/// A private message is about to be sent.
///
/// `body` is yours to change: cutting a link out of a message reads better than
/// refusing the whole thing, and a refusal is something the sender has to be
/// told about.
///
/// Cancelling here is indistinguishable to the sender from being blocked — that
/// is deliberate, and it is why the refusal carries no module name to the
/// player. What you cancel is not delivered and not stored: for the recipient
/// the message never existed.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DmPreSend {
    pub ctx: EventCtx,
    pub from: Player,
    pub to: Player,
    pub body: String,
    /// `site`, `game` or `module` — where it was written.
    pub source: String,
    /// The game server it was written on, when it was written in game.
    #[serde(default)]
    pub game_server_id: Option<Uuid>,
    #[serde(default, flatten)]
    pub cancel: crate::events::Cancel,
}

/// A private message was stored and delivered.
///
/// After the fact: nothing here changes anything. For counters, for a bot that
/// answers, for anything that has to happen once the message is real.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DmSent {
    pub ctx: EventCtx,
    pub id: Uuid,
    pub thread_id: Uuid,
    pub from: Player,
    pub to: Player,
    pub body: String,
    pub source: String,
    #[serde(default)]
    pub game_server_id: Option<Uuid>,
}

impl_event!(DmPreSend, super::EV_DM_PRE_SEND, Pre);
impl_event!(DmSent, super::EV_DM_SENT, Post);
