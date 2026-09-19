---
title: Testing modules
description: Writing fast, isolated unit tests for WASM modules with noro_sdk::testing.
---

Testing business logic in modules should not require running a full master server, spinning up PostgreSQL, or launching Minecraft clients.

The `noro_sdk::testing` module provides lightweight test fixtures and mock constructors. You can test event subscriptions, HTTP route handlers, and WebSocket actions locally using standard Rust unit tests.

## Available mock helpers

Import the testing module in your test block:

```rust
use noro_sdk::testing::*;
```

The testing module provides four primary builders:

| Helper | Returns | Purpose |
|---|---|---|
| `mock_player(name)` | `Player` | Creates a mock player with an ID, username, Minecraft UUID, and default roles |
| `mock_context()` | `EventCtx` | Creates an event context originated from `Origin::Web` with `ActorRef::System` |
| `mock_web_message(name, payload)` | `WebMessage` | Creates a realtime WebSocket message from a mock player with serialized JSON payload |
| `mock_request(method, path, body, user)` | `HttpRequest` | Creates a mock HTTP request with headers, path, JSON body, and optional user ID |

## Testing event handlers

Event handlers receive strongly typed structs. You can construct them directly using mock builders and verify the logic:

```rust
// In your module implementation
pub fn check_chat_message(text: &str) -> bool {
    !text.contains("forbidden_word")
}

#[cfg(test)]
mod tests {
    use super::*;
    use noro_sdk::testing::*;

    #[test]
    fn test_chat_filter() {
        let player = mock_player("Steve");
        assert_eq!(player.label(), "Steve");
        assert!(!player.banned);

        assert!(check_chat_message("Hello world!"));
        assert!(!check_chat_message("This has forbidden_word in it"));
    }
}
```

## Testing HTTP endpoints

You can invoke route handler functions directly by supplying an `HttpRequest` constructed via `mock_request`:

```rust
use noro_sdk::prelude::*;

pub fn handle_profile_status(req: HttpRequest) -> Result<String> {
    let user_id = req.require_user()?;
    Ok(format!("Status for user: {user_id}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use noro_sdk::testing::*;

    #[test]
    fn test_authenticated_endpoint() {
        let player = mock_player("Alex");
        let req = mock_request("GET", "/api/modules/stats/status", None, Some(player.id));

        let res = handle_profile_status(req).expect("handler should succeed");
        assert!(res.contains(&player.id.to_string()));
    }

    #[test]
    fn test_unauthenticated_fails() {
        let req = mock_request("GET", "/api/modules/stats/status", None, None);
        let err = handle_profile_status(req).expect_err("should reject guest");
        assert_eq!(err.code(), "UNAUTHORIZED");
    }
}
```

## Testing WebSocket actions

When mini-apps send events over the bridge, the master wraps them in `WebMessage`. You can simulate these browser messages using `mock_web_message`:

```rust
use noro_sdk::prelude::*;

pub fn handle_bid(msg: WebMessage) -> Result<u64> {
    let amount = msg.payload.get("amount")
        .and_then(|v| v.as_u64())
        .ok_or_else(|| ModuleError::invalid("missing amount"))?;

    Ok(amount * 2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use noro_sdk::testing::*;
    use serde_json::json;

    #[test]
    fn test_websocket_bid() {
        let msg = mock_web_message("Dalynkaa", &json!({ "action": "bid", "amount": 250 }));

        assert_eq!(msg.player.label(), "Dalynkaa");
        let result = handle_bid(msg).unwrap();
        assert_eq!(result, 500);
    }
}
```

## Running tests

Run unit tests on your development machine using standard Cargo:

```bash
cargo test
```

:::note[Host target vs WASM target]
While modules compile to `wasm32-unknown-unknown` for packaging, tests run on your native host target (`x86_64` or `aarch64`). Mocks isolate your business logic without requiring WASM host function linkages.
:::
