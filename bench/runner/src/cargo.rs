//! A Cargo workspace's benchmarks, written with CodSpeed's compatibility crates.
//!
//! Built with `--cfg codspeed`, as `cargo codspeed build` builds them, a benchmark target
//! runs each benchmark once around CodSpeed's instrument hooks, printing `Checked: <name>`
//! where nothing measures it. With `CODSPEED_ISOLATED_BENCHMARK` set to a name it runs that
//! benchmark alone, and with `CODSPEED_PROCESS_ISOLATION=false` it runs every benchmark in
//! its own process rather than one child each.

use std::collections::{BTreeMap, BTreeSet};
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use serde_json::Value;

use crate::Job;

/// The cfg CodSpeed's compatibility crates take to run each benchmark once around its hooks.
const CFG: &str = "--cfg=codspeed";

/// The hidden argument that runs this executable as `cargo bench`'s runner, recording how
/// Cargo launches each benchmark target instead of running it.
pub const CAPTURE: &str = "__capture";

/// Where the capturing runner writes each launch, set on the `cargo bench` it runs under.
const CAPTURE_DIR: &str = "RUST2_BENCH_CAPTURE";

/// A benchmark target cargo built.
struct Target {
	name: String,
	executable: PathBuf,
}

/// How Cargo launches a benchmark target: its directory and its whole environment, with
/// `[env]`, `CARGO_MANIFEST_DIR`, and the library path Cargo sets.
#[derive(Clone)]
struct Launch {
	dir: PathBuf,
	env: BTreeMap<String, String>,
}

impl Launch {
	/// `executable` as Cargo launches it.
	fn command(&self, executable: &Path, root: &Path) -> Command {
		let mut command = Command::new(executable);
		command
			.current_dir(&self.dir)
			.envs(&self.env)
			.env("CODSPEED_CARGO_WORKSPACE_ROOT", root);
		command
	}
}

/// Runs as `cargo bench`'s runner: records the target it is handed, as JSON in the directory
/// `CAPTURE_DIR` names, and exits without running it.
pub fn capture(args: &[String]) -> Result<(), String> {
	let executable = args.first().ok_or("the capturing runner takes the target to launch")?;
	let out = std::env::var_os(CAPTURE_DIR).ok_or(format!("the capturing runner needs {CAPTURE_DIR}"))?;
	let dir = std::env::current_dir().map_err(|e| format!("cannot read the working directory: {e}"))?;
	let mut env = serde_json::Map::new();
	for (key, value) in std::env::vars_os() {
		let (Some(key), Some(value)) = (key.to_str(), value.to_str()) else {
			return Err(format!("cargo set {} to a value not UTF-8", key.to_string_lossy()));
		};
		if key != CAPTURE_DIR {
			env.insert(key.to_owned(), value.into());
		}
	}
	let launch = serde_json::json!({
		"executable": executable,
		"dir": dir.to_str().ok_or("the target's directory is not UTF-8")?,
		"env": env,
	});
	let path = Path::new(&out).join(format!("{}.json", std::process::id()));
	use std::io::Write;
	use std::os::unix::fs::OpenOptionsExt;
	std::fs::OpenOptions::new()
		.write(true)
		.create_new(true)
		.mode(0o600)
		.open(&path)
		.and_then(|mut file| file.write_all(launch.to_string().as_bytes()))
		.map_err(|e| format!("cannot write {}: {e}", path.display()))
}

/// A private directory for the capturing runner's launches, removed when dropped.
struct Captures(PathBuf);

impl Captures {
	fn create() -> Result<Self, String> {
		use std::os::unix::fs::DirBuilderExt;
		let dir = std::env::temp_dir().join(format!("rust2-bench-launches-{}", std::process::id()));
		// A directory already there may be another user's, so it is never reused.
		std::fs::DirBuilder::new()
			.mode(0o700)
			.create(&dir)
			.map_err(|e| format!("cannot create {}: {e}", dir.display()))?;
		Ok(Self(dir))
	}
}

impl Drop for Captures {
	fn drop(&mut self) {
		let _ = std::fs::remove_dir_all(&self.0);
	}
}

/// The launches the capturing runner recorded in `dir`, by executable.
fn launches(dir: &Path) -> Result<BTreeMap<PathBuf, Launch>, String> {
	let mut launches = BTreeMap::new();
	for entry in std::fs::read_dir(dir).map_err(|e| format!("cannot read {}: {e}", dir.display()))? {
		let path = entry.map_err(|e| format!("cannot read {}: {e}", dir.display()))?.path();
		let text = std::fs::read_to_string(&path).map_err(|e| format!("cannot read {}: {e}", path.display()))?;
		let value: Value = serde_json::from_str(&text).map_err(|e| format!("cannot parse {}: {e}", path.display()))?;
		let field = |name: &str| {
			value[name]
				.as_str()
				.map(PathBuf::from)
				.ok_or(format!("{} has no {name}", path.display()))
		};
		let env = value["env"]
			.as_object()
			.ok_or(format!("{} has no env", path.display()))?
			.iter()
			.map(|(key, value)| {
				Ok((
					key.clone(),
					value
						.as_str()
						.ok_or(format!("{} has a value not a string", path.display()))?
						.to_owned(),
				))
			})
			.collect::<Result<_, String>>()?;
		launches.insert(
			field("executable")?,
			Launch {
				dir: field("dir")?,
				env,
			},
		);
	}
	Ok(launches)
}

/// Builds the benchmark targets of the workspace in `dir`, passing `args` to `cargo bench`,
/// and lists the benchmarks each runs as a job, one benchmark a process.
/// Unless `strict`, a workspace with no benchmark target has no jobs, as a compare's base
/// before the first one does.
pub fn jobs(dir: &Path, args: &[String], strict: bool) -> Result<Vec<Job>, String> {
	let root = workspace_root(dir, args)?;
	let targets = build(dir, args)?;
	if targets.is_empty() {
		if strict {
			return Err("cargo built no benchmark target".to_owned());
		}
		return Ok(Vec::new());
	}
	// Every target lists its benchmarks at once: each runs them natively, far faster than
	// under Valgrind.
	let listed = std::thread::scope(|scope| {
		let handles: Vec<_> = targets
			.iter()
			.map(|(target, launch)| scope.spawn(|| list(target, launch, &root)))
			.collect();
		handles
			.into_iter()
			.map(|h| h.join().expect("a listing thread panicked"))
			.collect::<Vec<_>>()
	});
	let mut seen = BTreeSet::new();
	let mut jobs = Vec::new();
	for ((target, launch), names) in targets.iter().zip(listed) {
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
			let mut command = launch.command(&target.executable, &root);
			command.env("CODSPEED_ISOLATED_BENCHMARK", &name);
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
	// A config may move the workspace, so the build's configs locate it too.
	let mut rest = args.iter();
	while let Some(arg) = rest.next() {
		if arg == "--" {
			break;
		}
		if arg == "--config" {
			cargo.arg(arg).args(rest.next());
		} else if arg.starts_with("--config=") {
			cargo.arg(arg);
		}
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
/// among `args`, or a config file it reads from `dir` up, or from Cargo's home, following
/// each file's `include`s.
fn configured_rustflags(dir: &Path, args: &[String]) -> Result<Option<String>, String> {
	for (key, _) in std::env::vars_os() {
		let key = key.to_string_lossy();
		if key == "CARGO_BUILD_RUSTFLAGS"
			|| ((key.starts_with("CARGO_TARGET_") || key.starts_with("CARGO_HOST_")) && key.ends_with("_RUSTFLAGS"))
		{
			return Ok(Some(format!("the environment's {key}")));
		}
	}
	let dir = dir
		.canonicalize()
		.map_err(|e| format!("cannot resolve {}: {e}", dir.display()))?;
	let mut args = args.iter();
	while let Some(arg) = args.next() {
		if arg == "--" {
			break;
		}
		let config = if arg == "--config" {
			args.next().map(String::as_str)
		} else {
			arg.strip_prefix("--config=")
		};
		let Some(config) = config else {
			continue;
		};
		// Cargo reads a relative path from its own directory, which is `dir`.
		let path = dir.join(config);
		let found = if path.is_file() {
			config_file(&path, &mut Vec::new())?
		} else {
			let table: toml::Table = config
				.parse()
				.map_err(|e| format!("cannot parse --config {config}: {e}"))?;
			config_table(&table, &dir, &format!("--config {config}"), &mut Vec::new())?
		};
		if found.is_some() {
			return Ok(found);
		}
	}
	let home = std::env::var_os("CARGO_HOME")
		.map(PathBuf::from)
		.or_else(|| std::env::var_os("HOME").map(|home| Path::new(&home).join(".cargo")));
	let dirs = dir.ancestors().map(|dir| dir.join(".cargo")).chain(home);
	for cargo in dirs {
		for name in ["config.toml", "config"] {
			let path = cargo.join(name);
			if path.is_file()
				&& let Some(found) = config_file(&path, &mut Vec::new())?
			{
				return Ok(Some(found));
			}
		}
	}
	Ok(None)
}

/// Where the config file at `path`, or one it includes, sets rustflags, if anywhere.
/// `including` holds the files whose includes led here, so a cycle fails instead of recursing.
fn config_file(path: &Path, including: &mut Vec<PathBuf>) -> Result<Option<String>, String> {
	let canonical = path
		.canonicalize()
		.map_err(|e| format!("cannot resolve {}: {e}", path.display()))?;
	if including.contains(&canonical) {
		return Err(format!("{} includes itself", path.display()));
	}
	let text = std::fs::read_to_string(path).map_err(|e| format!("cannot read {}: {e}", path.display()))?;
	let table: toml::Table = text
		.parse()
		.map_err(|e| format!("cannot parse {}: {e}", path.display()))?;
	let base = path.parent().expect("a config file has a directory");
	including.push(canonical);
	let found = config_table(&table, base, &path.display().to_string(), including);
	including.pop();
	found
}

/// Where one config table, from `source`, sets rustflags, if anywhere, reading the files it
/// includes relative to `base`.
fn config_table(
	table: &toml::Table,
	base: &Path,
	source: &str,
	including: &mut Vec<PathBuf>,
) -> Result<Option<String>, String> {
	let build = table.get("build").and_then(|build| build.get("rustflags")).is_some();
	// `target.<triple>.rustflags`, and the unstable host config's `host.rustflags` and
	// `host.<triple>.rustflags`.
	let per = |name: &str| {
		table
			.get(name)
			.and_then(toml::Value::as_table)
			.is_some_and(|tables| tables.values().any(|table| table.get("rustflags").is_some()))
	};
	let host = table.get("host").and_then(|host| host.get("rustflags")).is_some();
	if build || host || per("target") || per("host") {
		return Ok(Some(source.to_owned()));
	}
	// `include` is a path, or an array of paths or of tables with a `path`.
	let includes = match table.get("include") {
		None => Vec::new(),
		Some(toml::Value::Array(items)) => items.iter().collect(),
		Some(item) => vec![item],
	};
	for item in includes {
		let path = match item {
			toml::Value::String(path) => path,
			toml::Value::Table(item) => item
				.get("path")
				.and_then(toml::Value::as_str)
				.ok_or_else(|| format!("{source} includes a table without a path"))?,
			_ => return Err(format!("{source} includes something not a path")),
		};
		let path = base.join(path);
		// An optional include may be absent; Cargo itself fails a required one.
		if path.is_file()
			&& let Some(found) = config_file(&path, including)?
		{
			return Ok(Some(found));
		}
	}
	Ok(None)
}

/// The manifest Cargo args select, when they name one.
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

/// Builds every benchmark target with CodSpeed's cfg, as `cargo codspeed build` does, and
/// returns each with how Cargo launches it: `cargo bench` runs each through this executable as
/// its runner, which records the launch rather than running it.
fn build(dir: &Path, args: &[String]) -> Result<Vec<(Target, Launch)>, String> {
	let exe = std::env::current_exe().map_err(|e| format!("cannot find rust2-bench's own executable: {e}"))?;
	let exe = exe.to_str().ok_or("rust2-bench's own path is not UTF-8")?;
	// A launch's environment holds whatever secrets the caller exported, so it goes to a
	// directory only this user can read, removed once read.
	let captures = Captures::create()?;
	let captures = captures.0.as_path();
	let runner = format!(
		"target.'cfg(all())'.runner=[{}, {}]",
		toml::Value::from(exe),
		toml::Value::from(CAPTURE)
	);
	let mut cargo = Command::new("cargo");
	cargo.args(["bench", "--message-format=json-render-diagnostics", "--config", &runner]);
	cargo.env(CAPTURE_DIR, captures);
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
	let built = (|| {
		let mut targets = Vec::new();
		for line in BufReader::new(stdout).lines() {
			let line = line.map_err(|e| format!("cannot read cargo's output: {e}"))?;
			let message: Value =
				serde_json::from_str(&line).map_err(|e| format!("cargo printed a line not JSON ({e}): {line}"))?;
			if let Some(target) = bench(&message)? {
				targets.push(target);
			}
		}
		Ok::<_, String>(targets)
	})();
	let built = match built {
		Ok(built) => built,
		Err(err) => {
			// Stop the build rather than leave it writing the target directory the next step uses.
			let _ = child.kill();
			let _ = child.wait();
			return Err(err);
		}
	};
	let status = child.wait().map_err(|e| format!("cannot wait for cargo: {e}"))?;
	if !status.success() {
		return Err(format!("cargo bench failed ({status})"));
	}
	let mut launches = launches(captures)?;
	// The fingerprint reads GLIBC_TUNABLES from rust2-bench's environment, so a Cargo `[env]`
	// that sets another would count under a label that does not describe it.
	let tunables = std::env::var("GLIBC_TUNABLES").ok();
	for launch in launches.values() {
		if launch.env.get("GLIBC_TUNABLES") != tunables.as_ref() {
			return Err(
				"a Cargo [env] sets GLIBC_TUNABLES, which the fingerprint would not see: set it in the environment instead"
					.to_owned(),
			);
		}
	}
	built
		.into_iter()
		.map(|target| {
			let launch = launches.remove(&target.executable).ok_or(format!(
				"cargo launched benchmark target {} through a runner other than rust2-bench's: \
				 a config or the environment sets one",
				target.name
			))?;
			Ok((target, launch))
		})
		.collect()
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
	Ok(Some(Target { name, executable }))
}

/// The benchmarks `target` runs, as it names them when nothing measures it.
fn list(target: &Target, launch: &Launch, root: &Path) -> Result<Vec<String>, String> {
	let out = launch
		.command(&target.executable, root)
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
			("[host]\nrustflags = []\n", true),
			("[host.x86_64-unknown-linux-gnu]\nrustflags = []\n", true),
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

		// An include, in each of its forms, read relative to the including file.
		let flags = root.join(".cargo/flags.toml");
		std::fs::write(&flags, "[build]\nrustflags = [\"--cfg=foo\"]\n").unwrap();
		let flags = flags.canonicalize().unwrap();
		for text in [
			"include = \"flags.toml\"\n",
			"include = [\"absent.toml\", \"flags.toml\"]\n",
			"include = [{ path = \"flags.toml\", optional = true }]\n",
		] {
			std::fs::write(&config, text).unwrap();
			let found = configured_rustflags(&inner, &[]).unwrap();
			assert_eq!(found.as_deref(), Some(flags.to_str().unwrap()), "{text}");
		}
		std::fs::remove_file(&config).unwrap();

		// A relative --config file is the build directory's, not the runner's.
		std::fs::write(inner.join("extra.toml"), "include = \"../../.cargo/flags.toml\"\n").unwrap();
		let found = configured_rustflags(&inner, &args(&["--config", "extra.toml"])).unwrap();
		let found = Path::new(&found.unwrap()).canonicalize().unwrap();
		assert_eq!(found, flags);
		std::fs::remove_file(inner.join("extra.toml")).unwrap();
		std::fs::remove_file(&flags).unwrap();

		// An include cycle is refused rather than followed forever.
		std::fs::write(&config, "include = \"config.toml\"\n").unwrap();
		let err = configured_rustflags(&inner, &[]).unwrap_err();
		assert!(err.ends_with("includes itself"), "{err}");
		std::fs::remove_file(&config).unwrap();
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
	}

	#[test]
	fn skips_what_is_no_benchmark_target() {
		let lib = serde_json::json!({"reason": "compiler-artifact", "target": {"name": "net", "kind": ["lib"]}});
		assert!(bench(&lib).unwrap().is_none());
		let done = serde_json::json!({"reason": "build-finished", "success": true});
		assert!(bench(&done).unwrap().is_none());
	}
}
