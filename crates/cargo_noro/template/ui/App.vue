<script setup lang="ts">
/**
 * The module's page in the player's cabinet.
 *
 * Ordinary Vue: `<script setup>`, reactivity, types. The components come from
 * `@noroproject/module-ui` — they are the very atoms the panel is drawn with,
 * so this matches the rest of it without a line of styling.
 */
import { AtomBadge, EmptyState, NoroCard, useNoro } from '@noroproject/module-ui'
import { onMounted, ref } from 'vue'

interface Stats {
    joins: number
    greeted: boolean
}

const noro = useNoro()

const data = ref<Stats | null>(null)
const error = ref<string | null>(null)
const pending = ref(true)

onMounted(async () => {
    try {
        data.value = await noro.api<Stats>('/me')
    } catch (e) {
        error.value = e instanceof Error ? e.message : String(e)
    } finally {
        pending.value = false
    }
})
</script>

<template>
    <NoroCard
        icon="i-lucide-box"
        :title="noro.t('mod-{{id}}-title', 'Template')"
    >
        <div
            v-if="pending"
            class="h-16 animate-pulse rounded-[var(--noro-r-md)] bg-[var(--noro-input)]"
        />

        <EmptyState v-else-if="error" icon="i-lucide-triangle-alert" :title="error" bare />

        <div v-else-if="data" class="flex items-center gap-2">
            <span class="grow text-sm">
                {{ noro.t('mod-{{id}}-joins', 'Joins') }}
            </span>
            <AtomBadge tone="success">{{ data.joins }}</AtomBadge>
        </div>
    </NoroCard>
</template>
