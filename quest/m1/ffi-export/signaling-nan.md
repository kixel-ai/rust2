# [M] NaN bits cross Python, Dart, and the web exactly

## Goal

Every `f32` and `f64` crosses each binding and returns with its exact bits, a NaN's
payload, sign, and quiet bit included. Today Python and Dart quiet an `f32`'s signaling
NaN, and the web binding loses any NaN's bits: it quiets a signaling NaN, and once V8
optimizes the code it canonicalizes a quiet NaN's payload and sign too. The shape tests'
`KNOWN` entries for them are deleted.

## Plan

Found by the shape tests (`ffi/tests/fixtures/shapes.rs`), whose floats draw signaling and
quiet NaNs, and whose native transcript marks each call that digests a NaN of its own bits
(`nan signaling f32`, `nan quiet f64`), which `KNOWN` skips. AGENTS.md's lossless rule
allows a native type only where it is exact.

Python's `float` and Dart's `double` are 64-bit, and widening an `f32` to them with the
hardware's conversion sets the quiet bit (`cvtss2sd` does), as narrowing back does.
Recommended: widen and narrow by bits instead, keeping the native type: an `f32` NaN's
payload shifted into an `f64` NaN's, its quiet bit kept, so the round trip is exact.

On the web the engine is where the bits go, so no glue fix keeps them in a number. Run in a
loop under Node 24, `Holder246.swap(h, v)` with `v[0]` the quiet NaN `0x7ffde633` hands Rust
those bits for the first 8 iterations, then `0x7fc00000` once V8 tiers the code up; the
nightly for `RUST2_SHAPES_SEED=20261119` failed the same way on `held 246` and `held 253`
(`f64` `0xfffff56f76bde633`). `tests/api.ts` already keeps NaN bits out of numbers for
Deno's V8.

## Waiting

- Maintainer: how the web binding holds a float, since a number cannot keep a NaN's bits.
  Options: (a) a runtime float, `rt.f32` and `rt.f64`, a number but for a NaN of its own
  bits, which crosses as its bits, as `rt.u64` is a number but past a safe integer,
  breaking for every web caller of a float; (b) floats stay numbers, and the README says
  the web keeps no NaN's bits, which breaks the lossless rule for one value class. An agent
  cannot pick, since it is a public API break against a rule only the maintainer may relax.
  Recommended: (a), since V8 is where the bits are lost.
