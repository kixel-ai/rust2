# [M] The web binding on npm and JSR

## Goal

The web export installs and imports from a registry: it writes `.js` and `.d.ts` beside
its `.ts`, a `package.json` with an `exports` map, and a `load()` that defaults to the
`.wasm` beside it through `new URL(..., import.meta.url)`, which Vite and webpack follow.
The builder publishes it to npm with trusted publishing and to JSR. Clean consumers under
Node, Deno, Bun, Vite, and webpack import it and run the transcript.
[The binding benchmark](/bench/ffi/README.md)'s size table gains the web binding's rows:
the `.wasm` stripped, gzipped, and brotli'd, the `.ts`, and a minified, tree-shaken
bundle of one caller.

## Plan

Found by the 2026-11-18 language audit: `web/` ships only `.ts`, which Node refuses to
strip under `node_modules` ([nodejs#57215](https://github.com/nodejs/node/issues/57215),
closed as not planned), with no `package.json`; and the epic's requirement that every
generator report its size raw and gzipped has no web row. The generator already writes
only syntax type stripping erases, so emitting `.js` is mechanical. Document `import
source`, which `load` already accepts as a `WebAssembly.Module`.

Public API: the package layout and `load()`'s default source.
