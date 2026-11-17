//! Compile `src/needs.ll` into `libneeds.a` for a wasm32 target, with the toolchain's own
//! llc and llvm-ar, which its `llvm-tools` component ships.

use std::path::PathBuf;
use std::process::Command;

fn main() {
	println!("cargo::rerun-if-changed=src/needs.ll");
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
	let object = out.join("needs.o");
	let run = |cmd: &mut Command| {
		let status = cmd.status().unwrap();
		assert!(status.success(), "{cmd:?} failed");
	};
	run(Command::new(bin.join("llc"))
		.args(["-mtriple=wasm32-wasip1", "-filetype=obj", "src/needs.ll", "-o"])
		.arg(&object));
	let archive = out.join("libneeds.a");
	let _ = std::fs::remove_file(&archive);
	run(Command::new(bin.join("llvm-ar")).arg("rcs").arg(&archive).arg(&object));
	println!("cargo::rustc-link-search=native={}", out.display());
}
