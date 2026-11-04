# [M] Every panic of a hooked program has its location

## Goal

In a program that reaches `set_hook`, [rust2-ts](/ts/README.md) gives every panic the
`Location` native code gives it, so none throws the runtime's "a panic its hook can't
locate" error, proven by the panics fixture's hooked cases.

## Plan

What remains is `bytes`: the panics of `Bytes`, `BytesMut`, and the `Buf` and `BufMut`
methods whose mapping makes them, each where `bytes`' own source has it. Some are at a call
of its `#[track_caller]` `range`, some in its macros' calls, as `Buf::get_*`'s, and some
in std, as `Vec::with_capacity`'s, whose `SITES` key already holds.

Native names that source by the path cargo hands rustc: the host's registry, as
`$CARGO_HOME/registry/src/index.crates.io-<hash>/bytes-<version>/src/bytes.rs`, unless the
build remaps it. The session's source map names it the same, so matching native puts the
host's path in the output, which is then as reproducible as native's and no more. Which
name to give is the maintainer's decision:

- As rustc names it, with the fixtures remapping `CARGO_HOME` in both builds so their
  snapshots hold on any host. Recommended: it is what native gives, under any remap.
- A fixed prefix in place of the registry, as std's vendored crates have, which differs
  from native unless the native build remaps to the same prefix.

Each throws the runtime's error only once it panics. Refusing their calls at translation
instead refuses most hook programs: forced into hook mode, the `lang` fixtures reach 64
std functions whose mapping shares a runtime function that takes a location, most of which
never panic natively, such as `slice::iter` and an atomic's `load`.

A crate's own `#[track_caller]` function that the runtime calls, through std's `Fn` impls,
stays refused in every program: its location is in std's source.
