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

Decided with the maintainer on 2026-11-22, in the pick of
[kixel-ai/rust2#959](https://github.com/kixel-ai/rust2/pull/959): ✅ keep this quest
(rejected: drop it), and ✅ its agent may post the Discussion, disclosing AI help, once the
numbers show a material cost (rejected: asking the maintainer with the numbers first).

## Required

- [Measure the cached export](/quest/m0/ffi-export-cache.md) - measures what workspace
  members still cost once every other crate is cached
