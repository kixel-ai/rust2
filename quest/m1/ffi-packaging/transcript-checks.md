# [S] The api fixture's release checks find their transcript modules

## Goal

`bindings-test.yml`'s api fixture passes again: its Python, C++, and web consumers run
`ffi/tests/api.py`, `api.cpp`, and `api.ts` on the installed packages and print native
Rust's transcript byte for byte.

## Plan

Found while running [the Apple quest](https://github.com/kixel-ai/rust2/pull/912)'s Check:
since [#885](https://github.com/kixel-ai/rust2/pull/885), each of those checks imports a
`transcript` module that `ffi/tests/fixtures/transcript` generates from
`fixtures/api/examples/transcript.rs`, but the builder's `Checks` step copies only the check
file, so every api consumer fails to import it (`No module named 'transcript'`, Vite's
`Could not resolve './transcript.ts'`).

Recommendation: the builder takes one check per language as it does now, and the release
test generates each check's transcript module beside it before calling the builder, since a
caller's own check is one file and the transcript is the fixture's test machinery, not the
builder's. Rejected: a builder input for extra check files, which only this fixture needs.
