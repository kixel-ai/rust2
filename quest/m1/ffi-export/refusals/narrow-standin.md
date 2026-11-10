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
[session-quic](/quest/m3/session-quic.md) instead.

Decided with the maintainer on 2026-11-09, in the quest audit: ✅ no landing order. Each
sibling leaves the moq-tokio items outside the session side alone, and whichever PR lands
later measures the census again. Rejected: this landing first, as decided on 2026-11-07,
which no `Required` link held and which two siblings ready for review had already passed.

The list above is not closed over what the fixtures and moq-ffi call:
[#683](https://github.com/moq-dev/rust2/pull/683) found `just ffi moq` fails with it, since
`ffi/moq/moq.*` use about 40 moq-tokio names it leaves out, such as `client::Config`,
`server::Config`, `origin::spawn`, `Backoff`, and `listen::Bind`.

Narrowing drops moq-tokio's `rustls`, `notify`,
`congestion`, `cli::usage_rs`, and `quic::web`'s `http` and `proto` re-exports, and
`quic::driver`'s `Side` refusals. Every item named exists at moq `ffcb7cd9`
(`rs/moq-tokio/src/lib.rs`). Update the counts in the
[epic](/quest/m1/ffi-export/refusals/README.md) and its children from the new census.

Public API: the stand-in's exports, which the census and the bindings' fixtures read.

## Waiting

- Maintainer: how the stand-in closes over what its session items name, from
  [#683](https://github.com/moq-dev/rust2/pull/683), since the Goal's list alone fails
  `just ffi moq`. Options: add those types and `origin::spawn` to the list, keeping the CLI
  helpers and listener types out (recommended: a stand-in that cannot construct a client
  is not the session side); or keep the list and rewrite the seven fixtures to it, losing
  their coverage of client config, server config, and origin spawn. An agent cannot pick:
  it changes which of moq-tokio's API bindings get.
