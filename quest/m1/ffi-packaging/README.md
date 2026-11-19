# Release every language the exporter generates

## Goal

[The binding release builder](/ffi/release/README.md) packages every language moq ships,
so moq's switch to [rust2-ffi](/ffi/README.md) replaces its hand-written packaging rather
than keeping it beside the export. It completes when every child has merged.

## Plan

Found by the 2026-11-18 language audit: the builder ([#852](https://github.com/kixel-ai/rust2/pull/852))
releases Python wheels and a Swift package only. moq hand-maintains the rest: CMake and
pkg-config files for libmoq, an Android library, a Go module with a per-platform `cgo.go`
and `lib/<goos>_<goarch>/libmoq_ffi.a`, and the `moq_ffi` pub.dev package with a
hand-written build hook.

Decided with the maintainer on 2026-11-18: ✅ an epic in m1, one child per registry plus
reproducible builds, beside [the exporter epic](/quest/m1/ffi-export/README.md) and not
holding moq's pin (rejected: m2; only the npm and C gaps).

Each child follows the builder's split: the shared, read-only builder derives everything
from Cargo metadata and the export, and the caller's workflow holds only what Cargo cannot
know, such as who may publish. Each builds, checks in a clean consumer against native
Rust's transcript, and publishes through trusted publishing where the registry has it.
`ffi/release/README.md`'s "Other registries" table loses each row as its child lands; its
npm row ("needs a Node generator first") is stale since the web target landed.
