//! Instance settings.
//!
//! The same settings the owner edits in the panel. They are stored as JSON and
//! not typed by the ABI: the set grows, and freezing it here would mean a new
//! setting needing an ABI release.
//!
//! ```ignore
//! let name: Option<String> = instance::get("brand.name")?;
//! instance::set("shop.enabled", &true)?;
//! ```

use noro_module_abi::error::ModuleError;
use noro_module_abi::ops::SettingWrite;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::Value;
use std::collections::BTreeMap;

/// Every setting, by key.
///
/// Requires `instance = ["read"]`.
pub fn all() -> Result<BTreeMap<String, Value>, ModuleError> {
    crate::host::instance_all_call(())
}

/// One setting, parsed into the type you expect.
///
/// `None` means the key is not set — the master then uses its own default, and
/// what that default is, is its business rather than something to duplicate
/// here.
///
/// Requires `instance = ["read"]`.
pub fn get<T: DeserializeOwned>(key: &str) -> Result<Option<T>, ModuleError> {
    let raw: Option<Value> = crate::host::instance_get_call(key.to_string())?;
    match raw {
        None => Ok(None),
        Some(v) => serde_json::from_value(v)
            .map(Some)
            .map_err(|e| ModuleError::invalid(format!("setting `{key}` did not parse: {e}"))),
    }
}

/// Writes a setting.
///
/// Careful: these are the instance's own settings, and a module rewriting one
/// the owner set by hand is a surprise. Write the ones your module owns.
///
/// Requires `instance = ["read", "write"]`.
pub fn set<T: Serialize>(key: &str, value: &T) -> Result<(), ModuleError> {
    let value = serde_json::to_value(value)
        .map_err(|e| ModuleError::invalid(format!("value was not serialized: {e}")))?;
    crate::host::instance_set_call(SettingWrite {
        key: key.to_string(),
        value,
    })
}
