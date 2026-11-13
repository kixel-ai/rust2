# [XS] api_swift waits for the error an async closure's timeout raises

## Goal

`api_swift` in `ffi/tests/fixtures.rs` passes every run: its "Rust gives up on an async
closure" check in `ffi/tests/api.swift` no longer fails intermittently.

## Plan

Found while validating [#740](https://github.com/kixel-ai/rust2/pull/740) on 2026-11-13:
`just ffi test --test fixtures` failed `api_swift` there in 2 of 4 runs, and at `main`
`7008d1d3` in 1 of 5, each time with `"futures" failed: Rust gives up on an async closure`.
The test passes alone.

The check reads `threw` after a single `q.drain()`, so it fails if the task's resumption
with `within(0)`'s `Overflow` error reaches the queue after `drain` returned, as it can
when the Rust future is polled off the queue. The check above it already waits with
`until { q.drain(); return got.value != 0 }`; confirm the race first, such as by delaying
the resumption, then wait the same way rather than retrying the test.
