---
title: SDK domains
description: Every call the SDK offers, with what it needs granted.
---

Each domain is one module of `noro_sdk`, and one line in `[capabilities]`. An
action the operator withheld answers `CapabilityDenied` naming what is missing;
it does not crash your module.

Full signatures, types and the longer explanations are in the
[rustdoc](/noro-shared/api/noro_sdk/).

| Domain | What it is | Calls |
|---|---|---|
| [`access`](./sdk/access/) | Access to servers and builds. | 6 |
| [`agent`](./sdk/agent/) | Things done to a player in the running game. | 4 |
| [`bank`](./sdk/bank/) | The hub's bank. | 6 |
| [`bots`](./sdk/bots/) | A face of your own to write from. | 2 |
| [`builds`](./sdk/builds/) | Client builds and the files inside them. | 9 |
| [`cases`](./sdk/cases/) | Moderation cases. | 5 |
| [`db`](./sdk/db/) | Your own tables. | 6 |
| [`dm`](./sdk/dm/) | Private messages between players. | 5 |
| [`events`](./sdk/events/) | Events of your own, for other modules to handle. | 2 |
| [`files`](./sdk/files/) | The shared file store. | 4 |
| [`http`](./sdk/http/) | Calling out to the internet. | 2 |
| [`hub`](./sdk/hub/) | A server's hub: the feed, its members, towns, market, court, petitions and | 20 |
| [`identities`](./sdk/identities/) | Linked logins: Discord, Twitch, Google, anything else a player signs in with. | 4 |
| [`instance`](./sdk/instance/) | Instance settings. | 3 |
| [`launcher`](./sdk/launcher/) | Talking to the launcher. | 4 |
| [`news`](./sdk/news/) | The instance's news. | 6 |
| [`permissions`](./sdk/permissions/) | Permissions. | 8 |
| [`players`](./sdk/players/) | Players. | 11 |
| [`punish`](./sdk/punish/) | Sanctions. | 8 |
| [`restarts`](./sdk/restarts/) | Restart schedules of game servers. | 3 |
| [`roles`](./sdk/roles/) | Roles. | 8 |
| [`roster`](./sdk/roster/) | Who is in game right now. | 3 |
| [`servers`](./sdk/servers/) | Server builds, their client builds, and the game servers behind them. | 7 |
| [`sessions`](./sdk/sessions/) | A player's sessions — the launchers and browsers they are signed in from. | 3 |
| [`store`](./sdk/store/) | The module's own storage. | 3 |
| [`telemetry`](./sdk/telemetry/) | How a game server is holding up. | 1 |
| [`tickets`](./sdk/tickets/) | Conversations with players. | 6 |
| [`web_ws`](./sdk/web_ws/) | Talking to the player's web browser tab. | 4 |
