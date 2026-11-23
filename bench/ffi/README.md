# Binding benchmark

What [rust2-ffi](../../ffi/README.md)'s bindings cost over Rust, per language, on the moq
stand-in `just ffi bench` exports, against BoltFFI's numbers below as targets.

```bash
nix develop .#bench --command just ffi bench  # build, run 3 rounds, and write ffi/target/bench/report.md
nix develop .#bench --command just ffi bench-run 5  # rerun what bench-build built, 5 rounds
nix develop .#android --command just ffi bench-android  # the Kotlin rows on HotSpot and on the Android device adb reaches
```

`ffi/moq/bench/export.*` time the same cases in each language, `export.rs` the floor
on the stand-in's API from Rust, and `moq-c.c` moq's hand-written rs/libmoq, which
has only the stamped write. `export.ts` runs on Node and, through `browser.ts`, in
headless Chromium and Firefox, over a wasm32-wasip1 build of the stand-in
([TypeScript](#typescript)). Each program runs its cases twice and prints the second,
so a JIT has warmed up, in wall time and in the process's CPU time on every thread.
`just ffi bench-run` runs each program once a round, records the 1-minute load average
over each run, and [`report.py`](report.py) prints each case's median over the rounds
beside the median load, then the export's build cost and sizes. Each round also times,
from an emptied target and without a compiler cache, the export (`rust2-ffi`, which
runs a release `cargo check` of moq under its driver) beside moq's own of the same
crates, and the generated crate's release build beside moq's own release build, each with
its peak RSS, through [`measure.py`](measure.py). Nightly runs it on arm64 Linux into
the run's summary; nothing fails on a number. [CodSpeed](#codspeed) counts each
single-threaded case on every push to main and on each PR that can move one.

| case | what it times | BoltFFI's nearest |
|---|---|---|
| handle call | `track.is_closed()`: one borrowed handle, a bool back, and moq's own read lock | `noop()` |
| u64 getter | `demand.start()`: one borrowed handle and a field read, so all but 0.4 ns is the binding's | `group.sequence()` |
| shared call, own call | the handle call on 4 threads at once, on one handle they share, then on a clone each | none |
| shared getter, own getter | the u64 getter the same way | none |
| record return | `track.largest()`: an `Option` of two `u64`s, copied across | `track.info()`, a record |
| object return | `track.info()`: a new handle, freed | none |
| field | `info.priority`: an `i8` `pub` field read, which no user code reaches, so a leaf in C, Kotlin, Python, and Dart alone | none |
| hop call, hop field | Dart's handle call and field read while Rust holds a closure of the isolate | none |
| write | `group.write_frame(at, frame)` with one `Timestamp` a group and 100 B, copied into Rust | `write_frame` |
| stamped write | the same with a new `Timestamp` per frame, as a caller holding a `u64` must | none |
| ready read | a 100 B frame written, then read through a new future ready at its first poll | `await noop_async()`, plus a write |
| woken read | a read started before its frame is written, as the language starts work concurrently, so it suspends and the write's wake resumes it | `await yield_once()`, plus a write |
| reader loop | one reader and writer live for all groups, including their handoff; Go waits for a real pending poll before writing | none |
| Swift drained reader | a separate executor diagnostic: one reader drained before each write, without a writer task or permit handoff | none |
| payload | `frame.payload` of 100 B, handed to the language | none |
| payload by size | the same from 16 B to 1 MiB, TypeScript only, where it finds where lending beats a copy | none |
| throughput | groups of 100 frames written and read, each payload's first byte touched, about 64 MiB | throughput |

The four threaded cases start their threads, wait until all are ready, then time each
making a million calls (Python 100,000), as the single-thread rows do. Wall time is per call
on one thread and CPU time per call over all of them, so where the threads never meet,
each equals its single-thread row. A clone is a new handle over the same Rust value, so
both cases share moq's own state and differ only in the binding's: the gap between them is
what a binding's run-time checks cost under contention. The handle call also takes moq's
read lock, which all four threads share in both cases; the getter takes none. C, C++,
Kotlin, Swift, Python, and C# use OS threads, and Go goroutines. Dart has no number: an
isolate cannot receive another's object, so no two threads ever borrow one handle.
TypeScript has none either: the module runs on the one JavaScript thread that loaded it.

A woken read starts its read the language's own way: by polling in C, C++, and Rust; a
`Task` and a yield in Swift and Python; a goroutine and a yield in Go; an undispatched
coroutine in Kotlin; an eager call in Dart; a `ValueTask` made before the write in C#,
whose wake polls it again on the thread pool; and in TypeScript, a promise made before the
write, which polls once and pends. So it measures a task's start and its
wake, and in Swift, Python, and Go, a read the scheduler had not yet started is ready
instead of woken.

The `loop` case removes task creation per frame: one reader loops over all groups and
one writer supplies them. Kotlin, Python, Dart, Swift, C#, and TypeScript include a permit handoff per
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

Each case that writes groups writes them to a track of its own, created before its clock
starts and freed after it stops. A track caches every group written to it until it is
freed: a broadcast's pool evicts by its expiry only when something polls it, and nothing
here does. Before, every case wrote to one track, so a round held all of its frames, about
1.4 GB in C and Rust, and each later case ran on a heap that grew as it ran; now a round
peaks near 290 MB there. Go and Swift peak at 1.1 and 1.2 GB, as their reads run on many
threads, each with a glibc malloc arena that keeps what it freed: with
`MALLOC_ARENA_MAX=1` they peak near 330 and 400 MB.

## CodSpeed

[CodSpeed](https://app.codspeed.io/kixel-ai/rust2) keeps main's history of each
single-threaded case, and comments on a PR whose run changes a case, with its change per
case.
[`codspeed.yml`](../../.github/workflows/codspeed.yml) runs on every push to main, its
baseline, and on each PR that changes what `just ffi bench` reads: it builds the programs in
the bench shell, then runs `just ffi codspeed` under CodSpeed's CPU simulation in that
shell's environment, so Nix itself never runs under Valgrind. It is informational: no
threshold fails it and no ruleset requires it.

```bash
nix develop .#bench --command just ffi bench-build
nix develop .#bench --command codspeed run --mode simulation -- just ffi codspeed  # CodSpeed's CLI, on Ubuntu or Debian
```

Each program calls CodSpeed's
[instrument-hooks](https://github.com/CodSpeedHQ/instrument-hooks), which the flake pins,
through [`codspeed.h`](../../ffi/moq/bench/codspeed.h) and its own FFI around each case:
one benchmark per language and case, such as `python::write` or `c::throughput[1024]`.
Under CodSpeed a program runs its round once, without the warm-up round, and with a tenth
of the groups (so 100,000 calls a case, Python's 10,000), since the simulation counts
every call exactly and costs 50 to 100 times the program's own time. It skips the
`shared:*`, `own:*`, and Kotlin's `objects` cases: the simulation runs one thread at a
time, so it would count a contention that never happens. It counts instructions and
simulated cache misses, not time, so a system call or a wait costs nothing in it. The
tables below keep wall and CPU time, and a case's absolute count is for its whole loop,
not a call: read CodSpeed for each case's change.

A language joins only if its counts hold still: the same build, run twice, must move no
case far enough for CodSpeed to report a change, which its default 10% regression
threshold decides. `codspeed_langs` in [`ffi/justfile`](../../ffi/justfile) lists those
that pass. Measured 2026-11-14 on **nixos** with stock Valgrind 3.27.1 and the flags
CodSpeed's runner passes, without its fork's cycle estimation, in pairs of runs of one
build, the largest move of any case between two runs:

| language | largest move | |
|---|---|---|
| Rust, C, C++ | none: every count identical | joins |
| moq-c | 0.36% (`stamped`, with all the calls), 0.1% with a tenth | joins: moq's runtime thread runs beside it |
| Python | 0.21% (`throughput[1024]`) | joins: asyncio's poll of its selector depends on time |
| Swift | 52% (`throughput[16384]`), 8.8% in another pair | leaves: the global executor's threads interleave by time |
| Go | 362% (`stamped`), 330% (`record`), 84% (`loop`) | leaves: its collector and scheduler run inside each case, and its preemption signal crashes Callgrind unless `GODEBUG=asyncpreemptoff=1` |
| Kotlin | 172% (`call`), 108% (`getter`) | leaves: without the warm-up, the JIT's and collector's threads compile and collect inside each case, and the JVM's C2 code crashes under Valgrind unless its implicit null checks are off |
| Dart | does not run | leaves: the VM maps its code at an address hint off a page boundary, which Valgrind refuses |
| C# | not measured | leaves until a pair of runs holds: CoreCLR's tiered JIT and collector run on their own threads |
| TypeScript (Node) | 2.6% (`ready`), every other case under 2.3% | joins, with V8's flags below |

Node joined on 2026-11-16, measured on **fedora** the same way, Node 24.22.2. With V8's
default flags, two runs moved `write` 113%, `stamped` 69%, and `ready` 49%: V8 compiles,
sweeps, and collects on its own threads, by time. `--predictable --single-threaded
--v8-pool-size=0` keeps all of that on the main thread, which left 36%
(`payload[1048576]`); `--predictable-gc-schedule`, so a collection follows allocation
rather than time, left 16% (`throughput[1048576]`); and a full `gc()` before each case,
through `--expose-gc`, left the 2.6% above. `just ffi codspeed` passes all four flags
under `RUST2_CODSPEED`, and `bench-run`, which times, passes none. The Chromium and Firefox
columns stay out: CodSpeed's simulation runs one process, not a browser. Building the
wasm32 module adds about 5 minutes to the workflow's build, and Node's cases about 6 to its
simulation.

On CI, two runs of one commit (kixel-ai/rust2#805's 11a94ff, a `pull_request` run and a
`workflow_dispatch` one) left all 58 benchmarks untouched in CodSpeed's comparison: the
largest move was Python's `throughput[1024]`, 2.4%, then its `throughput[100]` and
`ready`, under 2%, and every one of Rust's, C's, C++'s, and moq-c's rounded to 0%.

Each leaver keeps its hooks, so it rejoins by `codspeed_langs` once a pair of runs holds.

## Concurrent calls

Measured 2026-11-15 on **nixos**, an AMD Ryzen 9 5900XT (16 cores in two 8-core dies, 32
threads), through `quest gate`, against moq `ffcb7cd9`. Before is rust2 `f84ec06c`, where
every shared borrow counts on the handle's one `state`. After is the reader-biased
borrow below. Each program alternated before and after for 5 rounds, at a load average
of 1.4 to 2.9, and each cell is the median. Rust 1.98.1, GCC 16.2.0, OpenJDK 21.0.13 with
Kotlin 2.4.21, Swift 6.4, Go 1.27.3. Nanoseconds per call, CPU first, then wall. Kotlin's
CPU clock ticks in 10 ns, so its CPU cells under 100 ns are coarse.

Before:

| call | C | C++ | Kotlin | Swift | Go |
|---|---|---|---|---|---|
| u64 getter, one thread | 3.4 / 3.4 | 3.3 / 3.3 | 20 / 13 | 3.8 / 3.8 | 30 / 30 |
| 4 threads, one shared handle | 99 / 117 | 123 / 137 | 392 / 403 | 123 / 135 | 232 / 233 |
| 4 threads, a handle each | 3.8 / 4.1 | 4.2 / 4.5 | 10 / 10 | 7.4 / 8.3 | 34 / 36 |
| handle call, one thread | 6.4 / 6.5 | 6.5 / 6.5 | 20 / 12 | 10.5 / 10.5 | 31 / 31 |
| 4 threads, one shared handle | 185 / 190 | 204 / 211 | 415 / 390 | 328 / 335 | 291 / 293 |
| 4 threads, a handle each | 136 / 141 | 135 / 140 | 168 / 175 | 148 / 153 | 171 / 180 |

After:

| call | C | C++ | Kotlin | Swift | Go |
|---|---|---|---|---|---|
| u64 getter, one thread | 3.4 / 3.4 | 3.4 / 3.4 | 20 / 13 | 4.6 / 4.6 | 32 / 32 |
| 4 threads, one shared handle | 6.9 / 7.7 | 8.0 / 10.5 | 20 / 26 | 11.7 / 12.5 | 39 / 40 |
| 4 threads, a handle each | 4.2 / 4.6 | 4.2 / 4.6 | 10 / 11 | 9.0 / 9.8 | 34 / 35 |
| handle call, one thread | 6.4 / 6.5 | 6.4 / 6.5 | 20 / 12 | 9.9 / 10.0 | 32 / 32 |
| 4 threads, one shared handle | 133 / 145 | 122 / 126 | 128 / 106 | 73 / 83 | 182 / 192 |
| 4 threads, a handle each | 132 / 141 | 119 / 127 | 192 / 177 | 163 / 168 | 177 / 185 |

What they show:

1. **A shared handle's borrow no longer bounces a cache line.** Before, every shared
   borrow at the C ABI was a compare-and-swap in and a `fetch_sub` out of the handle's
   `state`, so 4 threads on one handle cost C's getter 99 ns against 3.8 on a handle each.
   Now, once two borrows meet on the count, the handle is biased, and each later borrow
   publishes it in its thread's slot of a table instead (`ffi/runtime/rt/borrow.rs`, after
   BRAVO, Dice and Kogan, USENIX ATC 2019), so threads sharing it write nothing in common.
   C's shared getter is 6.9 ns, within 2 of a handle each.
2. **The language hold counts take the same table.** Go's, Kotlin's, and a Swift class's
   hold count on the object was a second contended line, which now goes to a table of the
   binding's own: Kotlin's shared getter fell from 392 to 20 ns, Go's from 232 to 39, and
   Swift's shared handle call on `TrackProducer`, a class, from 328 to 73.
3. **moq's read lock is the rest of the handle call.** With the handle's lines gone, a
   shared handle call costs what a handle each does, about 120 to 190 ns: the clones share
   moq's state, so its lock contends in both.
4. **The single-thread rows hold, except Swift's getter.** C's, C++'s, and Kotlin's are
   unchanged. Go's hold costs 0.1 ns more in a Go microbenchmark, within this row's run to
   run spread of 1 to 2 ns. Swift's getter on a noncopyable `GroupDemand` calls the same
   Rust export as C's, whose row is flat, yet costs 0.8 ns more from Swift.
5. **Swift's getter moved with its code's placement, not the borrow.** Swift calls the
   export through a merged thunk, by an indirect call, at the same bytes before and after.
   Remeasured at rust2 `c12c4c0e` (3 rounds a cell, load 1.3 to 3.5, CPU ns): shifting all
   code in 16-byte steps moves the getter between 4.0 and 4.5 with a 64-byte period, and
   shifting the Rust export alone, by up to 128 bytes, moves it by no more than 0.1. The
   earlier count-only borrow, rebuilt on the same tree, costs 4.15. A Swift wrapper marked
   `@inlinable`, so the loop calls the export directly, costs 3.5 at every shift, as
   every generated wrapper now is: [Swift's inlined calls](#swifts-inlined-calls).

The load limits these numbers: 4 threads beside others' leave the scheduler free to place
them on one die or across both, whose line transfer is slower. Read the shared rows as
their order of magnitude, against a handle each, at the same load. Rust's and Python's
columns are not remeasured: Rust borrows natively, and Python's threads serialize on the
GIL, which dwarfs the handle's own contention.

## Leaf calls

A leaf is a call the export proves runs no code but std's and the glue's, waits on no
lock, and calls no binding: a method or `pub` field of a `Send` object, taking and returning
scalars, whose body and each `Deref`'s on the way its MIR proves so, or a `PartialEq` or
`Hash` derived down to primitives ([`Abi::leaf`](../../ffi/src/abi.rs)).
Python keeps the GIL across one, Dart makes it a leaf call (`isLeaf`) to the C function
itself on the isolate's thread, with no serving thunk, no thread hop, and no `_Arena`, and
Kotlin marks its native method `@FastNative` ([Kotlin on ART](#kotlin-on-art)).
The `field` case reads `track::Info::priority`, a field; the u64 getter, `demand.start()`,
a method returning one, is a leaf since the export reads its body ([below](#leaf-bodies)).

Measured 2026-11-19 on **fedora**, an AMD Ryzen 9 9950X (16 cores, 32 threads), through
`quest gate`, against moq `ffcb7cd9`. Before is rust2 `52070b3d` with this change's
programs; after is this change. Each program alternated before and after for 5 rounds at a
load average of 4 to 26, and each cell is the median wall time in ns per call.

| case | before | after |
|---|---|---|
| C `field`, which no binding wraps | 8.1 | 8.0 |
| Python `field` | 63.3 | 26.2 |
| Dart `field` | 30.8 | 12.6 |
| Dart `hop:field` | 2,296 | 12.3 |
| Dart `hop:call`, no leaf | 2,718 | 2,549 |

Dart's three levers, apart, as variants of the generated `priorityGet` built over each side's
library, in the same rounds (wall ns):

| variant | `field` | `hop:field` |
|---|---|---|
| through the serving thunk, in an arena holding the handle (before) | 30.8 | 2,296 |
| through the thunk, with no arena or hold | 21.7 | 2,480 |
| the C function directly, not as a leaf, with no arena | 21.4 | 21.5 |
| a leaf call, in an arena holding the handle | 19.6 | 21.4 |
| a leaf call, with no arena (after) | 12.6 | 12.3 |

So the arena costs about 9 ns, `isLeaf` saves another 9, and the thunk costs nothing until
Rust holds a closure, when it hands each call to a worker thread for 2 to 3 us. A call that
is no leaf but holds only its receiver needs no arena either, only the hold: it now holds
the receiver itself. Measured 2026-11-19 the same way, 5 rounds at a load of 16 to 24,
against the leaf change alone: Dart's handle call fell from 46.1 to 37.9 ns, the u64 getter
from 31.1 to 23.4, the record return from 56.5 to 50.0, and `frame.payload` from 65.6 to
60.7, with `field` at 13.0; a call the arena still serves, such as the write, moved only
within the spread of its rounds.

Python's 37 ns is more than the 10 to 15 the per-language audit estimated for the GIL's
round trip, as `rt::call` does more around it: it may look for the event loop running, and
after the call drains what other threads freed and resumes what the call woke, all of which
a leaf skips. A call that is no leaf, such as `track.is_closed()`, which takes moq's lock,
still pays it.

### Leaf bodies

A method's body, and a `Deref`'s on the way to a member, is a leaf when its MIR
([`leaf.rs`](../../ffi/src/leaf.rs)) calls only bodies proven the same way and core's
intrinsics that only compute, drops nothing whose drop runs other code, has no loop, no
recursion, no assertion, and no static. So `demand.start()` and `group::Producer`'s
`start`, through its `Deref`, are leaves, as are 91 more of the stand-in's methods and
accessors: Dart makes 348 of its calls leaf calls, against 255.

Measured 2026-11-20 on **fedora**, through `quest gate`, against moq `1aa86d5e`: before is
rust2 `8798a5d7`, after is this change, each program alternating before and after for 5
rounds at a load average of 7 to 15; each cell is the median wall time in ns per call.

| case | before | after |
|---|---|---|
| Python `getter` | 64.5 | 26.7 |
| Python `shared:getter` | 1,790 | 119 |
| Python `own:getter` | 1,779 | 113 |
| Dart `getter` | 26.5 | 15.8 |

The four threads no longer hand the GIL to each other on every call, which was most of
their 1.8 us. Every other case of both languages moved within 4% either way, Python's
`field` included, whose code is the same.

## Kotlin on ART

`just ffi bench-android` runs `export.kt` on HotSpot on the host and, shrunk by R8 with the
generated rules as an app is, on ART through `dalvikvm` on the Android device adb reaches,
over a library cargo-ndk builds for it, round after round, into one report. Nightly runs it
in the Android job, on its x86_64 API 35 emulator with 4 cores, into the run's summary; the
other languages' rows run on arm64, so read Kotlin's on HotSpot beside ART's from this
report alone. ART has no `java.lang.management`, so its CPU time is the kernel's tick count
in `/proc/self/stat`, at HotSpot's own 10 ms resolution on Linux.

Measured 2026-11-20 on **fedora**, an AMD Ryzen 9 9950X, through `quest gate`, against moq
`1aa86d5e`, with this change, 3 rounds at a load of 6 to 8, on an Android 15 (API 35)
x86_64 emulator with 4 cores under KVM. OpenJDK 21.0.13 with Kotlin 2.4.21, R8 8.6 from
build-tools 35.0.0. Wall ns per call, the median; the CPU cells are coarse at this load's
tick:

| call | HotSpot | ART |
|---|---|---|
| handle call | 32 | 41 |
| u64 getter | 23 | 32 |
| field, a leaf | 23 | 25 |
| record return | 56 | 129 |
| object return | 97 | 230 |
| write, 100 B | 224 | 382 |
| ready read | 630 | 1,190 |
| woken read | 1,761 | 3,751 |
| `frame.payload`, 100 B | 64 | 143 |
| throughput, 16 KiB frames | 9,268 MB/s | 652 MB/s |

1. **A plain JNI call costs ART about 9 ns more than HotSpot.** The handle call and the
   getter run the same Kotlin and the same Rust on both.
2. **A leaf is `@FastNative`, which keeps ART's thread runnable across the call instead of
   switching its state there and back.** Each leaf's native method takes it, as
   `_Native.track_Info_priority_get` does, and `_Native`'s first call registers each through
   `RegisterNatives` from the library's one exported `__contract`, since ART binds such a
   method by its name only from Android 12. The annotation blocks the collector for the
   call, so only a leaf takes it: a call that may wait on a lock, as `is_closed()` waits on
   moq's, or call back into Kotlin, could stall every thread's collection behind it.
3. **Throughput falls off from 16 KiB on ART alone**, near 650 MB/s against HotSpot's 6.7 to
   9.3 GB/s, while 100 B and 1 KiB frames stay within 3x. These rounds kept every case's
   frames on one track; a track per case ([below](#a-track-per-case)) raised both runtimes
   but HotSpot more, so the gap stands. [ART's throughput](#arts-throughput)
   finds why.

The spike, 5 rounds of each side alternating on one emulator at a load of 12 to 29, wall ns,
before (main) and after (`@FastNative` on leaves, registered):

| case | API 35 before | API 35 after | API 30 before | API 30 after |
|---|---|---|---|---|
| field, a leaf | 34.1 | 25.8 | 43.5 | 28.1 |
| handle call, no leaf | 40.3 | 43.6 | 80.3 | 80.8 |
| u64 getter, no leaf | 32.6 | 33.8 | 43.2 | 43.4 |

Marking the handle call and the getter too, by hand, on API 35 (3 rounds), cost 36.6 against
41.1 and 25.7 against 34.2: the same few ns, which a call the proof cannot bound may not
take. moq's export has 200 leaves, the api fixture's 17. Registration covers only those:
registering all of moq's 6,455 native methods would scan the class's methods once each, and
R8 strips the natives nothing calls, which `RegisterNatives` would then refuse, so the
generated rules keep each `@FastNative` method instead. `kotlin/FastNative.kt` declares
the annotation, which Android's SDK makes public only from API 34, for the JVM and older
SDKs. Compiled as a program class, R8 keeps it on the methods only by the rule naming it;
against android.jar's own class, R8 keeps it with no rule, so a build against SDK 34 or
later leaves the file out, as one of two exports in an app must, since D8 refuses a class
defined twice. Only leaves whose `eq` and `hash` walk no value of a caller's size are
marked, as Dart's leaf calls are.

`just ffi android` passes, with these rules, on API 24 (Android 7, which ignores the
annotation), 28, 30, and 35.

### A track per case

Before this change, `export.kt` never finished on API 28: within 15 s its resident memory
passed 1.7 GB under a 256 MB heap, and the emulator, with 2 GB and no swap, stopped
answering adb for the 10 minutes it was given. The binding leaked nothing: the cases that
make and free a million handles grew it by 3 MB in all, while each case writing a million
100 B frames grew it by 270 to 300 MB, the frames its one track cached. HotSpot held the
same, 2.1 GB at its peak, and API 35's emulator swapped it to zram. With each writing case
on a track of its own ([above](#binding-benchmark)), the API 28 run peaks near 380 MB and
finishes, and nightly runs it there with 2 GB, where a bench that outgrows the device
hangs and the step's 20 minutes fail it.

Measured 2026-11-20 on **desktop**, an Intel Core Ultra 7 270K Plus (24 cores), through
`quest gate`, against moq `1aa86d5e`, on an Android 15 (API 35) x86_64 emulator with 4
cores and 2.5 GB under KVM. Before is rust2 `cd40ee4d`, after is this change, 3 rounds
each at a load of 26 to 28 before and 20 to 21 after; throughput in wall MB/s, the
median:

| frame | HotSpot before | HotSpot after | ART before | ART after |
|---|---|---|---|---|
| 100 B | 159 | 161 | 52 | 74 |
| 1 KiB | 1,298 | 1,304 | 398 | 393 |
| 16 KiB | 2,553 | 7,705 | 906 | 1,368 |
| 64 KiB | 1,839 | 5,610 | 941 | 1,168 |
| 1 MiB | 2,805 | 4,017 | 744 | 935 |

The call rows moved within their rounds' spread; on ART, the 100 B write fell from 496
to 404 ns and the ready read from 1,324 to 1,162.

### ART's throughput

Profiled 2026-11-20 on **desktop**, an Intel Core Ultra 7 270K Plus, against moq `1aa86d5e`,
on an Android 15 (API 35) x86_64 emulator with 4 cores and 2 GB under KVM, beside another
agent's emulator at a load of 18 to 29, with `simpleperf` from NDK 30 as root. Each program
ran the throughput case alone in a fresh process, 6 passes of about 64 MiB, and each cell is
the median over 5 rounds of the median of its last 5 passes, every program alternating
within a round (MB/s, the slowest and fastest round in brackets):

| program | 16 KiB | 64 KiB | 1 MiB |
|---|---|---|---|
| C, `export.c`'s case through the same library | 1,973 (1,098 to 2,027) | 2,292 (2,205 to 2,304) | 2,131 (2,024 to 2,587) |
| Kotlin, each frame read but not its payload | 1,407 (1,351 to 1,431) | 1,467 (1,443 to 1,882) | 1,235 (1,170 to 1,344) |
| Kotlin, before | 878 (860 to 894) | 1,171 (1,071 to 1,230) | 1,006 (956 to 1,039) |
| Kotlin, after | 1,259 (1,249 to 1,328) | 1,403 (1,381 to 1,783) | 1,212 (1,149 to 1,225) |

1. **Fresh memory is most of the cost, in C too.** A track's default cache never evicts, so
   each frame's copy into moq, `Bytes::copy_from_slice` in `write_frame`, lands in pages
   the process never touched: 54% of C's samples and 62% of Kotlin's, after, are the page
   fault, 21 and 44% the kernel zeroing the page. A C loop copying 16 KiB on the same
   emulator moves 138 GB/s into a buffer it reuses and 61 GB/s into a fresh `malloc` freed
   after, so its bandwidth is no limit, and HotSpot's host faults far more cheaply.
2. **Before, reads past the lent budget copied into ART's large-object space.** At most
   64 MiB are lent to Kotlin at once, and a program making little garbage of its own fills
   that before ART collects the `Bytes` holding them, so later reads cross copied until it
   does. ART puts each array of 12 KiB or more in its own mapping: in 5 other rounds at
   16 KiB, a library always copying moved 509 MB/s, one always lending 1,374, and reading
   no payload 1,437, so about a third of before's reads copied.
3. **After, Rust asks ART to collect once 32 MiB are lent**, on a thread of its own, so the
   collection frees them before the budget runs out. It asks again only once frees bring
   them under 16 MiB, so bytes a program keeps reachable never make it collect over and
   over. HotSpot is never asked: it copies into its young generation cheaply, and
   collects everything at once when asked.

These rounds kept every case's frames on one track, as `export.kt` did before [a track per
case](#a-track-per-case). Measured again on 2026-11-21 over it, on the same host and moq,
on an API 35 emulator with 4 cores and 2.5 GB at a load of 17 to 29: one library, its
request to collect turned off for the before side by an environment variable, 5 rounds with
the sides alternating in fresh processes, MB/s, the median (the slowest and fastest round):

| program | 16 KiB | 64 KiB | 1 MiB |
|---|---|---|---|
| the throughput case alone, before | 861 (605 to 901) | 1,131 (1,040 to 1,151) | 1,039 (927 to 1,069) |
| the throughput case alone, after | 1,338 (1,059 to 1,356) | 1,465 (1,321 to 1,468) | 1,211 (1,178 to 1,228) |
| `export.kt`, before | 1,392 (1,196 to 1,402) | 1,086 (949 to 1,341) | 858 (718 to 936) |
| `export.kt`, after | 1,336 (1,289 to 1,378) | 1,375 (1,354 to 1,711) | 1,108 (1,067 to 1,284) |

A track per case left the budget's cost as it was: a program streaming past 64 MiB still
loses a third at 16 KiB without the request. `export.kt` moves each size's 64 MiB once a
round, so its 16 KiB pass starts with the budget empty and copies little either way, which
is why its 16 KiB row rose with a track per case alone; the larger sizes start with the
16 KiB pass's bytes still lent. Every call row moved within its rounds' spread.

## Swift's inlined calls

Measured 2026-11-16 on **nixos**, as above, through `quest gate`, against moq `ffcb7cd9`.
Before is rust2 `8ae7404b`, whose wrappers a consumer calls into the Swift module; after,
every generated body is `@inlinable`, so the bench program's loop calls the C export
itself. Each cell is the median of 3 rounds, before and after alternating at each
placement, at a load average of 2.3 to 6.2: a `.text` pad of 0, 16, 32, or 48 bytes,
linked ahead of the Swift objects, shifts all code through the 64-byte period the getter's
cost followed. Nanoseconds per call, CPU first, then wall; C's getter costs 3.4 and its
handle call 6.4.

| call | pad 0 | pad 16 | pad 32 | pad 48 |
|---|---|---|---|---|
| u64 getter, before | 4.84 / 4.87 | 4.46 / 4.48 | 4.38 / 4.40 | 4.71 / 4.73 |
| u64 getter, after | 3.39 / 3.40 | 3.39 / 3.42 | 3.38 / 3.39 | 3.33 / 3.34 |
| handle call, before | 10.08 / 10.12 | 10.11 / 10.16 | 9.95 / 10.02 | 9.97 / 10.02 |
| handle call, after | 9.87 / 9.92 | 9.85 / 9.91 | 9.90 / 9.95 | 9.79 / 9.84 |
| getter, 4 threads, a handle each, before | 9.84 / 10.49 | 8.75 / 9.99 | 8.21 / 9.11 | 9.32 / 10.09 |
| getter, 4 threads, a handle each, after | 7.24 / 7.88 | 6.99 / 7.71 | 6.71 / 7.19 | 6.92 / 7.93 |
| object, before | 26.52 / 26.61 | 26.55 / 26.68 | 25.68 / 25.77 | 25.91 / 25.99 |
| object, after | 27.62 / 27.74 | 30.22 / 30.34 | 27.72 / 27.84 | 26.68 / 26.78 |
| ready read, before | 458 / 460 | 461 / 463 | 452 / 454 | 449 / 451 |
| ready read, after | 443 / 445 | 445 / 447 | 436 / 438 | 440 / 442 |

1. **The getter costs C's at every placement.** Its loop calls `moq_group_Demand_sequence`
   directly, with no merged thunk between, so where the linker puts code no longer moves it.
2. **The handle call inlines too, but keeps its hold.** `TrackProducer` is a class, whose
   call holds its handle with a compare-and-swap and lets go with an add of its own, about
   3.4 ns over C's. Its body first inlined only once a failed check's error, a handle's
   free, and the release of a hold in the table moved out of line, and the handle's
   constants became computed properties, which another module folds instead of reading
   through an accessor.
3. **A call the optimizer declines to inline still goes through the thunk.** The program's
   `track.info()`, whose object it then drops, calls the module's copy before and after, so
   its cost still follows placement: 26.7 to 30.2 against 25.7 to 26.6. Most other rows
   fell a little: the record by up to 2 ns at three placements, a write by 1.5 to 6, a
   stamped write by 2.6 to 5.5, and the ready read, an `async` call, by 9 to 16.
   Throughput is bimodal on either side, about 2,200 or 8,600 MB/s for 16 KiB frames in a
   run, as the global executor's threads interleave.
4. **The cost is in the Swift module's build.** moq's module, 90,000 lines of Swift, takes
   81 to 89 s to build with `-O` against 66, its `.swiftmodule` 19.1 MB against 3.6, and its
   library 11.6 to 11.8 MB against 11.5; the bench program's own build is unchanged, 1.2 to
   2.1 s.

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
and doubles the peak: that build, not the export, is what a consumer pays on every export.
Nearly all of the gap is the glue crate's one rustc, and most of that is the fat LTO in one
codegen unit that moq-ffi's release profile asks for. moq's own build makes only rlibs, so
it never runs that step, while the generated crate's links optimize everything the export
reaches in one module. moq's UniFFI moq-ffi, built with that profile, took 877 CPU s and
2,259 MiB peak on 2026-11-13, beside 720 CPU s and 3,275 MiB for the generated crate a
revision later, on the same host and load (1 and 3 rounds).

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
| TypeScript | 684 | 4,498 KiB | 1,000 KiB | 4,535 KiB / 259 KiB |

| native library | as built | stripped |
|---|---|---|
| `libmoq.a`, the C ABI that C, C++, Swift, and Go link | 193,455 KiB | 104,224 KiB |
| `libmoq.so`, with the Kotlin and Dart glue | 61,361 KiB | 46,029 KiB |
| `python/libmoq.so`, the CPython extension | 56,145 KiB | 42,166 KiB |
| `moq.wasm`, the wasm32 stand-in's module, with the web glue | 19,596 KiB | 17,632 KiB |

| web binding | raw | gzip | brotli |
|---|---|---|---|
| `moq.wasm`, stripped | 17,632 KiB | 4,657 KiB | 2,893 KiB |
| the TypeScript, 684 modules | 4,498 KiB | 1,000 KiB | 855 KiB |
| the npm package's JavaScript and declarations | 5,670 KiB | 1,454 KiB | 1,226 KiB |
| `export.ts` bundled by esbuild, minified | 23 KiB | 7 KiB | 7 KiB |

The stand-in exports all of moq-net, kio, and moq-pattern, and hang, moq-mux, and
moq-tokio's session side, far more than moq-ffi's UniFFI interface, so its sizes do not
compare with the fork's 653 / 914 / 684 KiB below. "Rust glue" is the generated crate's
source each binding builds with: the C ABI's `src/lib.rs` and runtime, which every binding
but Python's links, and `src/jni`, `src/python`, and `src/dart`. The native libraries are
the export's default `release` build, which links all of moq's crates; "stripped" is
`strip --strip-all` on a copy.

The web rows (measured 2026-11-19 on fedora at the same moq pin, with this table's other
rows from 2026-11-13) are the wasm32 stand-in's export, which leaves out moq-tokio, built
with moq's release profile: fat LTO at `opt-level = 3`. A module "stripped" has no custom
section, its names among them; each file is compressed alone, gzip at level 6 and brotli at
11, as a host serves it. The release builder makes the packaged module at `opt-level =
"s"` when the crate sets no level: 13,359 KiB as built, 10,981 KiB stripped, 3,250 KiB
gzipped, and 2,135 KiB brotli'd. A caller pays for what it imports: `export.ts`, the
TypeScript benchmark, names 13 of the 684 modules, and esbuild's bundle of it and the
binding is 23 KiB, against 4.5 MiB of TypeScript. JSR caps a version at 20 MiB, which the
TypeScript and the `opt-level = 3` module pass, and the `"s"` module fits.

### What a call spends

From the CPU tables of 2026-11-10, measured at rust2 `7383790d` at a load average of 22 to
29 and since replaced by the tables above, read against the code each call runs. In order of what it costs moq's
frame path:

1. **A call per frame timestamp.** `Timestamp` is `Copy` with private fields, so it
   crosses as its 16 bytes, and a stamped frame allocates and frees nothing in Rust. As a
   handle, at moq `1aa86d5e6` before its `Option<Timescale>` field crossed by value, it
   cost 21 ns in C, 27 in Swift, 140 in Python, 250 in Kotlin, 260 in Go, and 245 in Dart
   (stamped write minus write, in the tables above); as a value, see
   [Copy values](#copy-values): what remains is each language's call and, in Kotlin and
   Python, an object of its own.
2. **Ownership of a returned object.** An object return costs C 25 ns, but Go 263 and
   Dart 242, ten times C, and most of the difference is each runtime's per-object
   registration with its collector. Profiled 2026-11-14 on **nixos** with `perf`, for an
   object returned and closed: Go spends 39% of it in `runtime.AddCleanup` and the
   `Cleanup.Stop` its `Close` makes, a third in its three cgo crossings (the call, the
   value's drop, the handle's free), and a tenth in the collector's own threads. Dart's
   `Dart_NewFinalizableHandle`, its crossing, and the collector finalizing it cost 1,600
   instructions an object, three quarters of it: without them, a return and close took
   62 to 65 ns against 176 to 245. Since kixel-ai/rust2#828, a Go object and its handle
   are one allocation (70 fewer instructions) and a Dart close drops the value without
   an `_Arena` (136 fewer). Since kixel-ai/rust2#838, an object closed within 64 returns
   of its own skips the registration: each Go P and each Dart isolate holds its last 64
   returned objects in a nursery, registering only one pushed out unclosed, and a checked
   object's close drops and frees in one crossing. Measured 2026-11-15 on **nixos** at a
   load average of 4 to 24, 8 rounds alternating with main at `72880f48`, median wall ns:
   Go's object return fell from 254 to 174 and Dart's from 225 to 96, Go's ready read from
   833 to 728 and Dart's from 1,020 to 786, and Dart's woken read from 2,930 to 2,274.
   What Go has left is mostly its two cgo crossings, the call and the close, and the pool
   that finds its P's nursery. Since kixel-ai/rust2#880, each Kotlin thread holds one too, and
   a close no longer links and unlinks a phantom reference under a lock: Kotlin's object return
   fell from 124 to 118 ns median wall (from 112 to 94 at best) and from 56 to 38 on 4
   threads at once. Since kixel-ai/rust2#906, a checked close no call holds drops and frees in
   one JNI crossing, as Go's and Dart's do: measured 2026-11-20 on **fedora** at a load
   average of 11 to 30, 8 rounds alternating with main at `1fd44bdd`, Kotlin's object return
   fell from 93 to 77 ns wall at best, and from 132 to 83 median, as half of main's rounds
   ran at 169 to 199 and none of this one's past 92; and from 260 to 170 median CPU
   ([TypeScript](#the-nursery) has the web's). Since kixel-ai/rust2#942, the drop stores the
   handle closed as Rust commits it, so a close meeting it waits for the outcome without
   waiting on a `Drop`: Kotlin pays a JNI field write for it, and Go stores nothing after.
   Measured 2026-11-21 on **desktop** at a load average of 1 to 5, alternating with main at
   `ca5cf0b5`, median wall: Kotlin's object return rose from 91 to 94 ns (12 rounds), and
   Go's fell from 152 to 149 (8 rounds).
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

   Profiled 2026-11-15 on **nixos** in user-space instructions a read adds to a write, a
   million reads alone in each language: C 1,360, Swift 2,248, Go 4,790, and Dart 5,776.
   Swift's await spent most of its share in `Task.isCancelled`'s lookup of the current
   task, the async frame's allocation, and `swift_task_switch`, the hop a `nonisolated`
   async function makes onto the global executor; since kixel-ai/rust2#839 a future is
   `nonisolated(nonsending)` and makes no hop (2,104), which on a caller's actor saves two
   jobs a read (9.9 us a ready read on the main actor, then 0.32). Dart's start made a call
   object and a completer before it knew it was ready; it now returns `Future.syncValue`
   (5,298), leaving Dart's own await, which schedules a microtask even for a complete
   future. Go's is the returned frame's ownership (`AddCleanup`, `Cleanup.Stop`, and a
   close's two crossings), and cgo's check and heap allocation of each out pointer, which
   [kixel-ai/rust2#837](https://github.com/kixel-ai/rust2/pull/837) took: what remains after both
   is one crossing. Off the main thread, glibc's malloc arena for the thread grows and
   trims with `mprotect`, which adds about 60 ns to a write and 20 to a read in C on a
   pthread (`MALLOC_ARENA_MAX=1` removes it). Swift's ready read runs in a task on such a
   thread and its write above on the main one, so its ready row carries it and C's does not.
5. **Kotlin's record codec.** A record return costs Kotlin 113 ns wall against C's 31,
   through the byte-array codec; Swift, which reads the C struct, costs 37.
6. **The handle borrow at the C ABI.** The u64 getter costs Rust 0.4 ns and C 10: the
   C ABI's borrow of each handle, a compare-and-swap in and a `fetch_sub` out of its
   state, is the whole call. Every handle argument pays it again.
7. **The binding's own call.** Over C's getter: Swift adds 8 ns (its hold count's two
   atomics, since dropped from all but its classes, as below), Kotlin 14 wall (JNI and its hold count), Dart 19 (an `_Arena` per call and
   its hold), Go 31 (cgo, `defer`, and its hold), and Python 37 (argument parsing and the
   GIL released and taken back, so Rust may call Python from another thread); Python's and
   Dart's hold counts are since dropped for a checked type, as below. BoltFFI's
   Python getter, which holds the GIL, measured 63 ns. A leaf keeps the GIL in Python and
   skips Dart's arena, and a Dart call holding only its receiver skips it too:
   [Leaf calls](#leaf-calls).

### Per-handle baseline

For [#730](https://github.com/kixel-ai/rust2/pull/730)'s one accounting: the u64 getter
is one borrowed handle and nothing else, so its row less Rust's 0.4 ns is what a handle
costs a call. Measured 2026-11-13 on desktop at a load average of 2.4 to 4.1, main at
`7b0958b3` against #730 before Go and Kotlin took their counts back, run alternately: 12
rounds of main and 6 of #730, median wall ns (CPU ns where it differs). #730 drops the hold
counts Python and Dart kept beside the runtime's for a checked type, so its call borrows
through the runtime's two atomics alone, and close drops the value through Rust.

| row | C | Python | Swift | Dart |
|---|---|---|---|---|
| handle call, main | 18 | 54 | 26 | 41 (45) |
| handle call, #730 | 18 | 53 | 26 | 42 (45) |
| u64 getter, main | 10 | 48 | 10 | 27 |
| u64 getter, #730 | 10 | 47 | 10 | 28 |
| object return, main | 29 | 99 | 37 | 238 |
| object return, #730 | 30 | 98 | 38 | 214 |
| stamped write, main | 159 | 337 | 205 | 433 |
| stamped write, #730 | 168 | 332 | 212 | 450 |

Go and Kotlin dropped theirs too at first, and gained 5 to 8 ns on the getter, but a
handle closed or moved out of was then freed only once the GC found its object
unreachable, which kept Go's cleanup and Kotlin's phantom reference registered: Go's
object return went from 183 to 691 ns and its stamped write from 429 to 1,061, Kotlin's
stamped write from 310 to 613. So each still counts its calls, freeing the handle once a
close or a move leaves no call holding it. Measured 2026-11-13 on nixos (AMD, 32 threads)
at a load average of 28 to 47, main at `e9cca797` against #730, 10 runs of each program
alternating, median wall ns:

| row | Kotlin | Go |
|---|---|---|
| handle call, main | 13 | 35 |
| handle call, #730 | 14 | 49 |
| u64 getter, main | 12 | 34 |
| u64 getter, #730 | 11 | 50 |
| object return, main | 156 | 261 |
| object return, #730 | 66 | 317 |
| stamped write, main | 457 | 745 |
| stamped write, #730 | 441 | 835 |

Kotlin's rows match main's within the noise: its object return's fastest runs took 57 and
60 ns. Go's checked call allocated its status then, since cgo moves to the heap a local
whose address it passes C; it now calls through a C function of its preamble that keeps
the status in a C local and returns it. Measured 2026-11-14 on fedora at a load average of
12 to 26, main at `11949356` against that change, 8 runs of each Go program alternating,
median wall ns (CPU ns where it differs): handle call 43 (45) to 35, u64 getter 44 (45) to
33, object return 325 (357) to 276 (312), and moq's packages went from 4,594 locals moved to
the heap to 80. Go's close calls Rust's `_drop`, a second crossing an object return and a
stamped write pay. Dart's checked calls first lost 4 ns to a status allocated per call,
now one cell per isolate, which the glue fills from the call's own once the call stopped
serving the isolate, for 1 ns more. Swift's getter costs C's since it dropped its hold count for
each noncopyable object (2026-11-11), and a `Copy` type is a value since [Copy values](#copy-values). A change to the accounting moves these rows;
measure them at the load beside them.

### Copy values

A `Copy` type with private fields whose every value fills its bytes crosses by value, as
`Timestamp` does, its `None` scale written as its niche's whole `u64`: a struct of its bytes
in C, C++, Swift, Go, and Dart, a class of a `Long` per word in Kotlin (a word value class
when it fits one), and a class of the runtime's `Value` holding its bytes in Python.
Measured 2026-11-13 at rust2 `7766768f`, moq `ffcb7cd9`, on an AMD Ryzen 9 9950X (32 threads),
Linux, at a load average of 13 to 15, the median of 3 rounds, in CPU time:

| | C | C++ | Swift | Go | Dart | Kotlin | Python |
|---|---|---|---|---|---|---|---|
| write | 163 | 165 | 173 | 260 | 251 | 280 | 224 |
| stamped write | 161 | 168 | 174 | 293 | 296 | 430 | 384 |
| stamped minus write | 0 | 3 | 1 | 33 | 45 | 150 (35 wall) | 160 |

C, C++, and Swift pay nothing for the timestamp; Go and Dart pay about their getter's call,
the `from_micros` itself. Kotlin's 35 ns wall was its call, and its CPU column adds the
collector's work on each `LongArray`. Since its value is a class of a `Long` per word, and a
`Result` of one returns its words through the thread's slots rather than encoded, the
stamped write costs Kotlin 30 ns of wall time over the write, against 270 on main's build:
measured 2026-11-14 at rust2 `2ec35316`, moq `ffcb7cd9`, on the same host, interleaved over 7
rounds at a load average of 6 to 17, the medians of each round's difference. Its CPU time,
which counts the JIT's and collector's threads, ranged from 30 to 300 ns at that load.
The thread allocates 32 B a stamped frame over the write, the `Timestamp` itself, which
`write_frame` keeps from the JIT's escape analysis, against 112 on main's. Python's is a call and a new object. The handle's cost
in the tables above came from another load, so Python's was also measured against main's
build on the same host, interleaved over 5 rounds at a load of 10 to 12: stamped minus write
fell from 205 ns to 166, and the stamped write from 441 to 391.

At moq `1aa86d5e6`, `Timestamp` gained an `Option<Timescale>` and crossed as a handle until
the exporter counted a `None` as the niche it writes. Measured 2026-11-19 on **fedora**, the
same CPU, at a load average of 17 to 24, the median of 3 rounds of `just ffi bench` each, in
CPU time, stamped minus write:

| | C | C++ | Swift | Go | Dart | Kotlin | Python | TS (Node) |
|---|---|---|---|---|---|---|---|---|
| handle, main at `2f63f110` | 40 | 30 | 56 | 149 | 111 | 210 | 218 | 283 |
| value | 20 | 10 | 7 | 58 | 36 | 220 | 138 | 138 |

Rust's own `from_micros` takes 8 to 9 ns of each, against none at `ffcb7cd9`. Kotlin's CPU
time, which counts its JIT and collector, moved within its noise.

### Against the targets

Where the cases match, on wall time: Kotlin's getter costs 24 ns against BoltFFI's 5.8,
its record return 74 against 61, its write 114 ns over Rust against BoltFFI's 56, and 100 B
throughput 166 MB/s against 105. Python's getter costs 49 ns against 63, its record return
99 against 1,120, and 100 B throughput 138 MB/s against 12.1. So Python meets BoltFFI's
and Kotlin's calls trail it by 1.2 to 4 times, behind JNI's per-call cost and the hold count.

## TypeScript

The web binding runs over its own export of the stand-in, `target/moq-ffi-web`: the same
crates less moq-tokio, which pulls in rustls-native-certs, with no wasm32-wasip1 platform
module.
Nothing builds for wasm32-unknown-unknown, where getrandom 0.4 has no backend without a
config flag. `export.ts` imports the generated modules as a caller would, and disposes each
handle and payload it gets. A browser gives no process CPU clock, so its columns are wall
time only.

Measured 2026-11-16 on **fedora**, an AMD Ryzen 9 9950X (16 cores, 32 threads), at rust2
`ecfb751c` plus this harness, moq `ffcb7cd9`, 3 rounds at a load average of 31 to 51. Node
24.22.2, Chromium 157.0.8103.38, Firefox 159.0.1, Rust 1.98.1 for the module (moq's release
profile), Dart 3.13.0. C and Dart are from the same run, for scale; wall ns per call.

| call | C | Dart | Node | Chromium | Firefox |
|---|---|---|---|---|---|
| `track.is_closed()`, a handle call | 19 | 70 | 64 | 52 | 148 |
| `demand.sequence()`, a u64 getter | 9.1 | 46 | 69 | 54 | 126 |
| `track.largest()`, a record return | 32 | 87 | 106 | 89 | 204 |
| `track.info()`, an object return | 33 | 272 | 355 | 272 | 1,104 |
| `group.write_frame(at, frame)`, 100 B | 193 | 349 | 415 | 639 | 827 |
| the same, with a new `Timestamp` each | 196 | 474 | 575 | 650 | 1,108 |
| a 100 B frame written and read, ready | 369 | 1,057 | 1,344 | 1,041 | 2,878 |
| the same, read after a wake | 518 | 1,966 | 2,487 | 2,936 | 4,491 |
| one reader and writer, with their handoff |  | 2,434 | 2,269 | 3,686 | 5,146 |
| `frame.payload`, 100 B | 29 | 81 | 385 | 299 | 1,157 |

| frame, MB/s | C | Dart | Node | Chromium | Firefox |
|---|---|---|---|---|---|
| 100 B | 271 | 95 | 60 | 58 | 21 |
| 1 KiB | 2,061 | 1,044 | 605 | 591 | 255 |
| 16 KiB | 12,814 | 6,423 | 6,541 | 6,469 | 2,513 |
| 64 KiB | 10,312 | 6,871 | 12,228 | 11,070 | 7,218 |
| 1 MiB | 8,912 | 8,164 | 16,127 | 16,986 | 12,071 |

An object return paid the handle's `FinalizationRegistry` entry, as Go's and Dart's paid
their collectors', until [the nursery](#the-nursery). Firefox runs every row at about 2 to
3 times Node's.

### The nursery

Since kixel-ai/rust2#880, the last 64 objects and lent bytes the module returned wait in a
nursery, where disposing one frees it without a `FinalizationRegistry` entry, and only one
pushed out undisposed registers, as Go's, Dart's, and Kotlin's objects do. Measured
2026-11-19 on **fedora**, 5 rounds alternating with main at `52070b3d`, at a load average of
16 to 36; median wall ns, main then this, beside Kotlin's 8 rounds at a load of 17 to 25.
Kotlin lends only from 1 KiB, so its 100 B payload is a copy either way:

| call | Kotlin | Node | Chromium | Firefox |
|---|---|---|---|---|
| `track.info()`, an object return | 124 / 118 | 357 / 181 | 255 / 177 | 908 / 239 |
| `frame.payload`, 100 B | 67 / 66 | 415 / 212 | 244 / 209 | 1,008 / 227 |
| a 100 B frame written and read, ready | 676 / 625 | 1,258 / 998 | 1,099 / 1,129 | 2,595 / 1,534 |
| the same, read after a wake | 1,867 / 1,803 | 2,246 / 1,932 | 2,493 / 2,714 | 4,162 / 3,056 |

100 B throughput rose from 57 to 78 MB/s on Node and from 27 to 56 on Firefox. Chromium's
other rows moved both ways by up to a quarter between rounds at this load, so read only its
object and payload rows. A lent payload now costs 205 to 255 ns at every length on all three
engines (see below).

An object dropped undisposed in a nursery no later return pushes it out of waits for the
nursery's flush: a sentinel in a registry of its own, which a collection finds and then
registers all the nursery holds, as a ring the GC drops does in Kotlin. In the api fixture's
tests, a forced full collection at a time, such an object was freed by the second, on Node
and the JVM alike, in every run, against the first on main; one pushed out by later returns
was freed by the first, as before.

### The export's build cost

The web export [refuses what its host lacks](../../ffi/README.md#typescript-web) by
walking MIR, so it builds every crate with `-Zalways-encode-mir`, takes std's from the
dev shell's MIR sysroot, and checks the glue crate a second time to walk it. Measured
2026-11-17 on the same host, the stand-in's export from an emptied target through
[`measure.py`](measure.py), at rust2 `3b2e79f0` (signatures alone) and `0dc269dc` (with
the refusal), 3 rounds each, alternating, at a load average of 24 to 31; medians:

| export | wall s | CPU s | peak RSS |
|---|---|---|---|
| signatures alone | 27.1 | 76.4 | 856 MiB |
| with the refusal | 45.7 | 87.8 | 1,773 MiB |

The refusal adds about 11 CPU seconds, and its walk of the stand-in's 299,455 reachable
instances takes 6.5 s of them. Wall time ranged 24 to 60 s and 39 to 69 s between rounds
at this load, so its gap is rough; the walk's check runs after moq's crates, with nothing
beside it. Peak memory doubles, steady across rounds.

A native export reads MIR only for its [leaf bodies](#leaf-bodies), which every crate
encodes for it with `-Zalways-encode-mir`, as `cargo check` encodes none. Measured
2026-11-20 on **fedora** the same way, the stand-in's native export from an emptied target
before (rust2 `8798a5d7`) and after this change, 10 rounds each, alternating, at a load
average of 9 to 19; medians:

| export | wall s | CPU s | peak RSS |
|---|---|---|---|
| before | 23.0 | 101.8 | 896 MiB |
| after | 25.1 | 110.7 | 937 MiB |

So leaves cost the export about 9 CPU seconds, 9%, and 5% of its peak memory, and the
crates' metadata grows from 135 to 228 MB. The flag alone, measured apart at rust2
`8798a5d7` in 7 rounds at a load of 12 to 25, added 8 CPU seconds and 44 MiB, so nearly
all of it is the flag's: the proof walks only the bodies of methods whose signature a leaf
could have. CPU time ranged 95 to 117 s and 104 to 134 s between rounds, so its gap is
rough. Recording each crate's leaves in the driver as it compiles the crate would cost
less, but needs a store of facts keyed by each crate's build beside cargo's own, which
9 seconds does not repay.

The export checks the release profile, which libraries ship, so its leaves are proven
against the bodies a release build runs ([leaf profiles](../../ffi/README.md#targets)).
Measured 2026-11-21 on **fedora** the same way, before (rust2 `e8dcd892`, a dev check) and
after, 3 rounds each, alternating, at a load average of 9 to 11; medians:

| export | wall s | CPU s | peak RSS |
|---|---|---|---|
| dev check | 28.6 | 107.3 | 936 MiB |
| release check | 28.3 | 105.9 | 934 MiB |

Its own MIR optimizations, inlining most, would have cost 17 CPU seconds and 160 MiB more
in one round apiece, for no leaf more, so the check optimizes MIR as a dev build does,
which keeps what each body does. The stand-in's API is the same under both profiles, and
two more of its methods are leaves, `hang::Extra`'s `len` and `is_empty`, whose
`indexmap` makes a `debug_assert_eq!` a release build compiles out.

### The export's cache

The export wraps only the workspace's own crates, so a compiler cache in cargo's
`RUSTC_WRAPPER` serves the rest ([how](../../ffi/README.md)). mbx 1.30 caches a compilation
only when it models every flag on its command line, and it models none of the export's
`-Zalways-encode-mir`, `-Zmir-opt-level=1`, and the web target's `-Zinline-mir=no`
("rustc flag is not modeled by the cache adapter"), so it serves the build scripts and
proc macros alone, which take no target rustflags. Measured 2026-11-22 on **desktop**, the
stand-in's native export by a dev build of rust2-ffi, as the merge queue's `cargo run`
runs it, from an emptied target through mbx, at a load average of about 19; the after
rounds follow one that stored the host units:

| export | rounds | wall s | CPU s | mbx |
|---|---|---|---|---|
| before (`4cdfa151`) | 4 | 126 to 137 | 201 to 221 | none looked up |
| after | 3 | 118 to 139 | 170 to 198 | 107 hits, 226 bypassed |
| after, flags modeled | 2 | 132 to 142 | 125 to 136 | 332 hits, 1 bypassed |
| after, flags modeled, release rust2-ffi | 2 | 19 to 32 | 23 to 27 | 332 hits, 1 bypassed |

The last two rounds of each stand in for an mbx that models the flags, with only the
modeled `-Coverflow-checks=on` added: every crate but the facade is then served. That
leaves the facade's own check, which a dev build of rust2-ffi takes about two minutes on
one thread to analyze and generate, and a release build under 20 seconds.

### Lending returned bytes

Bytes a call returns are an [`rt.Bytes`](../../ffi/README.md#typescript-web): up to 64 B a copy, and
past that, Rust's own buffer, read through the memory on each access and freed on dispose.
Measured before and after it on the same host on 2026-11-16, one build each, a round apart
at a load average of 13 to 30, `frame.payload` in wall ns, copy then lend:

| payload | Node | Chromium | Firefox |
|---|---|---|---|
| 16 B, copied in both | 160 / 220 | 170 / 141 | 154 / 208 |
| 64 B, copied in both | 152 / 202 | 145 / 150 | 145 / 197 |
| 100 B | 527 / 418 | 602 / 254 | 329 / 702 |
| 256 B | 489 / 400 | 1,104 / 242 | 352 / 759 |
| 1 KiB | 1,123 / 381 | 1,122 / 235 | 440 / 780 |
| 4 KiB | 1,433 / 395 | 1,402 / 233 | 724 / 717 |
| 16 KiB | 2,798 / 364 | 3,996 / 236 | 1,586 / 900 |
| 64 KiB | 9,370 / 359 | 15,332 / 232 | 4,432 / 690 |
| 1 MiB | 96,547 / 378 | 210,601 / 225 | 190,762 / 527 |

V8 keeps a typed array of up to 64 B in its own heap, and past it gives the copy a backing
store of its own, so its copy jumps from about 150 ns to 500 at 100 B, where a lend costs
250 to 420 at any length. That is the crossover, and V8 runs Chrome, Edge, Node, Deno, and
Electron. SpiderMonkey's copy stays cheap to 1 KiB and its lend costs it about 750 ns, so
its crossover is near 4 KiB; JavaScriptCore's, measured in Bun 1.4.4, is between 256 and
512 B (a copy of 256 B costs 112 ns against a lend's 123, of 512 B 192 against 126). The
threshold is one constant for every engine: 64 B costs Firefox about 350 to 400 ns a
payload from 100 B to 1 KiB, and any higher one would cost V8 more at each length below it.
The bytes up to it now pay about 50 ns on Node and Firefox for the `rt.Bytes` around their
copy, against a bare `Uint8Array` before. Since [the nursery](#the-nursery), a lend disposed
skips its `FinalizationRegistry` entry and costs 205 to 255 ns on all three engines (Node 207
at 100 B against 421, Firefox 246 against 913), so SpiderMonkey's crossover falls from near
4 KiB to between 64 and 100 B, and the one threshold suits V8 and SpiderMonkey alike;
JavaScriptCore's was not remeasured.

A lend needs a dispose: the collector cannot see wasm memory, so it never hurries to free
what it lends. In a probe of a million 100 B lends left to the collector on Node, all
million were still held at the loop's end, and each cost 628 ns against a disposed lend's
275. So `rt.Bytes` lends only while lent bytes stay under a quarter of the memory, and
while the memory has grown by no more than a third since none were lent, which bounds
a returned slice that keeps a larger buffer whole; it copies past either.

## C#

Measured 2026-11-22 on **desktop** (Intel Core Ultra 7 270K Plus, 24 threads) at rust2
`34d46770` with returned bytes lent past 512, moq `1aa86d5e`, .NET 10.0.14 on CoreCLR's
tiered JIT, beside C from the same export, in turn for 3 rounds at a load of 30 from other
agents' builds, so read the C# column against C's from the same runs, not against the
tables above. CPU time, ns per call, and MB per CPU second for throughput:

| case | C | C# |
|---|---|---|
| `track.is_closed()`, a handle call | 16 | 34 |
| `demand.start()`, a u64 getter | 8.4 | 22 |
| the getter on 4 threads, one shared handle | 11 | 24 |
| the getter on 4 threads, a clone each | 9.3 | 21 |
| `track.largest()`, a record return | 28 | 71 |
| `track.info()`, an object return | 35 | 115 |
| `info.priority`, a field | 8.8 | 24 |
| `group.write_frame(at, frame)`, 100 B | 164 | 175 |
| the same, with a new `Timestamp` each | 178 | 261 |
| a 100 B frame written and read, ready | 303 | 422 |
| the same, read after a wake | 444 | 5,159 |
| `frame.payload`, 100 B | 26 | 64 |
| throughput, 100 B frames | 307 | 188 |
| throughput, 1 KiB | 2,564 | 1,408 |
| throughput, 64 KiB | 7,869 | 5,447 |
| throughput, 1 MiB | 7,841 | 6,198 |

`frame.payload` by size, C# alone, ns per call: 16 B 47, 64 B 54, 100 B 56, 256 B 79,
1 KiB 81, 4 KiB 88, 64 KiB 87, 1 MiB 85. Past 512 bytes it lends Rust's buffer, so a read
costs the same at any size; at 1 KiB, a copy measured 111. Before the lend, a copy into a
managed array from the large object heap, 1 MiB frames ran at 1,832 MB/s against C's
10,422. What separates 1 MiB from C now is the write: per frame, `write_frame` took 240 µs
at 1 MiB against 5 at 64 KiB, Rust allocating its copy of the frame in fresh pages, while
the read stayed under 0.4 µs.

A call holds its handle as Kotlin's and Go's do ([Concurrent calls](#concurrent-calls)).
Before it did, with a count of its own on each object that every call wrote, 4 threads
sharing one handle cost 208 ns a getter, against 37 on a clone each, at a load of 22. A
woken read awaits a `ValueTask` whose wake posts its next poll to the thread pool, never
running it inside Rust's wake, so a pool thread's handoff dominates it.

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
