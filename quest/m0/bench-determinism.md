# [M] A count does not move with its job's paths or environment

## Goal

rust2-bench launches every counted job so its initial stack is the same whatever the length of its checkout's path and the size of the host's environment, so a benchmark's instruction count is the same for the same code across checkouts and CI runs on hosts of one fingerprint.
Done when all of moq's counted benchmarks agree exactly between two checkouts whose paths differ in length, and between a run on `main` and one on a branch with a longer name, as a Kixel Quest bisect step runs, once on an idle host and once on a host loaded to its core count.

## Plan

[kixel-ai/rust2#1023](https://github.com/kixel-ai/rust2/pull/1023) gave every counted process a fixed-seed stream for randomness and a deterministic clock (maintainer, 2026-11-24), padded every job's arguments and environment to 1 MiB (option 1 of this quest's Waiting item, a coordinator choice of 2026-11-25 while the maintainer was away), and gave CodSpeed's crates the workspace relative to its Git repository.
That last one was found while measuring the padding: CodSpeed's crates resolve the workspace path they are given, and the different-length strings they allocate move the heap enough to switch glibc's `memcpy` between copy loops, 10.5% on `decode_clock_drain::backward/4096B[10000]`.
What it measured on moq at [moq-dev/moq#9574](https://github.com/moq-dev/moq/pull/9574)'s head, all 1,284 counted benchmarks, on a shared 24-core host:

- Two checkouts whose paths differ in length (`.tmp/moq-a` and `.tmp/moq-second-checkout`): 1,284 agree, before at 288.
- A bisect step's environment (`GITHUB_REF=refs/heads/kixel-bisect/<run>/<commit>` and 30 KB more of variables) against none: 1,284 agree.
- Runs that started at load 10 to 22 against runs at up to load 55, both checkouts at once: 1,284 agree. So do runs whose processes had 6-digit and 7-digit ids.

What remains:

- No run started on an idle host: this one never fell below load 10. Before the padding, one run that started at load 6 disagreed with later loaded ones on 30 counts, up to 0.024% (`cmaf-interleaved-import::4tracks[1runs-1bytes-7484chunk]`, 1,236 instructions), mostly moq-mux's, with a few of moq-net's and moq-quic's under 0.001%. That is the signature of a launch that moved, which the padding now holds fixed, but no idle start has shown it gone. Count moq once from an idle start and compare against a loaded run; if any count differs, find the cause.
- A CodSpeed benchmark package at its repository's root still resolves the checkout's path, since its file's name is a path from the job's directory; [the README](../../bench/runner/README.md) says so. moq has none.

How this splits from [`bench-compare-one-path`](https://github.com/kixel-ai/rust2/blob/quest-plan/bench-followups/quest/m0/bench-compare-one-path.md), planned in [kixel-ai/rust2#1010](https://github.com/kixel-ai/rust2/pull/1010): that quest counts `compare`'s base and head from one directory, which also removes any path a build compiles in. This quest fixes every job's launch, `run`'s and `compare`'s alike; that one keeps the paths compiled into the build.

## Related

- [`bench-compare-one-path`](https://github.com/kixel-ai/rust2/blob/quest-plan/bench-followups/quest/m0/bench-compare-one-path.md), planned in [kixel-ai/rust2#1010](https://github.com/kixel-ai/rust2/pull/1010) - one directory for `compare`'s two sides
