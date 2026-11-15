use anyhow::{Context, Result, bail, ensure};
use std::{
	fs, io,
	io::Write,
	path::{Path, PathBuf},
	process::Command,
};

/// The caller workflow, pinned to the pipeline release's commit.
const TEMPLATE: &str = include_str!("binaries.yml");

/// Where the caller lives, relative to the repository root.
pub const PATH: &str = ".github/workflows/binaries.yml";

/// Write the pipeline's caller workflow into the current git repository.
pub fn run() -> Result<()> {
	let root =
		PathBuf::from(git(None, &["rev-parse", "--show-toplevel"]).context("crabx init runs inside a git repository")?);
	let manifests = git(Some(&root), &["ls-files", "--", ":(glob)**/Cargo.toml"])?;
	ensure!(!manifests.is_empty(), "{} tracks no Cargo.toml", root.display());
	let head = git(Some(&root), &["symbolic-ref", "--short", "refs/remotes/origin/HEAD"])
		.context("cannot find origin's default branch; run `git remote set-head origin --auto`")?;
	let branch = head
		.strip_prefix("origin/")
		.context("origin/HEAD names another remote")?;
	let contents = workflow(branch);

	// Walk to the workflow directory without following a symlink, which a
	// hostile clone could point outside the repository.
	let mut dir = root.clone();
	for component in Path::new(PATH).parent().expect("PATH has a parent") {
		dir.push(component);
		match fs::symlink_metadata(&dir) {
			Ok(meta) if meta.is_dir() => {}
			Ok(_) => bail!("{} is not a directory inside the repository", dir.display()),
			Err(error) if error.kind() == io::ErrorKind::NotFound => {
				fs::create_dir(&dir).with_context(|| format!("create {}", dir.display()))?
			}
			Err(error) => return Err(error).with_context(|| format!("read {}", dir.display())),
		}
	}
	let path = root.join(PATH);
	match fs::read(&path) {
		Ok(existing) if existing == contents.as_bytes() => {
			println!("{} is up to date", path.display());
			return Ok(());
		}
		Ok(_) => bail!(
			"{} already exists; delete it to write the crabx pipeline's caller",
			path.display()
		),
		Err(error) if error.kind() == io::ErrorKind::NotFound => {}
		Err(error) => return Err(error).with_context(|| format!("read {}", path.display())),
	}
	// Publish only a complete file, and never over one another process wrote
	// since the read above.
	let mut file =
		tempfile::NamedTempFile::new_in(&dir).with_context(|| format!("create a file in {}", dir.display()))?;
	file.write_all(contents.as_bytes())
		.with_context(|| format!("write {}", path.display()))?;
	file.persist_noclobber(&path)
		.with_context(|| format!("write {}", path.display()))?;
	println!("wrote {}; commit it and push it to {branch}", path.display());
	Ok(())
}

/// The caller workflow whose pushes to `branch` trigger the pipeline.
pub fn workflow(branch: &str) -> String {
	// Escape GitHub's filter pattern characters, then quote for YAML.
	let mut pattern = String::new();
	for c in branch.chars() {
		if "*?+[]!\\".contains(c) {
			pattern.push('\\');
		}
		pattern.push(c);
	}
	let quoted = format!("'{}'", pattern.replace('\'', "''"));
	TEMPLATE.replacen("BRANCH", &quoted, 1)
}

fn git(dir: Option<&Path>, args: &[&str]) -> Result<String> {
	let mut command = Command::new("git");
	if let Some(dir) = dir {
		command.current_dir(dir);
	}
	let output = command.args(args).output().context("run git")?;
	if !output.status.success() {
		bail!(
			"git {}: {}",
			args.join(" "),
			String::from_utf8_lossy(&output.stderr).trim()
		);
	}
	Ok(String::from_utf8(output.stdout)
		.context("git printed non-UTF-8 output")?
		.trim_end_matches(['\n', '\r'])
		.to_owned())
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn branch_patterns_are_literal() {
		assert!(workflow("main").contains("    branches: ['main']\n"));
		assert!(workflow("it's+1").contains("    branches: ['it''s\\+1']\n"));
		assert!(workflow("!x[").contains("    branches: ['\\!x\\[']\n"));
	}

	#[test]
	fn pipeline_readme_shows_the_written_workflow() {
		let readme = fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/../.github/crabx/README.md")).unwrap();
		let block = format!("```yaml\n{}```", workflow("main"));
		assert!(readme.contains(&block), "README's caller differs from crabx init's");
	}
}
