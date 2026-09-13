---
title: "store"
description: "The module's own storage."
---

:::note
Generated from the SDK's own sources. Editing it by hand has no effect.
:::

The module's own storage.

| Call | What it does | Needs |
|---|---|---|
| `instance() -> Store` | Shared across the instance: module settings, counters, flags. | — |
| `server(id: Uuid) -> Store` | Separate for each server. | — |
| `user(id: Uuid) -> Store` | Separate for each player. Removed together with the player. | — |
