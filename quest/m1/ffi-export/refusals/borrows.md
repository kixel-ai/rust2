# [M] Borrow shapes export

## Goal

The census at the moq pin refuses no item of moq's own for a borrow shape: 41 items at moq
`ffcb7cd9` take a reference inside a parameter (`Option<&T>` in
`mux::Source::{bind, follow, resolve, subscribe_track}`), `&mut` of a value (`&mut Vec<u8>`
and `&mut [u8]` in `hang::container::Frame::encode`), a reference to a handle inside a
value (`&Bytes`), an `Arc<T>`, a `'static` borrow, or return a borrow of a type that is not
`Clone`.

## Plan

Keep the C ABI's rules: no buffer is lent mutably across it, and bulk payloads stay
zero-copy. A borrow that cannot meet them, such as a returned borrow of an object that
cannot be cloned, keeps its refusal and goes to the maintainer as moq's signature to change.
