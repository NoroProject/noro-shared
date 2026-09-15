---
title: "events"
description: "Events of your own, for other modules to handle."
---

Events of your own, for other modules to handle.

| Call | What it does | Needs |
|---|---|---|
| `emit<T: Serialize>(name: &str, payload: T) -> Result<(), ModuleError>` | Announces something. Handlers run after your call returns, not during it. | `events = ["emit"]` |
| `emit_on<T: Serialize>(name: &str, server_id: Uuid, payload: T) -> Result<(), ModuleError>` | The same, about one server build. | `events = ["emit"]` |
