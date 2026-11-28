//! Write a translation, or a census of what it refused.

use std::collections::BTreeMap;
use std::path::Path;

use rustc_middle::ty::TyCtxt;
use rustc_span::def_id::LOCAL_CRATE;

use crate::lower::Extraction;

/// The runtime every translation imports.
const RUNTIME: &str = include_str!("../lean/Rust.lean");

/// The Lean module a crate translates to: its name in upper camel case.
pub fn module(tcx: TyCtxt<'_>) -> String {
	tcx.crate_name(LOCAL_CRATE)
		.as_str()
		.split('_')
		.map(|w| {
			let mut c = w.chars();
			c.next()
				.map(|f| f.to_uppercase().chain(c).collect::<String>())
				.unwrap_or_default()
		})
		.collect()
}

/// Write `Rust.lean` and the crate's module into `out`.
pub fn write(tcx: TyCtxt<'_>, extraction: &Extraction, out: &Path) -> Result<(), String> {
	let krate = tcx.crate_name(LOCAL_CRATE);
	let checks = if tcx.sess.overflow_checks() { "on" } else { "off" };
	let mut text = format!(
		"-- Translated from the `{krate}` crate by rust2-lean, with overflow checks {checks}.\nimport Rust\n\nset_option linter.unusedVariables false\n-- Rust's wide tuples need deeper instance searches than Lean's default.\nset_option synthInstance.maxSize 1024\n\nnamespace {krate}\n\n"
	);
	for ty in &extraction.types {
		text += ty;
		text += "\n";
	}
	for f in &extraction.fns {
		text += f;
		text += "\n";
	}
	text += &format!("end {krate}\n");
	let module = module(tcx);
	for (name, contents) in [
		("Rust.lean".to_string(), RUNTIME.to_string()),
		(format!("{module}.lean"), text),
	] {
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
