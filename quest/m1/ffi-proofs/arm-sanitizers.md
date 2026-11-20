# [S] The nightly sanitizers run on the arm runner

## Goal

The nightly's `ffi-sanitize` job passes on `ubuntu-24.04-arm`. Today `api_asan`,
`api_tsan`, and `api_swift_tsan` fail there on their first program run, which prints
nothing. Each one passes on x86_64 Linux.

## Plan

Decided with the maintainer on 2026-11-19, in the pick of [#891](https://github.com/kixel-ai/rust2/pull/891): ✅ kept (rejected: dropping it).

Found by the checkers quest
([kixel-ai/rust2#891](https://github.com/kixel-ai/rust2/pull/891)). The job has failed on
every nightly since it was added on 2026-11-14, for example
[run 44339014570](https://github.com/kixel-ai/rust2/actions/runs/44339014570). Valgrind,
Miri, Go's race detector, and the debug CPython in the same job pass.

A failed run now names its exit status (`check` in `ffi/tests/fixtures.rs`). Read that
first. The leading suspect is the runner kernel's ASLR entropy (`vm.mmap_rnd_bits`): it
breaks the sanitizer runtimes' shadow layouts, and the common fix lowers it with `sysctl`
in the job. The other suspect is a mismatch between nixpkgs clang's compiler-rt and the
nightly rustc's instrumentation on aarch64. Fix the cause, not the symptom. Running on
x86_64 instead would hide it.

## Hosts

- linux - the failure is on aarch64 Linux; reproduce on an arm64 Linux host or the runner
