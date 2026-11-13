# [M] Browser APIs through wasm-bindgen

## Goal

[rust2-ts](/ts/README.md) translates a crate that calls browser APIs through wasm-bindgen
(web-sys, js-sys, wasm-bindgen-futures, and crates built on them such as
[web-rs](https://github.com/kixelated/web-rs)) from source with no `ts.toml` binding, each
import becoming a direct JS call, so Rust developers keep using idiomatic Rust crates
instead of JS types. web-rs's web-codecs translates end to end as a stock crate, its
decoders and audio included, which is this quest's remaining work: the
[encoder fixture](/ts/fixtures/encoder/Cargo.toml) proves a `VideoEncoder`'s calls on a
copy of web-codecs' encoder ([#681](https://github.com/kixel-ai/rust2/pull/681)), not on
the crate.

Non-goals: `#[wasm_bindgen]` exports, since a translation already exports the crate's `pub`
API, and imports of custom JS (`module`, `raw_module`, `inline_js`), which no consumer uses
yet. Both are refused at translation with their spans.

## Plan

Decided with the maintainer on 2026-11-09, in the quest audit: the remainder waits on
[the gate](/quest/m2/wasm/README.md), since no consumer calls it.

Decided with the maintainer on 2026-11-06:

- ✅ Translate wasm-bindgen's imports (rejected: mapping each browser API into the runtime
  by hand, as std is mapped). web-sys and js-sys are only `extern` declarations naming the
  JS each call stands for, and the caller is already JS, so the call needs no glue. An
  idiomatic wrapper such as web-codecs is then a stock crate, and the same crate keeps
  working in real WASM.
- ✅ Every import attribute web-sys and js-sys use is supported, and any other attribute,
  intrinsic, or wasm-bindgen item is refused at translation with its span, so partial
  support never reaches run time (rejected: only what one consumer's path needs; custom JS
  imports now, with no consumer). At web-sys and js-sys 0.3.107 those attributes are
  `method`, `js_class`, `js_name`, `js_namespace`, `getter`, `setter`, `constructor`,
  `static_method_of`, `catch`, `extends`, `variadic`, `indexing_getter`, `indexing_setter`,
  `indexing_deleter`, `typescript_type`, `is_type_of`, `final`, `structural`,
  `thread_local_v2`, and `vendor_prefix`. Three things those crates use are refused,
  each at its span (`ts/README.md`): an `experimental_generic_mono` import, a mutable
  slice within an `Option`, and 0.2.130's `Nullable`. js-sys's JSPI bridge also declares a
  `suspending` import, which suspends the WASM stack on a promise. No JS call can, so it
  is refused where reached, as is `jspi_block_on_promise`, which suspends on it. No
  translated path reaches it: `spawn_local` is mapped, and no JSPI context exists.
- ✅ The wasm-bindgen crate is mapped into the runtime, as std is (rejected: translating it
  from source). Its real implementation is `unsafe` glue between WASM memory and a JS heap
  table, which means nothing when the caller is JS: `JsValue` is the JS value itself,
  `JsCast` checks with `instanceof` or the import's `is_type_of`, `Closure` is a JS
  function, and `spawn_local`, which js-sys now defines and wasm-bindgen-futures
  re-exports, is a microtask. `JsFuture` then translates from js-sys's source, its
  callbacks `Closure`s.
- ✅ web-rs's web-codecs is the first consumer, and the epic joins m3 (rejected: moq's clock
  first, staying in m2). moq calls no wasm-bindgen import at the
  [model fixture](/ts/fixtures/model/Cargo.toml)'s pin: kio's clock reads `std::time` and
  tokio, so `ts/moq/model.toml`'s clock binds stay with
  [the no-ts.toml quest](/quest/m3/ts-configless.md). Rejected: having moq give kio a
  wasm32 clock over js-sys first, a moq change made only for translation.
- ✅ Translation reads each import's JS shape from the program the macro encodes in a
  `link_section` static, not from the `extern` blocks' source. The
  [spike](https://github.com/kixel-ai/rust2/pull/638) has the measurements:
  `macro_rules!`-made imports, which source reading misses, are on the encoder's path.
- ✅ web-codecs' encoders and decoders also hand their frames and errors through tokio's
  `sync::mpsc` and `sync::watch`, which translation refused then (about 50 refusals in
  the spike's census), so
  [tokio's sync channels are mapped into the runtime](/ts/src/lower/tokio.rs)
  (rejected: translating tokio's source, whose intrusive lists and raw pointers into
  `UnsafeCell`s are unsafe code a JS caller has no use for).

## Required

- [rust2-wasm](/quest/m2/wasm/README.md) - its gate decides whether translation continues past the model
