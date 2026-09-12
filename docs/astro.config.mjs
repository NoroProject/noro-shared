// @ts-check
import starlight from '@astrojs/starlight'
import { defineConfig } from 'astro/config'

// GitHub Pages serves a project site from a subdirectory, so every absolute
// path needs this prefix. Astro adds it itself — including to a plain `link:`
// in the sidebar — so nothing here should prepend it by hand.
//
// In dev the prefix only gets in the way: the page would sit at
// /noro-shared/start/… instead of /start/…, which is not the address anyone
// would guess. So dev serves from the root.
//
// The command is read from argv rather than from an environment variable
// because `bun run dev` has to behave the same as `scripts/docs.sh dev` — a
// variable only the script sets is a trap for whoever starts the server
// directly. DOCS_BASE still overrides, for previewing the built prefix.
const dev = process.argv.includes('dev')
const base = process.env.DOCS_BASE ?? (dev ? '/' : '/noro-shared')

export default defineConfig({
    site: 'https://noroproject.github.io',
    base,
    integrations: [
        starlight({
            title: 'Noro Modules',
            // Один язык, объявленный корневым: языкового переключателя не будет.
            //
            // Сборка при этом всё равно пишет два предупреждения — про пустую
            // коллекцию `i18n` и про отсутствующую запись `404`. Оба про
            // необязательное содержимое, которого у одноязычного сайта нет, и
            // конфигом они не убираются. Заводить `404.md` не надо: своя
            // страница конфликтует с маршрутом Starlight, и он её не отдаёт.
            defaultLocale: 'root',
            locales: { root: { label: 'English', lang: 'en' } },
            description:
                'Write a module for a Noro instance: events, data, endpoints and mini-apps, without the master sources.',
            social: [
                {
                    icon: 'github',
                    label: 'GitHub',
                    href: 'https://github.com/NoroProject/noro-shared',
                },
            ],
            editLink: {
                baseUrl: 'https://github.com/NoroProject/noro-shared/edit/master/docs/',
            },
            sidebar: [
                {
                    label: 'Start here',
                    items: [
                        { label: 'What a module is', slug: 'start/what-a-module-is' },
                        { label: 'Your first module', slug: 'start/first-module' },
                        { label: 'Inside the package', slug: 'start/package' },
                    ],
                },
                {
                    label: 'Guides',
                    items: [
                        { label: 'Events', slug: 'guides/events' },
                        { label: 'Reaching the platform', slug: 'guides/platform' },
                        { label: 'Storing data', slug: 'guides/data' },
                        { label: 'Your own endpoints', slug: 'guides/endpoints' },
                        { label: 'Mini-apps', slug: 'guides/mini-apps' },
                        { label: 'Scheduled work', slug: 'guides/tasks' },
                        { label: 'Development mode', slug: 'guides/dev-mode' },
                    ],
                },
                {
                    label: 'Reference',
                    items: [
                        { label: 'cargo noro', slug: 'reference/cli' },
                        { label: 'Event catalog', slug: 'reference/events' },
                        { label: 'Capabilities', slug: 'reference/capabilities' },
                        { label: 'Errors', slug: 'reference/errors' },
                        // Built by `cargo doc`, not by Astro — hence a raw
                        // link rather than a slug. The `base` prefix is not
                        // written here: Astro prepends it to a leading slash,
                        // and adding it manually gave /noro-shared/noro-shared/api/.
                        { label: 'API reference (rustdoc)', link: '/api/', attrs: { target: '_blank' } },
                    ],
                },
            ],
        }),
    ],
})
