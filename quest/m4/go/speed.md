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

What remains is the **allocator**. The translated codec is now a small part of each
stream's profile; most of the rest is growing the output: allocating each larger
buffer, zeroing and copying into it, and the collector reclaiming the ones it leaves
behind, along with the page faults of memory the scavenger returned to the kernel.
Rust's `realloc` reuses what the old buffer frees and leaves spare capacity
uninitialized. Two levers do the same in Go:

- **No zeroing:** growing through the runtime's `growslice` (linked as
  [go.dev/issue/67401](https://go.dev/issue/67401) keeps open), which leaves a
  pointer-free buffer's new storage unzeroed until appends write it.
- **Recycling:** a free list of power-of-two byte buffers, each grow taking its storage
  from it and giving back the buffer it outgrew.

Neither is sound once a `Vec`'s storage reaches Go code outside the translation, where
Rust's borrow checker no longer holds. Go code may read a slice's spare capacity
through `s[:cap(s)]`, which would show stale heap contents; and Go keeps an array an
`append` moved off untouched, so a caller may still read a grown-out buffer through
another slice, such as a struct's field read before a method that grows it. Ratios of
wall time to cgo's, medians of six runs alternating the variants, on a host at load 17
to 50 of 32 cores, with both levers applied everywhere (unsound, measured in
[moq-dev/rust2#609](https://github.com/moq-dev/rust2/pull/609)):

| Workload | Go | No zeroing | Both | cgo |
| --- | --- | --- | --- | --- |
| `stream/video/lite-06/encode` | 3.0x | 2.1x | 1.6x | 111..139 µs |
| `stream/video/lite-06/decode` | 2.9x | 2.2x | 1.9x | 100..130 µs |
| `stream/audio/lite-06/encode` | 3.0x | 3.3x | 1.9x | 2.4..3.1 µs |
| `stream/audio/lite-06/decode` | 2.6x | 2.7x | 1.7x | 2.7..2.8 µs |
| `stream/video/lite-07/encode` | 2.7x | 1.9x | 1.6x | 121..158 µs |
| `stream/video/lite-07/decode` | 3.6x | 2.4x | 1.6x | 117..135 µs |
| `stream/audio/lite-07/encode` | 2.9x | 2.8x | 1.8x | 2.7..3.5 µs |
| `stream/audio/lite-07/decode` | 3.4x | 2.5x | 1.8x | 2.6..3.7 µs |

Recycling alone, zeroing each fresh buffer, measured 1.6 to 3.0x. Growing by 4x cut
fewer bytes than recycling while overshooting memory, and `GODEBUG=madvdontneed=0`
removed the page faults but is the main program's setting, not a library's; both are
dropped.

Decided with the maintainer on 2026-11-07, settling
[moq-dev/rust2#609](https://github.com/moq-dev/rust2/pull/609):

- ✅ **The translation owns a `Vec`'s storage** while it stays inside the translation:
  the runtime grows it without zeroing, through `growslice`, and recycles outgrown
  power-of-two buffers. Where storage reaches Go code outside the translation, as a
  public function's argument or result or a field of one, it is handed over safely and
  never recycled again. Rejected: skipping zeroing and recycling everywhere, with
  documented rules for Go callers, since breaking a rule leaks or corrupts data
  silently; and dropping the translator now.
- ✅ **Escape hides spare capacity by clipping:** the translation hands Go code
  `s[:len(s):len(s)]`. Rejected: zeroing `[len:cap]` lazily at escape, measuring both,
  and a wrapper buffer type in the public Go API in place of `[]byte`.
- ✅ **The bar is unchanged:** every `stream/` workload within 2x of cgo, measured with
  the ownership bookkeeping in place, else the epic's drop rule.

Open: how a grow tells storage the translation owns from storage Go code has seen. That
bookkeeping eats into the 1.6 to 1.9x margin above, so measure it on the streams.

A video group is 60 frames, 577 KB; an audio group is 50 frames of 160 bytes. Measure
each change on these, comparing `cpu-ns/item` and `faults/op` as well as wall time on a
loaded host, since CPU time counts the collector's workers. The per-call diagnostics: a
2-byte QUIC varint decodes in 8.2 ns, where cgo takes 1.0, and an audio frame header in
25 ns, where cgo takes 8.6.

Public API: none beyond clipped capacity: a slice the translation returns has no spare
capacity, so a Go caller's `append` to it copies.
