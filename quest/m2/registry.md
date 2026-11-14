# [M] Spike: publish bindings to every registry from one workflow

## Goal

A report, from a working spike: one configless reusable workflow turns
[rust2-ffi](/ffi/README.md)'s export of the api fixture into Python wheels and an
XCFramework with a `Package.swift`, and a clean consumer job installs each and runs the
fixture's script. It says what the full version would take for the other registries
(an AAR on Maven, npm, NuGet, pub.dev, and a Go module), and whether any needs settings
`Cargo.toml` cannot supply.

## Plan

Implementation in progress: [packaging spike](/bench/registry/README.md) and
`.github/workflows/registry.yml` build the api fixture into native wheels and a
host-only macOS XCFramework with a Swift source package. Keep this quest until
the macOS SwiftPM consumer has run successfully. The Linux x86_64 wheel
consumer passed on CPython 3.14.10, including the native transcript and futures,
ownership, values and calls. The Linux host cannot execute xcodebuild, and kixtop was
unreachable on 2026-11-13. The workflow has not been executed in this session.
The report recommends templates because registry namespace ownership cannot be
inferred from Cargo metadata; it does not claim moq's workflows can be deleted.

Remaining: exercise the reusable call and macOS clean artifact consumer, record
the actual results, then finish the report and delete this quest. Do not publish
the fixture or infer successful packaging from lint alone.

Decided with the maintainer on 2026-11-12, in the quest audit: ✅ a spike, in m2, with moq
as its consumer. moq carries about 13 `release-*-ffi` and `release-*-lib`
[workflows](https://github.com/moq-dev/moq/tree/main/.github/workflows) to do this by
hand, and the audit's survey found no tool that publishes one Rust library to all of
these registries.

- Prior art, and why it falls short: [BoltFFI](https://github.com/boltffi/boltffi)'s
  `pack all` is closest (XCFramework, wheels, npm, NuGet) but needs annotations, builds
  local artifacts only, and emits jniLibs rather than an AAR;
  [maturin](https://www.maturin.rs/distribution.html) covers PyPI only; cargo-swift,
  cargo-xcframework, cargo-ndk, and napi-rs each cover one ecosystem.
- It shares [crabx](/quest/m2/crabx/README.md)'s shape: a workflow a repository opts
  into, building each target in its own CI. Reuse its target matrix where the targets
  match.
- Go: both packages install and run with nothing in the consumer's repository but the
  workflow call, and moq's Python and Swift release workflows would become deletable.
  No-go: a registry needs per-project settings, such as signing identities or a
  namespace, that no manifest carries; then the report recommends templates instead.
- The spike publishes nothing: it builds the packages and installs them from the job's
  artifacts.

Public API: none; the report proposes one.

## Related

- [crabx](/quest/m2/crabx/README.md) - the same opt-in workflow, for binaries
