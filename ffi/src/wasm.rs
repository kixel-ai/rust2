//! What a wasm32 target's host lacks, refused at export at the call that reaches it.
//!
//! A target's binding implements its module's imports with its host's APIs, and refuses at
//! `load` a module importing anything else. So once the crate is generated, the export
//! checks it for the target, as `cargo build` would compile it, and walks every function
//! the module's exports reach: through std, `dyn`, function pointers, statics, and drop
//! glue, as rustc's monomorphization collector does, but into every crate's MIR rather than
//! stopping at another crate's code, from each export and each function the module runs
//! when it loads. Each walk ends at a function the module imports, or one it links from C,
//! whose imports a link of that function alone shows, and which goes on into each Rust
//! function the C calls back. What reaches an
//! import the host lacks is refused at the call in the crate's own code nearest it, so a
//! new std needs no change here: the imports, not a list of std's paths, decide.

use std::path::{Path, PathBuf};
use std::process::Command;

use rustc_attr_ir::LangItem;
use rustc_attr_parsing::eval_config_entry;
use rustc_crate_store::NativeLib;
use rustc_data_structures::fx::{FxHashMap, FxHashSet, FxIndexMap, FxIndexSet};
use rustc_driver::{Callbacks, Compilation};
use rustc_hir::def::DefKind;
use rustc_hir::def_id::{CrateNum, DefId, DefIndex, LOCAL_CRATE};
use rustc_interface::interface::{Compiler, Config};
use rustc_middle::middle::codegen_fn_attrs::CodegenFnAttrFlags;
use rustc_middle::mir::interpret::{AllocId, GlobalAlloc};
use rustc_middle::mono::{CollectionMode, MonoItem};
use rustc_middle::ty::{self, Instance, InstanceKind, TyCtxt};
use rustc_span::{DUMMY_SP, Span};
use rustc_structures::NativeLibKind;

/// A target whose binding implements its module's imports, so the export refuses what its
/// host lacks.
pub struct Host {
	pub target: &'static str,
	/// What a refusal calls the target.
	name: &'static str,
	/// Whether the binding implements the function `name` the module imports from `module`.
	implements: fn(&str, &str) -> bool,
}

/// Each target the export knows the host of.
const HOSTS: &[Host] = &[Host {
	target: crate::web::TARGET,
	name: "the web target",
	implements: |module, name| match module {
		"wasi_snapshot_preview1" => crate::web::wasi().contains(&name),
		_ => false,
	},
}];

/// The module a binding implements the generated crate's imports from: what JavaScript
/// implements, each beside its declaration, so another crate's import from it is one the
/// host lacks, by any name.
const GLUE_MODULE: &str = "rust2";

/// The host of `target`, if the export knows it.
pub fn host(target: &str) -> Option<&'static Host> {
	HOSTS.iter().find(|host| host.target == target)
}

/// The rustc flags of every crate compiled for a target with a host: all of its MIR, which
/// `cargo check` encodes for generic and inline functions alone, with each call where the
/// source makes it, as inlining would not keep it.
const MIR: [&str; 2] = ["-Zalways-encode-mir", "-Zinline-mir=no"];

/// The std crates `-Zbuild-std` checks for a target with a host when no sysroot is set,
/// which flake.nix's sysroot holds too.
pub const STD: &str = "-Zbuild-std=core,alloc,std,panic_abort";

/// A sysroot of std checked with its MIR for each target with a host, which flake.nix
/// builds once per toolchain beside rust2-ts's, if set: absolute.
fn sysroot_dir() -> Result<Option<String>, String> {
	let dir = match std::env::var(SYSROOT) {
		Ok(dir) if !dir.is_empty() => dir,
		Err(std::env::VarError::NotUnicode(_)) => return Err(format!("{SYSROOT} is not UTF-8")),
		_ => return Ok(None),
	};
	let dir = std::path::absolute(&dir)
		.map_err(|err| format!("resolving {SYSROOT}: {err}"))?
		.into_os_string()
		.into_string()
		.expect("a UTF-8 path resolves to a UTF-8 path");
	Ok(Some(dir))
}

/// The [`sysroot_dir`] a crate for `target`, which has a host, compiles with, if set, or
/// an error if it holds no std for `target`. Only a target with a host reads it, so a
/// native export runs under whatever sysroot is set.
pub fn sysroot(target: &str) -> Result<Option<String>, String> {
	let Some(dir) = sysroot_dir()? else {
		return Ok(None);
	};
	let lib = Path::new(&dir).join("lib/rustlib").join(target).join("lib");
	if !lib.is_dir() {
		return Err(format!(
			"{SYSROOT} is {dir}, which holds no std for {target} in {}",
			lib.display()
		));
	}
	Ok(Some(dir))
}

/// The variable naming [`sysroot`]: rust2-ts's, whose sysroot holds these targets' std too.
const SYSROOT: &str = "RUST2_TS_SYSROOT";

/// Add to rustc's `args` what a crate for a target with a host compiles with: its MIR, and
/// std's from [`sysroot`]. Cargo's query of the compiler, whose answer it hashes into every
/// unit's fingerprint, names the flags and the sysroot, so a target directory keeps
/// nothing compiled without them.
pub fn flags(args: &[String], added: &mut Vec<String>) -> Result<(), String> {
	if args[1..] == ["-vV"] {
		let sysroot = sysroot_dir()?;
		println!("rust2-ffi-mir: {} {}", MIR.join(" "), sysroot.as_deref().unwrap_or(STD));
	}
	// The target's crates only: build scripts and proc macros run on the host, with its std.
	let target = args.windows(2).find(|w| w[0] == "--target" && host(&w[1]).is_some());
	if let Some([_, target]) = target {
		added.extend(MIR.map(str::to_string));
		if let Some(sysroot) = sysroot(target)? {
			added.extend(["--sysroot".to_string(), sysroot]);
		}
	}
	Ok(())
}

/// What the CLI hands the wrapper for the check of the generated crate: its package, and
/// the crates of the exported workspace's own packages, which a refusal points into.
pub mod env {
	/// The generated crate's package, whose library the wrapper walks.
	pub const PACKAGE: &str = "RUST2_FFI_WALK";
	/// The library crate names of every package of the export's own source, not a
	/// registry's or git's, comma-separated.
	pub const OWN: &str = "RUST2_FFI_OWN";
}

/// The walk of the generated crate's exports, after rustc checks it.
pub struct Walk {
	host: &'static Host,
	own: FxHashSet<String>,
}

impl Walk {
	/// The job for this rustc invocation, if it compiles the generated crate for a target
	/// with a host.
	pub fn for_args(args: &[String]) -> Result<Option<Self>, String> {
		let Ok(package) = std::env::var(env::PACKAGE) else {
			return Ok(None);
		};
		// Its library, which builds the module and a static library, not its build script.
		let ours = std::env::var("CARGO_PKG_NAME").is_ok_and(|name| name == package);
		let lib = (args.windows(2)).any(|w| w[0] == "--crate-type" && matches!(w[1].as_str(), "cdylib" | "staticlib"));
		if !ours || !lib {
			return Ok(None);
		}
		let target = args.windows(2).find(|w| w[0] == "--target").map(|w| w[1].as_str());
		let Some(host) = target.and_then(host) else {
			return Err(format!(
				"{package} is checked for {target:?}, whose host the export does not know"
			));
		};
		let own = std::env::var(env::OWN).map_err(|_| format!("{} is unset", env::OWN))?;
		Ok(Some(Self {
			host,
			own: own.split(',').filter(|s| !s.is_empty()).map(str::to_string).collect(),
		}))
	}
}

impl Callbacks for Walk {
	fn config(&mut self, config: &mut Config) {
		// The collector then follows a call into any crate, and to a foreign function,
		// rather than stopping where another crate's code would be linked.
		config.override_queries = Some(|_, providers| {
			providers.hooks.should_codegen_locally = |_, _| true;
		});
	}

	fn after_analysis<'tcx>(&mut self, _compiler: &Compiler, tcx: TyCtxt<'tcx>) -> Compilation {
		if tcx.dcx().has_errors().is_none() {
			let own = |krate: CrateNum| krate != LOCAL_CRATE && self.own.contains(tcx.crate_name(krate).as_str());
			let graph = Graph::walk(tcx, self.host);
			let lacks = Lacks::find(&graph, self.host);
			report(tcx, &graph, &lacks, &own, self.host);
		}
		match tcx.dcx().has_errors() {
			None => Compilation::Continue,
			Some(_) => Compilation::Stop,
		}
	}
}

/// A function or static the module holds.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Node<'tcx> {
	Fn(Instance<'tcx>),
	Static(DefId),
}

impl<'tcx> Node<'tcx> {
	fn def_id(self) -> DefId {
		match self {
			Node::Fn(instance) => instance.def_id(),
			Node::Static(def_id) => def_id,
		}
	}

	/// The Rust function or static `def_id` defines under a symbol of its own.
	fn defined(tcx: TyCtxt<'tcx>, def_id: DefId) -> Self {
		match tcx.def_kind(def_id) {
			DefKind::Static { .. } => Node::Static(def_id),
			_ => Node::Fn(Instance::mono(tcx, def_id)),
		}
	}
}

/// Where a walk ends: a function the module imports, or one it links from C.
enum Leaf {
	Import {
		module: String,
		name: String,
	},
	C(String),
	/// A browser API wasm-bindgen would import, which on a WASI target it compiles to a
	/// function that panics instead.
	Bindgen(String),
}

/// Every function and static the generated crate's exports reach, and what each uses.
struct Graph<'tcx> {
	nodes: FxIndexSet<Node<'tcx>>,
	/// What each node uses, by index, and where.
	edges: Vec<Vec<(usize, Span)>>,
	/// The export each node was first reached from.
	origin: Vec<usize>,
	/// The nodes the walk ends at.
	leaves: FxHashMap<usize, Leaf>,
	/// Each C leaf's link, or none for a symbol no library defines.
	linked: FxHashMap<usize, Option<Linked>>,
	/// The statics the module runs a function of when it loads, which it holds whether or not
	/// an export reaches them.
	inits: FxIndexSet<usize>,
	/// What another crate exports from the module, which it holds whether or not the
	/// generated crate's exports reach it, though no binding calls it.
	kept: FxIndexSet<usize>,
}

/// What a C function's link shows: what it imports from the host, and what it calls of the
/// module's own code, each by symbol, which once walked is what no Rust crate defines
/// either, failing the module's link.
struct Linked {
	imports: Vec<Import>,
	undefined: Vec<String>,
}

impl<'tcx> Graph<'tcx> {
	fn walk(tcx: TyCtxt<'tcx>, host: &Host) -> Self {
		let mut graph = Graph {
			nodes: FxIndexSet::default(),
			edges: Vec::new(),
			origin: Vec::new(),
			leaves: FxHashMap::default(),
			linked: FxHashMap::default(),
			inits: FxIndexSet::default(),
			kept: FxIndexSet::default(),
		};
		whole_archives(tcx, host);
		// The module's exports: each function the crate gives a symbol of its own.
		for def_id in tcx.hir_crate_items(()).definitions() {
			let def_id = def_id.to_def_id();
			if matches!(tcx.def_kind(def_id), DefKind::Fn | DefKind::AssocFn)
				&& !tcx.generics_of(def_id).requires_monomorphization(tcx)
				&& tcx.codegen_fn_attrs(def_id).contains_extern_indicator()
			{
				let at = graph.add(Node::Fn(Instance::mono(tcx, def_id)), None);
				graph.origin[at] = at;
			}
		}
		let symbols = Symbols::new(tcx);
		for &def_id in &symbols.inits {
			let at = graph.add(Node::Static(def_id), None);
			graph.origin[at] = at;
			graph.inits.insert(at);
		}
		// A static the module runs when it loads is called, whatever else exports it.
		for &def_id in &symbols.kept {
			let at = graph.add(Node::defined(tcx, def_id), None);
			if !graph.inits.contains(&at) {
				graph.kept.insert(at);
			}
		}
		let mut linker = None;
		// wasm-bindgen names each import's stand-in after the import: `__wbg_<name>_<hash>`,
		// with 16 hex digits of hash, for a browser API its macro declares in any crate, and
		// `__wbindgen_<name>` for its own, in its own crate. A stand-in has no symbol of its
		// own, unlike its runtime functions the module exports to JavaScript, such as
		// `__wbindgen_free`.
		let bindgen = (tcx.crates(()).iter()).find(|&&krate| tcx.crate_name(krate).as_str() == "wasm_bindgen");
		let stub = |def_id: DefId| {
			let bindgen = *bindgen?;
			let name = tcx.item_name(def_id);
			let name = name.as_str();
			let generated = name
				.strip_prefix("__wbg_")
				.and_then(|rest| rest.rsplit_once('_'))
				.is_some_and(|(_, hash)| hash.len() == 16 && hash.bytes().all(|b| b.is_ascii_hexdigit()));
			let own = def_id.krate == bindgen && name.starts_with("__wbindgen_");
			((generated || own) && !tcx.codegen_fn_attrs(def_id).contains_extern_indicator()).then(|| name.to_string())
		};
		let limit = tcx.recursion_limit();
		// Depth first, as rustc's collector walks, counting the instances of each function on
		// the way down: one within instances of itself past the limit, as of a function
		// calling itself with an ever larger type, is refused as the collector refuses it,
		// rather than walked without end.
		enum Step {
			Enter(usize, Span),
			Exit(DefId),
		}
		let mut stack: Vec<Step> = (0..graph.nodes.len())
			.rev()
			.map(|at| Step::Enter(at, DUMMY_SP))
			.collect();
		let mut entered = FxHashSet::default();
		let mut depths: FxHashMap<DefId, usize> = FxHashMap::default();
		while let Some(step) = stack.pop() {
			let (at, span) = match step {
				Step::Enter(at, span) => (at, span),
				Step::Exit(def_id) => {
					*depths.get_mut(&def_id).expect("entered") -= 1;
					continue;
				}
			};
			if !entered.insert(at) {
				continue;
			}
			let node = graph.nodes[at];
			let def_id = node.def_id();
			let depth = depths.entry(def_id).or_default();
			// Drop glue nests tightly, and rustc gives it more room.
			let deep = match node {
				Node::Fn(instance) if tcx.is_lang_item(instance.def_id(), LangItem::DropGlue) => *depth / 4,
				_ => *depth,
			};
			if !limit.value_within_limit(deep) {
				let span = if span.is_dummy() { tcx.def_span(def_id) } else { span };
				tcx.dcx().span_fatal(
					span,
					format!(
						"rust2-ffi: reached the recursion limit while instantiating `{}`",
						tcx.def_path_str(def_id)
					),
				);
			}
			*depth += 1;
			stack.push(Step::Exit(def_id));
			'expand: {
				if tcx.is_foreign_item(def_id) {
					match symbols.resolve(tcx, node) {
						Resolved::Rust(def_id) => {
							graph.add(Node::defined(tcx, def_id), Some((at, DUMMY_SP)));
						}
						Resolved::Leaf(leaf) => {
							if let Leaf::C(name) = &leaf {
								graph.link(tcx, host, &symbols, &mut linker, at, name);
							}
							graph.leaves.insert(at, leaf);
						}
						Resolved::Nothing => {}
					}
					break 'expand;
				}
				match node {
					Node::Fn(instance) => {
						if let InstanceKind::Item(def_id) = instance.def
							&& matches!(tcx.def_kind(def_id), DefKind::Fn)
							&& let Some(name) = stub(def_id)
						{
							graph.leaves.insert(at, Leaf::Bindgen(name));
							break 'expand;
						}
						if let InstanceKind::Item(def_id) = instance.def {
							// A constructor's body only builds its value.
							if matches!(tcx.def_kind(def_id), DefKind::Ctor(..)) {
								break 'expand;
							}
							if !tcx.is_mir_available(def_id) {
								tcx.dcx().span_err(
									tcx.def_span(def_id),
									format!(
										"rust2-ffi: `{}` of crate `{}` has no MIR to follow, which every crate a target with a host compiles with; this is a bug in rust2-ffi",
										tcx.def_path_str(def_id),
										tcx.crate_name(def_id.krate)
									),
								);
								break 'expand;
							}
						}
						let Ok((used, _)) = tcx.items_of_instance((instance, CollectionMode::UsedItems)) else {
							tcx.dcx().span_err(
								tcx.def_span(def_id),
								format!("rust2-ffi: `{instance}` does not normalize; this is a bug in rust2-ffi"),
							);
							break 'expand;
						};
						for item in used {
							let to = match item.node {
								MonoItem::Fn(instance) => Node::Fn(instance),
								MonoItem::Static(def_id) => Node::Static(def_id),
								MonoItem::GlobalAsm(_) => continue,
							};
							graph.add(to, Some((at, item.span)));
						}
					}
					Node::Static(def_id) => {
						// What its value points to: functions, vtables' methods, and other statics.
						if let Ok(alloc) = tcx.eval_static_initializer(def_id) {
							let mut seen = FxHashSet::default();
							for &prov in alloc.inner().provenance().ptrs().values() {
								graph.alloc(tcx, prov.alloc_id(), at, &mut seen);
							}
						}
					}
				}
			}
			for &(to, span) in graph.edges[at].iter().rev() {
				if !entered.contains(&to) {
					stack.push(Step::Enter(to, span));
				}
			}
		}
		graph
	}

	/// The node for `node`, added if new and used by `from` at a span if given.
	fn add(&mut self, node: Node<'tcx>, from: Option<(usize, Span)>) -> usize {
		let (at, new) = self.nodes.insert_full(node);
		if new {
			self.edges.push(Vec::new());
			self.origin.push(from.map_or(at, |(from, _)| self.origin[from]));
		}
		if let Some((from, span)) = from {
			self.edges[from].push((at, span));
		}
		at
	}

	/// What the allocation `id` points to, used by `from`, as the collector scans a static.
	fn alloc(&mut self, tcx: TyCtxt<'tcx>, id: AllocId, from: usize, seen: &mut FxHashSet<AllocId>) {
		if !seen.insert(id) {
			return;
		}
		match tcx.global_alloc(id) {
			GlobalAlloc::Static(def_id) => {
				self.add(Node::Static(def_id), Some((from, DUMMY_SP)));
			}
			GlobalAlloc::Memory(alloc) => {
				for &prov in alloc.inner().provenance().ptrs().values() {
					self.alloc(tcx, prov.alloc_id(), from, seen);
				}
			}
			GlobalAlloc::Function { instance } => {
				self.add(Node::Fn(instance), Some((from, DUMMY_SP)));
			}
			GlobalAlloc::VTable(ty, dyn_ty) => {
				let principal =
					(dyn_ty.principal()).map(|principal| tcx.instantiate_bound_regions_with_erased(principal));
				self.alloc(tcx, tcx.vtable_allocation((ty, principal)), from, seen);
			}
			GlobalAlloc::TypeId { .. } => {}
		}
	}

	/// Link the C leaf `at`, named `name`, and walk on into each Rust function or static it
	/// calls back, as a C library may call what a Rust crate defines for it.
	fn link(
		&mut self,
		tcx: TyCtxt<'tcx>,
		host: &Host,
		symbols: &Symbols,
		linker: &mut Option<Option<Linker>>,
		at: usize,
		name: &str,
	) {
		let linker = linker.get_or_insert_with(|| {
			Linker::new(tcx, host)
				.inspect_err(|err| {
					tcx.dcx().err(format!("rust2-ffi: {err}"));
				})
				.ok()
		});
		let Some(linker) = linker else {
			return;
		};
		match linker.imports(name) {
			Ok(None) => {
				self.linked.insert(at, None);
			}
			Ok(Some(mut linked)) => {
				linked.undefined.retain(|need| match symbols.defined.get(need) {
					Some(&def_id) => {
						self.add(Node::defined(tcx, def_id), Some((at, DUMMY_SP)));
						false
					}
					None => true,
				});
				self.linked.insert(at, Some(linked));
			}
			Err(err) => {
				tcx.dcx().err(format!("rust2-ffi: {err}"));
			}
		}
	}
}

/// What a foreign item's symbol is.
enum Resolved {
	/// A Rust function or static some crate defines under it.
	Rust(DefId),
	Leaf(Leaf),
	/// One that uses nothing the host lacks: the allocator's marker static the link makes,
	/// or a function the generated crate imports, which its binding implements.
	Nothing,
}

/// The Rust items each crate defines under a symbol of their own, which a foreign item of
/// another crate, or a C library, may name; those of them other crates than the generated
/// one export from the module; and the statics the module runs a function of when it loads.
struct Symbols {
	defined: FxHashMap<String, DefId>,
	kept: Vec<DefId>,
	inits: Vec<DefId>,
}

impl Symbols {
	/// Read from every definition of every crate: a crate `cargo check` compiled records no
	/// symbols it exports, which only a crate compiled to code computes.
	fn new(tcx: TyCtxt<'_>) -> Self {
		let (mut defined, mut kept, mut inits) = (FxHashMap::default(), Vec::new(), Vec::new());
		let cstore = rustc_metadata::creader::CStore::from_tcx(tcx);
		// A proc macro links nothing into the module, and its metadata holds its macros alone.
		let macros: FxHashSet<CrateNum> = cstore.all_proc_macro_def_ids(tcx).map(|def_id| def_id.krate).collect();
		for krate in std::iter::once(LOCAL_CRATE).chain(tcx.crates(()).iter().copied()) {
			let definitions: Vec<DefId> = match krate {
				LOCAL_CRATE => tcx.hir_crate_items(()).definitions().map(|d| d.to_def_id()).collect(),
				_ if macros.contains(&krate) => continue,
				_ => (0..cstore.num_def_ids_untracked(krate))
					.map(|index| DefId {
						krate,
						index: DefIndex::from_usize(index),
					})
					.collect(),
			};
			for def_id in definitions {
				let kind = tcx.def_kind(def_id);
				if !matches!(kind, DefKind::Fn | DefKind::AssocFn | DefKind::Static { .. })
					|| tcx.is_foreign_item(def_id)
					|| tcx.generics_of(def_id).requires_monomorphization(tcx)
				{
					continue;
				}
				let attrs = tcx.codegen_fn_attrs(def_id);
				if attrs.contains_extern_indicator() {
					defined.insert(tcx.symbol_name(Instance::mono(tcx, def_id)).name.to_string(), def_id);
					// What another crate exports from the module as C's, as a cdylib does
					// each such item of every crate but the compiler's builtins, unless std
					// keeps it to itself.
					if krate != LOCAL_CRATE
						&& !tcx.is_compiler_builtins(krate)
						&& !attrs.flags.contains(CodegenFnAttrFlags::RUSTC_STD_INTERNAL_SYMBOL)
					{
						kept.push(def_id);
					}
				}
				// An implementation of an externally implementable item, under the item's name.
				for &(foreign, ..) in &attrs.foreign_item_symbol_aliases {
					defined.insert(tcx.symbol_name(Instance::mono(tcx, foreign)).name.to_string(), def_id);
				}
				// A static the link keeps in `.init_array`, each of whose functions the
				// module's start calls.
				if matches!(kind, DefKind::Static { .. })
					&& attrs
						.flags
						.intersects(CodegenFnAttrFlags::USED_COMPILER | CodegenFnAttrFlags::USED_LINKER)
					&& attrs
						.link_section
						.is_some_and(|section| section.as_str().starts_with(".init_array"))
				{
					inits.push(def_id);
				}
			}
		}
		Self { defined, kept, inits }
	}

	fn resolve<'tcx>(&self, tcx: TyCtxt<'tcx>, node: Node<'tcx>) -> Resolved {
		let def_id = node.def_id();
		if let Some(module) = tcx.wasm_import_module_map(def_id.krate).get(&def_id) {
			if module == GLUE_MODULE && def_id.krate == LOCAL_CRATE {
				return Resolved::Nothing;
			}
			// Imported by its link name, under a symbol rustc mangles apart from other modules'.
			let name = tcx
				.codegen_fn_attrs(def_id)
				.symbol_name
				.unwrap_or_else(|| tcx.item_name(def_id));
			return Resolved::Leaf(Leaf::Import {
				module: module.clone(),
				name: name.to_string(),
			});
		}
		let name = tcx.symbol_name(Instance::mono(tcx, def_id)).name.to_string();
		if let Some(&def_id) = self.defined.get(&name) {
			return Resolved::Rust(def_id);
		}
		// The allocator shim the link makes: each `__rust_*` function std's default calls,
		// when no `#[global_allocator]` defines it, and the marker it reads.
		use rustc_ast::expand::allocator::{
			ALLOC_ERROR_HANDLER, ALLOCATOR_METHODS, NO_ALLOC_SHIM_IS_UNSTABLE, default_fn_name, global_fn_name,
		};
		let mangle = |name: &str| rustc_symbol_mangling::mangle_internal_symbol(tcx, name);
		if name == mangle(NO_ALLOC_SHIM_IS_UNSTABLE) {
			return Resolved::Nothing;
		}
		let methods = ALLOCATOR_METHODS.iter().map(|m| m.name).chain([ALLOC_ERROR_HANDLER]);
		for method in methods {
			if name == mangle(&global_fn_name(method))
				&& let Some(&def_id) = self.defined.get(&mangle(&default_fn_name(method)))
			{
				return Resolved::Rust(def_id);
			}
		}
		Resolved::Leaf(Leaf::C(name))
	}
}

/// What each leaf needs that the module cannot have.
struct Lacks {
	/// Each leaf's, empty for one the host has all of.
	by_leaf: FxHashMap<usize, Vec<Lack>>,
}

/// What the module cannot have.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Lack {
	/// An import the host lacks, as `module.name`.
	Import(String),
	/// A C function or static no library the module links defines, which fails its link.
	Undefined(String),
	/// A browser API wasm-bindgen panics on rather than import, by its stand-in's name.
	Bindgen(String),
}

impl Lacks {
	fn find(graph: &Graph<'_>, host: &Host) -> Self {
		let import = |module: &str, name: &str| {
			(!(host.implements)(module, name)).then(|| Lack::Import(format!("{module}.{name}")))
		};
		let mut by_leaf = FxHashMap::default();
		for (&at, leaf) in &graph.leaves {
			let lacks = match leaf {
				Leaf::Import { module, name } => import(module, name).into_iter().collect(),
				Leaf::C(name) => match graph.linked.get(&at) {
					Some(Some(linked)) => (linked.imports.iter())
						.filter_map(|(module, name)| import(module, name))
						.chain(linked.undefined.iter().cloned().map(Lack::Undefined))
						.collect(),
					Some(None) => vec![Lack::Undefined(name.clone())],
					// Its link failed, which is refused already.
					None => Vec::new(),
				},
				Leaf::Bindgen(name) => vec![Lack::Bindgen(name.clone())],
			};
			by_leaf.insert(at, lacks);
		}
		Self { by_leaf }
	}
}

/// A link of one C function alone, as the target's linker would make the module, to see
/// what it imports: rustc's own `rust-lld`, over the target's C library and each static
/// library a crate links.
struct Linker {
	lld: PathBuf,
	libs: Vec<PathBuf>,
	dir: PathBuf,
}

impl Linker {
	fn new(tcx: TyCtxt<'_>, host: &Host) -> Result<Self, String> {
		// The toolchain's, whatever sysroot the MIR came from.
		let sysroot = &tcx.sess.opts.sysroot.default;
		let lld = rustc_session::filesearch::make_target_bin_path(sysroot, rustc_session::config::host_tuple())
			.join("rust-lld");
		let own = sysroot.join("lib/rustlib").join(host.target).join("lib/self-contained");
		let libc = own.join("libc.a");
		let mut libs = Vec::new();
		let search: Vec<PathBuf> = (tcx.sess.target_filesearch())
			.search_paths(rustc_session::search_paths::PathKind::Native)
			.map(|path| path.dir.to_path_buf())
			.chain([own])
			.collect();
		// In the link's order, the crate's own first and the C library last, so a library
		// defining one of libc's symbols defines it here as it does in the module.
		for krate in std::iter::once(LOCAL_CRATE).chain(tcx.postorder_cnums(()).iter().rev().copied()) {
			for lib in c_libraries(tcx, krate) {
				let name = lib.name;
				// `+verbatim` names the archive's file as it is.
				let file = match lib.verbatim {
					Some(true) => name.to_string(),
					_ => format!("lib{name}.a"),
				};
				let Some(found) = search.iter().map(|dir| dir.join(&file)).find(|path| path.is_file()) else {
					return Err(format!(
						"`{}` links the C library `{name}`, which is in no directory the check searches",
						tcx.crate_name(krate)
					));
				};
				if found != libc && !libs.contains(&found) {
					libs.push(found);
				}
			}
		}
		libs.push(libc);
		let dir = tcx.sess.io.temps_dir.clone().or_else(|| tcx.sess.io.output_dir.clone());
		let dir = dir.ok_or("the check has no output directory to link in")?;
		Ok(Self { lld, libs, dir })
	}

	/// What the module imports for the C function or static `name`, and what it calls of the
	/// module's own code, or none if no library defines it.
	fn imports(&self, name: &str) -> Result<Option<Linked>, String> {
		let out = self.dir.join(format!("rust2-ffi-{}-{name}.wasm", std::process::id()));
		// What the C library needs of the module's own code, such as compiler-builtins' or a
		// crate's callback, it leaves undefined: a function the link imports from `env`, and
		// a static, which no import can stand in for, it warns of by name.
		let output = Command::new(&self.lld)
			.args([
				"-flavor",
				"wasm",
				"--no-entry",
				"--no-demangle",
				"--color-diagnostics=never",
			])
			.args(["--import-undefined", "--warn-unresolved-symbols"])
			// What each C object it pulls in runs when the module loads, as the module's
			// start calls it.
			.arg("--export-if-defined=__wasm_call_ctors")
			.arg(format!("--export-if-defined={name}"))
			.arg("-o")
			.arg(&out)
			.args(&self.libs)
			.output()
			.map_err(|err| format!("running {}: {err}", self.lld.display()))?;
		if !output.status.success() {
			return Err(format!(
				"linking `{name}` to see what it imports: {}",
				String::from_utf8_lossy(&output.stderr)
			));
		}
		let wasm = std::fs::read(&out).map_err(|err| format!("reading {}: {err}", out.display()));
		let _ = std::fs::remove_file(&out);
		let (imports, exports) = linked(&wasm?).map_err(|err| format!("reading the link of `{name}`: {err}"))?;
		if !exports.iter().any(|export| export == name) {
			return Ok(None);
		}
		let (needs, imports): (Vec<Import>, Vec<Import>) = imports.into_iter().partition(|(module, _)| module == "env");
		let statics = (String::from_utf8_lossy(&output.stderr).lines())
			.filter_map(|line| {
				line.split_once(": undefined symbol: ")
					.map(|(_, name)| name.trim().to_string())
			})
			.collect::<Vec<_>>();
		let mut undefined: Vec<String> = needs.into_iter().map(|(_, name)| name).chain(statics).collect();
		undefined.sort();
		undefined.dedup();
		Ok(Some(Linked { imports, undefined }))
	}
}

/// The C libraries `krate` links for the target being checked: wasm32 links no other kind,
/// and a `wasm_import_module` is none.
fn c_libraries<'a>(tcx: TyCtxt<'a>, krate: CrateNum) -> impl Iterator<Item = &'a NativeLib> {
	(tcx.native_libraries(krate).iter())
		.filter(|lib| matches!(lib.kind, NativeLibKind::Static { .. } | NativeLibKind::Unspecified))
		.filter(move |lib| (lib.cfg.as_ref()).is_none_or(|cfg| eval_config_entry(tcx.sess, cfg).as_bool()))
}

/// Refuse each C library a crate links whole: the module runs or exports what it holds
/// that no call names, which the walk cannot reach.
fn whole_archives(tcx: TyCtxt<'_>, host: &Host) {
	for krate in std::iter::once(LOCAL_CRATE).chain(tcx.crates(()).iter().copied()) {
		for lib in c_libraries(tcx, krate) {
			if let NativeLibKind::Static {
				whole_archive: Some(true),
				..
			} = lib.kind
			{
				let span = (lib.foreign_module).map_or_else(|| tcx.def_span(krate.as_def_id()), |m| tcx.def_span(m));
				tcx.dcx().span_err(
					span,
					format!(
						"rust2-ffi: `{}` links the C library `{}` with `+whole-archive`, whose code no call names, so the export cannot check what {}'s module imports for it; link it without `+whole-archive` for {}",
						tcx.crate_name(krate),
						lib.name,
						host.target,
						host.target
					),
				);
			}
		}
	}
}

/// A function a module imports, by module and name.
type Import = (String, String);

/// A wasm module's function imports, by module and name, and its exports: a function's, or
/// a static's global.
fn linked(wasm: &[u8]) -> wasmparser::Result<(Vec<Import>, Vec<String>)> {
	let (mut imports, mut exports) = (Vec::new(), Vec::new());
	for payload in wasmparser::Parser::new(0).parse_all(wasm) {
		match payload? {
			wasmparser::Payload::ImportSection(section) => {
				for import in section.into_imports() {
					let import = import?;
					if let wasmparser::TypeRef::Func(_) = import.ty {
						imports.push((import.module.to_string(), import.name.to_string()));
					}
				}
			}
			wasmparser::Payload::ExportSection(section) => {
				for export in section {
					exports.push(export?.name.to_string());
				}
			}
			_ => {}
		}
	}
	Ok((imports, exports))
}

/// Refuse each call in the crate's own code that reaches an import the host lacks without
/// passing through that code again, or else the export whose item does. What the generated
/// crate reaches through std alone is the binding's to implement, refused once as a bug.
fn report<'tcx>(tcx: TyCtxt<'tcx>, graph: &Graph<'tcx>, lacks: &Lacks, own: &dyn Fn(CrateNum) -> bool, host: &Host) {
	let len = graph.nodes.len();
	let krate = |at: usize| match graph.nodes[at] {
		// A type's drop is its crate's, as its `Drop` impl is.
		Node::Fn(Instance {
			def: InstanceKind::Shim(ty::ShimKind::DropGlue(_, Some(ty))),
			..
		}) if let ty::Adt(adt, _) = ty.kind() => adt.did().krate,
		node => node.def_id().krate,
	};
	// A leaf is where the module leaves Rust, whoever declares it.
	let mine = |at: usize| !graph.leaves.contains_key(&at) && own(krate(at));
	let glue = |at: usize| !graph.leaves.contains_key(&at) && krate(at) == LOCAL_CRATE;
	let lacking = |at: usize| lacks.by_leaf.get(&at).is_some_and(|lacking| !lacking.is_empty());
	// What reaches a lacking leaf without passing through the crate's own code or the
	// generated crate's, and the next node on the way, found backwards from the leaves.
	let mut users = vec![Vec::new(); len];
	for (from, edges) in graph.edges.iter().enumerate() {
		for &(to, _) in edges {
			users[to].push(from);
		}
	}
	let mut toward: Vec<Option<usize>> = vec![None; len];
	let mut tainted: Vec<bool> = (0..len).map(lacking).collect();
	let mut queue: std::collections::VecDeque<usize> = (0..len).filter(|&at| tainted[at]).collect();
	while let Some(at) = queue.pop_front() {
		for &user in &users[at] {
			if !tainted[user] && !mine(user) && !glue(user) {
				tainted[user] = true;
				toward[user] = Some(at);
				queue.push_back(user);
			}
		}
	}
	// What the generated crate's exports and the module's start reach, which a binding or
	// the module's load calls, unlike what only another crate's export reaches.
	let mut called = vec![false; len];
	let mut stack: Vec<usize> = (0..len)
		.filter(|&at| graph.origin[at] == at && !graph.kept.contains(&at))
		.collect();
	while let Some(at) = stack.pop() {
		if !std::mem::replace(&mut called[at], true) {
			stack.extend(graph.edges[at].iter().map(|&(next, _)| next));
		}
	}
	// Each call into that, from the crate's own code or from the generated crate's, by
	// where it is refused, with the node it calls, the export it serves, and whether
	// anything calls it.
	let mut refused: FxIndexMap<(Span, usize), (usize, bool)> = FxIndexMap::default();
	let mut runtime: FxIndexSet<usize> = FxIndexSet::default();
	for (from, edges) in graph.edges.iter().enumerate() {
		if !mine(from) && !glue(from) {
			continue;
		}
		for &(to, span) in edges {
			if !tainted[to] || mine(to) {
				continue;
			}
			// A shim's own spans are the compiler's: a type's drop is refused at the type.
			let shim = !matches!(
				graph.nodes[from],
				Node::Fn(Instance {
					def: InstanceKind::Item(_),
					..
				})
			);
			let span = match glue(from) {
				false if span.is_dummy() || shim => item_span(tcx, graph.nodes[from]),
				false => span,
				// What the generated crate reaches through std alone is its runtime's.
				true if tcx.lookup_stability(krate(to).as_def_id()).is_some() => {
					runtime.insert(to);
					continue;
				}
				// Its call names the export's item, whose own span says where.
				true => item_span(tcx, graph.nodes[to]),
			};
			// A generic function's instances share its spans, some perhaps called and some not.
			let entry = refused.entry((span, to)).or_insert((graph.origin[from], called[from]));
			if called[from] && !entry.1 {
				*entry = (graph.origin[from], true);
			}
		}
	}
	// What another crate runs when the module loads, which no call of the crate's own code
	// reaches, at that crate's static.
	for &at in &graph.inits {
		if tainted[at] && !mine(at) {
			refused.entry((item_span(tcx, graph.nodes[at]), at)).or_insert((at, true));
		}
	}
	let path = |to: usize| {
		let mut path = vec![to];
		while let Some(next) = toward[*path.last().expect("a path has a node")] {
			path.push(next);
		}
		path
	};
	// Everything `to` reaches the same way that the module cannot have, said: only what
	// fails the module's load or link unless something calls `to`.
	let reached = |to: usize, called: bool| {
		let mut seen = FxHashSet::default();
		let mut stack = vec![to];
		let mut reached = Vec::new();
		while let Some(at) = stack.pop() {
			if !seen.insert(at) {
				continue;
			}
			reached.extend(lacks.by_leaf.get(&at).into_iter().flatten().cloned());
			stack.extend(
				(graph.edges[at].iter())
					.map(|&(next, _)| next)
					.filter(|&next| tainted[next]),
			);
		}
		reached.sort();
		reached.dedup();
		let (mut imports, mut undefined, mut bindgen) = (Vec::new(), Vec::new(), Vec::new());
		for lack in &reached {
			match lack {
				Lack::Import(name) => imports.push(name.as_str()),
				Lack::Undefined(name) => undefined.push(name.as_str()),
				Lack::Bindgen(name) if called => bindgen.push(name.as_str()),
				Lack::Bindgen(_) => {}
			}
		}
		let mut said = Vec::new();
		if !imports.is_empty() {
			said.push(format!("`{}`, which {}'s host lacks", imports.join("`, `"), host.name));
		}
		if !undefined.is_empty() {
			said.push(format!(
				"`{}`, which no library {}'s module links defines",
				undefined.join("`, `"),
				host.target
			));
		}
		if !bindgen.is_empty() {
			said.push(format!(
				"`{}`, which wasm-bindgen panics on in {} rather than import from JavaScript",
				bindgen.join("`, `"),
				host.target
			));
		}
		said.join(", and ")
	};
	let through = |path: &[usize]| {
		let named: Vec<String> = path.iter().filter_map(|&at| describe(tcx, graph, at)).collect();
		format!("through {}", named.join(", then "))
	};
	let mut bugs = FxHashSet::default();
	for to in runtime {
		let path = path(to);
		if bugs.insert(*path.last().expect("a path has a node")) {
			tcx.dcx()
				.struct_err(format!(
					"rust2-ffi: the generated crate reaches {} through std; this is a bug in rust2-ffi",
					reached(to, true),
				))
				.with_note(through(&path))
				.emit();
		}
	}
	// What another crate exports, which no call of the crate's own code reaches, at that
	// crate's item, if the module cannot load or link with it.
	for &at in &graph.kept {
		if tainted[at] && !mine(at) {
			refused.entry((item_span(tcx, graph.nodes[at]), at)).or_insert((at, false));
		}
	}
	for ((span, to), (export, called)) in refused {
		// What no binding calls is refused only where the module cannot load or link with it.
		let said = reached(to, called);
		if said.is_empty() {
			continue;
		}
		let path = path(to);
		let what = match graph.leaves.get(&to) {
			Some(Leaf::Import { .. }) => "the module imports".to_string(),
			Some(Leaf::Bindgen(_)) => "this calls".to_string(),
			// A C function nothing defines is what it lacks itself.
			Some(Leaf::C(name))
				if lacks
					.by_leaf
					.get(&to)
					.is_some_and(|l| l[..] == [Lack::Undefined(name.clone())]) =>
			{
				"this calls".to_string()
			}
			Some(Leaf::C(name)) => format!("the C function `{name}` reaches"),
			_ => format!(
				"{} reaches",
				describe(tcx, graph, to).unwrap_or_else(|| "this".to_string())
			),
		};
		let mut err = tcx.dcx().struct_span_err(
			span,
			format!(
				"rust2-ffi: {what} {said}; leave it out of {}'s build with a `cfg`, or exclude its export in the config",
				host.target
			),
		);
		if path.len() > 1 {
			err = err.with_note(through(&path));
		}
		let name = tcx.item_name(graph.nodes[export].def_id());
		err.with_note(match graph.inits.contains(&export) {
			true => format!("the module runs it when it loads, through `{name}`"),
			false => format!("the module exports it through `{name}`"),
		})
		.emit();
	}
}

/// Where a refusal from the generated crate points: the item it calls, or the type it
/// drops.
fn item_span<'tcx>(tcx: TyCtxt<'tcx>, node: Node<'tcx>) -> Span {
	if let Node::Fn(instance) = node
		&& let InstanceKind::Shim(ty::ShimKind::DropGlue(_, Some(ty))) = instance.def
		&& let ty::Adt(adt, _) = ty.kind()
	{
		return tcx.def_span(adt.did());
	}
	tcx.def_span(node.def_id())
}

/// A node as a refusal names it, or none for a shim the compiler makes between two calls.
fn describe<'tcx>(tcx: TyCtxt<'tcx>, graph: &Graph<'tcx>, at: usize) -> Option<String> {
	match graph.leaves.get(&at) {
		Some(Leaf::Import { module, name }) => return Some(format!("the import `{module}.{name}`")),
		Some(Leaf::C(name)) => return Some(format!("`{name}`, from C")),
		Some(Leaf::Bindgen(name)) => return Some(format!("wasm-bindgen's `{name}`")),
		None => {}
	}
	match graph.nodes[at] {
		Node::Fn(instance) => match instance.def {
			InstanceKind::Item(def_id) => Some(format!("`{}`", tcx.def_path_str(def_id))),
			InstanceKind::Shim(ty::ShimKind::DropGlue(_, Some(ty))) => Some(format!("the drop of `{ty}`")),
			_ => None,
		},
		Node::Static(def_id) => Some(format!("the static `{}`", tcx.def_path_str(def_id))),
	}
}
