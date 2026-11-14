# [M] Bench TPDE on moq

## Goal

The [build-speed benchmark](/bench/build/README.md) gains a `tpde` variant, on
the nightly that ships `-Zllvm-tpde`, against `nightly` in the same job. Its
report says how much TPDE, a fast O0 machine-code generator for LLVM IR, cuts
moq's clean build and non-incremental hub edit, and how many codegen units fall
back to LLVM.

## Plan

Decided with the maintainer on 2026-11-07: m4. Decided with the maintainer on 2026-11-12,
in the quest audit: ✅ m3, with upstream's other build-speed levers (rejected: m4).

What it targets, from the subset-compiler report
([kixel-ai/rust2#615](https://github.com/kixel-ai/rust2/pull/615)) at moq main:
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
- Recommended: list it in the moq job in [bench.yml](/.github/workflows/bench.yml)
  beside `cranelift`. That job runs only on dispatch or a harness change since
  [#686](https://github.com/kixel-ai/rust2/pull/686), so a later upstream gain shows only
  at the next pin bump.
- The nightly bump here is the benchmark's own (`flake.nix`'s `rust-bin.nightly`), a
  corpus change that lands apart from the variant (`bench/build/corpus.toml`).
  [Relink](/quest/m3/relink.md) needs the same bump once its flags reach nightly.

## Waiting

- rust-lang/rust: a nightly must ship the TPDE plugin (`tpde-plugin`), which
  [rust-lang/rust-project-goals#802](https://github.com/rust-lang/rust-project-goals/issues/802)
  tracks. [rust-lang/rust#164108](https://github.com/rust-lang/rust/pull/164108) merged the
  flag at 2d1164a6 on 2026-11-13, but installation does not install the plugin, on purpose,
  and it builds only with `rust.tpde = true` in `bootstrap.toml`. On 2026-11-14,
  nightly-2026-11-14 (rustc 5f4effd57) lists `llvm-tpde` in `rustc -Z help`, but its
  manifest has no TPDE component, and a hello world with `-Zllvm-tpde=only` or `=try`
  fails: "TPDE backend not found in the sysroot: failed to find the `tpde-plugin` shared
  library". `rust-bin.nightly` in `flake.nix` had no 2026-11-14 entry yet. Check: install
  the newest nightly's `rustc` and `rust-std` from `static.rust-lang.org/dist/<date>/`, and
  build a hello world with `-Zllvm-tpde=only`.
