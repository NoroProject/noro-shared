---
title: "restarts"
description: "Restart schedules of game servers."
---

Restart schedules of game servers.

| Call | What it does | Needs |
|---|---|---|
| `of(game_server_id: Uuid) -> Result<Vec<RestartSchedule>, ModuleError>` | The schedules of one game server. | `restarts = ["read"]` |
| `add(draft: ScheduleDraft) -> Result<RestartSchedule, ModuleError>` | Adds a schedule and returns it, with the next run already computed. | `restarts = ["read", "manage"]` |
| `remove(id: Uuid) -> Result<bool, ModuleError>` | Removes a schedule. `false` — it was not there. | `restarts = ["read", "manage"]` |
