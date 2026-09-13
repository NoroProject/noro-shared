---
title: "bank"
description: "The hub's bank."
---

:::note
Generated from the SDK's own sources. Editing it by hand has no effect.
:::

The hub's bank.

| Call | What it does | Needs |
|---|---|---|
| `account(server_id: Uuid, who: impl IntoPlayerRef) -> Result<Option<Account>, ModuleError>` | The account a player would call theirs — their primary card. | `bank = ["read"]` |
| `accounts(server_id: Uuid, who: impl IntoPlayerRef) -> Result<Vec<Account>, ModuleError>` | Every account a player holds on this server. | `bank = ["read"]` |
| `balance(server_id: Uuid, who: impl IntoPlayerRef) -> Result<i64, ModuleError>` | What a player has, in the smallest unit. Zero when they have no account. | `bank = ["read"]` |
| `treasury(server_id: Uuid) -> Result<Account, ModuleError>` | The server's own account — where rewards come from and fees go. | `bank = ["read"]` |
| `transfer(server_id: Uuid, from: Uuid, to: Uuid, amount: i64, comment: &str) -> Result<i64, ModuleError>` | Moves money. Returns the transaction id. | `bank = ["read", "transfer"]` |
| `transfer_once(server_id: Uuid, from: Uuid, to: Uuid, amount: i64, comment: &str, key: &str) -> Result<i64, ModuleError>` | The same, but safe to repeat. | `bank = ["read", "transfer"]` |
