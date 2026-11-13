# [M] An API lock that fails the export on a break

## Goal

`rust2-ffi --baseline <api.json>` fails the export when the crate's exported API breaks
the one the file records: an exported name that now means another item, an item removed,
or a signature, field, or variant changed. Each failure has the span of the item and says
what changed, once for every language rather than once per binding. moq's release check
runs it in place of the half of its
[`api-diff`](https://github.com/moq-dev/moq/blob/main/sh/api-diff/ffi.sh) that reads
UniFFI metadata, which goes when moq switches.

## Plan

Decided with the maintainer on 2026-11-12, in the quest audit: ✅ a child of the exporter
epic, so moq's first pinned release already has a baseline.

The hole today: a C name depends on every item exported. Among names that clash, the
first by kind and Rust path keeps the name and each later one takes a trailing `_`
(`ffi/src/abi.rs`'s `rename`). So adding `audio::encoder_finish` beside an existing
`audio_encoder::finish` gives the new function `moq_audio_encoder_finish` and renames the
old one, as `ffi/README.md` describes. With equal signatures a C caller compiles and calls
the wrong function, and the contract hash only catches a header and library from
different builds. Stable mangling was rejected when the naming rule was decided
([#599](https://github.com/kixel-ai/rust2/pull/599)); this catches the break instead.

- The baseline covers every target the binding serves. `--api` writes one target's API
  and refuses `--with` or a second `--target` (`ffi/src/cli.rs`), so a host-written file
  would miss a break in an item only Android or only unix has, such as
  `tokio::tls::init_android`. The export that merges the staged targets writes the merged
  API as the baseline, in the same format, and `--baseline` compares against the merge.
- The consumer commits it beside its lockfile. It is a lockfile, not config: the exporter
  reads it only to compare, and the export is the same with or without it. Record that in
  `ffi/README.md`.
- An addition passes. A rebound name, a removal, and a changed signature fail, each
  listed, and the consumer accepts a break by writing the file again.
- Not planned: compiling the last release's generated glue against the new facade as a
  witness. It would catch a break the two files' diff cannot name, but old glue may fail
  on the runtime's internals instead; try it only if the diff misses a real break.

Prove it on the api fixture with a pair of edits per kind of break, one of them in an
item only a staged target has, and by replaying two moq commits a known break apart.

Public API: the `--baseline` flag and its errors, and the merged API file an export writes.
