//! rust2-lean: translate a pure subset of a crate's MIR to Lean 4.
//!
//! The binary is two programs, as the other drivers are. Run directly, it is the CLI: it
//! runs `cargo check` on a package with itself as rustc. Run by cargo as that wrapper, it
//! is rustc, which translates the package's library from the functions the CLI names once
//! rustc has checked it.

#![feature(rustc_private)]

extern crate rustc_abi;
extern crate rustc_driver;
extern crate rustc_hir;
extern crate rustc_interface;
extern crate rustc_metadata;
extern crate rustc_middle;
extern crate rustc_span;

mod cli;
mod emit;
mod job;
mod lower;
mod structure;

use std::process::ExitCode;

fn main() -> ExitCode {
	let args: Vec<String> = std::env::args().collect();
	if rust2_driver::rustc::wrapped(&args) {
		return job::run(args);
	}
	cli::run(&args[1..])
}
