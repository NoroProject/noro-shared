//! Everything done to a player, sorted by what it is.
//!
//! A flat list of thirty methods on `Player` read as a heap: `mute` next to
//! `set_cape`, `can` next to `revoke_sessions`. Now each occupation has its own
//! handle — `player.punish().mute(…)`, `player.perms().grant(…)` — and
//! completion offers eight occupations before it offers thirty verbs.
//!
//! A handle holds nothing but an id and does nothing itself: it forwards to the
//! same domain function. The same trick as `player.store()`, which has been
//! here from the start.

use noro_module_abi::entity::{Account, Punishment, Role};
use noro_module_abi::error::ModuleError;
use noro_module_abi::ops::{Identity, IntoRoleRef, Session, SkinPreset};
use uuid::Uuid;

/// The account itself: name, skin, cape, and shutting it down.
///
/// The ban here is administrative — it closes the door and is not the
/// punishment a player reads and appeals. For those there is [`Punish`].
#[derive(Debug, Clone, Copy)]
pub struct Profile(pub(crate) Uuid);

impl Profile {
    /// Closes the account. See [`crate::players::ban`].
    pub fn ban(&self, reason: Option<&str>) -> Result<(), ModuleError> {
        crate::players::ban(self.0, reason)
    }
    /// Lifts the account ban. See [`crate::players::unban`].
    pub fn unban(&self) -> Result<(), ModuleError> {
        crate::players::unban(self.0)
    }
    /// Renames them. Refused if the name is taken. See [`crate::players::rename`].
    pub fn rename(&self, username: &str) -> Result<(), ModuleError> {
        crate::players::rename(self.0, username)
    }
    /// Sets the skin; `slim` travels with it. See [`crate::players::set_skin`].
    pub fn set_skin(&self, url: Option<&str>, slim: bool) -> Result<(), ModuleError> {
        crate::players::set_skin(self.0, url, slim)
    }
    /// Puts a cape on, or takes it off with `None`. See [`crate::players::set_cape`].
    pub fn set_cape(&self, cape_id: Option<Uuid>) -> Result<(), ModuleError> {
        crate::players::set_cape(self.0, cape_id)
    }
    /// Their saved skin presets. See [`crate::players::presets`].
    pub fn presets(&self) -> Result<Vec<SkinPreset>, ModuleError> {
        crate::players::presets(self.0)
    }
}

/// Their roles.
#[derive(Debug, Clone, Copy)]
pub struct Roles(pub(crate) Uuid);

impl Roles {
    /// Which roles they hold. See [`crate::roles::of`].
    pub fn list(&self) -> Result<Vec<Role>, ModuleError> {
        crate::roles::of(self.0)
    }
    /// Grants a role by name or id. See [`crate::roles::grant`].
    pub fn grant(&self, role: impl IntoRoleRef) -> Result<(), ModuleError> {
        crate::roles::grant(self.0, role)
    }
    /// Takes a role away. See [`crate::roles::revoke`].
    pub fn revoke(&self, role: impl IntoRoleRef) -> Result<(), ModuleError> {
        crate::roles::revoke(self.0, role)
    }
}

/// Permissions: checking them, and granting one personally.
#[derive(Debug, Clone, Copy)]
pub struct Perms(pub(crate) Uuid);

impl Perms {
    /// Whether they effectively hold it, roles included. See [`crate::permissions::has`].
    pub fn has(&self, node: &str) -> Result<bool, ModuleError> {
        crate::permissions::has(self.0, node)
    }
    /// The same on one server build. See [`crate::permissions::has_on`].
    pub fn has_on(&self, node: &str, server_id: Uuid) -> Result<bool, ModuleError> {
        crate::permissions::has_on(self.0, node, server_id)
    }
    /// Grants it to them personally, beside their roles. See [`crate::permissions::grant`].
    pub fn grant(&self, node: &str) -> Result<(), ModuleError> {
        crate::permissions::grant(self.0, node)
    }
    /// Takes a personal permission away. See [`crate::permissions::revoke`].
    pub fn revoke(&self, node: &str) -> Result<(), ModuleError> {
        crate::permissions::revoke(self.0, node)
    }
}

/// Access to builds: who is let onto a server.
#[derive(Debug, Clone, Copy)]
pub struct Access(pub(crate) Uuid);

impl Access {
    /// Lets them into a server build. See [`crate::access::allow_join`].
    pub fn allow_join(&self, server_id: Uuid) -> Result<(), ModuleError> {
        crate::access::allow_join(self.0, server_id)
    }
    /// Takes that away. See [`crate::access::revoke_join`].
    pub fn revoke_join(&self, server_id: Uuid) -> Result<(), ModuleError> {
        crate::access::revoke_join(self.0, server_id)
    }
}

/// Punishments: the kind a player reads and can appeal.
#[derive(Debug, Clone, Copy)]
pub struct Punish(pub(crate) Uuid);

impl Punish {
    /// A ban with a reason and a term. See [`crate::punish::ban`].
    pub fn ban(&self, reason: &str, seconds: Option<i64>) -> Result<Punishment, ModuleError> {
        crate::punish::ban(self.0, reason, seconds)
    }
    /// Mutes them. See [`crate::punish::mute`].
    pub fn mute(&self, reason: &str, seconds: Option<i64>) -> Result<Punishment, ModuleError> {
        crate::punish::mute(self.0, reason, seconds)
    }
    /// Warns them. See [`crate::punish::warn`].
    pub fn warn(&self, reason: &str) -> Result<Punishment, ModuleError> {
        crate::punish::warn(self.0, reason)
    }
    /// Bans them from one server only. See [`crate::punish::server_ban`].
    pub fn server_ban(
        &self,
        server_id: Uuid,
        reason: &str,
        seconds: Option<i64>,
    ) -> Result<Punishment, ModuleError> {
        crate::punish::server_ban(self.0, server_id, reason, seconds)
    }
    /// Their punishments in force. See [`crate::punish::active`].
    pub fn active(&self) -> Result<Vec<Punishment>, ModuleError> {
        crate::punish::active(self.0)
    }
}

/// Their money on one server build.
///
/// With the build named, because a player has more than one account — one per
/// server — and "their balance" without saying where has no answer.
#[derive(Debug, Clone, Copy)]
pub struct Bank {
    pub(crate) who: Uuid,
    pub(crate) server_id: Uuid,
}

impl Bank {
    /// Their account. See [`crate::bank::account`].
    pub fn account(&self) -> Result<Option<Account>, ModuleError> {
        crate::bank::account(self.who, self.server_id)
    }
    /// What is on it, in the smallest unit. See [`crate::bank::balance`].
    pub fn balance(&self) -> Result<i64, ModuleError> {
        crate::bank::balance(self.who, self.server_id)
    }
}

/// The player in game: say something, throw them out, ask if they are there.
#[derive(Debug, Clone, Copy)]
pub struct InGame(pub(crate) Uuid);

impl InGame {
    /// A private line in chat. `false` — they are not in game. See [`crate::agent::tell`].
    pub fn tell(&self, message: &str) -> Result<bool, ModuleError> {
        crate::agent::tell(self.0, message)
    }
    /// Throws them off the server. Not a punishment: it leaves no record.
    /// See [`crate::agent::kick`].
    pub fn kick(&self, reason: &str) -> Result<bool, ModuleError> {
        crate::agent::kick(self.0, reason)
    }
    /// Whether they are in game right now. See [`crate::roster::is_online`].
    pub fn online(&self) -> Result<bool, ModuleError> {
        crate::roster::is_online(self.0)
    }
}

/// Their launcher.
#[derive(Debug, Clone, Copy)]
pub struct Launcher(pub(crate) Uuid);

impl Launcher {
    /// Whether it is connected. See [`crate::launcher::is_online`].
    pub fn online(&self) -> Result<bool, ModuleError> {
        crate::launcher::is_online(self.0)
    }
    /// Sends it a frame. See [`crate::launcher::send`].
    pub fn send(&self, payload: impl serde::Serialize) -> Result<bool, ModuleError> {
        crate::launcher::send(self.0, payload)
    }
}

/// Their browser website tab.
#[derive(Debug, Clone, Copy)]
pub struct WebWs(pub(crate) Uuid);

impl WebWs {
    /// Whether they have a website tab open. See [`crate::web_ws::is_online`].
    pub fn online(&self) -> Result<bool, ModuleError> {
        crate::web_ws::is_online(self.0)
    }
    /// Sends a frame to their open website tab. See [`crate::web_ws::send`].
    pub fn send(&self, payload: impl serde::Serialize) -> Result<bool, ModuleError> {
        crate::web_ws::send(self.0, payload)
    }
}

/// How they sign in, and where they are signed in.
#[derive(Debug, Clone, Copy)]
pub struct Logins(pub(crate) Uuid);

impl Logins {
    /// Their linked logins. See [`crate::identities::of`].
    pub fn identities(&self) -> Result<Vec<Identity>, ModuleError> {
        crate::identities::of(self.0)
    }
    /// Their sessions in the panel and the launcher. See [`crate::sessions::of`].
    pub fn sessions(&self) -> Result<Vec<Session>, ModuleError> {
        crate::sessions::of(self.0)
    }
    /// Ends every session they have. See [`crate::sessions::revoke_all`].
    pub fn revoke_all(&self) -> Result<u64, ModuleError> {
        crate::sessions::revoke_all(self.0)
    }
}
