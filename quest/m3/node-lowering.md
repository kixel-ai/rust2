# [L] Node's addon lowers each call itself

## Goal

The Node binding's call into the addon costs no more than the bare Node-API crossing, so
it beats the web target per call on Node, as [the report](https://github.com/kixel-ai/rust2/pull/970)
found a thin addon does. Each native function lowers its own arguments, checks its
handles' borrows, and throws a failed status, as React Native's JSI module does in C++,
so the TypeScript of a call is one call, and every binding rule
([every binding's](/ffi/README.md#every-binding)) still holds, proved by the api
fixture's transcript and cases on Node, Bun, and Deno.

## Plan

- Measured 2026-11-23 on **fedora**, at a load average near 25: the generated handle call
  `track.is_closed()` costs 65 to 80 ns through the addon, of which the bare native call
  is 30 ns, and 17 ns is any Node-API call's floor (`moq_contract`, no arguments). On the
  wasm32 stand-in the same wrapper costs 38 to 58 ns over a 5 to 8 ns call, so the
  generated TypeScript per call, `rt.enter`, `rt.hold`, the status slot, `rt.status`, and
  `rt.leave`, costs both about 35 to 50 ns, and Node-API's crossing alone is what the
  addon adds. The addon's column on [the benchmark](/bench/ffi/README.md#node-api) has
  the per-case cost.
- [React Native's binding](/ffi/README.md) is the model: its TypeScript calls `rt.n.<fn>`
  and its JSI module lowers, holds, and checks. Here a native function would read a
  handle's borrow state from the addon's table, which already holds each handle, rather
  than from the `rt.Handle`.
- First measure the floor of a native function that takes a handle and throws through
  `napi_throw_error` on a status, against the 30 ns bare call, before generating any.

## Related

- [The binding benchmark's Node-API column](/bench/ffi/README.md#node-api) - what it starts from
