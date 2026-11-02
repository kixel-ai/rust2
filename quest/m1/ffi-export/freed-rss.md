# [S] Lent bytes freed off the calling thread return their memory

## Goal

A Go program reading many large `Bytes` from Rust holds about as much RSS as its lent
backlog plus Go's heap, not the memory the allocator kept after the GC freed it.

## Plan

Measured for the Go lent-bytes cap, on a 32-core Linux host with glibc's malloc as Rust's
allocator: reading 16 KiB `Bytes` at full speed held at most 64 MiB unfreed and 20 MiB of
Go's heap, yet RSS reached 0.9 GiB in 2 s and 1.5 GiB in 30 s. With `MALLOC_ARENA_MAX=1`
it stayed at 78 MiB. Rust allocates each buffer on the thread that called it, and
`runtime.AddCleanup` frees it on the cleanup goroutine's thread, so glibc's per-thread
arenas keep memory a call on another thread never reuses. Kotlin's cleaner thread
likely does the same; measure it too.

Find where to fix it for any consumer, without config: freeing on a thread whose arena
allocated it, or what glibc offers. A consumer choosing its global allocator, such as
mimalloc, may be the answer to document instead; measure before deciding.

## Related

- [Go: cheaper lent Bytes](/quest/m1/ffi-export/go-bytes.md) - tried freeing queued buffers on the calling goroutine, for speed
