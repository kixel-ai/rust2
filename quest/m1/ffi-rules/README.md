# A new exporter rule touches fewer files

## Goal

Adding a rule to [rust2-ffi](/ffi/README.md) costs fewer files and lines than the 27 to 47
files recent rules touched. It completes when every child has merged.

## Plan

Measured by the 2026-11-18 language audit:

| PR | Rule | Files | Generator and runtime | Tests |
| --- | --- | ---: | ---: | ---: |
| [#821](https://github.com/kixel-ai/rust2/pull/821) | `IpAddr` | 34 | 18 | 11 |
| [#731](https://github.com/kixel-ai/rust2/pull/731) | `NonZero` | 27 | 10 | 12 |
| [#708](https://github.com/kixel-ai/rust2/pull/708) | `[u8; N]` | 30 | 15 | 11 |
| [#710](https://github.com/kixel-ai/rust2/pull/710) | closures, reader | 38 | 22 | 13 |
| [#844](https://github.com/kixel-ai/rust2/pull/844) | not `Send` | 39 | 17 | 12 |
| [#832](https://github.com/kixel-ai/rust2/pull/832) | lossless types | 47 | 22 | 12 |

About 8 files per rule are the hand-ported `ffi/tests/api.*`: 17.6K lines porting the
1.2K-line `examples/transcript.rs` into eight languages.

Decided with the maintainer on 2026-11-18: ✅ both cuts, the transcript as data first,
then a time-boxed walker spike on Go (rejected: the walker spike alone; the transcript
alone; neither). ✅ A new value type's rule relies on [the shape tests](/ffi/README.md#tests)
alone and adds no `tests/api.*` case, unless it needs an idiom or protocol test of its own.
