//! A Cargo workspace's benchmarks, written with CodSpeed's compatibility crates.
//!
//! Built with `--cfg codspeed`, as `cargo codspeed build` builds them, a benchmark target
//! runs each benchmark once around CodSpeed's instrument hooks, printing `Checked: <name>`
//! where nothing measures it. With `CODSPEED_ISOLATED_BENCHMARK` set to a name it runs that
//! benchmark alone, and with `CODSPEED_PROCESS_ISOLATION=false` it runs every benchmark in
//! its own process rather than one child each.

use std::collections::BTreeSet;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use serde_json::Value;

use crate::Job;

/// The cfg CodSpeed's compatibility crates take to run each benchmark once around its hooks.
const CFG: &str = "--cfg=codspeed";

/// A benchmark target cargo built.
struct Target {
	name: String,
	executable: PathBuf,
	/// Its package's directory, where `cargo bench` runs it.
	dir: PathBuf,
}

/// Builds the benchmark targets of the workspace in `dir`, passing `args` to `cargo build`,
/// and lists the benchmarks each runs as a job, one benchmark a process.
pub fn jobs(dir: &Path, args: &[String]) -> Result<Vec<Job>, String> {
	let root = workspace_root(dir, args)?;
	let targets = build(dir, args)?;
	if targets.is_empty() {
		return Err("cargo built no benchmark target".to_owned());
	}
	// Every target lists its benchmarks at once: each runs them natively, far faster than
	// under Valgrind.
	let listed = std::thread::scope(|scope| {
		let handles: Vec<_> = targets
			.iter()
			.map(|target| scope.spawn(|| list(target, &root)))
			.collect();
		handles
			.into_iter()
			.map(|h| h.join().expect("a listing thread panicked"))
			.collect::<Vec<_>>()
	});
	let mut seen = BTreeSet::new();
	let mut jobs = Vec::new();
	for (target, names) in targets.iter().zip(listed) {
		let names = names?;
		if names.is_empty() {
			return Err(format!(
				"benchmark target {} ran no benchmark through CodSpeed's compatibility crates, built with --cfg=codspeed \
				 (which a config's target rustflags would drop): leave it out with cargo's --bench",
				target.name
			));
		}
		for name in names {
			if !seen.insert(name.clone()) {
				return Err(format!("two benchmarks are named {name}"));
			}
			let mut command = Command::new(&target.executable);
			command
				.current_dir(&target.dir)
				.env("CODSPEED_CARGO_WORKSPACE_ROOT", &root)
				.env("CODSPEED_ISOLATED_BENCHMARK", &name);
			jobs.push(Job {
				label: name.clone(),
				command,
				expect: Some(name),
			});
		}
	}
	Ok(jobs)
}

/// The root of the workspace in `dir`, which CodSpeed's crates name each benchmark's file
/// relative to.
fn workspace_root(dir: &Path, args: &[String]) -> Result<PathBuf, String> {
	let mut cargo = Command::new("cargo");
	cargo.args(["locate-project", "--workspace", "--message-format", "plain"]);
	if let Some(manifest) = manifest_path(args) {
		cargo.args(["--manifest-path", manifest]);
	}
	let out = cargo
		.current_dir(dir)
		.stderr(Stdio::inherit())
		.output()
		.map_err(|e| format!("cannot run cargo: {e}"))?;
	if !out.status.success() {
		return Err(format!("cargo locate-project failed ({})", out.status));
	}
	let manifest = String::from_utf8(out.stdout).map_err(|_| "cargo located a manifest at a path not UTF-8")?;
	let manifest = Path::new(manifest.trim());
	Ok(manifest
		.parent()
		.ok_or("cargo located a manifest with no directory")?
		.to_owned())
}

/// Where Cargo's configuration sets rustflags, if anywhere: its environment, a `--config`
/// among `args`, or a config file it reads from `dir` up, or from Cargo's home.
fn configured_rustflags(dir: &Path, args: &[String]) -> Result<Option<String>, String> {
	for (key, _) in std::env::vars_os() {
		let key = key.to_string_lossy();
		if key == "CARGO_BUILD_RUSTFLAGS" || (key.starts_with("CARGO_TARGET_") && key.ends_with("_RUSTFLAGS")) {
			return Ok(Some(format!("the environment's {key}")));
		}
	}
	let mut args = args.iter();
	while let Some(arg) = args.next() {
		let config = if arg == "--config" {
			args.next().map(String::as_str)
		} else {
			arg.strip_prefix("--config=")
		};
		if let Some(config) = config {
			let text = if Path::new(config).is_file() {
				std::fs::read_to_string(config).map_err(|e| format!("cannot read {config}: {e}"))?
			} else {
				config.to_owned()
			};
			if text.contains("rustflags") {
				return Ok(Some(format!("--config {config}")));
			}
		}
	}
	let dir = dir
		.canonicalize()
		.map_err(|e| format!("cannot resolve {}: {e}", dir.display()))?;
	let home = std::env::var_os("CARGO_HOME")
		.map(PathBuf::from)
		.or_else(|| std::env::var_os("HOME").map(|home| Path::new(&home).join(".cargo")));
	let dirs = dir.ancestors().map(|dir| dir.join(".cargo")).chain(home);
	for cargo in dirs {
		for name in ["config.toml", "config"] {
			let path = cargo.join(name);
			let Ok(text) = std::fs::read_to_string(&path) else {
				continue;
			};
			let config: toml::Table = text
				.parse()
				.map_err(|e| format!("cannot parse {}: {e}", path.display()))?;
			let build = config.get("build").and_then(|build| build.get("rustflags")).is_some();
			let target = config
				.get("target")
				.and_then(toml::Value::as_table)
				.is_some_and(|targets| targets.values().any(|target| target.get("rustflags").is_some()));
			if build || target {
				return Ok(Some(path.display().to_string()));
			}
		}
	}
	Ok(None)
}

/// The manifest `cargo build` args select, when they name one.
fn manifest_path(args: &[String]) -> Option<&str> {
	let mut args = args.iter();
	while let Some(arg) = args.next() {
		if arg == "--" {
			break;
		}
		if arg == "--manifest-path" {
			return args.next().map(String::as_str);
		}
		if let Some(path) = arg.strip_prefix("--manifest-path=") {
			return Some(path);
		}
	}
	None
}

/// Builds every benchmark target with CodSpeed's cfg, as `cargo codspeed build` does.
fn build(dir: &Path, args: &[String]) -> Result<Vec<Target>, String> {
	let mut cargo = Command::new("cargo");
	cargo.args([
		"build",
		"--profile",
		"bench",
		"--message-format=json-render-diagnostics",
	]);
	// Every benchmark target, unless the caller names some.
	if !args.iter().any(|arg| arg == "--bench" || arg.starts_with("--bench=")) {
		cargo.arg("--benches");
	}
	cargo.args(args).current_dir(dir).stdout(Stdio::piped());
	// Cargo takes one source of rustflags and ignores the rest, so the cfg joins the flags the
	// environment sets, or else is the only flags there are: a config that sets its own is
	// refused before anything builds, rather than one source silently replacing another.
	if let Ok(flags) = std::env::var("CARGO_ENCODED_RUSTFLAGS") {
		let flags = if flags.is_empty() {
			CFG.to_owned()
		} else {
			format!("{flags}\x1f{CFG}")
		};
		cargo.env("CARGO_ENCODED_RUSTFLAGS", flags);
	} else if let Ok(flags) = std::env::var("RUSTFLAGS") {
		cargo.env("RUSTFLAGS", format!("{flags} {CFG}"));
	} else {
		if let Some(source) = configured_rustflags(dir, args)? {
			return Err(format!(
				"{source} sets rustflags, which the cfg CodSpeed's crates take would replace or be replaced by: \
				 set RUSTFLAGS to the flags the build needs, and rust2-bench adds the cfg to them"
			));
		}
		cargo.env("CARGO_ENCODED_RUSTFLAGS", CFG);
	}
	eprintln!("rust2-bench: building benchmarks in {}", dir.display());
	let mut child = cargo.spawn().map_err(|e| format!("cannot run cargo: {e}"))?;
	let stdout = child.stdout.take().expect("stdout is piped");
	let mut targets = Vec::new();
	for line in BufReader::new(stdout).lines() {
		let line = line.map_err(|e| format!("cannot read cargo's output: {e}"))?;
		let message: Value =
			serde_json::from_str(&line).map_err(|e| format!("cargo printed a line not JSON ({e}): {line}"))?;
		if let Some(target) = bench(&message)? {
			targets.push(target);
		}
	}
	let status = child.wait().map_err(|e| format!("cannot wait for cargo: {e}"))?;
	if !status.success() {
		return Err(format!("cargo build failed ({status})"));
	}
	Ok(targets)
}

/// The benchmark target a cargo message reports built, if it reports one.
fn bench(message: &Value) -> Result<Option<Target>, String> {
	if message["reason"] != "compiler-artifact" {
		return Ok(None);
	}
	let kinds = message["target"]["kind"]
		.as_array()
		.ok_or("cargo reported an artifact without a target kind")?;
	if !kinds.iter().any(|kind| kind == "bench") {
		return Ok(None);
	}
	let field = |value: &Value, what: &str| {
		value
			.as_str()
			.map(str::to_owned)
			.ok_or(format!("cargo reported a benchmark without {what}"))
	};
	let name = field(&message["target"]["name"], "a name")?;
	let executable = PathBuf::from(field(&message["executable"], "an executable")?);
	let manifest = PathBuf::from(field(&message["manifest_path"], "a manifest")?);
	let dir = manifest
		.parent()
		.ok_or("cargo reported a manifest with no directory")?
		.to_owned();
	Ok(Some(Target { name, executable, dir }))
}

/// The benchmarks `target` runs, as it names them when nothing measures it.
fn list(target: &Target, root: &Path) -> Result<Vec<String>, String> {
	let out = Command::new(&target.executable)
		.current_dir(&target.dir)
		.env("CODSPEED_CARGO_WORKSPACE_ROOT", root)
		.env("CODSPEED_PROCESS_ISOLATION", "false")
		.env_remove("CODSPEED_ISOLATED_BENCHMARK")
		.stdin(Stdio::null())
		.output()
		.map_err(|e| format!("cannot run benchmark target {}: {e}", target.name))?;
	if !out.status.success() {
		return Err(format!(
			"benchmark target {} failed listing its benchmarks ({}):\n{}{}",
			target.name,
			out.status,
			String::from_utf8_lossy(&out.stdout),
			String::from_utf8_lossy(&out.stderr)
		));
	}
	Ok(checked(&String::from_utf8_lossy(&out.stdout)))
}

/// The benchmarks a target's output says it ran unmeasured.
fn checked(stdout: &str) -> Vec<String> {
	stdout
		.lines()
		.filter_map(|line| line.strip_prefix("Checked: "))
		.map(str::to_owned)
		.collect()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn lists_each_checked_benchmark() {
		let out = "Harness: codspeed-criterion-compat v6.0.0\nChecked: benches/a.rs::g::f[1]\nSkipping: benches/a.rs::c (CodSpeed doesn't support custom iterations)\nChecked: benches/a.rs::g::h\n";
		assert_eq!(checked(out), ["benches/a.rs::g::f[1]", "benches/a.rs::g::h"]);
	}

	#[test]
	fn finds_rustflags_a_config_file_sets() {
		let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("target/configured-rustflags");
		let inner = root.join("ws/crate");
		std::fs::create_dir_all(&inner).unwrap();
		std::fs::create_dir_all(root.join(".cargo")).unwrap();
		let config = root.join(".cargo/config.toml");
		for (text, sets) in [
			("[build]\nrustflags = \"--cfg=foo\"\n", true),
			("[build]\nrustflags = [\"--cfg=foo\"]\n", true),
			(
				"[target.x86_64-unknown-linux-gnu]\nrustflags = [\"-Ctarget-cpu=native\"]\n",
				true,
			),
			("[target.'cfg(unix)']\nrustflags = []\n", true),
			("[build]\njobs = 4\n", false),
		] {
			std::fs::write(&config, text).unwrap();
			// Found through the canonical path, as target/ may be a link.
			let config = config.canonicalize().unwrap();
			let found = configured_rustflags(&inner, &[]).unwrap();
			// Cargo's home may set its own, which this host's tests cannot rule out.
			if sets {
				assert_eq!(found.as_deref(), Some(config.to_str().unwrap()), "{text}");
			} else {
				assert_ne!(found.as_deref(), Some(config.to_str().unwrap()), "{text}");
			}
		}
		let args = |list: &[&str]| list.iter().map(|s| (*s).to_owned()).collect::<Vec<_>>();
		std::fs::remove_file(&config).unwrap();
		let found = configured_rustflags(&inner, &args(&["--config", "build.rustflags=['-Cdebuginfo=2']"])).unwrap();
		assert_eq!(found.as_deref(), Some("--config build.rustflags=['-Cdebuginfo=2']"));
	}

	#[test]
	fn finds_the_manifest_the_build_args_select() {
		let args = |list: &[&str]| list.iter().map(|s| (*s).to_owned()).collect::<Vec<_>>();
		assert_eq!(
			manifest_path(&args(&["-p", "a", "--manifest-path", "x/Cargo.toml"])),
			Some("x/Cargo.toml")
		);
		assert_eq!(
			manifest_path(&args(&["--manifest-path=y/Cargo.toml"])),
			Some("y/Cargo.toml")
		);
		assert_eq!(manifest_path(&args(&["--bench", "v"])), None);
	}

	#[test]
	fn reads_a_built_benchmark_target() {
		let message = serde_json::json!({
			"reason": "compiler-artifact",
			"manifest_path": "/w/rs/net/Cargo.toml",
			"target": {"name": "varint", "kind": ["bench"]},
			"executable": "/w/target/release/deps/varint-0123",
		});
		let target = bench(&message).unwrap().unwrap();
		assert_eq!(target.name, "varint");
		assert_eq!(target.executable, Path::new("/w/target/release/deps/varint-0123"));
		assert_eq!(target.dir, Path::new("/w/rs/net"));
	}

	#[test]
	fn skips_what_is_no_benchmark_target() {
		let lib = serde_json::json!({"reason": "compiler-artifact", "target": {"name": "net", "kind": ["lib"]}});
		assert!(bench(&lib).unwrap().is_none());
		let done = serde_json::json!({"reason": "build-finished", "success": true});
		assert!(bench(&done).unwrap().is_none());
	}
}
