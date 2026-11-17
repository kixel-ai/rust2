# [L] The exporter's web target off wasm32-wasip1's std, or kept on it

## Goal

rust2-ffi's web target ([`ffi/src/web.rs`](/ffi/src/web.rs), documented in
[the README](/ffi/README.md#typescript-web)) builds the crate for `wasm32-wasip1`, and
`rt.ts` implements the six imports its std makes (`clock_time_get`, `environ_get`,
`environ_sizes_get`, `fd_write`, `proc_exit`, `sched_yield`) with web APIs. Its test runs
on Node and Deno in the gate and in headless Chromium and Firefox nightly, and it has a
column on [the binding benchmark](/bench/ffi/README.md), which lends returned bytes past
the measured crossover.

What remains is the 2026-11-09 goal that the web module ship nothing of WASI: its clock,
randomness, and output reaching web APIs through the binding's own `rust2_*` imports, and
`rt.ts` implementing no `wasi_snapshot_preview1` import. The Plan recommends keeping the
web target on `wasm32-wasip1`'s std instead, which the maintainer decides.

## Plan

Decided with the maintainer on 2026-11-09: ✅ web and WASI are separate targets, and the
web's uses web APIs wherever one exists ("like performance.now()"). Rejected: one
`wasm32-wasip1` module for both, whose imports the binding satisfies.
[The WASI target](/quest/m3/wasi.md) is its own quest.

Decided with the maintainer on 2026-11-16, picking
[#847](https://github.com/kixel-ai/rust2/pull/847) to merge: ✅ merge the first slice on
`wasm32-wasip1`'s std, its WASI imports implemented with web APIs; the 2026-11-09 decision,
no WASI runtime or polyfill in the web bundle, stands as the goal, so this quest moves the
web target off `wasm32-wasip1`. Rejected: retargeting before merging; skipping the slice.

Found sizing the move, in [#851](https://github.com/kixel-ai/rust2/pull/851), on
`nightly-2026-11-12`:

- Off `wasm32-wasip1`, std has no clock, randomness, or output. On
  `wasm32-unknown-unknown`, `Instant::now` and `SystemTime::now` panic ("time not
  implemented on this platform"), std's `fill_bytes` panics, `HashMap`'s keys come from
  addresses, and stdout and stderr are discarded. std has no hook to give it any: a June
  2026 internals thread proposing a `#[global_time_source]` has no RFC, and crates swap in
  `web-time`, which is wasm-bindgen. Giving std a clock without a source change needs a std
  patched through `-Zbuild-std`, which takes a nightly and a consumer's cargo config, or a
  rewrite of the module after `cargo build`: #847 rejected both.
- moq, the first consumer, does not build there. getrandom 0.4, under rand 0.10 in
  moq-net, hang, and moq-mux, is a compile error on `wasm32-unknown-unknown` without its
  `wasm_js` feature, which is wasm-bindgen, or a `--cfg` in `RUSTFLAGS`. moq-net, kio, and
  moq-mux call `Instant::now` or `SystemTime::now` in about 80 places, each a panic there.
  On `wasm32-wasip1`, kio, moq-net, hang, moq-mux, and moq-pattern all build.
- A middle route keeps `wasm32-wasip1`'s std and leaves no WASI import. The web feature's
  glue defines wasi-libc's `__imported_wasi_snapshot_preview1_*` symbols, which wasm-ld then
  resolves inside the module, each calling a `rust2_*` import: on a scratch cdylib, the
  module imported only those. std's randomness imports `random_get` through its own
  `wasip1` crate instead, under a v0-mangled symbol
  (`_RNvNtNtCsimJ0PVPQ56H_6wasip113lib_generated22wasi_snapshot_preview110random_get` on
  this nightly), which the exporter would compute from its toolchain and a consumer
  building on another would miss, leaving the import for `load` to refuse. It ships as much
  of WASI as today, its 24 KB of std on the api fixture, with the shim moved from `rt.ts`
  into Rust.

Recommended: keep the web target on `wasm32-wasip1`'s std, `rt.ts` implementing its
imports with web APIs, delete this quest, and drop "ships nothing of WASI" from
[the epic](/quest/m2/wasm/README.md). Off it, no crate calling std's clock exports for the
web, moq included, and the middle route ships as much of WASI as today. Revisit once std
gains a time source hook or a web target.

## Waiting

- Maintainer: whether the web target leaves `wasm32-wasip1`'s std, which the 2026-11-16
  decision named this quest's. (1) Keep it, as today, and delete this quest: recommended,
  since std works there, moq builds, and the 24 KB of WASI std is the price. (2) The middle
  route: the module imports only `rust2`'s, through a toolchain-mangled symbol a consumer's
  other toolchain breaks, shipping as much of WASI. (3) `wasm32-unknown-unknown`: every
  reachable call of std's clock, randomness, or output refused at export through
  [the refusal](/quest/m2/wasm/refuse.md), so moq and every crate keeping time export for
  no web. Answered in [#851](https://github.com/kixel-ai/rust2/pull/851) or this Plan.

## Related

- [Refusing what a wasm32 target lacks](/quest/m2/wasm/refuse.md) - what option (3) needs, and the refusal this quest's Goal held
- [The WASI target](/quest/m3/wasi.md) - the other wasm32 target
