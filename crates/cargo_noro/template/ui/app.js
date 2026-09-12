/**
 * The mini-app's entry point.
 *
 * The panel calls this function once and mounts whatever it returns as one of
 * its own components. The context carries its Vue, its components, translation,
 * calls to your module's endpoints and its notifications.
 *
 * The context arrives as an argument rather than an import: the module is built
 * separately from the panel and knows nothing about its internal paths.
 */
import App from './App.vue'

export default function (ctx) {
    // The component is wrapped so the context reaches it through provide:
    // threading it down as props through every level gets old fast.
    return ctx.vue.defineComponent({
        name: '{{pascal}}App',
        setup() {
            ctx.vue.provide('noro', ctx)
            return () => ctx.vue.h(App)
        },
    })
}
