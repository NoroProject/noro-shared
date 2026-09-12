---
title: Storing data
description: The scoped key-value store, and the state of your own Postgres schema.
---

## The key-value store

Your module gets a store with three scopes. The scope is part of the key rather than a
filter, so a player's `points` and a server's `points` never collide and you never splice
identifiers into strings yourself.

```rust
store::instance().set("last_payout", &now())?;
store::server(server_id).set("cfg", &ShopCfg { tax: 5 })?;
store::user(player_id).incr("points", 10)?;

let cfg: Option<ShopCfg> = store::server(server_id).get("cfg")?;
let points = store::user(player_id).get_or::<i64>("points")?;
```

Values are anything `serde` can handle. Requires `store = true`.

### get returns None only when the key is absent

If a value is there but does not parse into the type you asked for, that is an error, not
a `None`. A silent `None` would be data loss you notice a week later.

### incr is one operation

```rust
let total = store::user(id).incr("points", 10)?;
```

Not a read followed by a write. Event handlers can run at the same time, and a
read-add-write trio would lose accruals. If what is under the key is not a number, the
call fails rather than overwriting it.

### Listing

```rust
for (key, value) in store::instance().list("payout:", 100, 0)? {
    // …
}
```

By prefix, in pages, capped at 500 per call. There is no unpaginated read — of anything,
anywhere in this platform.

### Lifetime

Data under `user` goes when the player does; under `server`, when the server build does.
Everything goes when your module is uninstalled.

## Your own Postgres schema

Ship `migrations/0001_init.sql` in the package and the master creates a schema
`mod_<your_id>` and applies them, tracking what it has applied. On uninstall the schema is
dropped with everything in it.

```sql
-- migrations/0001_init.sql
CREATE TABLE orders (
    id          bigserial PRIMARY KEY,
    player_id   uuid NOT NULL,
    total       bigint NOT NULL,
    paid_at     timestamptz
);
```

:::caution[The tables exist; you cannot query them yet]
Migrations run and the schema is real, but the host function for running your own SQL is
not in place. So today you can create tables and not read them.

Use the key-value store for anything you need now. Reach for a migration only when you
are laying groundwork for tables you will query later.
:::

When it does arrive, queries will run with the search path pinned to your schema and
under a restricted Postgres role, so `SELECT * FROM users` from a module will be refused.
Reading platform data goes through the typed SDK, where capabilities are checked — not
through SQL.

## Migrations are checksummed

Editing a migration that has already been applied is how you stop a module from loading.
Add another file instead. This is the same rule the master's own migrations follow, for
the same reason.
