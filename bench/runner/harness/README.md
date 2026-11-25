# rust2-bench-harness

Benchmarks as plain functions marked `#[bench]`, which [rust2-bench](../README.md) counts in
instructions under Valgrind, each alone in its own process, in parallel, for Kixel Quest's
`metrics.json` or a local compare. A benchmark written for it needs neither Criterion nor
CodSpeed's crates.

```toml
[dev-dependencies]
rust2-bench-harness = { git = "https://github.com/kixel-ai/rust2" }

[[bench]]
name = "varint"
harness = false
```

```rust
use rust2_bench_harness::bench;

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
fn write(frames: &mut Vec<Frame>) {
	for frame in frames.drain(..) {
		sink(frame);
	}
}

fn main() {
	rust2_bench_harness::main();
}
```

```bash
cargo bench                                          # each benchmark once, natively
nix run github:kixel-ai/rust2#bench -- run          # each one counted under Valgrind
nix run github:kixel-ai/rust2#bench -- compare main  # each one's change from main
```

## What a benchmark is

A function, its body the only thing counted. It takes one parameter at most:

- With neither key, none.
- `args = <expr>`, an expression a `for` loop takes, such as an array or a `const`, declares
  one case an argument, named `[<arg>]` by the argument's `Display`. Without `setup` the
  function takes the argument itself, moved in. The expression is evaluated once, so an
  iterable of unstable order, such as a `HashSet`, still runs each case the argument it is
  named for; its items must own what they hold, or borrow only from statics.
- `setup = <path>` names a function that builds the benchmark's input before the count
  starts, taking the case's argument when there are args. The benchmark takes the input by
  `&`, `&mut`, or by value, as its parameter's type says. Borrowed, the input drops after the
  count stops; moved in, it drops where the benchmark drops it.

What the function returns goes through `black_box` and drops after the count stops, so
returning a value keeps its computation and its free out of the count. The argument or input
goes through `black_box`, and the function is called through a pointer the optimizer cannot
see through, so it compiles as written rather than folded into its caller. A function that
is async, unsafe, generic, or a method, or that takes more than that one parameter, fails to
compile, saying why.

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

Under Valgrind with `--instr-atstart=no`, as rust2-bench runs it, each case's count is
everything its call executes on every thread, between `ZERO_STATS` and
`START_INSTRUMENTATION` before it and `STOP_INSTRUMENTATION` and `DUMP_STATS_AT <name>` after:
the body, its call, and the requests' own few dozen instructions at most. A program whose benchmarks rust2-bench runs as a
`--program` runs every case in one process, each dumped under its own name.

The requests are x86_64's and aarch64's; elsewhere the harness requests nothing, and
rust2-bench fails a job that measured no benchmark.
