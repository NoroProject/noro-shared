//! Requests to a module's in-memory TTL cache.

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheGet {
    pub key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheSet {
    pub key: String,
    pub value: Value,
    pub ttl_secs: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheDelete {
    pub key: String,
}
