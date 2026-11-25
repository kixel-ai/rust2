# [M] rust2-bench compare counts base and head from one directory

## Goal

`rust2-bench compare <base>` of identical code reports no change: it counts the base and the working tree from the same directory, so neither side's count moves by where it was checked out.

Non-goals: noise inside a benchmark itself, such as wall-clock timers or thread interleaving, which each benchmark's own repository fixes (moq: [quest/m0/repeatable-instruction-counts.md](https://github.com/moq-dev/moq/blob/main/quest/m0/repeatable-instruction-counts.md)).

## Plan

Found in [moq-dev/moq#9555](https://github.com/moq-dev/moq/pull/9555): `compare` counts the base in its cached worktree under `~/.cache/rust2-bench` and the head in the caller's checkout. Counts that repeat exactly within one checkout then differ by location alone, such as moq's `rs/moq-mux/benches/jitter.rs::benches::handoff::jitter_handoff::4[64]` (459,450 in one, 486,025 in the other, 5.5%) and `rs/moq-json/benches/codec.rs::benches::marshal::marshal::full[big_static]` (170,198 against 169,680), measured at kixel-ai/rust2@6ead6d56.

Decided 2026-11-24 (maintainer, `/quest-plan`): ✅ count both sides in the cached worktree: after counting the base, check the working tree's state (its `HEAD` plus uncommitted and untracked changes) out there and count it at the same path (rejected: keep two directories and neutralize each cause, such as `--remap-path-prefix` and a fixed environment and working-directory length, which chases causes one by one and may miss one). m0, since moq's and rind's agents use `compare` to back performance claims now.

Recommendations for the implementer:

- Snapshot the working tree as a commit without touching the caller's index or stash, such as `git stash create` plus untracked files through a temporary index, and check that commit out in the cached worktree.
- The head then rebuilds incrementally in the cached worktree from the base's build; measure what that costs against today's head build in the caller's `target/`.
- Prove it by comparing a commit against itself on moq, including `jitter_handoff`, and finding every count equal; then confirm a real change still shows, as #995's doubled `encode` did.

Public API: none; `compare`'s output is unchanged. Wire: none.

## Related

- [Per-side selection in compare](/quest/m1/bench-compare-selection.md) - also changes how compare builds each side
- [The harness's counts](/quest/m2/harness/counts.md) - absorbs the runner's compare later
