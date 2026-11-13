mod cache;
mod cli;
#[path = "../vendor/binstalk-fetchers/src/lib.rs"]
#[allow(dead_code)] // The imported library exposes more than the runner consumes.
mod fetchers;
mod install;
mod network;
#[path = "../vendor/binstalk-target.rs"]
mod target;
mod trust;

use anyhow::{Context, Result};
use clap::Parser;
use std::{path::PathBuf, process::Command};

fn main() {
	if let Err(error) = run(cli::Args::parse()) {
		eprintln!("crabx: {error:#}");
		std::process::exit(1);
	}
}

fn run(args: cli::Args) -> Result<()> {
	let krate = args
		.command
		.first()
		.and_then(|arg| arg.to_str())
		.context("the crate name must be UTF-8")?;
	let spec = cli::Spec::parse(krate)?;
	let cache = cache::Cache::new(&spec, args.mode())?;
	if args.refresh {
		network::runtime()?.block_on(network::refresh(&cache, &spec, &args))?;
		return Ok(());
	}
	if spec.version.is_none() {
		network::report_refresh_error(&cache)?;
	}
	let version = match &spec.version {
		Some(version) => Some(version.clone()),
		None => cache.latest()?,
	};
	let cached = version.as_ref().map(|v| cache.load(v)).transpose()?.flatten();
	let record = match cached {
		Some(record) => record,
		None => network::runtime()?.block_on(install::resolve(&cache, &spec, &args))?,
	};
	let binary = record.binary(args.bin.as_deref(), &spec.name)?;
	let path = cache.version(&record.version).join("bin").join(binary);
	if spec.version.is_none() {
		// The updater owns no terminal handles and never delays this invocation.
		if let Err(error) = network::spawn_refresh(&spec, &args) {
			eprintln!("crabx: could not start background update: {error:#}");
		}
	}
	execute(path, args.command.into_iter().skip(1).collect())
}

fn execute(path: PathBuf, args: Vec<std::ffi::OsString>) -> Result<()> {
	let mut command = Command::new(&path);
	command.args(args);
	#[cfg(unix)]
	{
		use std::os::unix::process::CommandExt;
		Err(command.exec()).with_context(|| format!("execute {}", path.display()))
	}
	#[cfg(windows)]
	{
		let status = command
			.status()
			.with_context(|| format!("execute {}", path.display()))?;
		std::process::exit(status.code().unwrap_or(1));
	}
}
