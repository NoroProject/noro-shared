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


/** A page of something, the shape every list endpoint answers with. */
export interface NoroPage<T> {
    items: T[]
    total: number
}

/**
 * Reading platform data.
 *
 * Functions rather than addresses on purpose: the panel's endpoints are its own
 * business, and a module built against `/api/me/hubs` would break the day that
 * path changed. What is promised is this surface.
 *
 * Everything here is a read. Writing goes through your module's own endpoint on
 * the Rust side, where the capabilities the operator granted are checked — a
 * write from here would travel under the token of whoever happened to open the
 * page.
 *
 * Types are deliberately loose (`unknown`, `Record`) where the panel's model is
 * its own: pinning them here would freeze internal shapes as a public contract.
 * Cast what you need, and treat a missing field as possible.
 */
export interface NoroPlatform {
    /** The person looking at the page. Their own data, so nothing is withheld. */
    me: {
        profile(): Promise<Record<string, unknown>>
        /** The hubs they are a member of. */
        hubs(): Promise<unknown[]>
        punishments(): Promise<unknown[]>
        sessions(): Promise<unknown[]>
        /** Linked logins: discord, twitch, google. */
        identities(): Promise<unknown[]>
        tickets(): Promise<unknown>
        cape(): Promise<unknown>
    }

    players: {
        /** One player by exact Minecraft username. */
        byName(name: string): Promise<Record<string, unknown> | null>
    }

    servers: {
        list(): Promise<unknown[]>
        /** The rules that apply on one server build. */
        rules(serverId: string): Promise<unknown>
    }

    rules: {
        list(): Promise<unknown>
        scopes(): Promise<unknown>
    }

    legal: {
        list(): Promise<unknown[]>
        get(slug: string): Promise<unknown>
    }

    capes: {
        list(): Promise<unknown[]>
    }

    /**
     * A server's hub. The slug is always explicit: an instance has several
     * hubs, and "the current one" in the panel means whatever the browser has
     * open — not what your module meant.
     */
    hub: {
        info(slug: string): Promise<Record<string, unknown>>
        feed(slug: string, page?: number): Promise<NoroPage<unknown>>
        members(slug: string, options?: { page?: number; q?: string }): Promise<NoroPage<unknown>>
        towns(slug: string, page?: number): Promise<NoroPage<unknown>>
        town(slug: string, town: string): Promise<unknown>
        myTowns(slug: string): Promise<unknown[]>
        court(slug: string, page?: number): Promise<NoroPage<unknown>>
        fines(slug: string, page?: number): Promise<NoroPage<unknown>>
        /** The viewer's bank cards on this hub. */
        cards(slug: string): Promise<unknown[]>
        communities(slug: string): Promise<unknown[]>
        mapPlayers(slug: string): Promise<unknown[]>
        mapMarkers(slug: string): Promise<unknown[]>
    }

    /**
     * Anything not covered above, by method name.
     *
     * The escape hatch, for when the panel gained a method before this package
     * did. Unknown names fail with the list of what exists.
     */
    call<T = unknown>(method: string, args?: Record<string, unknown>): Promise<T>
}

/** Formatting the panel already does, so a module looks the same as the rest. */
export interface NoroFormat {
    /**
     * A hub amount, in its minor units.
     *
     * The currency settings come from `platform.hub.info(slug)` — precision and
     * symbol are per hub, and formatting without them turns `500` into five
     * times the price when the hub keeps two decimals. This is the same
     * function the panel's own forms use, and it mirrors the master's
     * `money::format_amount`: a mismatch here is one price looking different on
     * the form and in the bank's refusal.
     */
    money(minor: number, currency: { precision?: number; symbol?: string; name?: string }): string

    /**
     * A duration as `7d 12h 30m`.
     *
     * Seconds in, because that is what the Rust side deals in; below a minute
     * there is nothing to show.
     */
    duration(seconds: number): string

    /** A timestamp as a date the viewer reads, in their locale. */
    date(value: string | number | Date): string

    /**
     * A timestamp as `5 минут назад`, switching to an absolute date once it is
     * far enough back that "eleven months ago" stops being useful.
     */
    ago(value: string | number | Date): string
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

    /**
     * Whether the viewer holds a permission.
     *
     * The same matcher the panel uses for its own menus, wildcards included —
     * `noro.admin.*` answers true for `noro.admin.modules.view`. Use it to
     * decide what to *show*; the master decides what to allow, and your
     * endpoint checks again on its side.
     *
     * ```ts
     * if (noro.can('noro.module.shop.manage')) { … }
     * ```
     */
    can(permission: string): boolean

    /** True when the viewer holds at least one of these. */
    canAny(permissions: string[]): boolean

    /**
     * Whether they hold it **on one server build** — global permissions plus
     * the roles of that server. A role granted on one build deliberately does
     * not count anywhere else.
     */
    canOn(permission: string, serverId: string): boolean

    /**
     * Asks the viewer to confirm, with the panel's own dialog.
     *
     * Resolves to `false` when they decline, so an unanswered question reads as
     * "no" rather than as an exception to handle.
     */
    confirm(options: {
        title: string
        text?: string
        /** Label of the confirming button. */
        confirmLabel?: string
        /** Paint the confirming button as destructive. */
        danger?: boolean
    }): Promise<boolean>

    /** Reading platform data. See [[NoroPlatform]]. */
    platform: NoroPlatform

    /** Formatting the panel already does. */
    format: NoroFormat

    /**
     * A paginated list from one of **your** endpoints.
     *
     * Your endpoint has to answer `{ items, total }` — the shape every list in
     * this platform answers with. What you get back is refs you can bind
     * straight into a template, plus the fetch, so a module does not reimplement
     * paging and debounced search for the fourth time.
     *
     * ```ts
     * const list = noro.paged<Order>('/orders', { perPage: 25 })
     * await list.load()
     * ```
     */
    paged<T = unknown>(
        path: string,
        options?: { perPage?: number; params?: () => Record<string, string | undefined> },
    ): NoroPagedList<T>
}

/** A paginated list, ready to bind. */
export interface NoroPagedList<T> {
    items: import('vue').Ref<T[]>
    total: import('vue').Ref<number>
    page: import('vue').Ref<number>
    /** What the person is typing. Debounced before it reaches the request. */
    search: import('vue').Ref<string>
    pending: import('vue').Ref<boolean>
    error: import('vue').Ref<string | null>
    pages: import('vue').ComputedRef<number>
    load(): Promise<void>
    /** Re-reads the current page — after your own mutation, for instance. */
    refresh(): Promise<void>
}

/**
 * The mini-app's context. Called in `setup`, like any Vue composable.
 */
export declare function useNoro(): NoroContext
