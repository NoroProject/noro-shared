---
title: Методы на сущностях
description: Те же доменные вызовы — от того, что уже в руках.
---

## Методы на сущностях

Методы на том, что уже в руках. Каждый ведёт в доменную функцию того же имени — та же возможность, то же поведение. Prelude их импортирует.

```rust
let player = players::require("Dalynkaa")?;
player.punish().server_ban(server_id, "гриф", Some(7 * 24 * 3600))?;
```

### `Player`

What you can do to a player you are holding.

| Метод | Что делает |
|---|---|
| `profile() -> player::Profile` | Their account: name, skin, cape, closing it. |
| `roles() -> player::Roles` | Their roles. |
| `perms() -> player::Perms` | Their permissions. |
| `access() -> player::Access` | Which servers they are let onto. |
| `punish() -> player::Punish` | Punishments — the kind they read and can appeal. |
| `bank(server_id: Uuid) -> player::Bank` | Their money on one build. |
| `in_game() -> player::InGame` | Them in game: tell, kick, ask if they are there. |
| `launcher() -> player::Launcher` | Their launcher. |
| `web_ws() -> player::WebWs` | Their website browser tab. |
| `logins() -> player::Logins` | How they sign in and where they are signed in. |
| `dm() -> crate::dm::Conversations` | Their private messages: `player.dm().send_to(other, "…")`. |
| `presence() -> Result<DmPresence, ModuleError>` | Where they are: in game, on the site, in the launcher, nowhere. |
| `store() -> crate::store::Store` | Their corner of your storage. |

### `Server`

What you can do with a server you are holding.

| Метод | Что делает |
|---|---|
| `builds() -> server::Builds` | Its client builds. |
| `game_servers() -> server::GameServers` | Its game servers, and announcements to them. |
| `treasury() -> server::Treasury` | Its money. |
| `hub() -> server::Hub` | Its hub: the feed and the members. |
| `store() -> crate::store::Store` | This server's corner of your storage. |

### `Build`

What you can do with a build you are holding.

| Метод | Что делает |
|---|---|
| `files() -> server::Files` | What is inside it. |
| `access() -> server::BuildAccess` | Who may download it. |
| `unpublish() -> Result<(), ModuleError>` | Stops the launcher handing it out. |

### `GameServer`

What you can do with a game server you are holding.

| Метод | Что делает |
|---|---|
| `set_maintenance(enabled: bool) -> Result<(), ModuleError>` | Puts it into maintenance, or takes it out. |
| `telemetry() -> Result<Option<noro_module_abi::ops::Telemetry>, ModuleError>` | Its last telemetry. |

### `Account`

What you can do with an account you are holding.

| Метод | Что делает |
|---|---|
| `transfer_to(to: &Account, amount: i64, comment: &str) -> Result<i64, ModuleError>` | Moves money to another account. |
| `transfer_once_to(to: &Account, amount: i64, comment: &str, key: &str) -> Result<i64, ModuleError>` | The same, but a repeat with the same key pays once. |

### `Punishment`

What you can do with a punishment you are holding.

| Метод | Что делает |
|---|---|
| `lift() -> Result<bool, ModuleError>` | Lifts it. |

### `Role`

What you can do with a role you are holding.

| Метод | Что делает |
|---|---|
| `grant_to(who: impl noro_module_abi::player::IntoPlayerRef)` | Gives it to a player. |
| `revoke_from(who: impl noro_module_abi::player::IntoPlayerRef) -> Result<(), ModuleError>` | Takes it from a player. |
| `delete() -> Result<(), ModuleError>` | Deletes it. |

## Хендлы

Что возвращает каждая группа. `player.punish()` отдаёт `Punish`, и глаголы живут внутри него.

### `Profile`

The account itself: name, skin, cape, and shutting it down.

| Метод | Что делает |
|---|---|
| `ban(reason: Option<&str>) -> Result<(), ModuleError>` | Closes the account. |
| `unban() -> Result<(), ModuleError>` | Lifts the account ban. |
| `rename(username: &str) -> Result<(), ModuleError>` | Renames them. Refused if the name is taken. |
| `set_skin(url: Option<&str>, slim: bool) -> Result<(), ModuleError>` | Sets the skin; `slim` travels with it. |
| `set_cape(cape_id: Option<Uuid>) -> Result<(), ModuleError>` | Puts a cape on, or takes it off with `None`. |
| `presets() -> Result<Vec<SkinPreset>, ModuleError>` | Their saved skin presets. |

### `Roles`

Their roles.

| Метод | Что делает |
|---|---|
| `list() -> Result<Vec<Role>, ModuleError>` | Which roles they hold. |
| `grant(role: impl IntoRoleRef) -> Result<(), ModuleError>` | Grants a role by name or id. |
| `revoke(role: impl IntoRoleRef) -> Result<(), ModuleError>` | Takes a role away. |

### `Perms`

Permissions: checking them, and granting one personally.

| Метод | Что делает |
|---|---|
| `has(node: &str) -> Result<bool, ModuleError>` | Whether they effectively hold it, roles included. |
| `has_on(node: &str, server_id: Uuid) -> Result<bool, ModuleError>` | The same on one server build. |
| `grant(node: &str) -> Result<(), ModuleError>` | Grants it to them personally, beside their roles. |
| `revoke(node: &str) -> Result<(), ModuleError>` | Takes a personal permission away. |

### `Access`

Access to builds: who is let onto a server.

| Метод | Что делает |
|---|---|
| `allow_join(server_id: Uuid) -> Result<(), ModuleError>` | Lets them into a server build. |
| `revoke_join(server_id: Uuid) -> Result<(), ModuleError>` | Takes that away. |

### `Punish`

Punishments: the kind a player reads and can appeal.

| Метод | Что делает |
|---|---|
| `ban(reason: &str, seconds: Option<i64>) -> Result<Punishment, ModuleError>` | A ban with a reason and a term. |
| `mute(reason: &str, seconds: Option<i64>) -> Result<Punishment, ModuleError>` | Mutes them. |
| `warn(reason: &str) -> Result<Punishment, ModuleError>` | Warns them. |
| `server_ban(server_id: Uuid, reason: &str, seconds: Option<i64>) -> Result<Punishment, ModuleError>` | Bans them from one server only. |
| `active() -> Result<Vec<Punishment>, ModuleError>` | Their punishments in force. |

### `Bank`

Their money on one server build.

| Метод | Что делает |
|---|---|
| `account() -> Result<Option<Account>, ModuleError>` | Their account. |
| `balance() -> Result<i64, ModuleError>` | What is on it, in the smallest unit. |

### `InGame`

The player in game: say something, throw them out, ask if they are there.

| Метод | Что делает |
|---|---|
| `tell(message: &str) -> Result<bool, ModuleError>` | A private line in chat. `false` — they are not in game. |
| `kick(reason: &str) -> Result<bool, ModuleError>` | Throws them off the server. Not a punishment: it leaves no record. |
| `online() -> Result<bool, ModuleError>` | Whether they are in game right now. |

### `Launcher`

Their launcher.

| Метод | Что делает |
|---|---|
| `online() -> Result<bool, ModuleError>` | Whether it is connected. |
| `send(payload: impl serde::Serialize) -> Result<bool, ModuleError>` | Sends it a frame. |

### `WebWs`

Their browser website tab.

| Метод | Что делает |
|---|---|
| `online() -> Result<bool, ModuleError>` | Whether they have a website tab open. |
| `send(payload: impl serde::Serialize) -> Result<bool, ModuleError>` | Sends a frame to their open website tab. |

### `Logins`

How they sign in, and where they are signed in.

| Метод | Что делает |
|---|---|
| `identities() -> Result<Vec<Identity>, ModuleError>` | Their linked logins. |
| `sessions() -> Result<Vec<Session>, ModuleError>` | Their sessions in the panel and the launcher. |
| `revoke_all() -> Result<u64, ModuleError>` | Ends every session they have. |

### `Builds`

The client builds of one server.

| Метод | Что делает |
|---|---|
| `list() -> Result<Vec<Build>, ModuleError>` | All of them. |
| `published() -> Result<Option<Build>, ModuleError>` | The one players are getting. |

### `Hub`

The hub of one server: its feed and its members.

| Метод | Что делает |
|---|---|
| `feed(page: i64) -> Result<HubPage, ModuleError>` | Its feed, paginated. |
| `members(page: i64) -> Result<HubPage, ModuleError>` | Its members, paginated. |

### `Files`

What is inside a build.

| Метод | Что делает |
|---|---|
| `list() -> Result<Vec<BuildFile>, ModuleError>` | Everything inside it. |
| `read(path: &str) -> Result<Option<String>, ModuleError>` | The text of one file. |
| `write(path: &str, text: &str) -> Result<BuildFile, ModuleError>` | Writes a text file into it. |
| `attach(path: &str, sha1: &str) -> Result<BuildFile, ModuleError>` | Puts an already-stored file in by hash. |
| `remove(path: &str) -> Result<bool, ModuleError>` | Removes a file. |

### `BuildAccess`

Who may download a build.

| Метод | Что делает |
|---|---|
| `allow(who: impl IntoPlayerRef) -> Result<(), ModuleError>` | Lets one player download it. |

### `Treasury`

The money of one server.

| Метод | Что делает |
|---|---|
| `account() -> Result<Account, ModuleError>` | Its treasury account. |

### `GameServers`

The game servers of one build.

| Метод | Что делает |
|---|---|
| `list() -> Result<Vec<GameServer>, ModuleError>` | All of them. |
| `announce(message: &str) -> Result<(), ModuleError>` | An announcement to everybody on the server. |
