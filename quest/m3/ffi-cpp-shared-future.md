# [S] C++: a move out of a shared copy while its future pends

## Goal

In the generated C++, a move out of the last copy of a shared object succeeds, as Rust's
`clone` then move would, even while a future made by a `&self` call on a copy since
destroyed still pends, instead of aborting with "a handle is borrowed".

## Plan

Found by [#430](https://github.com/moq-dev/rust2/pull/430), whose design this fixes:
types passing its clone guard hold their handle through `std::shared_ptr`, and a move
out of a copy that others share moves out a clone instead. A pending future borrows
Rust's handle without holding a `shared_ptr`, so `use_count` reads 1 and the move takes
the borrowed value, which Rust refuses. Starts once #430 merges, and changes with it if
the maintainer settles its `Arc` decision another way.

Options to weigh: a future holds a `shared_ptr` copy of the handle it borrows, so
`use_count` sees it; or the move asks Rust whether the handle is borrowed and clones if so.
Prove it with a test that destroys the copy a pending future came from, then moves out
of the last copy.

## Required

- [Lifetimes with no undefined behavior](/quest/m1/ffi-export/lifetimes.md) - #430's shared C++ handles land first
