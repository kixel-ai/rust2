# [M] Addresses as identity

## Goal

[rust2-ts](/ts/README.md) translates a pointer's address read as an integer where a crate
uses it only as an identity: compared, hashed, or ordered as a key, such as
`Arc::as_ptr(a) as usize` and `Weak::as_ptr(w) as usize`, proven by differential tests
against native Rust. Today each pointer-to-integer cast is refused.

## Plan

moq-net's model keys its cache and frame accounting by address six times at the pin
(`model/cache.rs`, `model/frame.rs`, `model/group/content.rs`).

Open question for the maintainer, since the 2026-10-28 decision refuses pointer-to-integer
casts:

1. An address is an id unique among live objects, assigned on first read and kept in a
   `WeakMap`, a multiple of the type's alignment so alignment checks hold, while an
   integer cast back to a pointer stays refused (recommended: it closes the common
   identity idiom for every crate, and Rust promises nothing more of an address's value).
2. Keep the refusal, and moq gives these a `cfg(rust2_ts)` twin keyed by an id of its own.

Either way, crossbeam-epoch's tagged pointers stay refused: they cast the integer back.
