# [S] The runtime's namespace is rs

## Goal

[rust2-ts](/ts/README.md)'s runtime is imported and named `rs` rather than `rt`, in the
translator's output, the runtime's own files, the fixtures and their snapshots, the tests,
and the README, so a translated Rust type reads as `rs.Vec`, `rs.HashMap`, `rs.u64`.
Nothing named `rt` is left.

## Plan

Decided with the maintainer on 2026-11-03: rename, as its own PR, started next. The
wrapper quests ([rs.String](/quest/m3/ts-string-capacity.md),
[rs.Slice](/quest/m3/ts-slice-object.md)) run in parallel rather than waiting on it;
whichever lands second merges the rename.

It touches every open TS PR, so land it when few are in flight and merge main into
them rather than leaving them to conflict. It's mechanical: the snapshots change by name
only, and the bundles should differ by nothing but the name.

Public API: the translator's output and every hand-written import of the runtime.
