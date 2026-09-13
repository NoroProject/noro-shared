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
            // Английский корневой, русский рядом. Корневым он не случайно:
            // адреса страниц остаются прежними, а ссылки, которые кто-то уже
            // сохранил, продолжают работать.
            //
            // Страницы без перевода Starlight отдаёт на языке по умолчанию и
            // помечает. Это лучше, чем прятать их из меню: ненайденная
            // страница выглядит как отсутствующая возможность.
            defaultLocale: 'root',
            locales: {
                root: { label: 'English', lang: 'en' },
                ru: { label: 'Русский', lang: 'ru' },
            },
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
                    translations: { ru: 'Начало' },
                    items: [
                        {
                            label: 'What a module is',
                            translations: { ru: 'Что такое модуль' },
                            slug: 'start/what-a-module-is',
                        },
                        {
                            label: 'Your first module',
                            translations: { ru: 'Первый модуль' },
                            slug: 'start/first-module',
                        },
                        {
                            label: 'Inside the package',
                            translations: { ru: 'Внутри пакета' },
                            slug: 'start/package',
                        },
                    ],
                },
                {
                    label: 'Guides',
                    translations: { ru: 'Руководства' },
                    items: [
                        { label: 'Events', translations: { ru: 'События' }, slug: 'guides/events' },
                        {
                            label: 'Reaching the platform',
                            translations: { ru: 'Доступ к платформе' },
                            slug: 'guides/platform',
                        },
                        {
                            label: 'Storing data',
                            translations: { ru: 'Хранение данных' },
                            slug: 'guides/data',
                        },
                        {
                            label: 'Your own endpoints',
                            translations: { ru: 'Свои ручки' },
                            slug: 'guides/endpoints',
                        },
                        {
                            label: 'Mini-apps',
                            translations: { ru: 'Мини-аппы' },
                            slug: 'guides/mini-apps',
                        },
                        {
                            label: 'Talking to the launcher',
                            translations: { ru: 'Разговор с лаунчером' },
                            slug: 'guides/launcher',
                        },
                        {
                            label: 'Scheduled work',
                            translations: { ru: 'Работа по расписанию' },
                            slug: 'guides/tasks',
                        },
                        {
                            label: 'Development mode',
                            translations: { ru: 'Режим разработки' },
                            slug: 'guides/dev-mode',
                        },
                    ],
                },
                {
                    label: 'Reference',
                    translations: { ru: 'Справочник' },
                    items: [
                        { label: 'cargo noro', slug: 'reference/cli' },
                        {
                            label: 'SDK domains',
                            translations: { ru: 'Домены SDK' },
                            slug: 'reference/sdk',
                        },
                        {
                            // Автогенерация, а не список руками: доменов
                            // двадцать семь, и новый появляется вместе с новым
                            // файлом в SDK — перечислять их здесь значит
                            // однажды забыть.
                            label: 'By domain',
                            translations: { ru: 'По доменам' },
                            collapsed: true,
                            items: [{ autogenerate: { directory: 'reference/sdk' } }],
                        },
                        {
                            label: 'Mini-app API',
                            translations: { ru: 'API мини-аппа' },
                            slug: 'reference/mini-app-api',
                        },
                        {
                            label: 'Event catalog',
                            translations: { ru: 'Каталог событий' },
                            slug: 'reference/events',
                        },
                        {
                            label: 'Capabilities',
                            translations: { ru: 'Возможности' },
                            slug: 'reference/capabilities',
                        },
                        { label: 'Errors', translations: { ru: 'Ошибки' }, slug: 'reference/errors' },
                        // Built by `cargo doc`, not by Astro — hence a raw
                        // link rather than a slug. The `base` prefix is not
                        // written here: Astro prepends it to a leading slash,
                        // and adding it manually gave /noro-shared/noro-shared/api/.
                        {
                            label: 'API reference (rustdoc)',
                            translations: { ru: 'Справочник API (rustdoc)' },
                            link: '/api/',
                            attrs: { target: '_blank' },
                        },
                    ],
                },
            ],
        }),
    ],
})
