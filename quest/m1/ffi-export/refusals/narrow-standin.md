# [XS] The stand-in exports only moq-tokio's session side

## Goal

The stand-in facade (`just ffi standin`, in `ffi/justfile`) re-exports only the session
side moq-ffi ships from moq-tokio, in place of `pub use ::moq_tokio::*;`: `Client`,
`Server`, `Connection`, `Transport`, `Log`, `tls`, `panic`, the errors, and the types
those items and the fixtures name: `client::Config`, `connect::Config`, `listen::Config`
with `listen::Bind`, `quic::Config`, `server::Config` with `server::Request` and
`server::Listener`, `origin::spawn`, `Backoff`, `Restart`, `Status`, `connection::Monitor`,
and `cli::Duration`. `ffi/moq/ffi.toml`
loses the excludes that only hid items outside it, and the census at the pin reports the
refusals before and after, per crate.

## Plan

Decided with the maintainer on 2026-11-07, from
[moq-dev/rust2#655](https://github.com/moq-dev/rust2/pull/655): ✅ the stand-in re-exports
only that session side. Rejected: all of moq-tokio, whose tokio QUIC driver, CLI helpers,
worker groups, and listeners no binding calls; bindings drive the sans-IO driver of
[session-quic](/quest/m3/session-quic.md) instead.

Decided with the maintainer on 2026-11-09, in the quest audit: ✅ no landing order. Each
sibling leaves the moq-tokio items outside the session side alone, and whichever PR lands
later measures the census again. Rejected: this landing first, as decided on 2026-11-07,
which no `Required` link held and which two siblings ready for review had already passed.

The list above was not closed over what the fixtures and moq-ffi call. Its closure was
decided on 2026-11-09 in the iterate of
[#683](https://github.com/moq-dev/rust2/pull/683), which found `just ffi moq` fails
without it: `ffi/moq/moq.*` use about 40 moq-tokio names the list left out.

Narrowing drops moq-tokio's `rustls`, `notify`,
`congestion`, `cli::usage_rs`, and `quic::web`'s `http` and `proto` re-exports, and
`quic::driver`'s `Side` refusals. Every item named exists at moq `ffcb7cd9`
(`rs/moq-tokio/src/lib.rs`). Update the counts in the
[epic](/quest/m1/ffi-export/refusals/README.md) and its children from the new census.

Decided with the maintainer on 2026-11-09, in the iterate of
[moq-dev/rust2#683](https://github.com/moq-dev/rust2/pull/683):

- ✅ The closure over the session side adds the types the session items and the fixtures
  name, and keeps the CLI helpers and listener internals out, so `Client::new`,
  `Server::new`, and the like export again (rejected: rewriting the seven fixtures to the
  narrow list).
- ✅ No rename of the three `Config`s: Rust keeps `connect::Config`, `listen::Config`, and
  `quic::Config`, and C keeps `moq_tokio_connect_Config_*`. Go, which clashes, keeps its
  module segment for a name still shared, so they are `tokio.ConnectConfig`,
  `tokio.ListenConfig`, and `tokio.QuicConfig`, the names its fixture already uses. Other
  languages keep their names (rejected: renaming to `ConnectConfig`, `ListenConfig`, and
  `QuicConfig` in the stand-in; re-exporting whole modules).

The census at this change refuses 337 items, 51 of them in moq-tokio, and 104 types, down
from 1111 items, 84 in moq-tokio, and 321 types.

Public API: the stand-in's exports, which the census and the bindings' fixtures read.

