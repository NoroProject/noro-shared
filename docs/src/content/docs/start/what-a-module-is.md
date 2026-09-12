---
title: What a module is
description: How modules run, what they can reach, and why the sandbox is there.
---

A module is a single `.noromod` file. The instance owner uploads it in the admin panel,
grants it capabilities, and enables it. Nothing is recompiled and the master is not
restarted.

## Where the code runs

Inside the master process, compiled to WebAssembly and executed in a sandbox. That has
two consequences worth understanding before you write anything.

**Calls are cheap.** When your handler asks for a player, it reaches the same code that
serves the admin panel — no router, no middleware, no network. Bytes do cross the wasm
boundary, because that is the only way anything crosses it, but that is a `memcpy` and a
JSON parse measured in microseconds.

**The environment is not there.** No sockets, no files, no threads, no clock, no
entropy. This is not a restriction the platform added on top; it is what
`wasm32-unknown-unknown` is. See [what will not
compile](#what-will-not-compile) below.

## Why a sandbox at all

Not to protect the instance from you. The owner runs the master, creates the server
builds, and installs the modules themselves — in that model there is no untrusted author
to defend against.

The sandbox is there so a module can be **loaded as a file** without rebuilding the
master, and so its code can be published without handing out the master. Those two
properties are what make module authoring possible in the first place.

## What a module can reach

| Through | What for |
|---|---|
| `players` | finding a player by id, username, Minecraft uuid, Discord or any linked login; banning, renaming, skins and capes |
| `roles` | reading roles, and granting or revoking them |
| `permissions` | asking what a player effectively has, and granting personal permissions |
| `access` | letting a player into a server or one of its client builds |
| `servers` | servers, their builds, their game servers, and maintenance mode |
| `punish` | bans, mutes and warnings, issued the way staff issue them |
| `bank` | accounts, balances and transfers on a hub |
| `chat` | private messages, announcements and kicks, in game |
| `identities` | linked logins: Discord, Twitch, anything else |
| `db` | your own tables, in your own Postgres schema |
| `http` | requests to hosts you listed in the manifest |
| `store` | your own key-value data, scoped to the instance, a server or a player |
| `log` | lines into the master's `tracing`, tagged with your module |
| `#[event]` | ~50 events across players, access, infrastructure, moderation and economy |
| `#[route]` | your own endpoints under `/api/modules/<id>/…` |
| mini-apps | your own pages in the admin panel, the hub or the cabinet |

Every domain is gated by a capability. The manifest says what you ask for; the operator
decides what you get, and may give less. A call into something ungranted returns
`CapabilityDenied` naming what is missing.

## What will not compile

Pure computation works as is — `regex`, `rust_decimal`, `serde`, `sha2`, `base64`.
Anything that reaches into the environment does not:

| What a library needs | What to use instead |
|---|---|
| Network (`reqwest`, `tokio`) | [`http::send`](../../guides/platform/#reaching-outside), with hosts allow-listed in the manifest |
| System clock (`chrono` with `clock`) | `noro_sdk::now()` |
| Randomness (`uuid/v4`, `rand`) | identifiers come from the master |
| Files, threads, processes | `store` and your own Postgres schema |

Dependencies with those features usually only need `default-features = false` — that is
exactly what the SDK does with `uuid` and `chrono`.

Third-party crates are added with a plain `cargo add` and compiled into your `.wasm`.
There is no shading, no relocation and no dependency declaration in the manifest: each
module carries its own copy, so two modules with different versions of the same library
cannot conflict.
