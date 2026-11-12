# [S] Refuse unsafe functions

## Goal

[rust2-ffi](/ffi/README.md) fails the export at the span of every `unsafe fn` it would
otherwise call: a free function, an inherent or trait method, a C-variadic definition
(stable since Rust 1.99), and an `extern "custom"` function (stable since 1.100). Today a
`pub unsafe fn raw(x: u32) -> u32` exports as an ordinary call that safe Kotlin, Swift, and
Python can make without upholding its preconditions, and
`pub unsafe extern "C" fn sum(n: u32, mut args: ...)` exports as `probe_sum(p0: u32)`, its
variadic arguments silently dropped (reproduced on 2026-11-12 against main at `b9ccf342`).

## Plan

Decided with the maintainer on 2026-11-12, planning from the Rust 1.100 release:

- ✅ Refuse with a span (rejected: leaving it out and listing it in the census as C can
  never hold it). AGENTS.md makes an unsupported construct a compile error, and a binding
  checks at run time only the rules its language cannot express, which a function's
  documented safety contract is not. The census's left-out list is for what C cannot
  hold, which an `unsafe fn` is not.
- ✅ m1, in this epic (rejected: m0, m3): a soundness hole in the exporter moq is about
  to pin.

Recommended: refuse on the signature's safety, and also on `c_variadic` and an ABI the
glue cannot call from Rust, so a C-variadic or custom-ABI function is refused under its
own construct name rather than as a plain `unsafe fn`. Add each case to the reject fixture
and its `expected.txt`.

moq's facade has one `pub unsafe fn`, `moq_ffi_test_clock_now` in moq-ffi, already
refused for its raw pointer, so moq needs no change.
