//! Methods on the things you already have in hand.
//!
//! ```ignore
//! let player = players::require("Dalynkaa")?;
//! player.punish().server_ban(server_id, "griefing", Some(7 * 24 * 3600))?;
//! player.roles().grant("vip")?;
//! player.in_game().tell("See you in a week.")?;
//! ```
//!
//! Every method here forwards to the domain function of the same name and does
//! nothing else — same capability, same behaviour, same error. The difference is
//! that you stop carrying an identifier from one call to the next, and stop
//! reading `punish::server_ban(player.id, server_id, …)` where the first
//! argument is the only one that ever varies.
//!
//! # Why handles and not thirty methods
//!
//! `Player` used to carry a flat list: `mute` beside `set_cape`, `can` beside
//! `revoke_sessions`. Nothing was wrong with any single name, but together they
//! read as a heap, and completion offered thirty verbs with no hint of which
//! belonged together. Now each occupation is a handle — `player.punish()`,
//! `player.perms()`, `player.dm()` — and the verbs live inside it.
//!
//! The handles hold nothing but an id. They exist to group, not to cache: two
//! calls through `player.perms()` are the two calls you would have made anyway.
//!
//! # Why traits and not plain methods
//!
//! `Player`, `Server` and the rest are defined in `noro-module-abi`, which knows
//! nothing about host functions — it is the wire contract and is linked by both
//! sides. A method has to live where the call lives, and Rust will not let one
//! crate add inherent methods to another's type. So they arrive as traits, and
//! the prelude brings them in: `use noro_sdk::prelude::*` and the methods are
//! simply there.
//!
//! # Where there is no handle
//!
//! `Build` and `GameServer` have no `store()`. The storage has three scopes —
//! the instance, a server, a player — and neither of those two is one of them.
//! A `store()` on a game server would have to hand back its server's scope, and
//! two game servers of one build would then quietly share a key that reads as
//! if it were theirs.
//!
//! # What is not here
//!
//! Anything that does not begin with a thing you are holding. `players::get`,
//! `servers::list`, `bank::transfer` between two accounts — those start from
//! nothing or from two equals, and a method would have to pick one of them to
//! hang off, which reads as if that one mattered more.

pub mod player;
pub mod server;

use noro_module_abi::entity::{Account, Build, GameServer, Punishment, Role, Server};
use noro_module_abi::error::ModuleError;
use noro_module_abi::ops::DmPresence;
use noro_module_abi::player::Player;
use uuid::Uuid;

/// What you can do to a player you are holding.
pub trait PlayerActions {
    /// Their account: name, skin, cape, closing it. See [`player::Profile`].
    fn profile(&self) -> player::Profile;
    /// Their roles. See [`player::Roles`].
    fn roles(&self) -> player::Roles;
    /// Their permissions. See [`player::Perms`].
    fn perms(&self) -> player::Perms;
    /// Which servers they are let onto. See [`player::Access`].
    fn access(&self) -> player::Access;
    /// Punishments — the kind they read and can appeal. See [`player::Punish`].
    fn punish(&self) -> player::Punish;
    /// Their money on one build. See [`player::Bank`].
    fn bank(&self, server_id: Uuid) -> player::Bank;
    /// Them in game: tell, kick, ask if they are there. See [`player::InGame`].
    fn in_game(&self) -> player::InGame;
    /// Their launcher. See [`player::Launcher`].
    fn launcher(&self) -> player::Launcher;
    /// How they sign in and where they are signed in. See [`player::Logins`].
    fn logins(&self) -> player::Logins;

    /// Their private messages: `player.dm().send_to(other, "…")`.
    /// See [`crate::dm::Conversations`].
    fn dm(&self) -> crate::dm::Conversations;
    /// Where they are: in game, on the site, in the launcher, nowhere.
    /// See [`crate::dm::presence`].
    fn presence(&self) -> Result<DmPresence, ModuleError>;

    /// Their corner of your storage. See [`crate::store::user`].
    ///
    /// ```ignore
    /// e.player.store().incr("joins", 1)?;
    /// ```
    fn store(&self) -> crate::store::Store;
}

impl PlayerActions for Player {
    fn profile(&self) -> player::Profile {
        player::Profile(self.id)
    }
    fn roles(&self) -> player::Roles {
        player::Roles(self.id)
    }
    fn perms(&self) -> player::Perms {
        player::Perms(self.id)
    }
    fn access(&self) -> player::Access {
        player::Access(self.id)
    }
    fn punish(&self) -> player::Punish {
        player::Punish(self.id)
    }
    fn bank(&self, server_id: Uuid) -> player::Bank {
        player::Bank {
            who: self.id,
            server_id,
        }
    }
    fn in_game(&self) -> player::InGame {
        player::InGame(self.id)
    }
    fn launcher(&self) -> player::Launcher {
        player::Launcher(self.id)
    }
    fn logins(&self) -> player::Logins {
        player::Logins(self.id)
    }
    fn dm(&self) -> crate::dm::Conversations {
        crate::dm::of(self.id)
    }
    fn presence(&self) -> Result<DmPresence, ModuleError> {
        crate::dm::presence(self.id)
    }
    fn store(&self) -> crate::store::Store {
        crate::store::user(self.id)
    }
}

/// What you can do with a server you are holding.
pub trait ServerActions {
    /// Its client builds. See [`server::Builds`].
    fn builds(&self) -> server::Builds;
    /// Its game servers, and announcements to them. See [`server::GameServers`].
    fn game_servers(&self) -> server::GameServers;
    /// Its money. See [`server::Treasury`].
    fn treasury(&self) -> server::Treasury;
    /// Its hub: the feed and the members. See [`server::Hub`].
    fn hub(&self) -> server::Hub;
    /// This server's corner of your storage. See [`crate::store::server`].
    fn store(&self) -> crate::store::Store;
}

impl ServerActions for Server {
    fn builds(&self) -> server::Builds {
        server::Builds(self.id)
    }
    fn game_servers(&self) -> server::GameServers {
        server::GameServers(self.id)
    }
    fn treasury(&self) -> server::Treasury {
        server::Treasury(self.id)
    }
    fn hub(&self) -> server::Hub {
        server::Hub(self.id)
    }
    fn store(&self) -> crate::store::Store {
        crate::store::server(self.id)
    }
}

/// What you can do with a build you are holding.
pub trait BuildActions {
    /// What is inside it. See [`server::Files`].
    fn files(&self) -> server::Files;
    /// Who may download it. See [`server::BuildAccess`].
    fn access(&self) -> server::BuildAccess;
    /// Stops the launcher handing it out. See [`crate::builds::unpublish`].
    ///
    /// On its own rather than inside a handle: publication is the build's own
    /// state, not a collection of anything.
    fn unpublish(&self) -> Result<(), ModuleError>;
}

impl BuildActions for Build {
    fn files(&self) -> server::Files {
        server::Files(self.id)
    }
    fn access(&self) -> server::BuildAccess {
        server::BuildAccess(self.id)
    }
    fn unpublish(&self) -> Result<(), ModuleError> {
        crate::builds::unpublish(self.id)
    }
}

/// What you can do with a game server you are holding.
pub trait GameServerActions {
    /// Puts it into maintenance, or takes it out. See [`crate::servers::set_maintenance`].
    fn set_maintenance(&self, enabled: bool) -> Result<(), ModuleError>;
    /// Its last telemetry. See [`crate::telemetry::of`].
    fn telemetry(&self) -> Result<Option<noro_module_abi::ops::Telemetry>, ModuleError>;
}

impl GameServerActions for GameServer {
    fn set_maintenance(&self, enabled: bool) -> Result<(), ModuleError> {
        crate::servers::set_maintenance(self.id, enabled)
    }
    fn telemetry(&self) -> Result<Option<noro_module_abi::ops::Telemetry>, ModuleError> {
        crate::telemetry::of(self.id)
    }
}

/// What you can do with an account you are holding.
pub trait AccountActions {
    /// Moves money to another account. See [`crate::bank::transfer`].
    fn transfer_to(&self, to: &Account, amount: i64, comment: &str) -> Result<i64, ModuleError>;
    /// The same, but a repeat with the same key pays once.
    /// See [`crate::bank::transfer_once`].
    fn transfer_once_to(
        &self,
        to: &Account,
        amount: i64,
        comment: &str,
        key: &str,
    ) -> Result<i64, ModuleError>;
}

impl AccountActions for Account {
    fn transfer_to(&self, to: &Account, amount: i64, comment: &str) -> Result<i64, ModuleError> {
        crate::bank::transfer(self.server_id, self.id, to.id, amount, comment)
    }
    fn transfer_once_to(
        &self,
        to: &Account,
        amount: i64,
        comment: &str,
        key: &str,
    ) -> Result<i64, ModuleError> {
        crate::bank::transfer_once(self.server_id, self.id, to.id, amount, comment, key)
    }
}

/// What you can do with a punishment you are holding.
pub trait PunishmentActions {
    /// Lifts it. See [`crate::punish::revoke`].
    fn lift(&self) -> Result<bool, ModuleError>;
}

impl PunishmentActions for Punishment {
    fn lift(&self) -> Result<bool, ModuleError> {
        crate::punish::revoke(self.id)
    }
}

/// What you can do with a role you are holding.
pub trait RoleActions {
    /// Gives it to a player. See [`crate::roles::grant`].
    fn grant_to(&self, who: impl noro_module_abi::player::IntoPlayerRef)
        -> Result<(), ModuleError>;
    /// Takes it from a player. See [`crate::roles::revoke`].
    fn revoke_from(
        &self,
        who: impl noro_module_abi::player::IntoPlayerRef,
    ) -> Result<(), ModuleError>;
    /// Deletes it. See [`crate::roles::delete`].
    fn delete(&self) -> Result<(), ModuleError>;
}

impl RoleActions for Role {
    fn grant_to(
        &self,
        who: impl noro_module_abi::player::IntoPlayerRef,
    ) -> Result<(), ModuleError> {
        crate::roles::grant(who, self.id)
    }
    fn revoke_from(
        &self,
        who: impl noro_module_abi::player::IntoPlayerRef,
    ) -> Result<(), ModuleError> {
        crate::roles::revoke(who, self.id)
    }
    fn delete(&self) -> Result<(), ModuleError> {
        crate::roles::delete(self.id)
    }
}
