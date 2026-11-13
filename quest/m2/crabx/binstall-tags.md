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

## Waiting

- cargo-bins/cargo-binstall: maintainers answer #2749. If they accept, send the PR; once
  a release carries it, bump binstalk in crabx and drop the patch. Check the issue.
