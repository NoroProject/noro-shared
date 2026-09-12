//! Errors on the boundary.

use serde::{Deserialize, Serialize};

pub type ModuleResult<T> = Result<T, ModuleError>;

/// Why a call failed.
///
/// The master returns this to a module, and a module returns this to the
/// master: one shape in both directions, so the author never has to keep two
/// sets in mind.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleError {
    pub kind: ErrorKind,
    pub message: String,
    /// The master's error code (`1500`–`1599` and the shared ones), when the error came from there.
    #[serde(default)]
    pub code: Option<i32>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ErrorKind {
    /// The module was not granted a capability it asks for.
    CapabilityDenied,
    /// The object was not found.
    NotFound,
    /// The arguments failed validation.
    Invalid,
    /// The state does not allow it: insufficient funds, a duplicate, a conflict.
    Conflict,
    /// The module's quota is exceeded.
    Quota,
    /// A failure inside the master.
    Internal,
}

impl ModuleError {
    pub fn new(kind: ErrorKind, message: impl Into<String>) -> Self {
        Self {
            kind,
            message: message.into(),
            code: None,
        }
    }

    pub fn not_found(what: impl Into<String>) -> Self {
        Self::new(ErrorKind::NotFound, what)
    }

    pub fn invalid(why: impl Into<String>) -> Self {
        Self::new(ErrorKind::Invalid, why)
    }

    pub fn conflict(why: impl Into<String>) -> Self {
        Self::new(ErrorKind::Conflict, why)
    }

    pub fn denied(capability: impl std::fmt::Display) -> Self {
        Self::new(
            ErrorKind::CapabilityDenied,
            format!("the capability `{capability}` was not granted to the module"),
        )
    }

    pub fn with_code(mut self, code: i32) -> Self {
        self.code = Some(code);
        self
    }
}

impl std::fmt::Display for ModuleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for ModuleError {}
