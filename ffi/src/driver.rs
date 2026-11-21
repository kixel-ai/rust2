//! The binary as rustc: plain for every crate but the facade, whose API it also collects
//! once rustc has checked it, following its re-exports into the crates it depends on.

use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

use rustc_ast as ast;
use rustc_data_structures::fx::FxHashSet;
use rustc_driver::{Callbacks, Compilation};
use rustc_interface::interface::Compiler;
use rustc_middle::ty::TyCtxt;
use rustc_parse::lexer::StripTokens;
use rustc_parse::parser::{AllowConstBlockItems, ForceCollect};
use rustc_span::{FileName, Span};

use crate::abi::Abi;
use crate::config;
use crate::ir::{self, Api};
use crate::{census, collect, glue, merge};

/// Run as rustc, with `args` as cargo passed them: the wrapped rustc's path first.
pub(crate) fn run(args: Vec<String>) -> ExitCode {
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
	rust2_driver::rustc::run("rust2-ffi", args, |args, added| {
		crate::leaf::flags(args, added);
		crate::wasm::flags(args, added)?;
		if let Some(walk) = crate::wasm::Walk::for_args(args)? {
			return Ok(Some(Box::new(walk)));
		}
		Ok(Export::for_args(args)?.map(|job| Box::new(job) as rust2_driver::rustc::Job))
	})
}

/// The file in the export that holds its API, which `--baseline` reads back.
pub(crate) const API_FILE: &str = "api.json";

/// Compiles the facade, exporting its API after analysis.
struct Export {
	/// Each crate the facade builds with, as [`config::env::CRATES`] has them.
	crates: std::collections::BTreeMap<String, String>,
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
		let Some(line) = std::env::var_os(config::env::PACKAGE) else {
			return Ok(None);
		};
		let line = (line.to_str()).ok_or_else(|| format!("{} is not UTF-8", config::env::PACKAGE))?;
		let mut facade = config::Package::parse(line)?;
		if !rust2_driver::rustc::compiles_lib(args, &facade.name) {
			return Ok(None);
		}
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
	/// Load std, unnamed, into a `no_std` facade too: the glue links it whatever the facade
	/// does, so the check that lists the system libraries a static library needs must see it.
	fn after_crate_root_parsing(&mut self, compiler: &Compiler, krate: &mut ast::Crate) -> Compilation {
		let source = "extern crate std as _;";
		let item = rustc_parse::new_parser_from_source_str(
			&compiler.sess.psess,
			FileName::anon_source_code(source),
			source.to_string(),
			StripTokens::Nothing,
		)
		.ok()
		.and_then(|mut parser| {
			parser
				.parse_item(ForceCollect::No, AllowConstBlockItems::DoesNotMatter)
				.ok()
		})
		.flatten()
		.expect("an extern crate parses");
		krate.items.insert(0, item);
		Compilation::Continue
	}

	fn after_analysis<'tcx>(&mut self, _compiler: &Compiler, tcx: TyCtxt<'tcx>) -> Compilation {
		if tcx.dcx().has_errors().is_some() {
			return Compilation::Stop;
		}
		let facade = collect::Facade {
			root: self.facade.root(),
			package: &self.facade.name,
			crates: &self.crates,
		};
		let mut collected = collect::collect(tcx, &facade);
		if let Output::Census(dir) = &self.output {
			let abi = Abi::new(&collected.api, &self.facade.lib);
			// What Swift refuses fails the export too, so the census lists it with the rest.
			let (api, swift) = swift_refused(&collected.api, &self.facade.lib);
			let report = &mut collected.report;
			report.refusals.extend(swift);
			let text = census::render(&api, report, &abi.clashes, &|span| at(tcx, span));
			// A census reports what a crate uses, but a missed instance is the exporter's bug.
			for refusal in &report.refusals {
				if refusal.why.iter().any(|why| why.construct == collect::NOT_INFERRED) {
					refuse(tcx, refusal, Some("inference missed an instance, a bug in rust2-ffi"));
				}
			}
			if let Err(err) = write(&dir.join(format!("{}.md", report.package)), &text) {
				tcx.dcx().err(format!("rust2-ffi: {err}"));
			}
			return stop_on_errors(tcx);
		}
		let report = &collected.report;
		for refusal in &report.refusals {
			refuse(tcx, refusal, None);
		}
		// Refusals still let the export go on to report each clash, but write nothing. Those
		// of the CLI's later steps, the lockfile's and the web target's walk, come once the
		// export is written, which they check.
		let target = merge::Target {
			names: None,
			package: self.facade.name.clone(),
			features: self.facade.features.clone(),
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

/// `api` without what Swift refuses, and each it refuses as a census refusal: to a fixed
/// point, since leaving out a function that throws an object bare can make that object
/// noncopyable, refusing another that throws it inside a value. Swift exports the API
/// without what takes a JNI value, as the export does.
fn swift_refused(api: &Api, lib: &str) -> (Api, Vec<collect::Refusal>) {
	let mut api = api.clone();
	let mut refused = Vec::new();
	loop {
		let abi = Abi::new(&api, lib);
		let rest = api.without_jni();
		// A function refused for each of several reasons is one refusal of them all, its
		// reasons in a row. A field's accessors are apart from a method of its name.
		let mut found: Vec<(bool, collect::Refusal)> = Vec::new();
		for r in crate::swift::refusals(&abi.without_jni(&rest)) {
			let why = collect::Why {
				construct: r.construct,
				detail: String::new(),
				note: None,
			};
			let path = r.function.shown[1..].join("::");
			let field = matches!(r.function.access, ir::Access::Field { .. });
			match found.last_mut() {
				Some((was, last)) if *was == field && last.path == path => last.why.push(why),
				_ => found.push((
					field,
					collect::Refusal {
						path,
						kind: r.kind,
						span: r.function.span,
						why: vec![why],
					},
				)),
			}
		}
		if found.is_empty() {
			return (api, refused);
		}
		let gone: FxHashSet<(bool, &str)> = found.iter().map(|(field, r)| (*field, r.path.as_str())).collect();
		let keep = |methods: &mut Vec<ir::Function>| {
			methods.retain(|f| {
				let field = matches!(f.access, ir::Access::Field { .. });
				!gone.contains(&(field, f.shown[1..].join("::").as_str()))
			})
		};
		api.records.iter_mut().for_each(|r| keep(&mut r.methods));
		api.enums.iter_mut().for_each(|e| keep(&mut e.methods));
		api.objects.iter_mut().for_each(|o| keep(&mut o.methods));
		keep(&mut api.functions);
		refused.extend(found.into_iter().map(|(_, r)| r));
	}
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
		// Swift exports the API without what takes a JNI value, which only Kotlin passes.
		let rest = api.without_jni();
		for refusal in crate::swift::refusals(&abi.without_jni(&rest)) {
			let mut err = tcx.dcx().struct_span_err(
				refusal.function.span,
				format!(
					"rust2-ffi: cannot export {} `{}`: {}",
					refusal.kind,
					refusal.function.shown[1..].join("::"),
					refusal.why
				),
			);
			if let Some(help) = crate::help::of(refusal.construct) {
				err = err.with_help(help);
			}
			err.emit();
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

/// Emits `refusal` as an error at its span, with every reason, any `advice`, its notes, and how to
/// change the facade.
fn refuse(tcx: TyCtxt<'_>, refusal: &collect::Refusal, advice: Option<&str>) {
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
			"rust2-ffi: cannot export {} `{}`: {}{}",
			refusal.kind,
			refusal.path,
			reasons.join("; "),
			advice.map_or_else(String::new, |advice| format!("; {advice}"))
		),
	);
	for (span, note) in refusal.why.iter().filter_map(|why| why.note.as_ref()) {
		err = err.with_span_note(*span, note.clone());
	}
	// One note a construct, in the order found, though several reasons may share one.
	let mut helped: Vec<&str> = Vec::new();
	for help in refusal.why.iter().filter_map(|why| crate::help::of(why.construct)) {
		if !helped.contains(&help) {
			helped.push(help);
			err = err.with_help(help);
		}
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

/// Write `text` to `path` unless it already holds it, so its mtime moves only on a change.
/// A changed file is written beside `path` and renamed over it, so a reader sees the old
/// content or the new, never neither or part, and a link at `path` goes and leaves what it
/// points to.
pub(crate) fn write(path: &Path, text: &str) -> Result<(), String> {
	if std::fs::read(path).is_ok_and(|old| old == text.as_bytes()) {
		return Ok(());
	}
	let (Some(dir), Some(name)) = (path.parent(), path.file_name()) else {
		return Err(format!("writing {}: not a file path", path.display()));
	};
	std::fs::create_dir_all(dir).map_err(|err| format!("creating {}: {err}", dir.display()))?;
	let mut temp = std::ffi::OsString::from(".");
	temp.push(name);
	temp.push(".rust2-ffi");
	let temp = dir.join(temp);
	// A link left at the temporary path goes first, so the write cannot follow it.
	match std::fs::remove_file(&temp) {
		Err(err) if err.kind() != std::io::ErrorKind::NotFound => {
			return Err(format!("removing {}: {err}", temp.display()));
		}
		_ => {}
	}
	std::fs::File::options()
		.write(true)
		.create_new(true)
		.open(&temp)
		.and_then(|mut file| std::io::Write::write_all(&mut file, text.as_bytes()))
		.map_err(|err| format!("writing {}: {err}", temp.display()))?;
	std::fs::rename(&temp, path).map_err(|err| {
		let _ = std::fs::remove_file(&temp);
		format!("writing {}: {err}", path.display())
	})
}
