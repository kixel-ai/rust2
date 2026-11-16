# [L] Release templates with shared binding package builders

## Goal

Generate release workflow templates backed by shared package builders, using moq's
existing Python and Swift binding releases as the first consumer. A repository can
adopt the generated workflows without changing its stock Rust crates or existing
registry identities, artifact URLs, supported targets, or minimum platform versions.
Clean consumer jobs install the built packages and compare their behavior with
native Rust before any publication.

## Plan

Decided with the maintainer on 2026-11-16 at the packaging spike's merge pick:
✅ keep this follow-up unchanged (rejected: drop it).

Decided with the maintainer on 2026-11-16 in the
[packaging spike](https://github.com/kixel-ai/rust2/pull/761#issuecomment-6930429582):
✅ release templates with shared builders (rejected: keep investigating a
configless publisher; stop the spike with no full version). Registry ownership
and hosting policy are platform boundaries Cargo metadata cannot establish.

- Start from the [spike report](/bench/registry/README.md), then inspect moq's
  `release-py-ffi`, `release-swift-ffi`, and `release-swift-lib` workflows. Preserve
  their current public contracts; the fixture spike did not validate moq packaging.
- Keep registry identities and publishing authorization visible in the generated
  repository-owned workflow. Derive crate facts from Cargo metadata; do not add
  exporter annotations or a separate packaging configuration file.
- Share artifact construction and verification, with immutable builder revisions.
  Reuse crabx's platform matrix where the targets match. Keep publication separate
  from read-only build and consumer jobs, behind the repository's release policy.
- Python: build the supported wheel matrix, inspect and repair native dependencies,
  preserve stable-ABI claims, and validate installation outside the source checkout.
- Swift: package the macOS/iOS device/simulator slices moq actually ships, merge
  target API differences, preserve deployment targets, and generate hosted ZIP
  checksums and the release Package.swift. Validate a clean SwiftPM consumer.
- Exercise the generated workflows on moq's source and compare installed packages
  with native Rust. Replace moq's handwritten Python and Swift packaging logic only
  after those consumers pass; this quest does not authorize a production publish.
- Document how the same builder/template boundary extends to Maven, npm, NuGet,
  pub.dev, and Go, using the report's constraints. Missing binding backends are
  unsupported, never a silently incomplete package. Backend implementation and
  other registry publishers are outside this Python/Swift migration.

## Hosts

- linux - portable Python wheel construction and clean consumers
- macos - Apple slices and SwiftPM consumers

## Related

- [crabx](/quest/m2/crabx/README.md) - reusable build matrices for Rust binaries
