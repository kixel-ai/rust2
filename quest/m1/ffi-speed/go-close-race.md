# [S] Go's Close waits out a one-crossing drop

## Goal

A Go `Close()` that meets another goroutine's one-crossing drop of a checked object returns
only once that drop's outcome stands: nil once the handle is closed, `rt.ErrBorrowed` once
Rust refused the drop and the handle reopened. A `Drop` that calls back into Go and closes
the same object still returns at once.

## Plan

Found in review of [#906](https://github.com/kixel-ai/rust2/pull/906), which fixed the same
gap in Kotlin. In `ffi/runtime/rt.go`, `Handle_.Close`'s slow path returns nil when `Hold`
finds the handle closed, though the closed bit may be the `closed_|closing_` claim of a drop
Rust then refuses, which restores `held_`.

Kotlin tells the drop's own callback from another thread by the closing thread. Go has no
goroutine identity, so the callback needs another mark: such as a flag the cgo callback
entry sets for its goroutine's duration, or the drop's glue passing the handle being dropped
to callbacks it runs. Model it as `checked_closes_refused` in `ffi/tests/hold.rs` models
Kotlin's.
