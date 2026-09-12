---
title: Capabilities
description: What a module can ask for, what the operator grants, and what is wired up today.
---

A module **asks**; the operator **grants**. The two need not match — granting less than
was asked for is normal and supported. A call into something ungranted returns
`CapabilityDenied` naming what is missing; it does not crash the module.

```toml
[capabilities]
players = ["read"]
store = true
http = ["discord.com"]
```

An empty list is a denial. A domain you do not mention is closed.

## Status

The capability model, the manifest fields and the operator's grant screen are all in
place. The host functions behind each domain are not all there yet, and this table says
which are.

| Capability | Actions | What you can call |
|---|---|---|
| `players` | `read` | `players::get`, `players::require`, `players::capes` |
| | `ban` | `players::ban`, `players::unban` |
| | `rename` | `players::rename` |
| | `skin` | `players::set_skin`, `set_cape`, `presets`, `save_preset`, `delete_preset` |
| `identities` | `read` | `identities::of`, `identities::find` |
| | `link` | `identities::link`, `identities::unlink` |
| `roles` | `read` | `roles::list`, `roles::get`, `roles::of` |
| | `grant` | `roles::grant`, `roles::revoke` |
| `permissions` | `read` | `permissions::has`, `permissions::effective` (and the `_on` variants) |
| | `grant` | `permissions::grant`, `permissions::revoke` |
| `access` | `grant` | `access::allow_join`, `access::allow_build`, and their `revoke_*` |
| `servers` | `read` | `servers::list`, `get`, `by_slug` |
| `builds` | `read` | `servers::builds`, `servers::published_build` |
| | `publish` | `servers::unpublish` — publishing itself stays with the operator |
| `gameservers` | `read` | `servers::game_servers`, `servers::game_server` |
| | `maintenance` | `servers::set_maintenance` |
| `punish` | `read` | `punish::active`, `punish::history` |
| | `issue` | `punish::ban`, `mute`, `warn`, `server_ban`, `issue` |
| | `revoke` | `punish::revoke` |
| `bank` | `read` | `bank::account`, `accounts`, `balance`, `treasury` |
| | `transfer` | `bank::transfer`, `bank::transfer_once` |
| `agent` | `tell` | `chat::tell` |
| | `announce` | `chat::announce`, `chat::announce_on` |
| | `kick` | `chat::kick` |
| `store` | `true` | the whole key-value store |
| `db` | `true` | `db::query`, `execute`, `one`, `scalar`, plus your migrations |
| `http` | a host allow-list | `http::send`, `http::get_json` |

## What is not there yet

These are the ones worth having next, in roughly the order they are worth it. Asking for
one in the manifest is harmless — the field parses and the operator sees it — but there
is no call to make with it.

| Capability | Actions | What it would give you |
|---|---|---|
| `hub` | `read`, `post` | the server's feed: posts, comments, members |
| `towns` | `read`, `manage` | towns, their treasuries and their members |
| `market` | `read`, `list`, `sell` | lots, orders and deliveries |
| `court` | `read`, `file` | claims, hearings, rulings |
| `petitions` | `read`, `create` | petitions and their votes |
| `fines` | `read`, `issue` | fines, and what became of them |
| `tickets` | `read`, `reply` | player conversations, in the panel and in game |
| `cases` | `read`, `claim`, `resolve` | moderation cases and their timelines |
| `news` | `read`, `publish` | the instance's news |
| `files` | `read`, `write` | the shared file store, deduplicated by hash |
| `roster` | `read` | who is in game right now, and where |
| `telemetry` | `read` | a game server's load, TPS and memory |
| `restarts` | `read`, `manage` | restart schedules |
| `instance` | `read`, `write` | instance settings |
| `optional_mods` | `grant` | access to individual optional mods of a build |
| `sessions` | `read`, `revoke` | a player's sessions, and closing them |
| `roles` | `manage` | creating, editing and deleting roles, not just granting |
| `builds` | `files` | reading and writing the files inside a build |
| `events` | `emit` | publishing your own events for other modules to handle |

Two of these are bigger than a host function. `events = ["emit"]` needs the reentrancy
guard that `EventCtx.depth` is there for. And everything under `hub`, `towns`, `market`,
`court` and `petitions` is most useful to a module with `scope = "server"`, which also
wants widgets and a section in the hub — that is a wave of its own rather than a row in
this table.

## Writes are audited as if you were staff

A role granted or a player banned by a module is written to the audit log under **the
same action** an operator's would use, signed `модуль «your-id»`. This is deliberate: a
ban has to appear in the list staff already read, not in a feed of its own. Reads are not
audited; they go in the module's call log instead.

Nothing a module writes is attributed to a person. `granted_by` stays empty rather than
naming whoever enabled the module — they did not make this decision, and pinning every
later action on them would be a lie the audit log cannot take back.

## Why these are words and not a bitmask

The operator reads this list in full before granting it. Anything that needs decoding
before it can be judged would not get judged.

## The HTTP allow-list

```toml
[capabilities]
http = ["discord.com", "*.example.org"]
```

Hosts, not URLs — a path or a port in there is refused at install time. `*.example.org`
matches subdomains but **not** `example.org` itself: a wildcard should not quietly grant
the parent nobody named.

When outbound HTTP does arrive it will resolve DNS and refuse private address ranges,
because otherwise an allow-list is bypassed by pointing a permitted name at `127.0.0.1`.
