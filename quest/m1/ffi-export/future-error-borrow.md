# [S] A future whose error borrows its arguments

## Goal

`async fn parse(text: &str) -> Result<u32, dep::Error<'_>>`, whose error is another
crate's and crosses as its message, either exports or is refused with a span; today, unless
a handle would hold the error, the export accepts it and generates glue that fails to build
(E0597: the glue awaits the future past the call that owns `text`, and the error's borrow
outlives it). Found on 2026-11-13 by
[kixel-ai/rust2#733](https://github.com/kixel-ai/rust2/pull/733)'s review, which refuses the
held case (`parse_later` in the reject fixture); main has the bug for every such error.

## Plan

The maintainer kept this quest on 2026-11-13 (rejected: drop) and decided its fix:
✅ the glue converts the borrowed foreign error to its message inside the `async` block,
as foreign errors already cross, while the arguments the future owns are alive, so nothing
borrowed outlives the call and the function exports as its sync form already does
(`parse_cert` in the api fixture); the held case's refusal then goes too.
Rejected: refuse every future whose error names a lifetime, which leaves out what maps.

Public API: each such function newly exports as a future of its `Ok` type and a message.
