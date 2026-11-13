# Iterator adaptor spike

Preliminary Chrome measurements, 2026-11-13. No compiler or runtime changes.
The experiment specializes chain 8 in the checked-in codec fixture's output,
leaving both slice iterators in the runtime. Its exact, equal leaf lengths let
the generated loop remove tuple allocation and adaptor dispatch together.
It does not isolate dispatch from allocation or prove a general lowering.

The attached `route.py` edits a copied route, never the golden output.
`measure.ts` is the small exploratory harness used here, copied to `.tmp/iter/`.
It imports the baseline from `base/` and the edited copy from
`special/translated/`. Both contain `ts/fixtures/codec/expected/`, augmented
with `ts/runtime/rs.ts` and `ts/runtime/rs/`. Build with:

```sh
python3 bench/iter-adaptors/route.py .tmp/iter/special
quest gate bun build .tmp/iter/measure.ts --target browser --format iife --outfile .tmp/iter/measure.js
```

Load the IIFE through `agent-browser eval --stdin` in a dedicated session.
`measure()` returns all 15 rounds; `validate()` compares the edited output
with the original over lengths 0 through 1024. Use `quest gate` for browser work.
The measured input is 1024 bytes cycling through 0..255, 10000 calls per round,
alternating baseline and specialization order. The direct loop writes the
same count with `putUsize`. All routes include output allocation. There are
no fresh workers, randomized input, or confidence intervals in this harness.
The direct loop closes over the input, so its result is only a rough reference.

Median nanoseconds per input byte:

| route | ns/byte |
| --- | ---: |
| translated | 17.01 |
| specialized | 1.99 |
| direct loop | 2.77 |

A preceding Chrome CPU trace exercised all nine chains for 100000 calls with
`devtools.timeline,v8.execute,disabled-by-default-v8.cpu_profiler` enabled.
The unminified bundle's slice `next` had 932 leaf samples, `MapIter.next` 88,
and trusted zip `next` 155. The trace also captured unrelated browser activity;
these are function samples, not percentages of workload time. This supports
investigating leaf calls and allocation as well as adaptor dispatch, and does
not confirm megamorphic map calls as the cause.

The specialization is about 8.5 times faster in this exploratory harness.
Recommendation: complete the specified experiment before deciding to implement
compiler lowering. Still needed: native Rust differential validation, full
`--route specialized=...` comparison, Firefox, and a second profile isolating
inlining from allocation. The repository benchmark launches browsers directly,
whereas this session's instructions require agent-browser, which only gives
access to Chrome. Resolve that instruction conflict before running Firefox.
No go/no-go conclusion is claimed.

The browser comparison passed for all 1025 tested lengths. This is comparison
against translated output, not the required native Rust differential proof.
Standalone minified `bench.ts` exports measured 87704 raw / 27758 gzip bytes
before, and 87729 raw / 27788 gzip after (+25 raw / +30 gzip). These are not the
full consumer bundles produced by `--route`.
