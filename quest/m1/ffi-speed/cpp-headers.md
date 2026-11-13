# [M] A C++ header per crate

## Goal

[rust2-ffi](/ffi/README.md)'s C++ binding is one header per exported crate over a shared
runtime header, with `<prefix>.hpp` as the umbrella that includes them all, so a
translation unit pays only for the crates it names. Measured by the 2026-11-12 audit: a
translation unit that only includes the api fixture's 206 KiB `api.hpp` takes 0.89 s and
292 MB at C++17 and 1.72 s and 361 MB at C++23, against 0.18 s for a handful of std
headers, and moq's header is 2,664 KiB.

## Plan

- Split as Go's packages are split per crate (`ffi/README.md`'s Go section): a crate's
  header includes those of the crates its signatures name, and the runtime's.
- The umbrella keeps today's include working, so nothing breaks for a caller.
- Rust's crates may name each other in a cycle, which C++ headers allow only through
  forward declarations: declare every exported type in one shared forward header.
- Report the include time and memory per crate header and for the umbrella, on the
  fixture and on moq.

Public API: the per-crate headers are new; the umbrella header is unchanged.
