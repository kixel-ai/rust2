# [M] A WASI target

## Goal

A report, then a target if the report earns it: [rust2-ffi](/ffi/README.md)'s export of
the `api` fixture built for WASI and called from a host with no JavaScript, with the host
supplying the clock, randomness, files, and sockets. The report runs the fixture's script
under wasmtime two ways, as a core module over the exporter's C ABI and as a component
whose WIT world the exporter generates from the `pub` API, and says which a WASI target
should be, what each costs a call, and which of the exporter's rules each can keep.

## Plan

Decided with the maintainer on 2026-11-09: ✅ web and WASI are separate targets.
[The web target](/quest/m2/wasm/backend.md) calls web APIs directly; this one is for
hosts that are WASI runtimes. Planned in m3 before a consumer asks, as
[m3](/quest/m3/README.md#plan) records for the other targets. Rejected: one
`wasm32-wasip1` module serving both.

- Who would use it: a host language that embeds a WASI runtime instead of linking native
  code, which is how [the pure-Go epic](/quest/m4/go/README.md) says to weigh
  [wazero](https://wazero.io) before finishing a translation, and plugin hosts that load
  untrusted modules.
- A core module keeps the C ABI as it is, so each host language needs a binding over its
  runtime's API, as each native language has one over C today.
- A component replaces that with the Component Model's own interface: the exporter emits
  WIT, and the host's bindings come from its runtime's generator. The audit's survey
  found WASI 0.3 released on 2026-06-11 with native async, a sync call costing about 3.5
  times a plain one for now, and no browser engine running components, which is why the
  web target does not use them. `wasm32-wasip3` is Tier 2 since Rust 1.100, with a
  prebuilt std, so the component builds for it directly. Check what WIT cannot say that the C ABI does: a borrow
  checked at run time, a lent closure, a future that loses nothing on cancel, and bytes
  lent without a copy.
- What a WASI host lacks is refused at export, at the span of the call that reaches it,
  as on every target: threads on `wasm32-wasip1`, and sockets there too.

Public API: the name of the WASI target and what it writes, if it lands.

## Required

- [Exporter](/quest/m1/ffi-export/README.md) - the C ABI, its lossless types, and its checks are settled before a new target copies them
- [The exporter's web target](/quest/m2/wasm/backend.md) - the wasm32 layouts, lent bytes, and single-thread rules this shares

## Related

- [moq in pure Go](/quest/m4/go/README.md) - parked until a consumer needs pure Go, and to weigh a WASI runtime first when one does
