# [S] Windows static libraries hide their Rust runtime, or the builder says why they cannot

## Goal

The two static libraries `ffi/release/package.py` still ships with every Rust runtime
symbol global, the Go module's `x86_64-pc-windows-gnu` library and the C package's MSVC
`<lib>.lib`, are hidden as every other platform's are: no global symbol but the C ABI's
can clash with another Rust static library's, and `coexist` checks it on the Windows build
leg. Or the release README says why one cannot be.

## Plan

Every other static library is hidden by renaming (`hide`): `llvm-objcopy --redefine-syms`
renames each global the archive defines but the C ABI's to `<name>.<lib>.<digest>`,
which needs no relocatable link, the reason Windows was left out before.

- Reproduced on 2026-11-24 for `windows-gnu`: two static libraries from rustc 1.98.1 and
  1.96.0, linked by MinGW gcc 16.2 (binutils 2.46), fail on a duplicate
  `rust_eh_personality`, as on ELF.
- Rename only what the archive's own code defines: the members `dlltool` made for
  raw-dylib imports define `__imp_<fn>` and `<fn>` of the DLLs' functions, which must keep
  their names. COFF has no visibility to hide the renamed symbols with.
- A relocatable link is no way: MinGW's `ld -r --gc-sections`, then `objcopy
  --keep-global-symbols`, left `__imp_` locals of std's statics without a section, and
  MSVC's linker has none.

## Waiting

- Maintainer: hide the Windows libraries or drop this quest. No consumer yet links two Rust
  static libraries into one Windows program; renaming likely makes it small, but it needs
  the import members told apart and a run on the Windows leg. Recommended: drop it until a
  consumer needs it, and keep the README's note.
