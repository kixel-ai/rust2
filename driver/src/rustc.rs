//! The binary as rustc, run by cargo as its `RUSTC_WRAPPER`.

use std::path::Path;
use std::process::ExitCode;

use rustc_driver::Callbacks;
use rustc_session::EarlyDiagCtxt;
use rustc_session::config::ErrorOutputType;

const BUG_REPORT_URL: &str = "https://github.com/moq-dev/rust2/issues/new";

/// Whether cargo ran this binary as its wrapper, which it does with the rustc it would
/// have run as the first argument.
pub fn wrapped(args: &[String]) -> bool {
	args.get(1).map(Path::new).and_then(Path::file_stem) == Some("rustc".as_ref())
}

/// The callbacks for one rustc invocation.
pub type Job = Box<dyn Callbacks + Send>;

/// Run as rustc, with `args` as cargo passed them. `job` picks the callbacks from
/// rustc's own arguments, with their argfiles read, and may add arguments to the second
/// list, or returns `None` to compile the crate as rustc would; its error ends the run
/// before rustc starts, prefixed with `tool`.
pub fn run(
	tool: &str,
	mut args: Vec<String>,
	job: impl FnOnce(&[String], &mut Vec<String>) -> Result<Option<Job>, String>,
) -> ExitCode {
	args.remove(1);
	let early_dcx = EarlyDiagCtxt::new(ErrorOutputType::default());
	// The job reads `@path` argfiles' arguments as rustc does: cargo passes them in one once
	// they overflow a command line, as many `-L`s do on Windows. rustc still gets cargo's
	// own, since it reads an argfile once, and an argument inside one starting with `@` is
	// no other file.
	let expanded = [&args[..1], &rustc_driver::args::arg_expand_all(&early_dcx, &args[1..])].concat();
	rustc_driver::init_rustc_env_logger(&early_dcx);
	rustc_driver::install_ice_hook(BUG_REPORT_URL, |_| ());
	let job = match job(&expanded, &mut args) {
		Ok(job) => job,
		Err(err) => early_dcx.early_fatal(format!("{tool}: {err}")),
	};
	rustc_driver::catch_with_exit_code(|| match job {
		Some(mut job) => {
			rustc_driver::compiler_entrypoint(&args, job.as_mut());
			// Tell the CLI its library was compiled with the job.
			if let Some(ran) = std::env::var_os(RAN) {
				let _ = std::fs::write(ran, "");
			}
		}
		None => rustc_driver::compiler_entrypoint(&args, &mut Plain),
	})
}

/// The file the wrapper creates once it has run a job, so the CLI knows cargo compiled
/// the configured library at all.
pub(crate) const RAN: &str = "RUST2_DRIVER_RAN";

/// Compiles a crate as rustc would.
struct Plain;

impl Callbacks for Plain {}

/// Whether rustc's `args` compile the library of the cargo package `package`, not its
/// tests. Cargo names the package being compiled in `CARGO_PKG_NAME`, whatever its
/// library target is called.
pub fn compiles_lib(args: &[String], package: &str) -> bool {
	let ours = std::env::var("CARGO_PKG_NAME").is_ok_and(|name| name == package);
	let lib = args
		.windows(2)
		.any(|w| w[0] == "--crate-type" && matches!(w[1].as_str(), "lib" | "rlib"));
	ours && lib && !args.iter().any(|a| a == "--test")
}
