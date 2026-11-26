# [L] Move rust2-bench to its own repository, kixel-ai/swole

## Goal

`rust2-bench` and `rust2-bench-harness` move out of `rust2` into a new public `MIT OR Apache-2.0` repository, `kixel-ai/swole`, with filtered history from `rust2`, split into `swole` (tiny `[dev-dependencies]` harness library re-exporting `#[swole::bench]`), `swole-macros` (proc-macro crate), and `cargo-swole` (CLI runner shipping both `swole` and `cargo-swole` binaries). `rust2` removes `bench/runner/`, retargets its own workflows and `ffi` benchmark to `kixel-ai/swole`, and carries [the swole epic](/quest/m2/swole/README.md) and its deferred `m3` quests into `kixel-ai/swole`'s quest tree so `moq` and `rind` rewrite their Criterion benches straight to `swole` once.

## Plan

Decided 2026-11-25 (maintainer, `/quest-plan` in rust2):

- ✅ Name: `swole` (`swole`, `swole-macros`, and `cargo-swole` verified free on crates.io 2026-11-25) (rejected: `gains`, `barpath`, `tonnage`, `deadlift`, keeping `rust2-bench`).
- ✅ Split into a new public `MIT OR Apache-2.0` repository `kixel-ai/swole` right after `#1032` (`--allow-missing`) and `#1033` (paired timing) merge in `rust2`, mirroring `crabx`'s filtered-history move ([kixel-ai/rust2#985](https://github.com/kixel-ai/rust2/pull/985)) (rejected: keeping in `rust2` and renaming before or after moq/rind rewrites; creating from `main` before `#1032`/`#1033` and porting both PRs; `GPL-3.0-only`, which adds friction for crates putting `swole` in `[dev-dependencies]`).
- ✅ Crate split: `swole` (tiny harness library + `swole-macros`) and `cargo-swole` (CLI runner shipping `swole` and `cargo-swole` binaries, plus `nix run github:kixel-ai/swole`) (rejected: single `swole` crate combining harness and CLI, which pulls runner dependencies into every consumer's dev build).
- m1: moving and renaming before `moq` and `rind` rewrite ~117 bench files to `#[bench]` avoids touching every bench file in both repositories twice.

Recommendations for the implementer (following [kixel-ai/rust2#974](https://github.com/kixel-ai/rust2/commit/67dc8ee05a)):

- Filter history with `git filter-repo` over `bench/runner/` and its quest files, renaming `bench/runner/` to the new repository's root layout (`Cargo.toml` workspace with `crates/swole`, `crates/swole-macros`, `crates/cargo-swole`, or root + `harness/`). Rewrite bare `#N` references in commit messages to `kixel-ai/rust2#N`.
- Copy `rust2`'s reusable setup trimmed to what `swole` needs: Nix flake (exposing `swole` / `cargo-swole` with pinned Valgrind on Linux, all four systems), direnv, `just ci`, `AGENTS.md`, `CONTRIBUTING.md`, `quest init`, and org rulesets via dotfiles' `repo-settings`.
- Rename `RUST2_BENCH*` env vars and protocol prefixes (`rust2-bench ran`, `rust2-bench counted`, `~/.cache/rust2-bench`, `target/rust2-bench`) to `SWOLE_*` / `swole` while keeping CodSpeed compatibility intact until [the CodSpeed epic](/quest/m1/codspeed/README.md) retires it.
- Carry [the swole epic](/quest/m2/swole/README.md) and its `m3` siblings ([scaling](/quest/m3/swole-scaling.md), [noise floors](/quest/m3/swole-noise-floors.md), [faster simulation spike](/quest/m3/swole-fast-sim.md)) into `kixel-ai/swole`'s quest tree, dropping their `Required` link to this quest.
- In `rust2`: delete `bench/runner/`, point `.github/workflows/valgrind.yml` and `ffi/justfile` at `kixel-ai/swole`'s flake input, update `AGENTS.md`, `README.md`, `quest/README.md`, and [the CodSpeed epic](/quest/m1/codspeed/README.md) with a pointer to `kixel-ai/swole`.
- Add follow-up notes for `moq`'s and `rind`'s bench-rewrite quests so they depend on `swole` from `kixel-ai/swole` instead of `rust2-bench-harness`.

## Required

- [A bisect step counts the named benchmarks a commit has](/quest/m0/bench-missing-names.md) - lands `--allow-missing` before filtering history into `kixel-ai/swole`
- [Paired base-vs-head wall-clock timing](/quest/m1/codspeed/paired.md) - lands paired timing and `#[bench]` updates before filtering history into `kixel-ai/swole`
