//! Methods on the things you already have in hand.
//!
//! ```ignore
//! let player = players::require("Dalynkaa")?;
//! player.server_ban(server_id, "griefing", Some(7 * 24 * 3600))?;
//! player.grant_role("vip")?;
//! player.tell("See you in a week.")?;
//! ```
//!
//! Every method here forwards to the domain function of the same name and does
//! nothing else — same capability, same behaviour, same error. The difference is
//! that you stop carrying an identifier from one call to the next, and stop
//! reading `punish::server_ban(player.id, server_id, …)` where the first
//! argument is the only one that ever varies.
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
//! # Where there is no method
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

use noro_module_abi::entity::{Account, Build, BuildFile, GameServer, Punishment, Role, Server};
use noro_module_abi::error::ModuleError;
use noro_module_abi::ops::{Identity, IntoRoleRef, Session, SkinPreset};
use noro_module_abi::player::Player;
use uuid::Uuid;

/// What you can do to a player you are holding.
pub trait PlayerActions {
    /// Bans the account. See [`crate::players::ban`].
    fn ban(&self, reason: Option<&str>) -> Result<(), ModuleError>;
    /// Lifts the account ban. See [`crate::players::unban`].
    fn unban(&self) -> Result<(), ModuleError>;
    /// Renames them. Refused if the name is taken. See [`crate::players::rename`].
    fn rename(&self, username: &str) -> Result<(), ModuleError>;
    /// Sets the skin; `slim` travels with it. See [`crate::players::set_skin`].
    fn set_skin(&self, url: Option<&str>, slim: bool) -> Result<(), ModuleError>;
    /// Puts a cape on, or takes it off with `None`. See [`crate::players::set_cape`].
    fn set_cape(&self, cape_id: Option<Uuid>) -> Result<(), ModuleError>;
    /// Their saved skin presets. See [`crate::players::presets`].
    fn presets(&self) -> Result<Vec<SkinPreset>, ModuleError>;

    /// Their roles. See [`crate::roles::of`].
    fn roles(&self) -> Result<Vec<Role>, ModuleError>;
    /// Grants a role by name or id. See [`crate::roles::grant`].
    fn grant_role(&self, role: impl IntoRoleRef) -> Result<(), ModuleError>;
    /// Takes a role away. See [`crate::roles::revoke`].
    fn revoke_role(&self, role: impl IntoRoleRef) -> Result<(), ModuleError>;

    /// Whether they effectively hold a permission. See [`crate::permissions::has`].
    fn can(&self, node: &str) -> Result<bool, ModuleError>;
    /// The same on one server build. See [`crate::permissions::has_on`].
    fn can_on(&self, node: &str, server_id: Uuid) -> Result<bool, ModuleError>;
    /// Grants a personal permission. See [`crate::permissions::grant`].
    fn grant(&self, node: &str) -> Result<(), ModuleError>;
    /// Takes a personal permission away. See [`crate::permissions::revoke`].
    fn revoke(&self, node: &str) -> Result<(), ModuleError>;

    /// Lets them into a server build. See [`crate::access::allow_join`].
    fn allow_join(&self, server_id: Uuid) -> Result<(), ModuleError>;
    /// Takes that away. See [`crate::access::revoke_join`].
    fn revoke_join(&self, server_id: Uuid) -> Result<(), ModuleError>;

    /// Bans them with a punishment — the kind the player reads and can appeal.
    /// See [`crate::punish::ban`].
    fn punish_ban(&self, reason: &str, seconds: Option<i64>) -> Result<Punishment, ModuleError>;
    /// Mutes them. See [`crate::punish::mute`].
    fn mute(&self, reason: &str, seconds: Option<i64>) -> Result<Punishment, ModuleError>;
    /// Warns them. See [`crate::punish::warn`].
    fn warn(&self, reason: &str) -> Result<Punishment, ModuleError>;
    /// Bans them from one server only. See [`crate::punish::server_ban`].
    fn server_ban(
        &self,
        server_id: Uuid,
        reason: &str,
        seconds: Option<i64>,
    ) -> Result<Punishment, ModuleError>;
    /// Their punishments in force. See [`crate::punish::active`].
    fn punishments(&self) -> Result<Vec<Punishment>, ModuleError>;

    /// Their account on a server. See [`crate::bank::account`].
    fn account(&self, server_id: Uuid) -> Result<Option<Account>, ModuleError>;
    /// What they have, in the smallest unit. See [`crate::bank::balance`].
    fn balance(&self, server_id: Uuid) -> Result<i64, ModuleError>;

    /// A private message. `false` — they are not in game. See [`crate::agent::tell`].
    fn tell(&self, message: &str) -> Result<bool, ModuleError>;
    /// Throws them off the server. See [`crate::agent::kick`].
    fn kick(&self, reason: &str) -> Result<bool, ModuleError>;
    /// Whether they are in game right now. See [`crate::roster::is_online`].
    fn is_online(&self) -> Result<bool, ModuleError>;
    /// Whether their launcher is connected. See [`crate::launcher::is_online`].
    fn launcher_online(&self) -> Result<bool, ModuleError>;
    /// Sends their launcher a frame. See [`crate::launcher::send`].
    fn send(&self, payload: impl serde::Serialize) -> Result<bool, ModuleError>;

    /// Their corner of your storage. See [`crate::store::user`].
    ///
    /// ```ignore
    /// e.player.store().incr("joins", 1)?;
    /// ```
    fn store(&self) -> crate::store::Store;

    /// Their linked logins. See [`crate::identities::of`].
    fn identities(&self) -> Result<Vec<Identity>, ModuleError>;
    /// Their sessions in the panel and the launcher. See [`crate::sessions::of`].
    fn sessions(&self) -> Result<Vec<Session>, ModuleError>;
    /// Ends every session they have. See [`crate::sessions::revoke_all`].
    fn revoke_sessions(&self) -> Result<u64, ModuleError>;
}

impl PlayerActions for Player {
    fn ban(&self, reason: Option<&str>) -> Result<(), ModuleError> {
        crate::players::ban(self.id, reason)
    }
    fn unban(&self) -> Result<(), ModuleError> {
        crate::players::unban(self.id)
    }
    fn rename(&self, username: &str) -> Result<(), ModuleError> {
        crate::players::rename(self.id, username)
    }
    fn set_skin(&self, url: Option<&str>, slim: bool) -> Result<(), ModuleError> {
        crate::players::set_skin(self.id, url, slim)
    }
    fn set_cape(&self, cape_id: Option<Uuid>) -> Result<(), ModuleError> {
        crate::players::set_cape(self.id, cape_id)
    }
    fn presets(&self) -> Result<Vec<SkinPreset>, ModuleError> {
        crate::players::presets(self.id)
    }

    fn roles(&self) -> Result<Vec<Role>, ModuleError> {
        crate::roles::of(self.id)
    }
    fn grant_role(&self, role: impl IntoRoleRef) -> Result<(), ModuleError> {
        crate::roles::grant(self.id, role)
    }
    fn revoke_role(&self, role: impl IntoRoleRef) -> Result<(), ModuleError> {
        crate::roles::revoke(self.id, role)
    }

    fn can(&self, node: &str) -> Result<bool, ModuleError> {
        crate::permissions::has(self.id, node)
    }
    fn can_on(&self, node: &str, server_id: Uuid) -> Result<bool, ModuleError> {
        crate::permissions::has_on(self.id, node, server_id)
    }
    fn grant(&self, node: &str) -> Result<(), ModuleError> {
        crate::permissions::grant(self.id, node)
    }
    fn revoke(&self, node: &str) -> Result<(), ModuleError> {
        crate::permissions::revoke(self.id, node)
    }

    fn allow_join(&self, server_id: Uuid) -> Result<(), ModuleError> {
        crate::access::allow_join(self.id, server_id)
    }
    fn revoke_join(&self, server_id: Uuid) -> Result<(), ModuleError> {
        crate::access::revoke_join(self.id, server_id)
    }

    fn punish_ban(&self, reason: &str, seconds: Option<i64>) -> Result<Punishment, ModuleError> {
        crate::punish::ban(self.id, reason, seconds)
    }
    fn mute(&self, reason: &str, seconds: Option<i64>) -> Result<Punishment, ModuleError> {
        crate::punish::mute(self.id, reason, seconds)
    }
    fn warn(&self, reason: &str) -> Result<Punishment, ModuleError> {
        crate::punish::warn(self.id, reason)
    }
    fn server_ban(
        &self,
        server_id: Uuid,
        reason: &str,
        seconds: Option<i64>,
    ) -> Result<Punishment, ModuleError> {
        crate::punish::server_ban(self.id, server_id, reason, seconds)
    }
    fn punishments(&self) -> Result<Vec<Punishment>, ModuleError> {
        crate::punish::active(self.id)
    }

    fn account(&self, server_id: Uuid) -> Result<Option<Account>, ModuleError> {
        crate::bank::account(server_id, self.id)
    }
    fn balance(&self, server_id: Uuid) -> Result<i64, ModuleError> {
        crate::bank::balance(server_id, self.id)
    }

    fn tell(&self, message: &str) -> Result<bool, ModuleError> {
        crate::agent::tell(self.id, message)
    }
    fn kick(&self, reason: &str) -> Result<bool, ModuleError> {
        crate::agent::kick(self.id, reason)
    }
    fn is_online(&self) -> Result<bool, ModuleError> {
        crate::roster::is_online(self.id)
    }
    fn launcher_online(&self) -> Result<bool, ModuleError> {
        crate::launcher::is_online(self.id)
    }
    fn send(&self, payload: impl serde::Serialize) -> Result<bool, ModuleError> {
        crate::launcher::send(self.id, payload)
    }

    fn store(&self) -> crate::store::Store {
        crate::store::user(self.id)
    }

    fn identities(&self) -> Result<Vec<Identity>, ModuleError> {
        crate::identities::of(self.id)
    }
    fn sessions(&self) -> Result<Vec<Session>, ModuleError> {
        crate::sessions::of(self.id)
    }
    fn revoke_sessions(&self) -> Result<u64, ModuleError> {
        crate::sessions::revoke_all(self.id)
    }
}

/// What you can do with a server build you are holding.
pub trait ServerActions {
    /// Its client builds. See [`crate::builds::of`].
    fn builds(&self) -> Result<Vec<Build>, ModuleError>;
    /// The build players are getting. See [`crate::builds::published`].
    fn published_build(&self) -> Result<Option<Build>, ModuleError>;
    /// Its game servers. See [`crate::servers::game_servers`].
    fn game_servers(&self) -> Result<Vec<GameServer>, ModuleError>;
    /// Its treasury account. See [`crate::bank::treasury`].
    fn treasury(&self) -> Result<Account, ModuleError>;
    /// An announcement to everybody on it. See [`crate::agent::announce_on`].
    fn announce(&self, message: &str) -> Result<(), ModuleError>;
    /// This server's corner of your storage. See [`crate::store::server`].
    fn store(&self) -> crate::store::Store;
    /// Its hub feed, paginated. See [`crate::hub::feed`].
    fn feed(&self, page: i64) -> Result<noro_module_abi::ops::HubPage, ModuleError>;
    /// Its hub members. See [`crate::hub::members`].
    fn members(&self, page: i64) -> Result<noro_module_abi::ops::HubPage, ModuleError>;
}

impl ServerActions for Server {
    fn builds(&self) -> Result<Vec<Build>, ModuleError> {
        crate::builds::of(self.id)
    }
    fn published_build(&self) -> Result<Option<Build>, ModuleError> {
        crate::builds::published(self.id)
    }
    fn game_servers(&self) -> Result<Vec<GameServer>, ModuleError> {
        crate::servers::game_servers(self.id)
    }
    fn treasury(&self) -> Result<Account, ModuleError> {
        crate::bank::treasury(self.id)
    }
    fn announce(&self, message: &str) -> Result<(), ModuleError> {
        crate::agent::announce_on(self.id, message)
    }
    fn store(&self) -> crate::store::Store {
        crate::store::server(self.id)
    }
    fn feed(&self, page: i64) -> Result<noro_module_abi::ops::HubPage, ModuleError> {
        crate::hub::feed(self.id, page)
    }
    fn members(&self, page: i64) -> Result<noro_module_abi::ops::HubPage, ModuleError> {
        crate::hub::members(self.id, page)
    }
}

/// What you can do with a build you are holding.
pub trait BuildActions {
    /// Everything inside it. See [`crate::builds::files`].
    fn files(&self) -> Result<Vec<BuildFile>, ModuleError>;
    /// The text of one file. See [`crate::builds::read`].
    fn read(&self, path: &str) -> Result<Option<String>, ModuleError>;
    /// Writes a text file into it. See [`crate::builds::write`].
    fn write(&self, path: &str, text: &str) -> Result<BuildFile, ModuleError>;
    /// Puts an already-stored file in by hash. See [`crate::builds::attach`].
    fn attach(&self, path: &str, sha1: &str) -> Result<BuildFile, ModuleError>;
    /// Removes a file. See [`crate::builds::remove`].
    fn remove(&self, path: &str) -> Result<bool, ModuleError>;
    /// Stops the launcher handing it out. See [`crate::builds::unpublish`].
    fn unpublish(&self) -> Result<(), ModuleError>;
    /// Lets one player download it. See [`crate::access::allow_build`].
    fn allow(&self, who: impl noro_module_abi::player::IntoPlayerRef) -> Result<(), ModuleError>;
}

impl BuildActions for Build {
    fn files(&self) -> Result<Vec<BuildFile>, ModuleError> {
        crate::builds::files(self.id)
    }
    fn read(&self, path: &str) -> Result<Option<String>, ModuleError> {
        crate::builds::read(self.id, path)
    }
    fn write(&self, path: &str, text: &str) -> Result<BuildFile, ModuleError> {
        crate::builds::write(self.id, path, text)
    }
    fn attach(&self, path: &str, sha1: &str) -> Result<BuildFile, ModuleError> {
        crate::builds::attach(self.id, path, sha1)
    }
    fn remove(&self, path: &str) -> Result<bool, ModuleError> {
        crate::builds::remove(self.id, path)
    }
    fn unpublish(&self) -> Result<(), ModuleError> {
        crate::builds::unpublish(self.id)
    }
    fn allow(&self, who: impl noro_module_abi::player::IntoPlayerRef) -> Result<(), ModuleError> {
        crate::access::allow_build(who, self.id)
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
