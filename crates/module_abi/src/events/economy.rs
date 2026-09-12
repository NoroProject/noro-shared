//! События про банк и подсайт.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::context::EventCtx;
use crate::events::impl_event;
use crate::player::Player;

/// Деньги собираются перевести.
///
/// Отменяемое и изменяемое: `amount` можно уменьшить, и на этом держатся
/// комиссии и налоги. Увеличивать тоже можно, но мастер всё равно проверит
/// баланс отправителя в своей транзакции — модуль не может выдать деньги из
/// воздуха, переписав сумму.
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

/// Перевод состоялся.
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

/// Открыт счёт.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankAccountOpened {
    pub ctx: EventCtx,
    pub server_id: Uuid,
    pub account_id: Uuid,
    #[serde(default)]
    pub owner: Option<Player>,
}

/// Пост собираются опубликовать в ленте.
///
/// Отменяемое и изменяемое: `body` можно переписать — так работает
/// автоматическая чистка текста до того, как его кто-то увидел.
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

/// Игрок вступил в сообщество подсайта.
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
