//! The module manifest — `manifest.toml` inside the package.
//!
//! The file runs past a hundred and fifty lines deliberately: this is one
//! declaration, and splitting its halves across files would mean hunting for
//! manifest fields in two places.
//!
//! Two parties parse the manifest: the master when installing and `cargo-noro`
//! when building the package. That is why the types live here rather than in
//! the master's private code — otherwise a module author could not check their
//! own manifest without installing it.
//!
//! Only what has to be known **before** the module's code runs is here: who it
//! is, which ABI it needs, what it asks for, and which permission nodes it
//! introduces. What it *does* — events, endpoints, tasks — is declared in code
//! and arrives as [`crate::Registration`]: no handler name duplicated as a
//! string there, and the event name derived from the type.

use serde::{Deserialize, Serialize};

/// A parsed `manifest.toml`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Manifest {
    pub module: ModuleMeta,
    #[serde(default)]
    pub capabilities: Capabilities,
    #[serde(default)]
    pub apps: Vec<AppDecl>,
    #[serde(default)]
    pub permissions: Vec<PermissionDecl>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleMeta {
    /// `[a-z0-9-]` only: the identifier travels into a Postgres schema name,
    /// into the locale key prefix and into URLs, and nobody is going to escape
    /// it in three places.
    pub id: String,
    pub name: String,
    pub version: String,
    /// The required ABI version, e.g. `"1.0"`. The master compares the major.
    pub api: String,
    #[serde(default)]
    pub scope: Scope,
    #[serde(default)]
    pub author: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub homepage: Option<String>,
}

/// Where the module operates.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Scope {
    /// Across the whole instance. Sees global events such as registration too.
    #[default]
    Instance,
    /// Enabled on selected servers. Receives no events without a server
    /// attached at all: there would be nowhere to address them.
    Server,
}

/// What a module is allowed to touch.
///
/// An empty set is a denial. The operator sees this entire list at install
/// time, so the field names have to read as human language rather than be a
/// bitmask.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Capabilities {
    /// `read`, `ban`, `rename`, `skin`.
    #[serde(default)]
    pub players: Vec<String>,
    /// `read`, `link`.
    #[serde(default)]
    pub identities: Vec<String>,
    /// `read`, `grant`.
    #[serde(default)]
    pub roles: Vec<String>,
    /// `read`, `grant`.
    #[serde(default)]
    pub permissions: Vec<String>,
    /// `grant` — handing out access to servers and builds.
    #[serde(default)]
    pub access: Vec<String>,
    /// `read`.
    #[serde(default)]
    pub servers: Vec<String>,
    /// `read`, `maintenance`.
    #[serde(default)]
    pub gameservers: Vec<String>,
    /// `read`, `publish`. `publish` covers taking a build *out* of publication;
    /// putting one in rebuilds and signs the manifest, which takes minutes and
    /// stays with the operator.
    #[serde(default)]
    pub builds: Vec<String>,
    /// `read`, `transfer`.
    #[serde(default)]
    pub bank: Vec<String>,
    /// `read`, `issue`, `revoke`.
    #[serde(default)]
    pub punish: Vec<String>,
    /// `tell`, `announce`, `kick`.
    #[serde(default)]
    pub agent: Vec<String>,
    /// Its own KV store.
    #[serde(default)]
    pub store: bool,
    /// Its own Postgres schema.
    #[serde(default)]
    pub db: bool,
    /// The hosts a module may reach. Empty means no outbound access.
    #[serde(default)]
    pub http: Vec<String>,
}

impl Capabilities {
    /// Whether an action in a domain is allowed. A domain with no actions is closed entirely.
    pub fn allows(&self, domain: &str, action: &str) -> bool {
        let list = match domain {
            "players" => &self.players,
            "identities" => &self.identities,
            "roles" => &self.roles,
            "permissions" => &self.permissions,
            "access" => &self.access,
            "servers" => &self.servers,
            "gameservers" => &self.gameservers,
            "builds" => &self.builds,
            "bank" => &self.bank,
            "punish" => &self.punish,
            "agent" => &self.agent,
            "store" => return self.store,
            "db" => return self.db,
            _ => return false,
        };
        list.iter().any(|a| a == action)
    }

    /// Whether this host may be reached.
    pub fn allows_host(&self, host: &str) -> bool {
        self.http.iter().any(|h| {
            h == host
                || h.strip_prefix("*.")
                    .is_some_and(|suffix| host.ends_with(suffix) && host.len() > suffix.len())
        })
    }
}

/// The order of handlers for one event.
///
/// It repeats the ladder familiar from Bukkit: whoever decides runs after
/// whoever watches. `Monitor` is for observers — its decision to cancel is
/// ignored.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Priority {
    Lowest,
    Low,
    #[default]
    Normal,
    High,
    Highest,
    Monitor,
}

/// Who is let into an endpoint.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Auth {
    /// Anyone, even unauthenticated. For receiving webhooks from outside.
    Public,
    /// Any signed-in player.
    #[default]
    User,
    /// A player holding a permission.
    Permission(String),
    /// An admin token only, or staff holding a permission.
    Admin(String),
}

/// A mini-app and where it is embedded.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppDecl {
    pub placement: Placement,
    /// What opens it: a sandboxed page, or a panel component.
    #[serde(default)]
    pub kind: AppKind,
    /// A file inside the package's `web/`.
    pub entry: String,
    /// The Fluent key holding the entry's name.
    pub title: String,
    /// An `i-lucide-*` icon name.
    #[serde(default)]
    pub icon: Option<String>,
    /// The permission without which the entry is not shown.
    #[serde(default)]
    pub permission: Option<String>,
    /// The slot, when `placement = "widget"`.
    #[serde(default)]
    pub slot: Option<String>,
}

/// How a mini-app reaches the screen.
///
/// The choice is about isolation, not convenience. `Page` is a separate page in
/// a sandbox: its own origin, sight of nothing that is not its own, talking
/// over the bridge. `Vue` is a component the panel mounts inside itself: full
/// Vue, the panel's atoms and the panel's own look — but also its entire
/// environment, including the ability to break the page.
///
/// The second is acceptable because the instance owner installs the modules
/// themselves. For somebody else's code, the first remains.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AppKind {
    /// A page in an iframe. The default: isolation must not be lost to a
    /// forgotten field.
    #[default]
    Page,
    /// A Vue component, built by the module and mounted by the panel.
    Vue,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Placement {
    /// A section in the admin panel.
    Admin,
    /// A section in the server's hub.
    Hub,
    /// A page in the cabinet.
    Cabinet,
    /// A widget inside somebody else's page; `slot` sets the place.
    Widget,
}

/// A settings form field. It repeats the hub settings field model so the admin
/// panel can draw them with the same code.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingDecl {
    pub key: String,
    #[serde(rename = "type")]
    pub kind: SettingKind,
    /// The Fluent key holding the field's label.
    pub label: String,
    #[serde(default)]
    pub hint: Option<String>,
    #[serde(default)]
    pub min: Option<i64>,
    #[serde(default)]
    pub max: Option<i64>,
    #[serde(default)]
    pub options: Vec<String>,
    #[serde(default)]
    pub default: Option<serde_json::Value>,
}

impl SettingDecl {
    /// The value substituted until the operator has chosen anything.
    pub fn default(&mut self, value: impl Into<serde_json::Value>) -> &mut Self {
        self.default = Some(value.into());
        self
    }

    /// Bounds for a number. The admin panel checks them, not the module.
    pub fn range(&mut self, min: i64, max: i64) -> &mut Self {
        self.min = Some(min);
        self.max = Some(max);
        self
    }

    /// The note under the field. A Fluent key, like the label.
    pub fn hint(&mut self, key: impl Into<String>) -> &mut Self {
        self.hint = Some(key.into());
        self
    }

    /// The options for a `select`.
    pub fn options(&mut self, values: impl IntoIterator<Item = impl Into<String>>) -> &mut Self {
        self.options = values.into_iter().map(Into::into).collect();
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SettingKind {
    Number,
    Text,
    Toggle,
    Select,
    /// A comma-separated list of strings.
    List,
}

/// A permission node a module introduces.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionDecl {
    /// Has to start with `noro.module.<id>.`.
    pub node: String,
    /// The Fluent key holding the explanation.
    pub label: String,
}
