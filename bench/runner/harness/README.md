# rust2-bench-harness

Benchmarks as plain functions marked `#[bench]`, which [rust2-bench](../README.md) counts in
instructions under Valgrind, each alone in its own process, in parallel, and times by wall clock,
alone or paired with a base commit's, for Kixel Quest's `metrics.json` or a local compare. A
benchmark written for it needs neither Criterion nor CodSpeed's crates.

```toml
[dev-dependencies]
rust2-bench-harness = { git = "https://github.com/kixel-ai/rust2" }

[[bench]]
name = "varint"
harness = false
```

```rust
use std::time::Duration;

use rust2_bench_harness::{Iterations, bench};

#[bench]
fn empty() -> Vec<u8> {
	Vec::new()
}

#[bench(args = [1, 300, 70000])]
fn encode(n: u64) -> Vec<u8> {
	varint::encode(n)
}

fn frames(size: usize) -> Vec<Frame> {
	(0..100).map(|_| Frame::zeroed(size)).collect()
}

#[bench(args = [100, 16384], setup = frames)]
fn write(frames: Vec<Frame>) {
	for frame in frames {
		sink(frame);
	}
}

fn relay() -> Relay {
	Relay::with_paths(10_000)
}

#[bench(setup = relay, before = Relay::touch)]
fn tick(relay: &mut Relay) -> Bytes {
	relay.tick()
}

#[bench]
fn busy(iterations: Iterations) -> Duration {
	let mut pair = Pair::connected();
	let start = pair.server.busy();
	for _ in iterations {
		pair.exchange();
	}
	pair.server.busy() - start
}

fn main() {
	rust2_bench_harness::main();
}
```

```bash
cargo bench                                          # each benchmark once, natively
nix run github:kixel-ai/rust2#bench -- run          # each one counted under Valgrind
nix run github:kixel-ai/rust2#bench -- compare main  # each one's change from main
nix run github:kixel-ai/rust2#bench -- time          # each one timed
nix run github:kixel-ai/rust2#bench -- time main     # each one timed beside main's
```

## What a benchmark is

A function, its body the only thing counted or timed. A count calls it once; a time calls it
many times, in samples of as many calls as fill a millisecond. It takes one input at most:

- With neither key, none.
- `args = <expr>`, an expression a `for` loop takes, such as an array or a `const`, declares
  one case an argument, named `[<arg>]` by the argument's `Display`. Without `setup` the
  function takes the argument itself, moved in: each call a clone of it, so it must be
  `Clone`. The expression is evaluated once, so an iterable of unstable order, such as a
  `HashSet`, still runs each case the argument it is named for; its items must own what they
  hold, or borrow only from statics.
- `setup = <path>` names a function that builds the benchmark's input before the count or the
  clock starts, taking the case's argument when there are args. The benchmark takes the input
  by `&`, `&mut`, or by value, as its parameter's type says. Borrowed, the input is built once
  and kept across every call, so a `&mut` benchmark sees what the calls before it left;
  moved in, each call gets one of its own, built just before it outside the clock, and drops
  it where the benchmark does, so only one is live at a time.

A borrowed input may take steps between calls that neither the count nor the clock sees:
`before = <path>` names a function run on it before each call, and `after = <path>` one run
after each, such as a step that undoes what the call did. Each takes the input as the benchmark
borrows it, `&` or `&mut`. A step makes the clock read around each call rather than around a
sample's, which costs a few dozen nanoseconds a call, as does an input or a return value that
needs dropping; neither steps, inputs built between calls, nor drops count against the
millisecond a sample fills, up to 50 ms a sample.

A benchmark whose time is not its calls' wall clock, such as a server's own busy time, a thread's
CPU time, or a fixture sized by how many calls it serves, times itself: its last parameter is
`Iterations`, the calls it is to make, which a `for` loop takes, and it returns the
`std::time::Duration` they took, after its one input if it has one. A count calls it for one
iteration and counts all of it, untimed parts included.

What the function returns goes through `black_box` and drops after the count or the call's
clock stops, before the next call, so returning a value keeps its computation and its free out
of both, and a returned guard never blocks the next call; it must own what it holds. The argument or input goes through `black_box`, and the function is called
through a pointer the optimizer cannot see through, so it compiles as written rather than
folded into its caller. A function that is async, unsafe, generic, or a method, or that takes
more than that one input, fails to compile, saying why: an async benchmark builds its runtime
in setup, or in a `static` when its input is moved in, and blocks on its future inside it.

## Names

A benchmark is named by its file, relative to the workspace root, then its module path within
the crate, its function, and its argument, such as `rs/net/benches/varint.rs::encode[300]`
or `ffi/moq/bench/export.rs::rust::call`, as CodSpeed's crates name theirs by file, group, and
function. Two of one name fail the run, as does a name holding a control character, such as
an argument that shows a line break: rust2-bench reads names a line each. Cargo names a member's file relative to the
workspace root; a file it names by an absolute path, as for a target whose path lies outside
its package, fails to compile, since a name holding a host's path would never match another
host's: remap it with `--remap-path-prefix`, as [`just ffi bench-build`](../../../ffi/justfile)
does.

## How it runs

`rust2_bench_harness::main()` runs every benchmark once, sorted by name, setting up each case
and bracketing its call with Callgrind's client requests, then prints `rust2-bench ran <name>`
after it. Natively the requests are instructions that do nothing, so `cargo bench` and
`cargo test --benches` run each case once as a check. With `RUST2_BENCH` set to a name, it
runs that benchmark alone, which rust2-bench sets on each job. It takes no arguments but
Cargo's own `--bench` and `--quiet`; rust2-bench's `--benchmarks` chooses benchmarks.

With `RUST2_BENCH_TIME=1` as well, it sets that benchmark up and then times it, a sample for
each iteration count a line of stdin holds, printing `rust2-bench sample <nanoseconds>` on a
line of its own after each, until stdin ends. rust2-bench drives it so, and reads a benchmark's
own output on stdout around its samples.

Under Valgrind with `--instr-atstart=no`, as rust2-bench runs it, each case's count is
everything its call executes on every thread, between `ZERO_STATS` and
`START_INSTRUMENTATION` before it and `STOP_INSTRUMENTATION` and `DUMP_STATS_AT <name>` after:
the body, its call, and the requests' own few dozen instructions at most. A program whose benchmarks rust2-bench runs as a
`--program` runs every case in one process, each dumped under its own name.

The requests are x86_64's and aarch64's; elsewhere the harness requests nothing, and
rust2-bench fails a job that measured no benchmark.

With `RUST2_BENCH_PERF` set to a core PMU's perf type, or 0 on a host with one, it also counts
each case's user-space instructions with Linux perf counters, on every thread, and prints
`rust2-bench counted <instructions> <alone|threads> <name>` before `rust2-bench ran <name>`:
`threads` when another thread ran during the count. `rust2-bench run --perf` sets it, on that
PMU's cores, as its [README](../README.md#perf-counters) says.
