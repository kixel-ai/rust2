# [M] Sample more shapes, positions, and seeds

## Goal

The shape tests (`ffi/tests/fixtures/shapes.rs`) cover:

- the web binding, which `languages!` leaves out today;
- a `Result` with each sampled error, a `&mut` in-out parameter, several parameters,
  `Option<&T>`, an object's methods, and a callback, beside today's echo, closure, and
  future positions;
- floats compared by `to_bits`, NaN payloads included, since `Debug` cannot show them;
- besides the fixed `0x5eed` set the `KNOWN` list checks, a set drawn nightly from a dated
  seed the run prints, so the nightly explores new shapes and a failure reproduces.

## Plan

Found by the 2026-11-18 language audit: `languages!(c, cpp, kotlin, python, swift, go,
dart)` at `shapes.rs:1623` predates the web target ([#847](https://github.com/kixel-ai/rust2/pull/847)),
and JavaScript is where lossless `u64` and `f32` are hardest; floats exclude NaN today,
so "every value crosses exactly" is untested for them; a `Result` whose error is `()`,
which crashed once_cell's export, is refused since the corpus found it.

A shape refused by design stays left out by the census, as today. Each new mismatch found
goes into `KNOWN` with a fix quest, or is fixed here.

Public API: none.
