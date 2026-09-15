---
title: cargo noro
description: Creating, building, checking and packaging a module from the command line.
---

```bash
cargo install cargo-noro
```

It installs as a cargo subcommand. Every command below works from anywhere inside the
module — the tool walks up until it finds `manifest.toml`, the way `cargo build` finds
`Cargo.toml`.

## `cargo noro new <id>`

Creates a module: manifest, crate, locales, a mini-app, and a `src/lib.rs` with one
working example of each thing a module can declare — or nothing at all, with `--bare`.

| Flag | Effect |
|---|---|
| `--ui vue` | a Vue component the panel mounts inside itself (default) |
| `--ui none` | no screen — events, endpoints and tasks only |
| `--bare` | an empty `impl Module {}` instead of the examples |
| `--name "My Module"` | the display name; derived from the identifier otherwise |
| `--path DIR` | where to put it; a directory named after the module otherwise |

`--bare` is for when you know what you are writing. The examples are quick to read but
somebody else's code in your project: you recognise them as foreign before you delete
them. The bare crate carries a comment listing the five attributes and a link to the
catalog, and nothing else — the screen it ships, if you asked for one, calls no endpoint,
because there is none to call.

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

## `cargo noro key new` · `key show`

Makes the key your packages are signed with, and prints its public half.

Signing is not about a master trusting a stranger: the owner installs the file
themselves, and a file they put there is already familiar. It is about the **update** — a
package with the same identifier and a different key is a different author, and the
master refuses it rather than accepting it quietly. It already holds that module's
granted capabilities, its schema and its players' data.

The key lands in `~/.config/noro/keys/<id>.key`, not next to the project: there it would
travel into git with the first `git add .`, and that would be the end of the signature.
Keep a copy somewhere safe — without it, shipping an update to that module means the
operator has to remove it and install anew.

## `cargo noro package [--debug] [--sign]`

Build, then zip into `dist/<id>.noromod`. Prints the size and the sha256 the master will
identify the package by.

`--sign` adds `signature.toml`. What is signed is the **entries** — each name and its
bytes, sorted — not the archive: a zip carries timestamps, ordering and a compression
method of its own, so two identical builds would differ, and you would see "wrong key"
after an ordinary rebuild.

An unsigned package installs normally. Going from unsigned to signed is fine — you made a
key. The other way is not: that is exactly what a substitution would look like, and
nothing distinguishes it from a forgotten key.

The archive is written directly rather than by calling `zip`: that is not installed
everywhere, and on macOS it slips `__MACOSX` entries in, which makes the master see files
you never put there. Entries are sorted, so building the same sources twice gives the
same hash.

## `cargo noro dev`

Rebuilds on every save, for the master's [development mode](../../guides/dev-mode/). A
failed build prints the error and keeps watching — you fix the typo and save again rather
than restarting the loop.

## `cargo noro add <subcommand>`

Scaffolds module components in `src/lib.rs` and `migrations/`:

| Command | What it creates |
|---|---|
| `cargo noro add event <name> [--priority P]` | Appends an event handler to `src/lib.rs`. |
| `cargo noro add route <METHOD> <path>` | Appends an HTTP route handler (e.g. `POST /buy`) to `src/lib.rs`. |
| `cargo noro add task <name> [every]` | Appends a scheduled task to `src/lib.rs` (interval or cron). |
| `cargo noro add migration <name>` | Creates the next numbered SQL migration in `migrations/`. |

## `cargo noro ui`

Launches a local Vite development server (`http://localhost:5173`) with HMR and an integrated Noro test environment (`window.__noroUi`). Allows developing and previewing your Vue mini-app interface locally without running a master server.

## `cargo noro gen-types [-o FILE]`

Generates TypeScript interface definitions from Rust structs in your module's crate (e.g. structures decorated with `Serialize` or `Deserialize`).

| Option | Description |
|---|---|
| `-o`, `--output` | Destination file for generated TypeScript definitions. Defaults to `ui/types/module-generated.d.ts` (or `types/module-generated.d.ts`). |

Ensures type safety across the boundary between your Web/UI components and module backend endpoints.
