//! The player, and the ways to refer to one.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// How to name a player in an SDK call.
///
/// An account has four natural keys (the internal id, mc_uuid, the username,
/// plus any linked login), and a real module meets all of them: the agent knows
/// only `mc_uuid`, a web form knows the username, a Discord integration knows
/// its own id. One entry point instead of four `by_*` functions removes the
/// choice.
///
/// The variants are structs rather than tuples: on an internally tagged enum
/// (`tag = "by"`) serde requires the content to be an object. With `Id(Uuid)`
/// serialization fails on the very first call with "cannot serialize tagged
/// newtype variant" — and only at runtime.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "by", rename_all = "snake_case")]
pub enum PlayerRef {
    /// The master's internal identifier.
    Id { id: Uuid },
    /// The Minecraft account's UUID.
    McUuid { id: Uuid },
    /// The Minecraft username; case does not matter.
    Name { name: String },
    /// The Discord identifier.
    Discord { id: String },
    /// Any linked login: `("twitch", "12345")`.
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

/// Everything a player can be named by.
///
/// Exists so that `players::get(uuid)` and `players::get("Dalynkaa")` work in
/// the same place. A bare `Uuid` is read as the master's internal id: `mc_uuid`
/// has to be named explicitly, because confusing the two silently is the worst
/// possible outcome.
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

/// A linked login.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Identity {
    pub provider: String,
    pub external_id: String,
    #[serde(default)]
    pub username: Option<String>,
    pub linked_at: DateTime<Utc>,
}

/// The player as a module sees them.
///
/// Deliberately narrower than the internal profile: no tokens, no hashes, no
/// housekeeping flags. Anything beyond this a module requests with a separate
/// call, where capabilities are checked.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub id: Uuid,
    /// The Minecraft username. Empty when the account is not linked to the game yet.
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub mc_uuid: Option<Uuid>,
    #[serde(default)]
    pub discord_id: Option<String>,
    /// Roles by name: comparing strings is handier than carrying identifiers around.
    #[serde(default)]
    pub roles: Vec<String>,
    pub banned: bool,
    pub created_at: DateTime<Utc>,
    /// Whether this is the player's first join. Filled in join events only.
    #[serde(default)]
    pub first_join: bool,
}

impl Player {
    /// The username, or the identifier when there is none. For logs and messages.
    pub fn label(&self) -> String {
        self.name.clone().unwrap_or_else(|| self.id.to_string())
    }

    pub fn has_role(&self, role: &str) -> bool {
        self.roles.iter().any(|r| r == role)
    }
}
