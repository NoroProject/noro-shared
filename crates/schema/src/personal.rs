// Over 150 lines: what a player installed, where it lands and why it might not
// be there — one subject, and the three halves are unreadable apart.
//! Personal content: mods, resource packs and shaders a player adds on top of a
//! build.
//!
//! The set lives on the master, not on the player's disk. Two reasons. The
//! manifest is built per viewer and signed on the way out, so content listed in
//! it is covered by the same signature as the build's own files — `clean_extra`
//! then leaves it alone and the integrity report stays quiet, with no list of
//! exceptions to keep in step on the client. And an operator can see what is
//! actually running on a machine without asking the machine, which is the one
//! answer a client-side list can never give honestly.
//!
//! What the player picked and what they get are deliberately two different
//! lists: a mod can be shadowed by the build, blocked, or switched off, and an
//! entry that vanishes with no explanation reads as a launcher fault.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// What kind of thing it is, which is also where it goes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum ContentKind {
    #[default]
    Mod,
    ResourcePack,
    Shader,
}

impl ContentKind {
    /// Folder inside the instance. Also the reason a kind exists at all: a
    /// shader dropped into `mods/` does nothing and looks like a broken install.
    pub fn folder(self) -> &'static str {
        match self {
            ContentKind::Mod => "mods",
            ContentKind::ResourcePack => "resourcepacks",
            ContentKind::Shader => "shaderpacks",
        }
    }

    /// Project type as the catalogue providers name it.
    pub fn project_type(self) -> &'static str {
        match self {
            ContentKind::Mod => "mod",
            ContentKind::ResourcePack => "resourcepack",
            ContentKind::Shader => "shader",
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            ContentKind::Mod => "mod",
            ContentKind::ResourcePack => "resource_pack",
            ContentKind::Shader => "shader",
        }
    }

    /// `parse` rather than `from_str`: the real `FromStr` would have to pick an
    /// error type for a case that has none — an unknown kind is simply not one
    /// of the three, and the caller falls back to a mod.
    pub fn parse(raw: &str) -> Option<Self> {
        match raw {
            "mod" => Some(ContentKind::Mod),
            "resource_pack" | "resourcepack" => Some(ContentKind::ResourcePack),
            "shader" => Some(ContentKind::Shader),
            _ => None,
        }
    }

    /// Only mods go on the classpath and only they can collide with the build's
    /// own jars. A resource pack with the same name as one in the build is two
    /// packs in a folder, which is fine.
    pub fn collides_with_build(self) -> bool {
        matches!(self, ContentKind::Mod)
    }
}

/// One installed item, as the signed manifest carries it.
///
/// Only the file is needed to sync it; the rest travels so the launcher can
/// name what it is showing without a second request, and so a support bundle
/// says "Sodium 0.6.0 from Modrinth" rather than a hash.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PersonalContent {
    pub id: Uuid,
    #[serde(default)]
    pub kind: ContentKind,
    /// `modrinth` or `curseforge`.
    pub provider: String,
    pub project_id: String,
    pub version_id: String,
    /// Human version, e.g. `0.6.0+mc1.21.1`.
    #[serde(default)]
    pub version_name: String,
    pub title: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    /// Path inside the instance, e.g. `mods/sodium-0.6.0.jar`.
    pub path: String,
}

/// Why an installed item is not in the instance right now.
///
/// Carried to the player rather than resolved silently: every one of these is
/// something they chose that is not happening, and the difference between "the
/// build ships it now" and "staff blocked it" changes what they do next.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum PersonalState {
    /// Installed and in the manifest.
    Active,
    /// The player switched it off; the file is not synced.
    Disabled,
    /// The build itself now ships this mod. The build's copy wins: two jars of
    /// one mod in `mods/` is a crash on launch, not a choice.
    Superseded {
        /// Which file of the build took over, for the explanation.
        by: String,
    },
    /// Refused by the build's blocklist.
    Blocked { reason: String },
    /// Not published for this build's loader or Minecraft version. Kept rather
    /// than deleted: the build moves, and the same mod usually comes back.
    Incompatible {
        #[serde(default)]
        mc_version: String,
        #[serde(default)]
        loader: String,
    },
}

impl PersonalState {
    pub fn is_active(&self) -> bool {
        matches!(self, PersonalState::Active)
    }
}

/// An installed item with everything the launcher and the admin panel show.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalItem {
    #[serde(flatten)]
    pub content: PersonalContent,
    pub state: PersonalState,
    pub size: u64,
    pub sha1: String,
    pub installed_at: chrono::DateTime<chrono::Utc>,
    /// Which build it was installed against, for the admin panel.
    pub server_id: Uuid,
}

/// What the launcher sends to install something.
///
/// The version is chosen by the caller, not resolved from "latest" here: a mod
/// that silently moves to a newer file every time the launcher opens is the
/// behaviour nobody asks for and everybody debugs.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstallRequest {
    pub server_id: Uuid,
    #[serde(default)]
    pub kind: ContentKind,
    pub provider: String,
    pub project_id: String,
    pub version_id: String,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub icon_url: Option<String>,
}
