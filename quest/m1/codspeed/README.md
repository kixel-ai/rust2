# No CodSpeed anywhere

## Goal

No CodSpeed service, app, or code remains in any moq-dev or kixel-ai repository: every benchmark counts through rust2-bench and rust2-bench-harness, the CodSpeed GitHub App is uninstalled from both organizations, and rust2-bench drops its CodSpeed compatibility.

Non-goals: what the benchmarks measure beyond [system metrics](/quest/m1/codspeed/sys-metrics.md), which land before the rewrites so each bench is written once, how often they run, and native perf counters, which `rust2-bench run --perf` counts.

The epic's own work, once its children have merged: rust2-bench drops what it keeps for CodSpeed's crates and hooks, `--cfg codspeed`, `CODSPEED_ISOLATED_BENCHMARK`, the `Metadata: ...` dump rule, and the `ADD_DESC` note, and its README and the harness's stop describing them.

## Plan

Decided 2026-11-25 (maintainer, `/quest-plan` in rust2):

- ✅ Remove the service and the code (rejected: the service and app only, leaving CodSpeed's crates and hooks until the harness epic's schedule; drop the work, since kixel-ai/rind#1013 already ends the service). The service ends with [kixel-ai/rind#1013](https://github.com/kixel-ai/rind/pull/1013), the last CodSpeed Action; moq has counted through rust2-bench since [moq-dev/moq#9555](https://github.com/moq-dev/moq/pull/9555).
- ✅ moq's and rind's Criterion benches are rewritten to `#[bench]` (rejected: a Criterion-API count layer in the harness, deleted later by the rewrite; waiting for the m2 harness epic). That is about 25.7k lines over 84 bench files in moq and 9.2k over 33 in rind, counted 2026-11-25.
- ✅ Wall-clock timing ships first, so each bench is written once and Criterion and `codspeed-criterion-compat` leave together (rejected: rewrite now and stop timing until it ships; keep a Criterion copy of each bench for timing, which kixel-ai/kixel.quest#113 rejected). moq's Nightly times every bench by wall clock through Criterion, including the async and `iter_custom` ones counts skip, and rind's native job times its rustcrypto rows against aws-lc-rs and ring. Paired timing moved here from the m2 harness epic as the rewrite's only prerequisite, and shipped in [kixel-ai/rust2#1033](https://github.com/kixel-ai/rust2/pull/1033); perf counters stop blocking it.
- ✅ m1, since moq and rind wait on it once planned (rejected: m2 beside the harness epic, which ranks it with research).
- The rewrites are quests in moq's and rind's own trees, each waiting on paired timing, planned there from this PR's follow-ups.
- Each rewrite is a boundary for its repository's paired timing: a base before it has only Criterion benches and a head after it only `#[bench]` ones. `rust2-bench time <base>` times a base's benchmarks of CodSpeed's crates as none, so the first runs after the rewrite report the head's times alone, with no change; its quest says whether its nightly also keeps Criterion's timing of the base until the base is past it.

## Waiting

- moq-dev/moq: its Criterion benches rewritten to `#[bench]`; check `codspeed-criterion-compat` and `criterion` are gone from `Cargo.lock` on moq's `main`.
- kixel-ai/rind: its Criterion benches rewritten to `#[bench]`; check `codspeed-criterion-compat` and `criterion` are gone from `Cargo.lock` on rind's `main`.

## Related

- [The harness epic](/quest/m2/harness/README.md) - the harness the rewrites move to
