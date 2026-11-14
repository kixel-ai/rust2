//! The binary as rustc: plain for every crate but the facade, whose API it also collects
//! once rustc has checked it, following its re-exports into the crates it depends on.

use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

use rustc_ast as ast;
use rustc_data_structures::fx::FxHashSet;
use rustc_driver::{Callbacks, Compilation};
use rustc_hir::def::{DefKind, Res};
use rustc_hir::def_id::{DefId, LOCAL_CRATE};
use rustc_interface::interface::Compiler;
use rustc_lexer::{FrontmatterAllowed, TokenKind};
use rustc_middle::middle::resolve::Reexport;
use rustc_middle::ty::TyCtxt;
use rustc_parse::lexer::StripTokens;
use rustc_parse::parser::{AllowConstBlockItems, ForceCollect};
use rustc_span::{FileName, Span, sym};

use crate::abi::Abi;
use crate::config::{self, Config};
use crate::ir::{self, Api};
use crate::{census, collect, glue, merge};

/// Run as rustc, with `args` as cargo passed them: the wrapped rustc's path first.
pub fn run(args: Vec<String>) -> ExitCode {
	// A build script's probe of the compiler, as `rustversion` and `autocfg` make, runs the
	// rustc cargo resolved, the one the glue builds with, rather than this nightly, so the
	// check exports no item a nightly probe enables that the glue's build then lacks. Cargo
	// sets `CARGO_CFG_*` for build scripts alone, and the CLI runs cargo without them.
	if std::env::var_os("CARGO_CFG_TARGET_ARCH").is_some() {
		return match Command::new(&args[1]).args(&args[2..]).status() {
			Ok(status) => ExitCode::from(status.code().and_then(|code| u8::try_from(code).ok()).unwrap_or(1)),
			Err(err) => {
				eprintln!("rust2-ffi: running {}: {err}", args[1]);
				ExitCode::FAILURE
			}
		};
	}
	rust2_driver::rustc::run("rust2-ffi", args, |args, _| {
		Ok(Export::for_args(args)?.map(|job| Box::new(job) as rust2_driver::rustc::Job))
	})
}

/// The file in the export that holds its API, which `--baseline` reads back.
pub const API_FILE: &str = "api.json";

/// Compiles the facade, exporting its API after analysis.
struct Export {
	/// Each crate the facade builds with, as [`config::env::CRATES`] has them.
	crates: std::collections::BTreeMap<String, String>,
	config: Config,
	output: Output,
	/// The facade's package, with the features rustc compiles it with.
	facade: config::Package,
	/// The facade's workspace's patches, for the glue's workspace.
	patch: String,
	/// The other targets' APIs to merge with this one's, as [`crate::merge::Target::write`]
	/// wrote them.
	merge: Vec<PathBuf>,
	/// The directories a staged API's spans are relative to.
	roots: Vec<PathBuf>,
	/// The API an earlier export wrote, which this one's must not break.
	baseline: Option<PathBuf>,
}

/// What the job writes.
enum Output {
	/// The export, into this directory.
	Dir(PathBuf),
	/// A census, into this directory, instead of an export.
	Census(PathBuf),
	/// The target's API, into this file, for an export of several targets to merge.
	Api(PathBuf),
}

impl Export {
	/// The job for this rustc invocation, if it compiles the facade's library.
	fn for_args(args: &[String]) -> Result<Option<Self>, String> {
		let Some(path) = std::env::var_os(config::env::CONFIG) else {
			return Ok(None);
		};
		let line = std::env::var(config::env::PACKAGE).map_err(|_| format!("{} is unset", config::env::PACKAGE))?;
		let mut facade = config::Package::parse(&line)?;
		if !rust2_driver::rustc::compiles_lib(args, &facade.name) {
			return Ok(None);
		}
		let config = Config::load(path.as_ref())?;
		let output = match (
			std::env::var_os(config::env::OUT),
			std::env::var_os(config::env::CENSUS),
			std::env::var_os(config::env::API),
		) {
			(Some(out), None, None) => Output::Dir(out.into()),
			(None, Some(census), None) => Output::Census(census.into()),
			(None, None, Some(api)) => Output::Api(api.into()),
			_ => {
				return Err(format!(
					"exactly one of {}, {}, and {} must be set",
					config::env::OUT,
					config::env::CENSUS,
					config::env::API
				));
			}
		};
		let merge = std::env::var_os(config::env::MERGE).unwrap_or_default();
		let merge = std::env::split_paths(&merge)
			.filter(|p| !p.as_os_str().is_empty())
			.collect();
		let roots = std::env::var_os(config::env::ROOTS).unwrap_or_default();
		let roots = std::env::split_paths(&roots)
			.filter(|p| !p.as_os_str().is_empty())
			.collect();
		facade.features = args
			.windows(2)
			.filter(|w| w[0] == "--cfg")
			.filter_map(|w| w[1].strip_prefix("feature=\""))
			.filter_map(|f| f.strip_suffix('"'))
			.map(str::to_string)
			.collect();
		let patch = std::env::var(config::env::PATCH).map_err(|_| format!("{} is unset", config::env::PATCH))?;
		let crates = std::env::var(config::env::CRATES).map_err(|_| format!("{} is unset", config::env::CRATES))?;
		let crates: toml::Table = crates
			.parse()
			.map_err(|err| format!("{} is malformed: {err}", config::env::CRATES))?;
		let crates = (crates.into_iter())
			.map(|(lib, keys)| match keys {
				toml::Value::String(keys) => Ok((lib, keys)),
				_ => Err(format!("{} is malformed: {lib}", config::env::CRATES)),
			})
			.collect::<Result<_, String>>()?;
		let baseline = std::env::var_os(config::env::BASELINE)
			.filter(|p| !p.is_empty())
			.map(PathBuf::from);
		Ok(Some(Self {
			crates,
			config,
			output,
			facade,
			patch,
			merge,
			roots,
			baseline,
		}))
	}
}

impl Callbacks for Export {
	fn after_analysis<'tcx>(&mut self, _compiler: &Compiler, tcx: TyCtxt<'tcx>) -> Compilation {
		if tcx.dcx().has_errors().is_some() {
			return Compilation::Stop;
		}
		let facade = collect::Facade {
			root: self.facade.root(),
			package: &self.facade.name,
			// A census counts the whole API, whatever the config leaves out.
			crates: &self.crates,
			exclude: match self.output {
				Output::Census(_) => &[],
				Output::Dir(_) | Output::Api(_) => &self.config.exclude,
			},
			// It lists instead each item one names that exports.
			audit: match self.output {
				Output::Census(_) => &self.config.exclude,
				Output::Dir(_) | Output::Api(_) => &[],
			},
		};
		let collected = collect::collect(tcx, &facade);
		let report = &collected.report;
		if let Output::Census(dir) = &self.output {
			let abi = Abi::new(&collected.api, &self.facade.lib);
			// What Swift refuses needs its exclude, as the export says, so it is not exporting.
			let swift: FxHashSet<_> = crate::swift::refusals(&abi)
				.into_iter()
				.map(|(f, _)| f.shown.clone())
				.collect();
			let exporting = exporting(&collected.api, &report.audited, &swift);
			let text = census::render(&collected.api, report, &exporting, &abi.clashes, &|span| at(tcx, span));
			// A census reports what a crate uses, but a missed instance is the exporter's bug.
			for refusal in &report.refusals {
				if refusal.why.iter().any(|why| why.construct == collect::NOT_INFERRED) {
					refuse(tcx, refusal, "inference missed an instance, a bug in rust2-ffi");
				}
			}
			if let Err(err) = write(&dir.join(format!("{}.md", report.package)), &text) {
				tcx.dcx().err(format!("rust2-ffi: {err}"));
			}
			// An exclude that leaves out what would export hides that much of the API.
			for (exclude, exported) in &exporting {
				tcx.dcx().err(format!(
					"rust2-ffi: exclude `{exclude}` in the config leaves out `{}`, which exports",
					exported.join("`, `")
				));
			}
			return stop_on_errors(tcx);
		}
		for refusal in &report.refusals {
			refuse(tcx, refusal, "exclude it in the config to leave it out");
		}
		for (path, count) in &report.excluded {
			if *count == 0 && !configured_out(tcx, path) {
				tcx.dcx().err(format!(
					"rust2-ffi: exclude `{path}` in the config names nothing exported"
				));
			}
		}
		// Refusals still let the export go on to report each clash, but write nothing.
		let target = merge::Target {
			names: None,
			package: self.facade.name.clone(),
			features: self.facade.features.clone(),
			exclude: self.config.exclude.clone(),
			api: collected.api,
		};
		let written = match &self.output {
			Output::Census(_) => unreachable!("written above"),
			Output::Api(_) if tcx.dcx().has_errors().is_some() => Ok(()),
			Output::Api(path) => target.write(path, tcx.sess.source_map(), &self.roots),
			Output::Dir(out) => match self.merged(tcx, target) {
				Some(api) => self.export(tcx, api, out),
				_ => Ok(()),
			},
		};
		if let Err(err) = written {
			tcx.dcx().err(format!("rust2-ffi: {err}"));
		}
		stop_on_errors(tcx)
	}
}

/// Each exclude whose item would export without it, and that item's paths: what an
/// exclude checked, from [`collect::Report::audited`], that the census's API holds by the
/// same path, except a function `refused` names by its shown path, since a generic's
/// instances share a span but Swift may refuse only some.
fn exporting(
	api: &Api,
	audited: &[(String, Vec<String>)],
	refused: &FxHashSet<ir::Path>,
) -> Vec<(String, Vec<String>)> {
	// Every path an exported item is checked by, from the facade's root, as the walk checks it.
	let mut paths = FxHashSet::default();
	let mut add = |path: &[String]| {
		paths.insert(path[1..].join("::"));
	};
	let methods = |methods: &[ir::Function], add: &mut dyn FnMut(&[String])| {
		for f in methods.iter().filter(|f| !refused.contains(&f.shown)) {
			match f.access {
				// A field is checked by `field@` and, unless a method or constant takes its
				// name, by its path, which then names that method or constant instead.
				ir::Access::Field { shared, .. } => {
					let mut key = f.shown.clone();
					key[1].insert_str(0, "field@");
					add(&key);
					if !shared {
						add(&f.shown);
					}
				}
				_ => add(&f.shown),
			}
		}
	};
	for r in &api.records {
		add(&r.shown);
		methods(&r.methods, &mut add);
	}
	for e in &api.enums {
		add(&e.shown);
		methods(&e.methods, &mut add);
	}
	for o in &api.objects {
		add(&o.shown);
		methods(&o.methods, &mut add);
	}
	for c in &api.callbacks {
		add(&c.path);
		methods(&c.methods, &mut add);
	}
	methods(&api.functions, &mut add);
	for c in &api.constants {
		add(&c.path);
	}
	(audited.iter())
		.filter_map(|(exclude, named)| {
			let mut exported: Vec<String> = named.iter().filter(|path| paths.contains(*path)).cloned().collect();
			exported.sort();
			exported.dedup();
			(!exported.is_empty()).then(|| (exclude.clone(), exported))
		})
		.collect()
}

impl Export {
	/// This target's API merged with the other targets', or none once a difference
	/// between two of them is an error.
	fn merged(&self, tcx: TyCtxt<'_>, ours: merge::Target) -> Option<merge::Target> {
		if self.merge.is_empty() {
			return Some(ours);
		}
		let mut apis = vec![ours.api.clone()];
		let mut unmergeable = false;
		for path in &self.merge {
			let theirs = match merge::Target::read(path, tcx.sess.source_map(), &self.roots) {
				Ok(theirs) => theirs,
				Err(err) => {
					tcx.dcx().err(format!("rust2-ffi: {err}"));
					unmergeable = true;
					continue;
				}
			};
			if theirs.api.targets.len() != 1 {
				unmergeable = true;
				tcx.dcx().err(format!(
					"rust2-ffi: the API from {} lists {} targets, not one",
					path.display(),
					theirs.api.targets.len()
				));
				continue;
			}
			let target = &theirs.api.targets[0];
			let differs = [
				("package", theirs.package != ours.package),
				("features", theirs.features != ours.features),
				("config's exclude", theirs.exclude != ours.exclude),
			];
			for (what, _) in differs.iter().filter(|(_, differs)| *differs) {
				unmergeable = true;
				tcx.dcx().err(format!(
					"rust2-ffi: {target}'s API, from {}, has another {what} than this export's",
					path.display()
				));
			}
			if apis.iter().any(|api| api.targets == theirs.api.targets) {
				unmergeable = true;
				tcx.dcx().err(format!("rust2-ffi: {target} is exported twice"));
			}
			apis.push(theirs.api);
		}
		if apis.len() > 64 {
			unmergeable = true;
			tcx.dcx().err("rust2-ffi: one binding serves at most 64 targets");
		}
		if unmergeable {
			return None;
		}
		let (api, differs) = merge::merge(apis);
		for differs in &differs {
			let (mut first, mut second) = (&differs.first, &differs.second);
			// Where a staged span's file is not on this host as it was, the error is at the other.
			if first.1.is_dummy() {
				std::mem::swap(&mut first, &mut second);
			}
			let ((first, first_span), (second, second_span)) = (first, second);
			let mut err = tcx.dcx().struct_span_err(
				*first_span,
				format!(
					"rust2-ffi: {} differs between {first} and {second}, which one binding cannot serve",
					differs.what
				),
			);
			err = match second_span.is_dummy() {
				true => err.with_note(format!("{second}'s API was staged where its file is elsewhere")),
				false => err.with_span_note(*second_span, format!("on {second}, it is here")),
			};
			err.emit();
		}
		differs.is_empty().then_some(merge::Target { api, ..ours })
	}

	/// Read and validate the baseline once before comparing either compatibility layer.
	fn baseline(&self, tcx: TyCtxt<'_>, ours: &merge::Target) -> Result<Option<merge::Target>, ()> {
		let Some(path) = &self.baseline else {
			return Ok(None);
		};
		let base = match merge::Target::read(path, tcx.sess.source_map(), &self.roots) {
			Ok(base) => base,
			Err(err) => {
				tcx.dcx().err(format!("rust2-ffi: the baseline: {err}"));
				return Err(());
			}
		};
		if base.package != ours.package {
			tcx.dcx().err(format!(
				"rust2-ffi: the baseline {} is package `{}`'s API, not `{}`'s",
				path.display(),
				base.package,
				ours.package
			));
			return Err(());
		}
		Ok(Some(base))
	}

	/// The baseline's binding names, or an error if it lacks any language's. A baseline from
	/// before names were kept still has its structure checked first, so its re-export accepts
	/// no break that check would have reported.
	fn baseline_names<'b>(&self, tcx: TyCtxt<'_>, base: &'b merge::Target) -> Option<&'b crate::lock::Names> {
		let Some(names) = &base.names else {
			tcx.dcx()
				.err("rust2-ffi: the baseline lacks binding names; export it again with this rust2-ffi");
			return None;
		};
		if ["C++", "Kotlin", "Swift", "Go", "Dart", "Python"]
			.iter()
			.any(|language| !names.contains_key(*language))
		{
			tcx.dcx()
				.err("rust2-ffi: the baseline lacks a binding's name table; export it again with this rust2-ffi");
			return None;
		}
		Some(names)
	}

	/// Compare one compatibility layer against the baseline: its structure, or with `names`,
	/// the baseline's binding names and the export's, the names the generators made.
	fn locked(
		&self,
		tcx: TyCtxt<'_>,
		base: &merge::Target,
		abi: &Abi<'_>,
		names: Option<(&crate::lock::Names, &crate::lock::Names)>,
	) -> bool {
		let path = self.baseline.as_ref().expect("baseline supplied");
		let breaks = match names {
			None => crate::lock::breaks(&base.api, abi.api, &self.facade.lib),
			Some((then, now)) => crate::lock::name_breaks(then, now, &base.api, abi),
		};
		let help = format!(
			"to accept the break, export without --baseline and replace {} with the export's {API_FILE}",
			path.display()
		);
		for b in &breaks {
			let what = format!("rust2-ffi: the API breaks the baseline at {}", b.what);
			let err = match b.span.is_dummy() {
				true => tcx.dcx().struct_err(what),
				false => tcx.dcx().struct_span_err(b.span, what),
			};
			err.with_help(help.clone()).emit();
		}
		breaks.is_empty()
	}

	/// Write the export of `api` into `out`, or fail on two items that share a C name.
	fn export(&self, tcx: TyCtxt<'_>, mut target: merge::Target, out: &Path) -> Result<(), String> {
		let api = &target.api;
		let abi = Abi::new(api, &self.facade.lib);
		for (name, (first, first_span), (second, second_span)) in &abi.clashes {
			tcx.dcx()
				.struct_span_err(
					*second_span,
					format!("rust2-ffi: {second} and {first} are both `{name}` in C"),
				)
				.with_span_note(*first_span, format!("{first} is here"))
				.emit();
		}
		for (function, why) in crate::swift::refusals(&abi) {
			tcx.dcx().span_err(
				function.span,
				format!("rust2-ffi: {why}; exclude it in the config to leave it out"),
			);
		}
		let can_generate = tcx.dcx().has_errors().is_none();
		let baseline = match self.baseline(tcx, &target) {
			Ok(base) => base,
			Err(()) => return Ok(()),
		};
		let structure_kept = baseline.as_ref().is_none_or(|base| self.locked(tcx, base, &abi, None));
		// A baseline without names fails the export, but the generators still run, so their
		// own refusals show now rather than after the baseline is exported again.
		let names = baseline.as_ref().map(|base| self.baseline_names(tcx, base));
		let names_complete = names.is_none_or(|names| names.is_some());
		let baseline = baseline.as_ref().zip(names.flatten());
		if !can_generate {
			return Ok(());
		}
		let glue = glue::Crate {
			abi: &abi,
			facade: &self.facade,
			patch: &self.patch,
			crates: &self.crates,
		};
		let generated = glue.generate(out)?;
		if let Some(path) = &self.baseline {
			generated.check_baseline(path)?;
		}
		let names_kept =
			baseline.is_none_or(|(base, names)| self.locked(tcx, base, &abi, Some((names, &generated.names))));
		if !structure_kept || !names_complete || !names_kept {
			return Ok(());
		}
		target.names = Some(generated.write()?);

		target.write(&out.join(API_FILE), tcx.sess.source_map(), &self.roots)
	}
}

fn stop_on_errors(tcx: TyCtxt<'_>) -> Compilation {
	match tcx.dcx().has_errors() {
		None => Compilation::Continue,
		Some(_) => Compilation::Stop,
	}
}

/// Emits `refusal` as an error at its span, with every reason and its notes, then `advice`.
fn refuse(tcx: TyCtxt<'_>, refusal: &collect::Refusal, advice: &str) {
	let reasons: Vec<String> = refusal
		.why
		.iter()
		.map(|why| match why.detail.is_empty() {
			true => why.construct.to_string(),
			false => format!("{} `{}`", why.construct, why.detail),
		})
		.collect();
	let mut err = tcx.dcx().struct_span_err(
		refusal.span,
		format!(
			"rust2-ffi: cannot export {} `{}`: {}; {advice}",
			refusal.kind,
			refusal.path,
			reasons.join("; ")
		),
	);
	for (span, note) in refusal.why.iter().filter_map(|why| why.note.as_ref()) {
		err = err.with_span_note(*span, note.clone());
	}
	err.emit();
}

/// Where `span` starts, relative to the crate's directory when inside it.
fn at(tcx: TyCtxt<'_>, span: Span) -> String {
	let sm = tcx.sess.source_map();
	let (file, line, col, ..) = sm.span_to_location_info(span.source_callsite());
	let Some(file) = file else {
		return "no location".to_string();
	};
	let name = file
		.name
		.display(rustc_span::RemapPathScopeComponents::DIAGNOSTICS)
		.to_string();
	let dir = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_default();
	let name = match name.strip_prefix(&format!("{dir}/")) {
		Some(rel) => rel.to_string(),
		None => name,
	};
	format!("{name}:{line}:{col}")
}

pub fn write(path: &Path, text: &str) -> Result<(), String> {
	if let Some(dir) = path.parent() {
		std::fs::create_dir_all(dir).map_err(|err| format!("creating {}: {err}", dir.display()))?;
	}
	std::fs::write(path, text).map_err(|err| format!("writing {}: {err}", path.display()))
}

/// Whether the struct, enum, or union `adt` declares a field or variant `name` under a
/// `cfg` this build left it out by. rustc records only the items a `cfg` strips, not their
/// members, so this reads the declaration from its source.
fn stripped_member(tcx: TyCtxt<'_>, adt: DefId, name: &str) -> bool {
	if tcx.adt_def(adt).all_fields().any(|f| f.name.as_str() == name)
		|| tcx.adt_def(adt).variants().iter().any(|v| v.name.as_str() == name)
	{
		return false;
	}
	// The item from its name on, up to the `;` or closing brace that ends it: past it the
	// file may close a module the item is in, which the parser would refuse unmatched. Each
	// place nesting returns to the item's own level may end it, such as a const generic's
	// block before its fields, so the first that parses as an item does.
	let source_map = tcx.sess.source_map();
	let head = tcx.def_span(adt);
	let source_file = source_map.lookup_source_file(head.lo());
	let Ok(source) = source_map.span_to_snippet(head.with_hi(source_file.end_position())) else {
		return false;
	};
	let (mut depth, mut at) = (0usize, 0usize);
	let mut item = None;
	for token in rustc_lexer::tokenize(&source, FrontmatterAllowed::No) {
		at += token.len as usize;
		let end = match token.kind {
			TokenKind::OpenParen | TokenKind::OpenBrace | TokenKind::OpenBracket => {
				depth += 1;
				false
			}
			TokenKind::CloseParen | TokenKind::CloseBrace | TokenKind::CloseBracket => {
				// A delimiter the item never opened closes the module it is in.
				let Some(inner) = depth.checked_sub(1) else { break };
				depth = inner;
				depth == 0 && token.kind == TokenKind::CloseBrace
			}
			TokenKind::Semi => depth == 0,
			_ => false,
		};
		if end && let Some(parsed) = parse_item(tcx, &source[..at]) {
			item = Some(parsed);
			break;
		}
	}
	let Some(item) = item else { return false };
	// A `cfg`, or one a `cfg_attr` adds, at any depth: the attributes past its predicate.
	fn adds_cfg(items: &[ast::MetaItemInner]) -> bool {
		items.iter().skip(1).any(|item| {
			item.has_name(sym::cfg) || (item.has_name(sym::cfg_attr) && item.meta_item_list().is_some_and(adds_cfg))
		})
	}
	let configured = |attrs: &[ast::Attribute]| {
		attrs.iter().any(|attr| {
			attr.has_name(sym::cfg)
				|| (attr.has_name(sym::cfg_attr) && attr.meta_item_list().is_some_and(|items| adds_cfg(&items)))
		})
	};
	match &item.kind {
		ast::ItemKind::Struct(_, _, data) | ast::ItemKind::Union(_, _, data) => {
			(data.fields().iter()).any(|f| f.ident.is_some_and(|ident| ident.as_str() == name) && configured(&f.attrs))
		}
		ast::ItemKind::Enum(_, _, def) => {
			(def.variants.iter()).any(|v| v.ident.as_str() == name && configured(&v.attrs))
		}
		_ => false,
	}
}

/// `source` parsed as one item, or none if it is not one.
fn parse_item(tcx: TyCtxt<'_>, source: &str) -> Option<Box<ast::Item>> {
	let file = FileName::anon_source_code(source);
	let mut parser = match rustc_parse::new_parser_from_source_str(
		&tcx.sess.psess,
		file,
		source.to_string(),
		StripTokens::Nothing,
	) {
		Ok(parser) => parser,
		Err(errors) => {
			errors.into_iter().for_each(|e| e.cancel());
			return None;
		}
	};
	match parser.parse_item(ForceCollect::No, AllowConstBlockItems::DoesNotMatter) {
		Ok(item) => item,
		Err(e) => {
			e.cancel();
			None
		}
	}
}

/// Whether `path`, from the facade's root, names an item a `cfg` leaves out of this build,
/// or a field or variant of one: the config names it for the builds that have it. Its first
/// segment no module has is stripped from that module or from one the module
/// glob-re-exports.
fn configured_out(tcx: TyCtxt<'_>, path: &str) -> bool {
	let path = path.strip_prefix("field@").unwrap_or(path);
	let segments: Vec<&str> = path.split("::").collect();
	let mut module = LOCAL_CRATE.as_def_id();
	for (i, segment) in segments.iter().enumerate() {
		let name = segment.split('<').next().unwrap_or(segment);
		let children = match module.as_local() {
			Some(local) => tcx.module_children_local(local),
			None => tcx.module_children(module),
		};
		if let Some(child) = children.iter().find(|child| child.ident.as_str() == name) {
			match child.res {
				Res::Def(DefKind::Mod, def_id) => {
					module = def_id;
					continue;
				}
				Res::Def(DefKind::Struct | DefKind::Enum | DefKind::Union, def_id) if i + 2 == segments.len() => {
					return stripped_member(tcx, def_id, segments[i + 1]);
				}
				// A method, or a member past one, which no `cfg` this checks strips.
				_ => return false,
			}
		}
		// Each source of a glob the item came through, while its imports are globs: the
		// module of the next import, or else the item's own. A single re-export brings in no
		// other name of its module.
		let sources = children
			.iter()
			.flat_map(|child| {
				let chain = &child.reexport_chain;
				let globs = chain
					.iter()
					.take_while(|reexport| matches!(reexport, Reexport::Glob(_)));
				(globs.enumerate()).filter_map(move |(i, _)| match chain.get(i + 1).and_then(|next| next.id()) {
					Some(import) => tcx.opt_parent(import),
					None => child.res.opt_def_id().and_then(|def_id| tcx.opt_parent(def_id)),
				})
			})
			.chain([module]);
		let stripped = |source: DefId| {
			(tcx.stripped_cfg_items(source.krate).iter())
				.any(|item| item.parent_scope == source && item.ident.as_str() == name)
		};
		return sources.collect::<FxHashSet<_>>().into_iter().any(stripped);
	}
	false
}
