# Prove the exporter on more inputs

## Goal

[rust2-ffi](/ffi/README.md) is checked against inputs and call sequences no one wrote by
hand: stock crates, a wider shape sample, random protocol sequences, and every language's
own checkers. It completes when every child has merged.

## Plan

Decided with the maintainer on 2026-11-18, in the per-language audit: ✅ all four proofs
the audit proposed are kept (rejected: any subset). Existing proofs stay: differential
transcripts against native Rust in every language, seeded shape round-trips, ASan and TSan,
Go's race detector, Miri, and loom on `rt/borrow.rs` and the hold counts.

This epic changes no generated API, so it does not hold moq's pin
([the exporter epic](/quest/m1/ffi-export/README.md#plan)). Considered and dropped by the
audit: Kani, since the runtime's `unsafe` is FFI and concurrency, where Miri and loom fit
better; and mutation testing of the generators until [the shapes](/quest/m1/ffi-proofs/shapes.md)
land, since every mutant reruns eight languages.
