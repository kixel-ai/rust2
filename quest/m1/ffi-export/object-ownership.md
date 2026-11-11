# [M] Cheaper returned-object ownership in Go and Dart

## Goal

A [rust2-ffi](/ffi/README.md) Go or Dart binding returns an object for close to what C
pays. [The binding benchmark](/bench/ffi/README.md) measures an object return at 263 ns
in Go and 242 in Dart against C's 25, and every returned frame, group, and future pays it.
Ownership stays automatic: a dropped object is still freed without a call from the user.

## Plan

- Profile first: split the cost between the handle allocation, Go's `runtime.AddCleanup`,
  Dart's `NativeFinalizer.attach`, and the crossing, and fix what dominates.
- Candidates, recommended in this order once profiled: allocate the wrapper and its cleanup
  argument together, so registration allocates nothing more; in Dart, attach with an
  `externalSize` of 0 and a shared token object; in Go, register one cleanup per handle on
  a pointer the wrapper already holds rather than a fresh closure.
- Explicit `Close` stays, and the cleanup stays as the backstop; dropping it is refused,
  since a missed `Close` would leak.
- Re-measure the bench's object return and stamped write.

## Related

- [Copy values](/quest/m1/ffi-export/copy-values.md) - removes the per-frame `Timestamp`
  object, the other half of Go's and Dart's stamped write
