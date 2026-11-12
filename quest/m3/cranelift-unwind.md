# [XS] Retrial Cranelift on moq's tests once they pass

## Goal

58 of moq's 6379 tests fail under the Cranelift backend
that ships with nightly, so moq's CI cannot build its tests with it:

- It builds every panic as an abort, so the 27 tests that catch one fail.
  Unwinding on Unix is a flag on cg_clif's own build, off by default over its
  compile-time cost
  ([rustc_codegen_cranelift#1567](https://github.com/rust-lang/rustc_codegen_cranelift/issues/1567)).
- It lacks the x86 intrinsic `llvm.x86.sse2.pmulhu.w`, which 31 hit
  ([rustc_codegen_cranelift#1712](https://github.com/rust-lang/rustc_codegen_cranelift/issues/1712)).

It also cannot compile rind, moq's crypto, whose `naked_asm!` takes `sym`
operands
([rustc_codegen_cranelift#1620](https://github.com/rust-lang/rustc_codegen_cranelift/pull/1620)),
so a Cranelift build of moq picks LLVM for rind by profile, as the benchmark
does.

Once `test-run` cranelift fails no tests, decide again from the test scenarios whether moq's CI builds its tests with
Cranelift, and delete this quest.

## Waiting

- rust-lang/rustc_codegen_cranelift: [#1567](https://github.com/rust-lang/rustc_codegen_cranelift/issues/1567)
  and [#1712](https://github.com/rust-lang/rustc_codegen_cranelift/issues/1712) are fixed in
  the Cranelift backend nightly ships. Both still open on 2026-11-12. Check the issues, or the
  [build benchmark](/bench/build/README.md)'s moq report after a nightly pin bump.
