# [M] Spike: how rust2-ts reads wasm-bindgen imports

## Goal

A report, in this quest's PR, that sizes translating web-rs's web-codecs `VideoEncoder`
through web-sys, js-sys, and wasm-bindgen, and rewrites
[the imports quest](/quest/m3/ts-wasm-bindgen/imports.md) to the mechanism it picks. No
translator change lands.

## Plan

After macro expansion the `#[wasm_bindgen]` attributes are gone. Decided with the
maintainer on 2026-11-06: the spike compares both sources of each import's JS shape (its
class, `js_name`, kind, `catch`) and picks one (rejected: choosing one now).

- wasm-bindgen's encoded program, the `link_section` static the wasm-bindgen CLI reads back
  out of the binary. It is exact, but its `SCHEMA_VERSION` changes with every release
  (0.2.130 now): measure what decoding costs across the versions web-codecs' lockfile and
  its dependents' pin, and whether `wasm-bindgen-shared` or the CLI's decoder is usable as
  a library.
- The `extern` blocks in the dependency's source, read again with their attributes. They
  are stable across versions, but miss imports a `macro_rules!` generates: count those in
  web-sys and js-sys.

It also answers, for the imports quest:

- How a mapped `JsValue`, `JsCast`, and `Closure` meet the generic code around them, such
  as `Closure::new` of a closure, which web-codecs' output and error callbacks need, given
  that trait objects, `TypeId` included, already translate.
- How the differential test's native side runs a wasm-bindgen build: the wasm-bindgen CLI
  in the dev shell at the fixture's pinned version, and a host with WebCodecs, which Bun
  lacks, so likely a headless browser.
- Whether translation builds with the consumer's own cargo config: web-sys's
  `VideoEncoder` is behind `--cfg=web_sys_unstable_apis`, which web-rs sets in its
  `.cargo/config.toml` rustflags.

Public API: none.
