//! Benchmarks as plain functions marked `#[bench]`, which [rust2-bench] counts in
//! instructions under Valgrind. See the README.
//!
//! [rust2-bench]: https://github.com/kixel-ai/rust2/tree/main/bench/runner

use std::collections::BTreeMap;

/// Marks a function as a benchmark: `#[bench]`, `#[bench(args = [..])]`, `#[bench(setup = f)]`,
/// or both keys. See the README for what each passes the function.
pub use rust2_bench_macros::bench;

/// The variable naming the one benchmark a run counts, which rust2-bench sets on each job.
const ONLY: &str = "RUST2_BENCH";

/// What a run prints after each benchmark, before its name, which rust2-bench lists them by.
const RAN: &str = "rust2-bench ran ";

/// The variable that, set to a PMU's perf type (0 on a host with one), counts each benchmark's
/// user-space instructions with Linux perf counters, natively.
const PERF: &str = "RUST2_BENCH_PERF";

/// What a perf count prints before each benchmark's count, `alone` or `threads`, and name.
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
/// `RUST2_BENCH` names, each counted alone under Valgrind and once natively. Call it from a
/// benchmark target's `main`.
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
	match std::env::var(ONLY) {
		Ok(only) => {
			let case = cases
				.remove(&only)
				.ok_or(format!("{ONLY} names {only}, which no benchmark is"))?;
			cases = BTreeMap::from([(only, case)]);
		}
		Err(std::env::VarError::NotPresent) => {}
		Err(std::env::VarError::NotUnicode(_)) => return Err(format!("{ONLY} is not UTF-8")),
	}
	let perf = match std::env::var(PERF) {
		Ok(kind) => Some(
			kind.parse::<u32>()
				.map_err(|_| format!("{PERF} is {kind}, not a PMU's perf type"))?,
		),
		Err(std::env::VarError::NotPresent) => None,
		Err(std::env::VarError::NotUnicode(_)) => return Err(format!("{PERF} is not UTF-8")),
	};
	for (name, case) in cases {
		let meter = __private::Meter::new(&name, perf)?;
		(case.run)(&meter);
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

	pub use core::hint::black_box;
	pub use inventory;

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

	/// One case of a benchmark, ready to run.
	pub struct Case {
		/// Its argument, shown, when the benchmark declares args.
		pub(crate) arg: Option<String>,
		/// Runs it, counting what `Meter` brackets.
		pub(crate) run: Box<dyn FnOnce(&Meter)>,
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

	/// Whether `path`, a `file!()`, is relative, so a benchmark's name holds no host's path.
	pub const fn relative(path: &str) -> bool {
		match path.as_bytes() {
			[b'/' | b'\\', ..] => false,
			// A Windows drive.
			[_, b':', ..] => false,
			_ => true,
		}
	}

	/// The one case of a benchmark without args.
	pub fn single(run: impl FnOnce(&Meter) + 'static) -> Vec<Case> {
		vec![Case {
			arg: None,
			run: Box::new(run),
		}]
	}

	/// A case per argument among `args`, shown, each moving its argument into the case `make`
	/// builds from it. `make`'s body gives the argument the type it runs with.
	pub fn each<A: Display, R: FnOnce(&Meter) + 'static>(
		args: impl IntoIterator<Item = A>,
		make: impl Fn(A) -> R,
	) -> Vec<Case> {
		args.into_iter()
			.map(|arg| Case {
				arg: Some(arg.to_string()),
				run: Box::new(make(arg)),
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
	use super::__private::{Bench, Meter, each, relative, single};
	use super::*;

	fn bench(module: &'static str, name: &'static str) -> Bench {
		Bench {
			file: "benches/a.rs",
			module,
			name,
			cases: || single(|_| {}),
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
				cases: || each(["16", "1024"], |_| |_: &Meter| {}),
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
			cases: || each([1, 1], |_| |_: &Meter| {}),
		}];
		assert_eq!(
			cases(&twice).err().unwrap(),
			"two benchmarks are named benches/a.rs::f[1]"
		);
		let lines = [Bench {
			file: "benches/a.rs",
			module: "a",
			name: "f",
			cases: || each(["a\nb"], |_| |_: &Meter| {}),
		}];
		assert!(cases(&lines).err().unwrap().contains("control character"));
		let empty = [Bench {
			file: "benches/a.rs",
			module: "a",
			name: "f",
			cases: || each(Vec::<u8>::new(), |_| |_: &Meter| {}),
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
}
