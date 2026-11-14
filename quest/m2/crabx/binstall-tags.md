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

Upstream accepted the idea on #2749, and
[cargo-bins/cargo-binstall#2752](https://github.com/cargo-bins/cargo-binstall/pull/2752)
adds both templates. Codeberg's arm reuses `GITHUB_RELEASE_PATHS`, so it gets them with no
code change; the 2026-11-14 update merged upstream's `main` and regenerated SUPPORT.md
with `just update-docs`, so both its GitHub and Codeberg lists show the templates. Its
CI on that head waits for a cargo-bins maintainer to approve the fork's run.

## Waiting

- cargo-bins/cargo-binstall: merge
  [#2752](https://github.com/cargo-bins/cargo-binstall/pull/2752) and ship a release
  carrying it; then bump binstalk in crabx and drop the patch. Check the PR and the
  releases page.
- Maintainer: answer NobodyXu's review on #2752 (Codeberg reuses `GITHUB_RELEASE_PATHS`,
  and the regenerated SUPPORT.md now lists the templates under Codeberg). Agents don't
  post upstream; check that the PR thread has the reply.
