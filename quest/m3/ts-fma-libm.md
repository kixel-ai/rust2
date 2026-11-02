# [S] wasm32's mul_add misrounds a near tie

## Goal

The lang fixture's differential test of `f64::mul_add` passes on every input, though
wasm32's native `fma`, compiler-builtins' generic `fma_round`, drops a sticky bit, while
[rust2-ts](/ts/README.md)'s `rt.f64MulAdd` stays rounded once, as `mul_add` documents.

## Plan

Found on 2026-11-02: a lang case added shifted a random input onto `floats::mul_adds`, and
for `a = 0xbe8c27a8edc66e22`, `b = 0xbd522f6292409f88`, `c = 0x3f41e8176d8609aa` the exact
`a * b + c` is just above halfway to the next double. `rt.f64MulAdd` and x86-64's `fma` give
`0x3f41e8176d8609ab`; wasm32, and `libm` 0.2.16's soft `fma` on any host, give `...aa`.

`libm/src/math/generic/fma.rs`, where `z`'s exponent is at least 64 bits above the product's,
computes the sticky bit from `rlo` after shifting it, where musl's `fma.c` reads the
original: `rlo = rhi<<(64-d) | rlo>>d | !!(rlo<<(64-d))`. The sum then ties and rounds to
even. The input no longer lands on `mul_adds`, so the gate passes, until the case count
changes again.

The maintainer's decision:

1. Recommended: accept exactly this divergence in `caller.ts`'s `within`, where native's
   result is the correctly rounded one with that bit dropped, recomputed with `BigInt`,
   report it to rust-lang/compiler-builtins, and drop the allowance once the toolchain has
   the fix.
2. Port the bug into `rt.f64MulAdd`, matching wasm32 but no target with a hardware `fma`.
3. Wait for an upstream fix and toolchain bump, with the gate at risk until then.

Public API: none.
