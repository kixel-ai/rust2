use crate::cli::Spec;
use anyhow::{Context, Result, bail, ensure};
use semver::Version;
use serde::{Deserialize, Serialize};
use std::{
	fs::{self, File, TryLockError},
	io::Write,
	path::{Path, PathBuf},
};

pub struct Cache {
	pub root: PathBuf,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Record {
	pub version: Version,
	pub bins: Vec<String>,
	pub source: String,
}

impl Record {
	pub fn binary(&self, requested: Option<&str>, krate: &str) -> Result<&str> {
		let extension = std::env::consts::EXE_SUFFIX;
		if let Some(requested) = requested {
			let name = format!("{requested}{extension}");
			return self
				.bins
				.iter()
				.find(|bin| **bin == name)
				.map(String::as_str)
				.with_context(|| format!("no binary {requested:?}; available: {}", self.bins.join(", ")));
		}
		let name = format!("{krate}{extension}");
		let mut matching = self.bins.iter().filter(|bin| bin.eq_ignore_ascii_case(&name));
		if let Some(bin) = matching.next() {
			ensure!(
				matching.next().is_none(),
				"several binaries match the crate name; choose --bin NAME: {}",
				self.bins.join(", ")
			);
			return Ok(bin);
		}
		match self.bins.as_slice() {
			[bin] => Ok(bin),
			[] => bail!("the crate contains no binaries"),
			bins => bail!(
				"the crate contains several binaries; choose --bin NAME: {}",
				bins.join(", ")
			),
		}
	}
}

impl Cache {
	pub fn new(spec: &Spec, mode: &str) -> Result<Self> {
		let base = directories::BaseDirs::new().context("cannot find the user's cache directory")?;
		let root = base
			.cache_dir()
			.join("crabx")
			.join("v1")
			.join(binstalk::TARGET)
			.join(mode)
			.join(spec.name.to_ascii_lowercase());
		Ok(Self { root })
	}

	pub fn version(&self, version: &Version) -> PathBuf {
		self.root.join(version.to_string())
	}

	pub fn latest(&self) -> Result<Option<Version>> {
		match fs::read_to_string(self.root.join("latest")) {
			Ok(value) => Ok(Some(
				Version::parse(value.trim()).context("corrupt latest version in cache")?,
			)),
			Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
			Err(e) => Err(e.into()),
		}
	}

	pub fn load(&self, version: &Version) -> Result<Option<Record>> {
		let path = self.version(version);
		// Check the atomically published directory first. A missing record cannot
		// then be confused with a complete directory published after that read.
		if !path
			.try_exists()
			.with_context(|| format!("inspect cache entry {}", path.display()))?
		{
			return Ok(None);
		}
		let file = File::open(path.join("record.json"))
			.with_context(|| format!("incomplete cache entry {}", path.display()))?;
		let record: Record =
			serde_json::from_reader(file).with_context(|| format!("corrupt cache record in {}", path.display()))?;
		ensure!(
			&record.version == version && !record.bins.is_empty(),
			"invalid cache record in {}",
			path.display()
		);
		for bin in &record.bins {
			ensure!(
				valid_binary(bin) && path.join("bin").join(bin).is_file(),
				"missing or invalid cached binary {bin:?} in {}",
				path.display()
			);
		}
		Ok(Some(record))
	}

	pub fn lock(&self) -> Result<File> {
		fs::create_dir_all(&self.root)?;
		let file = File::options()
			.create(true)
			.truncate(false)
			.read(true)
			.write(true)
			.open(self.root.join("lock"))?;
		file.lock()?;
		Ok(file)
	}

	/// Create staging space while holding the installation lock.
	pub fn stage(&self) -> Result<tempfile::TempDir> {
		// The caller holds the install lock, so any earlier staging directory
		// has lost its owner. Published version directories have semver names.
		for entry in fs::read_dir(&self.root)? {
			let entry = entry?;
			if entry
				.file_name()
				.to_str()
				.is_some_and(|name| name.starts_with("install-"))
				&& entry.file_type()?.is_dir()
			{
				fs::remove_dir_all(entry.path())?;
			}
		}
		Ok(tempfile::Builder::new().prefix("install-").tempdir_in(&self.root)?)
	}

	pub fn try_refresh_lock(&self) -> Result<Option<File>> {
		let file = File::options()
			.create(true)
			.truncate(false)
			.read(true)
			.write(true)
			.open(self.root.join("refresh.lock"))?;
		match file.try_lock() {
			Ok(()) => Ok(Some(file)),
			Err(TryLockError::WouldBlock) => Ok(None),
			Err(TryLockError::Error(e)) => Err(e.into()),
		}
	}

	pub fn set_latest(&self, version: &Version) -> Result<()> {
		atomic(&self.root.join("latest"), version.to_string().as_bytes())
	}
}

pub fn valid_binary(name: &str) -> bool {
	!name.is_empty() && name != "." && name != ".." && !name.contains(['/', '\\', ':', '\0'])
}

pub fn atomic(path: &Path, bytes: &[u8]) -> Result<()> {
	let parent = path.parent().context("cache file has no parent")?;
	fs::create_dir_all(parent)?;
	let mut file = tempfile::NamedTempFile::new_in(parent)?;
	file.write_all(bytes)?;
	file.as_file().sync_all()?;
	file.persist(path)?;
	Ok(())
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn selection_rejects_ambiguity_and_preserves_names() {
		let mut record = Record {
			version: Version::new(1, 2, 3),
			bins: vec![
				format!("first{}", std::env::consts::EXE_SUFFIX),
				format!("second{}", std::env::consts::EXE_SUFFIX),
			],
			source: "author".into(),
		};
		assert!(record.binary(None, "other").is_err());
		assert!(record.binary(Some("missing"), "first").is_err());
		assert_eq!(record.binary(Some("second"), "other").unwrap(), record.bins[1]);
		assert_eq!(record.binary(None, "first").unwrap(), record.bins[0]);
		assert_eq!(record.binary(None, "FiRsT").unwrap(), record.bins[0]);
		record.bins.pop();
		assert_eq!(record.binary(None, "other").unwrap(), record.bins[0]);
		record.bins.push(format!("FIRST{}", std::env::consts::EXE_SUFFIX));
		assert!(record.binary(None, "first").is_err());
		assert_eq!(record.binary(Some("first"), "first").unwrap(), record.bins[0]);
	}

	#[test]
	fn entries_are_atomic_and_incomplete_entries_fail() {
		let temp = tempfile::tempdir().unwrap();
		let cache = Cache {
			root: temp.path().to_owned(),
		};
		let version = Version::new(2, 3, 4);
		assert!(cache.load(&version).unwrap().is_none());
		cache.set_latest(&version).unwrap();
		assert_eq!(cache.latest().unwrap(), Some(version.clone()));
		fs::create_dir(cache.version(&version)).unwrap();
		let path = cache.version(&version);
		assert!(format!("{:#}", cache.load(&version).unwrap_err()).contains(&path.display().to_string()));
		fs::write(path.join("record.json"), "malformed").unwrap();
		assert!(format!("{:#}", cache.load(&version).unwrap_err()).contains(&path.display().to_string()));
		fs::write(
			path.join("record.json"),
			r#"{"version":"2.3.4","bins":["tool"],"source":"fixture"}"#,
		)
		.unwrap();
		assert!(format!("{:#}", cache.load(&version).unwrap_err()).contains(&path.display().to_string()));
		for name in ["../evil", "C:evil", "a/b", "a\\b", ".", "..", ""] {
			assert!(!valid_binary(name));
		}
	}

	#[test]
	fn staging_reclaims_interrupted_installs_and_preserves_versions() {
		let temp = tempfile::tempdir().unwrap();
		let cache = Cache {
			root: temp.path().to_owned(),
		};
		let _lock = cache.lock().unwrap();
		fs::create_dir_all(cache.root.join("install-orphan/bin")).unwrap();
		fs::create_dir(cache.root.join("1.2.3")).unwrap();
		let stage = cache.stage().unwrap();
		assert!(stage.path().is_dir());
		assert!(!cache.root.join("install-orphan").exists());
		assert!(cache.root.join("1.2.3").is_dir());
	}
}
