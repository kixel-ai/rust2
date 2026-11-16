# [L] Export what a mutable method returns borrowing its receiver

## Goal

[rust2-ffi](/ffi/README.md) exports a method taking `&mut self` that returns an object
borrowing its receiver, such as moq-mux's `catalog::Producer::modify(&mut self) ->
Result<Guard<'_, E>>`, in every language. The census then no longer lists `catalog::Guard`
under what C can never hold, and moq's switch can drop the three `modify` calls in
moq-ffi's hand-written wrapper (`rs/moq-ffi/src/media.rs`). Adding exports breaks no generated
API, so this does not hold [moq's pin](/quest/m1/ffi-export/ship.md).

## Plan

Decided with the maintainer on 2026-11-16, picking
[kixel-ai/rust2#844](https://github.com/kixel-ai/rust2/pull/844) to merge: ✅ an exclusive
lend of the receiver, held until the returned object closes. Rejected: leaving `Guard` out.

- Today a `&self` method's returned borrowing object is a lent handle that holds its
  receiver and checks it on each use. The `&mut self` case takes the receiver's exclusive
  borrow for the lent handle's whole life, as a call borrowing it as `&mut` does for one
  call: any other use of the receiver while it is open fails `<PREFIX>_STATUS_BORROWED`,
  and closing the lent handle, or its consuming method such as `Guard::commit(self)`, gives
  the borrow back. The receiver's `_free` while lent leaves the drop to the lent handle's
  close, as a `&self` lend does.
- `Guard` is only useful through `Deref<Target = Catalog<E>>` and `DerefMut`: moq-ffi calls
  `guard.set_section(..)` and `guard.video.set_properties(..)`, so the exporter must reach
  the target too. Recommended: a type with `Deref` gets its target's `&self` methods and
  public fields on its own handle, and with `DerefMut` its `&mut self` methods and field
  setters, each calling through `deref` or `deref_mut`, as Rust's auto-deref does. That
  keeps `Guard`'s `updated` flag exact, since only `deref_mut` sets it. Rejected: a getter
  returning the target cloned and a setter writing it back, which calls `deref_mut` on
  every write and leaves `video.set_properties` unreachable on a copy.
- Prove it with the api fixture: an object whose `&mut self` method returns a guard
  borrowing it, with a `Deref`, a `DerefMut`, and a consuming method, used in every
  language and compared with native Rust in the transcript, plus a use of the receiver
  while the guard is open, which fails.
