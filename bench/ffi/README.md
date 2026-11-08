# Binding benchmark

`just bench-ffi` measures what each of moq's binding wrappers adds over Rust, with moq-ffi at the revision pinned in [pins.toml](pins.toml) built against [the UniFFI fork](../../uniffi/README.md) in this tree.
It runs nightly in CI ([bench-ffi.yml](../../.github/workflows/bench-ffi.yml)), and every fork perf change states its effect with its numbers.

```bash
just bench-ffi                              # every wrapper this host runs, three rounds
just bench-ffi --runner py --rounds 1       # a quick smoke test
just bench-ffi --base origin/main           # a fork change against its base
```

Results land in `target/bench/ffi/`, newest at `target/bench/ffi/latest/report.md`.
Checkouts and builds live in `/var/tmp/rust2-bench-<user>/ffi/`, with their own `CARGO_HOME` and none of the caller's cargo or rustc variables, as in the [build-speed benchmark](../build/README.md).
Runs on one host share that directory, so a second run waits for the first to finish.

## What runs

The native runners, their recipes, and the Rust baselines stay in moq beside the wrappers they measure (see moq's [bench/README.md](https://github.com/moq-dev/moq/blob/main/bench/README.md)); this harness drives them.
The browser runner lives here, since moq has no browser wrapper over moq-ffi yet.
Every runner runs the same in-process rounds with no network: each publisher writes one group while every subscriber reads it on its own task.
They sweep frame sizes from 100 B to 1 MiB at one publisher and subscriber, then 1 and 4 publishers by 1 and 4 subscribers at 1 KiB, and time two sync calls (`sequence`, `info`).
Each case reports delivered MB/s and process CPU per MB, and checks every received frame byte for byte.

- `rust-ffi` calls moq-ffi's API from Rust without crossing the FFI, so a wrapper's ratio to it is what its boundary and wrapper cost.
- `rust-net` runs the same rounds on moq-net, so its ratio is what moq-ffi's handles and runtime cost.
- `go`, `py`, `kt`, `cpp`, `dart`, and `browser` run on Linux and macOS; `swift` runs on macOS only.
- `browser` builds moq-ffi for wasm32, generates its TypeScript with ubrn's `wasm2` player, and runs [browser-page.ts](browser-page.ts) in headless Chromium through [browser.ts](browser.ts). Its CPU is every thread of the renderer, read over CDP. No ubrn release reads UniFFI 0.32 yet, so [pins.toml](pins.toml) pins the head of [jhugman/uniffi-bindgen-react-native#468](https://github.com/jhugman/uniffi-bindgen-react-native/pull/468), and the run builds its CLI and packs its `@ubjs/core` and `@ubjs/wasm` runtimes from that source.

The run enters the pinned moq's own dev shell, so every wrapper builds with the toolchains moq ships with.
moq's Go, Dart, and C++ scripts find their generators on `PATH`, where the run puts the fork's.
Its Kotlin, Python, and Swift generator and moq-ffi's runtime are the `uniffi` crate, which the run points at the fork's path in every manifest; the run fails if any UniFFI crate still resolves elsewhere.
For the browser, the run also links ubrn's `uniffi-runtime-wasm` helper crate into moq-ffi on wasm32, as ubrn's player requires.
Bump a pin in its own change, since numbers on either side of it do not compare.

## Variants

- `fork` builds against this tree.
- `control` reruns fork's builds, so its change against fork is the run's noise floor: an A/A control.
- `base`, with `--base REV`, builds against the fork at another rust2 revision, so fork's change against it is the effect of the change.

An untimed warm-up round builds every runner once per checkout.
The timed rounds interleave the variants and rotate which goes first each round, so drift over a long job spreads across all of them.
The report shows each case's trimmed mean over the rounds (without the fastest and slowest) with its cost against `rust-ffi`, taken within each round so drift between rounds cancels, then each comparison as a change in that cost, so one slow round on a busy host does not move it.
`results.jsonl` keeps every round's value, mean, and standard deviation.
The host, its load average before and after, and every toolchain's version head the report.
No threshold is set: the numbers inform a decision, they do not gate one.

## BoltFFI comparison

Before forking UniFFI, moq compared it with [BoltFFI](https://github.com/boltffi/boltffi) 0.31.1 on a slice of moq-ffi bound by each generator, measured from Python and Kotlin with a Rust floor.
Its harness is deleted; [moq-dev/moq@0f986d9](https://github.com/moq-dev/moq/tree/0f986d9fdb7c074b0bd4b34739f7199c3bf2d9e5/bench/ffi-gen) holds it if a decision needs fresh numbers.
BoltFFI's numbers are the targets for the fork's backends: its speed comes from calling conventions a UniFFI fork can adopt one backend at a time (a CPython extension instead of `ctypes`, JNI instead of JNA, and bulk byte copies), not from its core.

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
