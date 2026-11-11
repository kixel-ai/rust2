# [M] Returned wakers and contexts export

## Goal

kio's `Waiter::waker`, which returns `&Waker`, and `Waiter::context`, which returns
`Context<'_>`, export at moq's pin: the last of the census's `Poll`, `Context`, and
`Waker` refusals that no sibling owns.

## Plan

A function returning `Poll` already exports as a future, and a `&Context` parameter as a
`<prefix>_waker`, so a waker crosses into Rust; this adds the other direction.

The maintainer settled on 2026-11-09: ✅ proper waker support (rejected: moq marks both
`#[doc(hidden)]`, since rust2 neither hides APIs nor worsens moq's Rust API for bindings).

- The C ABI returns a `<prefix>_waker` wrapping a clone of Rust's waker, with functions
  to wake it and drop it. A returned `&Waker` clones; a returned `Context<'_>` exports as
  its waker, since a context is only a waker lent for a call.
- Each of the seven generators wraps the returned waker in its language's callable, which
  wakes it, and frees it when the callable is dropped or collected.
- The api fixture's `Waiter` gains both methods, and every language's test wakes a pending
  future through the returned waker.
- moq-net's `Transport::poll_lease`, whose `Poll` holds `Self::Lease<'_>` borrowed from the
  transport, stays refused as a borrowed `Poll`, since its future would own what the lease
  borrows; this quest owns it if narrowing the stand-in keeps it.
