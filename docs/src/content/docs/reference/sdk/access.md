---
title: "access"
description: "Access to servers and builds."
---

:::note
Generated from the SDK's own sources. Editing it by hand has no effect.
:::

Access to servers and builds.

| Call | What it does | Needs |
|---|---|---|
| `allow_join(who: impl IntoPlayerRef, server_id: Uuid) -> Result<(), ModuleError>` | Lets a player into a server build. | `access = ["grant"]` |
| `revoke_join(who: impl IntoPlayerRef, server_id: Uuid) -> Result<(), ModuleError>` | Takes that away again. | `access = ["grant"]` |
| `allow_build(who: impl IntoPlayerRef, build_id: Uuid) -> Result<(), ModuleError>` | Lets a player download one client build. | `access = ["grant"]` |
| `revoke_build(who: impl IntoPlayerRef, build_id: Uuid) -> Result<(), ModuleError>` | Takes build access away. | `access = ["grant"]` |
| `allow_mod(who: impl IntoPlayerRef, server_id: Uuid, mod_name: &str) -> Result<(), ModuleError>` | Lets a player use one optional mod of a server. | `optional_mods = ["grant"]` |
| `revoke_mod(who: impl IntoPlayerRef, server_id: Uuid, mod_name: &str) -> Result<(), ModuleError>` | Takes that away again. | `optional_mods = ["grant"]` |
