# [S] Go's checked calls allocate no status

## Goal

A [rust2-ffi](/ffi/README.md) Go call that can fail a check passes its
`<prefix>_status` without a heap allocation. Today each declares `var c_status_` and
passes `&c_status_` to C, which cgo moves to the heap: `go build -gcflags=-m` reports it
for every checked function (756 in moq's `net` package), and a bare cgo call measured
6 ns more with the pointer than without, about what Go's checked getter lost against
main in [the bench](/bench/ffi/README.md#per-handle-baseline).

## Plan

- Recommended: pass the stack cell's address hidden from escape analysis, as a `uintptr`
  turned back into a pointer at the call. A goroutine inside a cgo call is never shrunk
  (the runtime's `isShrinkStackSafe` refuses one in a syscall), and the glue writes the
  status only when a check fails, before any Rust code runs, so before any callback into
  Go could grow and move the stack. Make that ordering a checked invariant of the glue,
  not a convention. The fixtures run `go vet`, which flags a `uintptr` made back into a
  pointer.
- Rejected: a thread-local last error, refused on 2026-11-12 as hidden state a caller can
  forget to read; a `sync.Pool` of C cells, which costs more than the allocation; Dart's
  one cell per isolate, since goroutines share threads.
- Re-measure the Go getter and handle call against main, CPU first.

## Waiting

- Maintainer: whether Go may pass a stack address that escape analysis cannot see, which
  relies on the runtime never moving a stack in a cgo call before a callback, a behavior
  cgo's documented rules do not promise. Recommended: yes, with the ordering checked in
  the glue, as it wins back 6 ns on every checked Go call; the other way is to keep the
  allocation.
