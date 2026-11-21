# [M] A close that waits on its own thread's undecided drop

## Goal

In Go and Kotlin, a `close()` of a checked object, made before Rust commits another
close's one-crossing drop of it from code that drop's crossing runs or waits on, never
waits on itself: a `Drop` of another value that is not `Send`, which the crossing's
drain of what other threads freed runs, and a callback on a value's home while another
thread's close of that value waits for the home.

## Plan

Found in [#942](https://github.com/kixel-ai/rust2/pull/942), whose waiting close waits
until Rust commits or refuses the drop. Both cases run user code on the closing crossing's
thread of execution before its check: `rt::Object::close` drains first, so a borrow a value
freed earlier held ends before the check, and a value that is not `Send` drops on its home,
which runs the whole close there. A second close in either waits on a crossing that cannot
go on meanwhile; before #942 it returned at once.

Recommended: only in the waiting path, cross into Rust to ask whether this thread runs, or
is the home that would run, a close of the handle not yet decided. On a home, run what is
queued for it, as a home waiting on another does, so the close decides; on the closing
thread itself, return at once, as the drop's own close does. Model both in
`ffi/tests/hold.rs` with a flaw each, and test both on Go and Kotlin.
