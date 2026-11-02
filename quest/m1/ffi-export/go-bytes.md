# [M] Go: cheaper lent Bytes

## Goal

A `Bytes` the generated Go lends from Rust costs no more than a copy of 1 KiB, so
`lendFrom_` in `ffi/runtime/rt.go`, the 2 KiB below which returned bytes are copied, can
fall or go.

## Plan

Measured for moq-dev/rust2#411 on a loaded host, so read the ratios: a lent buffer costs
about 600 to 800 ns at any size, against a copy's 150 ns at 100 B, 450 ns at 1.5 KiB, and
10 µs at 64 KiB. Most of it is `runtime.AddCleanup` on the calling goroutine: the walk of
the span's specials (`specialFindSplicePoint`), `fixalloc`, and their lock. The cleanup's
`moq_buf_free` runs on the cleanup goroutine instead.

These made no difference outside the noise:

- freeing 256 buffers per cgo call from the cleanup;
- freeing queued buffers on the calling goroutine at its next lift;
- a 256-byte owner, so a span holds fewer specials.

Left to try:

- one owner for every buffer a call returns, such as a record's or a `Vec`'s;
- what Go's runtime has changed, or proposes, for the cost of `AddCleanup`;
- the same measurements on an idle host.
