# [S] Ask mbx to cache what a workspace wrapper hands back

## Goal

When `rust2-ffi` exports a crate from inside a larger workspace, as moq exports its own
`moq-ffi`, the sibling workspace crates the driver only passes through are served from
mbx's cache, not compiled uncached under the driver.

## Plan

Decided with the maintainer on 2026-11-22: ✅ ask mbx upstream, if the ask holds up, beside
the exporter's own fix, which makes every non-workspace crate cacheable whose paths mbx can
key: not a wasm32 export's over the dev shell's MIR sysroot, under `/nix/store`, which
[the `/nix/store` ask](/quest/m1/mbx-nix-store.md) takes up. mbx defers every workspace member to a `RUSTC_WORKSPACE_WRAPPER`
("deferring to it for workspace crates, so those compilations are not cached"), which is
right for a wrapper with side effects, such as the export itself, but leaves uncached the
members the driver hands to rustc unchanged.

Recommended ask: a documented way for a wrapper to hand one compilation back to mbx's
cache, such as running a command mbx names in the environment in place of `rustc`, so
the wrapper keeps the compilations it analyzes and mbx caches the rest.

Measured 2026-11-25 on **desktop** with mbx 1.31.0 (bench/ffi/README.md's "The export's
cache"): moq's own `moq-ffi`, a `staticlib` and `cdylib` only, does not export, so the
native stand-in stood in as a member of moq's workspace. Warm from a store one export
filled, its 16 sibling units under the wrapper cost the export 13 s wall and 19 CPU
seconds more than the stand-in's 8 and 12 outside it, two and a half times its CPU
(medians of 4 rounds at a load average of 11 to 51); cold, about the same, 28 s and 122 CPU
seconds against 28 and 115. Post only if that cost is material, with the numbers:
jdx/mr-boxington's
[CONTRIBUTING](https://github.com/jdx/mr-boxington/blob/main/CONTRIBUTING.md) takes
proposals in Discussions, allows AI help if disclosed, and closes reports found only by
reading the source. If mbx ships the hook, the driver adopts it here.

Decided with the maintainer on 2026-11-22, in the pick of
[kixel-ai/rust2#959](https://github.com/kixel-ai/rust2/pull/959): ✅ keep this quest
(rejected: drop it), and ✅ its agent may post the Discussion, disclosing AI help, once the
numbers show a material cost (rejected: asking the maintainer with the numbers first).
