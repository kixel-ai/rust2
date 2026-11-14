//! The binary as rustc: plain for every crate but a local package's, whose definitions and
//! references it writes once rustc has checked it, reading the `pub` tokens the CLI
//! narrows as `pub(crate)` without touching the files.

use std::collections::hash_map::Entry;
use std::io;
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::sync::Arc;

use rustc_data_structures::fx::{FxHashMap, FxHashSet};
use rustc_driver::{Callbacks, Compilation};
use rustc_hir::def::{DefKind, Res};
use rustc_hir::def_id::{DefId, LocalDefId};
use rustc_hir::intravisit::{self, Visitor};
use rustc_hir::{self as hir, ItemKind, Node, UseKind};
use rustc_interface::interface::{Compiler, Config};
use rustc_middle::hir::nested_filter;
use rustc_middle::ty::{self, Ty, TyCtxt, TypeckResults};
use rustc_span::source_map::{FileLoader, RealFileLoader};
use rustc_span::{FileName, SyntaxContext, kw};
use rustc_structures::CrateType;

use crate::facts::{self, Def, Edits, Hash, Impl, Ref, Site, Unit};

/// Run as rustc, with `args` as cargo passed them: the wrapped rustc's path first.
pub fn run(args: Vec<String>) -> ExitCode {
	rust2_driver::rustc::run("rust2-pub", args, |_, _| {
		Ok(Collect::for_package()?.map(|job| Box::new(job) as rust2_driver::rustc::Job))
	})
}

/// Collects a local crate's facts, reading it narrowed by [`Collect::edits`].
struct Collect {
	/// Where to write them.
	facts: PathBuf,
	edits: Arc<Edits>,
}

impl Collect {
	/// The job for this rustc invocation, if it compiles a crate of a local package.
	fn for_package() -> Result<Option<Self>, String> {
		let Some(facts) = std::env::var_os(facts::env::FACTS) else {
			return Ok(None);
		};
		// A build script's probe of the compiler, as `autocfg` makes, compiles no package.
		if std::env::var_os("CARGO_CFG_TARGET_ARCH").is_some() {
			return Ok(None);
		}
		let Some(package) = std::env::var_os("CARGO_MANIFEST_DIR") else {
			return Ok(None);
		};
		let local = read_json::<Vec<PathBuf>>(std::env::var_os(facts::env::LOCAL).as_deref(), facts::env::LOCAL)?;
		if !local.iter().any(|dir| dir == Path::new(&package)) {
			return Ok(None);
		}
		let edits = match std::env::var_os(facts::env::EDITS) {
			Some(path) => read_json(Some(&path), facts::env::EDITS)?,
			None => Edits::new(),
		};
		Ok(Some(Self {
			facts: facts.into(),
			edits: Arc::new(edits),
		}))
	}
}

fn read_json<T: serde::de::DeserializeOwned>(path: Option<&std::ffi::OsStr>, var: &str) -> Result<T, String> {
	let path = path.ok_or_else(|| format!("{var} is unset"))?;
	let text = std::fs::read_to_string(path).map_err(|err| format!("reading {}: {err}", path.display()))?;
	serde_json::from_str(&text).map_err(|err| format!("parsing {}: {err}", path.display()))
}

impl Callbacks for Collect {
	fn config(&mut self, config: &mut Config) {
		if !self.edits.is_empty() {
			config.file_loader = Some(Box::new(Narrowed(self.edits.clone())));
		}
	}

	fn after_analysis<'tcx>(&mut self, _compiler: &Compiler, tcx: TyCtxt<'tcx>) -> Compilation {
		let unit = Collector::unit(tcx);
		let name = format!("{}-{:016x}-{}.json", unit.crate_name, unit.crate_id, std::process::id());
		let path = self.facts.join(name);
		let json = serde_json::to_vec(&unit).expect("facts serialize");
		if let Err(err) = std::fs::write(&path, json) {
			tcx.dcx().fatal(format!("rust2-pub: writing {}: {err}", path.display()));
		}
		Compilation::Continue
	}
}

/// Reads each file as it is but for the `pub` tokens the edits name, which it reads as
/// `pub(crate)`.
struct Narrowed(Arc<Edits>);

impl FileLoader for Narrowed {
	fn file_exists(&self, path: &Path) -> bool {
		RealFileLoader.file_exists(path)
	}

	fn read_file(&self, path: &Path) -> io::Result<String> {
		let mut text = RealFileLoader.read_file(path)?;
		let Some(offsets) = std::fs::canonicalize(path).ok().and_then(|path| self.0.get(&path)) else {
			return Ok(text);
		};
		// Last first, so each offset still points into the original text.
		for &offset in offsets.iter().rev() {
			let at = offset as usize;
			if !is_pub(text.as_bytes(), at) {
				return Err(io::Error::other(format!(
					"rust2-pub: byte {offset} of {} is not a `pub` to narrow; was it edited since the audit began?",
					path.display()
				)));
			}
			text.replace_range(at..at + 3, "pub(crate)");
		}
		Ok(text)
	}

	fn read_binary_file(&self, path: &Path) -> io::Result<Arc<[u8]>> {
		RealFileLoader.read_binary_file(path)
	}

	fn current_directory(&self) -> io::Result<PathBuf> {
		RealFileLoader.current_directory()
	}
}

/// Whether `text` holds the keyword `pub` at `at`. Whether it is bare, rather than
/// `pub(crate)`, is rustc's to say: `pub (u8, u8)` is a public tuple field.
pub fn is_pub(text: &[u8], at: usize) -> bool {
	let ident = |b: &u8| b.is_ascii_alphanumeric() || *b == b'_';
	text.get(at..at + 3) == Some(b"pub")
		&& !at.checked_sub(1).and_then(|i| text.get(i)).is_some_and(ident)
		&& !text.get(at + 3).is_some_and(ident)
}

/// Walks one crate's HIR, recording each definition and what refers to what.
struct Collector<'tcx> {
	tcx: TyCtxt<'tcx>,
	cwd: PathBuf,
	files: Vec<PathBuf>,
	/// Each source file's index in `files`, or `None` for one with no path on disk.
	file_index: FxHashMap<usize, Option<u32>>,
	/// The item-like whose source is being walked.
	owner: LocalDefId,
	typeck: Option<&'tcx TypeckResults<'tcx>>,
	/// Each pair's first site, and whether any of its references is in the interface.
	refs: FxHashMap<(Hash, Hash), (Option<facts::Span>, bool)>,
	sites: Vec<Site>,
	/// The expansions already traced to their macros, per owner.
	traced: FxHashSet<(LocalDefId, SyntaxContext)>,
}

impl<'tcx> Collector<'tcx> {
	fn unit(tcx: TyCtxt<'tcx>) -> Unit {
		let mut this = Collector {
			tcx,
			cwd: std::env::current_dir().unwrap_or_default(),
			files: Vec::new(),
			file_index: FxHashMap::default(),
			owner: rustc_hir::def_id::CRATE_DEF_ID,
			typeck: None,
			refs: FxHashMap::default(),
			sites: Vec::new(),
			traced: FxHashSet::default(),
		};
		let types = tcx.crate_types();
		let proc_macro = types.contains(&CrateType::ProcMacro);
		let lib = types
			.iter()
			.any(|t| matches!(t, CrateType::Rlib | CrateType::Dylib | CrateType::ProcMacro));
		let visibilities = tcx.effective_visibilities(());
		let mut defs = Vec::new();
		let mut roots = Vec::new();
		let mut impls = Vec::new();
		if let Some((main, _)) = tcx.entry_fn(()) {
			roots.push(this.hash(main));
		}
		for id in tcx.iter_local_def_id() {
			let def = id.to_def_id();
			let kind = tcx.def_kind(def);
			// What source can name, or what makes a crate need it: not generic parameters,
			// closures, or the definitions rustc makes up while lowering.
			if !matches!(
				kind,
				DefKind::Mod
					| DefKind::Struct
					| DefKind::Union
					| DefKind::Enum
					| DefKind::Variant
					| DefKind::Trait
					| DefKind::TyAlias
					| DefKind::ForeignTy
					| DefKind::TraitAlias
					| DefKind::AssocTy
					| DefKind::Fn
					| DefKind::Const
					| DefKind::Static { .. }
					| DefKind::Ctor(..)
					| DefKind::AssocFn
					| DefKind::AssocConst
					| DefKind::Macro(_)
					| DefKind::ExternCrate
					| DefKind::Use
					| DefKind::ForeignMod
					| DefKind::Impl { .. }
					| DefKind::Field
			) {
				continue;
			}
			let hash = this.hash(def);
			let symbol = matches!(kind, DefKind::Fn | DefKind::AssocFn | DefKind::Static { .. })
				&& tcx.codegen_fn_attrs(def).contains_extern_indicator();
			let exported = visibilities.is_exported(id);
			// What runs or links with nothing in Rust naming it: an exported symbol, a
			// `const _` that asserts at compile time, and the macros a proc-macro crate
			// exports.
			if symbol
				|| matches!(kind, DefKind::Const) && tcx.opt_item_name(def) == Some(kw::Underscore)
				|| proc_macro && exported
			{
				roots.push(hash);
			}
			let parent = tcx.opt_local_parent(id).map(|p| this.hash(p.to_def_id()));
			let member_of = match kind {
				DefKind::AssocFn | DefKind::AssocConst | DefKind::AssocTy => {
					let container = tcx.parent(def);
					match tcx.def_kind(container) {
						DefKind::Impl { of_trait: false } => {
							match tcx.type_of(container).instantiate_identity().skip_norm_wip().kind() {
								ty::Adt(adt, _) => Some(this.hash(adt.did())),
								_ => None,
							}
						}
						_ => None,
					}
				}
				_ => None,
			};
			this.owner = id;
			this.signature(id, kind);
			if let DefKind::Impl { of_trait: true } = kind {
				let mut requires = vec![this.hash(tcx.impl_trait_id(def))];
				if let ty::Adt(adt, _) = tcx.type_of(def).instantiate_identity().skip_norm_wip().kind() {
					requires.push(this.hash(adt.did()));
				}
				impls.push(Impl { hash, requires });
				// An impl needs every item it declares, whatever calls them.
				for &item in tcx.associated_item_def_ids(def) {
					this.refer(item, None);
				}
			}
			let token = this.token(id);
			let span = match tcx.hir_get_if_local(def) {
				// A diagnostic about an import points anywhere in it.
				Some(Node::Item(item)) if matches!(item.kind, ItemKind::Use(..)) => item.span,
				_ => tcx.def_span(def),
			};
			defs.push(Def {
				hash,
				kind: format!("{kind:?}"),
				path: match kind {
					// An import's path is `{use#n}`: show what it imports instead.
					DefKind::Use => {
						let snippet = tcx.sess.source_map().span_to_snippet(tcx.def_span(def));
						let parent = tcx.def_path_str(tcx.parent(def));
						let parent = if parent.is_empty() { "crate".into() } else { parent };
						format!("{parent}::{{use {}}}", snippet.unwrap_or_default())
					}
					_ => tcx.def_path_str(def),
				},
				span: this.span(span),
				token,
				exported,
				symbol,
				parent,
				member_of,
			});
		}
		tcx.hir_visit_all_item_likes_in_crate(&mut this);
		let crate_id = tcx.stable_crate_id(rustc_hir::def_id::LOCAL_CRATE).as_u64();
		let refs = this
			.refs
			.into_iter()
			.map(|((from, to), (site, interface))| Ref {
				from,
				to,
				site,
				interface,
			})
			.collect();
		Unit {
			crate_name: tcx.crate_name(rustc_hir::def_id::LOCAL_CRATE).to_string(),
			crate_id,
			package: std::env::var_os("CARGO_MANIFEST_DIR").unwrap_or_default().into(),
			test: tcx.sess.opts.test,
			lib,
			files: this.files,
			defs,
			refs,
			sites: this.sites,
			roots,
			impls,
		}
	}

	fn hash(&self, def: DefId) -> Hash {
		let (a, b) = self.tcx.def_path_hash(def).0.split();
		[a.as_u64(), b.as_u64()]
	}

	/// The `pub` written on `id`, if any.
	fn token(&mut self, id: LocalDefId) -> Option<facts::Span> {
		let vis = match self.tcx.hir_node_by_def_id(id) {
			Node::Item(item) => item.vis_span,
			Node::Field(field) => field.vis_span,
			Node::ImplItem(item) => match item.impl_kind {
				hir::ImplItemImplKind::Inherent { vis_span } => vis_span,
				hir::ImplItemImplKind::Trait { .. } => return None,
			},
			Node::ForeignItem(item) => item.vis_span,
			_ => return None,
		};
		let snippet = self.tcx.sess.source_map().span_to_snippet(vis).ok()?;
		(snippet == "pub").then(|| self.span(vis)).flatten()
	}

	/// `span` in source on disk, if it is there.
	fn span(&mut self, span: rustc_span::Span) -> Option<facts::Span> {
		if span.is_dummy() {
			return None;
		}
		let sm = self.tcx.sess.source_map();
		let lo = sm.lookup_byte_offset(span.lo());
		let hi = sm.lookup_byte_offset(span.hi());
		if !Arc::ptr_eq(&lo.sf, &hi.sf) {
			return None;
		}
		let key = Arc::as_ptr(&lo.sf) as usize;
		let file = match self.file_index.entry(key) {
			Entry::Occupied(entry) => *entry.get(),
			Entry::Vacant(entry) => {
				let path = match &lo.sf.name {
					FileName::Real(real) => real.local_path().map(|p| self.cwd.join(p)),
					_ => None,
				};
				let index = path.and_then(|p| std::fs::canonicalize(p).ok()).map(|p| {
					self.files.push(p);
					(self.files.len() - 1) as u32
				});
				*entry.insert(index)
			}
		}?;
		Some(facts::Span {
			file,
			lo: lo.pos.0,
			hi: hi.pos.0,
		})
	}

	/// The owner refers to `to`, at `site` if the source names it there.
	fn refer(&mut self, to: DefId, site: Option<rustc_span::Span>) {
		self.refer_as(to, site, self.typeck.is_none());
	}

	/// [`Self::refer`], saying whether the owner's interface names `to`: a path's
	/// prefix, such as the module in `api::Item`, names only where `Item` is.
	fn refer_as(&mut self, to: DefId, site: Option<rustc_span::Span>, interface: bool) {
		let from = self.hash(self.owner.to_def_id());
		let to_hash = self.hash(to);
		let site = site.and_then(|s| self.span(s));
		if let Some(span) = site {
			self.sites.push(Site { span, to: to_hash });
		}
		let entry = self.refs.entry((from, to_hash)).or_insert((site, interface));
		entry.1 |= interface;
		// A tuple constructor is as visible as its least visible field.
		if let DefKind::Ctor(..) = self.tcx.def_kind(to) {
			let variant = self.tcx.parent(to);
			let adt = match self.tcx.def_kind(variant) {
				DefKind::Variant => self.tcx.parent(variant),
				_ => variant,
			};
			for field in &self.tcx.adt_def(adt).variant_with_id(variant).fields {
				self.refs
					.entry((from, self.hash(field.did)))
					.or_insert((None, interface))
					.1 |= interface;
			}
		}
	}

	/// Every definition `ty` names, referred to by the owner.
	fn refer_ty(&mut self, ty: Ty<'tcx>) {
		for arg in ty.walk() {
			let Some(ty) = arg.as_type() else { continue };
			match *ty.kind() {
				ty::Adt(adt, _) => self.refer(adt.did(), None),
				ty::Foreign(def) | ty::FnDef(def, _) => self.refer(def, None),
				ty::Alias(_, alias) => {
					let (ty::Projection { def_id }
					| ty::Inherent { def_id }
					| ty::Opaque { def_id }
					| ty::Free { def_id }) = alias.kind;
					self.refer(def_id, None);
				}
				ty::Dynamic(preds, ..) => {
					for pred in preds {
						match pred.skip_binder() {
							ty::ExistentialPredicate::Trait(t) => self.refer(t.def_id, None),
							ty::ExistentialPredicate::Projection(p) => self.refer(p.def_id, None),
							ty::ExistentialPredicate::AutoTrait(def) => self.refer(def, None),
						}
					}
				}
				_ => {}
			}
		}
	}

	/// What `id`'s signature names, beyond the paths written in it: types rustc
	/// resolved, such as `T::Assoc`, and bounds.
	fn signature(&mut self, id: LocalDefId, kind: DefKind) {
		let tcx = self.tcx;
		let def = id.to_def_id();
		match kind {
			DefKind::Fn | DefKind::AssocFn => {
				let sig = tcx.fn_sig(def).instantiate_identity().skip_norm_wip().skip_binder();
				for ty in sig.inputs_and_output {
					self.refer_ty(ty);
				}
			}
			DefKind::Field | DefKind::TyAlias | DefKind::Static { .. } | DefKind::Const => {
				self.refer_ty(tcx.type_of(def).instantiate_identity().skip_norm_wip());
			}
			// A trait's associated item has a type of its own only with a default.
			DefKind::AssocTy | DefKind::AssocConst
				if tcx.def_kind(tcx.parent(def)) != DefKind::Trait || tcx.defaultness(def).has_value() =>
			{
				self.refer_ty(tcx.type_of(def).instantiate_identity().skip_norm_wip());
			}
			DefKind::Impl { of_trait } => {
				self.refer_ty(tcx.type_of(def).instantiate_identity().skip_norm_wip());
				if of_trait {
					let trait_ref = tcx.impl_trait_ref(def).instantiate_identity().skip_norm_wip();
					self.refer(trait_ref.def_id, None);
					for ty in trait_ref.args.types() {
						self.refer_ty(ty);
					}
				}
			}
			_ => {}
		}
		if matches!(
			kind,
			DefKind::Fn
				| DefKind::AssocFn
				| DefKind::Struct
				| DefKind::Enum
				| DefKind::Union
				| DefKind::Trait
				| DefKind::TraitAlias
				| DefKind::TyAlias
				| DefKind::Impl { .. }
				| DefKind::AssocTy
		) {
			let predicates = tcx.explicit_clauses_of(def).clauses;
			let supers = match kind {
				DefKind::Trait | DefKind::TraitAlias => tcx.explicit_super_clauses_of(def).skip_binder(),
				_ => &[],
			};
			for &(clause, _) in predicates.iter().chain(supers) {
				match clause.kind().skip_binder() {
					ty::ClauseKind::Trait(pred) => {
						self.refer(pred.def_id(), None);
						for ty in pred.trait_ref.args.types() {
							self.refer_ty(ty);
						}
					}
					ty::ClauseKind::Projection(pred) => {
						self.refer(pred.projection_term.expect_projection_def_id(), None);
						for ty in pred.projection_term.args.types() {
							self.refer_ty(ty);
						}
						if let Some(ty) = pred.term.as_type() {
							self.refer_ty(ty);
						}
					}
					ty::ClauseKind::TypeOutlives(pred) => self.refer_ty(pred.0),
					_ => {}
				}
			}
		}
	}

	/// The macros `span` came from, referred to by the owner.
	fn provenance(&mut self, mut span: rustc_span::Span) {
		while span.from_expansion() {
			if !self.traced.insert((self.owner, span.ctxt())) {
				return;
			}
			let data = span.ctxt().outer_expn_data();
			if let Some(def) = data.macro_def_id {
				self.refer(def, None);
			}
			span = data.call_site;
		}
	}

	/// Each re-export another crate's module `module` names `name` through, which the
	/// path depends on as much as on what it names.
	fn reexports(&mut self, module: DefId, name: rustc_span::Symbol, res: Res, interface: bool) {
		if module.is_local() {
			return;
		}
		let tcx = self.tcx;
		for child in tcx.module_children(module) {
			if child.ident.name == name && child.res.opt_def_id() == res.opt_def_id() {
				for reexport in &child.reexport_chain {
					if let Some(id) = reexport.id() {
						self.refer_as(id, None, interface);
					}
				}
			}
		}
	}

	/// A field of the ADT `ty`'s variant `res` (or its only one), named at `site`.
	fn field(&mut self, ty: Ty<'tcx>, res: Option<Res>, hir_id: hir::HirId, site: rustc_span::Span) {
		let Some(typeck) = self.typeck else { return };
		let ty::Adt(adt, _) = ty.peel_refs().kind() else { return };
		let Some(index) = typeck.opt_field_index(hir_id) else {
			return;
		};
		let variant = match res {
			Some(res) => adt.variant_of_res(res),
			None if adt.is_enum() => return,
			None => adt.non_enum_variant(),
		};
		self.refer(variant.fields[index].did, Some(site));
	}

	/// Each re-export in another crate's module a glob in `tree` imports, any of which
	/// it may use.
	fn globs(&mut self, tree: hir::UseTree<'tcx>) {
		match tree.kind {
			UseKind::Glob => {
				for res in tree.prefix.res.present_items() {
					let Res::Def(DefKind::Mod, module) = res else { continue };
					if module.is_local() {
						continue;
					}
					for child in self.tcx.module_children(module) {
						for reexport in &child.reexport_chain {
							if let Some(id) = reexport.id() {
								self.refer(id, None);
							}
						}
					}
				}
			}
			UseKind::Nested { items } => {
				for &(item, ..) in items {
					self.globs(item);
				}
			}
			UseKind::Single(_) => {}
		}
	}

	/// Refer to what each of `segments` names, and the re-exports each is named through,
	/// after the module `prev` a `use` list's prefix names, the interface naming only the
	/// `last` segment, if it is the path's last. Returns the module the last one names, if
	/// it names one.
	fn segments(&mut self, segments: &[hir::PathSegment<'tcx>], mut prev: Option<DefId>, last: bool) -> Option<DefId> {
		let interface = self.typeck.is_none();
		for (i, segment) in segments.iter().enumerate() {
			if let Res::Def(kind, def) = segment.res {
				let interface = interface && last && i + 1 == segments.len();
				if let Some(module) = prev {
					self.reexports(module, segment.ident.name, segment.res, interface);
				}
				self.refer_as(def, Some(segment.ident.span), interface);
				prev = matches!(kind, DefKind::Mod).then_some(def);
			} else {
				prev = None;
			}
		}
		prev
	}

	/// An import, its list's paths continuing after the module `prev` names.
	fn use_tree(&mut self, tree: &hir::UseTree<'tcx>, prev: Option<DefId>) {
		let mut module = prev;
		if let Some((last, init)) = tree.prefix.segments.split_last() {
			let parent = self.segments(init, prev, false);
			// The last segment names one thing in each namespace, which the import's
			// resolutions hold and the segment may not.
			let mut named: Vec<Res> = tree.prefix.res.present_items().collect();
			if !named.contains(&last.res) {
				named.push(last.res);
			}
			module = None;
			for res in named {
				let Res::Def(kind, def) = res else { continue };
				if let Some(parent) = parent {
					self.reexports(parent, last.ident.name, res, self.typeck.is_none());
				}
				self.refer(def, Some(last.ident.span));
				if let DefKind::Mod = kind {
					module = Some(def);
				}
			}
		}
		if let UseKind::Nested { items } = tree.kind {
			for &(ref item, _, def) in items {
				self.with_owner(def, |this| this.use_tree(item, module));
			}
		}
	}

	fn with_owner(&mut self, owner: LocalDefId, walk: impl FnOnce(&mut Self)) {
		let outer = std::mem::replace(&mut self.owner, owner);
		walk(self);
		self.owner = outer;
	}
}

impl<'tcx> Visitor<'tcx> for Collector<'tcx> {
	type NestedFilter = nested_filter::OnlyBodies;

	fn maybe_tcx(&mut self) -> TyCtxt<'tcx> {
		self.tcx
	}

	fn visit_nested_body(&mut self, id: hir::BodyId) {
		let outer = self.typeck.replace(self.tcx.typeck_body(id));
		self.visit_body(self.tcx.hir_body(id));
		self.typeck = outer;
	}

	fn visit_item(&mut self, item: &'tcx hir::Item<'tcx>) {
		self.with_owner(item.owner_id.def_id, |this| {
			this.provenance(item.span);
			// A glob of another crate's module may use any re-export in it.
			if let ItemKind::Use(tree) = item.kind {
				this.globs(tree);
			}
			intravisit::walk_item(this, item);
		});
	}

	fn visit_trait_item(&mut self, item: &'tcx hir::TraitItem<'tcx>) {
		self.with_owner(item.owner_id.def_id, |this| intravisit::walk_trait_item(this, item));
	}

	fn visit_impl_item(&mut self, item: &'tcx hir::ImplItem<'tcx>) {
		self.with_owner(item.owner_id.def_id, |this| intravisit::walk_impl_item(this, item));
	}

	fn visit_foreign_item(&mut self, item: &'tcx hir::ForeignItem<'tcx>) {
		self.with_owner(item.owner_id.def_id, |this| intravisit::walk_foreign_item(this, item));
	}

	fn visit_field_def(&mut self, field: &'tcx hir::FieldDef<'tcx>) {
		self.with_owner(field.def_id, |this| intravisit::walk_field_def(this, field));
	}

	fn visit_variant(&mut self, variant: &'tcx hir::Variant<'tcx>) {
		self.with_owner(variant.def_id, |this| intravisit::walk_variant(this, variant));
	}

	fn visit_path(&mut self, path: &hir::Path<'tcx>, _id: hir::HirId) {
		self.segments(path.segments, None, true);
		intravisit::walk_path(self, path);
	}

	fn visit_use(&mut self, tree: &'tcx hir::UseTree<'tcx>, _id: hir::HirId, def: LocalDefId) {
		self.with_owner(def, |this| this.use_tree(tree, None));
	}

	fn visit_expr(&mut self, expr: &'tcx hir::Expr<'tcx>) {
		self.provenance(expr.span);
		if let Some(typeck) = self.typeck {
			if let Some(def) = typeck.type_dependent_def_id(expr.hir_id) {
				let site = match expr.kind {
					hir::ExprKind::MethodCall(segment, ..) => segment.ident.span,
					hir::ExprKind::Path(hir::QPath::TypeRelative(_, segment)) => segment.ident.span,
					_ => expr.span,
				};
				self.refer(def, Some(site));
			}
			if let Some(ty) = typeck.node_type_opt(expr.hir_id) {
				self.refer_ty(ty);
			}
			if let Some(ty) = typeck.expr_ty_adjusted_opt(expr) {
				self.refer_ty(ty);
			}
			match expr.kind {
				hir::ExprKind::Field(base, ident) => {
					if let Some(ty) = typeck.expr_ty_adjusted_opt(base) {
						self.field(ty, None, expr.hir_id, ident.span);
					}
				}
				hir::ExprKind::Struct(qpath, fields, tail) => {
					let res = typeck.qpath_res(qpath, expr.hir_id);
					if let Some(ty) = typeck.node_type_opt(expr.hir_id) {
						for field in fields {
							self.field(ty, Some(res), field.hir_id, field.ident.span);
						}
						// `..base` moves every field it does not name, each of which must
						// be visible here.
						if let hir::StructTailExpr::Base(_) = tail
							&& let ty::Adt(adt, _) = ty.kind()
						{
							for field in &adt.variant_of_res(res).fields {
								self.refer(field.did, None);
							}
						}
					}
				}
				_ => {}
			}
		}
		intravisit::walk_expr(self, expr);
	}

	fn visit_pat(&mut self, pat: &'tcx hir::Pat<'tcx>) {
		self.provenance(pat.span);
		if let Some(typeck) = self.typeck
			&& let Some(ty) = typeck.node_type_opt(pat.hir_id)
		{
			self.refer_ty(ty);
			if let hir::PatKind::Struct(qpath, fields, _) = pat.kind {
				let res = typeck.qpath_res(&qpath, pat.hir_id);
				for field in fields {
					self.field(ty, Some(res), field.hir_id, field.ident.span);
				}
			}
		}
		intravisit::walk_pat(self, pat);
	}

	fn visit_pat_expr(&mut self, expr: &'tcx hir::PatExpr<'tcx>) {
		if let Some(typeck) = self.typeck
			&& let Some(def) = typeck.type_dependent_def_id(expr.hir_id)
		{
			self.refer(def, Some(expr.span));
		}
		intravisit::walk_pat_expr(self, expr);
	}

	fn visit_ty(&mut self, ty: &'tcx hir::Ty<'tcx, hir::AmbigArg>) {
		self.provenance(ty.span);
		intravisit::walk_ty(self, ty);
	}
}
