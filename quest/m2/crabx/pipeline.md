# [M] The release pipeline

## Goal

A reusable workflow in this repository that a crate's repo calls with a few lines and no
inputs. For each crates.io version the repo releases, it builds every binary of that
crate for x86_64 and aarch64 Linux (gnu), macOS, and Windows on GitHub-hosted runners,
attests each with `actions/attest`, and attaches the archives to that version's GitHub
release under binstall's default file names. `cargo binstall` and crabx then run them.

## Plan

Decided in [the epic](/quest/m2/crabx/README.md#plan): author CI, crates.io versions
only, gnu Linux through cargo-zigbuild against a glibc floor such as 2.28.

Recommendations:

- One publishing job, run on release tags (`v{version}` and `{name}-v{version}`), on each
  push to the default branch, on a schedule, and on `workflow_dispatch`. Each run backfills
  each crates.io version of the repo's binary crates, published since the workflow was
  added, whose release lacks our assets.
  Tags that release-plz or release-please push with the default `GITHUB_TOKEN` start no
  workflow, so the push, schedule, and dispatch triggers cover them. One gap remains:
  GitHub disables a public repo's schedule after 60 days without activity, so a version
  published that way after such a lull waits for the next push or a dispatch.
- Build the commit cargo recorded in the published crate's `.cargo_vcs_info.json`, with
  `--locked`, so an untagged `cargo publish` builds too. Fail loud when that file is
  missing or marks the tree dirty. Attach to the release of the version's existing tag
  (`v{version}` or `{name}-v{version}`), creating the release when the tag has none; fail
  loud when that tag points at another commit; create the tag `{name}-v{version}` only
  when none exists.
- Serialize publishing per crate and version (a concurrency group keyed on both, since a
  tag run and a push run don't share a ref), and make release creation and uploads
  idempotent.
- Native runners per target (`ubuntu-24.04`, `ubuntu-24.04-arm`, `macos-15` with x86_64
  cross-compiled, `windows-latest`, `windows-11-arm`), since crates with native
  dependencies break a single cross-compiling runner.
- Keep PR CI read-only: only the publishing job gets `contents: write`,
  `id-token: write`, and `attestations: write`, on every trigger it serves.
- Release the reusable workflow itself under tags (`pipeline-v{version}`), so a caller
  pins a release; [`crabx init`](/quest/m2/crabx/init.md) writes that pin.
- Prove it on a fixture crate in this repo; crabx is its first real caller when
  [shipped](/quest/m2/crabx/ship.md).
