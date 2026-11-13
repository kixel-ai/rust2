# [S] Go's checked calls allocate no status

## Goal

A [rust2-ffi](/ffi/README.md) Go call that can fail a check passes its
`<prefix>_status` without a heap allocation. Today each declares `var c_status_` and
passes `&c_status_` to C, which cgo moves to the heap: `go build -gcflags=-m` reports it
for every checked function (756 in moq's `net` package), and a bare cgo call measured
6 ns more with the pointer than without, about what Go's checked getter lost against
main in [the bench](/bench/ffi/README.md#per-handle-baseline).

## Plan

- ✅ Pass the stack cell's address hidden from escape analysis, as a `uintptr` turned back
  into a pointer at the call (rejected: keep the heap allocation; decide later). Settled by
  the maintainer 2026-11-13. A goroutine inside a cgo call is never shrunk (the runtime's
  `isShrinkStackSafe` refuses one in a syscall), and the glue writes the status only when a
  check fails, before any Rust code runs, so before any callback into Go could grow and
  move the stack. The generated glue checks that ordering at run time: a status written
  after a callback into Go began fails loudly, never silently. The fixtures run `go vet`,
  which flags a `uintptr` made back into a pointer.
- Rejected: a thread-local last error, refused on 2026-11-12 as hidden state a caller can
  forget to read; a `sync.Pool` of C cells, which costs more than the allocation; one
  shared cell, since goroutines share threads.
- Re-measure the Go getter and handle call against main, CPU first.
