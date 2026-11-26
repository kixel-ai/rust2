//! rust2-bench's wall-clock times end to end: the harness's fixture timed alone, and a scratch
//! repository's benchmarks paired with its base commit's. Times move with the host's load, so
//! these check what a time is, and only a difference far past any load's.

#![cfg(unix)]

use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use serde_json::Value;

fn scratch(name: &str) -> PathBuf {
	let dir = Path::new(env!("CARGO_TARGET_TMPDIR")).join(name);
	let _ = std::fs::remove_dir_all(&dir);
	std::fs::create_dir_all(&dir).unwrap();
	dir
}

fn succeeded(out: &Output) {
	assert!(
		out.status.success(),
		"{}\n{}",
		String::from_utf8_lossy(&out.stdout),
		String::from_utf8_lossy(&out.stderr)
	);
}

/// The one measure a `metrics.json` records, which must be a time, and the file.
fn read(path: &Path) -> (String, Value) {
	let metrics: Value = serde_json::from_str(&std::fs::read_to_string(path).unwrap()).unwrap();
	let units = metrics["units"].as_object().unwrap();
	assert_eq!(units.len(), 1);
	let (measure, unit) = units.iter().next().unwrap();
	assert_eq!(unit, "ns");
	assert!(
		measure.starts_with(&format!("time ({}-", std::env::consts::ARCH)),
		"{measure}"
	);
	(measure.clone(), metrics)
}

/// Each benchmark's time in a sample, a bare value: a time of one side has no interval.
fn times(sample: &Value, measure: &str) -> Vec<(String, f64)> {
	sample["metrics"]
		.as_object()
		.unwrap()
		.iter()
		.map(|(name, measures)| {
			let time = measures[measure].as_object().unwrap();
			assert_eq!(time.len(), 1, "{name}: {time:?}");
			let value = time["value"].as_f64().unwrap();
			assert!(value > 0.0, "{name}: {value}");
			(name.clone(), value)
		})
		.collect()
}

#[test]
fn times_each_harness_benchmark() {
	let dir = scratch("time-alone");
	let out = Command::new(env!("CARGO_BIN_EXE_rust2-bench"))
		.args(["time", "--out"])
		.arg(dir.join("metrics.json"))
		.args(["--", "-p", "rust2-bench-harness", "--bench", "fixture"])
		.current_dir(env!("CARGO_MANIFEST_DIR"))
		// The target directory the count tests build the fixture in, apart from this test's own.
		.env(
			"CARGO_TARGET_DIR",
			Path::new(env!("CARGO_TARGET_TMPDIR")).join("harness"),
		)
		.output()
		.unwrap();
	succeeded(&out);
	let (measure, metrics) = read(&dir.join("metrics.json"));
	let samples = metrics["samples"].as_array().unwrap();
	assert_eq!(samples.len(), 1);
	assert!(samples[0].get("commit").is_none());
	let times = times(&samples[0], &measure);
	let file = "harness/benches/fixture.rs::";
	let names: Vec<&str> = times.iter().map(|(name, _)| name.strip_prefix(file).unwrap()).collect();
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
			"stepped",
			"timed",
			"unstable[1000]",
			"unstable[8000]",
			"word[word]"
		]
	);
	let time = |name: &str| times.iter().find(|(n, _)| n == &format!("{file}{name}")).unwrap().1;
	// Eight times the spins, which no load makes look like under twice.
	assert!(time("unstable[8000]") > 2.0 * time("unstable[1000]"), "{times:?}");
	let table = String::from_utf8(out.stdout).unwrap();
	assert!(table.starts_with("| benchmark | time |\n"), "{table}");
	assert_eq!(table.lines().count(), 2 + names.len(), "{table}");
}

/// A benchmark target spinning `work` times in `work`, a thousand in `same`, and with `extra`,
/// a benchmark of that name.
fn spins(work: u64, extra: &str) -> String {
	format!(
		"use std::hint::black_box;\n\
		 use rust2_bench_harness::bench;\n\
		 fn spin(n: u64) -> u64 {{ let mut sum = 0u64; for i in 0..n {{ sum = black_box(sum.wrapping_add(i)); }} sum }}\n\
		 #[bench] fn work() -> u64 {{ spin({work}) }}\n\
		 #[bench] fn same() -> u64 {{ spin(1000) }}\n\
		 #[bench] fn {extra}() -> u64 {{ spin(1000) }}\n\
		 fn main() {{ rust2_bench_harness::main(); }}\n"
	)
}

fn git(dir: &Path, args: &[&str]) -> String {
	let out = Command::new("git").args(args).current_dir(dir).output().unwrap();
	succeeded(&out);
	String::from_utf8(out.stdout).unwrap().trim().to_owned()
}

#[test]
fn pairs_a_base_commit_with_the_working_tree() {
	let dir = scratch("time-paired");
	let repo = dir.join("repo");
	std::fs::create_dir_all(repo.join("benches")).unwrap();
	let harness = Path::new(env!("CARGO_MANIFEST_DIR")).join("harness");
	std::fs::write(
		repo.join("Cargo.toml"),
		format!(
			"[package]\nname = \"paired\"\nversion = \"0.0.0\"\nedition = \"2024\"\npublish = false\n\n[workspace]\n\n\
			 [dev-dependencies]\nrust2-bench-harness = {{ path = {:?} }}\n\n[[bench]]\nname = \"spin\"\nharness = false\n",
			harness.to_str().unwrap()
		),
	)
	.unwrap();
	// The runner's own lock, so the scratch workspace resolves the harness's dependencies offline.
	std::fs::copy(
		Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.lock"),
		repo.join("Cargo.lock"),
	)
	.unwrap();
	std::fs::write(repo.join("benches/spin.rs"), spins(1000, "gone")).unwrap();
	std::fs::write(repo.join(".gitignore"), "/target\n").unwrap();
	git(&repo, &["init", "--quiet"]);
	git(&repo, &["add", "."]);
	git(
		&repo,
		&[
			"-c",
			"user.name=t",
			"-c",
			"user.email=t@t",
			"-c",
			"commit.gpgsign=false",
			"commit",
			"--quiet",
			"-m",
			"base",
		],
	);
	let base = git(&repo, &["rev-parse", "HEAD"]);
	// The working tree spins twice as long in `work`, and trades `gone` for `new`.
	std::fs::write(repo.join("benches/spin.rs"), spins(2000, "new")).unwrap();

	let out = Command::new(env!("CARGO_BIN_EXE_rust2-bench"))
		.args(["time", "HEAD", "--out", "metrics.json"])
		.current_dir(&repo)
		// The base's worktree, outside the scratch repository, and one target directory both
		// sides share, so the harness builds once.
		.env("XDG_CACHE_HOME", dir.join("cache"))
		.env("CARGO_TARGET_DIR", dir.join("target"))
		.env("CARGO_NET_OFFLINE", "true")
		.output()
		.unwrap();
	succeeded(&out);
	let (measure, metrics) = read(&repo.join("metrics.json"));
	let samples = metrics["samples"].as_array().unwrap();
	assert_eq!(samples.len(), 2);
	assert_eq!(samples[0]["commit"], base);
	assert!(samples[1].get("commit").is_none());
	let (before, after) = (times(&samples[0], &measure), times(&samples[1], &measure));
	let names = |times: &[(String, f64)]| times.iter().map(|(name, _)| name.clone()).collect::<Vec<_>>();
	assert_eq!(
		names(&before),
		[
			"benches/spin.rs::gone",
			"benches/spin.rs::same",
			"benches/spin.rs::work"
		]
	);
	assert_eq!(
		names(&after),
		["benches/spin.rs::new", "benches/spin.rs::same", "benches/spin.rs::work"]
	);
	// Twice the spins, which no load makes look like under half again.
	assert!(after[2].1 > 1.5 * before[2].1, "{before:?} {after:?}");
	let table = String::from_utf8(out.stdout).unwrap();
	let row = |name: &str| {
		table
			.lines()
			.find(|line| line.starts_with(&format!("| benches/spin.rs::{name} |")))
			.unwrap_or_else(|| panic!("{table}"))
			.to_owned()
	};
	assert!(row("gone").contains("| removed |"), "{table}");
	assert!(row("new").contains("| added |"), "{table}");
	// `+NN.NN%`, then its interval.
	let change: f64 = row("work")
		.split('|')
		.nth(4)
		.unwrap()
		.trim()
		.trim_end_matches('%')
		.parse()
		.unwrap();
	assert!(change > 50.0, "{table}");
}

/// Writes a workspace package `name` under `repo` with one harness benchmark target, `bench`.
fn package(repo: &Path, name: &str, bench: &str) {
	let dir = repo.join(name);
	std::fs::create_dir_all(dir.join("benches")).unwrap();
	std::fs::create_dir_all(dir.join("src")).unwrap();
	let harness = Path::new(env!("CARGO_MANIFEST_DIR")).join("harness");
	std::fs::write(
		dir.join("Cargo.toml"),
		format!(
			"[package]\nname = \"{name}\"\nedition = \"2024\"\npublish = false\n\n\
			 [dev-dependencies]\nrust2-bench-harness = {{ path = {harness:?} }}\n\n\
			 [[bench]]\nname = \"{bench}\"\nharness = false\n"
		),
	)
	.unwrap();
	std::fs::write(dir.join("src/lib.rs"), "").unwrap();
	std::fs::write(dir.join("benches").join(format!("{bench}.rs")), spins(1000, "other")).unwrap();
}

#[test]
fn times_a_selection_only_one_side_has() {
	let dir = scratch("time-select");
	let repo = dir.join("repo");
	std::fs::create_dir_all(&repo).unwrap();
	std::fs::copy(
		Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.lock"),
		repo.join("Cargo.lock"),
	)
	.unwrap();
	std::fs::write(repo.join(".gitignore"), "/target\n").unwrap();
	std::fs::write(
		repo.join("Cargo.toml"),
		"[workspace]\nmembers = [\"a\"]\nresolver = \"3\"\n",
	)
	.unwrap();
	package(&repo, "a", "x");
	git(&repo, &["init", "--quiet"]);
	git(&repo, &["add", "."]);
	git(
		&repo,
		&[
			"-c",
			"user.name=t",
			"-c",
			"user.email=t@t",
			"-c",
			"commit.gpgsign=false",
			"commit",
			"--quiet",
			"-m",
			"base",
		],
	);
	// The head adds a package, which a run names before committing it.
	std::fs::write(
		repo.join("Cargo.toml"),
		"[workspace]\nmembers = [\"a\", \"b\"]\nresolver = \"3\"\n",
	)
	.unwrap();
	package(&repo, "b", "y");
	let time = |cargo: &[&str]| {
		Command::new(env!("CARGO_BIN_EXE_rust2-bench"))
			.args(["time", "HEAD", "--"])
			.args(cargo)
			.current_dir(&repo)
			.env("XDG_CACHE_HOME", dir.join("cache"))
			.env("CARGO_TARGET_DIR", dir.join("target"))
			.env("CARGO_NET_OFFLINE", "true")
			.output()
			.unwrap()
	};

	let out = time(&["-p", "a", "--package=b", "--bench", "x", "--bench=y"]);
	let err = String::from_utf8_lossy(&out.stderr).into_owned();
	assert!(out.status.success(), "{err}");
	assert!(err.contains("HEAD has no -p b"), "{err}");
	assert!(err.contains("HEAD has no --bench y"), "{err}");
	let table = String::from_utf8(out.stdout).unwrap();
	// Each package's three benchmarks, `b`'s only at the head.
	assert_eq!(table.lines().count(), 2 + 6, "{table}");
	for row in table.lines().skip(2) {
		assert_eq!(
			row.contains("| added |"),
			row.starts_with("| b/benches/y.rs::"),
			"{table}"
		);
	}

	// The head's package alone: the base times nothing, rather than every benchmark.
	let out = time(&["-p", "b"]);
	let err = String::from_utf8_lossy(&out.stderr).into_owned();
	assert!(out.status.success(), "{err}");
	assert!(
		err.contains("HEAD has nothing the cargo args select, so times none"),
		"{err}"
	);
	let table = String::from_utf8(out.stdout).unwrap();
	assert_eq!(table.matches("| added |").count(), 3, "{table}");
	assert_eq!(table.lines().count(), 2 + 3, "{table}");

	// A selector neither side has still fails, before either side builds.
	let out = time(&["-p", "a", "-p", "c"]);
	let err = String::from_utf8_lossy(&out.stderr);
	assert!(!out.status.success());
	assert!(err.contains("neither HEAD nor the working tree has -p c"), "{err}");
	assert!(!err.contains("building"), "{err}");
}
