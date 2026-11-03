# [S] Repeated values that drop

## Goal

[rust2-ts](/ts/README.md) translates `vec![x; n]` where `x` drops, as std's `from_elem`
makes it: `n - 1` clones of `x` in order, then `x` itself, or `x` dropped where `n` is 0,
proven by differential tests against native Rust.

## Plan

Found writing `lang/clones`' `clone_froms` case: `vec![row; 4]` of a `Vec<Logged>`
and of a `[Logged; 1]` refuse as "passing `T`, which drops, to `vec::from_elem`".
`Vec::resize`'s runtime already clones `n - 1` times and moves the value into the last
slot, which may serve here. No consumer calls it yet; it is planned as part of std.

Public API: the translator's output.

## Related

- [Fresh writes through a zip, a copy, and a repeat](/quest/m3/ts-fresh-shapes.md) - `vec![x; n]` over a `Copy` type
