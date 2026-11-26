# [M] Zero-privilege heap allocation counters via preload

## Goal

`swole` reports exact per-benchmark heap allocation metrics—`alloc_count`, `alloc_bytes`, and `peak_live_bytes`—across every mode each OS supports (`quick`, `sim`, and `bench` on Linux; `bench` on macOS) with zero OS privileges and zero noise, by interposing allocator calls in a dedicated allocation-only library during the bracketed benchmark window.

## Plan

Decided 2026-11-25 (maintainer, `/quest-plan` in rust2):

- ✅ Include zero-privilege heap allocation counters via a dedicated allocator-interposer library, built and loaded like `preload.rs`, in the core `swole` epic. Every OS exposes this without root or `CAP_PERFMON`, it is deterministic for single-threaded code whose allocations do not depend on the host clock (which `bench` uses), and it catches allocation regressions that wall-clock and instruction counts often mask.

Recommendations for the implementer:

- Interpose `malloc`, `calloc`, `realloc`, `free`, `posix_memalign`, `aligned_alloc`, and `memalign` in that library, separate from `preload.rs`'s randomness and clock shim (`LD_PRELOAD` on Linux, `DYLD_INSERT_LIBRARIES` on macOS, using `dlsym(RTLD_NEXT, ...)` to forward to the real allocator and avoiding recursion during initialization).
- Expose a tiny C symbol or control hook from that library that the harness calls in `Meter::start()` / `Meter::stop()` to reset, enable, pause, and read thread-safe atomic counters (`alloc_count`, `alloc_bytes`, `live_bytes`, `peak_live_bytes`) so setup and teardown allocations never pollute the benchmark's numbers.
- Handle static binaries or custom Rust `#[global_allocator]` crates (e.g. jemalloc/mimalloc linked statically without replacing libc malloc): detect at probe/run time when an allocator bypasses libc interposition rather than silently reporting zero for an allocating benchmark, or document the exact contract and test with stock `std::alloc::System`.
- Prove exactness with a differential test comparing the interposer's `alloc_count`, `alloc_bytes`, and `peak_live_bytes` against a Rust `#[global_allocator]` counting wrapper on workloads with `Vec`, `Box`, `String`, reallocations, and aligned allocations.
- Keep the hooks' instructions out of every other metric, in every mode: only a dedicated single-shot native allocation pass loads the interposer. The `sim` Callgrind run and the `quick` perf-counter pass keep `preload.rs`'s fixed-seed randomness and clock shim, which `sim` requires and which makes RNG and clock inputs repeatable, but not the interposer. `bench`'s timed and counter passes load neither library, so a benchmark that uses real timers (such as an absolute `timerfd_settime`, which the shim rejects) measures against the host clock. No pass but the allocation one carries allocation-hook instructions, so `Ir`, simulated cycles, attribution, counters, and wall-clock times exclude them. The allocation pass loads the same shim as its companion pass plus the interposer for `quick` and `sim`, and only the interposer for `bench`, so it takes the same RNG and clock inputs as the pass whose workload it counts; it takes only the three allocation metrics, and `Meter::start()` / `Meter::stop()` bracket the same window in each pass.

## Required

- [Three modes (quick, sim, bench), capability probe, CAP_PERFMON, L2/L1 perf, and OS rusage](/quest/m2/swole/modes-and-counters.md) - reports allocation metrics in the unified CLI tables and `metrics.json`
