# crabx: run a Rust CLI without compiling it

## Goal

Someone with no Rust toolchain runs a Rust CLI as `crabx <crate>[@version] args`, as fast
as npx on a warm cache, from a prebuilt binary for their platform, and never compiles
unless they pass `--build`. Any crate whose releases carry assets binstall finds works,
with quickinstall's third-party builds behind a flag. A repo opts into prebuilt builds of each crates.io version, for Linux, macOS, and Windows on x86_64 and arm64, by
adding one configless workflow. moq's CLIs (moq-cli, moq-relay) use it first.

Non-goals: hosting a builder, building repos that haven't opted in, builds of `main` or
any version not on crates.io, private repos, and replacing cargo-binstall or crates.io.

What stays here once every child has merged: an end-to-end check that runs a published
crate through crabx on all six targets from a machine with no Rust toolchain, a docs
page, and crabx in the charter line of AGENTS.md and README.md.

## Plan

Decided with the maintainer on 2026-11-12, in /quest-plan:

- ✅ Builds run in the author's own CI on GitHub-hosted runners, free for public repos,
  and publish to the author's GitHub releases with GitHub attestations. Rejected: a
  builder we host (it runs strangers' build.rs with our credentials, makes us the trust
  root, and GitHub's Actions terms forbid one repo's runners building unrelated
  projects); author CI now with a hosted builder later.
- ✅ Our own runner, built on the binstalk crate. Rejected: upstreaming a `run` mode into
  cargo-binstall (it installs to `~/.cargo/bin` and asks crates.io every run); adopting
  or forking [crgx](https://github.com/yfedoseev/crgx) (one maintainer; it lacks
  attestation checks, and turns quickinstall on by default, against the trust rule below)
  or [cgx](https://github.com/anelson/cgx) (it compiles by default and needs config).
- ✅ A runner installed once, and also published to npm so `npx crabx <crate>` needs
  nothing new. Rejected: runner only; each tool published to npm or PyPI on its own.
- ✅ Anything binstall resolves from the author's releases, so crates that already attach
  release assets work on day one; the pipeline primarily exists to set those assets up. Rejected: opted-in repos
  only; any crates.io crate built on demand.
- ✅ Trust: crabx runs our pipeline's builds and an author's own release assets, and
  verifies an attestation wherever the repo publishes one (a bad one is an error).
  cargo-quickinstall, built and signed by a third party, is refused unless a flag allows
  it. Rejected: attested only; everything binstall does.
- ✅ Only crates.io versions are built. Rejected: a rolling `main` prerelease, GHCR OCI
  artifacts, and per-commit prereleases.
- ✅ Linux binaries target gnu through cargo-zigbuild against an old glibc floor, such
  as 2.28 (RHEL 8, Debian 10). Rejected: static musl (a slow allocator for moq-relay, and
  native deps often fail to build); both. On musl Linux, crabx fails loudly and points
  at `--build`.
- ✅ An unpinned run is stale-while-revalidate: it runs the cached latest and checks
  crates.io's sparse index in the background for the next run. Once cached, a pinned version
  runs with no network. Rejected: a check every run; a 24-hour TTL.
- ✅ The name `crabx`, free on npm and crates.io, following npx, bunx, and uvx.
  Rejected: xcrate, rusx, carx.
- ✅ A pipeline workflow and `crabx init` now, the GitHub App
  [later](/quest/m3/crabx-app.md). Rejected: the App in the first cut; no App.
- ✅ m2, the App in m3. Rejected: m1; m3.

moq switching its release workflows to the pipeline is moq's quest, filed from this
plan's PR; it owns keeping moq's armv7 build, Homebrew asset names, and `.deb`/`.rpm`
packages, alongside or instead of the pipeline.
