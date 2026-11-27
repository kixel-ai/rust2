# [M] Pick a test runner that bounds hangs without a process per test

## Goal

A report, in this quest's PR, that recommends one way to run Rust tests in rind, moq, and here. It must bound each hung test and name it, run tests in parallel, and stay cheap on emulated targets: the iOS simulator, qemu, wasm, and SDE. The report compares the candidates on rind's iOS simulator, qemu, and native rows: wall time, what a hang costs and reports, and what changes in each consumer's recipes and CI. It ends with a plan for adopting the pick in rind and moq, as follow-up quests for their own trees. If a candidate needs new code, such as a watchdog or a harness, a prototype good enough to measure is part of this quest. Shipping it is a follow-up.

## Plan

Decided 2026-11-27 (maintainer, from [kixel-ai/rind#1137](https://github.com/kixel-ai/rind/pull/1137)): ✅ investigate a better test runner. That PR moved rind's CI from `cargo test` to `cargo nextest run` to bound hung tests. nextest runs every test in its own process, so rind's nightly iOS simulator job went from 32 to 70 minutes, one `simctl spawn` per test. The maintainer kept nextest on iOS but called that cost gross.

Home: rust2, milestone m2 (research with named consumers: rind and moq). rust2 planned and landed nextest's hang bound in its own workspaces ([kixel-ai/rust2#1056](https://github.com/kixel-ai/rust2/pull/1056), [kixel-ai/rust2#1057](https://github.com/kixel-ai/rust2/pull/1057)), and its ffi nightly runs iOS and Android simulators too. [swole](https://github.com/kixel-ai/swole) was the other candidate home, but its charter is measuring benchmarks, not running tests.

rind's #1137 already routes around nextest in three places, and the recommendation should cover each one or say why it stays:

- admin/x86-cpu keeps one SDE process per test binary, bounded at 45 minutes, since a process per test made rind's library tests over three times slower under SDE.
- admin/coverage keeps `cargo test` under `timeout 2h`, since a process per test writes one profraw file per test.
- Android stays on `cross test` under the step's `timeout-minutes`, since cross's images have no nextest.

Candidates, each judged on the Goal's terms:

- nextest as it is, with a cheaper per-test launch: a target runner that keeps one simulator booted, or anything nextest offers to batch tests per process. First measure how much of the 38 extra minutes is `simctl spawn` itself.
- libtest, one process per binary, with a watchdog that names and kills a hung test. libtest prints each test still running after 60 seconds, so a per-binary `timeout` plus those lines is the baseline. It falls short of the Goal: with tests in parallel, healthy slow tests print the same line as the hung one, and a binary's deadline gives a test that starts late less time. A watchdog that meets the Goal times each test from its own start and names the one it kills.
- A custom harness (`harness = false`, such as libtest-mimic) with a per-test watchdog thread that aborts the process and names the test. This changes every test target's manifest, so weigh that against configless.
- A split: nextest on native rows, where a process costs little and buys isolation, and in-process runs with a watchdog on emulated rows.

Recommendation to start (afk, reversible): measure the cost of each process launch on each row before building anything. If launches dominate, the split is the likely pick, since it keeps nextest's isolation where it is cheap. Doctests are out of scope: every candidate leaves them to `cargo test --doc` under a `timeout`, as rind and rust2 do now.

Measure by dispatching rind's existing workflows on a branch: the iOS simulator nightly, sh/qemu.sh's rows, a native row, and a wasm row and admin/x86-cpu's SDE run, so the pick is checked on every emulated target it must serve. Use the same rows for every candidate, and run each one at least twice to see the noise. A hang test is a scratch test that blocks forever: record when each candidate kills it and what it reports.

