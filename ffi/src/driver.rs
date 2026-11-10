//! The binary as rustc: plain for every crate but the facade, whose API it also collects
//! once rustc has checked it, following its re-exports into the crates it depends on.

use std::path::{Path, PathBuf};
use std::process::ExitCode;

use rustc_data_structures::fx::FxHashSet;
use rustc_driver::{Callbacks, Compilation};
use rustc_hir::def::{DefKind, Res};
use rustc_hir::def_id::{DefId, LOCAL_CRATE};
use rustc_interface::interface::Compiler;
use rustc_middle::middle::resolve::Reexport;
use rustc_middle::ty::TyCtxt;
use rustc_span::Span;

use crate::abi::Abi;
use crate::config::{self, Config};
use crate::{census, collect, glue};

/// Run as rustc, with `args` as cargo passed them: the wrapped rustc's path first.
pub fn run(args: Vec<String>) -> ExitCode {
	rust2_driver::rustc::run("rust2-ffi", args, |args, _| {
		Ok(Export::for_args(args)?.map(|job| Box::new(job) as rust2_driver::rustc::Job))
	})
}

/// Compiles the facade, exporting its API after analysis.
struct Export {
	config: Config,
	output: Output,
	/// The facade's package, with the features rustc compiles it with.
	facade: config::Package,
	/// The facade's workspace's patches, for the glue's workspace.
	patch: String,
}

/// What the job writes.
enum Output {
	/// The export, into this directory.
	Dir(PathBuf),
	/// A census, into this directory, instead of an export.
	Census(PathBuf),
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
		) {
			(Some(out), None) => Output::Dir(out.into()),
			(None, Some(census)) => Output::Census(census.into()),
			_ => {
				return Err(format!(
					"exactly one of {} and {} must be set",
					config::env::OUT,
					config::env::CENSUS
				));
			}
		};
		facade.features = args
			.windows(2)
			.filter(|w| w[0] == "--cfg")
			.filter_map(|w| w[1].strip_prefix("feature=\""))
			.filter_map(|f| f.strip_suffix('"'))
			.map(str::to_string)
			.collect();
		let patch = std::env::var(config::env::PATCH).map_err(|_| format!("{} is unset", config::env::PATCH))?;
		Ok(Some(Self {
			config,
			output,
			facade,
			patch,
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
			exclude: match self.output {
				Output::Census(_) => &[],
				Output::Dir(_) => &self.config.exclude,
			},
		};
		let collected = collect::collect(tcx, &facade);
		let report = &collected.report;
		let abi = Abi::new(&collected.api, &self.facade.lib);
		let written = match &self.output {
			Output::Census(dir) => {
				let text = census::render(&collected.api, report, &abi.clashes, &|span| at(tcx, span));
				// A census reports what a crate uses, but a missed instance is the exporter's bug.
				for refusal in &report.refusals {
					if refusal.why.iter().any(|why| why.construct == collect::NOT_INFERRED) {
						refuse(tcx, refusal, "inference missed an instance, a bug in rust2-ffi");
					}
				}
				write(&dir.join(format!("{}.md", report.package)), &text)
			}
			Output::Dir(out) => {
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
				for (name, (first, first_span), (second, second_span)) in &abi.clashes {
					tcx.dcx()
						.struct_span_err(
							*second_span,
							format!("rust2-ffi: {second} and {first} are both `{name}` in C"),
						)
						.with_span_note(*first_span, format!("{first} is here"))
						.emit();
				}
				for (span, why) in crate::swift::refusals(&abi) {
					tcx.dcx().span_err(
						span,
						format!("rust2-ffi: {why}; exclude it in the config to leave it out"),
					);
				}
				if tcx.dcx().has_errors().is_some() {
					return Compilation::Stop;
				}
				let glue = glue::Crate {
					abi: &abi,
					facade: &self.facade,
					patch: &self.patch,
					target: tcx.sess.opts.target_triple.tuple(),
				};
				glue.write(out)
			}
		};
		if let Err(err) = written {
			tcx.dcx().err(format!("rust2-ffi: {err}"));
		}
		match tcx.dcx().has_errors() {
			None => Compilation::Continue,
			Some(_) => Compilation::Stop,
		}
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

/// Whether `path`, from the facade's root, names an item a `cfg` leaves out of this build:
/// the config names it for the builds that have it. Its first segment no module has is
/// stripped from that module or from one the module glob-re-exports.
fn configured_out(tcx: TyCtxt<'_>, path: &str) -> bool {
	let path = path.strip_prefix("field@").unwrap_or(path);
	let mut module = LOCAL_CRATE.as_def_id();
	for segment in path.split("::") {
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
				// A member of an item, which no `cfg` this checks strips.
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
