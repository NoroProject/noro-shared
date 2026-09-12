---
title: Errors
description: The error you return, the errors you receive, and the codes the master answers with.
---

## One shape, both directions

The master returns `ModuleError` to a module, and a module returns `ModuleError` to the
master. One type either way, so there is never a second set to keep in mind.

```rust
pub struct ModuleError {
    pub kind: ErrorKind,
    pub message: String,
    pub code: Option<i32>,   // the master's code, when the error came from there
}
```

| `ErrorKind` | Means | Constructor |
|---|---|---|
| `CapabilityDenied` | the module was not granted what it asked for | — |
| `NotFound` | the object is not there | `ModuleError::not_found(…)` |
| `Invalid` | the arguments do not pass | `ModuleError::invalid(…)` |
| `Conflict` | the state does not allow it — insufficient funds, a duplicate | `ModuleError::conflict(…)` |
| `Quota` | the module's quota is exceeded | — |
| `Internal` | a failure inside the master | — |

Returning `Err` from a handler is the normal way to fail. Panicking is caught, becomes a
module failure, and counts against you — see the circuit breaker below.

## Codes the master answers with

Your endpoints answer with these, and they show up in the panel's call log.

| Code | Name | When |
|---|---|---|
| `1501` | `module_not_found` | no module by that id |
| `1502` | `module_disabled` | the module exists but is switched off |
| `1503` | `module_cancelled` | a handler cancelled the action |
| `1504` | `module_timeout` | the handler did not answer in time |
| `1505` | `module_failed` | the handler panicked or the sandbox trapped |
| `1506` | `module_capability_denied` | a call into something ungranted |
| `1507` | `module_quota` | a limit was hit |
| `1508` | `module_bad_package` | the `.noromod` is malformed or its manifest is invalid |
| `1509` | `module_api_mismatch` | the module was built against a different ABI major |
| `1510` | `module_migration_failed` | a migration of your schema failed |
| `1511` | `module_route_not_found` | no endpoint answers that method and path |

## The circuit breaker

Ten failures in five minutes and the module is disabled automatically, with a line in the
audit log and a notification in any open admin tab. This is deliberate: a module failing
in a loop is worse than a module that is off, and nobody watches logs at three in the
morning.

What counts as a failure: a panic, a trap, a timeout. An `Err` you returned on purpose
does not.

## Timeouts

A handler gets five seconds. `#[init]` gets thirty, because setting up on first enable
legitimately takes longer than answering an event.

Two timeouts guard a call, not one. The sandbox's own epoch interrupt stops a module
spinning in its own code — but it cannot stop one that is blocked *inside* a host
function, so there is a second, outer deadline for that case.
