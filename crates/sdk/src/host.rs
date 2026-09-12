//! The bridge to the master.
//!
//! Every function here is a call into the master's own process, not a network
//! call. JSON travels across the wasm boundary because nothing else travels
//! across it at all: that is a property of the sandbox, not an architectural
//! choice. It costs single-digit microseconds.

use extism_pdk::*;
use noro_module_abi::error::{ErrorKind, ModuleError};
use serde::de::DeserializeOwned;
use serde::Serialize;

#[host_fn]
extern "ExtismHost" {
    fn player_get(input: String) -> String;
    fn store_get(input: String) -> String;
    fn store_set(input: String) -> String;
    fn store_delete(input: String) -> String;
    fn store_incr(input: String) -> String;
    fn store_list(input: String) -> String;
    fn host_now(input: String) -> String;
    fn log(input: String);
}

/// The shared wrapper: serialize the input, call the master, unwrap the answer.
macro_rules! call {
    ($f:ident, $arg:expr) => {{
        let raw = ::serde_json::to_string(&$arg).map_err(encode_failed)?;
        let out = unsafe { $f(raw) }.map_err(bridge_failed)?;
        decode(&out)
    }};
}

pub(crate) fn player_get_call<A: Serialize, R: DeserializeOwned>(arg: A) -> Result<R, ModuleError> {
    call!(player_get, arg)
}

pub(crate) fn store_get_call<A: Serialize, R: DeserializeOwned>(arg: A) -> Result<R, ModuleError> {
    call!(store_get, arg)
}

pub(crate) fn store_set_call<A: Serialize, R: DeserializeOwned>(arg: A) -> Result<R, ModuleError> {
    call!(store_set, arg)
}

pub(crate) fn store_delete_call<A: Serialize, R: DeserializeOwned>(
    arg: A,
) -> Result<R, ModuleError> {
    call!(store_delete, arg)
}

pub(crate) fn store_incr_call<A: Serialize, R: DeserializeOwned>(arg: A) -> Result<R, ModuleError> {
    call!(store_incr, arg)
}

pub(crate) fn store_list_call<A: Serialize, R: DeserializeOwned>(arg: A) -> Result<R, ModuleError> {
    call!(store_list, arg)
}

/// The master's clock, in epoch seconds.
pub fn now_secs() -> i64 {
    let raw = match unsafe { host_now(String::from("null")) } {
        Ok(v) => v,
        Err(_) => return 0,
    };
    decode::<i64>(&raw).unwrap_or(0)
}

/// Writes a line to the master's log. Bridge errors are swallowed here on
/// purpose: crashing over a failed log write is the worst thing a module can
/// do.
pub fn log_line(level: &str, message: &str) {
    #[derive(Serialize)]
    struct Entry<'a> {
        level: &'a str,
        message: &'a str,
    }
    if let Ok(raw) = serde_json::to_string(&Entry { level, message }) {
        let _ = unsafe { log(raw) };
    }
}

/// The master answers with a serde-serialized `Result`: `{"Ok":…}` or `{"Err":…}`.
fn decode<R: DeserializeOwned>(raw: &str) -> Result<R, ModuleError> {
    match serde_json::from_str::<Result<R, ModuleError>>(raw) {
        Ok(v) => v,
        Err(e) => Err(ModuleError::new(
            ErrorKind::Internal,
            format!("the master's answer did not parse: {e}"),
        )),
    }
}

fn encode_failed(e: serde_json::Error) -> ModuleError {
    ModuleError::new(ErrorKind::Invalid, format!("the argument was not serialized: {e}"))
}

/// Only breakages of the bridge itself land here: the master could not write
/// the answer into the module's memory, or the export was not found.
fn bridge_failed(e: extism_pdk::Error) -> ModuleError {
    ModuleError::new(
        ErrorKind::Internal,
        format!("the call to the master did not go through: {e}"),
    )
}
