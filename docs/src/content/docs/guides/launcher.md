---
title: Talking to the launcher
description: Giving a fork of the launcher a half that lives on the master.
---

The launcher is open source and people change it. A change almost always needs a half on
the master — something counted, something stored, something only the server can know —
and the master is not open. A module is that half.

Nothing here is a special case: it is the ordinary endpoint you already have, plus one
frame for the direction an endpoint cannot cover.

## Receiving what the fork sends

There is no special macro. A frame from the launcher is an event like any other, and you
subscribe to it the same way — by the type of the argument.

```rust
use noro_sdk::prelude::*;

#[noro::module]
impl Module {
    #[event]
    fn on_frame(e: LauncherMessage) -> Result<()> {
        let kind = e.payload["kind"].as_str().unwrap_or("");
        log::info(format!("{} sent {kind}", e.player.label()));
        Ok(())
    }
}
```

The fork writes into the socket it already has open:

```json
{"t":"ModuleMessage","d":{"module":"your-id","payload":{"kind":"hello"}}}
```

The frame reaches **only the module it names** — this is one half of a conversation
between a fork and its own module, and a neighbouring module reading it would surprise
both of them. Receiving needs no `launcher` capability: the frame is addressed to you,
and there is nothing to permit.

There is no reply. Where you need one, use the endpoint.

## Answering a question: an endpoint

```rust
#[route(GET, "/my-fork/state")]
fn state(req: HttpRequest) -> Result<Value> {
    let id = req.require_user()?;
    Ok(json!({ "points": store::user(id).get_or::<i64>("points")? }))
}
```

Your fork calls `GET /api/modules/<your-id>/my-fork/state` with the player's bearer token
— the one it already holds to talk to the master at all. There is nothing to enable: the
endpoint is reachable to any signed-in player unless your manifest says otherwise, and
`req.require_user()` tells you who called.

Use this whenever the fork is asking a question. It has a status code, a body and an
error you can show.

## Speaking first: a frame outward

```rust
#[event]
fn on_paid(e: BankTransferred) -> Result<()> {
    launcher::send(e.to, json!({ "kind": "paid", "amount": e.amount }))?;
    Ok(())
}
```

`send` answers `false` when that player has no launcher connected. That is a state, not a
failure — and there is no queue behind it: a frame nobody was there to hear is gone. If
it has to survive being offline, write it down and let the fork ask on startup.

`broadcast` reaches every signed-in launcher. `is_online` and `connected` answer who is
there, and need only `launcher = ["read"]`.

## What the payload looks like

That is between your fork and your module. The protocol carries it as opaque JSON on
purpose: a described shape would make every change to your fork a release of the wire
contract, and the point of doing this in a module is that it does not.

## The stock launcher ignores all of it

An unknown frame is dropped where it is parsed — no error, no dropped connection. That is
what lets you roll a fork out to some machines and not others: players on the stock build
simply never see the frames, and nothing breaks for them.

:::caution[The launcher is AGPL]
Changing it is expected; so is publishing what you changed. The module you write against
this API is yours and separate — it talks over a documented boundary and links nothing.
:::
