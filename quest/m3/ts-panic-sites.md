# [M] Every panic of a hooked program has its location

## Goal

In a program that reaches `set_hook`, [rust2-ts](/ts/README.md) gives every panic the
`Location` native code gives it, so none throws the runtime's "a panic its hook can't
locate" error, proven by the panics fixture's hooked cases.

## Plan

The panics that carry none yet, each a std function whose panic is in std's own code that
neither tracks its caller nor sits in a macro the translator locates:

- `Bytes` and `BytesMut`'s, at a path under the native build's cargo registry.
- `Duration`'s `Sum`, defined by `sum_durations!`.
- An integer range's `Step::forward`, in `step_integer_impls!`.
- An atomic's load or store with an ordering it refuses, through the generic `Atomic<T>`.
- A slice's `join` and `concat` whose length overflows.
- std's `#[track_caller]` functions that lower from MIR, such as `char::to_digit` with a
  radix past 36.

A crate's own `#[track_caller]` function that the runtime calls, through std's `Fn` impls,
stays refused in every program: its location is in std's source.
