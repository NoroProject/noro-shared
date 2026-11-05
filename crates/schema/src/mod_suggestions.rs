//! A player's own mod suggestions, as the cabinet and the launcher show them.
//!
//! The author only ever sees their own rows, and never who decided: the
//! decision belongs to the build, not to a person the player could chase.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Where a suggestion stands.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SuggestionStatus {
    /// Nobody has looked at it yet.
    Pending,
    /// Accepted as a card in the build's optional mods, without a file yet.
    Approved,
    /// Turned down; the reason, if given, is in `decision_reason`.
    Rejected,
    /// The mod's file is in the build.
    Installed,
    /// A status this client does not know yet. Launchers in the wild are older
    /// than the master, and one new word must not break the whole list.
    #[serde(other)]
    Unknown,
}

/// One suggestion of the signed-in player.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MyModSuggestion {
    pub id: Uuid,
    pub server_id: Uuid,
    /// The project's name, so a list across projects reads without a lookup.
    #[serde(default)]
    pub server_name: String,
    #[serde(default)]
    pub build_id: Option<Uuid>,
    pub provider: String,
    pub project_id: String,
    pub title: String,
    #[serde(default)]
    pub icon_url: Option<String>,
    pub status: SuggestionStatus,
    /// Why it was turned down, as the staff member wrote it.
    #[serde(default)]
    pub decision_reason: Option<String>,
    pub created_at: DateTime<Utc>,
    #[serde(default)]
    pub decided_at: Option<DateTime<Utc>>,
}

impl SuggestionStatus {
    /// The status as the master stores it.
    pub fn parse(value: &str) -> Self {
        match value {
            "pending" => Self::Pending,
            "approved" => Self::Approved,
            "rejected" => Self::Rejected,
            "installed" => Self::Installed,
            _ => Self::Unknown,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_unknown_status_does_not_break_the_list() {
        let status: SuggestionStatus = serde_json::from_str("\"archived\"").unwrap();
        assert_eq!(status, SuggestionStatus::Unknown);
        let installed: SuggestionStatus = serde_json::from_str("\"installed\"").unwrap();
        assert_eq!(installed, SuggestionStatus::Installed);
        assert_eq!(
            SuggestionStatus::parse("rejected"),
            SuggestionStatus::Rejected
        );
    }
}
