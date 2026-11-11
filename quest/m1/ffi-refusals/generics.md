# [L] Futures and kept closures as parameters export

## Goal

The census at the moq pin refuses no item of moq's own for a future or a kept closure
taken as a parameter, or a reader it seeks: 11 items at moq `ffcb7cd9`, once
[moq-dev/rust2#683](https://github.com/moq-dev/rust2/pull/683) drops the moq-tokio items
outside its session side.

- A future parameter: kio's `Waiter::poll_future`, `clock::timeout`, and
  `clock::timeout_at`, and moq-tokio's `panic::isolate` and `server::Request::within`.
- A closure kept past the call: kio's `clock::drive` and `wait`, and `Waiter::batch`, whose
  lent closure returns a type parameter.
- moq-mux's `container::mp4::repair`, which seeks a `&mut impl Read + Seek`.
- moq-tokio's `panic::message`, which takes a `&(dyn Any + Send)` panic payload, and
  `tls::Serve::resolver`, which takes rustls's `Arc<dyn ResolvesServerCert>`.

## Plan

Generic parameters and traits already export: a parameter bounded by a blanket impl's trait,
as `Client::connect`'s `Boxable` is, exports at each type implementing what the impl requires;
a parameter whose bounds name an earlier one, as `track`'s `impl Into<Option<R>>` does, is
chosen at each choice of it; `impl io::Read` takes bytes and `impl IntoIterator<Item = &X>` a
slice; `Bytes` and `Duration` are type arguments; and a trait C cannot implement is left out
and listed in the census. moq-mux's `codec::annexb::build_prefix` exports once moq stops
re-exporting `bytes` whole, as [moq-pin](/quest/m1/ffi-refusals/moq-pin.md) has it,
since the re-export makes `Bytes` a handle.

Decided 2026-11-11 (maintainer, /quest-unblock): ✅ (a) the exporter gains a foreign future (a
poll function, its data, and `free`, the inverse of the futures it returns) and a kept closure
(a function, its data, and `free`), each wrapped in every language's native future and
function, so moq changes nothing (rejected: (b) moq's signatures change). `repair` then takes
a foreign `Read + Seek`; `panic::message` and `tls::Serve::resolver` go to moq to wrap or
drop. The maintainer dislikes callbacks and closures in an API, so a moq quest for a
closure-free API is planned separately only if a simpler one exists; it does not gate this
quest.
