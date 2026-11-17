//! The `ts.toml` that says what to translate.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use serde::Deserialize;

/// What to translate from one crate.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub(crate) struct Config {
	/// The cargo package whose library is translated.
	pub(crate) package: String,
	/// Paths from the crate root to the items to translate: a function, or a type with
	/// its non-generic methods. Everything they reach comes along.
	pub(crate) roots: Vec<String>,
	/// Paths from the crate root to types whose `Display` and `Debug` impls come along,
	/// so callers can print them as Rust does.
	#[serde(default)]
	pub(crate) printable: Vec<String>,
	/// Hand-written TypeScript that stands in for a Rust item, by the item's path: from
	/// the crate root, or else from a crate it depends on, by the name the crate gives it.
	#[serde(default)]
	pub(crate) bind: BTreeMap<String, Bind>,
	/// The directory `ts.toml` is in, which binding files are relative to.
	#[serde(skip)]
	pub(crate) dir: PathBuf,
}

/// The TypeScript file that stands in for an item.
#[derive(Debug, Deserialize)]
#[serde(from = "BindToml")]
pub(crate) struct Bind {
	/// Relative to `ts.toml`, and copied to the same path in the output.
	pub(crate) file: PathBuf,
	/// Whether a bound type's value drops through its `[Symbol.dispose]` where Rust drops
	/// it. Without it, dropping one does nothing JS sees.
	pub(crate) drop: bool,
}

/// A binding as written: its file alone, or a table.
#[derive(Deserialize)]
#[serde(untagged)]
enum BindToml {
	File(PathBuf),
	Table(BindTable),
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct BindTable {
	file: PathBuf,
	#[serde(default)]
	drop: bool,
}

impl From<BindToml> for Bind {
	fn from(bind: BindToml) -> Self {
		match bind {
			BindToml::File(file) => Self { file, drop: false },
			BindToml::Table(BindTable { file, drop }) => Self { file, drop },
		}
	}
}

impl Config {
	pub(crate) fn load(path: &Path) -> Result<Self, String> {
		let text = std::fs::read_to_string(path).map_err(|err| format!("reading {}: {err}", path.display()))?;
		let mut config: Self = toml::from_str(&text).map_err(|err| format!("parsing {}: {err}", path.display()))?;
		config.dir = path.parent().unwrap_or(Path::new("")).to_path_buf();
		Ok(config)
	}
}

/// Environment variables the CLI hands the wrapper.
pub(crate) mod env {
	/// The `ts.toml` path, absolute.
	pub(crate) const CONFIG: &str = "RUST2_TS_CONFIG";
	/// The output directory, absolute.
	pub(crate) const OUT: &str = "RUST2_TS_OUT";
	/// Where to write a census instead of translating, absolute.
	pub(crate) const CENSUS: &str = "RUST2_TS_CENSUS";
	/// A sysroot of std checked for translation, which flake.nix builds once per toolchain.
	pub(crate) const SYSROOT: &str = "RUST2_TS_SYSROOT";
}
