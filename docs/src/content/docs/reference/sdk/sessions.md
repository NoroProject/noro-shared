---
title: "sessions"
description: "A player's sessions — the launchers and browsers they are signed in from."
---

A player's sessions — the launchers and browsers they are signed in from.

| Call | What it does | Needs |
|---|---|---|
| `of(who: impl IntoPlayerRef) -> Result<Vec<Session>, ModuleError>` | Every session a player has open. | `sessions = ["read"]` |
| `revoke(who: impl IntoPlayerRef, session_id: Uuid) -> Result<bool, ModuleError>` | Closes one session. `false` — it was already gone. | `sessions = ["read", "revoke"]` |
| `revoke_all(who: impl IntoPlayerRef) -> Result<u64, ModuleError>` | Closes all of them and returns how many were closed. | `sessions = ["read", "revoke"]` |
