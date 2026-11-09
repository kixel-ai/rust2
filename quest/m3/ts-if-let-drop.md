# [S] If-let of a temporary whose rest drops type-checks

## Goal

[rust2-ts](/ts/README.md) translates `if let Some(E::A(x)) = temp()`, where the rest of
the temporary has a destructor, into TypeScript that type-checks, proven by differential
tests against native Rust over enums whose variants hold a `Drop` struct, an `Option` of
one, and a plain struct with `Drop`.

## Plan

Found by [#654](https://github.com/moq-dev/rust2/pull/654), whose `tokio` fixture works
around it by reading bounded senders by reference. The drop of the rest of the temporary
sits behind a second `tag !== "A"` test, which `tsc` rejects (TS2367) once it narrowed the
temporary in the first test's arm. It is safe Rust, so any crate can hit it; m3's
[unsafe policy](/quest/m3/README.md) puts translation effort there.

Decided with the maintainer on 2026-11-07: ✅ plan it, in m3 (from #654's follow-ups).

Public API: the translator's output.
