/**
 * The Noro panel's components, for module mini-apps.
 *
 * The package is declarations and nothing else: at runtime it is not there, and
 * the build turns its imports into lookups in `window.__noroUi`, where the panel
 * puts its own components. Two things follow — you get autocomplete and types,
 * and your module's bundle carries neither Vue nor the panel's markup, so it
 * weighs kilobytes.
 *
 * ```ts
 * import { NoroCard, AtomButton, useNoro } from '@noroproject/module-ui'
 *
 * const noro = useNoro()
 * const me = await noro.api('/me')
 * ```
 *
 * Everything listed here is the panel's public contract. It can grow, but what
 * is already in it does not get renamed: modules already built would not
 * survive that.
 */
import type { DefineComponent } from 'vue'

/** A tone, shared by badges and buttons. */
export type Tone = 'neutral' | 'blue' | 'cream' | 'amber' | 'danger' | 'success' | 'outline'

export type Size = 'sm' | 'md' | 'lg'

/** A label you cannot click: a state, a count, a version. */
export interface AtomBadgeProps {
    tone?: Tone
    /** Monospace — for identifiers and versions. */
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
    /** An `i-lucide-*` icon name. */
    icon?: string
    iconRight?: string
    loading?: boolean
    disabled?: boolean
    /** Stretch to the full width. */
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

/** A toggle in a row: a filter, a set of values. */
export interface AtomChipProps {
    active?: boolean
    disabled?: boolean
    size?: 'sm' | 'md'
    /** Upper case. */
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
    /** A multi-line field instead of a single-line one. */
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

/** A row of mutually exclusive values — instead of a two-item dropdown. */
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

/** The placeholder for an empty list. `bare` drops its own frame, for use inside a card. */
export interface EmptyStateProps {
    icon?: string
    title: string
    text?: string
    bare?: boolean
}

export declare const EmptyState: DefineComponent<EmptyStateProps>

/** A section card. The icon is required; the `actions` slot sits on the right. */
export interface NoroCardProps {
    title: string
    /** An `i-lucide-*` icon name. */
    icon: string
    subtitle?: string
    /** No inner padding — for full-width tables. */
    flush?: boolean
}

export declare const NoroCard: DefineComponent<NoroCardProps>

/** Who opened the mini-app. */
export interface NoroUser {
    id: string
    username?: string | null
    mc_username?: string | null
    [key: string]: unknown
}

/** What the panel gives a mini-app. */
export interface NoroContext {
    /**
     * Calls one of your own module's endpoints.
     *
     * The path is the one declared in `#[route]`. Other modules are out of
     * reach: the panel builds the address.
     *
     * ```ts
     * const me = await noro.api<Points>('/me')
     * await noro.api('/buy', { method: 'POST', body: { id } })
     * ```
     */
    api<T = unknown>(path: string, options?: { method?: string; body?: unknown }): Promise<T>

    /**
     * A translation, by catalog key.
     *
     * Your keys start with `mod-<id>-` and live in the package's
     * `locales/*.ftl`. The second argument is what to show while the string is
     * missing.
     */
    t(key: string, fallback?: string): string

    /** Who opened the page. `null` when nobody is signed in. */
    user: NoroUser | null

    /** The panel's notifications. */
    notify: { ok(text?: string): void; fail(error: unknown): void }

    /** Moving around the panel: `noro.navigate('/cabinet')`. */
    navigate(path: string): void
}

/**
 * The mini-app's context. Called in `setup`, like any Vue composable.
 */
export declare function useNoro(): NoroContext
