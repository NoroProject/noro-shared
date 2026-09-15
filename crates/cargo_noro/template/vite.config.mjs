/**
 * The mini-app build.
 *
 * The result is a single IIFE bundle the panel loads with a `<script>` tag and
 * mounts as one of its own components.
 *
 * `external` together with `globals` is the part that matters. Imports of Vue
 * inside the component become lookups in `window.__noroVue`, so the module gets
 * the **same** copy of Vue the panel uses. With its own copy the reactivity
 * would be separate and mounting into the panel's tree would not work at all.
 * As a side effect the bundle carries no Vue and weighs kilobytes.
 */
import vue from '@vitejs/plugin-vue'
import { defineConfig } from 'vite'

const noroDevPlugin = {
    name: 'noro-ui-dev-mock',
    resolveId(id) {
        if (id === '@noroproject/module-ui') {
            return '\0noro-ui-virtual'
        }
    },
    load(id) {
        if (id === '\0noro-ui-virtual') {
            return `
const ui = (typeof window !== 'undefined' ? window.__noroUi : null) || {};
export const NoroCard = ui.NoroCard;
export const AtomButton = ui.AtomButton;
export const AtomInput = ui.AtomInput;
export const AtomBadge = ui.AtomBadge;
export const EmptyState = ui.EmptyState;
export const useNoro = () => (ui.useNoro ? ui.useNoro() : {});
export const useModuleWs = () => (ui.useModuleWs ? ui.useModuleWs() : (ui.useNoro ? ui.useNoro().ws : null));
export default ui;
`
        }
    },
}

export default defineConfig(({ command }) => ({
    plugins: [
        vue(),
        ...(command === 'serve' ? [noroDevPlugin] : []),
    ],
    build: {
        outDir: 'web',
        emptyOutDir: false,
        lib: {
            entry: 'ui/app.js',
            formats: ['iife'],
            // One global name for every module: the panel reads it straight
            // after loading the script, before it loads the next one.
            name: '__noroModuleExport',
            fileName: () => 'app.js',
        },
        rollupOptions: {
            external: ['vue', '@noroproject/module-ui'],
            output: {
                globals: {
                    vue: '__noroVue',
                    '@noroproject/module-ui': '__noroUi',
                },
            },
        },
    },
})
