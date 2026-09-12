# Noro Shared

Shared protocols, data models, internationalization, and Minecraft utilities for Noro — plus everything needed to write a **module**.

## Crates

| Crate | What it is |
|---|---|
| `schema` | Data transfer objects, protocols, and integrity models |
| `i18n` | Fluent FTL localization bundles (Russian and English) |
| `mc_mod_utils` | Mod JAR metadata & icon extraction utilities |
| `noro-module-abi` | The module boundary: events, entities, manifest. Its version **is** the ABI version |
| `noro-sdk` | What a module author writes against: `players`, `store`, `log` |
| `noro-sdk-macros` | `#[noro::module]` and the handler attributes |

`packages/module-ui` holds the type declarations for mini-app UI, and `examples/playtime-rewards` is a working module you can copy.

## Writing a module

A module is a single `.noromod` file the instance owner drops into the admin panel. It runs sandboxed (WebAssembly) **inside the master process** and reaches the platform through host functions.

```rust
use noro_sdk::prelude::*;

pub struct Greeter;

#[noro::module]
impl Greeter {
    /// Fields of the settings form, declared once at install time.
    #[register]
    fn setup(reg: &mut Registration) {
        reg.setting("per_hour", SettingKind::Number, "mod-greeter-per-hour").default(100);
    }

    /// The event name comes from the argument type — never from a string.
    #[event]
    fn on_join(e: PlayerJoined) -> Result<()> {
        if e.first_join {
            log::info(format!("{} is new here", e.player.label()));
        }
        store::user(e.player.id).set("seen", &true)?;
        Ok(())
    }

    /// Reachable at `GET /api/modules/greeter/me`.
    #[route(GET, "/me")]
    fn me(req: HttpRequest) -> Result<bool> {
        Ok(store::user(req.require_user()?).get("seen")?.unwrap_or(false))
    }
}
```

Nothing here is repeated in `manifest.toml`. That file is a passport — identity, version, required ABI, and the capabilities the module asks for:

```toml
[module]
id = "greeter"
name = "Greeter"
version = "1.0.0"
api = "1.0"

[capabilities]
players = ["read"]
store = true
```

The operator grants those capabilities at install time and may grant less than you ask. A call to something ungranted returns `CapabilityDenied` — it does not crash.

### Mini-apps

A module can bring its own UI. Declare it in `[[apps]]` and write an ordinary Vue component:

```vue
<script setup lang="ts">
import { AtomBadge, NoroCard, useNoro } from '@noro/module-ui'

const noro = useNoro()
const seen = await noro.api<boolean>('/me')
</script>

<template>
    <NoroCard icon="i-lucide-hand" :title="noro.t('mod-greeter-title')">
        <AtomBadge :tone="seen ? 'success' : 'neutral'">{{ seen }}</AtomBadge>
    </NoroCard>
</template>
```

Those are the panel's own components, so the page looks like the rest of it without a line of styling. Both `vue` and `@noro/module-ui` are external in the build, which keeps the bundle at a couple of kilobytes and — more importantly — gives the module the panel's **exact** Vue instance.

`kind = "page"` is the other option: a plain HTML page in a sandboxed iframe, talking over a `postMessage` bridge. Use it when isolation matters more than convenience.

### Building

```bash
./scripts/build-module.sh          # from the module folder → dist/<id>.noromod
./scripts/dev-module.sh            # rebuild on every save
```

For development, point the master at your folder (admin panel → Modules → Dev mode). It then reads the wasm, mini-app and locales straight from disk and reloads the module itself whenever you rebuild. **The master never needs restarting.**

### What will not compile

The `wasm32-unknown-unknown` target has no clock, no entropy, no sockets and no files. Pure computation works as is — `regex`, `rust_decimal`, `serde`, `sha2`. Anything reaching into the environment does not:

| Needs | Use instead |
|---|---|
| Network (`reqwest`) | the HTTP host function, with hosts allow-listed in the manifest |
| System clock (`chrono` with `clock`) | `noro_sdk::now()` |
| Randomness (`uuid/v4`, `rand`) | identifiers come from the master |
| Files, threads, processes | `store` and your own Postgres schema |

Dependencies with those features usually just need `default-features = false` — that is exactly what `noro-sdk` does with `uuid` and `chrono`.

## Verification

```bash
cargo check --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo fmt --all
```
