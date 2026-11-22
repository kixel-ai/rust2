# [S] Ask mbx to cache what a workspace wrapper hands back

## Goal

When `rust2-ffi` exports a crate from inside a larger workspace, as moq exports its own
`moq-ffi`, the sibling workspace crates the driver only passes through are served from
mbx's cache, not compiled uncached under the driver.

## Plan

Decided with the maintainer on 2026-11-22: ✅ ask mbx upstream, if the ask holds up, beside
[the exporter's own fix](/quest/m0/ffi-export-cache.md), which makes every non-workspace
crate cacheable. mbx 1.30 defers every workspace member to a `RUSTC_WORKSPACE_WRAPPER`
("deferring to it for workspace crates, so those compilations are not cached"), which is
right for a wrapper with side effects, such as the export itself, but leaves uncached the
members the driver hands to rustc unchanged.

Recommended ask: a documented way for a wrapper to hand one compilation back to mbx's
cache, such as running a command mbx names in the environment in place of `rustc`, so
the wrapper keeps the compilations it analyzes and mbx caches the rest.

First measure, after the exporter's fix, what moq's real `moq-ffi` export spends on
uncached workspace members, cold and warm. Post only if that cost is material, with the
numbers: jdx/mr-boxington's
[CONTRIBUTING](https://github.com/jdx/mr-boxington/blob/main/CONTRIBUTING.md) takes
proposals in Discussions, allows AI help if disclosed, and closes reports found only by
reading the source. If mbx ships the hook, the driver adopts it here.

## Waiting

- Maintainer: approve posting the measured proposal as a Discussion on jdx/mr-boxington,
  outside the orgs an agent may post to without asking. Recommended: approve once the
  numbers are in. Check: the Discussion exists.

## Required

- [Exports that mbx can cache](/quest/m0/ffi-export-cache.md) - makes every other crate
  cacheable and measures what workspace members still cost
