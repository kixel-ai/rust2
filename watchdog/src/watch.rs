//! Times each test from the output of `cargo test`: cargo's `Running` and `Doc-tests` lines
//! name the binary, and libtest's lines name each test that runs.
//!
//! A multithreaded binary prints `test NAME has been running for over 60 seconds` 60 s after
//! a test starts and `test NAME ... ok` when it ends, so a test is timed from its own start
//! once it has run that long. A one-thread binary instead prints `test NAME ... ` as a test
//! starts, which starts its clock. Time is a `Duration` since the run started, so tests can
//! drive it.

use std::time::Duration;

use anyhow::{Result, bail};

use crate::cargo::Artifacts;
use crate::config::{Bound, Bounds, WARN_AFTER};

// A test's line is short; a longer partial line is output no rule reads, so it is dropped
// rather than held.
const MAX_LINE: usize = 64 * 1024;

const WARNED: &str = " has been running for over 60 seconds";

/// Why to kill the run, and the grace its bound gives between SIGTERM and SIGKILL.
#[derive(Debug, PartialEq)]
pub struct Kill {
	pub reason: String,
	pub grace: Duration,
}

/// The tests known to be running, and when each runs past its bound.
pub struct Watch<'a> {
	bounds: &'a Bounds,
	artifacts: &'a Artifacts,
	binary: Option<Binary>,
	running: Vec<Test>,
	// The test whose `test NAME ... ` started a line that has not ended in its result.
	pending: Option<String>,
	last_output: Duration,
	line: Vec<u8>,
	overlong: bool,
}

struct Binary {
	label: String,
	package: String,
}

struct Test {
	name: String,
	start: Duration,
	bound: Bound,
}

impl<'a> Watch<'a> {
	pub fn new(bounds: &'a Bounds, artifacts: &'a Artifacts) -> Self {
		Watch {
			bounds,
			artifacts,
			binary: None,
			running: Vec::new(),
			pending: None,
			last_output: Duration::ZERO,
			line: Vec::new(),
			overlong: false,
		}
	}

	/// Read output the command wrote at `now`; an error means the watchdog cannot time it.
	pub fn output(&mut self, bytes: &[u8], now: Duration) -> Result<()> {
		self.last_output = now;
		for &byte in bytes {
			if byte == b'\n' {
				let line = std::mem::take(&mut self.line);
				if !std::mem::take(&mut self.overlong) {
					self.complete(&clean(&line), now)?;
				}
			} else if self.line.len() < MAX_LINE {
				self.line.push(byte);
			} else {
				self.line.clear();
				self.overlong = true;
			}
		}
		// A one-thread binary ends the line only when the test ends.
		if !self.overlong
			&& let Some(name) = clean(&self.line)
				.strip_prefix("test ")
				.and_then(|rest| rest.strip_suffix(" ... "))
		{
			self.start(name, now)?;
		}
		Ok(())
	}

	/// When the earliest bound runs out, if any can.
	pub fn deadline(&self) -> Option<Duration> {
		if self.running.is_empty() {
			return Some(self.last_output + self.silence());
		}
		self.running
			.iter()
			.filter_map(|test| Some(test.start + test.bound.limit?))
			.min()
	}

	/// The bound that has run out by `now`, if one has.
	pub fn expired(&self, now: Duration) -> Option<Kill> {
		if now < self.deadline()? {
			return None;
		}
		let binary = self
			.binary
			.as_ref()
			.map_or("before cargo ran a test binary", |b| b.label.as_str());
		if self.running.is_empty() {
			return Some(Kill {
				reason: format!(
					"no output for {} and no test running, in {binary}",
					humantime::format_duration(self.silence())
				),
				grace: self.bounds.default.grace,
			});
		}
		let test = self
			.running
			.iter()
			.find(|test| test.bound.limit.is_some_and(|limit| now >= test.start + limit))?;
		Some(Kill {
			reason: format!(
				"test {} in {binary} ran past its {} bound",
				test.name,
				humantime::format_duration(test.bound.limit?)
			),
			grace: test.bound.grace,
		})
	}

	// A multithreaded binary names a test only once it has run 60 s, so silence counts only
	// after that, and a test with a longer bound is named before silence could end the run.
	fn silence(&self) -> Duration {
		WARN_AFTER + self.bounds.silence
	}

	fn complete(&mut self, line: &str, now: Duration) -> Result<()> {
		// cargo indents its status lines, which keeps other output from posing as one.
		if line.starts_with(' ') {
			let status = line.trim_start();
			if let Some(binary) = status.strip_prefix("Running ") {
				// `unittests src/lib.rs (target/debug/deps/rind-0123abcd)`
				let Some(path) = binary
					.strip_suffix(')')
					.and_then(|b| b.rsplit_once(" ("))
					.map(|(_, path)| path)
				else {
					bail!("cannot read the binary from cargo's line `{line}`");
				};
				let package = self.artifacts.executable(path)?.to_owned();
				self.enter(format!("{package}'s {binary}"), package);
			} else if let Some(name) = status.strip_prefix("Doc-tests ") {
				let package = self.artifacts.library(name)?.to_owned();
				self.enter(format!("{package}'s doc tests"), package);
			}
		} else if line.starts_with("test result: ") {
			self.running.clear();
			self.pending = None;
		} else if let Some(name) = warned(line) {
			if !self.running.iter().any(|test| test.name == name) {
				let bound = self.bound(name)?;
				self.running.push(Test {
					name: name.to_owned(),
					start: now.saturating_sub(WARN_AFTER),
					bound,
				});
			}
		} else if let Some(rest) = line.strip_prefix("test ") {
			if let Some((name, outcome)) = rest.split_once(" ... ") {
				if is_result(outcome) {
					self.end(name);
				} else {
					// Other output on the pipe, such as a cargo wrapper's, landed after a one-thread start.
					self.start(name, now)?;
				}
			}
		} else if is_result(line)
			&& let Some(name) = self.pending.take()
		{
			// A one-thread binary's result, on a line of its own after that other output.
			self.end(&name);
		}
		Ok(())
	}

	fn enter(&mut self, label: String, package: String) {
		self.binary = Some(Binary { label, package });
		self.running.clear();
		self.pending = None;
	}

	fn start(&mut self, name: &str, now: Duration) -> Result<()> {
		let name = strip_mode(name);
		self.pending = Some(name.to_owned());
		if !self.running.iter().any(|test| test.name == name) {
			let bound = self.bound(name)?;
			self.running.push(Test {
				name: name.to_owned(),
				start: now,
				bound,
			});
		}
		Ok(())
	}

	fn end(&mut self, name: &str) {
		let name = strip_mode(name);
		self.running.retain(|test| test.name != name);
		if self.pending.as_deref() == Some(name) {
			self.pending = None;
		}
	}

	fn bound(&self, name: &str) -> Result<Bound> {
		let Some(binary) = &self.binary else {
			bail!("libtest named test {name} before cargo named its binary");
		};
		Ok(self.bounds.get(&binary.package, name).clone())
	}
}

// libtest pads a name to align its results and marks a test's mode after it, as in
// `test tests::panics - should panic ... ok`.
fn strip_mode(name: &str) -> &str {
	let name = name.trim_end();
	["should panic", "compile fail", "compile"]
		.iter()
		.find_map(|mode| name.strip_suffix(mode)?.strip_suffix(" - "))
		.unwrap_or(name)
}

// The test libtest says is still running.
fn warned(line: &str) -> Option<&str> {
	line.strip_prefix("test ")?.strip_suffix(WARNED)
}

fn is_result(outcome: &str) -> bool {
	matches!(
		outcome.split([' ', ',']).next(),
		Some("ok" | "FAILED" | "ignored" | "bench:")
	)
}

// Colour, as cargo prints under CARGO_TERM_COLOR=always, and a carriage return would hide a
// line from the rules above.
fn clean(line: &[u8]) -> String {
	let text = String::from_utf8_lossy(line);
	let mut out = String::with_capacity(text.len());
	let mut chars = text.chars();
	while let Some(c) = chars.next() {
		if c == '\x1b' {
			// A CSI sequence ends at its first byte in @ to ~.
			if chars.next() == Some('[') {
				for c in chars.by_ref() {
					if ('@'..='~').contains(&c) {
						break;
					}
				}
			}
		} else if c != '\r' {
			out.push(c);
		}
	}
	out
}

#[cfg(test)]
mod tests {
	use super::*;

	const CONFIG: &str = r#"
[profile.default]
slow-timeout = { period = "60s", terminate-after = 5, grace-period = "3s" }

[[profile.default.overrides]]
filter = 'package(pkg-b) & test(=slow)'
slow-timeout = { period = "60s", terminate-after = 2 }

[[profile.default.overrides]]
filter = 'test(=forever)'
slow-timeout = "60s"
"#;

	const BUILD: &str = r#"{"reason":"compiler-artifact","package_id":"path+file:///w/a#pkg-a@0.1.0","target":{"name":"pkg_a","kind":["lib"]},"executable":"/w/target/debug/deps/pkg_a-aaaa"}
{"reason":"compiler-artifact","package_id":"path+file:///w/b#pkg-b@0.1.0","target":{"name":"pkg_b","kind":["lib"]},"executable":"/w/target/debug/deps/pkg_b-bbbb"}"#;

	const A: &str = "     Running unittests src/lib.rs (target/debug/deps/pkg_a-aaaa)\n\nrunning 3 tests\n";
	const B: &str = "     Running unittests src/lib.rs (target/debug/deps/pkg_b-bbbb)\n\nrunning 3 tests\n";

	struct Fixture {
		bounds: Bounds,
		artifacts: Artifacts,
	}

	impl Fixture {
		fn new() -> Self {
			Fixture {
				bounds: Bounds::parse(CONFIG, "default").unwrap(),
				artifacts: Artifacts::parse(BUILD, |_| {}).unwrap(),
			}
		}

		fn watch(&self) -> Watch<'_> {
			Watch::new(&self.bounds, &self.artifacts)
		}
	}

	fn secs(n: u64) -> Duration {
		Duration::from_secs(n)
	}

	fn feed(watch: &mut Watch<'_>, at: u64, text: &str) {
		watch.output(text.as_bytes(), secs(at)).unwrap();
	}

	fn reason(watch: &Watch<'_>, at: u64) -> Option<String> {
		watch.expired(secs(at)).map(|kill| kill.reason)
	}

	#[test]
	fn a_hung_test_is_timed_from_its_own_start() {
		let fixture = Fixture::new();
		let mut watch = fixture.watch();
		feed(&mut watch, 0, A);
		feed(&mut watch, 10, "test quick ... ok\n");
		// It started at 40 s, 60 s before libtest's warning.
		feed(&mut watch, 100, "test hangs has been running for over 60 seconds\n");
		assert_eq!(watch.deadline(), Some(secs(340)));
		assert_eq!(reason(&watch, 339), None);
		assert_eq!(
			watch.expired(secs(340)),
			Some(Kill {
				reason:
					"test hangs in pkg-a's unittests src/lib.rs (target/debug/deps/pkg_a-aaaa) ran past its 5m bound"
						.to_owned(),
				grace: secs(3),
			})
		);
	}

	#[test]
	fn a_slow_test_that_ends_is_forgotten() {
		let fixture = Fixture::new();
		let mut watch = fixture.watch();
		feed(&mut watch, 0, A);
		feed(&mut watch, 60, "test slow has been running for over 60 seconds\n");
		feed(&mut watch, 200, "test slow ... ok\n");
		// Only silence is bounded now, from the last output.
		assert_eq!(watch.deadline(), Some(secs(560)));
		assert_eq!(reason(&watch, 559), None);
		assert!(
			reason(&watch, 560)
				.unwrap()
				.starts_with("no output for 6m and no test running, in pkg-a's")
		);
	}

	#[test]
	fn overrides_match_the_package_cargo_ran() {
		let fixture = Fixture::new();
		let mut watch = fixture.watch();
		feed(&mut watch, 0, A);
		feed(&mut watch, 60, "test slow has been running for over 60 seconds\n");
		assert_eq!(watch.deadline(), Some(secs(300)));
		feed(&mut watch, 61, "test slow ... ok\n\ntest result: ok. 1 passed\n\n");
		feed(&mut watch, 62, B);
		feed(&mut watch, 122, "test slow has been running for over 60 seconds\n");
		assert_eq!(watch.deadline(), Some(secs(182)));
		assert!(reason(&watch, 182).unwrap().contains("pkg-b's"));
	}

	#[test]
	fn silence_waits_for_libtest_to_name_a_test() {
		// A test with a longer bound than the profile's is named at 60 s, before silence ends.
		let config = "[profile.default]\nslow-timeout = { period = \"30s\", terminate-after = 2 }\n[[profile.default.overrides]]\nfilter = 'test(=long)'\nslow-timeout = { period = \"60s\", terminate-after = 4 }";
		let bounds = Bounds::parse(config, "default").unwrap();
		let artifacts = Artifacts::parse(BUILD, |_| {}).unwrap();
		let mut watch = Watch::new(&bounds, &artifacts);
		feed(&mut watch, 0, A);
		assert_eq!(watch.deadline(), Some(secs(120)));
		feed(&mut watch, 61, "test long has been running for over 60 seconds\n");
		assert_eq!(watch.deadline(), Some(secs(241)));
	}

	#[test]
	fn a_test_without_terminate_after_is_never_killed() {
		let fixture = Fixture::new();
		let mut watch = fixture.watch();
		feed(&mut watch, 0, A);
		feed(&mut watch, 60, "test forever has been running for over 60 seconds\n");
		assert_eq!(watch.deadline(), None);
		assert_eq!(reason(&watch, 100_000), None);
	}

	#[test]
	fn a_one_thread_binary_starts_the_clock_at_the_start() {
		let fixture = Fixture::new();
		let mut watch = fixture.watch();
		feed(&mut watch, 0, A);
		feed(&mut watch, 1, "test quick ... ");
		feed(&mut watch, 1, "ok\n");
		feed(&mut watch, 2, "test hangs ... ");
		assert_eq!(watch.deadline(), Some(secs(302)));
		assert!(reason(&watch, 302).unwrap().starts_with("test hangs in"));
	}

	#[test]
	fn other_output_splits_a_one_thread_line() {
		let fixture = Fixture::new();
		let mut watch = fixture.watch();
		feed(&mut watch, 0, A);
		// The start and another process's line arrive together...
		feed(&mut watch, 5, "test chatty ... hello\nworld\n");
		assert_eq!(watch.deadline(), Some(secs(305)));
		// ...and the result ends a line of its own.
		feed(&mut watch, 6, "ok\n");
		assert_eq!(watch.deadline(), Some(secs(366)));
		assert!(reason(&watch, 366).unwrap().starts_with("no output"));
	}

	#[test]
	fn a_result_split_from_its_name_keeps_the_hung_test() {
		// A multithreaded binary flushes `test NAME ... ` before the result, so a read may
		// see the name alone while a warned test still runs.
		let fixture = Fixture::new();
		let mut watch = fixture.watch();
		feed(&mut watch, 0, A);
		feed(&mut watch, 60, "test hangs has been running for over 60 seconds\n");
		feed(&mut watch, 70, "test quick ... ");
		feed(&mut watch, 70, "ok\n");
		assert_eq!(watch.deadline(), Some(secs(300)));
		assert!(reason(&watch, 300).unwrap().starts_with("test hangs in"));
	}

	#[test]
	fn a_test_s_mode_and_padding_are_not_its_name() {
		let fixture = Fixture::new();
		let mut watch = fixture.watch();
		feed(&mut watch, 0, B);
		feed(&mut watch, 1, "test slow - should panic ... ");
		// pkg-b's slow is bounded at 2 minutes by name.
		assert_eq!(watch.deadline(), Some(secs(121)));
		feed(&mut watch, 2, "ok\n");
		feed(
			&mut watch,
			3,
			"test bench_x      ... bench:         12 ns/iter (+/- 1)\n",
		);
		assert_eq!(watch.deadline(), Some(secs(363)));
	}

	#[test]
	fn doc_tests_map_to_their_library_s_package() {
		let fixture = Fixture::new();
		let mut watch = fixture.watch();
		feed(&mut watch, 0, "   Doc-tests pkg_b\n\nrunning 1 test\n");
		feed(
			&mut watch,
			60,
			"test b/src/lib.rs - f (line 3) has been running for over 60 seconds\n",
		);
		assert!(
			reason(&watch, 300)
				.unwrap()
				.starts_with("test b/src/lib.rs - f (line 3) in pkg-b's doc tests")
		);
	}

	#[test]
	fn colour_and_carriage_returns_are_ignored() {
		let fixture = Fixture::new();
		let mut watch = fixture.watch();
		feed(
			&mut watch,
			0,
			"\x1b[1m\x1b[92m     Running\x1b[0m unittests src/lib.rs (target/debug/deps/pkg_b-bbbb)\r\n",
		);
		feed(&mut watch, 60, "test slow has been running for over 60 seconds\r\n");
		assert_eq!(watch.deadline(), Some(secs(120)));
	}

	#[test]
	fn silence_before_any_binary_is_bounded() {
		let fixture = Fixture::new();
		let watch = fixture.watch();
		assert_eq!(
			reason(&watch, 360).unwrap(),
			"no output for 6m and no test running, in before cargo ran a test binary"
		);
	}

	#[test]
	fn output_the_watchdog_cannot_time_fails() {
		let fixture = Fixture::new();
		let mut watch = fixture.watch();
		let error = watch.output(b"test a ... \n", secs(0)).unwrap_err();
		assert!(error.to_string().contains("before cargo named its binary"));
		let error = watch
			.output(b"     Running tests/x.rs (target/debug/deps/x-ffff)\n", secs(0))
			.unwrap_err();
		assert!(error.to_string().contains("did not build"));
	}

	#[test]
	fn an_overlong_line_is_dropped() {
		let fixture = Fixture::new();
		let mut watch = fixture.watch();
		feed(&mut watch, 0, A);
		let long = "x".repeat(MAX_LINE + 1);
		feed(&mut watch, 1, &long);
		assert!(watch.line.len() <= MAX_LINE);
		feed(&mut watch, 2, "\ntest quick ... ");
		assert_eq!(watch.deadline(), Some(secs(302)));
	}
}
