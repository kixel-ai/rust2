//! The command line: run cargo on the package with this binary as rustc.

use std::path::Path;
use std::process::ExitCode;

use rust2_driver::cargo::{Args, Check, Output};

use crate::driver::env;

const USAGE: &str = "usage: rust2-go --out <dir> [--manifest-path <Cargo.toml>] [-- <cargo args>]

Translates the library of the package whose Cargo.toml is in the current directory, or
wherever --manifest-path says, to a Go package in <dir> named after the crate, exporting
its whole `pub` API. Cargo args, such as --features, are passed to `cargo check`, which
checks with the dev profile on the host, with std checked from source.";

pub fn run(args: &[String]) -> ExitCode {
	match try_run(args) {
		Ok(code) => code,
		Err(err) => {
			eprintln!("rust2-go: {err}");
			ExitCode::FAILURE
		}
	}
}

fn try_run(args: &[String]) -> Result<ExitCode, String> {
	let Some(args) = Args::parse_configless(args, USAGE)? else {
		return Ok(ExitCode::SUCCESS);
	};
	let package = package(&args.manifest)?;
	let Output::Dir(out) = &args.output else {
		return Err(format!("--census is not supported\n\n{USAGE}"));
	};
	let mut cargo = profile_args();
	// The profile `PROFILE` sets, which cargo refuses to combine with one the cargo args pick.
	cargo.extend(["--profile".into(), "dev".into()]);
	cargo.extend(args.cargo.iter().cloned());
	Check {
		tool: "rust2-go",
		manifest: &args.manifest,
		package: &package,
		root: None,
		// The host's: Go's `int` and `uint` are 64 bits on every platform the output
		// targets, as the host's `usize` is.
		target: Some(rustc_session::config::host_tuple()),
		args: &cargo,
		env: &[
			(env::PACKAGE, package.as_ref()),
			(env::OUT, out.as_os_str()),
			("CARGO_UNSTABLE_PROFILE_RUSTFLAGS", "true".as_ref()),
		],
	}
	.run()
}

/// The name of the package `manifest` declares.
fn package(manifest: &Path) -> Result<String, String> {
	let text = std::fs::read_to_string(manifest).map_err(|err| format!("reading {}: {err}", manifest.display()))?;
	let table: toml::Table = text
		.parse()
		.map_err(|err| format!("parsing {}: {err}", manifest.display()))?;
	match table
		.get("package")
		.and_then(|p| p.get("name"))
		.and_then(|n| n.as_str())
	{
		Some(name) => Ok(name.to_string()),
		None => Err(format!("{} declares no package", manifest.display())),
	}
}

/// The cargo config of the profile every crate compiles with, one `--config` per line.
const PROFILE: &str = include_str!("profile.toml");

/// Cargo args that set [`PROFILE`] and check std from source.
fn profile_args() -> Vec<String> {
	let mut args = vec!["-Zbuild-std=core,alloc,std,panic_abort".to_string()];
	for line in PROFILE
		.lines()
		.filter(|line| !line.is_empty() && !line.starts_with('#'))
	{
		args.extend(["--config".to_string(), line.to_string()]);
	}
	args
}
