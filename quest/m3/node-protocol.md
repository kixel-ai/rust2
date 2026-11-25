# [M] Node's binding passes the ownership protocol

## Goal

The api fixture's ownership protocol, the 500 scripts `api_web_protocol` runs through the
web's binding, gives the model's transcript through the Node binding's addon on Node and
Bun, every run, checked by a test beside `api_node`.

## Plan

- Measured 2026-11-24 on **fedora** by [the lowering PR](https://github.com/kixel-ai/rust2/pull/1004),
  running `tests/protocol.ts` from beside the fixture's `node` directory, its imports
  rewritten to `./node/` and its load to `api.load("node/api.node")`: origin/main's
  addon parted from the model on 28 to 31 of the 500 scripts and the lowered one on 12 to
  18, and 10 to 39 scripts gave a different transcript on a second run of the same build.
  Seed 34 parts every run: step 77, `drop 14` after `local_later 19 14`, throws "a handle
  is already borrowed" where the model's drop succeeds, since the `later` future still
  holds its `borrow_mut_arc` of the `Local`.
- The script assumes a future progresses only as it awaits, as on the web, where the
  module polls its futures on microtasks. The addon polls a woken future from its event
  loop's queue, which a thread-safe function tells, so how far each future got by a step
  depends on when the event loop served it. Find what the protocol needs of Node's
  progress, such as the addon polling woken futures on a microtask as the web's does, or
  `drain` and `await` yielding until the queue is empty, and keep it out of the generated
  binding's hot path.

## Related

- [Node's lowering](/quest/m3/node-lowering.md) - the addon's other open work
