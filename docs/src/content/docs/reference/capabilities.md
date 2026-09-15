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

## What each one reaches

Every action below has a host function behind it, and a test that says so — see the note
after the table.

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
| `builds` | `read` | `builds::of`, `builds::published`, `builds::files`, `builds::read` |
| | `files` | `builds::write`, `builds::attach`, `builds::remove` |
| | `publish` | `builds::unpublish` — publishing itself stays with the operator |
| `gameservers` | `read` | `servers::game_servers`, `servers::game_server` |
| | `maintenance` | `servers::set_maintenance` |
| | `command` | `servers::command` |
| `punish` | `read` | `punish::active`, `punish::history` |
| | `issue` | `punish::ban`, `mute`, `warn`, `server_ban`, `issue` |
| | `revoke` | `punish::revoke` |
| `bank` | `read` | `bank::account`, `accounts`, `balance`, `treasury` |
| | `transfer` | `bank::transfer`, `bank::transfer_once` |
| `agent` | `tell` | `agent::tell` |
| | `announce` | `agent::announce`, `agent::announce_on` |
| | `kick` | `agent::kick` |
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
| | `manage` | `hub::found_town` |
| `web_ws` | `read` | `web_ws::is_online`, `web_ws::connected` |
| | `notify` | `web_ws::send`, `web_ws::broadcast`, `player.push_web_event` |
| `market` | `read` | `hub::market`, `hub::lot` |
| | `sell` | `hub::list_lot` |
| `court` | `read` | `hub::court`, `hub::court_case` |
| | `file` | `hub::file_claim` |
| `petitions` | `read` | `hub::petitions` |
| | `sign` | `hub::sign`, `hub::unsign` |
| | `create` | `hub::start_petition` |
| `fines` | `read` | `hub::fines`, `hub::fines_of` |
| | `issue` | `hub::fine` |
| `events` | `emit` | `events::emit`, `events::emit_on` |
| `launcher` | `read` | `launcher::is_online`, `launcher::connected` |
| | `notify` | `launcher::send`, `launcher::broadcast` |
| `dm` | `read` | `dm::threads`, `dm::history`, `dm::presence` |
| | `send` | `dm::send` |
| `bots` | `create` | `bots::ensure`, `bots::list` |
| `cache` | `read` | `cache::get`, `cache::get_or` |
| | `write` | `cache::set`, `cache::delete` |
| `modules` | `call` | `modules::call` |
| `store` | `true` | the whole key-value store |
| `db` | `true` | `db::query`, `execute`, `one`, `scalar`, plus your migrations |
| `http` | a host allow-list | `http::send`, `http::get_json` |

## The one that reads other people's mail

`dm = ["read"]` opens private messages — what two players wrote to each other, on the
site or in game. The instance's privacy policy promises that staff do not read them in
the course of moderation; a module holding this capability is the one exception the
operator has agreed to.

Ask for it when the job needs it: filtering links, an answering machine for someone who
is away, a bot that replies. Do not ask for it "in case it comes in handy" — the operator
sees the line in your manifest and has to decide, and a module that reads correspondence
without needing to is a module they should refuse.

`dm = ["send"]` writes on a player's behalf, and the recipient sees their name. That is
deliberate: they are going to answer, and a message from nobody leaves them with nobody
to answer to. The history records which module wrote it, and the conversation shows it.

## The one with a long arm

`builds = ["files"]` writes what the launcher downloads and puts in somebody's game
directory. Nothing has to be re-signed afterwards: the manifest is assembled and signed
on every launcher request from the current rows, so a written file is live from the next
one. There is no review step between the call and the player.

That is why it is its own action rather than part of `read`, and why the operator sees it
spelled out at install time. Text goes in directly and is capped at a megabyte —
configuration, not mods. Anything bigger is stored once with `files::put` and attached by
hash, because dragging tens of megabytes through the sandbox as a string is not a plan.

Publishing a build still stays with the operator: that reassembles artifacts and fetches
what is missing, minutes of work, and a module's call has seconds.

:::note[The list and the code cannot drift]
Every action in the first table is checked by a test that reads the master's own source:
a capability nobody checks, and a check for a capability nobody can ask for, both fail
the build. Seventeen domains once had host functions and no manifest field at all —
asking for them was dropped without a word, and every call was refused no matter what the
operator granted.
:::

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
