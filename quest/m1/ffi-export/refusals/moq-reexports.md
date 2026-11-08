# [XS] moq drops its whole-crate re-exports

## Goal

The moq pin moves to a revision whose crates re-export no whole crate in the facade, and
`just ffi census` there lists no whole-crate re-export refusal.

## Plan

Decided with the maintainer on 2026-11-07, from
[moq-dev/rust2#655](https://github.com/moq-dev/rust2/pull/655): ✅ moq drops all ten of its
whole-crate re-exports (moq-net's `bytes` and `kio`; `rustls` in moq-quic, moq-rtmp, and
moq-tokio; moq-tokio's `notify`, `usage_rs`, and `http`; moq-uring's `http`; moq-mux's
`mp4_atom`), since each makes that crate's whole API part of moq's and any bump of it breaks
moq's semver. moq wraps the few third-party types its signatures need instead.

Then bump the pin, as `ts/justfile`'s `moq_rev` sets it, and report the census before and
after.

## Waiting

- moq-dev/moq: [quest/m2/api-shape/reexports.md](https://github.com/moq-dev/moq/blob/main/quest/m2/api-shape/reexports.md),
  planned in [moq-dev/moq#8233](https://github.com/moq-dev/moq/pull/8233), drops them. It
  waits on the maintainer to decide whether it also drops the re-exports the ten leave,
  among them hang's and moq-tokio's `pub use moq_net;`, moq-tokio's `congestion`, and
  `quic::web::proto`, which
  [reexports](/quest/m1/ffi-export/refusals/reexports.md) refuses by form. Done once that
  quest is gone from moq's `main` and the census at its head lists no whole-crate re-export.
