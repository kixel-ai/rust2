//! A base commit's counts against the working tree's, on this host.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::fingerprint::Fnv;
use crate::{Counts, Options, measure, missing};

/// Counts `base` in a worktree of its own, then the working tree, and prints each change.
pub fn compare(base: &str, options: &Options) -> Result<(), String> {
	let top = PathBuf::from(git(Path::new("."), &["rev-parse", "--show-toplevel"])?);
	// Where the run is within the repository, so the base runs from the same place in its own.
	let prefix = git(Path::new("."), &["rev-parse", "--show-prefix"])?;
	let sha = git(
		Path::new("."),
		&["rev-parse", "--verify", &format!("{base}^{{commit}}")],
	)?;
	let real = top
		.canonicalize()
		.map_err(|e| format!("cannot resolve {}: {e}", top.display()))?;
	let tree = base_tree(&real)?;
	// Another compare of this repository shares the base's worktree, so it waits until this
	// one has counted the base rather than check out another commit under it.
	let lock = tree.with_extension("lock");
	let lock = std::fs::File::create(&lock).map_err(|e| format!("cannot create {}: {e}", lock.display()))?;
	lock.lock()
		.map_err(|e| format!("cannot lock the base's worktree: {e}"))?;
	checkout(&top, &tree, &sha)?;
	eprintln!(
		"rust2-bench: counting {base} ({}) in {}",
		&sha[..12.min(sha.len())],
		tree.display()
	);
	let mut base = options.clone();
	base.cargo = rebase(&options.cargo, &real, &tree)?;
	// A wanted benchmark may be new at the head or gone from it, but must be on one side.
	let before = measure(&tree.join(&prefix), &base, false)?;
	drop(lock);
	eprintln!("rust2-bench: counting the working tree");
	let after = measure(Path::new("."), options, false)?;
	if let Some(wanted) = &options.benchmarks {
		missing(wanted, |name| {
			before.counts.contains_key(name) || after.counts.contains_key(name)
		})?;
	}
	if before.counts.is_empty() && after.counts.is_empty() {
		return Err("no benchmark ran".to_owned());
	}
	print!("{}", table(&before, &after)?);
	Ok(())
}

/// Where the base's worktree for the repository at `top` lives: in the user's cache, outside
/// the repository, since Cargo reads every `.cargo/config.toml` above where it builds, and the
/// working tree's own must not configure the base's build. `top` is canonical.
fn base_tree(top: &Path) -> Result<PathBuf, String> {
	let cache = crate::cache()?;
	if cache.starts_with(top) {
		return Err(format!(
			"the cache {} is inside the repository, where its .cargo/config.toml would configure the base: point XDG_CACHE_HOME outside it",
			cache.display()
		));
	}
	let mut hash = Fnv::default();
	hash.write(&top.to_string_lossy());
	let name = top.file_name().map_or("repo".into(), |name| name.to_string_lossy());
	Ok(cache.join(format!("{name}-{:012x}", hash.0 >> 16)))
}

/// `args` for the base's worktree: an absolute `--manifest-path` in the repository at `top`
/// moves to the same file under `tree`, and one outside it, which would count the same
/// workspace twice, is refused.
fn rebase(args: &[String], top: &Path, tree: &Path) -> Result<Vec<String>, String> {
	let mut out = Vec::with_capacity(args.len());
	let mut args = args.iter();
	while let Some(arg) = args.next() {
		if arg == "--" {
			out.push(arg.clone());
			out.extend(args.by_ref().cloned());
			break;
		}
		let (flag, path) = if arg == "--manifest-path" {
			out.push(arg.clone());
			match args.next() {
				Some(path) => ("", path.as_str()),
				None => break,
			}
		} else if let Some(path) = arg.strip_prefix("--manifest-path=") {
			("--manifest-path=", path)
		} else {
			out.push(arg.clone());
			continue;
		};
		let rebased = if Path::new(path).is_absolute() {
			let canonical = Path::new(path)
				.canonicalize()
				.map_err(|e| format!("cannot resolve {path}: {e}"))?;
			let relative = canonical.strip_prefix(top).map_err(|_| {
				format!("--manifest-path {path} is outside the repository, so compare would count it twice")
			})?;
			tree.join(relative)
				.to_str()
				.ok_or("the base's worktree path is not UTF-8")?
				.to_owned()
		} else {
			path.to_owned()
		};
		out.push(format!("{flag}{rebased}"));
	}
	Ok(out)
}

/// Checks `sha` out at `tree`, a detached worktree of the repository at `top` kept between
/// compares, so its build products stay warm for the next.
fn checkout(top: &Path, tree: &Path, sha: &str) -> Result<(), String> {
	let path = tree.to_str().ok_or("the base's worktree path is not UTF-8")?;
	if tree.join(".git").exists() {
		git(tree, &["checkout", "--quiet", "--detach", "--force", sha])?;
	} else {
		// Forget a worktree whose directory a cache cleanup removed, so it can be added again.
		git(top, &["worktree", "prune"])?;
		git(top, &["worktree", "add", "--quiet", "--detach", path, sha])?;
	}
	if tree.join(".gitmodules").exists() {
		git(tree, &["submodule", "update", "--quiet", "--init", "--recursive"])?;
	}
	Ok(())
}

/// Runs git in `dir`, returning its trimmed stdout.
fn git(dir: &Path, args: &[&str]) -> Result<String, String> {
	let out = Command::new("git")
		.args(args)
		.current_dir(dir)
		.output()
		.map_err(|e| format!("cannot run git: {e}"))?;
	if !out.status.success() {
		return Err(format!(
			"git {} failed: {}",
			args.join(" "),
			String::from_utf8_lossy(&out.stderr).trim()
		));
	}
	String::from_utf8(out.stdout)
		.map(|s| s.trim().to_owned())
		.map_err(|_| "git printed something not UTF-8".to_owned())
}

/// The change of each benchmark from `before` to `after`, a Markdown table, refusing counts
/// from two environments, which do not compare.
fn table(before: &Counts, after: &Counts) -> Result<String, String> {
	if before.fingerprint != after.fingerprint {
		return Err(format!(
			"the base counted under fingerprint {} and the working tree under {}: their counts do not compare",
			before.fingerprint, after.fingerprint
		));
	}
	let names: BTreeSet<&String> = before.counts.keys().chain(after.counts.keys()).collect();
	let mut out = String::from("| benchmark | base | head | change |\n|---|---:|---:|---:|\n");
	let cell = |counts: &BTreeMap<String, u64>, name: &str| counts.get(name).map_or("-".to_owned(), u64::to_string);
	for name in names {
		let change = match (before.counts.get(name), after.counts.get(name)) {
			(Some(&b), Some(&a)) => format!("{:+.2}%", (a as f64 - b as f64) / b as f64 * 100.0),
			(None, Some(_)) => "added".to_owned(),
			(Some(_), None) => "removed".to_owned(),
			(None, None) => unreachable!("every name comes from one side"),
		};
		// A `|` in a name would split its cell.
		let shown = name.replace('|', "\\|");
		writeln!(
			out,
			"| {shown} | {} | {} | {change} |",
			cell(&before.counts, name),
			cell(&after.counts, name)
		)
		.expect("a String takes every write");
	}
	Ok(out)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn rebases_an_absolute_manifest_path_into_the_base() {
		let top = Path::new(env!("CARGO_MANIFEST_DIR")).canonicalize().unwrap();
		let manifest = top.join("Cargo.toml");
		let manifest = manifest.to_str().unwrap();
		let tree = Path::new("/base");
		let args = |list: &[&str]| list.iter().map(|s| (*s).to_owned()).collect::<Vec<_>>();
		let rebased = rebase(
			&args(&[
				"--manifest-path",
				manifest,
				"--locked",
				"--",
				"--manifest-path",
				manifest,
			]),
			&top,
			tree,
		)
		.unwrap();
		assert_eq!(
			rebased,
			args(&[
				"--manifest-path",
				"/base/Cargo.toml",
				"--locked",
				"--",
				"--manifest-path",
				manifest
			])
		);
		let equals = format!("--manifest-path={manifest}");
		assert_eq!(
			rebase(&args(&[&equals]), &top, tree).unwrap(),
			args(&["--manifest-path=/base/Cargo.toml"])
		);
		assert_eq!(
			rebase(&args(&["--manifest-path", "Cargo.toml"]), &top, tree).unwrap(),
			args(&["--manifest-path", "Cargo.toml"])
		);
		let err = rebase(&args(&["--manifest-path", manifest]), &top.join("src"), tree).unwrap_err();
		assert!(err.contains("outside the repository"), "{err}");
	}

	fn counts(fingerprint: &str, names: &[(&str, u64)]) -> Counts {
		Counts {
			fingerprint: fingerprint.to_owned(),
			counts: names.iter().map(|(name, ir)| ((*name).to_owned(), *ir)).collect(),
		}
	}

	#[test]
	fn tables_each_change_and_each_benchmark_one_side_has() {
		let before = counts("f", &[("a", 200), ("gone", 5)]);
		let after = counts("f", &[("a", 150), ("new", 7)]);
		assert_eq!(
			table(&before, &after).unwrap(),
			"| benchmark | base | head | change |\n|---|---:|---:|---:|\n\
			 | a | 200 | 150 | -25.00% |\n| gone | 5 | - | removed |\n| new | - | 7 | added |\n"
		);
	}

	#[test]
	fn refuses_two_fingerprints() {
		let err = table(&counts("f", &[("a", 1)]), &counts("g", &[("a", 1)])).unwrap_err();
		assert!(err.contains("do not compare"), "{err}");
	}
}
