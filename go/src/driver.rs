//! The binary as rustc: plain for every crate but the package's library, which it also
//! translates once rustc has checked it.

use std::path::PathBuf;
use std::process::ExitCode;

use rustc_driver::{Callbacks, Compilation};
use rustc_interface::interface::Compiler;
use rustc_middle::ty::TyCtxt;

use crate::lower;

/// Environment variables the CLI hands the wrapper.
pub mod env {
	/// The cargo package whose library is translated.
	pub const PACKAGE: &str = "RUST2_GO_PACKAGE";
	/// The output directory, absolute.
	pub const OUT: &str = "RUST2_GO_OUT";
}

/// Run as rustc, with `args` as cargo passed them: the wrapped rustc's path first.
pub fn run(args: Vec<String>) -> ExitCode {
	rust2_driver::rustc::run("rust2-go", args, |args, _| {
		Ok(Translate::for_args(args)?.map(|job| Box::new(job) as rust2_driver::rustc::Job))
	})
}

/// Compiles the package's library, translating it after analysis.
struct Translate {
	out: PathBuf,
}

impl Translate {
	/// The job for this rustc invocation, if it compiles the package's library.
	fn for_args(args: &[String]) -> Result<Option<Self>, String> {
		let Ok(package) = std::env::var(env::PACKAGE) else {
			return Ok(None);
		};
		if !rust2_driver::rustc::compiles_lib(args, &package) {
			return Ok(None);
		}
		let out = std::env::var_os(env::OUT).ok_or(format!("{} is unset", env::OUT))?;
		Ok(Some(Self { out: out.into() }))
	}
}

impl Callbacks for Translate {
	fn after_expansion<'tcx>(&mut self, _compiler: &Compiler, tcx: TyCtxt<'tcx>) -> Compilation {
		tcx.ensure_ok().analysis(());
		if tcx.dcx().has_errors().is_some() {
			return Compilation::Stop;
		}
		// Only a program rustc accepts is translated, and only a whole one is written.
		if let Some(files) = lower::translate(tcx)
			&& tcx.dcx().has_errors().is_none()
			&& let Err(err) = lower::write(&self.out, &files)
		{
			tcx.dcx().err(format!("rust2-go: {err}"));
		}
		match tcx.dcx().has_errors() {
			None => Compilation::Continue,
			Some(_) => Compilation::Stop,
		}
	}
}
