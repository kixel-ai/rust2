# [L] A panic's Location

## Goal

[rust2-ts](/ts/README.md) translates `panic::Location`: `Location::caller` in the crate's
functions, `#[track_caller]` ones included, and `file`, `line`, `column`, `Display`,
`Debug`, and comparisons of what it gives, matching native Rust's values, proven by a
differential test.

## Plan

Split from the PR that translated the rest of `std::panic`, which refuses `Location` with a
span. Natively, `Location::caller` in a function without `#[track_caller]` is a constant of
its call's span, and in one with it, the location its caller passed, so a translation has to
thread that location through each call of such a function, as rustc's codegen does.

Things to settle while building it:

- The file path rustc reports, which must match the native build's.
- Calls that reach a `#[track_caller]` function indirectly: through a function pointer, a
  trait object, or std's generic code calling the crate's trait impl. Refuse what can't
  match, with a span.
- What MIR inlining does to the spans the translator reads.

Public API: the translator's output.

## Related

- [A panic hook](/quest/m3/ts-panic-hook.md) - a hook reads each panic's `Location`
