# [S] A pre-push hook runs the diff's CI

## Goal

A `pre-push` git hook runs `just ci` on the push's diff (`sh/dispatch.sh ci`, the same
diff-scoped lint and tests the merge queue runs), so an agent can't push without it and
nothing else needs to run the checks before the queue. Git installs no hooks on clone,
so the repository's normal setup (the dev shell or `just install`) activates it, such as
with `core.hooksPath`, and AGENTS.md keeps `just ci` before every push as the rule for a
checkout where the hook is absent.

## Plan

Decided 2026-11-23 (maintainer, /quest-run): ✅ switch only once the PR CI
[#991](https://github.com/kixel-ai/rust2/pull/991) added, `CI / Build` on GitHub-hosted
runners, runs reliably (rejected: switching now; keeping `just ci` for good).

Decided 2026-11-24 (maintainer, afk, via /quest-run): ✅ wait for a week of hosted
history before switching (rejected: switching on [#998](https://github.com/kixel-ai/rust2/pull/998)'s few hours of history).

Decided 2026-11-27 (maintainer, `/quest-plan`), superseding both: ✅ run the checks
locally before pushing, possibly from a git hook, and trust that they ran; the merge
queue catches what they miss (rejected: waiting on a week of PR CI, which
[#1078](https://github.com/kixel-ai/rust2/pull/1078) deletes). The switch no longer waits
on hosted history.

- `just ci` is already diff-scoped, so the hook automates the current rule rather than
  lightening it. Report its time on a typical push; if it is too slow to run on every
  push, name which modules move to the queue alone, and why the queue catching them is
  enough.
- The change edits AGENTS.md's Development section and CONTRIBUTING.md's, if it repeats
  the rule.
