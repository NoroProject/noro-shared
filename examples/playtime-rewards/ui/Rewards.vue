<script setup lang="ts">
/**
 * Награды за время — страница в личном кабинете.
 *
 * Обычный компонент Vue: `<script setup>`, реактивность, типы. Компоненты
 * приходят из `@noroproject/module-ui` — это те же атомы, которыми нарисована сама
 * панель, поэтому вид совпадает без единой строки стилей.
 */
import { AtomBadge, AtomButton, EmptyState, NoroCard, useNoro } from '@noroproject/module-ui'
import { onMounted, onUnmounted, ref } from 'vue'

interface Points {
    player: string
    points: number
    seconds_played: number
}

interface BonusStatusMessage {
    type: string
    status: string
    added?: number
    total_points?: number
    message?: string
}

const noro = useNoro()

const data = ref<Points | null>(null)
const error = ref<string | null>(null)
const pending = ref(true)
const bonusStatus = ref<string | null>(null)

onMounted(async () => {
    try {
        data.value = await noro.api<Points>('/me')
    } catch (e) {
        error.value = e instanceof Error ? e.message : String(e)
    } finally {
        pending.value = false
    }
})

// Подписка на пуши по WebSocket от модуля
const unsubscribe = noro.ws.on<BonusStatusMessage>((msg) => {
    if (msg.type === 'bonus_status') {
        if (msg.status === 'claimed' && msg.total_points !== undefined) {
            if (data.value) data.value.points = msg.total_points
            bonusStatus.value = `+${msg.added} очков!`
            noro.notify.ok(bonusStatus.value)
        } else if (msg.message) {
            bonusStatus.value = msg.message
            noro.notify.fail(msg.message)
        }
    }
})
onUnmounted(() => unsubscribe())

function claimBonus() {
  noro.ws.send({ action: 'claim_bonus' })
}

/** Право модуля: кнопка сброса есть только у тех, кому можно. */
const canReset = noro.can('noro.module.playtime-rewards.reset')

async function reset() {
    const ok = await noro.confirm({
        title: noro.t('mod-playtime-rewards-reset-title', 'Сбросить очки?'),
        text: noro.t('mod-playtime-rewards-reset-text', 'Начисленное вернуть будет нельзя.'),
        danger: true,
    })
    if (!ok) return
    await noro.api('/reset', { method: 'POST' })
    data.value = await noro.api<Points>('/me')
    noro.notify.ok()
}
</script>

<template>
    <NoroCard icon="i-lucide-gift" :title="noro.t('mod-playtime-rewards-title', 'Награды за время')">
        <div v-if="pending" class="h-16 animate-pulse rounded-[var(--noro-r-md)] bg-[var(--noro-input)]" />

        <EmptyState v-else-if="error" icon="i-lucide-triangle-alert" :title="error" bare />

        <div v-else-if="data" class="space-y-4">
            <div class="flex items-center gap-2">
                <span class="grow text-sm">
                    {{ noro.t('mod-playtime-rewards-played', 'Наиграно') }}
                </span>
                <span class="text-sm font-medium">{{ data.seconds_played }} с</span>
            </div>
            <div class="flex items-center gap-2">
                <span class="grow text-sm">
                    {{ noro.t('mod-playtime-rewards-points', 'Начислено очков') }}
                </span>
                <AtomBadge tone="success">{{ data.points }}</AtomBadge>
            </div>

            <div class="flex items-center gap-2 pt-2 border-t border-[var(--noro-border)]">
                <AtomButton size="sm" @click="claimBonus">
                    {{ noro.t('mod-playtime-rewards-claim', 'Получить бонус') }}
                </AtomButton>
                <span v-if="bonusStatus" class="text-xs opacity-60">{{ bonusStatus }}</span>

                <div class="grow" />

                <AtomButton v-if="canReset" variant="danger-soft" size="sm" @click="reset">
                    {{ noro.t('mod-playtime-rewards-reset', 'Сбросить') }}
                </AtomButton>
            </div>
        </div>
    </NoroCard>
</template>
