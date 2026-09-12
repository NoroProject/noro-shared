/**
 * Компоненты панели Noro для мини-аппов модулей.
 *
 * Пакет состоит из одних объявлений: в рантайме его нет, а импорты при сборке
 * превращаются в обращения к `window.__noroUi`, куда панель кладёт свои
 * компоненты. Отсюда два следствия — автор получает автодополнение и типы, а
 * бандл модуля не тащит ни Vue, ни вёрстку панели и весит килобайты.
 *
 * ```ts
 * import { NoroCard, AtomButton, useNoro } from '@noro/module-ui'
 *
 * const noro = useNoro()
 * const me = await noro.api('/me')
 * ```
 *
 * Всё перечисленное здесь — публичный контракт панели. Он может пополняться, но
 * то, что уже есть, не переименовывается: собранные модули этого не переживут.
 */
import type { DefineComponent } from 'vue'

/** Оттенок, общий для бейджей и кнопок. */
export type Tone = 'neutral' | 'blue' | 'cream' | 'amber' | 'danger' | 'success' | 'outline'

export type Size = 'sm' | 'md' | 'lg'

/** Некликабельная метка: состояние, число, версия. */
export interface AtomBadgeProps {
    tone?: Tone
    /** Моноширинный шрифт — для идентификаторов и версий. */
    mono?: boolean
}

export declare const AtomBadge: DefineComponent<AtomBadgeProps>

export interface AtomButtonProps {
    variant?:
        | 'primary'
        | 'secondary'
        | 'dark'
        | 'warning'
        | 'danger'
        | 'danger-soft'
        | 'outline'
        | 'outline-blue'
        | 'ghost'
    size?: Size
    /** Имя иконки `i-lucide-*`. */
    icon?: string
    iconRight?: string
    loading?: boolean
    disabled?: boolean
    /** Растянуть на всю ширину. */
    block?: boolean
}

export declare const AtomButton: DefineComponent<AtomButtonProps>

export interface AtomCheckboxProps {
    modelValue?: boolean | unknown[]
    label?: string
    hint?: string
    value?: string | number
    disabled?: boolean
}

export declare const AtomCheckbox: DefineComponent<AtomCheckboxProps>

/** Переключатель в ряду: фильтр, набор значений. */
export interface AtomChipProps {
    active?: boolean
    disabled?: boolean
    size?: 'sm' | 'md'
    /** Заглавными буквами. */
    caps?: boolean
}

export declare const AtomChip: DefineComponent<AtomChipProps>

export interface AtomInputProps {
    modelValue?: string | number
    label?: string
    placeholder?: string
    type?: string
    hint?: string
    error?: string
    disabled?: boolean
    readonly?: boolean
    /** Многострочное поле вместо однострочного. */
    textarea?: boolean
    rows?: number
}

export declare const AtomInput: DefineComponent<AtomInputProps>

export interface AtomModalProps {
    modelValue: boolean
    title?: string
    subtitle?: string
    size?: 'md' | 'lg' | 'xl' | 'full'
}

export declare const AtomModal: DefineComponent<AtomModalProps>

export interface AtomNumberInputProps {
    modelValue?: number
    label?: string
    hint?: string
    min?: number
    max?: number
    step?: number
    disabled?: boolean
}

export declare const AtomNumberInput: DefineComponent<AtomNumberInputProps>

/** Ряд взаимоисключающих значений — вместо выпадающего списка на два пункта. */
export interface AtomSegmentedProps {
    modelValue?: string | number
    options: { label: string; value: string | number; count?: number; icon?: string }[]
    size?: Size
    block?: boolean
}

export declare const AtomSegmented: DefineComponent<AtomSegmentedProps>

export interface AtomSelectMenuProps {
    modelValue?: string | number
    options: { label: string; value: string | number; disabled?: boolean; group?: string }[]
    placeholder?: string
    disabled?: boolean
    size?: 'sm' | 'md'
}

export declare const AtomSelectMenu: DefineComponent<AtomSelectMenuProps>

export interface AtomToggleProps {
    modelValue: boolean
    label?: string
    disabled?: boolean
    loading?: boolean
}

export declare const AtomToggle: DefineComponent<AtomToggleProps>

/** Заглушка для пустого списка. `bare` — без своей рамки, внутри карточки. */
export interface EmptyStateProps {
    icon?: string
    title: string
    text?: string
    bare?: boolean
}

export declare const EmptyState: DefineComponent<EmptyStateProps>

/** Карточка-раздел. Иконка обязательна, справа — слот `actions`. */
export interface NoroCardProps {
    title: string
    /** Имя иконки `i-lucide-*`. */
    icon: string
    subtitle?: string
    /** Без внутренних отступов — для таблиц во всю ширину. */
    flush?: boolean
}

export declare const NoroCard: DefineComponent<NoroCardProps>

/** Кто открыл мини-апп. */
export interface NoroUser {
    id: string
    username?: string | null
    mc_username?: string | null
    [key: string]: unknown
}

/** То, что панель даёт мини-аппу. */
export interface NoroContext {
    /**
     * Вызов ручки своего модуля.
     *
     * Путь — тот, что объявлен в `#[route]`. Чужие модули недоступны: адрес
     * собирает панель.
     *
     * ```ts
     * const me = await noro.api<Points>('/me')
     * await noro.api('/buy', { method: 'POST', body: { id } })
     * ```
     */
    api<T = unknown>(path: string, options?: { method?: string; body?: unknown }): Promise<T>

    /**
     * Перевод по ключу каталога.
     *
     * Ключи модуля начинаются с `mod-<id>-` и живут в `locales/*.ftl` пакета.
     * Второй аргумент — что показать, пока строки нет.
     */
    t(key: string, fallback?: string): string

    /** Кто открыл страницу. `null` у неавторизованного. */
    user: NoroUser | null

    /** Уведомления панели. */
    notify: { ok(text?: string): void; fail(error: unknown): void }

    /** Переход по панели: `noro.navigate('/cabinet')`. */
    navigate(path: string): void
}

/**
 * Контекст мини-аппа. Зовётся в `setup`, как обычный composable Vue.
 */
export declare function useNoro(): NoroContext
