# [M] A cheaper woken read in Swift and Go

## Goal

A [rust2-ffi](/ffi/README.md) Swift or Go read that suspends and is woken costs close to
Kotlin's and Dart's. [The binding benchmark](/bench/ffi/README.md) measures 5.1 us wall in
Go and 5.7 us wall (16.6 us CPU) in Swift, against Kotlin's 1.6 and Dart's 1.8.

## Plan

- The long-lived reader case now exists in every language as `loop`, with a single
  writer and one permit per frame, plus acknowledgement before advancing groups.
  C, C++, and Rust repeat their existing manual-poll floor, which already creates no tasks.
- [Measurements](/bench/ffi/README.md#long-lived-reader-measurement) on 2026-11-13 show
  Go at 3.71 us wall, Swift at 5.58, Kotlin at 1.94, and Dart at 2.11. Swift's CPU cost
  roughly halves, but its wall cost remains similar. Task creation is not established
  as the sole cause. The host was loaded; the permit handoff is included and Swift/Go
  writes can race the read's first poll.
- Profile the remaining wake and permit costs independently before changing the runtimes.
  Separate actual suspensions from ready reads so a scheduler race cannot look like a
  cheaper wake. Retain stock public API semantics and the caller's executor behavior.
- Build on [async-start](https://github.com/kixel-ai/rust2/pull/741) once it lands:
  `_start` uses no waker, and a pending call still registers one in a second poll.
- Then fix the measured binding cost and compare against Kotlin and Dart again.
  Do not close this quest on the streaming measurement alone.
