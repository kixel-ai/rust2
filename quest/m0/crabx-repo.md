# [L] Move crabx to its own repository, kixel-ai/crabx

## Goal

crabx lives in `kixel-ai/crabx`, a public GPL-3.0-only repository whose history is
rust2's crabx history, filtered. Its runner, vendored fetchers, installers, release
pipeline, workflows, Pages site at `crabx.kixel.ai`, and quests all move there, and
rust2 keeps no crabx code. rust2's root toolchain returns to moq's 1.98.1, so the
`channel_stable` fixture and the `dev-stable` Python build prove the generated glue on
the stable toolchain moq builds it with.

## Plan

Decided by the maintainer on 2026-11-22, in
[#948](https://github.com/kixel-ai/rust2/pull/948#issuecomment-7074616370): ✅ move crabx
to `kixel-ai/crabx`, keep its history with a filtered copy, and plan the move as this
quest (rejected: a fresh start without history; holding #948 to redo binstalk 0.31 in the
new repository; an interim crabx-only toolchain). The reason: binstalk 0.31 needs Rust
1.100, which moved rust2's root toolchain off moq's 1.98.1. The doc comment on
`stable_rustc()` in `ffi/tests/fixtures.rs` names this gap.

Recommendations, the maintainer's to change in review:

- [First publish](/quest/m2/crabx/first-publish.md) waits for the move, so the
  registries trust `kixel-ai/crabx` from the first version.
- crabx sits at the new repository's root: `Cargo.toml`, `src/`, `vendor/`, `npm/`, and
  the installers. rust2's `.github/crabx/` becomes `pipeline/`. Its quests become the new
  repository's quest tree: the epic README's Goal and decisions go to its root README,
  first publish goes to its m0, and [the GitHub App](/quest/m3/crabx-app.md) goes to a
  later milestone. Nothing in rust2 is waiting on them.
- The history is `git filter-repo` over every path crabx has had: `crabx/`,
  `.github/crabx/`, the `crabx*.yml`, `installers.yml`, and `release.yml` workflows,
  `quest/m2/crabx/`, and `quest/m3/crabx-app.md`, renamed to the new layout. Rewrite bare
  `#N` references in commit messages to `kixel-ai/rust2#N`, or they link to the new
  repository's own PRs.
- The new repository takes rust2's setup, trimmed to what crabx uses: the nix flake and
  direnv on Rust 1.100, just with `just ci`, AGENTS.md, `quest init`, Pullfrog, and the
  rulesets from dotfiles' `repo-settings`. Its CI runs on GitHub-hosted runners, since the
  repository is public.
- Every hard-coded `kixel-ai/rust2` in crabx is retargeted. The pipeline's reusable
  workflow is called from its new path, and `pipeline-v1.0.0` is cut again there, after a
  dispatch whose archives pass `gh attestation verify -R kixel-ai/crabx --signer-workflow
  kixel-ai/crabx/.github/workflows/crabx.yml`. `crabx init`'s `binaries.yml` and the
  pipeline README pin that tag's commit. The other targets are the trust check's pipeline
  signer (`src/trust.rs`), `release.py`'s `REPO`, the installers' download URLs,
  `Cargo.toml`'s `repository`, the pipeline's User-Agent and caller check, and the tests
  that pin them.
- The trust check accepts only `kixel-ai/crabx`'s pipeline as a signer, no longer
  rust2's. No other repository calls rust2's pipeline (a GitHub code search finds only
  rust2), and no crabx release was ever attested. Leave rust2's `pipeline-v1.0.0` tag in
  place: nothing calls it, and deleting it gains nothing.
- The new repository gets the `github-pages`, `crates-io`, and `npm` environments, which
  only `main` may deploy to. The `crabx.kixel.ai` custom domain moves from rust2's Pages to
  it. The domain's DNS stays as it is (a CNAME to `kixel-ai.github.io`), and `kixel.ai` is
  verified for the organization. Last time, GitHub issued the certificate only after the
  domain was removed and set again. The site is down while the domain moves, which
  matters to no one before first publish.
- rust2's PR, the one finishing this quest, merges only once the new repository's `main`
  passes CI and serves the installers. That PR:
  - deletes `crabx/`, `.github/crabx/`, the five workflows, the `crabx` module in
    `sh/dispatch.sh` and its tests, and `mod crabx` and `just crabx fix` in the justfile;
  - returns `rust-toolchain.toml` to 1.98.1, fixing whatever landed since #948 that needs
    a newer stable;
  - drops the 1.100 paragraph from `stable_rustc()`'s comment;
  - points `ffi/release/README.md`'s crabx link at the new repository;
  - rewrites the crabx lines in `quest/README.md`, `quest/m2/README.md`, and
    `quest/m3/README.md` into a one-line pointer;
  - deletes rust2's `crates-io`, `npm`, and `github-pages` environments and its Pages
    site, which only crabx used (moq publishes the bindings).

Validation:

- The new repository's CI passes on `main`.
- `curl -fsSL https://crabx.kixel.ai/install.sh` serves an installer that downloads from
  `kixel-ai/crabx`.
- The pipeline's fixture dispatch passes the `gh attestation verify` above.
- crabx's trust test accepts the crabx signer and refuses rust2's.
- In rust2's dev shell, `rustc -V` prints 1.98.1, the `channel_stable` fixture runs on it,
  and `git grep -i crabx` finds only the quest tree's pointer.

## Waiting

- Maintainer: confirm the milestone. Recommended: m0, which holds urgent fixes to what a
  consumer already depends on. moq depends on the stable-glue checks, and they test 1.100
  instead of its 1.98.1 until this lands. The alternative is a child of
  [the crabx epic](/quest/m2/crabx/README.md) in m2, which would rank it with crabx's
  research, behind m1's exporter work, and leave the gap open longer. Done when this file
  sits in the chosen milestone and this item is removed.
