# [M] Every panic of a hooked program has its location

## Goal

In a program that reaches `set_hook`, [rust2-ts](/ts/README.md) gives every panic the
`Location` native code gives it, so none throws the runtime's "a panic its hook can't
locate" error, proven by the panics fixture's hooked cases.

## Plan

The panics that carry none yet, each in std's own code that neither tracks its caller
nor sits in a macro the translator locates, found by running every `lang` fixture as a
hook program:

- An integer `Sum` or `Product` that overflows, defined by `integer_sum_product!` (slices,
  iters).
- `Duration`'s `Sum`, defined by `sum_durations!` (durations).
- An integer range's `Step::forward` overflow, in `step_integer_impls!` (iters).
- A slice's `join` and `concat` whose length overflows (joins).
- `Vec::extract_if` with a range out of bounds (edits).
- `RefCell`'s comparisons of one already mutably borrowed (collections).
- `Bytes` and `BytesMut`'s, at a path under the native build's cargo registry.

A crate's own `#[track_caller]` function that the runtime calls, through std's `Fn` impls,
stays refused in every program: its location is in std's source.
