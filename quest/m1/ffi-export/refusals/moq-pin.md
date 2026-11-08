# [XS] Bump the moq pin past its re-export and reachability fixes

## Goal

The moq pin moves to a revision whose crates re-export no whole crate, moq's own included,
and where every type a facade export names is reachable from the facade's root. `just ffi
census` there lists no whole-crate re-export refusal and no "type not reachable from the
crate root" refusal of moq's own (18 at moq `ffcb7cd9`).

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

Then bump the pin in `ts/fixtures/model/Cargo.toml`, which `ts/justfile`'s `moq_rev` reads,
and report the census before and after.

## Waiting

- moq-dev/moq: [quest/m2/api-shape/reexports.md](https://github.com/moq-dev/moq/blob/main/quest/m2/api-shape/reexports.md),
  planned in [moq-dev/moq#8233](https://github.com/moq-dev/moq/pull/8233), drops every
  whole-crate re-export, moq's own included, and re-exports `Guard` from `container`. Done
  once that quest is gone from moq's `main` and the census at its head lists no whole-crate
  re-export.
- moq-dev/moq: [quest/m2/moq-quic-unused-surface.md](https://github.com/moq-dev/moq/blob/main/quest/m2/moq-quic-unused-surface.md)
  removes `stream::Id::{new, initiator}`. Done once that quest is gone from moq's `main`.
