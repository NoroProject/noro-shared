//! What the master asks a node to do.
//!
//! Every variant names the server it is about: one socket carries the whole
//! node, unlike the wrapper channel where the socket *was* the server.

use crate::build::FileEntry;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::time::Duration;

/// Everything a node needs to build the container. Assembled by the master, so
/// the heap rule and the startup line have one home and show up in the panel.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerSpec {
    pub image: String,
    pub memory_mb: i64,
    #[serde(default)]
    pub swap_mb: Option<i64>,
    pub disk_mb: i64,
    /// Percent of one core; 200 means two cores. 0 is unlimited.
    #[serde(default)]
    pub cpu_percent: i64,
    #[serde(default)]
    pub pids_limit: i64,
    pub jvm_args: Vec<String>,
    /// Server jar relative to the server root, or `@libraries/…/unix_args.txt`
    /// for NeoForge — passed through untouched, exactly as the wrapper does.
    pub jar: String,
    #[serde(default)]
    pub server_args: Vec<String>,
    pub ports: Vec<PortMapping>,
    #[serde(default)]
    pub env: BTreeMap<String, String>,
    /// Yggdrasil root for `-javaagent`. Absent means vanilla auth.
    #[serde(default)]
    pub authlib_url: Option<String>,
    /// Game agent to install into `mods/`/`plugins/`; the node verifies the
    /// ed25519 descriptor itself before writing anything.
    #[serde(default)]
    pub agent: Option<AgentSpec>,
}

/// What a server shows in the client's server list.
///
/// Carried to the node rather than kept there: everything in it is edited on
/// the site, and a node holding its own copy would answer with yesterday's
/// wording the first time somebody changed it.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Listing {
    /// Two lines, as the protocol allows. Already rendered — placeholders are
    /// substituted by the master, the only side that knows the name and the
    /// player count.
    pub motd: String,
    /// PNG 64×64 as a `data:image/png;base64,…` URI, the form the protocol
    /// wants. Absent means the client draws its default.
    #[serde(default)]
    pub favicon: Option<String>,
    /// Text next to the player count. The client only draws it when the
    /// protocol does not match — which for a sleeping server is always, since
    /// the one answering is not a server.
    #[serde(default)]
    pub version_name: String,
    /// `-1` keeps the mismatch, and with it the version line visible.
    #[serde(default = "minus_one")]
    pub protocol: i32,
}

fn minus_one() -> i32 {
    -1
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct PortMapping {
    pub port: u16,
    #[serde(default)]
    pub protocol: PortProtocol,
    /// The one the players connect to; it also becomes `server-port`.
    #[serde(default)]
    pub primary: bool,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "lowercase")]
pub enum PortProtocol {
    #[default]
    Tcp,
    Udp,
    Both,
}

/// The game agent jar to install alongside the server.
///
/// No signature here, unlike the descriptor the wrapper fetches over HTTP: this
/// one arrives over the node's authenticated socket, and `sha1` — checked while
/// downloading — already pins the contents.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentSpec {
    pub url: String,
    pub sha1: String,
    pub size: u64,
    /// `mods` or `plugins`, decided by the platform.
    pub dir: String,
}

/// Where the server core comes from. Public cores are fetched by the node;
/// only a custom upload is served from the master's store.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreSource {
    pub url: String,
    #[serde(default)]
    pub sha1: Option<String>,
    #[serde(default)]
    pub sha256: Option<String>,
    /// Forge and NeoForge ship an installer rather than a runnable jar. The node
    /// runs it inside a throwaway container of the target image — an installer
    /// is arbitrary code and has no business running on the host.
    #[serde(default)]
    pub installer: bool,
    #[serde(default)]
    pub installer_args: Vec<String>,
}

/// Which directories the build owns. Outside them nothing is ever deleted: a
/// world lost to a sync is not recoverable by apologising.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncPolicy {
    pub managed_roots: Vec<String>,
    /// Never touched even inside a managed root.
    pub keep: Vec<String>,
    pub prune: bool,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum PowerAction {
    Start,
    Stop,
    Restart,
    Kill,
}

/// Everything the install needs, in one box.
///
/// Boxed as a whole rather than inlined: `ServerSpec` alone is several hundred
/// bytes, and an enum is as large as its biggest variant — a `Power` frame
/// would otherwise carry that weight through every queue it passes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstallSpec {
    pub core: CoreSource,
    pub spec: ServerSpec,
    /// Seeded into `server.properties` before the first start.
    #[serde(default)]
    pub properties: BTreeMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "op", content = "args", rename_all = "snake_case")]
pub enum NodeOp {
    // --- lifecycle ---------------------------------------------------------
    ServerCreate {
        server: uuid::Uuid,
        spec: Box<ServerSpec>,
    },
    ServerUpdate {
        server: uuid::Uuid,
        spec: Box<ServerSpec>,
    },
    ServerDelete {
        server: uuid::Uuid,
        #[serde(default)]
        keep_files: bool,
    },
    Install {
        server: uuid::Uuid,
        install: Box<InstallSpec>,
    },

    // --- power and console -------------------------------------------------
    Power {
        server: uuid::Uuid,
        action: PowerAction,
    },
    Command {
        server: uuid::Uuid,
        line: String,
    },
    ConsoleAttach {
        server: uuid::Uuid,
    },
    ConsoleDetach {
        server: uuid::Uuid,
    },

    // --- files -------------------------------------------------------------
    FsList {
        server: uuid::Uuid,
        path: String,
    },
    FsRead {
        server: uuid::Uuid,
        path: String,
        max_bytes: u64,
    },
    FsWrite {
        server: uuid::Uuid,
        path: String,
        content: String,
    },
    FsDelete {
        server: uuid::Uuid,
        paths: Vec<String>,
    },
    FsMkdir {
        server: uuid::Uuid,
        path: String,
    },
    FsRename {
        server: uuid::Uuid,
        from: String,
        to: String,
    },
    /// Fetch a file by URL into the server directory, verifying the hash.
    FsPull {
        server: uuid::Uuid,
        url: String,
        sha1: String,
        dest: String,
    },
    FsArchive {
        server: uuid::Uuid,
        paths: Vec<String>,
        dest: String,
    },
    FsUnarchive {
        server: uuid::Uuid,
        path: String,
        dest_dir: String,
    },
    /// One-shot ticket for a direct upload or download against the node.
    FsTicket {
        server: uuid::Uuid,
        path: String,
        mode: TicketMode,
        ttl_secs: u64,
    },

    // --- build rollout -----------------------------------------------------
    BuildSync {
        server: uuid::Uuid,
        /// Server-side files of the build, reusing the launcher's entry type.
        files: Vec<FileEntry>,
        policy: SyncPolicy,
    },

    /// Hold the port of a sleeping server, or stop holding it.
    ///
    /// While held, the server still answers the server list with `motd`, and
    /// the first login attempt starts it. Without this a sleeping server is
    /// simply offline, and the player has no way to bring it back.
    ///
    /// The node starts the container itself on that knock: routing the wake-up
    /// through the master would add its round trip to a player already staring
    /// at a loading screen.
    SleepPlaceholder {
        server: uuid::Uuid,
        port: u16,
        /// How the server looks in the list while it sleeps.
        listing: Box<Listing>,
        /// `false` releases the port — the container needs it back.
        enable: bool,
    },

    /// Ask the server itself whether it is alive, over the Minecraft protocol.
    ///
    /// The node asks, not the master: the game port may only be reachable from
    /// the machine itself, and the master would have to be told a routable
    /// address for every server just to send one packet.
    ///
    /// This is the only source of "is anybody playing" for a server without our
    /// agent — a plain Paper with somebody else's plugins answers a server list
    /// ping and nothing else.
    ServerPing {
        server: uuid::Uuid,
        /// Published port on the node. The master knows the allocation; the
        /// daemon does not keep one, and asking Docker for it would be a round
        /// trip for a number we already have.
        port: u16,
    },

    /// Fill a freshly installed server from another one on the same node.
    ///
    /// Copying happens on the node because that is where both directories live:
    /// pulling a world through the master and pushing it back would move
    /// gigabytes across the network to land them a directory away.
    ///
    /// Only within one node. Moving a server to a different machine is a
    /// transfer, with its own ticket and its own rollback.
    ServerCloneFiles {
        /// The new server — the one being filled.
        server: uuid::Uuid,
        /// Where to copy from.
        from: uuid::Uuid,
        /// Take the worlds too. Without them the clone is a copy of the setup:
        /// same mods, same configs, empty map.
        #[serde(default)]
        include_worlds: bool,
    },

    // --- backups -----------------------------------------------------------
    BackupCreate {
        server: uuid::Uuid,
        backup: uuid::Uuid,
        name: String,
        #[serde(default)]
        ignore: Vec<String>,
        /// Where to push it after packing. Absent means it stays on the node.
        #[serde(default)]
        upload: Option<Box<BackupUpload>>,
    },
    BackupRestore {
        server: uuid::Uuid,
        backup: uuid::Uuid,
        #[serde(default)]
        download: Option<Box<BackupUpload>>,
    },
    BackupDelete {
        server: uuid::Uuid,
        backup: uuid::Uuid,
    },
    BackupTicket {
        server: uuid::Uuid,
        backup: uuid::Uuid,
        ttl_secs: u64,
    },

    // --- node itself -------------------------------------------------------
    NodeInfo,
    ImagePull {
        image: String,
    },
    SftpReload,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum TicketMode {
    Upload,
    Download,
}

/// Credentials for the node to talk to a backup storage directly. Handed out
/// per operation — a node never holds a standing copy of them.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupUpload {
    pub storage: uuid::Uuid,
    pub kind: String,
    pub config: serde_json::Value,
    pub path: String,
}

impl NodeOp {
    /// How long to wait for the reply. A world archive packs for minutes while
    /// `stop` waits for a save — one shared timeout would cut both in half.
    pub fn timeout(&self) -> Duration {
        match self {
            NodeOp::Install { .. } => Duration::from_secs(1800),
            NodeOp::BuildSync { .. }
            | NodeOp::ServerCloneFiles { .. }
            | NodeOp::BackupCreate { .. }
            | NodeOp::BackupRestore { .. }
            | NodeOp::FsUnarchive { .. }
            | NodeOp::FsArchive { .. } => Duration::from_secs(900),
            NodeOp::ImagePull { .. } | NodeOp::ServerCreate { .. } => Duration::from_secs(600),
            NodeOp::Power { .. } | NodeOp::ServerDelete { .. } => Duration::from_secs(180),
            NodeOp::FsPull { .. } => Duration::from_secs(300),
            _ => Duration::from_secs(30),
        }
    }

    /// Which server it concerns, for routing replies and refusing a server that
    /// lives on another node.
    pub fn server(&self) -> Option<uuid::Uuid> {
        match self {
            NodeOp::ServerCreate { server, .. }
            | NodeOp::ServerUpdate { server, .. }
            | NodeOp::ServerDelete { server, .. }
            | NodeOp::Install { server, .. }
            | NodeOp::Power { server, .. }
            | NodeOp::Command { server, .. }
            | NodeOp::ConsoleAttach { server }
            | NodeOp::ConsoleDetach { server }
            | NodeOp::FsList { server, .. }
            | NodeOp::FsRead { server, .. }
            | NodeOp::FsWrite { server, .. }
            | NodeOp::FsDelete { server, .. }
            | NodeOp::FsMkdir { server, .. }
            | NodeOp::FsRename { server, .. }
            | NodeOp::FsPull { server, .. }
            | NodeOp::FsArchive { server, .. }
            | NodeOp::FsUnarchive { server, .. }
            | NodeOp::FsTicket { server, .. }
            | NodeOp::BuildSync { server, .. }
            | NodeOp::ServerPing { server, .. }
            | NodeOp::SleepPlaceholder { server, .. }
            | NodeOp::ServerCloneFiles { server, .. }
            | NodeOp::BackupCreate { server, .. }
            | NodeOp::BackupRestore { server, .. }
            | NodeOp::BackupDelete { server, .. }
            | NodeOp::BackupTicket { server, .. } => Some(*server),
            NodeOp::NodeInfo | NodeOp::ImagePull { .. } | NodeOp::SftpReload => None,
        }
    }
}
