//! The command line: run cargo on the package with this binary as rustc.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

use rust2_driver::cargo::{Args, Check, Output, Wrap};

use crate::config;

const USAGE: &str = "usage: rust2-ffi (--out <dir> | --census <dir> | --api <file>) [--manifest-path <Cargo.toml>] [--target <triple>]... [--with <file>]... [--baseline <api.json>] [-- <cargo args>]

Exports the pub API of the package's library, following its `pub use` re-exports into
other crates, as one library: a crate in <dir> that builds the C ABI, with its C header
in <dir>/include, every C name prefixed by the library's crate name. An item that cannot
export fails the export. The package's Cargo.toml is in the current directory unless
--manifest-path says where. Cargo args, such as --features, are passed to `cargo check`,
which checks the release profile unless they pick another, as a library ships.

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

pub(crate) fn run(args: &[String]) -> ExitCode {
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
	let Some(mut args) = Args::parse_configless(&args, USAGE)? else {
		return Ok(ExitCode::SUCCESS);
	};
	args.cargo = ungroup(&args.cargo);
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
	if let Some(baseline) = &own.baseline {
		// The export's own api.json would silently replace the baseline it passed.
		let api = output.1.join(crate::driver::API_FILE);
		if same_file(baseline, &api) {
			return Err(format!(
				"--baseline {} is the api.json the export writes: commit the baseline outside --out\n\n{USAGE}",
				baseline.display()
			));
		}
	}
	if output.0 != config::env::OUT && (targets.len() > 1 || !own.with.is_empty()) {
		return Err(format!(
			"--census and --api write one target's: give at most one --target and no --with\n\n{USAGE}"
		));
	}
	let dir = args.manifest.parent().expect("a manifest path has a parent");
	let picks = Picks::parse(&args.cargo, dir)?;
	let (facade, patch, checked) = facade(&args.manifest, &targets, &picks)?;
	let crates = crates(&checked.metadata, &checked.top)?;
	let profile = check_profile(&args.cargo, &checked.metadata.workspace_root)?;
	// The check's cargo args, with `--release` first where they pick no profile, so as not to
	// follow a `--` of theirs.
	let cargo = match profile.args == profile_args(&args.cargo) {
		true => args.cargo.clone(),
		false => [profile.args.clone(), args.cargo.clone()].concat(),
	};
	let line = facade.line();
	let vars = profile.vars.join(",");
	// A staged API's spans are relative to the workspace and cargo's home, which hold the
	// same files at other paths on another host.
	let cargo_home = std::env::var_os("CARGO_HOME")
		.map(PathBuf::from)
		.or_else(|| std::env::home_dir().map(|home| home.join(".cargo")));
	let workspace = rust2_driver::cargo::workspace_root(Some(&args.manifest))?;
	let roots = std::env::join_paths(std::iter::once(workspace).chain(cargo_home))
		.map_err(|err| format!("joining the roots of spans: {err}"))?;
	let baseline = own.baseline.unwrap_or_default();
	// Where the wrapper writes each target's `cfg`s on cargo's query of it, which comes before
	// cargo makes its target directory, and so outside it.
	let queried = private_dir("cfgs")?;
	// A target with a host checks std's MIR too, from the sysroot or else from source.
	let cargo_args = |target: &str, args: Vec<String>| -> Result<Vec<String>, String> {
		let build_std = crate::wasm::host(target).is_some() && crate::wasm::sysroot(target)?.is_none();
		Ok(match build_std {
			true => [args, vec![crate::wasm::STD.to_string()]].concat(),
			false => args,
		})
	};
	// Each target's `cfg`s are written under its index, as a target may be a path.
	let check =
		|index: usize, target: &str, output: (&str, &std::ffi::OsStr), merge: &std::ffi::OsStr, baseline: &Path| {
			let rustflags = Rustflags::of(&checked.targets[target], target)?;
			let [flags, rustc, added] = rustflags.env();
			Check {
				tool: "rust2-ffi",
				manifest: &args.manifest,
				package: &facade.name,
				root: None,
				// Always named, so cargo's config never picks another: the exports are found on it.
				target: Some(target),
				args: &cargo_args(target, cargo.clone())?,
				target_dir: None,
				dir: None,
				wrap: Wrap::Workspace,
				env: &[
					// The wrapper writes the target's `cfg`s on cargo's query of it, which cargo
					// would otherwise answer from its cache.
					("CARGO_CACHE_RUSTC_INFO", "0".as_ref()),
					flags,
					rustc,
					added,
					(config::env::PACKAGE, line.as_ref()),
					(config::env::PATCH, patch.as_ref()),
					(config::env::CRATES, crates.as_ref()),
					(config::env::MERGE, merge),
					(config::env::BASELINE, baseline.as_os_str()),
					(config::env::ROOTS, roots.as_ref()),
					(config::env::PROFILE, profile.root.as_ref()),
					(
						config::env::PROFILE_DIR,
						profile.dir.as_deref().unwrap_or_default().as_ref(),
					),
					(config::env::PROFILE_VARS, vars.as_ref()),
					(config::env::CFGS, queried.join(index.to_string()).as_os_str()),
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
		for (index, target) in rest.iter().enumerate() {
			let staged = crate::merge::staged(&stage, target);
			let code = check(
				index,
				target,
				(config::env::API, staged.as_os_str()),
				"".as_ref(),
				"".as_ref(),
			)?;
			if code != ExitCode::SUCCESS {
				return Ok(code);
			}
			merge.push(staged);
		}
		let merge = std::env::join_paths(&merge).map_err(|err| format!("joining the paths to merge: {err}"))?;
		check(rest.len(), last, (output.0, output.1.as_os_str()), &merge, &baseline)
	};
	let code = run();
	for dir in [&stage, &queried] {
		if dir.exists() {
			std::fs::remove_dir_all(dir).map_err(|err| format!("removing {}: {err}", dir.display()))?;
		}
	}
	let code = code?;
	if output.0 != config::env::OUT || code != ExitCode::SUCCESS {
		return Ok(code);
	}
	lock(&checked, &output.1, &picks)?;
	// The web binding is written when the export serves its target, whose host lacks what
	// the generated crate may reach: walked in the facade's target directory, which holds
	// every crate it builds with already.
	if !output.1.join("web").join("index.ts").exists() {
		return Ok(code);
	}
	let glue = format!("{}-ffi", facade.name);
	let own: Vec<String> = (checked.metadata.packages.iter())
		.filter(|p| p.source.is_none())
		.flat_map(|p| p.targets.iter().filter(|t| t.kind.iter().any(|k| k.contains("lib"))))
		.map(|t| t.name.replace('-', "_"))
		.collect();
	let own = own.join(",");
	let target_dir = rust2_driver::cargo::target_dir("rust2-ffi", Some(&args.manifest))?;
	// Resolved as `lock` resolved it, under the check's profile, whose `cfg`s the glue was
	// generated against.
	let mut cargo = picks.resolving().args();
	cargo.extend(profile.args);
	cargo.extend(["--features".to_string(), "web".to_string()]);
	let walked = match checked.targets.get(crate::web::TARGET) {
		Some(recorded) => recorded.clone(),
		None => on_target(&args.manifest, crate::web::TARGET, &picks)?.1,
	};
	let rustflags = Rustflags::of(&walked, crate::web::TARGET)?;
	let [flags, rustc, added] = rustflags.env();
	Check {
		tool: "rust2-ffi",
		manifest: &output.1.join("Cargo.toml"),
		package: &glue,
		root: None,
		target: Some(crate::web::TARGET),
		args: &cargo_args(crate::web::TARGET, cargo)?,
		target_dir: Some(&target_dir),
		dir: Some(picks.dir),
		wrap: Wrap::Workspace,
		env: &[
			flags,
			rustc,
			added,
			(crate::wasm::env::PACKAGE, glue.as_ref()),
			(crate::wasm::env::OWN, own.as_ref()),
		],
	}
	.run()
}

/// The environment cargo checks `target`'s crates in: as rustc, the executable compiles every
/// crate the workspace does not own, which a cache in cargo's `RUSTC_WRAPPER` serves, so each
/// sees every flag on its command line: the consumer's rustflags for the target, and the
/// export's after them, which cargo passes no build script or proc macro.
struct Rustflags {
	rustc: std::ffi::OsString,
	encoded: String,
	added: String,
}

/// The most bytes of rustflags the check passes cargo, which every platform's environment
/// holds in one variable.
const ENV_MAX: usize = 32_000;

impl Rustflags {
	fn of(recorded: &crate::driver::Recorded, target: &str) -> Result<Self, String> {
		let mut added: Vec<String> = crate::leaf::FLAGS.map(str::to_string).to_vec();
		added.extend(crate::wasm::flags(target)?);
		let encoded = [recorded.flags.clone(), added.clone()].concat().join("\x1f");
		// Windows holds at most 32,767 characters in one environment variable, and Linux
		// 128 KiB.
		if encoded.len() > ENV_MAX {
			return Err(format!(
				"{target}'s rustflags take {} bytes, more than the {ENV_MAX} rust2-ffi can pass cargo in CARGO_ENCODED_RUSTFLAGS: put most in an argfile, as `@<path>`",
				encoded.len()
			));
		}
		Ok(Self {
			rustc: recorded.rustc.clone().into_os_string(),
			encoded,
			added: added.join("\x1f"),
		})
	}

	fn env(&self) -> [(&'static str, &std::ffi::OsStr); 3] {
		[
			("CARGO_ENCODED_RUSTFLAGS", self.encoded.as_ref()),
			(config::env::RUSTC, &self.rustc),
			(config::env::ADDED, self.added.as_ref()),
		]
	}
}

/// A new directory in the temp directory, named for `what`: private and unguessable, as the
/// shared temp directory holds other users' files, and made afresh, so that no one else's
/// can stand in for it.
fn private_dir(what: &str) -> Result<PathBuf, String> {
	let dir = std::env::temp_dir().join(format!(
		"rust2-ffi-{what}-{:016x}",
		std::hash::BuildHasher::hash_one(&std::hash::RandomState::new(), std::process::id())
	));
	let mut private = std::fs::DirBuilder::new();
	#[cfg(unix)]
	std::os::unix::fs::DirBuilderExt::mode(&mut private, 0o700);
	private
		.create(&dir)
		.map_err(|err| format!("creating {}: {err}", dir.display()))?;
	Ok(dir)
}

/// Write the generated crate's lockfile: the check's, with what the crate adds of its own
/// resolved beside it, or fail if the glue would build the facade with any dependency the
/// check did not resolve for the same dependent, which the glue was not generated against.
fn lock(checked: &Checked, out: &Path, picks: &Picks<'_>) -> Result<(), String> {
	let lockfile = checked.metadata.workspace_root.join("Cargo.lock");
	let written = out.join("Cargo.lock");
	let earlier = std::fs::read(&written)
		.ok()
		.zip(std::fs::metadata(&written).and_then(|meta| meta.modified()).ok());
	std::fs::copy(&lockfile, &written)
		.map_err(|err| format!("copying {} into {}: {err}", lockfile.display(), out.display()))?;
	let output = run_cargo(
		&out.join("Cargo.toml"),
		&["metadata", "--format-version", "1"],
		&[],
		&picks.resolving(),
	)?;
	// Resolved as before, the lockfile keeps its mtime, as every unchanged file of the
	// export does: cargo reruns a build script when any file of its package is newer.
	if let Some((bytes, mtime)) = earlier
		&& std::fs::read(&written).is_ok_and(|now| now == bytes)
	{
		std::fs::File::options()
			.write(true)
			.open(&written)
			.and_then(|file| file.set_modified(mtime))
			.map_err(|err| format!("writing {}: {err}", written.display()))?;
	}
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
	/// What cargo passes each target's crates, by the target as the check names it.
	targets: BTreeMap<String, crate::driver::Recorded>,
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
fn facade(
	manifest: &Path,
	targets: &[String],
	picks: &Picks<'_>,
) -> Result<(config::Package, String, Checked), String> {
	// On the first target, which records what cargo passes its crates; each other's then.
	let (output, first) = on_target(manifest, &targets[0], picks)?;
	let mut recorded = BTreeMap::from([(targets[0].clone(), first)]);
	for target in &targets[1..] {
		if !recorded.contains_key(target) {
			recorded.insert(target.clone(), on_target(manifest, target, picks)?.1);
		}
	}
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
		version: top.version.clone(),
		dir: manifest.parent().expect("a manifest path has a parent").into(),
		features: Vec::new(),
	};
	// Every platform's dependencies, which cargo resolves wherever the glue builds.
	let output = run_cargo(manifest, &["metadata", "--format-version", "1"], &[], picks)?;
	let resolved: Metadata = serde_json::from_slice(&output).map_err(|err| format!("reading cargo metadata: {err}"))?;
	let patch = overrides(&resolved, top)?;
	let checked = Checked {
		metadata: resolved,
		top: top.id.clone(),
		targets: recorded,
	};
	Ok((facade, patch, checked))
}

/// Each crate the facade builds with, by its library's crate name, as the glue would depend
/// on the very package: by its directory, its exact crates.io version, or its git commit.
/// A name two packages share, or one from another registry, is left out, so a type of its
/// crate the facade does not re-export stays refused.
fn crates(metadata: &Metadata, top: &str) -> Result<String, String> {
	let packages: BTreeMap<&str, &Package> = metadata.packages.iter().map(|p| (p.id.as_str(), p)).collect();
	let nodes: BTreeMap<&str, &Node> = metadata.resolve.nodes.iter().map(|n| (n.id.as_str(), n)).collect();
	let mut named: BTreeMap<String, Option<String>> = BTreeMap::new();
	let mut seen = BTreeSet::new();
	let mut queue = vec![top];
	while let Some(id) = queue.pop() {
		let (true, Some(package), Some(node)) = (seen.insert(id), packages.get(id), nodes.get(id)) else {
			continue;
		};
		if id != top
			&& let Some(lib) = (package.targets.iter()).find(|t| t.kind.iter().any(|k| k.contains("lib")))
		{
			let keys = dependency(package);
			let lib = lib.name.replace('-', "_");
			match named.get(&lib) {
				Some(other) if *other != keys => {
					named.insert(lib, None);
				}
				Some(_) => {}
				None => {
					named.insert(lib, keys);
				}
			}
		}
		let built = |kind: &Option<String>| kind.as_deref() != Some("dev");
		for dep in node.deps.iter().filter(|d| d.dep_kinds.iter().any(|k| built(&k.kind))) {
			queue.push(&dep.pkg);
		}
	}
	let table: toml::Table = (named.into_iter())
		.filter_map(|(lib, keys)| Some((lib, toml::Value::String(keys?))))
		.collect();
	toml::to_string(&table).map_err(|err| format!("writing the crates: {err}"))
}

/// The TOML keys of a dependency on exactly `package`, if the glue can name it.
fn dependency(package: &Package) -> Option<String> {
	let name = &package.name;
	match package.source.as_deref() {
		None => {
			let dir = package.manifest_path.parent().expect("a manifest path has a parent");
			Some(format!("package = {name:?}, path = {:?}", dir.display().to_string()))
		}
		Some("registry+https://github.com/rust-lang/crates.io-index" | "sparse+https://index.crates.io/") => {
			Some(format!("package = {name:?}, version = \"={}\"", package.version))
		}
		// By the reference the facade names it by, which is part of its source's identity:
		// `?branch=`, `?tag=`, or `?rev=`, or none for the default branch. The lockfile
		// pins the commit as the facade's does.
		Some(git) if git.starts_with("git+") => {
			let url = git["git+".len()..].split('#').next().unwrap_or(git);
			let (url, reference) = url.split_once('?').unwrap_or((url, ""));
			let reference = match reference.split_once('=') {
				Some((key @ ("branch" | "tag" | "rev"), value)) => format!(", {key} = {:?}", decode(value)?),
				Some(_) => return None,
				None => String::new(),
			};
			Some(format!("package = {name:?}, git = {url:?}{reference}"))
		}
		Some(_) => None,
	}
}

/// `value` with each `%XX` cargo's source id escapes, as in `?branch=feature%2Fpending`,
/// decoded, or `None` if it is malformed.
fn decode(value: &str) -> Option<String> {
	let mut bytes = Vec::with_capacity(value.len());
	let mut rest = value.as_bytes();
	while let [first, tail @ ..] = rest {
		if *first == b'%' {
			let hex = std::str::from_utf8(tail.get(..2)?).ok()?;
			bytes.push(u8::from_str_radix(hex, 16).ok()?);
			rest = &tail[2..];
		} else {
			bytes.push(*first);
			rest = tail;
		}
	}
	String::from_utf8(bytes).ok()
}

#[cfg(test)]
mod decode_tests {
	#[test]
	fn decodes_a_branch() {
		assert_eq!(super::decode("feature%2Fpending").as_deref(), Some("feature/pending"));
		assert_eq!(super::decode("main").as_deref(), Some("main"));
		assert_eq!(super::decode("bad%2"), None);
	}
}

/// The `[patch]` the glue's workspace resolves the exports by as the check did, wherever
/// it builds, as TOML with each path absolute, which the glue writes relative to its export,
/// and the `[profile]` the exports build with.
fn overrides(metadata: &Metadata, top: &Package) -> Result<String, String> {
	let (patch, built) = patch(metadata, top)?;
	let mut overrides = toml::Table::new();
	if !patch.is_empty() {
		overrides.insert("patch".to_string(), patch.into());
	}
	let manifest = metadata.workspace_root.join("Cargo.toml");
	let source = std::fs::read_to_string(&manifest).map_err(|err| format!("reading {}: {err}", manifest.display()))?;
	if let Some(profile) = profile(&manifest, &source, &metadata.packages, top, &built)? {
		overrides.insert("profile".to_string(), profile.into());
	}
	toml::to_string(&overrides).map_err(|err| format!("writing the patches and profiles: {err}"))
}

/// The workspace's `[profile]` table, so the glue builds as the consumer ships. A
/// per-package override of a package the library builds is carried, and one of another
/// package of the workspace's resolve, such as a tool's dependency, dropped, since the
/// glue's workspace would not resolve it. One naming no package at all is an error at its
/// line: cargo only warns, and the consumer's intent would silently not apply. So is `*`,
/// which would newly reach the workspace's own crates. The facade's own override also
/// goes to the glue package, which builds the artifact the facade shipped.
fn profile(
	manifest: &Path,
	source: &str,
	packages: &[Package],
	top: &Package,
	built: &BTreeSet<&str>,
) -> Result<Option<toml::Table>, String> {
	let spanned = toml::de::DeTable::parse(source).map_err(|err| format!("{}: {err}", manifest.display()))?;
	let Some(spanned) = spanned.get_ref().get("profile") else {
		return Ok(None);
	};
	let at = |span: std::ops::Range<usize>| {
		let line = source[..span.start].matches('\n').count() + 1;
		format!("{}:{line}", manifest.display())
	};
	let toml::de::DeValue::Table(spanned) = spanned.get_ref() else {
		return Err(format!("{}: `profile` is not a table", at(spanned.span())));
	};
	let table: toml::Table = toml::from_str(source).map_err(|err| format!("{}: {err}", manifest.display()))?;
	let mut profiles = table
		.get("profile")
		.and_then(|p| p.as_table())
		.cloned()
		.unwrap_or_default();
	for (name, profile) in spanned {
		let name = name.get_ref().as_ref();
		let toml::de::DeValue::Table(profile) = profile.get_ref() else {
			return Err(format!("{}: `profile.{name}` is not a table", at(profile.span())));
		};
		let Some(overrides) = profile.get("package") else {
			continue;
		};
		let toml::de::DeValue::Table(overrides) = overrides.get_ref() else {
			return Err(format!(
				"{}: `profile.{name}.package` is not a table",
				at(overrides.span())
			));
		};
		let carried = profiles[name]["package"].as_table_mut().expect("a table parsed as one");
		let mut facade = Vec::new();
		for spec in overrides.keys() {
			let spec_str = spec.get_ref().as_ref();
			// `*` skips workspace members, but the glue's workspace makes the exported crate
			// and its siblings dependencies, so carried it would reach them too.
			if spec_str == "*" {
				return Err(format!(
					"{}: `profile.{name}.package.\"*\"` would reach the exported workspace's own crates in the glue's workspace; name each package instead",
					at(spec.span())
				));
			}
			let mut matched = Vec::new();
			for package in packages {
				if matches(spec_str, package).map_err(|err| format!("{}: {err}", at(spec.span())))? {
					matched.push(package);
				}
			}
			if matched.is_empty() {
				return Err(format!(
					"{}: `profile.{name}.package.{spec_str}` names no package the workspace resolves",
					at(spec.span())
				));
			}
			if !matched.iter().any(|p| built.contains(p.id.as_str())) {
				carried.remove(spec_str);
			} else if matched.iter().any(|p| p.id == top.id) {
				facade.push(spec);
			}
		}
		// After the removals, so an override of a dev-only package named like the glue
		// is not taken for a second one of the exported crate.
		let glue = format!("{}-ffi", top.name);
		for spec in facade {
			let spec_str = spec.get_ref().as_ref();
			let value = carried[spec_str].clone();
			if carried.insert(glue.clone(), value).is_some() {
				return Err(format!(
					"{}: `profile.{name}.package.{spec_str}` is a second override of the exported crate, which its glue package `{glue}` takes",
					at(spec.span())
				));
			}
		}
		if carried.is_empty() {
			profiles[name]
				.as_table_mut()
				.expect("a table parsed as one")
				.remove("package");
		}
	}
	Ok(Some(profiles))
}

/// Whether the package spec of a profile override, `name` or `name@version`, names
/// `package`.
fn matches(spec: &str, package: &Package) -> Result<bool, String> {
	let (name, version) = spec.split_once(['@', ':']).map_or((spec, None), |(n, v)| (n, Some(v)));
	if spec.contains(['/', '#']) {
		return Err(format!(
			"the profile override `{spec}` names a package by URL, which the export's library cannot carry"
		));
	}
	// An omitted build field, like an omitted minor or patch, matches any.
	let core = package
		.version
		.split_once('+')
		.map_or(package.version.as_str(), |(core, _)| core);
	let numbers = core.split_once('-').map_or(core, |(numbers, _)| numbers);
	// As in cargo, a prerelease package matches only a spec naming its prerelease whole.
	let exact =
		|v: &str| package.version == v || core == v || (core == numbers && numbers.starts_with(&format!("{v}.")));
	Ok(package.name == name && version.is_none_or(exact))
}

/// Each patch cargo applied in resolving what `top` builds with on any platform, wherever
/// it was declared: the manifest, cargo's config, `--config`, or the environment. A
/// dependency resolved to a package of another source than the one it names was patched,
/// or `[replace]`d by a package of the same version, which a patch resolves to alike; it
/// is keyed by the URL of the source it names, which reads the same without that config.
/// Also the IDs of every package the library builds.
fn patch<'a>(metadata: &'a Metadata, top: &'a Package) -> Result<(toml::Table, BTreeSet<&'a str>), String> {
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
	Ok((patch, seen))
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

/// `cargo metadata` of `manifest` filtered to `target`, and what cargo passes that target's
/// crates, which this executable records as cargo's workspace wrapper on cargo's query of the
/// target: the consumer's rustflags, from wherever cargo reads them, which the check passes on
/// with the export's own after them, and the rustc cargo resolved. One target a run, as cargo
/// names it to rustc, which may not be as the export names it, such as a relative path.
fn on_target(manifest: &Path, target: &str, picks: &Picks<'_>) -> Result<(Vec<u8>, crate::driver::Recorded), String> {
	let exe = std::env::current_exe().map_err(|err| format!("finding this executable: {err}"))?;
	let record = private_dir("record")?;
	let env = [
		("RUSTC_WORKSPACE_WRAPPER", exe.as_os_str()),
		("RUSTC_WRAPPER", "".as_ref()),
		// Cargo would otherwise answer a query it has made before from its cache.
		("CARGO_CACHE_RUSTC_INFO", "0".as_ref()),
		(config::env::RECORD, record.as_os_str()),
	];
	let subcommand = ["metadata", "--format-version", "1", "--filter-platform", target];
	let output = run_cargo(manifest, &subcommand, &env, picks).and_then(|output| {
		let path = crate::driver::Recorded::path(&record);
		let json = std::fs::read_to_string(&path).map_err(|err| {
			format!(
				"cargo never queried {target} through rust2-ffi ({}: {err})",
				path.display()
			)
		})?;
		let recorded = serde_json::from_str(&json).map_err(|err| format!("reading {}: {err}", path.display()))?;
		Ok((output, recorded))
	});
	std::fs::remove_dir_all(&record).map_err(|err| format!("removing {}: {err}", record.display()))?;
	output
}

/// What a cargo subcommand prints for a package, run as the check resolves: from the
/// facade's directory, where cargo reads its config, with the check's picks.
fn run_cargo(
	manifest: &Path,
	subcommand: &[&str],
	env: &[(&str, &std::ffi::OsStr)],
	picks: &Picks<'_>,
) -> Result<Vec<u8>, String> {
	let output = Command::new(std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into()))
		.current_dir(picks.dir)
		.envs(env.iter().copied())
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

/// The check's profile: the cargo args that pick it, `--release` unless `cargo` picks one,
/// as a library ships in it, since its leaves are proven against the bodies it compiles.
struct CheckProfile {
	args: Vec<String>,
	/// The `PROFILE` cargo gives a build script under it: `release` for a profile that is
	/// or inherits from `release`, else `debug`.
	root: &'static str,
	/// The directory cargo builds it in, unless a build under another profile could share it
	/// with other settings: `dev`, `test`, and `debug` share `debug`, and `release` and
	/// `bench` share `release`, each the same profile as its directory's root unless the
	/// manifest sets any of its own, which a package override of the glue's may hide from the
	/// glue's build script while every other crate compiles under it.
	dir: Option<String>,
	/// The `CARGO_PROFILE_*` variables that may change what a crate compiles under it, or
	/// under another profile that shares its directory.
	vars: Vec<String>,
}

/// The keys of a profile, as a `CARGO_PROFILE_<NAME>_<KEY>` variable spells them, that may
/// change what a crate compiles: the settings a build script reads, and the debug
/// assertions of the build scripts and proc macros themselves. Overflow checks the check
/// always makes, and the panic strategy the crate's `PROVEN` compares.
const PROFILE_KEYS: [&str; 4] = [
	"OPT_LEVEL",
	"DEBUG",
	"DEBUG_ASSERTIONS",
	"BUILD_OVERRIDE_DEBUG_ASSERTIONS",
];

/// The [`PROFILE_KEYS`] variables of each profile in `chain`.
fn profile_vars(chain: &[&str]) -> Vec<String> {
	(chain.iter())
		.flat_map(|name| {
			let name = name.to_uppercase().replace('-', "_");
			PROFILE_KEYS.map(|key| format!("CARGO_PROFILE_{name}_{key}"))
		})
		.collect()
}

/// The [`CheckProfile`] `cargo` picks, following `inherits` through the workspace's
/// manifest, whose profiles alone the glue carries. One cargo's config defines is an error.
fn check_profile(cargo: &[String], workspace: &Path) -> Result<CheckProfile, String> {
	let mut args = profile_args(cargo);
	let picked = match args.last().map(String::as_str) {
		None => {
			args = vec!["--release".to_string()];
			"release"
		}
		Some("--release" | "-r") => "release",
		Some(last) => last.strip_prefix("--profile=").unwrap_or(last),
	}
	.to_string();
	let manifest = workspace.join("Cargo.toml");
	let source = std::fs::read_to_string(&manifest).map_err(|err| format!("reading {}: {err}", manifest.display()))?;
	let table: toml::Table = toml::from_str(&source).map_err(|err| format!("{}: {err}", manifest.display()))?;
	let profiles = table.get("profile").and_then(|p| p.as_table());
	let (root, dir, chain) =
		resolve_profile(&picked, profiles).map_err(|err| format!("{}: {err}", manifest.display()))?;
	let (dir, vars) = shared_vars(dir, &chain, |var| std::env::var_os(var).is_some());
	Ok(CheckProfile { args, root, dir, vars })
}

/// The [`CheckProfile::dir`] and [`CheckProfile::vars`] of a profile in `dir` that inherits
/// from the profiles in `chain`, given which variables the export's environment `set`s. A
/// shared directory proves a build under any profile in it, so its other profiles'
/// variables are watched too, and must be unset in the build as in the check: a check whose
/// environment sets one proves nothing.
fn shared_vars(dir: Option<String>, chain: &[&str], set: impl Fn(&str) -> bool) -> (Option<String>, Vec<String>) {
	let others: &[&str] = match dir.as_deref() {
		Some("debug") => &["test", "debug"],
		Some("release") => &["bench"],
		_ => &[],
	};
	let others = profile_vars(others);
	let dir = dir.filter(|_| !others.iter().any(|var| set(var)));
	let mut vars = profile_vars(chain);
	for var in others {
		if !vars.contains(&var) {
			vars.push(var);
		}
	}
	(dir, vars)
}

/// The profile cargo's own `inherits` names where the manifest names none: `test`'s and
/// `bench`'s, and that of `debug`, which newer cargos define (1.98 refuses it).
fn default_inherits(name: &str) -> Option<&'static str> {
	match name {
		"test" | "debug" => Some("dev"),
		"bench" => Some("release"),
		_ => None,
	}
}

/// The [`CheckProfile::root`] and [`CheckProfile::dir`] of the profile `picked` under the
/// manifest's `profiles`, and the profiles it inherits from, itself first.
fn resolve_profile<'a>(
	picked: &'a str,
	profiles: Option<&'a toml::Table>,
) -> Result<(&'static str, Option<String>, Vec<&'a str>), String> {
	let table = |name: &str| profiles.and_then(|p| p.get(name)).and_then(|p| p.as_table());
	// Whether a profile that shares its directory's root's is that root: its table sets
	// nothing but the `inherits` cargo gives it anyway.
	let same = |name: &str| {
		table(name).is_none_or(|own| {
			own.iter()
				.all(|(key, value)| key == "inherits" && value.as_str() == default_inherits(name))
		})
	};
	let dir = match picked {
		"dev" | "test" | "debug" => (same("test") && same("debug")).then(|| "debug".to_string()),
		"release" | "bench" => same("bench").then(|| "release".to_string()),
		name => Some(name.to_string()),
	};
	let mut name = picked;
	let mut chain = Vec::new();
	// Cargo refuses a cycle, which this bound stops at before cargo runs.
	for _ in 0..=profiles.map_or(0, |p| p.len()) + 1 {
		chain.push(name);
		match name {
			"dev" => return Ok(("debug", dir, chain)),
			"release" => return Ok(("release", dir, chain)),
			_ => {}
		}
		let inherits = table(name).and_then(|p| p.get("inherits")).and_then(|i| i.as_str());
		name = inherits.or_else(|| default_inherits(name)).ok_or_else(|| {
			format!(
				"no profile `{name}` inherits from another here, where the profiles the export's library builds with are"
			)
		})?;
	}
	Err(format!("the profile `{name}` inherits from itself"))
}

/// The cargo args with each group of short flags, such as `-qr` or `-vFa`, apart, as
/// `-q -r` and `-v -Fa`, so that each reader of them finds the one it picks. A group with
/// a letter cargo gives no short flag stays as it is, for cargo to refuse.
fn ungroup(cargo: &[String]) -> Vec<String> {
	let mut out = Vec::new();
	let mut args = cargo.iter();
	while let Some(arg) = args.next() {
		if arg == "--" {
			out.extend(std::iter::once(arg).chain(args).cloned());
			break;
		}
		let group = arg
			.strip_prefix('-')
			.filter(|g| g.len() > 1 && g.starts_with(['q', 'v', 'r', 'h']));
		let Some(group) = group else {
			out.push(arg.clone());
			continue;
		};
		let mut split = Vec::new();
		let mut whole = true;
		for (at, flag) in group.char_indices() {
			match flag {
				'q' | 'v' | 'r' | 'h' => split.push(format!("-{flag}")),
				// A flag that takes a value takes the rest of the group as written, an empty
				// `=` value included.
				'p' | 'j' | 'F' | 'Z' | 'C' => {
					split.push(format!("-{}", &group[at..]));
					break;
				}
				_ => {
					whole = false;
					break;
				}
			}
		}
		match whole {
			true => out.extend(split),
			false => out.push(arg.clone()),
		}
	}
	out
}

/// The cargo args that pick the profile: `--release`, `-r`, and `--profile`.
fn profile_args(cargo: &[String]) -> Vec<String> {
	let mut picked = Vec::new();
	let mut args = cargo.iter();
	while let Some(arg) = args.next() {
		match arg.as_str() {
			"--release" | "-r" => picked.push(arg.clone()),
			"--profile" => picked.extend([Some(arg), args.next()].into_iter().flatten().cloned()),
			_ if arg.starts_with("--profile=") => picked.push(arg.clone()),
			_ => {}
		}
	}
	picked
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

/// Whether `a` and `b` are one file, through a symlink or a hard link.
fn same_file(a: &Path, b: &Path) -> bool {
	#[cfg(unix)]
	{
		use std::os::unix::fs::MetadataExt;
		matches!((std::fs::metadata(a), std::fs::metadata(b)), (Ok(a), Ok(b)) if (a.dev(), a.ino()) == (b.dev(), b.ino()))
	}
	#[cfg(not(unix))]
	{
		matches!((std::fs::canonicalize(a), std::fs::canonicalize(b)), (Ok(a), Ok(b)) if a == b)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn matches_partial_versions_as_cargo_does() {
		let package = |version: &str| Package {
			id: String::new(),
			name: "foo".into(),
			version: version.into(),
			source: None,
			manifest_path: PathBuf::new(),
			targets: Vec::new(),
			dependencies: Vec::new(),
		};
		for (spec, version, want) in [
			("foo", "1.0.0-alpha", true),
			("foo@1", "1.2.3", true),
			("foo@1.2", "1.2.3+build", true),
			("foo@1.2.3", "1.2.3+build", true),
			("foo@1", "1.0.0-alpha", false),
			("foo@1.0", "1.0.0-alpha", false),
			("foo@1.0.0", "1.0.0-alpha", false),
			("foo@1.0.0-alpha", "1.0.0-alpha+build", true),
			("foo@1.0.0-alpha", "1.0.0-alpha.1", false),
			("foo@1.1", "1.10.0", false),
			("bar@1", "1.0.0", false),
		] {
			assert_eq!(matches(spec, &package(version)), Ok(want), "{spec} against {version}");
		}
	}

	#[test]
	fn reads_what_picks_the_profile() {
		for (cargo, want) in [
			(&["--release", "--locked"][..], &["--release"][..]),
			(&["-r"], &["-r"]),
			(&["--features", "a", "--profile", "bench"], &["--profile", "bench"]),
			(&["--profile=bench", "--offline"], &["--profile=bench"]),
			(&["--features", "release"], &[]),
		] {
			let cargo: Vec<String> = cargo.iter().map(|s| s.to_string()).collect();
			assert_eq!(profile_args(&cargo), want, "{cargo:?}");
		}
	}

	#[test]
	fn ungroups_short_flags() {
		for (cargo, want) in [
			(&["-qr", "--locked"][..], &["-q", "-r", "--locked"][..]),
			(&["-vvFa,b"], &["-v", "-v", "-Fa,b"]),
			(&["-qF=a"], &["-q", "-F=a"]),
			(&["-qF="], &["-q", "-F="]),
			(&["-qp", "foo"], &["-q", "-p", "foo"]),
			(&["-r", "-Frelease", "-q"], &["-r", "-Frelease", "-q"]),
			(&["-qx"], &["-qx"]),
			(&["--", "-qr"], &["--", "-qr"]),
		] {
			let cargo: Vec<String> = cargo.iter().map(|s| s.to_string()).collect();
			assert_eq!(ungroup(&cargo), want, "{cargo:?}");
		}
	}

	#[test]
	fn watches_the_variables_of_each_inherited_profile() {
		let profiles: toml::Table = toml::from_str("[ship-it]\ninherits = \"test\"").unwrap();
		let (_, _, chain) = resolve_profile("ship-it", Some(&profiles)).unwrap();
		assert_eq!(chain, ["ship-it", "test", "dev"]);
		let vars = profile_vars(&chain);
		assert_eq!(vars.len(), 3 * PROFILE_KEYS.len());
		assert!(vars.contains(&"CARGO_PROFILE_SHIP_IT_DEBUG".to_string()), "{vars:?}");
		assert!(
			vars.contains(&"CARGO_PROFILE_DEV_BUILD_OVERRIDE_DEBUG_ASSERTIONS".to_string()),
			"{vars:?}"
		);
	}

	#[test]
	fn watches_the_other_profiles_of_a_shared_directory() {
		let release = Some("release".to_string());
		let bench = "CARGO_PROFILE_BENCH_BUILD_OVERRIDE_DEBUG_ASSERTIONS";
		let (dir, vars) = shared_vars(release.clone(), &["release"], |_| false);
		assert_eq!(dir, release);
		assert_eq!(vars.len(), 2 * PROFILE_KEYS.len());
		assert!(vars.contains(&bench.to_string()), "{vars:?}");
		// Watched once though the check's profile is the other one.
		let (_, vars) = shared_vars(release.clone(), &["bench", "release"], |_| false);
		assert_eq!(vars.len(), 2 * PROFILE_KEYS.len());
		assert_eq!(shared_vars(release, &["release"], |var| var == bench).0, None);
		let (_, vars) = shared_vars(Some("debug".to_string()), &["dev"], |_| false);
		assert_eq!(vars.len(), 3 * PROFILE_KEYS.len());
		let (_, vars) = shared_vars(Some("ship-it".to_string()), &["ship-it", "release"], |_| false);
		assert_eq!(vars.len(), 2 * PROFILE_KEYS.len());
	}

	#[test]
	fn resolves_the_profile_and_its_directory() {
		let resolve = |picked: &str, manifest: &str| {
			let profiles: toml::Table = toml::from_str(manifest).unwrap();
			(resolve_profile(picked, Some(&profiles)))
				.map(|(root, dir, _)| (root, dir))
				.map_err(|err| err.contains("inherits from another"))
		};
		let debug = Some("debug".to_string());
		let release = Some("release".to_string());
		for (picked, manifest, want) in [
			("dev", "", Ok(("debug", debug.clone()))),
			("release", "", Ok(("release", release.clone()))),
			// Cargo defines `debug` itself, inheriting `dev`'s, in `dev`'s directory.
			("debug", "", Ok(("debug", debug.clone()))),
			("debug", "[debug]\ninherits = \"dev\"", Ok(("debug", debug.clone()))),
			("dev", "[test]\ninherits = \"dev\"", Ok(("debug", debug.clone()))),
			(
				"release",
				"[bench]\ninherits = \"release\"",
				Ok(("release", release.clone())),
			),
			// A profile that sets its own, here only for a dependency, shares its directory
			// with another of other settings.
			("debug", "[debug]\nopt-level = 1", Ok(("debug", None))),
			(
				"dev",
				"[debug.package.dep]\ndebug-assertions = true",
				Ok(("debug", None)),
			),
			(
				"debug",
				"[test.package.dep]\ndebug-assertions = true",
				Ok(("debug", None)),
			),
			("dev", "[debug]\ninherits = \"release\"", Ok(("debug", None))),
			("debug", "[debug]\ninherits = \"release\"", Ok(("release", None))),
			(
				"release",
				"[bench.package.dep]\ndebug-assertions = true",
				Ok(("release", None)),
			),
			("release", "[debug]\nopt-level = 1", Ok(("release", release.clone()))),
			(
				"ship",
				"[ship]\ninherits = \"release\"",
				Ok(("release", Some("ship".to_string()))),
			),
			(
				"ship",
				"[ship]\ninherits = \"debug\"",
				Ok(("debug", Some("ship".to_string()))),
			),
			// `test` and `bench` follow an `inherits` of their own.
			(
				"ship",
				"[ship]\ninherits = \"test\"\n[test]\ninherits = \"release\"",
				Ok(("release", Some("ship".to_string()))),
			),
			("test", "[test]\ninherits = \"release\"", Ok(("release", None))),
			("bench", "[bench]\ninherits = \"dev\"", Ok(("debug", None))),
			("ship", "", Err(true)),
			("ship", "[ship]\ninherits = \"ship\"", Err(false)),
		] {
			assert_eq!(resolve(picked, manifest), want, "{picked} under {manifest:?}");
		}
	}

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
