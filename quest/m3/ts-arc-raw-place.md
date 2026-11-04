# [M] An Arc's raw pointer to a value JS copies

## Goal

[rust2-ts](/ts/README.md) translates two uses of an `Arc`'s or `Rc`'s raw pointer to a
value JS copies, such as a number or an atomic, as native Rust runs them. Differential
tests in the `pointers` fixture against native Rust prove it:

- `Arc::from_raw(ptr::from_ref(&*p))`, and the strong counts, of a pointer remade from a
  shared reference to the value, which today fails at run time.
- `Arc::into_raw(arc) as *const ()` and back, as a waker's data, for an
  `Arc<AtomicBool>`, which today is refused where it is cast.

## Plan

Decided with the maintainer on 2026-11-03, in
[#562](https://github.com/moq-dev/rust2/pull/562): that PR landed with both gaps failing
loudly, and this quest closes them.

The `Arc` gives such a value a stable identity: `rt.arcPlace` makes one place per
allocation, which `into_raw` and `as_ptr` give and `rt.arcRaw` notes. Elsewhere, a
pointer to a value JS copies is a new place each borrow. So a shared reference to the
value, which is the value itself, loses that identity, and `ptr::from_ref` of it makes a
place no table knows. For the same reason, #550's cast to `*const ()` refuses every
pointer to a value JS copies. Either fix has to tell an `Arc`'s place apart from a
borrow's, at translation time where the body shows where the pointer came from and at
run time otherwise, and keep refusing a borrow's place with its span.
