# [XS] The stand-in exports only moq-tokio's session side

## Goal

The stand-in facade (`just ffi standin`, in `ffi/justfile`) re-exports only the session
side moq-ffi ships from moq-tokio: `Client`, `Server`, `server::Request`,
`connect::Config`, `listen::Config`, `quic::Config`, `Connection`, `Transport`, `Log`,
`tls`, `panic`, and the errors, in place of `pub use ::moq_tokio::*;`. `ffi/moq/ffi.toml`
loses the excludes that only hid items outside it, and the census at the pin reports the
refusals before and after, per crate.

## Plan

Decided with the maintainer on 2026-11-07, from
[moq-dev/rust2#655](https://github.com/moq-dev/rust2/pull/655): ✅ the stand-in re-exports
only that session side. Rejected: all of moq-tokio, whose tokio QUIC driver, CLI helpers,
worker groups, and listeners no binding calls; bindings drive the sans-IO driver of
[session-quic](/quest/m1/ffi-export/session-quic.md) instead.

Decided with the maintainer on 2026-11-07, in the quest audit: ✅ this lands first, so the
other children count only what stays. Narrowing drops moq-tokio's `rustls`, `notify`,
`congestion`, `cli::usage_rs`, and `quic::web`'s `http` and `proto` re-exports, and
`quic::driver`'s `Side` refusals. Every item named exists at moq `ffcb7cd9`
(`rs/moq-tokio/src/lib.rs`). Update the counts in the
[epic](/quest/m1/ffi-export/refusals/README.md) and its children from the new census.

Public API: the stand-in's exports, which the census and the bindings' fixtures read.
