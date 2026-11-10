# [M] The exporter on the binding benchmark

## Goal

The [binding benchmark](/bench/ffi/README.md) measures [rust2-ffi](/ffi/README.md)'s
bindings and commits the numbers: per language, the calls and throughput cases of its
BoltFFI table against a Rust floor, and each generated binding's size, raw and gzipped,
for the moq stand-in. Its report ranks what a call spends, and plans the speed work the
numbers justify as quests.

## Plan

Found in the 2026-11-09 quest audit: no exporter measurement is committed. The benchmark
measures moq-ffi over the UniFFI fork, `just ffi bench` (`ffi/justfile`,
`ffi/moq/bench/export.*`) prints timings and records nothing, and the epic's rule that
every generator "is measured on the binding benchmark" and "reports its generated size"
had no owner. Speed ranks above ergonomics here (AGENTS.md), so it needs a baseline.

- The targets: BoltFFI's cells in `bench/ffi/README.md` (Kotlin `noop()` 5.6 ns, a record
  return 61 ns; Python `noop()` 21.6 ns), and UniFFI's unreleased JNI bindgen, which its
  maintainer measured at 7.7 ns a no-op against BoltFFI's 8.2
  ([gist](https://gist.github.com/bendk/5054d1477361125ef430e2ccabfd5aed), from
  [mozilla/uniffi-rs#2911](https://github.com/mozilla/uniffi-rs/pull/2911)). The fork's
  sizes for moq-ffi: Python 653 KiB, Kotlin 914 KiB, Swift 684 KiB.
- The structural costs the audit read off the code, for the report to confirm or drop:
  - an owned `String`, `Vec`, or `Bytes` argument is always copied at the C ABI
    (`glue.rs`), and again in Kotlin and Dart, on moq's frame-write path;
  - Kotlin crosses records, `Duration`, and `SocketAddr` through a byte-array codec, about
    four JVM allocations and three copies an argument, resolves JNI symbols by name, and
    takes a global lock on each handle's creation and free (`kotlin.rs`, `rt.kt`, `jni.rs`);
  - a future costs three heap allocations and three crossings when ready at once
    (`rt.rs`, `glue.rs`), and a hot read makes one per read: measure it before adding a
    per-handle poll;
  - each returned buffer is a `Box` and a free crossing, each returned object an
    `Arc::new`, and each object lent to a closure an `Arc::new` per call (`rt.rs`);
  - Dart hands each call to a worker thread under a global mutex while a relay is held
    (`ffi/runtime/dart.rs`), and Go takes a global mutex on lent-buffer frees (`rt.go`).
  The doubled borrow accounting is [checks-raise](/quest/m1/ffi-export/checks-raise.md)'s.
- No threshold is set, as in the other benchmarks: the numbers inform the quests the
  report plans.

Public API: none.

## Required

- [Delete the UniFFI fork](/quest/m0/delete-uniffi.md) - the benchmark's harness stops building the fork, so this rebuilds it on the exporter
