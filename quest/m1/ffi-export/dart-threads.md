# [M] Dart: calls into Dart while the isolate waits

## Goal

A trait's method that Rust calls from another thread while the isolate waits in a call
that lends no closure either runs or fails loud, rather than deadlocking.

## Plan

Rust reaches Dart from another thread through the isolate's port, and waits for its event
loop to run the call. Only a call that lends a closure runs on a thread of its own while
the isolate's runs those calls, through `Home::serve` in `ffi/runtime/dart.rs`. Any other
call holds the isolate's thread, so a call into Dart it waits on, through a trait object
Rust holds, never runs.

Open: whether every call on an object holding a Dart implementation should serve, what
that costs per call, and whether `Home::call` can tell the isolate waits and abort instead.
