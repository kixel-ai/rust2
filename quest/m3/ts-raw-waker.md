# [M] A raw waker's data read back

## Goal

[rust2-ts](/ts/README.md) translates the vtable functions of a hand-built waker that read
their data: a `*const T` cast to `*const ()` and back, and `Arc::into_raw`, `from_raw`,
`increment_strong_count`, and `decrement_strong_count`, which a raw waker's data most
often goes through, proven by the lang fixture's `raw` differential case against native
Rust.

## Plan

[#550](https://github.com/moq-dev/rust2/pull/550) translated the rest of the raw waker
API: vtables, `RawWaker`, `Waker::from_raw` and `new`, `RawWaker::from` an `Arc`, and
`==`. Its vtable functions can tell their data apart only by identity: null, dangling, or
another waker's data passed through.

- A cast between `*const T` and `*const ()` is refused today as reading one type's bytes
  as another. Erasing keeps the pointer's JS value, which is its identity, but casting
  back to another type than the one erased, which Rust defines for some pairs such as
  `[u8; 4]` and `u32`, must fail loudly at run time rather than read the wrong JS value.
  How an erased pointer remembers its type is the open decision on #550.
- An erased pointer to a value JS copies is an `rt.Place`, a new identity each borrow, so
  erasing one breaks `will_wake`: refuse it, as comparing such pointers is.
- `Arc::into_raw`'s pointer must lead `from_raw` back to the `Arc` and its count, though
  a pointer to a value JS keeps as an object is that object.
- `Debug` of a `Waker`, a `Context`, a `RawWaker`, and a `RawWakerVTable` writes
  addresses, which JS has no counterpart of: those stay refused.

## Related

- [Unsafe code the translator can model](/quest/m3/ts-unsafe.md) - the pointer model this extends
