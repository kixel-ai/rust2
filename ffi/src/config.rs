//! The facade package the CLI resolves, and the environment it hands the wrapper.

use std::path::PathBuf;

/// The facade: the package whose library's `pub` API is exported, as the CLI resolves it
/// for the wrapper.
pub(crate) struct Package {
	pub(crate) name: String,
	/// Its library's crate name, Cargo's `[lib] name`: the prefix of every C name.
	pub(crate) lib: String,
	/// Its Cargo version, which the web binding's `package.json` takes.
	pub(crate) version: String,
	pub(crate) dir: PathBuf,
	/// The features rustc compiles it with, which the generated crate enables in it.
	pub(crate) features: Vec<String>,
}

impl Package {
	/// The first segment of its items' paths in the IR, which the glue depends on it by:
	/// its crate's name, as Rust code spells its paths.
	pub(crate) fn root(&self) -> String {
		self.lib.clone()
	}

	/// [`env::PACKAGE`]'s value.
	pub(crate) fn line(&self) -> String {
		format!("{}\t{}\t{}\t{}", self.name, self.lib, self.version, self.dir.display())
	}

	/// The package [`env::PACKAGE`] holds, without its features.
	pub(crate) fn parse(line: &str) -> Result<Self, String> {
		match line.split('\t').collect::<Vec<_>>()[..] {
			[name, lib, version, dir] => Ok(Self {
				name: name.to_string(),
				lib: lib.to_string(),
				version: version.to_string(),
				dir: PathBuf::from(dir),
				features: Vec::new(),
			}),
			_ => Err(format!("{} is malformed: {line}", env::PACKAGE)),
		}
	}
}

/// Environment variables the CLI hands the wrapper.
pub(crate) mod env {
	/// The output directory, absolute.
	pub(crate) const OUT: &str = "RUST2_FFI_OUT";
	/// The directory to write the census into instead of exporting, absolute.
	pub(crate) const CENSUS: &str = "RUST2_FFI_CENSUS";
	/// The file to write the target's API into instead of exporting, absolute, for the
	/// export of several targets to merge.
	pub(crate) const API: &str = "RUST2_FFI_API";
	/// The API an earlier export wrote, absolute, which the export's must not break.
	pub(crate) const BASELINE: &str = "RUST2_FFI_BASELINE";
	/// The other targets' APIs the export merges with its own, as a list of paths.
	pub(crate) const MERGE: &str = "RUST2_FFI_MERGE";
	/// The directories a staged API's spans are relative to, as a list of paths: the
	/// workspace's root and cargo's home, so an API staged on another host keeps them.
	pub(crate) const ROOTS: &str = "RUST2_FFI_ROOTS";
	/// The facade's [`Package`](super::Package).
	pub(crate) const PACKAGE: &str = "RUST2_FFI_PACKAGE";
	/// The `[patch]` resolving the facade as cargo did, as TOML with each path absolute.
	pub(crate) const PATCH: &str = "RUST2_FFI_PATCH";
	/// The dependency the glue would name each crate the facade builds with by, as a TOML
	/// table from its library's crate name to its keys.
	pub(crate) const CRATES: &str = "RUST2_FFI_CRATES";
	/// The `PROFILE` cargo gives a build script of the check: `release` or `debug`.
	pub(crate) const PROFILE: &str = "RUST2_FFI_PROFILE";
	/// The directory cargo builds the check's profile in, or empty where a build under
	/// another profile could share it.
	pub(crate) const PROFILE_DIR: &str = "RUST2_FFI_PROFILE_DIR";
	/// The `CARGO_PROFILE_*` variables that may change the check's profile, comma-separated.
	pub(crate) const PROFILE_VARS: &str = "RUST2_FFI_PROFILE_VARS";
	/// The file the wrapper writes the target's `cfg`s into, as cargo's rustc prints them
	/// under the check's `RUSTFLAGS`, absolute.
	pub(crate) const CFGS: &str = "RUST2_FFI_CFGS";
	/// The rustc cargo resolved, which a build script's probe runs, absolute or as cargo
	/// found it: set wherever cargo runs this binary as rustc.
	pub(crate) const RUSTC: &str = "RUST2_FFI_RUSTC";
	/// The rustflags the export adds after the consumer's, separated by `0x1f` as cargo's
	/// `CARGO_ENCODED_RUSTFLAGS` are.
	pub(crate) const ADDED: &str = "RUST2_FFI_ADDED";
	/// The directory to record what cargo passes each target's crates into, as
	/// [`crate::driver::Recorded`], set for the `cargo metadata` that finds them.
	pub(crate) const RECORD: &str = "RUST2_FFI_RECORD";
}
