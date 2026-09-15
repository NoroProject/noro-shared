---
title: "agent"
description: "Things done to a player in the running game."
---

Things done to a player in the running game.

| Вызов | Что делает | Нужно |
|---|---|---|
| `tell(who: impl IntoPlayerRef, message: &str) -> Result<bool, ModuleError>` | Sends a private message to a player. | `agent = ["tell"]` |
| `announce(message: &str) -> Result<(), ModuleError>` | Announces something to everyone in game. | `agent = ["announce"]` |
| `announce_on(server_id: Uuid, message: &str) -> Result<(), ModuleError>` | The same, to one server build only. | `agent = ["announce"]` |
| `kick(who: impl IntoPlayerRef, reason: &str) -> Result<bool, ModuleError>` | Throws a player out of the game with a reason they will see. | `agent = ["kick"]` |
