# [S] The web binding reads a typed array as the engine holds it

## Goal

A typed array the web binding lowers, such as `Uint8Array` bytes, a `[u8; N]`, an address's
octets, or a block list, is read by what the engine keeps for it, as React Native's JSI
module reads one: its length, `byteOffset`, `buffer`, and kind, never a `length`,
`byteLength`, `byteOffset`, `buffer`, or `Symbol.toStringTag` a subclass or the caller
overrides, and never through a Proxy. A value whose properties lie crosses as its real
bytes or throws a `TypeError` or `RangeError`, and never makes the binding write past the
buffer it allocated.

## Plan

`putBytes` and `copiedBytes` in `ffi/runtime/rt.ts` allocate `data.length` bytes, then
`mem().set(data, p)` copies the array's real length. So
`class Liar extends Uint8Array { get length() { return 1; } }` passed as 64 bytes writes 63
bytes past the allocation; on a heap buffer that corrupts Rust's heap. `arrayPut` and
`putOctets` check `data.length` against the size Rust expects, and then copy the real length.
Block lists already build the copy from `v.buffer`, `v.byteOffset`, and the length they read
once, which throws past the buffer, so they cannot overrun, but a faked `buffer` or
`byteOffset` still reads other bytes of wasm memory than the array's.

Recommended: when the runtime loads, capture `%TypedArray%.prototype`'s `length`,
`byteOffset`, `buffer`, and `Symbol.toStringTag` getters, as
`Object.getOwnPropertyDescriptor(Object.getPrototypeOf(Uint8Array.prototype), "length")!.get!`,
and read every lowered array through them; called on a Proxy or any other non-typed-array,
each throws a `TypeError`, which is the refusal. Measure them against the property reads on
the benchmark's `write` and `payload` rows, since speed ranks above ergonomics.

Tests: add a check to `tests/api.ts` that passes a lying subclass to `len`,
`Digest.flipped`, and `sameIpAddr`. Its `lent` case then checks the refusal on the web too,
so its `engineRead` branch goes, and `tests/react-native.ts`'s `typed` case moves to
`api.ts`, running on both.

React Native's review found the `byteOffset`, `buffer`, tag, and Proxy cases on 2026-11-23.
