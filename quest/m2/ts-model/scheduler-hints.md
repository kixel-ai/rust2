# [XS] Scheduler hints

## Goal

[rust2-ts](/ts/README.md) translates `thread::yield_now` and `hint::spin_loop` to nothing,
as neither promises anything observable, proven by a differential test against native
Rust. At the moq pin `model/frame.rs`'s `attach` refuses on `yield_now`.

## Plan

A spin loop that waits on another thread hangs natively on one thread too, so translating
the hint to nothing matches Rust.

Public API: the translator's output.
