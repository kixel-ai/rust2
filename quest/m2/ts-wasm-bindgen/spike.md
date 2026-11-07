# [M] Spike: how rust2-ts reads wasm-bindgen imports

## Goal

A report, in this quest's PR, that sizes translating wasm-bindgen imports on moq-net's
wasm32 clock path (web-async, wasmtimer, js-sys, web-sys) and on web-rs's web-codecs
`VideoEncoder`, and rewrites [the imports quest](/quest/m2/ts-wasm-bindgen/imports.md) to
the mechanism it picks. No translator change lands.

## Plan

After macro expansion the `#[wasm_bindgen]` attributes are gone. Decided with the
maintainer on 2026-11-06: the spike compares both sources of each import's JS shape (its
class, `js_name`, kind, `catch`) and picks one (rejected: choosing one now).

- wasm-bindgen's encoded program, the `link_section` static the wasm-bindgen CLI reads back
  out of the binary. It is exact, but its `SCHEMA_VERSION` changes with every release
  (0.2.130 now): measure what decoding costs across the versions the consumers' lockfiles
  pin, and whether `wasm-bindgen-shared` or the CLI's decoder is usable as a library.
- The `extern` blocks in the dependency's source, read again with their attributes. They
  are stable across versions, but miss imports a `macro_rules!` generates: count those in
  web-sys, js-sys, and wasmtimer.

It also answers, for the imports quest:

- How a mapped `JsValue`, `JsCast`, and `Closure` meet the generic code around them, such
  as `Closure::new` of a closure, which web-codecs' callbacks need, given that
  [trait objects](/quest/m3/ts-dyn.md) other than `TypeId` already translate.
- How the differential test's native side runs a wasm-bindgen build: the wasm-bindgen CLI
  in the dev shell at the fixture's pinned version, and which host provides the browser
  APIs. Bun has `performance.now()` and `setTimeout`, while WebCodecs needs a browser.

Public API: none.
