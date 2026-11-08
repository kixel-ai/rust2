# [M] u64 wrapping arithmetic near a plain add in V8

## Goal

[rust2-ts](/ts/README.md)'s `u64` wrapping arithmetic (`rs.u64WrappingAdd` and its kin)
costs close to a plain `+` while the value stays a safe integer, so the codec bench's
`parts/prefix-writes` runs near its hand-written plain-array route in Node and Chrome, not
12-13x off it. Exact for every input, proven by differential tests against native Rust past
2^53 and at the wrap.

## Plan

Found by [#656](https://github.com/moq-dev/rust2/pull/656): with `iter_mut` loops stepped by
index, `rs.u64WrappingAdd` costs about 7x a plain `+` per item in Node, and the same loop
with a plain `+ 1` ran at 1.2x the plain array. Bun is already at 2.6-3.5x. Every `u64` loop
in moq pays it. Measure with CPU time or instruction counts in Node, Chrome, and Firefox
(the [bench](/ts/README.md#bench)), and keep the 64-bit representation decided in the
[Design](/ts/README.md#design) section: numbers while safe integers.

Decided with the maintainer on 2026-11-07: ✅ plan it, in m3 (from #656's follow-ups).

Public API: the translator's output and `rs.ts`.
