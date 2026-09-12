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
    fn chat_tell(input: String) -> String;
    fn punish_issue(input: String) -> String;
    fn bank_account(input: String) -> String;
    fn bank_accounts(input: String) -> String;
    fn bank_treasury(input: String) -> String;
    fn bank_transfer(input: String) -> String;
    fn punish_revoke(input: String) -> String;
    fn punish_active(input: String) -> String;
    fn punish_list(input: String) -> String;
    fn chat_announce(input: String) -> String;
    fn chat_kick(input: String) -> String;
    fn player_ban(input: String) -> String;
    fn roles_list(input: String) -> String;
    fn role_get(input: String) -> String;
    fn roles_of(input: String) -> String;
    fn role_grant(input: String) -> String;
    fn role_revoke(input: String) -> String;
    fn perm_has(input: String) -> String;
    fn perm_effective(input: String) -> String;
    fn perm_grant(input: String) -> String;
    fn perm_revoke(input: String) -> String;
    fn access_join(input: String) -> String;
    fn access_join_revoke(input: String) -> String;
    fn access_build(input: String) -> String;
    fn access_build_revoke(input: String) -> String;
    fn servers_list(input: String) -> String;
    fn server_get(input: String) -> String;
    fn server_by_slug(input: String) -> String;
    fn builds_list(input: String) -> String;
    fn build_published(input: String) -> String;
    fn gameservers_list(input: String) -> String;
    fn gameserver_get(input: String) -> String;
    fn gameserver_maintenance(input: String) -> String;
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

pub(crate) fn bank_account_call<A: Serialize, R: DeserializeOwned>(
    arg: A,
) -> Result<R, ModuleError> {
    call!(bank_account, arg)
}

pub(crate) fn bank_accounts_call<A: Serialize, R: DeserializeOwned>(
    arg: A,
) -> Result<R, ModuleError> {
    call!(bank_accounts, arg)
}

pub(crate) fn bank_treasury_call<A: Serialize, R: DeserializeOwned>(
    arg: A,
) -> Result<R, ModuleError> {
    call!(bank_treasury, arg)
}

pub(crate) fn bank_transfer_call<A: Serialize, R: DeserializeOwned>(
    arg: A,
) -> Result<R, ModuleError> {
    call!(bank_transfer, arg)
}
pub(crate) fn punish_issue_call<A: Serialize, R: DeserializeOwned>(
    arg: A,
) -> Result<R, ModuleError> {
    call!(punish_issue, arg)
}

pub(crate) fn punish_revoke_call<A: Serialize, R: DeserializeOwned>(
    arg: A,
) -> Result<R, ModuleError> {
    call!(punish_revoke, arg)
}

pub(crate) fn punish_active_call<A: Serialize, R: DeserializeOwned>(
    arg: A,
) -> Result<R, ModuleError> {
    call!(punish_active, arg)
}

pub(crate) fn punish_list_call<A: Serialize, R: DeserializeOwned>(
    arg: A,
) -> Result<R, ModuleError> {
    call!(punish_list, arg)
}
pub(crate) fn chat_tell_call<A: Serialize, R: DeserializeOwned>(arg: A) -> Result<R, ModuleError> {
    call!(chat_tell, arg)
}

pub(crate) fn chat_announce_call<A: Serialize, R: DeserializeOwned>(
    arg: A,
) -> Result<R, ModuleError> {
    call!(chat_announce, arg)
}

pub(crate) fn chat_kick_call<A: Serialize, R: DeserializeOwned>(arg: A) -> Result<R, ModuleError> {
    call!(chat_kick, arg)
}

pub(crate) fn player_ban_call<A: Serialize, R: DeserializeOwned>(arg: A) -> Result<R, ModuleError> {
    call!(player_ban, arg)
}

pub(crate) fn roles_list_call<A: Serialize, R: DeserializeOwned>(arg: A) -> Result<R, ModuleError> {
    call!(roles_list, arg)
}

pub(crate) fn role_get_call<A: Serialize, R: DeserializeOwned>(arg: A) -> Result<R, ModuleError> {
    call!(role_get, arg)
}

pub(crate) fn roles_of_call<A: Serialize, R: DeserializeOwned>(arg: A) -> Result<R, ModuleError> {
    call!(roles_of, arg)
}

pub(crate) fn role_grant_call<A: Serialize, R: DeserializeOwned>(arg: A) -> Result<R, ModuleError> {
    call!(role_grant, arg)
}

pub(crate) fn role_revoke_call<A: Serialize, R: DeserializeOwned>(
    arg: A,
) -> Result<R, ModuleError> {
    call!(role_revoke, arg)
}

pub(crate) fn perm_has_call<A: Serialize, R: DeserializeOwned>(arg: A) -> Result<R, ModuleError> {
    call!(perm_has, arg)
}

pub(crate) fn perm_effective_call<A: Serialize, R: DeserializeOwned>(
    arg: A,
) -> Result<R, ModuleError> {
    call!(perm_effective, arg)
}

pub(crate) fn perm_grant_call<A: Serialize, R: DeserializeOwned>(arg: A) -> Result<R, ModuleError> {
    call!(perm_grant, arg)
}

pub(crate) fn perm_revoke_call<A: Serialize, R: DeserializeOwned>(
    arg: A,
) -> Result<R, ModuleError> {
    call!(perm_revoke, arg)
}

pub(crate) fn access_join_call<A: Serialize, R: DeserializeOwned>(
    arg: A,
) -> Result<R, ModuleError> {
    call!(access_join, arg)
}

pub(crate) fn access_join_revoke_call<A: Serialize, R: DeserializeOwned>(
    arg: A,
) -> Result<R, ModuleError> {
    call!(access_join_revoke, arg)
}

pub(crate) fn access_build_call<A: Serialize, R: DeserializeOwned>(
    arg: A,
) -> Result<R, ModuleError> {
    call!(access_build, arg)
}

pub(crate) fn access_build_revoke_call<A: Serialize, R: DeserializeOwned>(
    arg: A,
) -> Result<R, ModuleError> {
    call!(access_build_revoke, arg)
}

pub(crate) fn servers_list_call<A: Serialize, R: DeserializeOwned>(
    arg: A,
) -> Result<R, ModuleError> {
    call!(servers_list, arg)
}

pub(crate) fn server_get_call<A: Serialize, R: DeserializeOwned>(arg: A) -> Result<R, ModuleError> {
    call!(server_get, arg)
}

pub(crate) fn server_by_slug_call<A: Serialize, R: DeserializeOwned>(
    arg: A,
) -> Result<R, ModuleError> {
    call!(server_by_slug, arg)
}

pub(crate) fn builds_list_call<A: Serialize, R: DeserializeOwned>(
    arg: A,
) -> Result<R, ModuleError> {
    call!(builds_list, arg)
}

pub(crate) fn build_published_call<A: Serialize, R: DeserializeOwned>(
    arg: A,
) -> Result<R, ModuleError> {
    call!(build_published, arg)
}

pub(crate) fn gameservers_list_call<A: Serialize, R: DeserializeOwned>(
    arg: A,
) -> Result<R, ModuleError> {
    call!(gameservers_list, arg)
}

pub(crate) fn gameserver_get_call<A: Serialize, R: DeserializeOwned>(
    arg: A,
) -> Result<R, ModuleError> {
    call!(gameserver_get, arg)
}

pub(crate) fn gameserver_maintenance_call<A: Serialize, R: DeserializeOwned>(
    arg: A,
) -> Result<R, ModuleError> {
    call!(gameserver_maintenance, arg)
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
    ModuleError::new(
        ErrorKind::Invalid,
        format!("the argument was not serialized: {e}"),
    )
}

/// Only breakages of the bridge itself land here: the master could not write
/// the answer into the module's memory, or the export was not found.
fn bridge_failed(e: extism_pdk::Error) -> ModuleError {
    ModuleError::new(
        ErrorKind::Internal,
        format!("the call to the master did not go through: {e}"),
    )
}
