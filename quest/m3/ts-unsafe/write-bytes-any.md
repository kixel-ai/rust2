# [S] ptr::write_bytes beyond bytes

## Goal

[rust2-ts](/ts/README.md) translates `ptr::write_bytes` for pointees other than bytes whose
every byte pattern the runtime can build, such as integers, floats, and arrays and tuples of
them (zeroing a `[u32]` is the common case), proven by differential tests against native
Rust. Any other pointee stays refused with a span.

## Plan

Suggested on [#642](https://github.com/moq-dev/rust2/pull/642), which translated
`write_bytes` for `u8` and `MaybeUninit<u8>` pointers through `rs.ptrWriteBytes`. A value
of `n` repeated bytes is the integer or float those bytes spell in little-endian wasm32
order; the zero case can reuse [#639](https://github.com/moq-dev/rust2/pull/639)'s
zero-value code. A pattern that is invalid for the pointee (a `bool` of 2) is UB in Rust and
need not be reproduced.

Decided with the maintainer on 2026-11-07: ✅ m3, in this epic (from #642's follow-ups), ✅
any byte pattern (rejected: zero only). No consumer at the moq pin.

Public API: the translator's output.
