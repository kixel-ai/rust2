# [S] 64-bit math through BigInt.asUintN

## Goal

An A/B route on `just ts bench` measures 64-bit arithmetic past 2^53 as
`BigInt.asUintN(64, a op b)` (and `asIntN`) against today's two-word objects
(`ts/runtime/rs/int.ts:1463`, `:1494`), in Chrome and Firefox, and the translation adopts
whichever is faster per operation, with the differential tests passing.

## Plan

V8 compiles `BigInt.asUintN(64, ...)` to native 64-bit operations once inlined
([v8:9407](https://groups.google.com/g/v8-reviews/c/SF2tmxAUpB8)); a Go-to-JS compiler
measured multiply from 38 to 2.2 ns and division from 123 to 6 ns in Chrome
([goscript#480](https://github.com/s4wave/goscript/pull/480)). [#89](https://github.com/kixel-ai/rust2/pull/89)
measured `u128` as a plain `bigint`, not the wrapped 64-bit form. If V8 and SpiderMonkey
disagree, the browsers decide over Node, and a split between them is reported for the
gate.

Public API: none.
