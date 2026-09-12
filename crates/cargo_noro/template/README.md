# {{name}}

A Noro module. Documentation: <https://noroproject.github.io/noro-shared/>

```bash
cargo noro dev       # rebuild on every save, for the master's dev mode
cargo noro package   # dist/{{id}}.noromod — this is what you upload
cargo noro check     # the manifest, the locale keys, the layout
```

## Where things are

| Path | What it is |
|---|---|
| `manifest.toml` | who the module is and what it asks the operator for |
| `src/lib.rs` | events, endpoints, tasks — declared in code, not in the manifest |
| `ui/` | the mini-app; `web/` is its build output and is not committed |
| `locales/` | your Fluent keys, all starting with `mod-{{id}}-` |

## Before the first build

```bash
rustup target add wasm32-unknown-unknown
```

The module compiles to `wasm32-unknown-unknown`, where there is no clock, no
randomness, no sockets and no files. Everything it needs comes from the master
through the SDK — `now()` instead of `SystemTime`, `http::send` instead of a
socket. A crate that reaches for any of those will not build, and the error is
usually further from the cause than you would like.
