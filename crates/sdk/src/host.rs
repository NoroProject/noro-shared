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
    fn player_rename(input: String) -> String;
    fn player_skin(input: String) -> String;
    fn player_cape(input: String) -> String;
    fn capes_list(input: String) -> String;
    fn presets_list(input: String) -> String;
    fn preset_save(input: String) -> String;
    fn preset_delete(input: String) -> String;
    fn identities_of(input: String) -> String;
    fn identity_link(input: String) -> String;
    fn identity_unlink(input: String) -> String;
    fn build_publish(input: String) -> String;
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
    fn event_emit(input: String) -> String;
    fn hub_feed(input: String) -> String;
    fn hub_members(input: String) -> String;
    fn hub_playtime(input: String) -> String;
    fn hub_post(input: String) -> String;
    fn hub_towns(input: String) -> String;
    fn hub_town(input: String) -> String;
    fn hub_market(input: String) -> String;
    fn hub_lot(input: String) -> String;
    fn hub_court(input: String) -> String;
    fn hub_court_case(input: String) -> String;
    fn hub_petitions(input: String) -> String;
    fn hub_sign(input: String) -> String;
    fn hub_unsign(input: String) -> String;
    fn hub_fines(input: String) -> String;
    fn hub_fines_of(input: String) -> String;
    fn hub_fine(input: String) -> String;
    fn tickets_queue(input: String) -> String;
    fn ticket_get(input: String) -> String;
    fn ticket_messages(input: String) -> String;
    fn ticket_reply(input: String) -> String;
    fn ticket_open(input: String) -> String;
    fn ticket_close(input: String) -> String;
    fn case_get(input: String) -> String;
    fn case_events(input: String) -> String;
    fn case_open_on(input: String) -> String;
    fn case_claim(input: String) -> String;
    fn case_resolve(input: String) -> String;
    fn file_put(input: String) -> String;
    fn file_read(input: String) -> String;
    fn file_exists(input: String) -> String;
    fn file_url(input: String) -> String;
    fn roster_online(input: String) -> String;
    fn roster_where(input: String) -> String;
    fn telemetry_of(input: String) -> String;
    fn role_save(input: String) -> String;
    fn role_delete(input: String) -> String;
    fn optional_grant(input: String) -> String;
    fn optional_revoke(input: String) -> String;
    fn news_list(input: String) -> String;
    fn news_get(input: String) -> String;
    fn news_save(input: String) -> String;
    fn news_delete(input: String) -> String;
    fn instance_all(input: String) -> String;
    fn instance_get(input: String) -> String;
    fn instance_set(input: String) -> String;
    fn sessions_of(input: String) -> String;
    fn session_revoke(input: String) -> String;
    fn sessions_revoke_all(input: String) -> String;
    fn restarts_of(input: String) -> String;
    fn restart_add(input: String) -> String;
    fn restart_remove(input: String) -> String;
    fn http_send(input: String) -> String;
    fn db_query(input: String) -> String;
    fn db_execute(input: String) -> String;
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

pub(crate) fn player_rename_call<A: Serialize, R: DeserializeOwned>(
    arg: A,
) -> Result<R, ModuleError> {
    call!(player_rename, arg)
}

pub(crate) fn player_skin_call<A: Serialize, R: DeserializeOwned>(
    arg: A,
) -> Result<R, ModuleError> {
    call!(player_skin, arg)
}

pub(crate) fn player_cape_call<A: Serialize, R: DeserializeOwned>(
    arg: A,
) -> Result<R, ModuleError> {
    call!(player_cape, arg)
}

pub(crate) fn capes_list_call<A: Serialize, R: DeserializeOwned>(arg: A) -> Result<R, ModuleError> {
    call!(capes_list, arg)
}

pub(crate) fn presets_list_call<A: Serialize, R: DeserializeOwned>(
    arg: A,
) -> Result<R, ModuleError> {
    call!(presets_list, arg)
}

pub(crate) fn preset_save_call<A: Serialize, R: DeserializeOwned>(
    arg: A,
) -> Result<R, ModuleError> {
    call!(preset_save, arg)
}

pub(crate) fn preset_delete_call<A: Serialize, R: DeserializeOwned>(
    arg: A,
) -> Result<R, ModuleError> {
    call!(preset_delete, arg)
}

pub(crate) fn identities_of_call<A: Serialize, R: DeserializeOwned>(
    arg: A,
) -> Result<R, ModuleError> {
    call!(identities_of, arg)
}

pub(crate) fn identity_link_call<A: Serialize, R: DeserializeOwned>(
    arg: A,
) -> Result<R, ModuleError> {
    call!(identity_link, arg)
}

pub(crate) fn identity_unlink_call<A: Serialize, R: DeserializeOwned>(
    arg: A,
) -> Result<R, ModuleError> {
    call!(identity_unlink, arg)
}

pub(crate) fn build_publish_call<A: Serialize, R: DeserializeOwned>(
    arg: A,
) -> Result<R, ModuleError> {
    call!(build_publish, arg)
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
pub(crate) fn event_emit_call<A: Serialize, R: DeserializeOwned>(arg: A) -> Result<R, ModuleError> {
    call!(event_emit, arg)
}

pub(crate) fn hub_feed_call<A: Serialize, R: DeserializeOwned>(arg: A) -> Result<R, ModuleError> {
    call!(hub_feed, arg)
}

pub(crate) fn hub_members_call<A: Serialize, R: DeserializeOwned>(
    arg: A,
) -> Result<R, ModuleError> {
    call!(hub_members, arg)
}

pub(crate) fn hub_playtime_call<A: Serialize, R: DeserializeOwned>(
    arg: A,
) -> Result<R, ModuleError> {
    call!(hub_playtime, arg)
}

pub(crate) fn hub_post_call<A: Serialize, R: DeserializeOwned>(arg: A) -> Result<R, ModuleError> {
    call!(hub_post, arg)
}

pub(crate) fn hub_towns_call<A: Serialize, R: DeserializeOwned>(arg: A) -> Result<R, ModuleError> {
    call!(hub_towns, arg)
}

pub(crate) fn hub_town_call<A: Serialize, R: DeserializeOwned>(arg: A) -> Result<R, ModuleError> {
    call!(hub_town, arg)
}

pub(crate) fn hub_market_call<A: Serialize, R: DeserializeOwned>(arg: A) -> Result<R, ModuleError> {
    call!(hub_market, arg)
}

pub(crate) fn hub_lot_call<A: Serialize, R: DeserializeOwned>(arg: A) -> Result<R, ModuleError> {
    call!(hub_lot, arg)
}

pub(crate) fn hub_court_call<A: Serialize, R: DeserializeOwned>(arg: A) -> Result<R, ModuleError> {
    call!(hub_court, arg)
}

pub(crate) fn hub_court_case_call<A: Serialize, R: DeserializeOwned>(
    arg: A,
) -> Result<R, ModuleError> {
    call!(hub_court_case, arg)
}

pub(crate) fn hub_petitions_call<A: Serialize, R: DeserializeOwned>(
    arg: A,
) -> Result<R, ModuleError> {
    call!(hub_petitions, arg)
}

pub(crate) fn hub_sign_call<A: Serialize, R: DeserializeOwned>(arg: A) -> Result<R, ModuleError> {
    call!(hub_sign, arg)
}

pub(crate) fn hub_unsign_call<A: Serialize, R: DeserializeOwned>(arg: A) -> Result<R, ModuleError> {
    call!(hub_unsign, arg)
}

pub(crate) fn hub_fines_call<A: Serialize, R: DeserializeOwned>(arg: A) -> Result<R, ModuleError> {
    call!(hub_fines, arg)
}

pub(crate) fn hub_fines_of_call<A: Serialize, R: DeserializeOwned>(
    arg: A,
) -> Result<R, ModuleError> {
    call!(hub_fines_of, arg)
}

pub(crate) fn hub_fine_call<A: Serialize, R: DeserializeOwned>(arg: A) -> Result<R, ModuleError> {
    call!(hub_fine, arg)
}
pub(crate) fn tickets_queue_call<A: Serialize, R: DeserializeOwned>(
    arg: A,
) -> Result<R, ModuleError> {
    call!(tickets_queue, arg)
}

pub(crate) fn ticket_get_call<A: Serialize, R: DeserializeOwned>(arg: A) -> Result<R, ModuleError> {
    call!(ticket_get, arg)
}

pub(crate) fn ticket_messages_call<A: Serialize, R: DeserializeOwned>(
    arg: A,
) -> Result<R, ModuleError> {
    call!(ticket_messages, arg)
}

pub(crate) fn ticket_reply_call<A: Serialize, R: DeserializeOwned>(
    arg: A,
) -> Result<R, ModuleError> {
    call!(ticket_reply, arg)
}

pub(crate) fn ticket_open_call<A: Serialize, R: DeserializeOwned>(
    arg: A,
) -> Result<R, ModuleError> {
    call!(ticket_open, arg)
}

pub(crate) fn ticket_close_call<A: Serialize, R: DeserializeOwned>(
    arg: A,
) -> Result<R, ModuleError> {
    call!(ticket_close, arg)
}

pub(crate) fn case_get_call<A: Serialize, R: DeserializeOwned>(arg: A) -> Result<R, ModuleError> {
    call!(case_get, arg)
}

pub(crate) fn case_events_call<A: Serialize, R: DeserializeOwned>(
    arg: A,
) -> Result<R, ModuleError> {
    call!(case_events, arg)
}

pub(crate) fn case_open_on_call<A: Serialize, R: DeserializeOwned>(
    arg: A,
) -> Result<R, ModuleError> {
    call!(case_open_on, arg)
}

pub(crate) fn case_claim_call<A: Serialize, R: DeserializeOwned>(arg: A) -> Result<R, ModuleError> {
    call!(case_claim, arg)
}

pub(crate) fn case_resolve_call<A: Serialize, R: DeserializeOwned>(
    arg: A,
) -> Result<R, ModuleError> {
    call!(case_resolve, arg)
}
pub(crate) fn file_put_call<A: Serialize, R: DeserializeOwned>(arg: A) -> Result<R, ModuleError> {
    call!(file_put, arg)
}

pub(crate) fn file_read_call<A: Serialize, R: DeserializeOwned>(arg: A) -> Result<R, ModuleError> {
    call!(file_read, arg)
}

pub(crate) fn file_exists_call<A: Serialize, R: DeserializeOwned>(
    arg: A,
) -> Result<R, ModuleError> {
    call!(file_exists, arg)
}

pub(crate) fn file_url_call<A: Serialize, R: DeserializeOwned>(arg: A) -> Result<R, ModuleError> {
    call!(file_url, arg)
}

pub(crate) fn roster_online_call<A: Serialize, R: DeserializeOwned>(
    arg: A,
) -> Result<R, ModuleError> {
    call!(roster_online, arg)
}

pub(crate) fn roster_where_call<A: Serialize, R: DeserializeOwned>(
    arg: A,
) -> Result<R, ModuleError> {
    call!(roster_where, arg)
}

pub(crate) fn telemetry_of_call<A: Serialize, R: DeserializeOwned>(
    arg: A,
) -> Result<R, ModuleError> {
    call!(telemetry_of, arg)
}

pub(crate) fn role_save_call<A: Serialize, R: DeserializeOwned>(arg: A) -> Result<R, ModuleError> {
    call!(role_save, arg)
}

pub(crate) fn role_delete_call<A: Serialize, R: DeserializeOwned>(
    arg: A,
) -> Result<R, ModuleError> {
    call!(role_delete, arg)
}

pub(crate) fn optional_grant_call<A: Serialize, R: DeserializeOwned>(
    arg: A,
) -> Result<R, ModuleError> {
    call!(optional_grant, arg)
}

pub(crate) fn optional_revoke_call<A: Serialize, R: DeserializeOwned>(
    arg: A,
) -> Result<R, ModuleError> {
    call!(optional_revoke, arg)
}
pub(crate) fn news_list_call<A: Serialize, R: DeserializeOwned>(arg: A) -> Result<R, ModuleError> {
    call!(news_list, arg)
}

pub(crate) fn news_get_call<A: Serialize, R: DeserializeOwned>(arg: A) -> Result<R, ModuleError> {
    call!(news_get, arg)
}

pub(crate) fn news_save_call<A: Serialize, R: DeserializeOwned>(arg: A) -> Result<R, ModuleError> {
    call!(news_save, arg)
}

pub(crate) fn news_delete_call<A: Serialize, R: DeserializeOwned>(
    arg: A,
) -> Result<R, ModuleError> {
    call!(news_delete, arg)
}

pub(crate) fn instance_all_call<A: Serialize, R: DeserializeOwned>(
    arg: A,
) -> Result<R, ModuleError> {
    call!(instance_all, arg)
}

pub(crate) fn instance_get_call<A: Serialize, R: DeserializeOwned>(
    arg: A,
) -> Result<R, ModuleError> {
    call!(instance_get, arg)
}

pub(crate) fn instance_set_call<A: Serialize, R: DeserializeOwned>(
    arg: A,
) -> Result<R, ModuleError> {
    call!(instance_set, arg)
}

pub(crate) fn sessions_of_call<A: Serialize, R: DeserializeOwned>(
    arg: A,
) -> Result<R, ModuleError> {
    call!(sessions_of, arg)
}

pub(crate) fn session_revoke_call<A: Serialize, R: DeserializeOwned>(
    arg: A,
) -> Result<R, ModuleError> {
    call!(session_revoke, arg)
}

pub(crate) fn sessions_revoke_all_call<A: Serialize, R: DeserializeOwned>(
    arg: A,
) -> Result<R, ModuleError> {
    call!(sessions_revoke_all, arg)
}

pub(crate) fn restarts_of_call<A: Serialize, R: DeserializeOwned>(
    arg: A,
) -> Result<R, ModuleError> {
    call!(restarts_of, arg)
}

pub(crate) fn restart_add_call<A: Serialize, R: DeserializeOwned>(
    arg: A,
) -> Result<R, ModuleError> {
    call!(restart_add, arg)
}

pub(crate) fn restart_remove_call<A: Serialize, R: DeserializeOwned>(
    arg: A,
) -> Result<R, ModuleError> {
    call!(restart_remove, arg)
}
pub(crate) fn http_send_call<A: Serialize, R: DeserializeOwned>(arg: A) -> Result<R, ModuleError> {
    call!(http_send, arg)
}

pub(crate) fn db_query_call<A: Serialize, R: DeserializeOwned>(arg: A) -> Result<R, ModuleError> {
    call!(db_query, arg)
}

pub(crate) fn db_execute_call<A: Serialize, R: DeserializeOwned>(arg: A) -> Result<R, ModuleError> {
    call!(db_execute, arg)
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
