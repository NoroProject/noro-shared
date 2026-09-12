// @ts-check
import starlight from '@astrojs/starlight'
import { defineConfig } from 'astro/config'

// GitHub Pages serves a project site from a subdirectory, so every absolute
// path needs this prefix. Kept in one place because the link to the rustdoc
// below is not an Astro route and gets no prefix of its own.
const base = '/noro-shared'

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
                        { label: 'Storing data', slug: 'guides/data' },
                        { label: 'Your own endpoints', slug: 'guides/endpoints' },
                        { label: 'Mini-apps', slug: 'guides/mini-apps' },
                        { label: 'Development mode', slug: 'guides/dev-mode' },
                    ],
                },
                {
                    label: 'Reference',
                    items: [
                        { label: 'Event catalog', slug: 'reference/events' },
                        { label: 'Capabilities', slug: 'reference/capabilities' },
                        { label: 'Errors', slug: 'reference/errors' },
                        // Built by `cargo doc`, not by Astro — hence a raw link.
                        { label: 'API reference (rustdoc)', link: `${base}/api/`, attrs: { target: '_blank' } },
                    ],
                },
            ],
        }),
    ],
})
