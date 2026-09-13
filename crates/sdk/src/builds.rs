//! Client builds and the files inside them.
//!
//! ```ignore
//! let build = builds::published(server_id)?.ok_or_else(|| ModuleError::invalid("no build"))?;
//! builds::write(build.id, "config/mymod.toml", &rendered)?;
//! ```
//!
//! # Writing here reaches players
//!
//! A file in a build is downloaded and put on disk by the launcher. Nothing has
//! to be re-signed afterwards — the manifest is assembled and signed on every
//! launcher request from the current rows, so a written file is live from the
//! next one. That is convenient and it is also the warning: there is no review
//! step between your write and somebody's game directory.
//!
//! Which is why `files` is its own capability rather than part of `read`.

use noro_module_abi::entity::{Build, BuildFile};
use noro_module_abi::error::ModuleError;
use noro_module_abi::ops::{BuildFileAttach, BuildFileRef, BuildTextWrite, PublishRequest};
use uuid::Uuid;

/// The client builds of a server.
///
/// Requires `builds = ["read"]`.
pub fn of(server_id: Uuid) -> Result<Vec<Build>, ModuleError> {
    crate::host::builds_list_call(server_id)
}

/// The build players are currently getting, if there is one.
///
/// Requires `builds = ["read"]`.
pub fn published(server_id: Uuid) -> Result<Option<Build>, ModuleError> {
    crate::host::build_published_call(server_id)
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

/// Everything inside a build, with the hash the launcher downloads by.
///
/// Requires `builds = ["read"]`.
pub fn files(build_id: Uuid) -> Result<Vec<BuildFile>, ModuleError> {
    crate::host::build_files_call(build_id)
}

/// The text of one file, by its path in the build.
///
/// `None` covers both "no such file" and "not text" — a jar has no text to
/// return, and calling that an error would make the ordinary case look broken.
/// Files over a megabyte are refused: they are not configuration.
///
/// Requires `builds = ["read"]`.
pub fn read(build_id: Uuid, path: &str) -> Result<Option<String>, ModuleError> {
    crate::host::build_file_read_call(BuildFileRef {
        build_id,
        path: path.to_string(),
    })
}

/// Writes a text file into the build, replacing whatever was at that path.
///
/// For configuration — a rendered `.toml`, a server list, a message of the day.
/// Anything binary goes in with [`attach`].
///
/// Requires `builds = ["files"]`.
pub fn write(build_id: Uuid, path: &str, text: &str) -> Result<BuildFile, ModuleError> {
    write_for(build_id, path, text, "both")
}

/// The same, for one side only: `client`, `server` or `both`.
///
/// Requires `builds = ["files"]`.
pub fn write_for(
    build_id: Uuid,
    path: &str,
    text: &str,
    side: &str,
) -> Result<BuildFile, ModuleError> {
    crate::host::build_file_write_call(BuildTextWrite {
        build_id,
        path: path.to_string(),
        text: text.to_string(),
        side: side.to_string(),
    })
}

/// Puts an already-stored file into the build under a path.
///
/// This is how a mod gets in. Store the bytes once with [`crate::files::put`],
/// then attach the hash you got back — to one build or to twenty. Dragging tens
/// of megabytes through the sandbox as a string is the alternative, and it is
/// not one.
///
/// Requires `builds = ["files"]` and `files = ["read"]`.
pub fn attach(build_id: Uuid, path: &str, sha1: &str) -> Result<BuildFile, ModuleError> {
    crate::host::build_file_attach_call(BuildFileAttach {
        build_id,
        path: path.to_string(),
        sha1: sha1.to_string(),
        side: String::new(),
    })
}

/// Removes a file from the build. `false` means there was nothing at that path.
///
/// The bytes stay in the store — another build may be using the same ones.
///
/// Requires `builds = ["files"]`.
pub fn remove(build_id: Uuid, path: &str) -> Result<bool, ModuleError> {
    crate::host::build_file_remove_call(BuildFileRef {
        build_id,
        path: path.to_string(),
    })
}
