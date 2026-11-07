# moq in pure Go

## Goal

[rust2-go](/go/README.md) translates moq-net to pure Go, with no cgo, close enough to the
exporter's [cgo bindings](/ffi/README.md#go) in speed that Go users can take it
in their place: `go get` installs it and it cross-compiles.

## Plan

Decided with the maintainer on 2026-11-01: Go users avoid cgo, which breaks `go get`'s
plain install and cross-compiling, so a pure-Go moq is worth researching. The cgo
exporter stays moq's Go route until a translation matches it.

Decided with the maintainer on 2026-11-01, settling
[#410](https://github.com/moq-dev/rust2/pull/410):

- **Continue as research,** speed first, then
  [coverage](/quest/m4/go/coverage.md). If speed cannot bring the translated crate
  within 2x of cgo on a realistic whole-crate workload (a group's stream end to end,
  decided on 2026-11-02 in [#428](https://github.com/moq-dev/rust2/pull/428)), delete
  `go/` and this epic, keeping the measurements in the PR that does.
- **Share rust2-ts's front end piece by piece,** as Go needs each piece, starting with
  the control-flow structuring, now `driver/src/cfg.rs`. No shared crate lands ahead
  of a Go use. About a quarter of `ts/src` would serve Go, nearly all of it inside files
  whose lowering returns TypeScript's AST.

Decided with the maintainer on 2026-11-07, settling
[moq-dev/rust2#643](https://github.com/moq-dev/rust2/pull/643):

- **Speed met the bar.** With the translation owning a `Vec<u8>`'s storage and clipping
  it at escape, every `stream/` workload of `just go bench` runs within 2x of cgo, the
  worst at 2.07x CPU time, which counts as noise.
- ✅ **Parked until a consumer needs pure Go.** None has asked, and the cgo exporter
  stays moq's Go route. This epic and its children don't start until one does; each
  child's `Waiting` item holds it. When one does, first weigh running moq compiled to
  WASM inside Go on a pure-Go runtime such as [wazero](https://wazero.io) (no cgo, a
  plain `go get`; unmeasured for moq-net's I/O) against finishing this translation.
  Rejected: splitting [coverage](/quest/m4/go/coverage.md) into a quest per construct
  now; deleting `go/`, whose translator and measurements are kept.

The spike ([moq-dev/rust2#410](https://github.com/moq-dev/rust2/pull/410)) translated
moq's codec held to rust2-ts's bar: 35,273 codec cases and 124,193 integer cases match
native Rust, panics included. It runs 2 to 113 times slower than cgo on batched loops,
and beats it only on a single short call, where cgo's crossing dominates. Stripped, it
adds 324 KB to a Go binary where cgo adds 1.15 MB and a libc dependency.

## Required

- [moq-net beyond the codec](/quest/m4/go/coverage.md) - `Drop`, floats, 128-bit integers, trait objects, and collections, so moq-net translates whole
- [Exported names off the runtime's](/quest/m4/go/runtime-names.md) - a crate's own `Result` or `Formatter` exports beside the runtime's
- [A Go API for exported enums](/quest/m4/go/enums.md) - Go callers in another package build, match, and pass a `pub enum`, pinned by an external test package
- [Trait-impl methods](/quest/m4/go/trait-impls.md) - a `pub` type's `Display`, `Debug`, `From`, and other impls callable from Go by a naming rule
- [Methods through a specialized alias](/quest/m4/go/alias-methods.md) - a private generic type's methods exported through `pub type Byte = Value<u8>` instead of refused
