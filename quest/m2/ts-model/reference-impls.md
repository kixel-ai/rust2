# [XS] std's impls through a reference

## Goal

[rust2-ts](/ts/README.md) translates calls of std's blanket impls on references that
forward to the referent's, such as `<&[u8] as AsRef<[u8]>>::as_ref`, which a generic
`B: AsRef<[u8]>` instantiated at `&[u8]` reaches, proven by differential tests against
native Rust.

## Plan

moq-net's `model/frame.rs` reaches `AsRef` this way three times at the pin. Check
`Borrow`, `Deref`, and the comparison traits through `&` for the same gap while there.
