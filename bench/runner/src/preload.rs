//! The library every counted process preloads, built from `preload.rs` beside this crate: it
//! hands the process a fixed-seed stream for randomness and a clock that advances only as the
//! process reads it and waits, so a count repeats.

use std::path::{Path, PathBuf};

use crate::fingerprint::Fnv;

/// The library, which build.rs built for this target.
const LIBRARY: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/preload.so"));

/// What the library logs to Valgrind once it loads: preload.rs's `MARKER`, which a test keeps the
/// same.
const MARKER: &str = "rust2-bench preload: deterministic randomness and clock";

/// The library's hash, which the fingerprint holds, since its code runs inside what is counted.
pub fn hash() -> String {
	let mut hash = Fnv::default();
	hash.bytes(LIBRARY);
	format!("{:012x}", hash.0 >> 16)
}

/// Writes the library where every run on this host finds it at the same path, under the user's
/// cache, and returns that path. Named by its hash, it is rewritten only when missing or altered.
pub fn install() -> Result<PathBuf, String> {
	let path = crate::cache()?.join(format!("preload-{}.so", hash()));
	if std::fs::read(&path).is_ok_and(|bytes| bytes == LIBRARY) {
		return Ok(path);
	}
	// Written aside, then renamed, so a run beside this one never loads a partial library.
	let partial = path.with_extension(format!("so.{}", std::process::id()));
	std::fs::write(&partial, LIBRARY).map_err(|e| format!("cannot write {}: {e}", partial.display()))?;
	std::fs::rename(&partial, &path).map_err(|e| format!("cannot write {}: {e}", path.display()))?;
	Ok(path)
}

/// Fails unless the process `pid` of a job, whose Valgrind log is in `dir`, loaded the library.
pub fn check(dir: &Path, pid: &str, label: &str) -> Result<(), String> {
	let log = dir.join(format!("valgrind.{pid}.log"));
	let text = std::fs::read_to_string(&log).map_err(|e| format!("cannot read {}: {e}", log.display()))?;
	if text.contains(MARKER) {
		return Ok(());
	}
	Err(format!(
		"{label} measured a benchmark in a process that never loaded rust2-bench's preload, so its count \
		 follows the host's randomness and clock: a static executable, one whose environment drops \
		 LD_PRELOAD, or a glibc before 2.34 bypasses it; its output is in {}",
		dir.display()
	))
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
	fn the_library_logs_the_marker() {
		assert!(
			LIBRARY.windows(MARKER.len()).any(|window| window == MARKER.as_bytes()),
			"preload.rs's MARKER is not {MARKER:?}"
		);
	}
}
