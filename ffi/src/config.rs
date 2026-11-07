//! The `ffi.toml` that says what to leave out, and the facade package the CLI resolves.

use std::path::{Path, PathBuf};

use serde::Deserialize;

/// What the export leaves out of the facade's API.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub struct Config {
	/// Paths from the facade's root left out of the export: a module, an item, or one
	/// method or field (`track::Producer::poll_closed`). Each must name something
	/// exported, or what a `cfg` leaves out of this build. As in rustdoc, a path a method or constant shares with a field names the method, and
	/// `field@` before it names the field.
	#[serde(default)]
	pub exclude: Vec<String>,
}

/// The keys `exclude` names the item at `shown` by, its path with its instances'
/// arguments, longest first: each prefix of it as is, then without the arguments, which
/// names the generic item's every instance. Each comes with its length.
pub fn keys(shown: &[String]) -> impl Iterator<Item = (usize, String)> + '_ {
	(1..=shown.len()).rev().flat_map(move |len| {
		let exact = shown[..len].join("::");
		let generic: Vec<&str> = shown[..len].iter().map(|s| s.split('<').next().unwrap_or(s)).collect();
		let generic = Some(generic.join("::")).filter(|generic| *generic != exact);
		std::iter::once(exact).chain(generic).map(move |key| (len, key))
	})
}

impl Config {
	pub fn load(path: &Path) -> Result<Self, String> {
		let text = std::fs::read_to_string(path).map_err(|err| format!("reading {}: {err}", path.display()))?;
		toml::from_str(&text).map_err(|err| format!("parsing {}: {err}", path.display()))
	}
}

/// The facade: the package whose library's `pub` API is exported, as the CLI resolves it
/// for the wrapper.
pub struct Package {
	pub name: String,
	/// Its library's crate name, Cargo's `[lib] name`: the prefix of every C name.
	pub lib: String,
	pub dir: PathBuf,
	/// The features rustc compiles it with, which the generated crate enables in it.
	pub features: Vec<String>,
}

impl Package {
	/// The first segment of its items' paths in the IR, which the glue depends on it by:
	/// its crate's name, as Rust code spells its paths.
	pub fn root(&self) -> String {
		self.lib.clone()
	}

	/// [`env::PACKAGE`]'s value.
	pub fn line(&self) -> String {
		format!("{}\t{}\t{}", self.name, self.lib, self.dir.display())
	}

	/// The package [`env::PACKAGE`] holds, without its features.
	pub fn parse(line: &str) -> Result<Self, String> {
		match line.split('\t').collect::<Vec<_>>()[..] {
			[name, lib, dir] => Ok(Self {
				name: name.to_string(),
				lib: lib.to_string(),
				dir: PathBuf::from(dir),
				features: Vec::new(),
			}),
			_ => Err(format!("{} is malformed: {line}", env::PACKAGE)),
		}
	}
}

/// Environment variables the CLI hands the wrapper.
pub mod env {
	/// The `ffi.toml` path, absolute.
	pub const CONFIG: &str = "RUST2_FFI_CONFIG";
	/// The output directory, absolute.
	pub const OUT: &str = "RUST2_FFI_OUT";
	/// The directory to write the census into instead of exporting, absolute.
	pub const CENSUS: &str = "RUST2_FFI_CENSUS";
	/// The facade's [`Package`](super::Package).
	pub const PACKAGE: &str = "RUST2_FFI_PACKAGE";
	/// The `[patch]` resolving the facade as cargo did, as TOML with each path absolute.
	pub const PATCH: &str = "RUST2_FFI_PATCH";
}
