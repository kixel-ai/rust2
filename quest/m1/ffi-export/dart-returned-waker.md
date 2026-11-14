# [M] A Dart program releases a returned waker without the garbage collector

## Goal

A Rust waker returned to Dart, today a plain function, no longer keeps the isolate alive
until the garbage collector finalizes it. A Dart program whose wakers from Rust are
unreachable exits once its `main` returns, rather than about a minute later at idle GC.

## Plan

Found by [the Dart api script exits when its transcript ends](https://github.com/kixel-ai/rust2/pull/800):
`tests/api.dart`'s waiter section holds clones of a waker made by `wakerOf`, whose token
keeps the isolate alive (`_hold` in `runtime/rt.dart`) until the last clone drops. The
script exits explicitly to avoid the wait.

Options: give a returned waker a `close`, as every other Dart value from Rust has
(recommended, matching the binding's ownership model); or, when the waker Rust returns is
the isolate's own (`dart.rs`'s `Wake`), count Dart-held clones out of the keep-alive.
Then drop the explicit `exit` from `tests/api.dart`'s `main`.
