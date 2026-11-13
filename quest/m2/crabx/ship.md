# [M] Ship crabx

## Goal

crabx's release publishes it to crates.io, and its own pipeline then attaches
attested binaries for the six targets, and publishes it to npm, so `npx crabx <crate>`
and `bunx crabx <crate>` work with nothing else installed. A one-line shell and
PowerShell installer fetch the release binary.

## Plan

Recommendations:

- npm as per-platform packages selected by `os`, `cpu`, and, on Linux, `libc: ["glibc"]`
  (the esbuild pattern), with no postinstall download. On musl no package matches, and
  the launcher fails loudly rather than run a glibc binary.
- The npm publish is the one job with npm publish rights, kept minimal and pinned by a
  test; npm trusted publishing needs a GitHub-hosted runner.
- This adds rust2's first published crate, so it sets up crate release tagging; the
  pipeline's own release tags come from [the pipeline](/quest/m2/crabx/pipeline.md).

## Required

- [The pipeline](/quest/m2/crabx/pipeline.md) - builds and attests crabx's binaries
