---
title: "store"
description: "The module's own storage."
---

:::note
Собирается из исходников самого SDK. Править руками бесполезно.
:::

The module's own storage.

| Вызов | Что делает | Нужно |
|---|---|---|
| `instance() -> Store` | Shared across the instance: module settings, counters, flags. | — |
| `server(id: Uuid) -> Store` | Separate for each server. | — |
| `user(id: Uuid) -> Store` | Separate for each player. Removed together with the player. | — |
