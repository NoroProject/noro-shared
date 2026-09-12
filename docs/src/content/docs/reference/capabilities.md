---
title: Capabilities
description: What a module can ask for, what the operator grants, and what is wired up today.
---

A module **asks**; the operator **grants**. The two need not match — granting less than
was asked for is normal and supported. A call into something ungranted returns
`CapabilityDenied` naming what is missing; it does not crash the module.

```toml
[capabilities]
players = ["read"]
store = true
http = ["discord.com"]
```

An empty list is a denial. A domain you do not mention is closed.

## Status

The capability model, the manifest fields and the operator's grant screen are all in
place. The host functions behind each domain are not all there yet, and this table says
which are.

| Capability | Actions | Wired up |
|---|---|---|
| `players` | `read`, `ban`, `rename` | `read` — `players::get` / `players::require` |
| `store` | `true` | yes — the full key-value store |
| `identities` | `read`, `link` | not yet |
| `roles` | `read`, `grant` | not yet |
| `permissions` | `read`, `grant` | not yet |
| `access` | `grant` | not yet |
| `servers` | `read` | not yet |
| `gameservers` | `read`, `maintenance` | not yet |
| `builds` | `read`, `publish` | not yet |
| `bank` | `read`, `transfer` | not yet |
| `punish` | `issue`, `revoke` | not yet |
| `agent` | `tell`, `announce`, `kick` | not yet |
| `db` | `true` | schema and migrations yes, queries not yet |
| `http` | a host allow-list | not yet |

Asking for something in the second column is harmless — the manifest accepts it and the
operator sees it — but the call is not there to make. Build on the first two rows.

## Why these are words and not a bitmask

The operator reads this list in full before granting it. Anything that needs decoding
before it can be judged would not get judged.

## The HTTP allow-list

```toml
[capabilities]
http = ["discord.com", "*.example.org"]
```

Hosts, not URLs — a path or a port in there is refused at install time. `*.example.org`
matches subdomains but **not** `example.org` itself: a wildcard should not quietly grant
the parent nobody named.

When outbound HTTP does arrive it will resolve DNS and refuse private address ranges,
because otherwise an allow-list is bypassed by pointing a permitted name at `127.0.0.1`.
