---
title: "dm"
description: "Private messages between players."
---

:::note
Собирается из исходников самого SDK. Править руками бесполезно.
:::

Private messages between players.

| Вызов | Что делает | Нужно |
|---|---|---|
| `threads(who: impl IntoPlayerRef, page: i64, per_page: i64) -> Result<Vec<DmThread>, ModuleError>` | The conversations a player has, most recent first. | `dm = ["read"]` |
| `history(a: impl IntoPlayerRef, b: impl IntoPlayerRef, page: i64, per_page: i64) -> Result<Vec<DmMessage>, ModuleError>` | The messages between two players, newest first. | `dm = ["read"]` |
| `send(from: impl IntoPlayerRef, to: impl IntoPlayerRef, body: &str) -> Result<uuid::Uuid, ModuleError>` | Writes a message on a player's behalf. Returns its id. | `dm = ["send"]` |
| `presence(who: impl IntoPlayerRef) -> Result<DmPresence, ModuleError>` | Where a player is: `in_game`, `on_site`, `in_launcher` or `offline`. | `dm = ["read"]` |
| `of(who: uuid::Uuid) -> Conversations` | Their conversations by id. Prefer `player.dm()`. | — |
