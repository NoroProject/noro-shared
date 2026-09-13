---
title: "permissions"
description: "Permissions."
---

:::note
Собирается из исходников самого SDK. Править руками бесполезно.
:::

Permissions.

| Вызов | Что делает | Нужно |
|---|---|---|
| `has(who: impl IntoPlayerRef, node: &str) -> Result<bool, ModuleError>` | Whether a player has a permission, counting their roles. | `permissions = ["read"]` |
| `has_on(who: impl IntoPlayerRef, node: &str, server_id: Uuid) -> Result<bool, ModuleError>` | The same question, on one server build. | `permissions = ["read"]` |
| `effective(who: impl IntoPlayerRef) -> Result<Vec<String>, ModuleError>` | Every permission a player effectively has, roles included. | `permissions = ["read"]` |
| `effective_on(who: impl IntoPlayerRef, server_id: Uuid) -> Result<Vec<String>, ModuleError>` | The same list, as it stands on one server build. | `permissions = ["read"]` |
| `grant(who: impl IntoPlayerRef, node: &str) -> Result<(), ModuleError>` | Grants a permission to a player personally, everywhere. | `permissions = ["read", "grant"]` |
| `grant_on(who: impl IntoPlayerRef, node: &str, server_id: Uuid) -> Result<(), ModuleError>` | Grants it on one server build only. | `permissions = ["read", "grant"]` |
| `revoke(who: impl IntoPlayerRef, node: &str) -> Result<(), ModuleError>` | Takes a personal permission away. | `permissions = ["read", "grant"]` |
| `revoke_on(who: impl IntoPlayerRef, node: &str, server_id: Uuid) -> Result<(), ModuleError>` | Takes it away on one server build. | `permissions = ["read", "grant"]` |
