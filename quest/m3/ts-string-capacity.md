# [L] rs.String

## Goal

[rust2-ts](/ts/README.md) translates `String`, `Box<str>`, and `&str` as a runtime type
rather than a JS `string`, holding what std's do: a `String`'s exact capacity through
`with_capacity`, `reserve`, `reserve_exact`, push growth, `shrink_to_fit`, `shrink_to`,
`format!`'s estimate, and `into_bytes`, `Vec::from`, and `Bytes::from`, which take it.
Differential tests against native Rust prove it, and the refusal of reading a `Bytes`'
capacity where a `String`'s bytes may reach it is lifted. Hand-written callers convert at
the boundary with helpers, as for `rs.Vec`.

## Plan

Decided with the maintainer on 2026-11-03: Rust types the runtime can't hold as std's do
get a runtime wrapper, converted at API boundaries, rather than a JS native type. In
their words on [#555](https://github.com/moq-dev/rust2/pull/555), "trying to support
native types is probably a mistake". `Vec` became one there and `Cow<str>` in
[#567](https://github.com/moq-dev/rust2/pull/567). Primitives, `Option`, tuples, and
closures stay native.

What it holds:

- ✅ Measure both, and bring the numbers back as a decision.
- A JS string and its capacity: engine string operations and `Display` stay cheap, but
  byte offsets still convert from UTF-16.
- UTF-8 bytes, as `rs.ByteVec` holds: byte indexing and in-place edits are as Rust's, but
  every `Display` and JS boundary decodes.

Measure on `just ts bench` and moq's translated codec in Node, Chrome, Bun, and Firefox,
with an A/A run of main for noise, as #555 did.

Earlier, on [#532](https://github.com/moq-dev/rust2/pull/532), the maintainer chose to
track capacity rather than refuse it; until this lands, the refusal stays.

Public API: the translator's output and what a hand-written caller passes for a string.

## Related

- [The runtime's namespace is rs](/quest/m3/ts-rs-rename.md) - runs in parallel
- [rs.Slice](/quest/m3/ts-slice-object.md) - the other native mapping becoming a wrapper
