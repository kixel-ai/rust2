# [S] The web binding reads a typed array as the engine holds it

## Goal

The web binding lowers a typed array argument by what the engine holds of it, as React
Native's JSI module does, so a caller's shadowed `length`, `byteOffset`, or `buffer`, a
relabeled prototype, or a Proxy never changes which bytes Rust reads, and each is a
`TypeError` or reads the array's own bytes.

## Plan

- Found 2026-11-23 in React Native's review: `web.rs`'s lowering of a `Type::List` block
  and `rt.ts`'s `putBytes` read `v.length`, `v.buffer`, and `v.byteOffset` as JavaScript
  properties. A faked one stays inside wasm memory, so it reads the wrong bytes or throws
  a `RangeError`, but is no memory-safety bug.
- The intrinsic getters on `%TypedArray%.prototype` (`length`, `byteOffset`, `buffer`,
  `Symbol.toStringTag`), taken once at load and called on the value, give the engine's
  own; measure them against the property reads on the benchmark's `write` and `payload`
  rows before choosing, since speed ranks above ergonomics.
- `tests/api.ts`'s `lent` then checks the refusal on the web too, and its `engineRead`
  branch goes; `tests/react-native.ts`'s `typed` case moves to `api.ts`.
