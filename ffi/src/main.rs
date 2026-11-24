//! rust2-ffi: export a crate's `pub` API as a C ABI and header, with no attributes.
//!
//! The binary is two programs, as rust2-ts is. Run directly, it is the CLI: it runs
//! cargo on the configured package with itself as cargo's rustc and the workspace's
//! wrapper. Run by cargo, it is rustc, which collects the configured library's API after
//! type checking and writes the export.

#![feature(rustc_private)]

extern crate rustc_abi;
extern crate rustc_ast;
extern crate rustc_attr_ir;
extern crate rustc_attr_parsing;
extern crate rustc_crate_store;
extern crate rustc_data_structures;
extern crate rustc_driver;
extern crate rustc_errors;
extern crate rustc_hir;
extern crate rustc_infer;
extern crate rustc_interface;
extern crate rustc_lexer;
extern crate rustc_metadata;
extern crate rustc_middle;
extern crate rustc_parse;
extern crate rustc_resolve;
extern crate rustc_session;
extern crate rustc_span;
extern crate rustc_structures;
extern crate rustc_symbol_mangling;
extern crate rustc_trait_selection;

mod abi;
mod c;
mod census;
mod cli;
mod collect;
mod config;
mod cpp;
mod csharp;
mod dart;
mod doc;
mod driver;
mod gate;
mod glue;
mod go;
mod help;
mod ir;
mod js;
mod jsi;
mod kotlin;
mod leaf;
mod lock;
mod merge;
mod node;
mod python;
mod swift;
mod wasm;
mod web;

use std::process::ExitCode;

fn main() -> ExitCode {
	let args: Vec<String> = std::env::args().collect();
	if let Some(code) = driver::run(&args) {
		return code;
	}
	cli::run(&args[1..])
}
