# [S] Vec::from_raw_parts of a typed allocation

## Goal

[rust2-ts](/ts/README.md) translates `Vec::from_raw_parts` of memory `alloc` gave for a
`Layout::array::<T>(n)` (or an equal layout), so the `Vec` adopts that JS array of `T`s
with its length and capacity and drops its elements and frees it as std does, proven by
differential tests against native Rust that check length, capacity, and drops. Today it
is an unsupported call.

## Plan

Typed allocation is a fresh JS array of `T`s since the 2026-11-01 decision in
[the epic](/quest/m3/ts-unsafe/README.md), and `rs.Vec` holds std's capacity since
[#555](https://github.com/moq-dev/rust2/pull/555). smallvec spills its inline buffer this
way: 2 of its 10 model refusals at the moq pin, which moq drops or binds.

Public API: the translator's output.
