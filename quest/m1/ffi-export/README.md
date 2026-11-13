# Exporter: moq's bindings from its crates' pub API

## Goal

moq can pin [rust2-ffi](/ffi/README.md) and switch its bindings to it: the exporter exports
what moq's bindings need from moq's own crates, with no attributes, no config, and no
hand-written wrapper beyond what the census shows is genuinely new API, in every language
moq ships. It ends once every child has merged, the last of them
[ship](/quest/m1/ffi-export/ship.md), which is what
[moq's wait](https://github.com/moq-dev/moq/blob/main/quest/m3/rust2-exporter.md) checks.

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
  ones ([lossless](/quest/m1/ffi-export/lossless.md)). Rejected: idiomatic types whose
  truncations and aborts become errors; today's mappings.
- ✅ A failed run-time check raises the language's own error, and a Rust panic aborts
  ([checks-raise](/quest/m1/ffi-export/checks-raise.md)). Rejected: catching panics at the
  boundary; aborting on every failed check.
- ✅ A type or future that is not `Send` exports, checked by thread
  ([not-send](/quest/m1/ffi-export/not-send.md)). Rejected: a Rust-owned thread for every
  such object; keeping the refusal.
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

Every generator, the [targets m3 plans](/quest/m3/README.md) included:

- checks `<prefix>_contract()` against the header's contract when it loads, before any
  other call, and fails loud on a mismatch (done in all six);
- keeps the C ABI's bulk and zero-copy payloads, with no per-byte loop on either side (done);
- wraps the C ABI's futures in the language's native future (done). Those futures already
  take the result in the poll that finishes them, so a cancel never loses a frame, cost one
  crossing when ready at once, and reuse a repeated waker. The native future must latch a
  wake that lands between `PENDING` and parking, prove no loss on cancel in a test that
  forces that window, and pick a semantic for concurrent pollers. Decided with the
  maintainer on 2026-10-27: async lands with each generator, not as its own quest.
- drives each of moq's `Driver`s from the language's own event loop (done in all seven
  `ffi/moq/moq.*`). Decided with the maintainer on 2026-10-27: the C ABI exports the poll,
  with no generated thread and no moq-tokio helpers, which would tie bindings to tokio. A
  binding holds one `kio::Waiter` per driver, made from its waker, since what a poll
  registers lives only as long as its waiter; it polls on each wake and at the deadline the
  poll returns, and frees the driver to cancel it. Decided with the maintainer on
  2026-10-27, for Go: a cancel ends only that read and leaves the stream usable, as in
  Rust, since a cancelled C ABI future loses nothing; moq's `go/wrapper/doc.go`, which ends
  the stream, changes with the generator. Each driver runs on a goroutine that lives as
  long as its object, and a call's `ctx` bounds only that call.
- is measured, with its generated size raw and gzipped, on
  [the binding benchmark](/bench/ffi/README.md), whose `just ffi bench` covers it.

Decided with the maintainer on 2026-11-12, in the quest audit:

- ✅ The epic's closing work is a child, [ship](/quest/m1/ffi-export/ship.md), which alone
  requires [the refusals epic](/quest/m1/ffi-refusals/README.md). Rejected: the epic
  requiring it, as [#701](https://github.com/kixel-ai/rust2/pull/701) left it, since an
  epic's `Required` holds back every child, so none could start until moq's pin bump, which
  then waited on a rind release the maintainer held.
- ✅ Every change that breaks the C ABI or a generated API is a child here, so it lands
  before moq pins: the fused async start,
  [owned buffers](/quest/m1/ffi-export/owned-bufs.md), and
  numeric slices ([#743](https://github.com/kixel-ai/rust2/pull/743)). Work that changes no generated API is
  [its own epic](/quest/m1/ffi-speed/README.md), which does not hold the pin.
- Not planned: one walker emitting lift and lower instructions every generator
  implements, as wit-bindgen's `Bindgen` does. A new rule touches 19 to 38 files today,
  but it would collide with the four `[L]` children that each rewrite every generator.
  Spike it on Go once they land.

Decided with the maintainer on 2026-11-02
([kixel-ai/rust2#460](https://github.com/kixel-ai/rust2/pull/460)): the RSS glibc's per-thread
arenas keep under Go's lent `Bytes` is the consumer's to fix with its global allocator, as
`ffi/README.md` documents. moq declares mimalloc in its exported crate, in its own tree.

Decided with the maintainer on 2026-11-04: the exporter is configless, with opinionated
names and defaults, and fails the export on a public item it cannot export.
