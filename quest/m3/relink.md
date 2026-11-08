# [XS] Relink without rebuilding

## Goal

Upstream's relinking without rebuilding reaches the pinned nightly, and the
[build-speed benchmark](/bench/build/README.md) measures it as a nightly
variant on moq's edits.

## Plan

It is the one upstream lever the moq-net edit leaves room for. The benchmark's
attribution ([run](https://github.com/moq-dev/rust2/actions/runs/40093020939))
puts 4.5 s of that edit's 6.95 s critical path in crates rebuilt only because
moq-net changed, 0.6 s of it the final link that would remain. Cranelift and
the parallel frontend move the edit within noise; the backend is 6% of its
rustc time.

Status on 2026-11-07: neither flag is on the pinned nightly 2026-10-22. rust#156940 is
a draft that its author is splitting into smaller PRs, tracked in
[rust-lang/rust#158844](https://github.com/rust-lang/rust/issues/158844), whose steps
are all open. Cargo's early cutoff is a design discussion.

The early cutoff spike (2026-10-24) replayed moq-net's real edits through
rust#156940 and decided against carrying it: its public hash stayed unchanged
on 4 of 171 non-test edits on PR branches, and on none of 81 squash merges.
What decides whether this lever ever pays on moq:

- Every edit touching an `async fn` body changed the hash, since rmeta encodes
  the coroutine's MIR and moq-net is mostly async.
- Span positions are hashed, so a line added above a public item changes it.
  Hashing them position-independently would add 24 of the 171 edits, mostly
  plain sync body edits.
- With incremental on, an edit and its revert leave different rmeta bytes on
  the pinned nightly, and a different rust#156940 hash, so a cutoff keyed on
  either needs that fixed first.

The [semantic API hash replay](https://github.com/moq-dev/rust2/pull/62)
(2026-10-25) hashed only what a dependent can reach and observe, over the same
edits. Differential tests show it sound, and it held on 18 of 250 non-test
edits under moq's dev profile (17 of 169 on PR branches, 1 of 81 squash merges):

- Panic locations and debuginfo lines in the generic and inline bodies that
  dependents codegen move with every line added above them. Building
  dependents with `debug = 0` and `-Z location-detail=none` would make it 65.
- Edits to `async fn` bodies held on none of 84, and on 7 with positions
  ignored: generic `async fn`s' coroutines, closures, and tracing callsites
  reach dependents. Gating the `async fn` wrappers (poll-first) adds 2.
- On stock rustc the cutoff must also keep every DefIndex, which a dependent's
  rmeta encodes raw, and patch the SVH, or the next crate fails with E0460.

When the flags land, measure on the benchmark's `edit-sync`, a body edit the
hash can cut off: the `edit-hub` marker is public and appended to the crate
root, whose module span is the whole file, so it always changes the hash.
`edit-async` is the case the replay says it misses.

Public API: a benchmark variant.

## Waiting

- rust-lang/rust: a public API hash, as in [rust-lang/rust#156940](https://github.com/rust-lang/rust/pull/156940),
  reaches nightly. Check `rustc -Z help` on a nightly from `rust-bin.nightly` in `flake.nix`.
- rust-lang/cargo: an early cutoff, as in [rust-lang/cargo#17439](https://github.com/rust-lang/cargo/issues/17439),
  reaches nightly. Check `cargo -Z help`.
