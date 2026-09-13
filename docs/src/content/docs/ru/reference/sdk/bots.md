---
title: "bots"
description: "A face of your own to write from."
---

:::note
Собирается из исходников самого SDK. Править руками бесполезно.
:::

A face of your own to write from.

| Вызов | Что делает | Нужно |
|---|---|---|
| `ensure(name: &str) -> Result<Player, ModuleError>` | Your bot by that name, created on first use. | `bots = ["create"]` |
| `list() -> Result<Vec<Player>, ModuleError>` | Every bot your module has. | `bots = ["create"]` |
