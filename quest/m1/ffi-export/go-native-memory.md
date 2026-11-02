# [S] Go: lent bytes freed before native memory piles up

## Goal

A Go program that reads many large `Bytes` from Rust holds no more of Rust's memory than
its live `Bytes` reach plus a bounded backlog, even when it makes little Go garbage, as
Kotlin's does.

## Plan

`liftBytes_` in `ffi/runtime/rt.go` lends bytes from 2 KiB, freed by `runtime.AddCleanup`
once the GC collects their owner, and Go's GC does not see C memory. Kotlin had the same
shape: on HotSpot, reading 16 KiB `Bytes` held 2.9 GiB of Rust's buffers after 2 s at full
speed, and grew about 1 MiB/s at 60 frames a second with no collection at all. Its fix caps
the bytes lent at once at 64 MiB in the JNI glue, past which they cross copied, so the
copies' garbage runs the GC that frees the backlog. That held its native memory at the cap
and ran 2 to 3.4 times faster at full speed.

Measure Go the same way first, then likely take the same cap: a count of lent bytes that
the lift reserves and the cleanup releases.

## Related

- [Go: cheaper lent Bytes](/quest/m1/ffi-export/go-bytes.md) - changes the same threshold, so land them in either order
