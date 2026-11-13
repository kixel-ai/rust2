# [S] Object cache hits on an identical queue rebuild

## Goal

A merge-queue build that recompiles crates an earlier queue build already compiled serves
them from mbx's object cache, so the `mbx[cache]: object cache` line shows hits for them.

## Waiting

- Maintainer: keep this quest until a queue build shows a repeat miss, or delete it, since identical rebuilds on the runner now hit. Recommend deleting it: four runs on the runner hit, and gate CPU already beats the baseline.

## Plan

Two queue builds on the `nixos-rust2-ci` runner (nspawn container `rust2-ci` on `nixos`)
compiled the same 55 crates and both logged 0 hits, 55 misses, 66.2 MiB stored locally:
[run 43265518031](https://github.com/moq-dev/rust2/actions/runs/43265518031) and
[run 43290025781](https://github.com/moq-dev/rust2/actions/runs/43290025781). A later one,
[run 43304642908](https://github.com/moq-dev/rust2/actions/runs/43304642908), got 17 hits
of 204 lookups. Each job still used 9 to 19 CPU-minutes (the `quest-gate` scopes' `Consumed`
CPU in the container journal, `journalctl -D /var/lib/nixos-containers/rust2-ci/var/log/journal`
on `nixos`, not Actions wall time) against the 65 of the 2026-11-01 baseline, so the cache is headroom, not the gate's main saving.

Find the cause before fixing it: whether the local store persists across queue jobs in the
container (path, owner, cleanup between jobs), and whether keys change between identical
runs (paths, env, timestamps in the key). Reproduce by running the same build twice in the
container and diffing the keys mbx logs. Fix at the root, and confirm with a queue build
whose repeated crates hit.

Findings of 2026-11-12 ([#735](https://github.com/kixel-ai/rust2/pull/735)): the misses don't reproduce.
Both runs' misses came from `ffi`'s `tests/fixtures.rs` session, and only quest Markdown
differed between their heads, yet each stored the same 66.2 MiB again.
On the runner itself, as `ci` with the service's environment and the job's mbx 1.29.2, four
fresh clones at one path each ran `just ffi test --test fixtures`. After the first, each
got 62 hits and 0 misses, including two runs with new `GITHUB_RUN_ID`, `GITHUB_SHA`, and `CI` values.
mbx 1.29.2 also hit locally after touching every source file (62 of 64).
These are ruled out: store persistence (one store bind-mounted from `/var/cache/ci-mbx/actions`
into all five runner containers, no cleanup between jobs), eviction (every job boundary from
17:05 to 19:26 on rust2 and moq logged `evicted 0`), XFS project quota (284 of 574 GiB), the
exporter's per-run `api-<pid>` stage path, and GitHub's per-run variables.
The session history that would show the two runs' keys was pruned (256 sessions shared by
all five runners), so the cause is unknown. Two facts stand out: the store sits at its
50 GiB `MBX_GC_MAX_SIZE` cap, and the job-boundary GC hook runs mbx 1.27.0 while jobs run 1.29.2.

Next: the next queue build that recompiles the `ffi` fixtures with code unchanged is the
evidence. If it misses, run `mbx explain --last` in `/var/cache/github-runner-work/rust2/rust2/ffi`
inside the container before another build prunes the session.

Decided 2026-11-12 (maintainer, the pick of [#720](https://github.com/moq-dev/rust2/pull/720)):
✅ keep this quest (rejected: drop it, since gate CPU already beats the baseline).
