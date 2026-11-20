# [M] Kotlin: a closure's refused value throws through a call that writes an argument back

## Goal

A Kotlin call that writes an argument back, through a `Mut` or a value's `&mut self` handed back,
throws `IllegalArgumentException` when a closure or callback it reached returns a value its
Rust type cannot hold, as every other call does, instead of aborting. Before it throws, the
`Mut` holds what Rust wrote into it before the closure failed, as a Rust caller catching the
closure's panic would see.

## Plan

Found by the local review of [kixel-ai/rust2#922](https://github.com/kixel-ai/rust2/pull/922), which made a closure's refused
value unwind the Rust frames to the native method below (`rt::entry`, `ffi/runtime/jni.rs`).
A `&mut` argument crosses copy-in, copy-out: the C glue (`ffi/src/glue.rs`) writes it to its
out pointer after the call (`rt::write(out, || a0)`), and the JNI glue hands it to Kotlin's
`__return` factory, which sets each `Mut`. Unwinding skips both, so #922 leaves such a
native method on `rt::writing`, which aborts describing the exception.

Recommended: write the copy-out from a drop guard in the C glue, holding the argument in
`ManuallyDrop` behind a raw pointer the call reborrows, so it writes on unwinding too; then
have the JNI entry catch the unwind, encode the outs, and call a write-back-only Kotlin
factory before throwing. A value's `&mut self` handed back has no Kotlin place to write, so it
keeps aborting unless a `Mut` receiver is added. Prove it with a fixture function that bumps a
`&mut u32` and then calls a closure, checked against native Rust catching the panic.

Public API: none. A behavior change: an abort becomes an exception.
