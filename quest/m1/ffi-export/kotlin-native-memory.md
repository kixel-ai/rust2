# [M] Kotlin: lent bytes freed before native memory piles up

## Goal

A Kotlin program that reads many large `Bytes` from Rust, such as moq's Android player
reading frames, holds no more of Rust's memory than its live `Bytes` reach plus a bounded
backlog, even when it makes little Java garbage, so a GC that never runs leaves no
growing native heap behind it.

## Plan

Found by [#432](https://github.com/moq-dev/rust2/pull/432), which made bytes over 1 KiB
GC-owned: Rust's buffer is
freed by a cleaner once the GC collects its `Native.Lent`, and the JVM does not see the
buffer's size. A scratch loop returning a fresh Rust buffer per call measured 1.6 to 13 µs
per lent `Bytes` from 1025 B to 16 KiB, against 0.15 to 3.4 µs with every size copied, as
unfreed buffers piled up between collections.

Measure first, at moq's real frame sizes and rates on the JVM and on ART, then pick what
the numbers support. Options to weigh:

- tell the GC the native size where the platform allows it: ART's
  `VMRuntime.registerNativeAllocation` and `NativeAllocationRegistry` are not public API
  at API 24, so check which levels expose either, while the JVM has no equivalent;
- a bounded count or byte budget of live lent buffers, past which a call copies instead
  of lending, or nudges the GC;
- raising the 1 KiB copy threshold where a copy costs less than the backlog.

No `close()` comes back: a free while a slice is live is what the lifetimes quest removed.
Go's `Bytes` has the same shape (`runtime.AddCleanup`); note whether its fix applies there.
