# [M] Language names keep module paths apart

## Goal

Every [rust2-ffi](/ffi/README.md) generator gives two items with distinct C names distinct
names of its own, so a facade never re-exports an item under a second name to dodge a
clash, and `just ffi standin` drops `ConnectTargetError`.

## Plan

C keeps each path segment's case, but Kotlin, Swift, Python, Go, and Dart join a type's
segments in Pascal case, which loses where they split: moq-tokio's `ConnectError` and
`connect::Error` are both `TokioConnectError`, so the stand-in re-exports the second as
`ConnectTargetError`.

Decided with the maintainer on 2026-11-07, and built by
[moq-dev/rust2#599](https://github.com/moq-dev/rust2/pull/599): Go has ✅ one package per
Rust crate (rejected: one package for the whole export; alias packages per module; one
package per import cycle; one package per module, which Go's ban on import cycles rules
out, since moq's modules name each other's types in three cycles). Crates cannot form a
cycle, so callers write `mux.CodecOpusConfig_`, as uniffi-bindgen-go lays out crates.
Within a crate, names stay flat with the `New` and trailing `_` rule, so the clash above
remains in Go's `tokio` package (`tokio.ConnectError` twice): this quest settles how Go
keeps those two apart, or whether Go alone fails the export on them.

Each other generator either nests a facade module as its language's namespace (Kotlin
and Swift nested objects, Python submodules, Dart prefixed imports) or fails the export
at both spans, as C does on a residual clash. Measure which moq would hit before
choosing.

Public API: the generated names of items in a facade module, in every language but C and
C++.
