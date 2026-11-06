# [XS] Slices not yet written

## Goal

[rust2-ts](/ts/README.md) translates `Box::<[T]>::new_uninit_slice` and
`<[MaybeUninit<T>]>::write_clone_of_slice`, which are refused with their span, proven by
differential tests against native Rust.

## Plan

A `MaybeUninit<T>` is a `T` in JS, and one of a byte is the byte, so a `[MaybeUninit<u8>]`
is an `rs.ByteSlice`; `write_copy_of_slice` and a slice's `assume_init_ref` and
`assume_init_mut` already map. `write_clone_of_slice` clones in without dropping what was
there, unlike `clone_from_slice`. Found translating `spare_capacity_mut`. moq-net's `model/frame.rs` calls
`Box::<[u8]>::new_uninit_slice` in `Segment::new` at the moq pin.
