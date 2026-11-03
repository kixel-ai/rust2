# [M] A raw waker's data through an Arc

## Goal

[rust2-ts](/ts/README.md) translates `Arc::into_raw`, `from_raw`,
`increment_strong_count`, and `decrement_strong_count`, which a hand-built waker's data
most often goes through, proven by the lang fixture's `raw` differential case against
native Rust.

## Plan

[#550](https://github.com/moq-dev/rust2/pull/550) translated the rest of the raw waker
API, and a `*const T` cast to `*const ()` and back, which keeps the pointer's JS value and
checks its type at run time.

- `Arc::into_raw`'s pointer must lead `from_raw` back to the `Arc` and its count, though
  a pointer to a value JS keeps as an object is that object.
- A pointer to a struct and one to the object of its field at offset 0, both cast to
  `*const ()`, are one address in Rust and two objects in JS, so `==` and `will_wake` of
  them differ from native's. Refuse the second cast where one body shows both, and fail
  at run time otherwise, as `ptr::addr_eq` of pointers to two types is refused.
- `Debug` of a `Waker`, a `Context`, a `RawWaker`, and a `RawWakerVTable` writes
  addresses, which JS has no counterpart of: those stay refused.

## Related

- [Unsafe code the translator can model](/quest/m3/ts-unsafe.md) - the pointer model this extends
