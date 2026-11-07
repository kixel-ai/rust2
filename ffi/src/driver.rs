//! The binary as rustc: plain for every crate but the configured one, whose API it also
//! collects once rustc has checked it, with the API of each other export it depends on.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use rustc_driver::{Callbacks, Compilation};
use rustc_hir::def_id::{CrateNum, LOCAL_CRATE};
use rustc_interface::interface::Compiler;
use rustc_middle::ty::TyCtxt;
use rustc_span::Span;

use crate::abi::Abi;
use crate::config::{self, Config};
use crate::{census, collect, glue};

/// Run as rustc, with `args` as cargo passed them: the wrapped rustc's path first.
pub fn run(args: Vec<String>) -> ExitCode {
	rust2_driver::rustc::run("rust2-ffi", args, |args| {
		Ok(Export::for_args(args)?.map(|job| Box::new(job) as rust2_driver::rustc::Job))
	})
}

/// Compiles the configured crate, exporting its API after analysis.
struct Export {
	config: Config,
	output: Output,
	/// Each export's package: the configured package first, with the features rustc
	/// compiles it with.
	crates: Vec<config::Package>,
	/// The configured workspace's patches, for the glue's workspace.
	patch: String,
}

/// What the job writes.
enum Output {
	/// The export, into this directory.
	Dir(PathBuf),
	/// A census of each export, into this directory, instead of an export.
	Census(PathBuf),
}

impl Export {
	/// The job for this rustc invocation, if it compiles the configured library.
	fn for_args(args: &[String]) -> Result<Option<Self>, String> {
		let Some(path) = std::env::var_os(config::env::CONFIG) else {
			return Ok(None);
		};
		let config = Config::load(path.as_ref())?;
		if !rust2_driver::rustc::compiles_lib(args, &config.package) {
			return Ok(None);
		}
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
		let crates = std::env::var(config::env::CRATES).map_err(|_| format!("{} is unset", config::env::CRATES))?;
		let mut crates = config::Package::parse(&crates)?;
		let Some(top) = crates.first_mut() else {
			return Err(format!("{} lists no package", config::env::CRATES));
		};
		top.features = args
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
			crates,
			patch,
		}))
	}
}

impl Callbacks for Export {
	fn after_analysis<'tcx>(&mut self, _compiler: &Compiler, tcx: TyCtxt<'tcx>) -> Compilation {
		if tcx.dcx().has_errors().is_some() {
			return Compilation::Stop;
		}
		let Some(crates) = self.crates(tcx) else {
			return Compilation::Stop;
		};
		let collected = collect::collect(tcx, &crates);
		// Each rename, by the Rust path it names, which starts with its crate's name, after
		// a field's `field@`.
		let mut renames = BTreeMap::new();
		for (report, package) in collected.reports.iter().zip(&self.crates) {
			for (path, name) in self.items(&package.name).1 {
				let (field, path) = field(path);
				renames.insert(format!("{field}{}::{path}", report.krate), name.clone());
			}
		}
		let abi = Abi::new(&collected.api, &self.config.prefix(), &renames);
		for key in abi.unused_renames() {
			let (field, key) = field(key);
			let (krate, path) = key.split_once("::").expect("a rename starts with its crate");
			let cnum = crates
				.iter()
				.find(|c| c.root == krate)
				.expect("a rename starts with an export")
				.krate;
			if configured_out(tcx, cnum, path) {
				continue;
			}
			tcx.dcx().err(format!(
				"rust2-ffi: rename `{field}{path}` in {} names nothing exported",
				self.table(&collected, krate)
			));
		}
		let written = match &self.output {
			Output::Census(dir) => {
				let written = collected.reports.iter().enumerate().try_for_each(|(i, report)| {
					let text = census::render(&collected.api, report, i == 0, &abi.clashes, &|span| at(tcx, span));
					write(&dir.join(format!("{}.md", report.package)), &text)
				});
				// A census reports what a crate uses, but a missed instance is the exporter's bug.
				for refusal in collected.reports.iter().flat_map(|report| &report.refusals) {
					if refusal.why.iter().any(|why| why.construct == collect::NOT_INFERRED) {
						refuse(tcx, refusal, "inference missed an instance, a bug in rust2-ffi");
					}
				}
				written
			}
			Output::Dir(out) => {
				for (report, krate) in collected.reports.iter().zip(&crates) {
					let table = self.table(&collected, &report.krate);
					for refusal in &report.refusals {
						refuse(tcx, refusal, &format!("exclude it in {table} to leave it out"));
					}
					for (path, count) in &report.excluded {
						if *count == 0 && !configured_out(tcx, krate.krate, field(path).1) {
							tcx.dcx()
								.err(format!("rust2-ffi: exclude `{path}` in {table} names nothing exported"));
						}
					}
				}
				for (name, (first, first_span), (second, second_span)) in &abi.clashes {
					tcx.dcx()
						.struct_span_err(
							*second_span,
							format!("rust2-ffi: {second} and {first} are both `{name}` in C"),
						)
						.with_span_note(*first_span, format!("{first} is here"))
						.with_help(match first.starts_with("field ") || second.starts_with("field ") {
							true => "rename or exclude one in the config, a field by `field@` and its path",
							false => "rename or exclude one in the config",
						})
						.emit();
				}
				if tcx.dcx().has_errors().is_some() {
					return Compilation::Stop;
				}
				let glue = glue::Crate {
					abi: &abi,
					exports: &self.crates,
					patch: &self.patch,
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

impl Export {
	/// Each export's crate, or `None` after an error naming one that is not a dependency.
	fn crates<'a>(&'a self, tcx: TyCtxt<'_>) -> Option<Vec<collect::Crate<'a>>> {
		let mut crates = Vec::new();
		for (
			i,
			config::Package {
				name: package, krate, ..
			},
		) in self.crates.iter().enumerate()
		{
			let found: Vec<CrateNum> = match i {
				0 => vec![LOCAL_CRATE],
				_ => {
					// The artifact cargo passed for the dependency, which is the package's
					// own: another package's crate may share its name and source.
					let files = extern_files(tcx, krate);
					(tcx.crates(()).iter().copied())
						.filter(|&cnum| {
							let source = tcx.used_crate_source(cnum);
							let paths = [&source.rlib, &source.rmeta, &source.dylib];
							(paths.into_iter().flatten())
								.any(|path| canonical(path).is_some_and(|p| files.contains(&p)))
						})
						.collect()
				}
			};
			// Rust loads only the crates a crate's code names.
			let [cnum] = found[..] else {
				tcx.dcx().err(format!(
					"rust2-ffi: [crates.{package}] needs `{}` to use its dependency `{krate}`, and it uses {} crates of it",
					self.config.package,
					found.len()
				));
				return None;
			};
			crates.push(collect::Crate {
				krate: cnum,
				root: self.crates[i].root(),
				package,
				// A census counts the whole API, whatever the config leaves out.
				exclude: match self.output {
					Output::Census(_) => &[],
					Output::Dir(_) => self.items(package).0,
				},
			});
		}
		Some(crates)
	}

	/// The config's `exclude` and `rename` for `package`.
	fn items(&self, package: &str) -> (&[String], &BTreeMap<String, String>) {
		match self.config.crates.get(package) {
			Some(items) => (&items.exclude, &items.rename),
			None => (&self.config.exclude, &self.config.rename),
		}
	}

	/// Where the config says what to export from `krate`.
	fn table(&self, collected: &collect::Collected, krate: &str) -> String {
		match collected.reports.iter().position(|r| r.krate == krate) {
			Some(0) => "the config".to_string(),
			Some(i) => format!("the config's [crates.{}]", collected.reports[i].package),
			None => unreachable!("every path starts with an exported crate"),
		}
	}
}

/// The files `--extern <name>=<file>` passes, canonical.
fn extern_files(tcx: TyCtxt<'_>, name: &str) -> Vec<PathBuf> {
	let Some(entry) = tcx.sess.opts.externs.get(name) else {
		return Vec::new();
	};
	match &entry.location {
		rustc_session::config::ExternLocation::ExactPaths(paths) => {
			paths.iter().filter_map(|p| canonical(p.canonicalized())).collect()
		}
		rustc_session::config::ExternLocation::FoundInLibrarySearchDirectories => Vec::new(),
	}
}

fn canonical(path: &Path) -> Option<PathBuf> {
	std::fs::canonicalize(path).ok()
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

/// Whether `path`, from `krate`'s root, names an item a `cfg` leaves out of this build, by
/// the path of the module declaring it: the config names it for the builds that have it.
fn configured_out(tcx: TyCtxt<'_>, krate: CrateNum, path: &str) -> bool {
	tcx.stripped_cfg_items(krate).iter().any(|item| {
		let module = tcx.def_path(item.parent_scope).data;
		let names = module.iter().filter_map(|d| d.data.get_opt_name());
		let stripped = names
			.chain([item.ident.name])
			.map(|n| n.to_string())
			.collect::<Vec<_>>()
			.join("::");
		path.strip_prefix(&stripped)
			.is_some_and(|rest| rest.is_empty() || rest.starts_with("::") || rest.starts_with('<'))
	})
}

/// A config path's `field@`, if it names a field by it, and the path after it.
fn field(path: &str) -> (&str, &str) {
	match path.strip_prefix("field@") {
		Some(path) => ("field@", path),
		None => ("", path),
	}
}
