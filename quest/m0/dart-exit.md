# [S] The Dart api script exits when its transcript ends

## Goal

`tests/api.dart` run with no case, as `api_dart` runs it, exits once its `main` returns.
Today it prints its whole transcript in about 0.5 s, then the Dart VM stays alive about 61 s
more before it exits 0: alone on an idle host, and every run. That wait is the critical path
of a warm `just ffi test`, about 61 of its 70 s, and every agent's run pays it. Each other
case (`futures`, `owned`, `closed`, `bytes`, `bounds`) exits in about 0.4 s.

## Plan

Found on 2026-11-13 by measuring the warm run after
[FFI test programs build once per input change](https://github.com/kixel-ai/rust2/pull/780).

The lead: the generated Dart's port keeps the isolate alive while any wake token is held
(`_hold` and `listen` in the runtime `dart.rs` writes). Something the transcript path does
leaves a token held until a Rust-side wake about a minute later, such as a future dropped or
cancelled before it finished whose token is never forgotten. Recommended: bisect the
script's sections to the one that holds it, then fix the root cause. If the generated
binding leaks the hold, every consumer's isolate outlives its work the same way. That fix
is in `dart.rs`, with an `api_dart` case that would hang without it. If the hold is the
test's own, fix the test.
