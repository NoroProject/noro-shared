---
title: "telemetry"
description: "How a game server is holding up."
---

:::note
Generated from the SDK's own sources. Editing it by hand has no effect.
:::

How a game server is holding up.

| Call | What it does | Needs |
|---|---|---|
| `of(game_server_id: Uuid) -> Result<Option<Telemetry>, ModuleError>` | The latest measurement, or `None` when the agent has never reported. | `telemetry = ["read"]` |
