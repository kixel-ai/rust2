# [S] Whole-crate re-exports are refused with one span

## Goal

The exporter refuses a `pub use` of a crate's root, as in `pub use rustls;` or
`pub use web_transport_proto as proto;`, with one refusal whose span is that `pub use`, and
does not walk into the crate. The census at moq `ffcb7cd9` then lists one refusal per
re-export in the facade's crates instead of about 790 for the items behind them: moq-net's
`bytes` and `kio`, moq-mux's `container::fmp4::mp4_atom`, and moq-tokio's `rustls`,
`notify`, `congestion`, `cli::usage_rs`, and `quic::web`'s `http` and `proto`.

## Plan

Decided with the maintainer on 2026-11-07, from
[moq-dev/rust2#655](https://github.com/moq-dev/rust2/pull/655): ✅ refuse each with one span
at its `pub use`, and moq drops them, since a whole-crate re-export makes that crate's
whole API part of moq's and any breaking release of it breaks moq's semver. Rejected:
skipping them silently, as rustdoc does; exporting their whole APIs.

The rule goes by form, with no list and no exception for moq's own crates: hang's and
moq-tokio's `pub use moq_net;` refuse too, although the census folds them into the root's
moq-net today. A facade's own named re-exports from a crate, such as the stand-in's
`pub use kio::{Park, Waiter, ...};`, are not whole-crate and export as now. So does a glob
of a crate's root, such as the stand-in's `pub use moq_net::*;`: it re-exports that crate's
items, not the crate as a module (decided with the maintainer on 2026-11-07, in the quest
audit: "we probably need to support re-exports", while moq re-exports only the items it
needs, not all of `rustls`). [moq-pin](/quest/m1/ffi-export/refusals/moq-pin.md) waits on
moq to drop the whole-crate ones.

Document the rule in `ffi/README.md` beside the other refusals, and test it on a fixture
that re-exports a crate whole, by rename, and by a glob of its root, the last exporting its
items.
