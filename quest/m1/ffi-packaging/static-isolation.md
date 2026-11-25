# [S] The Go module's Windows library exports only the C ABI, or says why it cannot

## Goal

The Go module's `windows/amd64` library, an `x86_64-pc-windows-gnu` static library that
Go's MinGW linker links, is the one static library `ffi/release/package.py` ships for a
platform with a relocatable link that is still unhidden. Either no global symbol but the
C ABI's `<lib>_*` can clash with another library's, and `coexist` checks it as every other
library's build does, or the release README says why it cannot be.

## Plan

[#1014](https://github.com/kixel-ai/rust2/pull/1014) hid every Apple, Android, and Linux
static library (prelinked on ELF, its other globals renamed apart on Mach-O, whose
relocatable link drops DWARF) and made `coexist` link the other library from a toolchain
two releases older, which fails on an unhidden library.

Reproduced on 2026-11-24 for `windows-gnu`: two static libraries from rustc 1.98.1 and
1.96.0, linked by MinGW gcc 16.2 (binutils 2.46), fail on a duplicate
`rust_eh_personality`, as on ELF.

A trial of the ELF recipe with MinGW binutils (`ld -r --gc-sections -u <root>`, then
`objcopy --keep-global-symbols`) produced an object exporting only the root, but linking it
warned that local symbols such as `__imp__RNvNtCs..._5alloc4sync18STATIC_INNER_SLICE` have
no section: the `__imp_` stubs of std's statics, which windows-gnu reaches through the
runtime pseudo-relocations of auto-import, do not survive a relocatable PE link as they
are. Hiding it needs a way around that and a run of the result on Windows, which the
Windows build leg can do. Mach-O's way may serve: no relocatable link, but `llvm-objcopy
--redefine-syms` renaming each global the archive defines, sparing the `__imp_` symbols of
its import-library members, which name the DLLs' own functions.

## Waiting

- Maintainer: hide the Go module's `windows-gnu` library or leave it unhidden. Hiding it
  needs the `__imp_` problem above solved and run on Windows; leaving it documents that a
  Go program on Windows can link only one Rust static library, as MSVC's C programs link
  the DLL instead. Recommended: leave it unhidden and delete this quest, since a second
  Rust static library in a Windows Go program has no consumer yet, and the README already
  says why.
