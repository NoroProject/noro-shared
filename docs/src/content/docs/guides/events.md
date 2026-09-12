---
title: Events
description: Subscribing, priorities, context, and cancelling an action before it happens.
---

## Subscribing

```rust
#[event]
fn on_join(e: PlayerJoined) -> Result<()> {
    log::info(format!("{} joined {}", e.player.label(), e.ctx.server_name()));
    Ok(())
}
```

There is no event name anywhere. `#[event]` reads it from the argument type, which is
why the [event catalog](../../reference/events/) lists the payload struct next to each
name — that struct *is* how you subscribe.

## Where an event happened

Every event carries an `EventCtx`. On an instance with several server builds an event
without it is nearly useless: you know a player joined, but not what they joined.

```rust
#[event]
fn on_rename(e: UserRenamed) -> Result<()> {
    if e.ctx.from_game() { return Ok(()) }          // came from the game
    if e.ctx.caused_by("my-module") { return Ok(()) } // our own echo
    Ok(())
}
```

| Field | What it tells you |
|---|---|
| `origin` | `Game` · `Web` · `Launcher` · `Cli` · `Module(id)` · `System` — *through what* |
| `actor` | the user, module or token behind it — *who* |
| `server_id`, `server_slug` | the server build; `None` on global events like registration |
| `game_server_id` | the specific game server, when it came from the game |
| `at` | when |
| `depth` | how deeply publications are nested |

:::caution[Watch for your own echo]
If your handler changes the same data it watches, the change publishes an event that
wakes your handler again. `ctx.caused_by("your-id")` is the guard. The master also stops
publishing once `depth` hits its ceiling, but that is a backstop, not a design.
:::

## Priorities

Handlers for one event run in a ladder, the same one Bukkit uses: whoever decides runs
after whoever watches.

```rust
#[event(priority = high)]
fn decide(e: BankPreTransfer) -> Result<()> { Ok(()) }
```

`lowest` → `low` → `normal` (default) → `high` → `highest` → `monitor`. `monitor` is for
observers only; a cancellation from it is ignored.

## Saying no — not yet

:::caution[Pre events are declared but not delivered yet]
The catalog already lists the `Pre` half of the boundary, and the payload structs carry
their `cancel` field. The master, however, currently publishes **`Post` events only**.
Subscribing to a `Pre` event compiles and installs; your handler simply never runs.

Cancellation and mutation land in a later wave, together with priorities being honoured
across modules. Until then, treat the `Pre` rows in the catalog as the shape of what is
coming, not as something you can build on.
:::

When they do arrive, this is the shape they will have — a reason that is a **Fluent
key** rather than finished text, because the player reads it and only the master knows
their language:

```rust
// Not yet delivered — shown so the eventual shape is no surprise.
#[event]
fn gate(e: &mut PlayerPreJoin) -> Result<()> {
    if e.player.discord_id.is_none() {
        e.cancel("mod-gate-needs-discord");
    }
    Ok(())
}
```

Two properties of that design are worth knowing in advance, because they shape what you
should write:

A `Pre` handler will sit **in the path of a live request** — the master waits for the
answer, so a slow handler delays a player's login or their transfer.

It will run *before* the master's transaction opens, so state can change in between.
Anything that must hold at commit time is re-checked by the master under a row lock;
raising `amount` on a transfer will not conjure money.

And if a module is slow or fails, the default is **fail-open**: the action goes through.
A broken module must not keep players out.

## Events of your own

The catalog is what the platform announces. Your module can announce things too, so that
a second module reacts without either knowing the other exists.

```rust
// in the shop module
events::emit("purchase", json!({ "player": id.to_string(), "item": "vip" }))?;
```

```rust
// in some other module
#[event("mod.shop.purchase")]
fn on_purchase(e: serde_json::Value) -> Result<()> { … }
```

You pass the last part of the name; the master publishes it as
`mod.<your-id>.<name>`. The prefix is added rather than trusted, for the same reason
locale keys carry one: otherwise a module could publish `player.banned` and every handler
of the real event would believe it.

`emit_on(name, server_id, payload)` attaches a server, which is what makes the event
reach modules scoped to it.

### Loops

Module A announces, B reacts and announces, A reacts to that. Left alone this spins until
somebody notices the load.

Every event carries how deeply it is nested. An event you emit from inside a handler
inherits that depth and adds one, and past a ceiling of three the master refuses to
publish — you get an error rather than silence, because a module that has accidentally
built a loop should find out from the call, not from a graph.

The other half is yours. Before reacting to something, check whether you caused it:

```rust
if e.ctx.caused_by("my-module") { return Ok(()) }
```

### What emitting does not do

Handlers run after your call returns, not during it. `emit` hands the event to the master
and comes straight back, exactly like the platform's own `Post` events — so you cannot
learn from it whether anybody handled anything, and a handler that fails does not fail
your call.
