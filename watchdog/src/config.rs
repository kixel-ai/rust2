//! Each test's bound, read from a nextest profile's `slow-timeout` and its overrides, so one
//! `.config/nextest.toml` bounds both runners.

use std::time::Duration;

use anyhow::{Context, Result, bail};
use toml::{Table, Value};

use crate::filter::Filter;

/// How long a test runs before libtest says it is still running; a multithreaded binary
/// names a test no sooner, so no bound may be shorter.
pub const WARN_AFTER: Duration = Duration::from_secs(60);

// nextest's default grace between SIGTERM and SIGKILL.
const GRACE: Duration = Duration::from_secs(10);

/// How long a test may run, and how long the run then gets between SIGTERM and SIGKILL.
#[derive(Clone, Debug, PartialEq)]
pub struct Bound {
	/// `None` where nextest would let the test run forever: a `slow-timeout` without
	/// `terminate-after`.
	pub limit: Option<Duration>,
	pub grace: Duration,
}

/// A profile's bounds: its own, and its overrides' in nextest's order.
#[derive(Debug)]
pub struct Bounds {
	/// The profile's bound, which also bounds silence while no test is known to run.
	pub default: Bound,
	/// The profile's limit, which a profile must set.
	pub silence: Duration,
	overrides: Vec<(Filter, Bound)>,
}

impl Bounds {
	/// Read profile `profile` from a nextest config, failing on anything that would change a
	/// bound that the watchdog cannot honor.
	pub fn parse(text: &str, profile: &str) -> Result<Bounds> {
		let config: Table = text.parse().context("parsing the nextest config")?;
		let empty = Table::new();
		let profiles = match config.get("profile") {
			None => &empty,
			Some(value) => table(value, "profile")?,
		};
		let default = match profiles.get("default") {
			None => &empty,
			Some(value) => table(value, "profile.default")?,
		};
		let selected = match profiles.get(profile) {
			Some(value) => table(value, &format!("profile.{profile}"))?,
			None if profile == "default" => &empty,
			None => bail!("the nextest config has no profile `{profile}`"),
		};
		for (name, table) in [(profile, selected), ("default", default)] {
			if table.contains_key("inherits") {
				bail!("profile.{name}.inherits is not supported");
			}
		}

		let (at, value) = match selected.get("slow-timeout") {
			Some(value) => (format!("profile.{profile}.slow-timeout"), value),
			None => match default.get("slow-timeout") {
				Some(value) => ("profile.default.slow-timeout".to_owned(), value),
				None => bail!("profile `{profile}` sets no slow-timeout, so nothing bounds its tests"),
			},
		};
		let default_bound = slow_timeout(value, &at)?;
		let Some(silence) = default_bound.limit else {
			bail!("{at} has no terminate-after, so nothing bounds its tests");
		};

		// The profile's own overrides come before the default profile's, and the first that
		// matches and sets slow-timeout wins.
		let mut sources = vec![(profile, selected)];
		if profile != "default" {
			sources.push(("default", default));
		}
		let mut overrides = Vec::new();
		for (name, table) in sources {
			let Some(list) = table.get("overrides") else {
				continue;
			};
			let Value::Array(list) = list else {
				bail!("profile.{name}.overrides is not an array");
			};
			for (i, entry) in list.iter().enumerate() {
				let at = format!("profile.{name}.overrides[{i}]");
				let entry = self::table(entry, &at)?;
				// An override that sets no bound, such as a test group's, cannot change one.
				let Some(value) = entry.get("slow-timeout") else {
					continue;
				};
				if entry.contains_key("platform") {
					bail!("{at}: a bound's platform is not supported");
				}
				let Some(Value::String(filter)) = entry.get("filter") else {
					bail!("{at}: a bound needs a filter");
				};
				let filter = Filter::parse(filter).with_context(|| at.clone())?;
				overrides.push((filter, slow_timeout(value, &at)?));
			}
		}

		Ok(Bounds {
			default: default_bound,
			silence,
			overrides,
		})
	}

	/// The bound of test `test` in package `package`.
	pub fn get(&self, package: &str, test: &str) -> &Bound {
		self.overrides
			.iter()
			.find(|(filter, _)| filter.matches(package, test))
			.map_or(&self.default, |(_, bound)| bound)
	}
}

fn table<'a>(value: &'a Value, at: &str) -> Result<&'a Table> {
	match value {
		Value::Table(table) => Ok(table),
		_ => bail!("{at} is not a table"),
	}
}

fn duration(value: &Value, at: &str) -> Result<Duration> {
	let Value::String(text) = value else {
		bail!("{at} is not a duration string");
	};
	humantime::parse_duration(text).with_context(|| format!("{at}: `{text}`"))
}

fn slow_timeout(value: &Value, at: &str) -> Result<Bound> {
	let (period, terminate_after, grace) = match value {
		Value::String(_) => (duration(value, at)?, None, GRACE),
		Value::Table(table) => {
			let mut period = None;
			let mut terminate_after = None;
			let mut grace = GRACE;
			for (key, value) in table {
				let at = format!("{at}.{key}");
				match key.as_str() {
					"period" => period = Some(duration(value, &at)?),
					"grace-period" => grace = duration(value, &at)?,
					"terminate-after" => match value {
						Value::Integer(n) if *n >= 1 => {
							terminate_after = Some(u32::try_from(*n).with_context(|| at.clone())?);
						}
						_ => bail!("{at} is not a positive integer"),
					},
					"on-timeout" => match value {
						Value::String(s) if s == "fail" => {}
						// A timed-out test that passes would need the rest of its binary to
						// go on, but the watchdog can only kill the whole run.
						_ => bail!("{at}: only \"fail\" is supported"),
					},
					_ => bail!("{at} is not a slow-timeout key"),
				}
			}
			let Some(period) = period else {
				bail!("{at} has no period");
			};
			(period, terminate_after, grace)
		}
		_ => bail!("{at} is neither a duration nor a table"),
	};
	let limit = terminate_after.map(|n| period * n);
	if let Some(limit) = limit
		&& limit < WARN_AFTER
	{
		bail!(
			"{at}: a bound of {} is under libtest's {} warning, the first the watchdog hears of a running test",
			humantime::format_duration(limit),
			humantime::format_duration(WARN_AFTER),
		);
	}
	Ok(Bound { limit, grace })
}

#[cfg(test)]
mod tests {
	use super::*;

	const RIND: &str = r#"
[profile.default]
slow-timeout = { period = "60s", terminate-after = 15 }

[[profile.default.overrides]]
filter = 'test(/rsa8192_/) | test(=rsa::key_generation_8192)'
slow-timeout = { period = "120s", terminate-after = 60 }

[profile.emulated]
slow-timeout = { period = "120s", terminate-after = 10 }

[[profile.emulated.overrides]]
filter = 'package(rind) & test(=rsa::key_generation_8192)'
slow-timeout = { period = "5m", terminate-after = 1, grace-period = "30s" }

[[profile.emulated.overrides]]
filter = 'test(=unbounded)'
slow-timeout = "60s"

[[profile.emulated.overrides]]
filter = 'kind(bench)'
threads-required = 2
"#;

	fn minutes(n: u64) -> Option<Duration> {
		Some(Duration::from_secs(n * 60))
	}

	#[test]
	fn profile_and_overrides_in_nextest_order() {
		let bounds = Bounds::parse(RIND, "default").unwrap();
		assert_eq!(bounds.silence, Duration::from_secs(15 * 60));
		assert_eq!(bounds.get("rind", "aes::gcm").limit, minutes(15));
		assert_eq!(bounds.get("rind", "rsa::tests::rsa8192_sign").limit, minutes(120));
		assert_eq!(bounds.get("rind", "unbounded").limit, minutes(15));

		let bounds = Bounds::parse(RIND, "emulated").unwrap();
		assert_eq!(bounds.silence, Duration::from_secs(20 * 60));
		assert_eq!(bounds.get("rind", "aes::gcm").limit, minutes(20));
		// The profile's own override wins over the default profile's...
		let own = bounds.get("rind", "rsa::key_generation_8192");
		assert_eq!(own.limit, minutes(5));
		assert_eq!(own.grace, Duration::from_secs(30));
		// ...only where its filter matches; the default profile's still applies.
		assert_eq!(bounds.get("rind-ring", "rsa::key_generation_8192").limit, minutes(120));
		assert_eq!(bounds.get("rind", "unbounded").limit, None);
		assert_eq!(bounds.get("rind", "aes::gcm").grace, GRACE);
	}

	#[test]
	fn a_profile_inherits_the_default_bound() {
		let config =
			"[profile.default]\nslow-timeout = { period = \"30s\", terminate-after = 2 }\n[profile.ci]\nretries = 0\n";
		assert_eq!(Bounds::parse(config, "ci").unwrap().silence, WARN_AFTER);
	}

	#[test]
	fn unusable_configs_fail_naming_the_key() {
		for (config, profile, says) in [
			("", "default", "sets no slow-timeout"),
			("", "ci", "no profile `ci`"),
			(
				"[profile.default]\nslow-timeout = \"60s\"",
				"default",
				"no terminate-after",
			),
			(
				"[profile.default]\nslow-timeout = { period = \"20s\", terminate-after = 2 }",
				"default",
				"under libtest's 1m warning",
			),
			(
				"[profile.default]\nslow-timeout = { period = \"60s\", terminate-after = 2, on-timeout = \"pass\" }",
				"default",
				"on-timeout: only \"fail\"",
			),
			(
				"[profile.default]\nslow-timeout = { period = \"60s\", terminate-after = 0 }",
				"default",
				"not a positive integer",
			),
			(
				"[profile.default]\nslow-timeout = { period = \"60s\", terminate-after = 2 }\n[[profile.default.overrides]]\nfilter = 'test(=a)'\nplatform = 'cfg(unix)'\nslow-timeout = \"2m\"",
				"default",
				"platform is not supported",
			),
			(
				"[profile.default]\nslow-timeout = { period = \"60s\", terminate-after = 2 }\n[[profile.default.overrides]]\nfilter = 'kind(test)'\nslow-timeout = \"2m\"",
				"default",
				"overrides[0]",
			),
			(
				"[profile.ci]\ninherits = \"default\"\nslow-timeout = { period = \"60s\", terminate-after = 2 }",
				"ci",
				"inherits is not supported",
			),
		] {
			let error = format!("{:#}", Bounds::parse(config, profile).unwrap_err());
			assert!(error.contains(says), "{config}: {error}");
		}
	}
}
