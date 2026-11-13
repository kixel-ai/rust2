# [S] Owned buffers are taken, not copied

## Goal

A byte payload a [rust2-ffi](/ffi/README.md) binding passes to Rust is copied at most
once, and Kotlin's and Dart's writes on [the binding benchmark](/bench/ffi/README.md) show
the allocation and copy they lost.

## Plan

[kixel-ai/rust2#744](https://github.com/kixel-ai/rust2/pull/744) landed the C ABI's taken
buffers, `<prefix>_buf_clone`, and each binding's way to pass a lent payload back with no
copy. What remains:

- Kotlin: its own bytes inside an encoded value, such as a record, cross three times (into
  the `Writer`'s array, the JNI copy, and the lift); text arguments twice
  (`encodeToByteArray`, then the JNI copy). Cross each once, as top-level `Bytes` do.
- Re-measure the bench's write and throughput rows on a quiet host against main at the same
  load, and update [the benchmark's tables](/bench/ffi/README.md). The run in #744 was at a
  load average of 28 to 57, so it could not resolve the change.
