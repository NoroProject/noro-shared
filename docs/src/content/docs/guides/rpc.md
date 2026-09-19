---
title: Inter-module RPC
description: Exposing methods for other modules and calling cross-module procedures directly.
---

Modules in Noro execute in isolated WebAssembly sandboxes with their own private storage and database schemas. When separate modules need to coordinate—such as a Quest module checking an item count in an Inventory module, or an Auction system verifying player balances—Inter-Module RPC provides direct, strongly typed method calls between them.

Calls happen in-process within the master server. There is no HTTP overhead, no network serialization hop, and no coupling of database schemas.

## Exposing an RPC method

To make a method callable by other modules, mark it with `#[rpc]` inside your `#[noro::module]` block:

```rust
use noro_sdk::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct DiscountReq {
    pub player_id: Uuid,
    pub original_price: u64,
}

#[derive(Serialize, Deserialize)]
pub struct DiscountRes {
    pub final_price: u64,
    pub discount_applied: u64,
}

#[noro::module]
impl LoyaltyModule {
    #[rpc("calculate_discount")]
    fn calculate_discount(Json(req): Json<DiscountReq>) -> Result<DiscountRes> {
        let discount = req.original_price / 10; // 10% discount
        Ok(DiscountRes {
            final_price: req.original_price - discount,
            discount_applied: discount,
        })
    }
}
```

### Method naming

- `#[rpc("custom_name")]`: Exposes the method under the given explicit name.
- `#[rpc]`: Uses the function's Rust identifier as the method name.

### Parameters and return values

- Parameters use the declarative `Json<T>` extractor to parse the incoming payload.
- The return type can be any type that implements `serde::Serialize`, wrapped in `Result<T>` or `Result<T, ModuleError>`.

## Calling an RPC method from another module

Use `modules::call`:

```rust
use noro_sdk::prelude::*;

let result: DiscountRes = modules::call(
    "loyalty",
    "calculate_discount",
    &DiscountReq {
        player_id: player.id,
        original_price: 1000,
    },
)?;

log::info(format!("Final price after discount: {}", result.final_price));
```

The arguments are:
1. `target: &str` — the ID of the target module as declared in its `manifest.toml`.
2. `method: &str` — the name of the exposed RPC method.
3. `payload: &P` — reference to any serializable payload.

`modules::call` returns `Result<R, ModuleError>`, automatically deserializing the response into your requested type `R`.

## Required capability

Calling other modules requires declaring the `modules` capability in your `manifest.toml`:

```toml
[capabilities]
modules = ["call"]
```

Without this permission, attempts to call `modules::call` will be rejected by the master server with an authorization error.

## Error handling and edge cases

| Situation | Outcome |
|---|---|
| Target module not installed or disabled | `Err(ModuleError::not_found("module not found"))` |
| Method not found on target module | `Err(ModuleError::not_found("route not found"))` |
| Calling module lacks `modules = ["call"]` capability | `Err(ModuleError::permission_denied(...))` |
| Target handler returns `Err(ModuleError)` | Returns the target's error to caller |
| Target handler panics | Returns `Err(ModuleError)` and logs a failure for the target module |

Because the target module might not be installed or enabled on every instance, callers should handle `ModuleError::not_found` gracefully when a dependency is optional.
