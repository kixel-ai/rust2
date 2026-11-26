//! Counts as Kixel Quest's `metrics.json`, and as a table to read.

use std::collections::BTreeMap;
use std::fmt::Write;
use std::path::Path;

use serde_json::{Map, Number, Value, json};

use crate::Counts;

/// The most bytes of `metrics.json` Kixel Quest reads from one run attempt.
const LIMIT: usize = 1 << 20;
/// The longest benchmark name Kixel Quest takes, in characters.
const NAME_LIMIT: usize = 1024;
/// The longest measure name Kixel Quest takes, in characters.
const MEASURE_LIMIT: usize = 64;
/// The unit of every count.
const UNIT: &str = "instructions";

/// The measure counts under `fingerprint` record, so a series never mixes two environments.
pub fn measure(fingerprint: &str) -> String {
	format!("{UNIT} ({fingerprint})")
}

/// A `metrics.json`: one measure, in one unit, of each sample's benchmarks.
pub struct File {
	pub measure: String,
	pub unit: &'static str,
	pub samples: Vec<Sample>,
}

/// One commit's values: the run's own, or another's, such as a paired comparison's base.
pub struct Sample {
	/// The commit's full ID, when it is not the run's.
	pub commit: Option<String>,
	/// Each benchmark's value.
	pub values: BTreeMap<String, Number>,
}

impl File {
	/// `counts` as one sample of the run's own commit, a measure per benchmark.
	pub fn counts(counts: &Counts) -> Self {
		let values = counts
			.counts
			.iter()
			.map(|(name, &ir)| (name.clone(), ir.into()))
			.collect();
		Self {
			measure: measure(&counts.fingerprint),
			unit: UNIT,
			samples: vec![Sample { commit: None, values }],
		}
	}

	/// The file's JSON, refusing what Kixel Quest would.
	pub fn json(&self) -> Result<String, String> {
		let measure = &self.measure;
		if measure.chars().count() > MEASURE_LIMIT {
			return Err(format!("the measure {measure} is longer than Kixel Quest takes"));
		}
		let mut samples = Vec::new();
		for sample in &self.samples {
			let mut metrics = Map::new();
			for (name, value) in &sample.values {
				if name.is_empty() || name.chars().count() > NAME_LIMIT || name.chars().any(char::is_control) {
					return Err(format!("Kixel Quest takes no benchmark named {name:?}"));
				}
				metrics.insert(name.clone(), json!({ measure: { "value": value } }));
			}
			if metrics.is_empty() {
				continue;
			}
			let mut sample_json = Map::new();
			if let Some(commit) = &sample.commit {
				sample_json.insert("commit".to_owned(), commit.clone().into());
			}
			sample_json.insert("metrics".to_owned(), Value::Object(metrics));
			samples.push(Value::Object(sample_json));
		}
		if samples.is_empty() {
			return Err("no benchmark to write".to_owned());
		}
		let file = json!({ "units": { measure: self.unit }, "samples": samples });
		let text = format!("{file}\n");
		if text.len() > LIMIT {
			return Err(format!(
				"metrics.json is {} bytes, over the {LIMIT} Kixel Quest reads",
				text.len()
			));
		}
		Ok(text)
	}

	/// Writes the file at `path`.
	pub fn write(&self, path: &Path) -> Result<(), String> {
		let text = self.json()?;
		if let Some(dir) = path.parent().filter(|dir| !dir.as_os_str().is_empty()) {
			std::fs::create_dir_all(dir).map_err(|e| format!("cannot create {}: {e}", dir.display()))?;
		}
		std::fs::write(path, text).map_err(|e| format!("cannot write {}: {e}", path.display()))
	}
}

/// Each benchmark's count, a line each.
pub fn table(counts: &BTreeMap<String, u64>) -> String {
	let width = counts.values().map(|ir| ir.to_string().len()).max().unwrap_or(0);
	let mut out = String::new();
	for (name, ir) in counts {
		writeln!(out, "{ir:>width$}  {name}").expect("a String takes every write");
	}
	out
}

#[cfg(test)]
mod tests {
	use super::*;

	fn counts(names: &[(&str, u64)]) -> Counts {
		Counts {
			fingerprint: "x86_64-0123456789ab".to_owned(),
			counts: names.iter().map(|(name, ir)| ((*name).to_owned(), *ir)).collect(),
			threaded: Default::default(),
		}
	}

	#[test]
	fn writes_one_sample_with_the_fingerprint_in_the_measure() {
		let text = File::counts(&counts(&[("b::two", 20), ("a::one", 10)])).json().unwrap();
		assert_eq!(
			text,
			"{\"samples\":[{\"metrics\":{\"a::one\":{\"instructions (x86_64-0123456789ab)\":{\"value\":10}},\
			 \"b::two\":{\"instructions (x86_64-0123456789ab)\":{\"value\":20}}}}],\
			 \"units\":{\"instructions (x86_64-0123456789ab)\":\"instructions\"}}\n"
		);
	}

	#[test]
	fn refuses_what_kixel_quest_would() {
		let json = |counts: &Counts| File::counts(counts).json();
		assert!(json(&counts(&[])).is_err());
		assert!(json(&counts(&[("a\nb", 1)])).is_err());
		assert!(json(&counts(&[("", 1)])).is_err());
		let long = "x".repeat(NAME_LIMIT + 1);
		assert!(json(&counts(&[(&long, 1)])).is_err());
		let names: Vec<String> = (0..20_000)
			.map(|i| format!("benches/a/very/long/path.rs::group::case[{i}]"))
			.collect();
		let many: Vec<(&str, u64)> = names.iter().map(|name| (name.as_str(), u64::MAX)).collect();
		assert!(json(&counts(&many)).unwrap_err().contains("over the"));
	}

	#[test]
	fn writes_another_commits_sample() {
		let value = |value: f64| Number::from_f64(value).unwrap();
		let base = "a".repeat(40);
		let file = File {
			measure: "time (aarch64-0123456789ab)".to_owned(),
			unit: "ns",
			samples: vec![
				Sample {
					commit: Some(base.clone()),
					values: BTreeMap::from([("a".to_owned(), value(2.5))]),
				},
				Sample {
					commit: None,
					values: BTreeMap::from([("a".to_owned(), value(2.0))]),
				},
				Sample {
					commit: Some("b".repeat(40)),
					values: BTreeMap::new(),
				},
			],
		};
		assert_eq!(
			file.json().unwrap(),
			format!(
				"{{\"samples\":[{{\"commit\":\"{base}\",\"metrics\":{{\"a\":{{\"time (aarch64-0123456789ab)\":\
				 {{\"value\":2.5}}}}}}}},\
				 {{\"metrics\":{{\"a\":{{\"time (aarch64-0123456789ab)\":{{\"value\":2.0}}}}}}}}],\
				 \"units\":{{\"time (aarch64-0123456789ab)\":\"ns\"}}}}\n"
			)
		);
	}
}
