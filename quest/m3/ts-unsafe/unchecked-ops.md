# [XS] Unchecked integer operations

## Goal

[rust2-ts](/ts/README.md) translates the integers' `unchecked_add`, `unchecked_sub`,
`unchecked_mul`, `unchecked_neg`, `unchecked_shl`, `unchecked_shr`,
`unchecked_funnel_shl`, and `unchecked_funnel_shr`, and `NonZero::new_unchecked`, proven
by differential tests against native Rust. Today each is refused as an unsupported call.

## Plan

Each is its checked form where Rust defines it. A violated precondition is UB, so per
[Throw only where Rust panics](/quest/m3/ts-runtime-throws.md) it aborts worded like std's
`unsafe precondition(s) violated` check, as a debug build does. No consumer at the moq pin.

Public API: the translator's output.
