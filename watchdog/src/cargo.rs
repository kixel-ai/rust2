//! The cargo side: which `test` commands the watchdog takes, the build pass that maps each
//! test executable to its package, and where the workspace's nextest config lives.

use std::collections::HashMap;
use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use anyhow::{Context, Result, bail};
use serde_json::Value;

/// A `test` command for cargo or a tool that takes its arguments, such as `cross`.
pub struct Test {
	pub program: OsString,
	pub args: Vec<OsString>,
	// Where cargo's own arguments end: the first `--`, after which the test binaries' begin.
	split: usize,
}

impl Test {
	/// Check that the command runs `test` with libtest's per-test lines.
	pub fn new(command: Vec<OsString>) -> Result<Test> {
		let mut command = command.into_iter();
		let Some(program) = command.next() else {
			bail!("no command to run");
		};
		let args: Vec<OsString> = command.collect();
		let split = args.iter().position(|a| a == "--").unwrap_or(args.len());
		let cargo = &args[..split];
		let flag = |arg: &OsString, names: &[&str]| {
			let arg = arg.to_string_lossy();
			names
				.iter()
				.any(|name| arg == *name || arg.starts_with(&format!("{name}=")))
		};
		if !cargo.iter().any(|a| a == "test") {
			bail!("the command must run `test`, such as `cargo test` or `cross test`");
		}
		for arg in cargo {
			if flag(arg, &["--no-run", "--message-format"]) {
				bail!("the watchdog runs the build itself; drop {}", arg.display());
			}
			if flag(arg, &["--doc"]) {
				bail!("`--doc` builds nothing the watchdog can map to a package");
			}
		}
		for arg in &args {
			if flag(arg, &["-q", "--quiet", "--format"]) {
				bail!("{} hides the test names the watchdog times; drop it", arg.display());
			}
		}
		Ok(Test { program, args, split })
	}

	/// The same command, building the tests without running them and reporting each artifact
	/// as JSON on stdout.
	pub fn build(&self) -> Command {
		let mut command = Command::new(&self.program);
		command
			.args(&self.args[..self.split])
			.args(["--no-run", "--message-format=json-render-diagnostics"])
			.args(&self.args[self.split..]);
		command
	}

	/// The command as given.
	pub fn run(&self) -> Command {
		let mut command = Command::new(&self.program);
		command.args(&self.args);
		command
	}

	/// The workspace's `.config/nextest.toml`.
	pub fn nextest_config(&self) -> Result<PathBuf> {
		let mut locate = Command::new("cargo");
		locate.args(["locate-project", "--workspace", "--message-format", "plain"]);
		if let Some(path) = self.manifest_path() {
			locate.arg("--manifest-path").arg(path);
		}
		let output = locate
			.stderr(Stdio::inherit())
			.output()
			.context("running `cargo locate-project`")?;
		if !output.status.success() {
			bail!("`cargo locate-project` failed: {}", output.status);
		}
		let manifest = String::from_utf8(output.stdout).context("`cargo locate-project`")?;
		let root = Path::new(manifest.trim_end())
			.parent()
			.context("`cargo locate-project` printed no directory")?;
		Ok(root.join(".config/nextest.toml"))
	}

	fn manifest_path(&self) -> Option<&OsStr> {
		let cargo = &self.args[..self.split];
		cargo.iter().enumerate().find_map(|(i, arg)| {
			if arg == "--manifest-path" {
				cargo.get(i + 1).map(OsString::as_os_str)
			} else {
				arg.to_str()?.strip_prefix("--manifest-path=").map(OsStr::new)
			}
		})
	}
}

/// Which package built each test executable and each library, from the build pass's
/// `compiler-artifact` messages.
#[derive(Debug, Default)]
pub struct Artifacts {
	// By file name, which holds the unit's hash, so a path from inside a cross container or
	// relative to cargo's working directory finds it too.
	executables: HashMap<String, String>,
	// A workspace library's crate name, as a `Doc-tests` line names it; `None` where two
	// packages share it.
	libraries: HashMap<String, Option<String>>,
}

impl Artifacts {
	/// Read cargo's JSON messages, one per line; lines that are not one go to `other`.
	pub fn parse(stdout: &str, mut other: impl FnMut(&str)) -> Result<Artifacts> {
		let mut artifacts = Artifacts::default();
		for line in stdout.lines() {
			let Ok(message) = serde_json::from_str::<Value>(line) else {
				other(line);
				continue;
			};
			if message["reason"] != "compiler-artifact" {
				continue;
			}
			let id = message["package_id"]
				.as_str()
				.context("a compiler-artifact message has no package_id")?;
			let package = package_name(id)?;
			if let Some(executable) = message["executable"].as_str() {
				let name = file_name(executable)?;
				artifacts.executables.insert(name.to_owned(), package.clone());
			}
			let kinds = message["target"]["kind"].as_array();
			let library = kinds.is_some_and(|kinds| {
				kinds.iter().any(|kind| {
					matches!(
						kind.as_str(),
						Some("lib" | "rlib" | "dylib" | "cdylib" | "staticlib" | "proc-macro")
					)
				})
			});
			// Doc-tests run only for path packages, and a registry dependency may share a
			// crate name with one.
			if library && id.starts_with("path+") {
				let name = message["target"]["name"]
					.as_str()
					.context("a compiler-artifact message has no target name")?
					.replace('-', "_");
				artifacts
					.libraries
					.entry(name)
					.and_modify(|known| {
						if known.as_deref() != Some(package.as_str()) {
							*known = None;
						}
					})
					.or_insert(Some(package));
			}
		}
		Ok(artifacts)
	}

	/// The package that built the test executable at `path`, as a `Running` line prints it.
	pub fn executable(&self, path: &str) -> Result<&str> {
		let name = file_name(path)?;
		match self.executables.get(name) {
			Some(package) => Ok(package),
			None => bail!("cargo ran {path}, which the build pass did not build"),
		}
	}

	/// The package of the library whose doc tests run as crate `name`.
	pub fn library(&self, name: &str) -> Result<&str> {
		match self.libraries.get(name) {
			Some(Some(package)) => Ok(package),
			Some(None) => bail!("more than one package builds a library named {name}"),
			None => bail!("cargo ran the doc tests of {name}, whose library the build pass did not build"),
		}
	}
}

fn file_name(path: &str) -> Result<&str> {
	Path::new(path)
		.file_name()
		.and_then(OsStr::to_str)
		.with_context(|| format!("{path} names no file"))
}

// A package ID spec, `URL#name@version`, or `URL#version` where the URL's last segment is the
// name; before cargo 1.77, `name version (URL)`.
fn package_name(id: &str) -> Result<String> {
	if let Some((name, _)) = id.split_once(' ') {
		return Ok(name.to_owned());
	}
	let Some((url, fragment)) = id.rsplit_once('#') else {
		bail!("cannot read a package name from package ID {id}");
	};
	if let Some((name, _)) = fragment.split_once('@') {
		return Ok(name.to_owned());
	}
	let path = url.split(['?', '#']).next().unwrap_or(url);
	match path.trim_end_matches('/').rsplit('/').next() {
		Some(name) if !name.is_empty() => Ok(name.to_owned()),
		_ => bail!("cannot read a package name from package ID {id}"),
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	fn test(command: &str) -> Result<Test> {
		Test::new(command.split(' ').map(OsString::from).collect())
	}

	fn args(command: &Command) -> Vec<String> {
		command.get_args().map(|a| a.to_string_lossy().into_owned()).collect()
	}

	#[test]
	fn the_build_pass_goes_before_the_binaries_arguments() {
		let command = test("cross test -p rind --release -- --test-threads=1").unwrap();
		assert_eq!(command.build().get_program(), "cross");
		assert_eq!(
			args(&command.build()),
			[
				"test",
				"-p",
				"rind",
				"--release",
				"--no-run",
				"--message-format=json-render-diagnostics",
				"--",
				"--test-threads=1"
			]
		);
		assert_eq!(
			args(&command.run()),
			["test", "-p", "rind", "--release", "--", "--test-threads=1"]
		);
		let command = test("cargo +nightly test").unwrap();
		assert_eq!(
			args(&command.build()),
			[
				"+nightly",
				"test",
				"--no-run",
				"--message-format=json-render-diagnostics"
			]
		);
	}

	#[test]
	fn commands_the_watchdog_cannot_time_fail() {
		for (command, says) in [
			("cargo build", "must run `test`"),
			("cargo build -- test", "must run `test`"),
			("cargo test --no-run", "drop --no-run"),
			("cargo test --message-format=json", "drop --message-format=json"),
			("cargo test --doc", "--doc"),
			("cargo test -q", "-q hides"),
			("cargo test -- --format=terse", "--format=terse hides"),
		] {
			let error = test(command).err().unwrap().to_string();
			assert!(error.contains(says), "{command}: {error}");
		}
	}

	#[test]
	fn manifest_path_in_either_form() {
		assert_eq!(
			test("cargo test --manifest-path a/Cargo.toml").unwrap().manifest_path(),
			Some(OsStr::new("a/Cargo.toml"))
		);
		assert_eq!(
			test("cargo test --manifest-path=b/Cargo.toml -- --manifest-path=c")
				.unwrap()
				.manifest_path(),
			Some(OsStr::new("b/Cargo.toml"))
		);
	}

	#[test]
	fn package_names_from_every_id_form() {
		for (id, name) in [
			("path+file:///w/odd-dir#odd@0.1.0", "odd"),
			("path+file:///w/rind#0.1.0", "rind"),
			(
				"registry+https://github.com/rust-lang/crates.io-index#regex@1.13.2",
				"regex",
			),
			("git+https://github.com/a/b.git?rev=abc#c@0.1.0", "c"),
			("git+https://github.com/a/b?branch=main#0.2.0", "b"),
			("rind 0.1.0 (path+file:///w/rind)", "rind"),
		] {
			assert_eq!(package_name(id).unwrap(), name, "{id}");
		}
	}

	#[test]
	fn artifacts_map_executables_and_libraries() {
		let stdout = [
            r#"{"reason":"compiler-artifact","package_id":"path+file:///w/a#pkg-a@0.1.0","target":{"name":"pkg_a","kind":["lib"]},"executable":null}"#,
            r#"{"reason":"compiler-artifact","package_id":"path+file:///w/a#pkg-a@0.1.0","target":{"name":"slow","kind":["test"]},"executable":"/w/target/debug/deps/slow-0c44"}"#,
            r#"{"reason":"compiler-artifact","package_id":"registry+https://x#rand@0.9.0","target":{"name":"rand","kind":["lib"]},"executable":null}"#,
            r#"{"reason":"compiler-artifact","package_id":"path+file:///w/r#my-rand@0.1.0","target":{"name":"rand","kind":["lib"]},"executable":null}"#,
            r#"{"reason":"compiler-artifact","package_id":"path+file:///w/s#other-rand@0.1.0","target":{"name":"rand","kind":["lib"]},"executable":null}"#,
            r#"{"reason":"build-finished","success":true}"#,
            "not json",
        ]
        .join("\n");
		let mut other = Vec::new();
		let artifacts = Artifacts::parse(&stdout, |line| other.push(line.to_owned())).unwrap();
		assert_eq!(other, ["not json"]);
		assert_eq!(artifacts.executable("target/debug/deps/slow-0c44").unwrap(), "pkg-a");
		assert_eq!(artifacts.executable("/target/debug/deps/slow-0c44").unwrap(), "pkg-a");
		assert!(artifacts.executable("target/debug/deps/slow-ffff").is_err());
		assert_eq!(artifacts.library("pkg_a").unwrap(), "pkg-a");
		assert!(
			artifacts
				.library("rand")
				.unwrap_err()
				.to_string()
				.contains("more than one")
		);
	}
}
