//! A fixture's native benchmark route: times its `bench::run` on the host, as
//! `ts/bench/time.ts` times the other routes. Each fixture's `examples/bench.rs` runs it.
//!
//! usage: bench <sample ms> <rounds> < workloads
//!
//! Each input line is `<name> <input hex>`. Each output line is `<name> <calls per
//! sample> <ns per call, one per round, comma-separated> <output hex>`.

use std::hint::black_box;
use std::io::{BufRead, Write};
use std::time::{Duration, Instant};

/// Time `run` on the workloads read from stdin.
pub(crate) fn main(run: fn(&[u8]) -> Vec<u8>) {
	let args: Vec<String> = std::env::args().skip(1).collect();
	let [sample, rounds] = args.as_slice() else {
		panic!("usage: bench <sample ms> <rounds> < workloads");
	};
	let sample = Duration::from_millis(sample.parse().expect("sample ms"));
	let rounds: usize = rounds.parse().expect("rounds");

	let workloads: Vec<(String, Vec<u8>)> = std::io::stdin()
		.lock()
		.lines()
		.map(|line| {
			let line = line.expect("reading stdin");
			let (name, hex) = line.split_once(' ').expect("a line is `<name> <input hex>`");
			(name.to_string(), unhex(hex))
		})
		.collect();

	// Calibrate each workload to a sample of at least `sample`, which also warms it.
	let calls: Vec<u64> = workloads
		.iter()
		.map(|(_, input)| {
			let mut calls = 1;
			while time(run, input, calls) < sample {
				calls *= 2;
			}
			calls
		})
		.collect();

	// Interleave the workloads in each round, so drift spreads over all of them.
	let mut ns = vec![Vec::with_capacity(rounds); workloads.len()];
	for _ in 0..rounds {
		for (i, (_, input)) in workloads.iter().enumerate() {
			ns[i].push(time(run, input, calls[i]).as_nanos() as f64 / calls[i] as f64);
		}
	}

	let mut stdout = std::io::stdout().lock();
	for (i, (name, input)) in workloads.iter().enumerate() {
		let samples: Vec<String> = ns[i].iter().map(|ns| format!("{ns:.3}")).collect();
		let output: String = run(input).iter().map(|b| format!("{b:02x}")).collect();
		writeln!(stdout, "{name} {} {} {output}", calls[i], samples.join(",")).unwrap();
	}
}

fn time(run: fn(&[u8]) -> Vec<u8>, input: &[u8], calls: u64) -> Duration {
	let start = Instant::now();
	for _ in 0..calls {
		black_box(run(black_box(input)));
	}
	start.elapsed()
}

fn unhex(hex: &str) -> Vec<u8> {
	(0..hex.len())
		.step_by(2)
		.map(|i| u8::from_str_radix(&hex[i..i + 2], 16).expect("input hex"))
		.collect()
}
