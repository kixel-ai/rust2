//! Compile `src/needs.ll` and `src/ctor.ll` into `libneeds.a` for a wasm32 target, with the
//! toolchain's own llc and llvm-ar, which its `llvm-tools` component ships.

use std::path::PathBuf;
use std::process::Command;

fn main() {
	println!("cargo::rerun-if-changed=src/needs.ll");
	println!("cargo::rerun-if-changed=src/ctor.ll");
	if std::env::var("CARGO_CFG_TARGET_ARCH").unwrap() != "wasm32" {
		return;
	}
	let rustc = std::env::var("RUSTC").unwrap();
	let sysroot = Command::new(rustc).args(["--print", "sysroot"]).output().unwrap();
	assert!(sysroot.status.success(), "rustc --print sysroot failed");
	let sysroot = String::from_utf8(sysroot.stdout).unwrap();
	let bin = PathBuf::from(sysroot.trim())
		.join("lib/rustlib")
		.join(std::env::var("HOST").unwrap())
		.join("bin");
	let out = PathBuf::from(std::env::var("OUT_DIR").unwrap());
	let run = |cmd: &mut Command| {
		let status = cmd.status().unwrap();
		assert!(status.success(), "{cmd:?} failed");
	};
	let archive = out.join("libneeds.a");
	let _ = std::fs::remove_file(&archive);
	let mut ar = Command::new(bin.join("llvm-ar"));
	ar.arg("rcs").arg(&archive);
	// Each its own object, so a call pulls only its own object, and its constructor, into
	// the link.
	for name in ["needs", "ctor"] {
		let object = out.join(format!("{name}.o"));
		run(Command::new(bin.join("llc"))
			.args(["-mtriple=wasm32-wasip1", "-filetype=obj"])
			.arg(format!("src/{name}.ll"))
			.arg("-o")
			.arg(&object));
		ar.arg(object);
	}
	run(&mut ar);
	println!("cargo::rustc-link-search=native={}", out.display());
}
