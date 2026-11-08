# [XS] Waiting: drop the lang fixture's mul_add allowance

## Goal

wasm32's `fma`, compiler-builtins' generic libm, rounds some
near-ties a last place off
([rust-lang/compiler-builtins#1353](https://github.com/rust-lang/compiler-builtins/issues/1353)),
so the `lang/floats` fixture's differential accepts a `floats::mul_adds` result where the
translation is the exactly rounded one and native is a last place from it
(`ts/fixtures/lang/floats/fma.ts`).

Then delete `fma.ts` and every use of it: `floats/caller.ts`'s import and
`within`'s call, and its test `ts/tests/fma.ts` with `fma_allowance` in
`ts/tests/runtime.rs`, so any difference fails again, and delete this quest.

## Waiting

- rust-lang/compiler-builtins: [#1358](https://github.com/rust-lang/compiler-builtins/pull/1358)
  ships in the compiler-builtins that the pinned nightly in `ts/rust-toolchain.toml`
  builds.
