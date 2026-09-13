<script setup lang="ts">
/**
 * The module's page in the panel.
 *
 * Ordinary Vue: `<script setup>`, reactivity, types. The components come from
 * `@noroproject/module-ui` — they are the very atoms the panel is drawn with,
 * so this matches the rest of it without a line of styling.
 *
 * The injected context reaches your own endpoints — the ones declared with
 * `#[route]` in src/lib.rs — and translates your keys. What it offers:
 * https://noroproject.github.io/noro-shared/guides/mini-apps/
 */
import { NoroCard, useNoro } from '@noroproject/module-ui'

const noro = useNoro()
</script>

<template>
    <NoroCard icon="i-lucide-box" :title="noro.t('mod-{{id}}-title', '{{name}}')">
        <p class="text-sm opacity-70">Nothing here yet.</p>
    </NoroCard>
</template>
