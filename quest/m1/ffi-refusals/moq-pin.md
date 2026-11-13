# [S] Bump the moq pin past its re-export and reachability fixes

## Goal

The moq pin moves to moq's current `main`, whose crates re-export no whole crate, moq's own
included. `just ffi census` there lists no whole-crate re-export refusal. Its "type not
reachable from the crate root" refusals were `kio::Pending` and moq-tokio's `cli::Deprecated`,
which the exporter change below exports, `lite::version::Version`, which no public path names, and moq-quic's stats and varint types under moq-tokio's `quic::driver`,
which [kixel-ai/rust2#683](https://github.com/kixel-ai/rust2/pull/683) drops. At `ffcb7cd9` there were 20 such refusals (18 Guard and Side, and
`Version`), and 9 at moq's head on 2026-11-09, measured in
[#676](https://github.com/kixel-ai/rust2/pull/676).

## Plan

Decided with the maintainer on 2026-11-07, from
[kixel-ai/rust2#655](https://github.com/kixel-ai/rust2/pull/655) and
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
([#390](https://github.com/kixel-ai/rust2/pull/390)) also changed `ffi/moq/ffi.toml`, every
`ffi/moq/moq.*`, and rust2-ts's model fixture, bench, and expected output, which share the
pin: run `just ts census` too, and update the count in the
[model epic](/quest/m2/ts-model/README.md).

Decided with the maintainer on 2026-11-09: bump to moq's current main now, and narrow the
Goal to the two moq-owned types. Rejected: wait for moq to fix the rest first. Not landed:
moq's `main` fails `just ts moq` and `just ffi moq-cpp`, so the bump waits on Waiting.

Decided 2026-11-11 (maintainer, /quest-unblock): ✅ (a) rust2 exports a type the crate root does
not reach by its defining path, such as `kio::Pending` and `lite::version::Version`; a type
whose defining path is itself private stays refused (rejected: (b) moq makes both reachable).
[kixel-ai/rust2#721](https://github.com/kixel-ai/rust2/pull/721) made the exporter change: a
type a signature names that the root does not reach exports by its shortest public path in
its own crate, so `kio::Pending` (defined in kio's private `pollable`) and moq-tokio's
`cli::Deprecated` export, the glue naming them through its own dependency on their crate's
package. At moq `ba47ea3` the census, which ignores `ffi/moq/ffi.toml`'s excludes, then lists
no such refusal; only `lite::version::Version`, which no public path names, stays out, as an
object no export returns. The bump drops the stale `kio` exclude, which would otherwise leave
`kio::Pending` out of the export.

Measured on 2026-11-12 at moq's `main` `6936f0a`: `just ffi moq-cpp` fails the export with
36 errors, which the bump fixes in `ffi/moq/ffi.toml` or the exporter: 15 on the foreign
type `std::num::NonZero` (such as `Client::with_peer_hop`), 11 stale excludes naming nothing
exported (such as `bytes` and `kio`), and single refusals such as `qmux::Session`,
`CertificateDer`, a borrowing future from the non-async `Server::accept`, and borrowed
returns that are not `Clone`.

## Waiting

- moq-dev/moq: `rind` builds for wasm32. moq-net depends on `rind`, whose md5 inline asm has
  no wasm32 arm, so `just ts moq <checkout>` fails at moq's `main`. Done when it passes there.
  The fix merged in rind's `main` ([moq-dev/rind#543](https://github.com/moq-dev/rind/pull/543))
  and ships in rind 0.4.0. Its release PR
  ([moq-dev/rind#511](https://github.com/moq-dev/rind/pull/511)) stays unmerged on purpose,
  as rind's `quest/m2/api/release-0.4.md` decides, until the CPU-check removal, the benchmark
  refresh, and moq's move are ready (reconfirmed by the maintainer on 2026-11-12); then
  [moq-dev/moq#8432](https://github.com/moq-dev/moq/pull/8432) moves moq to rind 0.4. Check:
  `cargo search rind` shows 0.4.0. On 2026-11-12 moq `c681b7a` still locks rind 0.3.1, and
  rind 0.3.2, tagged 2026-11-08, predates the fix.
- moq-dev/moq: moq-tokio builds without tripping `recursion_depth_exceeding_limit` (an
  overflow proving `Send` on `Instrumented<...>`), which `just ffi moq-cpp <checkout>` denies.
  Done when it passes at moq's `main`. Still failing on 2026-11-12 at moq `c681b7a`, at
  `rs/moq-tokio/src/client.rs:748`, on the `Instrumented<...>` from
  `tokio::spawn(panic::isolate(time::run(driver)).instrument(..))`, and in the moq-ffi facade
  on `moq_net::Driver: Sync`.

## Required

- [Own pin](/quest/m1/ffi-refusals/own-pin.md) - the exporter's pin moves without rust2-ts's, which ends the wait on `rind` for wasm32
- [Every refusal at moq's main](/quest/m1/ffi-refusals/moq-main.md) - the rest of them, so the bump adds no exclude
