---
title: cargo noro
description: Creating, building, checking and packaging a module from the command line.
---

```bash
cargo install --git https://github.com/NoroProject/noro-shared.git cargo-noro
```

It installs as a cargo subcommand. Every command below works from anywhere inside the
module — the tool walks up until it finds `manifest.toml`, the way `cargo build` finds
`Cargo.toml`.

## `cargo noro new <id>`

Creates a module: manifest, crate, locales, a mini-app, and a `src/lib.rs` with one
working example of each thing a module can declare.

| Flag | Effect |
|---|---|
| `--ui vue` | a Vue component the panel mounts inside itself (default) |
| `--ui none` | no screen — events, endpoints and tasks only |
| `--name "My Module"` | the display name; derived from the identifier otherwise |
| `--path DIR` | where to put it; a directory named after the module otherwise |

The identifier is asked for once because it travels into four places that have to agree:
the crate name, the manifest `id`, the prefix of every locale key, and the mini-app
package. Changing it later is work in all four.

The template is built into the binary rather than fetched, so the command works offline
and gives you exactly what this version of the tool can check and build.

## `cargo noro check`

Reads the manifest with the **same types the master uses** — `noro-module-abi`, not a
copy — and reports what would be refused at install time:

- a capability action that does not exist (`players = ["banish"]`), listing the ones that do
- a locale key without its `mod-<id>-` prefix
- a key the manifest points at that no `.ftl` defines
- a permission node outside `noro.module.<id>.`
- migrations with no leading number, or two with the same one
- a required ABI version this master would not accept

None of this needs a running master, and none of it is a second implementation of the
rules. The value is the timing: without it these are all install-time errors, found after
a build, an upload and a click.

## `cargo noro build [--debug]`

The mini-app first — its errors read better and wasm takes longer — then the wasm, then
`wasm-opt -Oz` if it is installed. Missing `wasm-opt` is a line in the output, not a
failure: the package is simply larger.

`--debug` skips the release profile and the optimiser. Faster to build, roughly four
times the size, and not what you ship.

## `cargo noro package [--debug]`

Build, then zip into `dist/<id>.noromod`. Prints the size and the sha256 the master will
identify the package by.

The archive is written directly rather than by calling `zip`: that is not installed
everywhere, and on macOS it slips `__MACOSX` entries in, which makes the master see files
you never put there. Entries are sorted, so building the same sources twice gives the
same hash.

## `cargo noro dev`

Rebuilds on every save, for the master's [development mode](../../guides/dev-mode/). A
failed build prints the error and keeps watching — you fix the typo and save again rather
than restarting the loop.
