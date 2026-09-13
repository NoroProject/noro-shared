# noro-sdk

Write a module for a [Noro](https://github.com/NoroProject/noro-shared) instance: events,
storage, your own endpoints and your own pages in the panel.

A module is one `.noromod` file the instance owner drops into the admin panel. It runs
sandboxed inside the master process and reaches the platform through host functions — a
call is a function call, not an HTTP request.

```rust
use noro_sdk::prelude::*;

pub struct Greeter;

#[noro::module]
impl Greeter {
    #[event]
    fn on_join(e: PlayerJoined) -> Result<()> {
        store::user(e.player.id).incr("joins", 1)?;
        Ok(())
    }

    #[route(GET, "/me")]
    fn me(req: HttpRequest) -> Result<i64> {
        Ok(store::user(req.require_user()?).get("joins")?.unwrap_or(0))
    }
}
```

```bash
cargo install cargo-noro
cargo noro new my-module
```

**Documentation: <https://noroproject.github.io/noro-shared/>** — guides, the event
catalog, every SDK call and what each needs granted. Also in Russian.

## Licence

MIT. Your module is yours: the crates on this boundary are permissive on purpose, so
what you write against them carries whatever terms you choose. Noro itself is
AGPL-3.0-only — see `LICENSING.md` in the repository.
