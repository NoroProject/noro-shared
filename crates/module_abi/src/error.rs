//! Ошибки на границе.

use serde::{Deserialize, Serialize};

pub type ModuleResult<T> = Result<T, ModuleError>;

/// Почему вызов не удался.
///
/// Мастер возвращает это модулю, и модуль возвращает это мастеру: одна форма в
/// обе стороны, чтобы автору не приходилось держать в голове два набора.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleError {
    pub kind: ErrorKind,
    pub message: String,
    /// Код ошибки мастера (`1500`–`1599` и общие), если ошибка пришла оттуда.
    #[serde(default)]
    pub code: Option<i32>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ErrorKind {
    /// Модулю не выдана возможность, которую он просит.
    CapabilityDenied,
    /// Объект не найден.
    NotFound,
    /// Аргументы не прошли проверку.
    Invalid,
    /// Состояние не позволяет: недостаточно средств, дубликат, конфликт.
    Conflict,
    /// Превышена квота модуля.
    Quota,
    /// Сбой внутри мастера.
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
            format!("возможность «{capability}» не выдана модулю"),
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
