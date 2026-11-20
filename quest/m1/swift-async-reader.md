# [S] Swift streams of noncopyable objects

## Goal

A stream whose `next` yields a noncopyable object in Swift, such as moq's
`announce::Consumer` of `AnnounceEvent`, gets a Swift adapter as a copyable item's does,
instead of keeping `next` alone: 10 of moq's 17 streams, among them
`announce::Consumer`, `broadcast::Tracks`, `origin::Routes`, and the catalog consumers.

## Plan

Decided with the maintainer on 2026-11-19, in the pick of [#904](https://github.com/kixel-ai/rust2/pull/904): ✅ kept, waiting on a Swift release whose async iteration accepts noncopyable elements (rejected: boxing items as a workaround; dropping it).

Found by the stream adapters' quest: Swift's `AsyncSequence` requires a copyable
`Element` ([SE-0298](https://github.com/swiftlang/swift-evolution/blob/main/proposals/0298-asyncsequence.md)),
and swiftc 6.4 refuses a noncopyable one, so those streams keep `next` alone, as
`ffi/README.md`'s Swift section says. The noncopyable-ready protocol is
swift-async-algorithms' draft `AsyncReader`, whose `ReadElement` is `~Copyable`
([AsyncStreaming](https://github.com/apple/swift-async-algorithms/blob/main/Sources/AsyncStreaming/NNNN-async-streaming.md)).

Recommended: adopt whichever ships first of a generalized `AsyncSequence` in the
standard library or a released `AsyncReader`, conforming the generated `<T>Stream` to it
beside `AsyncSequence`. Rejected: yielding each item in a copyable box with a `take()`,
lossless but no Swift idiom, which every caller would unwrap.

Public API: added conformances in Swift.

## Waiting

- swiftlang/swift: a Swift or swift-async-algorithms release whose async iteration takes a
  `~Copyable` element; check by compiling a `for try await` over a noncopyable element
  with the dev shell's swiftc after each Swift or swift-async-algorithms bump
