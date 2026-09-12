/**
 * Сборка мини-аппа модуля.
 *
 * Результат — один IIFE-бандл, который панель подключает тегом `<script>` и
 * монтирует как свой компонент.
 *
 * Ключевое здесь — `external: ['vue']` вместе с `globals`. Импорты Vue внутри
 * компонента превращаются в обращение к `window.__noroVue`, то есть модуль
 * получает **ту же** копию Vue, что и панель. Со своей копией реактивность
 * оказалась бы отдельной, и смонтировать компонент в дерево панели было бы
 * нельзя. Заодно бандл не тащит Vue и весит килобайты.
 */
import vue from '@vitejs/plugin-vue'
import { defineConfig } from 'vite'

export default defineConfig({
    plugins: [vue()],
    build: {
        outDir: 'web',
        emptyOutDir: false,
        lib: {
            entry: 'ui/app.js',
            formats: ['iife'],
            // Имя глобали одно на все модули: панель читает её сразу после
            // загрузки скрипта, до того как подключит следующий.
            name: '__noroModuleExport',
            fileName: () => 'app.js',
        },
        rollupOptions: {
            // Обе зависимости живут в панели, а не в бандле: Vue должен быть
            // тем же экземпляром, а компоненты — её собственными.
            external: ['vue', '@noro/module-ui'],
            output: {
                globals: {
                    vue: '__noroVue',
                    '@noro/module-ui': '__noroUi',
                },
            },
        },
    },
})
