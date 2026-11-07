# [M] Bench TPDE on moq

## Goal

The [build-speed benchmark](/bench/build/README.md) gains a `tpde` variant, on
the nightly that ships `-Zllvm-tpde`, against `nightly` in the same job. Its
report says how much TPDE, a fast O0 machine-code generator for LLVM IR, cuts
moq's clean build and non-incremental hub edit, and how many codegen units fall
back to LLVM.

## Plan

Decided with the maintainer on 2026-11-07: m4.

What it targets, from the subset-compiler report
([moq-dev/rust2#615](https://github.com/moq-dev/rust2/pull/615)) at moq main:
LLVM is 21% of a clean build's rustc self-time and 28% of a non-incremental
moq-net edit's (36% with building the IR, which TPDE does not replace), almost
all of it O0 machine-code emission. The incremental edit spends 0.1% in LLVM,
so expect nothing there. Cranelift, the existing lever on the same step, cut
instructions by 16% clean and 25% on the non-incremental edit.

- Bump the nightly in `flake.nix` to one that ships the flag, then add the
  variant beside `cranelift` in [corpus.toml](/bench/build/corpus.toml), with
  the same scenarios. `nightly` moves with the bump, so the variant's change
  compares within one run.
- Recommended: `-Zllvm-tpde=try`, which falls back to LLVM per module where
  TPDE fails (inline and global assembly, SIMD, sanitizers); `=only` fails the
  build instead. Report the fallback count.
- Recommended: list it in the nightly moq job in
  [bench.yml](/.github/workflows/bench.yml) beside `cranelift`, since upstream
  plans to optimize the TPDE path after it lands.

## Waiting

- rust-lang/rust: [rust-lang/rust#164108](https://github.com/rust-lang/rust/pull/164108)
  adds `-Zllvm-tpde`. Check that it merged and that a nightly from
  `rust-bin.nightly` in `flake.nix` accepts the flag: `rustc -Z help` lists
  `llvm-tpde`, and a `-Zllvm-tpde=only` build of a hello world succeeds, since
  a shipped toolchain may build without TPDE.
