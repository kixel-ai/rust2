# [L] Placeholders written to a crate's own fmt::Write

## Goal

[rust2-ts](/ts/README.md) translates `write!` with placeholders into a crate's own
`fmt::Write` impl: its `write_str` and `write_char` see the same calls, with the same
text, in the same order, as std makes. This is proven by differential tests against
native Rust with a writer that logs each call and one that fails partway.

## Plan

Split from [#269](https://github.com/moq-dev/rust2/pull/269), which refuses this with a
span. The orchestrator decided on 2026-10-30, pending maintainer review, while the
maintainer was away.

std splits what a placeholder writes across many calls. Natively,
`write!(w, "a{:>4}|{}|{:?}|{:e}|{:?}", 1, -5, "q\"b", 1.5, P { x: -1 })` makes 28:
`"a", " ", " ", " ", "1", "|", "-", "5", "|", "\"", "q", "\\\"", "b", "\"", "|", "1", ".", "5", "e", "0", "|", "P", " { ", "x", ": ", "-", "1", " }"`.
The runtime's port of `core::fmt` writes the same bytes in fewer pieces, such as
`"   1"`, `"-5"`, and `"P { x: -1 }"`. A `String` or a `Formatter` can't tell the
difference. A writer that counts calls or fails partway, like a capped buffer, can.

- Port each runtime write to std's sequence: each fill char a `write_char`, the sign a
  `write_char` before the prefix and digits, the `Debug` builders' name and `": "` apart,
  a string's `Debug` escapes in runs between quotes, a float's formatted parts, and a
  byte slice's `escape_ascii`, whose spent front and back bytes are each a `write_char` and
  each run it leaves as is, empty or not, and each escape a `write_str`.
- Add `writeChar` to the runtime's `Write`, so a crate's own `write_char` override runs.
- Measure what the extra calls cost `format!` into a `String`.
- Add a differential case for `Debug` of a `Cell` into a writer that sets that cell in
  `write_str`. std reads the value before writing `Cell { value: `, and
  [#484](https://github.com/moq-dev/rust2/pull/484) lowers it that way, but only such a writer
  can observe the order.
- Rejected: allowing placeholders with the runtime's chunking. The bytes match, but a
  crate's `write_str` would see calls std doesn't make.

Public API: the translator's output.

## Related

- [Trait objects](/quest/m3/ts-dyn.md) - `dyn fmt::Write` itself
