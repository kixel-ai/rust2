# rust2-bench

Counts each benchmark's instructions under stock Valgrind, one Valgrind process a job, in
parallel across the host's cores, and writes them as Kixel Quest's `metrics.json` or
compares a base commit with the working tree. It runs on any x86_64 Linux host: hosted CI, a
self-hosted runner, or an agent's own, with no third-party service in the loop. It replaces
CodSpeed's runner over the benchmarks as they are written for it, and counts benchmarks
written as plain `#[bench]` functions with [its harness](harness/README.md).

```bash
nix run github:kixel-ai/rust2#bench -- run --out metrics.json   # or, without Nix:
cargo install --locked --git https://github.com/kixel-ai/rust2 rust2-bench   # Valgrind from PATH
rust2-bench run [--benchmarks <file>] [--out metrics.json] [--jobs <n>] [-- <cargo bench args>]
rust2-bench compare <base> [the same options]
rust2-bench fingerprint
```

## What it counts

In a Cargo workspace, its benchmark targets, written with [rust2-bench-harness](harness/README.md)
or with CodSpeed's compatibility crates (`codspeed-criterion-compat` 6). It builds them once
with `--cfg codspeed`, which CodSpeed's crates take, as `cargo codspeed build` does, passing any arguments after `--` to Cargo, such as `-p moq-net --bench
varint`. Cargo takes rustflags from one source and ignores the rest, so the cfg
joins `RUSTFLAGS` or `CARGO_ENCODED_RUSTFLAGS` when the environment sets them, and is
otherwise the only flags. A Cargo config that sets its own, `build.rustflags` or a
`target.*.rustflags` (or the unstable `host` config's) in a config file or one it includes,
the environment, or a `--config`,
fails the run before anything builds: set `RUSTFLAGS` to the flags the build needs instead.
Each target lists its benchmarks by running once natively, then each benchmark runs alone in
its own Valgrind process, through the harness's `RUST2_BENCH` or CodSpeed's
`CODSPEED_ISOLATED_BENCHMARK`. A target that runs no benchmark through either fails the run,
as does one that runs some through both: leave it out with `--bench`.
It builds them with `cargo bench`, with itself as the targets' runner, which records how Cargo
launches each target, its directory and whole environment (`[env]`, `CARGO_MANIFEST_DIR`,
the library path), instead of running it; every listing and count launches the target the
same way. Those records live in a directory only the user can read, removed once read. A
runner a config or the environment sets, for any target, fails the run before anything
builds, as does, once built, an `[env]` that sets `GLIBC_TUNABLES`, which the fingerprint
reads from the environment, and an argument for the targets after Cargo's own `--`, which
neither harness takes: `--benchmarks` chooses benchmarks. Every count, like the
fingerprint, runs the Valgrind on rust2-bench's own `PATH`, whatever `PATH` a target gets.

Elsewhere, the programs `--program` names, each a shell command whose program brackets its
benchmarks itself, with rust2-bench-harness or CodSpeed's
[instrument hooks](https://github.com/CodSpeedHQ/instrument-hooks), after `--build`'s command
builds them. Each program is one Valgrind process, which
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

## Randomness and the clock

A count follows everything before it in the process, down to the allocator's state, so a seed
or a clock read that differs from run to run moves it. So every process a job runs preloads a
library, [preload.rs](preload.rs), built with rust2-bench and kept under
`~/.cache/rust2-bench` (or `$XDG_CACHE_HOME`), which replaces glibc's:

- randomness (`getrandom`, `getentropy`, and `syscall(SYS_getrandom)`) with one fixed-seed
  stream, the same in every process, so std's `HashMap` seeds and the `getrandom` crate's
  bytes repeat;
- every clock (`clock_gettime`, `gettimeofday`, `time`) with one that starts at the same
  instant in every process, a day of uptime and 2026-01-01 on the wall, and advances 1 µs a
  read;
- and timed waits (`epoll_wait`, `poll`, `nanosleep`, `clock_nanosleep`, a timed condition
  variable, futex, semaphore, mutex or rwlock lock, thread join, or message queue call) with
  ones that still wait as long as they ask, on the host's clock, and move this clock by what
  they waited when they expire. A timer armed at an absolute time (`timerfd_settime` or
  `timer_settime` with its `ABSTIME` flag) aborts the process, naming the call in its Valgrind
  log, since the kernel would read the deadline on the host's clock.

Each process logs that it loaded the library to its Valgrind log, and a process that measured
a benchmark without it fails the run: a static executable never loads it, nor does one whose
environment drops `LD_PRELOAD`. A process that reaches the kernel without glibc, as Go's
runtime does, reads the host's randomness and clock for what it does so. Load still decides
whether a real timeout or I/O comes first, so a benchmark that races them can still move.

## Paths and the environment

The same binary counts differently when its process starts with a longer path or a larger
environment, since both sit on the initial stack and in the heap, and move the addresses, and
so the copy loop glibc picks, after them. So every job's arguments and environment are padded
to 1 MiB with `RUST2_BENCH_PAD_00` to `_15`, which puts its initial stack at the same place
however long its checkout's path or however large the host's environment, such as CI's
`GITHUB_REF` on another branch. A job whose own take more fails the run, as does a stack limit
under 4.25 MiB (`ulimit -s 4352`), since Linux takes a quarter of it for them. A `--program`
job pads its shell's launch: the program the shell runs keeps that environment but gets its
own arguments, so a command that expands a checkout's path, such as `"$PWD/target/probe"`,
still moves with that path's length. And CodSpeed's crates get their workspace relative to its
Git repository, from which they name each benchmark's file, so they never resolve the
checkout's path; a benchmark package at the repository's root still does, as does every
benchmark outside a repository, which CodSpeed names by its absolute path, so their counts
still move with that path's length. A path a build compiles in, such as a panic's location in a
dependency outside the workspace, is the build's to keep fixed across checkouts; `compare`
counts both its sides from one directory, so it never moves between them.

## Fingerprints

Counts compare only within one environment. Valgrind presents a CPU derived from the host's,
and a binary that picks a path by CPU feature, such as glibc's string functions or Rust's
runtime detection, counts differently on another; so can another Valgrind, and glibc's
tunables, which mask features under the same CPU; so can another preload, whose code runs
inside a count and whose seed and clock steer it. So each run fingerprints its environment:
the architecture, every CPUID leaf a probe reads under the same Valgrind, `GLIBC_TUNABLES` as
the benchmarks inherit it, `valgrind --version`, and the preload's bytes, hashed into a label such as
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

`rust2-bench compare <base>` counts `<base>` and the working tree in one worktree kept under
`~/.cache/rust2-bench` (or `$XDG_CACHE_HOME`), each from the directory there that matches where
it runs, and prints a Markdown table of each benchmark's base, head, and change. One directory
counts both sides, so a path the build compiles in, such as a panic's location, is the same in
each and moves neither count. The worktree sits outside the repository, so no
`.cargo/config.toml` but each side's own configures its build, and is kept so its build stays
warm for the next side and the next compare: the head rebuilds only what differs from the base.
It first snapshots the working tree as a commit on `HEAD`, through an index of its own, so the
caller's index and stash stay as they were: every tracked file as it is on disk, staged or not,
and every untracked one Git does not ignore. An ignored file the caller has, such as a local
`.env`, is on neither side. An ignored one a build leaves in the worktree stays for the next
side, but for a `Cargo.lock` the side's commit does not track, which is removed before each side
builds, so each resolves its own dependencies. A submodule with uncommitted changes fails the run, since the snapshot holds
only its commit. A clean working tree counts `HEAD` itself. It refuses two fingerprints. A side
with no benchmark at all, as a base before the first, counts as none. An
absolute `--manifest-path` in the repository counts the same manifest in the base's worktree,
and one outside it is refused, since both sides would count it.
A change may add or remove a package or benchmark target, so each side builds only the `-p`
(`--package`) and `--bench` selectors its own revision has, from its `cargo metadata` after its
checkout, leaving out each `--features package/feature` of a package it lacks, and prints each it
left out; a `--bench` is looked for among the packages `-p` selects, or without one, the whole
workspace. A side left with none of the packages or none of the targets selected counts none, and
its benchmarks read as added or removed. A selector neither side has fails, as does one compare
cannot resolve by name alone, such as `name@1.0` or a glob.
Another compare of the same repository waits, on a lock beside the worktree, until this one
has counted both sides.

## Output

`--out` writes one sample of the run's commit:

```json
{"samples":[{"metrics":{"benches/varint.rs::varint::encode[1]":{"instructions (x86_64-3059cd1d9e65)":{"value":12345}}}}],
 "units":{"instructions (x86_64-3059cd1d9e65)":"instructions"}}
```

A benchmark's name is the one its hooks dump, which for CodSpeed's crates is the file
relative to the repository, then its group, function, and parameter: the same name CodSpeed
gave it, so only the measure differs. The harness names its own the same way, by
[file, module, function, and argument](harness/README.md#names).
