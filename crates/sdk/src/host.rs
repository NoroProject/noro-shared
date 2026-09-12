//! Мост к мастеру.
//!
//! Каждая функция отсюда — вызов внутрь процесса мастера, а не по сети. Через
//! границу wasm едет JSON, потому что иначе через неё ничего и не едет: это
//! свойство песочницы, а не выбор архитектуры. Стоит это единицы микросекунд.

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

/// Общая обёртка: сериализовать вход, позвать мастер, развернуть ответ.
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

/// Часы мастера в секундах эпохи.
pub fn now_secs() -> i64 {
    let raw = match unsafe { host_now(String::from("null")) } {
        Ok(v) => v,
        Err(_) => return 0,
    };
    decode::<i64>(&raw).unwrap_or(0)
}

/// Пишет строку в лог мастера. Ошибки моста здесь глотаются намеренно: падать
/// из-за неудачной записи в лог — худшее, что может сделать модуль.
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

/// Мастер отвечает `Result`, сериализованным serde: `{"Ok":…}` либо `{"Err":…}`.
fn decode<R: DeserializeOwned>(raw: &str) -> Result<R, ModuleError> {
    match serde_json::from_str::<Result<R, ModuleError>>(raw) {
        Ok(v) => v,
        Err(e) => Err(ModuleError::new(
            ErrorKind::Internal,
            format!("ответ мастера не разобран: {e}"),
        )),
    }
}

fn encode_failed(e: serde_json::Error) -> ModuleError {
    ModuleError::new(ErrorKind::Invalid, format!("аргумент не упакован: {e}"))
}

/// Сюда попадают только поломки самого моста: мастер не смог записать ответ в
/// память модуля или экспорт не найден.
fn bridge_failed(e: extism_pdk::Error) -> ModuleError {
    ModuleError::new(
        ErrorKind::Internal,
        format!("вызов к мастеру не прошёл: {e}"),
    )
}
