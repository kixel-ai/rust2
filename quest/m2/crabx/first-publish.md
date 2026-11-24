# [XS] Publish crabx 0.1.0 by hand once

## Goal

crabx 0.1.0 is on crates.io and npm, and both registries trust only
`.github/workflows/release.yml` to publish what follows, so `npx crabx <crate>`,
`bunx crabx <crate>`, `cargo install crabx`, and both installers in
[crabx's README](/crabx/README.md) work.

## Plan

Neither registry lets trusted publishing create a package, so each first version is
published with the maintainer's own login; release.yml publishes every later version.
Until crabx is on crates.io, release.yml fails at its plan job, saying so. The first run
after the crates.io publish attaches the binaries and then fails at its npm job, since npm
doesn't trust the workflow until the packages exist: that's expected, and the steps below
fix it.

The maintainer chose to keep this quest rather than fold its steps into the PR that added
release.yml ([#826](https://github.com/kixel-ai/rust2/pull/826)).

Decided by the maintainer on 2026-11-22, in
[#948](https://github.com/kixel-ai/rust2/pull/948#issuecomment-7074616370): ✅ this waits
for [the move to `kixel-ai/crabx`](/quest/m0/crabx-repo.md), so both registries trust that
repository's `release.yml` from the first version. The steps below already name it and its
root layout; the move carries this quest into its quest tree.

The installers are served at `crabx.kixel.ai` by GitHub Pages:
`.github/workflows/installers.yml` deploys `install.sh` and `install.ps1`
whenever main changes them. kixel.ai's DNS is on Cloudflare, so the record points the
subdomain at GitHub; a Cloudflare redirect to the raw files was rejected because it lives
outside the repository and serves whatever path layout main happens to have.

Done on 2026-11-18 (/quest-complete, with the maintainer's go-ahead): `kixel.ai` is verified
for Pages in the kixel-ai organization; Pages builds from Actions at `crabx.kixel.ai`;
Cloudflare has the challenge TXT record and the `crabx` CNAME to `kixel-ai.github.io`, DNS
only; and only `main` may deploy to the `github-pages`, `crates-io`, and `npm` environments.
The installers deployed and HTTPS is enforced, so `curl -fsSL https://crabx.kixel.ai/install.sh`
prints the installer. GitHub issued the certificate only after the custom domain was removed
and set again. The move redoes the Pages and environment steps in `kixel-ai/crabx`.

## Waiting

- Maintainer: confirm the npm user `crabx` is yours. The `crabx` npm organization already
  exists, and npm lists that user as its only member and owner
  (`https://registry.npmjs.org/-/org/crabx/user`); if it is someone else's, the `@crabx/*`
  platform packages need another scope. An agent can't tell: neither agent Chrome is signed
  in to npm.
- Maintainer: publish crabx 0.1.0 to crates.io from a clean checkout of `main` that
  includes release.yml, since the pipeline builds only versions published after it was
  added. An agent can't: publishing is public and permanent and needs the maintainer's
  crates.io token. Check `https://crates.io/crates/crabx`.

  ```sh
  git -C <crabx> switch --detach origin/main && cd <crabx>
  cargo publish --locked
  ```

- Maintainer: on crates.io, crabx → Settings → Trusted Publishing → add GitHub with owner
  `kixel-ai`, repository `crabx`, workflow `release.yml`, environment `crates-io`. After
  release.yml first publishes a version, turn on "Require trusted publishing". An agent
  can't: it needs the crate owner's session.
- Maintainer: once `gh workflow run release.yml -R kixel-ai/crabx` has attached all six
  archives to `crabx-v0.1.0` (`gh release view crabx-v0.1.0 -R kixel-ai/crabx`), publish
  the npm packages, which verifies each archive's attestation first, then trust the
  workflow for each (npm 11.15.0 or later). An agent can't: both need the maintainer's npm
  login and 2FA. Check `npx crabx --version` prints `crabx 0.1.0`.

  ```sh
  npm login
  GH_TOKEN=$(gh auth token) python3 release.py publish 0.1.0
  for p in crabx @crabx/linux-x64-gnu @crabx/linux-arm64-gnu @crabx/darwin-x64 \
    @crabx/darwin-arm64 @crabx/win32-x64 @crabx/win32-arm64; do
    npm trust github "$p" --file release.yml --repo kixel-ai/crabx --env npm --allow-publish --yes
    sleep 2
  done
  ```

## Required

- [Move to kixel-ai/crabx](/quest/m0/crabx-repo.md) - the repository the registries trust
  to publish crabx
