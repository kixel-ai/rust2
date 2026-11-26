# [M] Research spike: faster deterministic simulation (QEMU TCG / DynamoRIO vs Valgrind)

## Goal

A measured spike determines whether a QEMU TCG plugin (`qemu-user` + `libcache` + custom start/stop trigger) or DynamoRIO (`drcachesim` + custom client) can replace stock Valgrind Callgrind in `swole sim` with 3–8× faster runtime while preserving 0% run-to-run variance, cache/branch cycle simulation, and per-function call-graph attribution, ending in a go/no-go verdict.

## Plan

Decided 2026-11-25 (maintainer, `/quest-plan` in rust2):

- ✅ Plan as a deferred m3 research spike rather than blocking `swole sim`'s v1 on custom binary-instrumentation plugins (rejected: building a custom QEMU/DynamoRIO backend before shipping Valgrind `sim`; omitting the spike entirely). Valgrind Callgrind with `--cache-sim=yes --branch-sim=yes` is already pinned and working, at 50–100× native cost; this spike tests whether QEMU TCG or DynamoRIO beats it enough to justify maintaining a custom plugin and trigger mechanism. Carried into `kixel-ai/swole`'s quest tree when [Move rust2-bench to kixel-ai/swole](/quest/m1/swole-repo.md) executes.

Recommendations for the implementer:

- Prototype the minimum viable bracketed start/stop trigger + instruction/cache counter + per-function attribution on `swole`'s fixture suite (and a representative slice of `moq`'s benches) for QEMU TCG and DynamoRIO.
- Compare against stock Valgrind Callgrind (`sim-cycles`) on: wall-clock speedup, exact determinism across checkouts/environments/loads, cross-arch support (`aarch64` on `x86_64` for QEMU), branch-mispredict fidelity, call-graph attribution quality, Nix packaging simplicity, and ongoing maintenance burden.
- Record the measured numbers and go/no-go verdict in `swole`'s docs (and if go, plan the implementation quest as a follow-up).

## Required

- [Deterministic simulated cycles in sim (--cache-sim=yes --branch-sim=yes)](/quest/m2/swole/sim-cycles.md) - establishes the Valgrind cache/branch cycle baseline to beat

## Hosts

- linux - running Valgrind, QEMU TCG, and DynamoRIO head-to-head
