//! Игрок и способы на него сослаться.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Как назвать игрока в вызове SDK.
///
/// У аккаунта четыре естественных ключа (внутренний id, mc_uuid, ник, плюс
/// любая привязка входа), и в реальном модуле встречаются все: агент знает
/// только `mc_uuid`, веб-форма — ник, интеграция с Discord — свой id. Один
/// вход вместо четырёх функций `by_*` избавляет от выбора.
/// Варианты — структуры, а не кортежи: у внутренне помеченного перечисления
/// (`tag = "by"`) serde требует, чтобы содержимое было объектом. С
/// `Id(Uuid)` сериализация падает на первом же вызове с «cannot serialize
/// tagged newtype variant», причём только в рантайме.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "by", rename_all = "snake_case")]
pub enum PlayerRef {
    /// Внутренний идентификатор мастера.
    Id { id: Uuid },
    /// UUID аккаунта Minecraft.
    McUuid { id: Uuid },
    /// Ник Minecraft, регистр не важен.
    Name { name: String },
    /// Идентификатор в Discord.
    Discord { id: String },
    /// Любая привязка входа: `("twitch", "12345")`.
    Identity { provider: String, id: String },
}

impl PlayerRef {
    pub fn id(id: Uuid) -> Self {
        PlayerRef::Id { id }
    }

    pub fn mc_uuid(id: Uuid) -> Self {
        PlayerRef::McUuid { id }
    }

    pub fn name(name: impl Into<String>) -> Self {
        PlayerRef::Name { name: name.into() }
    }

    pub fn discord(id: impl Into<String>) -> Self {
        PlayerRef::Discord { id: id.into() }
    }

    pub fn identity(provider: impl Into<String>, id: impl Into<String>) -> Self {
        PlayerRef::Identity {
            provider: provider.into(),
            id: id.into(),
        }
    }
}

/// Всё, чем можно назвать игрока.
///
/// Существует ради `players::get(uuid)` и `players::get("Dalynkaa")` в одном и
/// том же месте. Голый `Uuid` трактуется как внутренний id мастера: `mc_uuid`
/// приходится называть явно, потому что перепутать их молча — худший из
/// возможных исходов.
pub trait IntoPlayerRef {
    fn into_player_ref(self) -> PlayerRef;
}

impl IntoPlayerRef for PlayerRef {
    fn into_player_ref(self) -> PlayerRef {
        self
    }
}

impl IntoPlayerRef for Uuid {
    fn into_player_ref(self) -> PlayerRef {
        PlayerRef::Id { id: self }
    }
}

impl IntoPlayerRef for &str {
    fn into_player_ref(self) -> PlayerRef {
        PlayerRef::Name {
            name: self.to_string(),
        }
    }
}

impl IntoPlayerRef for String {
    fn into_player_ref(self) -> PlayerRef {
        PlayerRef::Name { name: self }
    }
}

impl IntoPlayerRef for &Player {
    fn into_player_ref(self) -> PlayerRef {
        PlayerRef::Id { id: self.id }
    }
}

/// Привязка входа.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Identity {
    pub provider: String,
    pub external_id: String,
    #[serde(default)]
    pub username: Option<String>,
    pub linked_at: DateTime<Utc>,
}

/// Игрок в том виде, в каком его видит модуль.
///
/// Намеренно уже внутреннего профиля: ни токенов, ни хэшей, ни служебных
/// флагов. Всё, что сверх этого, модуль запрашивает отдельным вызовом с
/// проверкой возможностей.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub id: Uuid,
    /// Ник Minecraft. Пусто, если аккаунт ещё не привязан к игре.
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub mc_uuid: Option<Uuid>,
    #[serde(default)]
    pub discord_id: Option<String>,
    /// Роли по именам: сравнивать строки удобнее, чем таскать идентификаторы.
    #[serde(default)]
    pub roles: Vec<String>,
    pub banned: bool,
    pub created_at: DateTime<Utc>,
    /// Первый ли это вход игрока. Заполняется только в событиях входа.
    #[serde(default)]
    pub first_join: bool,
}

impl Player {
    /// Ник, а если его нет — идентификатор. Для логов и сообщений.
    pub fn label(&self) -> String {
        self.name.clone().unwrap_or_else(|| self.id.to_string())
    }

    pub fn has_role(&self, role: &str) -> bool {
        self.roles.iter().any(|r| r == role)
    }
}
