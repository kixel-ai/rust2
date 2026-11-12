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

- Trigger on a schedule as well as on release tags (`v{version}` and `{name}-v{version}`).
  Tags that release-plz or release-please push with the default `GITHUB_TOKEN` start no
  workflow, so the scheduled run finds each crates.io version of the repo's binary crates
  whose release lacks our assets and builds it; that also covers a manual
  `cargo publish`. GitHub disables schedules in a public repo after 60 days without
  activity, so a tag push or `workflow_dispatch` still works on its own.
- Build the version's tagged commit with `--locked`, and fail loud when the tag's
  version isn't on crates.io or doesn't match the crate's manifest. Create the release
  when the tag has none.
- Native runners per target (`ubuntu-24.04`, `ubuntu-24.04-arm`, `macos-15` with x86_64
  cross-compiled, `windows-latest`, `windows-11-arm`), since crates with native
  dependencies break a single cross-compiling runner.
- Keep PR CI read-only: only the tag-triggered job gets `contents: write`,
  `id-token: write`, and `attestations: write`.
- Prove it on a fixture crate in this repo; crabx is its first real caller when
  [shipped](/quest/m2/crabx/ship.md).
