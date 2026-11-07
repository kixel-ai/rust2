# [XS] TypeId snapshots alike on every host

## Goal

`ts/fixtures/lang/dyns/expected/` and `ts/fixtures/lang/panics/hook/expected/` come out
byte-identical in every checkout, on every host, and on CI, so local `just ci` passes on
main. TypeId behavior stays exact: comparisons within a run, `Debug`'s shape, and the
`type_id` differential tests against native, none of whose assertions loosen.

## Plan

Decided with the maintainer on 2026-11-07: m0, start now. Caused by
[moq-dev/rust2#605](https://github.com/moq-dev/rust2/pull/605), which emits each `TypeId`
as rustc's 128-bit hash; reported in the Follow-ups of
[moq-dev/rust2#632](https://github.com/moq-dev/rust2/pull/632).

Cause, reproduced: not the checkout's path. Cargo hashes the build profile into a crate's
`-C metadata`, which seeds its `StableCrateId` and so the hash of every type it defines. A
host's own cargo config changes it: `~/.cargo/config.toml`'s `[profile.dev] debug = false`
alone turns the committed snapshot into another, and `build.incremental` or a nightly bump
does too. Std's types hash alike everywhere.

Mask the hashes in the snapshot comparison only: each 32-hex string literal becomes a
placeholder numbered by first appearance in its file, written so by `UPDATE_EXPECT`, so a
snapshot still shows which ids are equal. Pinning the profile in the harness was rejected:
every hashed profile field, config file, and toolchain bump is another pin to keep.
