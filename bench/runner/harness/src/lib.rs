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
	for (name, (bench, case)) in cases {
		let meter = __private::Meter::new(&name)?;
		(bench.run)(case, &meter);
		if !quiet {
			println!("{RAN}{name}");
		}
	}
	Ok(())
}

/// Each case of `benches` by its name, with its benchmark and argument's index.
fn cases<'a>(
	benches: impl IntoIterator<Item = &'a __private::Bench>,
) -> Result<BTreeMap<String, (&'a __private::Bench, usize)>, String> {
	let mut cases = BTreeMap::new();
	for bench in benches {
		let base = bench.base();
		let names = match bench.args {
			None => vec![base],
			Some(args) => {
				let args = args();
				if args.is_empty() {
					return Err(format!("{base} has no args"));
				}
				args.iter().map(|arg| format!("{base}[{arg}]")).collect()
			}
		};
		for (case, name) in names.into_iter().enumerate() {
			// rust2-bench reads names a line each, and Kixel Quest takes none holding one.
			if name.chars().any(char::is_control) {
				return Err(format!(
					"{name:?} holds a control character, which no benchmark's name may"
				));
			}
			if cases.insert(name.clone(), (bench, case)).is_some() {
				return Err(format!("two benchmarks are named {name}"));
			}
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
		/// Each case's argument, shown, when it declares args.
		pub args: Option<fn() -> Vec<String>>,
		/// Runs one case, by its argument's index, counting what `Meter` brackets.
		pub run: fn(usize, &Meter),
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
	}

	impl Meter {
		pub(crate) fn new(name: &str) -> Result<Self, String> {
			let name = CString::new(name).map_err(|_| format!("{name:?} holds a NUL"))?;
			Ok(Self { name })
		}

		/// Zeroes the counts and starts counting.
		#[inline(always)]
		pub fn start(&self) {
			crate::valgrind::request(crate::valgrind::ZERO_STATS, 0);
			crate::valgrind::request(crate::valgrind::START_INSTRUMENTATION, 0);
		}

		/// Stops counting and dumps the count under the case's name.
		#[inline(always)]
		pub fn stop(&self) {
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

	/// The argument at `case` among `args`.
	pub fn arg<I: IntoIterator>(args: I, case: usize) -> I::Item {
		args.into_iter()
			.nth(case)
			.expect("args yields what it yielded when the cases were listed")
	}

	/// Each argument among `args`, shown. `_takes` is the function the argument goes to, never
	/// called: it gives each argument the type it has when its case runs.
	pub fn names<A: Display, R>(args: impl IntoIterator<Item = A>, _takes: impl FnOnce(A) -> R) -> Vec<String> {
		args.into_iter().map(|arg| arg.to_string()).collect()
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
	use super::__private::{Bench, Meter, relative};
	use super::*;

	fn bench(module: &'static str, name: &'static str, args: Option<fn() -> Vec<String>>) -> Bench {
		Bench {
			file: "benches/a.rs",
			module,
			name,
			args,
			run: |_, _| {},
		}
	}

	#[test]
	fn names_each_case_by_file_module_function_and_argument() {
		let benches = [
			bench("a", "top", None),
			bench("a::inner::deeper", "nested", None),
			bench("a", "sized", Some(|| vec!["16".to_owned(), "1024".to_owned()])),
		];
		let cases = cases(&benches).unwrap();
		let names: Vec<(&str, usize)> = cases.iter().map(|(name, (_, case))| (name.as_str(), *case)).collect();
		assert_eq!(
			names,
			[
				("benches/a.rs::inner::deeper::nested", 0),
				("benches/a.rs::sized[1024]", 1),
				("benches/a.rs::sized[16]", 0),
				("benches/a.rs::top", 0),
			]
		);
	}

	#[test]
	fn refuses_two_cases_of_one_name_and_args_of_none() {
		let twice = [bench("a", "f", Some(|| vec!["1".to_owned(), "1".to_owned()]))];
		assert_eq!(
			cases(&twice).err().unwrap(),
			"two benchmarks are named benches/a.rs::f[1]"
		);
		let lines = [bench("a", "f", Some(|| vec!["a\nb".to_owned()]))];
		assert!(cases(&lines).err().unwrap().contains("control character"));
		let empty = [bench("a", "f", Some(Vec::new))];
		assert_eq!(cases(&empty).err().unwrap(), "benches/a.rs::f has no args");
		assert_eq!(cases(&[]).err().unwrap(), "no function is marked #[bench]");
	}

	#[test]
	fn a_name_with_a_nul_is_refused() {
		assert!(Meter::new("a\0b").is_err());
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
