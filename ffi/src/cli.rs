//! The command line: run cargo on the configured package with this binary as rustc.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

use rust2_driver::cargo::{Args, Check, Output};

use crate::config::{self, Config};

const USAGE: &str = "usage: rust2-ffi --config <ffi.toml> (--out <dir> | --census <dir> | --api <file>) [--manifest-path <Cargo.toml>] [--target <triple>]... [--with <file>]... [--baseline <api.json>] [-- <cargo args>]

Exports the pub API of the package's library, following its `pub use` re-exports into
other crates, as one library: a crate in <dir> that builds the C ABI, with its C header
in <dir>/include, every C name prefixed by the library's crate name. ffi.toml says what
to leave out. The package's Cargo.toml is beside ffi.toml unless --manifest-path says
where. Cargo args, such as --features, are passed to `cargo check`.

--target exports the API the package has on that target, the host by default, whatever
cargo's config names: its `cfg`s and its layouts. Given several times, it exports each
target's API into one binding, checking each in turn: an item only some have is on those
alone, and one that differs between two is an error. The crate builds for those targets
alone.

--api writes one target's API to <file> instead of exporting, for an export on another
host, whose C toolchain the check of a target may need, to add with --with <file>.

Each export writes its API, of every target it serves, to <dir>/api.json. --baseline
fails the export where its API breaks the one an earlier export wrote there: an item
removed, or no longer on a target; a signature, field, or variant changed; or a C name
that now names another item. Commit api.json beside Cargo.lock as the baseline, and
replace it to accept a break.

--census writes a Markdown report, <dir>/<package>.md, of what exports and of every
construct the export refuses, instead of exporting.";

pub fn run(args: &[String]) -> ExitCode {
	match try_run(args) {
		Ok(code) => code,
		Err(err) => {
			eprintln!("rust2-ffi: {err}");
			ExitCode::FAILURE
		}
	}
}

fn try_run(args: &[String]) -> Result<ExitCode, String> {
	let (own, args) = Own::parse(args)?;
	let Some(args) = Args::parse(&args, USAGE)? else {
		return Ok(ExitCode::SUCCESS);
	};
	// Read now, so a malformed config fails before the check.
	Config::load(&args.config)?;
	let mut targets = own.targets;
	if targets.is_empty() {
		targets.push(rustc_session::config::host_tuple().to_string());
	}
	let output = match (&args.output, own.api) {
		(Output::Dir(out), None) => (config::env::OUT, out.clone()),
		(Output::Census(census), None) => (config::env::CENSUS, census.clone()),
		(Output::Dir(_), Some(api)) => (config::env::API, api),
		(Output::Census(_), Some(_)) => return Err(format!("--api and --census are each an output\n\n{USAGE}")),
	};
	if output.0 != config::env::OUT && own.baseline.is_some() {
		return Err(format!("--baseline checks an export: give it with --out\n\n{USAGE}"));
	}
	if output.0 != config::env::OUT && (targets.len() > 1 || !own.with.is_empty()) {
		return Err(format!(
			"--census and --api write one target's: give at most one --target and no --with\n\n{USAGE}"
		));
	}
	let dir = args.manifest.parent().expect("a manifest path has a parent");
	let picks = Picks::parse(&args.cargo, dir)?;
	let (facade, patch, checked) = facade(&args.manifest, &targets[0], &picks)?;
	let line = facade.line();
	// A staged API's spans are relative to the workspace and cargo's home, which hold the
	// same files at other paths on another host.
	let cargo_home = std::env::var_os("CARGO_HOME")
		.map(PathBuf::from)
		.or_else(|| std::env::home_dir().map(|home| home.join(".cargo")));
	let workspace = rust2_driver::cargo::workspace_root(Some(&args.manifest))?;
	let roots = std::env::join_paths(std::iter::once(workspace).chain(cargo_home))
		.map_err(|err| format!("joining the roots of spans: {err}"))?;
	let baseline = own.baseline.unwrap_or_default();
	let check = |target: &str, output: (&str, &std::ffi::OsStr), merge: &std::ffi::OsStr, baseline: &Path| {
		Check {
			tool: "rust2-ffi",
			manifest: &args.manifest,
			package: &facade.name,
			root: None,
			// Always named, so cargo's config never picks another: the exports are found on it.
			target: Some(target),
			args: &args.cargo,
			env: &[
				(config::env::CONFIG, args.config.as_os_str()),
				(config::env::PACKAGE, line.as_ref()),
				(config::env::PATCH, patch.as_ref()),
				(config::env::MERGE, merge),
				(config::env::BASELINE, baseline.as_os_str()),
				(config::env::ROOTS, roots.as_ref()),
				output,
			],
		}
		.run()
	};
	let (last, rest) = targets.split_last().expect("a target");
	// Each target but the last stages its API for the last one's check to merge.
	let stage =
		rust2_driver::cargo::target_dir("rust2-ffi", Some(&args.manifest))?.join(format!("api-{}", std::process::id()));
	let run = || {
		let mut merge = own.with.clone();
		for target in rest {
			let staged = crate::merge::staged(&stage, target);
			let code = check(target, (config::env::API, staged.as_os_str()), "".as_ref(), "".as_ref())?;
			if code != ExitCode::SUCCESS {
				return Ok(code);
			}
			merge.push(staged);
		}
		let merge = std::env::join_paths(&merge).map_err(|err| format!("joining the paths to merge: {err}"))?;
		check(last, (output.0, output.1.as_os_str()), &merge, &baseline)
	};
	let code = run();
	if stage.exists() {
		std::fs::remove_dir_all(&stage).map_err(|err| format!("removing {}: {err}", stage.display()))?;
	}
	let code = code?;
	if output.0 == config::env::OUT && code == ExitCode::SUCCESS {
		lock(&checked, &output.1, &picks)?;
	}
	Ok(code)
}

/// Write the generated crate's lockfile: the check's, with what the crate adds of its own
/// resolved beside it, or fail if the glue would build the facade with any dependency the
/// check did not resolve for the same dependent, which the glue was not generated against.
fn lock(checked: &Checked, out: &Path, picks: &Picks<'_>) -> Result<(), String> {
	let lockfile = checked.metadata.workspace_root.join("Cargo.lock");
	std::fs::copy(&lockfile, out.join("Cargo.lock"))
		.map_err(|err| format!("copying {} into {}: {err}", lockfile.display(), out.display()))?;
	let output = run_cargo(
		&out.join("Cargo.toml"),
		&["metadata", "--format-version", "1"],
		&picks.resolving(),
	)?;
	let generated: Metadata =
		serde_json::from_slice(&output).map_err(|err| format!("reading cargo metadata: {err}"))?;
	// The check's resolution holds more than its build: cargo metadata enables what the
	// facade's dev-dependencies enable, too. So each dependency the glue builds the facade
	// with must be one the check resolved for the same dependent, not the other way around.
	fn built(node: &Node) -> impl Iterator<Item = &str> {
		let deps = node.deps.iter();
		deps.filter(|d| d.dep_kinds.iter().any(|k| k.kind.as_deref() != Some("dev")))
			.map(|d| d.pkg.as_str())
	}
	let identities = |metadata: &Metadata| -> BTreeMap<String, String> {
		metadata.packages.iter().map(|p| (p.id.clone(), p.identity())).collect()
	};
	let (resolved, ids) = (identities(&checked.metadata), identities(&generated));
	let edges: BTreeSet<(&str, &str)> = (checked.metadata.resolve.nodes.iter())
		.flat_map(|n| built(n).map(move |dep| (n.id.as_str(), dep)))
		.map(|(from, to)| (resolved[from].as_str(), resolved[to].as_str()))
		.collect();
	let nodes: BTreeMap<&str, &Node> = generated.resolve.nodes.iter().map(|n| (n.id.as_str(), n)).collect();
	let top = checked.top.as_str();
	let mut moved = Vec::new();
	if ids.get(top) != resolved.get(top) {
		moved.push(top);
	}
	let mut seen = BTreeSet::new();
	let mut queue = vec![top];
	while let Some(id) = queue.pop() {
		if !seen.insert(id) {
			continue;
		}
		for dep in nodes.get(id).into_iter().flat_map(|n| built(n)) {
			if !edges.contains(&(ids[id].as_str(), ids[dep].as_str())) {
				moved.push(dep);
			}
			queue.push(dep);
		}
	}
	match moved.is_empty() {
		true => Ok(()),
		false => Err(format!(
			"the export's library resolves these packages to other versions than the check did: {}",
			moved.join(", ")
		)),
	}
}

/// The resolution the check built the exports with.
struct Checked {
	metadata: Metadata,
	/// The facade's package id.
	top: String,
}

/// The args rust2-ffi takes beyond the shared parser's.
struct Own {
	/// Each `--target`, in order.
	targets: Vec<String>,
	/// Each `--with`: another target's API, as `--api` wrote it.
	with: Vec<PathBuf>,
	/// `--api`: write the target's API here instead of exporting.
	api: Option<PathBuf>,
	/// `--baseline`: the API an earlier export wrote, which this one's must not break.
	baseline: Option<PathBuf>,
}

impl Own {
	/// The args, and the rest for the shared parser, which knows none of them. `--api`
	/// stands in that rest as `--out`, the output the shared parser requires.
	fn parse(args: &[String]) -> Result<(Self, Vec<String>), String> {
		let mut own = Self {
			targets: Vec::new(),
			with: Vec::new(),
			api: None,
			baseline: None,
		};
		let mut rest = Vec::new();
		let mut args = args.iter();
		while let Some(arg) = args.next() {
			let flag = arg.as_str();
			if !matches!(flag, "--target" | "--with" | "--api" | "--baseline") {
				// The cargo args, and the rest after them, are cargo's.
				if flag == "--" {
					rest.push(arg.clone());
					rest.extend(args.cloned());
					break;
				}
				rest.push(arg.clone());
				continue;
			}
			let Some(value) = args.next() else {
				return Err(format!("{flag} needs a value\n\n{USAGE}"));
			};
			match flag {
				"--target" if own.targets.contains(value) => {
					return Err(format!("--target {value} is given twice\n\n{USAGE}"));
				}
				"--target" => own.targets.push(value.clone()),
				"--with" => own.with.push(absolute(value)?),
				"--baseline" if own.baseline.is_some() => {
					return Err(format!("--baseline is given twice\n\n{USAGE}"));
				}
				"--baseline" => own.baseline = Some(absolute(value)?),
				_ if own.api.is_some() => return Err(format!("--api is given twice\n\n{USAGE}")),
				_ => {
					let api = absolute(value)?;
					rest.extend(["--out".to_string(), api.display().to_string()]);
					own.api = Some(api);
				}
			}
		}
		if own.api.is_some() && rest.iter().take_while(|a| *a != "--").filter(|a| *a == "--out").count() > 1 {
			return Err(format!("--api and --out are each an output\n\n{USAGE}"));
		}
		Ok((own, rest))
	}
}

fn absolute(path: &str) -> Result<PathBuf, String> {
	std::path::absolute(path).map_err(|err| format!("resolving {path}: {err}"))
}

/// The facade, the package of `manifest`, then the patches it resolved with, as
/// [`config::env::PATCH`] has them.
fn facade(manifest: &Path, target: &str, picks: &Picks<'_>) -> Result<(config::Package, String, Checked), String> {
	// On the target, so the resolved dependencies are the ones the check passes rustc.
	let output = run_cargo(
		manifest,
		&["metadata", "--format-version", "1", "--filter-platform", target],
		picks,
	)?;
	let metadata: Metadata = serde_json::from_slice(&output).map_err(|err| format!("reading cargo metadata: {err}"))?;
	let canonical =
		std::fs::canonicalize(manifest).map_err(|err| format!("resolving {}: {err}", manifest.display()))?;
	// Cargo keeps a symlink's spelling of the path.
	let top = (metadata.packages.iter())
		.find(|p| p.source.is_none() && std::fs::canonicalize(&p.manifest_path).is_ok_and(|p| p == canonical));
	let Some(top) = top else {
		return Err(format!("{} is no package's manifest", manifest.display()));
	};
	let lib = (top.targets.iter()).find(|t| t.kind.iter().any(|k| k.contains("lib")));
	let Some(lib) = lib else {
		return Err(format!("{} has no library", top.name));
	};
	picks.of_facade(top)?;
	let facade = config::Package {
		name: top.name.clone(),
		lib: lib.name.replace('-', "_"),
		dir: manifest.parent().expect("a manifest path has a parent").into(),
		features: Vec::new(),
	};
	// Every platform's dependencies, which cargo resolves wherever the glue builds.
	let output = run_cargo(manifest, &["metadata", "--format-version", "1"], picks)?;
	let resolved: Metadata = serde_json::from_slice(&output).map_err(|err| format!("reading cargo metadata: {err}"))?;
	let patch = overrides(&resolved, top)?;
	let checked = Checked {
		metadata: resolved,
		top: top.id.clone(),
	};
	Ok((facade, patch, checked))
}

/// The `[patch]` the glue's workspace resolves the exports by as the check did, wherever
/// it builds, as TOML with each path absolute.
fn overrides(metadata: &Metadata, top: &Package) -> Result<String, String> {
	let patch = patch(metadata, top)?;
	let mut overrides = toml::Table::new();
	if !patch.is_empty() {
		overrides.insert("patch".to_string(), patch.into());
	}
	toml::to_string(&overrides).map_err(|err| format!("writing the patches: {err}"))
}

/// Each patch cargo applied in resolving what `top` builds with on any platform, wherever
/// it was declared: the manifest, cargo's config, `--config`, or the environment. A
/// dependency resolved to a package of another source than the one it names was patched,
/// or `[replace]`d by a package of the same version, which a patch resolves to alike; it
/// is keyed by the URL of the source it names, which reads the same without that config.
fn patch(metadata: &Metadata, top: &Package) -> Result<toml::Table, String> {
	let packages: BTreeMap<&str, &Package> = metadata.packages.iter().map(|p| (p.id.as_str(), p)).collect();
	let nodes: BTreeMap<&str, &Node> = metadata.resolve.nodes.iter().map(|n| (n.id.as_str(), n)).collect();
	let mut patch = toml::Table::new();
	let mut seen = BTreeSet::new();
	let mut queue = vec![top.id.as_str()];
	while let Some(id) = queue.pop() {
		let (true, Some(package), Some(node)) = (seen.insert(id), packages.get(id), nodes.get(id)) else {
			continue;
		};
		// What the library builds, not what tests do.
		let built = |kind: &Option<String>| kind.as_deref() != Some("dev");
		for dep in node.deps.iter().filter(|d| d.dep_kinds.iter().any(|k| built(&k.kind))) {
			queue.push(&dep.pkg);
			let Some(resolved) = packages.get(dep.pkg.as_str()) else {
				continue;
			};
			// Unrenamed, the edge is named by the library's crate name.
			let lib = (resolved.targets.iter()).find(|t| t.kind.iter().any(|k| k.contains("lib") || k == "proc-macro"));
			let named = (package.dependencies.iter()).find(|d| {
				let name = d.rename.as_deref().or(lib.map(|t| t.name.as_str()));
				built(&d.kind) && d.name == resolved.name && name.map(|n| n.replace('-', "_")) == Some(dep.name.clone())
			});
			// A path dependency names no source to patch.
			let Some(named) = named.and_then(|d| d.source.as_deref()) else {
				continue;
			};
			// A git package's source adds the commit it resolved to.
			let source = resolved.source.as_deref().map(|s| s.split('#').next().unwrap_or(s));
			if source == Some(named) {
				continue;
			}
			let key = patch_key(named)?;
			let replacement = match resolved.source.as_deref() {
				None => {
					let dir = resolved.manifest_path.parent().expect("a manifest path has a parent");
					toml::toml! { path = (dir.display().to_string()) }
				}
				Some(git) if git.starts_with("git+") => {
					let (url, rev) = git["git+".len()..].split_once('#').unwrap_or((git, ""));
					let url = url.split('?').next().unwrap_or(url);
					// Only a `[replace]` resolves a package to another commit of the source
					// the dependency names; cargo refuses a patch from that source.
					if url == key {
						return Err(format!(
							"{} {} is replaced from its own source, which a `[patch]` of the export's library cannot carry",
							resolved.name, resolved.version
						));
					}
					// A repository may hold several versions of the package.
					let version = format!("={}", resolved.version);
					toml::toml! { git = url rev = rev version = version }
				}
				Some(other) => {
					return Err(format!(
						"{} {} is patched in from {other}, which the export's library cannot patch in",
						resolved.name, resolved.version
					));
				}
			};
			let into = patch.entry(key).or_insert_with(|| toml::Table::new().into());
			let into = into.as_table_mut().expect("a patch's source is a table");
			match into.get(&resolved.name) {
				Some(toml::Value::Table(existing)) if *existing == replacement => {}
				// Two versions of one package patched in from one source.
				Some(_) => {
					let mut renamed = replacement;
					renamed.insert("package".to_string(), resolved.name.clone().into());
					into.insert(format!("{}-{}", resolved.name, resolved.version), renamed.into());
				}
				None => {
					into.insert(resolved.name.clone(), replacement.into());
				}
			}
		}
	}
	Ok(patch)
}

/// The `[patch]` key of a source as `cargo metadata` spells it.
fn patch_key(source: &str) -> Result<String, String> {
	if source == "registry+https://github.com/rust-lang/crates.io-index" || source == "sparse+https://index.crates.io/"
	{
		return Ok("crates-io".to_string());
	}
	if let Some(url) = source.strip_prefix("registry+") {
		return Ok(url.to_string());
	}
	if source.starts_with("sparse+") {
		return Ok(source.to_string());
	}
	if let Some(url) = source.strip_prefix("git+") {
		return Ok(url.split(['?', '#']).next().unwrap_or(url).to_string());
	}
	Err(format!("a patch of {source}, which the export's library cannot patch"))
}

/// What a cargo subcommand prints for a package, run as the check resolves: from the
/// facade's directory, where cargo reads its config, with the check's picks.
fn run_cargo(manifest: &Path, subcommand: &[&str], picks: &Picks<'_>) -> Result<Vec<u8>, String> {
	let output = Command::new(std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into()))
		.current_dir(picks.dir)
		.args(subcommand)
		.arg("--manifest-path")
		.arg(manifest)
		.args(picks.args())
		.output()
		.map_err(|err| format!("running cargo {}: {err}", subcommand[0]))?;
	match output.status.success() {
		true => Ok(output.stdout),
		false => Err(String::from_utf8_lossy(&output.stderr).into_owned()),
	}
}

/// What the cargo args pick for how the check resolves its dependencies.
struct Picks<'a> {
	/// Where cargo runs, which its config and a `--config` path are read from: the facade's.
	dir: &'a Path,
	/// Each feature `--features` and `-F` name.
	named: Vec<&'a str>,
	/// The other args that pick a resolution: feature, lockfile, network, and `--config`.
	flags: Vec<&'a str>,
}

impl<'a> Picks<'a> {
	fn parse(cargo: &'a [String], dir: &'a Path) -> Result<Self, String> {
		let mut picks = Self {
			dir,
			named: Vec::new(),
			flags: Vec::new(),
		};
		let mut args = cargo.iter().map(String::as_str);
		while let Some(arg) = args.next() {
			let value = match arg {
				"--all-features" | "--no-default-features" | "--locked" | "--frozen" | "--offline" => {
					picks.flags.push(arg);
					continue;
				}
				"--target" => return Err("--target is rust2-ffi's, before --".to_string()),
				_ if arg.starts_with("--target=") => return Err(format!("{arg}: --target is rust2-ffi's, before --")),
				"--config" => {
					picks.flags.extend([Some(arg), args.next()].into_iter().flatten());
					continue;
				}
				_ if arg.starts_with("--config=") => {
					picks.flags.push(arg);
					continue;
				}
				"--features" | "-F" => args.next(),
				_ => (arg.strip_prefix("--features="))
					.or_else(|| arg.strip_prefix("-F").map(|v| v.strip_prefix('=').unwrap_or(v))),
			};
			let named = value.into_iter().flat_map(|v| v.split([',', ' ']));
			picks.named.extend(named.filter(|f| !f.is_empty()));
		}
		Ok(picks)
	}

	/// The same picks, as cargo args.
	fn args(&self) -> Vec<String> {
		let mut args: Vec<String> = self.flags.iter().map(|f| f.to_string()).collect();
		if !self.named.is_empty() {
			args.extend(["--features".to_string(), self.named.join(",")]);
		}
		args
	}

	/// The picks that resolve the generated crate: its own features are not the facade's,
	/// and its lockfile gains what it adds of its own, which `--locked` would forbid. The
	/// check's `--config`, such as a source replacement, still applies, from the same
	/// directory.
	fn resolving(&self) -> Self {
		let flags = self.flags.iter().map(|&f| match f {
			"--frozen" => "--offline",
			f => f,
		});
		Self {
			dir: self.dir,
			named: Vec::new(),
			flags: flags
				.filter(|f| !matches!(*f, "--locked" | "--all-features" | "--no-default-features"))
				.collect(),
		}
	}

	/// Refuse a feature named in a dependency, `<dependency>/<feature>`: the generated
	/// crate depends on the facade alone, which enables what its dependencies build with.
	fn of_facade(&self, top: &Package) -> Result<(), String> {
		for feature in &self.named {
			let Some((key, _)) = feature.split_once('/') else {
				continue;
			};
			if key.strip_suffix('?').unwrap_or(key) != top.name {
				return Err(format!(
					"--features {feature}: the export builds {}'s dependencies with only the features it enables in them",
					top.name
				));
			}
		}
		Ok(())
	}
}

/// What `cargo metadata` says of the packages, and how they depend on each other.
#[derive(serde::Deserialize)]
struct Metadata {
	packages: Vec<Package>,
	workspace_root: PathBuf,
	resolve: Resolve,
}

#[derive(serde::Deserialize)]
struct Resolve {
	nodes: Vec<Node>,
}

#[derive(serde::Deserialize)]
struct Node {
	id: String,
	deps: Vec<Dep>,
}

/// A resolved dependency.
#[derive(serde::Deserialize)]
struct Dep {
	/// The name the dependent calls its library by.
	name: String,
	pkg: String,
	dep_kinds: Vec<DepKind>,
}

#[derive(serde::Deserialize)]
struct DepKind {
	/// `dev` or `build`, or none for a normal dependency.
	kind: Option<String>,
}

#[derive(serde::Deserialize)]
struct Package {
	id: String,
	name: String,
	version: String,
	source: Option<String>,
	manifest_path: PathBuf,
	targets: Vec<Target>,
	dependencies: Vec<Dependency>,
}

impl Package {
	/// What names the same code in another resolution: a git source by its commit, whatever
	/// reference picked it, since the glue's `[patch]` pins one by `rev`.
	fn identity(&self) -> String {
		let Some(source) = &self.source else {
			return self.id.clone();
		};
		let (url, commit) = source.split_once('#').unwrap_or((source, ""));
		let url = url.split_once('?').map_or(url, |(url, _)| url);
		format!("{} {} {url}#{commit}", self.name, self.version)
	}
}

#[derive(serde::Deserialize)]
struct Dependency {
	/// The package's name.
	name: String,
	/// The name the dependent calls it by, when not the package's.
	rename: Option<String>,
	/// `dev` or `build`, or none for a normal dependency.
	kind: Option<String>,
	/// The source it names, such as `registry+<url>`, or none for a path dependency.
	source: Option<String>,
}

#[derive(serde::Deserialize)]
struct Target {
	name: String,
	kind: Vec<String>,
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn reads_what_picks_a_resolution() {
		for spelling in [
			&["--features", "a,dep/b"][..],
			&["--features=a dep/b"],
			&["-F", "a", "-F", "dep/b"],
			&["-Fa,dep/b"],
			&["-F=a", "--features", "dep/b"],
		] {
			let cargo: Vec<String> = spelling.iter().map(|s| s.to_string()).collect();
			assert_eq!(
				Picks::parse(&cargo, Path::new(".")).unwrap().args(),
				["--features", "a,dep/b"],
				"{spelling:?}"
			);
		}
		let cargo = [
			"--all-features",
			"--locked",
			"--config",
			"a=1",
			"--config=b=2",
			"--quiet",
		];
		let cargo: Vec<String> = cargo.iter().map(|s| s.to_string()).collect();
		let flags = ["--all-features", "--locked", "--config", "a=1", "--config=b=2"];
		assert_eq!(Picks::parse(&cargo, Path::new(".")).unwrap().args(), flags);
		for target in [
			&["--target", "wasm32-unknown-unknown"][..],
			&["--target=wasm32-unknown-unknown"],
		] {
			let cargo: Vec<String> = target.iter().map(|s| s.to_string()).collect();
			assert!(Picks::parse(&cargo, Path::new(".")).is_err(), "{target:?}");
		}
	}
}
