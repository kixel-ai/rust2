# [L] PR CI and the merge queue's build on GitHub-hosted runners

## Goal

Every PR into `main` gets a read-only CI run on GitHub-hosted runners, and the merge queue's
`Queue build / Build` runs there too, both through one reusable workflow that runs
`sh/dispatch.sh`'s selected modules, so neither loads the host the agents work on. Both
pass on the current tree, including every TS fixture.

## Plan

Planned from [#960](https://github.com/kixel-ai/rust2/pull/960), a draft that did this and
the [test-cost cuts](/quest/m0/ci-test-cost.md) in one PR. Decided 2026-11-23 (maintainer,
/quest-run): ✅ split #960 into quests (rejected: taking it over as one PR; leaving it).
rust2 is public, so the maintainer's rule that private repositories use self-hosted runners
does not apply.

#960's hosted run took about 13m47s elapsed and 53.5 job-minutes, with 86 TS fixtures
failing: hosted runners are feasible, not yet efficient. Find why those fixtures fail there
and fix it at the cause; the gate's result must not depend on its runner.

- One reusable workflow holds the build; queue.yml and a PR workflow call it. #960 copied
  about 250 lines of sharded jobs into both `ci.yml` and `queue-build.yml`, which drift.
  The modules and their selection stay `sh/dispatch.sh`'s, read with its `--query`.
- A module gets its own job, or a shard, only where a measured run shows it is the critical
  path; the rest share one job. Say each shard's measured time in the PR.
- One cache mechanism, not hand-keyed `actions/cache` steps per shard beside a Nix cache, as
  #960 had. Recommended: mbx, already the queue's Cargo cache, on a backend hosted runners
  reach, plus the Nix store's; measure warm and cold. Only main's runs write a cache that a
  merge group reads, keeping today's rule that nothing unreviewed writes the queue's store.
- The PR workflow keeps a read-only `GITHUB_TOKEN` and no secrets. `main`'s ruleset
  requires `Queue build / Build`; keep that name, or change the requirement in dotfiles'
  `bin/repo-settings`.
- queue-build.yml's comment and queue.yml's explain the self-hosted runner group that only
  admits queue-build.yml at main; rewrite them, and drop the main-only call if hosted
  runners make it moot. Once the queue runs green on hosted runners, the host's
  `rust2-queue` runner can retire; say so in the PR for the maintainer.
- [The cached export](/quest/m0/ffi-export-cache.md) measures the queue's `ffi` time across
  merge groups; note the runner change there so its before and after compare one runner.

Dropped from #960, and why:

- `[profile.dev] opt-level = 1` in `ffi/` and `ts/`: superseded for the exporter by
  [#971](https://github.com/kixel-ai/rust2/pull/971), which optimizes `rust2-ffi` and
  `rust2-driver` alone. Workspace-wide, it also optimizes every dependency, which was never
  measured.
- `.github/` changes as lint-only: actionlint shows a workflow parses, not that its build
  commands still work, so a workflow change keeps running the modules it can break.

Decided 2026-11-23 (maintainer, /quest-run): ✅ keep this quest (rejected: dropping it).

## Related

- [Measured test work cut](/quest/m0/ci-test-cost.md) - the rest of #960
