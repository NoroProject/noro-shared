//! WebSocket for a signed-in player in the browser.
//!
//! A third protocol rather than a reuse of the other two, and for the same
//! reason they are separate from each other. The launcher's frames ask a
//! machine to do things — collect logs, apply a remote action — and handing
//! those to a browser tab would undo the point of asking the machine. The admin
//! panel's frames are for staff. This one is for the person the messages belong
//! to.
//!
//! Unlike the admin protocol, frames here carry their payload. There a frame is
//! a nudge and the page refetches, because permissions are checked on the
//! refetch; here the socket is already the recipient's own, there is nothing
//! further to check, and a chat that does a round trip per message feels like
//! one.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "t", content = "d")]
pub enum PlayerWsClientMsg {
    /// The same session token the REST endpoints take.
    Authenticate {
        access_token: String,
    },
    /// Frame from the player's web tab targeted to a module on the master.
    ModuleMessage {
        module: String,
        payload: serde_json::Value,
    },
    Ping,
}

/// Where a player is, as the interface shows it.
///
/// Lives here rather than in the master because it is a wire contract: the
/// browser draws a dot from it, and a second definition would drift from the
/// one that fills it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Presence {
    /// In game, on a particular game server.
    InGame {
        game_server_id: Uuid,
    },
    /// A browser tab is open.
    OnSite,
    /// The launcher is running, the game is not.
    InLauncher,
    Offline,
}

impl Presence {
    pub fn is_online(&self) -> bool {
        !matches!(self, Presence::Offline)
    }
}

/// One message, as it arrives live.
///
/// Carries where it was written from: an instance has several game servers, and
/// "where this was said" is part of what was said.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectMessage {
    pub id: Uuid,
    pub thread_id: Uuid,
    pub author_id: Uuid,
    pub author_name: String,
    pub body: String,
    /// `game` · `site` · `module`.
    pub source: String,
    /// The game server's name, when it came from one.
    #[serde(default)]
    pub game_server_name: Option<String>,
    /// The module that sent it on somebody's behalf, if one did.
    #[serde(default)]
    pub via_module: Option<String>,
    pub at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "t", content = "d")]
pub enum PlayerWsMsg {
    /// Authenticated. The unread count comes with it so the badge is right
    /// before the first fetch finishes.
    AuthOk {
        unread: i64,
    },
    AuthFail,
    /// Somebody wrote to you.
    DmReceived {
        message: DirectMessage,
    },
    /// A notification arrived. The unread count rides along so the bell is
    /// right without a second request.
    NotificationReceived {
        notification: crate::notifications::Notification,
        unread: i64,
        /// Показать ли системную плашку браузера.
        ///
        /// Решает мастер, а не клиент: он один видит и вкладку, и лаунчер, и
        /// потому может отдать право показа ровно одному из них. Иначе человек
        /// с открытым сайтом и запущенным лаунчером получал бы две одинаковые
        /// плашки на каждое срочное событие.
        #[serde(default)]
        os_toast: bool,
    },
    /// The other side read what you had sent them.
    DmRead {
        thread_id: Uuid,
    },
    /// Somebody you can see changed where they are.
    PresenceChanged {
        user_id: Uuid,
        presence: Presence,
    },
    /// Frame from an installed module to the player's web tab.
    ModuleMessage {
        module: String,
        payload: serde_json::Value,
    },
    Pong,
}

impl PlayerWsClientMsg {
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).expect("PlayerWsClientMsg сериализуется")
    }
}

impl PlayerWsMsg {
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).expect("PlayerWsMsg сериализуется")
    }
}
