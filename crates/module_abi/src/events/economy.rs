//! Events about the bank and the hub.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::context::EventCtx;
use crate::events::impl_event;
use crate::player::Player;

/// Money is about to be transferred.
///
/// Cancellable and mutable: `amount` can be lowered, and fees and taxes rest
/// on that. Raising it is possible too, but the master still checks the
/// sender's balance inside its own transaction — a module cannot conjure money
/// out of nothing by rewriting the sum.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankPreTransfer {
    pub ctx: EventCtx,
    pub server_id: Uuid,
    pub from: Uuid,
    pub to: Uuid,
    pub amount: i64,
    #[serde(default)]
    pub comment: Option<String>,
    #[serde(default, flatten)]
    pub cancel: crate::events::Cancel,
}

impl BankPreTransfer {
    pub fn cancel(&mut self, reason_key: impl Into<String>) {
        self.cancel.cancel(reason_key);
    }
}

/// The transfer went through.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankTransferred {
    pub ctx: EventCtx,
    pub server_id: Uuid,
    pub tx_id: i64,
    pub from: Uuid,
    pub to: Uuid,
    pub amount: i64,
    #[serde(default)]
    pub comment: Option<String>,
}

/// An account was opened.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankAccountOpened {
    pub ctx: EventCtx,
    pub server_id: Uuid,
    pub account_id: Uuid,
    #[serde(default)]
    pub owner: Option<Player>,
}

/// A post is about to be published to the feed.
///
/// Cancellable and mutable: `body` can be rewritten — that is how automatic
/// text cleanup works before anyone has seen it.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HubPrePost {
    pub ctx: EventCtx,
    pub server_id: Uuid,
    pub author: Player,
    pub body: String,
    #[serde(default, flatten)]
    pub cancel: crate::events::Cancel,
}

impl HubPrePost {
    pub fn cancel(&mut self, reason_key: impl Into<String>) {
        self.cancel.cancel(reason_key);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HubPostCreated {
    pub ctx: EventCtx,
    pub server_id: Uuid,
    pub post_id: Uuid,
    pub author: Player,
}

/// A player joined the hub's community.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HubMemberJoined {
    pub ctx: EventCtx,
    pub server_id: Uuid,
    pub player: Player,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TownFounded {
    pub ctx: EventCtx,
    pub server_id: Uuid,
    pub town_id: Uuid,
    pub name: String,
    pub mayor: Player,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketLotListed {
    pub ctx: EventCtx,
    pub server_id: Uuid,
    pub lot_id: Uuid,
    pub seller: Player,
    pub price: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketLotSold {
    pub ctx: EventCtx,
    pub server_id: Uuid,
    pub lot_id: Uuid,
    pub seller: Player,
    pub buyer: Player,
    pub price: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FineIssued {
    pub ctx: EventCtx,
    pub server_id: Uuid,
    pub fine_id: Uuid,
    pub target: Player,
    pub amount: i64,
    pub reason: String,
}

impl_event!(BankPreTransfer, super::EV_BANK_PRE_TRANSFER, Pre);
impl_event!(BankTransferred, super::EV_BANK_TRANSFERRED, Post);
impl_event!(BankAccountOpened, super::EV_BANK_ACCOUNT_OPENED, Post);
impl_event!(HubPrePost, super::EV_HUB_PRE_POST, Pre);
impl_event!(HubPostCreated, super::EV_HUB_POST_CREATED, Post);
impl_event!(HubMemberJoined, super::EV_HUB_MEMBER_JOINED, Post);
impl_event!(TownFounded, super::EV_TOWN_FOUNDED, Post);
impl_event!(MarketLotListed, super::EV_MARKET_LOT_LISTED, Post);
impl_event!(MarketLotSold, super::EV_MARKET_LOT_SOLD, Post);
impl_event!(FineIssued, super::EV_FINE_ISSUED, Post);
