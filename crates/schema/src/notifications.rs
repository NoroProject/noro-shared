// Over 150 lines: one notification is a record, a look and a delivery rule, and the three only make sense together.
//! Notifications: what a player is told, and how it is shown.
//!
//! Shared rather than master-only because all three clients render the same
//! record: the site draws a feed, the launcher a panel, the game a chat line.
//! A type that lives on the master and is re-described in each client drifts
//! apart silently — the field a client forgot is simply never shown.
//!
//! The text travels as a translation key with arguments, not as a finished
//! string. The sender does not know which language the recipient reads, and a
//! feed where half the lines are in the master's language is what the other
//! approach produces the first time somebody switches locale.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use uuid::Uuid;

/// What a notification is about. Decides where it may be delivered: the
/// player's settings are a grid of category against channel.
///
/// A plain string rather than a closed enum: a module names its own category
/// (`module:market`), and every such module would otherwise mean a schema
/// change in three repositories.
pub type Category = String;

/// Built-in categories. Modules add their own; these are the ones the master
/// itself sends, and the ones the settings page always shows.
pub mod categories {
    /// Somebody wrote to you.
    pub const MESSAGES: &str = "messages";
    /// Punishment, appeal, case, support ticket.
    pub const MODERATION: &str = "moderation";
    /// Your own game server: crashed, out of memory, backup failed.
    pub const HOSTING: &str = "hosting";
    /// News, a launcher release, a build update, maintenance.
    pub const PROJECT: &str = "project";

    pub const BUILT_IN: &[&str] = &[MESSAGES, MODERATION, HOSTING, PROJECT];

    /// Category of a module's own notifications.
    pub fn of_module(id: &str) -> String {
        format!("module:{id}")
    }

    /// Module id, if this category belongs to one.
    pub fn module_of(category: &str) -> Option<&str> {
        category.strip_prefix("module:")
    }
}

/// How loud the notification is.
///
/// Level changes the colour, the order and whether the launcher raises an OS
/// toast. It never overrides the player's settings: a switched-off channel
/// stays off even for `Urgent`. Punching through somebody's own setting because
/// the sender considered it important is how people stop trusting the switches.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum Level {
    #[default]
    Normal,
    Important,
    Urgent,
}

/// The picture next to the text.
///
/// Presets rather than a bare URL: the head of a player and the icon of a
/// module are resolved by the client, which already knows how to draw them and
/// does it consistently. A raw URL stays available for whoever has a real
/// picture to show.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Icon {
    /// Icon of the category. The fallback when nothing better is known.
    #[default]
    Category,
    /// A player's head, by their id.
    Player { user_id: Uuid },
    /// A module's icon, by module id.
    Module { id: String },
    /// A project server's icon.
    Server { server_id: Uuid },
    /// A panel server — drawn from its kind and name.
    PanelServer { panel_server_id: Uuid },
    /// Named icon from the client's icon set, e.g. `i-lucide-triangle-alert`.
    Named { name: String },
    /// Any image by URL.
    Image { url: String },
}

/// Who sent it. Shown as a small line under the text — «from the market
/// module», «from the hosting panel» — so an unexpected notification can be
/// traced without guessing.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Source {
    /// The master itself.
    #[default]
    System,
    /// A module, by its id.
    Module { id: String },
    /// A person — staff writing to a player, for instance.
    User { user_id: Uuid },
    /// A subsystem of the master: the hosting panel, the launcher release flow.
    Service { id: String },
}

/// A button on the card.
///
/// Only a link and a label: an action that changes something would have to be
/// repeated in all three clients and have its permissions checked again in
/// each, so a notification sends the person to the place where that action
/// already exists.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Action {
    /// Translation key of the label.
    pub label_key: String,
    /// Where it leads. A site path, or an absolute URL.
    pub href: String,
    /// Draw it as the main button of the card.
    #[serde(default)]
    pub primary: bool,
    /// Style variant: primary, secondary, ghost, outline, danger, warning.
    #[serde(default)]
    pub variant: Option<String>,
}

/// Text of a notification.
///
/// `key` is looked up in the recipient's language; `args` fill its
/// placeholders. `fallback` is for senders with no key in our catalogues —
/// modules ship their own wording and cannot add entries to ours.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Text {
    #[serde(default)]
    pub key: Option<String>,
    #[serde(default)]
    pub args: BTreeMap<String, String>,
    #[serde(default)]
    pub fallback: Option<String>,
}

impl Text {
    pub fn key(key: impl Into<String>) -> Self {
        Self {
            key: Some(key.into()),
            ..Default::default()
        }
    }

    pub fn plain(text: impl Into<String>) -> Self {
        Self {
            fallback: Some(text.into()),
            ..Default::default()
        }
    }

    pub fn arg(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.args.insert(name.into(), value.into());
        self
    }
}

/// A notification as the client receives it.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notification {
    pub id: Uuid,
    pub category: Category,
    #[serde(default)]
    pub level: Level,
    /// Short line, already translated for the recipient.
    pub title: String,
    /// Longer text, already translated. Markdown, rendered by the site.
    #[serde(default)]
    pub body: String,
    #[serde(default)]
    pub icon: Icon,
    #[serde(default)]
    pub source: Source,
    /// Who sent it, spelled out — a username for a person, a title for a
    /// module. Resolved by the master when the card is rendered: only an
    /// identifier is stored, and a name written down at send time would be the
    /// old one by the time anybody reads the card.
    #[serde(default)]
    pub source_name: Option<String>,
    /// Where the card leads when clicked.
    #[serde(default)]
    pub link: Option<String>,
    #[serde(default)]
    pub actions: Vec<Action>,
    /// How many times the same thing happened; 1 unless collapsed.
    #[serde(default = "one")]
    pub repeat_count: i32,
    #[serde(default)]
    pub read: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

fn one() -> i32 {
    1
}

/// Where a notification may go. A string, not an enum, so a new delivery method
/// is a new implementation on the master and not a change to this file, to the
/// database and to every client at once.
pub type ChannelId = String;

/// Delivery methods the master ships with.
pub mod channels {
    /// The site: the bell, the feed, the live toast.
    pub const SITE: &str = "site";
    /// The launcher window.
    pub const LAUNCHER: &str = "launcher";
    /// A chat line in the game.
    pub const GAME: &str = "game";

    pub const BUILT_IN: &[&str] = &[SITE, LAUNCHER, GAME];
}

/// One row of the player's settings: a category and the channels it may use.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryPrefs {
    pub category: Category,
    pub channels: Vec<ChannelId>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_module_category_round_trips() {
        let category = categories::of_module("market");
        assert_eq!(category, "module:market");
        assert_eq!(categories::module_of(&category), Some("market"));
        assert_eq!(categories::module_of(categories::HOSTING), None);
    }

    /// Frames are shared with clients, so the shape has to survive a round trip
    /// through JSON — a field renamed on one side goes unnoticed otherwise.
    #[test]
    fn a_notification_survives_json() {
        let original = Notification {
            id: Uuid::nil(),
            category: categories::HOSTING.into(),
            level: Level::Urgent,
            title: "Server crashed".into(),
            body: "Out of memory".into(),
            icon: Icon::Player {
                user_id: Uuid::nil(),
            },
            source: Source::Service { id: "panel".into() },
            source_name: Some("panel".into()),
            link: Some("/panel/1".into()),
            actions: vec![Action {
                label_key: "open".into(),
                href: "/panel/1".into(),
                primary: true,
                variant: None,
            }],
            repeat_count: 3,
            read: false,
            created_at: chrono::Utc::now(),
        };

        let json = serde_json::to_string(&original).expect("сериализуется");
        let back: Notification = serde_json::from_str(&json).expect("разбирается");

        assert_eq!(back.category, original.category);
        assert_eq!(back.level, Level::Urgent);
        assert_eq!(back.repeat_count, 3);
        assert!(matches!(back.icon, Icon::Player { .. }));
        assert!(matches!(back.source, Source::Service { .. }));
    }

    /// Defaults matter: the master sends a minimal record for simple events, and
    /// every client has to read it without the optional fields.
    #[test]
    fn a_minimal_record_parses() {
        let json = r#"{
            "id": "00000000-0000-0000-0000-000000000000",
            "category": "project",
            "title": "News",
            "created_at": "2026-09-18T12:00:00Z"
        }"#;

        let parsed: Notification = serde_json::from_str(json).expect("разбирается");
        assert_eq!(parsed.level, Level::Normal);
        assert_eq!(parsed.repeat_count, 1);
        assert!(parsed.body.is_empty());
        assert!(matches!(parsed.icon, Icon::Category));
    }
}
