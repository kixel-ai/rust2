//! rust2-lint's mono budget on the fixture workspace in `fixtures/mono`, where `user`
//! instantiates each of `exec`'s generics with 20 closure types.

use std::path::{Path, PathBuf};
use std::process::{Command, Output};

fn fixture() -> PathBuf {
	PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("fixtures").join("mono")
}

/// An empty target directory for the test `name`: cargo can't tell a rebuilt rust2-lint
/// from the one that built what it keeps, so a kept build would skip the lint.
fn target(name: &str) -> PathBuf {
	let dir = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("mono").join(name);
	match std::fs::remove_dir_all(&dir) {
		Err(err) if err.kind() != std::io::ErrorKind::NotFound => panic!("removing {}: {err}", dir.display()),
		_ => dir,
	}
}

/// rust2-lint in the fixture workspace with `args`, into the target directory `target`.
fn command(target: &Path, args: &[&str]) -> Command {
	let mut command = Command::new(env!("CARGO_BIN_EXE_rust2-lint"));
	command
		.args(args)
		.current_dir(fixture())
		.env("CARGO_TARGET_DIR", target);
	command
}

fn lint(target: &Path, args: &[&str]) -> Output {
	command(target, args).output().expect("running rust2-lint")
}

fn stderr(output: &Output) -> String {
	String::from_utf8_lossy(&output.stderr).into_owned()
}

/// The line `needle` starts in `file` of the fixture, from 1.
fn line_of(file: &str, needle: &str) -> usize {
	let text = std::fs::read_to_string(fixture().join(file)).unwrap();
	text.lines().position(|line| line.starts_with(needle)).expect(needle) + 1
}

#[test]
fn fat_generic_fails_at_its_span_with_its_callers() {
	let build = ["--", "build", "--package", "user", "--features", "fat"];
	let target = target("fat");
	let passed = lint(&target, &[&["--mono-budget", "1000000"][..], &build].concat());
	assert!(passed.status.success(), "{}", stderr(&passed));

	// A new budget rebuilds the crate, which was fresh under the old one.
	let failed = lint(&target, &[&["--mono-budget", "1000"][..], &build].concat());
	let err = stderr(&failed);
	assert!(!failed.status.success(), "{err}");
	// Its async body is its cost: the function alone only makes the future.
	assert!(err.contains("error: `exec::fat` costs "), "{err}");
	assert!(err.contains(" in 21 copies of up to "), "{err}");
	let at = format!(
		"--> exec/src/lib.rs:{}:1",
		line_of("exec/src/lib.rs", "pub async fn fat")
	);
	assert!(err.contains(&at), "{err}");
	// The heaviest caller first.
	let fat = err.find("note: `fat` instantiates 20").expect(&err);
	let once = err.find("note: `once` instantiates 1").expect(&err);
	assert!(fat < once, "{err}");
	assert_eq!(err.matches("error: `").count(), 1, "only `fat` is over: {err}");
}

#[test]
fn forwarder_and_own_budget_pass() {
	// A forwarder passes any budget; `budgeted` sets its own.
	let output = lint(
		&target("pass"),
		&[
			"--mono-budget",
			"1",
			"--",
			"build",
			"--package",
			"user",
			"--features",
			"thin,budgeted",
		],
	);
	assert!(output.status.success(), "{}", stderr(&output));
}

#[test]
fn one_fat_copy_among_thin_ones_fails() {
	// Only `uneven::<Yes, 0>` instantiates its closure, so the average copy is thin.
	let build = ["--", "build", "--package", "user", "--features", "uneven"];
	let output = lint(&target("uneven"), &[&["--mono-budget", "100"][..], &build].concat());
	let err = stderr(&output);
	assert!(!output.status.success(), "{err}");
	assert!(err.contains("error: `exec::uneven` costs "), "{err}");
	assert!(err.contains(" in 21 copies of up to "), "{err}");
}

#[test]
fn a_local_dependency_is_checked() {
	// `dependent` builds `user` with `fat` as a dependency, which a later command could
	// select and cargo would not rebuild.
	let output = lint(
		&target("dependency"),
		&["--mono-budget", "1000", "--", "build", "--package", "dependent"],
	);
	let err = stderr(&output);
	assert!(!output.status.success(), "{err}");
	assert!(err.contains("error: `exec::fat` costs "), "{err}");
	assert!(err.contains("could not compile `user`"), "{err}");
}

#[test]
fn a_lint_cap_does_not_skip_the_check() {
	// Cargo caps a registry dependency's lints, but RUSTFLAGS can cap a local crate's too.
	let output = command(
		&target("cap"),
		&[
			"--mono-budget",
			"1000",
			"--",
			"build",
			"--package",
			"user",
			"--features",
			"fat",
		],
	)
	.env("RUSTFLAGS", "--cap-lints=warn")
	.output()
	.expect("running rust2-lint");
	let err = stderr(&output);
	assert!(!output.status.success(), "{err}");
	assert!(err.contains("error: `exec::fat` costs "), "{err}");
}

#[test]
fn a_locked_build_keeps_its_lock() {
	// Outside the fixture, which has its lock already.
	let package = target("locked");
	std::fs::create_dir_all(package.join("src")).unwrap();
	let manifest = "[package]\nname = \"locked\"\nedition = \"2024\"\n\n[workspace]\n";
	std::fs::write(package.join("Cargo.toml"), manifest).unwrap();
	std::fs::write(package.join("src").join("lib.rs"), "").unwrap();
	let path = package.join("Cargo.toml");
	let output = lint(
		&package.join("target"),
		&["--", "build", "--locked", "--manifest-path", path.to_str().unwrap()],
	);
	let err = stderr(&output);
	assert!(!output.status.success(), "{err}");
	assert!(err.contains("because --locked was passed"), "{err}");
	assert!(!package.join("Cargo.lock").exists());
}

#[test]
fn a_repeated_build_is_fresh() {
	let target = target("fresh");
	// The budget is tracked only where it is checked, so no crate is dirty on a rerun.
	let args = ["--mono-budget", "1000000", "--", "build", "--package", "dependent"];
	for run in ["first", "second"] {
		let output = lint(&target, &args);
		let err = stderr(&output);
		assert!(output.status.success(), "{err}");
		if run == "second" {
			assert!(!err.contains("Compiling"), "{err}");
		}
	}
}

#[test]
fn misplaced_attributes_fail() {
	let output = lint(&target("misplaced"), &["--", "build", "--package", "misplaced"]);
	let err = stderr(&output);
	assert!(!output.status.success(), "{err}");
	let file = "misplaced/src/lib.rs";
	let errors: Vec<&str> = err.split("error: ").filter(|error| error.contains(file)).collect();
	let expected = [
		("#[cfg_attr(rust2, rust2::mono_budget(limit = 10))]", "takes `limit"),
		(
			"#[cfg_attr(rust2, rust2::mono_budget(limit = 10, reason = \"  \"))]",
			"takes `limit",
		),
		(
			"#[cfg_attr(rust2, rust2::mono_budget(limit = 10, reason = \"not g",
			"goes on a generic function",
		),
		(
			"\t#[cfg_attr(rust2, rust2::mono_budget(limit = 10, reason = \"a st",
			"goes on a generic function",
		),
		(
			"#[cfg_attr(rust2, rust2::mono_bugdet",
			"unknown attribute `rust2::mono_bugdet`",
		),
	];
	for (needle, message) in expected {
		let at = format!("{file}:{}:", line_of(file, needle));
		assert!(
			errors
				.iter()
				.any(|error| error.contains(message) && error.contains(&at)),
			"no `{message}` at {at}: {err}"
		);
	}
	assert_eq!(errors.len(), expected.len(), "{err}");
}

#[test]
fn check_fails() {
	let output = lint(&target("check"), &["--", "check", "--package", "user"]);
	let err = stderr(&output);
	assert!(!output.status.success(), "{err}");
	assert!(err.contains("`cargo check` never collects"), "{err}");
}

#[test]
fn a_std_generic_fails_at_its_local_caller() {
	let build = ["--", "build", "--package", "user", "--features", "sorted"];
	let output = lint(&target("sorted"), &[&["--mono-budget", "1000"][..], &build].concat());
	let err = stderr(&output);
	assert!(!output.status.success(), "{err}");
	// Every error is at the local function that sorts, none in std.
	let errors: Vec<&str> = err.split("error: `").skip(1).collect();
	assert!(!errors.is_empty(), "{err}");
	let at = format!(
		"user/src/lib.rs:{}:",
		line_of("user/src/lib.rs", "\t\t\t$(v.sort_by_key")
	);
	for error in &errors {
		let primary = error
			.lines()
			.find(|line| line.trim_start().starts_with("-->"))
			.expect(error);
		assert!(primary.contains(&at), "{err}");
		assert!(error.contains("instantiated through `sorted`"), "{err}");
	}
}

#[test]
fn report_ranks_without_failing() {
	let build = ["--", "build", "--package", "user", "--features", "fat,sorted"];
	let target = target("report");
	for run in ["first", "fresh"] {
		let output = lint(&target, &[&["--report"][..], &build].concat());
		let err = stderr(&output);
		assert!(output.status.success(), "{run}: {err}");
		// Cargo replays a fresh crate's warnings, so a rerun reports the same.
		let fat = err.find("warning: `exec::fat` costs ").expect(&err);
		assert!(err[fat..].contains("% of the crate's "), "{err}");
		let warnings: Vec<u64> = err
			.split("warning: `")
			.skip(1)
			.filter_map(|w| w.split(" costs ").nth(1)?.split(' ').next()?.parse().ok())
			.collect();
		assert!(warnings.len() > 1, "{err}");
		assert!(warnings.is_sorted_by(|a, b| a >= b), "heaviest first: {err}");
	}
}
