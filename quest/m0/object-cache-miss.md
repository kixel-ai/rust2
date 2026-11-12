# [S] Object cache hits on an identical queue rebuild

## Goal

A merge-queue build that recompiles crates an earlier queue build already compiled serves
them from mbx's object cache, so the `mbx[cache]: object cache` line shows hits for them.

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

Decided 2026-11-12 (maintainer, the pick of [#720](https://github.com/moq-dev/rust2/pull/720)):
✅ keep this quest (rejected: drop it, since gate CPU already beats the baseline).
