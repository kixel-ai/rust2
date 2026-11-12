# [M] Ship crabx

## Goal

crabx publishes through its own pipeline, so a release puts it on crates.io, attaches
attested binaries for the six targets, and publishes it to npm, so `npx crabx <crate>`
and `bunx crabx <crate>` work with nothing else installed. A one-line shell and
PowerShell installer fetch the release binary.

## Plan

Recommendations:

- npm as per-platform packages selected by `os`, `cpu`, and, on Linux, `libc: ["glibc"]`
  (the esbuild pattern), with no postinstall download. On musl no package matches, and
  the launcher fails loudly rather than run a glibc binary.
- npm trusted publishing is the one job on a GitHub-hosted runner, kept minimal and
  pinned by a test.
- This adds rust2's first published crate, so it sets up release tagging too.

## Required

- [The pipeline](/quest/m2/crabx/pipeline.md) - builds and attests crabx's binaries
- [The runner](/quest/m2/crabx/runner.md) - what ships
