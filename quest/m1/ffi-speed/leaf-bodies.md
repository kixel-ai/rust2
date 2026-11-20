# [M] Leaf calls through trivial method bodies

## Goal

A method whose body the export proves reaches no user code, such as moq's
`group::Demand::sequence` (`self.sequence`), and a `pub` field reached through a `Deref`
whose `deref` is such a body, as `group::Producer`'s `sequence` is, are leaves as
`Abi::leaf` in `ffi/src/abi.rs` defines them: Python keeps the GIL across them and Dart
makes them leaf calls. A body is proven by its MIR, not by its shape or an attribute. It
lands only where [the binding benchmark](/bench/ffi/README.md#leaf-calls) shows it faster,
with the export's added time and memory measured beside it.

## Plan

Decided with the maintainer on 2026-11-19, in the pick of [#887](https://github.com/kixel-ai/rust2/pull/887): ✅ keep it, and measure what `-Zalways-encode-mir` on dependencies costs the export's build first, choosing between it and per-crate facts recorded in the driver by that measurement (rejected: either route chosen outright; dropping the quest).

Found by the [crossings](https://github.com/kixel-ai/rust2/pull/887) work. A leaf is now
only a primitive `pub` field read or written directly, or a `PartialEq` or `Hash` derived
down to primitives, since a native export reads no MIR. Most accessors are methods: the
benchmark's `demand.sequence()` costs Python 65 ns against 26 for a leaf field, and Dart
23 against 13.

Recommended: prove a body a leaf when its optimized MIR has no call but to bodies proven
the same way and to std's own functions known to run no other code, no drop of a value
whose drop runs code, no inline assembly, and no access to a `static`; a body that fails
is simply no leaf, as today. Upstream crates' MIR exists in a check only under
`-Zalways-encode-mir`, which the web export already passes (about 11 CPU s and twice the
peak memory on the stand-in, per the benchmark's "The export's build cost"). Measure that
cost on the native export first. If it is too high, the driver already wraps every crate's
rustc, so it could record each crate's leaf bodies while compiling it instead, keyed by
the crate's metadata hash; compare the two before choosing.
