# [M] Signaling NaNs cross Python, Dart, and the web exactly

## Goal

Every `f32` and `f64` crosses each binding and returns with its exact bits, a signaling
NaN's included. Today Python and Dart quiet a lone `f32`'s signaling NaN, and the web
binding quiets an `f32`'s and an `f64`'s, alone or inside a value; a quiet NaN's payload
and sign already survive. The shape tests' `KNOWN` entries for them are deleted.

## Plan

Found by the shape tests (`ffi/tests/fixtures/shapes.rs`), whose floats now draw
signaling and quiet NaNs: seed 1 of `f32` and `f64` in every position, `0x7f8eedfe` for
`f32`. AGENTS.md's lossless rule allows a native type only where it is exact.

Python's `float` and Dart's `double` are 64-bit, and widening an `f32` to them with the
hardware's conversion sets the quiet bit (`cvtss2sd` does), as narrowing back does.
Recommended: widen and narrow by bits instead, keeping the native type: an `f32` NaN's
payload shifted into an `f64` NaN's, its quiet bit kept, so the round trip is exact.

The web binding quiets an `f64` too, so a JavaScript number cannot be trusted with a
signaling NaN on its way through the boundary or an engine's store; `tests/api.ts`
already keeps NaN bits out of numbers for Deno's V8. First find where it is quieted: the
wasm-to-JS conversion, the glue's typed-array reads, or the engine.

## Waiting

- Maintainer: how the web binding holds a float where a number cannot be exact. Options:
  (a) a runtime float, `rt.f32` and `rt.f64`, a number but for a signaling NaN, which
  crosses as its bits, as `rt.u64` is a number but past a safe integer, breaking for every
  web caller of a float; (b) floats stay numbers, and the README says a signaling NaN is
  quieted on the web, which breaks the lossless rule for one value class. An agent cannot
  pick, since it is a public API break against a rule only the maintainer may relax.
  Recommended: (a), if the engine is where it is quieted; else fix the glue and keep
  numbers.
