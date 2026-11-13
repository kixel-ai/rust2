# Build-speed benchmark

`just bench` measures Rust build time on the pinned projects and build configurations in [corpus.toml](corpus.toml).
CI ([bench.yml](../../.github/workflows/bench.yml)) runs it on manual dispatch and on a PR that changes this harness, its workflow, the `build-bench` shell, or the `bench-ra-jit` package, since every input is pinned and nothing else can change its result.
A later push to that PR reruns it only if it changes one of those since the PR's last benched head, so a docs-only push costs no bench time.
Every build-speed change here states its effect with its numbers.

```bash
just bench                                    # everything; takes hours
just bench --variant baseline --runs 1        # a quick smoke test
just bench --corpus moq --variant baseline --variant mold
```

The script enters the `build-bench` dev shell itself, which holds only what the benchmark runs: the stable toolchain, a pinned nightly, hyperfine, mold, wild, measureme, and the script's tools.
Results land in `target/bench/results/`, newest at `target/bench/results/latest/report.md`.
Checkouts and builds live in `/var/tmp/rust2-bench-<user>/` with their own `CARGO_HOME`, since cargo reads every `.cargo/config.toml` above the directory it runs in and a user-level one (such as `build.incremental = false`) would change what is measured.
The run also ignores the caller's `CARGO*`, `RUSTC*`, `RUSTDOC*`, `RUSTUP*`, and `RUSTFLAGS` variables, and `CI` and `TF_BUILD`, which make newer cargo skip incremental compilation.

## Corpus

- **moq** builds `moq-relay`, which pulls in moq-net and the QUIC stack. Its crypto is on rind, which is pure Rust, so it compiles no C: the run fails if any crate in its build gains a `links` key or pulls in cc or cmake, while pure-Rust build scripts stay allowed.
The corpus pins a git revision.
Bump a pin in its own change, since numbers on either side of it do not compare.

## Scenarios

- `check`: a clean `cargo check`.
- `build`: a clean debug `cargo build`, dependencies included.
- `edit-leaf`: rebuild after a body edit in a crate nothing else in the build depends on.
- `edit-hub`: the same edit in a crate most of the build depends on (moq-net in moq).
- `edit-sync`: rebuild after an edit to the body of a sync function the build calls, in a module of the hub (moq: `goaway::Producer::send`, which moq-relay calls to drain a session).
- `edit-async`: the same in an `async fn` (moq: `Session::closed`, which moq-relay awaits, a wrapper over the sync `poll_closed`).
- `test-build`: a clean build of the corpus's tests with `cargo nextest run --no-run`, as CI builds them with a cold cache.
- `test-run`: run those tests, already built.
- `test-edit`: rebuild the tests after the hub edit, as CI does when its compilation cache serves every dependency.

The leaf and hub edits rewrite a marker function appended to the crate root, which nothing calls; the body edits rewrite a statement at the top of the body.
Either way every run changes code that reaches codegen and moves no other line.
The body edits run only on a corpus that names them and only for the variants that list them: cargo rebuilds the same crates for them as for `edit-hub`, so they set apart only the levers that skip work inside an edit, such as ra-jit and relinking without rebuilding.
The test scenarios run only on a corpus that names its tests (moq: every workspace crate `moq-relay` builds) and only for the variants that list them, since a test build costs more than `build`.
They build without incremental compilation, as CI does.
A test run runs every test even when some fail, and the report lists how many failed, since a failed test can end sooner than a passing one.
Before the timed runs, each variant does one untimed clean build that warms the disk cache and logs any failure.
Each timed run starts after a `sync`: on CI's 16 GiB runners, writing back the dirty pages that earlier runs left made the edits it overlapped about 10% slower.
An edit can still read its target directory back from disk when it follows every other variant's builds, since four of moq's (2.9 GB each) fill what those runners cache; each variant gets at most two such runs, and the trimmed mean drops the slowest.
Over 21 CI runs since the sync, the controls' change had an RMS of 1.3% on `edit-leaf` and 1.4% on `edit-hub`, against 1.9% on `check` and 1.6% on `build`.

## Variants

A variant is a toolchain plus rustflags and cargo config.
Each is compared against its `base`, run in the same job: the levers on stable against `baseline`, the nightly ones against `nightly`.
The `control` variants repeat their base, so their change is the run's noise floor: an A/A control.
Timed runs interleave the variants and rotate which goes first each round, so drift over a long job spreads across all of them.
To measure a new build-speed change, add a variant whose base is the configuration it improves on.

The `ra-jit` variant builds with `cargo ra-jit` ([ra-jit](https://github.com/indexable-inc/ra-jit), pinned in `flake.nix` and compiled from source on a run that selects it), a daemon that compiles only the functions an edit changed and splices them into the program cargo linked, or hands the build back to cargo.
ra-jit works with one rustc only, so its base, `ra-jit-baseline`, is plain cargo on that toolchain.
Both run on the moq corpus only, since ra-jit needs a program to load, and skip `check`, which `cargo ra-jit` fails on moq.
The daemon keeps working after a build returns, so each ra-jit run waits for it to go idle before and after, and none of its work overlaps another run.
Its CPU time adds the daemon's, its peak RSS adds what the daemon holds, and its instructions read `n/a`; the report also says how many of its runs the daemon built, and why cargo built the rest.
The leaf and hub markers are functions nothing calls: rustc recompiles their crate and every crate that depends on it, while ra-jit, when it builds the edit itself, compiles nothing, so its times there are a lower bound.
The body edits are functions the program runs, which ra-jit compiles.

ra-jit bugs seen on moq, not reported upstream:

- Async body edits in moq-net go to cargo (every `edit-async` run): "`parsed_values_given` is not compiled by ra-jit and rustc's copy is from before the edit: an instance of it does not compile", a generic of usage-argv's. The daemon then loads the workspace again: on CI such an edit took 50.6 s and 196 CPU-s, against plain cargo's 10.5 s and 14.5 CPU-s ([run](https://github.com/kixel-ai/rust2/actions/runs/42351623273)).
- At the previous pin (`f36c848`), `cargo ra-jit check` with no build loaded read every crate of the workspace, took minutes and GBs, and failed on errors rustc does not report (`unresolved macro moq_clock::test` in moq-net's integration tests).
- On a library-only build, `cargo ra-jit build` returns after `cargo clean` without building anything.

The report shows wall time (trimmed mean, and mean ± standard deviation), CPU time and its change against the base, retired instructions, and peak RSS, all from hyperfine.
A busy host adds an occasional slow run, which on an edit of a few seconds can move a mean by over 10%, so the change against a base compares trimmed means, which drop each variant's fastest and slowest run; CPU time and instructions are trimmed means too.
Instructions barely move with machine load, so they are the signal on a busy machine, where wall time is only a smoke test; they read `n/a` where the host exposes no performance counters.
No threshold is set: the numbers inform a decision, they do not gate one.

## Nix builds for moq

Decided on 2026-10-26: moq explores crate2nix, and keeps its crane layers as they are.

A since-deleted `just bench-nix` measured moq's Nix release builds (`moq-relay`, then `moq-cli`) in CPU time, built each way they could share compilations:

- `crane`: moq's flake as it is, a dependency layer and a workspace layer per package.
- `crane-mbx`: the same packages with no dependency layer, every compilation going through mbx on a store mounted into the sandbox.
- `crate2nix`: a derivation per crate, from `crate2nix generate`, so the Nix store itself is the cache.
- `cargo-mbx`: cargo through mbx outside Nix, with a fresh target directory per build.

Its scenarios ran in order on a private store, so only the first was cold: `clean` builds the first package from nothing, `second-package` the second, `edit-hub` the first after the moq-net body edit, and `lock-bump` the first after `cargo update` of one dependency.

Measured on 2026-10-25 and 2026-10-26 on a busy 32-CPU host, in billions of instructions, at moq [`011402e`](https://github.com/moq-dev/moq/commit/011402e005f6c8efeff07c46535fa2cfa95f021e):

| scenario | crane | crane-mbx | crate2nix | cargo-mbx |
| --- | ---: | ---: | ---: | ---: |
| `clean` | 3681 | 3377 | 4672 | 3378 |
| `second-package` | 5926 | 3945 | 6999 | 3924 |
| `edit-hub` | 1365 | 1351 | 1509 | 1276 |
| `lock-bump` | 3681 | 1123 | 1255 | 1113 |
| total, evaluation included | 14756 | 9894 | 14538 | 9692 |

- A lockfile bump costs crane a full dependency rebuild; the per-crate caches rebuild hyper and the 12 crates above it, about a third of crane's cost, most of it moq-relay's fat-LTO link.
- moq-cli shares 285 of its 548 compilations with moq-relay; the rest are crates or feature sets moq-relay does not build. crane shares none, and its dependency layer runs a `cargo check` of every dependency before building them.
- crate2nix costs 27% more than crane on a clean build and 18% more on the second package. Cargo compiles a dependency that only fat LTO reads to bitcode alone, and build scripts and proc macros at opt-level 0; buildRustCrate compiles every crate to machine code and bitcode at opt-level 3.

## Cranelift for moq's CI

Recommended on 2026-10-25: moq's CI keeps building its tests with LLVM, since 42 of its 5418 tests fail under Cranelift.
At the pin [#646](https://github.com/kixel-ai/rust2/pull/646) moved to on 2026-11-07, 58 of 6379 fail: 27 that catch a panic and 31 on the intrinsic ([rustc_codegen_cranelift#1712](https://github.com/rust-lang/rustc_codegen_cranelift/issues/1712)). The numbers below are the 2026-10-25 run's. No retrial is planned once upstream fixes both: the saving below is the ceiling, and unwinding can only lower it.

- 23 catch a panic, which the Cranelift backend that ships with nightly cannot unwind: it builds every panic as an abort (unwinding is a flag on cg_clif's own build, off by default: [rustc_codegen_cranelift#1567](https://github.com/rust-lang/rustc_codegen_cranelift/issues/1567)).
- 19 abort on an x86 SIMD intrinsic it does not implement, `llvm.x86.sse2.pmulhu.w` ([rustc_codegen_cranelift#1712](https://github.com/rust-lang/rustc_codegen_cranelift/issues/1712), under the SIMD tracker [#171](https://github.com/rust-lang/rustc_codegen_cranelift/issues/171)). moq's CI runs on aarch64, whose intrinsics this x86 benchmark does not try.

The `cranelift` variant's `build` numbers are for that abort build too.
Since moq moved its crypto onto rind, the variant builds rind with LLVM: its `naked_asm!` takes `sym` operands, which Cranelift cannot compile yet ([rustc_codegen_cranelift#1620](https://github.com/rust-lang/rustc_codegen_cranelift/pull/1620)).

Even passing, it would save little where CI spends its time.
CPU seconds from the [PR run](https://github.com/kixel-ai/rust2/actions/runs/40235946635) (4 CPUs; an A/A control on that run stayed within 3%):

| CI build | nightly | cranelift | change |
| --- | ---: | ---: | ---: |
| cold cache: `test-build` + `test-run` | 475 + 170 = 645 | 366 + 203 = 569 | -12% |
| warm cache: `test-edit` + `test-run` | 219 + 170 = 389 | 169 + 203 = 372 | -4% |

Cranelift builds the tests 23% cheaper but runs them 19% dearer, and the 42 failing tests abort early, so its run cost is a floor.
Wall time falls 8% either way.
moq's CI is usually warm: its compilation cache serves every dependency, and switching backends would miss that cache once.

## Attribution

After the timed variants, one nightly build per corpus (clean, then each edit) records cargo's build analysis log with section timings and rustc's self-profile.
The report splits the unit-seconds of each step by kind of unit (build script compile and run, proc macro, dependency, workspace lib, binary) and by compiler section (frontend, codegen, link), and adds macro and proc-macro expansion time and the costliest queries.
Each edit step also lists its rebuilt units: whether a source file changed or only a dependency, the wall seconds each held the critical path, and rustc's self-time by stage.
Self-profiling slows the build a little, so read this section for shares, not absolute times.

The stages map onto upstream's levers:

- **expand & resolve**: parsing, macro expansion, name resolution, and HIR lowering, which rerun in full for every rebuilt crate.
- **analysis**: every other query, chiefly type and borrow checking and MIR; the parallel frontend spreads it across threads.
- **metadata** and **incremental**: writing the crate's metadata, and loading and saving the incremental cache.
- **mono** and **backend**: monomorphization and partitioning into codegen units, then building and running LLVM; a faster backend (Cranelift, TPDE) shrinks only the backend.
- **link**: the rlib archive or the final link, where mold and wild apply.

Units rebuilt only for a dependency are what relinking without rebuilding would skip, so their critical-path seconds, less the final link, bound that saving.
