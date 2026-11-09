# Every public item exports

## Goal

[rust2-ffi](/ffi/README.md) exports every `pub` item of the moq facade it is given, so
`just ffi census` lists no refusals at the moq pin and `ffi.toml` is deleted: the exporter
reads no config, and an item it cannot export fails the export with a span.

What stays here once every child has merged: `ffi/moq/ffi.toml`, the `--config` flag,
`RUST2_FFI_CONFIG`, and `config.rs`'s `Config` and `exclude` are deleted, with the
fixtures' `ffi/fixtures/api/ffi.toml` and `ffi/fixtures/reject/ffi.toml` and the exclude
tests in `ffi/tests/fixtures.rs`; `ffi/README.md` and the recipes stop naming them.
`config.rs`'s `Package` and environment constants stay.

## Plan

Decided with the maintainer on 2026-11-04: configless, and an unsupported public item
fails the export rather than being excluded. ✅ Split: the naming rule first, then this.
Rejected: one quest that waits for the census to reach zero.

Decided with the maintainer on 2026-11-07, from
[moq-dev/rust2#655](https://github.com/moq-dev/rust2/pull/655):

- ✅ A whole-crate re-export (`pub use rustls;`) is refused with one span at its `pub use`,
  and moq drops its re-exports. A whole-crate re-export makes that crate's whole API part
  of moq's, so any breaking release of it breaks moq's semver; moq wraps the few
  third-party types its signatures need instead. Rejected: skipping them silently, as
  rustdoc does; exporting their whole APIs.
- ✅ No exemption: the rule covers every crate, moq's own included (`pub use moq_net;`,
  `moq_congestion as congestion`), so moq drops those too, per
  [moq-dev/moq#8233](https://github.com/moq-dev/moq/pull/8233). Rejected: exempting the
  crates the facade already exports, or moq's own; either needs a list, which is config.
- ✅ The stand-in re-exports only the session side moq-ffi ships from moq-tokio. Rejected:
  all of moq-tokio, whose tokio QUIC driver, CLI helpers, worker groups, and listeners no
  binding calls; the sans-IO driver bindings use instead is
  [session-quic](/quest/m1/ffi-export/session-quic.md). Children leave the moq-tokio items
  outside that session side alone, since narrowing the stand-in drops them.
- ✅ Narrowing the stand-in is the first child, so the others count only what stays
  (decided in the quest audit). Rejected: narrowing last, as this epic's own work.
- ✅ What C can never hold, an object no export returns or lends to a closure, or a generic
  type no export instantiates, is left out of the bindings and kept listed in the census.
  Rejected: refusing them, which would make moq add constructors it has no use for.
- ✅ This epic, with the children below ranked by what moq-ffi's hand-written wrapper still
  covers.

The census at moq `ffcb7cd9` (2026-11-07) refuses 1111 items, which `ffi/moq/ffi.toml`'s
300 excludes hide by naming whole modules. About 790 lie in crates moq re-exports whole.
The other 324 are moq's own, 107 of them in moq-tokio (61 under `quic`); each child gives
its construct's count, and an item counts under every construct it hits. The counts
include the moq-tokio items narrowing drops; each child measures again once it lands. 17 more refuse
only because a type they name is refused, and clear with it.

Public API: the exporter's command line and the C API of every item that exports.

## Required

- [The stand-in exports only moq-tokio's session side](/quest/m1/ffi-export/refusals/narrow-standin.md) - `just ffi standin` names the session items moq-ffi ships in place of `pub use ::moq_tokio::*;`
- [Whole-crate re-exports are refused with one span](/quest/m1/ffi-export/refusals/reexports.md) - the exporter stops at a `pub use` of a crate root, turning about 790 refusals into one per re-export
- [Futures and kept closures as parameters export](/quest/m1/ffi-export/refusals/generics.md) - `clock::timeout`'s future, `clock::drive`'s kept closure, and `mp4::repair`'s seekable reader cross from C
- [Standard and common types export by role](/quest/m1/ffi-export/refusals/std-types.md) - maps, sets, errors, `NonZero`, addresses, instants, sockets, levels, JSON maps, and iterator adapters
- [Poll and Context export](/quest/m1/ffi-export/refusals/poll.md) - a `poll_*` with no async twin takes a waker, as futures' polls do
- [Borrow shapes export](/quest/m1/ffi-export/refusals/borrows.md) - `Option<&T>`, `&mut` buffers, `&Bytes`, `Arc<T>`, `'static` borrows, and borrowed returns that are not `Clone`
- [Small shapes export](/quest/m1/ffi-export/refusals/small-shapes.md) - `u128`, arrays, const generics, unit and `Result` off a return, `!`, empty enums, and what is not `Send`
- [What C can never hold is left out and listed](/quest/m1/ffi-export/refusals/unholdable.md) - objects no export returns or lends to a closure, and generics no export instantiates leave the bindings but stay in the census
- [Bump the moq pin past its re-export and reachability fixes](/quest/m1/ffi-export/refusals/moq-pin.md) - the census has no whole-crate re-export refusal and no unreachable type of moq's own
