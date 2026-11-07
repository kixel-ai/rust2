# [XS] NonZero's unchecked arithmetic

## Goal

[rust2-ts](/ts/README.md) translates `NonZero::unchecked_add` and `NonZero::unchecked_mul`,
proven by differential tests against native Rust. Today each is refused by name.

## Plan

Left by [#622](https://github.com/moq-dev/rust2/pull/622), which translated the integers'
`unchecked_*` and `NonZero::new_unchecked` through `rs.precondition`. Both are unstable
(`nonzero_ops`) and not `#[track_caller]`, so a violated precondition aborts with std's
`unsafe precondition(s) violated` text located inside std's source, through `caller.rs`'s
`SITES`, rather than at the call.

Decided with the maintainer on 2026-11-07: ✅ m3, in this epic (rejected: skip). No
consumer at the moq pin.

Public API: the translator's output.
