//! The binary as rustc: plain for every crate but the package's library, which it also
//! translates once rustc has checked it.

use std::collections::HashSet;
use std::path::PathBuf;
use std::process::ExitCode;

use rustc_driver::{Callbacks, Compilation};
use rustc_interface::interface::Compiler;
use rustc_middle::ty::TyCtxt;

/// The variables the CLI passes the wrapper its job in.
pub(crate) mod env {
	/// The cargo package whose library to translate.
	pub const PACKAGE: &str = "RUST2_LEAN_PACKAGE";
	/// The `--start-from` paths, one per line.
	pub const ROOTS: &str = "RUST2_LEAN_ROOTS";
	/// The directory to write the translation into.
	pub const OUT: &str = "RUST2_LEAN_OUT";
	/// The file to write the census into.
	pub const CENSUS: &str = "RUST2_LEAN_CENSUS";
}

/// Run as rustc, with `args` as cargo passed them: the wrapped rustc's path first.
pub(crate) fn run(args: Vec<String>) -> ExitCode {
	rust2_driver::rustc::run("rust2-lean", args, |args, added| {
		let Ok(package) = std::env::var(env::PACKAGE) else {
			return Ok(None);
		};
		if !rust2_driver::rustc::compiles_lib(args, &package) {
			return Ok(None);
		}
		let roots: Vec<String> = std::env::var(env::ROOTS)
			.unwrap_or_default()
			.lines()
			.map(str::to_string)
			.collect();
		// rustc loads a dependency only once the source names it, so load each a root names,
		// by the crate's own name, under whatever name the package gives it.
		let named: HashSet<&str> = roots.iter().filter_map(|r| r.split("::").next()).collect();
		for w in args.windows(2).filter(|w| w[0] == "--extern") {
			let (alias, path) = w[1].split_once('=').unwrap_or((&w[1], ""));
			let alias = alias.rsplit(':').next().unwrap_or(alias);
			// Cargo names the artifact `lib<crate>-<hash>.<ext>`.
			let krate = std::path::Path::new(path)
				.file_stem()
				.and_then(|s| s.to_str()?.strip_prefix("lib")?.rsplit_once('-'))
				.map_or(alias, |(krate, _)| krate);
			if named.contains(alias) || named.contains(krate) {
				added.extend(["-Zunstable-options".into(), "--extern".into(), format!("force:{alias}")]);
			}
		}
		let output = match (std::env::var_os(env::OUT), std::env::var_os(env::CENSUS)) {
			(Some(out), None) => Output::Dir(out.into()),
			(None, Some(census)) => Output::Census(census.into()),
			_ => return Err(format!("exactly one of {} and {} must be set", env::OUT, env::CENSUS)),
		};
		Ok(Some(Box::new(Extract { roots, output }) as rust2_driver::rustc::Job))
	})
}

struct Extract {
	roots: Vec<String>,
	output: Output,
}

enum Output {
	Dir(PathBuf),
	Census(PathBuf),
}

impl Callbacks for Extract {
	fn after_analysis<'tcx>(&mut self, _compiler: &Compiler, tcx: TyCtxt<'tcx>) -> Compilation {
		if tcx.dcx().has_errors().is_some() {
			return Compilation::Stop;
		}
		let extraction = crate::lower::extract(tcx, &self.roots);
		for root in &extraction.unmatched {
			tcx.dcx()
				.err(format!("rust2-lean: `--start-from {root}` names no function"));
		}
		if tcx.dcx().has_errors().is_some() {
			return Compilation::Stop;
		}
		let written = match &self.output {
			Output::Dir(out) => {
				for refusal in &extraction.refusals {
					tcx.dcx().span_err(refusal.span, format!("rust2-lean: {}", refusal.msg));
				}
				if extraction.refusals.is_empty() {
					crate::emit::write(tcx, &extraction, out)
				} else {
					Ok(())
				}
			}
			Output::Census(path) => std::fs::write(path, crate::emit::census(tcx, &extraction))
				.map_err(|err| format!("writing {}: {err}", path.display())),
		};
		if let Err(err) = written {
			tcx.dcx().err(format!("rust2-lean: {err}"));
		}
		match tcx.dcx().has_errors() {
			None => Compilation::Continue,
			Some(_) => Compilation::Stop,
		}
	}
}
