---
title: "roster"
description: "Who is in game right now."
---

Who is in game right now.

| Call | What it does | Needs |
|---|---|---|
| `online() -> Result<Vec<OnlinePlayer>, ModuleError>` | Everyone in game, across every server. | `roster = ["read"]` |
| `locate(who: impl IntoPlayerRef) -> Result<Option<Uuid>, ModuleError>` | Which game server a player is on, if any. | `roster = ["read"]` |
| `is_online(who: impl IntoPlayerRef) -> Result<bool, ModuleError>` | Whether a player is in game at all. | `roster = ["read"]` |
