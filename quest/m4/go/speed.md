# [L] Match cgo on the crate

## Goal

The translated crate, on `just go bench`'s whole-crate workloads (`stream/`: a group's
stream encoded and decoded end to end, header, frames, and payloads, as moq's consumer
runs it), runs within 2x of the same crate called through cgo, and the differential
tests still pass. If it cannot, the translator is dropped, as
[the epic](/quest/m4/go/README.md) records. The per-call workloads, varints and frame
headers, are diagnostics, not the bar.

## Plan

Decided with the maintainer on 2026-11-02, settling
[moq-dev/rust2#428](https://github.com/moq-dev/rust2/pull/428): the 2x bar is for the
whole crate on a realistic workload, not an encode or decode micro benchmark. Even
hand-written Go, a loop parsing as quic-go's `quicvarint.Parse` does, decodes varints
3 to 8 times slower than cgo, so the old per-call bar measured Go against Rust rather
than the translation.

Folding each temporary assigned once and read once into its reader, as rust2-ts does,
cut decoding's time 35 to 55%. Growing a `Vec` as Rust's `RawVec` does, to at least
twice its capacity where Go's `append` grows a slice past 256 elements by about 1.25x,
cut a video stream's time 37 to 56%. Structuring control flow as rust2-ts does, through the `cfg` both now share
(`driver/src/cfg.rs`), moved none of these past noise. Splitting enums into their parts
cut decoding's time by more than half: an enum of over four words, which Go keeps in
memory, is its tag and its variants' fields, each a Go variable, parameter, and result,
and a field of over four words goes behind a pointer, which a function returning it
writes only for that variant, so `?` hands an error on in its caller's storage. Each
variable is declared where it is used, so Go zeroes it only on a path that reaches it.
Growing through the runtime's `growslice`, which leaves a pointer-free buffer's new
storage unzeroed for the append that writes it, cut a video stream's time 27 to 45% and
left audio's within noise.

What remains is the **allocator**. The translated codec is now a small part of each
stream's profile; most of the rest is growing the output: allocating each larger
buffer, copying into it, and the collector reclaiming the ones it leaves behind, along
with the page faults of memory the scavenger returned to the kernel. Rust's `realloc`
reuses what the old buffer frees. Recycling a grown-out buffer into the next grow, a
free list of power-of-two byte buffers, brings every stream within 2x, but no
translation does it soundly yet: Go keeps an array an `append` moved off untouched, so a
Go caller may still read one through another slice, such as a struct's field read
before a method that grows it, where Rust's borrow checker rules that out. Ratios of
wall time to cgo's, medians of six runs alternating the three, on a host at load 17
to 50 of 32 cores:

| Workload | Go before | Go | Recycling | cgo |
| --- | --- | --- | --- | --- |
| `stream/video/lite-06/encode` | 3.0x | 2.1x | 1.6x | 111..139 µs |
| `stream/video/lite-06/decode` | 2.9x | 2.2x | 1.9x | 100..130 µs |
| `stream/audio/lite-06/encode` | 3.0x | 3.3x | 1.9x | 2.4..3.1 µs |
| `stream/audio/lite-06/decode` | 2.6x | 2.7x | 1.7x | 2.7..2.8 µs |
| `stream/video/lite-07/encode` | 2.7x | 1.9x | 1.6x | 121..158 µs |
| `stream/video/lite-07/decode` | 3.6x | 2.4x | 1.6x | 117..135 µs |
| `stream/audio/lite-07/encode` | 2.9x | 2.8x | 1.8x | 2.7..3.5 µs |
| `stream/audio/lite-07/decode` | 3.4x | 2.5x | 1.8x | 2.6..3.7 µs |

A video group is 60 frames, 577 KB; an audio group is 50 frames of 160 bytes. Measure
each change on these, comparing `cpu-ns/item` as well as wall time on a loaded host,
since it counts the collector's workers. The per-call diagnostics: a 2-byte QUIC varint
decodes in 8.2 ns, where cgo takes 1.0, and an audio frame header in 25 ns, where cgo
takes 8.6.

Public API: none.

## Waiting

- Maintainer: whether to recycle a `Vec`'s grown-out storage, the one lever measured to
  bring every stream within 2x. (1) Plan sound recycling as its own quest: the runtime
  recycles only storage it allocated that never reached Go code outside the
  translation, tracked where a public function takes or returns a value holding a
  `Vec`; the tracking eats into a 1.6 to 1.9x margin. (2) Recycle every grown-out
  buffer and document that a Go caller keeps no other slice of a `Vec` it hands the
  translation or reads from it, which corrupts data silently when broken. (3) Drop the
  translator, as [the epic](/quest/m4/go/README.md) records for a missed bar.
  Recommended: (1), holding its result to the same bar, since recycling is the only
  route measured to meet it and (2) breaks the guarantee Go code relies on after an
  `append`. An agent cannot pick, since each option sets the epic's course.
