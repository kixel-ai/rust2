# [S] The binding benchmark builds where mbx manages ffi/target

## Goal

`just ffi bench` and `just ffi count` build from a fresh checkout on a host whose mbx manages target directories, as fedora's does, without bypassing mbx.

## Plan

Decided 2026-11-25 (maintainer, pick round on [kixel-ai/rust2#1034](https://github.com/kixel-ai/rust2/pull/1034)): ✅ keep this quest (rejected: drop it).

Found 2026-11-25 counting the [hooks cutover](https://github.com/kixel-ai/rust2/pull/1034) on **fedora** (mbx 1.31.0), from a fresh clone of main at ede41976: mbx turns `ffi/target` into a symlink to `/var/cache/mbx/targets/v1/<hash>`, and the recipes keep moq's checkout and the stand-in inside it (`target/moq`, `target/moq-ffi`).
`moq-export` then fails with Cargo's `package collision in the lockfile: packages kio v0.6.2 (.../ffi/target/moq/rs/kio) and kio v0.6.2 (/var/cache/mbx/targets/v1/<hash>/moq/rs/kio) are different`: one path reaches moq's crates through the symlink and another through its target.
Counting with mbx's shim off `PATH` and `ffi/target` a plain directory builds.

Recommendation: keep sources out of the target directory, such as moq's checkout and the generated crates under `ffi/.work/` (gitignored), so no path resolves two ways; otherwise reproduce it in a small fixture and ask mbx upstream.
Find every recipe that writes a crate or checkout under `target/` first, `just ts`'s `target/moq` included.

## Hosts

- linux - fedora's mbx manages target directories
