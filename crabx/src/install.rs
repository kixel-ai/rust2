use crate::{
	cache::{self, Cache, Record},
	cli::{Args, Spec},
	fetchers::{Data, Fetcher, GhCrateMeta, QuickInstall, SignaturePolicy, TargetData, TargetDataErased},
	target::TargetTriple,
	trust,
};
use anyhow::{Context, Result, bail, ensure};
use binstalk::{
	helpers::{gh_api_client::GhApiClient, remote, tasks::AutoAbortJoinHandle},
	manifests::cargo_toml_binstall::{PkgMeta, Strategy},
};
use binstalk_bins::{self as bins, BinFile};
use semver::{Version, VersionReq};
use std::{collections::BTreeSet, fs, path::Path, process::Command, str::FromStr, sync::Arc, time::Duration};

pub async fn resolve(cache: &Cache, spec: &Spec, args: &Args) -> Result<Record> {
	let _lock = cache.lock()?;
	let version = match &spec.version {
		Some(version) => Some(version.clone()),
		None => cache.latest()?,
	};
	if let Some(record) = version.as_ref().map(|v| cache.load(v)).transpose()?.flatten() {
		return Ok(record);
	}
	let stage = tempfile::tempdir_in(&cache.root)?;
	fs::create_dir(stage.path().join("bin"))?;
	let record = if args.build {
		build(stage.path(), spec, version.as_ref())?
	} else {
		ensure!(
			!cfg!(all(target_os = "linux", target_env = "musl")),
			"prebuilt binaries require glibc on Linux; use --build on musl"
		);
		fetch(
			stage.path(),
			cache,
			&Spec {
				name: spec.name.clone(),
				version,
			},
			args.allow_quickinstall,
		)
		.await?
	};
	cache::atomic(&stage.path().join("record.json"), &serde_json::to_vec(&record)?)?;
	let destination = cache.version(&record.version);
	if cache.load(&record.version)?.is_none() {
		fs::rename(stage.path(), &destination)?;
	}
	if spec.version.is_none() && cache.latest()?.is_none_or(|v| v <= record.version) {
		cache.set_latest(&record.version)?;
	}
	Ok(record)
}

struct Package {
	name: String,
	version: Version,
	repository: Option<String>,
	meta: PkgMeta,
	bins: Vec<(String, bool)>,
}

async fn fetch(stage: &Path, cache: &Cache, spec: &Spec, quickinstall: bool) -> Result<Record> {
	let client = remote::Client::new(
		concat!("crabx/", env!("CARGO_PKG_VERSION")),
		None,
		false,
		Default::default(),
		1000.try_into()?,
		100.try_into()?,
		[],
	)?;
	let req = spec
		.version
		.as_ref()
		.map(|v| VersionReq::parse(&format!("={v}")))
		.transpose()?
		.unwrap_or(VersionReq::STAR);
	let (manifest, _) = binstalk::registry::Registry::default()
		.fetch_crate_matched(client.clone(), &spec.name, &req)
		.await?;
	let mut package = manifest.package.context("crate manifest has no package")?;
	ensure!(
		package.name.eq_ignore_ascii_case(&spec.name),
		"registry returned a different crate"
	);
	let version = package.version().clone();
	ensure!(req.matches(&version), "registry returned a different version");
	if let Some(record) = cache.load(&version)? {
		return Ok(record);
	}
	let repository = package.repository().map(str::to_owned);
	let meta = package.metadata.take().and_then(|m| m.binstall).unwrap_or_default();
	let mut names = BTreeSet::new();
	let mut binaries = Vec::new();
	for bin in manifest.bin {
		let name = bin.name.context("crate manifest has an unnamed binary")?;
		ensure!(
			bins::is_valid_bin_name(&name) && names.insert(name.clone()),
			"invalid or duplicate binary name {name:?}"
		);
		binaries.push((name, bin.required_features.is_empty()));
	}
	ensure!(!binaries.is_empty(), "crate contains no binaries");
	let package = Package {
		name: package.name,
		version,
		repository,
		meta,
		bins: binaries,
	};
	let gh = GhApiClient::new(client.clone(), None);
	let data = Arc::new(Data::new(
		package.name.as_str().into(),
		package.version.to_string().into(),
		package.repository.clone(),
	));
	// The author can name a single binary differently from its package.
	let alternate = match package.bins.as_slice() {
		[(name, _)] if name != &package.name => Some(Arc::new(Data::new(
			name.as_str().into(),
			package.version.to_string().into(),
			package.repository.clone(),
		))),
		_ => None,
	};
	let mut candidates = Vec::new();
	for data in std::iter::once(data.clone()).chain(alternate) {
		for target in host_targets() {
			let target_related_info = TargetTriple::from_str(&target)?;
			let meta = package.meta.merge_overrides(
				package
					.meta
					.overrides
					.get_matching(&target, &target_related_info.cfgs()),
			);
			let target: Arc<TargetDataErased> = Arc::new(TargetData {
				target,
				meta,
				target_related_info,
			});
			if !disabled(&target, Strategy::CrateMetaData) {
				candidates.push(GhCrateMeta::new(
					client.clone(),
					gh.clone(),
					data.clone(),
					target,
					SignaturePolicy::IfPresent,
				));
			}
		}
	}
	if quickinstall {
		for target in host_targets() {
			let target_related_info = TargetTriple::from_str(&target)?;
			let meta = package.meta.merge_overrides(
				package
					.meta
					.overrides
					.get_matching(&target, &target_related_info.cfgs()),
			);
			let target: Arc<TargetDataErased> = Arc::new(TargetData {
				target,
				meta,
				target_related_info,
			});
			if !disabled(&target, Strategy::QuickInstall) {
				candidates.push(QuickInstall::new(
					client.clone(),
					gh.clone(),
					data.clone(),
					target,
					SignaturePolicy::IfPresent,
				));
			}
		}
	}
	for fetcher in candidates {
		let found = tokio::time::timeout(
			Duration::from_secs(30),
			AutoAbortJoinHandle::new(fetcher.clone().find()),
		)
		.await
		.context("release discovery timed out")???;
		if !found {
			continue;
		}
		return package.install(stage, fetcher.as_ref()).await;
	}
	bail!(
		"no usable prebuilt release for {}@{} on {}; use --build to compile explicitly",
		package.name,
		package.version,
		binstalk::TARGET
	)
}

fn disabled(target: &TargetDataErased, strategy: Strategy) -> bool {
	target
		.meta
		.disabled_strategies
		.as_deref()
		.is_some_and(|strategies| strategies.contains(&strategy))
}

impl Package {
	async fn install(&self, stage: &Path, fetcher: &dyn Fetcher) -> Result<Record> {
		let download = stage.join("download");
		let extracted = fetcher.fetch_and_extract(&download).await?;
		let source = if fetcher.is_third_party() {
			"quickinstall".to_owned()
		} else {
			trust::verify(
				fetcher.receipt().context("missing archive digest")?,
				&self.version,
				stage,
			)
			.await?
		};
		let version = self.version.to_string();
		let install = stage.join("bin");
		let data = bins::Data {
			name: &self.name,
			target: fetcher.target(),
			version: &version,
			repo: self.repository.as_deref(),
			meta: fetcher.target_meta(),
			bin_path: &download,
			install_path: &install,
			target_related_info: &fetcher.target_data().target_related_info,
		};
		let bin_dir = layout(&data, &self.bins)?;
		let template = leon::Template::parse(&bin_dir)?;
		let mut files = Vec::new();
		let mut sources = BTreeSet::new();
		for (name, required) in &self.bins {
			let file = BinFile::new(&data, name, &template, true)?;
			if !extracted.has_file(&file.archive_source_path) {
				ensure!(
					!required,
					"release archive omits required binary {name:?} at {}",
					file.archive_source_path.display()
				);
				continue;
			}
			ensure!(
				sources.insert(file.source.clone()),
				"release maps several binaries to the same file"
			);
			files.push(file);
		}
		ensure!(!files.is_empty(), "release contains no runnable binary");
		let mut binaries = Vec::new();
		for file in files {
			file.install_bin_noclobber()?;
			binaries.push(file.base_name.to_string());
		}
		if download.is_dir() {
			fs::remove_dir_all(&download)?;
		} else {
			fs::remove_file(&download)?;
		}
		Ok(Record {
			version: self.version.clone(),
			bins: binaries,
			source,
		})
	}
}

fn layout(data: &bins::Data<'_>, binaries: &[(String, bool)]) -> Result<String> {
	if let Some(template) = &data.meta.bin_dir {
		return Ok(template.clone());
	}
	let direct = bins::infer_bin_dir_template(data, &mut |p| data.bin_path.join(p).is_dir());
	let parent = Path::new(direct.as_ref()).parent().unwrap_or_else(|| Path::new(""));
	let nested = format!("{}/{{ bin }}{{ binary-ext }}", parent.join("bin").display());
	for candidate in [direct.as_ref(), &nested] {
		let template = leon::Template::parse(candidate)?;
		let mut found = false;
		let mut complete = true;
		for (name, required) in binaries {
			let file = BinFile::new(data, name, &template, true)?;
			let exists = file.source.is_file();
			found |= exists;
			complete &= exists || !required;
		}
		if found && complete {
			return Ok(candidate.to_owned());
		}
	}
	// Let installation report precisely which required binary is absent.
	Ok(direct.into_owned())
}

fn build(stage: &std::path::Path, spec: &Spec, version: Option<&Version>) -> Result<Record> {
	let cargo = std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into());
	let mut command = Command::new(cargo);
	command
		.arg("install")
		.arg(&spec.name)
		.arg("--locked")
		.arg("--target")
		.arg(binstalk::TARGET)
		.arg("--root")
		.arg(stage)
		.arg("--index")
		.arg("sparse+https://index.crates.io/")
		.env_remove("GITHUB_TOKEN")
		.env_remove("GH_TOKEN");
	if let Some(version) = version {
		command.arg("--version").arg(format!("={version}"));
	}
	ensure!(
		command
			.status()
			.context("--build needs cargo and a Rust toolchain")?
			.success(),
		"cargo install failed"
	);
	#[derive(serde::Deserialize)]
	struct Installs {
		installs: std::collections::BTreeMap<String, serde_json::Value>,
	}
	let installs: Installs = serde_json::from_reader(fs::File::open(stage.join(".crates2.json"))?)?;
	ensure!(
		installs.installs.len() == 1,
		"cargo wrote unexpected installation metadata"
	);
	let key = installs.installs.keys().next().context("cargo installed no crate")?;
	let mut parts = key.split_whitespace();
	ensure!(
		parts.next() == Some(spec.name.as_str()),
		"cargo installed the wrong crate"
	);
	let actual = Version::parse(parts.next().context("cargo omitted the version")?)?;
	ensure!(
		version.is_none_or(|v| v == &actual),
		"cargo installed a different version"
	);
	let mut bins = Vec::new();
	for entry in fs::read_dir(stage.join("bin"))? {
		let entry = entry?;
		ensure!(entry.file_type()?.is_file(), "cargo installed a non-file binary");
		let name = entry
			.file_name()
			.into_string()
			.map_err(|_| anyhow::anyhow!("non-UTF-8 binary name"))?;
		ensure!(cache::valid_binary(&name), "invalid installed binary name");
		bins.push(name);
	}
	bins.sort();
	Ok(Record {
		version: actual,
		bins,
		source: "cargo install --locked".into(),
	})
}

fn host_targets() -> Vec<String> {
	let mut targets = vec![binstalk::TARGET.to_owned()];
	if cfg!(target_os = "macos") {
		targets.extend([
			"universal-apple-darwin".to_owned(),
			"universal2-apple-darwin".to_owned(),
		]);
	}
	targets
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn author_layouts_include_nested_bin_and_respect_metadata() {
		let temp = tempfile::tempdir().unwrap();
		let target = TargetTriple::from_str(binstalk::TARGET).unwrap();
		let directory = format!("moq-cli-v0.14.16-{}", binstalk::TARGET);
		let nested = temp.path().join(&directory).join("bin");
		fs::create_dir_all(&nested).unwrap();
		let name = format!("moq{}", std::env::consts::EXE_SUFFIX);
		fs::write(nested.join(&name), b"native binary fixture").unwrap();
		let mut data = bins::Data {
			name: "moq-cli",
			target: binstalk::TARGET,
			version: "0.14.16",
			repo: None,
			meta: PkgMeta::default(),
			bin_path: temp.path(),
			install_path: temp.path(),
			target_related_info: &target,
		};
		let binaries = [("moq".to_owned(), true)];
		let source = |data: &bins::Data<'_>| {
			let chosen = layout(data, &binaries).unwrap();
			let template = leon::Template::parse(&chosen).unwrap();
			BinFile::new(data, "moq", &template, true).unwrap().source
		};
		assert_eq!(source(&data), nested.join(&name));
		// A direct executable wins even when the archive also has a bin directory.
		let direct = temp.path().join(&directory).join(&name);
		fs::write(&direct, b"direct native fixture").unwrap();
		assert_eq!(source(&data), direct);
		data.meta.bin_dir = Some("explicit/{ bin }{ binary-ext }".into());
		assert_eq!(source(&data), temp.path().join("explicit").join(&name));
	}
}
