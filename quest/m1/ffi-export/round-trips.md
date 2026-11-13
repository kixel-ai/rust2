# [S] Every binding takes back what it returns

## Goal

A value a [rust2-ffi](/ffi/README.md) binding returns passes back into a parameter of the
same Rust type as it is, with no conversion by the caller. Two bindings break this today,
each a `KNOWN` entry in [the shape tests](/ffi/tests/fixtures/shapes.rs) that this quest
deletes:

- C++ returns a collection of bytes, such as `Vec<String>` or `BTreeMap<String, Vec<u8>>`,
  as a `std::vector` of `buf`s, but takes it as a `const std::vector` of `view`s, which a
  vector of `buf`s does not convert to.
- Kotlin returns a tuple of one, `(T,)`, as its `T`, but takes it as `Tuple1<T>`.

## Plan

Found by the shape tests' nightly sample on 2026-11-13.

- C++: take a collection whose items hold bytes as any range whose items convert to a
  `view`, as `view` already takes any contiguous range of bytes, so a returned vector of
  `buf`s, and the caller's vector of `std::string`s, pass as they are.
- Kotlin: return a tuple of one as `Tuple1<T>`, as it takes one, rather than unwrapping it.
  Swift has no tuple of one, so its unwrapping stays.

Public API: C++'s parameter types for collections of bytes, which take more than before;
Kotlin's return type of a function returning a tuple of one (breaking).
