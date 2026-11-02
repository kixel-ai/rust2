# [M] Dart: calls into a held implementation while the isolate waits

## Goal

A trait's implementation that Rust holds, and calls from another thread while the isolate
waits in a call that neither lends a closure nor gives an implementation, either runs or
fails loud, rather than deadlocking.

## Plan

Decided with the maintainer on 2026-11-02, for moq-dev/rust2#422: the Dart generator lands
serving only a call that lends a closure or gives an implementation, and each poll of its
future, and this quest closes the rest once a consumer gives an implementation. moq gives
none today.

Rust reaches Dart from another thread through the isolate's port, and waits for its event
loop to run the call. A served call runs on a thread of its own while the isolate's runs
those calls, through `Home::serve` in `ffi/runtime/dart.rs`. Any other call holds the
isolate's thread, so a call into Dart it waits on, through an implementation Rust kept
from an earlier call, never runs. Two cases from #422's review:

- a method that joins a thread calling an implementation its object stored;
- `Cancelable.cancel()` freeing a future whose destructor joins a thread calling one.

Dart's API DL has no `Dart_EnterIsolate`, so only the isolate's own thread runs Dart.
Serving every call and poll while the isolate has an implementation alive in Rust fixes
both, at a served thunk and a second `@Native` per C function, about a third more
generated Dart, and a thread handoff per call meanwhile: a spawn, or a persistent worker
per isolate, measured first.

Open: that, or whether `Home::call` can tell the isolate waits and abort instead.
