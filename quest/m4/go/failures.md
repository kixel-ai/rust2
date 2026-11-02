# [XS] Show why a differential test fails

## Goal

A failing `go/tests/fixtures.rs` test prints its assertion's message, such as a
differential test's first 20 differing inputs with their native and Go outcomes, and
`go`'s or the translator's output when either fails.

## Plan

`native` installs a process-wide panic hook, once, that records each panic's message for
`Outcome::Panic` and prints nothing. It stays installed for every later panic on every
test thread, so a failing test's own `assert!` is recorded and dropped, and the run
shows only that the test failed and the summary line it printed. Seen while landing
[moq-dev/rust2#431](https://github.com/moq-dev/rust2/pull/431).

Silence only the panics `native` catches: set a thread-local flag around its
`catch_unwind`, and have the hook pass any other panic to the hook it replaced. Check it
by breaking a translation by hand: its differential test prints the differences.

If the maintainer drops the translator ([speed](/quest/m4/go/speed.md)), this quest goes
with `go/`.
