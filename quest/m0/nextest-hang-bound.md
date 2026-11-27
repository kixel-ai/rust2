# [S] Bound hung tests with nextest

## Goal

A deadlocked test in any of rust2's Rust workspaces (`ts`, `ffi`, `go`, `driver`, and the benchmark runner) ends after a bounded time instead of holding a run for as long as it likes. Each workspace's test recipe and CI run `cargo nextest run`, with doctests under `timeout <bound> cargo test --doc` where it has a library, and each has a `.config/nextest.toml` with a hang bound. Once the config exists, kixel-ai/quest's `quest hook` denies plain `cargo test` there and points agents at the recipe.

## Plan

Decided 2026-11-26 (maintainer, /quest-run in kixel-ai/quest, from [kixel-ai/quest#924](https://github.com/kixel-ai/quest/pull/924)): ✅ keep it for rust2. On 2026-11-25 two deadlocked tests in moq ran 40 and 49 minutes under plain `cargo test`, which has no hang bound, and rust2's recipes run plain `cargo test --locked` (rejected: leaving rust2 on `cargo test`; dropping the quest).

- Use moq's bound as the start: `slow-timeout = { period = "30s", terminate-after = 2 }`. Then check it against rust2's slowest legitimate tests, the exporter's fixture and corpus runs among them, and lengthen the period or add a per-test override where a healthy test would be killed, never a retry. The bound is for the gate; the nightly's long runs may need their own profile.
- nextest does not run doctests, so `slow-timeout` does not reach `cargo test --doc`. Bound that step with coreutils `timeout` set to a generous multiple of the slowest healthy doctest run (a failure exits 124), so the Goal holds for doctests too. First check which named workspaces have any doctest, such as the code block in `ffi/fixtures/api/src/lib.rs`, and skip the step where none does.
- The nightly recipes in `ffi/justfile` run `cargo test ... -- --ignored --exact`; under nextest that is `--run-ignored only` with the test filters. The `fixtures` binary runs its tests in one process, and nextest runs each in its own, so check that none shares state through the process.
- nextest is in the default dev shell already; the other shells inherit it. Confirm each recipe the nightly and `just ci` reach, in `sh/dispatch.sh` too, goes through it, and that the `ffi` recipes that build outside the default shell find it.
- Nothing is in production, so change the recipes directly. Run `just ci` before pushing.
