# [S] C++ checks ownership at compile time

## Goal

[rust2-ffi](/ffi/README.md)'s C++ header marks handles with Clang's typestate attributes,
checked by `-Wconsumed`; gives returned views `[[clang::lifetimebound]]` and what a callable
keeps `[[clang::lifetime_capture_by]]`; and lends a `&mut` object as `api::borrow<T>`, which
neither moves nor converts. Each sits behind a macro that is empty on GCC and MSVC, so a
consumer that does not enable the warnings pays nothing, and the run-time lend check stays
as a backstop. Proven by `ffi/tests/api.cpp`: the 6 ownership cases the spike found fail
under `-Wconsumed -Werror`, a view of a temporary buffer fails under `-Wdangling`, and the
script still matches the native transcript at C++17, 20, and 23 and builds with GCC.

## Plan

Decided with the maintainer on 2026-11-09, from the spike
[moq-dev/rust2#688](https://github.com/moq-dev/rust2/pull/688), whose PR body has the
report and whose [evidence comment](https://github.com/moq-dev/rust2/pull/688#issuecomment-6758972634)
has the scripts and test diffs:

- ✅ Adopt all three: typestate attributes, lifetime attributes, and `api::borrow<T>`
  (rejected: the attributes without `api::borrow<T>`; no change). The cost to callers in the
  spike was one lambda rewritten to call through `->`.
- ✅ m1, in the exporter epic (rejected: m2), since `api::borrow<T>` changes the C++ API,
  which is cheapest before moq adopts the bindings.
- `std::move(*borrow.operator->())` still compiles, which is why the run-time check stays.
- [moq-dev/rust2#678](https://github.com/moq-dev/rust2/pull/678)'s `&mut` in/out takes or
  yields `api::borrow<T>`; its `Option<&T>` pointers get nothing from typestate and keep
  the run-time check. Whichever lands second adapts to the other.

Public API: the C++ signatures that lend a `&mut` object (breaking), and the header's
attribute macros.

## Related

- [Swift handles are non-copyable](/quest/m1/ffi-export/swift-noncopyable.md) - the same spike's Swift half
