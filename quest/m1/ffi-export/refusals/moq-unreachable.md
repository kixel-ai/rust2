# [XS] moq's types are reachable from their crate root

## Goal

The moq pin moves to a revision where every type a facade export names is reachable from
the facade's root, and `just ffi census` there lists no "type not reachable from the crate
root" refusal of moq's own: 18 at moq `ffcb7cd9`.

## Plan

At `ffcb7cd9`, `container::Producer::modify` returns moq-mux's
`container::producer::Guard`, in a private module (16 refusals), and moq-tokio's
`quic::driver` re-exports moq-quic's `stream::Id` but not the `Side` its `new` and
`initiator` take (2). Both are moq's to fix; then bump the pin, as `ts/justfile`'s
`moq_rev` sets it.

## Waiting

- moq-dev/moq: [quest/m2/api-shape/reexports.md](https://github.com/moq-dev/moq/blob/main/quest/m2/api-shape/reexports.md),
  planned in [moq-dev/moq#8233](https://github.com/moq-dev/moq/pull/8233), re-exports
  `Guard` from `container`, and
  [quest/m2/moq-quic-unused-surface.md](https://github.com/moq-dev/moq/blob/main/quest/m2/moq-quic-unused-surface.md)
  removes `stream::Id::{new, initiator}`. The stand-in's narrowing to moq-tokio's session
  side also drops `quic::driver`. Done once both quests are gone from moq's `main`.
