//! Запросы к KV-хранилищу модуля.
//!
//! Типы общие для SDK и мастера: разойтись им нельзя, иначе модуль пишет по
//! одному ключу, а мастер читает по другому.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

/// К чему привязана запись.
///
/// Область — часть ключа, а не фильтр: `points` игрока и `points` сборки живут
/// раздельно, и модулю не приходится склеивать идентификатор в строку самому.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "scope", rename_all = "snake_case")]
pub enum StoreScope {
    /// Одна запись на весь инстанс.
    Instance,
    /// Своя на каждую сборку.
    Server { id: Uuid },
    /// Своя на каждого игрока.
    User { id: Uuid },
}

impl StoreScope {
    pub fn server(id: Uuid) -> Self {
        StoreScope::Server { id }
    }

    pub fn user(id: Uuid) -> Self {
        StoreScope::User { id }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoreGet {
    #[serde(flatten)]
    pub scope: StoreScope,
    pub key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoreSet {
    #[serde(flatten)]
    pub scope: StoreScope,
    pub key: String,
    pub value: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoreIncr {
    #[serde(flatten)]
    pub scope: StoreScope,
    pub key: String,
    pub delta: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoreList {
    #[serde(flatten)]
    pub scope: StoreScope,
    /// Начало ключа. Пусто — все ключи области.
    #[serde(default)]
    pub prefix: String,
    #[serde(default = "default_limit")]
    pub limit: i64,
    #[serde(default)]
    pub offset: i64,
}

fn default_limit() -> i64 {
    100
}

/// Потолок страницы. Модуль может попросить больше — получит столько.
pub const MAX_LIST_LIMIT: i64 = 500;
