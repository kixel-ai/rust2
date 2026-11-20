//! Rustdoc carried into every binding.
//!
//! Collection writes each doc comment in a canonical Markdown form, with no Rust code
//! example, since no binding runs one, and each intra-doc link rustc resolved to an item
//! the export may hold written `[label](rust2:<path>)`, by the item's path in the IR: the
//! language links its own name for it, or keeps `label` where it has none. A link written
//! with text of its own, such as `[the abort](Self::abort)`, is `rust2-text:` instead, and
//! keeps that text. A link rustc could not resolve, or that names no item, keeps its label.
//!
//! Each generator writes the Markdown into its comments as they are, then renders every
//! link once its names are final, with [`links`].

use std::collections::HashMap;
use std::ops::Range;

use rustc_resolve::rustdoc::pulldown_cmark::{
	BrokenLink, CodeBlockKind, CowStr, Event, HeadingLevel, LinkType, Options, Parser, Tag, TagEnd,
};

use crate::ir::{Api, Constant, Function};

/// A resolved link's scheme, which shows the binding's name for the item.
const AUTO: &str = "rust2:";
/// A resolved link's scheme, which keeps its own text.
const TEXT: &str = "rust2-text:";
/// What opens a resolved link's destination, of either scheme.
pub(crate) const LINK: &str = "](rust2";

/// The options rustdoc parses a doc comment with.
fn options() -> Options {
	rustc_resolve::rustdoc::main_body_opts()
}

/// What a doc comment links to, by the namespace its disambiguator or suffix names.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum Ns {
	Type,
	Value,
	Macro,
}

/// A link's path as rustc's resolver caches it, and the namespace it names, if any: rustc's
/// `preprocess_link`, which must agree with it, since its resolutions are looked up by it.
pub(crate) fn preprocess(dest: &str) -> (String, Option<Ns>) {
	let link = dest.replace('`', "");
	let link = link.split('#').next().unwrap_or("").trim();
	let (ns, link) = match link.split_once('@') {
		Some((kind, rest)) => (disambiguator(kind), rest),
		None => (None, link),
	};
	let suffixed = |suffix: &str| link.ends_with(suffix);
	let ns = ns.or(match () {
		_ if suffixed("()") => Some(Ns::Value),
		_ if suffixed("!") || suffixed("!{}") || suffixed("![]") || suffixed("!()") => Some(Ns::Macro),
		_ => None,
	});
	let link = link.strip_suffix("()").unwrap_or(link);
	let link = link.strip_suffix("{}").unwrap_or(link);
	let link = link.strip_suffix("[]").unwrap_or(link);
	let link = match link {
		"!" => link,
		_ => link.strip_suffix('!').unwrap_or(link),
	};
	let link = link.trim();
	let path = rustc_resolve::rustdoc::strip_generics_from_path(link).map_or_else(|_| link.to_string(), String::from);
	(path, ns)
}

/// The namespace a rustdoc disambiguator such as `struct` or `fn` names.
fn disambiguator(kind: &str) -> Option<Ns> {
	match kind {
		"struct" | "enum" | "trait" | "union" | "mod" | "module" | "type" | "tyalias" | "typealias" | "prim"
		| "primitive" => Some(Ns::Type),
		"fn" | "function" | "method" | "tymethod" | "const" | "constant" | "static" | "value" | "field" | "variant" => {
			Some(Ns::Value)
		}
		"macro" | "derive" | "attr" => Some(Ns::Macro),
		_ => None,
	}
}

/// Whether a fenced code block of `info` is Rust, as rustdoc reads its tags: with none,
/// with `rust`, or with only tags rustdoc gives a Rust example, such as `no_run`.
fn rust(info: &str) -> bool {
	let mut other = false;
	for tag in info
		.split(|c: char| c == ',' || c.is_whitespace())
		.filter(|t| !t.is_empty())
	{
		match tag {
			"rust" => return true,
			"should_panic" | "no_run" | "ignore" | "test_harness" | "compile_fail" | "standalone_crate" => {}
			_ if tag.starts_with("ignore-") || tag.starts_with("edition") || tag.starts_with('{') => {}
			_ if tag.len() == 5 && tag.starts_with('E') && tag[1..].bytes().all(|b| b.is_ascii_digit()) => {}
			_ => other = true,
		}
	}
	!other
}

/// Whether a link's destination is a URL rather than a Rust path: an absolute URL keeps its
/// link, and any other, such as rustdoc's own `struct.Foo.html`, only its label.
fn url(dest: &str) -> Option<bool> {
	if dest.contains("://") || dest.starts_with("mailto:") {
		return Some(true);
	}
	(dest.starts_with('#') || dest.contains('/') || dest.ends_with(".html") || dest.contains(".html#")).then_some(false)
}

/// `docs` in the canonical form, each intra-doc link's preprocessed path and namespace
/// resolved by `resolve` to the path of the item it names, if any.
pub(crate) fn canonical(docs: &str, mut resolve: impl FnMut(&str, Option<Ns>) -> Option<String>) -> String {
	rewrite(docs, |link| {
		let (path, ns) = preprocess(link.dest);
		let path = resolve(&path, ns)?;
		Some(format!(
			"[{}]({}{path})",
			link.label,
			if link.auto { AUTO } else { TEXT }
		))
	})
}

/// `note`, a deprecation's, as plain Markdown: each link only its label, as a language's
/// deprecation message links nothing.
pub(crate) fn plain(note: &str) -> String {
	rewrite(note, |_| None)
}

/// A link collection meets.
struct Found<'a> {
	/// Its label as written, on one line.
	label: String,
	/// Its destination: a reference's, for one written by reference.
	dest: &'a str,
	/// Whether its label only spells its destination, as `` [`Foo`] `` does, so a binding
	/// shows its own name for it.
	auto: bool,
}

/// `docs` without Rust examples or reference definitions, each intra-doc link `link`
/// renders, or its label where it renders none, and then without each heading left empty.
fn rewrite(docs: &str, mut link: impl FnMut(&Found<'_>) -> Option<String>) -> String {
	// As rustc's resolver does: an undefined reference is a link to its own text.
	let mut broken = |broken: BrokenLink<'_>| Some((CowStr::from(broken.reference.to_string()), CowStr::from("")));
	let mut events = Parser::new_with_broken_link_callback(docs, options(), Some(&mut broken)).into_offset_iter();
	let mut edits: Vec<(Range<usize>, String)> = Vec::new();
	while let Some((event, range)) = events.next() {
		match event {
			Event::Start(Tag::CodeBlock(kind)) => {
				let example = match &kind {
					CodeBlockKind::Indented => true,
					CodeBlockKind::Fenced(info) => rust(info),
				};
				if example {
					edits.push((range, String::new()));
				}
			}
			Event::Start(Tag::Link {
				link_type, dest_url, ..
			}) if rustc_resolve::rustdoc::may_be_doc_link(link_type) => {
				// The label's extent, from the events inside the link.
				let mut inner: Option<Range<usize>> = None;
				let mut depth = 0;
				for (event, at) in events.by_ref() {
					match event {
						Event::End(TagEnd::Link) if depth == 0 => break,
						Event::Start(_) => depth += 1,
						Event::End(_) => depth -= 1,
						_ => {}
					}
					inner = Some(match inner {
						Some(r) => r.start.min(at.start)..r.end.max(at.end),
						None => at,
					});
				}
				let label: String = inner
					.map(|r| docs[r].replace(['\n', '\r'], " ").replace(['[', ']'], ""))
					.unwrap_or_default();
				let dest = dest_url.to_string();
				let replacement = match url(&dest) {
					Some(true) => format!("[{label}]({dest})"),
					Some(false) => label.clone(),
					None => {
						let auto = !matches!(
							link_type,
							LinkType::Inline | LinkType::Reference | LinkType::ReferenceUnknown
						) || label.replace('`', "").trim() == dest.replace('`', "").trim();
						link(&Found {
							label: label.clone(),
							dest: &dest,
							auto,
						})
						.unwrap_or(label)
					}
				};
				edits.push((range, replacement));
			}
			_ => {}
		}
	}
	for (_, def) in events.reference_definitions().iter() {
		edits.push((def.span.clone(), String::new()));
	}
	let text = drop_empty(&apply(docs, edits));
	tidy(&text)
}

/// `text` with each of `edits`' ranges replaced, ignoring one inside an earlier one.
fn apply(text: &str, mut edits: Vec<(Range<usize>, String)>) -> String {
	edits.sort_by_key(|(range, _)| range.start);
	let mut out = String::with_capacity(text.len());
	let mut at = 0;
	for (range, replacement) in edits {
		if range.start < at {
			continue;
		}
		out.push_str(&text[at..range.start]);
		out.push_str(&replacement);
		at = range.end;
	}
	out.push_str(&text[at..]);
	out
}

/// Each top-level block of `text`: its range, and its heading level if it is a heading.
fn blocks(text: &str) -> Vec<(Range<usize>, Option<HeadingLevel>)> {
	let mut blocks = Vec::new();
	let mut depth = 0usize;
	for (event, range) in Parser::new_ext(text, options()).into_offset_iter() {
		match event {
			Event::Start(tag) => {
				if depth == 0 {
					let level = match tag {
						Tag::Heading { level, .. } => Some(level),
						_ => None,
					};
					blocks.push((range, level));
				}
				depth += 1;
			}
			Event::End(_) => depth -= 1,
			_ if depth == 0 => blocks.push((range, None)),
			_ => {}
		}
	}
	blocks
}

/// `text` without each heading whose section is empty, such as an `# Examples` that held
/// only Rust examples.
fn drop_empty(text: &str) -> String {
	let mut text = text.to_string();
	loop {
		let blocks = blocks(&text);
		let empty: Vec<(Range<usize>, String)> = (blocks.iter().enumerate())
			.filter_map(|(i, (range, level))| {
				let level = (*level)?;
				let next = blocks.get(i + 1).map(|(_, next)| *next);
				match next {
					None => Some((range.clone(), String::new())),
					Some(Some(next)) if next <= level => Some((range.clone(), String::new())),
					_ => None,
				}
			})
			.collect();
		if empty.is_empty() {
			return text;
		}
		text = apply(&text, empty);
	}
}

/// `text` trimmed, with no line's trailing space and at most one blank line in a row.
fn tidy(text: &str) -> String {
	let mut out = String::new();
	let mut blank = 0;
	for line in text.trim().lines() {
		let line = line.trim_end();
		match line.is_empty() {
			true => blank += 1,
			false => {
				if blank > 0 && !out.is_empty() {
					out.push('\n');
				}
				blank = 0;
				out.push_str(line);
				out.push('\n');
			}
		}
	}
	out.trim_end().to_string()
}

/// `docs` with `note`, a paragraph, after its text and before its first heading.
pub(crate) fn note(docs: &str, note: &str) -> String {
	let at = (blocks(docs).into_iter())
		.find(|(_, level)| level.is_some())
		.map_or(docs.len(), |(range, _)| range.start);
	tidy(&format!("{}\n\n{note}\n\n{}", &docs[..at], &docs[at..]))
}

/// A function's docs, apart from its `# Errors` and `# Panics` sections, which a language
/// writes in its own form.
#[derive(Default, Debug, PartialEq)]
pub(crate) struct Sections {
	/// The rest, in order.
	pub(crate) body: String,
	/// When it fails: its `# Errors` section.
	pub(crate) errors: Option<String>,
	/// When it panics, which aborts the process: its `# Panics` section.
	pub(crate) panics: Option<String>,
}

/// `docs` in its sections.
pub(crate) fn sections(docs: &str) -> Sections {
	let blocks = blocks(docs);
	let mut out = Sections::default();
	let mut edits = Vec::new();
	for (i, (range, level)) in blocks.iter().enumerate() {
		let Some(level) = level else {
			continue;
		};
		let title = docs[range.clone()].trim_start_matches('#').trim();
		let slot = match title {
			"Errors" => &mut out.errors,
			"Panics" => &mut out.panics,
			_ => continue,
		};
		let end = (blocks[i + 1..].iter())
			.find(|(_, next)| next.is_some_and(|next| next <= *level))
			.map_or(docs.len(), |(next, _)| next.start);
		*slot = Some(tidy(&docs[range.end..end]));
		edits.push((range.start..end, String::new()));
	}
	out.body = tidy(&apply(docs, edits));
	out
}

/// What a resolved link names.
#[derive(Clone, Copy)]
pub(crate) enum Target<'a> {
	Record(usize),
	Enum(usize),
	/// An enum's variant, by its index.
	Variant(usize, usize),
	Object(usize),
	Callback(usize),
	Constant(&'a Constant),
	/// A function, or a method of its owner.
	Function(Option<Owner>, &'a Function),
	/// A record's field, by its index.
	Field(usize, usize),
}

/// The type a method belongs to.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum Owner {
	Record(usize),
	Enum(usize),
	Object(usize),
	Callback(usize),
}

/// Each item a link may name, by its path: an instance of a generic type, or a function of
/// one, by its first.
fn index<'a>(api: &'a Api) -> HashMap<String, Target<'a>> {
	let mut index = HashMap::new();
	let mut add = |path: String, target: Target<'a>| {
		index.entry(path).or_insert(target);
	};
	for (i, r) in api.records.iter().enumerate() {
		add(r.path.join("::"), Target::Record(i));
		for (n, f) in r.fields.iter().enumerate() {
			add(format!("{}::{}", r.path.join("::"), f.name), Target::Field(i, n));
		}
	}
	for (i, e) in api.enums.iter().enumerate() {
		add(e.path.join("::"), Target::Enum(i));
		for (n, v) in e.variants.iter().enumerate() {
			add(format!("{}::{}", e.path.join("::"), v.name), Target::Variant(i, n));
		}
	}
	for (i, o) in api.objects.iter().enumerate() {
		add(o.path.join("::"), Target::Object(i));
	}
	for (i, c) in api.callbacks.iter().enumerate() {
		add(c.path.join("::"), Target::Callback(i));
	}
	for c in &api.constants {
		add(c.path.join("::"), Target::Constant(c));
	}
	let owned = (api
		.records
		.iter()
		.enumerate()
		.map(|(i, r)| (Owner::Record(i), &r.methods)))
	.chain(api.enums.iter().enumerate().map(|(i, e)| (Owner::Enum(i), &e.methods)))
	.chain(
		api.objects
			.iter()
			.enumerate()
			.map(|(i, o)| (Owner::Object(i), &o.methods)),
	)
	.chain(
		api.callbacks
			.iter()
			.enumerate()
			.map(|(i, c)| (Owner::Callback(i), &c.methods)),
	)
	.flat_map(|(owner, methods)| methods.iter().map(move |f| (Some(owner), f)));
	for (owner, f) in owned.chain(api.functions.iter().map(|f| (None, f))) {
		// A getter before its setter, which shares its path.
		if !matches!(f.access, crate::ir::Access::Field { set: true, .. }) {
			add(f.path.join("::"), Target::Function(owner, f));
		}
	}
	index
}

/// A resolved link a generator renders.
pub(crate) struct Link<'a> {
	pub(crate) target: Target<'a>,
	/// Its own text, for a link written with one, else none: the binding's name shows.
	pub(crate) text: Option<&'a str>,
}

/// `out`, generated code, with each resolved link in its docs as `render` writes it, or
/// its label where `render` writes none, as for an item the binding lacks.
pub(crate) fn links(out: &str, api: &Api, mut render: impl FnMut(&Link<'_>) -> Option<String>) -> String {
	let index = index(api);
	let mut result = String::with_capacity(out.len());
	let mut rest = out;
	while let Some(at) = rest.find(LINK) {
		let (before, after) = rest.split_at(at);
		let scheme = match () {
			_ if after[2..].starts_with(AUTO) => AUTO,
			_ if after[2..].starts_with(TEXT) => TEXT,
			_ => {
				result.push_str(&rest[..at + 1]);
				rest = &rest[at + 1..];
				continue;
			}
		};
		let (Some(open), Some(close)) = (before.rfind('['), after.find(')')) else {
			result.push_str(&rest[..at + 1]);
			rest = &rest[at + 1..];
			continue;
		};
		let label = &before[open + 1..];
		let path = &after[2 + scheme.len()..close];
		result.push_str(&before[..open]);
		let rendered = index.get(path).and_then(|&target| {
			render(&Link {
				target,
				text: (scheme == TEXT).then_some(label),
			})
		});
		result.push_str(rendered.as_deref().unwrap_or(label));
		rest = &after[close + 1..];
	}
	result.push_str(rest);
	result
}

#[cfg(test)]
mod tests {
	use super::*;

	/// Resolves `Self::*` and `Error::*` under `moq::Track`, and nothing else.
	fn resolve(path: &str, _: Option<Ns>) -> Option<String> {
		match path.split_once("::") {
			Some(("Self", rest)) => Some(format!("moq::Track::{rest}")),
			Some(("Error", rest)) => Some(format!("moq::Error::{rest}")),
			_ if path == "Track" => Some("moq::Track".to_string()),
			_ => None,
		}
	}

	#[test]
	fn links_resolve() {
		let docs = "Call [`Self::abort`] or [the close](Self::close()) on [`Track`], else [`Vec`].\n\n\
			See [its docs](https://example.com) and [`Error::Timeout`][timeout], not [rustdoc](struct.Track.html).\n\n\
			[timeout]: Error::Timeout";
		assert_eq!(
			canonical(docs, resolve),
			"Call [`Self::abort`](rust2:moq::Track::abort) or [the close](rust2-text:moq::Track::close) on [`Track`](rust2:moq::Track), else `Vec`.\n\n\
			See [its docs](https://example.com) and [`Error::Timeout`](rust2:moq::Error::Timeout), not rustdoc."
		);
	}

	#[test]
	fn examples_dropped() {
		let docs = "Starts.\n\n# Examples\n\n```\nlet x = 1;\n```\n\n## Output\n\n```text\nok\n```\n\n# Panics\n\n    indented();\n\nWhen empty.";
		assert_eq!(
			canonical(docs, resolve),
			"Starts.\n\n# Examples\n\n## Output\n\n```text\nok\n```\n\n# Panics\n\nWhen empty."
		);
		assert_eq!(canonical("Hi.\n\n# Examples\n\n```no_run\nx();\n```", resolve), "Hi.");
		assert!(!rust("text") && !rust("sh,no_run") && rust("") && rust("ignore,edition2021") && rust("rust,text"));
	}

	#[test]
	fn split() {
		let docs = "Opens it.\n\n# Errors\n\nWhen [`Error::Timeout`](rust2:x).\n\n## Detail\n\nMore.\n\n# Panics\n\nIf zero.\n\n# Notes\n\nKept.";
		assert_eq!(
			sections(docs),
			Sections {
				body: "Opens it.\n\n# Notes\n\nKept.".to_string(),
				errors: Some("When [`Error::Timeout`](rust2:x).\n\n## Detail\n\nMore.".to_string()),
				panics: Some("If zero.".to_string()),
			}
		);
		assert_eq!(note("A.\n\n# Errors\n\nB.", "N."), "A.\n\nN.\n\n# Errors\n\nB.");
	}

	#[test]
	fn plain_note() {
		assert_eq!(
			plain("use [`Self::poll_next`] instead"),
			"use `Self::poll_next` instead"
		);
	}
}
