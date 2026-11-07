# [S] Rc and Arc of an uninitialized value

## Goal

[rust2-ts](/ts/README.md) translates `Rc` and `Arc`'s `new_uninit`, `new_zeroed`, their
slice forms (`new_uninit_slice`, `new_zeroed_slice`), and `assume_init`, the way `Box`'s
forms translate, proven by differential tests against native Rust, with strong and weak
counts and drops matching.

## Plan

Suggested on [#639](https://github.com/moq-dev/rust2/pull/639) (`Box::new_uninit`,
`new_zeroed`, `write`, and `assume_init` for one value) and
[#606](https://github.com/moq-dev/rust2/pull/606) (`Box::new_uninit_slice`). These are
stable std APIs, and std gaps are deferred, not dropped. Cover both ways a program writes
the value before `assume_init`: `Rc::get_mut(&mut rc).unwrap().write(v)`, and a write
through `as_ptr`.

Decided with the maintainer on 2026-11-07: ✅ m3, in this epic (from #639's follow-ups). No
consumer at the moq pin.

Public API: the translator's output.
