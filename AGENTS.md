# noro-shared — Agent Guidelines (AGENTS.md)

Detailed repository instructions: **[./INSTRUCTIONS.md](./INSTRUCTIONS.md)**.
Root project ideology and universal rules: **[../INSTRUCTIONS.md](../INSTRUCTIONS.md)**.

## Key Directives:
- **Dependency Impact:** Consumed over git by branch in both launcher and master. Changes require a push and master rebuild to propagate to the web app.
- **i18n Localization:** Always add keys to BOTH `crates/i18n/locales/en.ftl` and `ru.ftl`. Follow prefix conventions (`admin-`, `web-`, `cabinet-`, `atom-`).
- **Schema Contracts:** Keep serde serialization backwards-compatible with older deployed launcher versions.
- **No Automatic Git Push:** NEVER push to git or push tags without explicit user instruction.

## Verification:
```bash
cargo check --workspace
cargo clippy --workspace --all-targets -- -D warnings
```
