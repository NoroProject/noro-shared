---
title: "tickets"
description: "Conversations with players."
---

:::note
Собирается из исходников самого SDK. Править руками бесполезно.
:::

Conversations with players.

| Вызов | Что делает | Нужно |
|---|---|---|
| `queue(page: i64, per_page: i64) -> Result<Vec<Ticket>, ModuleError>` | The staff queue: everything not closed, newest activity first. | `tickets = ["read"]` |
| `get(id: Uuid) -> Result<Option<Ticket>, ModuleError>` | One conversation. | `tickets = ["read"]` |
| `messages(id: Uuid) -> Result<Vec<TicketMessage>, ModuleError>` | The messages of a conversation, oldest first. | `tickets = ["read"]` |
| `reply(id: Uuid, content: &str) -> Result<(), ModuleError>` | Writes a reply, signed with your module. | `tickets = ["read", "reply"]` |
| `open(who: impl IntoPlayerRef, subject: &str, content: &str) -> Result<Uuid, ModuleError>` | Opens a conversation with a player and returns its identifier. | `tickets = ["read", "reply"]` |
| `close(id: Uuid) -> Result<(), ModuleError>` | Closes a conversation. | `tickets = ["read", "reply"]` |
