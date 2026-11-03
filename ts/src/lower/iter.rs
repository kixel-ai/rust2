//! Iterators, as `rt/iter.ts` gives them: std's iterators are `rt.Iter` objects whose
//! `next` returns an item or `rt.DONE`, and whose adaptors and consumers are runtime
//! functions taking the iterator first: `rt.iterMap(it, f)`. The crate's own `impl
//! Iterator` keeps that protocol in its `next`, and joins std's adaptors through
//! `rt.iterFrom`, with its own `size_hint`, `next_back`, and `len`.

use rustc_hir::def_id::DefId;
use rustc_middle::ty::{self, Instance, Ty, TyCtxt, TypeVisitableExt};
use rustc_span::Span;

use super::Translator;
use super::body::Body;
use super::library::{self, Call, Std};
use super::num::Int;
use crate::ts::{Expr, Key, Param, Type};

/// `Iterator::next`, and the trait's `Item`.
fn trait_items(tcx: ty::TyCtxt<'_>) -> Option<(DefId, DefId)> {
	let iterator = tcx.get_diagnostic_item(rustc_span::sym::Iterator)?;
	let items = tcx.associated_items(iterator);
	let next = items.filter_by_name_unhygienic(rustc_span::sym::next).next()?.def_id;
	let item = items.filter_by_name_unhygienic(rustc_span::sym::Item).next()?.def_id;
	Some((next, item))
}

/// `Iterator::next`.
pub fn next_fn(tcx: ty::TyCtxt<'_>) -> Option<DefId> {
	Some(trait_items(tcx)?.0)
}

/// Whether `def_id` is `Iterator::next`, or an impl's.
pub fn is_next(tcx: ty::TyCtxt<'_>, def_id: DefId) -> bool {
	let Some((next, _)) = trait_items(tcx) else {
		return false;
	};
	def_id == next || tcx.trait_item_of(def_id) == Some(next)
}

impl<'tcx> Translator<'_, 'tcx> {
	/// `Iterator::Item` of `ty`, if `ty` is an iterator. A type over parameters, which a
	/// generic type's fields and a census have, may leave it a projection.
	pub fn iter_item(&self, ty: Ty<'tcx>) -> Option<Ty<'tcx>> {
		let tcx = self.tcx;
		let (_, item) = trait_items(tcx)?;
		let projection = Ty::new_projection(tcx, ty::IsRigid::No, item, [ty]);
		tcx.try_normalize_erasing_regions(self.typing_env_pub(), ty::Unnormalized::new_wip(projection))
			.ok()
	}

	/// `Iterator::Item` of `ty`, if it is a `dyn Iterator`, `dyn DoubleEndedIterator`,
	/// `dyn ExactSizeIterator`, or `dyn FusedIterator`, whose JS value is an `rt.Iter` of its
	/// items.
	pub fn dyn_iter(&self, ty: Ty<'tcx>) -> Option<Ty<'tcx>> {
		let ty::Dynamic(preds, ..) = ty.kind() else {
			return None;
		};
		let principal = preds.principal_def_id()?;
		let name = library::std_path(self.tcx, principal).pop()?;
		if !matches!(
			name.as_str(),
			"Iterator" | "DoubleEndedIterator" | "ExactSizeIterator" | "FusedIterator"
		) {
			return None;
		}
		self.iter_item(ty)
	}
}

/// The crate's own iterator a `ty` is, through any references: its `next`, whose
/// instance the translation names.
fn own_next<'tcx>(b: &Body<'_, '_, 'tcx>, ty: Ty<'tcx>) -> Option<Instance<'tcx>> {
	own_method(b, ty, "Iterator", "next")
}

/// The methods of std's iterator traits, by trait, that an iterator of the crate may have
/// of its own: `rt.iterFrom` calls each where std's adaptors do.
const OWN: [(&str, &str); 10] = [
	("Iterator", "next"),
	("Iterator", "size_hint"),
	("Iterator", "nth"),
	("Iterator", "fold"),
	("Iterator", "count"),
	("Iterator", "last"),
	("DoubleEndedIterator", "next_back"),
	("DoubleEndedIterator", "nth_back"),
	("DoubleEndedIterator", "rfold"),
	("ExactSizeIterator", "len"),
];

/// The methods in `OWN` that take the iterator by value, which std's consumers call of
/// one they own, but not through a `&mut`, which reads it by `try_fold`.
const BY_VALUE: [(&str, &str); 4] = [
	("Iterator", "fold"),
	("DoubleEndedIterator", "rfold"),
	("Iterator", "count"),
	("Iterator", "last"),
];

/// Refuse an `impl Iterator` of the crate, whose `next` is `next_fn`, that std's adaptors
/// would read other than through the methods in `OWN`: one whose `Iterator` or
/// `DoubleEndedIterator` impl overrides another, such as `find`, which they call in its
/// place, or whose own `fold` or `rfold` tells its accumulator or closure apart from the
/// ones `fold_args` translates it for.
pub fn check_impl(t: &mut Translator<'_, '_>, next_fn: DefId) {
	let tcx = t.tcx;
	let Some(imp) = tcx.impl_of_assoc(next_fn) else {
		return;
	};
	let ty::Adt(adt, _) = tcx.type_of(imp).instantiate_identity().skip_norm_wip().kind() else {
		return;
	};
	// Only the type's crate can implement std's traits for it.
	for &other in tcx.trait_impls_in_crate(adt.did().krate) {
		let self_ty = tcx.type_of(other).instantiate_identity().skip_norm_wip();
		if !matches!(self_ty.kind(), ty::Adt(other, _) if other.did() == adt.did()) {
			continue;
		}
		let path = library::std_path(tcx, tcx.impl_trait_id(other));
		let Some(tr) = path.last().filter(|tr| OWN.iter().any(|(own, _)| own == tr)) else {
			continue;
		};
		for item in tcx.associated_items(other).in_definition_order() {
			let name = item.name();
			if !item.is_fn() {
				continue;
			}
			if !OWN.contains(&(tr.as_str(), name.as_str())) {
				t.err(
					tcx.def_span(item.def_id),
					format!("overriding `{tr}::{name}`, which std's adaptors call in place of `next` or beside it"),
				);
			} else if matches!(name.as_str(), "fold" | "rfold")
				&& let Some((span, what)) = erases(tcx, item.def_id)
			{
				t.err(
					span,
					format!("overriding `{tr}::{name}` with {what}, which is not supported yet"),
				);
			}
		}
	}
}

/// Where the crate's own `fold` or `rfold`, `def_id`, or a closure in it, tells its
/// accumulator `B` or closure `F` apart from the others it is generic over, and how: by an
/// `Option` or a `Poll` of `B`, whose `None` or `Pending` is the one of whichever `B` it is
/// translated for; a call generic over either, but `F`'s and std's iterators', which take
/// any unless the crate's own type holds one; a constant generic over either, such as its
/// size; or a drop of `B`, which drops as the one it is translated for does.
fn erases(tcx: TyCtxt<'_>, def_id: DefId) -> Option<(Span, &'static str)> {
	use rustc_middle::mir::visit::Visitor;
	use rustc_middle::mir::{self, ConstOperand, Location, TerminatorKind};

	struct Erases<'a, 'tcx> {
		tcx: TyCtxt<'tcx>,
		mir: &'a mir::Body<'tcx>,
		/// The indices of `B` and `F`.
		params: [u32; 2],
		closures: Vec<DefId>,
		found: Option<(Span, &'static str)>,
	}
	impl<'tcx> Erases<'_, 'tcx> {
		/// Whether `arg` is over one of `params`, a closure through what it captures, as its
		/// own body is checked in its place, or only under a type of the crate if `crate_only`.
		fn over(&mut self, arg: ty::GenericArg<'tcx>, params: &[u32], crate_only: bool) -> bool {
			let mut walk = arg.walk();
			let mut over = false;
			while let Some(a) = walk.next() {
				let Some(t) = a.as_type() else {
					continue;
				};
				match *t.kind() {
					ty::Closure(closure, args) => {
						self.closures.push(closure);
						walk.skip_current_subtree();
						over |= self.over(args.as_closure().tupled_upvars_ty().into(), params, crate_only);
					}
					ty::Adt(adt, args) if crate_only && !library::is_std(self.tcx, adt.did()) => {
						walk.skip_current_subtree();
						over |= args.iter().any(|a| self.over(a, params, false));
					}
					ty::Param(p) if !crate_only && params.contains(&p.index) => over = true,
					_ => {}
				}
			}
			over
		}

		/// Whether a drop of `ty` drops a `B`, or runs the crate's code over `B` or `F`.
		fn drops(&mut self, ty: Ty<'tcx>) -> bool {
			let params = self.params;
			self.over(ty.into(), &params[..1], false) || self.over(ty.into(), &params, true)
		}

		/// Whether a call of `def_id` for `args` runs the crate's code over `B` or `F`, or code
		/// that can tell them apart: anything but `F`'s calls and std's iterators', which take
		/// any `B` and `F`.
		fn tells(&mut self, def_id: DefId, args: ty::GenericArgsRef<'tcx>) -> bool {
			let tcx = self.tcx;
			let any = tcx.trait_of_assoc(def_id).is_some_and(|tr| {
				tcx.is_fn_trait(tr)
					|| [core_trait(tcx, "Iterator"), core_trait(tcx, "DoubleEndedIterator")].contains(&Some(tr))
			});
			let params = self.params;
			args.iter().any(|a| self.over(a, &params, any))
		}

		fn find(&mut self, span: Span, what: &'static str) {
			self.found.get_or_insert((span, what));
		}
	}
	impl<'tcx> Visitor<'tcx> for Erases<'_, 'tcx> {
		fn visit_terminator(&mut self, term: &mir::Terminator<'tcx>, loc: Location) {
			let span = term.source_info.span;
			match &term.kind {
				TerminatorKind::Call { func, args, .. } => {
					if let Some((callee, callee_args)) = func.const_fn_def() {
						if self.tcx.is_diagnostic_item(rustc_span::sym::mem_drop, callee) {
							if self.drops(callee_args.type_at(0)) {
								self.find(
									span,
									"a drop of its accumulator, or of the crate's type over it or its closure",
								);
							}
						} else if self.tells(callee, callee_args) {
							self.find(span, "a call generic over its accumulator or closure");
						}
					}
					for arg in args {
						self.visit_operand(&arg.node, loc);
					}
					return;
				}
				TerminatorKind::Drop { place, .. } => {
					let ty = place.ty(self.mir, self.tcx).ty;
					if self.drops(ty) {
						self.find(
							span,
							"a drop of its accumulator, or of the crate's type over it or its closure",
						);
					}
				}
				_ => {}
			}
			self.super_terminator(term, loc);
		}

		fn visit_const_operand(&mut self, c: &ConstOperand<'tcx>, loc: Location) {
			let args = match c.const_ {
				mir::Const::Unevaluated(uv, _) => uv.args,
				_ => ty::List::empty(),
			};
			let params = self.params;
			if self.over(c.const_.ty().into(), &params, false) || args.iter().any(|a| self.over(a, &params, false)) {
				self.find(
					self.mir.source_info(loc).span,
					"a constant generic over its accumulator or closure",
				);
			}
		}
	}

	let generics = tcx.generics_of(def_id);
	let mut params = generics
		.own_params
		.iter()
		.filter(|p| matches!(p.kind, ty::GenericParamDefKind::Type { .. }))
		.map(|p| p.index);
	let params = [params.next()?, params.next()?];
	let mut bodies = vec![def_id];
	let mut seen = Vec::new();
	while let Some(def_id) = bodies.pop() {
		if seen.contains(&def_id) {
			continue;
		}
		seen.push(def_id);
		let mir = tcx.optimized_mir(def_id);
		let mut erases = Erases {
			tcx,
			mir,
			params,
			closures: Vec::new(),
			found: None,
		};
		for local in &mir.local_decls {
			let wraps = local.ty.walk().filter_map(|arg| arg.as_type()).any(|t| match t.kind() {
				ty::Adt(adt, args) => {
					matches!(Std::of(tcx, adt.did()), Some(Std::Option | Std::Poll))
						&& matches!(args.type_at(0).kind(), ty::Param(p) if p.index == params[0])
				}
				_ => false,
			});
			if wraps {
				return Some((local.source_info.span, "its accumulator in an `Option` or a `Poll`"));
			}
			erases.over(local.ty.into(), &[], false);
		}
		erases.visit_body(mir);
		if erases.found.is_some() {
			return erases.found;
		}
		bodies.extend(erases.closures);
	}
	None
}

/// The generic arguments the crate's own `fold` or `rfold` of `item`s is translated for,
/// which a body `erases` doesn't refuse can't tell apart from any others: the accumulator,
/// as an `Option<u32>`, and the closure, as a `Box<dyn FnMut(acc, item) -> acc>`, whose JS
/// value is the function `rt.iterFrom` gives it and which drops what std's closure owns as
/// it drops.
fn fold_args<'tcx>(tcx: TyCtxt<'tcx>, item: Ty<'tcx>) -> Option<[Ty<'tcx>; 2]> {
	let lang = tcx.lang_items();
	let acc = Ty::new_option(tcx, tcx.types.u32);
	let inputs = Ty::new_tup(tcx, &[acc, item]);
	let call = ty::ExistentialTraitRef::new(tcx, lang.fn_mut_trait()?, [inputs]);
	let output = ty::ExistentialProjection::new(tcx, lang.fn_once_output()?, [inputs], acc.into());
	let preds = tcx.mk_poly_existential_predicates(&[
		ty::Binder::dummy(ty::ExistentialPredicate::Trait(call)),
		ty::Binder::dummy(ty::ExistentialPredicate::Projection(output)),
	]);
	let f = Ty::new_box(tcx, Ty::new_dynamic(tcx, preds, tcx.lifetimes.re_erased));
	Some([acc, f])
}

/// The crate's own `tr::name` of an iterator of type `ty`, through any references, if `ty`
/// is one of the crate's own that overrides it or has it as a required method: its
/// instance for `fold_args` if it is a `fold` or `rfold`.
fn own_method<'tcx>(b: &Body<'_, '_, 'tcx>, ty: Ty<'tcx>, tr: &str, name: &str) -> Option<Instance<'tcx>> {
	let tcx = b.t.tcx;
	let ty = ty.peel_refs();
	// Over parameters, only a census's instance, which never runs, would call it.
	if !matches!(ty.kind(), ty::Adt(adt, _) if b.t.own(adt.did())) || ty.has_param() || ty.has_aliases() {
		return None;
	}
	if !implements(b.t, ty, tr) {
		return None;
	}
	let method = tcx
		.associated_items(core_trait(tcx, tr)?)
		.filter_by_name_unhygienic(rustc_span::Symbol::intern(name))
		.next()?
		.def_id;
	let mut args = vec![ty::GenericArg::from(ty)];
	if matches!(name, "fold" | "rfold") {
		args.extend(fold_args(tcx, b.t.iter_item(ty)?)?.map(ty::GenericArg::from));
	}
	let instance = Instance::try_resolve(tcx, b.t.typing_env_pub(), method, tcx.mk_args(&args)).ok()??;
	(instance.def_id() != method).then_some(instance)
}

/// Whether `ty` holds, by value rather than through a reference, an iterator of the crate with a
/// method in `BY_VALUE` of its own, which std's consumers call where they take it.
fn folds_own<'tcx>(b: &Body<'_, '_, 'tcx>, ty: Ty<'tcx>) -> bool {
	let mut walk = ty.walk();
	while let Some(arg) = walk.next() {
		let Some(t) = arg.as_type() else {
			continue;
		};
		if t.is_ref() {
			walk.skip_current_subtree();
		} else if BY_VALUE.iter().any(|&(tr, name)| own_method(b, t, tr, name).is_some()) {
			return true;
		}
	}
	false
}

/// The `rt.Own` of the crate's iterator of type `ty`: each method of `OWN` but `next` that
/// it has of its own, as a function of its source, and those in `BY_VALUE` only if it is
/// `owned`. `undefined` if it has none.
fn own_methods<'tcx>(b: &mut Body<'_, '_, 'tcx>, ty: Ty<'tcx>, owned: bool, span: Span) -> Expr {
	let none = b.t.iter_item(ty.peel_refs()).and_then(|item| b.t.none_arg(item));
	let mut props = Vec::new();
	for (tr, name) in &OWN[1..] {
		if !owned && BY_VALUE.contains(&(tr, name)) {
			continue;
		}
		let Some(instance) = own_method(b, ty, tr, name) else {
			continue;
		};
		let sym = Expr::Sym(b.t.instance(instance));
		// Each that returns an `Option` of an item, as the protocol returns it: an item or
		// `rt.DONE`.
		let method = match *name {
			"next_back" | "nth" | "nth_back" | "last" => {
				let n = (*name != "next_back" && *name != "last").then_some("n");
				let args = [Expr::ident("it")].into_iter().chain(n.map(Expr::ident)).collect();
				let option = [sym.call(args)].into_iter().chain(none.clone()).collect();
				let it = Param {
					name: "it".into(),
					ty: Some(b.t.ty(ty.peel_refs(), span)),
				};
				// `n` is typed by its place in `rt.Own`.
				let params = [it].into_iter().chain(n.map(|n| Param {
					name: n.into(),
					ty: None,
				}));
				Expr::Lambda(params.collect(), Box::new(Expr::rt("iterFromOption", option)))
			}
			_ => sym,
		};
		let key = match *name {
			"size_hint" => "hint".into(),
			"next_back" => "back".into(),
			other => super::camel(other),
		};
		props.push((Key::Name(key), method));
	}
	match props.is_empty() {
		true => Expr::undefined(),
		false => Expr::Object(props),
	}
}

/// `iter.next_back()` of the crate's own iterator of type `ty`, as the protocol returns it:
/// an item or `rt.DONE`.
fn own_back<'tcx>(b: &mut Body<'_, '_, 'tcx>, ty: Ty<'tcx>, iter: Expr) -> Option<Expr> {
	let instance = own_method(b, ty, "DoubleEndedIterator", "next_back")?;
	let item = b.t.iter_item(ty.peel_refs())?;
	let sym = b.t.instance(instance);
	let none = b.t.none_arg(item);
	let option = Expr::Sym(sym).call(vec![iter]);
	Some(Expr::rt("iterFromOption", [option].into_iter().chain(none).collect()))
}

/// `iter.next()` by the protocol, for an iterator of type `ty`: an item or `rt.DONE`.
pub fn next<'tcx>(b: &mut Body<'_, '_, 'tcx>, ty: Ty<'tcx>, iter: Expr) -> Expr {
	match own_next(b, ty) {
		Some(instance) => {
			let sym = b.t.instance(instance);
			Expr::Sym(sym).call(vec![iter])
		}
		None => iter.member("next").call(Vec::new()),
	}
}

impl<'tcx> Body<'_, '_, 'tcx> {
	/// `value`, a `&`, `&mut`, or `Box` of an iterator unsized to `to`, the same pointer to a
	/// `dyn Iterator`: an `rt.iterDyn` of the `rt.Iter` it is, which reads it by the methods
	/// std's vtable has and else by `next`, as std's defaults read a trait object, and drops
	/// it if the box owned it. An upcast from another is the same `rt.Iter`.
	pub fn dyn_iter(&mut self, from: Ty<'tcx>, to: Ty<'tcx>, value: Expr, span: Span) -> Expr {
		let Some(pointee) = from.builtin_deref(true).filter(|_| to.builtin_deref(true).is_some()) else {
			return self.err(span, format!("a coercion to `{to}`, which is not supported yet"));
		};
		if self.t.dyn_iter(pointee).is_some() {
			return value;
		}
		// Another trait object, whose object has its trait's methods and is no `rt.Iter`.
		if pointee.is_trait() {
			return self.err(
				span,
				format!("an upcast from `{pointee}` to `{to}`, which is not supported yet"),
			);
		}
		// A census over type parameters: only an instance decides what the pointee is.
		if self.t.polymorphic() && pointee.has_param() {
			return Expr::undefined();
		}
		// What a box owns iterates as itself, and what a reference borrows as a `&mut` does,
		// which `into_iter` makes of a shared one. A raw pointer can be an `rt.Place`, which
		// no `rt.Iter` reads through.
		let ty = match from.kind() {
			_ if from.is_box() => pointee,
			ty::Ref(..) => from,
			_ => {
				return self.err(
					span,
					format!("a coercion of `{from}` to `{to}`, which is not supported yet"),
				);
			}
		};
		match into_iter(self, ty, value, span) {
			Some(it) => Expr::rt("iterDyn", vec![it]),
			None => self.err(
				span,
				format!("a coercion of `{from}` to `{to}`, which is not supported yet"),
			),
		}
	}
}

/// What `IntoIterator::into_iter` makes of `value`, a `ty`: an `rt.Iter`, or `None` for a
/// type without a mapping. One that owns what it yields, by value, is given what drops
/// each, and the crate's own iterator what drops it.
pub fn into_iter<'tcx>(b: &mut Body<'_, '_, 'tcx>, ty: Ty<'tcx>, value: Expr, span: Span) -> Option<Expr> {
	let (peeled, value) = peel_iter(b, ty, value);
	let it = peeled_into_iter(b, peeled, value, span)?;
	Some(boxes(b, ty, it))
}

/// `it`, the `rt.Iter` of what `peel_iter` read a `ty` through, read through `ty`'s boxes
/// as std's `Iterator` for `Box<I>` reads one: by what it forwards, and else by `next`.
/// Boxes before the first reference are one `rt.iterBox`, and a box past it makes that
/// reference fold by `next` too, whatever the box holds.
fn boxes<'tcx>(b: &Body<'_, '_, 'tcx>, mut ty: Ty<'tcx>, it: Expr) -> Expr {
	let held = |ty: Ty<'tcx>| ty.boxed_ty().filter(|held| b.t.iter_item(*held).is_some());
	let outer = held(ty).is_some();
	while let Some(inner) = held(ty) {
		ty = inner;
	}
	let mut end = ty;
	let borrowed = loop {
		if held(end).is_some() {
			break true;
		}
		match *end.kind() {
			ty::Ref(_, inner, _) => end = inner,
			_ => break false,
		}
	};
	let it = match borrowed {
		true => Expr::rt("iterBorrow", vec![Expr::rt("iterBox", vec![it])]),
		false => it,
	};
	match outer {
		true => Expr::rt("iterBox", vec![it]),
		false => it,
	}
}

/// `into_iter` of `value`, a `ty` `peel_iter` read through.
fn peeled_into_iter<'tcx>(b: &mut Body<'_, '_, 'tcx>, ty: Ty<'tcx>, value: Expr, span: Span) -> Option<Expr> {
	let tcx = b.t.tcx;
	let (ty, value) = match unbox_ref(tcx, ty) {
		unboxed if unboxed != ty => (unboxed, b.t.pointee_value(ty, value)),
		_ => (ty, value),
	};
	// A boxed slice owns its elements, as the `Vec` std's `into_iter` makes of it does, with
	// room for exactly them.
	let (ty, value) = match library::box_slice(tcx, ty) {
		Some(elem) if !matches!(elem.kind(), ty::Uint(ty::UintTy::U8)) => {
			let size = size_of(b, elem, span);
			(Ty::new_slice(tcx, elem), Expr::rt("vecExact", vec![value, size]))
		}
		Some(elem) => (Ty::new_slice(tcx, elem), value),
		None => (ty, value),
	};
	let owned = !ty.is_ref();
	if b.t.iter_item(ty).is_some() {
		not_bound(b, ty, span);
		return Some(match own_next(b, ty) {
			Some(instance) => {
				let next = Expr::Sym(b.t.instance(instance));
				let drop = match owned && b.t.drops(ty) {
					true => b.t.dropper(ty, span),
					false => Expr::undefined(),
				};
				let own = own_methods(b, ty, owned, span);
				Expr::rt("iterFrom", tail(vec![value, next], vec![drop, own]))
			}
			// One borrowed by `&mut`, which std's adaptors may drop once it ends, but which
			// is its owner's to drop, and which a consumer reads on through the borrow
			// rather than by its by-value overrides, as `Chain::count` ends both halves.
			None if !owned => Expr::rt("iterBorrow", vec![value]),
			None => value,
		});
	}
	// What drops each item of one that owns them.
	let each = |b: &mut Body<'_, '_, 'tcx>, elem: Ty<'tcx>, mut args: Vec<Expr>| {
		if owned && b.t.drops(elem) {
			args.push(b.t.dropper(elem, span));
		}
		args
	};
	// Items borrowed from what `ty` points at.
	let items = |iter: Expr, elem: Ty<'tcx>| match ty.is_ref() {
		true => by_ref(iter, elem),
		false => iter,
	};
	// Elements a `&mut` lends as their places, which JS copies.
	let places = |b: &Body<'_, '_, 'tcx>, elem: Ty<'tcx>| {
		ty.ref_mutability() == Some(ty::Mutability::Mut) && b.t.needs_place(elem)
	};
	match ty.peel_refs().kind() {
		ty::Slice(elem) | ty::Array(elem, _) if places(b, *elem) => {
			Some(Expr::rt("iterSliceMut", vec![library::elements(value, ty)]))
		}
		ty::Slice(elem) | ty::Array(elem, _) => {
			let values = each(b, *elem, vec![library::elements(value, ty)]);
			Some(items(Expr::rt("iterSlice", values), *elem))
		}
		ty::Adt(adt, args) => match Std::of(tcx, adt.did())? {
			Std::Vec if places(b, args.type_at(0)) => {
				Some(Expr::rt("iterSliceMut", vec![library::elements(value, ty)]))
			}
			// A `Vec<u8>`'s owned bytes keep its buffer, as a deque collected from them does.
			Std::Vec if owned && matches!(args.type_at(0).kind(), ty::Uint(ty::UintTy::U8)) => {
				Some(Expr::rt("byteVecIntoIter", vec![value]))
			}
			Std::Vec => {
				let values = each(b, args.type_at(0), vec![library::elements(value, ty)]);
				Some(items(Expr::rt("iterSlice", values), args.type_at(0)))
			}
			// A `&mut` lends the payload JS copies as the `Option`'s own place, while it has one.
			Std::Option if places(b, args.type_at(0)) => {
				let mut values = vec![value];
				values.extend(b.t.none_arg(args.type_at(0)));
				Some(Expr::rt("iterOption", vec![Expr::rt("optionAsMut", values)]))
			}
			// A `&mut` to one of another payload is its place, and yields the payload itself.
			Std::Option => {
				let inner = args.type_at(0);
				let mut values = each(b, inner, vec![b.t.pointee_value(ty, value)]);
				if let Some(none) = b.t.none_arg(inner) {
					values.resize(2, Expr::undefined());
					values.push(none);
				}
				Some(items(Expr::rt("iterOption", values), inner))
			}
			Std::Result if places(b, args.type_at(0)) => Some(result_places(value)),
			// One by value drops its `Err` as it makes the iterator.
			Std::Result => {
				let (ok, err) = (args.type_at(0), args.type_at(1));
				let mut values = each(b, ok, vec![b.t.pointee_value(ty, value)]);
				if owned && b.t.drops(err) {
					values.resize(2, Expr::undefined());
					values.push(b.t.dropper(err, span));
				}
				Some(items(Expr::rt("iterResult", values), ok))
			}
			_ => super::collections::into_iter(b, ty, value, span),
		},
		_ => None,
	}
}

/// The iterator of the payload of the `Result` at `place`, an `rt.Place`, as its own place.
fn result_places(place: Expr) -> Expr {
	Expr::rt("iterOption", vec![Expr::rt("resultAsMut", vec![place])])
}

/// `iter`, over the elements of a slice or `Vec` of `elem`s or the payload of an `Option`
/// or a `Result`, yielding references to them: a slice of each array of bytes.
pub(super) fn by_ref(iter: Expr, elem: Ty<'_>) -> Expr {
	match super::byte_array(elem) {
		true => Expr::rt("iterLend", vec![iter, Expr::Rt("byteSliceFrom".into())]),
		false => iter,
	}
}

/// A function doing `into_iter` on each `ty`, or `None` where it is the value itself.
fn into_fn<'tcx>(b: &mut Body<'_, '_, 'tcx>, ty: Ty<'tcx>, span: Span) -> Option<Option<Expr>> {
	let body = into_iter(b, ty, Expr::ident("value"), span)?;
	if matches!(&body, Expr::Ident(name) if name == "value") {
		return Some(None);
	}
	let param = Param {
		name: "value".into(),
		ty: Some(b.t.ty(ty, span)),
	};
	Some(Some(Expr::Lambda(vec![param], Box::new(body))))
}

/// `<ty as IntoIterator>::IntoIter`: the iterator `into_iter` makes of a `ty`.
pub(super) fn iter_of<'tcx>(t: &Translator<'_, 'tcx>, ty: Ty<'tcx>) -> Option<Ty<'tcx>> {
	let tcx = t.tcx;
	let into = tcx.get_diagnostic_item(rustc_span::sym::IntoIterator)?;
	let item = tcx
		.associated_items(into)
		.filter_by_name_unhygienic(rustc_span::Symbol::intern("IntoIter"))
		.next()?
		.def_id;
	let projection = Ty::new_projection(tcx, ty::IsRigid::No, item, [ty]);
	tcx.try_normalize_erasing_regions(t.typing_env_pub(), ty::Unnormalized::new_wip(projection))
		.ok()
}

/// `ty`, a reference to a box of a slice or an array, as one to what the box holds, which
/// JS keeps as the box.
pub fn unbox_ref<'tcx>(tcx: TyCtxt<'tcx>, ty: Ty<'tcx>) -> Ty<'tcx> {
	match *ty.kind() {
		ty::Ref(region, inner, mutability)
			if let Some(held) = inner.boxed_ty()
				&& matches!(held.kind(), ty::Slice(_) | ty::Array(..)) =>
		{
			Ty::new_ref(tcx, region, held, mutability)
		}
		_ => ty,
	}
}

/// Whether `ty` is a `&mut` to a `&mut`, which is an `rt.Place` of it, or a box of one,
/// which is what it holds.
pub fn through_mut(ty: Ty<'_>) -> bool {
	match ty.boxed_ty() {
		Some(inner) => through_mut(inner),
		None => matches!(*ty.kind(), ty::Ref(_, inner, ty::Mutability::Mut) if inner.is_ref()),
	}
}

/// `value`, a `ty`, read through each box and `&mut` to a `&mut` that std's `Iterator` for
/// `Box<I>` and `&mut I` forward through, down to the last `&mut` and its JS value.
pub fn peel_mut<'tcx>(mut ty: Ty<'tcx>, mut value: Expr) -> (Ty<'tcx>, Expr) {
	while through_mut(ty) {
		ty = match ty.boxed_ty() {
			Some(inner) => inner,
			None => {
				value = value.member("v");
				ty.builtin_deref(true).expect("a reference")
			}
		};
	}
	(ty, value)
}

/// `value`, a `ty`, read through each `&`, `&mut`, and box, in any order, that std's
/// `Iterator` for `&mut I` and `Box<I>` forward through, down to the iterator: a `&mut` to it
/// and that `&mut`'s JS value where a reference was read, else the iterator itself, and a
/// box of a trait object where none was, which drops it. JS holds a box as what it holds.
pub fn peel_iter<'tcx>(b: &Body<'_, '_, 'tcx>, mut ty: Ty<'tcx>, mut value: Expr) -> (Ty<'tcx>, Expr) {
	let tcx = b.t.tcx;
	let held = |ty: Ty<'tcx>, borrowed: bool| {
		ty.boxed_ty()
			.filter(|held| b.t.iter_item(*held).is_some() && (borrowed || !held.is_trait()))
	};
	let mut borrowed = false;
	loop {
		if let Some(inner) = held(ty, borrowed) {
			ty = inner;
			continue;
		}
		let ty::Ref(region, inner, mutability) = *ty.kind() else {
			return (ty, value);
		};
		let mut end = inner;
		while let Some(held) = held(end, true) {
			end = held;
		}
		// A reference to what is no iterator, such as a slice, iterates as it is.
		if b.t.iter_item(end).is_none() {
			return (ty, value);
		}
		borrowed = true;
		if end.is_ref() {
			value = b.t.pointee_value(ty, value);
			ty = inner;
			continue;
		}
		// The last reference: a `&mut` to the iterator is the place of the boxes before it, as
		// JS erases them, or the iterator itself, as a shared reference is too.
		return match (b.t.needs_place(end), mutability) {
			(true, ty::Mutability::Mut) => (Ty::new_mut_ref(tcx, region, end), value),
			(false, _) => (Ty::new_mut_ref(tcx, region, end), b.t.pointee_value(ty, value)),
			(true, ty::Mutability::Not) => (ty, value),
		};
	}
}

/// Refuse iterating `ty`, an iterator, if a binding stands in for it: it is no `rt.Iter`,
/// which std's adaptors and `for` take every iterator but the crate's own for.
fn not_bound<'tcx>(b: &mut Body<'_, '_, 'tcx>, ty: Ty<'tcx>, span: Span) {
	if let ty::Adt(adt, _) = ty.peel_refs().kind()
		&& b.t.bound_drops(adt.did()).is_some()
	{
		b.t.err(span, "iterating a bound type, which is not supported yet");
	}
}

/// The mapping of a call on or to an iterator, if it is one. Where std drops an item it
/// skips, rejects, or holds, or an iterator it takes by value, the runtime function is
/// given what drops it after its own arguments.
pub fn call<'tcx>(
	b: &mut Body<'_, '_, 'tcx>,
	call: &Call<'tcx>,
	owner: &str,
	name: &str,
	mut args: Vec<Expr>,
) -> Option<Expr> {
	let tcx = b.t.tcx;
	let span = call.span;
	let gargs = call.args;
	let targ = |n: usize| gargs.types().nth(n);
	let path = library::std_path(tcx, call.def_id);
	let path: Vec<&str> = path.iter().map(String::as_str).collect();

	// Where iterators come from.
	match (owner, name) {
		("slice", "iter_mut") if b.t.needs_place(targ(0)?) => {
			let s = library::elements(args.pop()?, Ty::new_slice(tcx, targ(0)?));
			return Some(Expr::rt("iterSliceMut", vec![s]));
		}
		("slice", "iter" | "iter_mut") => {
			let s = args.pop()?;
			let s = library::elements(s, Ty::new_slice(tcx, targ(0)?));
			return Some(by_ref(Expr::rt("iterSlice", vec![s]), targ(0)?));
		}
		("Option", "iter" | "iter_mut") => {
			let mut args = args;
			if let Some(none) = b.t.none_arg(targ(0)?) {
				args.extend([Expr::undefined(), none]);
			}
			return Some(Expr::rt("iterOption", args));
		}
		("Result", "iter_mut") if b.t.needs_place(targ(0)?) => return Some(result_places(args.pop()?)),
		("Result", "iter" | "iter_mut") => return Some(Expr::rt("iterResult", args)),
		// A `&mut` to a `&mut` iterator is itself, an `rt.Place` that each use reads through.
		("IntoIterator", "into_iter") if through_mut(targ(0)?) => return args.pop(),
		// So is a `&mut` iterator, which what takes it by value borrows as it does.
		("IntoIterator", "into_iter")
			if targ(0)?.ref_mutability() == Some(ty::Mutability::Mut) && b.t.iter_item(targ(0)?).is_some() =>
		{
			not_bound(b, targ(0)?, span);
			return args.pop();
		}
		("IntoIterator", "into_iter") => {
			let value = args.pop()?;
			return into_iter(b, targ(0)?, value, span);
		}
		// The `&mut` it was given, an `rt.Place` where that points at a `&mut`.
		("Iterator", "by_ref") => {
			not_bound(b, targ(0)?, span);
			return args.pop();
		}
		_ => {}
	}
	match path.as_slice() {
		["iter", "sources", "once", "once"] => {
			let dropper = b.t.dropper(targ(0)?, span);
			return Some(Expr::rt("iterOnce", tail(args, vec![dropper])));
		}
		["iter", "sources", "empty", "empty"] => return Some(Expr::rt("iterEmpty", Vec::new())),
		// `a.into_iter().zip(b)`, as std's is.
		["iter", "adapters", "zip", "zip"] => {
			let [a, other] = <[Expr; 2]>::try_from(args).ok()?;
			let a = into_iter(b, targ(0)?, a, span)?;
			return zip(b, a, targ(1)?, other, b.t.sig(call.def_id, gargs).output(), span);
		}
		// A closure that owns a value that drops is refused as any std mapping refuses one
		// it calls but never drops.
		// Each takes its closure's `None` last, where that is `rt.None`.
		["iter", "sources", "from_fn", "from_fn"] => {
			let none = b.t.none_arg(targ(0)?);
			return Some(Expr::rt("iterFromFn", [args, none.into_iter().collect()].concat()));
		}
		["iter", "sources", "repeat_with", "repeat_with"] => return Some(Expr::rt("iterRepeatWith", args)),
		// What drops the item it holds next.
		["iter", "sources", "successors", "successors"] => {
			let droppers = [b.t.dropper(targ(0)?, span)].into_iter().chain(b.t.none_arg(targ(0)?));
			return Some(Expr::rt("iterSuccessors", tail(args, droppers.collect())));
		}
		_ => {}
	}

	// What `collect` calls.
	if (owner, name) == ("FromIterator", "from_iter") {
		let source = targ(2)?;
		let iter = iter_of(b.t, source)?;
		let items = into_iter(b, source, args.pop()?, span)?;
		let trusted = implements(b.t, iter, "TrustedLen");
		return collect(b, targ(0)?, targ(1)?, items, iter, trusted, span);
	}

	// `Vec::extend` with an iterator, which it drops once done, `()`'s, and a tuple's of collections.
	if (owner, name) == ("Extend", "extend") {
		let (coll, item, source) = (targ(0)?, targ(1)?, targ(2)?);
		if !matches!(coll.kind(), ty::Tuple(_)) && b.t.std_of(coll) != Some(Std::Vec) {
			return None;
		}
		let [target, items] = <[Expr; 2]>::try_from(args).ok()?;
		let iter = iter_of(b.t, source)?;
		let mut items = into_iter(b, source, items, span)?;
		// `Extend<&T>` copies each element.
		if let Some(copy) = super::collections::copy_borrowed(b, coll, item, span) {
			items = Expr::rt("iterMap", vec![items, copy]);
		}
		return super::collections::extend(b, coll, item, target, items, iter, span);
	}

	let self_ty = match (owner, name) {
		("Iterator" | "DoubleEndedIterator" | "ExactSizeIterator", _) => targ(0)?,
		("Peekable", "peek" | "peek_mut" | "next_if" | "next_if_eq")
			if library::inherent_std(tcx, call.def_id) == Some(Std::Peekable) =>
		{
			let imp = tcx.inherent_impl_of_assoc(call.def_id)?;
			let generics = tcx.generics_of(imp).count();
			tcx.type_of(imp).instantiate(tcx, &gargs[..generics]).skip_norm_wip()
		}
		_ => return None,
	};
	let item = b.t.iter_item(self_ty.peel_refs())?;
	not_bound(b, self_ty, span);
	let mut values = args.into_iter();
	// What it reads through boxes and references, while std picks each specialization by
	// `self_ty`, which a box implements none of. `into_iter` peels `self_ty` itself, to tell
	// a box apart.
	let first = values.next()?;
	let (iter_ty, recv) = peel_iter(b, self_ty, first.clone());
	let rest: Vec<Expr> = values.collect();
	// What a consumer returns for `None`, after its other arguments, where that is `rt.None`.
	let none = b.t.none_arg(item);

	// `next` by the protocol, as an `Option`.
	if matches!(name, "next" | "next_back" | "nth_back") {
		let item = match name {
			"next" => next(b, iter_ty, recv),
			"next_back" => match own_back(b, iter_ty, recv.clone()) {
				Some(back) => back,
				None => recv.member("nextBack").call(Vec::new()),
			},
			_ => {
				let recv = into_iter(b, self_ty, first, span)?;
				let droppers = [b.t.dropper(item, span)].into_iter().chain(none).collect();
				return Some(Expr::rt("iterNthBack", tail([vec![recv], rest].concat(), droppers)));
			}
		};
		return Some(Expr::rt("iterToOption", [item].into_iter().chain(none).collect()));
	}
	let recv = into_iter(b, self_ty, first, span)?;
	// `rt.iter<Name>(recv, ...args)`, or `peekable<Name>` of a `Peekable`'s own.
	let method = |name: &str, args: Vec<Expr>| {
		let of = match owner {
			"Peekable" => "peekable",
			_ => "iter",
		};
		Some(Expr::rt_of(of, name).call([vec![recv.clone()], args].concat()))
	};
	let camel = super::camel(name);
	match name {
		"map" | "enumerate" | "rev" | "inspect" | "any" | "all" | "position" => method(&camel, rest),
		// Each takes its closure's `None` last, where that is `rt.None`.
		"filter_map" | "map_while" | "find_map" => {
			let none = b.t.none_arg(targ(1)?);
			method(&camel, [rest, none.into_iter().collect()].concat())
		}
		// What each drops: an item it skips or rejects, or one it peeked.
		"filter" | "skip_while" | "take_while" | "peekable" | "skip" | "take" | "step_by" => {
			let dropper = b.t.dropper(item, span);
			// std's `take` folds a `TrustedRandomAccess` iterator by index, as `next` does
			// one that has no side effects.
			let indexes = name == "take"
				&& implements(b.t, self_ty, "TrustedRandomAccess")
				&& may_have_side_effect(b.t, self_ty, span) != Some(false);
			method(&camel, tail(rest, vec![dropper, Expr::lit(indexes.to_string())]))
		}
		"fuse" => {
			let fused = fused(b, self_ty, None);
			method(name, tail(rest, vec![fused]))
		}
		// What clones its iterator, as it is made and each time a clone of it ends.
		"cycle" => {
			let clones = clones(b, self_ty, span)?;
			method(name, tail(rest, vec![clones]))
		}
		// `rt.owned<Name>`, which drops what `self_ty` holds and its items by `dropper` as
		// std's own does, where either drops, or counts a `Cycle` as std's `Take::count`
		// does, by its `advance_by`, which stops at a clone that yields nothing where
		// `try_fold` reads on: else `rt.iter<Name>`, so a bundle that never drops doesn't
		// carry it.
		"count" | "last" => {
			let dropper = b.t.dropper(item, span);
			let none: Vec<Expr> = none.into_iter().filter(|_| name == "last").collect();
			let cycles = name == "count"
				&& (self_ty.walk().filter_map(|arg| arg.as_type()))
					.any(|t| std_iter_path(tcx, t).is_some_and(|p| p == ["iter", "adapters", "cycle", "Cycle"]));
			match drops_iter(b, self_ty, span) || !undefined(&dropper) || cycles {
				true => Some(Expr::rt_of("owned", name).call(tail(vec![recv.clone()], [vec![dropper], none].concat()))),
				false => method(name, none),
			}
		}
		// Each owns a closure it is given by value, which drops as its fold ends.
		"for_each" | "fold" | "rfold" => {
			let drops = iter_drops(b, self_ty, span);
			let owns = flag(b.t.drops(targ(gargs.types().count() - 1)?));
			method(&camel, tail(rest, vec![drops, owns]))
		}
		// `try_fold::<B, F, R>` and `try_for_each::<F, R>`, which drop their closure as they
		// return, but not the iterator, which they borrow.
		"try_fold" | "try_rfold" | "try_for_each" => {
			let n = gargs.types().count();
			let (option, none) = try_flow(b, targ(n - 1)?)?;
			let owns = flag(b.t.drops(targ(n - 2)?));
			method(
				&camel,
				tail(rest, [flag(option), owns].into_iter().chain(none).collect()),
			)
		}
		// `partition::<B, F>`, into two of `B`'s defaults.
		"partition" => {
			let f = rest.into_iter().next()?;
			let flags = vec![iter_drops(b, self_ty, span), flag(b.t.drops(targ(2)?))];
			super::collections::partition(b, targ(1)?, item, recv.clone(), f, flags, span)
		}
		// `scan::<St, B, F>`, whose state is lent as its place where JS copies it, and drops
		// after the closure.
		"scan" => {
			let state = targ(1)?;
			let copied = flag(b.t.needs_place(state));
			let dropper = b.t.dropper(state, span);
			let none = b.t.none_arg(targ(2)?);
			method(name, tail(rest, [copied, dropper].into_iter().chain(none).collect()))
		}
		// std's `Sum` and `Product` are of the type or a reference to it: another is the crate's.
		"sum" | "product" if item.peel_refs() != targ(1)? => None,
		// `Duration`'s carries its nanoseconds and panics as std's does, not as `+` would.
		"sum" if b.t.std_of(targ(1)?) == Some(Std::Duration) => {
			let drops = iter_drops(b, self_ty, span);
			Some(Expr::rt("durationSum", tail(vec![recv.clone()], vec![drops])))
		}
		// std's float sum starts from -0.0, so a sum of none, or of -0.0s, is -0.0.
		"sum" | "product" if let Some(float) = super::float::Float::of(targ(1)?) => {
			let (op, unit) = match name {
				"sum" => ("add", -0.0),
				_ => ("mul", 1.0),
			};
			let drops = iter_drops(b, self_ty, span);
			method(
				name,
				tail(vec![Expr::rt_of(float.name(), op), float.lit(unit)], vec![drops]),
			)
		}
		// A `Wrapping`'s wraps and a `Saturating`'s saturates, as its `+` and `*` do.
		"sum" | "product" => {
			let overflowing = super::library::overflowing(b.t.tcx, targ(1)?);
			let int = Int::of(overflowing.map_or(targ(1)?, |(_, int)| int))?;
			let (op, unit) = match name {
				"sum" => ("Add", 0),
				_ => ("Mul", 1),
			};
			let op = match overflowing {
				Some((overflow, _)) => overflow.method(op),
				None => op.to_lowercase(),
			};
			let drops = iter_drops(b, self_ty, span);
			method(name, tail(vec![int.func(&op), int.lit(unit)], vec![drops]))
		}
		// `ExactSizeIterator::len` and `size_hint` are each iterator's own.
		"len" => Some(recv.member("len").call(rest)),
		"size_hint" => Some(recv.member("sizeHint").call(rest)),
		"nth" | "find" | "rfind" => {
			let dropper = b.t.dropper(item, span);
			method(&camel, tail(rest, [dropper].into_iter().chain(none).collect()))
		}
		"peek" | "next_if" => method(&camel, [rest, none.into_iter().collect()].concat()),
		// An item JS copies is lent as its place, and an object as itself.
		"peek_mut" => method(&camel, vec![Expr::lit(b.t.needs_place(item).to_string())]),
		// By the item's `PartialEq<T>`: `eq_fn` of its own type, else the impl's `eq`, as a
		// call to it lowers, given a `&T`.
		"next_if_eq" => {
			let expected = targ(1)?;
			let eq = match expected == item {
				true => super::collections::eq_fn(b, item, span),
				false => {
					let by = Ty::new_imm_ref(tcx, tcx.lifetimes.re_erased, expected);
					super::collections::impl_eq(b, "eq", item, expected, [item, by], span)
				}
			};
			method(&camel, [rest, vec![eq], none.into_iter().collect()].concat())
		}
		// By the comparator, which `best` calls with the best so far and then the next, as
		// std's `cmp::max_by` and `min_by` do.
		"max_by" | "min_by" => {
			let owns = flag(b.t.drops(targ(1)?));
			let droppers = [iter_drops(b, self_ty, span), b.t.dropper(item, span), owns]
				.into_iter()
				.chain(none);
			method(&camel, tail(rest, droppers.collect()))
		}
		"reduce" => {
			let owns = flag(b.t.drops(targ(1)?));
			let drops = iter_drops(b, self_ty, span);
			method(name, tail(rest, [drops, owns].into_iter().chain(none).collect()))
		}
		// By the items' `Ord::max` or `min`, or the keys', which test `<` of the next and the
		// best. Not of places, which the runtime would compare as the objects they are.
		"max" | "min" | "max_by_key" | "min_by_key"
			if !b.t.has_place(if name.ends_with("_by_key") { targ(1)? } else { item }) =>
		{
			let by_key = name.ends_with("_by_key");
			let key = if by_key { targ(1)? } else { item };
			// A B-tree's iterators read an end, comparing nothing, and a map's `into_keys` then
			// drops the rest with it.
			if !by_key && btree_end(tcx, self_ty, name) && b.t.drops(self_ty) {
				let back = Expr::lit((name == "max").to_string());
				return Some(Expr::rt(
					"ownedEnd",
					[recv.clone(), back].into_iter().chain(none).collect(),
				));
			}
			if !by_key && btree_end(tcx, self_ty, name) {
				let end = match name {
					"max" => recv.member("nextBack").call(Vec::new()),
					_ => recv.member("next").call(Vec::new()),
				};
				return Some(Expr::rt("iterToOption", [end].into_iter().chain(none).collect()));
			}
			if !by_key && own_ord(b, key, name) {
				return Some(b.err(
					span,
					format!("`{name}` of `{key}` through its own `Ord::{name}`, which is not supported yet"),
				));
			}
			let lt = super::collections::test_fn(b, key, "lt", span);
			// The `_by_key` forms own their function, and drop each key with its item.
			let keyed = match by_key {
				true => vec![flag(b.t.drops(targ(2)?)), b.t.dropper(key, span)],
				false => Vec::new(),
			};
			let droppers = [iter_drops(b, self_ty, span), b.t.dropper(item, span)]
				.into_iter()
				.chain(keyed)
				.chain(none);
			method(&camel, tail([rest, vec![lt]].concat(), droppers.collect()))
		}
		// Whether each item and the next are in order: by `PartialOrd::le`, by a comparator,
		// or by `le` of the keys a function makes, as std's maps to them. Not of places, as
		// `max` is not.
		"is_sorted" | "is_sorted_by" | "is_sorted_by_key" => {
			let drops = drops_iter(b, self_ty, span);
			let (items, compare, each, drops) = match name {
				"is_sorted_by" => (recv.clone(), rest.into_iter().next()?, item, drops),
				_ if b.t.has_place(item) => return None,
				"is_sorted" => (
					recv.clone(),
					super::collections::test_fn(b, item, "le", span),
					item,
					drops,
				),
				_ => {
					let key = targ(2)?;
					let le = super::collections::test_fn(b, key, "le", span);
					// std's maps the iterator by its function, which drops after it: one it is lent,
					// through a box or not, drops with its owner.
					let (mut f, f_ty) = (rest.into_iter().next()?, targ(1)?);
					let (_, lent, _) = library::called_through(b.t, f_ty, None);
					if lent && drops {
						f = Expr::rt("lent", vec![f]);
					}
					let drops = drops || b.t.drops(f_ty);
					(Expr::rt("iterMap", vec![recv.clone(), f]), le, key, drops)
				}
			};
			let drops = flag(drops);
			let mut droppers = vec![drops, b.t.dropper(each, span)];
			if name == "is_sorted_by" {
				droppers.push(b.t.dropper(targ(1)?, span));
			}
			Some(Expr::rt("iterIsSortedBy", tail(vec![items, compare], droppers)))
		}
		"rposition" => method(&camel, rest),
		"eq" | "ne" | "lt" | "le" | "gt" | "ge" | "cmp" | "partial_cmp" | "eq_by" | "cmp_by" | "partial_cmp_by" => {
			compare(b, call, name, self_ty, item, recv, rest)
		}
		"chain" => {
			let other = into_iter(b, targ(1)?, rest.into_iter().next()?, span)?;
			method(name, vec![other])
		}
		"zip" => zip(
			b,
			recv,
			targ(1)?,
			rest.into_iter().next()?,
			b.t.sig(call.def_id, gargs).output(),
			span,
		),
		// `flat_map::<U, F>`, of the source mapped by `F`, which drops as the source does.
		"flat_map" => {
			let inner = iter_of(b.t, targ(1)?)?;
			let into = into_fn(b, targ(1)?, span)?.unwrap_or_else(Expr::undefined);
			let fused = fused(b, self_ty, Some(targ(2)?));
			let size = array_size(b, targ(1)?);
			let one_shot = Expr::lit(implements(b.t, inner, "OneShot").to_string());
			method("flatMap", tail(rest, vec![into, fused, size, one_shot]))
		}
		"flatten" => {
			let inner = iter_of(b.t, item)?;
			let into = into_fn(b, item, span)?.unwrap_or_else(Expr::undefined);
			let fused = fused(b, self_ty, None);
			let size = array_size(b, item);
			let one_shot = Expr::lit(implements(b.t, inner, "OneShot").to_string());
			method("flatten", tail(Vec::new(), vec![into, fused, size, one_shot]))
		}
		"copied" | "cloned" => {
			let ty::Ref(_, inner, _) = item.kind() else {
				return None;
			};
			let clone = |b: &mut Body<'_, '_, 'tcx>| {
				let clone = tcx.lang_items().clone_fn()?;
				Some(b.fn_value(clone, tcx.mk_args(&[(*inner).into()]), span))
			};
			// A `Clone` written by hand may do other than copy, even for a `Copy` type. A JS
			// string is a value already.
			let copies = name == "copied"
				|| library::clone_copies(b.t, b.t.typing_env_pub(), *inner)
				|| library::string(tcx, *inner);
			let copy = match () {
				// Each item through its `Clone`.
				_ if !copies => Some(clone(b)?),
				// Out of the slice each item is.
				_ if super::byte_array(*inner) => Some(Expr::Rt("byteSliceToArray".into())),
				_ => b.t.copy_fn(*inner, span),
			};
			// std's `copied` skips through its iterator's `nth`, and `cloned` by its own `next`.
			match (name, copy) {
				("copied", Some(copy)) => method("copied", vec![copy]),
				("copied", None) => Some(recv),
				(_, Some(copy)) => method("map", vec![copy]),
				(_, None) => method("cloned", Vec::new()),
			}
		}
		"collect" => {
			let trusted = implements(b.t, self_ty, "TrustedLen");
			collect(b, targ(1)?, item, recv, self_ty, trusted, span)
		}
		// `extend` of the pair of collections' defaults.
		"unzip" => {
			let tuple = Ty::new_tup(tcx, &[targ(3)?, targ(4)?]);
			super::collections::tuple_extend(b, tuple, item, None, recv, self_ty, span)
		}
		_ => None,
	}
}

/// std's comparison `name` of `recv`, an iterator of type `self_ty` and items of type
/// `item`, against the `IntoIterator` first in `rest`, by the closure after it in a `_by`
/// form. Each is `iter_compare`'s: `ne` is `!eq`, and `lt`, `le`, `gt`, and `ge` test
/// `partial_cmp`. The rest compare each pair by the items' `PartialEq`, `Ord`, or
/// `PartialOrd`, in a closure that drops the pair.
fn compare<'tcx>(
	b: &mut Body<'_, '_, 'tcx>,
	call: &Call<'tcx>,
	name: &str,
	self_ty: Ty<'tcx>,
	item: Ty<'tcx>,
	recv: Expr,
	rest: Vec<Expr>,
) -> Option<Expr> {
	let span = call.span;
	let other_ty = call.args.types().nth(1)?;
	let other_iter = iter_of(b.t, other_ty)?;
	let other_item = b.t.iter_item(other_iter)?;
	let mut rest = rest.into_iter();
	let other = into_iter(b, other_ty, rest.next()?, span)?;
	let test = name.trim_end_matches("_by");
	let drops = [b.t.dropper(item, span), b.t.dropper(other_item, span)];
	let (f, owns) = match rest.next() {
		Some(f) => (f, b.t.drops(call.args.types().nth(2)?)),
		None => {
			let f = match test {
				"eq" | "ne" => super::collections::pair_eq(b, item, other_item, false, span),
				"cmp" => super::collections::cmp_fn(b, item, false, span),
				_ => super::collections::pair_partial_cmp(b, item, other_item, span),
			};
			let f = match drops.iter().all(undefined) {
				true => f,
				false => Expr::rt("consuming", [vec![f], drops.to_vec()].concat()),
			};
			(f, false)
		}
	};
	let [drop_x, drop_y] = drops;
	let compared = vec![
		iter_drops(b, self_ty, span),
		iter_drops(b, other_iter, span),
		if owns { Expr::lit("true") } else { Expr::undefined() },
		drop_x,
		drop_y,
	];
	Some(match test {
		// std's `eq` of two `TrustedLen` iterators reads neither unless their size hints agree.
		"eq" | "ne" => {
			let trusted = implements(b.t, self_ty, "TrustedLen") && implements(b.t, other_iter, "TrustedLen");
			let trusted = if trusted { Expr::lit("true") } else { Expr::undefined() };
			let eq = Expr::rt("iterEq", tail(vec![recv, other, f], [vec![trusted], compared].concat()));
			if name == "ne" { eq.not() } else { eq }
		}
		"cmp" | "partial_cmp" => Expr::rt("iterCmp", tail(vec![recv, other, f], compared)),
		_ => Expr::rt_of("iter", test).call(tail(vec![recv, other, f], compared)),
	})
}

/// `args`, then `tail` without the `undefined`s and `false`s it ends with, which a runtime
/// function's optional parameters default to.
pub(super) fn tail(mut args: Vec<Expr>, mut tail: Vec<Expr>) -> Vec<Expr> {
	while matches!(tail.last(), Some(Expr::Lit(l)) if l == "undefined" || l == "false") {
		tail.pop();
	}
	args.extend(tail);
	args
}

/// Whether `e` is `undefined`, as the dropper of what doesn't drop is.
fn undefined(e: &Expr) -> bool {
	matches!(e, Expr::Lit(l) if l == "undefined")
}

/// The size and alignment of a `ty` in bytes on wasm32, by which std's collections of it
/// reserve, or else what stands in for a figure of them: `undefined` over parameters, which
/// only a census's instance, which never runs, has.
pub(super) fn layout<'tcx>(b: &mut Body<'_, '_, 'tcx>, ty: Ty<'tcx>, span: Span) -> Result<(u64, u64), Expr> {
	match b.t.tcx.layout_of(b.t.typing_env_pub().as_query_input(ty)) {
		Ok(layout) => Ok((layout.size.bytes(), layout.align.abi.bytes())),
		Err(_) if ty.has_param() => Err(Expr::undefined()),
		Err(_) => Err(b.err(span, format!("reserving for `{ty}`, whose layout is unknown"))),
	}
}

/// The size of a `ty` in bytes, as `layout` gives it, by which std's `Vec` of it reserves.
pub(super) fn size_of<'tcx>(b: &mut Body<'_, '_, 'tcx>, ty: Ty<'tcx>, span: Span) -> Expr {
	layout(b, ty, span).map_or_else(|e| e, |(size, _)| Expr::lit(size.to_string()))
}

/// Whether an iterator of type `ty` is a `FusedIterator`, which std's `fuse` and `flatten`
/// keep once it ends rather than drop, and read on from: `true` or `false`. Only one that
/// drops, with `held` beside it, or holds an iterator of the crate's own, which may yield
/// again past its end, can tell.
fn fused<'tcx>(b: &mut Body<'_, '_, 'tcx>, ty: Ty<'tcx>, held: Option<Ty<'tcx>>) -> Expr {
	let drops = b.t.drops(ty) || held.is_some_and(|held| b.t.drops(held));
	let own = (ty.walk().filter_map(|arg| arg.as_type()))
		.any(|t| matches!(t.kind(), ty::Adt(adt, _) if b.t.own(adt.did())) && b.t.iter_item(t).is_some());
	Expr::lit(match (drops || own) && implements(b.t, ty, "FusedIterator") {
		true => "true",
		false => "false",
	})
}

/// How many items `ty` has, if it is an array or a reference to one, which std's `flatten`
/// counts toward its size hint: else `undefined`.
fn array_size<'tcx>(b: &Body<'_, '_, 'tcx>, ty: Ty<'tcx>) -> Expr {
	let ty = match *ty.kind() {
		ty::Ref(_, inner, _) => inner,
		_ => ty,
	};
	match ty.kind() {
		ty::Array(_, len) => len
			.try_to_target_usize(b.t.tcx)
			.map_or_else(Expr::undefined, |n| Expr::lit(n.to_string())),
		_ => Expr::undefined(),
	}
}

/// The path of the std type `ty` is, if it is one.
fn std_iter_path(tcx: ty::TyCtxt<'_>, ty: Ty<'_>) -> Option<Vec<String>> {
	let ty::Adt(adt, _) = ty.kind() else {
		return None;
	};
	Some(library::std_path(tcx, adt.did())).filter(|p| !p.is_empty())
}

/// The function or `Iterator` method named for `ty`, one of std's iterators of zero-sized
/// fields, as `from_fn` makes a `FromFn` and `map` a `Map`, and its generic arguments: what
/// MIR reads as a constant, rather than from that call, makes it of its fields in order.
pub(super) fn maker<'tcx>(t: &Translator<'_, 'tcx>, ty: Ty<'tcx>) -> Option<(DefId, ty::GenericArgsRef<'tcx>)> {
	let tcx = t.tcx;
	let ty::Adt(adt, args) = ty.kind() else {
		return None;
	};
	if !library::is_std(tcx, adt.did()) || !adt.is_struct() || !t.zero_sized(ty) {
		return None;
	}
	let item = t.iter_item(ty)?;
	let name = tcx
		.item_name(adt.did())
		.as_str()
		.chars()
		.enumerate()
		.fold(String::new(), |mut name, (i, c)| {
			if c.is_uppercase() && i > 0 {
				name.push('_');
			}
			name.extend(c.to_lowercase());
			name
		});
	let iterator = tcx.get_diagnostic_item(rustc_span::sym::Iterator)?;
	let method = (tcx.associated_items(iterator))
		.filter_by_name_unhygienic(rustc_span::Symbol::intern(&name))
		.find(|i| i.is_fn())
		.map(|i| i.def_id);
	let make = method.or_else(|| super::bind::resolve_in(tcx, tcx.parent(adt.did()), &[&name]))?;
	if !matches!(
		tcx.def_kind(make),
		rustc_hir::def::DefKind::Fn | rustc_hir::def::DefKind::AssocFn
	) || ty::GenericArgs::identity_for_item(tcx, make).consts().next().is_some()
	{
		return None;
	}
	// What its result names, and the item for what it doesn't, as `map`'s `B` is.
	let output = tcx
		.fn_sig(make)
		.instantiate_identity()
		.skip_norm_wip()
		.skip_binder()
		.output();
	let mut found = Vec::new();
	bind_params(output, ty, &mut found);
	let gargs = ty::GenericArgs::for_item(tcx, make, |param, _| match param.kind {
		ty::GenericParamDefKind::Lifetime => tcx.lifetimes.re_erased.into(),
		_ => (found.iter())
			.find(|&&(index, _)| index == param.index)
			.map_or(item, |&(_, ty)| ty)
			.into(),
	});
	let sig = tcx.fn_sig(make).instantiate(tcx, gargs).skip_norm_wip();
	let sig = tcx.instantiate_bound_regions_with_erased(sig);
	let fields = adt.non_enum_variant().fields.iter().map(|f| t.field_ty(f.did, args));
	(sig.output() == ty && sig.inputs().iter().copied().eq(fields)).then_some((make, gargs))
}

/// The type parameters `pattern` names, with what `ty` has in their place.
fn bind_params<'tcx>(pattern: Ty<'tcx>, ty: Ty<'tcx>, found: &mut Vec<(u32, Ty<'tcx>)>) {
	match (pattern.kind(), ty.kind()) {
		(ty::Param(param), _) => found.push((param.index, ty)),
		(ty::Adt(a, pattern_args), ty::Adt(b, args)) if a == b => {
			for (p, t) in pattern_args.types().zip(args.types()) {
				bind_params(p, t, found);
			}
		}
		_ => {}
	}
}

/// `Clone::clone` of `value`, one of std's iterators of type `ty`: its `clone`, given what
/// clones what the runtime can't clone itself, or `None` for one std's has no mapping of.
pub(super) fn clone<'tcx>(b: &mut Body<'_, '_, 'tcx>, ty: Ty<'tcx>, value: Expr, span: Span) -> Option<Expr> {
	let clones = clones(b, ty, span)?;
	Some(value.member("clone").call(tail(Vec::new(), vec![clones])))
}

/// The `rt.Clones` an iterator of type `ty` is cloned by, as std's `Clone` of it clones
/// each part, or `undefined` where each part clones as itself: `i` and `o` for the
/// iterators it wraps, `f` for its closure, and `v` for an item or another value it holds.
/// The crate's own iterator, which `into_iter` wraps, is that value, by its `Clone`.
fn clones<'tcx>(b: &mut Body<'_, '_, 'tcx>, ty: Ty<'tcx>, span: Span) -> Option<Expr> {
	let tcx = b.t.tcx;
	let value = |b: &mut Body<'_, '_, 'tcx>, ty: Ty<'tcx>| super::collections::clone_fn(b, ty, span);
	if own_next(b, ty).is_some() {
		return Some(parts(vec![("v", value(b, ty))]));
	}
	let ty::Adt(adt, args) = ty.kind() else {
		return None;
	};
	let arg = |n: usize| args.types().nth(n);
	let path = library::std_path(tcx, adt.did());
	let path: Vec<&str> = path.iter().map(String::as_str).collect();
	Some(match path.as_slice() {
		// Those that borrow what they read, as their clones do too, or hold nothing.
		["iter", "sources", "empty", "Empty"]
		| [
			"slice",
			"iter",
			"Iter" | "Windows" | "Chunks" | "ChunksExact" | "RChunks" | "RChunksExact",
		]
		| [
			"str",
			"iter",
			"Chars" | "CharIndices" | "Bytes" | "Lines" | "SplitWhitespace",
		]
		| ["ascii", "EscapeDefault"]
		| ["slice", "ascii", "EscapeAscii"]
		| ["option", "Iter"]
		| ["result", "Iter"]
		| ["ops", "range", "Range" | "RangeFrom" | "RangeInclusive"]
		| ["collections", "vec_deque", "iter", "Iter"]
		| ["collections", "linked_list", "Iter"]
		| ["collections", "binary_heap", "Iter"]
		| ["collections", "btree", "map", "Iter" | "Keys" | "Values" | "Range"]
		| ["collections", "hash", "map", "Iter" | "Keys" | "Values"]
		| [
			"collections",
			"btree" | "hash",
			"set",
			"Iter" | "Range" | "Difference" | "Intersection" | "Union" | "SymmetricDifference",
		] => Expr::undefined(),
		// std's copies what a `Vec`'s has left into a buffer of its own, even of items that
		// clone as themselves, which a deque collected from it takes.
		["vec", "into_iter", "IntoIter"] => match value(b, arg(0)?) {
			v if undefined(&v) => parts(vec![("v", Expr::Rt("itself".into()))]),
			v => parts(vec![("v", v)]),
		},
		// Those that own their items, each cloned.
		["array", "iter", "IntoIter"]
		| ["option", "IntoIter"]
		| ["result", "IntoIter"]
		| ["iter", "sources", "once", "Once"]
		| ["collections", "vec_deque", "into_iter", "IntoIter"]
		| ["collections", "linked_list", "IntoIter"]
		| ["collections", "binary_heap", "IntoIter"] => parts(vec![("v", value(b, arg(0)?))]),
		// A closure's searcher clones the closure; another holds what it searches for. One
		// from the back is `rt.iterRev` of one from the front.
		[
			"str",
			"iter",
			"Split" | "SplitTerminator" | "SplitN" | "SplitInclusive" | "Matches" | "MatchIndices",
		] => searcher(b, arg(0)?, span),
		[
			"str",
			"iter",
			"RSplit" | "RSplitTerminator" | "RSplitN" | "RMatches" | "RMatchIndices",
		] => parts(vec![("i", searcher(b, arg(0)?, span))]),
		// A slice's parts by a closure clone the closure.
		["slice", "iter", "Split" | "SplitInclusive" | "RSplit"] => parts(vec![("f", value(b, arg(1)?))]),
		// Its closure is what `rt.iterFrom` reads by.
		["iter", "sources", "from_fn", "FromFn"] => parts(vec![("v", value(b, arg(0)?))]),
		["iter", "sources", "successors", "Successors"] => {
			parts(vec![("v", value(b, arg(0)?)), ("f", value(b, arg(1)?))])
		}
		[
			"iter",
			"adapters",
			_,
			"Map" | "Filter" | "FilterMap" | "SkipWhile" | "TakeWhile" | "MapWhile" | "Inspect",
		] => parts(vec![("i", clones(b, arg(0)?, span)?), ("f", value(b, arg(1)?))]),
		[
			"iter",
			"adapters",
			_,
			"Enumerate" | "Skip" | "Take" | "StepBy" | "Fuse" | "Rev" | "Copied" | "Cloned" | "Cycle",
		] => parts(vec![("i", clones(b, arg(0)?, span)?)]),
		["iter", "adapters", _, "Chain" | "Zip"] => {
			parts(vec![("i", clones(b, arg(0)?, span)?), ("o", clones(b, arg(1)?, span)?)])
		}
		["iter", "adapters", "peekable", "Peekable"] => {
			let item = b.t.iter_item(arg(0)?)?;
			parts(vec![("i", clones(b, arg(0)?, span)?), ("v", value(b, item))])
		}
		// Its source, as `map` makes it, and the iterators of what that yields.
		["iter", "adapters", "flatten", "FlatMap"] => {
			let source = parts(vec![("i", clones(b, arg(0)?, span)?), ("f", value(b, arg(2)?))]);
			let inner = iter_of(b.t, arg(1)?)?;
			parts(vec![("i", source), ("o", clones(b, inner, span)?)])
		}
		["iter", "adapters", "flatten", "Flatten"] => {
			let inner = iter_of(b.t, b.t.iter_item(arg(0)?)?)?;
			parts(vec![("i", clones(b, arg(0)?, span)?), ("o", clones(b, inner, span)?)])
		}
		_ => return None,
	})
}

/// What clones a `str` search's searcher of `pattern`: its closure, if it is one.
fn searcher<'tcx>(b: &mut Body<'_, '_, 'tcx>, pattern: Ty<'tcx>, span: Span) -> Expr {
	match pattern.kind() {
		ty::Closure(..) => parts(vec![("f", super::collections::clone_fn(b, pattern, span))]),
		_ => Expr::undefined(),
	}
}

/// An `rt.Clones` of `parts` but those that clone as themselves, or `undefined` if all do.
fn parts(parts: Vec<(&str, Expr)>) -> Expr {
	let parts: Vec<(Key, Expr)> = (parts.into_iter())
		.filter(|(_, e)| !undefined(e))
		.map(|(name, e)| (Key::Name(name.into()), e))
		.collect();
	match parts.is_empty() {
		true => Expr::undefined(),
		false => Expr::Object(parts),
	}
}

/// Whether std's iterator `did` holds a closure that its `rt.Iter` drops, after the iterator
/// it wraps if any, as std's fields order them.
pub(super) fn holds_closure(tcx: ty::TyCtxt<'_>, did: DefId) -> bool {
	let path = library::std_path(tcx, did);
	let path: Vec<&str> = path.iter().map(String::as_str).collect();
	matches!(
		path.as_slice(),
		[
			"iter",
			"adapters",
			_,
			"Map" | "Filter" | "FilterMap" | "SkipWhile" | "TakeWhile" | "MapWhile" | "Inspect"
		] | ["iter", "adapters", "flatten", "FlatMap"]
			| ["iter", "adapters", "scan", "Scan"]
			| ["iter", "sources", "from_fn", "FromFn"]
			| ["iter", "sources", "repeat_with", "RepeatWith"]
			| ["iter", "sources", "successors", "Successors"]
			| ["collections", "linked_list", "ExtractIf"]
			| [
				"str",
				"iter",
				"Split"
					| "RSplit"
					| "SplitTerminator"
					| "RSplitTerminator"
					| "SplitN"
					| "RSplitN"
					| "SplitInclusive"
					| "Matches"
					| "RMatches"
					| "MatchIndices"
					| "RMatchIndices"
			]
			| [
				"slice",
				"iter",
				"Split"
					| "SplitMut"
					| "RSplit"
					| "RSplitMut"
					| "SplitInclusive"
					| "SplitInclusiveMut"
					| "SplitN"
					| "SplitNMut"
					| "RSplitN"
					| "RSplitNMut"
			]
	)
}

/// `rt.iterZip` of `a`, an iterator, and `other`, of type `other_ty`, into `zip`, the `Zip`
/// of their iterators.
fn zip<'tcx>(
	b: &mut Body<'_, '_, 'tcx>,
	a: Expr,
	other_ty: Ty<'tcx>,
	other: Expr,
	zip: Ty<'tcx>,
	span: Span,
) -> Option<Expr> {
	let other = into_iter(b, other_ty, other, span)?;
	let ty::Adt(_, sides) = zip.kind() else {
		return None;
	};
	let (a_iter, other_iter) = (sides.type_at(0), sides.type_at(1));
	let (item, other_item) = (b.t.iter_item(a_iter)?, b.t.iter_item(other_iter)?);
	let droppers = vec![
		zip_trusted(b, a_iter, other_iter, span),
		b.t.dropper(item, span),
		b.t.dropper(other_item, span),
	];
	Some(Expr::rt("iterZip", tail(vec![a, other], droppers)))
}

/// `rt.Trusted`, how std's `Zip` of `a` and `b` reads them, by the traits it specializes
/// on: by index if both are `TrustedRandomAccess`, as whether each may have side effects,
/// and folded by its size hint if both are `TrustedRandomAccessNoCoerce`, whose `fold`
/// reads by index as many pairs as their exact size hints, or `TrustedLen`, whose `fold`
/// reads by `next`. Their `next` reads as any other zip's does.
fn zip_trusted<'tcx>(b: &mut Body<'_, '_, 'tcx>, a: Ty<'tcx>, other: Ty<'tcx>, span: Span) -> Expr {
	let both = |name: &str| implements(b.t, a, name) && implements(b.t, other, name);
	if both("TrustedRandomAccess") {
		let effects = [a, other].map(|ty| match may_have_side_effect(b.t, ty, span) {
			Some(effect) => Expr::lit(effect.to_string()),
			None => b.err(
				span,
				format!("a zip of `{ty}`, whose `MAY_HAVE_SIDE_EFFECT` does not evaluate"),
			),
		});
		Expr::Array(effects.into())
	} else if both("TrustedRandomAccessNoCoerce") {
		Expr::Str("index".into())
	} else if both("TrustedLen") {
		Expr::Str("len".into())
	} else {
		Expr::undefined()
	}
}

/// `<ty as TrustedRandomAccessNoCoerce>::MAY_HAVE_SIDE_EFFECT`, whether std's `Zip` reads
/// an item of `ty` it never yields, evaluated as std's own constant.
fn may_have_side_effect<'tcx>(t: &Translator<'_, 'tcx>, ty: Ty<'tcx>, span: Span) -> Option<bool> {
	let tcx = t.tcx;
	let tr = core_trait(tcx, "TrustedRandomAccessNoCoerce")?;
	let item = tcx
		.associated_items(tr)
		.in_definition_order()
		.find(|item| item.name().as_str() == "MAY_HAVE_SIDE_EFFECT")?;
	let uv = rustc_middle::mir::UnevaluatedConst::new(item.def_id, tcx.mk_args(&[ty.into()]));
	let value = rustc_middle::mir::Const::Unevaluated(uv, tcx.types.bool);
	value.eval(tcx, t.typing_env_pub(), span).ok()?.try_to_bool()
}

/// Whether `ty` implements `core`'s trait `name`, one std specializes on: an iterator
/// marker it names nowhere else, or `Eq`.
pub(super) fn implements<'tcx>(t: &Translator<'_, 'tcx>, ty: Ty<'tcx>, name: &str) -> bool {
	use rustc_infer::infer::TyCtxtInferExt;
	use rustc_trait_selection::infer::InferCtxtExt;
	let tcx = t.tcx;
	let Some(tr) = core_trait(tcx, name) else {
		return false;
	};
	let (infcx, env) = tcx.infer_ctxt().build_with_typing_env(t.typing_env_pub());
	infcx.type_implements_trait(tr, [ty], env).must_apply_modulo_regions()
}

/// `core`'s trait `name`.
pub(super) fn core_trait(tcx: ty::TyCtxt<'_>, name: &str) -> Option<DefId> {
	let &core = tcx
		.crates(())
		.iter()
		.find(|&&c| tcx.crate_name(c) == rustc_span::sym::core)?;
	tcx.traits(core)
		.iter()
		.copied()
		.find(|&tr| tcx.item_name(tr).as_str() == name)
}

/// `iter.collect::<ty>()` of `item`s, from an iterator of type `from`, which it drops, and
/// which std's reserves for as it does a `TrustedLen` one if `trusted`.
fn collect<'tcx>(
	b: &mut Body<'_, '_, 'tcx>,
	ty: Ty<'tcx>,
	item: Ty<'tcx>,
	iter: Expr,
	from: Ty<'tcx>,
	trusted: bool,
	span: Span,
) -> Option<Expr> {
	let tcx = b.t.tcx;
	// `()`'s reads every item, each a `()` too, by `for_each`, which folds: from `undefined`,
	// so the result is typed as `()` is.
	if ty.is_unit() {
		let drops = iter_drops(b, from, span);
		let each = Expr::Lambda(Vec::new(), Box::new(Expr::undefined()));
		return Some(Expr::rt(
			"iterFold",
			tail(vec![iter, Expr::undefined(), each], vec![drops]),
		));
	}
	// A tuple's, which `extend`s the collections' defaults.
	if let ty::Tuple(_) = ty.kind() {
		return super::collections::tuple_extend(b, ty, item, None, iter, from, span);
	}
	let ty::Adt(adt, args) = ty.kind() else {
		return None;
	};
	let std = Std::of(tcx, adt.did())?;
	// An `Arc<[T]>` holds the `Box<[T]>` std's collects.
	if std == Std::Arc
		&& let Some(boxed) = super::task::shared_box(tcx, args.type_at(0))
	{
		let boxed = collect(b, boxed, item, iter, from, trusted, span)?;
		return Some(Expr::New(Box::new(Expr::Rt("Arc".into())), vec![boxed]));
	}
	// A `Cow<[T]>` owns the `Vec` std's collects, and a `Box<[T]>` becomes it.
	let boxed = library::box_slice(tcx, ty);
	let elem = match std {
		Std::Vec => Some(args.type_at(0)),
		_ => library::cow_slice(tcx, ty).or(boxed),
	};
	if let Some(elem) = elem {
		let v = match elem.kind() {
			ty::Uint(ty::UintTy::U8) => {
				let (take, by_index, src) = collect_take(b, from, elem, span);
				Expr::rt(
					"iterToByteVec",
					tail(
						vec![iter],
						vec![take, Expr::lit((trusted || by_index).to_string()), src_size(src)],
					),
				)
			}
			_ => to_array(b, iter, from, elem, trusted, span),
		};
		return Some(match boxed {
			Some(elem) => library::vec_boxed(v, elem),
			None => v,
		});
	}
	match std {
		// The values, collected as the `Ok` or `Some` type is, until the first `Err` or `None`.
		std @ (Std::Result | Std::Option) => {
			let ty::Adt(_, item_args) = item.kind() else {
				return None;
			};
			let value = item_args.type_at(0);
			let value_ty = b.t.ty(value, span);
			// What it collects from is what `from` yields up to the first `Err` or `None`, which
			// std's collects in place of `from`'s source as it would `from`, but never trusts
			// the length of.
			// A tuple's `Extend` is given the `GenericShunt` std's reads `from` through, which a
			// 1-tuple's collection's own `extend` sees as it is.
			let values_from = match sees_shunt(b.t, args.type_at(0)) {
				true => shunt(b.t, ty, item, from)?,
				false => from,
			};
			let values = collect(
				b,
				args.type_at(0),
				value,
				Expr::ident("values"),
				values_from,
				false,
				span,
			)?;
			let param = Param {
				name: "values".into(),
				ty: Some(Type::Rt("Iter", vec![value_ty])),
			};
			let dropper = b.t.dropper(args.type_at(0), span);
			let collected = vec![iter, Expr::Lambda(vec![param], Box::new(values))];
			Some(match std {
				Std::Result => Expr::rt("iterCollectResult", tail(collected, vec![dropper])),
				// The items' `None`, and the result's, each `rt.None` where `undefined` is a value.
				_ => {
					let nones = [value, args.type_at(0)].map(|t| b.t.none(t));
					Expr::rt(
						"iterCollectOption",
						tail(collected, [vec![dropper], nones.into()].concat()),
					)
				}
			})
		}
		// From chars or text, as `extend` of an empty one, but for an owned string's, which
		// the rest extend once `next` gives it. A `Cow<str>` and a `Box<str>` own the `String`
		// std's collects.
		Std::String | Std::Cow | Std::Box if std == Std::String || args.types().next().is_some_and(|t| t.is_str()) => {
			match b.t.std_of(item) {
				Some(Std::String | Std::Cow) => {
					let drops = iter_drops(b, from, span);
					Some(Expr::rt("stringFromStrings", tail(vec![iter], vec![drops])))
				}
				_ => string_extend(b, Expr::Str(String::new()), iter, item, from, span),
			}
		}
		_ => super::collections::collect(b, ty, iter, from, trusted, span),
	}
}

/// Whether `ty`, collected through a `GenericShunt`, is given it: a tuple, whose `Extend`
/// is, or a `Result` or `Option` of one, which shunts it again.
fn sees_shunt<'tcx>(t: &Translator<'_, 'tcx>, ty: Ty<'tcx>) -> bool {
	match ty.kind() {
		ty::Tuple(_) => true,
		ty::Adt(_, args) if matches!(t.std_of(ty), Some(Std::Result | Std::Option)) => sees_shunt(t, args.type_at(0)),
		_ => false,
	}
}

/// The `GenericShunt` std's `collect` into `ty`, a `Result` or an `Option`, of its `item`s
/// from an iterator of type `from` reads them through, as the values it collects.
fn shunt<'tcx>(t: &Translator<'_, 'tcx>, ty: Ty<'tcx>, item: Ty<'tcx>, from: Ty<'tcx>) -> Option<Ty<'tcx>> {
	let tcx = t.tcx;
	let from_iter = super::collections::trait_fn(tcx, "FromIterator", "from_iter");
	let args = tcx.mk_args(&[ty.into(), item.into(), from.into()]);
	// `try_process(iter, |shunt| shunt.collect())`.
	let args = library::callee_args(t, from_iter, args, |f| tcx.item_name(f).as_str() == "try_process")?;
	let ty::Closure(_, closure) = args.type_at(3).kind() else {
		return None;
	};
	let sig = tcx.instantiate_bound_regions_with_erased(closure.as_closure().sig());
	sig.inputs().first()?.tuple_fields().first().copied()
}

/// `s` extended by `items`, an iterator of type `iter` of `item`s, as `String`'s `Extend`
/// reads it, by `for_each`: of chars, through any `&`, or of text, a `&str`, `String`,
/// `Box<str>`, or `Cow<str>`.
pub(super) fn string_extend<'tcx>(
	b: &mut Body<'_, '_, 'tcx>,
	s: Expr,
	items: Expr,
	item: Ty<'tcx>,
	iter: Ty<'tcx>,
	span: Span,
) -> Option<Expr> {
	let extend = if text_items(b.t, item)? {
		"stringExtend"
	} else {
		"stringExtendChars"
	};
	let drops = iter_drops(b, iter, span);
	Some(Expr::rt(extend, tail(vec![s, items], vec![drops])))
}

/// Whether a `String` extends by `item`s of text, a `&str`, `String`, `Box<str>`, or
/// `Cow<str>`, rather than chars, through any `&`: `None` for any other.
pub(super) fn text_items<'tcx>(t: &Translator<'_, 'tcx>, item: Ty<'tcx>) -> Option<bool> {
	match *item.peel_refs().kind() {
		ty::Char => Some(false),
		ty::Str => Some(true),
		ty::Adt(_, args) => match t.std_of(item)? {
			Std::String => Some(true),
			Std::Box | Std::Cow => Some(args.types().next().is_some_and(|t| t.is_str())),
			_ => None,
		},
		_ => None,
	}
}

/// How a `try_` fold's closure returning an `r` breaks: by `None`, and the `rt.None` that is
/// where it is, if it is an `Option`, or a `ControlFlow` held as one, and else by `Err`, if
/// it is a `Result`, or a `ControlFlow` held as one.
fn try_flow<'tcx>(b: &Body<'_, '_, 'tcx>, r: Ty<'tcx>) -> Option<(bool, Option<Expr>)> {
	let ty::Adt(_, args) = r.kind() else {
		return None;
	};
	match b.t.std_of(r)? {
		Std::Option => Some((true, b.t.none_arg(args.type_at(0)))),
		Std::ControlFlow if library::Flow::of(b.t.tcx, args) == library::Flow::Option => {
			Some((true, b.t.none_arg(args.type_at(1))))
		}
		Std::Result | Std::ControlFlow => Some((false, None)),
		_ => None,
	}
}

/// Whether an iterator of type `ty` that a runtime function takes by value drops what it
/// holds, which its `rt.Iter` drops as std's does: `true`, or `undefined` for one that
/// drops nothing.
pub(super) fn iter_drops<'tcx>(b: &mut Body<'_, '_, 'tcx>, ty: Ty<'tcx>, span: Span) -> Expr {
	flag(drops_iter(b, ty, span))
}

/// A runtime function's optional flag: `true`, or `undefined` for its default.
fn flag(on: bool) -> Expr {
	match on {
		true => Expr::lit("true"),
		false => Expr::undefined(),
	}
}

/// How a runtime function takes an iterator of type `ty` by value, as `rt.Take` says: by
/// `next` unless std's `folds` it, and then by its own `fold`, or `undefined` for one that
/// drops nothing, which it reads as `folded` steps it.
pub(super) fn iter_take<'tcx>(b: &mut Body<'_, '_, 'tcx>, ty: Ty<'tcx>, folds: bool, span: Span) -> Expr {
	match (drops_iter(b, ty, span), folds) {
		(_, false) => Expr::Str("next".into()),
		(true, true) => Expr::Str("fold".into()),
		(false, true) => Expr::undefined(),
	}
}

/// Whether a runtime function takes an iterator of type `ty` by value as std's consumers
/// do, by its `fold` or its kind's own `count` and `last`, rather than as `folded` steps
/// it: if it drops, refusing one it can't drop, as the crate's own drops through
/// `rt.iterFrom`, or holds one of the crate's own that `folds_own`.
fn drops_iter<'tcx>(b: &mut Body<'_, '_, 'tcx>, ty: Ty<'tcx>, span: Span) -> bool {
	if !b.t.drops(ty) {
		return folds_own(b, ty);
	}
	if own_next(b, ty).is_none() {
		b.t.dropper(ty, span);
	}
	true
}

/// `rt.iterToArray` of `iter`, an iterator of type `from`, collected into a `Vec<elem>` as
/// `collect_take` says std's reads it. It reserves as std's does for a `TrustedLen`
/// iterator if `trusted`.
pub(super) fn to_array<'tcx>(
	b: &mut Body<'_, '_, 'tcx>,
	iter: Expr,
	from: Ty<'tcx>,
	elem: Ty<'tcx>,
	trusted: bool,
	span: Span,
) -> Expr {
	let (take, by_index, src) = collect_take(b, from, elem, span);
	let size = size_of(b, elem, span);
	Expr::rt(
		"iterToArray",
		tail(
			vec![iter],
			vec![take, size, Expr::lit((trusted || by_index).to_string()), src_size(src)],
		),
	)
}

/// The size of the elements of the `Vec` a `collect` writes in place of, which the runtime
/// keeps the capacity of as std's does, or `undefined`.
fn src_size(src: Option<u64>) -> Expr {
	src.map_or_else(Expr::undefined, |size| Expr::lit(size.to_string()))
}

/// How std's `collect` of an iterator of type `from` into a `Vec<elem>` takes it, as
/// `rt.Take`, and whether by index, reading no item past its length. In place of the `Vec`
/// its items come from, it reads a `TrustedRandomAccessNoCoerce` one by index and any other
/// by `try_fold`, then drops it whole, as reading by `next` and then dropping it does but
/// for a zip, whose `next` reads a side once past the other, its length reserved as a
/// `TrustedLen` one's. Elsewhere, it folds a `TrustedLen` one by its own `fold`, which
/// drops what it holds first, and reads any other by `next`. Last, the size of the elements
/// of the `Vec` it writes in place of, if it does.
fn collect_take<'tcx>(
	b: &mut Body<'_, '_, 'tcx>,
	from: Ty<'tcx>,
	elem: Ty<'tcx>,
	span: Span,
) -> (Expr, bool, Option<u64>) {
	let Some(src) = in_place(b.t, from, elem) else {
		let folds = implements(b.t, from, "TrustedLen");
		return (iter_take(b, from, folds, span), false, None);
	};
	let zips = from.walk().filter_map(|arg| arg.as_type()).any(|t| {
		matches!(t.kind(), ty::Adt(adt, _) if library::std_path(b.t.tcx, adt.did()).last().map(String::as_str) == Some("Zip"))
	});
	if zips && implements(b.t, from, "TrustedRandomAccessNoCoerce") {
		return (Expr::Str("index".into()), true, Some(src));
	}
	(iter_take(b, from, false, span), false, Some(src))
}

/// The size of the elements of the `Vec` std's `collect` of an iterator of type `iter` into
/// a `Vec<elem>` writes in place of, if it does, as its adaptors let it where each item's
/// layout leaves room.
fn in_place<'tcx>(t: &Translator<'_, 'tcx>, iter: Ty<'tcx>, elem: Ty<'tcx>) -> Option<u64> {
	let tcx = t.tcx;
	let env = t.typing_env_pub();
	let (Some(steps), Some(source)) = (core_trait(tcx, "InPlaceIterable"), core_trait(tcx, "SourceIter")) else {
		return None;
	};
	if !implements(t, iter, "InPlaceIterable") || !implements(t, iter, "SourceIter") {
		return None;
	}
	let assoc = |tr: DefId, name: &str| {
		tcx.associated_items(tr)
			.filter_by_name_unhygienic(rustc_span::Symbol::intern(name))
			.next()
			.map(|item| item.def_id)
	};
	// `<I as SourceIter>::Source`, a `vec::IntoIter` of what it collects in place of.
	let src = assoc(source, "Source").and_then(|item| {
		let projection = Ty::new_projection(tcx, ty::IsRigid::No, item, [iter]);
		tcx.try_normalize_erasing_regions(env, ty::Unnormalized::new_wip(projection))
			.ok()
	})?;
	let ty::Adt(adt, src_args) = src.kind() else {
		return None;
	};
	if library::std_path(tcx, adt.did()) != ["vec", "into_iter", "IntoIter"] {
		return None;
	}
	// `MERGE_BY` and `EXPAND_BY`: how many source items make how many it collects, if known.
	let step = |name: &str| -> Option<u128> {
		let item = assoc(steps, name)?;
		let instance = Instance::try_resolve(tcx, env, item, tcx.mk_args(&[iter.into()])).ok()??;
		let value = tcx.const_eval_instance(env, instance, rustc_span::DUMMY_SP).ok()?;
		Some(value.try_to_scalar_int()?.to_bits_unchecked())
	};
	let (Some(merge), Some(expand)) = (step("MERGE_BY"), step("EXPAND_BY")) else {
		return None;
	};
	let layout = |ty: Ty<'tcx>| tcx.layout_of(env.as_query_input(ty)).ok();
	let (Some(from), Some(to)) = (layout(src_args.type_at(0)), layout(elem)) else {
		return None;
	};
	// As std's `in_place_collectible`, where `None`, an overflow, is 0.
	(!from.is_zst()
		&& !to.is_zst()
		&& from.align.abi == to.align.abi
		&& merge > 0
		&& expand > 0
		&& u128::from(from.size.bytes()) * merge >= u128::from(to.size.bytes()) * expand)
		.then_some(from.size.bytes())
}

/// Whether the crate's own `Ord` of `ty` has its own `max` or `min`, by `name`, which
/// `Iterator::max` or `min` calls. A reference's that std's forwards through is the
/// default, which tests `<`.
fn own_ord<'tcx>(b: &Body<'_, '_, 'tcx>, ty: Ty<'tcx>, name: &str) -> bool {
	let tcx = b.t.tcx;
	let f = super::collections::trait_fn(tcx, "Ord", name);
	super::collections::by_own(b, super::collections::peel_ord(b, ty, false))
		&& Instance::try_resolve(tcx, b.t.typing_env_pub(), f, tcx.mk_args(&[ty.into()]))
			.ok()
			.flatten()
			.is_some_and(|instance| instance.def_id() != f)
}

/// Whether std's iterator `ty` has its own `min` or `max`, by `name`, which reads its first
/// or last item, as a B-tree's do.
fn btree_end(tcx: ty::TyCtxt<'_>, ty: Ty<'_>, name: &str) -> bool {
	let Some(path) = std_iter_path(tcx, ty) else {
		return false;
	};
	let path: Vec<&str> = path.iter().map(String::as_str).collect();
	match path[..] {
		[
			"collections",
			"btree",
			"map",
			"Iter" | "IterMut" | "Keys" | "Range" | "RangeMut" | "IntoKeys",
		]
		| ["collections", "btree", "set", "Iter" | "Range"] => true,
		[
			"collections",
			"btree",
			"set",
			"Difference" | "SymmetricDifference" | "Intersection" | "Union",
		] => name == "min",
		_ => false,
	}
}
