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

### Realtime browser messages: WebMessage

When a player interacts with your web mini-app and sends actions over WebSocket via `noro.ws.send(...)`, the master routes them directly to your module as a `WebMessage` event:

```rust
#[event]
fn on_web_action(e: WebMessage) -> Result<()> {
    log::info(format!("Action from {}: {:?}", e.player.label(), e.payload));
    Ok(())
}
```

See the dedicated [Realtime & WebSockets](../websockets/) guide for details.

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

## Saying no

A `Pre` handler takes its event by `&mut` — and that is the whole declaration. The
reference is what tells the master your answer is worth waiting for and sending back:

```rust
#[event]
fn rename(e: &mut UserPreRename) -> Result<()> {
    if reserved(&e.new_name) {
        e.stop("mod-gate-name-reserved");
        return Ok(());
    }
    // Changes travel back. The master writes the name you leave here.
    e.new_name = e.new_name.to_lowercase();
    Ok(())
}
```

`stop` takes a **Fluent key**, not finished text: the player reads it, and only the side
that knows their language can put the sentence together.

Handlers run in priority order, and each gets the event as the previous one left it — so
one module's fee lands on an amount another module already changed, rather than on the
original.

:::note[Where each one fires]
All nine `Pre` events are delivered. Two are worth knowing precisely:

`player.pre_join` is asked only when the master would otherwise let the player in — a ban
needs no confirmation from you — and only for a single-player lookup, never for the
roster batch. A stop becomes the agent's refusal, with your key as the reason.

`user.pre_role_granted` fires for a grant made **in the panel**. A role handed out by
another module does not go through it; module-to-module gating is not something this
boundary offers.

`player.pre_chat` is the one event the game server pays for per message, so the master
tells the agent whether anybody is subscribed and the agent stays silent until somebody
is. Your budget there is **700 ms** measured from the game server, not one second — the
player has already pressed Enter and is watching for their line. Past it the message goes
through unchanged.

Rewriting `text` works on Paper. On Fabric and Forge the game's own hook answers with a
yes or no and nothing else, so a refusal lands and an edit does not — write the filter so
that refusing is the fallback, not a surprise.
:::

### What a Pre handler must know

It sits **in the path of a live request**. The master waits, and the player waits with
it — so the budget is one second, a fifth of what a `Post` handler gets. Work that takes
longer belongs in `Post`.

It runs *before* the master's transaction opens, so state can change in between. Anything
that must hold at commit time is re-checked under a row lock: raising `amount` on a
transfer conjures no money, because the sender's balance is verified again inside the
transaction.

And if a module is slow, fails, or answers with something unparseable, the action **goes
through**. One broken module must not close the door for everybody — that is a failure
found at three in the morning. A refusal has to be a decision, not a side effect.

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
