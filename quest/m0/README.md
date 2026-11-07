# m0: immediate priorities

## Goal

Urgent fixes to anything a consumer already depends on.

## Required

- [Leave out a broken Swift on aarch64](/quest/m0/swift-guard.md) - the default dev shell evaluates on aarch64-linux, so the nightly's aarch64 jobs pass
- [Checks at moq's cost](/quest/m0/checks/README.md) - `just ci` selects by the diff, builds warm, and runs the differential optimized, with a nightly full `just gate`
