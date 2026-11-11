# [L] Swift handles are non-copyable

## Goal

[rust2-ffi](/ffi/README.md)'s Swift generator emits each object handle (a type that is not
`Copy` and not an error) as a `~Copyable` struct, with `consuming`, `borrowing`,
`mutating`, and `inout` where Rust takes `self`, `&self`, `&mut self`, and `&mut`. A call on
a moved handle, or a lent handle kept past its call, is a compile error where it traps today.
Proven by `ffi/tests/api.swift`: its `moved`, `boxed moved`, `lent moved`, and `lent kept`
cases fail to compile, and the rest of the script still matches the native transcript, on
Linux and on an Apple platform.

Errors stay classes, since `Swift.Error` requires `Copyable`. `Sendable` is
[not-send](/quest/m1/ffi-export/not-send.md)'s, and the other languages are out of scope.

## Plan

Decided with the maintainer on 2026-11-09, from the spike
[moq-dev/rust2#688](https://github.com/moq-dev/rust2/pull/688), whose PR body has the
report and whose [evidence comment](https://github.com/moq-dev/rust2/pull/688#issuecomment-6758972634)
has the scripts that generated the stricter forms:

- ✅ Switch handles to `~Copyable` structs, with tuples holding a handle as generated named
  structs and arrays as swift-collections' `UniqueArray` (rejected: a generated list type
  of rust2's own; waiting for Swift's own `Array` to hold noncopyable values; no switch).
  Swift 6.3 holds a noncopyable value in neither, which ruled out 9 of the api fixture's
  exports in the spike. The generated module gains the swift-collections dependency.
- ✅ Drop the per-handle atomic hold count once a release-build measurement confirms the
  gain (rejected: keeping it as a backstop). The compiler then refuses everything it
  guarded; the one case left, a data race in Swift 5 mode, is already undefined behavior
  in Swift. Measure it on [the binding benchmark](/bench/ffi/README.md), whose Swift handle
  call is the baseline. The spike measured `get()` at 24.6 ns with it and 17.9 ns without, against a
  debug library on a loaded host. This is the Swift side of
  [checks-raise](/quest/m1/ffi-export/checks-raise.md)'s one accounting.
- ✅ m1, in the exporter epic (rejected: m2), since the switch breaks the generated Swift
  API, which is cheapest before moq adopts the bindings.
- ✅ An `Option<&T>` parameter gets an overload per optional borrowed parameter, one taking
  the borrowed handle and one without it (rejected: one `borrowing T?` signature, which
  makes a caller holding a plain handle move it into an optional and back; a generated
  `Borrowed<T>` view built at each call). Wrapping a borrowed handle in an optional consumes
  it. moq's items have one such parameter each; refuse past a small bound at the item's span.
- Swift 6.4 conformances (`Equatable`, `Hashable`, `CustomStringConvertible`,
  `CustomDebugStringConvertible` for noncopyable types, from
  [SE-0499](https://github.com/swiftlang/swift-evolution/blob/main/proposals/0499-support-non-copyable-simple-protocols.md))
  come in under `#if compiler(>=6.4)`. Until then `debug(c)` is `c.debugDescription` and
  `"\(url)"` is `url.description`; `==` stays a static operator.
- A `&mut self` call needs the handle in a `var`.
- Check on an Apple platform that `Optional<~Copyable>` metadata back-deploys to the
  deployment target the generated package declares; the spike ran on Linux only.
- [moq-dev/rust2#678](https://github.com/moq-dev/rust2/pull/678)'s `&mut` in/out becomes
  `inout`, exclusive at compile time, and `Option<&mut T>` becomes `inout T?`. Whichever
  lands second adapts to the other.

Public API: every generated Swift handle type and the signatures taking one (breaking), and
the generated package's swift-collections dependency.

## Related

- [Failed checks raise](/quest/m1/ffi-export/checks-raise.md) - the run-time checks left after this switch raise rather than trap
