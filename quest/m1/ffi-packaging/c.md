# [M] C and C++ release archives

## Goal

The builder releases a per-target archive of the C and C++ binding: headers, static and
shared libraries, a `<prefix>.pc`, a CMake config package for `find_package`, and a CPS
file, each carrying the native libraries a program linking it needs. The shared library
has a versioned SONAME, and the static library exposes only `<prefix>_*` symbols. A clean
CMake consumer and a pkg-config consumer build and run the transcript.

## Plan

Found by the 2026-11-18 language audit: moq's libmoq ships `find_package(moq)` files and
`moq.pc.in` (`rs/libmoq/` in moq-dev/moq); a test cdylib with the export's `crate-type`
has no SONAME; the static library exports about 2,100 global symbols, including
compiler-builtins' unmangled `__addvdi3`, which can collide with another Rust static
library in one program. [CMake 4.3 made CPS stable](https://www.kitware.com/common-package-specification-is-out-the-gate/).

Recommended: hide symbols with a prelinked object (`ld -r`, then `objcopy
--keep-global-symbols`) on ELF and Mach-O, and say what MSVC gets. The native link
libraries come from rustc's `--print native-static-libs`, as the tests already read them.

Public API: the archive layout, and the SONAME.
