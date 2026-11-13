# [XS] Outline the per-export wrappers in the Dart and Python glue

## Goal

The generic wrapper each [rust2-ffi](/ffi/README.md) export instantiates in the Dart and
Python glue keeps only its fast path inline, so an export's library shrinks. Measured by
the 2026-11-12 audit on the api fixture's default release build, with `nm -S`: Dart's
`Home::serve<R, F>` (`ffi/runtime/dart.rs`) is 692,824 bytes in 1,018 symbols of a
2,644,048-byte stripped library, 26% of it, and Python's `rt::call<T, F>`
(`ffi/runtime/python.rs`) is 282,236 bytes in 441 symbols of 2,169,696, 13%.

## Plan

- Dart: keep the check for no relays and the call inline, and move the relay loop into
  one `#[inline(never)]` function taking `&mut dyn FnMut()`.
- Python: split `call` into non-generic enter and leave halves around the inlined call.
- Report the fixture's and moq's stripped sizes before and after, and the bench's handle
  call in both languages, which must not slow.

Public API: none.
