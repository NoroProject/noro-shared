---
title: Your first module
description: From the template repository to a running module in the panel.
---

## What you need

- The Rust toolchain with the wasm target: `rustup target add wasm32-unknown-unknown`
- [Bun](https://bun.sh) — only if your module has a mini-app
- `zip`, and optionally [`wasm-opt`](https://github.com/WebAssembly/binaryen) (it cuts
  the `.wasm` down by about a third; the build works without it)

## Start from the template

Press **Use this template** on
[NoroProject/noro-module-template](https://github.com/NoroProject/noro-module-template),
clone the result, and give your module its identity:

```bash
./scripts/rename.sh my-module "My Module"
```

That one script renames the crate, the manifest `id`, the locale key prefix and the
mini-app package. Doing it by hand means missing one of the four and finding out at
install time.

## Build it

```bash
./scripts/build.sh
```

The result is `dist/my-module.noromod`. That single file is what you upload.

## Install it

1. Open the admin panel → **Modules** → drop the `.noromod` file in
2. Read what it asks for, and grant what you agree to
3. Enable it

Your module's pages, endpoints and subscriptions appear immediately. The master is not
restarted at any point.

:::tip[Do not rebuild a package while developing]
Point the master at your module's folder instead — admin panel → **Modules** → **Dev
mode**. It then reads the wasm, the mini-app and the locales straight from disk and
reloads the module whenever you rebuild. See [development mode](../../guides/dev-mode/).
:::

## What the identifier is used for

`id` in `manifest.toml` is not a display name. It becomes:

- the Postgres schema `mod_<id>` holding your tables
- the prefix every locale key of yours must carry, `mod-<id>-…`
- the path of your endpoints, `/api/modules/<id>/…`
- the root of your permission nodes, `noro.module.<id>.…`

Hence `[a-z0-9-]` only, and hence changing it later means changing all four. Choose it
once.
