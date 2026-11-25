# [M] Node's calls cost the bare crossing while JavaScript holds a token

## Goal

A call into the Node binding's addon while JavaScript holds a token Rust may call through
from another thread costs about the bare Node-API crossing, as a call without one does,
and every binding rule ([every binding's](/ffi/README.md#every-binding)) still holds.

## Plan

- Decided 2026-11-24 (maintainer, /quest-run): ✅ keep this quest (rejected: drop it), since speed per call was half of why the report chose the addon.
- The Waiting question reopens that decision; keep the quest until the maintainer answers it.
- [The lowering PR](https://github.com/kixel-ai/rust2/pull/1004) moved argument conversion,
  handle holds, and status checks into each native function. Token-held calls still
  use a worker handoff with a short spin before parking. Measurements live in
  [the binding benchmark](/bench/ffi/README.md#node-api).
- moq's script holds no token: its session's `Waiter.new$` takes a waker, which a wake from
  another thread queues without one. Only `clockDrive`, of moq's API, keeps a closure Rust
  calls.
- A handoff between two cores costs a few cache-line transfers each way, so only a call on
  JavaScript's thread costs the bare crossing. A call is safe there only if nothing it waits
  on calls JavaScript from another thread, which a stock crate's signature does not say:
  `track.is_closed()` may wait on a lock another thread holds while it calls a closure.
  Configless, no annotation may say it either.

## Waiting

- Maintainer: reconsider the earlier keep decision: accept the spinning handoff as the floor
  and delete this quest, or reaffirm keeping it
  for a way to run such a call on JavaScript's thread. No sound static rule is known, and
  moq's script never holds a token. Recommended: delete it. Deferred 2026-11-24 to the
  maintainer's return by [the lowering PR](https://github.com/kixel-ai/rust2/pull/1004)'s
  decisions, so the PR lands with this quest in place; an agent cannot take it, as it
  weighs a cost moq does not pay today against a search with no known answer.

## Related

- [The binding benchmark's Node-API column](/bench/ffi/README.md#node-api) - what it starts from
- [Node's protocol](/quest/m3/node-protocol.md) - the addon's other open work
