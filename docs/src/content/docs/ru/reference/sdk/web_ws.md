---
title: "web_ws"
description: "Talking to the player's web browser tab."
---

:::note
Собирается из исходников самого SDK. Править руками бесполезно.
:::

Talking to the player's web browser tab.

| Вызов | Что делает | Нужно |
|---|---|---|
| `send(who: impl IntoPlayerRef, payload: impl Serialize) -> Result<bool, ModuleError>` | Sends a frame to one player's browser tab over WebSocket. | `web_ws = ["notify"]` |
| `broadcast(payload: impl Serialize) -> Result<bool, ModuleError>` | Sends a frame to every player who currently has a website tab open. | `web_ws = ["notify"]` |
| `is_online(who: impl IntoPlayerRef) -> Result<bool, ModuleError>` | Whether this player has a website tab open right now. | `web_ws = ["read"]` |
| `connected() -> Result<i64, ModuleError>` | How many players have a website tab open right now. | `web_ws = ["read"]` |
