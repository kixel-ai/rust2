# [M] Wakers made from a RawWaker

## Goal

[rust2-ts](/ts/README.md) translates a waker built by hand: `RawWakerVTable::new`,
`RawWaker::new`, `Waker::from_raw`, `Waker::new`, `RawWaker::from` an `Arc` of a `Wake`
impl, and `==` of two vtables. Each clone, wake, and drop calls the vtable's function as
std's does, proven by differential tests against native Rust that log each call.

## Plan

Found while closing the rest of `std::future` and `std::task`. No consumer builds a raw
waker yet; the maintainer plans all of std regardless.

- `rt.Waker` already keys its identity on `data` and `vtable`. A vtable could be a frozen
  object of its four functions, and a raw waker a class that calls them, with `data` the
  `*const ()` identity a waker's data already is.
- The vtable's functions are `unsafe fn` pointers in a `static`, so this may lean on
  [unsafe code the translator can model](/quest/m3/ts-unsafe.md).
- `Debug` of a `Waker`, a `Context`, a `RawWaker`, and a `RawWakerVTable` writes
  addresses, which JS has no counterpart of: those stay refused.

## Related

- [Unsafe code the translator can model](/quest/m3/ts-unsafe.md) - the `unsafe fn` pointers a vtable holds
