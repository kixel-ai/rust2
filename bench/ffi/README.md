# Binding benchmark

What [rust2-ffi](../../ffi/README.md)'s bindings cost over Rust, per language, on the moq
stand-in `just ffi bench` exports, against BoltFFI's numbers below as targets.

```bash
nix develop .#bench --command just ffi bench  # build, run 3 rounds, and write ffi/target/bench/report.md
nix develop .#bench --command just ffi bench-run 5  # rerun what bench-build built, 5 rounds
```

`ffi/moq/bench/export.*` time the same cases in each language, `export.rs` the floor
on the stand-in's API from Rust, and `moq-c.c` moq's hand-written rs/moq-c, which
has only the stamped write. Each program runs its cases twice and prints the second,
so a JIT has warmed up, in wall time and in the process's CPU time on every thread.
`just ffi bench-run` runs each program once a round, records the 1-minute load average
over each run, and [`report.py`](report.py) prints each case's median over the rounds
beside the median load, then the export's sizes. Nightly runs it on arm64 Linux into
the run's summary; nothing fails on a number.

| case | what it times | BoltFFI's nearest |
|---|---|---|
| handle call | `track.is_closed()`: one borrowed handle, a bool back, and moq's own read lock | `noop()` |
| u64 getter | `demand.sequence()`: one borrowed handle and a field read, so all but 0.4 ns is the binding's | `group.sequence()` |
| record return | `track.largest()`: an `Option` of two `u64`s, copied across | `track.info()`, a record |
| object return | `track.info()`: a new handle, freed | none |
| write | `group.write_frame(at, frame)` with one `Timestamp` handle and 100 B, copied into Rust | `write_frame` |
| stamped write | the same with a new `Timestamp` per frame, as a caller holding a `u64` must | none |
| ready read | a 100 B frame written, then read through a new future ready at its first poll | `await noop_async()`, plus a write |
| woken read | a read started before its frame is written, as the language starts work concurrently, so it suspends and the write's wake resumes it | `await yield_once()`, plus a write |
| payload | `frame.payload` of 100 B, handed to the language | none |
| throughput | groups of 100 frames written and read, each payload's first byte touched, about 64 MiB | throughput |

A woken read starts its read the language's own way: by polling in C, C++, and Rust; a
`Task` and a yield in Swift and Python; a goroutine and a yield in Go; an undispatched
coroutine in Kotlin; and an eager call in Dart. So it measures a task's start and its
wake, and in Swift, Python, and Go, a read the scheduler had not yet started is ready
instead of woken.

## rust2-ffi

Measured 2026-11-13 at rust2 `534ef663`, moq `ffcb7cd9`, on an Intel Core Ultra 7 270K
Plus (24 threads), Linux, on a quiet host: the 1-minute load average, in parentheses
beside each number, ran 2 to 3, and each cell is the median of 3 rounds. CPU time leaves
out time the program waited preempted, so take the CPU tables first. Rust
1.101.0-nightly 2026-10-14 (the export's default `release` profile, no LTO), GCC 16.2.0,
CPython 3.14.10, OpenJDK 21.0.13 with Kotlin 2.4.21 and kotlinx-coroutines 1.10.2, Swift
6.4, Go 1.27.3, Dart 3.13.0.

Against main (`7008d1d3`) at the same load, measured in turn four times, a 100 B write
costs C 4 to 6 ns less, and Swift 6 and Dart 10 more in every pair: per-call cost, since
each copies a 100 B frame as often as main does. Kotlin and Go each moved within the 15 to
25 ns its own runs spread, so no row resolves the copy Kotlin lost at 100 B or 1 KiB.
Bytes Rust returned and a binding passes back cross with no copy, which `ffi/tests`
checks by address and by drop count.

CPU time counts every thread of the process: the JVM's JIT and GC threads, Go's GC, and the
Swift global executor's workers. So Kotlin's CPU column runs above its wall column, and
Swift's woken read spends 14.1 us of CPU for 4.8 us of wall. Throughput from 16 KiB up is
bound by memory bandwidth: its cells moved up to 3x between runs even on this quiet host,
so read only the 100 B and 1 KiB rows as the bindings' cost.

### Calls (ns per call, CPU time; load average)

| call | Rust | C | C++ | moq-c | Kotlin | Python | Swift | Go | Dart |
|---|---|---|---|---|---|---|---|---|---|
| `track.is_closed()`, a handle call | 9.0 (3) | 18 (3) | 18 (3) |  | 60 (3) | 55 (3) | 26 (3) | 47 (3) | 45 (3) |
| `demand.sequence()`, a u64 getter | 0.4 (3) | 10 (3) | 10 (3) |  | 50 (3) | 49 (3) | 9.9 (3) | 38 (3) | 28 (3) |
| `track.largest()`, a record return | 18 (3) | 28 (3) | 29 (3) |  | 150 (3) | 99 (3) | 36 (3) | 69 (3) | 55 (3) |
| `track.info()`, an object return | 1.1 (3) | 31 (3) | 29 (3) |  | 190 (3) | 104 (3) | 38 (3) | 195 (3) | 239 (3) |
| `group.write_frame(at, frame)`, 100 B | 121 (3) | 143 (3) | 148 (3) |  | 240 (3) | 198 (3) | 179 (3) | 285 (3) | 214 (3) |
| the same, with a new `Timestamp` each | 122 (3) | 162 (3) | 170 (3) | 219 (3) | 390 (3) | 339 (3) | 208 (3) | 470 (3) | 455 (3) |
| a 100 B frame written and read, ready | 194 (3) | 301 (3) | 327 (3) |  | 650 (3) | 552 (3) | 471 (3) | 721 (3) | 739 (3) |
| the same, read after a wake | 308 (3) | 444 (3) | 450 (3) |  | 1,590 (3) | 6,917 (3) | 14,061 (3) | 4,765 (3) | 1,665 (3) |
| `frame.payload`, 100 B | 10 (3) | 20 (3) | 25 (3) |  | 70 (3) | 155 (3) | 85 (3) | 128 (3) | 68 (3) |

### Throughput (MB per CPU second, write plus ready read; load average)

| frame | Rust | C | C++ | Kotlin | Python | Swift | Go | Dart |
|---|---|---|---|---|---|---|---|---|
| 100 B | 563 (3) | 326 (3) | 297 (3) | 118 (3) | 138 (3) | 170 (3) | 114 (3) | 126 (3) |
| 1 KiB | 4,148 (3) | 2,823 (3) | 2,589 (3) | 1,341 (3) | 1,319 (3) | 1,218 (3) | 765 (3) | 1,104 (3) |
| 16 KiB | 11,331 (3) | 10,982 (3) | 11,715 (3) | 6,554 (3) | 8,784 (3) | 2,434 (3) | 1,648 (3) | 5,682 (3) |
| 64 KiB | 2,679 (3) | 8,637 (3) | 8,763 (3) | 2,185 (3) | 9,079 (3) | 2,522 (3) | 2,351 (3) | 6,767 (3) |
| 1 MiB | 3,742 (3) | 3,785 (3) | 6,300 (3) | 3,146 (3) | 3,586 (3) | 3,531 (3) | 4,357 (3) | 3,926 (3) |

### Calls (ns per call, wall time; load average)

| call | Rust | C | C++ | moq-c | Kotlin | Python | Swift | Go | Dart |
|---|---|---|---|---|---|---|---|---|---|
| `track.is_closed()`, a handle call | 9.0 (3) | 18 (3) | 18 (3) |  | 31 (3) | 55 (3) | 26 (3) | 46 (3) | 41 (3) |
| `demand.sequence()`, a u64 getter | 0.4 (3) | 10 (3) | 10 (3) |  | 24 (3) | 49 (3) | 9.9 (3) | 38 (3) | 28 (3) |
| `track.largest()`, a record return | 18 (3) | 28 (3) | 29 (3) |  | 74 (3) | 99 (3) | 36 (3) | 65 (3) | 54 (3) |
| `track.info()`, an object return | 1.1 (3) | 31 (3) | 29 (3) |  | 89 (3) | 104 (3) | 38 (3) | 181 (3) | 238 (3) |
| `group.write_frame(at, frame)`, 100 B | 122 (3) | 143 (3) | 148 (3) |  | 236 (3) | 198 (3) | 179 (3) | 280 (3) | 211 (3) |
| the same, with a new `Timestamp` each | 122 (3) | 162 (3) | 170 (3) | 219 (3) | 331 (3) | 339 (3) | 208 (3) | 445 (3) | 445 (3) |
| a 100 B frame written and read, ready | 194 (3) | 301 (3) | 327 (3) |  | 540 (3) | 552 (3) | 471 (3) | 694 (3) | 732 (3) |
| the same, read after a wake | 308 (3) | 444 (3) | 450 (3) |  | 1,574 (3) | 6,918 (3) | 4,757 (3) | 3,740 (3) | 1,652 (3) |
| `frame.payload`, 100 B | 10 (3) | 20 (3) | 25 (3) |  | 67 (3) | 155 (3) | 85 (3) | 115 (3) | 67 (3) |

### Throughput (MB/s, write plus ready read; load average)

| frame | Rust | C | C++ | Kotlin | Python | Swift | Go | Dart |
|---|---|---|---|---|---|---|---|---|
| 100 B | 563 (3) | 326 (3) | 297 (3) | 166 (3) | 138 (3) | 169 (3) | 121 (3) | 127 (3) |
| 1 KiB | 4,144 (3) | 2,823 (3) | 2,589 (3) | 1,404 (3) | 1,319 (3) | 1,218 (3) | 872 (3) | 1,130 (3) |
| 16 KiB | 11,330 (3) | 10,982 (3) | 11,715 (3) | 6,096 (3) | 8,784 (3) | 2,433 (3) | 2,196 (3) | 5,729 (3) |
| 64 KiB | 2,679 (3) | 8,637 (3) | 8,762 (3) | 6,651 (3) | 9,079 (3) | 2,522 (3) | 2,429 (3) | 6,971 (3) |
| 1 MiB | 3,742 (3) | 3,785 (3) | 6,298 (3) | 3,346 (3) | 3,586 (3) | 3,530 (3) | 4,847 (3) | 3,995 (3) |

### Size

| binding | files | raw | gzip | Rust glue, raw / gzip |
|---|---|---|---|---|
| C | 1 | 1,633 KiB | 242 KiB | 2,416 KiB / 134 KiB |
| C++ | 1 | 2,865 KiB | 320 KiB | none |
| Kotlin | 2 | 2,498 KiB | 361 KiB | 1,720 KiB / 79 KiB |
| Python | 1 | 1,454 KiB | 225 KiB | 3,762 KiB / 294 KiB |
| Swift | 3 | 2,147 KiB | 302 KiB | none |
| Go | 23 | 3,554 KiB | 435 KiB | none |
| Dart | 1 | 4,034 KiB | 376 KiB | 1,643 KiB / 62 KiB |

| native library | as built | stripped |
|---|---|---|
| `libmoq.a`, the C ABI that C, C++, Swift, and Go link | 193,455 KiB | 104,224 KiB |
| `libmoq.so`, with the Kotlin and Dart glue | 61,361 KiB | 46,029 KiB |
| `python/libmoq.so`, the CPython extension | 56,145 KiB | 42,166 KiB |

The stand-in exports all of moq-net, kio, and moq-pattern, and hang, moq-mux, and
moq-tokio's session side, far more than moq-ffi's UniFFI interface, so its sizes do not
compare with the fork's 653 / 914 / 684 KiB below. "Rust glue" is the generated crate's
source each binding builds with: the C ABI's `src/lib.rs` and runtime, which every binding
but Python's links, and `src/jni`, `src/python`, and `src/dart`. The native libraries are
the export's default `release` build, which links all of moq's crates; "stripped" is
`strip --strip-all` on a copy.

### What a call spends

From the CPU tables of 2026-11-10, measured at a load average of 22 to 29, read against
the code each call runs. In order of what it costs moq's
frame path:

1. **A handle per frame timestamp.** `Timestamp` is `Copy` in Rust, but its private fields
   make it an object, so each stamped frame makes and frees a handle: 21 ns in C, 27 in
   Swift, 140 in Python, 250 in Kotlin, 260 in Go, and 245 in Dart (stamped write minus
   write). In Go, Dart, and Kotlin that doubles the write.
2. **Ownership of a returned object.** An object return costs C 25 ns, but Go 263 and
   Dart 242, ten times C: each new Go object registers `runtime.AddCleanup` and each Dart
   one a `NativeFinalizer`, beside a handle allocation. Kotlin's 137 ns wall (210 CPU) is a phantom
   reference linked and unlinked under one global lock. Every returned frame, group, and future pays it.
3. **A woken read.** C's 427 ns is Rust's 329 plus a future's crossings. Kotlin's 1.6 us and
   Dart's 1.8 us add their event loops; Go's 5.1 us and Swift's 5.7 us wall (16.6 us CPU)
   are a goroutine or `Task` and a cross-thread wake per read, and Python's 6.9 us is
   asyncio's, as BoltFFI's Python measured 10 us. This case starts a task per read; a
   reader looping on one task would skip the start, and is what to measure before changing
   a generator.
4. **A future per read.** A ready read costs C 155 ns over the write where Rust costs 73:
   the future's three allocations and three crossings. Each language adds 160 (Swift) to 420 (Go) ns
   more in its own wrapper, so the C ABI's share is not yet where a per-handle poll would pay.
5. **Kotlin's record codec.** A record return costs Kotlin 113 ns wall against C's 31,
   through the byte-array codec; Swift, which reads the C struct, costs 37.
6. **The handle borrow at the C ABI.** The u64 getter costs Rust 0.4 ns and C 10: the
   C ABI's borrow of each handle, a compare-and-swap in and a `fetch_sub` out of its
   state, is the whole call. Every handle argument pays it again.
7. **The binding's own call.** Over C's getter: Swift adds 8 ns (its hold count's two
   atomics, since dropped from all but its classes, as below), Kotlin 14 wall (JNI and its hold count), Dart 19 (an `_Arena` per call and
   its hold), Go 31 (cgo, `defer`, and its hold), and Python 37 (argument parsing and the
   GIL released and taken back, so Rust may call Python from another thread). BoltFFI's
   Python getter, which holds the GIL, measured 63 ns.

### Per-handle baseline

For [checks-raise](/quest/m1/ffi-export/checks-raise.md)'s one accounting: the u64
getter is one borrowed handle and nothing else, so its row less Rust's 0.4 ns is what a
handle costs a call. C, C++: 10 ns, the runtime's two atomics. Swift: 18, its hold count
adding two more. Kotlin: 24 wall, Go 41, Dart 29, Python 47, each with its call's own
transition besides.

Swift has since made each object a noncopyable struct and dropped its hold count, but for
a class: a `Copy` object or one thrown as an error. Measured 2026-11-11 at that change, 5
rounds interleaved with C at a load average of 7 to 12, its getter costs 10 ns, as C's
does, and `track.is_closed()`, on `TrackProducer`, still a class since moq throws it, 26
against C's 18, as in the table. A change to the accounting moves this row and the handle call's by the
same amount; measure both, CPU first, at the load beside them.

### Against the targets

Where the cases match, on wall time: Kotlin's getter costs 24 ns against BoltFFI's 5.8,
its record return 113 against 61, its write 107 ns over Rust against BoltFFI's 56, and 100 B
throughput 170 MB/s against 105. Python's getter costs 47 ns against 63, its record return
97 against 1,120, and 100 B throughput 138 MB/s against 12.1. So Python meets BoltFFI's
and Kotlin's calls trail it by 2 to 4 times, behind JNI's per-call cost and the hold count.

## BoltFFI comparison

The targets. Before forking UniFFI, moq compared it with [BoltFFI](https://github.com/boltffi/boltffi) 0.31.1 on a slice of moq-ffi bound by each generator, measured from Python and Kotlin with a Rust floor.
Its harness is deleted; [moq-dev/moq@0f986d9](https://github.com/moq-dev/moq/tree/0f986d9fdb7c074b0bd4b34739f7199c3bf2d9e5/bench/ffi-gen) holds it if a decision needs fresh numbers.
BoltFFI's numbers are the targets for rust2-ffi's generators: its speed comes from calling conventions (a CPython extension instead of `ctypes`, JNI instead of JNA, and bulk byte copies), not from its core.

Measured 2026-10-16 on an Intel Core Ultra 7 270K Plus (24 threads), Linux, at moq `b4eab0db1`, on a shared host (load average 8 to 29), so cells are medians of 3 rounds.
Rust 1.98.1, CPython 3.14.7, OpenJDK 17 with Kotlin 2.4.20 and kotlinx-coroutines 1.11.0, JNA 5.19.1.
Gain is how many times better BoltFFI is than UniFFI.

### Calls (ns per call)

| call | Rust | Python UniFFI | Python BoltFFI | gain | Kotlin UniFFI | Kotlin BoltFFI | gain |
|---|---|---|---|---|---|---|---|
| `noop()` | 0.2 | 834 | 21.6 | 39x | 3,200 | 5.6 | 569x |
| `group.sequence()` | 0.4 | 1,920 | 63 | 30x | 7,440 | 5.8 | 1,290x |
| `track.info()`, a record return | 12 | 4,890 | 1,120 | 4.4x | 10,400 | 61 | 171x |
| `write_frame`, a 100 B record argument | 78 | 12,000 | 667 | 18x | 12,800 | 134 | 95x |
| `await noop_async()` | 0.3 | 7,190 | 5,330 | 1.3x | 8,320 | 342 | 24x |
| `await yield_once()`, one wakeup | 44 | 12,600 | 10,000 | 1.3x | 6,510 | 587 | 11x |

In Python, both generators sit at 5 to 13 us per await, which points at asyncio rather than the boundary.

### Throughput (MB/s, write plus async read)

| frame | Rust | Python UniFFI | Python BoltFFI | gain | Kotlin UniFFI | Kotlin BoltFFI | gain |
|---|---|---|---|---|---|---|---|
| 100 B | 473 | 3.8 | 12.1 | 3.2x | 2.9 | 105 | 36x |
| 1 KiB | 3,340 | 36 | 117 | 3.3x | 36 | 954 | 27x |
| 16 KiB | 10,400 | 323 | 1,410 | 4.4x | 286 | 2,730 | 9.6x |
| 64 KiB | 8,930 | 531 | 2,840 | 5.4x | 448 | 2,760 | 6.2x |
| 1 MiB | 6,120 | 605 | 3,220 | 5.3x | 562 | 1,560 | 2.8x |

### CPU (ms per MB)

| frame | Rust | Python UniFFI | Python BoltFFI | gain | Kotlin UniFFI | Kotlin BoltFFI | gain |
|---|---|---|---|---|---|---|---|
| 100 B | 2.1 | 261 | 83 | 3.2x | 1,290 | 15 | 85x |
| 1 KiB | 0.30 | 28 | 8.6 | 3.3x | 76 | 1.2 | 61x |
| 16 KiB | 0.10 | 3.1 | 0.71 | 4.4x | 10 | 0.39 | 27x |
| 64 KiB | 0.11 | 1.9 | 0.35 | 5.4x | 3.8 | 0.37 | 10x |
| 1 MiB | 0.16 | 1.7 | 0.31 | 5.3x | 2.3 | 0.66 | 3.4x |

At 100 B, Kotlin through UniFFI spends about 130 us of CPU per frame written and read, so 1,000 audio frames a second cost 13% of a core; BoltFFI spends 1.5 us.

### Size

Native libraries are `release` (fat LTO, `strip = "debuginfo"`, as moq-ffi ships); "stripped" is `strip --strip-all` on a copy.

| native library | as built | stripped |
|---|---|---|
| slice, UniFFI | 17,571 KiB | 14,553 KiB |
| slice, BoltFFI | 17,472 KiB | 14,482 KiB |
| BoltFFI's Python extension (`_native`), extra | 240 KiB | 64 KiB |
| BoltFFI's Kotlin JNI glue, extra | 52 KiB | 38 KiB |
| moq-ffi, default features | 34,100 KiB | 29,396 KiB |
| moq-ffi, no default features | 30,625 KiB | 26,416 KiB |

| generated bindings | files | raw | gzip |
|---|---|---|---|
| slice, Python: UniFFI / BoltFFI | 1 / 2 | 85 / 36 KiB | 12 / 7 KiB |
| slice, Kotlin: UniFFI / BoltFFI | 1 / 1 | 152 / 57 KiB | 17 / 9 KiB |
| slice, Swift: UniFFI / BoltFFI | 3 / 2 | 101 / 51 KiB | 14 / 8 KiB |
| moq-ffi, UniFFI: Python / Kotlin / Swift | 1 / 1 / 3 | 653 / 914 / 684 KiB | 66 / 73 / 72 KiB |

Browser builds use opt-level z after wasm-bindgen and no wasm-opt, as moq-wasm shipped before moq removed it; the stripped columns drop the `name` section.

| browser module | .wasm as built | .wasm stripped | stripped, gzip | JS glue | JS glue, gzip |
|---|---|---|---|---|---|
| slice, UniFFI | 1,245 KiB | 515 KiB | 174 KiB | not generated | not generated |
| slice, BoltFFI | 1,148 KiB | 456 KiB | 155 KiB | 126 KiB | 25 KiB |
| moq-ffi, UniFFI | 6,218 KiB | 4,403 KiB | 1,575 KiB | not generated | not generated |
| moq-wasm, wasm-bindgen | 2,116 KiB | 2,115 KiB | 691 KiB | 38 KiB | 8 KiB |
