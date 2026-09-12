//! Requests to a module's KV store.
//!
//! The types are shared by the SDK and the master: they must not diverge, or
//! the module writes under one key while the master reads another.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

/// What a record is attached to.
///
/// The scope is part of the key rather than a filter: a player's `points` and a
/// server's `points` live apart, and the module never has to splice the
/// identifier into the key itself.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "scope", rename_all = "snake_case")]
pub enum StoreScope {
    /// One record for the whole instance.
    Instance,
    /// Separate for each server.
    Server { id: Uuid },
    /// Separate for each player.
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
    /// The start of the key. Empty means every key in the scope.
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

/// The page ceiling. A module may ask for more — it gets this much.
pub const MAX_LIST_LIMIT: i64 = 500;
