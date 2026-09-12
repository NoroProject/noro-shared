//! Writing to the master's log.
//!
//! Lines go into the shared `tracing` tagged with which module wrote them — so
//! they show up where the rest of the instance's logs are, and they reach
//! Sentry.

/// An ordinary message.
pub fn info(message: impl AsRef<str>) {
    crate::host::log_line("info", message.as_ref());
}

/// Something went wrong, but the module keeps working.
pub fn warn(message: impl AsRef<str>) {
    crate::host::log_line("warn", message.as_ref());
}

/// A failure.
pub fn error(message: impl AsRef<str>) {
    crate::host::log_line("error", message.as_ref());
}

/// Debugging detail. Not shown in the master's usual configuration.
pub fn debug(message: impl AsRef<str>) {
    crate::host::log_line("debug", message.as_ref());
}
