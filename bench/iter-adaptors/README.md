# Iterator adaptor spike

**Go for compiler lowering.** Specializing `zip-map-filter-count` while retaining
runtime slice iterators reaches hand-written performance in Chromium and 1.76
times hand-written in Firefox. This is evidence for fusing adaptors, not proof
that megamorphic `MapIter.next` calls caused the gap. No compiler or runtime code
changes in this spike.

## Experiment

`route.py` replaces only chain 8 in a copy of generated `bench.ts`. It removes
map, zip and filter objects and tuple allocation together, retaining two
`rs.iterSlice` leaves and their `next` calls. Both leaves have the input slice's
exact length, so the loop reads left then right exactly that many times. This
case has primitive items, pure closures, and no observable item/closure drops.
It does not justify using that bound or drop behavior for arbitrary chains.

On 2026-11-13, the existing benchmark ran with `--filter iter/` and
`--route specialized=...`, 25 randomized rounds of at least 2 ms, fresh route
workers each round, seed 3461944156. The filter selects codec workload names by
prefix; async handoff workloads still run. It rejects a prefix matching nothing.
The watchdog, native Rust versus wasm checks, and each JS route's expected-output
checks remain enabled. All nine iterator workloads passed those checks in Node,
Bun, Chromium and Firefox.

Host: AMD Ryzen 9 9950X, 32 threads, 60 GiB, Linux 7.3.3 x64.
Engines: Chromium 157.0.8103.38, Firefox 159.0.1, Node 24.22.2, Bun 1.4.4.
The moq pin is ffcb7cd9748422daa30e935ec464108b5a8ebb71.

Medians for `iter/zip-map-filter-count`, ns per input byte (± half IQR):

| engine | native | wasm | hand-written | translated | A/A | specialized |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| Chromium | 0.352 | 0.442 | 3.45 ±3.9% | 26.8 ±11% | 25.1 ±8.5% | 3.41 ±23% |
| Firefox | 0.352 | 0.510 | 3.32 ±19% | 48.5 ±32% | 51.6 ±27% | 5.84 ±41% |
| Node | 0.352 | 0.429 | 3.05 ±6.9% | 26.9 ±6.9% | 27.0 ±3.8% | 3.04 ±18% |
| Bun | 0.352 | 0.599 | 9.59 ±30% | 38.5 ±29% | 30.4 ±27% | 3.86 ±35% |

Chromium improves 7.86 times and Firefox 8.30 times. The improvement exceeds
A/A noise, though Firefox's intervals are wide. Unchanged chains are controls,
not additional specialization results. In particular, enumerate was not edited.
The specialized chain remains slower than wasm; the go criterion is reaching a
small multiple of hand-written, not matching wasm or closing the entire family.

Full minified codec consumer bundle:

| route | raw bytes | gzip bytes |
| --- | ---: | ---: |
| translated | 87299 | 27730 |
| specialized | 87324 | 27773 |
| increase | 25 | 43 |

Async task bundles are unchanged, 16549 raw / 4895 gzip bytes. These numbers
measure one chain in this consumer; they do not predict total growth across a
large crate.

## Profile and limits

Before editing, a signed-in Chrome CPU trace exercised all nine chains for
100000 calls with
`devtools.timeline,v8.execute,disabled-by-default-v8.cpu_profiler` enabled.
The unminified bundle's slice `next` had 932 leaf samples, `MapIter.next` 88,
and trusted zip `next` 155. The trace captured unrelated browser activity;
these are function samples, not percentages of workload time. It identifies
leaf calls as well as adaptor calls, but cannot establish polymorphism or
inlining decisions. Fusion also removes tuple allocation, so attributing the
speedup specifically to map dispatch would overstate the evidence.

An earlier small Chrome harness suggested a similar speedup; the table above
supersedes its numbers and that harness was removed. A full baseline run
completed; the first full-suite A/B run hit Chrome's unchanged 30-second progress
watchdog. The successful focused run measures all nine iterator workloads with
the same benchmark machinery, rather than weakening that watchdog.

The first setup attempt also exposed a local generated cache directory missing
`CACHEDIR.TAG`; restoring the standard cache signature let Cargo clean it. This
was local cache metadata repair, not a benchmark or compiler change.

## Reproduce

Inside `nix develop .#bench`, run these from the repository root. Build and browser
commands belong under `quest gate`; a shell script containing the whole sequence
may be gated once.

```sh
quest gate just ts bench
mkdir -p .tmp/iter-route
cp -r ts/target/bench/translated ts/target/bench/task .tmp/iter-route/
python3 bench/iter-adaptors/route.py .tmp/iter-route
cd ts
quest gate bun bench/main.ts --translated target/bench/translated \
  --moq target/moq --out ../.tmp/iter-results \
  --wasm target/wasm32-unknown-unknown/wasm-release/codec_fixture.wasm \
  --native target/release/examples/bench --task target/bench/task \
  --task-wasm target/wasm32-unknown-unknown/wasm-release/task_fixture.wasm \
  --task-native target/release/examples/handoff \
  --filter iter/ --route specialized=../.tmp/iter-route --seed 3461944156
```

The coordinator clarified that headless browsers launched by the benchmark are
systems under test; agent-browser-only instructions govern manual/web tasks.

The rewritten quest records compiler landing work. Preserve laziness, overflow,
double-ended traversal, size hints and drop ordering with native differential
fixtures before landing any general specialization. Keep the runtime leaf
boundary; do not lower core's pointer-based slice iterator MIR.
