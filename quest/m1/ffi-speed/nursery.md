# [M] A nursery for Kotlin's and the web's returned objects

## Goal

Kotlin's and the web binding's object returns and lent `Bytes` register for cleanup only
when they leave a 64-object nursery unclosed, as Go's and Dart's do since
[#838](https://github.com/kixel-ai/rust2/pull/838), and the binding benchmark reports the
object-return and `frame.payload` rows before and after on HotSpot, Node, Chromium, and
Firefox.

## Plan

Found by the 2026-11-18 language audit. Kotlin's object return costs about 89 ns wall and
190 ns CPU, mostly a phantom reference linked and unlinked under a lock
(`ffi/runtime/rt.kt:415-466`); `__Handle` must stop being a `PhantomReference` itself.
The web registers every owned handle and lent `Bytes` with `FinalizationRegistry`
(`rt.ts:328`, `:357`, `:735`): an object return costs 355 ns on Node, 272 on Chromium, and
1,104 on Firefox against C's 33. The nursery cut Go's from 254 to 174 ns and Dart's from
225 to 96. JavaScript may register an object at eviction while the nursery still holds it,
so the design carries over.

Public API: none.
