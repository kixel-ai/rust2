# [S] First publish of swole, swole-macros, and cargo-swole to crates.io

## Goal

`swole-macros`, `swole`, and `cargo-swole` are published to crates.io at `0.1.0` from `kixel-ai/swole` with GitHub Actions trusted publishing and a tagged release workflow (and prebuilt CLI binaries via GitHub Releases / `crabx`), so any Rust workspace can add `swole` to `[dev-dependencies]` and install `cargo-swole` directly.

## Plan

Decided 2026-11-25 (maintainer, `/quest-plan` in rust2):

- ✅ Publish `swole-macros`, `swole`, and `cargo-swole` to crates.io under `MIT OR Apache-2.0` once the repo move and unified CLI/harness API settle (rejected: git/Nix pin only with no crates.io publish; single combined crate).

Recommendations for the implementer:

- Add a `release.yml` workflow in `kixel-ai/swole` (mirroring `kixel-ai/crabx`) with a `crates-io` environment restricted to `main`/release tags, publishing in dependency order (`swole-macros`, then `swole`, then `cargo-swole`), and verify `cargo publish --dry-run --locked` passes for all three crates before handing the initial publish to the maintainer.
- Verify package metadata (`description`, `license = "MIT OR Apache-2.0"`, `repository = "https://github.com/kixel-ai/swole"`, `readme`, `keywords`, `categories`) and ensure `swole`'s dependency tree stays minimal.
- Once published, file a follow-up quest for the maintainer to try and announce `swole`.

## Waiting

- Maintainer: run the first `cargo publish --locked` for `swole-macros`, `swole`, and `cargo-swole` from `kixel-ai/swole` once the crates exist there and this quest's `Required` items clear (crates.io requires the initial publish from a human token before Trusted Publishing can be enabled; an agent cannot access the maintainer's crates.io token). Check `https://crates.io/crates/swole` and `https://crates.io/crates/cargo-swole`.
- Maintainer: on crates.io, enable Trusted Publishing for `swole-macros`, `swole`, and `cargo-swole` pointing at `kixel-ai/swole`'s `release.yml` workflow and `crates-io` environment, then turn on "Require trusted publishing" after the first automated release (requires the crate owner's crates.io web session).

## Required

- [Three modes (quick, sim, bench), capability probe, CAP_PERFMON, L2/L1 perf, and OS rusage](/quest/m2/swole/modes-and-counters.md) - settles the initial public CLI and harness surface before 0.1.0
