# [M] The generated crate's release build

## Goal

The glue crate rust2-ffi generates for moq builds in release with less CPU time, wall
time, and peak memory, as [the binding benchmark](/bench/ffi/README.md)'s Build table
shows. Its baseline, from [#784](https://github.com/kixel-ai/rust2/pull/784) on nixos
(load 17 to 28, 3 rounds, medians):

| step | wall s | CPU s | peak RSS |
|---|---|---|---|
| generated crate, release | 417.2 | 843.7 | 3,168 MiB |
| moq, release | 94.1 | 542.0 | 1,550 MiB |

The generated build adds about 300 CPU s and doubles peak memory over moq's own. The
work finds why, and cuts it without changing any generated API.

## Plan

Decided with the maintainer on 2026-11-13, in #784: ✅ keep this quest (rejected: drop it).

Recommendations, open to the implementer's evidence:

- Profile first with `cargo build --timings` and `-Zself-profile` (or `-Ztime-passes`)
  on the generated crate, so the cut targets the measured pass, not a guess. The peak
  RSS is the largest single process, so find which rustc invocation reaches 3 GiB.
- Suspect monomorphization and inlining of the glue first: count lines with
  `cargo llvm-lines`, and prefer moving generic bodies behind non-generic shims over
  changing codegen units or LTO, since profile settings belong to the consumer.
- Report the Build table before and after in the PR.
