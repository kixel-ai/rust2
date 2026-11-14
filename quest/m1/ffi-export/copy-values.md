# [M] Copy types that change cross by value

## Goal

A [rust2-ffi](/ffi/README.md) export's `Copy` type with a `pub` field or a `pub` method
taking `&mut self` crosses as an opaque value, as a `Send` `Copy` type with only private
fields and no padding under any field order does since
[#742](https://github.com/kixel-ai/rust2/pull/742) and
[#811](https://github.com/kixel-ai/rust2/pull/811), instead of an `Arc` handle. A setter
or `&mut self` method writes the new bytes back into the language's own copy, so
`x.add(other)` changes `x` and only `x`, as in Rust. moq has five today, each
still a handle: `stats::Traffic`, `stats::Presence`, `stats::Failures`, and
`stats::Refusals` (`add`), and `clock::WallMap` (`sample`).

## Plan

- C takes `T *` for `&mut self` and writes the value back in place; C++ makes the method
  non-`const`; Swift makes it `mutating`; Go takes a pointer receiver. Each writes only
  the caller's copy.
- A `pub` field gets a getter and a setter over the bytes, through Rust, as an object's
  field does; the bytes stay opaque.
- Prove it with the api fixture's differential tests: a `Copy` type with a `pub` field and
  a `&mut self` method, mutated through a copy and through the original, and the copy left
  as it was.
- Kotlin, Dart, and Python, whose values are immutable, hand back a `&mut self` method's
  result as its new value, `x = x.add(o)`, with Rust's own return, if any, as a pair:
  Python's `Value` and Kotlin's class over words are shared by reference, so mutating one
  in place would change every alias, unlike a Rust copy. ✅ the method returns the new
  value (rejected: a mutable wrapper type, which allocates; keeping such types as handles
  in these three, which splits one Rust type across languages).
- Then `ffi/src/collect.rs`'s `changed`, which keeps these types objects today, goes.

Public API: how every generated API holds these types (breaking).

## Related

- [Kotlin values without a LongArray](/quest/m1/ffi-speed/kotlin-values.md) - how Kotlin holds the bytes this writes back
