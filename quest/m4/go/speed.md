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
Where the bar stands, as medians of six runs alternating with and without the split,
lowest and highest in parentheses, in µs per group on a host at load 6 to 13 of 32
cores:

| Workload | Go before | Go | cgo | Ratio |
| --- | --- | --- | --- | --- |
| `stream/video/lite-06/encode` | 294 (267..398) | 302 (247..376) | 99 | 3.0x |
| `stream/video/lite-06/decode` | 212 (186..260) | 227 (196..344) | 94 | 2.4x |
| `stream/audio/lite-06/encode` | 7.0 (4.4..7.8) | 6.7 (5.1..8.1) | 2.3 | 3.0x |
| `stream/audio/lite-06/decode` | 17.2 (14.4..18.5) | 7.6 (6.9..8.1) | 2.4 | 3.1x |
| `stream/video/lite-07/encode` | 302 (217..359) | 340 (296..572) | 119 | 2.9x |
| `stream/video/lite-07/decode` | 317 (288..394) | 326 (232..527) | 112 | 2.9x |
| `stream/audio/lite-07/encode` | 7.0 (6.0..7.5) | 7.9 (4.8..16.5) | 2.4 | 3.3x |
| `stream/audio/lite-07/decode` | 18.4 (17.6..19.9) | 7.0 (5.7..13.9) | 2.3 | 3.1x |

A video group is 60 frames, 577 KB; an audio group is 50 frames of 160 bytes. Measure
each change on these. The per-call diagnostics moved further: a 2-byte QUIC varint
decodes in 8.2 ns, from 95, where cgo takes 1.0, and an audio frame header in 25 ns,
from 212, where cgo takes 8.6.

What remains is the **output's growth**. The translated codec is now a small part of
each stream's profile: `vecExtend`, which `Vec::extend_from_slice` lowers to, is 45 to
60% of a stream's time, of which `growslice` copying the buffer to its new storage and
zeroing the rest is 30 to 38%, and the collector's workers take most of the rest. Rust
grows its buffer by `realloc`, which for a large block remaps pages without copying or
zeroing them; the cgo route allocates Go memory once, for the result's final size.
Levers to measure, none yet tried: growing so the new storage is not zeroed and then
written again, as `slices.Grow` does before `append` copies over it; and how much of
the collector's share the benchmark's own allocation of each run's output leaves to
the translation. If neither brings the streams within 2x, the remaining gap is Go's
allocator, which no translation closes, and the translator is dropped.

Public API: none.
