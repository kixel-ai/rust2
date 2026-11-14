//! Build steps the tests run, each replayed from a stamp while what it reads is unchanged,
//! and file writes that leave a file alone when it already holds what they would write.
//!
//! The target directory's `tmp`, where the steps write, persists between runs, so a second
//! run of the tests compiles and links nothing it did not change. Keys are on content, not
//! mtimes: the export rewrites nothing unchanged, but a copy or a checkout still moves them.

use std::collections::HashMap;
use std::ffi::OsString;
use std::hash::{DefaultHasher, Hasher};
use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus, Output};
use std::sync::{Arc, Mutex, OnceLock};

/// A build step: its stamp holds the key it last ran with, what it printed, and a digest of
/// what it wrote, so a run with the same key replays that output instead of running it.
pub struct Step<'a> {
	/// Where the key and the output are kept between runs.
	pub stamp: &'a Path,
	/// The files and directories the step reads besides its program, by content.
	pub reads: &'a [&'a Path],
	/// The files and directories it writes, which a replay needs as it left them; each is
	/// removed before the step runs, so nothing an earlier run wrote outlives it.
	pub writes: &'a [&'a Path],
}

impl Step<'_> {
	/// The output of `cmd`, which must succeed.
	pub fn run(&self, cmd: &mut Command) -> Output {
		let output = self.execute(cmd, false);
		super::check(cmd, &output);
		output
	}

	/// The output of `cmd`, whether it succeeds or fails, as a compile a test expects to
	/// fail does.
	pub fn output(&self, cmd: &mut Command) -> Output {
		self.execute(cmd, true)
	}

	fn execute(&self, cmd: &mut Command, failure: bool) -> Output {
		let key = key(cmd, self.reads);
		if let Some(output) = self.replay(&key) {
			return output;
		}
		// No stamp may outlive a run that stops partway.
		remove(self.stamp);
		for path in self.writes {
			remove(path);
		}
		let output = super::output(cmd);
		// A failed build is kept only where the test expects one: any other is run again,
		// and one a signal ended may not be what its inputs make.
		if (output.status.success() || failure) && output.status.code().is_some() {
			self.record(&key, &output);
		}
		output
	}

	fn replay(&self, key: &str) -> Option<Output> {
		let stamp = std::fs::read(self.stamp).ok()?;
		let mut fields = Fields(&stamp);
		if fields.next()? != key.as_bytes() || fields.next()? != digest(self.writes).to_le_bytes() {
			return None;
		}
		let status = status(i64::from_le_bytes(fields.next()?.try_into().ok()?));
		let stdout = fields.next()?.to_vec();
		let stderr = fields.next()?.to_vec();
		Some(Output { status, stdout, stderr })
	}

	fn record(&self, key: &str, output: &Output) {
		let code = raw(output.status);
		let mut stamp = Vec::new();
		for field in [
			key.as_bytes(),
			&digest(self.writes).to_le_bytes(),
			&code.to_le_bytes(),
			&output.stdout,
			&output.stderr,
		] {
			stamp.extend_from_slice(&(field.len() as u64).to_le_bytes());
			stamp.extend_from_slice(field);
		}
		// Whole or not at all: a stamp cut short would fail to parse, but say so plainly.
		let partial = self.stamp.with_extension("partial");
		std::fs::create_dir_all(self.stamp.parent().unwrap()).unwrap();
		std::fs::write(&partial, stamp).unwrap();
		std::fs::rename(&partial, self.stamp).unwrap();
	}
}

/// The length-prefixed fields of a stamp.
struct Fields<'a>(&'a [u8]);

impl<'a> Iterator for Fields<'a> {
	type Item = &'a [u8];

	fn next(&mut self) -> Option<&'a [u8]> {
		let (len, rest) = self.0.split_first_chunk::<8>()?;
		let len = usize::try_from(u64::from_le_bytes(*len)).ok()?;
		let field = rest.get(..len)?;
		self.0 = &rest[len..];
		Some(field)
	}
}

#[cfg(unix)]
fn raw(status: ExitStatus) -> i64 {
	use std::os::unix::process::ExitStatusExt;
	status.into_raw().into()
}

#[cfg(unix)]
fn status(raw: i64) -> ExitStatus {
	use std::os::unix::process::ExitStatusExt;
	ExitStatus::from_raw(raw as i32)
}

#[cfg(windows)]
fn raw(status: ExitStatus) -> i64 {
	status.code().unwrap().into()
}

#[cfg(windows)]
fn status(raw: i64) -> ExitStatus {
	use std::os::windows::process::ExitStatusExt;
	ExitStatus::from_raw(raw as u32)
}

/// What a step's output depends on besides its own files: its program, by path and version,
/// its arguments, directory, and environment, and the content of each file it `reads`.
fn key(cmd: &Command, reads: &[&Path]) -> String {
	let program = program(cmd);
	let mut key = format!("program {program:?}\nversion {:?}\n", version(&program));
	for arg in cmd.get_args() {
		key += &format!("arg {arg:?}\n");
	}
	key += &format!("dir {:?}\n", cmd.get_current_dir());
	let set: Vec<_> = cmd.get_envs().collect();
	for (name, value) in inherited() {
		if !set.iter().any(|(set, _)| set == name) {
			key += &format!("env {name:?}={value:?}\n");
		}
	}
	for (name, value) in set {
		key += &format!("env {name:?}={value:?}\n");
	}
	for path in reads {
		key += &format!("read {path:?} {:016x}\n", digest(&[path]));
	}
	key
}

/// The program `cmd` runs, resolved through `PATH` as the command would and through links,
/// so another toolchain on `PATH`, as another dev shell puts there, is another key.
fn program(cmd: &Command) -> PathBuf {
	let program = Path::new(cmd.get_program());
	let path = match program.components().count() {
		1 => super::which(&program.to_string_lossy()),
		_ => program.to_path_buf(),
	};
	std::fs::canonicalize(&path).unwrap_or_else(|err| panic!("resolving {}: {err}", path.display()))
}

/// What `program` says its version is, asked once each run: a toolchain updated in place,
/// or picked by `xcode-select`, keeps its path.
fn version(program: &Path) -> String {
	static VERSIONS: OnceLock<Mutex<HashMap<PathBuf, Arc<OnceLock<String>>>>> = OnceLock::new();
	let once = VERSIONS
		.get_or_init(Default::default)
		.lock()
		.unwrap()
		.entry(program.to_path_buf())
		.or_default()
		.clone();
	once.get_or_init(|| {
		let flag: &[&str] = match program.file_stem().and_then(|stem| stem.to_str()) {
			Some("go") => &["version"],
			Some("kotlinc") => &["-version"],
			// MSVC's compiler prints its banner bare.
			Some("cl") => &[],
			_ => &["--version"],
		};
		let output = Command::new(program)
			.args(flag)
			.output()
			.unwrap_or_else(|err| panic!("running {}: {err}", program.display()));
		format!(
			"{}{}",
			String::from_utf8_lossy(&output.stdout),
			String::from_utf8_lossy(&output.stderr)
		)
	})
	.clone()
}

/// The inherited variables a compiler, linker, or JVM reads, by prefix, such as the Nix C
/// wrapper's flags, the SDK an Apple or MSVC toolchain finds, and Go's; others, such as a
/// shell's temporary directory, would make each run's key new. So would a directory on
/// `PATH` in that temporary directory, such as a Cargo shim's session, which is left out.
fn inherited() -> &'static [(OsString, OsString)] {
	const PREFIXES: &[&str] = &[
		"PATH",
		"NIX_",
		"CC",
		"CXX",
		"CFLAGS",
		"CPPFLAGS",
		"CXXFLAGS",
		"LD",
		"AR",
		"CPATH",
		"C_INCLUDE_PATH",
		"CPLUS_INCLUDE_PATH",
		"LIBRARY_PATH",
		"INCLUDE",
		"LIB",
		"SDKROOT",
		"DEVELOPER_DIR",
		"MACOSX_",
		"JAVA",
		"KOTLIN",
		"GO",
		"CGO_",
		"DART",
		"SWIFT",
		"VC",
		"WINDOWSSDK",
		"UCRT",
	];
	// A Nix shell's own build directory, which each shell makes anew.
	const VOLATILE: &[&str] = &["NIX_BUILD_TOP"];
	static ENV: OnceLock<Vec<(OsString, OsString)>> = OnceLock::new();
	ENV.get_or_init(|| {
		let mut env: Vec<(OsString, OsString)> = std::env::vars_os()
			.filter(|(name, _)| {
				let upper = name.to_string_lossy().to_uppercase();
				PREFIXES.iter().any(|prefix| upper.starts_with(prefix)) && !VOLATILE.contains(&upper.as_str())
			})
			.map(
				|(name, value)| match name.to_string_lossy().eq_ignore_ascii_case("PATH") {
					true => (name, path()),
					false => (name, value),
				},
			)
			.collect();
		env.sort();
		env
	})
}

/// `PATH` without the directories in the temporary one, where a shell, or a Cargo shim's
/// session, puts what it makes anew each run.
pub fn path() -> OsString {
	let temp = std::env::temp_dir();
	let path = std::env::var_os("PATH").unwrap_or_default();
	let kept = std::env::split_paths(&path).filter(|dir| !dir.starts_with(&temp));
	std::env::join_paths(kept).unwrap()
}

/// A digest of the content of `paths`, each a file or a directory of them, by relative
/// path; a missing one digests as missing.
fn digest(paths: &[&Path]) -> u64 {
	let mut hasher = DefaultHasher::new();
	for path in paths {
		walk(path, Path::new(""), &mut hasher);
	}
	hasher.finish()
}

fn walk(path: &Path, rel: &Path, hasher: &mut DefaultHasher) {
	hasher.write(rel.as_os_str().as_encoded_bytes());
	hasher.write_u8(0);
	match std::fs::metadata(path) {
		Err(err) if err.kind() == std::io::ErrorKind::NotFound => hasher.write_u8(0),
		Err(err) => panic!("reading {}: {err}", path.display()),
		Ok(meta) if meta.is_dir() => {
			hasher.write_u8(1);
			let mut entries: Vec<_> = std::fs::read_dir(path)
				.unwrap_or_else(|err| panic!("reading {}: {err}", path.display()))
				.map(|entry| entry.unwrap().file_name())
				.collect();
			entries.sort();
			for name in entries {
				walk(&path.join(&name), &rel.join(&name), hasher);
			}
		}
		Ok(_) => {
			hasher.write_u8(2);
			let bytes = std::fs::read(path).unwrap_or_else(|err| panic!("reading {}: {err}", path.display()));
			hasher.write_usize(bytes.len());
			hasher.write(&bytes);
		}
	}
}

fn remove(path: &Path) {
	let removed = match std::fs::symlink_metadata(path) {
		Ok(meta) if meta.is_dir() => std::fs::remove_dir_all(path),
		Ok(_) => std::fs::remove_file(path),
		Err(err) if err.kind() == std::io::ErrorKind::NotFound => return,
		Err(err) => Err(err),
	};
	removed.unwrap_or_else(|err| panic!("removing {}: {err}", path.display()));
}

/// Write `text` to `path` unless it already holds it, creating its directory.
pub fn write(path: &Path, text: impl AsRef<[u8]>) {
	let text = text.as_ref();
	if std::fs::read(path).is_ok_and(|old| old == text) {
		return;
	}
	std::fs::create_dir_all(path.parent().unwrap()).unwrap();
	std::fs::write(path, text).unwrap_or_else(|err| panic!("writing {}: {err}", path.display()));
}

/// Copy the file `from` to `to` unless `to` already holds what it holds.
pub fn copy(from: &Path, to: &Path) {
	let bytes = std::fs::read(from).unwrap_or_else(|err| panic!("reading {}: {err}", from.display()));
	write(to, bytes);
}

/// Make the directory `to` a copy of `from`, rewriting only what differs and removing what
/// `from` lacks.
pub fn mirror(from: &Path, to: &Path) {
	std::fs::create_dir_all(to).unwrap();
	let mut names = Vec::new();
	for entry in std::fs::read_dir(from).unwrap() {
		let entry = entry.unwrap();
		let target = to.join(entry.file_name());
		if entry.file_type().unwrap().is_dir() {
			if target.is_file() {
				remove(&target);
			}
			mirror(&entry.path(), &target);
		} else {
			if target.is_dir() {
				remove(&target);
			}
			copy(&entry.path(), &target);
		}
		names.push(entry.file_name());
	}
	for entry in std::fs::read_dir(to).unwrap() {
		let entry = entry.unwrap();
		if !names.contains(&entry.file_name()) {
			remove(&entry.path());
		}
	}
}

/// The stamp of a step that writes `output`: beside it, named after all of it.
pub fn stamp(output: &Path) -> PathBuf {
	let mut name = output.as_os_str().to_owned();
	name.push(".stamp");
	name.into()
}
