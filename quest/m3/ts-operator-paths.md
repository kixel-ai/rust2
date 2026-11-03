# [S] Operator trait methods called by path

## Goal

[rust2-ts](/ts/README.md) translates each `std::ops` operator trait's method called by its
path, such as `<u8 as AddAssign>::add_assign(&mut n, 1)` or `Add::add(a, b)`, wherever its
operator form translates, as a written call and as a function value
(`apply(AddAssign::add_assign, &mut n, 1)`). That covers every operator trait: the
arithmetic, bitwise, and shift ones, their `*Assign` forms, `Neg`, and `Not`. A differential
test against native Rust proves each call means what its operator does, overflow checks
and integer widths included.

## Plan

Found on 2026-11-02 completing [moq-dev/rust2#470](https://github.com/moq-dev/rust2/pull/470):
`<u8 as AddAssign>::add_assign` called by path is an "unsupported call", written or as a value,
while `n += 1` translates. rustc lowers the operator on a primitive to a MIR binary op, but
a call by path stays a call to the trait method, which no mapping covers.

Lowering std's primitive impls from their MIR may cover most of these, since their bodies are
the operator itself. Measure that against a mapping per trait.

Public API: the translator's output.
