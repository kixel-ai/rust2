# [S] Lighter pre-push check once hosted CI holds

## Goal

AGENTS.md's pre-push rule moves from `just ci` to `just check` plus the tests a change
touches, `sh/dispatch.sh ci` on its own diff, so an agent stops paying for the whole suite
before every push, while PR CI still runs it.

## Plan

Decided 2026-11-23 (maintainer, /quest-run): ✅ switch only once the PR CI
[#991](https://github.com/kixel-ai/rust2/pull/991) added, `CI / Build` on GitHub-hosted
runners, runs reliably (rejected: switching now; keeping `just ci` for good).
[#998](https://github.com/kixel-ai/rust2/pull/998), which cut the suite's work, left the
switch out: by then the hosted record was a few hours long, with one merge group, and the
`ffi` job's time ranged from 17 to 34 minutes.

- Judge reliability from `gh run list --workflow ci.yml` over PRs and merge groups since
  #991 landed: every failure must trace to the change under test, none to the runner,
  the cache, or a timeout.
- The switch edits AGENTS.md's Development section and CONTRIBUTING.md's, if it repeats
  the rule.

## Scheduled

- 2026-12-01 - a week of hosted runs to judge
