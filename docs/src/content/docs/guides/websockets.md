---
title: Realtime & WebSockets
description: Full-duplex messaging between browser tabs and your module.
---

When your module powers an interactive web UI — a live auction, a casino, dynamic server monitoring, or instant player alerts — polling HTTP endpoints every few seconds wastes bandwidth and adds latency.

Noro Master maintains a single, permanent WebSocket connection (`/api/player/ws`) with every player logged into the web cabinet or admin panel. Your module plugs directly into this transport in both directions.

## Declarative Actions with `#[ws_action]`

When a player's browser tab sends a JSON frame containing an `"action"` or `"type"` field, you can route it directly to dedicated handler methods using `#[ws_action("action_name")]`:

```rust
use noro_sdk::prelude::*;

#[derive(Deserialize)]
struct BetPayload {
    amount: u64,
}

#[noro::module]
impl Casino {
    #[ws_action("place_bet")]
    fn on_bet(player: Player, Json(bet): Json<BetPayload>) -> Result<()> {
        log::info(format!("{} placed a bet of {}", player.label(), bet.amount));

        // Reply directly to this player over WebSocket:
        web_ws::send(player.id, json!({
            "type": "bet_confirmed",
            "amount": bet.amount
        }))?;
        Ok(())
    }
}
```

### Supported Extractors for `#[ws_action]`

Like `#[route]`, parameters are injected automatically using type extractors:
- `Player` — authenticated player details
- `AuthUser` — player UUID (or error)
- `OptionalUser` — optional player UUID
- `Json<T>` — deserialized payload into your typed struct `T`
- `RawParams` / `Value` — raw payload as `serde_json::Value`
- `EventCtx` — context (timestamp, origin, actor)
- `WebMessage` — the entire raw event

### Manual Event Subscription with `#[event]`

If you want to handle all incoming WebSocket messages in a single function without automatic dispatching, subscribe to the `WebMessage` event directly:

```rust
#[event]
fn on_web_message(e: WebMessage) -> Result<()> {
    log::info(format!("{} sent: {:?}", e.player.label(), e.payload));
    Ok(())
}
```

Receiving requires capability `web_ws = ["read"]` in your `noro.toml`:

```toml
[capabilities]
web_ws = ["read", "notify"]
```

## Listening and sending in the Web UI

In your mini-app (Vue component), use `noro.ws` provided by `@noroproject/module-ui`:

```ts
import { onUnmounted } from 'vue'
import { useNoro } from '@noroproject/module-ui'

const noro = useNoro()

// 1. Subscribe to incoming messages pushed by your module:
const unsubscribe = noro.ws.on<{ type: string; balance?: number }>((data) => {
    if (data.type === 'balance_updated') {
        currentBalance.value = data.balance ?? 0
    }
})

// Unsubscribe when component is unmounted:
onUnmounted(() => unsubscribe())

// 2. Send an action back to your module over WebSocket:
function sendAction() {
    noro.ws.send({
        action: 'spin_wheel',
        bet: 50
    })
}
```

No connection setup, authentication headers, or manual reconnect logic is required: `@noroproject/module-ui` shares the master's authenticated player connection.

## Pushing frames from your module

To push messages from Rust to open browser tabs without waiting for an HTTP request:

```rust
use noro_sdk::web_ws;

// Push to a specific player:
web_ws::send(player_id, json!({
    "type": "balance_updated",
    "balance": 1250
}))?;

// Broadcast to every player currently viewing the website:
web_ws::broadcast(json!({
    "type": "announcement",
    "text": "Server event started!"
}))?;
```

### Checking online status

You can check whether a player has an open browser tab before sending:

```rust
if web_ws::is_online(player_id)? {
    // Player is currently active in the web cabinet
}

let total_viewers = web_ws::online_count()?;
```

Pushing and checking status require `web_ws = ["notify"]` and `web_ws = ["read"]` in `[capabilities]`.
