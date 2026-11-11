# [L] Kept closures, a borrowed future, and a seeking reader export

## Goal

The census at the moq pin refuses none of the four items left for a kept closure, a
borrowed future, or a reader it seeks:

- kio's `clock::drive`, which keeps an
  `impl FnMut(Instant, &Waiter) -> Result<Option<Instant>, E> + Unpin` until it fails.
- kio's `Waiter::batch`, whose lent `impl FnOnce() -> R` returns a type parameter.
- kio's `Waiter::poll_future`, which polls a `Pin<&mut F>` of an `F: Future + ?Sized`.
- moq-mux's `container::mp4::repair`, which seeks a `&mut impl Read + Seek`.

## Plan

Decided 2026-11-11 (maintainer, /quest-unblock): ✅ (a) the exporter gains a foreign future (a
poll function, its data, and `free`, the inverse of the futures it returns) and a kept closure
(a function, its data, and `free`), each wrapped in every language's native future and
function, so moq changes nothing (rejected: (b) moq's signatures change). `repair` then takes
a foreign `Read + Seek`; `panic::message` and `tls::Serve::resolver` go to moq to wrap or
drop. The maintainer dislikes callbacks and closures in an API, so a moq quest for a
closure-free API is planned separately only if research finds a simpler one; it does not gate
this quest.

The foreign future landed in [moq-dev/rust2#705](https://github.com/moq-dev/rust2/pull/705):
a type parameter bounded by `Future` or `IntoFuture` of `()` is
`Pin<Box<dyn Future<Output = ()> + Send>>`, `<prefix>_foreign_future` in C, and each
language's own async form, so `clock::timeout` and `clock::timeout_at` export. The same PR
stopped the stand-in re-exporting `moq_tokio::panic`, so `panic::isolate` and
`panic::message` left. `server::Request::within` leaves through a planned moq quest, and
`kio::wait` through the [pin bump](/quest/m1/ffi-refusals/moq-pin.md).

What remains:

- A kept closure: a function, its data, and `free`, as `<prefix>_foreign_future` is, which
  Rust may call from any thread until it frees it. `drive` also needs its parameters and
  return to cross a closure: a lent `&Waiter` and a `Result<Option<Instant>, E>`, with `E`
  chosen as a type parameter is.
- `batch`'s `R`, which no bound fixes: `()` or `String` by the rule a type parameter's
  choice follows, decided by the closure's return.
- A borrowed future: `poll_future`'s `F` is `?Sized`, so `dyn Future<Output = ()> + Send`
  lent for the call may stand in for it, the foreign future's poll reused.
- A foreign `Read + Seek`: a table of `read` and `seek` functions over C's data, lent for
  the call, as a trait C implements is.
