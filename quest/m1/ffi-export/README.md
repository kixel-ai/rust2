# Exporter: moq's bindings from its crates' pub API

## Goal

[rust2-ffi](/ffi/README.md) replaces [the UniFFI fork](/uniffi/README.md) for moq: it exports
what moq's bindings need from moq's own crates, with no attributes and no hand-written wrapper
beyond what the census shows is genuinely new API, and generates the languages moq ships on its
C ABI. moq-ffi shrinks to that remainder. The fork stays in maintenance, building moq's bindings
until then.

## Plan

Decided with the maintainer on 2026-10-25, from the spike
([moq-dev/rust2#41](https://github.com/moq-dev/rust2/pull/41)): adopt the exporter, and fold
the fork's paused feature quests into it.

Decided with the maintainer on 2026-10-27: this is an epic, with each item a child ranked by
what the census refuses and what moq waits on first, then the runtime, then the generators.
What stays here is the end-to-end: moq-ffi shrinks to its remainder and takes the stand-in
package's place.

Every generator:

- checks `<prefix>_contract()` against the header's contract when it loads, before any
  other call, and fails loud on a mismatch;
- is measured on the [binding benchmark](/bench/ffi/README.md);
- keeps the C ABI's bulk and zero-copy payloads, with no per-byte loop on either side;
- reports its generated size, raw and gzipped, against the fork's (Python 653 KiB, Kotlin
  914 KiB, Swift 684 KiB for moq-ffi);
- wraps the C ABI's futures in the language's native future. Those futures already take the
  result in the poll that finishes them, so a cancel never loses a frame, cost three crossings
  when ready at once, and reuse a repeated waker. The native future must latch a wake that
  lands between `PENDING` and parking, prove no loss on cancel in a test that forces that
  window, and pick a semantic for concurrent pollers. A hot read still makes a future handle
  per read; measure it before adding a per-handle poll. Decided with the maintainer on
  2026-10-27: async lands with each generator, not as its own quest.
- drives each of moq's `Driver`s from the language's own event loop. Decided with the
  maintainer on 2026-10-27: the C ABI exports the poll, with no generated thread and no
  moq-tokio helpers, which would tie bindings to tokio. A binding holds one `kio::Waiter`
  per driver, made from its waker, since what a poll registers lives only as long as its
  waiter; it polls on each wake and at the deadline the poll returns, and frees the driver
  to cancel it. Decided with the maintainer on 2026-10-27, for Go: a cancel ends only that
  read and leaves the stream usable, as in Rust, since a cancelled C ABI future loses
  nothing; moq's `go/wrapper/doc.go`, which ends the stream, changes with the generator.
  Each driver runs on a goroutine that lives as long as its object, and a call's `ctx`
  bounds only that call.

Decided with the maintainer on 2026-11-01: the Swift, Python, Go, and Dart generators are
independent and unranked, and each generates the crate's own API in its language's idiom, as
Kotlin does. Only Python skips the C ABI, since a CPython extension calls about 40 times faster
than `ctypes`; the others call C natively. moq's packages switch to them in moq's own
[quest](https://github.com/moq-dev/moq/blob/main/quest/m2/uniffi-fork.md).

## Required

- [moq-clock's C names once it moves into kio](/quest/m1/ffi-export/crates.md) - `moq_now` and `moq_origin` become the clock's, after moq moves it into `kio::clock`
- [Sessions on sans-IO QUIC](/quest/m1/ffi-export/session-quic.md) - bindings move UDP datagrams and drive the session's poll themselves, over moq's runtime-free QUIC driver once it lands
- [C++ bindings on MSVC](/quest/m1/ffi-export/cpp.md) - the nightly's MSVC job passes the C++ checks
- [Waiting: moq-tokio within the recursion limit](/quest/m1/ffi-export/recursion-limit.md) - the exporter's moq pin moves past moq-dev/moq#7032 once moq-tokio's `Send` check fits the limit
- [Dart: calls into a held implementation while the isolate waits](/quest/m1/ffi-export/dart-threads.md) - an implementation Rust kept and calls from another thread runs, or fails loud, while the isolate waits in a call
- [Dart: cheaper ready futures](/quest/m1/ffi-export/dart-futures.md) - a read ready at its first poll costs no more than in Python or Go
- [Go: cheaper lent Bytes](/quest/m1/ffi-export/go-bytes.md) - a lent buffer costs no more than a 1 KiB copy, so Go's 2 KiB copy threshold can fall or go
- [Go: lent bytes freed before native memory piles up](/quest/m1/ffi-export/go-native-memory.md) - reading many large `Bytes` holds a bounded backlog of Rust's memory even when the GC rarely runs, as Kotlin's does
