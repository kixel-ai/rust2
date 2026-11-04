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
cut decoding's time 35 to 55% and left encoding about where it was: a 2-byte QUIC varint
decodes in about 88 ns, still 17 to 108 times cgo on every decode. The profile of what is
left is led by 16-byte moves copying `DecodeError`'s fields in and out of the frame.

Growing a `Vec` as Rust's `RawVec` does, to at least twice its capacity where Go's
`append` grows a slice past 256 elements by about 1.25x, cut a video stream's time 37 to
56%. Where the bar stands, as means of six runs alternating with and without that
growth, lowest and highest in parentheses, in µs per group on a host at load 2 to 17 of
32 cores:

| Workload | Go before | Go | cgo | Ratio |
| --- | --- | --- | --- | --- |
| `stream/video/lite-06/encode` | 443 (285..639) | 272 (180..353) | 92 (49..139) | 3.0x |
| `stream/video/lite-06/decode` | 478 (298..711) | 212 (116..278) | 82 (52..133) | 2.6x |
| `stream/audio/lite-06/encode` | 9 (6..14) | 6 (4..10) | 2 (1..3) | 3.3x |
| `stream/audio/lite-06/decode` | 19 (16..25) | 17 (15..21) | 2 (2..3) | 7.6x |
| `stream/video/lite-07/encode` | 489 (270..729) | 287 (157..365) | 118 (87..155) | 2.4x |
| `stream/video/lite-07/decode` | 520 (381..708) | 325 (204..427) | 101 (70..116) | 3.2x |
| `stream/audio/lite-07/encode` | 8 (6..10) | 6 (5..8) | 2 (2..3) | 3.0x |
| `stream/audio/lite-07/decode` | 22 (19..27) | 20 (17..22) | 2 (2..3) | 8.4x |

A video group is 60 frames, 577 KB; an audio group is 50 frames of 160 bytes. Measure
each step on these before the next.

1. **Structure control flow on a crate shared with rust2-ts.** Labels and `goto` put a
   function past Go's inlining budget: a one-line `Try::branch` costs 81 against 80, so
   each `?` is a call. Move rust2-ts's `cfg` (`ts/src/lower/mir/cfg.rs`) into a crate both
   translators use, generic over their statements, as the epic decides. rust2-ts's output
   must not change: its lang and reject snapshots stay put.
2. **Split enums into their parts:** a Go variable per part for a local, and Go results
   for a return, as rust2-ts's `Slotted` (`ts/src/lower/mir/scalar.rs`) hands back a
   `Result`'s parts. An audio stream's remaining gap is mostly its frame headers, which
   the per-call `frames/` workloads measure at 17 to 32 times cgo.

The cost step 2 removes is Go's register ABI. Go keeps a local of over four words in
memory and returns a result of over nine on the stack, so each `Option`, `Result`, and
`ControlFlow` a call returns is stored to its caller's frame, zeroed there first when it
holds a pointer, and copied out. An enum holds every variant's fields side by side,
where Rust overlaps them: `DecodeError` is 96 bytes, and `Result<(u64, &[u8]),
DecodeError>` 136. Hand edits of the translated 2-byte QUIC varint decode, where cgo
takes 1.0 to 1.2 ns, measure each lever:

| The translation, with | ns |
| --- | --- |
| folding, as it is now | 88 |
| `Result::Err` behind a pointer | 59, but decoding in 3-byte reads is 2.4 times slower, as each `?` allocates the error anew |
| a non-`Copy` enum's variant of over two words behind a pointer, `DecodeError` 96 to 56 bytes | 88 |
| structured control flow, so Go inlines `?`'s helpers | 73 |
| that, and `DecodeError` 40 bytes | 67 |
| each `Result` returned as Go results, its error by pointer, and the helpers inlined | 6.7, and 4.3 for 1 byte |

Laying variants out behind pointers is no lever, so it is not a step.

Public API: none.
