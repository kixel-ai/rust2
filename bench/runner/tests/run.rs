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

/// A timer setting, as `timerfd_settime` takes it.
#[repr(C)]
struct Itimerspec {
	interval: [i64; 2],
	value: [i64; 2],
}

unsafe extern "C" {
	fn timerfd_create(clock: i32, flags: i32) -> i32;
	fn timerfd_settime(fd: i32, flags: i32, new: *const Itimerspec, old: *mut Itimerspec) -> i32;
}

/// The fixture: runs each benchmark `FIXTURE` names as a loop of its iterations, between
/// the requests CodSpeed's hooks make. One whose name starts with `noisy` runs up to 2,000
/// more, as many as std's random `HashMap` seed and the clock choose. One whose name starts
/// with `abstimer` first arms a timer at an absolute time, repeating every 2^64 ns and more,
/// which the kernel takes though the span overflows a count of nanoseconds. One whose name
/// starts with `cwd` runs as many more as its working directory's path has bytes.
#[test]
#[ignore = "the program the other tests count"]
fn fixture() {
	let Ok(list) = std::env::var(FIXTURE) else { return };
	for bench in list.split(',').filter(|bench| !bench.is_empty()) {
		let (name, iterations) = bench.split_once('=').expect("name=iterations");
		let mut iterations: u64 = iterations.parse().expect("a count");
		if name.starts_with("noisy") {
			use std::hash::BuildHasher;
			let seeded = std::collections::hash_map::RandomState::new().hash_one(name) % 1000;
			let clock = std::time::SystemTime::now()
				.duration_since(std::time::UNIX_EPOCH)
				.unwrap()
				.subsec_nanos() as u64
				% 1000;
			iterations += seeded + clock;
		}
		if name.starts_with("cwd") {
			iterations += std::env::current_dir().unwrap().as_os_str().len() as u64;
		}
		if name.starts_with("abstimer") {
			const CLOCK_MONOTONIC: i32 = 1;
			const TFD_TIMER_ABSTIME: i32 = 1;
			let setting = Itimerspec {
				interval: [18_446_744_074, 0],
				value: [86_400, 50_000_000],
			};
			// SAFETY: a timerfd armed with a setting to read; nothing reads the timer.
			let armed = unsafe {
				timerfd_settime(
					timerfd_create(CLOCK_MONOTONIC, 0),
					TFD_TIMER_ABSTIME,
					&setting,
					std::ptr::null_mut(),
				)
			};
			assert_eq!(armed, 0, "the kernel takes the setting");
		}
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

/// The shell command that runs the fixture as a program counting `benches`, launched through
/// the command `wrap`, if any.
fn program(benches: &str, wrap: &str) -> String {
	let me = std::env::current_exe().unwrap();
	// Quoted for the shell, which may otherwise split the path.
	let me = me.to_str().unwrap().replace('\'', r"'\''");
	format!("{FIXTURE}={benches} {wrap} '{me}' --exact fixture --ignored --test-threads=1 -q")
}

/// rust2-bench, run in `dir` with `args`, counting the fixture as its one program, launched
/// through the command `wrap`, if any.
fn bench(dir: &Path, benches: &str, wrap: &str, args: &[&str]) -> Output {
	Command::new(env!("CARGO_BIN_EXE_rust2-bench"))
		.arg("run")
		.args(["--program", &program(benches, wrap)])
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
	let out = bench(&dir, "a=1000,b=2000", "", &["--out", "metrics.json"]);
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
	let again = bench(&dir, "a=1000,b=2000", "", &["--out", "again.json"]);
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
	let out = bench(
		&dir,
		"a=10,b=20",
		"",
		&["--benchmarks", "list", "--out", "metrics.json"],
	);
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
	let out = bench(&dir, "a=10", "", &["--benchmarks", "list"]);
	assert!(!out.status.success());
	let err = String::from_utf8_lossy(&out.stderr);
	assert!(err.contains("no benchmark is named suspect"), "{err}");
}

#[test]
fn fails_on_two_benchmarks_of_one_name() {
	let dir = scratch("twice");
	let out = bench(&dir, "a=10,a=20", "", &[]);
	assert!(!out.status.success());
	let err = String::from_utf8_lossy(&out.stderr);
	assert!(err.contains("two benchmarks are named a"), "{err}");
}

#[test]
fn fails_on_a_program_that_measures_nothing() {
	let dir = scratch("nothing");
	let out = bench(&dir, "", "", &[]);
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
		.env(
			"CARGO_TARGET_DIR",
			Path::new(env!("CARGO_TARGET_TMPDIR")).join("harness"),
		)
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
			"generic[4000]",
			"handed",
			"inner::nested[a]",
			"inner::nested[bc]",
			"sized[1000]",
			"sized[2000]",
			"sized[4000]",
			"spawned[100000]",
			"spawned[1000]",
			"unstable[1000]",
			"unstable[8000]",
			"word[word]"
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
		assert!(
			(0.9..1.1).contains(&ratio),
			"{name}: {} against {expected}",
			count(name)
		);
	};
	// Every one spins a thousand times but the larger sized ones; a setup's million spins and its
	// input's free would count a thousand times as much, were they counted.
	near("sized[1000]", alone);
	near("sized[2000]", 2 * alone);
	near("sized[4000]", 4 * alone);
	near("unstable[1000]", alone);
	near("unstable[8000]", 8 * alone);
	near("borrowed", alone);
	near("generic[4000]", alone);
	near("word[word]", alone);
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
	assert!(
		err.contains(&format!("no benchmark is named {file}sized[3000]")),
		"{err}"
	);
}

/// Each count and name of the table `run` prints.
fn table(stdout: &[u8]) -> std::collections::BTreeMap<String, u64> {
	String::from_utf8_lossy(stdout)
		.lines()
		.map_while(|line| {
			let (count, name) = line.trim_start().split_once("  ")?;
			Some((name.to_owned(), count.parse().ok()?))
		})
		.collect()
}

/// On a host that exposes perf counters, they count what Valgrind does, on every thread; on
/// one that exposes none, as a hosted CI runner, the run fails, saying so.
#[test]
fn counts_with_perf_counters_what_valgrind_does() {
	let out = harness(&["--perf"]);
	let err = String::from_utf8_lossy(&out.stderr);
	if !out.status.success() {
		assert!(err.contains("this host exposes no counters"), "{err}");
		return;
	}
	let perf = table(&out.stdout);
	let valgrind = harness(&[]);
	assert!(
		valgrind.status.success(),
		"{}",
		String::from_utf8_lossy(&valgrind.stderr)
	);
	let valgrind = table(&valgrind.stdout);
	assert_eq!(perf.keys().collect::<Vec<_>>(), valgrind.keys().collect::<Vec<_>>());
	assert_eq!(perf.len(), 14);
	let file = "harness/benches/fixture.rs::";
	let threaded = [
		format!("{file}handed"),
		format!("{file}spawned[100000]"),
		format!("{file}spawned[1000]"),
	];
	for (name, &count) in &perf {
		let under = valgrind[name];
		if threaded.contains(name) {
			// Starting a thread takes its own path under Valgrind's scheduler.
			let ratio = count as f64 / under as f64;
			assert!(
				(0.9..1.1).contains(&ratio),
				"{name}: {count} natively, {under} under Valgrind"
			);
		} else {
			// The same instructions, but for each bracket's own few dozen.
			assert!(
				count.abs_diff(under) < 100,
				"{name}: {count} natively, {under} under Valgrind"
			);
		}
	}
	// The other thread's spins count, the one setup started and the one the benchmark spawns.
	assert!(perf[&format!("{file}handed")] > 100 * perf[&format!("{file}alone")] * 9 / 10);
	assert!(
		perf[&format!("{file}spawned[100000]")] - perf[&format!("{file}spawned[1000]")]
			> 99 * perf[&format!("{file}alone")] * 9 / 10
	);
	let stdout = String::from_utf8_lossy(&out.stdout);
	assert!(
		stdout.ends_with(&format!(
			"\nOther threads ran during the perf counts of {}, which the scheduler makes noisy.\n",
			threaded.join(", ")
		)),
		"{stdout}"
	);
}

#[test]
fn repeats_a_count_that_reads_randomness_and_the_clock() {
	let dir = scratch("noisy");
	let count = |out: &str| {
		let run = bench(&dir, "noisy=1000", "", &["--out", out]);
		assert!(run.status.success(), "{}", String::from_utf8_lossy(&run.stderr));
		std::fs::read(dir.join(out)).unwrap()
	};
	// Each run would otherwise draw one of a million counts.
	assert_eq!(count("one.json"), count("two.json"));
}

#[test]
fn fails_on_a_process_that_never_loaded_the_preload() {
	let dir = scratch("bypass");
	let out = bench(&dir, "a=10", "env -u LD_PRELOAD", &[]);
	assert!(!out.status.success());
	let err = String::from_utf8_lossy(&out.stderr);
	assert!(err.contains("never loaded rust2-bench's preload"), "{err}");
}

#[test]
fn fails_on_a_timer_armed_at_an_absolute_time() {
	let dir = scratch("abstimer");
	let out = bench(&dir, "abstimer=10", "", &[]);
	assert!(
		!out.status.success(),
		"a timer the kernel reads on the host's clock was counted"
	);
}

/// Runs git in `dir` under a fixed identity, returning its stdout.
fn git(dir: &Path, args: &[&str]) -> String {
	let out = Command::new("git")
		.args([
			"-c",
			"user.name=test",
			"-c",
			"user.email=test@localhost",
			"-c",
			"commit.gpgsign=false",
		])
		.args(args)
		.current_dir(dir)
		.output()
		.unwrap();
	assert!(
		out.status.success(),
		"git {args:?}: {}",
		String::from_utf8_lossy(&out.stderr)
	);
	String::from_utf8(out.stdout).unwrap()
}

#[test]
fn compares_a_base_with_every_change_in_the_working_tree_from_one_directory() {
	let repo = scratch("compare");
	let cache = scratch("compare-cache");
	git(&repo, &["init", "--quiet"]);
	std::fs::write(repo.join("a.list"), "cwd=1000,a=1000,").unwrap();
	std::fs::write(repo.join("d.list"), "d=100,").unwrap();
	std::fs::write(repo.join(".gitignore"), "Cargo.lock\n").unwrap();
	git(&repo, &["add", "."]);
	git(&repo, &["commit", "--quiet", "-m", "base"]);
	// Changed but unstaged, staged only, untracked, and deleted.
	std::fs::write(repo.join("a.list"), "cwd=1000,a=2000,").unwrap();
	std::fs::write(repo.join("c.list"), "c=300,").unwrap();
	git(&repo, &["add", "c.list"]);
	std::fs::write(repo.join("b.list"), "b=500,").unwrap();
	std::fs::remove_file(repo.join("d.list")).unwrap();
	let status = git(&repo, &["status", "--porcelain"]);
	let out = Command::new(env!("CARGO_BIN_EXE_rust2-bench"))
		.args(["compare", "HEAD", "--program", &program("$(cat *.list)", "")])
		// Each side's build resolves its own ignored lockfile, never the side's before.
		.args(["--build", "! test -e Cargo.lock && touch Cargo.lock"])
		.current_dir(&repo)
		.env("XDG_CACHE_HOME", &cache)
		.output()
		.unwrap();
	assert!(out.status.success(), "{}", String::from_utf8_lossy(&out.stderr));
	let table = String::from_utf8(out.stdout).unwrap();
	let change = |name: &str| {
		let row = table
			.lines()
			.find(|row| row.starts_with(&format!("| {name} |")))
			.unwrap_or_else(|| panic!("no {name} in {table}"));
		row.rsplit('|').nth(1).unwrap().trim().to_owned()
	};
	// Both sides count from one working directory, whose path is in the count.
	assert_eq!(change("cwd"), "+0.00%");
	let a = change("a");
	assert!(a.starts_with('+') && a != "+0.00%", "{table}");
	assert_eq!(change("b"), "added");
	assert_eq!(change("c"), "added");
	assert_eq!(change("d"), "removed");
	// The caller's index, working tree, and stash are as they were.
	assert_eq!(git(&repo, &["status", "--porcelain"]), status);
	assert_eq!(git(&repo, &["stash", "list"]), "");
}
