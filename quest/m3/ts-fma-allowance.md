# [XS] Waiting: drop the lang fixture's mul_add allowance

## Goal

Waits on upstream. wasm32's `fma`, compiler-builtins' generic libm, rounds some
near-ties a last place off
([rust-lang/compiler-builtins#1353](https://github.com/rust-lang/compiler-builtins/issues/1353)),
so the lang fixture's differential accepts a `floats::mul_adds` result where the
translation is the exactly rounded one and native is a last place from it
(`ts/fixtures/lang/fma.ts`).

Once [rust-lang/compiler-builtins#1358](https://github.com/rust-lang/compiler-builtins/pull/1358)
ships in the compiler-builtins that the pinned nightly in `ts/rust-toolchain.toml`
builds, delete `fma.ts`, its test `ts/tests/fma.ts`, and `within`'s call to it, so
any difference fails again, and delete this quest.
