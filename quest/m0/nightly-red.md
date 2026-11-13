# [M] The nightly passes again

## Goal

The nightly passes on `main`: its "Export moq-net to C" and "Export to Kotlin on Android"
jobs, which failed in
[run 43246857664](https://github.com/kixel-ai/rust2/actions/runs/43246857664) at `53d822ed`
on 2026-11-12, after
[run 43038758163](https://github.com/kixel-ai/rust2/actions/runs/43038758163) passed at
`936ce441` the day before.

## Plan

Found by the 2026-11-12 audit: m0's Goal names a red nightly, and no quest owned this one.
Reproduce each before fixing it, at its root.

- "Export moq-net to C" fails compiling `ffi/moq/moq.swift` under warnings as errors:
  the `Task(executorPreference: self, operation: body)` in `launch` is an unstructured
  throwing task whose result is unused. The job runs on arm64 Linux, where
  [#711](https://github.com/kixel-ai/rust2/pull/711) first ran the Swift, so name #711
  under `## Caused by`.
- "Export to Kotlin on Android" fails `api_android` at `ffi/tests/api.kt`'s `budgeted`:
  "the GC frees bytes lent once the budget is free". The ten commits between the two runs
  include [#700](https://github.com/kixel-ai/rust2/pull/700) and
  [#708](https://github.com/kixel-ai/rust2/pull/708); bisect on the emulator, and check
  whether it fails every run before calling it a regression, since
  [#693](https://github.com/kixel-ai/rust2/pull/693) last fixed this test's lent bytes on
  ART. A test that depends on when the GC runs is fixed by making the collection
  deterministic, not by a retry.

## Closes

- [#716](https://github.com/kixel-ai/rust2/issues/716) - the nightly's failure issue
