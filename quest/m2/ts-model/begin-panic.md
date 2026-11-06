# [XS] panic! in a 2015 or 2018 crate

## Goal

[rust2-ts](/ts/README.md) translates `panic!("literal")` in a 2015 or 2018 edition crate,
which expands to `std::rt::begin_panic`, as it does in 2021, proven by a 2018-edition
panics fixture against native Rust. Today arrayvec's is refused.

## Plan

The mapping is keyed by `("rt", "begin_panic")`, but `library::key` builds keys from the
defining module, and std defines `begin_panic` in `std::panicking` and only re-exports it
from `rt`, so the mapping never matches. No fixture used a pre-2021 `panic!`.

Public API: the translator's output.
