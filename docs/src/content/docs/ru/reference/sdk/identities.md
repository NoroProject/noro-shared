---
title: "identities"
description: "Linked logins: Discord, Twitch, Google, anything else a player signs in with."
---

:::note
Собирается из исходников самого SDK. Править руками бесполезно.
:::

Linked logins: Discord, Twitch, Google, anything else a player signs in with.

| Вызов | Что делает | Нужно |
|---|---|---|
| `of(who: impl IntoPlayerRef) -> Result<Vec<Identity>, ModuleError>` | Every login linked to a player. | `identities = ["read"]` |
| `find(provider: &str, provider_user_id: &str) -> Result<Option<Player>, ModuleError>` | The player behind an identifier on some platform, if any. | `identities = ["read"]` |
| `link(who: impl IntoPlayerRef, provider: &str, provider_user_id: &str, username: Option<&str>) -> Result<bool, ModuleError>` | Links a login to a player. `false` — that identifier is already taken. | `identities = ["read", "link"]` |
| `unlink(who: impl IntoPlayerRef, provider: &str) -> Result<bool, ModuleError>` | Unlinks a platform. `false` — it was not linked. | `identities = ["read", "link"]` |
