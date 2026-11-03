# [M] std's char methods that have no mapping

## Goal

[rust2-ts](/ts/README.md) translates every stable `char` method and conversion, called or
named as a function value, as in `s.split(char::is_alphabetic)` or
`chars.map(char::len_utf8)`. Each returns what std's does for every `char`, by std's
Unicode version, proven by differential tests against native Rust over every code point.

## Plan

Found in [#495](https://github.com/moq-dev/rust2/pull/495). Probed on main at
`7c41f2b1`, each of these fails as "unsupported call", both called and as a value:

- Classes: `is_alphabetic`, `is_lowercase`, `is_uppercase`, `is_alphanumeric`,
  `is_control`, `is_numeric`, and `is_digit`.
- Case: `to_lowercase` and `to_uppercase`, and `Display` of the `ToLowercase` and
  `ToUppercase` they return.
- Escapes: `escape_unicode`, `escape_debug`, and `escape_default`, and `Display` of
  what each returns.
- Encoding: `len_utf8`, `len_utf16`, `encode_utf16`, and `char::decode_utf16`.
- Conversions: `char::from_digit` and `char::try_from(u32)`.

`encode_utf8` into a `[u8; 4]` fails instead as "a mutable reference to a value JS copies".

`is_whitespace`, `from_u32`, `to_digit`, `make_ascii_*`, the `is_ascii_*` family,
`From<u8>`, and `u32::from` already translate, and a mapped method already translates as a
value: the gap is the call's mapping, not the function value.

JS's own Unicode data (`\p{...}` regexes, `toLowerCase`) follows the engine's Unicode
version, which may differ from std's `char::UNICODE_VERSION`, and its case mapping is of
strings, not chars. Compare that against lowering core's own tables before choosing, and
measure the cost of either. The case and escape iterators are std iterators, which may
need their own `rt.Iter` and `Display`.

Public API: the translator's output, and the runtime's functions each needs.
