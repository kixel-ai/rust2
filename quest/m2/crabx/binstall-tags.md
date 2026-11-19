# [S] binstall guesses name-v-version release tags

## Goal

Stock cargo-binstall finds release assets on `{name}-v{version}` tags, the default of
release-plz, cargo-release, release-please, and cargo-smart-release, so binstall, crgx,
and mise run the pipeline's builds in workspace repos. crabx then drops its patched
binstalk for the released one.

## Plan

Decided with the maintainer on 2026-11-12: ask upstream in an issue rather than open a
PR first, and keep it brief. Asked in
[cargo-bins/cargo-binstall#2749](https://github.com/cargo-bins/cargo-binstall/issues/2749):
add `{ name }-v{ version }` and `{ name }-{ version }` to `GITHUB_RELEASE_PATHS`.
Rejected: changing moq's tags and `repository` URLs; leaving other tools unable to find
these builds.

Upstream accepted the idea on the issue, and
[cargo-bins/cargo-binstall#2752](https://github.com/cargo-bins/cargo-binstall/pull/2752)
adds both templates. Codeberg's arm reuses `GITHUB_RELEASE_PATHS`, so it gets them with no
code change. It merged on 2026-11-15.

## Waiting

- cargo-bins/cargo-binstall: ship a release carrying
  [#2752](https://github.com/cargo-bins/cargo-binstall/pull/2752); then bump binstalk in
  crabx and drop the patch. On 2026-11-18 the latest, v1.27.0 and binstalk v0.30.0, predate
  it. Check the releases page for a newer `v*` and `binstalk-v*` release.
