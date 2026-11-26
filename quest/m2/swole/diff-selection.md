# [M] Diff-scoped benchmark selection from Callgrind function footprints

## Goal

`swole` caches each benchmark's executed file and function footprint from `swole sim` Callgrind dumps and supports diff-scoped runs (`--changed <base>` or automatic selection on local compare when requested) that run only the benchmarks whose footprints intersect files or functions touched by `git diff <base>` (plus any newly added or modified benchmark targets), shrinking local compare and bisect runs from full suites (e.g. `moq`'s 1,284 benchmarks) to the touched subset.

## Plan

Decided 2026-11-25 (maintainer, `/quest-plan` in rust2):

- ✅ Include diff-scoped benchmark selection in the `swole` epic to cut agent and developer iteration time on large workspaces like `moq`.

Recommendations for the implementer:

- Reuse the Callgrind parser from [per-function attribution](/quest/m2/swole/callgrind-attribution.md) to record each benchmark's executed source files (`fl=`) and demangled function paths (`fn=`) in a compact cache under `~/.cache/swole/` keyed by repository and commit/tree.
- Map `git diff <base>` changed source files (and conservatively fall back to full runs when `Cargo.toml`, `Cargo.lock`, `build.rs`, or proc-macro crates change, or when no cached footprint exists for the base commit — never silently skip benchmarks on a stale or missing map).
- Always include any benchmark whose own file or `#[bench]` target was added or edited, and print how many benchmarks were selected vs skipped (and why) before running.
- Prove soundness with a fixture workspace containing two independent crates/modules: editing module A selects only module A's benchmarks, editing a shared helper selects both, and editing `Cargo.toml` selects all.

## Required

- [Per-function attribution from Callgrind dumps in every sim compare](/quest/m2/swole/callgrind-attribution.md) - reuses the per-function and per-file Callgrind parser
