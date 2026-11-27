# rust2-watchdog

Runs `cargo test`, or a tool that takes its arguments such as `cross test`, and kills it when one test runs past its bound, naming that test.
The bounds are the workspace's nextest bounds, so one `.config/nextest.toml` bounds both runners.

```bash
rust2-watchdog cargo test -p rind --target aarch64-apple-ios-sim
CROSS_CONTAINER_OPTS=--init rust2-watchdog --profile emulated cross test --release --target aarch64-linux-android
```

## When to use it

nextest runs each test in its own process.
Where a process launch is dear, such as the iOS simulator (0.5 to 1.6 s per `simctl spawn`), Intel SDE, an Android device or emulator, or coverage's one profile per process, that costs more than the tests: rind's iOS tests ran 42 minutes under nextest and 10 under the watchdog ([kixel-ai/rust2#1070](https://github.com/kixel-ai/rust2/pull/1070)).
The watchdog keeps `cargo test`'s one process per test binary and still bounds each test.
Where a launch is cheap, stay on nextest, which also isolates each test's failure and output.

## How it works

1. It builds the tests once, with `--no-run --message-format=json-render-diagnostics` added to the command, to learn which package built each test executable.
   The build is unbounded, so the bounds cover only the tests.
2. It runs the command as given, with stdout and stderr on one pipe in a process group of their own, and passes the output through.
3. It times each test from libtest's own lines.
   A multithreaded binary prints `test NAME has been running for over 60 seconds` 60 s after a test starts, which starts the test's clock 60 s back, and `test NAME ... ok` when it ends.
   A one-thread binary prints `test NAME ... ` as a test starts.
   Silence while no test is known to run, such as a binary hung before its first test, is bounded by 60 s plus the profile's bound, naming the binary cargo last started: the 60 s lets libtest name a test with a longer bound first.
4. When a bound runs out it names the test and its binary, sends the process group SIGTERM, then SIGKILL after the bound's `grace-period`, and exits 124.
   SIGTERM comes first because `simctl spawn` passes it on to the test in the simulator but dies to a SIGKILL and leaves the test running there.
   Under `cross`, set `CROSS_CONTAINER_OPTS=--init`: the container's first process is otherwise a shell that ignores SIGTERM.

Otherwise it exits with the command's status.
A setup error, or output it cannot time, exits 2.

## Bounds

The profile is `--profile`, else `NEXTEST_PROFILE`, else `default`, as in nextest.
A test's bound is its `slow-timeout`'s `period` times `terminate-after`, from the first override that matches it and sets `slow-timeout` (the profile's own, then the default profile's), else the profile's, else the default profile's.
A `slow-timeout` without `terminate-after` leaves a test unbounded, as in nextest, but the profile's own must have one.

An override's `filter` may combine `package(NAME)` and `test(NAME)` with `&`, `|`, and parentheses, where `NAME` is `=exact`, `~contains`, `/regex/`, or bare (equality for `package`, containment for `test`).
Anything else that could change a bound is an error naming it: another predicate, a glob, a `platform`, `inherits`, or `on-timeout = "pass"`.
So is a bound under 60 s, since a multithreaded binary names a running test no sooner.

## What it loses against nextest

A killed binary's other running tests do not report, and the hung test's captured output is lost, since libtest holds it in the process; rerun the named test under plain `cargo test -- --nocapture` to see it.
Each binary's tests share one process, as under `cargo test`.
`--doc`, `--quiet`, and libtest's `--format` are refused, since they name no test or package the watchdog can time, and so are `--nocapture` and `RUST_TEST_NOCAPTURE`, which let a test's own output pose as libtest's lines.
It runs on Unix only.
