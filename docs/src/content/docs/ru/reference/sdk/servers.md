---
title: "servers"
description: "Server builds, their client builds, and the game servers behind them."
---

:::note
Собирается из исходников самого SDK. Править руками бесполезно.
:::

Server builds, their client builds, and the game servers behind them.

| Вызов | Что делает | Нужно |
|---|---|---|
| `list() -> Result<Vec<Server>, ModuleError>` | Every server build. | `servers = ["read"]` |
| `get(id: Uuid) -> Result<Option<Server>, ModuleError>` | One server build by identifier. | `servers = ["read"]` |
| `by_slug(slug: &str) -> Result<Option<Server>, ModuleError>` | One server build by its hub slug — the `/s/<slug>` address. | `servers = ["read"]` |
| `game_servers(server_id: Uuid) -> Result<Vec<GameServer>, ModuleError>` | The game servers of a server build. | `gameservers = ["read"]` |
| `game_server(id: Uuid) -> Result<Option<GameServer>, ModuleError>` | One game server by identifier. | `gameservers = ["read"]` |
| `set_maintenance(game_server_id: Uuid, enabled: bool) -> Result<(), ModuleError>` | Puts a game server into maintenance, or takes it out. | `gameservers = ["read", "maintenance"]` |
| `command(game_server_id: Uuid, command: impl Into<String>) -> Result<bool, ModuleError>` | Sends a console command to a game server via wrapper. | `gameservers = ["command"]` |
