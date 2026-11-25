//! Wall-clock times of the working tree's benchmarks, each alone, or each paired with a base
//! commit's: a sample of one beside a sample of the other, in an order drawn anew for each pair,
//! so host load that drifts or recurs biases neither side. Load still widens the interval.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::process::{ChildStdin, ChildStdout, Stdio};
use std::time::{Duration, Instant};

use serde_json::Number;

use crate::compare::Checkout;
use crate::fingerprint::Fnv;
use crate::{Job, Options, cargo, metrics, missing, work};

/// How long a sample's timed part aims to take: long enough that the clock's reads and its
/// resolution vanish in it, short enough that a pair's two samples see the same host and that a
/// preemption spoils few samples, which the median then leaves out.
const SLICE: Duration = Duration::from_millis(1);

/// How long a whole sample may take before its timed part fills a slice, as when steps or inputs
/// built between calls take far longer than the calls.
const WALL: Duration = Duration::from_millis(50);

/// How long each benchmark's samples take in all, both sides of a pair together.
const BUDGET: Duration = Duration::from_secs(1);

/// The fewest samples, or pairs, a benchmark takes however long they take: enough for a 95%
/// interval around their median that leaves out the extremes on each side.
const MIN_SAMPLES: usize = 10;

/// The most iterations one sample takes: a benchmark that runs these in under a slice does
/// nothing the clock can see.
const MAX_ITERATIONS: u64 = 1 << 40;

/// The variable that has a harness target time the benchmark `RUST2_BENCH` names.
const TIME: &str = "RUST2_BENCH_TIME";

/// What a timed harness target prints, alone on a line, before each sample's nanoseconds.
const SAMPLE: &str = "rust2-bench sample ";

/// The unit every time is in.
const UNIT: &str = "ns";

/// Times the working tree's benchmarks, or with `base` each beside the base's, prints them, and
/// writes them to `--out` when given.
pub fn time(base: Option<&str>, options: &Options) -> Result<(), String> {
	if !options.programs.is_empty() || options.build.is_some() {
		return Err("time times a Cargo workspace's benchmarks, not --program's".to_owned());
	}
	if options.jobs.is_some() {
		return Err("time runs one benchmark at a time, so a time is never another's load: --jobs is run's".to_owned());
	}
	let host = host()?;
	eprintln!("rust2-bench: host {host}");
	// Held until the base is timed, so no other compare checks out another commit under it.
	let checkout = base.map(|base| Checkout::new(base, options)).transpose()?;
	let before = match &checkout {
		Some(checkout) => {
			eprintln!(
				"rust2-bench: building {} ({}) in {}",
				base.expect("a checkout is of the base"),
				checkout.short(),
				checkout.dir.display()
			);
			let jobs = cargo::jobs(&checkout.dir, &checkout.options.cargo, false)?;
			// A base from before a rewrite to rust2-bench-harness has only CodSpeed's benchmarks,
			// which a time leaves out, so every benchmark of the head times alone.
			let (codspeed, jobs): (Vec<Job>, Vec<Job>) = jobs.into_iter().partition(|job| job.codspeed);
			if !codspeed.is_empty() {
				eprintln!(
					"rust2-bench: the base's {} benchmark(s) of CodSpeed's crates time as none",
					codspeed.len()
				);
			}
			Some(by_name(jobs))
		}
		None => None,
	};
	eprintln!("rust2-bench: building the working tree");
	let after = by_name(cargo::jobs(Path::new("."), &options.cargo, before.is_none())?);
	if let Some(job) = after.values().find(|job| job.codspeed) {
		return Err(format!(
			"{} is a benchmark of CodSpeed's crates, which only a count measures: write it with rust2-bench-harness, \
			 or leave its target out with cargo's --bench",
			job.label
		));
	}
	let empty = BTreeMap::new();
	let mut names: BTreeSet<String> = before.as_ref().unwrap_or(&empty).keys().chain(after.keys()).cloned().collect();
	if let Some(wanted) = &options.benchmarks {
		missing(wanted, |name| names.contains(name))?;
		names.retain(|name| wanted.contains(name));
	}
	if names.is_empty() {
		return Err("no benchmark ran".to_owned());
	}
	let (mut before, mut after) = (before, after);
	let work = work(Path::new("."))?;
	let mut coin = Coin::seeded();
	let mut times = BTreeMap::new();
	let total = names.len();
	for (index, name) in names.into_iter().enumerate() {
		let started = Instant::now();
		let dir = work.join(index.to_string());
		let base = before.as_mut().and_then(|jobs| jobs.remove(&name));
		let head = after.remove(&name);
		let timed = match (base, head) {
			(Some(base), Some(head)) => {
				let mut base = Process::spawn(base, &dir.join("base"))?;
				let mut head = Process::spawn(head, &dir.join("head"))?;
				let (iterations, pairs) = paired(&mut base, &mut head, &mut coin)?;
				base.finish()?;
				head.finish()?;
				Times::pairs(iterations, &pairs).map_err(|err| format!("{name}: {err}"))?
			}
			(base, head) => {
				let (side, job) = match (base, head) {
					(Some(base), None) => (Which::Base, base),
					(None, Some(head)) => (Which::Head, head),
					_ => unreachable!("every name is one side's"),
				};
				let mut process = Process::spawn(job, &dir.join(side.name()))?;
				let (iterations, samples) = alone(&mut process)?;
				process.finish()?;
				Times::alone(side, iterations, &samples).map_err(|err| format!("{name}: {err}"))?
			}
		};
		eprintln!(
			"rust2-bench: [{}/{total}] {name}: {}, {:.1}s",
			index + 1,
			timed.summary(),
			started.elapsed().as_secs_f64()
		);
		times.insert(name, timed);
	}
	// Every process exited cleanly, so the run's output is no longer evidence.
	std::fs::remove_dir_all(&work).map_err(|e| format!("cannot remove {}: {e}", work.display()))?;
	print!("{}", table(&times, checkout.is_some()));
	if let Some(out) = &options.out {
		file(&times, &host, checkout.as_ref().map(|checkout| checkout.sha.as_str())).write(out)?;
		eprintln!("rust2-bench: wrote {}", out.display());
	}
	Ok(())
}

/// Jobs by the benchmark each runs.
fn by_name(jobs: Vec<Job>) -> BTreeMap<String, Job> {
	jobs.into_iter()
		.map(|job| (job.expect.clone().expect("a Cargo job names its benchmark"), job))
		.collect()
}

/// The label that names this host's hardware in every time's measure, so times from two kinds of
/// CPU never share a series: the architecture, then a hash of the CPU's model, as the system
/// describes it, and of how many CPUs this process may run on.
fn host() -> Result<String, String> {
	let mut hash = Fnv::default();
	hash.write(&cpu()?);
	let cpus = std::thread::available_parallelism().map_err(|e| format!("cannot count the CPUs: {e}"))?;
	hash.write(&cpus.to_string());
	Ok(format!("{}-{:012x}", std::env::consts::ARCH, hash.0 >> 16))
}

/// The CPU's model as Linux describes each logical CPU, by the fields that name it.
#[cfg(target_os = "linux")]
fn cpu() -> Result<String, String> {
	const MODEL: [&str; 9] = [
		"vendor_id",
		"cpu family",
		"model",
		"model name",
		"stepping",
		"CPU implementer",
		"CPU variant",
		"CPU part",
		"CPU revision",
	];
	let info = std::fs::read_to_string("/proc/cpuinfo").map_err(|e| format!("cannot read /proc/cpuinfo: {e}"))?;
	let model: String = info
		.lines()
		.filter(|line| line.split_once(':').is_some_and(|(key, _)| MODEL.contains(&key.trim())))
		.map(|line| format!("{line}\n"))
		.collect();
	if model.is_empty() {
		return Err("/proc/cpuinfo names no CPU model".to_owned());
	}
	Ok(model)
}

/// The CPU's model as macOS names it.
#[cfg(target_os = "macos")]
fn cpu() -> Result<String, String> {
	let out = std::process::Command::new("sysctl")
		.args(["-n", "machdep.cpu.brand_string"])
		.output()
		.map_err(|e| format!("cannot run sysctl: {e}"))?;
	if !out.status.success() {
		return Err(format!("sysctl failed ({})", out.status));
	}
	String::from_utf8(out.stdout).map_err(|_| "sysctl named the CPU in something not UTF-8".to_owned())
}

#[cfg(not(any(target_os = "linux", target_os = "macos")))]
fn cpu() -> Result<String, String> {
	Err("time names CPUs on Linux and macOS alone".to_owned())
}

/// One sample of one side.
#[derive(Clone, Copy, Debug)]
struct Sample {
	/// What the benchmark's timed part took.
	nanos: u64,
	/// What the request took in all, steps and setup between calls included.
	wall: Duration,
}

/// A benchmark set up in a process of one side, which runs a sample when asked.
trait Side {
	fn sample(&mut self, iterations: u64) -> Result<Sample, String>;
}

/// The iterations each sample takes: doubled from one until the timed part of a sample of either
/// side takes a slice, or the whole sample its most. The samples it takes warm both sides up, and
/// are dropped.
fn calibrate(sides: &mut [&mut dyn Side]) -> Result<u64, String> {
	let mut iterations = 1;
	loop {
		let mut full = false;
		for side in sides.iter_mut() {
			let sample = side.sample(iterations)?;
			full |= Duration::from_nanos(sample.nanos) >= SLICE || sample.wall >= WALL;
		}
		if full {
			return Ok(iterations);
		}
		if iterations >= MAX_ITERATIONS {
			return Err(format!(
				"{iterations} iterations took under {} ms: the benchmark does nothing the clock sees",
				SLICE.as_millis()
			));
		}
		iterations *= 2;
	}
}

/// Samples `side` alone until the budget is spent: the iterations each sample took, and each
/// sample's nanoseconds.
fn alone(side: &mut dyn Side) -> Result<(u64, Vec<u64>), String> {
	let iterations = calibrate(&mut [&mut *side])?;
	let mut spent = Duration::ZERO;
	let mut samples = Vec::new();
	while samples.len() < MIN_SAMPLES || spent < BUDGET {
		let sample = side.sample(iterations)?;
		spent += sample.wall;
		samples.push(sample.nanos);
	}
	Ok((iterations, samples))
}

/// Samples `base` and `head` in pairs, in an order `coin` draws anew for each, until the budget is
/// spent: the iterations each sample took, and each pair's nanoseconds, the base's first.
fn paired(base: &mut dyn Side, head: &mut dyn Side, coin: &mut Coin) -> Result<(u64, Vec<(u64, u64)>), String> {
	let iterations = calibrate(&mut [&mut *base, &mut *head])?;
	let mut spent = Duration::ZERO;
	let mut pairs = Vec::new();
	while pairs.len() < MIN_SAMPLES || spent < BUDGET {
		let (b, h) = if coin.flip() {
			let b = base.sample(iterations)?;
			(b, head.sample(iterations)?)
		} else {
			let h = head.sample(iterations)?;
			(base.sample(iterations)?, h)
		};
		spent += b.wall + h.wall;
		pairs.push((b.nanos, h.nanos));
	}
	Ok((iterations, pairs))
}

/// A stream of fair coin flips: xorshift64*, enough to order a pair, seeded apart in every run
/// so no order recurs with the host's own rhythm.
struct Coin(u64);

impl Coin {
	fn seeded() -> Self {
		use std::hash::BuildHasher;
		let seed = std::collections::hash_map::RandomState::new().hash_one(std::process::id());
		Self(seed | 1)
	}

	fn flip(&mut self) -> bool {
		self.0 ^= self.0 >> 12;
		self.0 ^= self.0 << 25;
		self.0 ^= self.0 >> 27;
		self.0.wrapping_mul(0x2545_f491_4f6c_dd1d) >> 63 == 1
	}
}

/// A median and a 95% confidence interval around it.
#[derive(Clone, Copy, Debug, PartialEq)]
struct Estimate {
	value: f64,
	lower: f64,
	upper: f64,
}

impl Estimate {
	/// The median of `values` and a distribution-free 95% interval around it: the order statistics
	/// that a binomial count of values below the median puts it between, which a few samples a
	/// spike of load stretched cannot move.
	fn median(mut values: Vec<f64>) -> Self {
		values.sort_by(f64::total_cmp);
		let n = values.len();
		let value = if n % 2 == 1 {
			values[n / 2]
		} else {
			(values[n / 2 - 1] + values[n / 2]) / 2.0
		};
		let k = rank(n);
		Self {
			value,
			lower: values[k - 1],
			upper: values[n - k],
		}
	}

	fn map(self, f: impl Fn(f64) -> f64) -> Self {
		Self {
			value: f(self.value),
			lower: f(self.lower),
			upper: f(self.upper),
		}
	}
}

/// The largest `k` for which at most 2.5% of samples of `n` values put fewer than `k` below their
/// median: the `k`th smallest and `k`th largest then hold the median with 95% confidence or more.
fn rank(n: usize) -> usize {
	// P(X <= j) for X ~ Binomial(n, 1/2), each term in logs so none underflows before it matters.
	let mut log_term = -(n as f64) * std::f64::consts::LN_2;
	let mut below = 0.0;
	let mut k = 0;
	for j in 0..n {
		below += log_term.exp();
		if below > 0.025 {
			break;
		}
		k = j + 1;
		log_term += ((n - j) as f64 / (j + 1) as f64).ln();
	}
	assert!(k > 0, "{n} samples are too few for a 95% interval");
	k
}

/// Which side a time is of.
#[derive(Clone, Copy, Debug, PartialEq)]
enum Which {
	Base,
	Head,
}

impl Which {
	fn name(self) -> &'static str {
		match self {
			Which::Base => "base",
			Which::Head => "head",
		}
	}
}

/// One benchmark's times, in nanoseconds an iteration: one side's, or both sides' and the
/// head's over the base's.
#[derive(Debug, PartialEq)]
enum Times {
	Base(Estimate),
	Head(Estimate),
	Both { base: Estimate, head: Estimate, ratio: Estimate },
}

impl Times {
	/// The times of a benchmark one side has.
	fn alone(side: Which, iterations: u64, samples: &[u64]) -> Result<Self, String> {
		let estimate = Estimate::median(each(iterations, samples.iter().copied())?);
		Ok(match side {
			Which::Base => Self::Base(estimate),
			Which::Head => Self::Head(estimate),
		})
	}

	/// The times of a benchmark both sides have, from pairs of samples, the base's first.
	fn pairs(iterations: u64, pairs: &[(u64, u64)]) -> Result<Self, String> {
		let base = each(iterations, pairs.iter().map(|&(base, _)| base))?;
		let head = each(iterations, pairs.iter().map(|&(_, head)| head))?;
		// Each pair's ratio, in logs, so a pair twice as slow weighs as one twice as fast.
		let ratios = base.iter().zip(&head).map(|(b, h)| (h / b).ln()).collect();
		Ok(Self::Both {
			base: Estimate::median(base),
			head: Estimate::median(head),
			ratio: Estimate::median(ratios).map(f64::exp),
		})
	}

	fn base(&self) -> Option<Estimate> {
		match self {
			Self::Base(base) | Self::Both { base, .. } => Some(*base),
			Self::Head(_) => None,
		}
	}

	fn head(&self) -> Option<Estimate> {
		match self {
			Self::Head(head) | Self::Both { head, .. } => Some(*head),
			Self::Base(_) => None,
		}
	}

	/// A line on what it took, for the progress log.
	fn summary(&self) -> String {
		match self {
			Self::Both { ratio, .. } => change(*ratio),
			Self::Base(base) => format!("{} (base only)", duration(base.value)),
			Self::Head(head) => duration(head.value),
		}
	}
}

/// Each sample's nanoseconds an iteration, refusing a sample that took none, whose ratio to
/// another would be meaningless.
fn each(iterations: u64, samples: impl Iterator<Item = u64>) -> Result<Vec<f64>, String> {
	samples
		.map(|nanos| match nanos {
			0 => Err(format!("a sample of {iterations} iterations took no time")),
			nanos => Ok(nanos as f64 / iterations as f64),
		})
		.collect()
}

/// `nanos` in the unit that suits it.
fn duration(nanos: f64) -> String {
	let (value, unit) = match nanos {
		n if n < 1e3 => (n, "ns"),
		n if n < 1e6 => (n / 1e3, "µs"),
		n if n < 1e9 => (n / 1e6, "ms"),
		n => (n / 1e9, "s"),
	};
	let precision = match value {
		v if v >= 100.0 => 1,
		v if v >= 10.0 => 2,
		_ => 3,
	};
	format!("{value:.precision$} {unit}")
}

/// A ratio of times as the change it is, with its interval.
fn change(ratio: Estimate) -> String {
	let percent = |ratio: f64| format!("{:+.2}%", (ratio - 1.0) * 100.0);
	format!(
		"{} ({} to {})",
		percent(ratio.value),
		percent(ratio.lower),
		percent(ratio.upper)
	)
}

/// Each benchmark's time as a Markdown table, and with a base, the base's and the change.
fn table(times: &BTreeMap<String, Times>, paired: bool) -> String {
	let interval = |estimate: Estimate| format!("{} to {}", duration(estimate.lower), duration(estimate.upper));
	let cell = |estimate: Option<Estimate>| estimate.map_or("-".to_owned(), |estimate| duration(estimate.value));
	let mut out = if paired {
		String::from("| benchmark | base | head | change | 95% interval |\n|---|---:|---:|---:|---:|\n")
	} else {
		String::from("| benchmark | time | 95% interval |\n|---|---:|---:|\n")
	};
	for (name, times) in times {
		// A `|` in a name would split its cell.
		let shown = name.replace('|', "\\|");
		if paired {
			let (change, interval) = match times {
				Times::Both { ratio, .. } => {
					let percent = |ratio: f64| format!("{:+.2}%", (ratio - 1.0) * 100.0);
					(
						percent(ratio.value),
						format!("{} to {}", percent(ratio.lower), percent(ratio.upper)),
					)
				}
				Times::Base(_) => ("removed".to_owned(), String::new()),
				Times::Head(_) => ("added".to_owned(), String::new()),
			};
			writeln!(
				out,
				"| {shown} | {} | {} | {change} | {interval} |",
				cell(times.base()),
				cell(times.head())
			)
		} else {
			let head = times.head().expect("an unpaired run times the working tree's alone");
			writeln!(out, "| {shown} | {} | {} |", duration(head.value), interval(head))
		}
		.expect("a String takes every write");
	}
	out
}

/// The times as a `metrics.json`: the working tree's as the run's own commit's, and the base's,
/// when there is one, as its commit's.
fn file(times: &BTreeMap<String, Times>, host: &str, base: Option<&str>) -> metrics::File {
	let points = |side: fn(&Times) -> Option<Estimate>| -> BTreeMap<String, metrics::Point> {
		times
			.iter()
			.filter_map(|(name, times)| side(times).map(|estimate| (name.clone(), point(estimate))))
			.collect()
	};
	let mut samples = Vec::new();
	if let Some(base) = base {
		samples.push(metrics::Sample {
			commit: Some(base.to_owned()),
			points: points(Times::base),
		});
	}
	samples.push(metrics::Sample {
		commit: None,
		points: points(Times::head),
	});
	metrics::File {
		measure: format!("time ({host})"),
		unit: UNIT,
		samples,
	}
}

/// An estimate as a point, to five significant digits, more than a time's interval resolves.
fn point(estimate: Estimate) -> metrics::Point {
	let number = |value: f64| {
		let rounded: f64 = format!("{value:.4e}").parse().expect("a float reads back");
		Number::from_f64(rounded).expect("a time is finite")
	};
	metrics::Point {
		value: number(estimate.value),
		interval: Some((number(estimate.lower), number(estimate.upper))),
	}
}

/// A harness target's process timing one benchmark, set up once, a sample for each request.
struct Process {
	label: String,
	dir: PathBuf,
	child: std::process::Child,
	stdin: ChildStdin,
	stdout: BufReader<ChildStdout>,
	/// The target's own output, apart from its samples.
	log: std::fs::File,
}

impl Process {
	/// Starts `job`'s target timing its benchmark, its output in `dir`.
	fn spawn(job: Job, dir: &Path) -> Result<Self, String> {
		let Job { label, mut command, .. } = job;
		std::fs::create_dir_all(dir).map_err(|e| format!("cannot create {}: {e}", dir.display()))?;
		let file = |name: &str| {
			std::fs::File::create(dir.join(name)).map_err(|e| format!("cannot create {}: {e}", dir.join(name).display()))
		};
		let mut child = command
			.env(TIME, "1")
			.stdin(Stdio::piped())
			.stdout(Stdio::piped())
			.stderr(file("stderr")?)
			.spawn()
			.map_err(|e| format!("cannot run {label}: {e}"))?;
		let stdin = child.stdin.take().expect("stdin is piped");
		let stdout = BufReader::new(child.stdout.take().expect("stdout is piped"));
		Ok(Self {
			label,
			dir: dir.to_owned(),
			child,
			stdin,
			stdout,
			log: file("stdout")?,
		})
	}

	/// Why the process stopped answering: how it exited, and where its output is.
	fn failure(&mut self) -> String {
		let status = match self.child.wait() {
			Ok(status) => status.to_string(),
			Err(e) => format!("cannot wait for it: {e}"),
		};
		format!(
			"{} stopped timing ({status}); its output is in {}",
			self.label,
			self.dir.display()
		)
	}

	/// Ends the benchmark: closes its requests, keeps the rest of its output, and fails unless it
	/// exits cleanly.
	fn finish(self) -> Result<(), String> {
		let Self {
			label,
			dir,
			mut child,
			stdin,
			mut stdout,
			mut log,
		} = self;
		drop(stdin);
		std::io::copy(&mut stdout, &mut log).map_err(|e| format!("cannot keep {label}'s output: {e}"))?;
		let status = child.wait().map_err(|e| format!("cannot wait for {label}: {e}"))?;
		if !status.success() {
			return Err(format!(
				"{label} failed ({status}); its output is in {}",
				dir.display()
			));
		}
		Ok(())
	}
}

impl Side for Process {
	fn sample(&mut self, iterations: u64) -> Result<Sample, String> {
		let started = Instant::now();
		if writeln!(self.stdin, "{iterations}")
			.and_then(|()| self.stdin.flush())
			.is_err()
		{
			return Err(self.failure());
		}
		let mut line = String::new();
		loop {
			line.clear();
			match self.stdout.read_line(&mut line) {
				Ok(0) | Err(_) => return Err(self.failure()),
				Ok(_) => {}
			}
			if let Some(nanos) = line.strip_prefix(SAMPLE).and_then(|rest| rest.strip_suffix('\n')) {
				let nanos = nanos
					.parse()
					.map_err(|_| format!("{} printed a sample of {nanos:?} nanoseconds", self.label))?;
				return Ok(Sample {
					nanos,
					wall: started.elapsed(),
				});
			}
			self.log
				.write_all(line.as_bytes())
				.map_err(|e| format!("cannot keep {}'s output: {e}", self.label))?;
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	/// A side whose samples take what `nanos` says an iteration takes, given the sample's index,
	/// and `step` nanoseconds more an iteration untimed, recording each request in a log shared
	/// with another side.
	struct Fake<'a> {
		name: &'static str,
		nanos: fn(usize) -> u64,
		step: u64,
		log: &'a std::cell::RefCell<Vec<(&'static str, u64)>>,
	}

	impl Side for Fake<'_> {
		fn sample(&mut self, iterations: u64) -> Result<Sample, String> {
			let mut log = self.log.borrow_mut();
			let index = log.iter().filter(|(name, _)| *name == self.name).count();
			log.push((self.name, iterations));
			let nanos = (self.nanos)(index) * iterations;
			Ok(Sample {
				nanos,
				wall: Duration::from_nanos(nanos + self.step * iterations),
			})
		}
	}

	#[test]
	fn calibrates_until_either_side_fills_a_slice() {
		let log = Default::default();
		let mut slow = Fake {
			name: "slow",
			nanos: |_| 1_000_000,
			step: 0,
			log: &log,
		};
		let mut fast = Fake {
			name: "fast",
			nanos: |_| 10_000,
			step: 0,
			log: &log,
		};
		assert_eq!(calibrate(&mut [&mut fast, &mut slow]).unwrap(), 1);
		assert_eq!(calibrate(&mut [&mut fast]).unwrap(), 128);
		// A step of 10 ms an iteration fills a sample's whole by eight iterations.
		let mut stepped = Fake {
			name: "stepped",
			nanos: |_| 1_000,
			step: 10_000_000,
			log: &log,
		};
		assert_eq!(calibrate(&mut [&mut stepped]).unwrap(), 8);
		let mut nothing = Fake {
			name: "nothing",
			nanos: |_| 0,
			step: 0,
			log: &log,
		};
		assert!(calibrate(&mut [&mut nothing]).unwrap_err().contains("does nothing"));
	}

	#[test]
	fn pairs_samples_in_both_orders_until_the_budget_is_spent() {
		let log = Default::default();
		let mut base = Fake {
			name: "base",
			nanos: |_| 1_000_000,
			step: 0,
			log: &log,
		};
		// A drift that slows each sample, which the pairs see on both sides alike.
		let mut head = Fake {
			name: "head",
			nanos: |index| 1_000_000 + index as u64,
			step: 0,
			log: &log,
		};
		let mut coin = Coin(0x9e37_79b9_7f4a_7c15);
		let (iterations, pairs) = paired(&mut base, &mut head, &mut coin).unwrap();
		assert_eq!(iterations, 1);
		// Each pair takes 2 ms, so 500 of them spend the budget.
		assert_eq!(pairs.len(), 500);
		let log = log.borrow();
		let measured = &log[log.len() - 2 * pairs.len()..];
		let firsts: Vec<&str> = measured.chunks(2).map(|pair| pair[0].0).collect();
		assert!(firsts.contains(&"base") && firsts.contains(&"head"), "{firsts:?}");
		for pair in measured.chunks(2) {
			assert_ne!(pair[0].0, pair[1].0, "a pair holds one sample of each side");
		}
	}

	#[test]
	fn takes_the_fewest_samples_however_long_each_takes() {
		let log = Default::default();
		let mut slow = Fake {
			name: "slow",
			nanos: |_| 1_000_000_000,
			step: 0,
			log: &log,
		};
		let (iterations, samples) = alone(&mut slow).unwrap();
		assert_eq!((iterations, samples.len()), (1, MIN_SAMPLES));
	}

	#[test]
	fn ranks_the_order_statistics_a_95_percent_interval_takes() {
		// P(X <= 0) = 1/64 for six, P(X <= 1) = 11/1024 for ten, and for a hundred, P(X <= 39)
		// is 1.8% and P(X <= 40) 2.8%.
		assert_eq!(rank(6), 1);
		assert_eq!(rank(10), 2);
		assert_eq!(rank(100), 40);
		// Enough samples that their first terms underflow.
		let k = rank(5000);
		assert!((2429..2432).contains(&k), "{k}");
	}

	#[test]
	fn estimates_a_median_its_extremes_cannot_move() {
		let mut values: Vec<f64> = (1..=10).map(f64::from).collect();
		values[9] = 1e9;
		assert_eq!(
			Estimate::median(values),
			Estimate {
				value: 5.5,
				lower: 2.0,
				upper: 9.0
			}
		);
	}

	#[test]
	fn a_pair_compares_in_ratios_of_its_own_samples() {
		// The base drifts from 100 to 200 ns; the head is 10% slower than the base beside it.
		let pairs: Vec<(u64, u64)> = (0..10).map(|i| (100 + 10 * i, (110 + 11 * i))).collect();
		let times = Times::pairs(1, &pairs).unwrap();
		let Times::Both { ratio, .. } = times else { panic!("{times:?}") };
		for value in [ratio.value, ratio.lower, ratio.upper] {
			assert!((value - 1.1).abs() < 1e-9, "{ratio:?}");
		}
		assert!(Times::pairs(1, &[(0, 1)]).unwrap_err().contains("took no time"));
	}

	#[test]
	fn tables_and_writes_each_side() {
		let estimate = |value: f64| Estimate {
			value,
			lower: value * 0.9,
			upper: value * 1.1,
		};
		let times = BTreeMap::from([
			(
				"a".to_owned(),
				Times::Both {
					base: estimate(2000.0),
					head: estimate(1500.0),
					ratio: estimate(0.75),
				},
			),
			(
				"gone".to_owned(),
				Times::Base(estimate(5.0)),
			),
			(
				"new|x".to_owned(),
				Times::Head(estimate(12_345_678.0)),
			),
		]);
		assert_eq!(
			table(&times, true),
			"| benchmark | base | head | change | 95% interval |\n|---|---:|---:|---:|---:|\n\
			 | a | 2.000 µs | 1.500 µs | -25.00% | -32.50% to -17.50% |\n\
			 | gone | 5.000 ns | - | removed |  |\n\
			 | new\\|x | - | 12.35 ms | added |  |\n"
		);
		let base = "c".repeat(40);
		let json = file(&times, "x86_64-0123456789ab", Some(&base)).json().unwrap();
		let json: serde_json::Value = serde_json::from_str(&json).unwrap();
		let measure = "time (x86_64-0123456789ab)";
		assert_eq!(json["units"][measure], "ns");
		assert_eq!(json["samples"][0]["commit"], base);
		assert_eq!(json["samples"][0]["metrics"]["gone"][measure]["value"], 5.0);
		assert_eq!(json["samples"][0]["metrics"]["a"][measure]["upper_value"], 2200.0);
		assert!(json["samples"][1].get("commit").is_none());
		assert_eq!(json["samples"][1]["metrics"]["new|x"][measure]["value"], 12_346_000.0);
		assert!(json["samples"][1]["metrics"].get("gone").is_none());
	}
}
