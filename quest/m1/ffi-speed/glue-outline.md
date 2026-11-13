# [XS] Outline the per-export wrappers in the Dart and Python glue

## Goal

The generic wrapper each [rust2-ffi](/ffi/README.md) export instantiates in the Dart and
Python glue keeps only its fast path inline, so an export's library shrinks. Measured by
the 2026-11-12 audit on the api fixture's default release build, with `nm -S`: Dart's
`Home::serve<R, F>` (`ffi/runtime/dart.rs`) is 692,824 bytes in 1,018 symbols of a
2,644,048-byte stripped library, 26% of it, and Python's `rt::call<T, F>`
(`ffi/runtime/python.rs`) is 282,236 bytes in 441 symbols of 2,169,696, 13%.

## Plan

- Python: `call` stays inline. Splitting it into non-generic halves shrinks the stripped
  library to 1,932,952 bytes, but adds 26 instructions to each handle call, which the
  handle call must not do. The split is in commit `24500300`, reverted by the next commit.

Public API: none.

## Waiting

- Maintainer: keep the Python glue's `call` inline, or take the split. The split saves
  236,744 bytes, 11% of the Python library, and costs 26 instructions per handle call
  (1,768 to 1,794 under callgrind, on `Counter.edited()`). Recommendation: keep it inline,
  since the handle call must not slow and the library's size is the lesser cost. To take
  the split, restore `ffi/runtime/python.rs` from `git show 24500300:ffi/runtime/python.rs`.
