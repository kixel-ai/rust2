use crate::{
	cache::{self, Cache},
	cli::{Args, Spec},
};
use anyhow::{Context, Result, ensure};
use reqwest::{Client, Response};
use semver::Version;
use serde::{Deserialize, Serialize};
use std::{
	fs::File,
	io::{BufReader, Read, Write},
	process::{Command, Stdio},
	time::{Duration, SystemTime},
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
	let mut state = Refresh::load(cache)?;
	if !state.due(SystemTime::now()) {
		return Ok(());
	}
	let result = async {
		let mut request = client()?.get(format!("https://index.crates.io/{}", index_path(&spec.name)));
		if let Some(etag) = &state.etag {
			request = request.header(reqwest::header::IF_NONE_MATCH, etag);
		}
		let response = request.send().await?.error_for_status()?;
		Update { cache, spec, args }.apply(&mut state, response).await
	}
	.await;
	state.finish(result.as_ref().err(), SystemTime::now());
	state.save(cache)?;
	result
}

#[derive(Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Refresh {
	etag: Option<String>,
	version: Option<Version>,
	failure: Option<Failure>,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Failure {
	message: String,
	version: Option<Version>,
	reported: bool,
	delay: Duration,
	retry_at: SystemTime,
}

impl Refresh {
	fn load(cache: &Cache) -> Result<Self> {
		let path = cache.root.join("refresh.json");
		let file = match File::open(&path) {
			Ok(file) => file,
			Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(Self::default()),
			Err(error) => return Err(error).with_context(|| format!("read {}", path.display())),
		};
		let state: Self =
			serde_json::from_reader(file).with_context(|| format!("corrupt refresh state {}", path.display()))?;
		ensure!(
			state.etag.is_none() || state.version.is_some(),
			"refresh ETag has no version in {}",
			path.display()
		);
		Ok(state)
	}

	fn save(&self, cache: &Cache) -> Result<()> {
		cache::atomic(&cache.root.join("refresh.json"), &serde_json::to_vec(self)?)
	}

	fn due(&self, now: SystemTime) -> bool {
		self.failure.as_ref().is_none_or(|failure| now >= failure.retry_at)
	}

	fn finish(&mut self, error: Option<&anyhow::Error>, now: SystemTime) {
		let Some(error) = error else {
			self.failure = None;
			return;
		};
		let message = format!("{error:#}");
		let previous = self
			.failure
			.as_ref()
			.filter(|failure| failure.message == message && failure.version == self.version);
		let delay = previous.map_or(Duration::from_secs(30), |failure| {
			failure.delay.saturating_mul(2).min(Duration::from_secs(30 * 60))
		});
		self.failure = Some(Failure {
			message,
			version: self.version.clone(),
			reported: previous.is_some_and(|failure| failure.reported),
			delay,
			retry_at: now + delay,
		});
	}
}

struct Update<'a> {
	cache: &'a Cache,
	spec: &'a Spec,
	args: &'a Args,
}

impl Update<'_> {
	async fn apply(&self, state: &mut Refresh, response: Response) -> Result<()> {
		if response.status() != reqwest::StatusCode::NOT_MODIFIED {
			let etag = response
				.headers()
				.get(reqwest::header::ETAG)
				.map(|value| value.to_str().map(str::to_owned))
				.transpose()?;
			let file = spool(response, &self.cache.root).await?;
			state.version = Some(latest(BufReader::new(File::open(file.path())?))?);
			state.etag = etag;
		}
		let version = state
			.version
			.as_ref()
			.context("index returned 304 without a cached version")?;
		// Publish only an installed version, retaining the working entry on failure.
		crate::install::resolve(
			self.cache,
			&Spec {
				name: self.spec.name.clone(),
				version: Some(version.clone()),
			},
			self.args,
		)
		.await?;
		let _lock = self.cache.lock()?;
		self.cache.set_latest(version)?;
		Ok(())
	}
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

pub fn report_refresh_error(cache: &Cache) -> Result<()> {
	if let Some(error) = take_refresh_error(cache)? {
		eprintln!("crabx: the last background update failed: {error}");
	}
	Ok(())
}

fn take_refresh_error(cache: &Cache) -> Result<Option<String>> {
	if !cache.root.try_exists()? {
		return Ok(None);
	}
	let Some(_lock) = cache.try_refresh_lock()? else {
		return Ok(None);
	};
	let mut state = Refresh::load(cache)?;
	let Some(failure) = state.failure.as_mut().filter(|failure| !failure.reported) else {
		return Ok(None);
	};
	failure.reported = true;
	let error = failure.message.clone();
	state.save(cache)?;
	Ok(Some(error))
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::fs;

	#[test]
	fn failed_install_keeps_latest_and_retries_after_not_modified() {
		use clap::Parser;
		fn response(status: &str, body: &str) -> Response {
			let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
			let address = listener.local_addr().unwrap();
			let wire = format!(
				"HTTP/1.1 {status}\r\nContent-Length: {}\r\nETag: new-validator\r\nConnection: close\r\n\r\n{body}",
				body.len()
			);
			let server = std::thread::spawn(move || {
				let (mut stream, _) = listener.accept().unwrap();
				let mut reader = BufReader::new(&mut stream);
				let mut line = String::new();
				loop {
					line.clear();
					assert!(std::io::BufRead::read_line(&mut reader, &mut line).unwrap() > 0);
					if line == "\r\n" {
						break;
					}
				}
				stream.write_all(wire.as_bytes()).unwrap();
			});
			let result = runtime()
				.unwrap()
				.block_on(reqwest::Client::new().get(format!("http://{address}")).send())
				.unwrap();
			server.join().unwrap();
			result
		}
		let temp = tempfile::tempdir().unwrap();
		let cache = Cache {
			root: temp.path().to_owned(),
		};
		let spec = Spec::parse("fixture").unwrap();
		let args = Args::parse_from(["crabx", "fixture"]);
		let update = Update {
			cache: &cache,
			spec: &spec,
			args: &args,
		};
		cache.set_latest(&Version::new(1, 0, 0)).unwrap();
		let mut state = Refresh {
			etag: Some("old-validator".into()),
			version: Some(Version::new(1, 0, 0)),
			..Refresh::default()
		};
		let version = Version::new(2, 0, 0);
		// An incomplete entry fails before network discovery.
		fs::create_dir(cache.version(&version)).unwrap();
		let index = r#"{"vers":"2.0.0","yanked":false}"#;
		let error = runtime()
			.unwrap()
			.block_on(update.apply(&mut state, response("200 OK", index)))
			.unwrap_err();
		state.finish(Some(&error), SystemTime::UNIX_EPOCH);
		state.save(&cache).unwrap();
		let mut state = Refresh::load(&cache).unwrap();
		assert_eq!(cache.latest().unwrap(), Some(Version::new(1, 0, 0)));
		assert_eq!(state.etag.as_deref(), Some("new-validator"));
		assert_eq!(state.version.as_ref(), Some(&version));
		fs::create_dir(cache.version(&version).join("bin")).unwrap();
		fs::write(cache.version(&version).join("bin/tool"), "fixture").unwrap();
		fs::write(
			cache.version(&version).join("record.json"),
			r#"{"version":"2.0.0","bins":["tool"],"source":"fixture"}"#,
		)
		.unwrap();
		runtime()
			.unwrap()
			.block_on(update.apply(&mut state, response("304 Not Modified", "")))
			.unwrap();
		state.finish(None, SystemTime::UNIX_EPOCH);
		assert_eq!(cache.latest().unwrap(), Some(version));
		assert!(state.failure.is_none());
		runtime()
			.unwrap()
			.block_on(update.apply(&mut state, response("304 Not Modified", "")))
			.unwrap();
		assert_eq!(state.etag.as_deref(), Some("new-validator"));
	}

	#[test]
	fn failures_back_off_and_each_diagnostic_has_one_claim() {
		let temp = tempfile::tempdir().unwrap();
		let cache = Cache {
			root: temp.path().to_owned(),
		};
		let now = SystemTime::UNIX_EPOCH + Duration::from_secs(1000);
		let error = anyhow::anyhow!("no release for host");
		let mut state = Refresh::default();
		state.finish(Some(&error), now);
		assert!(!state.due(now + Duration::from_secs(29)));
		assert!(state.due(now + Duration::from_secs(30)));
		state.save(&cache).unwrap();
		let lock = cache.try_refresh_lock().unwrap().unwrap();
		assert!(take_refresh_error(&cache).unwrap().is_none());
		drop(lock);
		assert_eq!(
			take_refresh_error(&cache).unwrap().as_deref(),
			Some("no release for host")
		);
		assert!(take_refresh_error(&cache).unwrap().is_none());
		let mut state = Refresh::load(&cache).unwrap();
		state.finish(Some(&error), now + Duration::from_secs(30));
		assert!(!state.due(now + Duration::from_secs(89)));
		assert!(state.due(now + Duration::from_secs(90)));
		state.save(&cache).unwrap();
		assert!(take_refresh_error(&cache).unwrap().is_none());
		for _ in 0..20 {
			state.finish(Some(&error), now);
		}
		assert_eq!(state.failure.as_ref().unwrap().delay, Duration::from_secs(1800));
		state.version = Some(Version::new(2, 0, 0));
		state.finish(Some(&error), now);
		assert_eq!(state.failure.as_ref().unwrap().delay, Duration::from_secs(30));
		state.save(&cache).unwrap();
		assert!(take_refresh_error(&cache).unwrap().is_some());
		state.finish(None, now);
		assert!(state.due(now));
		assert!(state.failure.is_none());
	}

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
