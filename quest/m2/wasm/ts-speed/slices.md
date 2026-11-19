# [M] Scalar slices of any element

## Goal

`rs.Slice<T>` of any element splits into scalar parts as byte slices already do
(`ts/README.md`), so `split_at_mut` and its kin build no tuple and no slice objects per
step, and `parts/*` on `just ts bench` reports before and after.

## Plan

Found by the 2026-11-18 audit: `parts/*` runs 2.5 to 7.4 times rust2-wasm in Chrome and up
to 50 in Firefox, from a tuple and two `rs.Slice` objects per `split_at_mut`
(`expected/bench.ts:792`). MIR's SROA does not help, since a slice reference is a pointer
and a length, not a struct.

Public API: none.
