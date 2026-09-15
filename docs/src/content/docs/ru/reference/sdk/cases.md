---
title: "cases"
description: "Moderation cases."
---

Moderation cases.

| Вызов | Что делает | Нужно |
|---|---|---|
| `get(id: Uuid) -> Result<Option<Case>, ModuleError>` | One case. | `cases = ["read"]` |
| `events(id: Uuid) -> Result<Vec<CaseEvent>, ModuleError>` | Its timeline: claims, punishments, notes, chat that was kept. | `cases = ["read"]` |
| `open_on(who: impl IntoPlayerRef) -> Result<Option<Case>, ModuleError>` | The open case on a player, if there is one. | `cases = ["read"]` |
| `claim(id: Uuid, moderator: impl IntoPlayerRef) -> Result<bool, ModuleError>` | Assigns a case to a moderator. `false` — somebody already has it. | `cases = ["read", "claim"]` |
| `resolve(id: Uuid, verdict: &str, resolution: &str, rule_code: Option<&str>) -> Result<bool, ModuleError>` | Closes a case. `verdict = "confirmed"` means the report held up. | `cases = ["read", "resolve"]` |
