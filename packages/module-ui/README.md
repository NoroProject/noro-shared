# @noroproject/module-ui

The Noro panel's components, for module mini-apps — type declarations.

```bash
bun add -d @noroproject/module-ui
```

At runtime the package is not there: it is marked external, and its imports
become lookups in the panel's globals. That is what gives a module the **same**
copy of Vue the panel uses, and keeps its bundle at kilobytes.

```js
// vite.config.mjs
export default defineConfig({
    plugins: [vue()],
    build: {
        lib: { entry: 'ui/app.js', formats: ['iife'], name: '__noroModuleExport' },
        rollupOptions: {
            external: ['vue', '@noroproject/module-ui'],
            output: { globals: { vue: '__noroVue', '@noroproject/module-ui': '__noroUi' } },
        },
    },
})
```

```vue
<script setup lang="ts">
import { AtomBadge, NoroCard, useNoro } from '@noroproject/module-ui'

const noro = useNoro()
const points = await noro.api<number>('/points')
</script>

<template>
    <NoroCard icon="i-lucide-gift" :title="noro.t('mod-shop-title')">
        <AtomBadge tone="success">{{ points }}</AtomBadge>
    </NoroCard>
</template>
```

The components and props listed here are the panel's public contract: it can
grow, but what is already in it does not get renamed. The panel checks them
against its real components during typecheck, so the promise cannot drift from
reality unnoticed.

How to write a whole module: **https://noroproject.github.io/noro-shared/**
