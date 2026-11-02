# [M] Dart: calls into a held implementation while the isolate waits

## Goal

A trait's implementation that Rust holds, and calls from another thread while the isolate
waits in a call that neither lends a closure nor gives an implementation, either runs or
fails loud, rather than deadlocking.

## Plan

Rust reaches Dart from another thread through the isolate's port, and waits for its event
loop to run the call. A call that lends a closure or gives an implementation runs on a
thread of its own while the isolate's runs those calls, through `Home::serve` in
`ffi/runtime/dart.rs`. Any other call holds the isolate's thread, so a call into Dart it
waits on, through an implementation Rust kept from an earlier call, never runs. moq gives
no implementation today.

Open: whether every call on an object holding a Dart implementation should serve, what
that costs per call, and whether `Home::call` can tell the isolate waits and abort instead.
