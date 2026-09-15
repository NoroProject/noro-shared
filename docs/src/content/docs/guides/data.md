---
title: Storing data
description: The scoped key-value store, and the state of your own Postgres schema.
---

## The key-value store

Your module gets a store with three scopes. The scope is part of the key rather than a
filter, so a player's `points` and a server's `points` never collide and you never splice
identifiers into strings yourself.

```rust
store::instance().set("last_payout", &now())?;
store::server(server_id).set("cfg", &ShopCfg { tax: 5 })?;
store::user(player_id).incr("points", 10)?;

let cfg: Option<ShopCfg> = store::server(server_id).get("cfg")?;
let points = store::user(player_id).get_or::<i64>("points")?;
```

Values are anything `serde` can handle. Requires `store = true`.

### Reaching it from what you are holding

```rust
#[event]
fn on_join(e: PlayerJoined) -> Result<()> {
    e.player.store().incr("joins", 1)?;
    Ok(())
}
```

`player.store()` is `store::user(player.id)` and `server.store()` is
`store::server(server.id)` — the same scope, spelled without the detour through an
identifier.

There is no `store()` on a build or a game server. The scopes are the instance, a server
and a player, and neither of those two is one of them: a `store()` on a game server would
have to hand back its server's scope, and two game servers of one build would then
quietly share a key that reads as if it were theirs.

### get returns None only when the key is absent

If a value is there but does not parse into the type you asked for, that is an error, not
a `None`. A silent `None` would be data loss you notice a week later.

### incr is one operation

```rust
let total = store::user(id).incr("points", 10)?;
```

Not a read followed by a write. Event handlers can run at the same time, and a
read-add-write trio would lose accruals. If what is under the key is not a number, the
call fails rather than overwriting it.

### Listing

```rust
for (key, value) in store::instance().list("payout:", 100, 0)? {
    // …
}
```

By prefix, in pages, capped at 500 per call. There is no unpaginated read — of anything,
anywhere in this platform.

### Lifetime

Data under `user` goes when the player does; under `server`, when the server build does.
Everything goes when your module is uninstalled.

## Your own Postgres schema

Ship `migrations/0001_init.sql` in the package and the master creates a schema
`mod_<your_id>` and applies them, tracking what it has applied. On uninstall the schema is
dropped with everything in it.

```sql
-- migrations/0001_init.sql
CREATE TABLE orders (
    id          bigserial PRIMARY KEY,
    player_id   uuid NOT NULL,
    total       bigint NOT NULL,
    paid_at     timestamptz
);
```

Then query them:

```rust
db::execute(query!(
    "INSERT INTO orders (player_id, total) VALUES ($1, $2)",
    player.to_string(),
    500
))?;

// Strongly-typed row deserialization:
let orders: Vec<Order> = db::query_as(query!(
    "SELECT * FROM orders WHERE paid_at > $1",
    since
))?;

// Fetch single row or scalar:
let latest: Option<Order> = db::one_as(query!("SELECT * FROM orders ORDER BY id DESC LIMIT 1"))?;
let count: Option<i64> = db::scalar(query!("SELECT count(*) FROM orders"))?;
```

Rows come back as objects keyed by column name, not as positional arrays: a query that
grows a column should not silently shift every index in the code reading it.

### Values are bound, never spliced

`$1`, `$2`, … and `.bind(…)`. Assembling a statement with `format!` works right up to the
first username with an apostrophe in it, and that is the good case.

JSON types map to Postgres as you would expect — a string to `text`, an integer to
`bigint`, a float to `double precision`, a bool to `boolean`, null to NULL — and an
object or an array to `jsonb`.

### What you cannot reach

Queries run with the search path pinned to your schema and under a restricted Postgres
role with no rights anywhere else. `SELECT * FROM users` is refused **by the database**,
not by a check somebody has to remember to write:

```text
ERROR: permission denied for table users
```

Platform data comes from the typed domains, where capabilities are checked and the
projection is deliberate. If the role could not be created — some managed Postgres will
not allow it — the master says so at startup and the search path is the only boundary
left; it does not pretend otherwise.

### Limits

A query returning more than ten thousand rows is refused rather than returned: the rows
travel into the sandbox's memory, and `SELECT *` over a million-row table would take the
module down with it. Add a `LIMIT`.

## Migrations are checksummed

Editing a migration that has already been applied is how you stop a module from loading.
Add another file instead. This is the same rule the master's own migrations follow, for
the same reason.

## Module Settings with `#[derive(Settings)]`

When declaring settings for operators to configure in the admin panel, define a struct with `#[derive(Settings)]`:

```rust
#[derive(Settings, Serialize, Deserialize, Default)]
pub struct ModuleConfig {
    #[setting(label = "mod-rewards-interval", type = "number", min = 1, max = 3600)]
    pub reward_interval_secs: i64,

    #[setting(label = "mod-rewards-enabled", type = "toggle")]
    pub enabled: bool,
}
```

Call `ModuleConfig::load()?` in your handlers or scheduled tasks to load the operator's current values directly from the store.

## In-memory TTL Cache

When you need temporary, volatile storage with automatic expiration (e.g. rate-limit tokens, session states, computed analytics), use `cache`:

```rust
use noro_sdk::prelude::*;

// Cache any serializable struct with a TTL in seconds:
cache::set("verify_token:12345", &VerifyData { user_id, attempts: 0 }, 300)?; // 5 minutes

// Retrieve:
let cached: Option<VerifyData> = cache::get("verify_token:12345")?;

// Or retrieve with fallback:
let count: u64 = cache::get_or("global_counter")?;

// Manually delete if completed early:
cache::delete("verify_token:12345")?;
```

Requires capability:
```toml
[capabilities]
cache = ["read", "write"]
```

## Inter-Module RPC (`#[rpc]` & `modules::call`)

Modules can expose RPC methods and call other installed modules directly.

### Exposing an RPC method

Annotate a method with `#[rpc("method_name")]` (or bare `#[rpc]` to use the function's name):

```rust
#[derive(Serialize, Deserialize)]
pub struct DiscountReq {
    pub player_id: Uuid,
    pub price: u64,
}

#[noro::module]
impl ShopService {
    #[rpc("calc_discount")]
    fn calc_discount(Json(req): Json<DiscountReq>) -> Result<u64> {
        let discount = req.price / 10;
        Ok(req.price - discount)
    }
}
```

### Calling from another module

Use `modules::call`:

```rust
let final_price: u64 = modules::call(
    "shop_service",
    "calc_discount",
    &DiscountReq { player_id, price: 100 },
)?;
```

Requires capability:
```toml
[capabilities]
modules = ["call"]
```

## Testing with `noro_sdk::testing`

Write unit tests for your event handlers, WebSocket actions, or HTTP endpoints without running the master server:

```rust
#[cfg(test)]
mod tests {
    use noro_sdk::testing::*;
    use noro_sdk::prelude::*;

    #[test]
    fn test_my_handler() {
        let msg = mock_web_message("Steve", &json!({ "action": "bid", "amount": 100 }));
        assert_eq!(msg.player.label(), "Steve");

        let req = mock_request("GET", "/status", None, Some(msg.player.id));
        assert_eq!(req.method, "GET");
    }
}
```
