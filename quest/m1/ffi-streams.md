# [M] Stream adapters for async next

## Goal

A method `async fn next(&mut self) -> Option<T>` or `-> Result<Option<T>, E>` gets a
generated adapter in each binding's iteration protocol: Go's `iter.Seq2`, TypeScript's
`AsyncIterable`, Kotlin's `Flow`, Swift's `AsyncSequence`, Python's async iterator, Dart's
`Stream`, and C++'s coroutine generator where the header has coroutines. Cancelling the
iteration cancels the pending `next`, losing nothing, as the futures do.

## Plan

Found by the 2026-11-18 language audit: moq's Go wrapper hand-writes `iter.Seq2` over its
streams, which [the exporter epic](/quest/m1/ffi-export/README.md#goal)'s goal of no
hand-written wrapper rules out; the rule needs no config, since the method's shape
decides it.

Decided with the maintainer on 2026-11-18: ✅ kept, outside the exporter epic since it only
adds API (rejected: dropping it). Recommended: add the adapter beside the method, never in
place of it.

Public API: added adapters in every binding.
