# [M] str's case maps and Unicode escapes

## Goal

[rust2-ts](/ts/README.md) translates `str`'s stable methods that read Unicode beyond ASCII,
called or named as function values: `to_lowercase` and `to_uppercase`, `escape_debug`,
`escape_default`, and `escape_unicode` with the `Display` of what each returns, and
`encode_utf16`. Each returns what std's does for every string, by std's Unicode version,
proven by differential tests against native Rust.

## Plan

Found in [#507](https://github.com/moq-dev/rust2/pull/507), which maps `char`'s methods
over std's own Unicode tables, generated into `rt/unicode.ts` by the runtime's `unicode`
test, and its iterators in `rt/char.ts`. Build on those once it merges, rather than on
JS's `toLowerCase` or `\p{..}`, whose Unicode data differs from std's even at the same
reported version.

Probe on main for which of these still fail, and for any other `str` method that reads
Unicode and has no mapping; this list is what was seen, not a full census.

Watch for where `str`'s differ from mapping each char:

- `to_lowercase` writes a capital sigma as the final form at the end of a word, by
  Unicode's rule over the cased and case-ignorable chars around it, which std reads from
  tables `rt/unicode.ts` does not hold yet.
- `escape_debug` escapes a grapheme extender only as the string's first char.
- The escapes' iterators flat-map each char's, so their size hints and `Display` partway
  are std's adaptors', not one char's.

Public API: the translator's output, and the runtime functions and tables each needs.
