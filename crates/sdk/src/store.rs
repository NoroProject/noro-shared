//! The module's own storage.
//!
//! Key-value with scopes: a record belongs either to the whole instance, or to
//! a server, or to a player. The scope is part of the key rather than a filter,
//! so a player's `points` and a server's `points` do not overlap and you never
//! have to splice identifiers into the key by hand.
//!
//! The data is removed together with the module. For tables, queries and
//! sorting there is your own Postgres schema — none of that lives here.

use noro_module_abi::error::ModuleError;
use noro_module_abi::store::{StoreGet, StoreIncr, StoreList, StoreScope, StoreSet};
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::Value;
use uuid::Uuid;

/// Storage in the chosen scope.
pub struct Store(StoreScope);

/// Shared across the instance: module settings, counters, flags.
pub fn instance() -> Store {
    Store(StoreScope::Instance)
}

/// Separate for each server.
pub fn server(id: Uuid) -> Store {
    Store(StoreScope::server(id))
}

/// Separate for each player. Removed together with the player.
pub fn user(id: Uuid) -> Store {
    Store(StoreScope::user(id))
}

impl Store {
    /// Reads a value and parses it into the requested type.
    ///
    /// `None` means the key is absent. If a value is there but does not parse
    /// into `T`, that is an error: a silent `None` here would mean data loss
    /// noticed a week later.
    pub fn get<T: DeserializeOwned>(&self, key: &str) -> Result<Option<T>, ModuleError> {
        let raw: Option<Value> = crate::host::store_get_call(StoreGet {
            scope: self.0,
            key: key.to_string(),
        })?;
        match raw {
            None => Ok(None),
            Some(v) => serde_json::from_value(v).map(Some).map_err(|e| {
                ModuleError::invalid(format!("value under key `{key}` did not parse: {e}"))
            }),
        }
    }

    /// Reads a value, substituting the default when the key is absent.
    pub fn get_or<T: DeserializeOwned + Default>(&self, key: &str) -> Result<T, ModuleError> {
        Ok(self.get(key)?.unwrap_or_default())
    }

    pub fn set<T: Serialize>(&self, key: &str, value: &T) -> Result<(), ModuleError> {
        let value = serde_json::to_value(value)
            .map_err(|e| ModuleError::invalid(format!("value was not serialized: {e}")))?;
        crate::host::store_set_call(StoreSet {
            scope: self.0,
            key: key.to_string(),
            value,
        })
    }

    /// Deletes a key. `true` means it was there.
    pub fn delete(&self, key: &str) -> Result<bool, ModuleError> {
        crate::host::store_delete_call(StoreGet {
            scope: self.0,
            key: key.to_string(),
        })
    }

    /// Adds to a number and returns the result.
    ///
    /// One operation rather than a read followed by a write: event handlers may
    /// well run at the same time, and a read-add-write trio would lose
    /// accruals.
    ///
    /// If what is under the key is not a number, the call fails instead of
    /// overwriting it.
    pub fn incr(&self, key: &str, delta: i64) -> Result<i64, ModuleError> {
        crate::host::store_incr_call(StoreIncr {
            scope: self.0,
            key: key.to_string(),
            delta,
        })
    }

    /// Keys and values by prefix, in pages.
    pub fn list(
        &self,
        prefix: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<(String, Value)>, ModuleError> {
        crate::host::store_list_call(StoreList {
            scope: self.0,
            prefix: prefix.to_string(),
            limit,
            offset,
        })
    }
}
