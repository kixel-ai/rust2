//! The command line: check the package with this binary as rustc, which translates it.

use std::path::{Path, PathBuf};
use std::process::ExitCode;

use rust2_driver::cargo::{Check, Wrap};

use crate::job::env;

const USAGE: &str = "usage: rust2-lean (--out <dir> | --census <file>) --start-from <path>... [--manifest-path <Cargo.toml>] [-- <cargo args>]

Translates the library of the package at --manifest-path (./Cargo.toml by default) to
Lean 4: every function a --start-from path names, and every function and type they reach,
in it and in its dependencies outside the sysroot. A path is `crate::module::Type::method`
or `crate::module::function`, or a prefix of such paths, such as `crate::module`, naming
every function under it; `dep::module::...` names a dependency's, by its crate's name.

--out writes Rust.lean, the runtime every translation imports, and <Crate>.lean for each
crate the translation reaches, its items under its crate's name, into <dir>; a construct
outside the subset is a compile error at its span.
--census writes every refusal to <file> instead, one per line with its span, followed by
how many reached functions translated.";

/// The profile every crate compiles with, as a cargo `--config`, which cargo's fingerprint
/// sees, so a crate cached without it compiles again: its MIR as the source wrote it, with no
/// inlining or folding across statements, and kept for the crates that call it, so the
/// package's translation reads its dependencies' bodies as its own. The sysroot comes
/// prebuilt, and keeps its models.
const PROFILE: &str = r#"profile.dev.rustflags = ["-Zmir-opt-level=0", "-Zalways-encode-mir"]"#;

struct Args {
	out: Option<PathBuf>,
	census: Option<PathBuf>,
	roots: Vec<String>,
	manifest: PathBuf,
	cargo: Vec<String>,
}

pub(crate) fn run(args: &[String]) -> ExitCode {
	match try_run(args) {
		Ok(code) => code,
		Err(err) => {
			eprintln!("rust2-lean: {err}");
			ExitCode::FAILURE
		}
	}
}

fn parse(args: &[String]) -> Result<Option<Args>, String> {
	let mut out = None;
	let mut census = None;
	let mut manifest = None;
	let mut roots = Vec::new();
	let mut cargo = Vec::new();
	let mut rest = args;
	while let [arg, tail @ ..] = rest {
		rest = tail;
		match arg.as_str() {
			"--out" | "--census" | "--manifest-path" | "--start-from" => {
				let [value, tail @ ..] = rest else {
					return Err(format!("{arg} needs a value\n\n{USAGE}"));
				};
				rest = tail;
				if arg == "--start-from" {
					roots.push(value.to_string());
					continue;
				}
				let path = std::path::absolute(value).map_err(|err| format!("resolving {value}: {err}"))?;
				let slot = match arg.as_str() {
					"--out" => &mut out,
					"--census" => &mut census,
					_ => &mut manifest,
				};
				if slot.replace(path).is_some() {
					return Err(format!("{arg} is given twice\n\n{USAGE}"));
				}
			}
			"--" => {
				cargo = rest.to_vec();
				break;
			}
			"-h" | "--help" => {
				println!("{USAGE}");
				return Ok(None);
			}
			_ => return Err(format!("unexpected argument {arg}\n\n{USAGE}")),
		}
	}
	if out.is_some() == census.is_some() {
		return Err(format!("exactly one of --out and --census is required\n\n{USAGE}"));
	}
	if roots.is_empty() {
		return Err(format!("--start-from is required\n\n{USAGE}"));
	}
	let manifest = match manifest {
		Some(manifest) => manifest,
		None => std::path::absolute("Cargo.toml").map_err(|err| format!("resolving Cargo.toml: {err}"))?,
	};
	Ok(Some(Args {
		out,
		census,
		roots,
		manifest,
		cargo,
	}))
}

fn try_run(args: &[String]) -> Result<ExitCode, String> {
	let Some(args) = parse(args)? else {
		return Ok(ExitCode::SUCCESS);
	};
	let package = package_name(&args.manifest)?;
	let roots = args.roots.join("\n");
	let mut vars: Vec<(&str, &std::ffi::OsStr)> = vec![(env::PACKAGE, package.as_ref()), (env::ROOTS, roots.as_ref())];
	if let Some(out) = &args.out {
		std::fs::create_dir_all(out).map_err(|err| format!("creating {}: {err}", out.display()))?;
		vars.push((env::OUT, out.as_os_str()));
	}
	if let Some(census) = &args.census {
		vars.push((env::CENSUS, census.as_os_str()));
	}
	vars.push(("CARGO_UNSTABLE_PROFILE_RUSTFLAGS", "true".as_ref()));
	// The profile `PROFILE` sets, which cargo refuses to combine with one the cargo args pick.
	let mut cargo: Vec<String> = ["--config", PROFILE, "--profile", "dev"].map(String::from).into();
	cargo.extend(args.cargo);
	Check {
		tool: "rust2-lean",
		manifest: &args.manifest,
		package: &package,
		root: None,
		target: None,
		args: &cargo,
		env: &vars,
		target_dir: None,
		dir: None,
		wrap: Wrap::All,
	}
	.run()
}

/// The name of the package whose manifest is `manifest`.
fn package_name(manifest: &Path) -> Result<String, String> {
	let output = std::process::Command::new(std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into()))
		.args(["metadata", "--no-deps", "--format-version", "1", "--manifest-path"])
		.arg(manifest)
		.output()
		.map_err(|err| format!("running cargo metadata: {err}"))?;
	if !output.status.success() {
		return Err(String::from_utf8_lossy(&output.stderr).into_owned());
	}
	let metadata: serde_json::Value =
		serde_json::from_slice(&output.stdout).map_err(|err| format!("parsing cargo metadata: {err}"))?;
	let canonical = |p: &Path| p.canonicalize().unwrap_or_else(|_| p.to_path_buf());
	let wanted = canonical(manifest);
	metadata["packages"]
		.as_array()
		.into_iter()
		.flatten()
		.find(|p| {
			p["manifest_path"]
				.as_str()
				.is_some_and(|m| canonical(Path::new(m)) == wanted)
		})
		.and_then(|p| p["name"].as_str())
		.map(str::to_string)
		.ok_or_else(|| format!("{} is no package's manifest", manifest.display()))
}
