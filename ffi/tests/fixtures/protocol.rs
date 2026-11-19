//! Random sequences of the ownership protocol's steps, each run against the model in
//! `tests/protocol/model.rs` of what it must return: a driver runs a script through its
//! surface, printing each step's outcome, and the model's transcript must match it.

#[path = "../protocol/model.rs"]
#[allow(dead_code, reason = "the Miri driver reads what writing scripts does not")]
pub mod model;

use std::path::Path;
use std::process::Command;

pub use model::{Profile, Script};

/// The scripts of `count` seeds from `first`, each `len` random steps long, for a surface
/// of `profile`.
pub fn scripts(first: u64, count: u64, len: usize, profile: Profile) -> Vec<Script> {
	(first..first + count)
		.map(|seed| model::script(seed, len, profile))
		.collect()
}

/// Fail unless `scripts` make every kind of handle and meet every outcome, each status
/// included, so a sampler that stops reaching one fails rather than passing on less.
pub fn covered(scripts: &[Script]) {
	use model::{Kind, Outcome, Status};
	let outcomes: Vec<Outcome> = scripts
		.iter()
		.flat_map(|script| script.steps.iter().map(|step| step.expect))
		.collect();
	for status in [
		Status::Moved,
		Status::Exclusive,
		Status::Borrowed,
		Status::Lent,
		Status::Outlived,
		Status::NotSend,
	] {
		assert!(
			outcomes.contains(&Outcome::Failed(status)),
			"no step fails as {status:?}"
		);
	}
	for outcome in [Outcome::Pending, Outcome::Error] {
		assert!(outcomes.contains(&outcome), "no step returns {outcome:?}");
	}
	for kind in [
		Kind::Counter,
		Kind::Ledger,
		Kind::Draft,
		Kind::Receipt,
		Kind::Tally,
		Kind::Local,
		Kind::Meters,
		Kind::Meter,
		Kind::PlusLater,
		Kind::BumpLater,
		Kind::LocalLater,
	] {
		assert!(
			scripts.iter().any(|script| script.kinds.contains(&kind)),
			"no script makes a {kind:?}"
		);
	}
}

/// Run each script through `driver`, which takes the script's path as its last argument,
/// in `dir`, failing at the first step whose outcome differs from the model's.
pub fn check(dir: &Path, scripts: &[Script], driver: impl Fn(&Path) -> Command) {
	std::fs::create_dir_all(dir).unwrap();
	for script in scripts {
		let path = dir.join(format!("script-{}.txt", script.seed));
		std::fs::write(&path, script.text()).unwrap();
		let output = driver(&path).arg(&path).output().unwrap();
		let stdout = String::from_utf8_lossy(&output.stdout);
		if output.status.success() && stdout == script.transcript() {
			continue;
		}
		panic!(
			"script {} ({}) differs from the model{}\nstderr:\n{}",
			script.seed,
			path.display(),
			difference(script, &stdout),
			String::from_utf8_lossy(&output.stderr),
		);
	}
}

/// Scripts per process of [`check_all`]'s driver, so its arguments stay well within any
/// platform's limit.
const BATCH: usize = 500;

/// Run every script through processes of `driver`, a batch each, which take each script's
/// path as an argument and write its outcomes to that path with `.out` added, as a driver
/// whose startup costs more than a script does: the model's transcript must match each.
pub fn check_all(dir: &Path, scripts: &[Script], driver: impl Fn() -> Command) {
	std::fs::create_dir_all(dir).unwrap();
	for batch in scripts.chunks(BATCH) {
		let paths: Vec<_> = batch
			.iter()
			.map(|script| {
				let path = dir.join(format!("script-{}.txt", script.seed));
				std::fs::write(&path, script.text()).unwrap();
				let _ = std::fs::remove_file(out(&path));
				path
			})
			.collect();
		let output = driver().args(&paths).output().unwrap();
		let stderr = String::from_utf8_lossy(&output.stderr);
		for (script, path) in batch.iter().zip(&paths) {
			let got = std::fs::read_to_string(out(path)).unwrap_or_default();
			if got == script.transcript() {
				continue;
			}
			panic!(
				"script {} ({}) differs from the model{}\nstderr:\n{stderr}",
				script.seed,
				path.display(),
				difference(script, &got),
			);
		}
		assert!(output.status.success(), "the driver failed:\n{stderr}");
	}
}

/// Where [`check_all`]'s driver writes a script's outcomes.
fn out(script: &Path) -> std::path::PathBuf {
	let mut path = script.as_os_str().to_owned();
	path.push(".out");
	path.into()
}

/// Where a driver's output first parts from the model's, with the steps leading there.
fn difference(script: &Script, stdout: &str) -> String {
	let expected = script.transcript();
	let mut got = stdout.lines();
	for (i, want) in expected.lines().enumerate() {
		let line = got.next();
		if line == Some(want) {
			continue;
		}
		let step = script
			.steps
			.get(i)
			.map_or("the waker's counts".to_string(), |step| step.line());
		let from = i.saturating_sub(12);
		let context: Vec<String> = script.steps[from..i.min(script.steps.len())]
			.iter()
			.map(|step| format!("  {} -> {}", step.line(), step.expect.line()))
			.collect();
		return format!(
			" at step {i}, `{step}`: the model says {want}, the driver {}\nsteps before it:\n{}",
			line.unwrap_or("nothing"),
			context.join("\n")
		);
	}
	": the driver printed more".to_string()
}

/// Each profile's scripts keep to it: no worker steps, failed checks, frees, drops, polls,
/// awaits, or drains where it has none, locals on a home where it says, and each future
/// awaited in the next step where none pends. Of the profiles its flags make, a fixed
/// sample, each with a script of its own seed.
#[test]
fn protocol_profiles() {
	use model::{Op, Outcome, Status};
	for seed in 0..4096u64 {
		// Fibonacci hashing spreads consecutive seeds over the flags' bits.
		let bits = seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) >> 44;
		let on = |bit: u16| bits & 1 << bit != 0;
		let profile = Profile {
			threads: on(0),
			homes: on(1),
			homed: on(2),
			frees: on(3),
			polls: on(4),
			failures: on(5),
			settles: on(6),
			cancels: on(7),
			lazy: on(8),
			drops: on(9),
			closes: on(10),
			moves_close: on(11),
			borrowck: on(12),
			consumes: on(13),
			pooled: on(14),
			drain: on(15),
			pending: match bits >> 16 & 3 {
				0 => 0,
				1 => 1,
				_ => usize::MAX,
			},
		};
		if (profile.lazy && profile.polls)
			|| !(profile.frees || profile.drops)
			|| (profile.polls && profile.pending == 0)
			|| (profile.lazy && !(profile.borrowck && profile.failures))
			|| !(profile.failures || profile.drain || !profile.threads)
		{
			continue;
		}
		for script in scripts(seed, 1, 100, profile) {
			for (i, step) in script.steps.iter().enumerate() {
				let line = format!("script {}: {} -> {}", script.seed, step.line(), step.expect.line());
				assert!(profile.threads || step.thread == 0, "{profile:?}: {line}");
				assert!(
					profile.failures || !matches!(step.expect, Outcome::Failed(_)),
					"{profile:?}: {line}"
				);
				assert!(
					profile.borrowck || step.expect != Outcome::Failed(Status::Moved),
					"{profile:?}: {line}"
				);
				match step.op {
					Op::Free(h) => assert!(
						profile.frees || (profile.cancels && script.kinds[h].future()),
						"{profile:?}: {line}"
					),
					Op::Drop(_) => assert!(profile.drops, "{profile:?}: {line}"),
					Op::Poll(_) => assert!(profile.polls, "{profile:?}: {line}"),
					Op::Await(_) => assert!(!profile.polls, "{profile:?}: {line}"),
					Op::Drain => assert!(profile.drain, "{profile:?}: {line}"),
					Op::LocalNew(_, homed) => assert!(
						if profile.homed { homed } else { profile.homes || !homed },
						"{profile:?}: {line}"
					),
					_ => {}
				}
				// A future where none pends is awaited in the next step, on its thread.
				if let Some(f) = step.op.made()
					&& script.kinds.get(f).is_some_and(|kind| kind.future())
					&& step.expect == Outcome::Ok
					&& profile.pending == 0
				{
					let next = &script.steps[i + 1];
					assert!(
						next.op == Op::Await(f) && next.thread == step.thread,
						"{profile:?}: {line}"
					);
				}
			}
		}
	}
}
