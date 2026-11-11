# [S] Bump the moq pin past its re-export and reachability fixes

## Goal

The moq pin moves to moq's current `main`, whose crates re-export no whole crate, moq's own
included. `just ffi census` there lists no whole-crate re-export refusal. Its "type not
reachable from the crate root" refusals are `kio::Pending` and `lite::version::Version`,
which Waiting covers, and moq-quic's stats and varint types under moq-tokio's `quic::driver`,
which [moq-dev/rust2#683](https://github.com/moq-dev/rust2/pull/683) drops. At `ffcb7cd9` there were 20 such refusals (18 Guard and Side, and
`Version`), and 9 at moq's head on 2026-11-09, measured in
[#676](https://github.com/moq-dev/rust2/pull/676).

## Plan

Decided with the maintainer on 2026-11-07, from
[moq-dev/rust2#655](https://github.com/moq-dev/rust2/pull/655) and
[moq-dev/moq#8233](https://github.com/moq-dev/moq/pull/8233): ✅ moq re-exports no whole
crate, its own included: the ten first listed (moq-net's `bytes` and `kio`; `rustls` in
moq-quic, moq-rtmp, and moq-tokio; moq-tokio's `notify`, `usage_rs`, and `http`;
moq-uring's `http`; moq-mux's `mp4_atom`), hang's and moq-tokio's `pub use moq_net;`,
moq-tokio's `moq_congestion as congestion`, `web_transport_proto as proto` in moq-tokio and
moq-uring, and the rest, such as `axum`, `url`, `object_store`, moq-video's platform crates,
`jni`, and `tikv_jemallocator`. moq-quic-unused-surface names the moq-quic items it needs
instead of adding `pub use moq_quic;`. Each whole-crate re-export makes that crate's whole
API part of moq's, so any breaking release of it breaks moq's semver; moq re-exports by
name only the items its signatures need, or wraps them. Rejected: only the ten; only
third-party crates.

At `ffcb7cd9`, `container::Producer::modify` returns moq-mux's
`container::producer::Guard`, in a private module (16 refusals), and moq-tokio's
`quic::driver` re-exports moq-quic's `stream::Id` but not the `Side` its `new` and
`initiator` take (2). Both are moq's to fix; the stand-in's narrowing to moq-tokio's
session side also drops `quic::driver`.

Decided with the maintainer on 2026-11-07, in the quest audit: one pin bump for both, since
each is an XS bump of the same pin line. Rejected: one bump per moq quest.

Both of moq's fixes merged on 2026-11-09:
[moq-dev/moq#8247](https://github.com/moq-dev/moq/pull/8247) dropped every whole-crate
re-export and re-exports `Guard`, and
[moq-dev/moq#8302](https://github.com/moq-dev/moq/pull/8302) removed
`stream::Id::{new, initiator}`. At moq's head the 9 unreachable items left are
`kio::Pending` (from `dial_tcp` and `dial_track`, reachable before only through
`pub use kio;`), `lite::version::Version`, which was already unreachable at the pin, and
moq-tokio's `quic::driver` items, which the narrowed stand-in drops.

Then bump the pin in `ts/fixtures/model/Cargo.toml`, which `ts/justfile`'s `moq_rev` reads,
and report the census before and after. The last bump
([#390](https://github.com/moq-dev/rust2/pull/390)) also changed `ffi/moq/ffi.toml`, every
`ffi/moq/moq.*`, and rust2-ts's model fixture, bench, and expected output, which share the
pin: run `just ts census` too, and update the count in the
[model epic](/quest/m2/ts-model/README.md).

Decided with the maintainer on 2026-11-09: bump to moq's current main now, and narrow the
Goal to the two moq-owned types. Rejected: wait for moq to fix the rest first. Not landed:
moq's `main` fails `just ts moq` and `just ffi moq-cpp`, so the bump waits on Waiting.

Decided 2026-11-11 (maintainer, /quest-unblock): ✅ (a) rust2 exports a type the crate root does
not reach by its defining path, such as `kio::Pending` and `lite::version::Version`; a type
whose defining path is itself private stays refused (rejected: (b) moq makes both reachable).
The exporter change lands before the bump.

## Waiting

- moq-dev/moq: `rind` builds for wasm32. moq-net depends on `rind`, whose md5 inline asm has
  no wasm32 arm, so `just ts moq <checkout>` fails at moq's `main`. Done when it passes there.
- moq-dev/moq: moq-tokio builds without tripping `recursion_depth_exceeding_limit` (an
  overflow proving `Send` on `Instrumented<...>`), which `just ffi moq-cpp <checkout>` denies.
  Done when it passes at moq's `main`.
