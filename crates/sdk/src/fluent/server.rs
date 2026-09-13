//! Handles for a server build and for the game servers running it.

use noro_module_abi::entity::{Account, Build, BuildFile, GameServer};
use noro_module_abi::error::ModuleError;
use noro_module_abi::ops::HubPage;
use noro_module_abi::player::IntoPlayerRef;
use uuid::Uuid;

/// The client builds of one server.
#[derive(Debug, Clone, Copy)]
pub struct Builds(pub(crate) Uuid);

impl Builds {
    /// All of them. See [`crate::builds::of`].
    pub fn list(&self) -> Result<Vec<Build>, ModuleError> {
        crate::builds::of(self.0)
    }
    /// The one players are getting. See [`crate::builds::published`].
    pub fn published(&self) -> Result<Option<Build>, ModuleError> {
        crate::builds::published(self.0)
    }
}

/// The hub of one server: its feed and its members.
#[derive(Debug, Clone, Copy)]
pub struct Hub(pub(crate) Uuid);

impl Hub {
    /// Its feed, paginated. See [`crate::hub::feed`].
    pub fn feed(&self, page: i64) -> Result<HubPage, ModuleError> {
        crate::hub::feed(self.0, page)
    }
    /// Its members, paginated. See [`crate::hub::members`].
    pub fn members(&self, page: i64) -> Result<HubPage, ModuleError> {
        crate::hub::members(self.0, page)
    }
}

/// What is inside a build.
///
/// Writing here reaches what the launcher downloads and puts in somebody's game
/// directory, with no review step in between. That is why it is its own handle:
/// `build.files().write(…)` says where the line ends up, and
/// `build.write(…)` did not.
#[derive(Debug, Clone, Copy)]
pub struct Files(pub(crate) Uuid);

impl Files {
    /// Everything inside it. See [`crate::builds::files`].
    pub fn list(&self) -> Result<Vec<BuildFile>, ModuleError> {
        crate::builds::files(self.0)
    }
    /// The text of one file. See [`crate::builds::read`].
    pub fn read(&self, path: &str) -> Result<Option<String>, ModuleError> {
        crate::builds::read(self.0, path)
    }
    /// Writes a text file into it. See [`crate::builds::write`].
    pub fn write(&self, path: &str, text: &str) -> Result<BuildFile, ModuleError> {
        crate::builds::write(self.0, path, text)
    }
    /// Puts an already-stored file in by hash. See [`crate::builds::attach`].
    pub fn attach(&self, path: &str, sha1: &str) -> Result<BuildFile, ModuleError> {
        crate::builds::attach(self.0, path, sha1)
    }
    /// Removes a file. See [`crate::builds::remove`].
    pub fn remove(&self, path: &str) -> Result<bool, ModuleError> {
        crate::builds::remove(self.0, path)
    }
}

/// Who may download a build.
#[derive(Debug, Clone, Copy)]
pub struct BuildAccess(pub(crate) Uuid);

impl BuildAccess {
    /// Lets one player download it. See [`crate::access::allow_build`].
    pub fn allow(&self, who: impl IntoPlayerRef) -> Result<(), ModuleError> {
        crate::access::allow_build(who, self.0)
    }
}

/// The money of one server.
#[derive(Debug, Clone, Copy)]
pub struct Treasury(pub(crate) Uuid);

impl Treasury {
    /// Its treasury account. See [`crate::bank::treasury`].
    pub fn account(&self) -> Result<Account, ModuleError> {
        crate::bank::treasury(self.0)
    }
}

/// The game servers of one build.
#[derive(Debug, Clone, Copy)]
pub struct GameServers(pub(crate) Uuid);

impl GameServers {
    /// All of them. See [`crate::servers::game_servers`].
    pub fn list(&self) -> Result<Vec<GameServer>, ModuleError> {
        crate::servers::game_servers(self.0)
    }
    /// An announcement to everybody on the server. See [`crate::agent::announce_on`].
    pub fn announce(&self, message: &str) -> Result<(), ModuleError> {
        crate::agent::announce_on(self.0, message)
    }
}
