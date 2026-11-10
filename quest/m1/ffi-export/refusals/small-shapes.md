# [XS] Arrays export

## Goal

The census at the moq pin refuses no item of moq's own for an array. One is left:
`tokio::tls::parse_fingerprint` returns `Result<[u8; 32]>`, a moq-tokio item the narrowed
stand-in still exports.

## Plan

Changing moq is a non-goal: the array gets a C shape here, never a request for moq to
change its signature.

`u128`, const generics, a `()` parameter, and an enum without variants export since
[moq-dev/rust2#675](https://github.com/moq-dev/rust2/pull/675); `Result` and `!` off a
return left the census with the narrowed stand-in.

## Waiting

- Maintainer: how `[T; N]` maps. Options: (a) a returned `[u8; N]` is bytes, as `Vec<u8>`
  is, and a parameter `[u8; N]` is bytes whose length the glue checks, raising the
  language's error on a mismatch; (b) a C struct `<prefix>_array_<T>_<N>` holding the
  array, with each language's fixed-size form above it; (c) leave arrays refused and
  keep this quest open until a consumer needs one. Recommended: (a), which reuses the
  bytes every binding already has and clears `parse_fingerprint` with no new type; any
  other element type stays refused until one is needed.
