---
title: "telemetry"
description: "How a game server is holding up."
---

How a game server is holding up.

| Вызов | Что делает | Нужно |
|---|---|---|
| `of(game_server_id: Uuid) -> Result<Option<Telemetry>, ModuleError>` | The latest measurement, or `None` when the agent has never reported. | `telemetry = ["read"]` |
