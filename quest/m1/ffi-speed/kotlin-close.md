# [S] Kotlin's checked close in one crossing

## Goal

A Kotlin checked object's `close()` that no call holds drops its value and frees its
handle in one JNI crossing, as Go's and Dart's have since
[#838](https://github.com/kixel-ai/rust2/pull/838), and the binding benchmark reports the
object-return row before and after on HotSpot.

## Plan

Found while profiling [#880](https://github.com/kixel-ai/rust2/pull/880)'s nursery under
async-profiler: once the nursery removed the phantom reference, a checked close's two JNI
crossings, `<type>_drop` and then `<type>_free` (`ffi/runtime/rt.kt`, `__Handle.close`),
held as many samples as `track.info()`'s own call. The object return is 118 ns median wall
against C's 31.

Follow Go's design: claim the handle closed so no call starts, then drop and free in one
glue entry, failing closed rather than waiting for calls that start during the drop, since
a `Drop` that calls back into Kotlin on the same object would wait on itself. A close that
fails under a borrow reopens the handle.

Public API: none.
