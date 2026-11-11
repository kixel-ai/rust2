# [XS] Arrays export

## Goal

The census at the moq pin refuses no item of moq's own for an array. Two are left:
`tokio::tls::parse_fingerprint` returns `Result<[u8; 32]>`, a moq-tokio item the narrowed
stand-in still exports, and `mux::codec::flac::Config` has a `pub md5: [u8; 16]` field,
which its getter returns and its setter takes.

## Plan

Changing moq is a non-goal: the array gets a C shape here, never a request for moq to
change its signature.

`u128`, const generics, a `()` parameter, and an enum without variants export since
[moq-dev/rust2#675](https://github.com/moq-dev/rust2/pull/675); `Result` and `!` off a
return left the census with the narrowed stand-in.

Decided 2026-11-11 (maintainer, /quest-unblock): ✅ (b) `[T; N]` crosses as a C struct
`<prefix>_array_<T>_<N>` holding the array by value, with each language's fixed-size form above
it, such as Swift's `InlineArray` and a generated Kotlin type that checks its size: a proper
new type, with no allocation, that C compiles out (rejected: (a) bytes with a length check;
(c) leaving arrays refused). Element types beyond `u8` follow the same shape when one is
needed.
