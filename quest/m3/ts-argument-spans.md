# [S] A name or an operator argument at its own Rust span

## Goal

When `tsc` rejects an argument of a translated call that is a plain name or an operator,
[rust2-ts](/ts/README.md) reports the error at that argument's Rust, not at the method name of
the call it is passed to, proven by the `spans` fixture's snapshot.

## Plan

Left out of [#520](https://github.com/moq-dev/rust2/pull/520), which maps an argument made by
a call to its own Rust. In `Meter.scaled(m, rt.u32WrappingAdd(a, 1), ...)`, the receiver `m`
still maps to `scaled`, as does an argument such as `(a ^ b) >>> 0`.

MIR gives each argument its own span (`Spanned<Operand>`). Locating names and operators the
way calls are located needs the lowering passes that match on expression shapes, such as
`Expr::Ident`, `Expr::Member`, or `Expr::Binary`, to look through the location wrapper first.
Wrapping members and binaries naively changed what the formatting code spills into a variable
before its closure reads it, so the generated code changed. Prove the output text unchanged by
the snapshots, as #520 did, and measure the source map's growth on moq's translation.

Public API: the translator's error spans.
