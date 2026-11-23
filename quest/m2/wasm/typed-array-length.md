# [S] The web binding sizes bytes by the engine's length, not a getter's

## Goal

A typed array the web binding lowers, such as `Uint8Array` bytes, a `[u8; N]`, an address's
octets, or a block list, is sized by the length the engine keeps for it, never by a
`length` or `byteLength` getter that a subclass can override. A subclass whose getter lies
crosses as its real bytes or throws a `RangeError`, and never makes the binding write past
the buffer it allocated.

## Plan

`putBytes` and `copiedBytes` in `ffi/runtime/rt.ts` allocate `data.length` bytes, then
`mem().set(data, p)` copies the array's real length. So
`class Liar extends Uint8Array { get length() { return 1; } }` passed as 64 bytes writes 63
bytes past the allocation; on a heap buffer that corrupts Rust's heap. `arrayPut` and
`putOctets` check `data.length` against the size Rust expects, and then copy the real length.
Block lists already build the copy from `v.buffer`, `v.byteOffset`, and the length they read
once, which throws past the buffer, so they cannot overrun.

Recommended: when the runtime loads, capture `%TypedArray%.prototype`'s `length` getter, as
`Object.getOwnPropertyDescriptor(Object.getPrototypeOf(Uint8Array.prototype), "length")!.get!`.
Read every lowered array's length through it. Add a check to `tests/api.ts` that passes a
lying subclass to `len`, `Digest.flipped`, and `sameIpAddr`.
