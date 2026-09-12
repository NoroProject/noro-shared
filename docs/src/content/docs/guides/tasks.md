---
title: Scheduled work
description: Running something on a timer, and what the schedule does and does not promise.
---

```rust
#[task("1h")]
fn payout() -> Result<()> {
    for p in roster::online()? {
        bank::transfer(server, treasury, account_of(p)?, 100, "hourly")?;
    }
    Ok(())
}
```

The interval is `30s`, `5m`, `1h`, `2d` — a number and a unit. It is checked when the
module is installed, so a typo is refused there rather than becoming a task that never
runs.

## What the interval means

Every hour of the master's uptime, not on the hour. A task with `1h` first run at 10:30
goes again at 11:30. "At 04:00" is a schedule rather than an interval, and that is what
[restart schedules](../platform/) are for.

The first run is one interval after the master starts, not at startup. A module that
wants something done at boot has `#[init]`, which runs once with capabilities already
granted.

Nothing is remembered across a restart: "once an hour" means once an hour of running. A
master restarted every twenty minutes runs an hourly task never — if that matters to
your module, keep the last run in your own store and check it.

## What it costs

A task gets thirty seconds, six times what an event handler gets: it is not standing in
the way of a live request, so it may go out to the network or walk its own tables.

Two runs of the same task cannot overlap. The module's worker handles one call at a time,
so a task still running when the next tick arrives simply delays it rather than running
twice.

A failing task counts against the module exactly as a failing event handler does: ten
failures in five minutes and the module is switched off. A task that throws every hour
will take a while to reach that, which is the point — the breaker is there for modules
failing in a loop, not for one that fails occasionally.

## What it is not

There is no way to run a task on demand, to skip one, or to ask when it last ran. If your
module needs that, it needs its own state and its own endpoint — and at that point the
task is just the thing that calls them.
