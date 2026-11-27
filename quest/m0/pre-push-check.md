# [S] Lighter pre-push check once hosted CI holds

## Goal

AGENTS.md's pre-push rule moves from `just ci` to `just check` plus the tests a change
touches, `sh/dispatch.sh ci` on its own diff, so an agent stops paying for the whole suite
before every push, while the merge queue's hosted build still runs it before anything lands.

## Plan

Decided 2026-11-23 (maintainer, /quest-run): ✅ switch only once the hosted CI
[#991](https://github.com/kixel-ai/rust2/pull/991) added, `build.yml` on GitHub-hosted
runners, runs reliably (rejected: switching now; keeping `just ci` for good).
[#998](https://github.com/kixel-ai/rust2/pull/998), which cut the suite's work, left the
switch out: by then the hosted record was a few hours long, with one merge group, and the
`ffi` job's time ranged from 17 to 34 minutes.

Decided 2026-11-24 (maintainer, afk, via /quest-run): ✅ wait for a week of hosted
history before switching, as the 2026-12-01 date schedules (rejected: switching on
#998's few hours of history).

Decided 2026-11-27 (maintainer, CI runner policy for every repository): ✅ pull requests
run no CI checks, so `ci.yml` runs only on a push to main and the merge queue is the only
CI a PR gets before it lands. A break the lighter check misses now first shows as a
merge group the queue ejects, not as a red PR before the pick.

- Judge reliability from `gh run list --workflow queue.yml --event merge_group` and
  `gh run list --workflow ci.yml --event push` since #991 landed: every failure must trace
  to the change under test, none to the runner, the cache, or a timeout.
- The switch edits AGENTS.md's Development section and CONTRIBUTING.md's Landing section,
  which repeats the rule.

## Waiting

- Maintainer: with no CI on pull requests, which local check runs before a push?
  - `just check` per push, `just ci` before opening a PR or marking it ready (Recommended) - each push stays cheap, and the full selection still runs once before the queue, as rind does
  - `just check` per push only - cheapest, but every miss costs a merge group the queue ejects
  - keep `just ci` before every push and delete this quest - no queue ejection from a skipped module, at the whole suite's cost per push

## Scheduled

- 2026-12-01 - a week of hosted runs to judge
