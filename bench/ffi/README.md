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
beside the median load, then the export's build cost and sizes. Each round also times,
from an emptied target and without a compiler cache, the export (`rust2-ffi`, which
runs `cargo check` of moq under its driver) beside moq's own `cargo check` of the same
crates, and the generated crate's release build beside moq's own release build, each with
its peak RSS, through [`measure.py`](measure.py). Nightly runs it on arm64 Linux into
the run's summary; nothing fails on a number.

| case | what it times | BoltFFI's nearest |
|---|---|---|
| handle call | `track.is_closed()`: one borrowed handle, a bool back, and moq's own read lock | `noop()` |
| u64 getter | `demand.sequence()`: one borrowed handle and a field read, so all but 0.4 ns is the binding's | `group.sequence()` |
| shared call, own call | the handle call on 4 threads at once, on one handle they share, then on a clone each | none |
| shared getter, own getter | the u64 getter the same way | none |
| record return | `track.largest()`: an `Option` of two `u64`s, copied across | `track.info()`, a record |
| object return | `track.info()`: a new handle, freed | none |
| write | `group.write_frame(at, frame)` with one `Timestamp` and 100 B, copied into Rust | `write_frame` |
| stamped write | the same with a new `Timestamp` per frame, as a caller holding a `u64` must | none |
| ready read | a 100 B frame written, then read through a new future ready at its first poll | `await noop_async()`, plus a write |
| woken read | a read started before its frame is written, as the language starts work concurrently, so it suspends and the write's wake resumes it | `await yield_once()`, plus a write |
| reader loop | one reader and writer live for all groups, including their handoff; Go waits for a real pending poll before writing | none |
| Swift drained reader | a separate executor diagnostic: one reader drained before each write, without a writer task or permit handoff | none |
| payload | `frame.payload` of 100 B, handed to the language | none |
| throughput | groups of 100 frames written and read, each payload's first byte touched, about 64 MiB | throughput |

The four threaded cases start their threads, wait until all are ready, then time each
making a million calls (Python 100,000), as the single-thread rows do. Wall time is per call
on one thread and CPU time per call over all of them, so where the threads never meet,
each equals its single-thread row. A clone is a new handle over the same Rust value, so
both cases share moq's own state and differ only in the binding's: the gap between them is
what a binding's run-time checks cost under contention. The handle call also takes moq's
read lock, which all four threads share in both cases; the getter takes none. C, C++,
Kotlin, Swift, and Python use OS threads, and Go goroutines. Dart has no number: an
isolate cannot receive another's object, so no two threads ever borrow one handle.

A woken read starts its read the language's own way: by polling in C, C++, and Rust; a
`Task` and a yield in Swift and Python; a goroutine and a yield in Go; an undispatched
coroutine in Kotlin; and an eager call in Dart. So it measures a task's start and its
wake, and in Swift, Python, and Go, a read the scheduler had not yet started is ready
instead of woken.

The `loop` case removes task creation per frame: one reader loops over all groups and
one writer supplies them. Kotlin, Python, Dart, and Swift include a permit handoff per
frame and an acknowledgement before advancing groups, bounding queued work. Swift runs
on its default global pool. Its writer can race the read's first poll, so this is a
streaming workload, not an isolated measurement of every read suspending.

Go uses the same pending-context notification as its future correctness tests: `Done`
notifies the writer only after a poll has pended with its real waker registered. It
returns nil because this benchmark never cancels. The writer cannot race that poll;
the final count asserts exactly one pending notification per frame. Go's measured
handoff is thus after registration, rather than before the call as in the other loops.

Swift also reports `drained`, a separate diagnostic on a preferred executor. This thread
drains one reader to suspension before each write, then drains it until that frame is
consumed, checking the count on both sides. It has no writer task or permit handoff and
must not be ranked against the managed `loop` row. C, C++, and Rust's existing `woken`
cases already manually poll in one loop; they remain the native floor, without printing
the same measurement twice under another name.

## Concurrent calls

Measured 2026-11-13 on **nixos**, an AMD Ryzen 9 5900XT (16 cores in two 8-core dies, 32
threads), at rust2 `a7db2666` plus these cases and moq `ffcb7cd9`, through `quest gate`.
Each cell is the median of 5 rounds, at a load average of 15 to 17 throughout. Rust
1.98.1, GCC 16.2.0, CPython 3.14.10 (with its GIL), OpenJDK 21.0.13 with Kotlin 2.4.21,
Swift 6.4, Go 1.27.3. Nanoseconds per call, CPU first, then wall:

| call | Rust | C | C++ | Kotlin | Python | Swift | Go |
|---|---|---|---|---|---|---|---|
| u64 getter, one thread | 0.4 / 0.4 | 3.7 / 3.7 | 3.6 / 3.6 | 20 / 9.6 | 75 / 75 | 3.7 / 3.7 | 33 / 33 |
| 4 threads, one shared handle | 0.5 / 0.7 | 107 / 129 | 127 / 143 | 378 / 417 | 1,617 / 2,496 | 114 / 131 | 252 / 255 |
| 4 threads, a handle each | 0.6 / 0.7 | 4.1 / 4.6 | 4.2 / 4.2 | 10 / 12 | 1,525 / 2,395 | 5.4 / 6.2 | 34 / 36 |
| handle call, one thread | 3.4 / 3.4 | 7.0 / 7.1 | 7.1 / 7.2 | 20 / 12 | 78 / 78 | 11 / 11 | 35 / 35 |
| 4 threads, one shared handle | 79 / 88 | 242 / 251 | 229 / 237 | 558 / 530 | 1,653 / 2,595 | 405 / 416 | 312 / 319 |
| 4 threads, a handle each | 80 / 89 | 138 / 146 | 149 / 159 | 198 / 201 | 1,722 / 2,755 | 162 / 167 | 201 / 206 |

What they show:

1. **A shared handle's borrow count is one contended cache line.** Every shared borrow at
   the C ABI is a compare-and-swap in and a `fetch_sub` out of the handle's `state`, so 4
   threads on one handle bounce its line: C's getter costs 107 ns against 4.1 on a handle
   each, near its 3.7 on one thread. A C model of that borrow alone, run on its own after
   the benchmark at the same load, cost 73 to 157 ns a borrow on one counter and 4 on a
   counter each, so the gap is that line and nothing else of the binding's.
2. **A hold count on the same object adds another.** Kotlin's (378) and Go's (252) shared
   getters, and Swift's shared handle call on `TrackProducer`, still a class (405 against
   C's 242), pay their own hold count's atomics on the shared object beside the C ABI's.
   Swift's getter on a noncopyable `GroupDemand`, which has none, costs what C's does.
3. **moq's read lock is the rest of the handle call.** Rust pays 79 ns on a shared value
   and 80 on clones of it: the clones share moq's state, so its lock contends in both, and
   no binding's handle call can drop below that.
4. **Python's threads serialize on the GIL.** Its glue releases the GIL around each call
   and takes it back after, so with 4 threads each call hands the GIL to another: 1.5 us of
   CPU a call either way, 20 times its single thread, and the handle's own contention is
   lost in it.

The load limits these numbers: 4 threads among 15 to 17 others' leave the scheduler free to
place them on one die or across both, whose line transfer is slower, which the model's
twofold spread shows. Read the shared rows as their order of magnitude, against a handle
each, at the same load.

## Long-lived reader measurement

Measured 2026-11-13 on **nixos**, an AMD Ryzen 9 5900XT (16 cores, 32 threads), after a
loaded fedora run proved inconclusive. Baseline: rust2 `03782045`, moq `ffcb7cd9`, Swift
6.4.0, Go 1.27.3, Kotlin 2.4.21 and coroutines 1.10.2, Dart 3.13.0. The binaries and their
Nix runtime closures were copied from fedora; builds and measurements on nixos ran
through `quest gate` over SSH. No CPU affinity or scheduler settings were changed.

Five rounds, each program warming up once before printing, in microseconds per 100 B
frame (write plus read); CPU includes every process thread:

| language / executor | task per read, wall / CPU | reader loop, wall / CPU |
|---|---|---|
| Kotlin, runBlocking | 1.88 / 1.94 | 1.80 / 1.83 |
| Dart, isolate | 2.41 / 2.40 | 2.78 / 2.77 |
| Swift, global pool | 5.14 / 18.88 | 4.30 / 9.50 |
| Go | 5.54 / 7.70 | 3.41 / 4.26 |

Go's row uses five fresh rounds. Its task-per-read column uses a goroutine and
`runtime.Gosched()` before each write; only its reader-loop column uses the pending
notification in place of the racing permit. Its `loop` verifies all one million reads actually pended with a real
waker. The other managed rows use the original streaming protocol and include its
handoff; differences in scheduling mean these numbers are workload costs, not a precise
ranking of the bindings' wake primitives.

Swift's separate `drained` diagnostic measured **1.22 / 1.21 us**, while its
contemporaneous default-global loop measured 4.40 / 9.63 us. A separate instrumented
run recorded 1,010,000 initial pending polls and 1,010,000 polls pending with a real
waker: all one million reads plus 10,000 group receives. Instrumentation supplied no
timing results. The global-loop counted run had only about 40% of calls pend with a real
waker; instrumentation can affect that ratio. The drained loop's structural suspension
check needs no instrumentation. This diagnostic establishes that the binding honors
the preferred executor; it does not establish that Swift is faster than other languages.

`perf record -F 499 -g` on the original full Swift program attributed 55.5% of sampled
cycles to the dispatch-worker subtree, including 43.4% under its semaphore-wait path.
Kernel addresses were restricted, so kernel function names are unresolved. A separate
language-only control using the same permit stream, an NSLock-protected continuation
mailbox, and no Rust measured 3.78 / 8.87 us (median of five after warmup). This supports
the global executor and permit handoff as the dominant cost. The control's lock differs
from the binding's atomics, so it cannot give a precise subtraction of binding costs.

Go's profile showed scheduling, cgo transitions, and `selectgo`. An experiment replaced
its nil-`Done` select with a direct receive. On the original racing loop, five
alternating pairs suggested a 6% improvement (3.10 to 2.91 us wall). Repeating five
alternating pairs after forcing every read to pend instead measured **3.41 / 4.26 us
before and 3.84 / 4.93 us after**. The benefit did not survive the stricter workload,
so the experiment was dropped. The generated Go wait remains unchanged.

The investigation found no supported runtime change that removes Swift's native pool
cost while preserving the caller's scheduling. The default Swift loop remains above
Kotlin's and Dart's. Close the optimization investigation on that explicit limitation,
rather than promise equal absolute costs across different language schedulers. Future
measurements retain both the default workload and the isolated executor diagnostic;
no threshold is an acceptance gate and no runtime speedup is claimed.

The async-start design in [kixel-ai/rust2#741](https://github.com/kixel-ai/rust2/pull/741)
was still unmerged during these runs. It removes ready-call crossings; a pending call
still polls again to register its real waker. The harness preserves that design and
uses the stock public binding APIs.

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

Re-measured 2026-11-14 on **nixos** (AMD Ryzen 9 5900XT) at rust2 `79c46e70`, which adds
[#742](https://github.com/kixel-ai/rust2/pull/742)'s by-value `Timestamp`, against
`7008d1d3`, in turn four times at a load of 5 to 8: a 100 B write cost Swift 124 to 127 ns
of CPU against 149 to 176, and Dart 236 to 243 against 239 to 346 (241 to 247 in the three
pairs below load 7). So neither write costs more than it did before #744. A profile of
Swift's shows nothing of `Native.take` or its `Owner` outside the inlined call, and Dart's
100 B write adds two compares and two null checks a call, below what these runs resolve.

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

### Build (median, from an emptied target)

Measured 2026-11-13 at rust2 `29e70775` plus this harness on the same host, 3 rounds, load
17 to 28, with no compiler cache. CPU time sums every process cargo ran; peak RSS is the
largest one's.

| step | wall s | CPU s | peak RSS | load average |
|---|---|---|---|---|
| rust2-ffi's export, a `cargo check` of moq under its driver | 36.6 | 231.1 | 873 MiB | 20 |
| moq's own `cargo check` of the same crates | 29.9 | 218.3 | 878 MiB | 24 |
| the generated crate's release build, Kotlin and Dart glue included | 417.2 | 843.7 | 3,168 MiB | 25 |
| moq's own release build of the same crates | 94.1 | 542.0 | 1,550 MiB | 27 |

The export adds about 13 CPU seconds over the check cargo runs anyway, and no peak memory.
The generated crate's build adds about 300 CPU seconds and 320 wall seconds over moq's own,
and doubles the peak, as one long rustc over the glue would: that build, not the export, is
what a consumer pays on every export.

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

From the CPU tables of 2026-11-10, measured at rust2 `7383790d` at a load average of 22 to
29 and since replaced by the tables above, read against the code each call runs. In order of what it costs moq's
frame path:

1. **A call per frame timestamp.** `Timestamp` is `Copy` with private fields, so it
   crosses as its 16 bytes, and a stamped frame allocates and frees nothing in Rust. As
   a handle, it cost 21 ns in C, 27 in Swift, 140 in Python, 250 in Kotlin, 260 in Go,
   and 245 in Dart (stamped write minus write, in the tables above); as a value, see
   [Copy values](#copy-values): what remains is each language's call and, in Kotlin and
   Python, an object of its own.
2. **Ownership of a returned object.** An object return costs C 25 ns, but Go 263 and
   Dart 242, ten times C: each new Go object registers `runtime.AddCleanup` and each Dart
   one a `NativeFinalizer`, beside a handle allocation. Kotlin's 137 ns wall (210 CPU) is a phantom
   reference linked and unlinked under one global lock. Every returned frame, group, and future pays it.
3. **A woken read.** C's 427 ns is Rust's 329 plus a future's crossings. Kotlin's 1.6 us and
   Dart's 1.8 us add their event loops; Go's 5.1 us and Swift's 5.7 us wall (16.6 us CPU)
   are a goroutine or `Task` and a cross-thread wake per read, and Python's 6.9 us is
   asyncio's, as BoltFFI's Python measured 10 us. This case starts a task per read; a
   reader looping on one task, which skips the start, is measured in
   [Long-lived reader measurement](#long-lived-reader-measurement).
4. **A future per read.** Since `<fn>_start` makes a future and polls it in one call, a
   ready read costs C 126 ns over the write where Rust costs 93: one crossing and one
   allocation, down from 176 with four allocations and three crossings. Measured 2026-11-13
   on an AMD Ryzen 9 9950X (32 threads) at a load average of 26, 12 rounds alternating with
   the commit before, so the tables above predate it. Over their writes, Swift pays 289,
   Go 414, Dart 522, Python 445, and Kotlin 590 (from 381, 556, 868, 659, and 615). Go's and
   Dart's include the returned frame's ownership, about 190 and 215 ns over C's object return;
   the rest is each language's own await.
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
handle costs a call. C, C++: 10 ns, the runtime's two atomics. Swift: 9.9, with no hold
count since the change below. Kotlin: 24 wall, Go 38, Dart 28, Python 49, each with its call's own
transition besides.

Swift has since made each object a noncopyable struct and dropped its hold count, but for
a class: one thrown as an error. A `Copy` type is a value since [Copy values](#copy-values). Measured 2026-11-11 at that change, 5
rounds interleaved with C at a load average of 7 to 12, its getter costs 10 ns, as C's
does, and `track.is_closed()`, on `TrackProducer`, still a class since moq throws it, 26
against C's 18, as in the table. A change to the accounting moves this row and the handle call's by the
same amount; measure both, CPU first, at the load beside them.

### Copy values

A `Copy` type with private fields, such as `Timestamp`, crosses by value: a struct of its
bytes in C, C++, Swift, Go, and Dart, a class over a `LongArray` in Kotlin (a word value
class when it fits one), and a class of the runtime's `Value` holding its bytes in Python.
Measured 2026-11-13 at rust2 `7766768f`, moq `ffcb7cd9`, on an AMD Ryzen 9 9950X (32 threads),
Linux, at a load average of 13 to 15, the median of 3 rounds, in CPU time:

| | C | C++ | Swift | Go | Dart | Kotlin | Python |
|---|---|---|---|---|---|---|---|
| write | 163 | 165 | 173 | 260 | 251 | 280 | 224 |
| stamped write | 161 | 168 | 174 | 293 | 296 | 430 | 384 |
| stamped minus write | 0 | 3 | 1 | 33 | 45 | 150 (35 wall) | 160 |

C, C++, and Swift pay nothing for the timestamp; Go and Dart pay about their getter's call,
the `from_micros` itself. Kotlin's 35 ns wall is its call, and its CPU column adds the
collector's work on each `LongArray`. Python's is a call and a new object. The handle's cost
in the tables above came from another load, so Python's was also measured against main's
build on the same host, interleaved over 5 rounds at a load of 10 to 12: stamped minus write
fell from 205 ns to 166, and the stamped write from 441 to 391.

### Against the targets

Where the cases match, on wall time: Kotlin's getter costs 24 ns against BoltFFI's 5.8,
its record return 74 against 61, its write 114 ns over Rust against BoltFFI's 56, and 100 B
throughput 166 MB/s against 105. Python's getter costs 49 ns against 63, its record return
99 against 1,120, and 100 B throughput 138 MB/s against 12.1. So Python meets BoltFFI's
and Kotlin's calls trail it by 1.2 to 4 times, behind JNI's per-call cost and the hold count.

## Consumer release profile

Measured on fedora on 2026-11-12, through `quest gate`, with the same pinned
moq stand-in and three rounds. The exported workspace now carries moq's release
profile: fat LTO, one codegen unit, and `strip = "symbols"`. Its package override,
`strip = "debuginfo"` for moq-ffi, was then carried only onto the facade's rlib, so
these artifacts got `strip = "symbols"`; the export now also gives it to the glue
package that builds them. Panic behavior is unchanged.
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
