# [S] The C ABI under sanitizers and Miri

## Goal

Nightly runs [rust2-ffi](/ffi/README.md)'s api fixture through its C and C++ scripts with
the glue built under AddressSanitizer and ThreadSanitizer, and drives the generated
`extern "C"` functions from a Rust test under Miri. A leak on cancel, a lent handle used
past its call, or a race between a wake and a poll fails the job.

## Plan

Found by the 2026-11-12 audit: no sanitizer, Miri, loom, or fuzz job exists in the
justfiles or workflows, and `ffi/runtime/rt.rs` is 1,394 lines of `unsafe` with no unit
test of its own.

- `-Zsanitizer=address` and `thread` need `-Zbuild-std` on the pinned nightly; the C and
  C++ scripts link the sanitized static library with the matching clang flag.
- Miri cannot call foreign code, so its test calls the glue's `extern "C"` functions from
  Rust, covering the handle state machine, buffers, and futures with a hand-rolled waker.
- Nightly only: none of it joins the merge gate.

Public API: none.
