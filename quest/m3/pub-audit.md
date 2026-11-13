# [L] Audit public visibility without changing resolution

## Goal

A compiler-resolved public-visibility audit for rust2 and moq that reports unreached,
crate-only and test-only items with their source spans and evidence. Its proposed
`pub(crate)` changes preserve original resolved references and pass the complete
selected Cargo checks together in a disposable source snapshot. It leaves the real
source unchanged and separates potentially external API from workspace findings.

## Plan

Recommended by the [completed spike](https://github.com/kixel-ai/rust2/pull/763),
2026-11-13, while the maintainer was away, for review:

- ✅ Go for a hybrid of compiler-resolved constraints and collective compiler
  validation. The graph's rust2 set checks unchanged; its moq set needs compiler
  restoration of publicity obligations. The independent compiler loop restores
  additional valid candidates conservatively and can silently change dispatch:
  hiding an inherent `len` redirects to a slice through `Deref`, changing the native
  result from 99 to 3. The graph retains that original method. Neither raw graph
  output nor successful compilation alone is a safe proposal.
- ✅ Reuse the shared driver front end and its pinned nightly, with a `rust2-pub`
  binary in the driver workspace. rust2 and moq are the first consumers. Keep it
  configless and accept ordinary Cargo target/feature selection, without source
  annotations or project-specific exceptions.
- ✅ Capture local path dependencies as well as primary workspace packages. Join
  compiler identities across invocations, retain configuration identity and macro
  provenance, and compute production/test reachability from actual entrypoints and
  compiler-required roots. Include impl obligations, generic/dynamic dispatch,
  implicit drop, signatures, associated types, constructors and re-export chains.
- ✅ Preserve the originally resolved cross-crate references, including method
  selection through `Deref`. Compile the entire proposed set in the same Cargo
  scope before reporting it as validated; resolve publicity and re-export errors
  through rustc rather than duplicating its privacy rules incompletely. An
  unsupported or unverifiable construct fails with its source span.
- ✅ Start with narrowing, retaining bodies. Default to a read-only report and
  validated patch evidence, never editing the consumer's real checkout. Publication
  policy and source fixtures are potential API boundaries: neither workspace
  reachability nor a passing compiler check proves those consumers absent. Name
  unchecked configurations and do not claim universal deadness.
- ✅ Differential native fixtures cover generic and dyn trait calls, required impl
  declarations, drop, dead recursive components, test-only roots, macro expansions,
  re-exports, associated-type publicity and method lookup before/after narrowing.
  In the `Deref` case, retain the method and the original result of 99. Repeat the
  collective rust2/moq experiment inside each repository's development environment;
  moq's own Nix shell supplies its native pkg-config dependencies.

Public API and wire: no changes to audited crates. The new audit command is the
planned user-facing API; this quest does not authorize applying its findings to
published APIs or translator fixtures.
