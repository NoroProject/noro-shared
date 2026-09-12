---
title: Inside the package
description: What a .noromod contains, and the split between the manifest and your code.
---

A `.noromod` is a zip. The build script assembles it; this is what ends up inside.

```
manifest.toml          required — identity and what you ask for
module.wasm            required — your compiled handlers
web/                   mini-app assets, when you ship one
locales/{en,ru}.ftl    your text, keys prefixed mod-<id>-
migrations/NNNN_*.sql  migrations for your own Postgres schema
icon.png               128×128
```

## The manifest is a passport

It holds only what has to be known **before** your code runs:

```toml
[module]
id = "playtime-rewards"
name = "Playtime Rewards"
version = "1.2.0"
api = "1.0"                    # required ABI version; the master compares the major
scope = "instance"             # instance | server

[capabilities]
players = ["read"]
store = true

[[apps]]
placement = "cabinet"          # admin | hub | cabinet | widget
kind = "vue"                   # vue | page
entry = "app.js"               # relative to web/ inside the package
title = "mod-playtime-rewards-title"
icon = "i-lucide-gift"

[[permissions]]
node = "noro.module.playtime-rewards.view"
label = "mod-playtime-rewards-perm-view"
```

Capabilities could not come from code even in principle: reading them would mean running
the module before the operator decided it was allowed to.

## Everything else is code

Subscriptions, endpoints, tasks and the settings form are declared with attributes and
collected by `#[noro::module]` into a declaration the master fetches on install:

```rust
#[noro::module]
impl Shop {
    #[register]
    fn setup(reg: &mut Registration) {
        reg.setting("tax", SettingKind::Number, "mod-shop-tax")
            .default(5)
            .range(0, 100);
    }

    #[init]
    fn start() -> Result<()> {
        log::info("shop is up");
        Ok(())
    }

    #[event(priority = high)]
    fn on_transfer(e: BankPreTransfer) -> Result<()> { Ok(()) }

    #[route(POST, "/buy", auth = permission("noro.module.shop.buy"))]
    fn buy(req: HttpRequest) -> Result<u64> { Ok(0) }
}
```

This used to live in the manifest, and two things were wrong with it. The handler name
was repeated as a string, so a typo produced a handler that silently never ran. And the
event name was a string too — nothing stopped you from subscribing to `player.joined`
while accepting a `PlayerLeft`. Now the name comes from the argument type, and that
mistake does not compile.

| Attribute | Signature | What it does |
|---|---|---|
| `#[event]` | `fn(E) -> Result<()>` | subscribes; the name comes from `E` |
| `#[event(priority = high)]` | same | `lowest` · `low` · `normal` · `high` · `highest` · `monitor` |
| `#[route(GET, "/path")]` | `fn(HttpRequest) -> Result<T>` | an endpoint under `/api/modules/<id>/path` |
| `#[route(POST, "/p", auth = public)]` | same | `public` · `user` · `permission("node")` · `admin("node")` |
| `#[task("1h")]` | `fn() -> Result<()>` | on a schedule: `30s` · `5m` · `1h` · `2d` — see [tasks](../../guides/tasks/) |
| `#[register]` | `fn(&mut Registration)` | adds settings fields |
| `#[init]` | `fn() -> Result<()>` | one-off work when enabled |

Priority defaults to `normal` and endpoint access to `user` — a public endpoint has to be
asked for, never the result of a forgotten argument.

:::note[No autocomplete inside attributes]
Your editor does not know another macro's grammar until it is expanded, which is also
true of `#[serde(…)]` and `#[clap(…)]`. Hovering `#[noro::module]` shows the whole table
above, and a typo inside an attribute produces an error listing the accepted values.
:::

## Locales

Every key you ship must start with `mod-<id>-`. The master checks this on install, so
your keys cannot collide with the panel's or another module's. Operator edits win over
yours for the same key.
