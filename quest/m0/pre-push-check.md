# [S] Lighter pre-push check

## Goal

AGENTS.md's pre-push rule moves from `just ci` to `just check` plus the tests a change
touches, `sh/dispatch.sh ci` on its own diff, so an agent stops paying for the whole suite
before every push, while the merge queue still runs it.

## Plan

Decided 2026-11-23 (maintainer, /quest-run): ✅ switch only once the PR CI
[#991](https://github.com/kixel-ai/rust2/pull/991) added, `CI / Build` on GitHub-hosted
runners, runs reliably (rejected: switching now; keeping `just ci` for good).

Decided 2026-11-24 (maintainer, afk, via /quest-run): ✅ wait for a week of hosted
history before switching (rejected: switching on [#998](https://github.com/kixel-ai/rust2/pull/998)'s few hours of history).

Decided 2026-11-27 (maintainer, `/quest-plan`), superseding both: ✅ run the checks
locally before pushing, possibly from a git hook, and trust that they ran; the merge
queue catches what they miss (rejected: waiting on a week of PR CI, which
[the queue-only quest](/quest/m0/ci-queue-only.md) deletes). The switch no longer waits
on hosted history.

- Recommended: a `pre-push` git hook that runs `sh/dispatch.sh ci` on the push's diff,
  so the rule holds without an agent remembering it. Weigh its time on a typical push
  against the queue failures it saves.
- The switch edits AGENTS.md's Development section and CONTRIBUTING.md's, if it repeats
  the rule.
