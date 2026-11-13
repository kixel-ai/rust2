# [S] Swift's async-closure timeout test races under load

## Goal

`api_swift`'s futures run never fails "Rust gives up on an async closure" because of host
load: the check waits for the timeout to fire rather than for one drain to happen to see it.

## Plan

`tests/api.swift` launches `within(0) {}` and checks `threw` after a single `q.drain()`
(added in [#705](https://github.com/kixel-ai/rust2/pull/705)). The timer behind a zero
deadline can fire after that drain on a loaded host. It failed once in `just ci` on
[#739](https://github.com/kixel-ai/rust2/pull/739) and passed three reruns. Reproduce it by
loading the host (or delaying the timer's wake) first, then wait with `until` as the
sibling checks do, or drive the deadline from a mocked clock if the runtime takes one.
