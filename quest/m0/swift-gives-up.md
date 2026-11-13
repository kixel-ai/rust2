# [XS] api_swift waits for the error an async closure's timeout raises

## Goal

`api_swift` in `ffi/tests/fixtures.rs` passes every run: its "Rust gives up on an async
closure" check in `ffi/tests/api.swift` no longer fails intermittently.

## Plan

Found while validating [#740](https://github.com/kixel-ai/rust2/pull/740) on 2026-11-13:
`just ffi test --test fixtures` failed `api_swift` there in 2 of 4 runs, and at `main`
`7008d1d3` in 1 of 5, each time with `"futures" failed: Rust gives up on an async closure`.
The test passes alone.

Two races fit, and the fix covers both:

- The closure is empty, so it can finish before the budget runs out: `Pollable.poll` in
  `ffi/runtime/rt.swift` starts the closure's Task and then loads `done` again, and
  `Within::poll` in `ffi/fixtures/api/src/lib.rs` polls the inner future before checking
  its budget. When the Task finishes in that window, `within(0) {}` returns `Ok(())` and
  nothing throws. The closure must stay pending, such as by awaiting a stream nothing
  yields to, as the check above it does.
- The check reads `threw` after a single `q.drain()`, so it also fails if the task's
  resumption with the `Overflow` error reaches the queue after `drain` returned. The check
  above it waits with `until { q.drain(); return got.value != 0 }`; wait the same way.

Confirm which one fails before fixing, such as by delaying the closure's Task or the
resumption, and fix the test's timing rather than retrying it.
