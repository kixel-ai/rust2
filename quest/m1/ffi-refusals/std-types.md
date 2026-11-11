# [M] Standard and common types export by role

## Goal

The census at the moq pin refuses no item of moq's own for naming a standard or common
foreign type. Maps, sets, JSON maps, and another crate's errors already export; 8 items
at moq `ffcb7cd9` remain: `hang::catalog::Audio::enabled` and `Video::enabled`, whose
`Filter` runs a closure; `mux::catalog::hang::Extra::iter`, a `serde_json::map::Iter`;
kio's `clock::Instant::from_std` and `into_std`, which take and return `std::time::Instant`;
and moq-tokio's `Log::new`, `Log::level`, and its `level` field, which name `tracing::Level`
and `LevelFilter`.

The rest of the original count lies in moq-tokio outside its session side (`NonZero` in
`quic::driver`, `IpAddr` in `quic::demux`, `UdpSocket` and `TcpListener` in `bind`, and
`io::Error` parameters in `accept` and `notify`), which
[moq-dev/rust2#683](https://github.com/moq-dev/rust2/pull/683) drops.

## Plan

Each decision below is a change in `ffi/src/collect.rs` and `ffi/src/glue.rs`, with the api
fixture and its seven language tests covering it.

Decided 2026-11-11 (maintainer, /quest-unblock):

- ✅ `Filter` collects at the call, for items whose drop runs nothing, reversing the `words`
  fixture's `Split` refusal for the same case (rejected: staying refused).
- ✅ `serde_json`'s map iterators (`Iter`, `IntoIter`, `Keys`, `Values`) collect as core's do
  (rejected: staying refused).
- ✅ `std::time::Instant` exports as an opaque object, compared and copied by its traits with
  no methods of its own (rejected: exporting nothing).
- ✅ `tracing::Level` exports as a plain enum of its five levels, with `LevelFilter` as an
  `Option<Level>` whose `None` is `OFF` (rejected: its text).
