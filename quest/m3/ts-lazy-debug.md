# [XS] Debug of a LazyCell or a LazyLock

## Goal

[rust2-ts](/ts/README.md) translates `{:?}` of a `LazyCell` and a `LazyLock` as std
writes them: `LazyCell(..)` of its value once forced, or `LazyCell(<uninit>)` unpadded
before, and likewise for `LazyLock`. This is proven by differential tests against native
Rust, plain and `{:#?}`, before and after a force, including a print from within its own
initialization.

## Plan

Found after [#484](https://github.com/moq-dev/rust2/pull/484) translated `Debug` of the
other cells and locks. A probe on 2026-11-02 against `main` refused both as an
"unsupported call to `<LazyCell<..> as Debug>::fmt`".

std reads `get` without forcing. The runtime's `rt.LazyCell` keeps its value and its
initialized flag in `#private` fields, so it needs a method that reads them, as
`onceDebug` reads a one-time cell's. `LazyCell::get` and `LazyLock::get` could come with
it if that is the same method.

Public API: the translator's output.

## Related

- [Whole writes into a lazy cell or thread-local key](/quest/m3/ts-lazy-writes.md) - the same `#private` state
