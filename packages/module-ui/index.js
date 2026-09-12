/*
 * This stub exists for the bundler alone.
 *
 * At runtime the package is not there: it is marked external, and its imports
 * become lookups in `window.__noroUi`, where the panel puts its components. The
 * package's real content is index.d.ts, with the types.
 *
 * If this file does end up in a bundle, external is not configured — better to
 * find that out at once than through a blank screen.
 */
throw new Error(
  '@noroproject/module-ui must be external in a mini-app build: check vite.config',
)
