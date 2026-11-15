# [M] Go's calls pass C no pointer to a Go local

## Goal

A [rust2-ffi](/ffi/README.md) Go call allocates nothing for the C values it passes by
pointer or reads back through one: a value receiver (`c_self_`), the other side of
`Equal` (`c_other_`), an out (`c_out_`, `c_out<k>_`), an error (`c_err_`), and a future's
handle (`f_`). cgo moves each such local to the heap, since C may call back into Go and
move the goroutine's stack: `go build -gcflags=-m` over moq's Go packages reported 201
`c_self_`, 71 `c_out_`, 65 `c_other_`, 35 `c_err_`, and 20 `f_` on 2026-11-14.

## Plan

- ✅ Keep this quest, settled by the maintainer on 2026-11-15 in
  [kixel-ai/rust2#829](https://github.com/kixel-ai/rust2/pull/829#issuecomment-6890842376)
  (rejected: drop it).
- Extend the shim each package's cgo preamble defines for a checked call (`checked_c` in
  `ffi/src/go.rs`) to every call that takes a pointer to a Go local: the shim takes a
  value by value and passes C a pointer to its own copy, and returns each out, the error,
  and the handle in the struct it already returns the status in. A C local never moves,
  and cgo writes a returned value into the goroutine's stack wherever it moved.
- A `&mut self` value's write-back returns the same way, as the value the shim's copy
  holds after the call.
- Rejected: hiding a Go pointer from escape analysis through a `uintptr`, which checkptr
  refuses under `-race` and the compiler may treat as not aliasing the local; and cgo's
  `#cgo noescape`, which takes effect only beside `nocallback`, and these calls can call
  back.
- Re-measure the bench's object return, record, and stamped write.
