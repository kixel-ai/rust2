//! wasm-bindgen, which translation maps itself, as it maps std: `JsValue` and every type
//! an import declares are the JS value itself, and each function `#[wasm_bindgen]` made
//! around an import is replaced whole by the JS call it stands for.
//!
//! Each invocation of the macro on wasm32 encodes what it declares in a
//! `#[link_section = "__wasm_bindgen_unstable"]` static, the program the wasm-bindgen CLI
//! reads, which every crate's metadata keeps. A function it made around an import declares
//! the import's `extern "C"` shim inside its own body, so the shim's parent's parent is
//! that function, and the shim's name keys the import's shape in the program: its class,
//! JS name, kind, namespace, and `catch`. What else the program declares, such as custom
//! JS or an export, is refused where translation reaches it.

mod convert;
pub mod decode;
mod lower;

use std::rc::Rc;

use rustc_data_structures::fx::FxHashMap;
use rustc_hir::def::DefKind;
use rustc_hir::def_id::{CrateNum, DefId, DefIndex, LOCAL_CRATE};
use rustc_middle::ty::{self, Instance, InstanceKind, Ty, TyCtxt};
use rustc_span::hygiene::{ExpnKind, MacroKind};

use super::Translator;

pub use lower::{call, constant};

/// The module wasm-bindgen's imports name in place of where a global is: the JS that runs
/// the module provides them, and wasm-bindgen's own intrinsics are its imports too.
const PLACEHOLDER: &str = "__wbindgen_placeholder__";

/// Whether `did` is wasm-bindgen's `__rt::LazyCell`, where a `thread_local_v2` import keeps
/// its value from first use: std's `LazyCell`, but for being `Sync` where WASM has no
/// threads, which translation makes an `rs.LazyCell` alike.
pub fn is_lazy(tcx: TyCtxt<'_>, did: DefId) -> bool {
	tcx.crate_name(did.krate).as_str() == "wasm_bindgen"
		&& matches!(tcx.def_kind(did), DefKind::Struct)
		&& tcx
			.def_path(did)
			.data
			.iter()
			.filter_map(|d| d.data.get_opt_name())
			.map(|n| n.to_string())
			.eq(["__rt", "LazyCell"])
}

/// Whether `did` is a type whose value is a JS value: wasm-bindgen's `JsValue`, or a type
/// an import declares, which the macro makes a `#[repr(transparent)]` struct whose `obj`
/// is the JS-represented type it extends, beside the `PhantomData` of its type
/// parameters, such as js-sys's `Promise<T>`, whose instances are then one JS value. Its
/// `obj` must be the macro's own, so that a struct a crate writes the same way keeps its
/// fields and its `Drop`.
pub fn represents(tcx: TyCtxt<'_>, did: DefId) -> bool {
	if !matches!(tcx.def_kind(did), DefKind::Struct) {
		return false;
	}
	if tcx.crate_name(did.krate).as_str() == "wasm_bindgen" && tcx.item_name(did).as_str() == "JsValue" {
		return true;
	}
	let adt = tcx.adt_def(did);
	if !adt.repr().transparent() {
		return false;
	}
	let mut obj = None;
	for field in &adt.non_enum_variant().fields {
		let ty = tcx.type_of(field.did).instantiate_identity().skip_norm_wip();
		match ty.kind() {
			ty::Adt(inner, _) if field.name.as_str() == "obj" && from_macro(tcx, field.did) => obj = Some(inner.did()),
			ty::Adt(inner, _) if inner.is_phantom_data() => {}
			_ => return false,
		}
	}
	obj.is_some_and(|inner| represents(tcx, inner))
}

/// Whether `#[wasm_bindgen]`, the attribute, wrote `did`, by whatever path it is named.
fn from_macro(tcx: TyCtxt<'_>, did: DefId) -> bool {
	let expn = tcx.def_span(did).ctxt().outer_expn_data();
	matches!(expn.kind, ExpnKind::Macro(MacroKind::Attr, _))
		&& expn
			.macro_def_id
			.is_some_and(|m| tcx.crate_name(m.krate).as_str() == "wasm_bindgen_macro")
}

/// What of wasm-bindgen a call reaches in place of its MIR.
pub enum Glue<'tcx> {
	/// A function the macro made around an import.
	Import(Wrapper),
	/// What is its argument, once JS values are themselves: `JsValue`'s clone, `JsCast`'s
	/// unchecked casts, and `transmute_copy` between JS-represented types.
	Identity,
	/// The value of wasm-bindgen's `LazyCell`, an `rs.LazyCell`, made on first use.
	Force,
	/// `JsValue`'s `Drop`, which frees its slot of wasm-bindgen's heap, where JS collects
	/// the value itself.
	Nothing,
	/// `throw_str`, which throws an `Error` of its message, or `throw_val`, which throws
	/// its value.
	Throw { error: bool },
	/// `wbg_cast::<From, To>`, which converts as an import's argument and result do.
	Cast { from: Ty<'tcx>, to: Ty<'tcx> },
}

/// What of wasm-bindgen `instance` is, if a call of it maps rather than lowers from MIR.
pub fn glue<'tcx>(t: &Translator<'_, 'tcx>, instance: Instance<'tcx>) -> Option<Glue<'tcx>> {
	let InstanceKind::Item(did) = instance.def else {
		return None;
	};
	let tcx = t.tcx;
	let krate = tcx.crate_name(did.krate);
	let name = tcx.opt_item_name(did)?;
	let represented = |ty: Ty<'tcx>| matches!(ty.peel_refs().kind(), ty::Adt(adt, _) if represents(tcx, adt.did()));
	match (krate.as_str(), name.as_str()) {
		("core", "transmute_copy") => {
			// What wasm-bindgen's glue passes: a reference to a `ManuallyDrop` of the value.
			let src = instance.args.type_at(0);
			let src = match src.kind() {
				ty::Adt(adt, args) if tcx.is_lang_item(adt.did(), rustc_attr_ir::LangItem::ManuallyDrop) => {
					args.type_at(0)
				}
				_ => src,
			};
			return (represented(src) && represented(instance.args.type_at(1))).then_some(Glue::Identity);
		}
		_ if crate::lower::of_std(tcx, did.krate) => return None,
		("wasm_bindgen", "throw_str") => return Some(Glue::Throw { error: true }),
		("wasm_bindgen", "throw_val") => return Some(Glue::Throw { error: false }),
		("wasm_bindgen", "wbg_cast") => {
			return Some(Glue::Cast {
				from: instance.args.type_at(0),
				to: instance.args.type_at(1),
			});
		}
		_ => {}
	}
	// A trait's method, as the trait declares it or an impl gives it, and the type it is of.
	let of_trait = match tcx.trait_of_assoc(did) {
		Some(trait_id) => Some((trait_id, instance.args.type_at(0))),
		None => tcx.trait_impl_of_assoc(did).map(|imp| {
			let trait_ref = tcx.impl_trait_ref(imp).instantiate(tcx, instance.args).skip_norm_wip();
			(trait_ref.def_id, trait_ref.self_ty())
		}),
	};
	let lazy = |ty: Ty<'tcx>| matches!(ty.peel_refs().kind(), ty::Adt(adt, _) if is_lazy(tcx, adt.did()));
	if let Some((trait_id, self_ty)) = of_trait
		&& lazy(self_ty)
		&& tcx.item_name(trait_id).as_str() == "Deref"
	{
		return Some(Glue::Force);
	}
	if name.as_str() == "force"
		&& let Some(imp) = tcx.inherent_impl_of_assoc(did)
		&& lazy(tcx.type_of(imp).instantiate_identity().skip_norm_wip())
	{
		return Some(Glue::Force);
	}
	// What the macro implements between the types an import declares and `JsValue`, which
	// are one JS value: `From`, `AsRef`, `Deref`, and the like, of one JS value to another.
	if of_trait.is_some()
		&& tcx
			.trait_impl_of_assoc(did)
			.is_some_and(|imp| tcx.is_automatically_derived(imp))
		&& let sig = t.sig(did, instance.args)
		&& let [input] = sig.inputs()
		&& represented(*input)
		&& represented(sig.output())
	{
		return Some(Glue::Identity);
	}
	if let Some((trait_id, self_ty)) = of_trait
		&& represented(self_ty)
	{
		let trait_name = tcx.item_name(trait_id);
		let trait_krate = tcx.crate_name(trait_id.krate);
		match (trait_krate.as_str(), trait_name.as_str(), name.as_str()) {
			// `JsValue`'s, and what an import's type derives, which clones its `obj`.
			("core", "Clone", "clone")
				if is_js_value(tcx, self_ty.peel_refs())
					|| tcx
						.trait_impl_of_assoc(did)
						.is_some_and(|imp| tcx.is_automatically_derived(imp)) =>
			{
				return Some(Glue::Identity);
			}
			("core", "Drop", "drop") => return Some(Glue::Nothing),
			("wasm_bindgen", "JsCast", "unchecked_from_js" | "unchecked_from_js_ref") => return Some(Glue::Identity),
			_ => {}
		}
	}
	t.programs().wrapper(did).cloned().map(Glue::Import)
}

/// Whether `ty` is wasm-bindgen's `JsValue`.
fn is_js_value(tcx: TyCtxt<'_>, ty: Ty<'_>) -> bool {
	matches!(ty.kind(), ty::Adt(adt, _)
		if tcx.crate_name(adt.did().krate).as_str() == "wasm_bindgen" && tcx.item_name(adt.did()).as_str() == "JsValue")
}

/// What an import is, from the program that declares it.
#[derive(Debug)]
pub struct Import {
	pub shim: String,
	/// Where it is from, if it is not a global: custom JS, or wasm-bindgen's placeholder.
	pub module: Option<Module>,
	pub namespace: Option<Vec<String>>,
	pub kind: Kind,
	/// The prefixes the program lists for the class or global it names, tried in turn
	/// where the unprefixed one is not defined.
	pub vendor_prefixes: Vec<String>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Module {
	/// `module = "..."`: a local JS file.
	Named(String),
	/// `raw_module = "..."`.
	Raw(String),
	/// `inline_js = "..."`.
	Inline,
}

#[derive(Debug)]
pub enum Kind {
	Function(Function),
	/// A static's value, read once.
	Static {
		name: String,
	},
	/// A string constant.
	String(String),
	/// A type's `instanceof` check.
	Instanceof {
		class: String,
	},
}

#[derive(Debug)]
pub struct Function {
	/// Its JS name, a path of properties.
	pub name: String,
	pub catch: bool,
	pub variadic: bool,
	pub structural: bool,
	pub suspending: bool,
	pub generic_per_mono: bool,
	pub asyncness: bool,
	pub jspi: bool,
	pub method: Option<Method>,
}

#[derive(Debug)]
pub struct Method {
	pub class: String,
	pub kind: MethodKind,
}

#[derive(Debug)]
pub enum MethodKind {
	Constructor,
	Operation { is_static: bool, op: Op },
}

#[derive(Debug)]
pub enum Op {
	Regular,
	RegularThis,
	Getter(String),
	Setter(String),
	IndexingGetter,
	IndexingSetter,
	IndexingDeleter,
}

/// A string enum: its JS strings, in the order of its Rust variants, the last of which
/// stands for any other.
#[derive(Debug)]
pub struct StringEnum {
	pub values: Vec<String>,
	/// The module path of its invocation, where the schema records one.
	pub module_path: Option<String>,
}

/// What a wrapper stands for: its import, or why translation refuses it.
pub type Wrapper = Result<Rc<Import>, String>;

/// The programs of every crate that translates, read once.
#[derive(Default)]
pub struct Programs {
	/// Each function the macro made around an import, by the function.
	wrappers: FxHashMap<DefId, Wrapper>,
	/// Each crate's string enums, by name.
	enums: FxHashMap<(CrateNum, String), Vec<Rc<StringEnum>>>,
	/// The functions each exported function's shim calls, with the export's JS name.
	exports: rustc_data_structures::fx::FxHashSet<DefId>,
}

impl Programs {
	/// Read the programs of `crates` and the package's.
	pub fn read(tcx: TyCtxt<'_>, crates: impl IntoIterator<Item = CrateNum>) -> Self {
		let mut programs = Self::default();
		for krate in crates.into_iter().chain([LOCAL_CRATE]) {
			programs.read_crate(tcx, krate);
		}
		programs
	}

	/// The import the macro made `did` around, if it did.
	pub fn wrapper(&self, did: DefId) -> Option<&Wrapper> {
		self.wrappers.get(&did)
	}

	/// Whether `did` is a `#[wasm_bindgen]` export: the function an export's shim calls.
	pub fn is_export(&self, did: DefId) -> bool {
		self.exports.contains(&did)
	}

	/// The string enum `did` is, by its crate, name, and module.
	pub fn string_enum(&self, tcx: TyCtxt<'_>, did: DefId) -> Option<Rc<StringEnum>> {
		let found = self.enums.get(&(did.krate, tcx.item_name(did).to_string()))?;
		let module = tcx.def_path_str(tcx.parent(did));
		let module = match did.is_local() {
			true => format!("{}::{module}", tcx.crate_name(LOCAL_CRATE)),
			false => module,
		};
		let mut matching = found
			.iter()
			.filter(|e| e.module_path.as_ref().is_none_or(|path| *path == module));
		let first = matching.next()?;
		// Two of one name, which a schema without module paths can't tell apart.
		matching.next().is_none().then(|| first.clone())
	}

	fn read_crate(&mut self, tcx: TyCtxt<'_>, krate: CrateNum) {
		let ids: Vec<DefId> = match krate {
			LOCAL_CRATE => tcx.iter_local_def_id().map(|id| id.to_def_id()).collect(),
			_ => {
				let count = rustc_metadata::creader::CStore::from_tcx(tcx).num_def_ids_untracked(krate);
				(0..count)
					.map(|index| DefId {
						krate,
						index: DefIndex::from_usize(index),
					})
					.collect()
			}
		};
		let mut statics = Vec::new();
		// The functions declaring each `extern "C"` shim in wasm-bindgen's placeholder module.
		let mut shims: FxHashMap<String, DefId> = FxHashMap::default();
		let mut generated = Vec::new();
		let mut generic = Vec::new();
		for did in ids {
			match tcx.def_kind(did) {
				DefKind::Static { .. }
					if tcx
						.codegen_fn_attrs(did)
						.link_section
						.is_some_and(|s| s.as_str() == "__wasm_bindgen_unstable") =>
				{
					statics.push(did)
				}
				DefKind::Fn if tcx.is_foreign_item(did) && in_placeholder(tcx, did) => {
					if let Some(wrapper) = enclosing_fn(tcx, did) {
						shims.insert(tcx.item_name(did).to_string(), wrapper);
					}
				}
				DefKind::Fn if tcx.item_name(did).as_str().starts_with("__wasm_bindgen_generated_") => {
					generated.push(did)
				}
				// An import generic per monomorphization declares no shim, but a function the
				// CLI rewrites per instance, which describes the instance's signature.
				DefKind::Fn if tcx.item_name(did).as_str() == "breaks_if_inlined" => {
					if let parent = tcx.parent(did)
						&& matches!(tcx.def_kind(parent), DefKind::Fn | DefKind::AssocFn)
					{
						generic.push(parent);
					}
				}
				_ => {}
			}
		}
		if statics.is_empty() {
			return;
		}
		for wrapper in generic {
			let why = "`experimental_generic_mono`, which the CLI makes JS of per instance, as translation does not";
			self.wrappers.insert(wrapper, Err(why.into()));
		}
		let mut imports = Vec::new();
		let mut exported = false;
		for did in statics {
			if let Err(why) = self.read_static(tcx, did, &mut imports, &mut exported) {
				// Every import of the crate is refused, as none of its shapes can be trusted.
				let name = tcx.crate_name(krate);
				for wrapper in shims.into_values() {
					self.wrappers.insert(wrapper, Err(format!("`{name}`'s program: {why}")));
				}
				return;
			}
		}
		for import in imports {
			if let Some(wrapper) = shims.remove(&import.shim) {
				self.wrappers.insert(wrapper, Ok(Rc::new(import)));
			}
		}
		// A shim no program names, which no supported schema declares.
		for (shim, wrapper) in shims {
			self.wrappers.insert(
				wrapper,
				Err(format!("`{shim}`, which no program of its crate declares")),
			);
		}
		if exported {
			for shim in generated {
				self.exports.extend(exported_callees(tcx, shim));
			}
		}
	}

	/// Read the programs of the static `did` into `imports` and the crate's string enums,
	/// noting whether any exports.
	fn read_static(
		&mut self,
		tcx: TyCtxt<'_>,
		did: DefId,
		imports: &mut Vec<Import>,
		exported: &mut bool,
	) -> Result<(), String> {
		let alloc = tcx
			.eval_static_initializer(did)
			.map_err(|_| "a program static that does not evaluate".to_string())?;
		let alloc = alloc.inner();
		let bytes = alloc.inspect_with_uninit_and_ptr_outside_interpreter(0..alloc.len());
		for (schema, bytes) in decode::section(bytes)? {
			let program = decode::program(bytes, schema)?;
			*exported |= !program.exports.is_empty() || !program.structs.is_empty();
			// A type's vendor prefixes apply to its constructor and static methods.
			let prefixes: FxHashMap<&str, Vec<String>> = (program.imports.iter())
				.filter_map(|import| match &import.kind {
					decode::ImportKind::Type(t) if !t.vendor_prefixes.is_empty() => {
						Some((t.name, t.vendor_prefixes.iter().map(|p| p.to_string()).collect()))
					}
					_ => None,
				})
				.collect();
			for import in program.imports {
				let module = import.module.map(|module| match module {
					decode::ImportModule::Named(m) => Module::Named(m.into()),
					decode::ImportModule::RawNamed(m) => Module::Raw(m.into()),
					decode::ImportModule::Inline(_) => Module::Inline,
				});
				let (shim, kind, item) = match import.kind {
					decode::ImportKind::Function(f) => {
						let method = f.method.map(|m| Method {
							class: m.class.into(),
							kind: match m.kind {
								decode::MethodKind::Constructor => MethodKind::Constructor,
								decode::MethodKind::Operation(op) => MethodKind::Operation {
									is_static: op.is_static,
									op: match op.kind {
										decode::OperationKind::Regular => Op::Regular,
										decode::OperationKind::RegularThis => Op::RegularThis,
										decode::OperationKind::Getter(g) => Op::Getter(g.into()),
										decode::OperationKind::Setter(s) => Op::Setter(s.into()),
										decode::OperationKind::IndexingGetter => Op::IndexingGetter,
										decode::OperationKind::IndexingSetter => Op::IndexingSetter,
										decode::OperationKind::IndexingDeleter => Op::IndexingDeleter,
									},
								},
							},
						});
						// What the CLI resolves the vendor prefixes of: the class, or the function.
						let item = method
							.as_ref()
							.map_or(f.function.name, |m| m.class.as_str())
							.to_string();
						let function = Function {
							name: f.function.name.into(),
							catch: f.catch,
							variadic: f.variadic,
							structural: f.structural,
							suspending: f.suspending,
							generic_per_mono: f.generic_per_mono,
							asyncness: f.function.asyncness,
							jspi: f.function.jspi,
							method,
						};
						(f.shim, Kind::Function(function), item)
					}
					decode::ImportKind::Static(s) => (s.shim, Kind::Static { name: s.name.into() }, s.name.into()),
					decode::ImportKind::String(s) => (s.shim, Kind::String(s.string.into()), String::new()),
					decode::ImportKind::Type(t) => (
						t.instanceof_shim,
						Kind::Instanceof { class: t.name.into() },
						t.name.into(),
					),
					decode::ImportKind::Enum(e) => {
						let string_enum = StringEnum {
							values: e.variant_values.iter().map(|v| v.to_string()).collect(),
							module_path: program.module_path.as_ref().map(|p| p.0.to_string()),
						};
						(self.enums.entry((did.krate, e.name.to_string())).or_default()).push(Rc::new(string_enum));
						continue;
					}
					// A union has no shim: what converts it is refused where it converts.
					decode::ImportKind::DynamicUnion(_) => continue,
				};
				imports.push(Import {
					shim: shim.into(),
					module,
					namespace: import.js_namespace,
					kind,
					vendor_prefixes: prefixes.get(item.as_str()).cloned().unwrap_or_default(),
				});
			}
		}
		Ok(())
	}
}

/// The function the shim `did` is declared in: its `extern` block's, or for an import
/// generic per monomorphization, that of the block it is nested in.
fn enclosing_fn(tcx: TyCtxt<'_>, did: DefId) -> Option<DefId> {
	let mut at = tcx.opt_parent(did)?;
	loop {
		at = tcx.opt_parent(at)?;
		match tcx.def_kind(at) {
			DefKind::Fn | DefKind::AssocFn => return Some(at),
			DefKind::Mod | DefKind::Impl { .. } | DefKind::Trait => return None,
			_ => {}
		}
	}
}

/// Whether the foreign function `did` is imported from wasm-bindgen's placeholder module.
fn in_placeholder(tcx: TyCtxt<'_>, did: DefId) -> bool {
	tcx.wasm_import_module_map(did.krate)
		.get(&did)
		.is_some_and(|module| module == PLACEHOLDER)
}

/// The functions the export shim `shim` calls, of which the export is one: in its body,
/// or in a closure it passes on, as it does to catch an unwind.
fn exported_callees(tcx: TyCtxt<'_>, shim: DefId) -> Vec<DefId> {
	let mut callees = Vec::new();
	let mut bodies = vec![shim];
	while let Some(body) = bodies.pop() {
		if !tcx.is_mir_available(body) {
			continue;
		}
		let mir = tcx.optimized_mir(body);
		for block in mir.basic_blocks.iter() {
			let rustc_middle::mir::TerminatorKind::Call { func, .. } = &block.terminator().kind else {
				continue;
			};
			let ty::FnDef(callee, args) = *func.ty(&mir.local_decls, tcx).kind() else {
				continue;
			};
			bodies.extend(args.skip_binder().types().filter_map(|t| match t.kind() {
				ty::Closure(closure, _) => Some(*closure),
				_ => None,
			}));
			if tcx.crate_name(callee.krate).as_str() != "wasm_bindgen" && !crate::lower::of_std(tcx, callee.krate) {
				callees.push(callee);
			}
		}
	}
	callees
}
