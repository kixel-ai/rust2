# [M] React Native's runtime holds Rust's memory against reentrant and replaced JavaScript

## Goal

Each of the four ways a Codex review of [the Node addon's PR](https://github.com/kixel-ai/rust2/pull/980)
found for JavaScript to reach memory React Native's runtime frees, or to deadlock it, is
reproduced by a case of `tests/react-native.ts` on `just ffi react-native-host`, then
fixed at its cause, or refuted with the case that shows it cannot happen.

## Plan

- Decided 2026-11-24 (maintainer, /quest-run): ✅ keep this quest (rejected: drop it), since three of the four findings would be use-after-free or deadlock if real.

The findings, as the review reported them on 2026-11-24, against what
[kixel-ai/rust2#972](https://github.com/kixel-ai/rust2/pull/972) landed:

- `ffi/runtime/rt.jsi.hpp`, a handle's disposal: a Rust destructor that calls JavaScript,
  which disposes the same handle, finds it not yet closed, so the inner release frees the
  allocation the outer destructor still uses, and the outer release underflows its count.
  Mark it closed before any callback can run, keeping the allocation alive through the drop.
- `ffi/runtime/rt.jsi.hpp`, the kit's `bytes`: a first install that forges the kit gets the
  Rust-backed `ArrayBuffer`, which it can keep and so read after `$bytesFree` frees it.
  Checking what the kit returns cannot take back what it was given.
- `ffi/runtime/rt.jsi.hpp`, the collector: when Hermes collects a waker never disposed, its
  Rust destructor runs during the sweep with the runtime locked. `log_waker`'s destructor
  joins a thread that calls JavaScript, which waits for the runtime, so the two deadlock.
  Queue a drop the collector makes and run it with the runtime unlocked, `cell`'s and
  `bytes_cell`'s direct frees included.
- `ffi/runtime/rt.rn.ts`, intrinsics: replacing the global `Uint8Array`, or
  `ArrayBuffer.prototype.byteLength`, after load hands caller code the private buffer of
  returned bytes past 64, which a caller could write or keep past their disposal. Use the
  constructor captured at load and an intrinsic length read.

## Related

- [React Native's release](/quest/m3/react-native-release.md) - the binding these hold up
