# [S] C++ bindings on MSVC

## Goal

The nightly's `Export moq-net to C++ (MSVC)` job passes: the exporter's C++ header passes
`just ffi test cpp` and `just ffi moq-cpp` with MSVC on Windows, as it does with GCC and
Clang on Linux and Apple Clang on macOS.

## Plan

- Read the next nightly's run, and fix what it finds at the source, in the generator or the
  recipes, never by loosening flags.
- The C++ checks pass with the job's cl (19.51), linking and running from Git Bash. Nothing
  after them has run on Windows yet: the Python's cases past its transcript, such as its
  futures on the proactor loop, and `just ffi moq-cpp`'s export of moq, link, and session.
