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

| Capability | Actions | What you can call |
|---|---|---|
| `players` | `read` | `players::get`, `players::require` |
| | `ban` | `players::ban`, `players::unban` |
| `store` | `true` | the whole key-value store |
| `roles` | `read` | `roles::list`, `roles::get`, `roles::of` |
| | `grant` | `roles::grant`, `roles::revoke` |
| `permissions` | `read` | `permissions::has`, `permissions::effective` (and the `_on` variants) |
| | `grant` | `permissions::grant`, `permissions::revoke` |
| `access` | `grant` | `access::allow_join`, `access::allow_build`, and their `revoke_*` |
| `servers` | `read` | `servers::list`, `get`, `by_slug` |
| `builds` | `read` | `servers::builds`, `servers::published_build` |
| `gameservers` | `read` | `servers::game_servers`, `servers::game_server` |
| | `maintenance` | `servers::set_maintenance` |
| `players` | `rename` | not yet |
| `identities` | `read`, `link` | not yet |
| `builds` | `publish` | not yet |
| `bank` | `read`, `transfer` | not yet |
| `punish` | `issue`, `revoke` | not yet |
| `agent` | `tell`, `announce`, `kick` | not yet |
| `db` | `true` | schema and migrations yes, queries not yet |
| `http` | a host allow-list | not yet |

Asking for one of the `not yet` actions is harmless — the manifest accepts it and the
operator sees it — but there is no call to make with it.

## Writes are audited as if you were staff

A role granted or a player banned by a module is written to the audit log under **the
same action** an operator's would use, signed `модуль «your-id»`. This is deliberate: a
ban has to appear in the list staff already read, not in a feed of its own. Reads are not
audited; they go in the module's call log instead.

Nothing a module writes is attributed to a person. `granted_by` stays empty rather than
naming whoever enabled the module — they did not make this decision, and pinning every
later action on them would be a lie the audit log cannot take back.

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
