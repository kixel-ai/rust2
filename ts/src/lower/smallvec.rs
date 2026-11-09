//! smallvec's `SmallVec`, which translation maps itself, as it maps hashbrown's tables to
//! std's: a `SmallVec<[T; N]>` is the runtime's `rs.Vec<T>`, whose capacity
//! `rs/smallvec.ts` reads and sets as smallvec's code does, from `N`, and its `Drain` and
//! `IntoIter` are std's `Vec`'s. Its methods whose behaviour is `Vec`'s map as `Vec`'s, and
//! the others to `rs/smallvec.ts`; its `unsafe` and raw-pointer methods, and the rest it
//! has no mapping of, are refused.

use rustc_attr_ir::LangItem;
use rustc_hir::def::DefKind;
use rustc_hir::def_id::{CrateNum, DefId};
use rustc_middle::ty::{self, GenericArg, Ty, TyCtxt, TypeFoldable, TypeFolder, TypeSuperFoldable, TypeVisitableExt};
use rustc_span::{Span, Symbol, sym};

use super::body::Body;
use super::library::{self, Call};
use crate::ts::{Expr, Stmt};

/// Whether `krate` is smallvec.
pub fn is_crate(tcx: TyCtxt<'_>, krate: CrateNum) -> bool {
	tcx.crate_name(krate).as_str() == "smallvec"
}

/// Whether `did`, an item of smallvec, maps rather than translates: all of it but
/// `CollectionAllocErr`, the error of its `try_` methods, and that error's impls, which
/// translate from its MIR.
pub fn mapped(tcx: TyCtxt<'_>, did: DefId) -> bool {
	let error =
		|ty: Ty<'_>| matches!(ty.kind(), ty::Adt(adt, _) if tcx.item_name(adt.did()).as_str() == "CollectionAllocErr");
	let mut item = did;
	loop {
		match tcx.def_kind(item) {
			DefKind::Impl { .. } => return !error(tcx.type_of(item).instantiate_identity().skip_norm_wip()),
			DefKind::Enum if tcx.item_name(item).as_str() == "CollectionAllocErr" => return false,
			_ => {}
		}
		match tcx.opt_parent(item) {
			Some(parent) => item = parent,
			None => return true,
		}
	}
}

/// What `ty` is in JS if it is smallvec's `SmallVec`, `Drain`, or `IntoIter` of an array:
/// std's `Vec`, `vec::Drain`, or `vec::IntoIter` of its elements.
pub fn as_vec<'tcx>(tcx: TyCtxt<'tcx>, ty: Ty<'tcx>) -> Option<Ty<'tcx>> {
	let ty::Adt(adt, args) = ty.kind() else {
		return None;
	};
	if adt.did().is_local() || !is_crate(tcx, adt.did().krate) {
		return None;
	}
	let ty::Array(elem, _) = args.types().last()?.kind() else {
		return None;
	};
	let vec = tcx.get_diagnostic_item(sym::Vec)?;
	let global = Ty::new_adt(
		tcx,
		tcx.adt_def(tcx.lang_items().get(LangItem::GlobalAlloc)?),
		ty::List::empty(),
	);
	let (did, args): (DefId, Vec<GenericArg<'tcx>>) = match tcx.item_name(adt.did()).as_str() {
		"SmallVec" => (vec, vec![(*elem).into(), global.into()]),
		"IntoIter" => (vec_item(tcx, vec, "IntoIter")?, vec![(*elem).into(), global.into()]),
		"Drain" => (
			vec_item(tcx, vec, "Drain")?,
			vec![args.region_at(0).into(), (*elem).into(), global.into()],
		),
		_ => return None,
	};
	Some(Ty::new_adt(tcx, tcx.adt_def(did), tcx.mk_args(&args)))
}

/// `value` with each of smallvec's types in it replaced by what [`as_vec`] says it is.
pub fn fold<'tcx, T: TypeFoldable<TyCtxt<'tcx>>>(tcx: TyCtxt<'tcx>, value: T) -> T {
	value.fold_with(&mut AsVec { tcx })
}

struct AsVec<'tcx> {
	tcx: TyCtxt<'tcx>,
}

impl<'tcx> TypeFolder<TyCtxt<'tcx>> for AsVec<'tcx> {
	fn cx(&self) -> TyCtxt<'tcx> {
		self.tcx
	}

	fn fold_ty(&mut self, t: Ty<'tcx>) -> Ty<'tcx> {
		let t = t.super_fold_with(self);
		as_vec(self.tcx, t).unwrap_or(t)
	}
}

/// The item `name` of the module of std's `Vec`.
fn vec_item(tcx: TyCtxt<'_>, vec: DefId, name: &str) -> Option<DefId> {
	tcx.module_children(tcx.parent(vec))
		.iter()
		.find(|c| c.ident.as_str() == name)?
		.res
		.opt_def_id()
}

/// The `[T; N]` of `ty` if it is a `SmallVec<[T; N]>`: its elements and `N`, if known.
fn array<'tcx>(tcx: TyCtxt<'tcx>, ty: Ty<'tcx>) -> Option<(Ty<'tcx>, Option<u64>)> {
	let ty::Adt(adt, args) = ty.kind() else {
		return None;
	};
	if !is_crate(tcx, adt.did().krate) || tcx.item_name(adt.did()).as_str() != "SmallVec" {
		return None;
	}
	match args.type_at(0).kind() {
		ty::Array(elem, n) => Some((*elem, n.try_to_target_usize(tcx))),
		_ => None,
	}
}

/// A call of smallvec's function or of a std trait's on a `SmallVec`: its mapping, `None` if
/// it has none, or its arguments back if it is no such call.
pub fn call<'tcx>(
	b: &mut Body<'_, '_, 'tcx>,
	call: &Call<'tcx>,
	owner: &str,
	name: &str,
	args: Vec<Expr>,
	out: &mut Vec<Stmt>,
) -> Result<Option<Expr>, Vec<Expr>> {
	let tcx = b.t.tcx;
	let span = call.span;
	let self_ty = call.args.types().next();
	if call.def_id.is_local() || !is_crate(tcx, call.def_id.krate) {
		// `Into::into` into one is its `From::from`.
		if (owner, name) == ("Into", "into")
			&& let Some(to) = call.args.types().nth(1)
			&& array(tcx, to).is_some()
		{
			let from = super::collections::trait_fn(tcx, "From", "from");
			let call = Call {
				def_id: from,
				args: tcx.mk_args(&[to.into(), self_ty.expect("a Self").into()]),
				span,
			};
			return Ok(trait_call(b, &call, "From", "from", to, args, out));
		}
		return match self_ty {
			Some(ty) if array(tcx, ty.peel_refs()).is_some() => Ok(trait_call(b, call, owner, name, ty, args, out)),
			_ => Err(args),
		};
	}
	// Its `Drain`'s and `IntoIter`'s own `as_slice`, and its `ToSmallVec`.
	let Some(imp) = tcx.inherent_impl_of_assoc(call.def_id) else {
		return Ok(None);
	};
	let self_ty = tcx.type_of(imp).instantiate(tcx, call.args).skip_norm_wip();
	if array(tcx, self_ty).is_none() {
		return Ok(None);
	}
	let elem = match known(b, self_ty, owner, name, span) {
		Ok((elem, _)) => elem,
		Err(e) => return Ok(Some(e)),
	};
	let (_, Some(inline)) = array(tcx, self_ty).expect("a SmallVec") else {
		unreachable!("a known array");
	};
	Ok(inherent(b, call, name, elem, inline, args, out))
}

/// The elements and inline size of `ty`, a `SmallVec` that `owner::name` is called on, or
/// its refusal where it has no mapping: of an array of a length not known, of a type
/// parameter's elements, which would be an `rs.VecOf`, of bytes, which would be an
/// `rs.ByteVec`, or of arrays, which a `Vec`'s slice keeps as an `rs.ArraySlice`.
fn known<'tcx>(
	b: &mut Body<'_, '_, 'tcx>,
	ty: Ty<'tcx>,
	owner: &str,
	name: &str,
	span: Span,
) -> Result<(Ty<'tcx>, u64), Expr> {
	let (elem, inline) = array(b.t.tcx, ty).expect("a SmallVec");
	let why = match inline {
		None => Some("whose array's length is not known"),
		Some(_) => unsupported(elem),
	};
	match (why, inline) {
		(None, Some(inline)) => Ok((elem, inline)),
		(why, _) => Err(b.err(
			span,
			format!(
				"`{owner}::{name}` of `{ty}`, {}, which is not supported yet",
				why.unwrap_or_default()
			),
		)),
	}
}

/// Why a `SmallVec` of `elem`s has no mapping, if it has none: what its `Vec` would be
/// that a `SmallVec`'s mapping doesn't follow.
fn unsupported(elem: Ty<'_>) -> Option<&'static str> {
	if elem.has_param() || elem.has_aliases() {
		Some("of a type parameter's elements")
	} else if super::byte(elem) {
		Some("of bytes")
	} else if elem.is_array() {
		Some("of arrays")
	} else {
		None
	}
}

/// One of `SmallVec`'s own methods, of a `SmallVec<[elem; inline]>`.
fn inherent<'tcx>(
	b: &mut Body<'_, '_, 'tcx>,
	call: &Call<'tcx>,
	name: &str,
	elem: Ty<'tcx>,
	inline: u64,
	mut args: Vec<Expr>,
	out: &mut Vec<Stmt>,
) -> Option<Expr> {
	let span = call.span;
	let size = super::iter::size_of(b, elem, span);
	let zero = super::iter::layout(b, elem, span).is_ok_and(|(size, _)| size == 0);
	let n = inline_size(inline, zero);
	let rs = |f: &str, args: Vec<Expr>| Some(Expr::rs(f, args));
	match name {
		// As `Vec`'s, which never read the capacity.
		"new" | "len" | "is_empty" | "push" | "pop" | "as_slice" | "as_mut_slice" | "retain_mut" => {
			vec_call(b, call, elem, name, args, out)
		}
		// smallvec's `retain` lends each item as a `&mut`, as `Vec::retain_mut` does.
		"retain" => vec_call(b, call, elem, "retain_mut", args, out),
		"inline_size" => Some(n),
		"capacity" => rs("smallVecCapacity", vec![args.pop()?, n]),
		"spilled" => rs("smallVecSpilled", vec![args.pop()?, n]),
		"with_capacity" => {
			let at = match at(b, call, Panic::Infallible) {
				Ok(at) => at,
				Err(e) => return Some(e),
			};
			rs("smallVecWithCapacity", [args, vec![n, size], at].concat())
		}
		"reserve" | "reserve_exact" => {
			let at = match at(b, call, Panic::Infallible) {
				Ok(at) => at,
				Err(e) => return Some(e),
			};
			let exact = Expr::lit((name == "reserve_exact").to_string());
			rs("smallVecReserve", [args, vec![n, size, exact], at].concat())
		}
		"try_grow" | "try_reserve" | "try_reserve_exact" => {
			let ok = match name {
				"try_grow" => Expr::rs("smallVecTryGrow", [args, vec![n, size]].concat()),
				_ => {
					let exact = Expr::lit((name == "try_reserve_exact").to_string());
					Expr::rs("smallVecTryReserve", [args, vec![n, size, exact]].concat())
				}
			};
			// `Result<(), CollectionAllocErr>`, whose error translates.
			let ty::Adt(_, result) = b.t.sig(call.def_id, call.args).output().kind() else {
				return None;
			};
			let ty::Adt(error, eargs) = result.type_at(1).kind() else {
				return None;
			};
			let overflow = error
				.variants()
				.iter()
				.find(|v| v.name.as_str() == "CapacityOverflow")?
				.ctor_def_id()?;
			let overflow = b.call_def(overflow, eargs, Vec::new(), span, out);
			Some(Expr::Cond(
				Box::new(ok),
				Box::new(Expr::Rs("OK".into())),
				Box::new(Expr::rs("Err", vec![overflow])),
			))
		}
		"grow" => {
			let at = match at(b, call, Panic::Infallible) {
				Ok(at) => at,
				Err(e) => return Some(e),
			};
			rs("smallVecGrow", [args, vec![n, size], at].concat())
		}
		"shrink_to_fit" => rs("smallVecShrinkToFit", [args, vec![n, size]].concat()),
		"from_vec" => rs("smallVecFromVec", [args, vec![n, size]].concat()),
		"into_vec" => rs("smallVecIntoVec", [args, vec![n, size]].concat()),
		"into_boxed_slice" => {
			let v = Expr::rs("smallVecIntoVec", [args, vec![n, size]].concat());
			vec_call(b, call, elem, "into_boxed_slice", vec![v], out)
		}
		"into_inner" => {
			let v = b.spill(args.pop()?, "small", out);
			let items = Expr::rs("smallVecIntoInner", vec![v.clone(), n, Expr::lit(inline.to_string())]);
			let items = b.spill(items, "inline", out);
			Some(Expr::Cond(
				Box::new(items.clone().binary("===", Expr::undefined())),
				Box::new(Expr::rs("Err", vec![v])),
				Box::new(Expr::rs("Ok", vec![items])),
			))
		}
		"from_buf" => Some(Expr::New(Box::new(Expr::Rs("Vec".into())), args)),
		"from_buf_and_len" => {
			let at = match at(b, call, Panic::Own) {
				Ok(at) => at,
				Err(e) => return Some(e),
			};
			rs(
				"smallVecFromBufAndLen",
				[args, vec![Expr::lit(inline.to_string())], at].concat(),
			)
		}
		"from_slice" => {
			let copy = b.t.copy_fn(elem, span).unwrap_or_else(Expr::undefined);
			rs("smallVecFromSlice", [args, vec![n, copy]].concat())
		}
		"from_elem" => {
			let clone = super::collections::clone_fn(b, elem, span);
			let drop = b.t.dropper(elem, span);
			rs("smallVecFromElem", [args, vec![n, clone, drop]].concat())
		}
		"truncate" | "clear" => {
			if name == "clear" {
				args.push(Expr::lit("0"));
			}
			args.push(b.t.dropper(elem, span));
			rs("smallVecTruncate", args)
		}
		"insert" => {
			let at = match at(b, call, Panic::Own) {
				Ok(at) => at,
				Err(e) => return Some(e),
			};
			rs("smallVecInsert", [args, vec![n, size], at].concat())
		}
		"remove" => {
			let at = match at(b, call, Panic::Own) {
				Ok(at) => at,
				Err(e) => return Some(e),
			};
			rs("smallVecRemove", [args, at].concat())
		}
		"swap_remove" => {
			let at = match at(b, call, Panic::Unlocated) {
				Ok(at) => at,
				Err(e) => return Some(e),
			};
			rs("smallVecSwapRemove", [args, at].concat())
		}
		"drain" => {
			let at = match at(b, call, Panic::Unlocated) {
				Ok(at) => at,
				Err(e) => return Some(e),
			};
			let ranged = super::collections::bounds(b, call, args.pop()?);
			args.push(ranged.bounds.clone());
			args.push(b.t.dropper(elem, span));
			Some(ranged.after(Expr::rs("smallVecDrain", [args, at].concat())))
		}
		"extend_from_slice" | "insert_from_slice" => {
			let at = match at(b, call, Panic::Own) {
				Ok(at) => at,
				Err(e) => return Some(e),
			};
			if name == "extend_from_slice" {
				let v = b.spill(args.remove(0), "small", out);
				let len = v.clone().member("items").member("length");
				args.splice(0..0, [v, len]);
			}
			let copy = b.t.copy_fn(elem, span).unwrap_or_else(Expr::undefined);
			rs("smallVecInsertFromSlice", [args, vec![n, size, copy], at].concat())
		}
		"append" => rs("smallVecAppend", [args, vec![n, size]].concat()),
		"resize" => {
			let clone = super::collections::clone_fn(b, elem, span);
			let drop = b.t.dropper(elem, span);
			rs("smallVecResize", [args, vec![n, size, clone, drop]].concat())
		}
		"resize_with" => {
			// `SmallVec<A>::resize_with::<F>`, which drops the closure once done.
			let f = library::targ(call.args, 1);
			if b.t.drops(f) {
				return Some(b.err(
					span,
					format!("passing `{f}`, which drops, to `SmallVec::{name}`, which is not supported yet"),
				));
			}
			let drop = b.t.dropper(elem, span);
			rs("smallVecResizeWith", [args, vec![n, size, drop]].concat())
		}
		_ => None,
	}
}

/// A call of `Vec`'s `name` in place of the `SmallVec` method `call`, which behaves as it.
fn vec_call<'tcx>(
	b: &mut Body<'_, '_, 'tcx>,
	call: &Call<'tcx>,
	elem: Ty<'tcx>,
	name: &str,
	args: Vec<Expr>,
	out: &mut Vec<Stmt>,
) -> Option<Expr> {
	let tcx = b.t.tcx;
	let vec = tcx.get_diagnostic_item(sym::Vec)?;
	let name = Symbol::intern(name);
	let f = tcx
		.inherent_impls(vec)
		.iter()
		.find_map(|&imp| tcx.associated_items(imp).filter_by_name_unhygienic(name).next())?
		.def_id;
	let global = Ty::new_adt(
		tcx,
		tcx.adt_def(tcx.lang_items().get(LangItem::GlobalAlloc)?),
		ty::List::empty(),
	);
	// `Vec<T>`'s or `Vec<T, A>`'s parameters, then the method's own, which follow
	// `SmallVec<A>`'s one.
	let mut vargs: Vec<GenericArg<'tcx>> = vec![elem.into()];
	if tcx.generics_of(f).parent_count == 2 {
		vargs.push(global.into());
	}
	vargs.extend(call.args.iter().skip(1).map(|a| fold(tcx, a)));
	let vcall = Call {
		def_id: f,
		args: tcx.mk_args(&vargs),
		span: call.span,
	};
	library::call(b, &vcall, args, out)
}

/// A std trait's method `owner::name` of `self_ty`, a `SmallVec` or a reference to one: as
/// `Vec`'s, but where smallvec's own impl differs.
fn trait_call<'tcx>(
	b: &mut Body<'_, '_, 'tcx>,
	call: &Call<'tcx>,
	owner: &str,
	name: &str,
	self_ty: Ty<'tcx>,
	mut args: Vec<Expr>,
	out: &mut Vec<Stmt>,
) -> Option<Expr> {
	let tcx = b.t.tcx;
	let span = call.span;
	let (elem, inline) = match known(b, self_ty.peel_refs(), owner, name, span) {
		Ok(known) => known,
		Err(e) => return Some(e),
	};
	let folded = Call {
		def_id: call.def_id,
		args: fold(tcx, call.args),
		span,
	};
	if self_ty.is_ref() {
		return library::call(b, &folded, args, out);
	}
	let size = super::iter::size_of(b, elem, span);
	let zero = super::iter::layout(b, elem, span).is_ok_and(|(size, _)| size == 0);
	let n = inline_size(inline, zero);
	match (owner, name) {
		// `SmallVec::from(self.as_slice())`, which collects clones of its items.
		("Clone", "clone") => {
			let clone = super::collections::clone_fn(b, elem, span);
			Some(Expr::rs("smallVecCloned", vec![args.pop()?.member("items"), clone]))
		}
		// Typed, since what reads it, such as a `mem::take`, may infer its items from it.
		("Default", "default") => {
			let ty = b.t.ty(self_ty, span);
			Some(Expr::As(Box::new(library::vec_new(false)), ty))
		}
		// smallvec's reserves for the iterator's lower bound, then reads it by `next`.
		("Extend", "extend") => {
			let coll = fold(tcx, self_ty);
			let (items, iter) = super::collections::extend_source(b, coll, &folded, owner, args.pop()?)?;
			let take = super::iter::iter_take(b, iter, false, span);
			Some(Expr::rs("smallVecExtend", vec![args.pop()?, items, n, size, take]))
		}
		("FromIterator", "from_iter") => {
			// `FromIterator<A>::from_iter::<T>`.
			let source = library::targ(call.args, 2);
			let iter = super::iter::iter_of(b.t, source)?;
			let items = super::iter::into_iter(b, source, args.pop()?, span)?;
			Some(extended(b, items, iter, n, size, span))
		}
		("From", "from") => {
			let from = library::targ(call.args, 1);
			let value = args.pop()?;
			match from.kind() {
				ty::Array(..) => Some(Expr::New(Box::new(Expr::Rs("Vec".into())), vec![value])),
				ty::Ref(_, slice, _) if slice.is_slice() => {
					let clone = super::collections::clone_fn(b, elem, span);
					let items = Expr::rs("sliceToArray", vec![value]);
					Some(Expr::rs("smallVecCloned", vec![items, clone]))
				}
				_ if b.t.std_of(from) == Some(library::Std::Vec) => {
					Some(Expr::rs("smallVecFromVec", vec![value, n, size]))
				}
				_ => None,
			}
		}
		// Each writes or reads its items as smallvec's own code does: `clone_from` truncates
		// from the back, and `Write` and the other `Extend` methods are its own.
		("Clone", _) | ("Extend", _) | ("Write", _) => None,
		// Its `Index` panics in smallvec's own code, where `Vec`'s locates its caller.
		("Index" | "IndexMut", _) if b.t.hooked => Some(b.err(
			span,
			format!("`{owner}::{name}` of `{self_ty}`, in a program with a panic hook, whose panic is in smallvec's code, which is not supported yet"),
		)),
		_ => library::call(b, &folded, args, out),
	}
}

/// What a `SmallVec` of `inline` elements holds inline: `usize::MAX` of zero-sized ones,
/// which never spill.
fn inline_size(inline: u64, zero: bool) -> Expr {
	Expr::lit(match zero {
		true => u32::MAX.to_string(),
		false => inline.to_string(),
	})
}

/// A new `SmallVec` of `items`, an `rs.Iter` of type `iter`, which it reserves for by their
/// lower bound and reads by `next`, as smallvec's `FromIterator` does.
fn extended<'tcx>(b: &mut Body<'_, '_, 'tcx>, items: Expr, iter: Ty<'tcx>, n: Expr, size: Expr, span: Span) -> Expr {
	let take = super::iter::iter_take(b, iter, false, span);
	Expr::rs("smallVecExtend", vec![super::library::vec_new(false), items, n, size, take])
}

/// `collect` into `ty`, if it is a `SmallVec`, of `items`, an `rs.Iter` of type `iter`.
pub fn collect<'tcx>(
	b: &mut Body<'_, '_, 'tcx>,
	ty: Ty<'tcx>,
	items: Expr,
	iter: Ty<'tcx>,
	span: Span,
) -> Option<Expr> {
	array(b.t.tcx, ty)?;
	Some(match known(b, ty, "Iterator", "collect", span) {
		Ok((elem, inline)) => {
			let size = super::iter::size_of(b, elem, span);
			let zero = super::iter::layout(b, elem, span).is_ok_and(|(size, _)| size == 0);
			let n = inline_size(inline, zero);
			extended(b, items, iter, n, size, span)
		}
		Err(e) => e,
	})
}

/// Which of smallvec's panics a method makes, which a program with a panic hook reads the
/// location of.
#[derive(Clone, Copy, PartialEq)]
enum Panic {
	/// Its own `panic!` or `assert!`.
	Own,
	/// `infallible`'s "capacity overflow".
	Infallible,
	/// One of several, or one in std's code it calls, which aren't located.
	Unlocated,
}

/// Where `call` panics, as the runtime function takes it last, in a program with a panic
/// hook: nothing in one without, or else its refusal where it isn't known.
fn at<'tcx>(b: &mut Body<'_, '_, 'tcx>, call: &Call<'tcx>, panic: Panic) -> Result<Vec<Expr>, Expr> {
	if !b.t.hooked {
		return Ok(Vec::new());
	}
	let tcx = b.t.tcx;
	let did = match panic {
		Panic::Own => Some(call.def_id),
		Panic::Infallible => infallible(tcx, call.def_id, 3),
		Panic::Unlocated => None,
	};
	match did.and_then(|did| super::caller::own_panic(tcx, did)) {
		Some(at) => Ok(vec![at]),
		None => Err(b.err(
			call.span,
			format!(
				"`{}`, in a program with a panic hook, whose compiled code shows no one panic, which is not supported yet",
				tcx.def_path_str(call.def_id)
			),
		)),
	}
}

/// smallvec's private `infallible`, which panics where a reservation overflows, as `did`
/// calls it, or a function of smallvec's it calls does, within `depth` calls.
fn infallible(tcx: TyCtxt<'_>, did: DefId, depth: u32) -> Option<DefId> {
	if depth == 0 || !tcx.is_mir_available(did) {
		return None;
	}
	tcx.optimized_mir(did).basic_blocks.iter().find_map(|block| {
		let rustc_middle::mir::TerminatorKind::Call { func, .. } = &block.terminator().kind else {
			return None;
		};
		let (callee, _) = func.const_fn_def()?;
		if callee.is_local() || !is_crate(tcx, callee.krate) {
			return None;
		}
		match tcx.item_name(callee).as_str() {
			"infallible" => Some(callee),
			_ => infallible(tcx, callee, depth - 1),
		}
	})
}
