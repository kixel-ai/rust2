# [S] Kotlin: a call closes the handles it moved, not those its argument holds after

## Goal

A Kotlin call that moves objects out of a collection argument, such as `logCounts`'s
`more: List<Counter>`, closes exactly the handles it encoded, however it ends, even when a
callback it reaches changes that collection before it returns: the handles Rust took are
abandoned, and any other stays open.

## Plan

Found by CodeRabbit on [kixel-ai/rust2#922](https://github.com/kixel-ai/rust2/pull/922).
`Gen::moving` in `ffi/src/kotlin.rs` runs `after`, `Gen::close`'s statements, which walk
the argument again (`more.forEach { v -> v.handle.abandon() }`). A `MutableList` passed in
and changed by a reentrant callback, A replaced by B, abandons B, which Rust never took, and
leaves A's wrapper open. It was so on main's success path before #922, which runs the same
`after` on the unwind path too.

Recommendation: collect the moved handles once, as the encoding walks the argument, and
abandon that snapshot: either an array the encoder fills beside `__Writer`'s `held`, or
the writer marking which held handles move. Avoid copying the collection per call. A
regression test in `tests/api.kt` replaces an element of `more` from `Log.log`.

## Waiting

- Maintainer: keep this, or drop it since a Kotlin caller changing a read-only `List`
  argument while the call runs is a misuse; recommended: keep, as abandoning a live handle
  leaks its value and leaves the object unusable.
