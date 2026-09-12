//! The boundary between the master and a module.
//!
//! The crate is public and added by the module author directly, so it holds
//! only what has to survive a master upgrade: entities, events, the manifest.
//! Not one dependency on tokio, axum or sqlx — otherwise a module could not be
//! built for `wasm32-unknown-unknown`.
//!
//! # Why these types duplicate the master's internal models
//!
//! Re-exporting `schema::UserProfile` is tempting, but then the master's
//! internal model becomes a public contract: renaming a field breaks other
//! people's modules, and removing a field is never possible again. What lives
//! here is deliberately a separate, narrower set — the master projects its own
//! rows into it on the way out.
//!
//! The price is known: a new field in `schema` does not appear here by itself.
//! Hence the rule — when adding a field to the user profile, decide in the same
//! commit whether it travels into [`Player`].
//!
//! # Compatibility
//!
//! The crate's version **is** the ABI version. A module declares the version it
//! requires in its manifest (`api = "1.0"`); the master compares them at
//! install time and refuses when the majors do not match. Every struct arriving
//! from the master carries `#[serde(default)]` on new fields: a module built
//! against an older ABI has to keep working.

pub mod context;
pub mod entity;
pub mod error;
pub mod events;
pub mod http;
pub mod http_out;
pub mod manifest;
pub mod ops;
pub mod player;
pub mod registration;
pub mod sql;
pub mod store;
pub mod validate;

pub use context::{ActorRef, EventCtx, Origin};
pub use entity::{Account, Build, GameServer, Punishment, Role, Server};
pub use error::{ModuleError, ModuleResult};
pub use events::{Cancel, Event, EventKind, EventMeta, ALL_EVENTS};
pub use http::HttpRequest;
pub use http_out::{HttpCall, HttpReply};
pub use manifest::Manifest;
pub use ops::{
    AccountQuery, Announcement, BanRequest, BuildAccess, Cape, CapeRequest, FileWrite, Identity,
    IntoRoleRef, LinkRequest, Maintenance, NewsDraft, NewsItem, OnlinePlayer, OptionalMod,
    PermissionOn, PermissionQuery, PlayerMessage, PresetRef, ProviderQuery, PublishRequest,
    PunishKind, PunishRequest, RenameRequest, RestartSchedule, RoleDraft, RoleGrant, RoleRef,
    SavePreset, ScheduleDraft, ServerAccess, Session, SessionRef, SettingWrite, SkinPreset,
    SkinRequest, StoredFile, Telemetry, Transfer,
};
pub use player::{IntoPlayerRef, Player, PlayerRef};
pub use registration::{EventReg, Registration, RouteReg, TaskReg};
pub use sql::{Query, Rows};

/// The ABI version this crate understands.
///
/// The major changes only on a breaking change to the boundary: the master will
/// refuse to install a module built against a different major and will say so
/// plainly, instead of failing on the first call with a mismatched struct.
pub const ABI_VERSION: &str = "1.0";
