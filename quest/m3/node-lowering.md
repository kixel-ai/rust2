# [M] Node's calls cost the bare crossing while JavaScript holds a token

## Goal

A call into the Node binding's addon while JavaScript holds a token Rust may call through
from another thread costs about the bare Node-API crossing, as a call without one does,
and every binding rule ([every binding's](/ffi/README.md#every-binding)) still holds.

## Plan

- Decided 2026-11-24 (maintainer, /quest-run): ✅ keep this quest (rejected: drop it), since speed per call was half of why the report chose the addon.
- [The lowering PR](https://github.com/kixel-ai/rust2/pull/1004) made each native function
  hold, throw, and convert itself: moq's `track.is_closed()` costs 40 ns of CPU against
  wasm32's 58 and the bare crossing's 24 to 30. While a token is held, the call runs on a
  worker thread as before, but the worker and JavaScript's thread spin 20 µs for each other
  before they park: on moq's addon, a pending `clockDrive` holding the token, the same call
  costs 184 to 215 ns of wall time and twice that of CPU, from 2,990 to 8,500 ns of wall
  time and 2,290 to 2,840 of CPU (**fedora**, at a load average of 15 to 30).
- moq's script holds no token: its session's `Waiter.new$` takes a waker, which a wake from
  another thread queues without one. Only `clockDrive`, of moq's API, keeps a closure Rust
  calls.
- A handoff between two cores costs a few cache-line transfers each way, so only a call on
  JavaScript's thread costs the bare crossing. A call is safe there only if nothing it waits
  on calls JavaScript from another thread, which a stock crate's signature does not say:
  `track.is_closed()` may wait on a lock another thread holds while it calls a closure.
  Configless, no annotation may say it either.

## Waiting

- Maintainer: accept the spinning handoff as the floor and delete this quest, or keep it
  for a way to run such a call on JavaScript's thread. No sound static rule is known, and
  moq's script never holds a token. Recommended: delete it.

## Related

- [The binding benchmark's Node-API column](/bench/ffi/README.md#node-api) - what it starts from
- [Node's protocol](/quest/m3/node-protocol.md) - the addon's other open work
