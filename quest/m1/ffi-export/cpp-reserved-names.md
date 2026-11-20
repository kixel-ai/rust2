# [S] C++: a Rust name C++ reserves exports under one it does not

## Goal

A Rust `pub` item whose name C++ reserves for the implementation, one holding `__` anywhere
or starting with `_` and a capital, such as the api fixture's `pub fn __all__`, exports in
C++ under a name the standard leaves to programs ([lex.name]), and `tests/api_strict.cpp`
builds with clang's `-Wreserved-identifier` on.

## Plan

Decided with the maintainer on 2026-11-19, in the pick of [#925](https://github.com/kixel-ai/rust2/pull/925): ✅ kept, escaping reserved names in C++ and turning on clang's `-Wreserved-identifier` in the strict build (rejected: dropping it).

Found on [kixel-ai/rust2#925](https://github.com/kixel-ai/rust2/pull/925), whose fixture's
`pub fn __all__` renders in `ffi/surface/api/cpp.hpp` as `api::__all__`: it compiles, but
a name the implementation reserves may break on any standard library. Rust's own
`#[doc(hidden)] pub` macro support, such as `__private_api`, takes such names too.

Recommendation: give the name the trailing `_` C++'s other escapes take only where that
clears the reservation; otherwise drop leading underscores and collapse each `__` to `_`,
then claim the result through `named` in `ffi/src/cpp.rs` as any other name, so
`__all__` and an `all` stay apart. Turn on `-Wreserved-identifier` in the strict build to
keep it so.
