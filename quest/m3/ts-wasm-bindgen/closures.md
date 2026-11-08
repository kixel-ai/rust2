# [L] wasm-bindgen closures and promises, proven on WebCodecs' encoder

## Goal

wasm-bindgen's `Closure` and js-sys's futures (`JsFuture`, `spawn_local`,
`future_to_promise`, which wasm-bindgen-futures now re-exports) are mapped into the
[rust2-ts](/ts/README.md) runtime. A fixture that drives web-sys's `VideoEncoder` with
web-codecs' calls translates with no `ts.toml` binding: its configure, encode, and flush,
its output and error callbacks, and `isConfigSupported`'s promise. A differential test in
headless Chromium proves it against the same fixture's wasm-bindgen build.

## Plan

- `Closure<T>` and `ScopedClosure` are a JS function over the closure it was made of, from
  `Closure::wrap` of a `Box<dyn FnMut>` or `Closure::new` of any closure, since
  `dyn FnMut` already translates as the closure itself. It throws as wasm-bindgen's does
  once the `Closure` drops and, for a `dyn FnMut` one (`WasmClosure::IS_MUT`), when
  called again while running; a `dyn Fn` one stays reentrant, as the CLI's `makeClosure`
  keeps it. `forget` and
  `into_js_value` keep it alive. Its args from JS convert as an import's result does. The
  census shows its glue refused today: `(usize, usize)` fat-pointer transmutes and fn
  pointer casts in `convert/closures.rs`.
- `JsFuture` polls a promise's settlement as an `rs.Future` on the runtime's executor, and
  `spawn_local` queues a microtask, as js-sys's `queue.rs` does. Mapping these stands in
  for js-sys's JSPI bridge, which `spawn_local` checks first: with no export translated,
  no JSPI context exists. `jspi_block_on_promise` is refused, as no JS blocks on a promise.
- The fixture keeps web-codecs' calls and its tokio channels, `mpsc::unbounded_channel`
  for frames and `watch::channel` for errors read in a `biased;` `select!`, which translate
  since [#654](https://github.com/moq-dev/rust2/pull/654). Decided with the maintainer on
  2026-11-07 (rejected: handing frames out through `Rc<RefCell<VecDeque>>`), since
  web-codecs end to end needs them.
- Chromium decodes and encodes VP8 headless: Chrome 155 there gave a key chunk with a
  `decoderConfig` and two deltas from three 64x64 frames. Only the bench shell carries
  Chromium today, apart for its size. Run this test where it does: in the merge gate if
  its run stays fast, or else nightly. Compare what the harness reports, such as chunk
  types, timestamps, sizes, and bytes, with both builds in one browser.

Public API: the translator's output for a crate calling wasm-bindgen closures and
futures.
