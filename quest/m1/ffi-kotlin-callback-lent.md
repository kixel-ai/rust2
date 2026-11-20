# [S] Keep a Kotlin callback's lent return alive until Rust reads it

## Goal

A Kotlin closure or callback that returns Rust's lent bytes (a `Bytes` over a `__Lent`, which Rust shares without a copy) keeps the owner reachable until Rust has read the encoded return, so the GC cannot run `__bytes_free` between the return and Rust's read. A differential test returns a lent `Bytes` over 1 KiB from a callback while forcing collections, and Rust reads it intact.

## Plan

Found by CodeRabbit on PR #876, which did not cause it: `__Writer.use { w -> write; w.finish() }` (ffi/src/kotlin.rs:962) releases the writer, and so fences `kept`, before the expression returns the encoding, and Rust reads the lent bytes after that. The `__Lent` KDoc covers only an argument of a JNI call, where the native frame keeps it alive. Hold the owner past the read, for example by returning it with the encoding and fencing after the Rust call returns, or by copying a lent return at this boundary; settle which with a test that fails first. The older `Reference.reachabilityFence` in `release` had the same window on HotSpot, so the API 24 monitor fallback only extends it.
