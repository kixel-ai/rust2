# [XL] Translate wasm-bindgen imports

## Goal

[rust2-ts](/ts/README.md) translates every wasm-bindgen import web-sys and js-sys declare,
and wasm-bindgen's own intrinsics, into a direct JS call. It reads each import's JS shape
from the program the `#[wasm_bindgen]` macro encodes. `JsValue` and every imported type
are the JS value itself. Any other wasm-bindgen import or item is refused where
translation reaches it, with its span. A differential test against a native wasm32 build
through the wasm-bindgen CLI proves it on synchronous js-sys and web-sys calls. Closures
and promises are [the next quest's](/quest/m3/ts-wasm-bindgen/closures.md).

## Plan

The epic's [Plan](/quest/m3/ts-wasm-bindgen/README.md#plan) holds the settled decisions.
The [spike](https://github.com/moq-dev/rust2/pull/638) picked the mechanism and measured
what follows.

- Shapes come from the encoded program (rejected: re-reading the `extern` blocks in the
  dependency's source). Each `#[wasm_bindgen]` invocation on `wasm32-unknown-unknown`
  emits a `#[link_section = "__wasm_bindgen_unstable"]` static, whose initializer every
  crate's metadata keeps, as rustc keeps every static's. A wrapper such as
  `VideoEncoder::encode_queue_size` calls one `extern "C"` shim in
  `wasm_import_module = "__wbindgen_placeholder__"`. Its link name is the `shim` of an
  `ImportFunction`, which gives the class, JS name, kind, namespace, `catch`, `variadic`,
  and `structural`. Find a crate's statics by their `link_section` through
  `CStore::num_def_ids_untracked`, since they sit in anonymous `const _` blocks. Read
  them once per crate. Source reading would miss every import a `macro_rules!` makes:
  js-sys's `arrays!` makes 308, for its 11 typed arrays, and the encoder path reaches
  `Uint8Array`'s. It would also re-implement the macro's ~400 lines of naming rules.
- The decoding is positional and changes with `SCHEMA_VERSION`. That changed 8 times in
  the 12 months to 0.2.130, each time adding or removing fields or variants. Decode with
  one field list, wasm-bindgen-shared's `shared_api!`, with each field and variant gated
  by the schemas that carry it. Support every schema from 0.2.128 on, moq's pin today.
  Refuse a program of another schema at its crate, naming the supported range.
  wasm-bindgen-shared can be depended on, but only at one version, since cargo unifies
  0.2.x. cli-support's decoder is private, so vendor its 181 lines (MIT or Apache-2.0)
  with their source and revision. A new schema adds its gates.
- JS-represented types: `JsValue`, each `ImportKind::Type`, and their `ErasableGeneric`
  instances, such as js-sys's `Promise<T>`, are the JS value. The generated glue between
  them is then the identity: the `obj` field, `transmute_copy`, and the
  `&*(val as *const JsValue as *const Self)` cast, all refused today. `JsCast` translates
  from MIR, over each type's `instanceof` shim, `x instanceof Class`, and its
  `is_type_of`. Clone and drop are the identity and nothing, which replaces the heap-table
  intrinsics (`__wbindgen_object_clone_ref`, `__wbindgen_object_drop_ref`).
- A wrapper is replaced whole, never translated from its ABI glue. Its arguments and
  result convert as the wasm-bindgen CLI's JS glue converts them for each Rust type, and
  `wbg_cast::<From, To>` converts the same way. That covers numbers, 64-bit and 128-bit
  integers, `bool`, `char`, strings, `Option`, slices and `Vec`s (a `&mut [u8]` copies
  back), string enums by their `variant_values`, and a `Result` under `catch`. A
  non-`structural` (`final`) method calls `Class.prototype.name.call(this, ...)`, and
  `vendor_prefixes` try each prefixed global.
- Intrinsics: in wasm-bindgen 0.2.130, 54 `__wbindgen_*` intrinsics are ordinary imports
  in the program (`raw_module = "__wbindgen_placeholder__"`). Give each one translation
  reaches the JS body the CLI's intrinsic table gives it. Treat that module as
  wasm-bindgen's own, not custom JS.
- Refusals are per reached import or item, at its span, never per program, since js-sys's
  program holds its JSPI bridge (`suspending`, `raw_module`) beside everything else: a
  `module`, `raw_module`, or `inline_js` import other than the placeholder, a
  `suspending` import, a `generic_per_mono` import, and any export.
- Cargo config: none needed. rust2-ts runs cargo from the package's directory, so a
  `.cargo/config.toml`'s `build.rustflags`, such as web-rs's
  `--cfg=web_sys_unstable_apis`, apply beside the profile's. The spike's census compiled
  web-sys's `VideoEncoder` that way.
- Differential test: a fixture whose harness calls js-sys (`Math`, `JSON`, `Reflect`,
  `Array`, typed arrays) and web-sys's non-DOM globals (`TextEncoder`, `console`). Its
  native side is the wasm32 build through `wasm-bindgen-cli_0_2_129`, which the flake's
  nixpkgs has, with the fixture's lockfile pinned to 0.2.129. Its host is Bun, as
  `tests/differential.ts` runs today. It covers each import kind, `catch`, the boundary
  conversions' edge values (an `f64` past `u32`, `NaN`, lone surrogates), `JsCast`
  failures, and refusals of each other module kind.

Docs: [well-known crates](/ts/README.md#well-known-crates) gains wasm-bindgen's mapping
and the supported schemas.

Public API: the translator's output for a crate calling wasm-bindgen imports.
