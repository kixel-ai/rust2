# [S] A bisect step counts the named benchmarks a commit has

## Goal

`rust2-bench run` takes an opt-in flag with `--benchmarks` that counts the named benchmarks the commit has and reports each missing one, on stderr and in the job summary, failing only when none of the names exist.
Without it, `--benchmarks` stays strict: a name no benchmark has fails the run.
rust2's own Valgrind workflow passes the flag on `workflow_dispatch`, so a Kixel Quest bisect step keeps every suspect it can count.

## Plan

Found in [moq-dev/moq#9555](https://github.com/moq-dev/moq/pull/9555)'s review: Kixel Quest bisects several suspects in one step, so a step at a commit before one suspect was added, or after a rename, fails whole and loses the others' counts. Kixel Quest already treats a measurement missing from `metrics.json` as unmeasured, not clean, so the strictness's purpose, never reading a suspect as clean, holds without failing the run.

Decided 2026-11-24 (maintainer, `/quest-plan`): ✅ an opt-in flag the bisect workflows pass (rejected: make tolerance the default, which lets a local typo among several names pass with only a warning). m0, since moq's and rind's nightly bisects run on rust2-bench now.

Recommendations for the implementer:

- Check in kixel-ai/kixel.quest that a suspect absent from a step's `metrics.json` is never read as clean, and cite where in the PR.
- moq-dev/moq's `.github/workflows/codspeed.yml` and kixel-ai/rind's `.github/workflows/bench.yml` pass `--benchmarks` on dispatch; add a follow-up quest in each repository's tree to pass the flag once this lands, under the PR's `## Follow-ups`.

Public API: a new `run` flag. Wire: none.
