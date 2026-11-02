# [L] Match cgo on the codec

## Goal

Every `just go bench` workload, translated, runs within 2x of the same crate called
through cgo, and the differential tests still pass. If it cannot, the translator is
dropped, as [the epic](/quest/m4/go/README.md) records.

## Plan

The spike measured the translation 2 to 113 times slower, decoding a 2-byte QUIC varint in
143 to 185 ns where cgo takes 1.0 to 1.6 ns. Neither the heap nor the collector explains
it:

- **No allocation in the loop:** a decode of 1,024 varints makes 2 allocations, 24 bytes,
  to cgo's 1. `go build -gcflags=-m=2` finds nothing on the decode path escaping but the
  constant `"unreachable"` panic strings, which are static, and the output's `append`.
- **No collector:** `mallocgc`, the mark workers, and write barriers take 2% of the
  profile.

The time is the translated code moving memory. In the decode path's own time, 58% is
copies of large structs and 23% is zeroing stack slots, leaving 19% for the work and the
calls:

- **An enum holds every variant's fields** side by side, where Rust overlaps them:
  `DecodeError` is 96 bytes and `Result<(u64, &[u8]), DecodeError>` 136. Each
  `Option`, `Result`, and `ControlFlow` on the path carries a `DecodeError`, and each is
  copied in and out of every call by value.
- **Every MIR local is a Go variable** declared at the top. Go zeroes each one that
  holds a pointer at entry, so `read_quic` reserves a 1,488-byte frame. 240 of its 980
  instructions zero 16 bytes each. A varint passes about 3.7 KB of frames.
- **Blocks are labels and `goto`**, which Go's inliner costs past its budget: a
  one-line `Try::branch` costs 81 against 80, so each `?` is a call. PGO recovers 0 to
  15%.

Folding each temporary assigned once and read once into its reader, as rust2-ts does,
cut decoding's time 35 to 55% and left encoding about where it was: every decode is
still 17 to 108 times cgo, encodes 2 to 6 times, and groups 0.4 to 2.9 times.

What is left is enums crossing calls through memory. Go keeps a local of over four words
in memory and returns a result of over nine on the stack, so each `Option`, `Result`, and
`ControlFlow` a call returns is stored to its caller's frame, zeroed there first when it
holds a pointer, and copied out. Hand edits of the translated 2-byte QUIC varint decode,
where cgo takes 1.0 to 1.2 ns, measure each lever, as means of three runs on a shared host:

| The translation, with | ns |
| --- | --- |
| folding, as it is now | 88 |
| `Result::Err` behind a pointer | 59, but decoding in 3-byte reads is 2.4 times slower, as each `?` allocates the error anew |
| a non-`Copy` enum's variant of over two words behind a pointer, `DecodeError` 96 to 56 bytes | 88 |
| structured control flow, so Go inlines `?`'s helpers | 73 |
| that, and `DecodeError` 40 bytes | 67 |
| each `Result` returned as Go results, its error by pointer, and the helpers inlined | 6.7, and 4.3 for 1 byte |

So laying out variants and structuring control flow, this plan's next steps, leave
every decode past 10 times cgo. Splitting an enum into its parts, Go variables for a
local and Go results for a return, as rust2-ts's `Slotted` hands back a `Result`'s parts,
reaches about what idiomatic Go does: a loop parsing as quic-go's `quicvarint.Parse`
does decodes the same varints in 3.6 to 9.7 ns, itself 3 to 8 times cgo. Within 2x of
cgo on the varint workloads is past what hand-written Go reaches.

Open, for the maintainer:

1. **Change the bar and split enums (recommended):** within 2x of idiomatic Go, or of
   cgo on calls a Go user makes one at a time, where cgo pays its crossing each time.
   Then split enums into their parts, on control flow structured by rust2-ts's `cfg`
   (`ts/src/lower/mir/cfg.rs`) made generic over its statements in a crate both share.
2. **Keep the bar, and drop the translator,** as the epic records: delete `go/` and the
   epic, keeping these measurements in the PR that does.
3. **Keep the bar and split enums anyway,** expecting 3 to 5 times cgo at best on varints.

Public API: none.
