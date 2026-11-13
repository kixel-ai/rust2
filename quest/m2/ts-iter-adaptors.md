# [S] Spike: iterator adaptors lowered per chain

## Goal

A report says whether lowering an iterator chain's adaptors (`map`, `filter`, `zip`,
`enumerate`) to code of its own, with leaf iterators left in the runtime, closes the
iterator workloads' gap. On the 2026-11-12 nightly
([run 43246857664](https://github.com/moq-dev/rust2/actions/runs/43246857664), Chromium)
the iterator family ran 12.9 times wasm's time: `zip-map-filter-count` 56.4 ns against
2.84 hand-written, and `enumerate-for` 12.2 against 1.89. If the spike earns it, this
quest is rewritten to the landing work and sized again.

## Plan

Decided with the maintainer on 2026-11-12, in the quest audit: ✅ before the gate, in m2.

- The suspected cause: every `map` in a program calls through one site,
  `ts/runtime/rs/iter.ts`'s `MapIter.next`, so its `inner.next()` and `f` calls see every
  type and V8 stops inlining them. Confirm it with a profile before changing anything.
- `ts/README.md`'s design rejected lowering adaptors from core's MIR because slice
  iterators are raw pointers. This keeps leaf iterators in the runtime and specializes
  only the adaptors, as [#129](https://github.com/moq-dev/rust2/pull/129) did for
  `Option`'s and `Result`'s combinators.
- Hand-edit one `iter/` workload's output and compare it through `--route name=dir`, in
  Chrome and Firefox, with the bundle's size, which grows per chain.
- Go: the hand-edited workload reaches a small multiple of hand-written. No-go: it does
  not, and the report says what the profile showed instead.

Public API: none.
