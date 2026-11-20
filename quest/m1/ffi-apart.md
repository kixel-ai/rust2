# [M] Keep apart the names each language and platform takes

## Goal

Every member a generator names is apart from what its language, runtime, or platform
headers already take, so the corpus's crates below bind and leave `KNOWN` in
`ffi/tests/fixtures/corpus.rs`. Each clash a check like `distinct` catches is renamed
rather than reported, since that check's error has no span.

## Plan

Kept by the maintainer on 2026-11-19, in the pick of [#878](https://github.com/kixel-ai/rust2/pull/878).

Found by [the corpus](/ffi/tests/fixtures/corpus.rs) on 2026-11-19, after its PR named apart
C++'s `default` and trait members, Python's keyword variants, Dart's core types, and
Kotlin's `_` parameter. What remains, by generator:

- Dart: a plain enum's value and a trait member, both `debug` (log, env_logger, object,
  redox_syscall); a method `index`, which every Dart enum inherits (petgraph).
- Swift: an inherent `default` and `Default`'s, both `static default` (darling,
  prettyplease, icu_locale_core, nu-ansi-term).
- Go: a variant's field and its method, both `Number` (der); a method go vet reads as a
  standard one, `Is` or `ReadByte`, of another signature (anyhow, untrusted).
- Kotlin: a method of the JVM signature `Object.wait()` (parking_lot, crossbeam-utils), or
  of a Kotlin property's getter, `isFirst` (itertools).
- C++: a name a system header defines as a macro, such as `WEXITSTATUS`,
  `PTHREAD_COND_INITIALIZER`, or `NULL`'s `__null` (libc, hermit-abi, wasm-bindgen).

Since [no config](https://github.com/kixel-ai/rust2/pull/903), the corpus exports only a
crate whose census refuses nothing, as a refusal fails the export, so a crate here that
refuses an item no longer reaches its bindings: prove each fix on its reduction in the api
fixture, and through the corpus only where the crate's census is clean.

Recommended: give each generator the `taken` set C++'s `kept_apart` uses, holding every
member the generator itself writes, so the rename happens where the name is made, and add
each case to the api fixture's `clash` module. For macros, rename every name a C or C++
standard or POSIX header may define, from a list, rather than `#undef` the user's macro.

Public API: the renamed members of those crates' bindings.
