---
title: Module settings
description: Exposing configurable options for server operators in the administration panel.
---

Modules often have parameters that instance operators need to adjust: tax rates, reward intervals, feature toggles, API keys, or currency names. Recompiling the module for every configuration change is inconvenient.

The Noro platform provides declarative settings via `#[derive(Settings)]`. You define a typed configuration struct in Rust; the master automatically generates an interactive form in the admin panel and persists values in the module's scoped store.

## Declaring settings

Derive `Settings`, `Serialize`, `Deserialize`, and `Default` on your configuration struct:

```rust
use noro_sdk::prelude::*;

#[derive(Settings, Serialize, Deserialize, Default)]
pub struct ModuleConfig {
    #[setting(label = "mod-rewards-enabled")]
    pub enabled: bool,

    #[setting(label = "mod-rewards-interval")]
    pub reward_interval_secs: i64,

    #[setting(label = "mod-rewards-currency")]
    pub currency_name: String,
}
```

### Supported field types

The `#[derive(Settings)]` macro inspects the Rust field types and automatically maps them to the appropriate admin UI control:

| Field type | UI component | Description |
|---|---|---|
| `bool` | Toggle switch | Boolean on/off switch |
| `i64`, `u64`, `i32`, `u32`, `f64` | Number input | Numeric value input |
| `String` (or other) | Text input | Single-line text input |

### Field attributes

- **`label = "key"`**: Specifies the Fluent translation key for the field title in the admin panel. If omitted, the Rust field name itself is displayed.

## Registering settings in the module

To expose your settings struct to the master and the admin panel, register it inside your `#[noro::module]` block using the `#[register]` hook:

```rust
#[noro::module]
impl RewardsModule {
    #[register]
    fn register(reg: &mut Registration) {
        ModuleConfig::register(reg);
    }

    // Other handlers (events, routes, tasks, rpc)...
}
```

`ModuleConfig::register(reg)` emits the fields and their metadata into the module's registration manifest when the module is loaded.

## Reading settings in your code

Call `ModuleConfig::load()?` whenever you need the active settings:

```rust
#[task("5m")]
fn distribute_rewards() -> Result<()> {
    let cfg = ModuleConfig::load()?;

    if !cfg.enabled {
        return Ok(());
    }

    log::info(format!(
        "Distributing rewards using currency: {}",
        cfg.currency_name
    ));

    // Payout logic...
    Ok(())
}
```

### How `load()` works

1. It reads each field by key from `store::instance()`.
2. If an operator has configured a value in the admin panel, that value is returned.
3. If no value has been set yet, `load()` transparently falls back to the field's default from `Self::default()`.

Because values are read from `store::instance()`, any changes made by operators in the admin panel take effect on the very next `load()` call—no restart or re-initialization needed.

## The operator experience

When an operator opens your module in the Admin Panel (**Modules** → select your module):

1. A **Settings** tab is shown with fields generated from your declaration.
2. Labels are translated according to the operator's active language using your module's Fluent files (`locales/en.ftl`, `locales/ru.ftl`).
3. Saving the form writes the new values into the instance store.
