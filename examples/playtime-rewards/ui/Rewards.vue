<script setup lang="ts">
/**
 * Награды за время — страница в личном кабинете.
 *
 * Обычный компонент Vue: `<script setup>`, реактивность, типы. Компоненты
 * приходят из `@noro/module-ui` — это те же атомы, которыми нарисована сама
 * панель, поэтому вид совпадает без единой строки стилей.
 */
import { AtomBadge, EmptyState, NoroCard, useNoro } from '@noro/module-ui'
import { onMounted, ref } from 'vue'

interface Points {
    player: string
    points: number
    seconds_played: number
}

const noro = useNoro()

const data = ref<Points | null>(null)
const error = ref<string | null>(null)
const pending = ref(true)

onMounted(async () => {
    try {
        data.value = await noro.api<Points>('/me')
    } catch (e) {
        error.value = e instanceof Error ? e.message : String(e)
    } finally {
        pending.value = false
    }
})

function played(seconds: number) {
    const h = Math.floor(seconds / 3600)
    const m = Math.floor((seconds % 3600) / 60)
    return `${h} ч ${m} мин`
}
</script>

<template>
    <NoroCard icon="i-lucide-gift" :title="noro.t('mod-playtime-rewards-title', 'Награды за время')">
        <div v-if="pending" class="h-16 animate-pulse rounded-[var(--noro-r-md)] bg-[var(--noro-input)]" />

        <EmptyState v-else-if="error" icon="i-lucide-triangle-alert" :title="error" bare />

        <div v-else-if="data" class="space-y-2">
            <div class="flex items-center gap-2">
                <span class="grow text-sm">
                    {{ noro.t('mod-playtime-rewards-played', 'Наиграно') }}
                </span>
                <AtomBadge tone="outline" mono>{{ played(data.seconds_played) }}</AtomBadge>
            </div>
            <div class="flex items-center gap-2">
                <span class="grow text-sm">
                    {{ noro.t('mod-playtime-rewards-points', 'Начислено очков') }}
                </span>
                <AtomBadge tone="success">{{ data.points }}</AtomBadge>
            </div>
        </div>
    </NoroCard>
</template>
