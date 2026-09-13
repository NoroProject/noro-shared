---
title: "launcher"
description: "Talking to the launcher."
---

:::note
Generated from the SDK's own sources. Editing it by hand has no effect.
:::

Talking to the launcher.

| Call | What it does | Needs |
|---|---|---|
| `send(who: impl IntoPlayerRef, payload: impl Serialize) -> Result<bool, ModuleError>` | Sends a frame to one player's launcher. | `launcher = ["notify"]` |
| `broadcast(payload: impl Serialize) -> Result<bool, ModuleError>` | Sends a frame to every connected launcher. | `launcher = ["notify"]` |
| `is_online(who: impl IntoPlayerRef) -> Result<bool, ModuleError>` | Whether this player has a launcher open right now. | `launcher = ["read"]` |
| `connected() -> Result<i64, ModuleError>` | How many signed-in launchers are connected. | `launcher = ["read"]` |
