# [M] FFI test programs build once per input change

## Goal

A second `just test` of `ffi` with nothing changed exports, builds, compiles, and links
nothing new for [`ffi/tests/fixtures.rs`](/ffi/tests/fixtures.rs) and its
[shapes](/ffi/tests/fixtures/shapes.rs): the glue crate's libraries and each language's
test program (C, C++, Kotlin, Python's extension, Swift, Go, Dart) are reused, and still run
and assert as before. A change to anything a step reads, such as a test source, the
exporter, the generated binding, the static library, or the compiler, rebuilds that step
and only the steps downstream of it. The PR reports the bytes written and the wall time
of a warm second run before and after.

## Plan

Found by the 2026-11-13 write audit of the `nixos` host
([kixel-ai/dotfiles#135](https://github.com/kixel-ai/dotfiles/pull/135)): a 15-minute trace
charged 2.8 GB of writes to `target/tmp/api`, across agents' `just test`, rust2-ci, and a
rust2 land fixer. Agents write about 250 GB a day on that host, and this is the part a
warm run should not write at all. No single PR caused it: the fixtures have rebuilt
everything since they were added.

Decided with the maintainer on 2026-11-13: ✅ add this quest, under the host's write
cuts; milestone ✅ m0 (rejected: m1), since every agent's `just test` pays these writes and
the wait.

What is already true, from reading the code on 2026-11-13:

- `CARGO_TARGET_TMPDIR` is Cargo's persistent `target/tmp`, not a fresh directory, so the
  outputs survive between runs; the steps just never check them.
- Every export rewrites every generated file (`ffi/src/driver.rs`'s `write`) and deletes
  the C++ and Go directories first (`ffi/src/glue.rs`), so each file's mtime moves even
  when its content doesn't. Cargo fingerprints path sources by mtime, so the glue crate
  likely recompiles and relinks both feature builds each run. `api()` also moves the
  Python extension out of Cargo's `target/debug`.
- Each language's compile and link (`cc`, `c++` per standard, `kotlinc`, `swiftc`, `go
  build`, `dart compile`) runs unconditionally.

Measure first: trace one warm second run, with bytes written and wall time per step: the
export, each glue build, and each language's compile. Fix what dominates. The
recommendations below are a starting point, not settled.

- Recommended: the exporter writes a file only when its content differs, and removes only
  the files an export no longer produces rather than whole directories. Then Cargo's own
  fingerprint skips the glue build, and a consumer that re-exports before each build
  gains the same. That changes rust2-ffi's behavior, not its API.
- Recommended: each test program's step is skipped when a hash of what it reads matches
  a stamp beside its output: its sources, the generated files it includes, the library it
  links, the compiler's path and version, and its arguments. Keep the outputs in
  `target/tmp`, which already persists, and is cleared with the rest of `target`. Rejected
  for now: each language's own incremental build (Gradle, SwiftPM, Go's build cache
  alone), since that adds a build system per language for what one stamp covers; Go's
  cache may already make its compile cheap, which the measurement shows.
- An mtime check is not enough on its own: the exporter's rewrite moves mtimes, and a
  stale program that passes would hide a real break, so key on content.
- A test that edits a cached output must edit a copy: `api_kotlin`'s contract-mismatch
  case rewrites `Native.class` in place, so a run that stops before restoring it would
  leave every later run reusing the broken class.
- A death test runs the same cached program; caching its build changes nothing about
  what it asserts.

Expect to merge in `fixtures.rs` and the test programs with
[kixel-ai/rust2#767](https://github.com/kixel-ai/rust2/pull/767)'s non-dumpable
`broken(case)` entry and with [failed checks raise](/quest/m1/ffi-export/checks-raise.md)
([kixel-ai/rust2#730](https://github.com/kixel-ai/rust2/pull/730)), which rewrites many of
the same cases. A content key includes their edits to the test sources, so neither makes
a cached program stale.

Out of scope: warming the queue's or a land fixer's `target`, which kixel-ai/dotfiles
owns; a cold run, which builds everything once as today.

Public API: none; the exporter may stop rewriting unchanged files.

## Related

- [The moq export gates exporter merges](/quest/m0/ffi-moq-gate.md) - adds moq's export and C test to the gate, which the same skip should cover
- [Failed checks raise](/quest/m1/ffi-export/checks-raise.md) - rewrites the same test programs and fixture cases
