//! Translate the fixture crate and compare it, and its census of refusals, with the
//! expected files beside it. `RUST2_LEAN_BLESS=1` rewrites them instead.

use std::path::{Path, PathBuf};
use std::process::Command;

const FIXTURE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixture");

/// Run rust2-lean on the fixture, in a target directory of its own, since each run cleans
/// the package first.
fn run(name: &str, args: &[&str]) {
	let target = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(format!("lean-{name}"));
	let status = Command::new(env!("CARGO_BIN_EXE_rust2-lean"))
		.args(args)
		.arg("--manifest-path")
		.arg(Path::new(FIXTURE).join("Cargo.toml"))
		.env("CARGO_TARGET_DIR", target)
		.status()
		.unwrap();
	assert!(status.success(), "rust2-lean {args:?} failed");
}

fn compare(expected: &str, actual: &Path) {
	let expected = Path::new(FIXTURE).join(expected);
	let got = std::fs::read_to_string(actual).unwrap();
	if std::env::var_os("RUST2_LEAN_BLESS").is_some() {
		std::fs::write(&expected, got).unwrap();
		return;
	}
	let want = std::fs::read_to_string(&expected).unwrap();
	assert!(
		want == got,
		"{} differs from what rust2-lean wrote:\n{got}",
		expected.display()
	);
}

#[test]
fn translates() {
	let out = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(format!("lean-out-{}", std::process::id()));
	run(
		"out",
		&["--out", out.to_str().unwrap(), "--start-from", "crate::Session"],
	);
	compare("Fixture.lean", &out.join("Fixture.lean"));
	let prelude = include_str!("../lean/Rust.lean");
	assert_eq!(std::fs::read_to_string(out.join("Rust.lean")).unwrap(), prelude);
}

#[test]
fn refuses() {
	let census = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(format!("lean-census-{}.txt", std::process::id()));
	run(
		"census",
		&["--census", census.to_str().unwrap(), "--start-from", "crate::refused"],
	);
	compare("census.txt", &census);
}
