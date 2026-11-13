# [M] A cheaper woken read in Swift and Go

## Goal

A [rust2-ffi](/ffi/README.md) Swift or Go read that suspends and is woken costs close to
Kotlin's and Dart's. [The binding benchmark](/bench/ffi/README.md) measures 5.1 us wall in
Go and 5.7 us wall (16.6 us CPU) in Swift, against Kotlin's 1.6 and Dart's 1.8.

## Plan

- Measure first: add a bench case where one long-lived reader loops on a track, woken by
  each write, in every language. Today's case starts a `Task` or goroutine per read, so it
  measures the start as much as the wake.
- If the loop case is already close to Kotlin's and Dart's, the task start is the cost and
  is the language's own: record that in the bench README and delete this quest.
- Otherwise fix what the loop case shows, such as Swift resuming its continuation on
  another executor thread or Go's waker crossing threads through a channel per wake.
