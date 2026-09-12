---
title: Your first module
description: From an empty directory to a running module in the panel.
---

## What you need

- The Rust toolchain with the wasm target: `rustup target add wasm32-unknown-unknown`
- [Bun](https://bun.sh) — only if your module has a mini-app
- Optionally [`wasm-opt`](https://github.com/WebAssembly/binaryen); it cuts the `.wasm`
  down by about a third, and the build works without it

## Get the tool

```bash
cargo install --git https://github.com/NoroProject/noro-shared.git cargo-noro
```

It becomes a cargo subcommand: `cargo noro …` from anywhere inside your module.

## Create it

```bash
cargo noro new my-module
cd my-module
```

You get a working module: a settings field, a startup step, an event handler and an
endpoint, one of each and all of them deletable. `--ui none` leaves out the mini-app;
`--name "My Module"` sets the display name.

The identifier is asked for once because it travels: into the crate name, the manifest,
the prefix of every locale key and the mini-app package. `cargo noro new` fills all four
in at the same time.

## Build it

```bash
cargo noro package
```

The result is `dist/my-module.noromod`. That single file is what you upload.

`cargo noro check` runs first, every time. It reads the manifest with the same types the
master uses, so an unknown capability action, a locale key without its prefix or a
mini-app pointing at a file that is not there are all refused here — before the upload,
rather than after it.

## Install it

1. Open the admin panel → **Modules** → drop the `.noromod` file in
2. Read what it asks for, and grant what you agree to
3. Enable it

Your module's pages, endpoints and subscriptions appear immediately. The master is not
restarted at any point.

:::tip[Do not rebuild a package while developing]
Point the master at your module's folder instead — admin panel → **Modules** → **Dev
mode**. It then reads the wasm, the mini-app and the locales straight from disk and
reloads the module whenever you rebuild — `cargo noro dev` is the other half. See
[development mode](../../guides/dev-mode/).
:::

## What the identifier is used for

`id` in `manifest.toml` is not a display name. It becomes:

- the Postgres schema `mod_<id>` holding your tables
- the prefix every locale key of yours must carry, `mod-<id>-…`
- the path of your endpoints, `/api/modules/<id>/…`
- the root of your permission nodes, `noro.module.<id>.…`

Hence `[a-z0-9-]` only, and hence changing it later means changing all four. Choose it
once.
