# [S] A String from UTF-16

## Goal

[rust2-ts](/ts/README.md) translates `String::from_utf16` and `String::from_utf16_lossy`,
called or named as function values: each returns what std's does for every slice of
units, unpaired surrogates included, with `FromUtf16Error`'s `Display` and `Debug`,
proven by differential tests against native Rust.

## Plan

Found in [#514](https://github.com/moq-dev/rust2/pull/514), which translates
`str::encode_utf16`: both are refused today as unsupported calls. `char::decode_utf16`
already maps in `rt/char.ts`, with std's errors, and std builds both on it, so they may
need only a mapping each. A JS string can hold an unpaired surrogate, so check that no
path lets one through where std errs or writes U+FFFD.

Public API: the translator's output, and the runtime functions and error type each needs.
