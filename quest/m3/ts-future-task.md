# [M] The rest of std::future and std::task

## Goal

[rust2-ts](/ts/README.md) translates the stable public API of `std::future` and `std::task`
that it still refuses: `future::ready` and its `Ready`, `Debug` of a `Poll` and of std's
futures, `Poll::from`, hashing a `Poll`, a `Pending`'s clone, and `Waker::clone_from`.
Differential tests against native Rust cover a `Ready`'s drops and its panics, a `Pending`
that never completes, and `Debug` plain and with `#`.

## Plan

Found by [#519](https://github.com/moq-dev/rust2/pull/519). No consumer calls these; the
maintainer plans all of std regardless.
