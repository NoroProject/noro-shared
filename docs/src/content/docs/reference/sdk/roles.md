---
title: "roles"
description: "Roles."
---

:::note
Generated from the SDK's own sources. Editing it by hand has no effect.
:::

Roles.

| Call | What it does | Needs |
|---|---|---|
| `list() -> Result<Vec<Role>, ModuleError>` | Every role on the instance. | `roles = ["read"]` |
| `get(role: impl IntoRoleRef) -> Result<Option<Role>, ModuleError>` | One role, by machine name or identifier. | `roles = ["read"]` |
| `of(who: impl IntoPlayerRef) -> Result<Vec<Role>, ModuleError>` | The roles a player holds. | `roles = ["read"]` |
| `grant(who: impl IntoPlayerRef, role: impl IntoRoleRef) -> Result<(), ModuleError>` | Gives a player a role. Doing it twice is not an error. | `roles = ["read", "grant"]` |
| `revoke(who: impl IntoPlayerRef, role: impl IntoRoleRef) -> Result<(), ModuleError>` | Takes a role away. Revoking one the player does not have is not an error. | `roles = ["read", "grant"]` |
| `create(draft: RoleDraft) -> Result<uuid::Uuid, ModuleError>` | Creates a role and returns its identifier. | `roles = ["read", "manage"]` |
| `update(id: uuid::Uuid, draft: RoleDraft) -> Result<(), ModuleError>` | Rewrites a role. Every field is replaced, not merged. | `roles = ["read", "manage"]` |
| `delete(id: uuid::Uuid) -> Result<(), ModuleError>` | Deletes a role. Players holding it simply stop holding it. | `roles = ["read", "manage"]` |
