//! Inter-module RPC calls.
//!
//! Enables direct communication between installed modules.

use noro_module_abi::error::ModuleError;
use noro_module_abi::rpc::ModuleCall;
use serde::de::DeserializeOwned;
use serde::Serialize;

/// Calls an RPC method exposed by another module.
///
/// Returns the parsed response `R` from the target module.
pub fn call<R: DeserializeOwned, P: Serialize>(
    target: &str,
    method: &str,
    payload: &P,
) -> Result<R, ModuleError> {
    let payload = serde_json::to_value(payload)
        .map_err(|e| ModuleError::invalid(format!("rpc payload was not serialized: {e}")))?;
    let raw: serde_json::Value = crate::host::module_call_call(ModuleCall {
        target: target.to_string(),
        method: method.to_string(),
        payload,
    })?;
    serde_json::from_value(raw).map_err(|e| {
        ModuleError::invalid(format!(
            "rpc response from `{target}::{method}` did not parse: {e}"
        ))
    })
}
