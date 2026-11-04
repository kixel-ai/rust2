# [S] C++ bindings on MSVC

## Goal

The nightly's `Export moq-net to C++ (MSVC)` job passes: the exporter's C++ header passes
`just ffi test cpp` and `just ffi moq-cpp` with MSVC on Windows, as it does with GCC and
Clang on Linux and Apple Clang on macOS.

## Plan

- Read the next nightly's run, and fix what it finds at the source, in the generator or the
  recipes, never by loosening flags.
- The exporter runs on Windows, and `tests/api.cpp` and `moq/moq.cpp` compile with the
  job's cl (19.51) at each standard the checks build, as Compiler Explorer's cl shows.
  Nothing has linked or run there yet: cl's lookup of `link.exe` from Git Bash, where Git's
  `/usr/bin/link` may shadow it; the native libraries rustc prints; the transcripts; and the
  job's Python step, between the C++ checks and `just ffi moq-cpp`.
