//! rust2-lint: fail a build whose generics cost its codegen more than a budget, or
//! report what they cost.
//!
//! The binary is two programs, as rust2-ts is. Run directly, it is the CLI: it runs a
//! cargo command with itself as `RUSTC_WRAPPER`. Run by cargo as that wrapper, it is
//! rustc, which compiles every crate as rustc would, checks its `rust2` attributes, and
//! checks the mono budget of each local package's crates.

#![feature(rustc_private)]

extern crate rustc_ast;
extern crate rustc_attr_ir;
extern crate rustc_data_structures;
extern crate rustc_driver;
extern crate rustc_hir;
extern crate rustc_interface;
extern crate rustc_middle;
extern crate rustc_span;

mod mono;

use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

use rust2_driver::{cargo, rustc};
use rustc_driver::{Callbacks, Compilation};
use rustc_interface::interface::Compiler;
use rustc_middle::ty::TyCtxt;
use rustc_span::Symbol;
use serde::Deserialize;

const TOOL: &str = "rust2-lint";

/// Carries the mono budget from the CLI to the wrapper.
const BUDGET: &str = "RUST2_MONO_BUDGET";

/// Set when `--report` replaces the budget with a report.
const REPORT: &str = "RUST2_MONO_REPORT";

/// Carries the directories of the local packages, whose crates the wrapper checks.
const LOCAL: &str = "RUST2_LOCAL_PACKAGES";

/// The mono budget without `--mono-budget`, in MIR statements: about 4% of moq-net's
/// test target, and 3x its heaviest generic once `moq_clock::sim::run` only forwards.
const DEFAULT_BUDGET: u64 = 50_000;

fn usage() -> String {
	format!(
		"usage: {TOOL} [--mono-budget <statements> | --report] -- <cargo command>

Runs the cargo command, such as `test --no-run -p <package>`, with this binary as
rustc, into its own target directory. A crate of a local package, a workspace member
or path dependency, fails to compile when a generic it instantiates costs more MIR
statements, across every copy, than the mono budget ({DEFAULT_BUDGET} unless given) or
the generic's own:

  #[cfg_attr(rust2, rust2::mono_budget(limit = <statements>, reason = \"<why>\"))]

A generic whose copies cost at most {} statements each only forwards, and passes.
A generic from std or a dependency fails at the local function that instantiates it.

With --report, nothing fails: each local crate warns of every generic past {}% of its
codegen, heaviest first.
Every crate compiles with `--cfg rust2` and the `rust2` tool registered. The command
must build: `cargo check` collects no mono items.",
		mono::THIN,
		mono::SHARE,
	)
}

fn main() -> ExitCode {
	let args: Vec<String> = std::env::args().collect();
	if rustc::wrapped(&args) {
		return rustc::run(TOOL, args, job);
	}
	match cli(&args[1..]) {
		Ok(code) => code,
		Err(err) => {
			eprintln!("{TOOL}: {err}");
			ExitCode::FAILURE
		}
	}
}

/// Run the cargo command in `args`, with this binary as rustc.
fn cli(args: &[String]) -> Result<ExitCode, String> {
	let mut budget = DEFAULT_BUDGET;
	let mut report = false;
	let mut rest = args;
	let command = loop {
		match rest {
			[flag, value, tail @ ..] if flag == "--mono-budget" => {
				budget = parse_budget(value)?;
				rest = tail;
			}
			[flag, tail @ ..] if flag == "--report" => {
				report = true;
				rest = tail;
			}
			[flag, ..] if flag == "-h" || flag == "--help" => {
				println!("{}", usage());
				return Ok(ExitCode::SUCCESS);
			}
			[flag, command @ ..] if flag == "--" && !command.is_empty() => break command,
			_ => return Err(format!("expected a cargo command after --\n\n{}", usage())),
		}
	};
	let exe = std::env::current_exe().map_err(|err| format!("finding this executable: {err}"))?;
	let program = std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into());
	let status = Command::new(&program)
		.args(command)
		.env("RUSTC_WRAPPER", exe)
		.env("CARGO_TARGET_DIR", cargo::target_dir(TOOL, manifest(command))?)
		.env(BUDGET, budget.to_string())
		.env(REPORT, if report { "1" } else { "" })
		.env(LOCAL, local_packages(&program, resolution(command))?)
		.status()
		.map_err(|err| format!("running cargo: {err}"))?;
	Ok(match status.success() {
		true => ExitCode::SUCCESS,
		false => ExitCode::FAILURE,
	})
}

/// The `--manifest-path` the cargo command names, if any, before the args it passes on.
fn manifest(command: &[String]) -> Option<&Path> {
	let own = command.iter().position(|arg| arg == "--").unwrap_or(command.len());
	let command = &command[..own];
	command
		.iter()
		.enumerate()
		.find_map(|(i, arg)| match arg.strip_prefix("--manifest-path")? {
			"" => command.get(i + 1).map(Path::new),
			value => value.strip_prefix('=').map(Path::new),
		})
}

/// The cargo command's flags that decide how cargo finds and resolves the workspace,
/// which its metadata must honor too: `--locked` must not let metadata write the lock.
fn resolution(command: &[String]) -> Vec<&String> {
	let own = command.iter().position(|arg| arg == "--").unwrap_or(command.len());
	let mut args = command[..own].iter();
	let mut flags = Vec::new();
	while let Some(arg) = args.next() {
		match arg.as_str() {
			"--locked" | "--frozen" | "--offline" => flags.push(arg),
			"--manifest-path" | "--config" | "-Z" => {
				flags.push(arg);
				flags.extend(args.next());
			}
			value
				if ["--manifest-path=", "--config=", "-Z"]
					.iter()
					.any(|flag| value.starts_with(flag)) =>
			{
				flags.push(arg)
			}
			_ => {}
		}
	}
	flags
}

fn parse_budget(value: &str) -> Result<u64, String> {
	value
		.parse()
		.map_err(|_| format!("the mono budget is a count of MIR statements, not `{value}`"))
}

/// Compile with the `rust2` tool and cfg, and lint the crate.
fn job(_: &[String], added: &mut Vec<String>) -> Result<Option<rustc::Job>, String> {
	added.extend(
		[
			"--cfg=rust2",
			"-Zcrate-attr=feature(register_tool)",
			"-Zcrate-attr=register_tool(rust2)",
		]
		.map(String::from),
	);
	let (Some(budget), Some(local)) = (std::env::var_os(BUDGET), std::env::var_os(LOCAL)) else {
		return Err(format!(
			"{BUDGET} or {LOCAL} is unset: run {TOOL} itself, not as RUSTC_WRAPPER"
		));
	};
	// A registry or git dependency's generics are its dependents' to answer for. Every local
	// crate is checked, selected or not: cargo would reuse one built unchecked as a
	// dependency once a later command selects it, since selection is no part of its
	// fingerprint.
	let dir = std::env::var_os("CARGO_MANIFEST_DIR");
	let checked = dir.is_some_and(|dir| std::env::split_paths(&local).any(|local| local == Path::new(&dir)));
	let mode = match (
		checked,
		std::env::var_os(REPORT).is_some_and(|report| !report.is_empty()),
	) {
		(false, _) => None,
		(true, true) => Some(mono::Mode::Report),
		(true, false) => Some(mono::Mode::Check(parse_budget(&budget.to_string_lossy())?)),
	};
	let local = std::env::split_paths(&local).collect();
	Ok(Some(Box::new(Lint { mode, local })))
}

/// The directories of the packages cargo builds from local source, workspace members and
/// path dependencies, which cargo metadata lists without a `source`.
fn local_packages(program: &OsStr, resolution: Vec<&String>) -> Result<OsString, String> {
	#[derive(Deserialize)]
	struct Metadata {
		packages: Vec<Package>,
	}
	#[derive(Deserialize)]
	struct Package {
		manifest_path: PathBuf,
		source: Option<String>,
	}
	let mut cmd = Command::new(program);
	// Every feature, so a path dependency only a feature enables is local too.
	cmd.args(["metadata", "--format-version", "1", "--all-features"])
		.args(resolution);
	let output = cmd.output().map_err(|err| format!("running cargo metadata: {err}"))?;
	if !output.status.success() {
		return Err(String::from_utf8_lossy(&output.stderr).into_owned());
	}
	let metadata: Metadata =
		serde_json::from_slice(&output.stdout).map_err(|err| format!("reading cargo metadata: {err}"))?;
	let dirs = metadata
		.packages
		.into_iter()
		.filter(|package| package.source.is_none())
		.filter_map(|package| package.manifest_path.parent().map(Path::to_path_buf));
	std::env::join_paths(dirs).map_err(|err| format!("joining package directories: {err}"))
}

/// Checks the crate's `rust2` attributes, and its mono budget once rustc has collected
/// its mono items.
struct Lint {
	/// The crate-wide mono budget or a report, for a local package.
	mode: Option<mono::Mode>,
	/// The directories of the local packages.
	local: Vec<PathBuf>,
}

impl Callbacks for Lint {
	fn after_expansion<'tcx>(&mut self, _compiler: &Compiler, tcx: TyCtxt<'tcx>) -> Compilation {
		if let Some(mode) = self.mode {
			// Cargo rebuilds the crate when the mode changes, as it does for `env!`.
			let (key, value) = match mode {
				mono::Mode::Check(budget) => (BUDGET, budget.to_string()),
				mono::Mode::Report => (REPORT, "1".into()),
			};
			tcx.sess
				.env_depinfo
				.borrow_mut()
				.insert((Symbol::intern(key), Some(Symbol::intern(&value))));
		}
		Compilation::Continue
	}

	fn after_analysis<'tcx>(&mut self, _compiler: &Compiler, tcx: TyCtxt<'tcx>) -> Compilation {
		mono::check_attributes(tcx);
		if tcx.dcx().has_errors().is_some() {
			return Compilation::Stop;
		}
		let Some(mode) = self.mode else {
			return Compilation::Continue;
		};
		if !tcx.sess.opts.output_types.should_codegen() {
			tcx.dcx().err(format!(
				"{TOOL}: the mono budget and its report need mono items, which `cargo check` never collects: build instead"
			));
			return Compilation::Stop;
		}
		// Codegen would collect them next; the timer counts only the lint's own work.
		let _ = tcx.collect_and_partition_mono_items(());
		tcx.sess.time("mono_budget", || mono::check(tcx, mode, &self.local));
		match tcx.dcx().has_errors() {
			Some(_) => Compilation::Stop,
			None => Compilation::Continue,
		}
	}
}
