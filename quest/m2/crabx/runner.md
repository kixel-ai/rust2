# [L] The crabx runner

## Goal

`crabx <crate>[@version] [args]` resolves the crate through binstalk, downloads the
binary for the host into its own cache, and `exec`s it, passing through arguments,
signals, and the exit code. It never compiles unless `--build` is passed, which runs
`cargo install` into the cache. A pinned version runs from the cache with no network.

## Plan

Decided in [the epic](/quest/m2/crabx/README.md#plan): built on binstalk; the trust rule
(author assets and our builds, attestations verified when published, quickinstall only
behind a flag); stale-while-revalidate for unpinned runs.

Decided with the maintainer on 2026-11-12: until binstall guesses `{name}-v{version}`
tags ([its quest](/quest/m2/crabx/binstall-tags.md)), crabx carries binstalk patched to
guess them, vendored or as a git dependency.

Recommendations:

- Verify an attestation once per binary digest, against the crate's repository and a
  workflow on its tag, with sigstore-rs and GitHub's attestations API; cache the result.
- Measure the warm path against `npx` and `bunx` running a cached package, and put the
  numbers in the PR.
