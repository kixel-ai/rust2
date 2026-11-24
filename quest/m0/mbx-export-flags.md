# [S] Ask mbx to cache the export's MIR flags

## Goal

mbx caches a compilation that carries `-Zalways-encode-mir`, `-Zmir-opt-level=<n>`, or
`-Zinline-mir=<bool>`, so a warm `rust2-ffi` export, native or web, is served every crate
but the workspace's own, and the merge queue's mbx pin in
`.github/workflows/build.yml` is a release that does.

## Plan

Found on 2026-11-22 while making exports cacheable: mbx 1.30 bypasses any compilation
holding a rustc flag its cache adapter does not model ("rustc flag is not modeled by the
cache adapter: -Zalways-encode-mir"), and the export adds those three to every crate of
its target. mbx already models some `-Z` flags, and added `-Zforce-unstable-if-unmarked`
on request in [jdx/mr-boxington#565](https://github.com/jdx/mr-boxington/issues/565).
Each of the three changes what rustc writes, so the ask is to key the action on it, not
drop it.

Asked in [jdx/mr-boxington#1003](https://github.com/jdx/mr-boxington/issues/1003),
posted on 2026-11-22 with the maintainer's approval, whose reproduction on mbx 1.30.0 shows
each of the three bypassing both crates of a two-crate workspace that the cache serves
without them. Once a release models them, bump the pin and show `mbx explain` serving the
stand-in's crates on a second export.

## Waiting

- [jdx/mr-boxington#1003](https://github.com/jdx/mr-boxington/issues/1003): a release
  whose rustc cache adapter keys `-Zalways-encode-mir`, `-Zmir-opt-level`, and
  `-Zinline-mir` instead of bypassing them; check that `mbx explain` on it no longer lists
  them under `unknown-flag`
