//! The event's context: where it happened and who caused it.
//!
//! Without context an event is useless on an instance with several servers: the
//! module sees "a player joined" but not what they joined, and has to guess
//! from the event's own data. That is why [`EventCtx`] travels with every
//! event, not only with the ones where the server seems to matter.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Where the action that produced the event came from.
///
/// Telling the source apart comes up constantly: a module handing out a reward
/// for logging in must not fire on a routine profile update from the CLI, while
/// an anti-fraud module is interested in web actions and nothing else.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Origin {
    /// A game server, through the agent.
    Game,
    /// The site, the cabinet or the admin panel.
    Web,
    /// The desktop launcher.
    Launcher,
    /// `noro-admin` or an admin token.
    Cli,
    /// Another module. Lets you avoid reacting to your own echo.
    Module { id: String },
    /// The master itself: the scheduler, a migration, a punishment expiring.
    System,
}

impl std::fmt::Display for Origin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Origin::Game => f.write_str("game"),
            Origin::Web => f.write_str("web"),
            Origin::Launcher => f.write_str("launcher"),
            Origin::Cli => f.write_str("cli"),
            Origin::Module { id } => write!(f, "module:{id}"),
            Origin::System => f.write_str("system"),
        }
    }
}

/// Who initiated the action.
///
/// Separate from [`Origin`]: the origin answers "through what", the actor
/// answers "who". A ban can arrive from the web by a moderator's hand and from
/// the CLI by that same moderator.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum ActorRef {
    User { id: Uuid, username: String },
    Module { id: String },
    Token { name: String },
    System,
}

impl ActorRef {
    /// The user's identifier, when a person performed the action.
    pub fn user_id(&self) -> Option<Uuid> {
        match self {
            ActorRef::User { id, .. } => Some(*id),
            _ => None,
        }
    }
}

/// The circumstances of an event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventCtx {
    pub origin: Origin,
    pub actor: ActorRef,
    /// The server the event belongs to. `None` on global ones: registering a
    /// user is not tied to any server.
    #[serde(default)]
    pub server_id: Option<Uuid>,
    /// The server's slug — so you need no extra call just for a log line.
    #[serde(default)]
    pub server_slug: Option<String>,
    /// The specific game server, when the event came from the game.
    #[serde(default)]
    pub game_server_id: Option<Uuid>,
    pub at: DateTime<Utc>,
    /// How deeply publications are nested.
    ///
    /// A module's action produces new events, those wake other modules, and
    /// without a counter this closes into a ring. The master stops publishing
    /// once the depth hits the ceiling.
    #[serde(default)]
    pub depth: u8,
}

impl EventCtx {
    /// Whether the event came from the game.
    pub fn from_game(&self) -> bool {
        matches!(self.origin, Origin::Game)
    }

    /// Whether the module with this identifier produced the event itself.
    ///
    /// A mandatory check in handlers that change the same data they watch:
    /// otherwise the module reacts to its own write and spins.
    pub fn caused_by(&self, module_id: &str) -> bool {
        matches!(&self.origin, Origin::Module { id } if id == module_id)
    }

    /// A label for logs: the slug, else the identifier, else "global".
    pub fn server_name(&self) -> String {
        self.server_slug
            .clone()
            .or_else(|| self.server_id.map(|id| id.to_string()))
            .unwrap_or_else(|| "global".to_string())
    }
}
