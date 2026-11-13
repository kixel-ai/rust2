# [M] Cheaper returned-object ownership in Go and Dart

## Goal

A [rust2-ffi](/ffi/README.md) Go or Dart binding returns an object for close to what C
pays. [The binding benchmark](/bench/ffi/README.md) measures an object return at 263 ns
in Go and 242 in Dart against C's 25, and every returned frame, group, and future pays it.
Ownership stays automatic: a dropped object is still freed without a call from the user.

## Plan

- Profile first: split the cost between the handle allocation, Go's `runtime.AddCleanup`,
  Dart's `Dart_NewFinalizableHandle`, and the crossing, and fix what dominates.
- Two candidates this quest first listed are already how the code works, found by the
  2026-11-12 audit: Dart objects use `Dart_NewFinalizableHandle` with a size of 0
  (`ffi/runtime/dart.rs`), and Go registers one cleanup with the handle as its argument
  (`ffi/src/go.rs`'s `own_`). So take candidates from the profile. One left: allocate the
  wrapper and its handle together, so a return allocates once.
- Explicit `Close` stays, and the cleanup stays as the backstop; dropping it is refused,
  since a missed `Close` would leak. Since #730, a checked type's `Close` drops the value
  through Rust's `_drop` and keeps no hold count, so the cleanup alone frees the handle,
  which costs Go's object return 500 ns; how that changes is checks-raise's open
  decision.
- Re-measure the bench's object return and stamped write.

## Required

- [Failed checks raise](/quest/m1/ffi-export/checks-raise.md) - rewrites the same handle word and decides when a closed handle is freed; profile after it lands, or the baselines move

## Related

- [Copy values](/quest/m1/ffi-export/copy-values.md) - removes the per-frame `Timestamp`
  object, the other half of Go's and Dart's stamped write
