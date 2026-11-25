//! The benchmarks rust2-bench's tests count and time: a loop of known length alone, by
//! argument, in a module, in an order that changes, behind a setup and between steps whose own
//! work must go uncounted, and timing itself.

use std::hint::black_box;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

use rust2_bench_harness::{Iterations, bench};

/// `n` iterations of a loop the optimizer keeps.
fn spin(n: u64) -> u64 {
	let mut sum = 0u64;
	for i in 0..n {
		sum = black_box(sum.wrapping_add(i));
	}
	sum
}

#[bench]
fn alone() -> u64 {
	spin(1000)
}

/// Its literals are `u64`s, as `sized` takes them, which the first would overflow as an `i32`.
#[bench(args = [1000, 2000, 4_294_967_296_000 >> 30])]
fn sized(n: u64) -> u64 {
	spin(n)
}

/// An input that costs a million iterations to build and a megabyte to free.
struct Heavy(Vec<u64>);

fn heavy() -> Heavy {
	spin(1_000_000);
	Heavy(vec![1; 1 << 17])
}

/// Counts as `alone` does: neither building nor freeing its input counts.
#[bench(setup = heavy)]
fn borrowed(heavy: &Heavy) -> u64 {
	spin(1000) + heavy.0[0]
}

/// The arguments `generic` takes, borrowed as `&&str`, which its call coerces to `&str`.
const WORDS: [&str; 1] = ["word"];

/// A setup whose argument's type only the benchmark's parameter decides.
fn same<T>(t: T) -> T {
	t
}

/// Named as it runs: `same` passes on `u64`s, and the words coerce to `&str`.
#[bench(args = [4_294_967_296_000 >> 30], setup = same)]
extern "C" fn generic(n: u64) -> u64 {
	spin(n / 4) + spin(WORDS.len() as u64 - 1)
}

/// A step that costs a million iterations.
fn touch(heavy: &mut Heavy) {
	heavy.0[0] = spin(1_000_000) & 1;
}

/// Counts as `alone` does: neither step around it counts.
#[bench(setup = heavy, before = touch, after = touch)]
fn stepped(heavy: &mut Heavy) -> u64 {
	spin(1000) + heavy.0[0]
}

/// Counts near `alone`, called for one iteration: its clock's reads count too.
#[bench]
fn timed(iterations: Iterations) -> Duration {
	let start = Instant::now();
	for _ in iterations {
		black_box(spin(1000));
	}
	start.elapsed()
}

#[bench(args = &WORDS)]
fn word(word: &str) -> u64 {
	spin(1000) + word.len() as u64 - 4
}

/// The arguments in an order that differs each time the expression is evaluated, as a
/// `HashSet`'s does: a case named for one argument must still run it.
fn shuffled() -> [u64; 2] {
	static FLIP: AtomicBool = AtomicBool::new(false);
	if FLIP.fetch_xor(true, Ordering::Relaxed) {
		[8000, 1000]
	} else {
		[1000, 8000]
	}
}

#[bench(args = shuffled())]
fn unstable(n: u64) -> u64 {
	spin(n)
}

mod inner {
	/// `heavy`, built from the case's argument.
	fn from(arg: &str) -> super::Heavy {
		let mut heavy = super::heavy();
		heavy.0[0] = arg.len() as u64;
		heavy
	}

	#[rust2_bench_harness::bench(args = ["a", "bc"], setup = from)]
	fn nested(heavy: &mut super::Heavy) -> u64 {
		heavy.0[0] += 1;
		super::spin(1000)
	}
}

fn main() {
	rust2_bench_harness::main();
}
