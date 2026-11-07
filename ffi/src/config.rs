//! The `ffi.toml` that says what to export.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use serde::Deserialize;

/// What to export: one package's crate, and other packages' it depends on, into one
/// library.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub struct Config {
	/// The cargo package whose library is checked, and exported first.
	pub package: String,
	/// The prefix of every C name, the package's name in snake case by default.
	pub prefix: Option<String>,
	/// Paths from the crate root left out of the export: a module, an item, or one
	/// method or field (`track::Producer::poll_closed`). Each must name something
	/// exported, or what a `cfg` leaves out of this build by its declared path. As in
	/// rustdoc, a path a method or constant shares with a field names the method, and
	/// `field@` before it names the field.
	#[serde(default)]
	pub exclude: Vec<String>,
	/// C names, without the prefix, for paths from the crate root whose names the rule
	/// makes clash: `"session::Stats" = "session_stats_snapshot"`. A type's methods are
	/// named through its new name. Paths match as `exclude`'s do, so an instance's renames
	/// it alone, and a field is named by its path as `exclude` names it.
	#[serde(default)]
	pub rename: BTreeMap<String, String>,
	/// Other packages exported into the same library, each a dependency of `package`, by
	/// name: an export names another's types across crates.
	#[serde(default)]
	pub crates: BTreeMap<String, Items>,
}

/// What leaves another export's items out and renames them, as `exclude` and `rename` do
/// the package's.
#[derive(Debug, Default, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub struct Items {
	#[serde(default)]
	pub exclude: Vec<String>,
	#[serde(default)]
	pub rename: BTreeMap<String, String>,
}

/// The keys `exclude` and `rename` name the item at `shown` by, its path with its
/// instances' arguments, longest first: each prefix of it as is, then without the
/// arguments, which names the generic item's every instance. Each comes with its length.
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
		let config: Self = toml::from_str(&text).map_err(|err| format!("parsing {}: {err}", path.display()))?;
		if config.crates.contains_key(&config.package) {
			return Err(format!(
				"{}: [crates.{}] names the package itself, whose `exclude` and `rename` are top-level",
				path.display(),
				config.package
			));
		}
		Ok(config)
	}

	/// The prefix of every C name.
	pub fn prefix(&self) -> String {
		self.prefix.clone().unwrap_or_else(|| self.package.replace('-', "_"))
	}
}

/// An export's package, as the CLI resolves it for the wrapper.
pub struct Package {
	pub name: String,
	/// The name the configured package calls its library by, which binds another export
	/// to the crate rustc loaded: the configured package's own crate name.
	pub krate: String,
	pub dir: PathBuf,
	/// The features the generated crate enables in it, beyond what the configured
	/// package's enable.
	pub features: Vec<String>,
}

impl Package {
	/// The first segment of its items' paths in the IR, which the glue depends on it by:
	/// its name in snake case, unique among the exports, where the names the configured
	/// package calls them by may coincide with each other and its own.
	pub fn root(&self) -> String {
		self.name.replace('-', "_")
	}

	/// One line of [`env::CRATES`].
	pub fn line(&self) -> String {
		let dir = self.dir.display();
		format!("{}\t{}\t{dir}\t{}\n", self.name, self.krate, self.features.join(","))
	}

	/// The packages [`env::CRATES`] lists, a line each.
	pub fn parse(lines: &str) -> Result<Vec<Self>, String> {
		(lines.lines())
			.map(|line| match line.split('\t').collect::<Vec<_>>()[..] {
				[name, krate, dir, features] => Ok(Self {
					name: name.to_string(),
					krate: krate.to_string(),
					dir: PathBuf::from(dir),
					features: features
						.split(',')
						.filter(|f| !f.is_empty())
						.map(str::to_string)
						.collect(),
				}),
				_ => Err(format!("{} holds a malformed line: {line}", env::CRATES)),
			})
			.collect()
	}
}

/// Environment variables the CLI hands the wrapper.
pub mod env {
	/// The `ffi.toml` path, absolute.
	pub const CONFIG: &str = "RUST2_FFI_CONFIG";
	/// The output directory, absolute.
	pub const OUT: &str = "RUST2_FFI_OUT";
	/// The directory to write each export's census into instead of exporting, absolute.
	pub const CENSUS: &str = "RUST2_FFI_CENSUS";
	/// Each export's [`Package`](super::Package), a line each.
	pub const CRATES: &str = "RUST2_FFI_CRATES";
	/// The `[patch]` resolving the configured package as cargo did, as TOML with each
	/// path absolute.
	pub const PATCH: &str = "RUST2_FFI_PATCH";
}
