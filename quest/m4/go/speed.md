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

Where the bar stands, as means of three runs alternating with a hand edit of step 1, in
µs per group on a host at load 35 to 45 of 32 cores, so absolute times are inflated:

| Workload | Go | cgo | Ratio | With step 1 | Ratio |
| --- | --- | --- | --- | --- | --- |
| `stream/video/lite-06/encode` | 4,263 | 1,259 | 3.4x | 1,229 | 1.6x |
| `stream/video/lite-06/decode` | 4,425 | 853 | 5.2x | 1,110 | 1.6x |
| `stream/audio/lite-06/encode` | 55 | 14 | 4.0x | 24 | 1.9x |
| `stream/audio/lite-06/decode` | 75 | 13 | 6.0x | 40 | 3.4x |
| `stream/video/lite-07/encode` | 3,315 | 895 | 3.7x | 1,083 | 1.6x |
| `stream/video/lite-07/decode` | 3,417 | 969 | 3.5x | 1,184 | 1.8x |
| `stream/audio/lite-07/encode` | 61 | 13 | 4.5x | 21 | 2.0x |
| `stream/audio/lite-07/decode` | 79 | 11 | 7.0x | 36 | 3.3x |

A video group is 60 frames, 577 KB; an audio group is 50 frames of 160 bytes. Measure
each step on these before the next.

1. **Grow a `Vec` as Rust does.** Half of a video stream's time is `runtime.memmove`
   in `append`, which grows a slice past 256 elements by about 1.25x where Rust's `Vec`
   doubles, so Go copies the output several times more. Translation refuses
   `Vec::capacity`, so the growth is not observable: a runtime `push` and `extend` that
   grow by `slices.Grow(s, max(n, cap(s)))` match Rust's amortized doubling.
2. **Structure control flow on a crate shared with rust2-ts.** Labels and `goto` put a
   function past Go's inlining budget: a one-line `Try::branch` costs 81 against 80, so
   each `?` is a call. Move rust2-ts's `cfg` (`ts/src/lower/mir/cfg.rs`) into a crate both
   translators use, generic over their statements, as the epic decides. rust2-ts's output
   must not change: its lang and reject snapshots stay put.
3. **Split enums into their parts:** a Go variable per part for a local, and Go results
   for a return, as rust2-ts's `Slotted` (`ts/src/lower/mir/scalar.rs`) hands back a
   `Result`'s parts. An audio stream's remaining gap is mostly its frame headers, which
   the per-call `frames/` workloads measure at 17 to 32 times cgo.

The cost step 3 removes is Go's register ABI. Go keeps a local of over four words in
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
