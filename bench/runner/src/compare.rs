//! A base commit's counts against the working tree's, on this host.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::{Counts, Options, measure};

/// Counts `base` in a worktree of its own, then the working tree, and prints each change.
pub fn compare(base: &str, options: &Options) -> Result<(), String> {
	let top = PathBuf::from(git(Path::new("."), &["rev-parse", "--show-toplevel"])?);
	// Where the run is within the repository, so the base runs from the same place in its own.
	let prefix = git(Path::new("."), &["rev-parse", "--show-prefix"])?;
	let sha = git(
		Path::new("."),
		&["rev-parse", "--verify", &format!("{base}^{{commit}}")],
	)?;
	let tree = top.join("target/rust2-bench/base");
	checkout(&top, &tree, &sha)?;
	eprintln!(
		"rust2-bench: counting {base} ({}) in {}",
		&sha[..12.min(sha.len())],
		tree.display()
	);
	let before = measure(&tree.join(&prefix), options)?;
	eprintln!("rust2-bench: counting the working tree");
	let after = measure(Path::new("."), options)?;
	print!("{}", table(&before, &after)?);
	Ok(())
}

/// Checks `sha` out at `tree`, a detached worktree of the repository at `top` kept between
/// compares, so its build products stay warm for the next.
fn checkout(top: &Path, tree: &Path, sha: &str) -> Result<(), String> {
	let parent = tree.parent().expect("the base's worktree has a parent");
	std::fs::create_dir_all(parent).map_err(|e| format!("cannot create {}: {e}", parent.display()))?;
	// Keep the worktree out of the repository's status even where target/ is not ignored.
	std::fs::write(parent.join(".gitignore"), "*\n").map_err(|e| format!("cannot write {}: {e}", parent.display()))?;
	let path = tree.to_str().ok_or("the base's worktree path is not UTF-8")?;
	if tree.join(".git").exists() {
		git(tree, &["checkout", "--quiet", "--detach", "--force", sha])?;
	} else {
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
		writeln!(
			out,
			"| {name} | {} | {} | {change} |",
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
