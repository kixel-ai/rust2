# [S] A function pointer or a boxed closure as a string pattern

## Goal

[rust2-ts](/ts/README.md) translates a `str` search whose pattern is a function pointer,
`fn(char) -> bool`, or a boxed closure, `Box<F>` or `Box<dyn FnMut(char) -> bool>`, by
value or behind a reference. The search calls it as std's does, and drops a box it owns
where std drops it: as a search that returns a value returns, or with the iterator it
returns. Differential tests against native Rust prove it for every `str` search taking a
pattern.

## Plan

Found in [#495](https://github.com/moq-dev/rust2/pull/495), which made a reference to a
closure translate as a pattern. On main at `7c41f2b1`, each of these fails as "unsupported
call to `core::str::<impl str>::split::<...>`" (or `contains`):

- `s.contains(p)` and `s.split(p)` of `p: fn(char) -> bool`, and `s.split(&p)`.
- `s.split(b)` and `s.contains(b)` of `b: Box<dyn FnMut(char) -> bool>`, and
  `s.contains(&mut b)`.
- `s.split(b)` and `s.contains(b)` of `b: Box<{closure}>`.

`pattern` in `ts/src/lower/library.rs` matches no `ty::FnPtr` and no `Box`.
`called_through` already sees through a `Box` to the closure it owns, and
`called_closures` drops an owned closure where std does. Check how a `Box<dyn FnMut>` is
called and dropped elsewhere before mapping it. A boxed closure that owns a value that
drops is the case to test hardest.

Public API: the translator's output.
