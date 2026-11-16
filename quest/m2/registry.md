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
the reusable invocation has run successfully. The Linux x86_64 wheel
consumer passed on CPython 3.14.10, including the native transcript and futures,
ownership, values and calls. On 2026-11-16, kixtop built the macOS arm64
XCFramework with Xcode 26.6, and a clean SwiftPM consumer with Apple Swift 6.3.3
matched the native transcript byte for byte and passed futures and ownership.
The [report](/bench/registry/README.md) records the commands and the fixes
this execution required. GitHub Actions run
[43968322384](https://github.com/kixel-ai/rust2/actions/runs/43968322384)
built both packages and passed the clean Linux consumer. The Mac wheel import
failed on a Nix-store libiconv dependency. The builder now selects Apple's linker,
and wheel packaging rejects non-system dylib dependencies; rerun both consumers
to validate the fix.
The maintainer accepted release templates with shared builders because registry
namespace ownership cannot be inferred from Cargo metadata; it does not claim moq's workflows can be deleted.

Remaining: exercise the reusable call, record the actual results, finish the report and delete this quest. Do not publish
the fixture or infer successful packaging from lint alone.

Decided with the maintainer on 2026-11-16 ([decision](https://github.com/kixel-ai/rust2/pull/761#issuecomment-6930429582)):
✅ accept the no-go and build release templates with shared builders (rejected:
keep investigating a configless publisher; stop the spike with no full version).
Cargo metadata cannot establish registry identities or hosting policy. The full
version is [release templates](/quest/m2/release-templates.md); it is outside this
fixture-packaging spike.

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
