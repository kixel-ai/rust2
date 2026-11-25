# [M] A count repeats from an idle start

## Goal

moq's counted benchmarks agree exactly between a run that starts on an idle host and one on a host loaded to its core count, or the cause of any that differ is found and fixed in rust2-bench.
Done when one idle-start count of moq agrees on every benchmark with a loaded one, on a quiet host or the nightly's runner, or a PR fixes what made them differ.

## Plan

[kixel-ai/rust2#1023](https://github.com/kixel-ai/rust2/pull/1023) gave every counted process a fixed-seed stream for randomness and a deterministic clock, padded every job's arguments and environment to 1 MiB, and gave CodSpeed's crates the workspace relative to its Git repository.
On moq at [moq-dev/moq#9574](https://github.com/moq-dev/moq/pull/9574)'s head, all 1,284 counts agree across checkouts whose paths differ in length, with a bisect step's environment, and from starts at load 10 to 22 against runs at load 55, on a shared 24-core host that never went idle.
Before the padding, one run that started at load 6 disagreed with later loaded ones on 30 counts, up to 0.024% (`cmaf-interleaved-import::4tracks[1runs-1bytes-7484chunk]`, 1,236 instructions), mostly moq-mux's, with a few of moq-net's and moq-quic's under 0.001%. That is the shape of a launch that moved, which the padding now holds fixed, but no idle start has shown it gone. #1023 landed before this count rather than wait for it (coordinator, 2026-11-25, while the maintainer was away).

1. Count moq with `sh/rs/instructions.sh run` once from an idle start (load under 2) and once with the host loaded to its core count, from the same checkout.
2. If every count agrees, delete this quest. If any differs, find what the two runs' processes saw differently, starting with the movers above.
