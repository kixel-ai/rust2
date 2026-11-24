# [S] React Native lends returned bytes only where a lend pays

## Goal

React Native's binding copies bytes a call returns up to the size where lending Rust's
buffer costs less on Hermes, measured on [the benchmark](/bench/ffi/README.md#react-native)'s
`frame.payload` by size, instead of the web's 64 bytes, and its README and the benchmark
say where that size is and why.

## Plan

- Measured 2026-11-23: a 64-byte copy is 0.7 µs and a 100-byte lend 1.4 µs of wall time,
  2.2 µs of CPU with Hermes's collector finalizing the lend's native state, flat to 64 KiB.
  `returned` in `ffi/runtime/rt.jsi.hpp` makes a `bytes_cell`, a `view`, an `ArrayBuffer`,
  and sets the cell as the `rt.Bytes`'s native state on every lend.
- First measure a copy at each size of the benchmark's payload rows, by a threshold the
  benchmark can set, then either move the threshold there or make the lend cheaper, such
  as by dropping the native state for a handle the runtime already holds, whichever leaves
  the faster binding at every size.
- A lend's point stands at large sizes: a copy of 1 MiB per read is what it avoids, so the
  threshold stays finite.
