# [XL] Every public item exports

## Goal

[rust2-ffi](/ffi/README.md) exports every `pub` item of moq's crates, so `just ffi census`
is empty at the moq pin and `ffi.toml` is deleted: the exporter reads no config, and an
item it cannot export fails the export with a span.

## Plan

Decided with the maintainer on 2026-11-04: configless, and an unsupported public item
fails the export rather than being excluded. ✅ Split: the naming rule
first, then this. Rejected: one quest that waits for the census to reach zero.

✅ Plan the children later: the first agent runs the census and splits it by refusal kind
into an epic through `/quest-plan`, ranked by what moq-ffi's hand-written wrapper still
covers. This quest's own remaining work, deleting `ffi.toml` and the config reader, stays
in that epic's README.

The census at moq `ffcb7cd9` (2026-11-07) refuses 1111 items; `ffi/moq/ffi.toml`'s 300
excludes hide them by naming whole modules. About 790 lie in other crates that moq's
crates re-export whole: moq-tokio's `rustls` (279), `cli::usage_rs` (341), `notify` (15),
`congestion` (8), and `quic::web`'s `http` (37) and `proto` (33); moq-net's `kio` (29) and
`bytes` (11); and moq-mux's `fmp4::mp4_atom` (34). The other 324 are moq's own, 107 of them
in moq-tokio (61 under `quic`), by construct, each item counted under every construct it
hits:

- 77 a foreign type: `BTreeMap` (20), `Poll` (18), `anyhow::Error` (12), `Context`,
  `NonZero`, `io::Error` (5 each), and `Level`, `Instant`, `UdpSocket`, `IpAddr`,
  `serde_json::Map`, and iterator adapters
- 71 an object no export returns, mostly error types, and 26 a generic type no export
  instantiates, such as moq-net's `transport::poll` futures and moq-mux's `Export`s
- 51 a generic parameter, type argument, trait object, supertrait, associated type, unmet
  bound, callback or closure signature, or blanket impl: `Client::connect` and
  `Server::accept` take `transport::poll::Boxable`
- 3 a future borrowed from a function that is not `async`, or a `Waker` not taken by value
- 18 a type not reachable from its crate's root, moq's to fix: moq-mux's
  `container::producer::Guard` (16) and moq-quic's `Side` (2)
- 17 a refused type, which clears with what refused it
- 41 a borrow shape: a reference inside a parameter, `&mut` of a value, a reference to a
  handle inside a value, `Arc`, a `'static` borrow, a borrowed return that is not `Clone`
- 26 a small shape: `u128`, an array, a const generic, unit outside a return, `Result`
  outside a return, `!`, an enum without variants, an object or future that is not `Send`

## Waiting

- Maintainer: run `/quest-plan` on this quest to split it into an epic. Background agents
  cannot run its interview. The proposed split, its ranking, and three decisions it needs
  are in [moq-dev/rust2#655](https://github.com/moq-dev/rust2/pull/655): whether a
  re-export of a whole crate is API, whether the stand-in re-exports all of moq-tokio, and
  whether what C can never hold is refused or left out. Done once this file is an epic.

Public API: the exporter's command line and the C API of every item that exports.
