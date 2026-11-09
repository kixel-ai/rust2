# m4: long-term research

## Goal

Ideas worth recording that no consumer needs yet, each kept as a quest so its
blockers and evidence are not lost.

## Plan

Decided with the maintainer on 2026-10-25: m4 holds long-term research, and
opens only when the work before it leaves room or new evidence revives an idea.

Declined on 2026-10-25: a drop-in build-cache wrapper with an early cutoff.
The cutoff is unsound on stock rustc (a stale SVH gives E0460, and DefIndex
shifts), mbx and cargo already cover caching and GC, and a sound cutoff holds on
only 7% of moq-net edits
([moq-dev/rust2#62](https://github.com/moq-dev/rust2/pull/62),
[moq-dev/rust2#70](https://github.com/moq-dev/rust2/pull/70)).
[The fork](/quest/m4/fork.md) collects the reasons to change rustc and cargo instead.

Closed on 2026-11-06: a compiler for a stricter Rust subset
([moq-dev/rust2#615](https://github.com/moq-dev/rust2/pull/615)). 61% of the
LLVM IR moq's own crates emit is std's and dependencies' generics instantiated
at moq's types, closures included, so it would have to compile std too. Levers
that need no new frontend reach its best case: ra-jit already applies the
moq-net body edits it serves in under half a second, and Cranelift cuts a clean
build's retired instructions by 16%.

## Required

- [Bench TPDE on moq](/quest/m4/tpde.md) - what TPDE cuts from moq's clean build and non-incremental edit; waits on `-Zllvm-tpde` reaching a nightly
- [Waiting: a rustc and cargo fork](/quest/m4/fork.md) - the evidence ledger for carrying a fork; condition: enough measured reasons to pay for one
- [moq in pure Go](/quest/m4/go/README.md) - rust2-go translating moq-net, with no cgo, fast enough to replace the cgo bindings for Go users; parked until a consumer needs moq in pure Go
