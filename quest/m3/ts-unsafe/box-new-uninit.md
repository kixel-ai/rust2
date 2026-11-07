# [S] A Box of one uninitialized value

## Goal

[rust2-ts](/ts/README.md) translates `Box::new_uninit` and `Box::new_zeroed` for one
value, written through `Box<MaybeUninit<T>>::write` or its pointer and taken by
`assume_init`, proven by differential tests against native Rust, including drops.

## Plan

Suggested on [#606](https://github.com/moq-dev/rust2/pull/606), which translated the slice
forms (`Box::new_uninit_slice`, `write_clone_of_slice`). `ts/src/lower/library.rs` maps
`new_uninit_slice` and `assume_init`, and `pointer::allocates` already names
`new_uninit` and `new_zeroed`, but neither single-value constructor is mapped. Check
which of `assume_init` and `write` already cover a single value before adding them.

Decided with the maintainer on 2026-11-07: ✅ m3, in this epic (rejected: m2, drop). No
consumer at the moq pin.

Public API: the translator's output.
