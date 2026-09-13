---
title: Домены SDK
description: Все вызовы SDK и то, что для каждого нужно выдать.
---

:::note
Собирается из исходников самого SDK при сборке сайта. Отстать от кода не может,
а править руками бесполезно.
:::

Домен — это один модуль `noro_sdk` и одна строка в `[capabilities]`. Действие,
которое оператор не выдал, отвечает `CapabilityDenied` с именем недостающего —
модуль от этого не падает.

Полные сигнатуры, типы и подробности — в
[rustdoc](/noro-shared/api/noro_sdk/). Описания там на английском: они живут
вместе с кодом, а он публичный.

## `access`

Access to servers and builds.

| Вызов | Что делает | Нужно |
|---|---|---|
| `allow_join(who: impl IntoPlayerRef, server_id: Uuid) -> Result<(), ModuleError>` | Lets a player into a server build. | `access = ["grant"]` |
| `revoke_join(who: impl IntoPlayerRef, server_id: Uuid) -> Result<(), ModuleError>` | Takes that away again. | `access = ["grant"]` |
| `allow_build(who: impl IntoPlayerRef, build_id: Uuid) -> Result<(), ModuleError>` | Lets a player download one client build. | `access = ["grant"]` |
| `revoke_build(who: impl IntoPlayerRef, build_id: Uuid) -> Result<(), ModuleError>` | Takes build access away. | `access = ["grant"]` |
| `allow_mod(who: impl IntoPlayerRef, server_id: Uuid, mod_name: &str) -> Result<(), ModuleError>` | Lets a player use one optional mod of a server. | `optional_mods = ["grant"]` |
| `revoke_mod(who: impl IntoPlayerRef, server_id: Uuid, mod_name: &str) -> Result<(), ModuleError>` | Takes that away again. | `optional_mods = ["grant"]` |

## `agent`

Things done to a player in the running game.

| Вызов | Что делает | Нужно |
|---|---|---|
| `tell(who: impl IntoPlayerRef, message: &str) -> Result<bool, ModuleError>` | Sends a private message to a player. | `agent = ["tell"]` |
| `announce(message: &str) -> Result<(), ModuleError>` | Announces something to everyone in game. | `agent = ["announce"]` |
| `announce_on(server_id: Uuid, message: &str) -> Result<(), ModuleError>` | The same, to one server build only. | `agent = ["announce"]` |
| `kick(who: impl IntoPlayerRef, reason: &str) -> Result<bool, ModuleError>` | Throws a player out of the game with a reason they will see. | `agent = ["kick"]` |

## `bank`

The hub's bank.

| Вызов | Что делает | Нужно |
|---|---|---|
| `account(server_id: Uuid, who: impl IntoPlayerRef) -> Result<Option<Account>, ModuleError>` | The account a player would call theirs — their primary card. | `bank = ["read"]` |
| `accounts(server_id: Uuid, who: impl IntoPlayerRef) -> Result<Vec<Account>, ModuleError>` | Every account a player holds on this server. | `bank = ["read"]` |
| `balance(server_id: Uuid, who: impl IntoPlayerRef) -> Result<i64, ModuleError>` | What a player has, in the smallest unit. Zero when they have no account. | `bank = ["read"]` |
| `treasury(server_id: Uuid) -> Result<Account, ModuleError>` | The server's own account — where rewards come from and fees go. | `bank = ["read"]` |
| `transfer(server_id: Uuid, from: Uuid, to: Uuid, amount: i64, comment: &str) -> Result<i64, ModuleError>` | Moves money. Returns the transaction id. | `bank = ["read", "transfer"]` |
| `transfer_once(server_id: Uuid, from: Uuid, to: Uuid, amount: i64, comment: &str, key: &str) -> Result<i64, ModuleError>` | The same, but safe to repeat. | `bank = ["read", "transfer"]` |

## `builds`

Client builds and the files inside them.

| Вызов | Что делает | Нужно |
|---|---|---|
| `of(server_id: Uuid) -> Result<Vec<Build>, ModuleError>` | The client builds of a server. | `builds = ["read"]` |
| `published(server_id: Uuid) -> Result<Option<Build>, ModuleError>` | The build players are currently getting, if there is one. | `builds = ["read"]` |
| `unpublish(build_id: Uuid) -> Result<(), ModuleError>` | Takes a build out of publication, so the launcher stops handing it out. | `builds = ["read", "publish"]` |
| `files(build_id: Uuid) -> Result<Vec<BuildFile>, ModuleError>` | Everything inside a build, with the hash the launcher downloads by. | `builds = ["read"]` |
| `read(build_id: Uuid, path: &str) -> Result<Option<String>, ModuleError>` | The text of one file, by its path in the build. | `builds = ["read"]` |
| `write(build_id: Uuid, path: &str, text: &str) -> Result<BuildFile, ModuleError>` | Writes a text file into the build, replacing whatever was at that path. | `builds = ["files"]` |
| `write_for(build_id: Uuid, path: &str, text: &str, side: &str) -> Result<BuildFile, ModuleError>` | The same, for one side only: `client`, `server` or `both`. | `builds = ["files"]` |
| `attach(build_id: Uuid, path: &str, sha1: &str) -> Result<BuildFile, ModuleError>` | Puts an already-stored file into the build under a path. | `builds = ["files"]` and `files = ["read"]` |
| `remove(build_id: Uuid, path: &str) -> Result<bool, ModuleError>` | Removes a file from the build. `false` means there was nothing at that path. | `builds = ["files"]` |

## `cases`

Moderation cases.

| Вызов | Что делает | Нужно |
|---|---|---|
| `get(id: Uuid) -> Result<Option<Case>, ModuleError>` | One case. | `cases = ["read"]` |
| `events(id: Uuid) -> Result<Vec<CaseEvent>, ModuleError>` | Its timeline: claims, punishments, notes, chat that was kept. | `cases = ["read"]` |
| `open_on(who: impl IntoPlayerRef) -> Result<Option<Case>, ModuleError>` | The open case on a player, if there is one. | `cases = ["read"]` |
| `claim(id: Uuid, moderator: impl IntoPlayerRef) -> Result<bool, ModuleError>` | Assigns a case to a moderator. `false` — somebody already has it. | `cases = ["read", "claim"]` |
| `resolve(id: Uuid, verdict: &str, resolution: &str, rule_code: Option<&str>) -> Result<bool, ModuleError>` | Closes a case. `verdict = "confirmed"` means the report held up. | `cases = ["read", "resolve"]` |

## `db`

Your own tables.

| Вызов | Что делает | Нужно |
|---|---|---|
| `query(q: Query) -> Result<Rows, ModuleError>` | Runs a query and returns its rows. | `db = true` |
| `execute(q: Query) -> Result<u64, ModuleError>` | Runs a statement and returns how many rows it touched. | `db = true` |
| `one(q: Query) -> Result<Option<serde_json::Map<String, serde_json::Value>>, ModuleError>` | The first row, or `None`. | `db = true` |
| `scalar<T: serde::de::DeserializeOwned>(q: Query) -> Result<Option<T>, ModuleError>` | The first column of the first row, parsed into `T`. | `db = true` |

## `events`

Events of your own, for other modules to handle.

| Вызов | Что делает | Нужно |
|---|---|---|
| `emit<T: Serialize>(name: &str, payload: T) -> Result<(), ModuleError>` | Announces something. Handlers run after your call returns, not during it. | `events = ["emit"]` |
| `emit_on<T: Serialize>(name: &str, server_id: Uuid, payload: T) -> Result<(), ModuleError>` | The same, about one server build. | `events = ["emit"]` |

## `files`

The shared file store.

| Вызов | Что делает | Нужно |
|---|---|---|
| `put(bytes: &[u8]) -> Result<StoredFile, ModuleError>` | Stores bytes and returns where they landed. | `files = ["write"]` |
| `read(sha1: &str) -> Result<Option<Vec<u8>>, ModuleError>` | Reads a file back. `None` — nothing is stored under that hash. | `files = ["read"]` |
| `exists(sha1: &str) -> Result<bool, ModuleError>` | Whether anything is stored under that hash. | `files = ["read"]` |
| `url(sha1: &str) -> Result<String, ModuleError>` | Where a stored file is served from. | `files = ["read"]` |

## `http`

Calling out to the internet.

| Вызов | Что делает | Нужно |
|---|---|---|
| `send(call: HttpCall) -> Result<HttpReply, ModuleError>` | Sends a request and waits for the answer. | the host in `http = [...]` |
| `get_json<T: for<'de> serde::Deserialize<'de>>(url: &str) -> Result<T, ModuleError>` | A `GET` whose answer is parsed as JSON. | the host in `http = [...]` |

## `hub`

A server's hub: the feed, its members, towns, market, court, petitions and

| Вызов | Что делает | Нужно |
|---|---|---|
| `feed(server_id: Uuid, page: i64) -> Result<HubPage, ModuleError>` | The feed, newest and pinned first. | `hub = ["read"]` |
| `members(server_id: Uuid, page: i64) -> Result<HubPage, ModuleError>` | The hub's members. | `hub = ["read"]` |
| `playtime(server_id: Uuid, who: impl IntoPlayerRef) -> Result<serde_json::Value, ModuleError>` | How long a player has played on this server, and when they were last seen. | `hub = ["read"]` |
| `post(server_id: Uuid, author: impl IntoPlayerRef, body: &str) -> Result<serde_json::Value, ModuleError>` | Writes a post into the feed on behalf of a player. | `hub = ["read", "post"]` |
| `towns(server_id: Uuid, page: i64) -> Result<HubPage, ModuleError>` | The towns of a server. | `towns = ["read"]` |
| `town(server_id: Uuid, id: Uuid) -> Result<Option<serde_json::Value>, ModuleError>` | One town. | `towns = ["read"]` |
| `market(server_id: Uuid, page: i64) -> Result<HubPage, ModuleError>` | Lots currently on sale. | `market = ["read"]` |
| `lot(server_id: Uuid, id: Uuid) -> Result<Option<serde_json::Value>, ModuleError>` | One lot. | `market = ["read"]` |
| `court(server_id: Uuid, page: i64) -> Result<HubPage, ModuleError>` | Court cases. | `court = ["read"]` |
| `court_case(server_id: Uuid, id: Uuid) -> Result<Option<serde_json::Value>, ModuleError>` | One court case. | `court = ["read"]` |
| `petitions(server_id: Uuid, page: i64) -> Result<HubPage, ModuleError>` | Petitions and how far along they are. | `petitions = ["read"]` |
| `sign(server_id: Uuid, petition_id: Uuid, who: impl IntoPlayerRef) -> Result<bool, ModuleError>` | Signs a petition on behalf of a player. `false` — they had already signed. | `petitions = ["read", "sign"]` |
| `unsign(server_id: Uuid, petition_id: Uuid, who: impl IntoPlayerRef) -> Result<bool, ModuleError>` | Takes a signature back. `false` — there was none. | `petitions = ["read", "sign"]` |
| `fines(server_id: Uuid, page: i64) -> Result<HubPage, ModuleError>` | The fines of a server. | `fines = ["read"]` |
| `fines_of(server_id: Uuid, who: impl IntoPlayerRef) -> Result<HubPage, ModuleError>` | The fines of one player. | `fines = ["read"]` |
| `fine(draft: FineDraft) -> Result<serde_json::Value, ModuleError>` | Issues a fine, in the name of a staff member. | `fines = ["read", "issue"]` |
| `found_town(draft: TownDraft) -> Result<serde_json::Value, ModuleError>` | Founds a town, in the name of the player who becomes its mayor. | `towns = ["read", "manage"]` |
| `list_lot(draft: LotDraft) -> Result<serde_json::Value, ModuleError>` | Lists a lot on the market, in the seller's name. | `market = ["read", "sell"]` |
| `file_claim(draft: ClaimDraft) -> Result<serde_json::Value, ModuleError>` | Files a claim in court, in the plaintiff's name. | `court = ["read", "file"]` |
| `start_petition(draft: PetitionDraft) -> Result<serde_json::Value, ModuleError>` | Starts a petition, in the name of its author. | `petitions = ["read", "create"]` |

## `identities`

Linked logins: Discord, Twitch, Google, anything else a player signs in with.

| Вызов | Что делает | Нужно |
|---|---|---|
| `of(who: impl IntoPlayerRef) -> Result<Vec<Identity>, ModuleError>` | Every login linked to a player. | `identities = ["read"]` |
| `find(provider: &str, provider_user_id: &str) -> Result<Option<Player>, ModuleError>` | The player behind an identifier on some platform, if any. | `identities = ["read"]` |
| `link(who: impl IntoPlayerRef, provider: &str, provider_user_id: &str, username: Option<&str>) -> Result<bool, ModuleError>` | Links a login to a player. `false` — that identifier is already taken. | `identities = ["read", "link"]` |
| `unlink(who: impl IntoPlayerRef, provider: &str) -> Result<bool, ModuleError>` | Unlinks a platform. `false` — it was not linked. | `identities = ["read", "link"]` |

## `instance`

Instance settings.

| Вызов | Что делает | Нужно |
|---|---|---|
| `all() -> Result<BTreeMap<String, Value>, ModuleError>` | Every setting, by key. | `instance = ["read"]` |
| `get<T: DeserializeOwned>(key: &str) -> Result<Option<T>, ModuleError>` | One setting, parsed into the type you expect. | `instance = ["read"]` |
| `set<T: Serialize>(key: &str, value: &T) -> Result<(), ModuleError>` | Writes a setting. | `instance = ["read", "write"]` |

## `launcher`

Talking to the launcher.

| Вызов | Что делает | Нужно |
|---|---|---|
| `send(who: impl IntoPlayerRef, payload: impl Serialize) -> Result<bool, ModuleError>` | Sends a frame to one player's launcher. | `launcher = ["notify"]` |
| `broadcast(payload: impl Serialize) -> Result<bool, ModuleError>` | Sends a frame to every connected launcher. | `launcher = ["notify"]` |
| `is_online(who: impl IntoPlayerRef) -> Result<bool, ModuleError>` | Whether this player has a launcher open right now. | `launcher = ["read"]` |
| `connected() -> Result<i64, ModuleError>` | How many signed-in launchers are connected. | `launcher = ["read"]` |

## `news`

The instance's news.

| Вызов | Что делает | Нужно |
|---|---|---|
| `list(page: i64, per_page: i64) -> Result<Vec<NewsItem>, ModuleError>` | A page of news, pinned items first and then newest first. | `news = ["read"]` |
| `get(id: Uuid) -> Result<Option<NewsItem>, ModuleError>` | One item by identifier. | `news = ["read"]` |
| `publish(title: &str, body: &str, preview_url: Option<&str>) -> Result<Uuid, ModuleError>` | Publishes an item and returns its identifier. | `news = ["read", "publish"]` |
| `publish_pinned(title: &str, body: &str, preview_url: Option<&str>) -> Result<Uuid, ModuleError>` | Publishes an item pinned to the top. | `news = ["read", "publish"]` |
| `edit(id: Uuid, title: &str, body: &str, preview_url: Option<&str>, pinned: bool) -> Result<(), ModuleError>` | Rewrites an existing item. | `news = ["read", "edit"]` |
| `delete(id: Uuid) -> Result<(), ModuleError>` | Deletes an item. | `news = ["read", "edit"]` |

## `permissions`

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

## `players`

Players.

| Вызов | Что делает | Нужно |
|---|---|---|
| `get(who: impl IntoPlayerRef) -> Result<Option<Player>, ModuleError>` | Finds a player by any of the ways they can be named. | the `players = ["read"]` capability |
| `require(who: impl IntoPlayerRef) -> Result<Player, ModuleError>` | The same lookup, except a missing player is an error. | — |
| `ban(who: impl IntoPlayerRef, reason: Option<&str>) -> Result<(), ModuleError>` | Bans an account outright. | `players = ["read", "ban"]` |
| `unban(who: impl IntoPlayerRef) -> Result<(), ModuleError>` | Lifts an account ban. Unbanning someone who is not banned is not an error. | `players = ["read", "ban"]` |
| `rename(who: impl IntoPlayerRef, username: &str) -> Result<(), ModuleError>` | Changes a player's Minecraft username. | `players = ["read", "rename"]` |
| `set_skin(who: impl IntoPlayerRef, url: Option<&str>, slim: bool) -> Result<(), ModuleError>` | Sets a player's skin. `url = None` clears it back to the default. | `players = ["read", "skin"]` |
| `set_cape(who: impl IntoPlayerRef, cape_id: Option<Uuid>) -> Result<(), ModuleError>` | Puts a cape from the instance's set on a player. `None` takes it off. | `players = ["read", "skin"]` |
| `capes() -> Result<Vec<Cape>, ModuleError>` | Every cape the instance has. | `players = ["read"]` |
| `presets(who: impl IntoPlayerRef) -> Result<Vec<SkinPreset>, ModuleError>` | The skins a player has saved, newest first. | `players = ["read"]` |
| `save_preset(who: impl IntoPlayerRef, name: &str, skin_url: &str, slim: Option<bool>) -> Result<SkinPreset, ModuleError>` | Saves a skin under a name. `slim = None` takes the geometry they wear now. | `players = ["read", "skin"]` |
| `delete_preset(who: impl IntoPlayerRef, preset_id: Uuid) -> Result<bool, ModuleError>` | Deletes a saved skin. `false` — it was not there, or not theirs. | `players = ["read", "skin"]` |

## `punish`

Sanctions.

| Вызов | Что делает | Нужно |
|---|---|---|
| `ban(who: impl IntoPlayerRef, reason: &str, seconds: Option<i64>) -> Result<Punishment, ModuleError>` | Bans a player. `seconds = None` means until it is lifted. | `punish = ["issue"]` |
| `mute(who: impl IntoPlayerRef, reason: &str, seconds: Option<i64>) -> Result<Punishment, ModuleError>` | Stops a player from speaking. | `punish = ["issue"]` |
| `warn(who: impl IntoPlayerRef, reason: &str) -> Result<Punishment, ModuleError>` | Leaves a warning the player has to acknowledge. | `punish = ["issue"]` |
| `server_ban(who: impl IntoPlayerRef, server_id: Uuid, reason: &str, seconds: Option<i64>) -> Result<Punishment, ModuleError>` | Keeps a player out of one server build. | `punish = ["issue"]` |
| `issue(who: impl IntoPlayerRef, kind: PunishKind, reason: &str, seconds: Option<i64>, server_id: Option<Uuid>) -> Result<Punishment, ModuleError>` | The general form, for when the kind is computed rather than written. | `punish = ["issue"]` |
| `revoke(id: Uuid) -> Result<bool, ModuleError>` | Lifts a sanction. `false` — it was already lifted or never existed. | `punish = ["revoke"]` |
| `active(who: impl IntoPlayerRef) -> Result<Vec<Punishment>, ModuleError>` | The sanctions in force on a player right now. | `punish = ["read"]` |
| `history(who: impl IntoPlayerRef) -> Result<Vec<Punishment>, ModuleError>` | Everything ever issued to a player, lifted and expired included. | `punish = ["read"]` |

## `restarts`

Restart schedules of game servers.

| Вызов | Что делает | Нужно |
|---|---|---|
| `of(game_server_id: Uuid) -> Result<Vec<RestartSchedule>, ModuleError>` | The schedules of one game server. | `restarts = ["read"]` |
| `add(draft: ScheduleDraft) -> Result<RestartSchedule, ModuleError>` | Adds a schedule and returns it, with the next run already computed. | `restarts = ["read", "manage"]` |
| `remove(id: Uuid) -> Result<bool, ModuleError>` | Removes a schedule. `false` — it was not there. | `restarts = ["read", "manage"]` |

## `roles`

Roles.

| Вызов | Что делает | Нужно |
|---|---|---|
| `list() -> Result<Vec<Role>, ModuleError>` | Every role on the instance. | `roles = ["read"]` |
| `get(role: impl IntoRoleRef) -> Result<Option<Role>, ModuleError>` | One role, by machine name or identifier. | `roles = ["read"]` |
| `of(who: impl IntoPlayerRef) -> Result<Vec<Role>, ModuleError>` | The roles a player holds. | `roles = ["read"]` |
| `grant(who: impl IntoPlayerRef, role: impl IntoRoleRef) -> Result<(), ModuleError>` | Gives a player a role. Doing it twice is not an error. | `roles = ["read", "grant"]` |
| `revoke(who: impl IntoPlayerRef, role: impl IntoRoleRef) -> Result<(), ModuleError>` | Takes a role away. Revoking one the player does not have is not an error. | `roles = ["read", "grant"]` |
| `create(draft: RoleDraft) -> Result<uuid::Uuid, ModuleError>` | Creates a role and returns its identifier. | `roles = ["read", "manage"]` |
| `update(id: uuid::Uuid, draft: RoleDraft) -> Result<(), ModuleError>` | Rewrites a role. Every field is replaced, not merged. | `roles = ["read", "manage"]` |
| `delete(id: uuid::Uuid) -> Result<(), ModuleError>` | Deletes a role. Players holding it simply stop holding it. | `roles = ["read", "manage"]` |

## `roster`

Who is in game right now.

| Вызов | Что делает | Нужно |
|---|---|---|
| `online() -> Result<Vec<OnlinePlayer>, ModuleError>` | Everyone in game, across every server. | `roster = ["read"]` |
| `locate(who: impl IntoPlayerRef) -> Result<Option<Uuid>, ModuleError>` | Which game server a player is on, if any. | `roster = ["read"]` |
| `is_online(who: impl IntoPlayerRef) -> Result<bool, ModuleError>` | Whether a player is in game at all. | `roster = ["read"]` |

## `servers`

Server builds, their client builds, and the game servers behind them.

| Вызов | Что делает | Нужно |
|---|---|---|
| `list() -> Result<Vec<Server>, ModuleError>` | Every server build. | `servers = ["read"]` |
| `get(id: Uuid) -> Result<Option<Server>, ModuleError>` | One server build by identifier. | `servers = ["read"]` |
| `by_slug(slug: &str) -> Result<Option<Server>, ModuleError>` | One server build by its hub slug — the `/s/<slug>` address. | `servers = ["read"]` |
| `game_servers(server_id: Uuid) -> Result<Vec<GameServer>, ModuleError>` | The game servers of a server build. | `gameservers = ["read"]` |
| `game_server(id: Uuid) -> Result<Option<GameServer>, ModuleError>` | One game server by identifier. | `gameservers = ["read"]` |
| `set_maintenance(game_server_id: Uuid, enabled: bool) -> Result<(), ModuleError>` | Puts a game server into maintenance, or takes it out. | `gameservers = ["read", "maintenance"]` |

## `sessions`

A player's sessions — the launchers and browsers they are signed in from.

| Вызов | Что делает | Нужно |
|---|---|---|
| `of(who: impl IntoPlayerRef) -> Result<Vec<Session>, ModuleError>` | Every session a player has open. | `sessions = ["read"]` |
| `revoke(who: impl IntoPlayerRef, session_id: Uuid) -> Result<bool, ModuleError>` | Closes one session. `false` — it was already gone. | `sessions = ["read", "revoke"]` |
| `revoke_all(who: impl IntoPlayerRef) -> Result<u64, ModuleError>` | Closes all of them and returns how many were closed. | `sessions = ["read", "revoke"]` |

## `store`

The module's own storage.

| Вызов | Что делает | Нужно |
|---|---|---|
| `instance() -> Store` | Shared across the instance: module settings, counters, flags. | — |
| `server(id: Uuid) -> Store` | Separate for each server. | — |
| `user(id: Uuid) -> Store` | Separate for each player. Removed together with the player. | — |

## `telemetry`

How a game server is holding up.

| Вызов | Что делает | Нужно |
|---|---|---|
| `of(game_server_id: Uuid) -> Result<Option<Telemetry>, ModuleError>` | The latest measurement, or `None` when the agent has never reported. | `telemetry = ["read"]` |

## `tickets`

Conversations with players.

| Вызов | Что делает | Нужно |
|---|---|---|
| `queue(page: i64, per_page: i64) -> Result<Vec<Ticket>, ModuleError>` | The staff queue: everything not closed, newest activity first. | `tickets = ["read"]` |
| `get(id: Uuid) -> Result<Option<Ticket>, ModuleError>` | One conversation. | `tickets = ["read"]` |
| `messages(id: Uuid) -> Result<Vec<TicketMessage>, ModuleError>` | The messages of a conversation, oldest first. | `tickets = ["read"]` |
| `reply(id: Uuid, content: &str) -> Result<(), ModuleError>` | Writes a reply, signed with your module. | `tickets = ["read", "reply"]` |
| `open(who: impl IntoPlayerRef, subject: &str, content: &str) -> Result<Uuid, ModuleError>` | Opens a conversation with a player and returns its identifier. | `tickets = ["read", "reply"]` |
| `close(id: Uuid) -> Result<(), ModuleError>` | Closes a conversation. | `tickets = ["read", "reply"]` |

## Методы на сущностях

Методы на том, что уже в руках. Каждый ведёт в доменную функцию того же имени — та же возможность, то же поведение. Prelude их импортирует.

```rust
let player = players::require("Dalynkaa")?;
player.server_ban(server_id, "гриф", Some(7 * 24 * 3600))?;
```

### `Player`

What you can do to a player you are holding.

| Метод | Что делает |
|---|---|
| `ban(reason: Option<&str>) -> Result<(), ModuleError>` | Bans the account. |
| `unban() -> Result<(), ModuleError>` | Lifts the account ban. |
| `rename(username: &str) -> Result<(), ModuleError>` | Renames them. Refused if the name is taken. |
| `set_skin(url: Option<&str>, slim: bool) -> Result<(), ModuleError>` | Sets the skin; `slim` travels with it. |
| `set_cape(cape_id: Option<Uuid>) -> Result<(), ModuleError>` | Puts a cape on, or takes it off with `None`. |
| `presets() -> Result<Vec<SkinPreset>, ModuleError>` | Their saved skin presets. |
| `roles() -> Result<Vec<Role>, ModuleError>` | Their roles. |
| `grant_role(role: impl IntoRoleRef) -> Result<(), ModuleError>` | Grants a role by name or id. |
| `revoke_role(role: impl IntoRoleRef) -> Result<(), ModuleError>` | Takes a role away. |
| `can(node: &str) -> Result<bool, ModuleError>` | Whether they effectively hold a permission. |
| `can_on(node: &str, server_id: Uuid) -> Result<bool, ModuleError>` | The same on one server build. |
| `grant(node: &str) -> Result<(), ModuleError>` | Grants a personal permission. |
| `revoke(node: &str) -> Result<(), ModuleError>` | Takes a personal permission away. |
| `allow_join(server_id: Uuid) -> Result<(), ModuleError>` | Lets them into a server build. |
| `revoke_join(server_id: Uuid) -> Result<(), ModuleError>` | Takes that away. |
| `punish_ban(reason: &str, seconds: Option<i64>) -> Result<Punishment, ModuleError>` | Bans them with a punishment — the kind the player reads and can appeal. |
| `mute(reason: &str, seconds: Option<i64>) -> Result<Punishment, ModuleError>` | Mutes them. |
| `warn(reason: &str) -> Result<Punishment, ModuleError>` | Warns them. |
| `server_ban(server_id: Uuid, reason: &str, seconds: Option<i64>) -> Result<Punishment, ModuleError>` | Bans them from one server only. |
| `punishments() -> Result<Vec<Punishment>, ModuleError>` | Their punishments in force. |
| `account(server_id: Uuid) -> Result<Option<Account>, ModuleError>` | Their account on a server. |
| `balance(server_id: Uuid) -> Result<i64, ModuleError>` | What they have, in the smallest unit. |
| `tell(message: &str) -> Result<bool, ModuleError>` | A private message. `false` — they are not in game. |
| `kick(reason: &str) -> Result<bool, ModuleError>` | Throws them off the server. |
| `is_online() -> Result<bool, ModuleError>` | Whether they are in game right now. |
| `launcher_online() -> Result<bool, ModuleError>` | Whether their launcher is connected. |
| `send(payload: impl serde::Serialize) -> Result<bool, ModuleError>` | Sends their launcher a frame. |
| `store() -> crate::store::Store` | Their corner of your storage. |
| `identities() -> Result<Vec<Identity>, ModuleError>` | Their linked logins. |
| `sessions() -> Result<Vec<Session>, ModuleError>` | Their sessions in the panel and the launcher. |
| `revoke_sessions() -> Result<u64, ModuleError>` | Ends every session they have. |

### `Server`

What you can do with a server build you are holding.

| Метод | Что делает |
|---|---|
| `builds() -> Result<Vec<Build>, ModuleError>` | Its client builds. |
| `published_build() -> Result<Option<Build>, ModuleError>` | The build players are getting. |
| `game_servers() -> Result<Vec<GameServer>, ModuleError>` | Its game servers. |
| `treasury() -> Result<Account, ModuleError>` | Its treasury account. |
| `announce(message: &str) -> Result<(), ModuleError>` | An announcement to everybody on it. |
| `store() -> crate::store::Store` | This server's corner of your storage. |
| `feed(page: i64) -> Result<noro_module_abi::ops::HubPage, ModuleError>` | Its hub feed, paginated. |
| `members(page: i64) -> Result<noro_module_abi::ops::HubPage, ModuleError>` | Its hub members. |

### `Build`

What you can do with a build you are holding.

| Метод | Что делает |
|---|---|
| `files() -> Result<Vec<BuildFile>, ModuleError>` | Everything inside it. |
| `read(path: &str) -> Result<Option<String>, ModuleError>` | The text of one file. |
| `write(path: &str, text: &str) -> Result<BuildFile, ModuleError>` | Writes a text file into it. |
| `attach(path: &str, sha1: &str) -> Result<BuildFile, ModuleError>` | Puts an already-stored file in by hash. |
| `remove(path: &str) -> Result<bool, ModuleError>` | Removes a file. |
| `unpublish() -> Result<(), ModuleError>` | Stops the launcher handing it out. |
| `allow(who: impl noro_module_abi::player::IntoPlayerRef) -> Result<(), ModuleError>` | Lets one player download it. |

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
