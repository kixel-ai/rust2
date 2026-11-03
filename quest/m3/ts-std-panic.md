# [M] std::panic

## Goal

[rust2-ts](/ts/README.md) translates the stable API of `std::panic` that can match native
Rust under wasm32's `panic=abort`: `AssertUnwindSafe` made, read, dereferenced, called,
awaited, printed, and defaulted, `catch_unwind`, `resume_unwind`, and `panic_any`, proven by
a differential test against native Rust. What can't match stays refused with its reason.

## Plan

Found by [#528](https://github.com/moq-dev/rust2/pull/528): `AssertUnwindSafe` can't be
made, so it can't be a future or a function either. A panic aborts, so `catch_unwind` only
ever returns `Ok`, and `resume_unwind` aborts without running the hook.

Public API: the translator's output.
