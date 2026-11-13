//! The census: what the facade's `pub` API exports, and every item the export
//! refuses, ranked by why, so the next mapping to build is picked from numbers.

use std::collections::BTreeMap;
use std::fmt::Write;

use rustc_span::Span;

use crate::abi::Clash;
use crate::collect::{Refusal, Report};
use crate::ir::{Access, Api, Function};

/// The census of the facade as Markdown; `at` locates a span.
pub fn render(api: &Api, collected: &Report, clashes: &[Clash], at: &dyn Fn(Span) -> String) -> String {
	let records = &api.records;
	let enums = &api.enums;
	let objects: Vec<_> = api.objects.iter().filter(|o| !o.foreign).collect();
	let functions = api.functions.len();
	let callbacks = api.callbacks.len();
	let constants = api.constants.len();
	let refused = |kinds: &[&str]| collected.refusals.iter().filter(|r| kinds.contains(&r.kind)).count();
	// A field counts once, by its getter, though it exports a setter too.
	let items = |methods: &[Function]| {
		(methods.iter())
			.filter(|f| !matches!(f.access, Access::Field { set: true, .. }))
			.count()
	};
	let methods: usize = objects.iter().map(|o| items(&o.methods)).sum::<usize>()
		+ records.iter().map(|r| items(&r.methods)).sum::<usize>()
		+ enums.iter().map(|e| items(&e.methods)).sum::<usize>();
	let types = records.len() + enums.len() + objects.len();

	let mut out = String::new();
	let _ = writeln!(out, "# Export census of `{}`\n", collected.package);
	let _ = writeln!(
		out,
		"| Item | Exported | Refused | Share exported |\n| --- | ---: | ---: | ---: |"
	);
	let mut row = |name: &str, exported: usize, refused: usize| {
		let total = exported + refused;
		let share = match total {
			0 => "-".to_string(),
			_ => format!("{}%", exported * 100 / total),
		};
		let _ = writeln!(out, "| {name} | {exported} | {refused} | {share} |");
	};
	row("Types", types, refused(&["struct", "enum", "union", "instance"]));
	row("Functions", functions, refused(&["fn"]));
	row(
		"Methods and fields of exported types",
		methods,
		refused(&["method", "field"]),
	);
	row("Traits", callbacks, 0);
	row("Constants", constants, refused(&["const"]));
	row("Other items", 0, refused(&["use", "macro", "static", "item"]));
	let _ = writeln!(
		out,
		"\nExported types: {} records, {} enums, {} objects. A refused type's methods are not counted, nor are the {} `poll_*` methods left out for their async twins, or the {} traits left out since C cannot implement them.",
		records.len(),
		enums.len(),
		objects.len(),
		collected.twins.len(),
		collected.traits.len()
	);

	// Each item counts once per construct, however many times it hits it.
	let mut constructs: BTreeMap<&str, Vec<&Refusal>> = BTreeMap::new();
	let mut details: BTreeMap<(&str, &str), Vec<&Refusal>> = BTreeMap::new();
	let mut occurrences: BTreeMap<&str, usize> = BTreeMap::new();
	for refusal in &collected.refusals {
		let mut seen = Vec::new();
		for why in &refusal.why {
			*occurrences.entry(why.construct).or_default() += 1;
			if !seen.contains(&why.construct) {
				seen.push(why.construct);
				constructs.entry(why.construct).or_default().push(refusal);
			}
			if !why.detail.is_empty() {
				let list = details.entry((why.construct, &why.detail)).or_default();
				if !list.iter().any(|r| std::ptr::eq(*r, refusal)) {
					list.push(refusal);
				}
			}
		}
	}

	let _ = writeln!(out, "\n## Refusals by construct\n");
	let _ = writeln!(
		out,
		"Items refused for each construct; an item refused for several counts under each.\n"
	);
	let _ = writeln!(out, "| Items | Uses | Construct | First |\n| ---: | ---: | --- | --- |");
	for (construct, list) in ranked(&constructs) {
		let _ = writeln!(
			out,
			"| {} | {} | {} | {} |",
			list.len(),
			occurrences[construct],
			construct,
			at(list[0].span)
		);
	}

	let _ = writeln!(out, "\n## Refusals by type or path\n");
	let _ = writeln!(out, "| Items | Construct | Names | First |\n| ---: | --- | --- | --- |");
	for ((construct, detail), list) in ranked(&details) {
		let _ = writeln!(
			out,
			"| {} | {} | `{}` | {} |",
			list.len(),
			construct,
			detail.replace('|', "\\|"),
			at(list[0].span)
		);
	}

	let _ = writeln!(out, "\n## Refused items\n");
	let mut refusals: Vec<&Refusal> = collected.refusals.iter().collect();
	refusals.sort_by(|a, b| a.path.cmp(&b.path));
	for refusal in refusals {
		let reasons: Vec<String> = refusal
			.why
			.iter()
			.map(|why| match why.detail.is_empty() {
				true => why.construct.to_string(),
				false => format!("{} `{}`", why.construct, why.detail),
			})
			.collect();
		let _ = writeln!(
			out,
			"- {} `{}` at {}: {}",
			refusal.kind,
			refusal.path,
			at(refusal.span),
			reasons.join("; ")
		);
	}

	if !collected.refusals.is_empty() {
		let _ = writeln!(out, "\n## The config that exports what maps\n");
		let _ = writeln!(out, "```toml\nexclude = [");
		let mut paths: Vec<&str> = collected.refusals.iter().map(|r| r.path.as_str()).collect();
		paths.sort();
		paths.dedup();
		for path in paths {
			let _ = writeln!(out, "    \"{path}\",");
		}
		let _ = writeln!(out, "]\n```");
	}
	if !clashes.is_empty() {
		let _ = writeln!(out, "\n## C names claimed twice\n");
		for (name, (first, _), (second, span)) in clashes {
			let _ = writeln!(out, "- `{name}`: {first}, and {second} at {}", at(*span));
		}
	}
	if !collected.opaque.is_empty() {
		let _ = writeln!(out, "\n## Values exported as objects, since a field does not map\n");
		for (path, why) in &collected.opaque {
			let _ = writeln!(out, "- `{path}`: {} `{}`", why.construct, why.detail);
		}
	}
	if !collected.traits.is_empty() {
		let _ = writeln!(out, "\n## Traits left out, since C cannot implement them\n");
		for (path, why) in &collected.traits {
			let _ = match why.detail.is_empty() {
				true => writeln!(out, "- `{path}`: {}", why.construct),
				false => writeln!(out, "- `{path}`: {} `{}`", why.construct, why.detail),
			};
		}
	}
	if !collected.twins.is_empty() {
		let _ = writeln!(
			out,
			"\n## `poll_*` methods left out, since their async twin exports as a future\n"
		);
		for path in &collected.twins {
			let _ = writeln!(out, "- `{path}`");
		}
	}
	if !collected.excluded.is_empty() {
		let _ = writeln!(out, "\n## Excluded by the config\n");
		for (path, count) in &collected.excluded {
			let _ = writeln!(out, "- `{path}`: {count} items");
		}
	}
	// An exclude that names what exports leaves out more than the export needs. What the
	// export leaves out otherwise does not export, nor does a member of it, and a field not
	// shared with a method is refused without its `field@`.
	let left_out: Vec<&str> = (collected.refusals.iter().map(|r| r.path.as_str()))
		.chain(collected.unheld.iter().map(|(path, _)| path.as_str()))
		.chain(collected.traits.iter().map(|(path, _)| path.as_str()))
		.chain(collected.twins.iter().map(String::as_str))
		.chain(collected.hidden.iter().map(String::as_str))
		.chain(collected.aliases.iter().map(String::as_str))
		.collect();
	let under = |path: &str| {
		let bare = path.strip_prefix("field@").unwrap_or(path);
		(left_out.iter()).any(|r| {
			[path, bare]
				.iter()
				.any(|path| *path == *r || path.strip_prefix(r).is_some_and(|rest| rest.starts_with("::")))
		})
	};
	let mut exporting = String::new();
	for (exclude, named) in &collected.audited {
		let mut exported: Vec<&str> = named.iter().map(String::as_str).filter(|path| !under(path)).collect();
		exported.sort();
		exported.dedup();
		if !exported.is_empty() {
			let _ = writeln!(exporting, "- `{exclude}`: `{}`", exported.join("`, `"));
		}
	}
	if !exporting.is_empty() {
		let _ = writeln!(out, "\n## Excluded by the config, though it exports\n\n{exporting}");
	}
	if !collected.hidden.is_empty() {
		let _ = writeln!(out, "\n## `#[doc(hidden)]`, left out\n");
		for path in &collected.hidden {
			let _ = writeln!(out, "- `{path}`");
		}
	}
	if !collected.unheld.is_empty() {
		let _ = writeln!(out, "\n## What C can never hold, left out\n");
		for (path, why) in &collected.unheld {
			let _ = match why.detail.is_empty() {
				true => writeln!(out, "- `{path}`: {}", why.construct),
				false => writeln!(out, "- `{path}`: {} `{}`", why.construct, why.detail),
			};
		}
	}
	if !collected.aliases.is_empty() {
		let _ = writeln!(out, "\n## Type aliases, which export nothing of their own\n");
		for path in &collected.aliases {
			let _ = writeln!(out, "- `{path}`");
		}
	}
	out
}

/// Groups by count, most first, then by key.
fn ranked<K: Ord, V>(groups: &BTreeMap<K, Vec<V>>) -> Vec<(&K, &Vec<V>)> {
	let mut ranked: Vec<_> = groups.iter().collect();
	ranked.sort_by(|a, b| b.1.len().cmp(&a.1.len()).then_with(|| a.0.cmp(b.0)));
	ranked
}
