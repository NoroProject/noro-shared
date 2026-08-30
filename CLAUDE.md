# noro-shared — Agent Guidelines (CLAUDE.md)

Shared crates repository: `schema`, `i18n`, `mc_mod_utils`. Public repository.

Detailed repository instructions: **[./INSTRUCTIONS.md](./INSTRUCTIONS.md)**.
Root project ideology and universal rules: **[../INSTRUCTIONS.md](../INSTRUCTIONS.md)**.

## Quick Reference
- **i18n:** Always add keys to both `crates/i18n/locales/en.ftl` and `ru.ftl`. Web UI will show raw key until git push + master rebuild.
- **schema:** Wire contract between master and launcher/admin-cli. Maintain serde backwards compatibility; no master-only fields.
- **mc_mod_utils:** Mod parsing and metadata extraction.

## Verification Commands
```bash
cargo check --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo fmt --all
```
