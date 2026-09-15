---
title: Mini-app API
description: Everything the panel hands a mini-app — every field of the context, in full.
---

A mini-app declared as `kind = "vue"` is mounted by the panel inside its own tree, and
everything it can reach arrives in one object. Take it with `useNoro()`:

```ts
import { useNoro } from '@noroproject/module-ui'

const noro = useNoro()
```

Nothing here is imported from the panel: the mini-app is built separately and knows none
of its paths. `@noroproject/module-ui` resolves at build time to what the panel already
has loaded, so the bundle carries no Vue and weighs kilobytes.

## `noro.api` — your own endpoints

```ts
const stats = await noro.api<Stats>('/me')
await noro.api('/reset', { method: 'POST', body: { hard: true } })
```

The path is relative to **your** module: `/me` becomes
`/api/modules/<your-id>/me`. Nothing else is reachable this way, and that is the point —
a mini-app talks to its own half and to nothing else.

The method defaults to `GET`, or `POST` when there is a body. The player's token is
attached for you. A refusal comes back as a thrown error whose message is what the master
said.

| Field | Type | Notes |
|---|---|---|
| `path` | `string` | leading slash optional |
| `options.method` | `'GET' \| 'POST'` | defaults by presence of a body |
| `options.body` | `object` | sent as JSON |

## `noro.ws` — realtime WebSocket

Full-duplex real-time communication between your mini-app and the module backend without manual socket setup.

```ts
// Subscribe to messages pushed by your module:
const unsubscribe = noro.ws.on<OrderUpdate>((data) => {
    console.log('Received frame:', data)
})

// Unsubscribe when done:
unsubscribe()

// Send a frame from the browser tab to your module:
noro.ws.send({ action: 'bid', amount: 100 })
```

| Method | Type | Notes |
|---|---|---|
| `ws.on<T>(callback)` | `(data: T) => void` | registers listener; returns unsubscribe function |
| `ws.send(payload)` | `(payload: any) => void` | pushes frame to module; received as `WebMessage` |

## `noro.platform` — reading the panel

Things the panel already knows and you should not re-implement: who the viewer is, what
servers exist, what a hub holds. Read-only, always.

```ts
const me = await noro.platform.me.profile()
const towns = await noro.platform.hub.towns(slug, 1)
```

Writes are deliberately absent. A write from here would carry the authority of whoever
opened the page, and the panel has no way to check what your module was granted — that
check lives in Rust, with the capability list. So writing is what your own endpoint is
for.

### The viewer

| Call | Answers |
|---|---|
| `me.profile()` | the full profile of whoever is looking |
| `me.hubs()` | hubs they are a member of |
| `me.punishments()` | their punishments |
| `me.sessions()` | their sessions in the panel and the launcher |
| `me.identities()` | linked Discord, Twitch and the rest |
| `me.tickets()` | their support conversations |
| `me.cape()` | their cape |

These need no permission: it is their own data.

### Players and servers

| Call | Answers |
|---|---|
| `players.byName(name)` | the public card of a player, or `null` |
| `servers.list()` | servers as the site shows them |
| `servers.rules(serverId)` | that server's rules |
| `rules.list()` · `rules.scopes()` | the rule catalogue |
| `legal.list()` · `legal.get(slug)` | legal documents |
| `capes.list()` | available capes |

### The hub

Every one of these takes the hub's `slug` first.

| Call | Answers |
|---|---|
| `hub.info(slug)` | the hub itself: what is turned on, what it is called |
| `hub.feed(slug, page?)` | the feed, paginated |
| `hub.members(slug, { page, q })` | members, searchable |
| `hub.towns(slug, page?)` · `hub.town(slug, town)` | towns |
| `hub.myTowns(slug)` | towns the viewer belongs to |
| `hub.court(slug, page?)` | court cases |
| `hub.fines(slug, page?)` | fines |
| `hub.cards(slug)` | the viewer's bank cards |
| `hub.communities(slug)` | communities they are in |
| `hub.mapPlayers(slug)` · `hub.mapMarkers(slug)` | the live map |

### `platform.call(method, args)`

The same dispatcher underneath, by name. There when a method exists and the typed facade
has not caught up; prefer the facade, because a name is something you can mistype and a
function is not.

## `noro.can`, `canAny`, `canOn` — what to show

```ts
if (noro.can('noro.admin.users.view')) { /* draw the button */ }
if (noro.canOn('noro.hub.moderate', serverId)) { /* draw the moderator tools */ }
```

These answer what to **show**, never what to allow. The decision is still the master's,
and your endpoint checks it again on arrival. A hidden button is a courtesy; a check in
the browser is not a lock.

`canOn` takes the permission first and the server second — the permission is the
question, the server only narrows it.

## `noro.t` — text

```ts
noro.t('mod-my-module-title', 'My module')
```

Looks the key up in the catalogue the panel loaded, module locales included. The second
argument is what to show when the key is missing — without it you would get the key
printed on screen, which is worse than English in a Russian panel.

## `noro.format` — numbers, money, time

| Call | Gives |
|---|---|
| `format.money(minor, currency)` | an amount in the hub's own currency |
| `format.duration(seconds)` | `45s`, `1m 30s`, `2h 15m` |
| `format.date(value)` | absolute date and time |
| `format.ago(value)` | "3 minutes ago", absolute past a week |

`duration` takes **seconds**, because that is the unit the master speaks in, and reads
them out down to the second.

## `noro.paged` — lists that page themselves

```ts
const top = noro.paged<Row>('/top', { perPage: 25 })
await top.load()
```

Wraps one of your own endpoints that answers `{ items, total }`. You get `items`,
`total`, `page`, `pages`, `search`, `pending`, `error` as refs and `load()`, `next()`,
`prev()` to move. Same behaviour as the panel's own lists, so the pager looks and acts
the same.

## `noro.confirm` — asking before something irreversible

```ts
const ok = await noro.confirm({
    title: noro.t('mod-my-module-wipe', 'Delete everything?'),
    text: noro.t('mod-my-module-wipe-hint'),
    danger: true,
})
if (!ok) return
```

The panel's own dialog, not `window.confirm`: it matches the rest of the page and cannot
be suppressed by a browser setting. `danger: true` colours the confirming button red.

## `noro.notify`

`notify.ok(text)` and `notify.fail(error)` raise the same toasts the panel uses. `fail`
takes the error object as it came — it knows how to read a master error and show the
message inside.

## `noro.user`

`{ id, username, mc_username }` of the viewer, or `null` when nobody is signed in. A
projection, not the profile: your mini-app has no business with their permissions,
linked accounts or texture URLs, and the narrow shape is what keeps a panel field from
wandering into your code by accident.

## `noro.navigate(path)`

Moves the panel to one of its own pages. Absolute paths only — an external address here
would be a redirect a player did not ask for.

## `noro.vue`

The panel's own Vue, the same copy the panel is running. Use it when you need `h`,
`defineComponent` or `provide` at the entry point; inside your components, import from
`vue` as usual — the build maps it to this.

## Components

`@noroproject/module-ui` re-exports the atoms the panel is drawn with, so a mini-app
matches it without a line of styling:

`AtomBadge` · `AtomButton` · `AtomCheckbox` · `AtomChip` · `AtomInput` · `AtomModal` ·
`AtomNumberInput` · `AtomSegmented` · `AtomSelectMenu` · `AtomToggle` · `EmptyState` ·
`NoroCard`

They are the panel's real components, not copies — a change to how a button looks reaches
your mini-app with the panel, not with your next release.

## Where a mini-app can appear

Declared per app in the manifest:

```toml
[[apps]]
placement = "cabinet"
kind = "vue"
entry = "app.js"
title = "mod-my-module-title"
icon = "i-lucide-box"
```

| `placement` | Where |
|---|---|
| `admin` | a section in the admin panel |
| `cabinet` | a page in the player's cabinet |
| `hub` | a section in a server's hub |
| `widget` | a card inside somebody else's page; needs `slot` |

`kind = "page"` is the alternative to `vue`: a static page in a sandboxed iframe, its own
origin, talking over a bridge. It sees less and can break less. `vue` is the default
choice because the owner installs the modules themselves — for somebody else's code, the
sandbox remains.

`permission` on an app hides the entry from whoever lacks it. As everywhere here: hiding,
not forbidding.
