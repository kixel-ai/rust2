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

Measured 2026-11-10 at rust2 `7383790d` plus this harness, moq `ffcb7cd9`, on an Intel
Core Ultra 7 270K Plus (24 threads), Linux, **on a loaded shared host**: the 1-minute load
average, in parentheses beside each number, ran 22 to 29 throughout, so each cell is the
median of 5 rounds. CPU time leaves out time the program waited preempted, so it moves less
under load than wall time, but caches and SMT siblings stay shared; take the CPU tables
first. Rust 1.101.0-nightly 2026-10-14 (the export's default `release` profile, no LTO), GCC 16.2.0,
CPython 3.14.8, OpenJDK 21.0.12.1 with Kotlin 2.4.21 and kotlinx-coroutines 1.10.2,
Swift 6.3.3, Go 1.26.10, Dart 3.13.0.

CPU time counts every thread of the process: the JVM's JIT and GC threads, Go's GC, and the
Swift global executor's workers. So Kotlin's CPU column runs above its wall column, and
Swift's woken read spends 16.6 us of CPU for 5.7 us of wall. Throughput from 16 KiB up is
bound by memory bandwidth, which the host's load shares: its cells moved 2x between runs,
the Rust floor's included, so read only the 100 B and 1 KiB rows as the bindings' cost.

### Calls (ns per call, CPU time; load average)

| call | Rust | C | C++ | moq-c | Kotlin | Python | Swift | Go | Dart |
|---|---|---|---|---|---|---|---|---|---|
| `track.is_closed()`, a handle call | 9.1 (26) | 18 (25) | 18 (26) |  | 60 (26) | 53 (25) | 26 (27) | 49 (26) | 47 (26) |
| `demand.sequence()`, a u64 getter | 0.4 (26) | 10 (25) | 10 (26) |  | 50 (26) | 47 (25) | 18 (27) | 41 (26) | 29 (26) |
| `track.largest()`, a record return | 18 (26) | 31 (25) | 31 (26) |  | 230 (26) | 97 (25) | 36 (27) | 76 (26) | 56 (26) |
| `track.info()`, an object return | 1.1 (26) | 25 (25) | 25 (26) |  | 210 (26) | 97 (25) | 50 (27) | 263 (26) | 242 (26) |
| `group.write_frame(at, frame)`, 100 B | 132 (26) | 148 (25) | 161 (26) |  | 230 (26) | 196 (25) | 194 (27) | 270 (26) | 209 (26) |
| the same, with a new `Timestamp` each | 133 (26) | 169 (25) | 177 (26) | 267 (28) | 480 (26) | 336 (25) | 221 (27) | 526 (26) | 454 (26) |
| a 100 B frame written and read, ready | 205 (26) | 303 (25) | 337 (26) |  | 690 (26) | 560 (25) | 509 (27) | 841 (26) | 735 (26) |
| the same, read after a wake | 329 (26) | 427 (25) | 461 (26) |  | 1,630 (26) | 6,912 (25) | 16,580 (27) | 4,711 (26) | 1,776 (26) |
| `frame.payload`, 100 B | 11 (26) | 23 (25) | 24 (26) |  | 70 (26) | 150 (25) | 96 (27) | 165 (26) | 67 (26) |

### Throughput (MB per CPU second, write plus ready read; load average)

| frame | Rust | C | C++ | Kotlin | Python | Swift | Go | Dart |
|---|---|---|---|---|---|---|---|---|
| 100 B | 472 (26) | 303 (25) | 300 (26) | 105 (26) | 138 (25) | 162 (27) | 106 (26) | 120 (26) |
| 1 KiB | 3,444 (26) | 2,394 (25) | 2,267 (26) | 1,341 (26) | 1,240 (25) | 1,135 (27) | 713 (26) | 991 (26) |
| 16 KiB | 7,324 (26) | 14,822 (25) | 6,923 (26) | 6,554 (26) | 9,460 (25) | 2,156 (27) | 1,820 (26) | 5,536 (26) |
| 64 KiB | 2,552 (26) | 10,612 (25) | 6,441 (26) | 3,277 (26) | 9,468 (25) | 2,206 (27) | 2,382 (26) | 6,574 (26) |
| 1 MiB | 3,647 (26) | 3,596 (25) | 5,546 (26) | 3,146 (26) | 3,464 (25) | 3,354 (27) | 2,767 (26) | 3,812 (26) |

### Calls (ns per call, wall time; load average)

| call | Rust | C | C++ | moq-c | Kotlin | Python | Swift | Go | Dart |
|---|---|---|---|---|---|---|---|---|---|
| `track.is_closed()`, a handle call | 9.1 (26) | 18 (25) | 18 (26) |  | 32 (26) | 53 (25) | 26 (27) | 48 (26) | 42 (26) |
| `demand.sequence()`, a u64 getter | 0.4 (26) | 10 (25) | 10 (26) |  | 24 (26) | 47 (25) | 18 (27) | 41 (26) | 28 (26) |
| `track.largest()`, a record return | 18 (26) | 31 (25) | 31 (26) |  | 113 (26) | 97 (25) | 37 (27) | 73 (26) | 56 (26) |
| `track.info()`, an object return | 1.1 (26) | 25 (25) | 25 (26) |  | 137 (26) | 97 (25) | 50 (27) | 255 (26) | 243 (26) |
| `group.write_frame(at, frame)`, 100 B | 132 (26) | 148 (25) | 161 (26) |  | 239 (26) | 196 (25) | 194 (27) | 274 (26) | 208 (26) |
| the same, with a new `Timestamp` each | 133 (26) | 172 (25) | 178 (26) | 233 (27) | 331 (26) | 336 (25) | 224 (27) | 539 (26) | 447 (26) |
| a 100 B frame written and read, ready | 205 (26) | 303 (25) | 341 (26) |  | 544 (26) | 560 (25) | 517 (27) | 903 (26) | 731 (26) |
| the same, read after a wake | 330 (26) | 431 (25) | 474 (26) |  | 1,600 (26) | 6,912 (25) | 5,697 (27) | 5,061 (26) | 1,766 (26) |
| `frame.payload`, 100 B | 11 (26) | 23 (25) | 24 (26) |  | 70 (26) | 150 (25) | 96 (27) | 150 (26) | 67 (26) |

### Throughput (MB/s, write plus ready read; load average)

| frame | Rust | C | C++ | Kotlin | Python | Swift | Go | Dart |
|---|---|---|---|---|---|---|---|---|
| 100 B | 472 (26) | 299 (25) | 290 (26) | 170 (26) | 138 (25) | 162 (27) | 105 (26) | 121 (26) |
| 1 KiB | 3,444 (26) | 2,394 (25) | 2,267 (26) | 1,384 (26) | 1,239 (25) | 1,135 (27) | 769 (26) | 1,019 (26) |
| 16 KiB | 7,324 (26) | 14,824 (25) | 6,923 (26) | 8,822 (26) | 9,464 (25) | 2,156 (27) | 2,072 (26) | 5,618 (26) |
| 64 KiB | 2,552 (26) | 10,612 (25) | 5,466 (26) | 6,048 (26) | 9,469 (25) | 2,206 (27) | 2,622 (26) | 6,679 (26) |
| 1 MiB | 3,647 (26) | 3,591 (25) | 5,253 (26) | 3,369 (26) | 3,464 (25) | 3,353 (27) | 2,693 (26) | 3,861 (26) |

### Size

| binding | files | raw | gzip | Rust glue, raw / gzip |
|---|---|---|---|---|
| C | 1 | 1,604 KiB | 235 KiB | 2,346 KiB / 125 KiB |
| C++ | 1 | 2,664 KiB | 299 KiB | none |
| Kotlin | 2 | 2,373 KiB | 318 KiB | 1,649 KiB / 72 KiB |
| Python | 1 | 1,424 KiB | 217 KiB | 3,678 KiB / 282 KiB |
| Swift | 3 | 2,231 KiB | 295 KiB | none |
| Go | 19 | 3,429 KiB | 404 KiB | none |
| Dart | 1 | 3,921 KiB | 360 KiB | 1,606 KiB / 59 KiB |

| native library | as built | stripped |
|---|---|---|
| `libmoq.a`, the C ABI that C, C++, Swift, and Go link | 191,403 KiB | 102,985 KiB |
| `libmoq.so`, with the Kotlin and Dart glue | 60,222 KiB | 45,126 KiB |
| `python/libmoq.so`, the CPython extension | 54,694 KiB | 41,023 KiB |

The stand-in exports all of moq-net, kio, and moq-pattern, and hang, moq-mux, and
moq-tokio's session side, far more than moq-ffi's UniFFI interface, so its sizes do not
compare with the fork's 653 / 914 / 684 KiB below. "Rust glue" is the generated crate's
source each binding builds with: the C ABI's `src/lib.rs` and runtime, which every binding
but Python's links, and `src/jni`, `src/python`, and `src/dart`. The native libraries are
the export's default `release` build, which links all of moq's crates; "stripped" is
`strip --strip-all` on a copy.

### What a call spends

From the CPU tables, read against the code each call runs. In order of what it costs moq's
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

## Consumer release profile

Measured on fedora on 2026-11-12, through `quest gate`, with the same pinned
moq stand-in and three rounds. The exported workspace now carries moq's release
profile: fat LTO, one codegen unit, and `strip = "symbols"`. Its package override
keeps debuginfo-only stripping for the moq-ffi dependency. Panic behavior is unchanged.
A paired build of the generated crate with Kotlin and Dart enabled took 404 s with
this profile and 107 s after removing the profile tables (297 s longer, 3.8 times).
Both used fresh target directories and the warm mbx compiler cache, so these are
elapsed build times on a shared host, not cold compiler timings. The corresponding
static libraries were 95.2 versus 186.7 MiB, and shared libraries 38.7 versus 59.7 MiB.
The Python extension in the table was built separately by the benchmark harness.

The older tables above remain the default-profile baseline; host load and intervening
generator changes prevent attributing every difference to LTO alone.

### Calls (ns per call, CPU time; load average)

| call | Rust | C | C++ | moq-c | Kotlin | Python | Swift | Go | Dart |
|---|---|---|---|---|---|---|---|---|---|
| `track.is_closed()`, a handle call | 7.9 (17) | 15 (17) | 15 (16) |  | 60 (15) | 58 (14) | 24 (13) | 45 (12) | 41 (11) |
| `demand.sequence()`, a u64 getter | 0.2 (17) | 7.6 (17) | 7.6 (16) |  | 40 (15) | 49 (14) | 7.9 (13) | 38 (12) | 25 (11) |
| `track.largest()`, a record return | 16 (17) | 24 (17) | 24 (16) |  | 150 (15) | 104 (14) | 33 (13) | 63 (12) | 49 (11) |
| `track.info()`, an object return | 1.0 (17) | 25 (17) | 25 (16) |  | 190 (15) | 100 (14) | 34 (13) | 184 (12) | 208 (11) |
| `group.write_frame(at, frame)`, 100 B | 141 (17) | 148 (17) | 148 (16) |  | 240 (15) | 201 (14) | 178 (13) | 244 (12) | 213 (11) |
| the same, with a new `Timestamp` each | 140 (17) | 162 (17) | 166 (16) | 199 (15) | 360 (15) | 372 (14) | 209 (13) | 430 (12) | 433 (11) |
| a 100 B frame written and read, ready | 216 (17) | 293 (17) | 308 (16) |  | 590 (15) | 608 (14) | 441 (13) | 642 (12) | 721 (11) |
| the same, read after a wake | 332 (17) | 428 (17) | 447 (16) |  | 1,460 (15) | 7,414 (14) | 10,023 (13) | 3,676 (12) | 1,628 (11) |
| `frame.payload`, 100 B | 14 (17) | 22 (17) | 25 (16) |  | 60 (15) | 176 (14) | 69 (13) | 116 (12) | 60 (11) |

### Throughput (MB per CPU second, write plus ready read; load average)

| frame | Rust | C | C++ | Kotlin | Python | Swift | Go | Dart |
|---|---|---|---|---|---|---|---|---|
| 100 B | 481 (17) | 329 (17) | 308 (16) | 118 (15) | 120 (14) | 209 (13) | 131 (12) | 134 (11) |
| 1 KiB | 3,285 (17) | 2,421 (17) | 2,319 (16) | 1,677 (15) | 1,056 (14) | 1,491 (13) | 899 (12) | 1,107 (11) |
| 16 KiB | 17,210 (17) | 15,604 (17) | 17,081 (16) | 6,554 (15) | 9,508 (14) | 5,074 (13) | 4,205 (12) | 8,353 (11) |
| 64 KiB | 6,562 (17) | 16,278 (17) | 14,699 (16) | 3,277 (15) | 14,664 (14) | 6,035 (13) | 8,951 (12) | 11,329 (11) |
| 1 MiB | 13,758 (17) | 14,106 (17) | 12,541 (16) | 5,243 (15) | 13,183 (14) | 13,543 (13) | 7,577 (12) | 17,641 (11) |

### Calls (ns per call, wall time; load average)

| call | Rust | C | C++ | moq-c | Kotlin | Python | Swift | Go | Dart |
|---|---|---|---|---|---|---|---|---|---|
| `track.is_closed()`, a handle call | 8.0 (17) | 15 (17) | 15 (16) |  | 31 (15) | 58 (14) | 24 (13) | 45 (12) | 38 (11) |
| `demand.sequence()`, a u64 getter | 0.2 (17) | 7.6 (17) | 7.6 (16) |  | 23 (15) | 49 (14) | 7.9 (13) | 38 (12) | 25 (11) |
| `track.largest()`, a record return | 16 (17) | 24 (17) | 24 (16) |  | 74 (15) | 104 (14) | 33 (13) | 61 (12) | 49 (11) |
| `track.info()`, an object return | 1.0 (17) | 25 (17) | 25 (16) |  | 80 (15) | 100 (14) | 34 (13) | 170 (12) | 208 (11) |
| `group.write_frame(at, frame)`, 100 B | 141 (17) | 149 (17) | 149 (16) |  | 233 (15) | 201 (14) | 179 (13) | 242 (12) | 213 (11) |
| the same, with a new `Timestamp` each | 141 (17) | 163 (17) | 167 (16) | 200 (15) | 321 (15) | 373 (14) | 209 (13) | 407 (12) | 431 (11) |
| a 100 B frame written and read, ready | 217 (17) | 294 (17) | 310 (16) |  | 499 (15) | 609 (14) | 442 (13) | 622 (12) | 722 (11) |
| the same, read after a wake | 334 (17) | 430 (17) | 451 (16) |  | 1,442 (15) | 7,436 (14) | 3,218 (13) | 2,980 (12) | 1,629 (11) |
| `frame.payload`, 100 B | 14 (17) | 22 (17) | 25 (16) |  | 61 (15) | 177 (14) | 69 (13) | 98 (12) | 60 (11) |

### Throughput (MB/s, write plus ready read; load average)

| frame | Rust | C | C++ | Kotlin | Python | Swift | Go | Dart |
|---|---|---|---|---|---|---|---|---|
| 100 B | 479 (17) | 328 (17) | 306 (16) | 176 (15) | 120 (14) | 208 (13) | 137 (12) | 134 (11) |
| 1 KiB | 3,276 (17) | 2,412 (17) | 2,306 (16) | 1,560 (15) | 1,053 (14) | 1,486 (13) | 1,068 (12) | 1,122 (11) |
| 16 KiB | 17,154 (17) | 15,558 (17) | 17,048 (16) | 11,063 (15) | 9,451 (14) | 5,045 (13) | 4,596 (12) | 8,346 (11) |
| 64 KiB | 6,544 (17) | 16,232 (17) | 14,568 (16) | 8,295 (15) | 14,643 (14) | 6,015 (13) | 12,256 (12) | 11,776 (11) |
| 1 MiB | 13,719 (17) | 14,061 (17) | 12,458 (16) | 7,168 (15) | 13,156 (14) | 13,517 (13) | 8,328 (12) | 17,742 (11) |

### Size

| binding | files | raw | gzip | Rust glue, raw / gzip |
|---|---|---|---|---|
| C | 1 | 1,633 KiB | 242 KiB | 2,408 KiB / 133 KiB |
| C++ | 1 | 2,865 KiB | 320 KiB | none |
| Kotlin | 2 | 2,472 KiB | 358 KiB | 1,697 KiB / 77 KiB |
| Python | 1 | 1,454 KiB | 225 KiB | 3,756 KiB / 293 KiB |
| Swift | 3 | 2,141 KiB | 300 KiB | none |
| Go | 23 | 3,538 KiB | 432 KiB | none |
| Dart | 1 | 4,019 KiB | 375 KiB | 1,643 KiB / 62 KiB |

| native library | as built | stripped |
|---|---|---|
| `libmoq.a`, the C ABI that C, C++, Swift, and Go link | 98,510 KiB | 58,787 KiB |
| `libmoq.so`, with the Kotlin and Dart glue | 39,847 KiB | 39,847 KiB |
| `python/libmoq.so`, the CPython extension | 35,277 KiB | 35,277 KiB |

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
