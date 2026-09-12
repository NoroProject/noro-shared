---
title: Mini-apps
description: Shipping your own pages — a Vue component in the panel, or a sandboxed page.
---

A module can bring its own UI. Declare it in the manifest and it appears as a section in
the admin panel, the server hub or the player's cabinet.

```toml
[[apps]]
placement = "cabinet"      # admin | hub | cabinet | widget
kind = "vue"               # vue | page
entry = "app.js"
title = "mod-my-module-title"
icon = "i-lucide-gift"
```

## Two kinds

| `kind` | How it runs | When to use it |
|---|---|---|
| `vue` | a Vue component the panel mounts in its own tree | the normal case |
| `page` (default) | a static page in a sandboxed iframe, talking over `postMessage` | when isolation matters more than convenience |

`page` is the default because isolation must not be lost to a forgotten field. `vue` is
the one you usually want: it gets the panel's components and its look, and its bundle is
a couple of kilobytes.

## Writing a Vue mini-app

Ordinary Vue with ordinary types:

```vue
<script setup lang="ts">
import { AtomBadge, EmptyState, NoroCard, useNoro } from '@noroproject/module-ui'
import { onMounted, ref } from 'vue'

const noro = useNoro()
const points = ref<number | null>(null)

onMounted(async () => {
    points.value = await noro.api<number>('/me')
})
</script>

<template>
    <NoroCard icon="i-lucide-gift" :title="noro.t('mod-my-module-title', 'Rewards')">
        <EmptyState v-if="points === null" icon="i-lucide-loader" title="Loading" bare />
        <AtomBadge v-else tone="success">{{ points }}</AtomBadge>
    </NoroCard>
</template>
```

```bash
bun add -d @noroproject/module-ui
```

Those are the panel's own components, so the page matches the rest of it without a line
of styling.

## What `useNoro()` gives you

```ts
const noro = useNoro()
```

| | |
|---|---|
| `api(path, options?)` | calls **your** endpoints and nobody else's — the panel builds the address |
| `t(key, fallback?)` | your locale keys |
| `user` | whoever opened the page, or `null` |
| `notify.ok()` / `notify.fail(e)` | the panel's own notifications |
| `navigate(path)` | moving around the panel |
| `can(node)` · `canAny([…])` · `canOn(node, serverId)` | what the viewer is allowed |
| `confirm({ title, text?, danger? })` | asks, with the panel's dialog |
| `format.duration` · `money` · `date` · `ago` | the panel's own formatting |
| `paged(path, opts?)` | a paginated list from your endpoint |
| `platform.*` | reading platform data |

### Deciding what to show

```ts
const canReset = noro.can('noro.module.shop.manage')
```

The same matcher the panel uses for its own menus, wildcards included —
`noro.admin.*` answers true for `noro.admin.modules.view`. This decides what to
**show**; the master decides what to allow, and your endpoint's `auth` checks again on
its side. A hidden button is a courtesy, not a boundary.

### Asking before doing

```ts
if (!await noro.confirm({ title: 'Reset points?', text: 'This cannot be undone.', danger: true })) {
    return
}
```

The dialog is drawn by the panel, so it looks like every other dialog and cannot be
covered by your own markup. Declining — or just closing it — resolves to `false`, so an
unanswered question reads as "no" rather than as an exception you have to catch.

### Paginated lists

```ts
const list = noro.paged<Order>('/orders', { perPage: 25 })
await list.load()
```

Your endpoint answers `{ items, total }` — the shape every list in this platform answers
with — and you get refs to bind straight into a template, with debounced search and page
state already wired. `list.refresh()` re-reads the current page after your own mutation.

### Reading platform data

```ts
const me = await noro.platform.me.profile()
const servers = await noro.platform.servers.list()
const feed = await noro.platform.hub.feed('survival', 1)
```

Functions, not addresses. The panel's endpoints are its own business: a module built
against `/api/me/hubs` would break the day that path changed, so what is promised is this
surface instead. There is `platform.call(method, args)` for anything the package does not
name yet.

Everything there is a **read**. Writing goes through your own endpoint on the Rust side,
where the capabilities the operator granted are checked — a write from here would travel
under the token of whoever happened to open the page.

Both kinds of mini-app get the identical surface. A sandboxed page reaches it over the
bridge rather than by fetching anything itself, which is why the addresses stay out of
the sandbox and there is only one implementation to keep correct.

### The build

Both `vue` and `@noroproject/module-ui` must be **external**, mapped to the globals the
panel publishes:

```js
export default defineConfig({
    plugins: [vue()],
    build: {
        outDir: 'web',
        lib: { entry: 'ui/app.js', formats: ['iife'], name: '__noroModuleExport', fileName: () => 'app.js' },
        rollupOptions: {
            external: ['vue', '@noroproject/module-ui'],
            output: { globals: { vue: '__noroVue', '@noroproject/module-ui': '__noroUi' } },
        },
    },
})
```

This is not a size optimisation — or not mainly. Bundling your own copy of Vue would give
your component its own reactivity instance, and the panel could not mount it into its
tree at all. The template ships with this config already correct.

## What the contract promises

`@noroproject/module-ui` holds type declarations only — the implementations are the
panel's. The components and props listed there will keep working: the panel has a
compile-time check asserting that its real components still satisfy every promise in the
package, and its typecheck fails naming the offending prop if they stop.

What you get: `NoroCard`, `EmptyState`, and the atoms `AtomBadge`, `AtomButton`,
`AtomCheckbox`, `AtomChip`, `AtomInput`, `AtomModal`, `AtomNumberInput`, `AtomSegmented`,
`AtomSelectMenu`, `AtomToggle`.

## Sandboxed pages

With `kind = "page"` you ship plain HTML and talk to the panel over a bridge the master
serves:

```html
<script src="/api/modules/bridge.js"></script>
<link rel="stylesheet" href="/api/modules/noro-ui.css" />
<script>
    noro.ready(async (ctx) => {
        const points = await noro.call('/me')
        document.body.textContent = `${points}`
    })
</script>
```

`noro.ready(fn)` is a function rather than an event on purpose: a page that loads fast
would otherwise miss the launch parameters entirely.
