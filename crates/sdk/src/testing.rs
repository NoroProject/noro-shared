//! Helpers for unit testing module functions without running the master server.

use chrono::Utc;
use noro_module_abi::context::{ActorRef, EventCtx, Origin};
use noro_module_abi::events::player::WebMessage;
use noro_module_abi::http::HttpRequest;
use noro_module_abi::player::Player;
use serde::Serialize;
use uuid::Uuid;

/// Constructs a mock [`Player`] for tests.
pub fn mock_player(name: &str) -> Player {
    Player {
        id: Uuid::new_v4(),
        name: Some(name.to_string()),
        mc_uuid: Some(Uuid::new_v4()),
        discord_id: None,
        roles: vec!["player".to_string()],
        banned: false,
        created_at: Utc::now(),
        first_join: false,
    }
}

/// Constructs a mock [`EventCtx`] originated from web or system.
pub fn mock_context() -> EventCtx {
    EventCtx {
        origin: Origin::Web,
        actor: ActorRef::System,
        server_id: None,
        server_slug: None,
        game_server_id: None,
        at: Utc::now(),
        depth: 0,
    }
}

/// Constructs a mock [`WebMessage`] with serialized payload.
pub fn mock_web_message<T: Serialize>(name: &str, payload: &T) -> WebMessage {
    WebMessage {
        ctx: mock_context(),
        player: mock_player(name),
        payload: serde_json::to_value(payload).expect("valid json payload"),
    }
}

/// Constructs a mock [`HttpRequest`].
pub fn mock_request(
    method: &str,
    path: &str,
    body: Option<serde_json::Value>,
    user: Option<Uuid>,
) -> HttpRequest {
    HttpRequest {
        method: method.to_uppercase(),
        path: path.to_string(),
        query: serde_json::json!({}),
        body,
        user,
    }
}
