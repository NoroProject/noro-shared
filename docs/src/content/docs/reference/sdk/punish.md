---
title: "punish"
description: "Sanctions."
---

Sanctions.

| Call | What it does | Needs |
|---|---|---|
| `ban(who: impl IntoPlayerRef, reason: &str, seconds: Option<i64>) -> Result<Punishment, ModuleError>` | Bans a player. `seconds = None` means until it is lifted. | `punish = ["issue"]` |
| `mute(who: impl IntoPlayerRef, reason: &str, seconds: Option<i64>) -> Result<Punishment, ModuleError>` | Stops a player from speaking. | `punish = ["issue"]` |
| `warn(who: impl IntoPlayerRef, reason: &str) -> Result<Punishment, ModuleError>` | Leaves a warning the player has to acknowledge. | `punish = ["issue"]` |
| `server_ban(who: impl IntoPlayerRef, server_id: Uuid, reason: &str, seconds: Option<i64>) -> Result<Punishment, ModuleError>` | Keeps a player out of one server build. | `punish = ["issue"]` |
| `issue(who: impl IntoPlayerRef, kind: PunishKind, reason: &str, seconds: Option<i64>, server_id: Option<Uuid>) -> Result<Punishment, ModuleError>` | The general form, for when the kind is computed rather than written. | `punish = ["issue"]` |
| `revoke(id: Uuid) -> Result<bool, ModuleError>` | Lifts a sanction. `false` — it was already lifted or never existed. | `punish = ["revoke"]` |
| `active(who: impl IntoPlayerRef) -> Result<Vec<Punishment>, ModuleError>` | The sanctions in force on a player right now. | `punish = ["read"]` |
| `history(who: impl IntoPlayerRef) -> Result<Vec<Punishment>, ModuleError>` | Everything ever issued to a player, lifted and expired included. | `punish = ["read"]` |
