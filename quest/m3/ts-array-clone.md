# [S] An array's clone of values that clone themselves

## Goal

[rust2-ts](/ts/README.md) translates `Clone::clone` of a `[T; N]` whose `T` clones by
its own `Clone`, such as a type that logs its clones or drops, each element cloned in
order as std's does, proven by differential tests against native Rust.

## Plan

Found writing the `lang/slices` fixture for
[moq-dev/rust2#510](https://github.com/moq-dev/rust2/pull/510): `<[Noisy; 2] as Clone>::clone`
is refused as an unsupported call, while a `Vec`'s and a tuple's clone of such values
translate. No consumer calls it yet; it is planned here as part of std.

Public API: the translator's output.
