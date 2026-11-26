# [M] Diff-scoped benchmark selection and Callgrind footprint ordering

## Goal

`swole` supports diff-scoped runs (`--changed <base>`) that conservatively select benchmarks from the Cargo packages touched by `git diff <base>` and their reverse dependencies (plus any newly added or modified benchmark targets), and uses cached Callgrind file/function footprints from `swole sim` to order those selected benchmarks so directly exercised benchmarks run and report first—cutting local compare and bisect turnaround on large workspaces (such as `moq`'s 1,284 benchmarks) without skipping benchmarks affected through untimed setup code.

## Plan

Decided 2026-11-25 (maintainer, `/quest-plan` in rust2):

- ✅ Include diff-scoped benchmark selection in the `swole` epic to cut agent and developer iteration time on large workspaces like `moq`.

Recommendations for the implementer:

- Compute affected benchmark targets conservatively from `cargo metadata`'s workspace dependency graph: any package containing a file changed by `git diff <base>`, plus all workspace reverse dependencies, plus the targets of untracked files (`git ls-files --others --exclude-standard`), since `git diff <base>` omits them and Cargo auto-discovers a new `benches/*.rs`. Fall back to the full workspace whenever any changed file could affect benchmarks and cannot be proven confined to a known package's build inputs—including files outside member directories (such as a workspace-root module loaded via `#[path]` or shared fixture files), root `Cargo.toml`, `Cargo.lock`, `build.rs`, or proc-macro crates. Because untimed setup runs before Callgrind instrumentation starts (e.g. `moq`'s `producer_tick` setup helpers), Callgrind footprints alone cannot safely exclude a benchmark in an affected package.
- Reuse the Callgrind parser from [per-function attribution](/quest/m2/swole/callgrind-attribution.md) to cache each benchmark's executed source files (`fl=`) and demangled function paths (`fn=`) under `~/.cache/swole/` keyed by repository and commit/tree, and prioritize benchmarks whose cached footprints directly intersect the diff to run first within the selected set.
- Print how many benchmarks were selected vs skipped by package graph, and how many directly matched Callgrind footprints, before running.
- Prove soundness with a fixture workspace containing two independent crates plus a shared crate and an untimed setup helper: editing an independent crate runs only its benchmarks, editing a shared crate selects its reverse dependents (ordered with footprint hits first), and editing a helper used only in untimed setup still runs the benchmark that calls it in setup.

## Required

- [Per-function attribution from Callgrind dumps in every sim compare](/quest/m2/swole/callgrind-attribution.md) - reuses the per-function and per-file Callgrind parser
