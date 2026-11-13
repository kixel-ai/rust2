# [S] Spike: find pub items nothing uses

## Goal

A report from a throwaway driver pass over this repository and moq: every `pub` item
that nothing uses, that only its own crate uses, or that only tests use, resolved
through macros, re-exports, generics, and `cfg`. It says whether applying every
suggestion as `pub(crate)` or a deletion still checks on all targets, and whether the
pass finds what an agent narrowing items and re-widening what breaks would not.

## Plan

Decided with the maintainer on 2026-11-12, in the quest audit: ✅ a spike, in m3. rustc's
`dead_code` stops at `pub`, and both repositories keep the rule that an item stays private
until a consumer needs it, with nothing to check it.

- Prior art: cargo-workspace-unused-pub covers methods only and works from SCIP, which
  resolves less than the compiler; warnalyzer is unmaintained. Unused dependencies are
  solved upstream, so they are out of scope.
- The driver already resolves every path and the exporter already walks every reachable
  `pub` item; record uses across the workspace's crates and subtract.
- A published crate's `pub` item may be API with no user in the workspace: list those
  apart, by whether the crate is published.
- Go: the workspace still checks after the suggestions, and the pass finds items the
  narrow-and-retry loop misses or gets wrong. No-go: the loop yields the same list; then
  the report says so and no tool is built.

Public API: none.
