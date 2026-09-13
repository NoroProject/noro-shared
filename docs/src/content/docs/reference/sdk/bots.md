---
title: "bots"
description: "A face of your own to write from."
---

:::note
Generated from the SDK's own sources. Editing it by hand has no effect.
:::

A face of your own to write from.

| Call | What it does | Needs |
|---|---|---|
| `ensure(name: &str) -> Result<Player, ModuleError>` | Your bot by that name, created on first use. | `bots = ["create"]` |
| `list() -> Result<Vec<Player>, ModuleError>` | Every bot your module has. | `bots = ["create"]` |
