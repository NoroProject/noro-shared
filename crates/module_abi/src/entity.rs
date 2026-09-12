//! Сущности мастера в проекции для модуля.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Сборка. В интерфейсе она же «сервер», но в данных это `servers`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Server {
    pub id: Uuid,
    pub name: String,
    /// Слаг подсайта. Пусто, если подсайт не заведён.
    #[serde(default)]
    pub slug: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
}

/// Игровой сервер: процесс, к которому подключается агент.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameServer {
    pub id: Uuid,
    pub server_id: Uuid,
    pub name: String,
    /// `proxy` или `server`.
    pub kind: String,
    pub online: bool,
    #[serde(default)]
    pub players_online: i32,
    #[serde(default)]
    pub max_players: Option<i32>,
    #[serde(default)]
    pub version: Option<String>,
}

/// Сборка клиента: то, что лаунчер скачивает игроку.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Build {
    pub id: Uuid,
    pub server_id: Uuid,
    pub name: String,
    pub published: bool,
    pub created_at: DateTime<Utc>,
    #[serde(default)]
    pub minecraft_version: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Role {
    pub id: Uuid,
    /// Машинное имя: `vip`, `moderator`.
    pub name: String,
    pub display_name: String,
    #[serde(default)]
    pub color: Option<String>,
    pub is_default: bool,
    /// Сборка, которой принадлежит роль. `None` — роль действует везде.
    #[serde(default)]
    pub server_id: Option<Uuid>,
}

/// Счёт в банке подсайта.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub id: Uuid,
    pub server_id: Uuid,
    /// Владелец. `None` у служебных счетов: казна, налоговый, штрафной.
    #[serde(default)]
    pub owner_id: Option<Uuid>,
    /// Машинный код служебного счёта, если он служебный.
    #[serde(default)]
    pub code: Option<String>,
    pub balance: i64,
    #[serde(default)]
    pub frozen: bool,
}

/// Наказание.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Punishment {
    pub id: Uuid,
    pub user_id: Uuid,
    /// `ban`, `mute`, `warn`, `kick`.
    pub kind: String,
    pub reason: String,
    pub issued_at: DateTime<Utc>,
    /// Когда истекает. `None` — навсегда.
    #[serde(default)]
    pub expires_at: Option<DateTime<Utc>>,
    #[serde(default)]
    pub revoked: bool,
    /// Сборка, на которой действует. `None` — на всех.
    #[serde(default)]
    pub server_id: Option<Uuid>,
}

impl Punishment {
    /// Действует ли наказание в указанный момент.
    ///
    /// Время передаётся, а не берётся из системных часов: у wasm их нет, и
    /// `Utc::now()` в модуле вернул бы начало эпохи. Текущее время даёт
    /// `noro_sdk::now()` — оно приходит от мастера.
    pub fn active_at(&self, now: DateTime<Utc>) -> bool {
        !self.revoked && self.expires_at.is_none_or(|e| e > now)
    }
}
