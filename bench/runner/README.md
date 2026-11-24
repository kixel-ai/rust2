# rust2-bench

Counts each benchmark's instructions under stock Valgrind, one Valgrind process a job, in
parallel across the host's cores, and writes them as Kixel Quest's `metrics.json` or
compares a base commit with the working tree. It runs on any x86_64 Linux host: hosted CI, a
self-hosted runner, or an agent's own, with no third-party service in the loop. It replaces
CodSpeed's runner over the benchmarks as they are written for it.

```bash
nix run github:kixel-ai/rust2#bench -- run --out metrics.json   # or, without Nix:
cargo install --locked --git https://github.com/kixel-ai/rust2 rust2-bench   # Valgrind from PATH
rust2-bench run [--benchmarks <file>] [--out metrics.json] [--jobs <n>] [-- <cargo bench args>]
rust2-bench compare <base> [the same options]
rust2-bench fingerprint
```

## What it counts

In a Cargo workspace, its benchmark targets, written with CodSpeed's compatibility crates
(`codspeed-criterion-compat` 6). It builds them once with `--cfg codspeed`, as `cargo codspeed
build` does, passing any arguments after `--` to Cargo, such as `-p moq-net --bench
varint`. Cargo takes rustflags from one source and ignores the rest, so the cfg
joins `RUSTFLAGS` or `CARGO_ENCODED_RUSTFLAGS` when the environment sets them, and is
otherwise the only flags. A Cargo config that sets its own, `build.rustflags` or a
`target.*.rustflags` (or the unstable `host` config's) in a config file or one it includes,
the environment, or a `--config`,
fails the run before anything builds: set `RUSTFLAGS` to the flags the build needs instead.
Each target lists its benchmarks by running once natively, then each benchmark runs alone in its own Valgrind process, through the crates' `CODSPEED_ISOLATED_BENCHMARK`. A
target that runs no benchmark through the crates fails the run: leave it out with `--bench`.
It builds them with `cargo bench`, with itself as the targets' runner, which records how Cargo
launches each target, its directory and whole environment (`[env]`, `CARGO_MANIFEST_DIR`,
the library path), instead of running it; every listing and count launches the target the
same way. Those records live in a directory only the user can read, removed once read. A
runner a config or the environment sets, for any target, fails the run before anything
builds, as does, once built, an `[env]` that sets `GLIBC_TUNABLES`, which the fingerprint
reads from the environment, and an argument for the targets after Cargo's own `--`, which
CodSpeed's harness takes none of: `--benchmarks` chooses benchmarks. Every count, like the
fingerprint, runs the Valgrind on rust2-bench's own `PATH`, whatever `PATH` a target gets.

Elsewhere, the programs `--program` names, each a shell command whose program brackets its
benchmarks with CodSpeed's [instrument hooks](https://github.com/CodSpeedHQ/instrument-hooks)
itself, after `--build`'s command builds them. Each program is one Valgrind process, which
follows each `exec`, so a command may go through a shell or `just`.
[`just ffi count`](../../ffi/justfile) counts rust2's [binding benchmark](../ffi/README.md)
this way.

Each benchmark is the Callgrind dump its hooks name after it, between `ZERO_STATS` and
`START_INSTRUMENTATION` before it and `STOP_INSTRUMENTATION` and `DUMP_STATS_AT` after it,
with Valgrind's `--instr-atstart=no`, so nothing outside a benchmark counts. Its count is the
dump's `totals` in `Ir`, instructions executed, on every thread. The run fails on a dump with
an unknown trigger, a benchmark without `totals` or with zero instructions, two benchmarks of
one name, a job that fails, and a name `--benchmarks` lists that no benchmark has, so a
bisect step never reads a suspect it could not count as clean. `compare` fails on such a
name only when neither side has it, since a change may add or remove a benchmark. A program's benchmarks all
run even when `--benchmarks` names only some; a Cargo benchmark runs only when named.
A dump named `Metadata: ...` is the hooks' own description of their integration, never a
benchmark, so no benchmark may take a name with that prefix: it would go uncounted.

The instrument hooks' `ADD_DESC` after each benchmark, a request only CodSpeed's Valgrind
fork knows, leaves an `unknown callgrind client request` warning in the job's Valgrind log,
under `target/rust2-bench/jobs-*`, beside its profiles and output. Each run takes a directory of
its own there, so runs in one tree never read each other's profiles, and removes it when it
succeeds; a failed run keeps it and names it.

## Fingerprints

Counts compare only within one environment. Valgrind presents a CPU derived from the host's,
and a binary that picks a path by CPU feature, such as glibc's string functions or Rust's
runtime detection, counts differently on another; so can another Valgrind, and glibc's
tunables, which mask features under the same CPU. So each run fingerprints its environment:
the architecture, every CPUID leaf a probe reads under the same Valgrind, `GLIBC_TUNABLES` as
the benchmarks inherit it, and `valgrind --version`, hashed into a label such as
`x86_64-3059cd1d9e65`. The measure every count records names it, `instructions
(x86_64-3059cd1d9e65)`, so counts from two environments never share a Kixel Quest series, and
a hardware change starts a new series instead of reading as a regression. `rust2-bench
fingerprint` prints the label, then its inputs on stderr.

Valgrind 3.27.1 on x86_64 presents one fixed Intel CPU, not the host's: an AMD Ryzen 9 9950X
reads `GenuineIntel` family 6 model 0x3c, Haswell, through it. So hosts that support the
same features share a fingerprint.

The flake pins Valgrind: a nixpkgs bump that moves it fails evaluation until `flake.nix`'s
version moves too, since a new Valgrind can count differently and starts new series.

## Compare

`rust2-bench compare <base>` checks `<base>` out in a worktree kept under
`~/.cache/rust2-bench` (or `$XDG_CACHE_HOME`), outside the repository so the working tree's
`.cargo/config.toml` never configures the base's build, and kept so its build stays warm for
the next compare, counts it from the same directory there, then counts the working tree, and prints a
Markdown table of each benchmark's base, head, and change. It refuses two fingerprints. A side
with no benchmark at all, as a base before the first, counts as none. An
absolute `--manifest-path` in the repository counts the same manifest in the base's worktree,
and one outside it is refused, since both sides would count it.
Another compare of the same repository waits, on a lock beside the worktree, until this one
has counted its base.

## Output

`--out` writes one sample of the run's commit:

```json
{"samples":[{"metrics":{"benches/varint.rs::varint::encode[1]":{"instructions (x86_64-3059cd1d9e65)":{"value":12345}}}}],
 "units":{"instructions (x86_64-3059cd1d9e65)":"instructions"}}
```

A benchmark's name is the one its hooks dump, which for CodSpeed's crates is the file
relative to the repository, then its group, function, and parameter: the same name CodSpeed
gave it, so only the measure differs.
