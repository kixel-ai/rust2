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

## Waiting

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
