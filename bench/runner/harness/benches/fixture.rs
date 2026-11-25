//! The benchmarks rust2-bench's tests count: a loop of known length alone, by argument, in a
//! module, and behind a setup whose own work must go uncounted.

use std::hint::black_box;

use rust2_bench_harness::bench;

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

#[bench(args = [1000, 2000])]
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
