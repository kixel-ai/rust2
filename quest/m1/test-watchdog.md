# [M] Ship the test watchdog for rows where a process per test is dear

## Goal

A released tool that runs a command running libtest binaries, such as `cargo test` or
`cross test`, and kills it when one test runs past its bound, naming that test, so rind and
moq can bound hangs on the rows where nextest's process per test costs too much: rind's iOS
simulator, Intel SDE, coverage, and cross's Android images, and moq's adb device runs. The
rows where a launch is cheap stay on nextest. Each consumer's adoption is its own tree's
follow-up; this quest ships the tool and proves it on one of rind's rows.

## Plan

Decided 2026-11-27 by the maintainer on [kixel-ai/rust2#1070](https://github.com/kixel-ai/rust2/pull/1070)'s
report: split the runners. nextest stays on native,
qemu-user, wasmtime, Miri, and valgrind rows; the watchdog takes the rest.

Start from the measured prototype, [sh/test-watchdog.py](https://github.com/kixel-ai/rust2/blob/15a938c0f79a3142f876eec93b47ad31072698f5/sh/test-watchdog.py),
which #1070 removed once measured. It needs no
flag in the test binary: libtest prints `test NAME has been running for over 60 seconds`
for each test 60 s after its own start and `test NAME ... ok` when it ends, so the watchdog
times each test from its own start, and tells a healthy slow test from a hung one. A
binary run on one thread, as wasip2's always is, prints `test NAME ... ` as the test
starts instead. Silence while no test runs is bounded too, naming the binary cargo last
started. On a bound it sends the process group SIGTERM, then SIGKILL after 10 s: `simctl
spawn` forwards SIGTERM to its child in the simulator, but a SIGKILL leaves the hung test
running there. Under cross, the tests run in a container whose PID 1, a shell, ignores
SIGTERM, so the container outlives the kill unless cross starts it with
`CROSS_CONTAINER_OPTS=--init`, whose tini passes the signal on; checked on rind's
x86_64 and aarch64 Android images.

Measured on rind's rows (#1070, two runs each, test phase only):

- iOS simulator, debug and release: nextest 2381 and 2650 s, the watchdog 594 and 561 s.
  `simctl spawn` costs 0.5 to 1.6 s per launch (median), and a launch from inside the
  simulator still 0.16 to 0.25 s, so a nextest runner keeping a helper in the simulator
  would still pay about 4 minutes over the watchdog for rind's 3690 tests.
- SDE (Prescott, rind's library tests): nextest 768 s, the watchdog 256 s on the same CPU.
- Native, qemu-user (riscv64, armv6), and wasmtime: within run-to-run noise, except wasm,
  where nextest is faster (509 and 580 s against 711 and 601 s), since wasip2's libtest
  runs one test at a time.

Shape, decided by the maintainer on #1070 (rejected: the Python script vendored into each
consumer; the script fetched from here at a pinned revision): a Rust binary here, published to crates.io so
`taiki-e/install-action` and the consumers' Nix shells install it, which reads its bounds
from the consumer's `.config/nextest.toml` profile (`slow-timeout`'s period times
`terminate-after`, and overrides whose filters are `test(=NAME)`, `test(/REGEX/)`, or
their `|`), so one file bounds both runners. Any other filter is an error naming it. Unit
tests drive its line parser and a mocked clock; one end-to-end test runs a hanging libtest
binary.

Prove it on rind's iOS nightly row on a scratch branch: the hung test named at its bound,
none left running in the simulator, and the test phase near the prototype's 10 minutes.
