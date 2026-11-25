//! Builds preload.rs, the library rust2-bench preloads into every counted process, which
//! rust2-bench embeds: it is a cdylib, which Cargo cannot build as a dependency of a binary.

use std::path::PathBuf;
use std::process::Command;

fn main() {
	println!("cargo::rerun-if-changed=preload.rs");
	let out = PathBuf::from(std::env::var_os("OUT_DIR").expect("Cargo sets OUT_DIR"));
	let target = std::env::var("TARGET").expect("Cargo sets TARGET");
	if target != "x86_64-unknown-linux-gnu" {
		// Only x86_64 Linux is fingerprinted, which fails the run first; an empty library
		// keeps the runner building elsewhere, as its unit tests do.
		std::fs::write(out.join("preload.so"), []).expect("OUT_DIR is writable");
		return;
	}
	let rustc = std::env::var_os("RUSTC").unwrap_or_else(|| "rustc".into());
	let status = Command::new(rustc)
		.args([
			"--crate-type=cdylib",
			"--crate-name=rust2_bench_preload",
			"--edition=2024",
			"-Cpanic=abort",
			"-Copt-level=3",
			// Its bytes enter the fingerprint, so nothing of where it was built may: no debug
			// info, and the toolchain's own lld rather than the host's C compiler, which may add
			// a search path of the build's, as Nix's does.
			"-Cstrip=debuginfo",
			"-Clinker=rust-lld",
			"-Clinker-flavor=ld.lld",
			"--target",
			&target,
			"-o",
		])
		.arg(out.join("preload.so"))
		.arg("preload.rs")
		.status()
		.expect("rustc runs");
	assert!(status.success(), "building preload.rs failed ({status})");
}
