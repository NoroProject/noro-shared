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

`useNoro()` gives you four things: `api(path, options?)` calls your own endpoints and
nobody else's — the panel builds the address; `t(key, fallback?)` looks up your locale
keys; `user` is whoever opened the page, or `null`; `notify.ok()` / `notify.fail()` raise
the panel's own notifications, and `navigate(path)` moves around it.

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
