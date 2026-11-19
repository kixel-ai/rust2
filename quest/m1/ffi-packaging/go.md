# [M] A Go module that go get builds

## Goal

The generated Go package links its prebuilt static library with no `CGO_LDFLAGS`: the
export writes a `#cgo <goos>,<goarch> LDFLAGS: ${SRCDIR}/...` line per target it exports,
with the system libraries rustc prints, and the builder releases the module with each
target's library, so `go get` and `go build` work for every target moq ships. CI builds
the Go test on Linux, macOS, and Windows (with a `-gnu` static library for Go's mingw
linker).

## Plan

Found by the 2026-11-18 language audit: the package comment says it links only through
`CGO_LDFLAGS` (`ffi/src/go.rs:188-189`); moq works around it with a hand-written
per-platform `cgo.go`, as [wasmtime-go](https://github.com/bytecodealliance/wasmtime-go/blob/main/ffi.go)
does. Go module zips are capped at 500 MiB, and the stand-in's `libmoq.a` is 59 to 104 MiB
stripped per target: report the module's size, and recommend a layout (one module, or one
per target) that stays under the cap with room to grow.

Public API: the generated package's cgo directives, and the module layout.
