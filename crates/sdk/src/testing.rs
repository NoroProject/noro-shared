//! Helpers for unit testing module functions without running the master server.

use noro_module_abi::context::{ActorRef, EventCtx, Origin};
use noro_module_abi::events::player::WebMessage;
use noro_module_abi::http::HttpRequest;
use noro_module_abi::player::Player;
use serde::Serialize;
use uuid::Uuid;

/// Constructs a mock [`Player`] for tests.
pub fn mock_player(name: &str) -> Player {
    Player {
        id: Uuid::from_u128(0x0123_4567_89ab_cdef_0123_4567_89ab_cdef),
        name: Some(name.to_string()),
        mc_uuid: Some(Uuid::from_u128(0xfeff_0000_1111_2222_3333_4444_5555_6666)),
        discord_id: None,
        roles: vec!["player".to_string()],
        banned: false,
        created_at: chrono::DateTime::from_timestamp(1_700_000_000, 0).unwrap_or_default(),
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
        at: chrono::DateTime::from_timestamp(1_700_000_000, 0).unwrap_or_default(),
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
