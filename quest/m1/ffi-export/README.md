# Exporter: moq's bindings from its crates' pub API

## Goal

moq can pin [rust2-ffi](/ffi/README.md) and switch its bindings to it: the exporter exports
what moq's bindings need from moq's own crates, with no attributes, no config, and no
hand-written wrapper beyond what the census shows is genuinely new API, in every language
moq ships. It ends once every child has merged, which
[moq's wait](https://github.com/moq-dev/moq/blob/main/quest/m3/rust2-exporter.md) checks by
this file being gone. The flake's `ffi` package, the root README's list of generators, and
the rules every generator follows, now in [ffi/README.md](/ffi/README.md#every-binding),
shipped first; the child that lands last deletes this file.

## Plan

Decided with the maintainer on 2026-10-25, from the spike
([kixel-ai/rust2#41](https://github.com/kixel-ai/rust2/pull/41)): adopt the exporter, and fold
the fork's paused feature quests into it.

Decided with the maintainer on 2026-10-27: this is an epic, with each item a child ranked by
what the census refuses and what moq waits on first.

Decided with the maintainer on 2026-11-09, in the quest audit:

- ✅ The epic ends where moq can pin it. Rejected: ending with moq-ffi shrunk to its
  remainder, which only [moq's switch](https://github.com/moq-dev/moq/blob/main/quest/m3/ffi-exporter.md)
  does, and that waits on this epic, so each waited on the other; ending at a clean census
  with the children below as a later epic, which would break the generated APIs after moq
  adopts them. Swapping the stand-in package for the real moq-ffi is a pin bump after
  moq's switch.
- ✅ [Sessions on sans-IO QUIC](/quest/m3/session-quic.md) left the epic for m3, since it
  waits on moq's m3 driver and would hold the epic open. Rejected: m2; staying in m1.
- ✅ A binding's types are lossless, with checked conversions to and from the idiomatic
  ones ([#832](https://github.com/kixel-ai/rust2/pull/832)). Rejected: idiomatic types whose
  truncations and aborts become errors; today's mappings.
- ✅ A failed run-time check raises the language's own error, and a Rust panic aborts
  ([#730](https://github.com/kixel-ai/rust2/pull/730)). Rejected: catching panics at the
  boundary; aborting on every failed check.
- ✅ A type or future that is not `Send` exports, checked by thread
  ([#844](https://github.com/kixel-ai/rust2/pull/844)). Rejected: a Rust-owned thread for
  every such object; keeping the refusal.
- ✅ No opt-out: `#[doc(hidden)]` items stay left out, as they are no part of a crate's API,
  but no quest or PR recommends the attribute to get an item past the exporter, and moq
  bans it ([moq-dev/moq#8392](https://github.com/moq-dev/moq/pull/8392)).
  An item C cannot take exports by a new rule, or is refused and its signature changes.
  Rejected: `#[doc(hidden)]` as the opt-out; exporting hidden items too.

The Swift, Python, Go, and Dart generators landed in
[#406](https://github.com/kixel-ai/rust2/pull/406), [#407](https://github.com/kixel-ai/rust2/pull/407),
[#411](https://github.com/kixel-ai/rust2/pull/411), and [#422](https://github.com/kixel-ai/rust2/pull/422),
each generating the crate's own API in its language's idiom, as Kotlin does (decided with
the maintainer on 2026-11-01). Only Python skips the C ABI, since a CPython extension calls
about 40 times faster than `ctypes`; the others call C natively.

The rules every generator follows, the [targets m3 plans](/quest/m3/README.md) included,
are in [ffi/README.md](/ffi/README.md#every-binding). Decided with the maintainer on
2026-10-27: async lands with each generator, not as its own quest; the C ABI exports each
driver's poll, with no generated thread and no moq-tokio helpers, which would tie bindings
to tokio; and in Go, a cancel ends only that read and leaves the stream usable, with each
driver on a goroutine that lives as long as its object, and a call's `ctx` bounding only
that call.

Decided with the maintainer on 2026-11-12, in the quest audit:

- ✅ The epic's closing work is a child, ship, which alone
  required the refusals epic, since finished. Rejected: the epic
  requiring it, as [#701](https://github.com/kixel-ai/rust2/pull/701) left it, since an
  epic's `Required` holds back every child, so none could start until moq's pin bump, which
  then waited on a rind release the maintainer held.
- ✅ Every change that breaks the C ABI or a generated API is a child here, so it lands
  before moq pins: the fused async start,
  owned buffers ([kixel-ai/rust2#744](https://github.com/kixel-ai/rust2/pull/744)), and
  numeric slices ([kixel-ai/rust2#743](https://github.com/kixel-ai/rust2/pull/743)). Work that changes no generated API is
  its own epic, which did not hold the pin and was retired on 2026-11-16 once its children
  landed.
- Not planned then: one walker emitting lift and lower instructions every generator
  implements, as wit-bindgen's `Bindgen` does, until the four `[L]` children that each
  rewrite every generator landed. They have, and its spike on Go
  ([kixel-ai/rust2#924](https://github.com/kixel-ai/rust2/pull/924)) recommends against
  it: Go's output stayed byte-identical, but the walker would have absorbed about 12 of the
  172 lines [kixel-ai/rust2#710](https://github.com/kixel-ai/rust2/pull/710) added to
  `go.rs`, and no files, since a rule's cost is each language's spelling, runtime, and tests.

Decided with the maintainer on 2026-11-18, in the per-language audit:

- ✅ Breaking changes to a generated API that the audit found are children here, and land
  as soon as possible: "You're allowed to break the API absolutely. Do it sooner." They
  are the API surface lock (`ffi/surface/`), which landed first,
  C and C++, Swift (landed),
  Kotlin (landed), Python (landed), Dart (landed), and docs.
  Rejected: only the surface lock holding the pin; all of them after the pin.
- ✅ Work that changes no generated API, or only adds to it, stays out of this epic, as
  decided on 2026-11-12: speed (landed), rule cost (landed),
  [packaging](/quest/m1/ffi-packaging/README.md), ordering and stream adapters (landed),
  with [Swift streams of noncopyable objects](/quest/m1/swift-async-reader.md).
- Dropped: a per-function contract in place of the whole-header hash, since moq links
  statically and the strict check is what fails loud; C layout `_Static_assert`s, since
  mainstream ABIs agree and the API lock catches layout changes; and Kotlin Multiplatform,
  since Kotlin's Swift export is Alpha in 2.4 and moq ships native Swift.

Decided with the maintainer on 2026-11-02
([kixel-ai/rust2#460](https://github.com/kixel-ai/rust2/pull/460)): the RSS glibc's per-thread
arenas keep under Go's lent `Bytes` is the consumer's to fix with its global allocator, as
`ffi/README.md` documents. moq declares mimalloc in its exported crate, in its own tree.

Decided with the maintainer on 2026-11-04: the exporter is configless, with opinionated
names and defaults, and fails the export on a public item it cannot export.
