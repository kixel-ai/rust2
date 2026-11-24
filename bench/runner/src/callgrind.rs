//! Each benchmark's instruction count in a Callgrind profile.
//!
//! A benchmark's hooks zero Callgrind's counters and start instrumenting before it, then stop
//! and dump, naming the dump after the benchmark: a part whose `desc: Trigger:` line is
//! `Client Request: <name>` and whose `totals:` line holds its cost. CodSpeed's instrument
//! hooks also dump a `Metadata: ...` part, and Valgrind dumps once more as the program ends.

/// What a `desc: Trigger:` line names a dump that measures nothing.
const TERMINATION: &str = "Program termination";

/// The prefix of a dump a client request asked for.
const REQUEST: &str = "Client Request: ";

/// The prefix of the dump CodSpeed's hooks write to name their integration.
const METADATA: &str = "Metadata: ";

/// Each benchmark one Callgrind profile measured, by name, with its instruction count (`Ir`).
/// `what` names the profile in errors.
pub fn parse(text: &str, what: &str) -> Result<Vec<(String, u64)>, String> {
	let mut lines = text.lines();
	if lines.next() != Some("# callgrind format") {
		return Err(format!("{what} is not a Callgrind profile"));
	}
	struct Part<'a> {
		trigger: Option<&'a str>,
		events: Option<&'a str>,
		totals: Option<&'a str>,
	}
	let mut parts: Vec<Part> = Vec::new();
	// A profile may name its events once, before its first part.
	let mut events = None;
	// Only a part's header lines begin with these keys; its cost lines begin with numbers,
	// `(`, or a `key=` such as `fn=`.
	for line in lines {
		if line.starts_with("part:") {
			parts.push(Part {
				trigger: None,
				events,
				totals: None,
			});
		} else if let Some(list) = line.strip_prefix("events:") {
			events = Some(list);
			if let Some(part) = parts.last_mut() {
				part.events = events;
			}
		} else if let Some(part) = parts.last_mut() {
			if let Some(trigger) = line.strip_prefix("desc: Trigger: ") {
				part.trigger = Some(trigger);
			} else if let Some(totals) = line.strip_prefix("totals:") {
				part.totals.get_or_insert(totals);
			}
		}
	}
	if parts.is_empty() {
		return Err(format!("{what} holds no part"));
	}
	let mut measured = Vec::new();
	for part in parts {
		let Some(trigger) = part.trigger else {
			return Err(format!("{what} has a part without a trigger"));
		};
		if trigger == TERMINATION {
			continue;
		}
		let Some(name) = trigger.strip_prefix(REQUEST) else {
			return Err(format!("{what} has a dump by an unknown trigger: {trigger}"));
		};
		if name.starts_with(METADATA) {
			continue;
		}
		let Some(events) = part.events else {
			return Err(format!("{what}: {name} names no events"));
		};
		let Some(index) = events.split_whitespace().position(|event| event == "Ir") else {
			return Err(format!("{what}: {name} counts no instructions (Ir)"));
		};
		let Some(totals) = part.totals else {
			return Err(format!("{what}: {name} has no totals"));
		};
		// Callgrind drops a line's trailing zero costs.
		let ir = match totals.split_whitespace().nth(index) {
			Some(cost) => cost
				.parse::<u64>()
				.map_err(|_| format!("{what}: {name}'s totals are not counts: {totals}"))?,
			None => 0,
		};
		if ir == 0 {
			return Err(format!("{what}: {name} ran no instructions"));
		}
		measured.push((name.to_owned(), ir));
	}
	Ok(measured)
}

#[cfg(test)]
mod tests {
	use super::*;

	const HEADER: &str = "# callgrind format\nversion: 1\ncreator: callgrind-3.27.1\npid: 7\ncmd: ./bench\n";

	fn part(number: u32, trigger: &str, totals: &str) -> String {
		format!(
			"part: {number}\n\ndesc: I1 cache: \ndesc: Timerange: Basic block 0 - 9\ndesc: Trigger: {trigger}\n\n\
			 positions: line\nevents: Ir Dr\nsummary: 0\n\nob=(1) /bench\nfl=(1) ???\nfn=(1) main\n0 12 3\n\ntotals: {totals}\n"
		)
	}

	fn profile(parts: &[String]) -> String {
		format!("{HEADER}{}", parts.concat())
	}

	#[test]
	fn reads_each_benchmark_and_skips_metadata_and_termination() {
		let text = profile(&[
			part(1, "Client Request: Metadata: codspeed-rust 6.0.0", "0"),
			part(2, "Client Request: benches/varint.rs::encode[1]", "12345 3"),
			part(3, "Client Request: benches/varint.rs::decode", "6789"),
			part(4, "Program termination", "0"),
		]);
		assert_eq!(
			parse(&text, "p").unwrap(),
			[
				("benches/varint.rs::encode[1]".to_owned(), 12345),
				("benches/varint.rs::decode".to_owned(), 6789)
			]
		);
	}

	#[test]
	fn reads_events_named_before_the_first_part() {
		let text =
			format!("{HEADER}events: Dr Ir\npart: 1\ndesc: Trigger: Client Request: a\nsummary: 0\ntotals: 4 9\n");
		assert_eq!(parse(&text, "p").unwrap(), [("a".to_owned(), 9)]);
	}

	#[test]
	fn a_termination_dump_alone_measures_nothing() {
		assert_eq!(
			parse(&profile(&[part(1, "Program termination", "0")]), "p").unwrap(),
			[]
		);
	}

	#[test]
	fn refuses_an_unknown_trigger() {
		let err = parse(&profile(&[part(1, "Client Request", "5")]), "p").unwrap_err();
		assert!(err.contains("unknown trigger"), "{err}");
		let err = parse(&profile(&[part(1, "Interactive", "5")]), "p").unwrap_err();
		assert!(err.contains("unknown trigger"), "{err}");
	}

	#[test]
	fn refuses_zero_instructions_including_a_dropped_trailing_zero() {
		let err = parse(&profile(&[part(1, "Client Request: a", "0 4")]), "p").unwrap_err();
		assert!(err.contains("ran no instructions"), "{err}");
		let text = format!("{HEADER}events: Dr Ir\npart: 1\ndesc: Trigger: Client Request: a\ntotals: 4\n");
		let err = parse(&text, "p").unwrap_err();
		assert!(err.contains("ran no instructions"), "{err}");
	}

	#[test]
	fn refuses_a_benchmark_without_totals_or_ir() {
		let text = format!("{HEADER}events: Ir\npart: 1\ndesc: Trigger: Client Request: a\nsummary: 0\n");
		assert!(parse(&text, "p").unwrap_err().contains("no totals"));
		let text = format!("{HEADER}events: Dr\npart: 1\ndesc: Trigger: Client Request: a\ntotals: 4\n");
		assert!(parse(&text, "p").unwrap_err().contains("counts no instructions"));
	}

	#[test]
	fn refuses_what_is_not_a_profile() {
		assert!(parse("", "p").unwrap_err().contains("not a Callgrind profile"));
		assert!(parse(HEADER, "p").unwrap_err().contains("no part"));
		let text = format!("{HEADER}part: 1\ntotals: 4\n");
		assert!(parse(&text, "p").unwrap_err().contains("without a trigger"));
	}
}
