# [S] Zeroed values beyond one Box

## Goal

[rust2-ts](/ts/README.md) translates `MaybeUninit::zeroed`, `mem::zeroed`, and
`Box::new_zeroed_slice`, proven by differential tests against native Rust. Each builds the
value all-zero bytes give, as `Box::new_zeroed` does.

## Plan

Suggested on [#639](https://github.com/moq-dev/rust2/pull/639), which translated
`Box::new_zeroed` for one value: it builds the zero value part by part through tuples,
arrays, the crate's structs, enums by their zero tag or niche, and std's wrappers (`Wrapping`,
`Cell`, `Mutex`, ...), and leaves a part that can't be zero, such as a reference or a
`String`, unwritten. Reuse that code rather than a second walk. `mem::zeroed` of a type with
a part that can't be zero is UB in Rust; translate it as the same partly unwritten value.

Decided with the maintainer on 2026-11-07: ✅ m3, in this epic (from #639's follow-ups). No
consumer at the moq pin.

Public API: the translator's output.
