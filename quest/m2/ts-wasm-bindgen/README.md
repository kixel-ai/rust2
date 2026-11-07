# Browser APIs through wasm-bindgen

## Goal

[rust2-ts](/ts/README.md) translates a crate that calls browser APIs through wasm-bindgen
(web-sys, js-sys, wasm-bindgen-futures, and crates built on them such as
[web-rs](https://github.com/kixelated/web-rs)) from source with no `ts.toml` binding, each
import becoming a direct JS call, so Rust developers keep using idiomatic Rust crates
instead of JS types. web-rs's web-codecs translates end to end, a `VideoEncoder` included,
which is this epic's own remaining work.

Non-goals: `#[wasm_bindgen]` exports, since a translation already exports the crate's `pub`
API, and imports of custom JS (`module`, `raw_module`, `inline_js`), which no consumer uses
yet. Both are refused at translation with their spans.

## Plan

Decided with the maintainer on 2026-11-06:

- ✅ Translate wasm-bindgen's imports (rejected: mapping each browser API into the runtime
  by hand, as std is mapped). web-sys and js-sys are only `extern` declarations naming the
  JS each call stands for, and the caller is already JS, so the call needs no glue. An
  idiomatic wrapper such as web-codecs is then a stock crate, and the same crate keeps
  working in real WASM.
- ✅ Every import attribute web-sys and js-sys use is supported, and any other attribute,
  intrinsic, or wasm-bindgen item is refused at translation with its span, so partial
  support never reaches run time (rejected: only what moq's clock path needs; custom JS
  imports now, with no consumer). At web-sys and js-sys 0.3.107 those attributes are
  `method`, `js_class`, `js_name`, `js_namespace`, `getter`, `setter`, `constructor`,
  `static_method_of`, `catch`, `extends`, `variadic`, `indexing_getter`, `indexing_setter`,
  `indexing_deleter`, `typescript_type`, `is_type_of`, `final`, `structural`, and
  `thread_local_v2`.
- ✅ The wasm-bindgen crate is mapped into the runtime, as std is (rejected: translating it
  from source). Its real implementation is `unsafe` glue between WASM memory and a JS heap
  table, which means nothing when the caller is JS: `JsValue` is the JS value itself,
  `JsCast` checks with `instanceof` or the import's `is_type_of`, `Closure` is a JS
  function, and `JsFuture` and `spawn_local` are a promise and a microtask.
- ✅ moq's clock is the first consumer, then web-codecs (rejected: web-codecs first; both at
  once). On wasm32, moq-net's `model::clock` and `time` call web-rs's `web_async::time`,
  which runs on wasmtimer (`performance.now()` and `setTimeout` through js-sys and
  web-sys), so translating imports removes `ts/moq/model.toml`'s three `model::clock`
  binds.
- ✅ The [spike](/quest/m2/ts-wasm-bindgen/spike.md) decides how translation reads each
  import's JS shape, and its report rescopes the quests after it.

## Required

- [Spike: how rust2-ts reads wasm-bindgen imports](/quest/m2/ts-wasm-bindgen/spike.md) - picks how translation reads each import's JS shape, and rescopes the imports quest
- [Translate wasm-bindgen imports](/quest/m2/ts-wasm-bindgen/imports.md) - web-sys, js-sys, and wasm-bindgen's runtime mapping, closing moq's clock binds

## Related

- [rust2-ts with no ts.toml](/quest/m3/ts-configless.md) - moq's clock binds are among the binds it removes
