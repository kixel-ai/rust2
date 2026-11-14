# [M] A Dart program releases a returned waker without the garbage collector

## Goal

A Dart program can release a Rust waker returned to it, today a plain function, without
waiting for the garbage collector to finalize it. A program that closes each waker it gets
from Rust exits once its `main` returns, rather than about a minute later at idle GC; an
unclosed one still holds the isolate until finalization, like any other unclosed value
from Rust.

## Plan

Found by [the Dart api script exits when its transcript ends](https://github.com/kixel-ai/rust2/pull/800):
`tests/api.dart`'s waiter section holds clones of a waker made by `wakerOf`, whose token
keeps the isolate alive (`_hold` in `runtime/rt.dart`) until the last clone drops. The
script exits explicitly to avoid the wait.

Decided (maintainer, 2026-11-14): ✅ keep this quest (rejected: drop it); ✅ give a
returned waker a `close`, as every other Dart value from Rust has, matching the binding's
ownership model (rejected: when the waker Rust returns is the isolate's own, `dart.rs`'s
`Wake`, count Dart-held clones out of the keep-alive).
Then have `tests/api.dart`'s waiter section keep and close the waker `waiter.waker()`
returns, rather than calling it inline and dropping it, and drop the explicit `exit` from
its `main`.
