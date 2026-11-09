# [S] Returned wakers and contexts export

## Goal

kio's `Waiter::waker`, which returns `&Waker`, and `Waiter::context`, which returns
`Context<'_>`, stop refusing at moq's pin: the last of the census's `Poll`, `Context`, and
`Waker` refusals that no sibling owns.

## Plan

A function returning `Poll` already exports as a future, and a `&Context` parameter as a
`<prefix>_waker`. A waker crosses only into Rust, so what is left is the other direction.
moq-tokio's `poll_lease`, whose `Poll` holds a borrowed `Lease<'_>`, is
[borrows](/quest/m1/ffi-export/refusals/borrows.md)'s, if narrowing the stand-in keeps it.

## Waiting

- Maintainer: pick how a returned `Waker` or `Context` exports, since either changes the
  exporter's rules or moq's API. Recommended: moq marks both `#[doc(hidden)]`, as only Rust
  code polling a foreign future needs them, and the exporter keeps refusing a returned
  waker. Rejected unless a binding needs one: the C ABI returns a `<prefix>_waker` that
  wraps Rust's waker, which C calls and drops, and each of the seven generators wraps it in
  its language's callable. Done when the decision is recorded here.
