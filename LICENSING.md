# Which licence applies to what

This repository carries two, on purpose.

| Crate | Licence | Why |
|---|---|---|
| `noro-module-abi` | MIT | linked into every module |
| `noro-sdk` | MIT | linked into every module |
| `noro-sdk-macros` | MIT | expands into every module |
| `cargo-noro` | MIT | the tool authors run |
| `schema` | AGPL-3.0-only | part of Noro itself |
| `i18n` | AGPL-3.0-only | part of Noro itself |
| `mc_mod_utils` | AGPL-3.0-only | part of Noro itself |

## Why the module crates are MIT

A module links the SDK into its own `.wasm`. Under a copyleft licence that would
make every module a derivative work and force it open — which is a decision about
**other people's** software, not about Noro.

What AGPL is here to do is stop a modified Noro from being run as a closed
service. A module written by somebody else is not a modified Noro; it is their
program talking to ours over a documented boundary. So the crates on that
boundary are MIT and the author of a module picks their own licence, closed
included.

The master, the launcher and the crates they are built from stay AGPL-3.0-only,
with the trademark terms in `TRADEMARK.md` of the launcher repository.

## If you are writing a module

You need no permission and owe no licence. Depend on `noro-sdk`, ship whatever
terms you like.
