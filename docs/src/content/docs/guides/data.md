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

Then query them:

```rust
db::execute(Query::new("INSERT INTO orders (player_id, total) VALUES ($1, $2)")
    .bind(player.to_string())
    .bind(500))?;

for row in db::query(Query::new("SELECT * FROM orders WHERE paid_at > $1").bind(since))? {
    let total = row["total"].as_i64().unwrap_or(0);
}

let count: Option<i64> = db::scalar(Query::new("SELECT count(*) FROM orders"))?;
```

Rows come back as objects keyed by column name, not as positional arrays: a query that
grows a column should not silently shift every index in the code reading it.

### Values are bound, never spliced

`$1`, `$2`, … and `.bind(…)`. Assembling a statement with `format!` works right up to the
first username with an apostrophe in it, and that is the good case.

JSON types map to Postgres as you would expect — a string to `text`, an integer to
`bigint`, a float to `double precision`, a bool to `boolean`, null to NULL — and an
object or an array to `jsonb`.

### What you cannot reach

Queries run with the search path pinned to your schema and under a restricted Postgres
role with no rights anywhere else. `SELECT * FROM users` is refused **by the database**,
not by a check somebody has to remember to write:

```text
ERROR: permission denied for table users
```

Platform data comes from the typed domains, where capabilities are checked and the
projection is deliberate. If the role could not be created — some managed Postgres will
not allow it — the master says so at startup and the search path is the only boundary
left; it does not pretend otherwise.

### Limits

A query returning more than ten thousand rows is refused rather than returned: the rows
travel into the sandbox's memory, and `SELECT *` over a million-row table would take the
module down with it. Add a `LIMIT`.

## Migrations are checksummed

Editing a migration that has already been applied is how you stop a module from loading.
Add another file instead. This is the same rule the master's own migrations follow, for
the same reason.
