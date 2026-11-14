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
code change; the 2026-11-14 update merged upstream's `main` and regenerated SUPPORT.md
with `just update-docs`, so both its GitHub and Codeberg lists show the templates. Its
CI on that head waits for a cargo-bins maintainer to approve the fork's run.

## Waiting

- cargo-bins/cargo-binstall: merge
  [#2752](https://github.com/cargo-bins/cargo-binstall/pull/2752) and ship a release
  carrying it; then bump binstalk in crabx and drop the patch. Check the PR and the
  releases page.
- Maintainer: answer NobodyXu's review on #2752 (recommended reply: Codeberg's arm reuses
  `GITHUB_RELEASE_PATHS`, and the regenerated SUPPORT.md lists both templates under
  Codeberg), and in the same reply ask a cargo-bins maintainer to approve the fork's CI
  run on the head. An agent can't take this: posting outside the maintainer's orgs needs
  the maintainer, and only cargo-bins can approve the run. Check that the thread has the
  reply and the head's CI has run.
