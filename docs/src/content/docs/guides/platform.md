---
title: Reaching the platform
description: Players, roles, permissions, access and servers — the typed calls, not a web API.
---

Every call below is a function call into the master's own code. No HTTP, no router, no
serialization of your own — you pass typed arguments and get typed results.

## Players

One entry point, because an account has four natural keys and a real module meets all of
them: the agent knows only `mc_uuid`, a web form knows the username, a Discord
integration knows its own id.

```rust
players::get(uuid)?;                                 // internal id
players::get("Dalynkaa")?;                           // Minecraft username
players::get(PlayerRef::mc_uuid(u))?;                // game account uuid
players::get(PlayerRef::discord("123456789"))?;      // Discord
players::get(PlayerRef::identity("twitch", "42"))?;  // any linked login
```

A username is also looked up among linked logins: a player may have renamed themselves in
Minecraft while the operator still calls them by the old name.

`players::require(who)` is the same lookup where a missing player is an error — for the
cases where they must exist, like a player who just joined.

```rust
players::ban(who, Some("cheating"))?;   // the account flag, no duration
players::unban(who)?;
```

## Roles

```rust
if !roles::of(player)?.iter().any(|r| r.name == "verified") {
    roles::grant(player, "verified")?;
}
```

A role is named by machine name or by id — `roles::grant(player, "vip")` and
`roles::grant(player, role_id)` are both fine. Granting twice is not an error.

A name is not globally unique: the same name can exist as a shared role and as a role of
several servers. Asking by name gives you the shared one, since that is what code without
a server in hand means.

## Permissions

```rust
if permissions::has(player, "noro.module.shop.buy")? { … }
let all = permissions::effective_on(player, server_id)?;

permissions::grant(player, "noro.optional.mymod")?;
permissions::revoke_on(player, "noro.optional.mymod", server_id)?;
```

`has` and `effective` count roles as well as personal grants — that is what the game and
the panel act on. `grant` and `revoke` touch only the personal ones: what a role carries
is a decision about everyone holding it, and a module does not get to make it.

The `_on` variants ask about one server build. Without a server you get the global
picture only — a role granted on one build deliberately stays out of it, or a "Banker" of
one server would open the panel and its neighbours.

Revoking is exact: `revoke` removes the global grant, `revoke_on` the one on that server.
Otherwise taking away "on this build" would silently drop the global one too.

## Access

Access to a server or a build is a permission on a generated node underneath. It has its
own domain because the node has to be spelled exactly right, and a module building
`noro.server.<uuid>.join` by hand will one day build it wrong and grant nothing at all.

```rust
access::allow_join(player, server_id)?;
access::allow_build(player, servers::published_build(server_id)?.unwrap().id)?;
access::revoke_join(player, server_id)?;
```

The server a build belongs to is looked up by the master, so a build id cannot be
attached to the wrong server.

## Servers, builds, game servers

Three different things, and the names are worth keeping straight. A **server** is what a
player picks in the launcher. A **build** is one downloadable client of it. A **game
server** is one running process with an agent attached.

```rust
for s in servers::list()? { … }
let s = servers::by_slug("survival")?;        // the /s/<slug> address
let builds = servers::builds(server_id)?;
let live = servers::published_build(server_id)?;

for gs in servers::game_servers(server_id)? {
    if gs.online && gs.players_online == 0 {
        servers::set_maintenance(gs.id, true)?;
    }
}
```

`gs.online` is whether the agent is alive, **not** a player count — `players_online` is
the count. The database column is named `online` and holds the count, which is exactly the
trap this naming avoids.

Maintenance both writes the flag and tells the running agents, so it takes effect in game
immediately, with the same sixty-second warning the panel gives by default.

Editing servers and publishing builds are not here. They change what the launcher hands
players, and that stays the operator's decision for now.

## Every write is signed

A role granted, a permission given, a player banned — all of it lands in the audit log
under the same action an operator's would, signed with your module's id. See
[capabilities](../../reference/capabilities/).
