# [S] C and C++: zero-copy numeric slices, must-use returns, lifetime checks

## Goal

- A borrowed numeric slice parameter (`&[f32]`, `&[u64]`, any element Rust borrows in
  place) takes a pointer-and-length view in C++ that converts from any contiguous range
  and, from C++20, `std::span`, instead of `const std::vector<T>&`, so a caller holding a
  C array or an audio callback's `float*` no longer allocates and copies.
- `result`, `future`, `task`, and `buf` are `[[nodiscard]]`; the C header marks every
  function that returns a handle, a `buf`, a `vec`, or a fallible `bool` with C23
  `[[nodiscard]]` or `__attribute__((warn_unused_result))`.
- `buf` is `[[gsl::Owner]]` and `view`/`items` are `[[gsl::Pointer]]`, and the strict
  build adds Clang's `-Wlifetime-safety`, so a view outliving its `buf` across statements
  fails to compile.
- The C header's comments are `/** */`, which Doxygen reads.

`tests/api_strict.cpp` gains a failing case for a discarded task, a discarded result,
and a view kept past its buffer.

## Plan

Found by the 2026-11-18 language audit. Evidence: `ffi/src/cpp.rs:258-265` returns
`std::vector<T>` for block element types; `nodiscard` appears nowhere in
`ffi/runtime/rt.hpp` or `ffi/src/cpp.rs`, and destroying a `task` cancels it
(`rt.hpp:752-777`), so `std::move(f).then(exec, cb);` never runs the callback;
`comment()` in `ffi/src/c.rs:610` emits `/* */`.

Decided with the maintainer on 2026-11-18: ✅ breaking changes land in this epic before
moq pins, as soon as possible (rejected: after the pin).

Public API: breaking for C++ callers passing `std::vector` lvalues to numeric slice
parameters only where no implicit conversion applies; C and C++ callers that drop a
must-use return now get a warning, an error under `-Werror`.

## Required

- [API surface lock](/quest/m1/ffi-export/api-surface.md) - its CI check shows this PR's break in every language's rendered API
