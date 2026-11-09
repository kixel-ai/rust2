# [M] Borrow shapes export

## Goal

The census at the moq pin refuses no item of moq's own for a borrow shape. At moq
`ffcb7cd9`, outside the moq-tokio items narrowing the stand-in drops, 12 items remain:

- `&mut` of a value: `hang::container::Frame::encode` and `varint::encode_quic` (an
  `impl BufMut`, taken as `&mut Vec<u8>`), `mux::codec::annexb::length_prefixed_to_annexb`
  (`&mut Vec<u8>`), and `mux::codec::{aac, opus}::Config::parse` and `varint::decode_quic`
  (an `impl Buf`, taken as `&mut Bytes`, which counts as a reference to a handle inside a
  value until `pub use bytes;` is refused);
- a `'static` borrow: `Path::from_static`;
- a borrowed return that is not `Clone`: `frame::Producer::group` and
  `mux::container::ts::Follower::export` at its 3 instances;
- an `Arc` of a rustls type: `tokio::tls::Listen::server_config` and
  `tokio::tls::Serve::config`.

## Plan

Keep the C ABI's rules: no buffer is lent mutably across it, and bulk payloads stay
zero-copy. A borrow that cannot meet them, such as a returned borrow of an object that
cannot be cloned, keeps its refusal and goes to the maintainer as moq's signature to change.

`Option<&T>` parameters export, lent as the reference alone is
([moq-dev/rust2#678](https://github.com/moq-dev/rust2/pull/678)).

## Waiting

- Maintainer: how a `&mut` of a value crosses. Options: (A) in and out: C passes the value
  in, as one taken by value, and Rust writes what the call left through a pointer on every
  path, its errors included; C++ takes a reference, Swift an `inout`, Go a pointer, and
  Kotlin, Python, and Dart a mutable holder of the value; (B) the call takes the value and
  returns what it left beside its result, which loses what a failed call left. Recommended:
  A, since B breaks the error path.
- Maintainer: whether `Path::from_static` stays refused, for moq to hide from bindings with
  `#[doc(hidden)]` since `Path::new` covers it, or exports by leaking a copy per call.
  Recommended: refused, since a binding has no literal to lend and a leak per call grows.
- Maintainer: `frame::Producer::group` and `ts::Follower::export` lend values that cannot
  be cloned, which no handle can hold; moq returns a `Clone` value or a new handle instead,
  or hides them. Recommended: moq changes them in its own tree.
- Maintainer: the two `Arc<rustls::ServerConfig>` returns become foreign-type refusals once
  `pub use rustls;` is refused, so mapping `Arc<T>` has no consumer. Recommended: moq wraps
  the config in a type of its own, as the refusals epic decided for third-party types.
