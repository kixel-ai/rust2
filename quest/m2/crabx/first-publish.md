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

The installers are served at `crabx.kixel.ai` by GitHub Pages from this repository:
`.github/workflows/installers.yml` deploys `crabx/install.sh` and `crabx/install.ps1`
whenever main changes them. kixel.ai's DNS is on Cloudflare, so the record points the
subdomain at GitHub; a Cloudflare redirect to the raw files was rejected because it lives
outside the repository and serves whatever path layout main happens to have.

## Waiting

- Maintainer: verify `kixel.ai` for GitHub Pages in the kixel-ai organization, so no other
  account can claim `crabx.kixel.ai` if Pages is ever unset. An agent can't: it needs an
  organization owner and the Cloudflare login. In organization Settings → Pages → Add a
  domain, enter `kixel.ai`, add the TXT record it shows
  (`_github-pages-challenge-kixel-ai.kixel.ai`) in Cloudflare, then click Verify.
- Maintainer: turn on Pages for this repository, built by GitHub Actions, at
  `crabx.kixel.ai`. An agent can run this once asked; it waits for the maintainer because
  it changes repository settings. Check `gh api repos/kixel-ai/rust2/pages` shows
  `"build_type": "workflow"` and `"cname": "crabx.kixel.ai"`, and that the `github-pages`
  environment lets only `main` deploy.

  ```sh
  gh api -X POST repos/kixel-ai/rust2/pages -f build_type=workflow
  gh api -X PUT repos/kixel-ai/rust2/pages -f cname=crabx.kixel.ai
  ```

- Maintainer: in Cloudflare's DNS for `kixel.ai`, add a `CNAME` record named `crabx` with
  target `kixel-ai.github.io` and proxy status **DNS only**, so GitHub can issue the
  certificate. An agent can't: it needs the Cloudflare login. Then deploy and enforce
  HTTPS once GitHub shows the certificate, and check
  `curl -fsSL https://crabx.kixel.ai/install.sh | head -3` prints the installer.

  ```sh
  gh workflow run installers.yml -R kixel-ai/rust2
  gh api -X PUT repos/kixel-ai/rust2/pages -F https_enforced=true
  ```


- Maintainer: create the `crabx` organization on npmjs.com (free, public packages). An
  agent can't: it needs the maintainer's npm account. Check `npm org ls crabx`.
- Maintainer: let only `main` deploy to the `crates-io` and `npm` environments, before
  release.yml's first run creates them unprotected. An agent can run this once asked; it
  waits for the maintainer because it changes repository settings. Check each
  environment's deployment branches in the repository settings.

  ```sh
  for env in crates-io npm; do
    gh api -X PUT repos/kixel-ai/rust2/environments/$env \
      -F 'deployment_branch_policy[protected_branches]=false' \
      -F 'deployment_branch_policy[custom_branch_policies]=true'
    gh api -X POST repos/kixel-ai/rust2/environments/$env/deployment-branch-policies -f name=main -f type=branch
  done
  ```

- Maintainer: publish crabx 0.1.0 to crates.io from a clean checkout of `main` that
  includes release.yml, since the pipeline builds only versions published after it was
  added. An agent can't: publishing is public and permanent and needs the maintainer's
  crates.io token. Check `https://crates.io/crates/crabx`.

  ```sh
  git -C <rust2> switch --detach origin/main && cd <rust2>/crabx
  cargo publish --locked
  ```

- Maintainer: on crates.io, crabx → Settings → Trusted Publishing → add GitHub with owner
  `kixel-ai`, repository `rust2`, workflow `release.yml`, environment `crates-io`. After
  release.yml first publishes a version, turn on "Require trusted publishing". An agent
  can't: it needs the crate owner's session.
- Maintainer: once `gh workflow run release.yml -R kixel-ai/rust2` has attached all six
  archives to `crabx-v0.1.0` (`gh release view crabx-v0.1.0 -R kixel-ai/rust2`), publish
  the npm packages, which verifies each archive's attestation first, then trust the
  workflow for each (npm 11.15.0 or later). An agent can't: both need the maintainer's npm
  login and 2FA. Check `npx crabx --version` prints `crabx 0.1.0`.

  ```sh
  npm login
  GH_TOKEN=$(gh auth token) python3 crabx/release.py publish 0.1.0
  for p in crabx @crabx/linux-x64-gnu @crabx/linux-arm64-gnu @crabx/darwin-x64 \
    @crabx/darwin-arm64 @crabx/win32-x64 @crabx/win32-arm64; do
    npm trust github "$p" --file release.yml --repo kixel-ai/rust2 --env npm --allow-publish --yes
    sleep 2
  done
  ```
