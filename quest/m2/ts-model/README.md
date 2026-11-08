# [L] Translate moq-net's model to TypeScript

## Goal

[rust2-ts](/ts/README.md) translates moq-net's model layer, with the workspace crates it
calls (kio, with its clock, and moq-pattern), as it translates the codec today, so moq can ship
`@moq/net` as JavaScript translated from its Rust, in place of the handwritten package.

## Plan

Decided with the maintainer on 2026-11-01, after moq removed its WASM build
([moq-dev/moq#7475](https://github.com/moq-dev/moq/pull/7475)): the translated model is
moq's browser route, which moq's [JS-only `@moq/net`](https://github.com/moq-dev/moq/blob/main/quest/m5/rs2ts-js.md)
adopts once it matches the handwritten package in a full browser workload. Once the model
translates, moq measures it on the [TS benchmark](/ts/README.md#bench), consuming the driver
as the `ts` package of this flake. That adds moq's `js/net/bench/frames.ts`, a group stream
read into a group, to the benchmark's workloads, which hold only its sync frame headers. The
codec's WASM route stays in the benchmark as the baseline of compiled Rust in a browser.

The codec's differential harness is the pattern to repeat. The shared npm runtime
(for checked integer ops and the rest of `rs.ts`) is published only once two
generated packages need it.

Decided with the maintainer on 2026-10-23, for the subset: `u128` stays, on the
[wide-integer](/ts/README.md#design) representation.

Decided with the maintainer on 2026-10-27, from a census of the model's 22 public types
(origin's, broadcast's, track's, group's, frame's, and bandwidth's producers and consumers,
`cache::Pool`, `listing::Tracks`, `time::Timestamp`): 4,595 refusals over 1,112 items.
2,588 are calls into the workspace crates, and 424 items refuse nothing else.

- Each group of refusals is its own quest. moq must work, but nothing in
  the translator is specific to moq: dependencies follow general rules, and moq's own
  changes (to its `unsafe` and its address keys, dropping or binding `hashlink` and
  `slab`) are one research-tier quest in moq's tree,
  [rs2ts-model-subset](https://github.com/moq-dev/moq/blob/main/quest/m5/rs2ts-model-subset.md).
- A dependency containing `unsafe` translates when it uses only the `unsafe`
  [m3's unsafe policy](/quest/m3/README.md) translates; a common crate whose `unsafe` it
  does not is mapped into the runtime at its safe API, as hashbrown is. Decided with the
  maintainer on 2026-10-27: a stock crate translates with no annotations or config. The
  model's binds (DEFLATE, its clock, and its task sets) are gaps
  [ts-configless](/quest/m3/ts-configless.md) closes.
- Each child measures this target by the census of `ts/moq/model.toml`, rooted at those
  types with the boundaries bound, which nightly reports beside the whole crate.
- The end-to-end work stays here: the model's roots, its differential harness, and the
  `frames.ts` workload. moq writes the binding files `model.toml` names, in its own tree.

The census at the pin on 2026-11-07: 45 refusals. `Timestamp` alone refuses nothing, and
the [model fixture](/ts/README.md#tests)'s harness runs it against native moq-net at the
pin; each case it adds as a type translates. Every other root reaches the rest of the
model, all of it moq's, for its research quest: kio's `log.rs` reads bytes as its segments
and retires them through crossbeam-epoch, whose tagged pointers and inline closures (36
refusals) translation refuses for good, so moq rewrites or drops them, as it does the
`unsafe` slice of `model/frame.rs`'s `Filled`. smallvec's union over a type parameter and
raw parts (4) go once [smallvec maps into the runtime](/quest/m2/ts-model/smallvec.md)
(decided with the maintainer on 2026-11-07, in the quest audit, under rule 2 of
[m3's unsafe policy](/quest/m3/README.md); rejected: moq dropping smallvec).

Decided with the maintainer on 2026-11-04, from a census at the bumped pin
(moq-dev/rust2#390): the refusals no quest owned became child quests, since finished.
✅ arrayvec's `ArrayString` needs three of them (const generic types, `begin_panic`,
`str::as_ptr`), kept separate since each lands alone; rejected: one quest for `ArrayString`.

Decided with the maintainer on 2026-11-07, after three designs for addresses as identity
([#596](https://github.com/moq-dev/rust2/pull/596)) each left values whose ids were never freed
or pointers whose comparison disagreed with Rust after an address was reused:

- ✅ Addresses stay refused: every read of a pointer's address as an integer (`as usize`,
  `addr`, `expose_provenance`, a raw pointer's `Hash` or `Ord`) is refused with a span, and
  pointer identity (`ptr::eq`, `Arc::ptr_eq`, `Weak::ptr_eq`) keeps translating as JS
  identity. Rejected: ids for an `Arc`'s or `Rc`'s value, and only for pointers that visibly
  come from one (each still refuses moq's two reads in `Drop`); exact addresses for every
  pointer, which needs a linear-memory model. Plan pointer addresses again once a library
  other than moq needs them.
- ✅ moq keys `Track` and `FrameContent` by an explicit id in both builds, in place of their
  addresses (`model/cache.rs`, `model/frame.rs`).
- ✅ moq gives its `unsafe` no `cfg(rust2_ts)` twins; it decides per item, when its quest
  starts, whether to rewrite one in safe Rust for both builds or wait for this translator.
  Rejected: twins until rust2-ts translates them, which [m3's unsafe policy](/quest/m3/README.md) mostly refuses.

Public API: the translator's output for moq-net's model.

## Waiting

- moq-dev/moq: [rs2ts-model-subset](https://github.com/moq-dev/moq/blob/main/quest/m5/rs2ts-model-subset.md)
  lands: moq rewrites kio's `log.rs` and its own `unsafe` in safe Rust, or drops them from the
  model's path. moq defers it to its m5. Check by the model census at the pin.

## Required

- [smallvec maps into the runtime](/quest/m2/ts-model/smallvec.md) - `SmallVec` is the runtime's `rs.Vec`, so smallvec's 4 refusals go without moq dropping it
