//! Write a translation, or a census of what it refused.

use std::collections::BTreeMap;
use std::path::Path;

use rustc_middle::ty::TyCtxt;

use crate::lower::Extraction;

/// The runtime every translation imports.
const RUNTIME: &str = include_str!("../lean/Rust.lean");

/// The Lean module a crate translates to: its name in upper camel case.
pub fn module(krate: &str) -> String {
	krate
		.split('_')
		.map(|w| {
			let mut c = w.chars();
			c.next()
				.map(|f| f.to_uppercase().chain(c).collect::<String>())
				.unwrap_or_default()
		})
		.collect()
}

/// Write `Rust.lean` and each reached crate's module into `out`.
pub fn write(tcx: TyCtxt<'_>, extraction: &Extraction, out: &Path) -> Result<(), String> {
	let checks = if tcx.sess.overflow_checks() { "on" } else { "off" };
	let mut files = vec![("Rust.lean".to_string(), RUNTIME.to_string())];
	for m in &extraction.crates {
		let name = format!("{}.lean", module(&m.krate));
		if files.iter().any(|f| f.0 == name) {
			return Err(format!(
				"the translation reaches two crates whose module is {name}, such as two versions of `{}`",
				m.krate
			));
		}
		let krate = &m.krate;
		// A dependency's arithmetic is checked where its MIR checks it, as its build had it.
		let mut text = match m.local {
			true => format!("-- Translated from the `{krate}` crate by rust2-lean, with overflow checks {checks}.\n"),
			false => format!("-- Translated from the `{krate}` crate by rust2-lean, which a translated crate calls.\n"),
		};
		text += "import Rust\n";
		for import in &m.imports {
			text += &format!("import {}\n", module(import));
		}
		text += &format!(
			"\nset_option linter.unusedVariables false\n-- Rust's wide tuples need deeper instance searches than Lean's default.\nset_option synthInstance.maxSize 1024\n\nnamespace {krate}\n\n"
		);
		for item in m.types.iter().chain(&m.fns) {
			text += item;
			text += "\n";
		}
		text += &format!("end {krate}\n");
		files.push((name, text));
	}
	for (name, contents) in files {
		let path = out.join(&name);
		std::fs::write(&path, contents).map_err(|err| format!("writing {}: {err}", path.display()))?;
	}
	Ok(())
}

/// Every refusal with its span, then how many reached functions translated.
pub fn census(tcx: TyCtxt<'_>, extraction: &Extraction) -> String {
	let sm = tcx.sess.source_map();
	let mut out = String::new();
	let mut kinds: BTreeMap<String, usize> = BTreeMap::new();
	let mut refusals: Vec<_> = extraction.refusals.iter().collect();
	refusals.sort_by_key(|r| r.span.lo());
	for refusal in refusals {
		out += &format!("{}: {}\n", sm.span_to_diagnostic_string(refusal.span), refusal.msg);
		// Group by the message with its quoted parts dropped.
		let kind = refusal
			.msg
			.split('`')
			.enumerate()
			.map(|(i, part)| if i % 2 == 1 { "_" } else { part })
			.collect::<String>();
		*kinds.entry(kind).or_default() += 1;
	}
	out += &format!(
		"\n{} of {} reached functions translated; {} refusals\n",
		extraction.outcomes.iter().filter(|o| o.1).count(),
		extraction.reached,
		extraction.refusals.len()
	);
	let mut kinds: Vec<_> = kinds.into_iter().collect();
	kinds.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
	for (kind, n) in &kinds {
		out += &format!("{n:>5}  {kind}\n");
	}
	out += "\n";
	for (path, ok) in &extraction.outcomes {
		out += &format!("{} {path}\n", if *ok { "translated" } else { "refused   " });
	}
	out
}
