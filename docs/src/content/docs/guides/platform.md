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

## Sanctions

A punishment is not the account ban flag. It has a kind, a reason the player reads, a
duration, and it reaches the running game the moment you issue it — a ban throws the
player out, a mute stops them mid-sentence.

```rust
punish::mute(player, "spam", Some(600))?;
punish::ban(player, "cheating", None)?;            // None — until lifted
punish::server_ban(player, server_id, "griefing", Some(7 * 24 * 3600))?;
punish::warn(player, "language")?;

for p in punish::active(player)? { … }
punish::revoke(p.id)?;
```

Issuing does all four things an operator's sanction does: the row, the `users` ban flag
that the launcher and Yggdrasil read, the updated profile pushed to open launchers, and
the frame to the agents. Skip any one of them and you get a ban that is not a ban.

A duration of zero or less is refused rather than treated as "already expired", and an
empty reason is refused too — the player is going to read it.

## Money

Money lives in accounts, and a player can hold several, so every call names an account
rather than a player.

```rust
let from = bank::treasury(server_id)?;
let to = bank::account(server_id, player)?.ok_or(/* no account yet */)?;
bank::transfer(server_id, from.id, to.id, 100, "playtime reward")?;
```

The transfer runs through the same code a transfer from the cabinet does: the balance is
checked inside a transaction under a row lock, and accounts are taken in id order so two
opposing transfers cannot deadlock. Insufficient funds come back as a `Conflict`.

If your handler might run twice — a retried event, a task that overlapped itself — use
`transfer_once` with a key. The second call returns the transfer already made instead of
paying again.

Transfers are not written to the audit log: the bank keeps its own ledger, and one
operation in two histories is one history too many.

## Talking to players

```rust
if !chat::tell(player, "Your rank is now VIP.")? {
    // not in game — tell them in the cabinet instead
}
chat::announce_on(server_id, "Maintenance in 10 minutes.")?;
chat::kick(player, "Take a break.")?;
```

`tell` and `kick` return `false` when the player is not in game. That is an ordinary
outcome, not an error — telling a player who has just left is exactly what a "player
left" handler does.

The text is finished text, not a locale key. Your catalog is built for the panel and is
not installed in the master's own `i18n`, so a key here would reach the player as the key
itself.

## Linked logins

```rust
for id in identities::of(player)? {
    if id.provider == "discord" { … }
}
let p = identities::find("twitch", "12345")?;
identities::link(player, "twitch", "12345", Some("streamer"))?;
identities::unlink(player, "twitch")?;
```

`link` returns `false` when that platform identifier already belongs to somebody:
identifiers are unique across the instance, and moving one between accounts is not
something a module does quietly.

The platform a player **registered** through cannot be unlinked at all — their Minecraft
UUID is derived from it, and with it their inventory, their progress and their
permissions everywhere. Asking for that is an error, not a `false`.

## Appearance

```rust
players::rename(player, "NewName")?;                    // refused if taken
players::set_skin(player, Some(url), true)?;            // true — the slim model
players::set_skin(player, None, false)?;                // back to the default

for cape in players::capes()? { … }
players::set_cape(player, Some(cape.id))?;
players::set_cape(player, None)?;                       // take it off

players::save_preset(player, "Winter", url, None)?;     // None — the geometry they wear
for p in players::presets(player)? { … }
players::delete_preset(player, p.id)?;
```

A cape is named by id and the file comes from the instance's set — accepting a URL would
put a cape on a player that the instance does not have.

`slim` travels with the skin rather than being set separately: a slim texture on classic
arms reads as broken, and both halves are one decision.

Every one of these pushes the updated profile to the player's open launchers and tells
the agents to re-read their textures. Without that the skin changes and nobody sees it
until they rejoin.

## Reaching outside

```rust
let reply = http::send(HttpCall::post(url, body).header("content-type", "application/json"))?;
if !reply.ok() {
    log::warn(format!("webhook answered {}", reply.status));
}
let data: Info = http::get_json("https://api.example.org/info")?;
```

The host must be in your manifest's `http` list, and the scheme must be `https`.

A non-2xx status is an answer, not an error: whether a 404 matters is your call.

### What is checked before the request leaves

The name is matched against your allow-list. Then it is resolved, and the request is
refused if **any** of the addresses it resolves to is inside the machine or the private
network — loopback, `10/8`, `192.168/16`, `169.254/16` where cloud metadata lives, CGNAT,
IPv6 unique-local and link-local, and IPv4 loopback wrapped as `::ffff:127.0.0.1`.

Then the connection goes to the address that was checked, not to whatever the name
resolves to a moment later. An allow-list alone would be worth very little: a permitted
name pointed at `127.0.0.1` turns a module into a way to reach the master's own services,
and the record can change between the check and the connection.

Redirects are not followed — the next hop is a host nobody checked. Follow it yourself if
you want it.

### What it costs

The call blocks your handler. A `Post` handler has five seconds in total and the request
itself is capped at four, so a slow endpoint is a handler that times out. An answer over
a megabyte is refused rather than loaded: it would come into the sandbox's memory.

## Conversations and cases

A ticket is one thread with a player — from the cabinet, from `/support` in game, or
opened out of a moderation case. A case is what a report becomes: a target, a timeline,
and eventually a verdict.

```rust
for t in tickets::queue(1, 25)? {
    if t.unread > 0 { … }
}
tickets::reply(t.id, "Lifted, sorry about that.")?;
let id = tickets::open(player, "Your appeal", "We have looked at it again.")?;

let case = cases::get(case_id)?;
cases::claim(case_id, on_duty_moderator)?;
cases::resolve(case_id, "confirmed", "Chat log speaks for itself.", Some("3.2"))?;
```

Two things are deliberate here.

**A module's message is signed with the module.** There is no person behind it, and
putting the name of whoever enabled the module on those words would be putting words in
their mouth — the player is going to read them and ask. It appears the way an admin
token's messages do, which is the same situation.

**Claiming a case names a moderator.** A case belongs to someone who will answer for it;
one claimed by a module would be a case nobody is working on. Automation can assign, it
cannot take responsibility.

Opening a conversation with a player who already has one open uses theirs. Two threads
about one thing is how an answer gets lost.

Resolving a case closes the reports behind it with the same outcome. A resolved case with
its reports still open is the state in which the work gets done twice.
