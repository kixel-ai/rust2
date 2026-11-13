use anyhow::{Result, bail, ensure};
use clap::Parser;
use semver::Version;
use std::ffi::OsString;

/// Run a Rust CLI from a prebuilt release.
#[derive(Debug, Parser)]
#[command(
	version,
	trailing_var_arg = true,
	override_usage = "crabx [OPTIONS] <CRATE[@VERSION]> [ARGS]..."
)]
pub struct Args {
	/// Compile with cargo install instead of downloading a binary.
	#[arg(long)]
	pub build: bool,
	/// Allow cargo-quickinstall's third-party builds.
	#[arg(long, conflicts_with = "build")]
	pub allow_quickinstall: bool,
	/// Run this binary when the crate ships several.
	#[arg(long, value_name = "NAME")]
	pub bin: Option<String>,
	/// Update the sparse-index version for the next invocation.
	#[arg(long, hide = true)]
	pub refresh: bool,
	/// A crates.io crate, optionally pinned, followed by its arguments.
	#[arg(value_name = "CRATE[@VERSION]", required = true, num_args = 1.., allow_hyphen_values = true)]
	pub command: Vec<OsString>,
}

impl Args {
	pub fn mode(&self) -> &'static str {
		if self.build {
			"build"
		} else if self.allow_quickinstall {
			"quickinstall"
		} else {
			"author"
		}
	}
}

pub struct Spec {
	pub name: String,
	pub version: Option<Version>,
}

impl Spec {
	pub fn parse(value: &str) -> Result<Self> {
		let (name, version) = match value.split_once('@') {
			Some((name, version)) => (name, Some(Version::parse(version)?)),
			None => (value, None),
		};
		ensure!(valid_name(name), "invalid crates.io crate name {name:?}");
		if version.as_ref().is_some_and(|v| !v.build.is_empty()) {
			bail!("version build metadata is not supported by crates.io");
		}
		Ok(Self {
			name: name.to_owned(),
			version,
		})
	}
}

pub fn valid_name(name: &str) -> bool {
	!name.is_empty()
		&& name.len() <= 64
		&& name.as_bytes()[0].is_ascii_alphabetic()
		&& name
			.bytes()
			.all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_')
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn exact_versions_and_opaque_arguments() {
		for invalid in ["../x", "x/y", "-x", "x@^1", "x@1", "x@1.2.3@4", "x@1.2.3+other"] {
			assert!(Spec::parse(invalid).is_err(), "{invalid}");
		}
		assert_eq!(
			Spec::parse("foo-bar@1.2.3-rc.1").unwrap().version.unwrap().to_string(),
			"1.2.3-rc.1"
		);
		let args = Args::try_parse_from(["crabx", "foo", "--build", "--bin", "x", "--", "a b"]).unwrap();
		assert!(!args.build);
		assert_eq!(args.command, ["foo", "--build", "--bin", "x", "--", "a b"]);
		let args = Args::try_parse_from(["crabx", "--build", "foo", "--build"]).unwrap();
		assert!(args.build);
		assert_eq!(args.command, ["foo", "--build"]);
	}
}
