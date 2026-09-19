---
title: "proxy"
description: "Velocity proxy routing, moving players, server evacuation, and titles."
---

Velocity proxy operations: moving players between subservers, server evacuations, and network titles.

| Call | What it does | Needs |
|---|---|---|
| `transfer(who, target_server_id) -> Result<bool, ModuleError>` | Transfers a player to another subserver by its game server ID. | `proxy = ["transfer"]` |
| `transfer_named(who, target_server_name) -> Result<bool, ModuleError>` | Transfers a player to another subserver by its registered name. | `proxy = ["transfer"]` |
| `evacuate(from_server, to_server, title) -> Result<u32, ModuleError>` | Evacuates all players from a subserver to another (e.g. Limbo / Lobby). | `proxy = ["manage"]` |
| `title(who, title_text, subtitle_text) -> Result<(), ModuleError>` | Displays an in-game Title to a specific player or all players via proxy. | `proxy = ["title"]` |
| `subservers(proxy_id) -> Result<Vec<GameServer>, ModuleError>` | Returns the subservers registered under this proxy. | `proxy = ["read"]` |
