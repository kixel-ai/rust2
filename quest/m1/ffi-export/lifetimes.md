# [M] Lifetimes with no undefined behavior

## Goal

No generated binding in a garbage-collected or reference-counted language lets a program
reach undefined behavior through an object or its bytes: Kotlin's and Go's `close()` leave
every later call an error, a lent object's lend never ends under a call, and returned bytes
are owned by the language's memory manager, so no view outlives its buffer. C++ owns
handles through the standard library's smart pointers. Each rule is proven by a test that
uses an object after its free, races a free against a call, and keeps a view past its owner.

## Plan

Decided with the maintainer on 2026-11-01, from an audit of each generator: avoid an
explicit `close()` where the language frees deterministically, since without Rust's
`close(self)` a program can keep a reference to a closed object. A `close()` stays only
where the language's GC can't free promptly, and then every call after it returns or throws
an error, never a crash or undefined behavior.

- **Kotlin**: objects keep `close()`, which already throws `IllegalStateException` after, and
  a racing close waits for the call. Its bytes over 1 KiB, a direct `ByteBuffer` that
  `Bytes.close()` or the GC can free under a live slice, become owned by the GC as Go's are:
  no `close()`, freed by a cleaner on the buffer that its slices and views keep alive. Verify
  that on ART at API 24 and 35, or else read them through methods with no buffer escaping.
  AOSP's source says ART fails it before API 27: a slice shares the original's
  `MemoryRef`, which holds the buffer `NewDirectByteBuffer` made only from Android 8.1
  (`new MemoryRef(addr, this)`), so on API 24 to 26 a cleaner on the original can run
  under a live slice.
- **Go**: settled in [#411](https://github.com/moq-dev/rust2/pull/411): objects keep `Close`,
  with every later call returning an error, and bytes are a GC-owned `Bytes` with no raw
  slice, read through methods and copied by `Copy()`.
- **A lend's end**: Swift, Python, Kotlin, and Go close an object Rust lent a closure once
  the closure returns, and Rust frees the lent handle once its own check finds no borrow.
  A call on another thread that held the binding's handle before that close but has not
  yet borrowed in Rust then reaches the freed handle. Closing a lent handle that a call
  still holds must fail loud, as Rust's own check does for a call already inside Rust.
  Reproduce it deterministically first, such as through a test hook between the binding's
  hold and the C call.
- **C**: out of scope; its `_free` follows C's norms.

Public API: breaking for Kotlin bytes; it gets a `MIGRATE.md` entry.
