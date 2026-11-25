//! rust2-bench end to end, under the Valgrind on PATH: this test binary is also the program it
//! counts, which brackets each benchmark with Callgrind's client requests as CodSpeed's
//! instrument hooks do.

// Valgrind, and the fingerprint, are x86_64 Linux's here.
#![cfg(all(target_os = "linux", target_arch = "x86_64"))]

use std::path::Path;
use std::process::{Command, Output};

/// The benchmarks the fixture runs, `name=iterations` comma-separated, when it is the program.
const FIXTURE: &str = "RUST2_BENCH_FIXTURE";

/// Callgrind's client request codes, from valgrind/callgrind.h.
const BASE: u64 = (b'C' as u64) << 24 | (b'T' as u64) << 16;
const DUMP_STATS_AT: u64 = BASE + 3;
const ZERO_STATS: u64 = BASE + 1;
const START_INSTRUMENTATION: u64 = BASE + 4;
const STOP_INSTRUMENTATION: u64 = BASE + 5;

/// A Valgrind client request: Valgrind's magic preamble, a no-op on the CPU, then the request.
fn request(code: u64, arg: u64) {
	let args = [code, arg, 0, 0, 0, 0];
	// SAFETY: the rotations sum to 128, leaving rdi as it was, and `xchg rbx, rbx` changes
	// nothing; under Valgrind, the request reads `args` and writes rdx.
	unsafe {
		std::arch::asm!(
			"rol rdi, 3", "rol rdi, 13", "rol rdi, 61", "rol rdi, 51", "xchg rbx, rbx",
			in("rax") args.as_ptr(),
			inout("rdx") 0u64 => _,
			options(nostack),
		);
	}
}

/// The fixture: runs each benchmark `FIXTURE` names as a loop of its iterations, between
/// the requests CodSpeed's hooks make.
#[test]
#[ignore = "the program the other tests count"]
fn fixture() {
	let Ok(list) = std::env::var(FIXTURE) else { return };
	for bench in list.split(',').filter(|bench| !bench.is_empty()) {
		let (name, iterations) = bench.split_once('=').expect("name=iterations");
		let iterations: u64 = iterations.parse().expect("a count");
		request(ZERO_STATS, 0);
		request(START_INSTRUMENTATION, 0);
		let mut sum = 0u64;
		for i in 0..iterations {
			sum = std::hint::black_box(sum.wrapping_add(i));
		}
		request(STOP_INSTRUMENTATION, 0);
		let name = std::ffi::CString::new(name).expect("no NUL");
		request(DUMP_STATS_AT, name.as_ptr() as u64);
	}
}

/// rust2-bench, run in `dir` with `args`, counting the fixture as its one program.
fn bench(dir: &Path, benches: &str, args: &[&str]) -> Output {
	let me = std::env::current_exe().unwrap();
	// Quoted for the shell, which may otherwise split the path.
	let me = me.to_str().unwrap().replace('\'', r"'\''");
	let program = format!("{FIXTURE}={benches} '{me}' --exact fixture --ignored --test-threads=1 -q");
	Command::new(env!("CARGO_BIN_EXE_rust2-bench"))
		.arg("run")
		.args(["--program", &program])
		.args(args)
		.current_dir(dir)
		.output()
		.unwrap()
}

fn scratch(name: &str) -> std::path::PathBuf {
	let dir = Path::new(env!("CARGO_TARGET_TMPDIR")).join(name);
	let _ = std::fs::remove_dir_all(&dir);
	std::fs::create_dir_all(&dir).unwrap();
	dir
}

#[test]
fn counts_each_benchmark_a_program_brackets() {
	let dir = scratch("counts");
	let out = bench(&dir, "a=1000,b=2000", &["--out", "metrics.json"]);
	assert!(out.status.success(), "{}", String::from_utf8_lossy(&out.stderr));
	let metrics: serde_json::Value =
		serde_json::from_str(&std::fs::read_to_string(dir.join("metrics.json")).unwrap()).unwrap();
	let units = metrics["units"].as_object().unwrap();
	assert_eq!(units.len(), 1);
	let (measure, unit) = units.iter().next().unwrap();
	assert_eq!(unit, "instructions");
	assert!(measure.starts_with("instructions (x86_64-"), "{measure}");
	let count = |name: &str| {
		metrics["samples"][0]["metrics"][name][measure]["value"]
			.as_u64()
			.unwrap()
	};
	let (a, b) = (count("a"), count("b"));
	// The loop dominates each count, so twice the iterations is near twice the count.
	assert!(a > 1000 && b > 1000, "{a} {b}");
	let ratio = b as f64 / a as f64;
	assert!((1.9..2.1).contains(&ratio), "{a} {b}");
	// The same program counts the same again.
	let again = bench(&dir, "a=1000,b=2000", &["--out", "again.json"]);
	assert!(again.status.success(), "{}", String::from_utf8_lossy(&again.stderr));
	assert_eq!(
		std::fs::read(dir.join("metrics.json")).unwrap(),
		std::fs::read(dir.join("again.json")).unwrap()
	);
}

#[test]
fn counts_only_the_benchmarks_a_list_names() {
	let dir = scratch("listed");
	std::fs::write(dir.join("list"), "b\n").unwrap();
	let out = bench(&dir, "a=10,b=20", &["--benchmarks", "list", "--out", "metrics.json"]);
	assert!(out.status.success(), "{}", String::from_utf8_lossy(&out.stderr));
	let metrics: serde_json::Value =
		serde_json::from_str(&std::fs::read_to_string(dir.join("metrics.json")).unwrap()).unwrap();
	let names: Vec<&String> = metrics["samples"][0]["metrics"].as_object().unwrap().keys().collect();
	assert_eq!(names, ["b"]);
}

#[test]
fn fails_on_a_listed_benchmark_no_program_has() {
	let dir = scratch("missing");
	std::fs::write(dir.join("list"), "a\nsuspect\n").unwrap();
	let out = bench(&dir, "a=10", &["--benchmarks", "list"]);
	assert!(!out.status.success());
	let err = String::from_utf8_lossy(&out.stderr);
	assert!(err.contains("no benchmark is named suspect"), "{err}");
}

#[test]
fn fails_on_two_benchmarks_of_one_name() {
	let dir = scratch("twice");
	let out = bench(&dir, "a=10,a=20", &[]);
	assert!(!out.status.success());
	let err = String::from_utf8_lossy(&out.stderr);
	assert!(err.contains("two benchmarks are named a"), "{err}");
}

#[test]
fn fails_on_a_program_that_measures_nothing() {
	let dir = scratch("nothing");
	let out = bench(&dir, "", &[]);
	assert!(!out.status.success());
	let err = String::from_utf8_lossy(&out.stderr);
	assert!(err.contains("measured no benchmark"), "{err}");
}

/// rust2-bench, run in this workspace with `args` on the harness's fixture benchmark target,
/// built in a target directory of its own, since this test's build holds the workspace's.
fn harness(args: &[&str]) -> Output {
	Command::new(env!("CARGO_BIN_EXE_rust2-bench"))
		.arg("run")
		.args(args)
		.args(["--", "-p", "rust2-bench-harness", "--bench", "fixture"])
		.current_dir(env!("CARGO_MANIFEST_DIR"))
		.env("CARGO_TARGET_DIR", Path::new(env!("CARGO_TARGET_TMPDIR")).join("harness"))
		.output()
		.unwrap()
}

#[test]
fn counts_each_harness_benchmark_alone_without_its_setup() {
	let dir = scratch("harness");
	let metrics = dir.join("metrics.json");
	let out = harness(&["--out", metrics.to_str().unwrap()]);
	assert!(out.status.success(), "{}", String::from_utf8_lossy(&out.stderr));
	let metrics: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(metrics).unwrap()).unwrap();
	let metrics = metrics["samples"][0]["metrics"].as_object().unwrap();
	let file = "harness/benches/fixture.rs::";
	let names: Vec<&str> = metrics.keys().map(|name| name.strip_prefix(file).unwrap()).collect();
	assert_eq!(
		names,
		[
			"alone",
			"borrowed",
			"inner::nested[a]",
			"inner::nested[bc]",
			"sized[1000]",
			"sized[2000]"
		]
	);
	let count = |name: &str| {
		let measures = metrics[&format!("{file}{name}")].as_object().unwrap();
		measures.values().next().unwrap()["value"].as_u64().unwrap()
	};
	let alone = count("alone");
	assert!(alone > 1000, "{alone}");
	let near = |name: &str, expected: u64| {
		let ratio = count(name) as f64 / expected as f64;
		assert!((0.9..1.1).contains(&ratio), "{name}: {} against {expected}", count(name));
	};
	// Every one spins a thousand times but sized[2000]; a setup's million spins and its
	// input's free would count a thousand times as much, were they counted.
	near("sized[1000]", alone);
	near("sized[2000]", 2 * alone);
	near("borrowed", alone);
	near("inner::nested[a]", alone);
	near("inner::nested[bc]", alone);

	// A list counts only what it names, and fails, before counting any, on a name none has.
	std::fs::write(dir.join("list"), format!("{file}sized[2000]\n")).unwrap();
	let only = dir.join("only.json");
	let out = harness(&[
		"--benchmarks",
		dir.join("list").to_str().unwrap(),
		"--out",
		only.to_str().unwrap(),
	]);
	assert!(out.status.success(), "{}", String::from_utf8_lossy(&out.stderr));
	let only: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(only).unwrap()).unwrap();
	let only = only["samples"][0]["metrics"].as_object().unwrap();
	assert_eq!(only.keys().collect::<Vec<_>>(), [&format!("{file}sized[2000]")]);
	std::fs::write(dir.join("list"), format!("{file}sized[3000]\n")).unwrap();
	let out = harness(&["--benchmarks", dir.join("list").to_str().unwrap()]);
	assert!(!out.status.success());
	let err = String::from_utf8_lossy(&out.stderr);
	assert!(err.contains(&format!("no benchmark is named {file}sized[3000]")), "{err}");
}
