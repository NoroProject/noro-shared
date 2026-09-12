//! Запись в лог мастера.
//!
//! Строки уходят в общий `tracing` с пометкой, какой модуль их написал, — то
//! есть видны там же, где остальные логи инстанса, и попадают в Sentry.

/// Обычное сообщение.
pub fn info(message: impl AsRef<str>) {
    crate::host::log_line("info", message.as_ref());
}

/// Что-то пошло не так, но модуль продолжает работать.
pub fn warn(message: impl AsRef<str>) {
    crate::host::log_line("warn", message.as_ref());
}

/// Сбой.
pub fn error(message: impl AsRef<str>) {
    crate::host::log_line("error", message.as_ref());
}

/// Подробности для отладки. В обычной конфигурации мастера не показываются.
pub fn debug(message: impl AsRef<str>) {
    crate::host::log_line("debug", message.as_ref());
}
