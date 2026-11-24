# [M] Every static library the builder ships exports only the C ABI

## Goal

Every static library `ffi/release/package.py` packs for an Apple, Android, or Linux target,
not just the C package's, is prelinked so its only global symbols are the C ABI's
`<lib>_*`: the Swift package's XCFramework slices, the Go module's libraries, and React Native's XCFramework and Android
libraries. An app can then link any of them beside another Rust static library, built by
any toolchain, in either order. Each build checks this as the C package's `coexist` does.
MSVC libraries are outside the goal, since MSVC has no relocatable link; the Plan decides
the Go module's `windows-gnu` library.

## Plan

Found by CodeRabbit on [#994](https://github.com/kixel-ai/rust2/pull/994), which packed
React Native's libraries unhidden, as the Swift and Go packages already were.

Reproduced on 2026-11-24: two `staticlib`s built by rustc 1.100.0 and 1.99.1, linked into
one C program, fail in both orders under GNU ld and lld with a duplicate
`rust_eh_personality`. Built by one toolchain, without LTO, they link. Two Rust packages
from different authors meet this in one app. React Native's module is a `STATIC` CMake
target that autolinking pulls into the app's `libappmodules.so`
(`ffi/src/jsi.rs`), and CocoaPods and SwiftPM link static libraries into the app binary.
Hiding only removes symbols the C ABI's callers never reference, so shipping unhidden first
breaks no consumer later.

- Hide once, where the slices are shared: React Native reuses the Swift package's iOS
  slices and the Go module's Android build, so the hidden library serves every package.
- `hide()` reads the C ABI from a cdylib and handles only macOS arm64 and the host's ELF.
  It needs iOS device and simulator targets (`cc -r` with `-target`), and Android's four
  ABIs, which the host's binutils cannot read: use the NDK's `ld.lld -r` and
  `llvm-objcopy`, then check whether binutils' weak-symbol workaround still applies.
  Where no cdylib is built, take the names from the header or the archive's
  `<lib>_*` globals.
- The check: the C package's `coexist` builds its other library with the same toolchain,
  which by the reproduction above may pass unhidden on ELF without LTO. Confirm what it
  catches, and build the other library with a second toolchain if it catches nothing. Where
  the build host cannot run the target, the check links both orders without running.
- Windows: MSVC has no relocatable link, so its libraries stay unhidden, as the C
  package's are; decide whether the Go module's `windows-gnu` library can be hidden.
