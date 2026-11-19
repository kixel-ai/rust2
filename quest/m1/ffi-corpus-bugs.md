# [S] Fix the corpus's binding bugs

## Goal

errno, bytes, axum, and hyper bind and leave `KNOWN` in `ffi/tests/fixtures/corpus.rs`.

## Plan

Kept by the maintainer on 2026-11-19, in the pick of [#878](https://github.com/kixel-ai/rust2/pull/878).

Found by [the corpus](/ffi/tests/fixtures/corpus.rs) on 2026-11-19:

- errno: C++ lifts `__errno_location`'s result as an `Errno` where its declaration returns
  a function pointer.
- bytes: Kotlin's `Bytes.splitOff` reads `this.handle`, which an object of `Bytes`'s
  shape lacks.
- axum and hyper: the glue crate depends on `bytes` twice, at the facade's version and at
  another, which cargo refuses.

Recommended: reduce each to a case in the api fixture first, since each is a generator bug
the fixtures do not reach yet.

Public API: none.
