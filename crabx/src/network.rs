use crate::{
	cache::{self, Cache},
	cli::{Args, Spec},
};
use anyhow::{Context, Result, ensure};
use reqwest::{Client, Response};
use semver::Version;
use serde::Deserialize;
use std::{
	fs::{self, File},
	io::{BufReader, Read, Write},
	process::{Command, Stdio},
	time::Duration,
};

pub fn runtime() -> Result<tokio::runtime::Runtime> {
	Ok(tokio::runtime::Builder::new_current_thread().enable_all().build()?)
}

pub fn client() -> Result<Client> {
	Ok(Client::builder()
		.https_only(true)
		.user_agent(concat!("crabx/", env!("CARGO_PKG_VERSION")))
		.timeout(Duration::from_secs(30))
		.build()?)
}

// Spool responses to disk instead of retaining the sparse index or pages of bundles.
pub async fn spool(mut response: Response, directory: &std::path::Path) -> Result<tempfile::NamedTempFile> {
	let mut file = tempfile::NamedTempFile::new_in(directory)?;
	while let Some(chunk) = response.chunk().await? {
		file.write_all(&chunk)?;
	}
	Ok(file)
}

#[derive(Deserialize)]
struct IndexEntry {
	vers: Version,
	yanked: bool,
}

fn latest(reader: impl Read) -> Result<Version> {
	// Ignore dependency lists as they stream past, retaining only the best version.
	let entries = serde_json::Deserializer::from_reader(reader).into_iter::<IndexEntry>();
	let mut latest: Option<Version> = None;
	for entry in entries {
		let entry = entry?;
		if !entry.yanked && entry.vers.pre.is_empty() && latest.as_ref().is_none_or(|v| *v < entry.vers) {
			latest = Some(entry.vers);
		}
	}
	latest.context("crate has no non-yanked stable version")
}

fn index_path(name: &str) -> String {
	let name = name.to_ascii_lowercase();
	match name.len() {
		1 => format!("1/{name}"),
		2 => format!("2/{name}"),
		3 => format!("3/{}/{name}", &name[..1]),
		_ => format!("{}/{}/{name}", &name[..2], &name[2..4]),
	}
}

pub async fn refresh(cache: &Cache, spec: &Spec, args: &Args) -> Result<()> {
	ensure!(spec.version.is_none(), "only unpinned crates can refresh");
	let Some(_lock) = cache.try_refresh_lock()? else {
		return Ok(());
	};
	let etag_path = cache.root.join("index-etag");
	let mut request = client()?.get(format!("https://index.crates.io/{}", index_path(&spec.name)));
	match fs::read_to_string(&etag_path) {
		Ok(etag) => {
			request = request.header(reqwest::header::IF_NONE_MATCH, etag);
		}
		Err(e) if e.kind() == std::io::ErrorKind::NotFound => (),
		Err(e) => return Err(e.into()),
	}
	let response = request.send().await?.error_for_status()?;
	if response.status() == reqwest::StatusCode::NOT_MODIFIED {
		return Ok(());
	}
	let etag = response.headers().get(reqwest::header::ETAG).cloned();
	let file = spool(response, &cache.root).await?;
	let version = latest(BufReader::new(File::open(file.path())?))?;
	// Publish only an installed version, retaining the working entry on failure.
	crate::install::resolve(
		cache,
		&Spec {
			name: spec.name.clone(),
			version: Some(version.clone()),
		},
		args,
	)
	.await?;
	let _lock = cache.lock()?;
	cache.set_latest(&version)?;
	// Remember a validator only after installation, so failures remain retryable.
	if let Some(etag) = etag {
		cache::atomic(&etag_path, etag.as_bytes())?;
	} else if etag_path.try_exists()? {
		fs::remove_file(etag_path)?;
	}
	Ok(())
}

pub fn spawn_refresh(spec: &Spec, args: &Args) -> Result<()> {
	let mut command = Command::new(std::env::current_exe()?);
	command.arg("--refresh");
	if args.build {
		command.arg("--build");
	}
	if args.allow_quickinstall {
		command.arg("--allow-quickinstall");
	}
	command
		.arg(&spec.name)
		.stdin(Stdio::null())
		.stdout(Stdio::null())
		.stderr(Stdio::null());
	#[cfg(unix)]
	{
		use std::os::unix::process::CommandExt;
		// Reparent the updater so a long-running CLI never inherits a zombie.
		// Only async-signal-safe libc calls run between fork and exec.
		unsafe {
			command.pre_exec(|| {
				if libc::setsid() < 0 {
					return Err(std::io::Error::last_os_error());
				}
				match libc::fork() {
					-1 => Err(std::io::Error::last_os_error()),
					0 => Ok(()),
					_ => libc::_exit(0),
				}
			});
		}
	}
	#[cfg(windows)]
	{
		use std::os::windows::process::CommandExt;
		command.creation_flags(0x00000008 | 0x00000200);
	}
	let mut child = command.spawn().context("start sparse-index refresh")?;
	#[cfg(unix)]
	ensure!(child.wait()?.success(), "start sparse-index refresh failed");
	#[cfg(windows)]
	let _ = &mut child;
	Ok(())
}

pub fn refresh_error(cache: &Cache, error: &anyhow::Error) -> Result<()> {
	cache::atomic(&cache.root.join("refresh-error"), format!("{error:#}\n").as_bytes())
}

pub fn report_refresh_error(cache: &Cache) -> Result<()> {
	let path = cache.root.join("refresh-error");
	match fs::read_to_string(&path) {
		Ok(error) => {
			eprintln!("crabx: the last background update failed: {}", error.trim());
			fs::remove_file(path)?;
		}
		Err(e) if e.kind() == std::io::ErrorKind::NotFound => (),
		Err(e) => return Err(e.into()),
	}
	Ok(())
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn sparse_index_selects_semver_instead_of_line_order() {
		let input = br#"{"vers":"2.0.0","yanked":false,"deps":[{"large":"ignored"}]}
            {"vers":"9.0.0-beta.1","yanked":false}
            {"vers":"10.0.0","yanked":true}
            {"vers":"1.0.0","yanked":false}"#;
		assert_eq!(latest(&input[..]).unwrap(), Version::new(2, 0, 0));
		assert!(latest(&b"{\"vers\":\"oops\",\"yanked\":false}"[..]).is_err());
		assert!(latest(&b""[..]).is_err());
		for (name, path) in [
			("A", "1/a"),
			("AB", "2/ab"),
			("AbC", "3/a/abc"),
			("crabx", "cr/ab/crabx"),
		] {
			assert_eq!(index_path(name), path);
		}
	}
}
