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

Each item below waits on a maintainer decision; once decided, each is a change in
`ffi/src/collect.rs` and `ffi/src/glue.rs`, with the api fixture and its seven language
tests covering it.

## Waiting

- Maintainer: `Filter` collects at the call or stays refused. Collecting runs the predicate
  on every item at the call, which a Rust caller that stops early would not run on the
  rest; `ffi/fixtures/reject`'s `words` refuses a `Split` by a closure for that reason, so
  collecting reverses it for both. The predicate is moq's own, pure, and a C caller can
  only take every item, as a Rust caller that collects does. Recommended: collect, for
  items whose drop runs nothing, so a refused item's drop never moves to the call.
- Maintainer: `serde_json`'s map iterators (`Iter`, `IntoIter`, `Keys`, `Values`) collect
  as core's do, or stay refused. The exporter already trusts serde_json for `Json`, and
  they only walk the map. Recommended: collect them.
- Maintainer: `std::time::Instant` exports as an opaque object, compared and copied by its
  traits with no methods of its own, or as nothing. It has no portable form C could read,
  and `into_std` and `from_std` only move it between kio and Rust. Recommended: the opaque
  object, adding no API std lacks.
- Maintainer: `tracing::Level` exports as a plain enum of its five levels, with
  `LevelFilter` as an `Option<Level>` whose `None` is `OFF`, or as its text, which a parse
  could fail on. Recommended: the enum, which no input can make invalid.
