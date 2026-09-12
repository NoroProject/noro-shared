//! SDK for writing Noro modules.
//!
//! A module is compiled for `wasm32-unknown-unknown` and installed into the
//! master as a single file. Inside the master it runs sandboxed, but the calls
//! from here are not network calls: every function in this SDK lands in the
//! same code that serves the admin panel.
//!
//! # What a module looks like
//!
//! ```ignore
//! use noro_sdk::prelude::*;
//!
//! pub struct Greeter;
//!
//! #[noro::module]
//! impl Greeter {
//!     /// Greets anyone arriving for the first time.
//!     #[event]
//!     fn on_join(e: PlayerJoined) -> Result<()> {
//!         if e.first_join {
//!             log::info(format!("{} is new to {}", e.player.label(), e.ctx.server_name()));
//!         }
//!         Ok(())
//!     }
//!
//!     /// Your own endpoint: `GET /api/modules/<id>/me`.
//!     #[route(GET, "/me")]
//!     fn me(req: HttpRequest) -> Result<u64> {
//!         Ok(store::user(req.require_user()?).get("points")?.unwrap_or(0))
//!     }
//! }
//! ```
//!
//! None of this is repeated in the manifest. The master derives the event name
//! from the argument type, so subscribing to one event while accepting another
//! event's struct is not possible — it will not compile. The manifest is left
//! with only what has to be known **before** any code runs: the identifier,
//! the version, the required ABI and the capabilities being asked for.
//!
//! # Failures
//!
//! Every call can return a [`ModuleError`](abi::error::ModuleError). The most common reason is a
//! capability the operator did not grant the module at install time: that
//! arrives as `CapabilityDenied`, naming what is missing.
//!
//! # Third-party libraries
//!
//! They are added with a plain `cargo add` and compiled straight into the
//! `.wasm`. Nothing like Paper's shading, relocation or dependency declarations
//! in the manifest is needed: every module has its own `.wasm` with its own
//! copy, so two modules carrying different versions of the same library cannot
//! conflict in the first place. The package ends up self-contained and does not
//! depend on somebody's repository being reachable at install time.
//!
//! There is one restriction: the library has to build for
//! `wasm32-unknown-unknown`. Pure computation works as is — `regex`,
//! `rust_decimal`, `serde`, `sha2`, `base64`. Anything reaching into the
//! environment does not work there, and cannot:
//!
//! | What the library needs | Why it will not work | What to use instead |
//! |---|---|---|
//! | Network (`reqwest`, `tokio`) | the sandbox has no sockets | the HTTP host function, with hosts allow-listed in the manifest |
//! | System clock (`chrono` with `clock`) | wasm has no clock; you get the epoch | [`now()`] |
//! | Randomness (`uuid/v4`, `rand`) | there is no entropy source | identifiers come from the master |
//! | Files, threads, processes | the sandbox does not provide them | [`store`] and your own Postgres schema |
//!
//! Dependencies with those features usually just need to be added with
//! `default-features = false` — that is exactly what this SDK does with `uuid`
//! and `chrono`.
//!
//! The second thing worth keeping in mind is size: everything you add travels
//! inside the `.wasm` and occupies the instance's memory.

pub mod access;
pub mod bank;
pub mod cases;
pub mod chat;
pub mod db;
pub mod files;
pub mod host;
pub mod http;
pub mod hub;
pub mod identities;
pub mod instance;
pub mod log;
pub mod news;
pub mod permissions;
pub mod players;
pub mod punish;
pub mod restarts;
pub mod roles;
pub mod roster;
pub mod servers;
pub mod sessions;
pub mod store;
pub mod telemetry;
pub mod tickets;

pub use noro_module_abi as abi;

/// The sandbox layer. Re-exported because `#[plugin_fn]` expands into paths of
/// the form `extism_pdk::…` — without that name in scope the macro does not
/// compile, and every module would have to add extism-pdk as a separate
/// dependency and know about it.
pub use extism_pdk;

/// The same serde_json the macros use: they expand into `noro_sdk::serde_json::…`.
pub use serde_json;

/// Handler attributes: `#[noro::module]`, `#[event]`, `#[route]`, `#[task]`.
pub use noro_sdk_macros as noro;

/// How a call ended. There is one error for the whole SDK — [`abi::error::ModuleError`].
pub type Result<T> = core::result::Result<T, abi::error::ModuleError>;

/// Everything an ordinary module needs, in one `use`.
pub mod prelude {
    pub use crate::abi::error::{ErrorKind, ModuleError};
    pub use crate::abi::events::*;
    pub use crate::abi::http_out::{HttpCall, HttpReply};
    pub use crate::abi::manifest::{Priority, SettingKind};
    pub use crate::abi::ops::{
        FineDraft, IntoRoleRef, PunishKind, RoleDraft, RoleRef, ScheduleDraft,
    };
    pub use crate::abi::sql::Query;
    pub use crate::abi::HttpRequest;
    pub use crate::abi::Registration;
    pub use crate::abi::{
        Account, ActorRef, Build, EventCtx, GameServer, IntoPlayerRef, Origin, Player, PlayerRef,
        Punishment, Role, Server,
    };
    pub use crate::{
        access, bank, chat, log, noro, now, permissions, players, punish, roles, servers, store,
        Result,
    };

    // The crate itself, not just its names: the `plugin_fn` macro expands into
    // `extism_pdk::…`, and without this import the module would not build.
    pub use crate::extism_pdk;
    pub use extism_pdk::{plugin_fn, FnResult, Json};
    pub use serde::{Deserialize, Serialize};
    pub use uuid::Uuid;
}

/// The master's current time.
///
/// The clock comes from the host: wasm has none of its own, and
/// `SystemTime::now()` on this target either fails to build or returns the
/// epoch.
pub fn now() -> chrono::DateTime<chrono::Utc> {
    chrono::DateTime::from_timestamp(host::now_secs(), 0).unwrap_or_default()
}
