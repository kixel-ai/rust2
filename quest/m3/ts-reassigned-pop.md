# [S] Popping a String that is reassigned elsewhere

## Goal

[rust2-ts](/ts/README.md) translates `String::pop`, `remove`, and `split_off` on a local
that another path assigns whole, proven by a differential test against native Rust.

## Plan

Found on 2026-11-02 testing [`String + &str`](https://github.com/moq-dev/rust2/pull/479):
with one arm `s = String::from("x")`, another arm's `s.pop()` reads `s` but writes the
result through `_3.v`, an `rt.Place` it never declares, so `tsc` fails. `truncate` and
`insert` in the same spot declare the place and pass.

```rust
pub fn local(op: u8) -> Option<char> {
	let mut s = String::new();
	match op {
		1 => return s.pop(),
		_ => s = String::from("x"),
	}
	None
}
```

The read and the write of the borrowed place disagree on whether the `&mut` is an alias of
`s` or its `rt.Place`; look for every edit that reads through `borrowed_place` and writes
through `store`.

Public API: the translator's output.
