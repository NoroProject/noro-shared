# noro-shared — Project Instructions

This is the single instruction file for `noro-shared`.
Shared ideology, design tokens, and universal rules: **[../INSTRUCTIONS.md](../INSTRUCTIONS.md)**.
Short agent cheatsheet: **[./AGENTS.md](./AGENTS.md)** / **[./CLAUDE.md](./CLAUDE.md)**.

---

## 1. Role & Architecture

`noro-shared` is a **public repository** containing shared Rust crates consumed by both the launcher (`noro-launcher`) and the master server (`noro-server`).

| Crate | Path | Responsibility |
|---|---|---|
| `schema` | `crates/schema` | Wire contract, shared API data types, pagination envelopes (`Page<T>`) |
| `i18n` | `crates/i18n` | Fluent translation catalogues (`en.ftl`, `ru.ftl`) and formatting helpers |
| `mc_mod_utils` | `crates/mc_mod_utils` | Minecraft mod jar parsing, metadata extraction (Fabric, Forge, NeoForge) |

### 1.1 Dependency & Distribution Model
- Both `noro-launcher` and `noro-server` consume `noro-shared` over **git by branch**.
- **Changes here do not reach other projects automatically.** They reach them only after a git commit, push, and a `cargo update` in the consuming repository.
- Master additionally embeds translation catalogues into its binary, which are then served to the Nuxt web application.

---

## 2. Crates Breakdown

### 2.1 `crates/i18n` (Translation Catalogues)

All user-facing UI text, labels, button titles, input placeholders, error messages, and field hints across all Noro applications live in this crate.

#### Catalogue Rules:
1. **Always edit both files:** `crates/i18n/locales/en.ftl` AND `crates/i18n/locales/ru.ftl`.
   - If a key is present in only one language, `i18n::t` silently returns the raw key name (e.g. `admin-set-description`). A missing translation is considered a bug.
2. **Key Naming Convention:** Keys are prefixed hierarchically by domain area:
   - `admin-…` — Admin panel and staff tools
   - `web-…` — Public site and common web UI
   - `cabinet-…` — Player cabinet (skins, capes, authorized apps)
   - `atom-…` — Design system atom components
   - `launcher-…` — Desktop launcher UI and status messages
3. **Placeholders:** Fluent variables use `{ $var }` syntax and are passed via `t_args`:
   ```fluent
   admin-user-banned = User { $username } has been suspended.
   ```
4. **Web Propagation Delay:**
   - The Nuxt 3 web panel fetches translations dynamically from master via `/api/launcher/locales/{locale}`.
   - Therefore, a newly added key will display as its raw key string in the browser until `noro-shared` is pushed to git and the master server is rebuilt and restarted. Always keep this in mind during frontend development.

---

### 2.2 `crates/schema` (Wire Contract)

The `schema` crate defines the exact types serialized over the wire between:
- Master server ↔ Desktop launcher (REST + WebSockets)
- Master server ↔ Nuxt web frontend (REST)
- Master server ↔ Admin CLI (`noro-admin`)

#### Contract Rules:
1. **Shared means contract:** Types here represent public agreements. Never add fields that are only used internally by master or launcher. Master-only DB models belong in `noro-server/crates/master/src/db/`.
2. **Serde backwards compatibility:** Distributed launcher clients in the wild are older than current development branches. Do not rename or remove serialized fields without fallback aliases (`#[serde(alias = "...")]`).
3. **Pagination Envelope (`Page<T>`):**
   - The unified pagination struct `Page<T>` lives in `crates/schema/src/page.rs`:
     ```rust
     pub struct Page<T> {
         pub items: Vec<T>,
         pub total: i64,
     }
     ```
   - It provides `Page::new(items, total)`, `Page::whole(items)`, and `page.map(f)`. All collection endpoints in master return this envelope.

---

### 2.3 `crates/mc_mod_utils` (Minecraft Mod Utilities)

Provides robust inspection of Minecraft mod archives:
- Reads `fabric.mod.json`, `mods.toml`, `mcmod.info`, and NeoForge manifests directly from jar files.
- Extracts mod IDs, versions, display names, loaders, environments (client/server), and dependencies without executing untrusted JVM code.

---

## 3. Development & Verification

Every change in `noro-shared` must pass workspace checks cleanly:

```bash
cargo check --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo fmt --all
```

> [!IMPORTANT]
> When modifying `schema` or `i18n`, verify that existing consumers (`noro-launcher` and `noro-server`) will compile cleanly before pushing.
