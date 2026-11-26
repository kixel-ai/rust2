//! Benchmarks as plain functions marked `#[bench]`, which [rust2-bench] counts in
//! instructions under Valgrind and times by wall clock. See the README.
//!
//! [rust2-bench]: https://github.com/kixel-ai/rust2/tree/main/bench/runner

use std::collections::BTreeMap;
use std::io::{BufRead, Write};

/// Marks a function as a benchmark: `#[bench]`, `#[bench(args = [..])]`, `#[bench(setup = f)]`,
/// with `before = g` or `after = h` steps on setup's output. See the README for what each
/// passes the function.
pub use rust2_bench_macros::bench;

/// The variable naming the one benchmark a run counts or times, which rust2-bench sets on each job.
const ONLY: &str = "RUST2_BENCH";

/// The variable that, set to `1`, times the benchmark `RUST2_BENCH` names, a sample for each
/// iteration count a line of stdin holds.
const TIME: &str = "RUST2_BENCH_TIME";

/// What a run prints after each benchmark, before its name, which rust2-bench lists them by.
const RAN: &str = "rust2-bench ran ";

/// What a timed run prints, alone on a line, before each sample's nanoseconds.
const SAMPLE: &str = "rust2-bench sample ";

/// How many times a benchmark that times itself runs its work, as a loop takes it:
/// `for _ in iterations`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Iterations(u64);

impl Iterations {
	/// The count.
	pub fn get(self) -> u64 {
		self.0
	}
}

impl IntoIterator for Iterations {
	type Item = u64;
	type IntoIter = std::ops::Range<u64>;

	fn into_iter(self) -> Self::IntoIter {
		0..self.0
	}
}

/// The variable that, set to a PMU's perf type (0 on a host with one), counts each benchmark's
/// user-space instructions with Linux perf counters, natively.
const PERF: &str = "RUST2_BENCH_PERF";

/// What a perf count prints before each benchmark's count, `alone` or `threads`, and name.
#[cfg(target_os = "linux")]
const COUNTED: &str = "rust2-bench counted ";

#[cfg(target_os = "linux")]
mod perf;

/// Fails unless this host lets a process count its own user-space instructions on the PMU of
/// perf type `kind`, saying why not: rust2-bench's check before it builds anything.
#[doc(hidden)]
pub fn __probe_perf(kind: u32) -> Result<(), String> {
	#[cfg(target_os = "linux")]
	return perf::probe(kind);
	#[cfg(not(target_os = "linux"))]
	{
		let _ = kind;
		Err("perf counters are Linux's: count under Valgrind instead".to_owned())
	}
}

/// Runs every benchmark the program's `#[bench]` functions declare, or only the one
/// `RUST2_BENCH` names, each counted alone under Valgrind and once natively, or timed when
/// `RUST2_BENCH_TIME` is set. Call it from a benchmark target's `main`.
pub fn main() {
	if let Err(err) = run() {
		eprintln!("rust2-bench-harness: {err}");
		std::process::exit(1);
	}
}

fn run() -> Result<(), String> {
	// Cargo passes `--bench` to a benchmark target, and `--quiet` under `cargo -q`; nothing
	// else, but what follows its `--`, which a filter would take: rust2-bench chooses instead.
	let mut quiet = false;
	for arg in std::env::args().skip(1) {
		match arg.as_str() {
			"--bench" => {}
			"--quiet" => quiet = true,
			_ => {
				return Err(format!(
					"unknown argument {arg}: a benchmark target takes none, and rust2-bench's --benchmarks chooses benchmarks"
				));
			}
		}
	}
	let mut cases = cases(inventory::iter::<__private::Bench>)?;
	let only = match std::env::var(ONLY) {
		Ok(only) => Some(only),
		Err(std::env::VarError::NotPresent) => None,
		Err(std::env::VarError::NotUnicode(_)) => return Err(format!("{ONLY} is not UTF-8")),
	};
	let timed = match std::env::var_os(TIME) {
		None => false,
		Some(value) if value == "1" => true,
		Some(value) => return Err(format!("{TIME} is {}, not 1", value.to_string_lossy())),
	};
	if let Some(only) = only {
		let case = cases
			.remove(&only)
			.ok_or(format!("{ONLY} names {only}, which no benchmark is"))?;
		cases = BTreeMap::from([(only, case)]);
	} else if timed {
		return Err(format!("{TIME} times one benchmark, which {ONLY} names"));
	}
	if timed {
		let (_, case) = cases.pop_first().expect("one case is left");
		return serve((case.build)(), std::io::stdin().lock(), std::io::stdout());
	}
	let perf = match std::env::var(PERF) {
		Ok(kind) => Some(
			kind.parse::<u32>()
				.map_err(|_| format!("{PERF} is {kind}, not a PMU's perf type"))?,
		),
		Err(std::env::VarError::NotPresent) => None,
		Err(std::env::VarError::NotUnicode(_)) => return Err(format!("{PERF} is not UTF-8")),
	};
	#[cfg(target_os = "linux")]
	if perf.is_some() {
		perf::preloaded()?;
	}
	for (name, case) in cases {
		let meter = __private::Meter::new(&name, perf)?;
		let mut sampler = (case.build)();
		sampler.count(&meter);
		drop(sampler);
		#[cfg(target_os = "linux")]
		if let Some(counter) = &meter.perf {
			let count = counter.count().map_err(|err| format!("{name}: {err}"))?;
			let threads = if count.threaded { "threads" } else { "alone" };
			println!("{COUNTED}{} {threads} {name}", count.instructions);
		}
		if !quiet {
			println!("{RAN}{name}");
		}
	}
	Ok(())
}

/// Times `sampler` once for each line of `commands`, an iteration count, printing each sample's
/// nanoseconds to `out` until `commands` ends. Each sample's line starts a line of its own, so a
/// benchmark's own unfinished line never runs into it.
fn serve(mut sampler: Box<dyn __private::Sampler>, commands: impl BufRead, mut out: impl Write) -> Result<(), String> {
	for line in commands.lines() {
		let line = line.map_err(|e| format!("cannot read a sample's iterations: {e}"))?;
		let iterations = line
			.parse::<u64>()
			.ok()
			.filter(|&n| n > 0)
			.ok_or(format!("cannot sample {line:?} iterations"))?;
		let nanos = sampler.time(iterations);
		write!(out, "\n{SAMPLE}{nanos}\n")
			.and_then(|()| out.flush())
			.map_err(|e| format!("cannot write a sample: {e}"))?;
	}
	Ok(())
}

/// Each case of `benches` by its name. Every benchmark's args are evaluated here, once, and
/// each argument moves into its case, so a name always holds the argument its case runs.
fn cases<'a>(
	benches: impl IntoIterator<Item = &'a __private::Bench>,
) -> Result<BTreeMap<String, __private::Case>, String> {
	let mut cases = BTreeMap::new();
	for bench in benches {
		let base = bench.base();
		let each = (bench.cases)();
		if each.is_empty() {
			return Err(format!("{base} has no args"));
		}
		for case in each {
			let name = match &case.arg {
				Some(arg) => format!("{base}[{arg}]"),
				None => base.clone(),
			};
			// rust2-bench reads names a line each, and Kixel Quest takes none holding one.
			if name.chars().any(char::is_control) {
				return Err(format!(
					"{name:?} holds a control character, which no benchmark's name may"
				));
			}
			if cases.contains_key(&name) {
				return Err(format!("two benchmarks are named {name}"));
			}
			cases.insert(name, case);
		}
	}
	if cases.is_empty() {
		return Err("no function is marked #[bench]".to_owned());
	}
	Ok(cases)
}

/// What `#[bench]` expands to, which only it may use.
#[doc(hidden)]
pub mod __private {
	use std::ffi::CString;
	use std::fmt::Display;
	use std::time::{Duration, Instant};

	pub use core::hint::black_box;
	pub use inventory;

	pub use crate::Iterations;

	/// One `#[bench]` function, registered at link time.
	pub struct Bench {
		/// Its file, as `file!()` names it: relative to the workspace root.
		pub file: &'static str,
		/// Its module, as `module_path!()` names it, starting with the crate's.
		pub module: &'static str,
		pub name: &'static str,
		/// Its cases: one without args, else one per argument, built by evaluating the args once.
		pub cases: fn() -> Vec<Case>,
	}

	/// Builds a case's sampler, running its setup.
	type Build = Box<dyn FnOnce() -> Box<dyn Sampler>>;

	/// One case of a benchmark, ready to set up.
	pub struct Case {
		/// Its argument, shown, when the benchmark declares args.
		pub(crate) arg: Option<String>,
		pub(crate) build: Build,
	}

	inventory::collect!(Bench);

	impl Bench {
		/// Its name before any argument: the file, the module path within the crate, then
		/// the function, as CodSpeed's crates name theirs by file, group, and function.
		pub(crate) fn base(&self) -> String {
			match self.module.split_once("::") {
				Some((_, path)) => format!("{}::{path}::{}", self.file, self.name),
				None => format!("{}::{}", self.file, self.name),
			}
		}
	}

	/// A case set up, which runs its benchmark once counted or many times timed.
	pub trait Sampler {
		/// Calls the benchmark once, counting only the call between the meter's requests.
		fn count(&mut self, meter: &Meter);
		/// Calls the benchmark `iterations` times, returning the nanoseconds its timed part took.
		fn time(&mut self, iterations: u64) -> u64;
	}

	/// Brackets one case with Callgrind's client requests, so Valgrind run with
	/// `--instr-atstart=no` counts it alone and dumps it under its name. Natively each
	/// request is an instruction sequence that does nothing.
	pub struct Meter {
		name: CString,
		/// The case's perf counters, when it counts natively.
		#[cfg(target_os = "linux")]
		pub(crate) perf: Option<crate::perf::Counter>,
	}

	impl Meter {
		/// A meter of the case `name`, which also counts with perf counters of the PMU of perf
		/// type `perf`, when given.
		pub(crate) fn new(name: &str, perf: Option<u32>) -> Result<Self, String> {
			let name = CString::new(name).map_err(|_| format!("{name:?} holds a NUL"))?;
			#[cfg(not(target_os = "linux"))]
			if perf.is_some() {
				return Err(format!("{}: perf counters are Linux's", crate::PERF));
			}
			Ok(Self {
				name,
				#[cfg(target_os = "linux")]
				perf: perf.map(crate::perf::Counter::new),
			})
		}

		/// Zeroes the counts and starts counting.
		#[inline(always)]
		pub fn start(&self) {
			crate::valgrind::request(crate::valgrind::ZERO_STATS, 0);
			crate::valgrind::request(crate::valgrind::START_INSTRUMENTATION, 0);
			#[cfg(target_os = "linux")]
			if let Some(perf) = &self.perf {
				perf.start();
			}
		}

		/// Stops counting and dumps the count under the case's name.
		#[inline(always)]
		pub fn stop(&self) {
			#[cfg(target_os = "linux")]
			if let Some(perf) = &self.perf {
				perf.stop();
			}
			crate::valgrind::request(crate::valgrind::STOP_INSTRUMENTATION, 0);
			crate::valgrind::request(crate::valgrind::DUMP_STATS_AT, self.name.as_ptr() as usize);
		}
	}

	/// The nanoseconds in `elapsed`, as a sample reports them.
	fn nanos(elapsed: Duration) -> u64 {
		u64::try_from(elapsed.as_nanos()).unwrap_or(u64::MAX)
	}

	/// A benchmark whose every call takes an input of its own, which `make` builds before the
	/// count or the clock starts, or none.
	struct Fresh<M, F> {
		make: M,
		call: F,
		/// Whether `make` is the benchmark's own setup, which runs just before each call, rather
		/// than a clone of its argument, which a sample may build for all of its calls first.
		setup: bool,
	}

	impl<I, O, M: FnMut() -> I, F: FnMut(I) -> O> Sampler for Fresh<M, F> {
		fn count(&mut self, meter: &Meter) {
			let input = (self.make)();
			meter.start();
			let output = black_box((self.call)(input));
			meter.stop();
			drop(output);
		}

		fn time(&mut self, iterations: u64) -> u64 {
			if !self.setup && !std::mem::needs_drop::<I>() && !std::mem::needs_drop::<O>() {
				// Clones with nothing to drop, so one clock around every call, the clones built
				// before it and their vector freed after it.
				let mut inputs: Vec<I> = (0..iterations).map(|_| (self.make)()).collect();
				let start = Instant::now();
				for input in inputs.drain(..) {
					black_box((self.call)(input));
				}
				let elapsed = start.elapsed();
				drop(inputs);
				return nanos(elapsed);
			}
			// A clock around each call, so each setup runs just before its call, and no call's
			// input or output, such as a large fixture or a lock's guard, is still held when the
			// next runs.
			let mut total = Duration::ZERO;
			for _ in 0..iterations {
				let input = (self.make)();
				let start = Instant::now();
				let output = black_box((self.call)(input));
				total += start.elapsed();
				drop(output);
			}
			nanos(total)
		}
	}

	/// A benchmark that borrows one input, built once, across every call, with steps on it
	/// before and after each call that neither the count nor the clock sees.
	struct Kept<I, F> {
		input: I,
		call: F,
		before: Option<Step<I>>,
		after: Option<Step<I>>,
	}

	/// A step on a kept input between calls.
	type Step<I> = Box<dyn FnMut(&mut I)>;

	impl<I, O, F: FnMut(&mut I) -> O> Sampler for Kept<I, F> {
		fn count(&mut self, meter: &Meter) {
			if let Some(before) = &mut self.before {
				before(&mut self.input);
			}
			meter.start();
			let output = black_box((self.call)(&mut self.input));
			meter.stop();
			drop(output);
			if let Some(after) = &mut self.after {
				after(&mut self.input);
			}
		}

		fn time(&mut self, iterations: u64) -> u64 {
			if self.before.is_none() && self.after.is_none() && !std::mem::needs_drop::<O>() {
				// No step and nothing to drop, so one clock around every call.
				let start = Instant::now();
				for _ in 0..iterations {
					black_box((self.call)(&mut self.input));
				}
				return nanos(start.elapsed());
			}
			// A step or a drop between calls, so a clock around each call: no output, such as a
			// lock's guard, is still held when the next call runs.
			let mut total = Duration::ZERO;
			for _ in 0..iterations {
				if let Some(before) = &mut self.before {
					before(&mut self.input);
				}
				let start = Instant::now();
				let output = black_box((self.call)(&mut self.input));
				total += start.elapsed();
				drop(output);
				if let Some(after) = &mut self.after {
					after(&mut self.input);
				}
			}
			nanos(total)
		}
	}

	/// A benchmark that times itself, given the iterations to run, with an input of its own
	/// each sample, which `make` builds, or none.
	struct FreshTimed<M, F> {
		make: M,
		call: F,
	}

	impl<I, M: FnMut() -> I, F: FnMut(I, Iterations) -> Duration> Sampler for FreshTimed<M, F> {
		fn count(&mut self, meter: &Meter) {
			let input = (self.make)();
			meter.start();
			black_box((self.call)(input, Iterations(1)));
			meter.stop();
		}

		fn time(&mut self, iterations: u64) -> u64 {
			let input = (self.make)();
			nanos((self.call)(input, Iterations(iterations)))
		}
	}

	/// A benchmark that times itself, given the iterations to run, borrowing one input
	/// across every sample.
	struct KeptTimed<I, F> {
		input: I,
		call: F,
	}

	impl<I, F: FnMut(&mut I, Iterations) -> Duration> Sampler for KeptTimed<I, F> {
		fn count(&mut self, meter: &Meter) {
			meter.start();
			black_box((self.call)(&mut self.input, Iterations(1)));
			meter.stop();
		}

		fn time(&mut self, iterations: u64) -> u64 {
			nanos((self.call)(&mut self.input, Iterations(iterations)))
		}
	}

	/// A benchmark whose every call takes an input its setup, `make`, builds just before it.
	pub fn fresh<I: 'static, O: 'static>(
		make: impl FnMut() -> I + 'static,
		call: impl FnMut(I) -> O + 'static,
	) -> Box<dyn Sampler> {
		Box::new(Fresh {
			make,
			call,
			setup: true,
		})
	}

	/// A benchmark whose every call takes a clone of its argument, which `make` makes, or none.
	pub fn cloned<I: 'static, O: 'static>(
		make: impl FnMut() -> I + 'static,
		call: impl FnMut(I) -> O + 'static,
	) -> Box<dyn Sampler> {
		Box::new(Fresh {
			make,
			call,
			setup: false,
		})
	}

	/// A benchmark that borrows `input` across every call, with optional steps between calls.
	pub fn kept<I: 'static, O: 'static>(
		input: I,
		call: impl FnMut(&mut I) -> O + 'static,
		before: Option<Step<I>>,
		after: Option<Step<I>>,
	) -> Box<dyn Sampler> {
		Box::new(Kept {
			input,
			call,
			before,
			after,
		})
	}

	/// A benchmark that times itself, with an input `make` builds each sample, or none.
	pub fn fresh_timed<I: 'static>(
		make: impl FnMut() -> I + 'static,
		call: impl FnMut(I, Iterations) -> Duration + 'static,
	) -> Box<dyn Sampler> {
		Box::new(FreshTimed { make, call })
	}

	/// A benchmark that times itself, borrowing `input` across every sample.
	pub fn kept_timed<I: 'static>(
		input: I,
		call: impl FnMut(&mut I, Iterations) -> Duration + 'static,
	) -> Box<dyn Sampler> {
		Box::new(KeptTimed { input, call })
	}

	/// A step on a kept input between calls, boxed.
	pub fn step<I>(step: impl FnMut(&mut I) + 'static) -> Option<Step<I>> {
		Some(Box::new(step))
	}

	/// Whether `path`, a `file!()`, is relative, so a benchmark's name holds no host's path.
	pub const fn relative(path: &str) -> bool {
		match path.as_bytes() {
			[b'/' | b'\\', ..] => false,
			// A Windows drive.
			[_, b':', ..] => false,
			_ => true,
		}
	}

	/// The one case of a benchmark without args, which `build` sets up.
	pub fn single(build: impl FnOnce() -> Box<dyn Sampler> + 'static) -> Vec<Case> {
		vec![Case {
			arg: None,
			build: Box::new(build),
		}]
	}

	/// A case per argument among `args`, shown, each moving its argument into the setup `make`
	/// builds from it. `make`'s body gives the argument the type it runs with.
	pub fn each<A: Display, B: FnOnce() -> Box<dyn Sampler> + 'static>(
		args: impl IntoIterator<Item = A>,
		make: impl Fn(A) -> B,
	) -> Vec<Case> {
		args.into_iter()
			.map(|arg| Case {
				arg: Some(arg.to_string()),
				build: Box::new(make(arg)),
			})
			.collect()
	}
}

/// Callgrind's client requests, from valgrind/callgrind.h.
mod valgrind {
	const BASE: usize = ((b'C' as usize) << 24) | ((b'T' as usize) << 16);
	pub const ZERO_STATS: usize = BASE + 1;
	pub const DUMP_STATS_AT: usize = BASE + 3;
	pub const START_INSTRUMENTATION: usize = BASE + 4;
	pub const STOP_INSTRUMENTATION: usize = BASE + 5;

	/// A client request: Valgrind's magic preamble, a no-op on the CPU, then the request,
	/// which Valgrind reads from `args`.
	#[inline(always)]
	pub fn request(code: usize, arg: usize) {
		let args = [code, arg, 0, 0, 0, 0];
		#[cfg(target_arch = "x86_64")]
		// SAFETY: the rotations sum to 128, leaving rdi as it was, and `xchg rbx, rbx` changes
		// nothing; under Valgrind, the request reads `args` and writes rdx.
		unsafe {
			std::arch::asm!(
				"rol rdi, 3", "rol rdi, 13", "rol rdi, 61", "rol rdi, 51", "xchg rbx, rbx",
				in("rax") args.as_ptr(),
				inout("rdx") 0usize => _,
				options(nostack),
			);
		}
		#[cfg(target_arch = "aarch64")]
		// SAFETY: the rotations sum to 128, leaving x12 as it was, and `orr x10, x10, x10`
		// changes nothing; under Valgrind, the request reads `args` and writes x3.
		unsafe {
			std::arch::asm!(
				"ror x12, x12, #3", "ror x12, x12, #13", "ror x12, x12, #51", "ror x12, x12, #61",
				"orr x10, x10, x10",
				in("x4") args.as_ptr(),
				inout("x3") 0usize => _,
				options(nostack),
			);
		}
		// Elsewhere Valgrind cannot run, so there is nothing to request.
		#[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
		let _ = args;
	}
}

#[cfg(test)]
mod tests {
	use std::cell::RefCell;
	use std::rc::Rc;
	use std::time::Duration;

	use super::__private::{
		Bench, Meter, Sampler, cloned, each, fresh, fresh_timed, kept, kept_timed, relative, single, step,
	};
	use super::*;

	fn nothing() -> Box<dyn Sampler> {
		fresh(|| (), |()| ())
	}

	fn bench(module: &'static str, name: &'static str) -> Bench {
		Bench {
			file: "benches/a.rs",
			module,
			name,
			cases: || single(nothing),
		}
	}

	#[test]
	fn names_each_case_by_file_module_function_and_argument() {
		let benches = [
			bench("a", "top"),
			bench("a::inner::deeper", "nested"),
			Bench {
				file: "benches/a.rs",
				module: "a",
				name: "sized",
				cases: || each(["16", "1024"], |_| nothing),
			},
		];
		let cases = cases(&benches).unwrap();
		let names: Vec<&str> = cases.keys().map(String::as_str).collect();
		assert_eq!(
			names,
			[
				"benches/a.rs::inner::deeper::nested",
				"benches/a.rs::sized[1024]",
				"benches/a.rs::sized[16]",
				"benches/a.rs::top",
			]
		);
	}

	#[test]
	fn refuses_two_cases_of_one_name_and_args_of_none() {
		let twice = [Bench {
			file: "benches/a.rs",
			module: "a",
			name: "f",
			cases: || each([1, 1], |_| nothing),
		}];
		assert_eq!(
			cases(&twice).err().unwrap(),
			"two benchmarks are named benches/a.rs::f[1]"
		);
		let lines = [Bench {
			file: "benches/a.rs",
			module: "a",
			name: "f",
			cases: || each(["a\nb"], |_| nothing),
		}];
		assert!(cases(&lines).err().unwrap().contains("control character"));
		let empty = [Bench {
			file: "benches/a.rs",
			module: "a",
			name: "f",
			cases: || each(Vec::<u8>::new(), |_| nothing),
		}];
		assert_eq!(cases(&empty).err().unwrap(), "benches/a.rs::f has no args");
		assert_eq!(cases(&[]).err().unwrap(), "no function is marked #[bench]");
	}

	#[test]
	fn a_name_with_a_nul_is_refused() {
		assert!(Meter::new("a\0b", None).is_err());
	}

	#[test]
	fn a_file_must_be_relative() {
		assert!(relative("benches/a.rs"));
		assert!(relative("rs/net/benches/a.rs"));
		assert!(!relative("/home/me/ws/benches/a.rs"));
		assert!(!relative("C:\\ws\\benches\\a.rs"));
		assert!(!relative("\\\\server\\ws\\a.rs"));
	}

	/// What a sampler did, in order.
	type Log = Rc<RefCell<Vec<&'static str>>>;

	/// An output that logs its drop.
	struct Dropped(Log);

	impl Drop for Dropped {
		fn drop(&mut self) {
			self.0.borrow_mut().push("drop");
		}
	}

	#[test]
	fn a_fresh_input_is_built_for_every_call_before_any_runs() {
		let log = Log::default();
		let (made, called) = (log.clone(), log.clone());
		let mut sampler = fresh(
			move || made.borrow_mut().push("make"),
			move |()| {
				called.borrow_mut().push("call");
				Dropped(called.clone())
			},
		);
		// An output to drop, so each call's input is built before it, and its output dropped after.
		sampler.time(2);
		assert_eq!(*log.borrow(), ["make", "call", "drop", "make", "call", "drop"]);
		log.borrow_mut().clear();
		sampler.count(&Meter::new("a", None).unwrap());
		assert_eq!(*log.borrow(), ["make", "call", "drop"]);

		// With nothing to drop, setup still runs just before each call, since it may reset what
		// the call reads, while clones of an argument are built for the whole sample first.
		for (sampler, order) in [
			(fresh as fn(_, _) -> _, ["make", "call", "make", "call"]),
			(cloned, ["make", "make", "call", "call"]),
		] {
			let log = Log::default();
			let (made, called) = (log.clone(), log.clone());
			let mut sampler = sampler(
				Box::new(move || {
					made.borrow_mut().push("make");
					1u64
				}) as Box<dyn FnMut() -> u64>,
				Box::new(move |n: u64| {
					called.borrow_mut().push("call");
					n
				}) as Box<dyn FnMut(u64) -> u64>,
			);
			sampler.time(2);
			assert_eq!(*log.borrow(), order);
		}
	}

	#[test]
	fn a_kept_input_takes_its_steps_around_each_call() {
		let log = Log::default();
		let (before, after, called) = (log.clone(), log.clone(), log.clone());
		let mut sampler = kept(
			0u64,
			move |n: &mut u64| {
				*n += 1;
				called.borrow_mut().push("call");
				Dropped(called.clone())
			},
			step(move |_: &mut u64| before.borrow_mut().push("before")),
			step(move |_: &mut u64| after.borrow_mut().push("after")),
		);
		sampler.time(2);
		let each = ["before", "call", "drop", "after"];
		assert_eq!(*log.borrow(), [each, each].concat());
		log.borrow_mut().clear();
		sampler.count(&Meter::new("a", None).unwrap());
		assert_eq!(*log.borrow(), each);

		// Without steps, every output still drops before the next call, so a guard it holds never
		// blocks the next.
		let log = Log::default();
		let called = log.clone();
		let mut sampler = kept(
			(),
			move |_: &mut ()| {
				called.borrow_mut().push("call");
				Dropped(called.clone())
			},
			None,
			None,
		);
		sampler.time(2);
		assert_eq!(*log.borrow(), ["call", "drop", "call", "drop"]);
	}

	#[test]
	fn a_benchmark_that_times_itself_reports_its_own_time() {
		let mut sampler = kept_timed(0u64, |runs: &mut u64, iterations: Iterations| {
			*runs += 1;
			Duration::from_nanos(iterations.into_iter().map(|i| i + 1).sum())
		});
		assert_eq!(sampler.time(3), 6);
		let mut sampler = fresh_timed(
			|| 7u64,
			|seven: u64, iterations: Iterations| Duration::from_nanos(seven * iterations.get()),
		);
		assert_eq!(sampler.time(2), 14);
	}

	#[test]
	fn serves_a_sample_for_each_line() {
		let mut out = Vec::new();
		let sampler = kept_timed((), |_: &mut (), iterations: Iterations| {
			Duration::from_nanos(10 * iterations.get())
		});
		serve(sampler, "1\n4\n".as_bytes(), &mut out).unwrap();
		assert_eq!(
			String::from_utf8(out).unwrap(),
			"\nrust2-bench sample 10\n\nrust2-bench sample 40\n"
		);
		for bad in ["0\n", "x\n", " 1\n"] {
			let err = serve(nothing(), bad.as_bytes(), Vec::new()).unwrap_err();
			assert!(err.contains("cannot sample"), "{bad:?}: {err}");
		}
	}
}
