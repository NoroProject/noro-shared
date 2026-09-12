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
| | `manage` | `roles::create`, `update`, `delete` |
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
| `optional_mods` | `grant` | `access::allow_mod`, `revoke_mod` |
| `news` | `read` | `news::list`, `news::get` |
| | `publish` | `news::publish`, `publish_pinned` |
| | `edit` | `news::edit`, `news::delete` |
| `instance` | `read` | `instance::all`, `instance::get` |
| | `write` | `instance::set` |
| `sessions` | `read` | `sessions::of` |
| | `revoke` | `sessions::revoke`, `revoke_all` |
| `restarts` | `read` | `restarts::of` |
| | `manage` | `restarts::add`, `restarts::remove` |
| `roster` | `read` | `roster::online`, `locate`, `is_online` |
| `telemetry` | `read` | `telemetry::of` |
| `files` | `read` | `files::read`, `exists`, `url` |
| | `write` | `files::put` |
| `tickets` | `read` | `tickets::queue`, `get`, `messages` |
| | `reply` | `tickets::reply`, `open`, `close` |
| `cases` | `read` | `cases::get`, `events`, `open_on` |
| | `claim` | `cases::claim` |
| | `resolve` | `cases::resolve` |
| `hub` | `read` | `hub::feed`, `members`, `playtime` |
| | `post` | `hub::post` |
| `towns` | `read` | `hub::towns`, `hub::town` |
| `market` | `read` | `hub::market`, `hub::lot` |
| `court` | `read` | `hub::court`, `hub::court_case` |
| `petitions` | `read` | `hub::petitions` |
| | `sign` | `hub::sign`, `hub::unsign` |
| `fines` | `read` | `hub::fines`, `hub::fines_of` |
| | `issue` | `hub::fine` |
| `store` | `true` | the whole key-value store |
| `db` | `true` | `db::query`, `execute`, `one`, `scalar`, plus your migrations |
| `http` | a host allow-list | `http::send`, `http::get_json` |

## What is not there yet

These are the ones worth having next, in roughly the order they are worth it. Asking for
one in the manifest is harmless — the field parses and the operator sees it — but there
is no call to make with it.

| Capability | Actions | What it would give you |
|---|---|---|
| `towns` | `manage` | founding a town, moving its borders, its treasury |
| `market` | `sell` | listing a lot and stocking it from a vault |
| `court` | `file` | filing a claim, with its fee |
| `petitions` | `create` | starting a petition rather than only signing one |
| `builds` | `files` | reading and writing the files inside a build |
| `events` | `emit` | publishing your own events for other modules to handle |

Each of these is bigger than a host function, for a different reason.

The three hub writes — founding a town, listing a lot, filing a claim — all move money
and run several steps that have to hold together: a fee charged, a vault stocked, a
deadline started. Half of that sequence executed is worse than none of it, and getting it
right means more than wrapping an existing query.

`events = ["emit"]` needs the reentrancy guard that `EventCtx.depth` is there for —
without it, two modules reacting to each other's events make a loop the master has to
break rather than merely notice.

`builds = ["files"]` touches what the launcher downloads and verifies by signature, so
writing there means resigning the manifest — the same reason publishing a build stays
with the operator.

## Acting for somebody

Some writes name a player: posting to the feed, issuing a fine, signing a petition,
claiming a case. These are things a **person** does, and a module has no person behind
it — a feed entry from nobody is not something the hub can represent, and not something a
reader could reply to.

So the module says whose act it is, and that name is what appears. Automation can act for
someone; it cannot be someone. The audit entry records both: the module that made the
call, and the player it acted for.

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

The list is only the first of three checks. The name is resolved and the request refused
if any address it resolves to is inside the machine or the private network, and the
connection then goes to the address that was checked rather than to whatever the name
resolves to a moment later. An allow-list on its own is worth very little: a permitted
name pointed at `127.0.0.1` would turn a module into a way to reach the master's own
services. See [reaching outside](../../guides/platform/#reaching-outside).
