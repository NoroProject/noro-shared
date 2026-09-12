---
title: Development mode
description: Editing a module without repackaging it or restarting the master.
---

Building a `.noromod` and uploading it for every change is not a development loop. Point
the master at your folder instead.

## Turning it on

Admin panel → **Modules** → **Dev mode**, and give it the absolute path to your module's
directory — the one with `manifest.toml` in it.

From then on the master reads three things straight from disk:

- the compiled wasm, from `target/wasm32-unknown-unknown/release/`
- the mini-app, from `web/`
- your locales, from `locales/`

## The loop

```bash
./scripts/dev.sh
```

That rebuilds on every save. The master notices the new wasm and **reloads the module
itself** — it re-reads the declaration, so a newly added event handler or endpoint starts
working without anything else happening. The master is not restarted at any point.

For the mini-app there is nothing to reload: reload the browser tab.

## What it watches, and how

A one-second poll, not filesystem notifications. `notify`-style watchers behave
differently on macOS and Linux, and both stumble over the atomic rename a compiler does
when writing output — you get told about a file that is momentarily not there. A poll is
duller and does not lie.

## What it does not skip

Dev mode replaces the package, not the install. Capabilities are still what the operator
granted, the manifest is still validated, and your declaration is still checked — a
subscription to an event that does not exist fails in dev mode exactly as it would on
install.

:::tip[Browser caching will waste your afternoon]
The master sends `no-cache` on the bridge, the styles and your mini-app files. Safari in
particular ignores this more often than you would expect; if a change refuses to appear,
check that you are not looking at a cached bundle before you look at your code.
:::

## Before you ship

Build the real package once and install it the normal way:

```bash
./scripts/build.sh
```

Dev mode reads loose files; a package is a zip with a manifest the master validates on
entry. They are close but not identical, and the difference is worth finding before
somebody else does.
