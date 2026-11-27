//! MIR to Lean: each reached function's blocks become straight-line Lean statements
//! between branches, which [`crate::structure`] nests into a `do` block.
//!
//! The subset is value-style code: a shared reference is the value it points to, and a
//! mutable reference is the place it borrows, written back after each call it is passed
//! to. Borrowck's exclusivity makes both exact. Anything else is refused with its span.

use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};

use rustc_abi::{FieldIdx, VariantIdx};
use rustc_hir::def::DefKind;
use rustc_hir::def_id::DefId;
use rustc_hir::definitions::DefPathData;
use rustc_middle::mir::interpret::GlobalId;
use rustc_middle::mir::{
	self, AggregateKind, AssertKind, BasicBlock, BinOp, Body, BorrowKind, CastKind, Local, Operand, Place,
	ProjectionElem, Rvalue, StatementKind, TerminatorKind, UnOp, VarDebugInfoContents,
};
use rustc_middle::ty::adjustment::PointerCoercion;
use rustc_middle::ty::{self, CallerBoundsIterator as _, Ty, TyCtxt, TypeVisitableExt, TypingEnv};

type GenericArgsRef<'tcx> = ty::GenericArgs<'tcx>;
use rustc_span::Span;

use crate::structure::{self, Ex, Node, Op, Term, Var};

/// A construct outside the subset, at its span.
pub struct Refusal {
	pub span: Span,
	pub msg: String,
}

type R<T> = Result<T, Refusal>;

fn refuse<T>(span: Span, msg: impl Into<String>) -> R<T> {
	Err(Refusal { span, msg: msg.into() })
}

/// What a crate's translation found.
pub struct Extraction {
	/// Their Lean definitions.
	pub types: Vec<String>,
	/// The functions it translated, each after those it calls, as Lean definitions.
	pub fns: Vec<String>,
	/// How many functions it reached.
	pub reached: usize,
	pub refusals: Vec<Refusal>,
	/// The roots that named no function, which is an error.
	pub unmatched: Vec<String>,
}

/// Translate every function `roots` names in the local crate, and what they reach.
pub fn extract(tcx: TyCtxt<'_>, roots: &[String]) -> Extraction {
	let mut cx = Cx {
		tcx,
		adts: Vec::new(),
		adt_seen: HashSet::new(),
		adt_visiting: HashSet::new(),
		shapes: HashMap::new(),
		refusals: Vec::new(),
		recursive: HashSet::new(),
	};
	let mut start = Vec::new();
	let mut matched = vec![false; roots.len()];
	for def in tcx.hir_body_owners() {
		let did = def.to_def_id();
		if !matches!(tcx.def_kind(did), DefKind::Fn | DefKind::AssocFn) {
			continue;
		}
		let path = cx.rust_path(did);
		let mut hit = false;
		for (root, m) in roots.iter().zip(&mut matched) {
			if path == *root || path.starts_with(&format!("{root}::")) {
				*m = true;
				hit = true;
			}
		}
		if hit {
			start.push(did);
		}
	}
	// Callees first, so each caller knows which calls can fail.
	let order = cx.call_order(&start);
	let mut fns = Vec::new();
	for &did in &order {
		if cx.recursive.contains(&did) {
			cx.shapes.get_mut(&did).expect("ordered functions have shapes").failed = true;
			continue;
		}
		match cx.function(did) {
			Ok(text) => fns.push(text),
			Err(refusal) => {
				cx.shapes.get_mut(&did).expect("ordered functions have shapes").failed = true;
				cx.refusals.push(refusal);
			}
		}
	}
	let mut types = Vec::new();
	for did in cx.adts.clone() {
		match cx.adt_text(did) {
			Ok(text) => types.push(text),
			Err(refusal) => cx.refusals.push(refusal),
		}
	}
	Extraction {
		types,
		fns,
		reached: order.len(),
		refusals: cx.refusals,
		unmatched: roots
			.iter()
			.zip(&matched)
			.filter(|(_, m)| !**m)
			.map(|(r, _)| r.clone())
			.collect(),
	}
}

/// How a translated function is called.
struct Shape {
	/// Whether its Lean definition runs in `Rust.M`.
	monadic: bool,
	failed: bool,
}

pub(crate) struct Cx<'tcx> {
	pub tcx: TyCtxt<'tcx>,
	adts: Vec<DefId>,
	/// Types queued with every field checked.
	adt_seen: HashSet<DefId>,
	/// Types whose fields are being checked.
	adt_visiting: HashSet<DefId>,
	shapes: HashMap<DefId, Shape>,
	refusals: Vec<Refusal>,
	/// Functions that call themselves, which are refused.
	recursive: HashSet<DefId>,
}

impl<'tcx> Cx<'tcx> {
	/// The item's Rust path within its crate, as `--start-from` names it.
	fn rust_path(&self, did: DefId) -> String {
		let mut parts = self.module_path(did);
		if let Some(impl_id) = self.tcx.impl_of_assoc(did) {
			let self_ty = self.tcx.type_of(impl_id).instantiate_identity().skip_norm_wip();
			if let ty::Adt(def, _) = self_ty.kind() {
				// A method is named under its type, wherever its `impl` is written.
				if def.did().is_local() {
					parts = self.module_path(def.did());
				}
				parts.push(self.tcx.item_name(def.did()).to_string());
			}
		}
		parts.push(self.tcx.item_name(did).to_string());
		parts.join("::")
	}

	/// The modules the item is in, from the crate root.
	fn module_path(&self, did: DefId) -> Vec<String> {
		let path = self.tcx.def_path(did);
		let mut parts = Vec::new();
		for data in &path.data[..path.data.len().saturating_sub(1)] {
			match data.data {
				DefPathData::TypeNs(sym) => parts.push(sym.to_string()),
				DefPathData::Impl => break,
				_ => {}
			}
		}
		parts
	}

	/// The Lean name of a local type.
	pub fn adt_name(&self, did: DefId) -> String {
		let mut parts = self.module_path(did);
		parts.push(self.tcx.item_name(did).to_string());
		parts.iter().map(|p| structure::ident(p)).collect::<Vec<_>>().join(".")
	}

	/// The Lean name of a local function.
	pub fn fn_name(&self, did: DefId) -> String {
		let tcx = self.tcx;
		let mut name = match tcx.impl_of_assoc(did) {
			Some(impl_id) => {
				let self_ty = tcx.type_of(impl_id).instantiate_identity().skip_norm_wip();
				let mut base = match self_ty.kind() {
					ty::Adt(def, _) if def.did().is_local() => self.adt_name(def.did()),
					_ => self.module_path(did).join("."),
				};
				if tcx.trait_impl_of_assoc(did).is_some() {
					let trait_id = tcx
						.impl_trait_ref(impl_id)
						.instantiate_identity()
						.skip_norm_wip()
						.def_id;
					base = format!("{base}.{}", tcx.item_name(trait_id));
				}
				base
			}
			None => self.module_path(did).join("."),
		};
		if !name.is_empty() {
			name.push('.');
		}
		let item = tcx.item_name(did);
		name.push_str(&structure::ident(item.as_str()));
		// A structure's own names: its fields' projections and what Lean generates.
		// Or a name the emitted code uses unqualified, which a body in this namespace would
		// resolve to the function instead.
		let clash = ["decide", "id", "pure"].contains(&item.as_str())
			|| tcx.impl_of_assoc(did).is_some_and(|imp| {
				match tcx.type_of(imp).instantiate_identity().skip_norm_wip().kind() {
					ty::Adt(def, _) => {
						def.variants().iter().any(|v| {
							v.fields
								.iter()
								.any(|f| field_ident(f) == structure::ident(item.as_str()))
								|| v.name == item
						}) || ["mk", "rec", "recOn", "casesOn", "noConfusion", "ctorIdx"].contains(&item.as_str())
					}
					_ => false,
				}
			});
		if clash {
			name.push('\'');
		}
		name
	}

	/// The local functions reachable from `start`, each after its callees.
	fn call_order(&mut self, start: &[DefId]) -> Vec<DefId> {
		let mut order = Vec::new();
		let mut state: HashMap<DefId, bool> = HashMap::new();
		for &root in start {
			self.visit(root, &mut state, &mut order);
		}
		for &did in &order {
			self.shapes.insert(
				did,
				Shape {
					monadic: true,
					failed: false,
				},
			);
		}
		order
	}

	fn visit(&mut self, did: DefId, state: &mut HashMap<DefId, bool>, order: &mut Vec<DefId>) {
		match state.get(&did) {
			Some(true) => return,
			Some(false) => {
				let span = self.tcx.def_span(did);
				self.recursive.insert(did);
				self.refusals.push(Refusal {
					span,
					msg: "recursion is not supported".into(),
				});
				return;
			}
			None => {}
		}
		state.insert(did, false);
		for callee in self.callees(did) {
			self.visit(callee, state, order);
		}
		state.insert(did, true);
		order.push(did);
	}

	/// The local functions `did`'s body calls.
	fn callees(&self, did: DefId) -> Vec<DefId> {
		let tcx = self.tcx;
		let body = tcx.optimized_mir(did);
		let env = TypingEnv::post_analysis(tcx, did);
		let mut out = Vec::new();
		for bb in body.basic_blocks.iter() {
			if let TerminatorKind::Call { func, .. } = &bb.terminator().kind
				&& let ty::FnDef(callee, args) = *func.ty(body, tcx).kind()
				&& let args = args.skip_binder()
				&& let Some((local, _)) = resolve_local(tcx, env, callee, args)
			{
				out.push(local);
			}
		}
		out
	}

	/// Whether the type's `PartialEq` is structural equality, which Lean's derived
	/// `DecidableEq` is.
	fn structural_eq(&self, ty: Ty<'tcx>) -> bool {
		self.structural_eq_in(ty, &mut HashSet::new())
	}

	/// `structural_eq`, skipping types in `seen`, which are being checked already.
	fn structural_eq_in(&self, ty: Ty<'tcx>, seen: &mut HashSet<Ty<'tcx>>) -> bool {
		match ty.kind() {
			ty::Bool | ty::Int(_) | ty::Uint(_) | ty::Char | ty::Param(_) => true,
			ty::Tuple(tys) => tys.iter().all(|t| self.structural_eq_in(t, seen)),
			ty::Array(t, _) | ty::Slice(t) | ty::Ref(_, t, _) => self.structural_eq_in(*t, seen),
			ty::Adt(def, args) => {
				if !seen.insert(ty) {
					return true;
				}
				if def.did().is_local() {
					// A derived `PartialEq` calls each field's, which must be structural too.
					self.derives_partial_eq(def.did())
						&& def
							.all_fields()
							.all(|f| self.structural_eq_in(f.ty(self.tcx, *args).skip_norm_wip(), seen))
				} else {
					self.std_container(def.did(), args)
						.is_some_and(|mut tys| tys.all(|t| self.structural_eq_in(t, seen)))
				}
			}
			_ => false,
		}
	}

	/// Whether cloning a value of `ty` copies it, as the translation's value semantics assume.
	/// `Copy` alone is not enough: a `Copy` type's `Clone` may be written by hand.
	fn clone_copies(&self, ty: Ty<'tcx>) -> bool {
		self.clone_copies_in(ty, &mut HashSet::new())
	}

	/// `clone_copies`, refusing a type in `active`, the types being checked further up, since
	/// recursive types are refused anyway.
	fn clone_copies_in(&self, ty: Ty<'tcx>, active: &mut HashSet<Ty<'tcx>>) -> bool {
		match ty.kind() {
			ty::Bool | ty::Int(_) | ty::Uint(_) | ty::Char | ty::Ref(_, _, ty::Mutability::Not) => true,
			ty::Tuple(tys) => tys.iter().all(|t| self.clone_copies_in(t, active)),
			ty::Array(t, _) => self.clone_copies_in(*t, active),
			ty::Adt(def, args) => {
				if !active.insert(ty) {
					return false;
				}
				let copies = if def.did().is_local() {
					let lang = self.tcx.lang_items();
					// `#[derive(Clone, Copy)]` on a type without type parameters derives a `clone`
					// that copies, whatever the fields' `Clone` does, and marks it `TrivialClone`.
					if self.derives(def.did(), lang.trivial_clone_trait()) {
						true
					} else {
						self.derives(def.did(), lang.clone_trait())
							&& def
								.all_fields()
								.all(|f| self.clone_copies_in(f.ty(self.tcx, *args).skip_norm_wip(), active))
					}
				} else {
					self.std_container(def.did(), args)
						.is_some_and(|mut tys| tys.all(|t| self.clone_copies_in(t, active)))
				};
				// A type met again outside its own fields is repeated, not recursive.
				active.remove(&ty);
				copies
			}
			_ => false,
		}
	}

	/// The element types of a `std` container whose `Clone` and `PartialEq` are structural, or
	/// `None` for any other type. A `Vec`'s allocator is not an element: the only one modeled is
	/// `Global`, which holds no data, and any other's `Clone` may do anything.
	fn std_container(&self, did: DefId, args: &'tcx ty::GenericArgs<'tcx>) -> Option<impl Iterator<Item = Ty<'tcx>>> {
		let n = match self.std_name(did).as_str() {
			"Option" => 1,
			"Vec" if self.global_alloc(args) => 1,
			"Result" => 2,
			_ => return None,
		};
		Some(args.types().take(n))
	}

	/// Whether a `Vec`'s arguments name the `Global` allocator, the only one modeled.
	fn global_alloc(&self, args: &'tcx ty::GenericArgs<'tcx>) -> bool {
		matches!(args.type_at(1).kind(), ty::Adt(a, _) if self.std_name(a.did()) == "Global")
	}

	/// Whether the local type's `PartialEq` impl is derived.
	fn derives_partial_eq(&self, adt: DefId) -> bool {
		self.derives(adt, self.tcx.lang_items().eq_trait())
	}

	/// Whether the local type's impl of `tr` is derived.
	fn derives(&self, adt: DefId, tr: Option<DefId>) -> bool {
		let tcx = self.tcx;
		let Some(tr) = tr else {
			return false;
		};
		tcx.all_impls(tr).any(|imp| {
			matches!(tcx.type_of(imp).instantiate_identity().skip_norm_wip().kind(), ty::Adt(d, _) if d.did() == adt)
				&& tcx.is_automatically_derived(imp)
		})
	}

	/// The item's name if it is from `core` or `alloc`, whose models match it by name.
	fn std_name(&self, did: DefId) -> String {
		if !self.is_std(did) {
			return String::new();
		}
		self.tcx.item_name(did).to_string()
	}

	/// Whether the item is from the sysroot's `core` or `alloc`, found by their lang items, so a
	/// crate that only shares their names is not.
	fn is_std(&self, did: DefId) -> bool {
		let items = self.tcx.lang_items();
		[items.sized_trait(), items.owned_box()]
			.into_iter()
			.flatten()
			.any(|lang| lang.krate == did.krate)
	}

	/// The Lean type of `ty`, queueing each local type it names.
	pub fn ty(&mut self, ty: Ty<'tcx>, span: Span) -> R<String> {
		let tcx = self.tcx;
		Ok(match ty.kind() {
			ty::Bool => "Bool".into(),
			ty::Uint(u) => match u.bit_width().unwrap_or(self.pointer_bits()) {
				8 => "UInt8".into(),
				16 => "UInt16".into(),
				32 => "UInt32".into(),
				64 => "UInt64".into(),
				_ => return refuse(span, format!("`{ty}` has no Lean model")),
			},
			ty::Int(i) => match i.bit_width().unwrap_or(self.pointer_bits()) {
				8 => "Int8".into(),
				16 => "Int16".into(),
				32 => "Int32".into(),
				64 => "Int64".into(),
				_ => return refuse(span, format!("`{ty}` has no Lean model")),
			},
			ty::Tuple(tys) if tys.is_empty() => "Unit".into(),
			ty::Tuple(tys) => {
				let parts = tys.iter().map(|t| self.ty(t, span)).collect::<R<Vec<_>>>()?;
				format!("({})", parts.join(" × "))
			}
			ty::Array(elem, len) => format!("(Vector {} {})", self.ty(*elem, span)?, self.len(*len, span)?),
			ty::Slice(elem) => format!("(Array {})", self.ty(*elem, span)?),
			ty::Ref(_, inner, _) => self.ty(*inner, span)?,
			ty::Param(p) => structure::ident(p.name.as_str()),
			ty::Adt(def, args) if def.did().is_local() => {
				if def.is_union() {
					return refuse(span, format!("union `{ty}` is not supported"));
				}
				if tcx.adt_destructor(def.did()).is_some() {
					return refuse(span, format!("`{ty}` has a `Drop` impl, which is not supported"));
				}
				if !ty.is_freeze(tcx, TypingEnv::fully_monomorphized()) && !ty.has_param() {
					return refuse(span, format!("`{ty}` has interior mutability, which is not supported"));
				}
				self.queue_adt(def.did(), span)?;
				let mut out = self.adt_name(def.did());
				for arg in args.iter() {
					if let Some(t) = arg.as_type() {
						out = format!("{out} {}", self.ty(t, span)?);
					} else if let Some(c) = arg.as_const() {
						out = format!("{out} {}", self.const_value(c, span)?);
					}
				}
				if args.iter().any(|a| a.as_type().is_some() || a.as_const().is_some()) {
					out = format!("({out})");
				}
				out
			}
			ty::Adt(..) if !ty.is_freeze(tcx, TypingEnv::fully_monomorphized()) && !ty.has_param() => {
				return refuse(span, format!("`{ty}` has interior mutability, which is not supported"));
			}
			ty::Adt(def, args) => match self.std_name(def.did()).as_str() {
				"Option" => format!("(Option {})", self.ty(args.type_at(0), span)?),
				"Vec" if !self.global_alloc(args) => {
					return refuse(
						span,
						format!("`{ty}` has an allocator other than `Global`, which is not supported"),
					);
				}
				"Vec" => format!("(Array {})", self.ty(args.type_at(0), span)?),
				"Result" => format!(
					"(Except {} {})",
					self.ty(args.type_at(1), span)?,
					self.ty(args.type_at(0), span)?
				),
				"ControlFlow" => format!(
					"(Rust.ControlFlow {} {})",
					self.ty(args.type_at(0), span)?,
					self.ty(args.type_at(1), span)?
				),
				"Infallible" => "Empty".into(),
				_ => return refuse(span, format!("`{ty}` has no Lean model")),
			},
			ty::Never => "Empty".into(),
			ty::Closure(..) => return refuse(span, "a closure is not supported"),
			_ => return refuse(span, format!("`{ty}` has no Lean model")),
		})
	}

	fn pointer_bits(&self) -> u64 {
		self.tcx.data_layout.pointer_size().bits()
	}

	/// An array length as a Lean `Nat`.
	fn len(&self, len: ty::Const<'tcx>, span: Span) -> R<String> {
		match len.kind() {
			ty::ConstKind::Param(p) => Ok(format!("{}.toNat", structure::ident(p.name.as_str()))),
			_ => match len.try_to_target_usize(self.tcx) {
				Some(n) => Ok(n.to_string()),
				None => refuse(span, format!("array length `{len}` is not supported")),
			},
		}
	}

	/// A const generic argument as a Lean value.
	fn const_value(&self, c: ty::Const<'tcx>, span: Span) -> R<String> {
		match c.kind() {
			ty::ConstKind::Param(p) => Ok(structure::ident(p.name.as_str())),
			_ => match c.try_to_target_usize(self.tcx) {
				Some(n) => Ok(n.to_string()),
				None => refuse(span, format!("const argument `{c}` is not supported")),
			},
		}
	}

	fn queue_adt(&mut self, did: DefId, span: Span) -> R<()> {
		if self.adt_seen.contains(&did) {
			return Ok(());
		}
		// A type reached again while its fields are checked is recursive, directly or not.
		if !self.adt_visiting.insert(did) {
			return refuse(
				span,
				format!("recursive type `{}` is not supported", self.tcx.item_name(did)),
			);
		}
		// Only a type whose fields all pass is kept, so a refused one is refused at each use.
		let res = self.queue_fields(did);
		self.adt_visiting.remove(&did);
		if res.is_ok() {
			self.adt_seen.insert(did);
		}
		res
	}

	fn queue_fields(&mut self, did: DefId) -> R<()> {
		// Its fields' types first, so Lean sees each before its use.
		let tcx = self.tcx;
		let def = tcx.adt_def(did);
		let args = ty::GenericArgs::identity_for_item(tcx, did);
		for variant in def.variants() {
			for field in &variant.fields {
				let fty = field.ty(tcx, args).skip_norm_wip();
				self.ty(fty, tcx.def_span(field.did))?;
			}
		}
		self.adts.push(did);
		Ok(())
	}

	/// A local type's Lean definition: a `structure`, or an `inductive` for an enum.
	fn adt_text(&mut self, did: DefId) -> R<String> {
		let tcx = self.tcx;
		let def = tcx.adt_def(did);
		let args = ty::GenericArgs::identity_for_item(tcx, did);
		let mut params = String::new();
		for param in &tcx.generics_of(did).own_params {
			let name = structure::ident(param.name.as_str());
			match param.kind {
				ty::GenericParamDefKind::Lifetime => {}
				ty::GenericParamDefKind::Const { .. } => {
					let ty = tcx.type_of(param.def_id).instantiate_identity().skip_norm_wip();
					params += &format!(" ({name} : {})", self.ty(ty, tcx.def_span(param.def_id))?);
				}
				ty::GenericParamDefKind::Type { .. } => params += &format!(" ({name} : Type)"),
			}
		}
		let name = self.adt_name(did);
		let mut out = format!("/-- `{}` -/\n", self.rust_path(did));
		if def.is_struct() {
			out += &format!("structure {name}{params} where\n");
			for field in &def.non_enum_variant().fields {
				let fty = self.ty(field.ty(tcx, args).skip_norm_wip(), tcx.def_span(field.did))?;
				out += &format!("  {} : {fty}\n", field_ident(field));
			}
		} else {
			out += &format!("inductive {name}{params} where\n");
			for variant in def.variants() {
				out += &format!("  | {}", structure::ident(variant.name.as_str()));
				for field in &variant.fields {
					let fty = self.ty(field.ty(tcx, args).skip_norm_wip(), tcx.def_span(field.did))?;
					out += &format!(" ({} : {fty})", field_ident(field));
				}
				out += "\n";
			}
		}
		out += "deriving Repr, DecidableEq, Inhabited\n";
		Ok(out)
	}

	/// The Lean binders of `did`'s generics: `{N : UInt64}` and `{T : Type} [...]`.
	pub fn binders(&mut self, did: DefId) -> R<String> {
		let tcx = self.tcx;
		let generics = tcx.generics_of(did);
		let mut out = String::new();
		let mut params = Vec::new();
		let mut g = Some(generics);
		while let Some(gen_) = g {
			params.splice(0..0, gen_.own_params.iter().cloned());
			g = gen_.parent.map(|p| tcx.generics_of(p));
		}
		let eq = tcx.lang_items().eq_trait();
		let env = tcx.param_env(did);
		for param in params {
			let name = structure::ident(param.name.as_str());
			match param.kind {
				ty::GenericParamDefKind::Lifetime => {}
				ty::GenericParamDefKind::Const { .. } => {
					let ty = tcx.type_of(param.def_id).instantiate_identity().skip_norm_wip();
					out += &format!(" {{{name} : {}}}", self.ty(ty, tcx.def_span(param.def_id))?);
				}
				ty::GenericParamDefKind::Type { .. } => {
					out += &format!(" {{{name} : Type}} [Inhabited {name}]");
					let bounded = env.caller_bounds().all_clauses(tcx).iter().any(|p| {
						p.as_trait_clause().is_some_and(|t| {
							Some(t.def_id()) == eq
								&& matches!(t.self_ty().skip_binder().kind(), ty::Param(pp) if pp.index == param.index)
						})
					});
					if bounded {
						out += &format!(" [DecidableEq {name}]");
					}
				}
			}
		}
		Ok(out)
	}

	/// Translate one function to its Lean definitions.
	fn function(&mut self, did: DefId) -> R<String> {
		let tcx = self.tcx;
		let body = tcx.optimized_mir(did);
		let span = tcx.def_span(did);
		if body.coroutine.is_some() {
			return refuse(span, "async functions and coroutines are not supported");
		}
		let mut fx = Fx::new(self, did, body)?;
		let nodes = fx.nodes()?;
		let sig = fx.signature()?;
		let lowered = structure::Function {
			name: fx.cx.fn_name(did),
			binders: fx.cx.binders(did)?,
			params: sig.params,
			ret: sig.ret,
			ret_value: sig.ret_value,
			vars: fx.vars,
			nodes,
		};
		let (text, monadic) = structure::emit(lowered).map_err(|msg| Refusal { span, msg })?;
		self.shapes
			.get_mut(&did)
			.expect("ordered functions have shapes")
			.monadic = monadic;
		Ok(text)
	}
}

/// A field's Lean name: a tuple field `0` is `x0`.
fn field_ident(field: &ty::FieldDef) -> String {
	match field.name.as_str() {
		n if n.chars().all(|c| c.is_ascii_digit()) => format!("x{n}"),
		n => structure::ident(n),
	}
}

/// The local function a call of `callee` with `args` runs, if it resolves to one.
fn resolve_local<'tcx>(
	tcx: TyCtxt<'tcx>,
	env: TypingEnv<'tcx>,
	callee: DefId,
	args: GenericArgsRef<'tcx>,
) -> Option<(DefId, GenericArgsRef<'tcx>)> {
	let instance = ty::Instance::try_resolve(tcx, env, callee, args).ok().flatten()?;
	let did = instance.def_id();
	(did.is_local()
		&& matches!(instance.def, ty::InstanceKind::Item(_))
		&& matches!(tcx.def_kind(did), DefKind::Fn | DefKind::AssocFn)
		&& !tcx
			.impl_of_assoc(did)
			.is_some_and(|imp| tcx.is_automatically_derived(imp)))
	.then_some((did, instance.args))
}

/// What a mutable reference local stands for.
#[derive(Clone, Copy)]
enum Mutable<'tcx> {
	/// A parameter: its pointee is a Lean variable, returned at the end.
	Param,
	/// A borrow of this place, written to directly.
	Borrow(Place<'tcx>),
}

/// A function's Lean signature.
struct Signature {
	params: Vec<(String, String)>,
	ret: String,
	/// What the function returns: its result, then each mutable parameter's final value.
	ret_value: Ex,
}

/// One function's lowering.
struct Fx<'a, 'tcx> {
	cx: &'a mut Cx<'tcx>,
	body: &'tcx Body<'tcx>,
	env: TypingEnv<'tcx>,
	vars: Vec<Var>,
	/// Each MIR local's variable, if it has one.
	local_var: Vec<Option<usize>>,
	mutable: HashMap<Local, Mutable<'tcx>>,
	/// How many times each local is read.
	uses: Vec<usize>,
	/// How many times each local is assigned whole.
	defs: Vec<usize>,
	/// How many times each local is written through a projection.
	partial: Vec<usize>,
	/// How many times each local is read through a projection.
	projected: Vec<usize>,
	/// Temporaries that copy a local that never changes, which reads of them read.
	copies: HashMap<Local, Local>,
	/// Checked operations' locals once asserted, which hold the result alone.
	asserted: HashSet<Local>,
	/// The node each block lowers into.
	node_of: Vec<Option<usize>>,
	/// The variables a match on a place's variant binds its fields to.
	pattern: HashMap<(Place<'tcx>, VariantIdx), Vec<usize>>,
	/// Locals holding a place's discriminant, which only a switch reads.
	discr: HashMap<Local, Place<'tcx>>,
	/// Locals holding `(result, overflowed)` of a checked operation, until its assert.
	checked: HashMap<Local, (BinOp, Ex, Ex)>,
	/// Temporaries whose one read follows their one write in the same node, by value.
	pending: HashMap<Local, Ex>,
	/// The node being lowered, and the place writes so far in it.
	written: HashSet<usize>,
}

impl<'a, 'tcx> Fx<'a, 'tcx> {
	fn new(cx: &'a mut Cx<'tcx>, did: DefId, body: &'tcx Body<'tcx>) -> R<Self> {
		let tcx = cx.tcx;
		let n = body.local_decls.len();
		let mut fx = Fx {
			cx,
			body,
			env: TypingEnv::post_analysis(tcx, did),
			vars: Vec::new(),
			local_var: vec![None; n],
			mutable: HashMap::new(),
			uses: vec![0; n],
			defs: vec![0; n],
			partial: vec![0; n],
			projected: vec![0; n],
			copies: HashMap::new(),
			asserted: HashSet::new(),
			node_of: vec![None; body.basic_blocks.len()],
			pattern: HashMap::new(),
			discr: HashMap::new(),
			checked: HashMap::new(),
			pending: HashMap::new(),
			written: HashSet::new(),
		};
		fx.count();
		fx.find_mutable()?;
		fx.find_copies();
		fx.make_vars()?;
		Ok(fx)
	}

	/// Count each local's reads and writes.
	fn count(&mut self) {
		struct Counter<'b> {
			uses: &'b mut Vec<usize>,
			defs: &'b mut Vec<usize>,
			partial: &'b mut Vec<usize>,
			projected: &'b mut Vec<usize>,
		}
		impl<'tcx> mir::visit::Visitor<'tcx> for Counter<'_> {
			fn visit_local(&mut self, local: Local, ctx: mir::visit::PlaceContext, _: mir::Location) {
				use mir::visit::{MutatingUseContext, NonUseContext, PlaceContext};
				match ctx {
					PlaceContext::NonUse(
						NonUseContext::StorageDead | NonUseContext::StorageLive | NonUseContext::VarDebugInfo,
					) => {}
					PlaceContext::MutatingUse(MutatingUseContext::Store | MutatingUseContext::Call) => {
						self.defs[local.as_usize()] += 1
					}
					PlaceContext::MutatingUse(MutatingUseContext::Drop) | PlaceContext::NonUse(_) => {}
					_ => self.uses[local.as_usize()] += 1,
				}
			}
			fn visit_place(&mut self, place: &Place<'tcx>, ctx: mir::visit::PlaceContext, loc: mir::Location) {
				use mir::visit::{MutatingUseContext, PlaceContext};
				// A write through a projection reads the rest of its base.
				if !place.projection.is_empty()
					&& matches!(
						ctx,
						PlaceContext::MutatingUse(MutatingUseContext::Store | MutatingUseContext::Call)
					) {
					self.uses[place.local.as_usize()] += 1;
					self.partial[place.local.as_usize()] += 1;
					for elem in place.projection {
						if let ProjectionElem::Index(i) = elem {
							self.uses[i.as_usize()] += 1;
						}
					}
					return;
				}
				if !place.projection.is_empty() {
					self.projected[place.local.as_usize()] += 1;
				}
				self.super_place(place, ctx, loc);
			}
		}
		let mut counter = Counter {
			uses: &mut self.uses,
			defs: &mut self.defs,
			partial: &mut self.partial,
			projected: &mut self.projected,
		};
		mir::visit::Visitor::visit_body(&mut counter, self.body);
		// An overflow assert reads the flag, which no variable holds.
		for bb in self.body.basic_blocks.iter() {
			if let TerminatorKind::Assert {
				cond: Operand::Move(p) | Operand::Copy(p),
				msg,
				..
			} = &bb.terminator().kind
				&& matches!(**msg, AssertKind::Overflow(..))
			{
				self.uses[p.local.as_usize()] -= 1;
			}
		}
	}

	/// Find what each mutable reference local stands for.
	fn find_mutable(&mut self) -> R<()> {
		let body = self.body;
		for (local, decl) in body.local_decls.iter_enumerated() {
			if !matches!(decl.ty.kind(), ty::Ref(_, _, ty::Mutability::Mut)) {
				continue;
			}
			if local.as_usize() >= 1 && local.as_usize() <= body.arg_count {
				self.mutable.insert(local, Mutable::Param);
				continue;
			}
			if local == mir::RETURN_PLACE {
				return refuse(decl.source_info.span, "returning a mutable reference is not supported");
			}
		}
		for bb in body.basic_blocks.iter() {
			for stmt in &bb.statements {
				let StatementKind::Assign(assign) = &stmt.kind else {
					continue;
				};
				let (place, rvalue) = &**assign;
				if !matches!(
					place.ty(body, self.cx.tcx).ty.kind(),
					ty::Ref(_, _, ty::Mutability::Mut)
				) {
					continue;
				}
				if !place.projection.is_empty() {
					return refuse(stmt.source_info.span, "storing a mutable reference is not supported");
				}
				let target = match rvalue {
					Rvalue::Ref(_, BorrowKind::Mut { .. }, borrowed) => *borrowed,
					Rvalue::Use(Operand::Move(from) | Operand::Copy(from), _) if from.projection.is_empty() => {
						Place::from(from.local).project_deeper(&[ProjectionElem::Deref], self.cx.tcx)
					}
					_ => return refuse(stmt.source_info.span, "this mutable reference is not supported"),
				};
				if self.defs[place.local.as_usize()] != 1 {
					return refuse(
						stmt.source_info.span,
						"a mutable reference assigned twice is not supported",
					);
				}
				for elem in target.projection {
					if let ProjectionElem::Index(i) = elem
						&& self.defs[i.as_usize()] > 1
					{
						return refuse(
							stmt.source_info.span,
							"borrowing at an index that changes is not supported",
						);
					}
				}
				self.mutable.insert(place.local, Mutable::Borrow(target));
			}
			if let TerminatorKind::Call {
				destination, fn_span, ..
			} = &bb.terminator().kind
				&& matches!(
					body.local_decls[destination.local].ty.kind(),
					ty::Ref(_, _, ty::Mutability::Mut)
				) {
				return refuse(*fn_span, "a call returning a mutable reference is not supported");
			}
		}
		Ok(())
	}

	/// Find temporaries that copy a local that never changes once set.
	fn find_copies(&mut self) {
		for bb in self.body.basic_blocks.iter() {
			for stmt in &bb.statements {
				let StatementKind::Assign(assign) = &stmt.kind else {
					continue;
				};
				let (place, rvalue) = &**assign;
				let Rvalue::Use(Operand::Copy(from) | Operand::Move(from), _) = rvalue else {
					continue;
				};
				let (t, l) = (place.local, from.local);
				if place.projection.is_empty()
					&& from.projection.is_empty()
					&& t.as_usize() > self.body.arg_count
					&& self.defs[t.as_usize()] == 1
					&& self.partial[t.as_usize()] == 0
					&& self.defs[l.as_usize()] <= 1
					// A parameter arrives set, so any assignment changes it.
					&& (l.as_usize() > self.body.arg_count || self.defs[l.as_usize()] == 0)
					// Nor may a mutable borrow write either, since the copy is a separate value.
					&& !self.mutable.values().any(|m| matches!(m, Mutable::Borrow(b) if b.local == l || b.local == t))
					&& self.partial[l.as_usize()] == 0
					&& !self.mutable.contains_key(&l)
					&& !self.mutable.contains_key(&t)
					&& !self
						.body
						.var_debug_info
						.iter()
						.any(|i| matches!(i.value, VarDebugInfoContents::Place(p) if p.local == t))
				{
					self.copies.insert(t, l);
				}
			}
		}
		// A copy of a copy reads the first.
		let keys: Vec<Local> = self.copies.keys().copied().collect();
		for k in keys {
			let mut l = self.copies[&k];
			let mut hops = 0;
			while let Some(&next) = self.copies.get(&l) {
				l = next;
				hops += 1;
				if hops > self.copies.len() {
					break;
				}
			}
			self.copies.insert(k, l);
		}
	}

	/// Name a variable for each local that holds a value.
	fn make_vars(&mut self) -> R<()> {
		let body = self.body;
		let mut names: Vec<Option<String>> = vec![None; body.local_decls.len()];
		for info in &body.var_debug_info {
			if let VarDebugInfoContents::Place(p) = info.value
				&& p.projection.is_empty()
				&& names[p.local.as_usize()].is_none()
			{
				names[p.local.as_usize()] = Some(info.name.to_string());
			}
		}
		for (local, decl) in body.local_decls.iter_enumerated() {
			let i = local.as_usize();
			if matches!(self.mutable.get(&local), Some(Mutable::Borrow(_))) || self.copies.contains_key(&local) {
				continue;
			}
			let (name, user) = match (&names[i], i) {
				(Some(n), _) => (n.clone(), true),
				(None, 0) => ("ret".into(), false),
				(None, i) if i <= body.arg_count => (format!("arg{i}"), true),
				(None, i) => (format!("t{i}"), false),
			};
			let ty = match decl.ty.kind() {
				ty::Ref(_, inner, ty::Mutability::Mut) => *inner,
				_ => decl.ty,
			};
			// A checked operation's local holds its result alone.
			let ty = match ty.kind() {
				ty::Tuple(tys) if tys.len() == 2 && tys[1].is_bool() && self.is_checked(local) => tys[0],
				_ => ty,
			};
			let lean_ty = match self.cx.ty(ty, decl.source_info.span) {
				Ok(t) => t,
				// A local only a refused construct uses is refused there, with a better span.
				Err(_) if !user && i > body.arg_count => "Unit".into(),
				Err(err) => return Err(err),
			};
			self.local_var[i] = Some(self.var(&name, lean_ty, user));
		}
		Ok(())
	}

	fn is_checked(&self, local: Local) -> bool {
		self.body.basic_blocks.iter().any(|bb| {
			bb.statements.iter().any(|s| {
				matches!(&s.kind, StatementKind::Assign(a) if a.0.local == local && a.0.projection.is_empty()
					&& matches!(a.1, Rvalue::BinaryOp(op, _) if op.overflowing_to_wrapping().is_some()))
			})
		})
	}

	fn var(&mut self, name: &str, ty: String, user: bool) -> usize {
		let base = structure::ident(name);
		let mut unique = base.clone();
		let mut n = 1;
		while self.vars.iter().any(|v| v.name == unique) {
			unique = format!("{base}{n}");
			n += 1;
		}
		self.vars.push(Var { name: unique, ty, user });
		self.vars.len() - 1
	}

	fn signature(&mut self) -> R<Signature> {
		let body = self.body;
		let mut params = Vec::new();
		let mut mutated = Vec::new();
		for local in body.args_iter() {
			let v = self.local_var[local.as_usize()].expect("parameters have variables");
			params.push((self.vars[v].name.clone(), self.vars[v].ty.clone()));
			if matches!(self.mutable.get(&local), Some(Mutable::Param)) {
				mutated.push(v);
			}
		}
		let ret_var = self.local_var[0].expect("the return place has a variable");
		let ret_unit = body.local_decls[mir::RETURN_PLACE].ty.is_unit();
		let mut tys = Vec::new();
		let mut vals = Vec::new();
		if !ret_unit || mutated.is_empty() {
			tys.push(self.vars[ret_var].ty.clone());
			vals.push(ret_var);
		}
		for &v in &mutated {
			tys.push(self.vars[v].ty.clone());
			vals.push(v);
		}
		let ret = if tys.len() == 1 {
			tys[0].clone()
		} else {
			format!("({})", tys.join(" × "))
		};
		let text = if vals.len() == 1 {
			self.vars[vals[0]].name.clone()
		} else {
			format!(
				"({})",
				vals.iter()
					.map(|&v| self.vars[v].name.clone())
					.collect::<Vec<_>>()
					.join(", ")
			)
		};
		Ok(Signature {
			params,
			ret,
			ret_value: Ex {
				text,
				atom: true,
				monadic: false,
				reads: vals.into_iter().collect(),
				prop: None,
				lit: None,
			},
		})
	}

	/// The blocks reachable from the entry without unwinding, as nodes: chains of blocks
	/// each the only successor of the last.
	fn nodes(&mut self) -> R<Vec<Node>> {
		let body = self.body;
		let mut preds = vec![0usize; body.basic_blocks.len()];
		let mut reachable = vec![false; body.basic_blocks.len()];
		let mut work = vec![mir::START_BLOCK];
		reachable[0] = true;
		while let Some(bb) = work.pop() {
			for succ in successors(body, bb) {
				preds[succ.as_usize()] += 1;
				if !reachable[succ.as_usize()] {
					reachable[succ.as_usize()] = true;
					work.push(succ);
				}
			}
		}
		// Chain heads: the entry, and every block not the one straight successor of another.
		let mut chains: Vec<Vec<BasicBlock>> = Vec::new();
		let mut in_chain = vec![false; body.basic_blocks.len()];
		let mut heads: VecDeque<BasicBlock> = VecDeque::from([mir::START_BLOCK]);
		while let Some(head) = heads.pop_front() {
			if in_chain[head.as_usize()] {
				continue;
			}
			let mut chain = vec![head];
			in_chain[head.as_usize()] = true;
			let mut cur = head;
			loop {
				match straight(body, cur) {
					Some(next)
						if preds[next.as_usize()] == 1 && next != mir::START_BLOCK && !in_chain[next.as_usize()] =>
					{
						chain.push(next);
						in_chain[next.as_usize()] = true;
						cur = next;
					}
					_ => break,
				}
			}
			for succ in successors(body, cur) {
				if !in_chain[succ.as_usize()] {
					heads.push_back(succ);
				}
			}
			let idx = chains.len();
			for bb in &chain {
				self.node_of[bb.as_usize()] = Some(idx);
			}
			chains.push(chain);
		}
		let mut nodes = Vec::new();
		for chain in &chains {
			nodes.push(self.node(chain)?);
		}
		Ok(nodes)
	}

	fn node(&mut self, chain: &[BasicBlock]) -> R<Node> {
		self.pending.clear();
		self.written.clear();
		let mut ops = Vec::new();
		let last = *chain.last().expect("chains are not empty");
		for &bb in chain {
			let data = &self.body.basic_blocks[bb];
			for stmt in &data.statements {
				self.statement(stmt, &mut ops)?;
			}
			if bb != last {
				self.straight_terminator(bb, &mut ops)?;
			}
		}
		let term = self.terminator(last, &mut ops)?;
		self.flush_all(&mut ops);
		Ok(Node { ops, term })
	}

	fn target(&self, bb: BasicBlock) -> usize {
		self.node_of[bb.as_usize()].expect("reachable blocks have nodes")
	}

	/// Emit every pending temporary, keeping its value: something it reads is about to change.
	fn flush(&mut self, ops: &mut Vec<Op>, when: impl Fn(&Ex) -> bool) {
		let mut ready: Vec<Local> = self.pending.iter().filter(|(_, e)| when(e)).map(|(l, _)| *l).collect();
		ready.sort();
		for local in ready {
			let ex = self.pending.remove(&local).expect("listed");
			let var = self.local_var[local.as_usize()].expect("temporaries have variables");
			push_set(ops, var, ex);
		}
	}

	fn flush_all(&mut self, ops: &mut Vec<Op>) {
		self.flush(ops, |_| true);
	}

	/// Before `ops` gets an operation that writes `var` or has an effect, emit the pending
	/// values that read `var` or have effects of their own.
	fn before(&mut self, ops: &mut Vec<Op>, var: Option<usize>, effect: bool) {
		self.flush(ops, |e| {
			var.is_some_and(|v| e.reads.contains(&v)) || (effect && e.monadic)
		});
	}

	fn inlinable(&self, local: Local) -> bool {
		let i = local.as_usize();
		i > self.body.arg_count
			&& self.local_var[i].is_some_and(|v| !self.vars[v].user)
			&& self.defs[i] == 1
			&& self.uses[i] == 1
			&& self.projected[i] == 0
			// A write through a mutable borrow reads and sets the variable itself.
			&& !self.mutable.values().any(|m| matches!(m, Mutable::Borrow(b) if b.local == local))
	}

	fn statement(&mut self, stmt: &mir::Statement<'tcx>, ops: &mut Vec<Op>) -> R<()> {
		let span = stmt.source_info.span;
		match &stmt.kind {
			StatementKind::Assign(assign) => {
				let (place, rvalue) = &**assign;
				if place.projection.is_empty() {
					if matches!(self.mutable.get(&place.local), Some(Mutable::Borrow(_)))
						|| self.copies.contains_key(&place.local)
					{
						return Ok(());
					}
					match rvalue {
						Rvalue::Discriminant(p) => {
							self.discr.insert(place.local, *p);
							return Ok(());
						}
						Rvalue::BinaryOp(op, ops2) if op.overflowing_to_wrapping().is_some() => {
							let a = self.operand(&ops2.0, span)?;
							let b = self.operand(&ops2.1, span)?;
							self.checked
								.insert(place.local, (op.overflowing_to_wrapping().expect("checked"), a, b));
							return Ok(());
						}
						_ => {}
					}
				}
				let value = self.rvalue(rvalue, place.ty(self.body, self.cx.tcx).ty, span)?;
				self.assign(*place, value, span, ops)
			}
			StatementKind::StorageLive(_)
			| StatementKind::StorageDead(_)
			| StatementKind::Nop
			| StatementKind::FakeRead(_)
			| StatementKind::AscribeUserType(..)
			| StatementKind::PlaceMention(_)
			| StatementKind::Coverage(_)
			| StatementKind::ConstEvalCounter
			| StatementKind::BackwardIncompatibleDropHint { .. } => Ok(()),
			_ => refuse(span, format!("statement `{stmt:?}` is not supported")),
		}
	}

	/// Write `value` to `place`, or hold it for the temporary's one read.
	fn assign(&mut self, place: Place<'tcx>, value: Ex, span: Span, ops: &mut Vec<Op>) -> R<()> {
		if place.projection.is_empty() && self.inlinable(place.local) {
			// One pending effect at a time keeps effects in order.
			if value.monadic {
				self.flush(ops, |e| e.monadic);
			}
			self.pending.insert(place.local, value);
			return Ok(());
		}
		let (var, update) = self.write(place, value, span)?;
		self.before(ops, Some(var), update.monadic);
		push_set(ops, var, update);
		self.written.insert(var);
		Ok(())
	}

	/// The variable a write to `place` changes, and its value after.
	fn write(&mut self, place: Place<'tcx>, value: Ex, span: Span) -> R<(usize, Ex)> {
		let (base, projs, base_ty) = self.resolve(place, span, true)?;
		let mut steps: Vec<(Ex, Proj)> = Vec::new();
		let mut cur = var_ex(&self.vars, base);
		let mut ty = base_ty;
		for proj in projs {
			let next_ty = self.proj_ty(ty, &proj);
			steps.push((cur.clone(), proj.clone()));
			cur = project(&cur, &proj);
			ty = next_ty;
		}
		let mut new = value;
		for (container, proj) in steps.into_iter().rev() {
			new = update(&container, &proj, new);
		}
		Ok((base, new))
	}

	fn proj_ty(&self, ty: Ty<'tcx>, _proj: &Proj) -> Ty<'tcx> {
		ty
	}

	/// A place as a variable and the projections from it. A write may not go through a
	/// variant's field.
	fn resolve(&mut self, place: Place<'tcx>, span: Span, write: bool) -> R<(usize, Vec<Proj>, Ty<'tcx>)> {
		let tcx = self.cx.tcx;
		let mut projs = Vec::new();
		let (mut base, mut rest): (usize, &[mir::PlaceElem<'tcx>]) = match self.mutable.get(&place.local).copied() {
			Some(Mutable::Borrow(target)) => {
				let [ProjectionElem::Deref, rest @ ..] = place.projection.as_slice() else {
					return refuse(span, "a mutable reference used as a value is not supported");
				};
				let (b, p, _) = self.resolve(target, span, write)?;
				projs = p;
				(b, rest)
			}
			Some(Mutable::Param) => {
				let [ProjectionElem::Deref, rest @ ..] = place.projection.as_slice() else {
					return refuse(span, "a mutable reference used as a value is not supported");
				};
				(
					self.local_var[place.local.as_usize()].expect("parameters have variables"),
					rest,
				)
			}
			None => {
				let local = self.copies.get(&place.local).copied().unwrap_or(place.local);
				(
					self.local_var[local.as_usize()].expect("value locals have variables"),
					place.projection.as_slice(),
				)
			}
		};
		let mut ty = place_ty(self.body, tcx, place, place.projection.len() - rest.len());
		while let [elem, tail @ ..] = rest {
			rest = tail;
			match *elem {
				ProjectionElem::Deref => match ty.ty.kind() {
					ty::Ref(_, _, ty::Mutability::Not) => {}
					_ => return refuse(span, format!("dereferencing `{}` is not supported", ty.ty)),
				},
				ProjectionElem::Field(f, _) => projs.push(self.field(ty.ty, ty.variant_index, f, span)?),
				ProjectionElem::Downcast(_, v) => {
					if write {
						return refuse(span, "writing a variant's field in place is not supported");
					}
					let [ProjectionElem::Field(f, _), tail2 @ ..] = rest else {
						return refuse(span, "a variant read without a field is not supported");
					};
					let consumed = place.projection.len() - rest.len() - 1;
					let prefix = Place {
						local: place.local,
						projection: tcx.mk_place_elems(&place.projection[..consumed]),
					};
					let Some(vars) = self.pattern.get(&(prefix, v)) else {
						return refuse(span, "a variant's field read away from its match is not supported");
					};
					base = vars[f.as_usize()];
					projs.clear();
					if self.written.contains(&base) {
						return refuse(span, "a variant's field read after its place changed is not supported");
					}
					rest = tail2;
					ty = place_ty(self.body, tcx, place, place.projection.len() - rest.len());
					continue;
				}
				ProjectionElem::Index(i) => {
					let idx = self.read_local(i, span)?;
					projs.push(Proj::Index(idx));
				}
				ProjectionElem::ConstantIndex {
					offset,
					from_end: false,
					..
				} => projs.push(Proj::Index(lit(offset.to_string(), "UInt64"))),
				_ => return refuse(span, format!("place `{place:?}` is not supported")),
			}
			ty = ty.projection_ty(tcx, *elem);
		}
		Ok((base, projs, ty.ty))
	}

	/// A field projection from a value of type `ty`.
	fn field(&mut self, ty: Ty<'tcx>, variant: Option<VariantIdx>, f: FieldIdx, span: Span) -> R<Proj> {
		match ty.kind() {
			ty::Tuple(tys) => Ok(Proj::Tuple(f.as_usize(), tys.len())),
			ty::Adt(def, _) if def.is_struct() && variant.is_none() => {
				Ok(Proj::Field(field_ident(&def.non_enum_variant().fields[f])))
			}
			_ => refuse(span, format!("a field of `{ty}` is not supported")),
		}
	}

	fn read_place(&mut self, place: Place<'tcx>, span: Span) -> R<Ex> {
		if place.projection.is_empty() {
			return self.read_local(place.local, span);
		}
		if self.asserted.contains(&place.local)
			&& let [ProjectionElem::Field(f, _)] = place.projection.as_slice()
			&& f.as_usize() == 0
		{
			return self.read_local(place.local, span);
		}
		let (base, projs, _) = self.resolve(place, span, false)?;
		let mut ex = var_ex(&self.vars, base);
		for proj in projs {
			ex = project(&ex, &proj);
		}
		Ok(ex)
	}

	fn read_local(&mut self, local: Local, span: Span) -> R<Ex> {
		let local = self.copies.get(&local).copied().unwrap_or(local);
		if self.mutable.contains_key(&local) {
			return refuse(span, "a mutable reference used as a value is not supported");
		}
		if let Some(ex) = self.pending.remove(&local) {
			return Ok(ex);
		}
		if let Some((op, a, b)) = self.checked.get(&local) {
			let _ = (op, a, b);
			return refuse(span, "an overflow flag read outside its assert is not supported");
		}
		let var = self.local_var[local.as_usize()].expect("value locals have variables");
		Ok(var_ex(&self.vars, var))
	}

	fn operand(&mut self, op: &Operand<'tcx>, span: Span) -> R<Ex> {
		match op {
			Operand::Copy(p) | Operand::Move(p) => {
				// `(result, overflowed).0` after its assert.
				if let [ProjectionElem::Field(f, _)] = p.projection.as_slice()
					&& f.as_usize() == 0
					&& self.checked.contains_key(&p.local)
				{
					return refuse(span, "a checked result read before its assert is not supported");
				}
				self.read_place(*p, span)
			}
			Operand::Constant(c) => self.constant(c, span),
			Operand::RuntimeChecks(_) => refuse(span, "runtime check intrinsics are not supported"),
		}
	}

	fn constant(&mut self, c: &mir::ConstOperand<'tcx>, span: Span) -> R<Ex> {
		let tcx = self.cx.tcx;
		let ty = c.ty();
		if let mir::Const::Ty(_, ct) = c.const_
			&& let ty::ConstKind::Param(p) = ct.kind()
		{
			return Ok(atom(structure::ident(p.name.as_str())));
		}
		if ty.is_unit() {
			return Ok(atom("()".into()));
		}
		let lean_ty = self.cx.ty(ty, span)?;
		let Some(int) = c.const_.try_eval_scalar_int(tcx, self.env) else {
			let valtree = match c.const_ {
				mir::Const::Ty(_, ct) => match ct.kind() {
					ty::ConstKind::Value(v) => Some(v.valtree),
					_ => None,
				},
				mir::Const::Unevaluated(uv, _) => match ty::Instance::try_resolve(tcx, self.env, uv.def, uv.args) {
					Ok(Some(instance)) => {
						let cid = GlobalId {
							instance,
							promoted: uv.promoted,
						};
						match tcx.const_eval_global_id_for_typeck(self.env, cid, span) {
							Ok(Ok(vt)) => Some(vt),
							_ => None,
						}
					}
					_ => None,
				},
				mir::Const::Val(..) => None,
			};
			return match valtree {
				Some(vt) => self.valtree(ty, vt, span),
				None => refuse(span, format!("constant `{c}` is not supported")),
			};
		};
		let size = int.size();
		let bits = int.to_bits(size);
		let text = match ty.kind() {
			ty::Bool => return Ok(atom(if bits != 0 { "true" } else { "false" }.into())),
			ty::Int(_) => size.sign_extend(bits).to_string(),
			_ => bits.to_string(),
		};
		Ok(lit(text, &lean_ty))
	}

	/// A constant's value tree as a Lean value of type `ty`.
	fn valtree(&mut self, ty: Ty<'tcx>, vt: ty::ValTree<'tcx>, span: Span) -> R<Ex> {
		let tcx = self.cx.tcx;
		let children = |v: ty::ValTree<'tcx>| -> Vec<(Ty<'tcx>, ty::ValTree<'tcx>)> {
			match &*v {
				ty::ValTreeKind::Branch(consts) => consts
					.iter()
					.filter_map(|c| match c.kind() {
						ty::ConstKind::Value(value) => Some((value.ty, value.valtree)),
						_ => None,
					})
					.collect(),
				ty::ValTreeKind::Leaf(_) => Vec::new(),
			}
		};
		match ty.kind() {
			ty::Ref(_, inner, _) => self.valtree(*inner, vt, span),
			ty::Bool => Ok(atom(
				if vt.try_to_leaf().is_some_and(|l| l.to_bits_unchecked() != 0) {
					"true"
				} else {
					"false"
				}
				.into(),
			)),
			ty::Int(_) | ty::Uint(_) => {
				let Some(int) = vt.try_to_leaf() else {
					return refuse(span, "an integer constant without a value");
				};
				let bits = int.to_bits(int.size());
				let text = if ty.is_signed() {
					int.size().sign_extend(bits).to_string()
				} else {
					bits.to_string()
				};
				Ok(lit(text, &self.cx.ty(ty, span)?))
			}
			ty::Tuple(tys) if tys.is_empty() => Ok(atom("()".into())),
			ty::Tuple(_) => {
				let parts = children(vt)
					.into_iter()
					.map(|(t, v)| self.valtree(t, v, span))
					.collect::<R<Vec<_>>>()?;
				Ok(atom(format!(
					"({})",
					parts.iter().map(|p| p.text.clone()).collect::<Vec<_>>().join(", ")
				)))
			}
			ty::Array(..) => {
				let parts = children(vt)
					.into_iter()
					.map(|(t, v)| self.valtree(t, v, span))
					.collect::<R<Vec<_>>>()?;
				Ok(atom(format!(
					"#v[{}]",
					parts.iter().map(|p| p.text.clone()).collect::<Vec<_>>().join(", ")
				)))
			}
			ty::Adt(def, _) => {
				let fields = children(vt);
				let (variant, fields) = if def.is_enum() {
					let Some(((_, discr), rest)) = fields.split_first() else {
						return refuse(span, "an enum constant without a variant");
					};
					(
						VariantIdx::from_u32(discr.try_to_leaf().map_or(0, |l| l.to_u32())),
						rest.to_vec(),
					)
				} else {
					(VariantIdx::from_u32(0), fields)
				};
				let values = fields
					.into_iter()
					.map(|(t, v)| self.valtree(t, v, span))
					.collect::<R<Vec<_>>>()?;
				let kind = AggregateKind::Adt(def.did(), variant, ty::GenericArgs::empty(), None, None);
				let _ = tcx;
				self.aggregate_values(&kind, values, ty, span)
			}
			_ => refuse(span, format!("a constant of type `{ty}` is not supported")),
		}
	}

	fn rvalue(&mut self, rvalue: &Rvalue<'tcx>, ty: Ty<'tcx>, span: Span) -> R<Ex> {
		let tcx = self.cx.tcx;
		match rvalue {
			Rvalue::Use(op, _) => self.operand(op, span),
			Rvalue::CopyForDeref(p) => self.read_place(*p, span),
			Rvalue::Ref(_, BorrowKind::Shared | BorrowKind::Fake(_), p) => self.read_place(*p, span),
			Rvalue::Ref(..) => refuse(span, "this mutable borrow is not supported"),
			Rvalue::Repeat(op, count) => {
				let v = self.operand(op, span)?;
				Ok(compound(
					format!("Vector.replicate {} {}", self.cx.len(*count, span)?, v.paren()),
					&[&v],
				))
			}
			Rvalue::Cast(CastKind::IntToInt, op, to) => {
				let v = self.operand(op, span)?;
				let from = op.ty(self.body, tcx);
				let to_lean = self.cx.ty(*to, span)?;
				if from.is_bool() {
					return Ok(compound(format!("if {} then (1 : {to_lean}) else 0", v.text), &[&v]));
				}
				if from == *to {
					return Ok(v);
				}
				if from.is_integral() && !from.is_signed() && !to.is_signed() {
					return Ok(compound(format!("{}.to{to_lean}", v.paren()), &[&v]));
				}
				Ok(compound(format!("Rust.cast {to_lean} {}", v.paren()), &[&v]))
			}
			Rvalue::Cast(CastKind::PointerCoercion(PointerCoercion::Unsize, _), op, _) => {
				let v = self.operand(op, span)?;
				match op.ty(self.body, tcx).builtin_deref(true).map(|t| t.kind()) {
					Some(ty::Array(..)) => Ok(compound(format!("{}.toArray", v.paren()), &[&v])),
					_ => refuse(span, "this unsizing coercion is not supported"),
				}
			}
			Rvalue::Cast(kind, ..) => refuse(span, format!("cast `{kind:?}` is not supported")),
			Rvalue::BinaryOp(op, ops) => {
				let a = self.operand(&ops.0, span)?;
				let b = self.operand(&ops.1, span)?;
				let ta = ops.0.ty(self.body, tcx);
				let tb = ops.1.ty(self.body, tcx);
				self.binop(*op, a, b, ta, tb, span)
			}
			Rvalue::UnaryOp(UnOp::Not, op) => {
				let v = self.operand(op, span)?;
				if op.ty(self.body, tcx).is_bool() {
					let mut ex = compound(format!("!{}", v.paren()), &[&v]);
					ex.prop = Some(match &v.prop {
						Some(p) => format!("¬({p})"),
						None => format!("¬{}", v.paren()),
					});
					Ok(ex)
				} else {
					Ok(compound(format!("~~~{}", v.paren()), &[&v]))
				}
			}
			Rvalue::UnaryOp(UnOp::Neg, op) => {
				let v = self.operand(op, span)?;
				Ok(compound(format!("-{}", v.paren()), &[&v]))
			}
			Rvalue::UnaryOp(UnOp::PtrMetadata, op) => {
				let v = self.operand(op, span)?;
				Ok(compound(format!("UInt64.ofNat {}.size", v.paren()), &[&v]))
			}
			Rvalue::Aggregate(kind, ops) => self.aggregate(kind, &ops.raw, ty, span),
			Rvalue::RawPtr(..) => refuse(span, "raw pointers are not supported"),
			_ => refuse(span, format!("`{rvalue:?}` is not supported")),
		}
	}

	fn binop(&mut self, op: BinOp, a: Ex, b: Ex, ta: Ty<'tcx>, tb: Ty<'tcx>, span: Span) -> R<Ex> {
		let (a, b) = strip_lits(a, b);
		let infix = |sym: &str| compound(format!("{} {sym} {}", a.paren(), b.paren()), &[&a, &b]);
		let cmp = |sym: &str| {
			let prop = format!("{} {sym} {}", a.paren(), b.paren());
			let mut ex = compound(format!("decide ({prop})"), &[&a, &b]);
			ex.prop = Some(prop);
			ex
		};
		Ok(match op {
			BinOp::Add | BinOp::AddUnchecked => infix("+"),
			BinOp::Sub | BinOp::SubUnchecked => infix("-"),
			BinOp::Mul | BinOp::MulUnchecked => infix("*"),
			BinOp::Div => infix("/"),
			BinOp::Rem => infix("%"),
			BinOp::BitAnd if ta.is_bool() => infix("&&"),
			BinOp::BitOr if ta.is_bool() => infix("||"),
			BinOp::BitXor if ta.is_bool() => infix("^^"),
			BinOp::BitAnd => infix("&&&"),
			BinOp::BitOr => infix("|||"),
			BinOp::BitXor => infix("^^^"),
			BinOp::Shl | BinOp::ShlUnchecked | BinOp::Shr | BinOp::ShrUnchecked => {
				let sym = if matches!(op, BinOp::Shl | BinOp::ShlUnchecked) {
					"<<<"
				} else {
					">>>"
				};
				let lean_a = self.cx.ty(ta, span)?;
				let a = match &a.lit {
					Some(l) => atom(format!("({l} : {lean_a})")),
					None => a.clone(),
				};
				let amount = if ta == tb {
					b.paren()
				} else if !tb.is_signed() {
					format!("{}.to{lean_a}", b.paren())
				} else {
					format!("(Rust.cast {lean_a} {})", b.paren())
				};
				compound(format!("{} {sym} {amount}", a.paren()), &[&a, &b])
			}
			BinOp::Eq => cmp("="),
			BinOp::Ne => cmp("≠"),
			BinOp::Lt => cmp("<"),
			BinOp::Le => cmp("≤"),
			BinOp::Gt => cmp(">"),
			BinOp::Ge => cmp("≥"),
			_ => return refuse(span, format!("operator `{op:?}` is not supported")),
		})
	}

	fn aggregate(&mut self, kind: &AggregateKind<'tcx>, ops: &[Operand<'tcx>], ty: Ty<'tcx>, span: Span) -> R<Ex> {
		let vals = ops.iter().map(|op| self.operand(op, span)).collect::<R<Vec<_>>>()?;
		self.aggregate_values(kind, vals, ty, span)
	}

	fn aggregate_values(&mut self, kind: &AggregateKind<'tcx>, vals: Vec<Ex>, ty: Ty<'tcx>, span: Span) -> R<Ex> {
		let refs: Vec<&Ex> = vals.iter().collect();
		match kind {
			AggregateKind::Tuple if vals.is_empty() => Ok(atom("()".into())),
			AggregateKind::Tuple => Ok(atom_of(
				format!(
					"({})",
					vals.iter().map(|v| v.text.clone()).collect::<Vec<_>>().join(", ")
				),
				&refs,
			)),
			AggregateKind::Array(_) => Ok(atom_of(
				format!(
					"#v[{}]",
					vals.iter().map(|v| v.text.clone()).collect::<Vec<_>>().join(", ")
				),
				&refs,
			)),
			AggregateKind::Adt(did, variant, _, _, None) => {
				let tcx = self.cx.tcx;
				let def = tcx.adt_def(*did);
				let lean_ty = self.cx.ty(ty, span)?;
				let var = def.variant(*variant);
				if def.is_struct() {
					let fields: Vec<String> = var
						.fields
						.iter()
						.zip(&vals)
						.map(|(f, v)| format!("{} := {}", field_ident(f), v.text))
						.collect();
					if fields.is_empty() {
						return Ok(atom(format!("({{}} : {lean_ty})")));
					}
					return Ok(atom_of(format!("({{ {} }} : {lean_ty})", fields.join(", ")), &refs));
				}
				let ctor = match (did.is_local(), self.cx.std_name(*did).as_str(), var.name.as_str()) {
					(false, "Option", "Some") => "some".to_string(),
					(false, "Option", "None") => return Ok(atom(format!("(none : {lean_ty})"))),
					(false, "Result", "Ok") => "Except.ok".to_string(),
					(false, "Result", "Err") => "Except.error".to_string(),
					(true, _, name) => format!("{}.{}", self.cx.adt_name(*did), structure::ident(name)),
					_ => return refuse(span, format!("`{ty}` has no Lean model")),
				};
				if vals.is_empty() {
					return Ok(atom(format!("({ctor} : {lean_ty})")));
				}
				let args: Vec<String> = vals.iter().map(|v| v.paren()).collect();
				Ok(atom_of(format!("({ctor} {} : {lean_ty})", args.join(" ")), &refs))
			}
			AggregateKind::Closure(..) => refuse(span, "a closure is not supported"),
			_ => refuse(span, format!("aggregate `{kind:?}` is not supported")),
		}
	}

	/// A terminator in the middle of a chain: its effect, before the next block.
	fn straight_terminator(&mut self, bb: BasicBlock, ops: &mut Vec<Op>) -> R<()> {
		let term = self.body.basic_blocks[bb].terminator();
		let span = term.source_info.span;
		match &term.kind {
			TerminatorKind::Goto { .. } | TerminatorKind::FalseEdge { .. } | TerminatorKind::FalseUnwind { .. } => {
				Ok(())
			}
			TerminatorKind::Drop { place, .. } => self.drop(*place, span),
			TerminatorKind::Assert {
				cond, expected, msg, ..
			} => self.assert(cond, *expected, msg, span, ops),
			TerminatorKind::Call { .. } => self.call(bb, ops).map(|_| ()),
			_ => unreachable!("only straight terminators continue a chain"),
		}
	}

	fn drop(&mut self, place: Place<'tcx>, span: Span) -> R<()> {
		let ty = place.ty(self.body, self.cx.tcx).ty;
		if let ty::Adt(def, _) = ty.kind()
			&& def.did().is_local()
			&& self.cx.tcx.adt_destructor(def.did()).is_some()
		{
			return refuse(
				span,
				format!("dropping `{ty}`, which has a `Drop` impl, is not supported"),
			);
		}
		Ok(())
	}

	fn assert(
		&mut self,
		cond: &Operand<'tcx>,
		expected: bool,
		msg: &AssertKind<Operand<'tcx>>,
		span: Span,
		ops: &mut Vec<Op>,
	) -> R<()> {
		if let AssertKind::Overflow(..) = msg
			&& let Operand::Move(p) | Operand::Copy(p) = cond
			&& let Some((op, a, b)) = self.checked.remove(&p.local)
		{
			let sym = match op {
				BinOp::Add => "+?",
				BinOp::Sub => "-?",
				BinOp::Mul => "*?",
				_ => return refuse(span, format!("checked `{op:?}` is not supported")),
			};
			let (a, b) = strip_lits(a, b);
			let ex = compound(format!("{} {sym} {}", a.paren(), b.paren()), &[&a, &b]);
			let ex = Ex { monadic: true, ..ex };
			let var = self.local_var[p.local.as_usize()].expect("checked locals have variables");
			self.asserted.insert(p.local);
			self.before(ops, None, true);
			if self.uses[p.local.as_usize()] == 1 && !self.vars[var].user {
				self.pending.insert(p.local, bound(ex));
			} else {
				ops.push(Op::Set {
					var,
					value: ex,
					bind: true,
				});
			}
			return Ok(());
		}
		let mut c = self.operand(cond, span)?;
		if !expected {
			let prop = c.prop.clone().map(|p| format!("¬({p})"));
			c = compound(format!("!{}", c.paren()), &[&c]);
			c.prop = prop;
		}
		// Rust's own panic message, as a Lean `String` term.
		let msg = match msg {
			AssertKind::BoundsCheck { len, index } => {
				let len = self.operand(len, span)?;
				let index = self.operand(index, span)?;
				c.reads.extend(len.reads.iter().copied());
				c.reads.extend(index.reads.iter().copied());
				format!(
					"s!\"index out of bounds: the len is {{{}}} but the index is {{{}}}\"",
					len.text, index.text
				)
			}
			AssertKind::Overflow(op, ..) => {
				let verb = match op {
					BinOp::Add | BinOp::AddUnchecked => "add",
					BinOp::Sub | BinOp::SubUnchecked => "subtract",
					BinOp::Mul | BinOp::MulUnchecked => "multiply",
					BinOp::Div => "divide",
					BinOp::Rem => "calculate the remainder",
					BinOp::Shl | BinOp::ShlUnchecked => "shift left",
					BinOp::Shr | BinOp::ShrUnchecked => "shift right",
					_ => return refuse(span, format!("an overflow check of `{op:?}` is not supported")),
				};
				format!("\"attempt to {verb} with overflow\"")
			}
			AssertKind::OverflowNeg(_) => "\"attempt to negate with overflow\"".into(),
			AssertKind::DivisionByZero(_) => "\"attempt to divide by zero\"".into(),
			AssertKind::RemainderByZero(_) => "\"attempt to calculate the remainder with a divisor of zero\"".into(),
			_ => return refuse(span, "this assertion is not supported"),
		};
		self.before(ops, None, true);
		ops.push(Op::Check { cond: c, msg });
		Ok(())
	}

	/// Lower a call; `true` if it never returns.
	fn call(&mut self, bb: BasicBlock, ops: &mut Vec<Op>) -> R<bool> {
		let tcx = self.cx.tcx;
		let TerminatorKind::Call {
			func,
			args,
			destination,
			target,
			fn_span,
			..
		} = &self.body.basic_blocks[bb].terminator().kind
		else {
			unreachable!("a call")
		};
		let span = *fn_span;
		let ty::FnDef(callee, gargs) = *func.ty(self.body, tcx).kind() else {
			return refuse(span, "calling a function pointer or closure is not supported");
		};
		let gargs = gargs.skip_binder();
		let path = tcx.def_path_str(callee);
		if ["core::panicking::", "std::panicking::", "std::rt::"]
			.iter()
			.any(|p| path.starts_with(p))
		{
			// At the `panic!` the source wrote, not inside the macro's expansion.
			return refuse(
				span.source_callsite(),
				"an explicit panic is not supported, since its message has no model",
			);
		}
		if path.starts_with("core::fmt::Arguments::") || path.starts_with("std::fmt::Arguments::") {
			self.assign(*destination, atom("()".into()), span, ops)?;
			return Ok(target.is_none());
		}
		// Structural equality, which Lean's derived `DecidableEq` decides.
		let eq_trait = tcx.lang_items().eq_trait();
		if tcx.trait_of_assoc(callee) == eq_trait && eq_trait.is_some() {
			let self_ty = gargs.type_at(0);
			let rhs_ty = gargs.type_at(1);
			// Lean's `=` needs both sides of one type.
			if tcx.erase_and_anonymize_regions(self_ty) != tcx.erase_and_anonymize_regions(rhs_ty) {
				return refuse(span, format!("comparing `{self_ty}` with `{rhs_ty}` is not supported"));
			}
			if !self.cx.structural_eq(self_ty) {
				return refuse(span, format!("`PartialEq` of `{self_ty}` is not structural equality"));
			}
			let a = self.operand(&args[0].node, span)?;
			let b = self.operand(&args[1].node, span)?;
			let (a, b) = strip_lits(a, b);
			let name = tcx.item_name(callee);
			let prop = match name.as_str() {
				"eq" => format!("{} = {}", a.paren(), b.paren()),
				"ne" => format!("{} ≠ {}", a.paren(), b.paren()),
				_ => return refuse(span, format!("`PartialEq::{name}` is not supported")),
			};
			let mut ex = compound(format!("decide ({prop})"), &[&a, &b]);
			ex.prop = Some(prop);
			self.assign(*destination, ex, span, ops)?;
			return Ok(false);
		}
		if let Some((local, gargs)) = resolve_local(tcx, self.env, callee, gargs) {
			return self
				.local_call(local, gargs, args, *destination, span, ops)
				.map(|_| target.is_none());
		}
		let instance = ty::Instance::try_resolve(tcx, self.env, callee, gargs).ok().flatten();
		let did = instance.map_or(callee, |i| i.def_id());
		let key = self.std_key(did);
		let arg = |fx: &mut Self, i: usize| fx.operand(&args[i].node, span);
		match key.as_str() {
			"Clone::clone" => {
				let t = gargs.type_at(0);
				if !self.cx.clone_copies(t) {
					return refuse(span, format!("cloning `{t}` may not copy it"));
				}
				let v = arg(self, 0)?;
				self.assign(*destination, v, span, ops)?;
			}
			"Vec::new" => {
				let lean_ty = self.cx.ty(destination.ty(self.body, tcx).ty, span)?;
				self.assign(*destination, atom(format!("(#[] : {lean_ty})")), span, ops)?;
			}
			"Vec::len" | "slice::len" => {
				let v = arg(self, 0)?;
				self.assign(
					*destination,
					compound(format!("UInt64.ofNat {}.size", v.paren()), &[&v]),
					span,
					ops,
				)?;
			}
			"Deref::deref for Vec" => {
				let v = arg(self, 0)?;
				self.assign(*destination, v, span, ops)?;
			}
			"Option::is_some" | "Option::is_none" => {
				let v = arg(self, 0)?;
				let m = if key.ends_with("some") { "isSome" } else { "isNone" };
				self.assign(*destination, compound(format!("{}.{m}", v.paren()), &[&v]), span, ops)?;
			}
			"Try::branch for Option" => {
				let v = arg(self, 0)?;
				self.assign(
					*destination,
					compound(format!("Rust.branch {}", v.paren()), &[&v]),
					span,
					ops,
				)?;
			}
			"FromResidual::from_residual for Option" => {
				let lean_ty = self.cx.ty(destination.ty(self.body, tcx).ty, span)?;
				self.assign(*destination, atom(format!("(none : {lean_ty})")), span, ops)?;
			}
			"u8::checked_add" | "u16::checked_add" | "u32::checked_add" | "u64::checked_add" | "usize::checked_add"
			| "u8::checked_sub" | "u16::checked_sub" | "u32::checked_sub" | "u64::checked_sub"
			| "usize::checked_sub" => {
				let (a, b) = (arg(self, 0)?, arg(self, 1)?);
				let f = if key.ends_with("add") {
					"checkedAdd"
				} else {
					"checkedSub"
				};
				self.assign(
					*destination,
					compound(format!("Rust.{f} {} {}", a.paren(), b.paren()), &[&a, &b]),
					span,
					ops,
				)?;
			}
			"Vec::truncate" => {
				let place = self.mutable_arg(&args[0].node, span)?;
				let n = arg(self, 1)?;
				let cur = self.read_place(place, span)?;
				let new = compound(format!("{}.extract 0 {}.toNat", cur.paren(), n.paren()), &[&cur, &n]);
				self.assign(place, new, span, ops)?;
			}
			"Vec::push" | "Vec::extend_from_slice" => {
				if key == "Vec::extend_from_slice" {
					let elem = gargs.type_at(0);
					if !self.cx.clone_copies(elem) {
						return refuse(span, format!("cloning `{elem}` may not copy it"));
					}
				}
				let place = self.mutable_arg(&args[0].node, span)?;
				let x = arg(self, 1)?;
				let cur = self.read_place(place, span)?;
				let new = if key == "Vec::push" {
					compound(format!("{}.push {}", cur.paren(), x.paren()), &[&cur, &x])
				} else {
					compound(format!("{} ++ {}", cur.paren(), x.paren()), &[&cur, &x])
				};
				self.assign(place, new, span, ops)?;
			}
			_ => return refuse(span, format!("`{}` has no Lean model", tcx.def_path_str(did))),
		}
		Ok(target.is_none())
	}

	/// The place a `&mut` argument borrows.
	fn mutable_arg(&self, op: &Operand<'tcx>, span: Span) -> R<Place<'tcx>> {
		let (Operand::Move(p) | Operand::Copy(p)) = op else {
			return refuse(span, "this mutable argument is not supported");
		};
		if !p.projection.is_empty() {
			return refuse(span, "this mutable argument is not supported");
		}
		match self.mutable.get(&p.local) {
			Some(Mutable::Borrow(target)) => Ok(*target),
			Some(Mutable::Param) => Ok(Place::from(p.local).project_deeper(&[ProjectionElem::Deref], self.cx.tcx)),
			None => refuse(span, "this mutable argument is not supported"),
		}
	}

	/// A key naming a standard library function by its impl, such as `Vec::push` or
	/// `Deref::deref for Vec`.
	fn std_key(&self, did: DefId) -> String {
		let tcx = self.cx.tcx;
		let clone = tcx.lang_items().clone_trait();
		if clone.is_some()
			&& (tcx.trait_of_assoc(did) == clone
				|| tcx.trait_impl_of_assoc(did).is_some_and(|imp| {
					Some(tcx.impl_trait_ref(imp).instantiate_identity().skip_norm_wip().def_id) == clone
				})) {
			return "Clone::clone".into();
		}
		if !self.cx.is_std(did) {
			return String::new();
		}
		let name = tcx.item_name(did);
		if let Some(trait_id) = tcx.trait_of_assoc(did) {
			return format!("{}::{name}", tcx.item_name(trait_id));
		}
		let Some(impl_id) = tcx.impl_of_assoc(did) else {
			return tcx.def_path_str(did);
		};
		let head = match tcx.type_of(impl_id).instantiate_identity().skip_norm_wip().kind() {
			ty::Adt(def, _) => tcx.item_name(def.did()).to_string(),
			ty::Slice(_) => "slice".into(),
			ty::Array(..) => "array".into(),
			ty::Uint(_) | ty::Int(_) => self_ty_name(tcx.type_of(impl_id).instantiate_identity().skip_norm_wip()),
			_ => "?".into(),
		};
		let trait_id = tcx
			.trait_impl_of_assoc(did)
			.map(|imp| tcx.impl_trait_ref(imp).instantiate_identity().skip_norm_wip().def_id);
		match trait_id {
			Some(t) if tcx.item_name(t).as_str() == "Clone" => "Clone::clone".into(),
			Some(t) => format!("{}::{name} for {head}", tcx.item_name(t)),
			None => format!("{head}::{name}"),
		}
	}

	fn local_call(
		&mut self,
		callee: DefId,
		gargs: GenericArgsRef<'tcx>,
		args: &[rustc_span::Spanned<Operand<'tcx>>],
		destination: Place<'tcx>,
		span: Span,
		ops: &mut Vec<Op>,
	) -> R<()> {
		let tcx = self.cx.tcx;
		let shape = self.cx.shapes.get(&callee);
		let monadic = shape.is_none_or(|s| s.monadic || s.failed);
		let sig = tcx.fn_sig(callee).instantiate_identity().skip_norm_wip().skip_binder();
		// A generic argument Lean cannot infer from the arguments' types.
		let mut text = self.cx.fn_name(callee);
		let generics = tcx.generics_of(callee);
		let mut all = Vec::new();
		let mut g = Some(generics);
		while let Some(gen_) = g {
			all.splice(0..0, gen_.own_params.iter().cloned());
			g = gen_.parent.map(|p| tcx.generics_of(p));
		}
		let eq = tcx.lang_items().eq_trait();
		for param in &all {
			let arg = gargs[param.index as usize];
			if let ty::GenericParamDefKind::Type { .. } = param.kind
				&& let Some(t) = arg.as_type()
				&& eq.is_some()
				&& !self.cx.structural_eq(t)
			{
				return refuse(span, format!("`{t}` passed where `PartialEq` must be structural"));
			}
			let shows = sig.inputs().iter().any(|t| {
				t.walk().any(|a| match (a.as_type(), a.as_const(), &param.kind) {
					(Some(t), _, ty::GenericParamDefKind::Type { .. }) => {
						matches!(t.kind(), ty::Param(p) if p.index == param.index)
					}
					(_, Some(c), ty::GenericParamDefKind::Const { .. }) => {
						matches!(c.kind(), ty::ConstKind::Param(p) if p.index == param.index)
					}
					_ => false,
				})
			});
			if shows {
				continue;
			}
			let value = match (&param.kind, arg.as_type(), arg.as_const()) {
				(ty::GenericParamDefKind::Type { .. }, Some(t), _) => self.cx.ty(t, span)?,
				(ty::GenericParamDefKind::Const { .. }, _, Some(c)) => self.cx.const_value(c, span)?,
				_ => continue,
			};
			text += &format!(" ({} := {value})", structure::ident(param.name.as_str()));
		}
		let mut reads = Vec::new();
		let mut writebacks = Vec::new();
		for (arg, pty) in args.iter().zip(sig.inputs()) {
			if matches!(pty.kind(), ty::Ref(_, _, ty::Mutability::Mut)) {
				let place = self.mutable_arg(&arg.node, span)?;
				let cur = self.read_place(place, span)?;
				text += &format!(" {}", cur.paren());
				reads.push(cur);
				writebacks.push(place);
			} else {
				let v = self.operand(&arg.node, span)?;
				text += &format!(" {}", v.paren());
				reads.push(v);
			}
		}
		let refs: Vec<&Ex> = reads.iter().collect();
		// `self.method args` where Lean can find it from the receiver's type.
		// A trait method keeps its full name: dot notation would find an inherent one first.
		let method = tcx.opt_associated_item(callee).is_some_and(|i| i.is_method())
			&& tcx.trait_impl_of_assoc(callee).is_none()
			&& !text.contains(":=")
			&& reads.first().is_some_and(|r| r.atom && !r.text.starts_with('('))
			&& matches!(sig.inputs().first().map(|t| t.peel_refs().kind()), Some(ty::Adt(d, _)) if d.did().is_local());
		let text = if method {
			let name = self.cx.fn_name(callee);
			let short = name.rsplit('.').next().expect("names have parts");
			let mut out = format!("{}.{short}", reads[0].text);
			for r in &reads[1..] {
				out += &format!(" {}", r.paren());
			}
			out
		} else {
			text
		};
		let call = compound(text, &refs);
		let ret_unit = sig.output().is_unit();
		if writebacks.is_empty() {
			let value = if monadic {
				bound(Ex { monadic: true, ..call })
			} else {
				call
			};
			return self.assign(destination, value, span, ops);
		}
		// `let r ← f ..`, then the result and each mutable argument from it.
		let mut parts = Vec::new();
		if !ret_unit {
			parts.push(Some(destination));
		}
		parts.extend(writebacks.iter().map(|p| Some(*p)));
		let r = self.var("r", String::new(), false);
		self.before(ops, None, monadic);
		ops.push(Op::Set {
			var: r,
			value: Ex { monadic, ..call },
			bind: monadic,
		});
		let n = parts.len();
		for (i, place) in parts.into_iter().enumerate() {
			let place = place.expect("listed");
			let text = tuple_field(&self.vars[r].name, i, n);
			let mut ex = atom(text);
			ex.reads.insert(r);
			let (var, update) = self.write(place, ex, span)?;
			self.before(ops, Some(var), false);
			push_set(ops, var, update);
			self.written.insert(var);
		}
		Ok(())
	}

	fn terminator(&mut self, bb: BasicBlock, ops: &mut Vec<Op>) -> R<Term> {
		let term = self.body.basic_blocks[bb].terminator();
		let span = term.source_info.span;
		Ok(match &term.kind {
			TerminatorKind::Goto { target, .. } => Term::Goto(self.target(*target)),
			TerminatorKind::FalseEdge { real_target, .. } => Term::Goto(self.target(*real_target)),
			TerminatorKind::FalseUnwind { real_target, .. } => Term::Goto(self.target(*real_target)),
			TerminatorKind::Return => Term::Return,
			TerminatorKind::Unreachable => Term::Unreachable,
			TerminatorKind::Drop { place, target, .. } => {
				self.drop(*place, span)?;
				Term::Goto(self.target(*target))
			}
			TerminatorKind::Assert {
				cond,
				expected,
				msg,
				target,
				..
			} => {
				self.assert(cond, *expected, msg, span, ops)?;
				Term::Goto(self.target(*target))
			}
			TerminatorKind::Call { target, .. } => {
				let diverges = self.call(bb, ops)?;
				match (target, diverges) {
					(Some(t), false) => Term::Goto(self.target(*t)),
					_ => {
						if !matches!(ops.last(), Some(Op::Throw(_))) {
							return refuse(span, "calling a function that never returns is not supported");
						}
						let Some(Op::Throw(msg)) = ops.pop() else {
							unreachable!("checked")
						};
						Term::Throw(msg)
					}
				}
			}
			TerminatorKind::SwitchInt { discr, targets } => self.switch(discr, targets, span, ops)?,
			TerminatorKind::UnwindResume | TerminatorKind::UnwindTerminate(_) => Term::Unreachable,
			_ => return refuse(span, format!("`{:?}` is not supported", term.kind)),
		})
	}

	/// The pattern matching `place` as variant `v`, and the variables it binds.
	fn pattern_for(
		&mut self,
		place: Place<'tcx>,
		def: ty::AdtDef<'tcx>,
		v: VariantIdx,
		scrut: &str,
		span: Span,
	) -> R<(String, Vec<usize>)> {
		let fields = def.variant(v).fields.len();
		let vars = match self.pattern.get(&(place, v)) {
			Some(vars) => vars.clone(),
			None => {
				let stem = scrut
					.split('.')
					.rev()
					.find(|s| {
						!s.is_empty()
							&& s.chars().all(|c| c.is_alphanumeric() || c == '_')
							&& !s.starts_with(|c: char| c.is_ascii_digit())
					})
					.unwrap_or("x")
					.to_string();
				let vars: Vec<usize> = (0..fields)
					.map(|i| self.var(&format!("{stem}_{i}"), String::new(), false))
					.collect();
				self.pattern.insert((place, v), vars.clone());
				vars
			}
		};
		let names: Vec<String> = vars.iter().map(|&x| self.vars[x].name.clone()).collect();
		let ctor = match (
			def.did().is_local(),
			self.cx.std_name(def.did()).as_str(),
			def.variant(v).name.as_str(),
		) {
			(false, "Option", "Some") => "some".to_string(),
			(false, "Option", "None") => "none".to_string(),
			(false, "Result", "Ok") => ".ok".to_string(),
			(false, "Result", "Err") => ".error".to_string(),
			(false, "ControlFlow", name) => format!(".{name}"),
			(true, _, name) => format!(".{}", structure::ident(name)),
			_ => {
				return refuse(
					span,
					format!("matching on `{}` is not supported", self.cx.tcx.item_name(def.did())),
				);
			}
		};
		Ok((
			if names.is_empty() {
				ctor
			} else {
				format!("{ctor} {}", names.join(" "))
			},
			vars,
		))
	}

	fn switch(
		&mut self,
		discr: &Operand<'tcx>,
		targets: &mir::SwitchTargets,
		span: Span,
		ops: &mut Vec<Op>,
	) -> R<Term> {
		let tcx = self.cx.tcx;
		if let Operand::Move(p) | Operand::Copy(p) = discr
			&& p.projection.is_empty()
			&& let Some(place) = self.discr.get(&p.local).copied()
		{
			let ty = place.ty(self.body, tcx).ty;
			let ty::Adt(def, _) = ty.kind() else {
				return refuse(span, format!("matching on `{ty}` is not supported"));
			};
			let scrut = self.read_place(place, span)?;
			self.before(ops, None, scrut.monadic);
			let mut arms = Vec::new();
			let mut covered = HashSet::new();
			for (value, target) in targets.iter() {
				let Some((v, _)) = def.discriminants(tcx).find(|(_, d)| d.val == value) else {
					return refuse(span, "a discriminant no variant has");
				};
				covered.insert(v);
				let (pat, vars) = self.pattern_for(place, *def, v, &scrut.text, span)?;
				arms.push((pat, vars, self.target(target)));
			}
			let otherwise = targets.otherwise();
			// One variant left: the default arm is its arm, binding its fields.
			let left: Vec<VariantIdx> = def.variants().indices().filter(|v| !covered.contains(v)).collect();
			if let [v] = left.as_slice() {
				let v = *v;
				let (pat, vars) = self.pattern_for(place, *def, v, &scrut.text, span)?;
				arms.push((pat, vars, self.target(otherwise)));
				return Ok(Term::Match {
					scrut,
					arms,
					default: None,
				});
			}
			let all = covered.len() == def.variants().len();
			let default = if all { None } else { Some(self.target(otherwise)) };
			return Ok(Term::Match { scrut, arms, default });
		}
		let d = self.operand(discr, span)?;
		self.before(ops, None, d.monadic);
		let d = if d.monadic {
			let ty = self.cx.ty(discr.ty(self.body, tcx), span)?;
			let v = self.var("d", ty, false);
			push_set(ops, v, d);
			var_ex(&self.vars, v)
		} else {
			d
		};
		if discr.ty(self.body, tcx).is_bool() {
			// `switchInt(b) -> [0: else, otherwise: then]`
			let Some((_, els)) = targets.iter().next() else {
				return Ok(Term::Goto(self.target(targets.otherwise())));
			};
			return Ok(Term::Cases {
				arms: vec![(d, self.target(targets.otherwise()))],
				default: self.target(els),
			});
		}
		let mut arms = Vec::new();
		for (value, target) in targets.iter() {
			let prop = format!("{} = {value}", d.paren());
			let mut c = compound(format!("decide ({prop})"), &[&d]);
			c.prop = Some(prop);
			arms.push((c, self.target(target)));
		}
		Ok(Term::Cases {
			arms,
			default: self.target(targets.otherwise()),
		})
	}
}

/// The type of `place` after its first `n` projections.
fn place_ty<'tcx>(body: &Body<'tcx>, tcx: TyCtxt<'tcx>, place: Place<'tcx>, n: usize) -> mir::PlaceTy<'tcx> {
	let mut ty = mir::PlaceTy::from_ty(body.local_decls[place.local].ty);
	for elem in &place.projection[..n] {
		ty = ty.projection_ty(tcx, *elem);
	}
	ty
}

/// A projection from a Lean value.
#[derive(Clone)]
enum Proj {
	Field(String),
	/// Field `.0` of a tuple of this many.
	Tuple(usize, usize),
	Index(Ex),
}

fn project(base: &Ex, proj: &Proj) -> Ex {
	match proj {
		Proj::Field(f) => {
			let mut ex = atom_of(format!("{}.{f}", base.paren()), &[base]);
			ex.atom = true;
			ex
		}
		Proj::Tuple(i, n) => atom_of(format!("{}.{}", base.paren(), tuple_path(*i, *n)), &[base]),
		Proj::Index(i) => atom_of(format!("{}[{}.toNat]!", base.paren(), i.paren()), &[base, i]),
	}
}

fn update(container: &Ex, proj: &Proj, value: Ex) -> Ex {
	match proj {
		Proj::Field(f) => atom_of(
			format!("{{ {} with {f} := {} }}", container.text, value.text),
			&[container, &value],
		),
		Proj::Tuple(i, n) => {
			let parts: Vec<String> = (0..*n)
				.map(|j| {
					if j == *i {
						value.text.clone()
					} else {
						format!("{}.{}", container.paren(), tuple_path(j, *n))
					}
				})
				.collect();
			atom_of(format!("({})", parts.join(", ")), &[container, &value])
		}
		Proj::Index(i) => compound(
			format!("{}.set! {}.toNat {}", container.paren(), i.paren(), value.paren()),
			&[container, i, &value],
		),
	}
}

/// The Lean path to field `i` of a right-nested tuple of `n`.
fn tuple_path(i: usize, n: usize) -> String {
	let mut parts = vec!["2"; i];
	if i + 1 < n {
		parts.push("1");
	}
	parts.join(".")
}

fn tuple_field(base: &str, i: usize, n: usize) -> String {
	if n == 1 {
		base.to_string()
	} else {
		format!("{base}.{}", tuple_path(i, n))
	}
}

fn successors(body: &Body<'_>, bb: BasicBlock) -> Vec<BasicBlock> {
	let term = body.basic_blocks[bb].terminator();
	match &term.kind {
		TerminatorKind::Goto { target, .. } => vec![*target],
		TerminatorKind::SwitchInt { targets, .. } => targets.all_targets().to_vec(),
		TerminatorKind::Assert { target, .. } | TerminatorKind::Drop { target, .. } => vec![*target],
		TerminatorKind::Call { target, .. } => target.iter().copied().collect(),
		TerminatorKind::FalseEdge { real_target, .. } | TerminatorKind::FalseUnwind { real_target, .. } => {
			vec![*real_target]
		}
		_ => vec![],
	}
}

/// The one successor of a terminator that falls through to it, with an effect at most.
fn straight(body: &Body<'_>, bb: BasicBlock) -> Option<BasicBlock> {
	match &body.basic_blocks[bb].terminator().kind {
		TerminatorKind::Goto { target, .. }
		| TerminatorKind::Assert { target, .. }
		| TerminatorKind::Drop { target, .. }
		| TerminatorKind::FalseEdge {
			real_target: target, ..
		}
		| TerminatorKind::FalseUnwind {
			real_target: target, ..
		} => Some(*target),
		TerminatorKind::Call {
			target: Some(target), ..
		} => Some(*target),
		_ => None,
	}
}

fn push_set(ops: &mut Vec<Op>, var: usize, value: Ex) {
	ops.push(Op::Set {
		var,
		value,
		bind: false,
	});
}

fn atom(text: String) -> Ex {
	Ex {
		text,
		atom: true,
		monadic: false,
		reads: BTreeSet::new(),
		prop: None,
		lit: None,
	}
}

fn lit(text: String, ty: &str) -> Ex {
	let shown = if text.starts_with('-') {
		format!("({text})")
	} else {
		text.clone()
	};
	Ex {
		text: format!("({shown} : {ty})"),
		atom: true,
		monadic: false,
		reads: BTreeSet::new(),
		prop: None,
		lit: Some(shown),
	}
}

/// Show a literal bare beside a typed operand, which Lean infers its type from.
fn strip_lits(a: Ex, b: Ex) -> (Ex, Ex) {
	match (&a.lit, &b.lit) {
		(Some(l), None) => (atom(l.clone()), b),
		(None, Some(l)) => (a, atom(l.clone())),
		_ => (a, b),
	}
}

fn var_ex(vars: &[Var], v: usize) -> Ex {
	let mut ex = atom(vars[v].name.clone());
	ex.reads.insert(v);
	ex
}

fn compound(text: String, parts: &[&Ex]) -> Ex {
	let mut ex = atom_of(text, parts);
	ex.atom = false;
	ex
}

fn atom_of(text: String, parts: &[&Ex]) -> Ex {
	Ex {
		text,
		atom: true,
		monadic: parts.iter().any(|p| p.monadic),
		reads: parts.iter().flat_map(|p| p.reads.iter().copied()).collect(),
		prop: None,
		lit: None,
	}
}

/// An action's result inside an expression: `(← action)`.
fn bound(action: Ex) -> Ex {
	Ex {
		text: format!("(← {})", action.text),
		atom: true,
		monadic: true,
		reads: action.reads,
		prop: None,
		lit: None,
	}
}

/// A primitive integer type's Rust name, as its inherent impl's functions are keyed.
fn self_ty_name(ty: Ty<'_>) -> String {
	ty.to_string()
}
