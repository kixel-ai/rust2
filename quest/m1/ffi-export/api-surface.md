# [M] Lock each language's rendered API

## Goal

A change to any generated API fails CI unless the PR updates a committed surface file
for that language: for the `api` fixture and moq's stand-in, the C header and `.hpp`,
Kotlin's ABI dump, Swift's API digester output, the Python `.pyi` or signatures,
`go doc -all`, the Dart public API, and the web binding's `.d.ts` or exported signatures.
The release builder passes `--baseline` with the last release's `api.json`, so a release
that breaks the Rust API without a breaking version fails, and its changelog comes from
the `api.json` diff.

## Plan

Found by the 2026-11-18 language audit: [the API lock](/ffi/README.md#the-api-lock)
compares the IR's structure and each generator's names (`ffi/src/lock.rs`), not the
signatures each language renders, so an exporter change such as
[#832](https://github.com/kixel-ai/rust2/pull/832) changes the Kotlin and Swift types
with no Rust change and nothing reports it. `.github/workflows/bindings.yml` never passes
`--baseline`.

Decided with the maintainer on 2026-11-18: ✅ a child of this epic, so the checks are in
place before moq pins (rejected: after the pin). Land it before the other breaking
children of the 2026-11-18 audit, so each of their PRs shows its diff in the surface files.

Recommended: prefer each language's own tool where one exists (Kotlin's `abiValidation`,
`swift api-digester`), else a sorted, normalized dump the generator already has in hand.
Generated, not hand-edited: a single recipe rewrites every surface file.

Public API: none; the release builder gains the baseline check.
