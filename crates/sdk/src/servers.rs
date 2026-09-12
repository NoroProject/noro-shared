//! Server builds, their client builds, and the game servers behind them.
//!
//! "Server" in the interface is a server build — the thing a player picks in
//! the launcher. A *game server* is one running process an agent is attached
//! to, and a *build* is one downloadable client of that server.

use noro_module_abi::entity::{Build, GameServer, Server};
use noro_module_abi::error::ModuleError;
use noro_module_abi::ops::{Maintenance, PublishRequest};
use uuid::Uuid;

/// Every server build.
///
/// Requires `servers = ["read"]`.
pub fn list() -> Result<Vec<Server>, ModuleError> {
    crate::host::servers_list_call(())
}

/// One server build by identifier.
///
/// Requires `servers = ["read"]`.
pub fn get(id: Uuid) -> Result<Option<Server>, ModuleError> {
    crate::host::server_get_call(id)
}

/// One server build by its hub slug — the `/s/<slug>` address.
///
/// Requires `servers = ["read"]`.
pub fn by_slug(slug: &str) -> Result<Option<Server>, ModuleError> {
    crate::host::server_by_slug_call(slug.to_string())
}

/// The client builds of a server.
///
/// Requires `builds = ["read"]`.
pub fn builds(server_id: Uuid) -> Result<Vec<Build>, ModuleError> {
    crate::host::builds_list_call(server_id)
}

/// The build players are currently getting, if there is one.
///
/// Requires `builds = ["read"]`.
pub fn published_build(server_id: Uuid) -> Result<Option<Build>, ModuleError> {
    crate::host::build_published_call(server_id)
}

/// The game servers of a server build.
///
/// Requires `gameservers = ["read"]`.
pub fn game_servers(server_id: Uuid) -> Result<Vec<GameServer>, ModuleError> {
    crate::host::gameservers_list_call(server_id)
}

/// One game server by identifier.
///
/// Requires `gameservers = ["read"]`.
pub fn game_server(id: Uuid) -> Result<Option<GameServer>, ModuleError> {
    crate::host::gameserver_get_call(id)
}

/// Puts a game server into maintenance, or takes it out.
///
/// Requires `gameservers = ["read", "maintenance"]`.
pub fn set_maintenance(game_server_id: Uuid, enabled: bool) -> Result<(), ModuleError> {
    crate::host::gameserver_maintenance_call(Maintenance {
        game_server_id,
        enabled,
    })
}

/// Takes a build out of publication, so the launcher stops handing it out.
///
/// For pulling a build that turned out broken — the case where waiting for
/// somebody to wake up and press the button is the expensive part. Players who
/// already downloaded it keep it.
///
/// # Why there is no `publish`
///
/// Publishing is not this flag. It bootstraps the build's artifacts, fetches
/// whatever assets and Java are missing and signs the manifest — minutes of
/// work on a first run. A module's call has seconds, so the call would time out
/// halfway through and leave a half-built publication behind. That one stays
/// with the operator.
///
/// Requires `builds = ["read", "publish"]`.
pub fn unpublish(build_id: Uuid) -> Result<(), ModuleError> {
    crate::host::build_publish_call(PublishRequest {
        build_id,
        published: false,
    })
}
