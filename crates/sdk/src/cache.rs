//! In-memory TTL cache for modules.
//!
//! Values are kept in the master's memory for the duration of `ttl_secs`.
//! The cache is volatile and cleared on master restart or module reload.

use noro_module_abi::cache::{CacheDelete, CacheGet, CacheSet};
use noro_module_abi::error::ModuleError;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::Value;

/// Reads a value from the cache.
///
/// Returns `None` if the key does not exist or has expired.
pub fn get<T: DeserializeOwned>(key: &str) -> Result<Option<T>, ModuleError> {
    let raw: Option<Value> = crate::host::cache_get_call(CacheGet {
        key: key.to_string(),
    })?;
    match raw {
        None => Ok(None),
        Some(v) => serde_json::from_value(v).map(Some).map_err(|e| {
            ModuleError::invalid(format!("cached value under key `{key}` did not parse: {e}"))
        }),
    }
}

/// Reads a value, substituting default if the key is absent or expired.
pub fn get_or<T: DeserializeOwned + Default>(key: &str) -> Result<T, ModuleError> {
    Ok(get(key)?.unwrap_or_default())
}

/// Stores a value in the cache with a specified TTL in seconds.
pub fn set<T: Serialize>(key: &str, value: &T, ttl_secs: u64) -> Result<(), ModuleError> {
    let value = serde_json::to_value(value)
        .map_err(|e| ModuleError::invalid(format!("cache value was not serialized: {e}")))?;
    crate::host::cache_set_call(CacheSet {
        key: key.to_string(),
        value,
        ttl_secs,
    })
}

/// Deletes a key from the cache. Returns `true` if the key was present.
pub fn delete(key: &str) -> Result<bool, ModuleError> {
    crate::host::cache_delete_call(CacheDelete {
        key: key.to_string(),
    })
}
