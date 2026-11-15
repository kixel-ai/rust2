# [L] Returned objects closed early skip the collector in Go and Dart

## Goal

A [rust2-ffi](/ffi/README.md) Go or Dart binding returns an object for close to what C
pays when the caller closes it soon after. [The binding benchmark](/bench/ffi/README.md)
measures an object return at 263 ns in Go and 242 in Dart against C's 25, and every
returned frame, group, and future pays it.
Ownership stays automatic: a dropped object is still freed without a call from the user.

## Plan

- The profile ([the benchmark's README](/bench/ffi/README.md#what-a-call-spends), item 2)
  puts most of the cost in each runtime's per-object registration with its collector: Go's
  `runtime.AddCleanup` and the `Cleanup.Stop` its `Close` makes take 39% of a return and
  close, and Dart's `Dart_NewFinalizableHandle`, its crossing, and its finalization take
  three quarters (1,600 instructions). Without the registration, Dart's return and close
  took 62 to 65 ns against 176 to 245. kixel-ai/rust2#828 took what it could without
  deferring the registration: one allocation for a Go object and its handle, and no
  `_Arena` in a Dart close.
- Rejected, measured: Dart's `NativeFinalizer.attach` without `detach` (770 to 1,070 ns
  a return), Go's `runtime.SetFinalizer` (13% fewer instructions to register and clear
  than `AddCleanup` and `Stop`, but it runs every free on one goroutine and keeps each
  object a collection longer), and leaving Go's cleanup registered at `Close` (290 to 450
  ns a return against 120, in a microbenchmark).
- Settled by the maintainer (2026-11-15, kixel-ai/rust2#828): a nursery of the last 64
  objects returned, which only registers an object
  with the collector once it leaves the nursery still open. A `close` of an object in the
  nursery frees its handle at once, with a checked value's drop in the same crossing, and
  it never registers. Explicit `Close` stays, and the registration stays as the backstop.
  - Dart: one ring in the isolate's `_n`, holding its objects, whose oldest is registered
    as today when a return pushes it out. A checked object's handle is then freed at
    `close`, so its calls check a closed flag in Dart, as a counted one's do (one load a
    call). One finalizable handle on the ring frees what it still holds at isolate exit.
  - Go: a ring per P, through a `sync.Pool` as `lending_` pools byte batches. Each
    object points at its ring and the ring at its objects, so one `AddCleanup` on the ring
    frees its open slots once all of them are unreachable, as when the pool drops it. A
    compare-and-swap on the object's slot decides between its `Close` and its eviction.
  - The later free is accepted: an object dropped without `Close` is freed only after up
    to 64 more returns, or in Go once its ring's other objects are unreachable too,
    rather than at the next collection. Rejected: keeping today's per-object
    registration and deleting this quest; flushing the nursery each event-loop turn (a
    timer each turn costs more than it saves when a turn returns one object, as a woken
    read does).
- Re-measure the bench's object return and ready read, and check by drop counts in
  `ffi/tests` that an object dropped in the nursery is freed once evicted and collected.

## Related

- [Shared borrows](/quest/m1/ffi-speed/shared-borrow.md) - rewrites the same handle word
