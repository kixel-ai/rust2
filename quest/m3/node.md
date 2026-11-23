# [L] A Node-API addon

## Goal

[rust2-ffi](/ffi/README.md) writes a Node-API addon for the crate, built into the same
library with `--features node`, as Python's CPython extension is, and TypeScript over it
in [the web target's](/ffi/README.md#typescript-web) API shape, so one application's code
calls either. It follows every generator's rules
([every binding's](/ffi/README.md#every-binding)), the `api` fixture's script runs through
it on Node, Bun, and Deno with a transcript matching native Rust's, including the items
the web target leaves out for want of threads, and it is a column on
[the binding benchmark](/bench/ffi/README.md).

The report that earned it ([kixel-ai/rust2#970](https://github.com/kixel-ai/rust2/pull/970))
compared three routes from Node: the web target, `node:ffi` over the C header, and a
Node-API addon. Decided with the maintainer on 2026-11-22: ✅ the Goal accepted all three
outcomes, an addon generator, a thin generated TypeScript file over `node:ffi`, or deleting
this quest with its numbers, judged on speed where every route runs a workload, and on
capability where the web target refuses the crate at export. The addon won both.

## Plan

Decided with the maintainer on 2026-11-09, in the quest audit: ✅ planned in m3 before a
consumer asks ([m3](/quest/m3/README.md#plan)).

Decided with the maintainer on 2026-11-22, confirming the 2026-11-12 audit's open point: ✅
the outcome was judged on speed where every route runs a workload, and on capability where
the web target refuses the crate at export. Rejected: the addon landing only where it beats
both others on speed alone.

Decided with the maintainer on 2026-11-22
([kixel-ai/rust2#970](https://github.com/kixel-ai/rust2/pull/970#issuecomment-7089673616)):
✅ the report landed alone, and the quest's next PR builds the generator (rejected: holding
the report's PR open until the generator lands in it).

The report, measured 2026-11-22 on **desktop** (24 cores), through `quest gate`, Node
26.13.1, 5 rounds alternating the routes at a load of 20 to 29, bench/ffi's cases on moq's
stand-in; median wall ns per call. The harness is in
[4d9d0698](https://github.com/kixel-ai/rust2/pull/970/commits/4d9d06986e980907c35a99896fb6dd84ef9caedb),
and the commit after it deletes it.

| case | web target | `node:ffi` | Node-API |
|---|---|---|---|
| handle call | 37.1 | 41.8 | 26.7 |
| u64 getter | 39.1 | 40.3 | 18.4 |
| record return | 61.6 | 84.7 | 52.8 |
| object return | 113 | 162 | 58.5 |
| write, 100 B | 193 | 366 | 183 |
| stamped write | 301 | 494 | 219 |
| ready read | 527 | 915 | 402 |
| woken read | 1,230 | 2,370 | 1,095 |
| payload, 100 B, lent | 124 | 322 | 61 |
| payload, 1 MiB, lent | 126 | 312 | 67 |
| throughput, 100 B, MB/s | 128 | 79 | 219 |
| throughput, 64 KiB, MB/s | 10,902 | 10,518 | 11,986 |

- **Speed**: the addon is fastest on every per-call case. Throughput trailed at 64 KiB frames
  only against the web target under `--no-liftoff` (11,986 MB/s against 12,050, within
  round-to-round noise), and at 1 MiB frames (3.4 GB/s against the web target's 5.9) only
  because glibc gives freed memory back and faults it in again, which wasm's linear memory
  never does: with
  `MALLOC_TRIM_THRESHOLD_` and `MALLOC_MMAP_THRESHOLD_` set to keep it, both native routes
  reach 13.7 to 14.2 GB/s against the web target's 5.9 to 11.4.
- **Capability**: the web target refuses moq-tokio, moq's native QUIC (it builds for no
  wasm32 target), and the api fixture leaves out for wasm32 every item that runs Rust on
  another thread: `Spawned`, `LogThread`, `emit_later`, and `filter`'s closure on another
  thread. Every one calls JavaScript from Rust's own thread, as each of moq-tokio's wakes
  does. `node:ffi` aborts the process on such a call ("Callbacks can only be invoked on
  the system thread they were created on"). Node-API takes it, through a threadsafe
  function while the caller's loop runs, or, while the caller waits in Rust, by serving
  the call on the caller's thread, as Dart's binding does.
- **`node:ffi` is rejected**: slower than the web target on every per-call case, since a
  pointer crosses as a `bigint` and the fast path allocates one per handle returned. It
  has no struct type, so 10 of the 40 C functions the benchmark calls needed a pointer-only twin in
  the glue (each passing or returning a `moq_buf`, a `moq_waker`, a `Timestamp`, or an
  `Option`), and it cannot take a call from another thread. Bun and Deno have their own
  FFI, so it would need a file per runtime.
- **One addon serves Node, Bun, and Deno**: the same `addon.node` ran all of the cases
  unchanged on Bun 1.4.4 and Deno 2.9.8.

What the generator copies from the report's addon:

- A handle crosses as its address in a number, and the TypeScript wraps it in an object
  that `[Symbol.dispose]` frees, as the web target's `rt.Handle`. A number holds an
  address exactly below 2^53, which a default 64-bit user space stays under: Linux maps
  above 47 bits on x86-64 (56-bit with 5-level paging) or 48 on arm64 (52-bit) only for an
  `mmap` hint that asks. The addon still checks each address it hands out against 2^53 and
  aborts above it, one compare, rather than assume no allocator passes such a hint. A `napi_external` per object return cost 89 ns more.
- Never a JavaScript object made in C per call: Node-API's `napi_create_arraybuffer` and
  its typed array cost 0.4 to 0.7 µs. An opaque value, a record's bytes, and a copy of up
  to 64 bytes go through a scratch buffer the TypeScript views, as the web target's stack
  in linear memory, and the TypeScript copies them out.
- Lent bytes are read through calls, `at(i)` and a `slice()` that copies, as `rt.Bytes`
  reads linear memory, never through an external `ArrayBuffer`, which cost 0.7 µs to make.
  That gives 61 ns a lend at any size.
- A wake on the loop's thread calls JavaScript at once and polls on a microtask. One from
  another thread goes through a threadsafe function. A call into JavaScript from Rust's
  thread while the caller waits in Rust is served on the caller's thread.
- The addon's `napi_*` symbols resolve from the host at load, so it builds apart from the
  library C programs link, as the CPython extension does.
- The addon calls the C ABI in-process, so it keeps its checks: a failed one throws the
  language's own error, naming the C function, as each binding raises.

- No tool was found that generates Node-API bindings from a C header; napi-rs is
  annotation-based.
- Node-API is ABI-stable across Node versions.
- The generated API is the one [the gate](/quest/m2/wasm/README.md) settled on
  2026-11-12: a module of functions per type, not classes.

Public API: the generated TypeScript API and the `node` feature.
