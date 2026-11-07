//! Slices of arrays, which are `rs.ArraySlice`: a `&[[T; N]]` lends each array as the slice a
//! `&[T; N]` is, where the slice holds it, whether its arrays are each an array's own, as a
//! `Vec<[T; N]>` holds them, or parts of one slice, as `as_chunks` splits it. The runtime's
//! `arraySlice` functions map std's methods that lend arrays or parts. Any other method of a
//! slice of arrays reads it as an `rs.Slice` of JS arrays: shared, of its arrays themselves, or
//! of copies of those `as_chunks` split from one slice, which have no array of their own; and
//! mutable, of copies that `rs.arraySliceBy` writes back once it returns, as Rust moves their
//! elements.

use rustc_middle::ty::{self, Ty, TyCtxt};

use super::body::Body;
use super::library::{self, Call};
use crate::ts::{Expr, Param, Stmt, StmtKind, Type};

/// Whether `ty` is a slice or array of arrays, which behind a reference is an `rs.ArraySlice`.
pub fn of_arrays(ty: Ty<'_>) -> bool {
	matches!(ty.kind(), ty::Slice(elem) | ty::Array(elem, _) if elem.is_array())
}

/// Whether std's `def_id` of `args` is a method of a slice of arrays the runtime's own
/// functions map, which take and lend the arrays where the slice holds them.
pub fn native<'tcx>(tcx: TyCtxt<'tcx>, def_id: rustc_hir::def_id::DefId, args: ty::GenericArgsRef<'tcx>) -> bool {
	let Some((owner, name)) = library::key(tcx, def_id) else {
		return false;
	};
	owner == "slice"
		&& args.types().next().is_some_and(|elem| elem.is_array())
		&& matches!(
			name.as_str(),
			"len"
				| "is_empty"
				| "first"
				| "last"
				| "get"
				| "first_mut"
				| "last_mut"
				| "get_mut"
				| "split_first"
				| "split_last"
				| "split_first_mut"
				| "split_last_mut"
				| "split_at"
				| "split_at_mut"
				| "split_at_checked"
				| "split_at_mut_checked"
				| "windows"
				| "chunks"
				| "chunks_exact"
				| "rchunks"
				| "rchunks_exact"
				| "chunks_mut"
				| "chunks_exact_mut"
				| "rchunks_mut"
				| "rchunks_exact_mut"
				| "as_chunks"
				| "as_rchunks"
				| "as_chunks_mut"
				| "as_rchunks_mut"
				| "first_chunk"
				| "first_chunk_mut"
				| "last_chunk"
				| "last_chunk_mut"
				| "split_first_chunk"
				| "split_first_chunk_mut"
				| "split_last_chunk"
				| "split_last_chunk_mut"
				| "swap"
				| "iter"
				| "iter_mut"
				| "split"
				| "split_mut"
				| "rsplit"
				| "rsplit_mut"
				| "split_inclusive"
				| "split_inclusive_mut"
				| "splitn"
				| "splitn_mut"
				| "rsplitn"
				| "rsplitn_mut"
				| "to_vec"
		)
}

/// `args` as std's mapping of `call` reads them, unless [`native`] maps it: each
/// `&[[T; N]]` it reads as a slice given as `rs.arraySliceAsSlice`' copies, and each `&mut` one by
/// name, which [`wrap`] binds to `rs.arraySliceBy`'s copies. A function that returns a borrow,
/// which may be of the copies, is refused.
pub fn give<'tcx>(b: &mut Body<'_, '_, 'tcx>, call: &Call<'tcx>, args: Vec<Expr>) -> Result<Given, Expr> {
	let tcx = b.t.tcx;
	let mut given = Given { args, by: Vec::new() };
	// Only std's own functions and methods; a trait's mapping reads arrays as their type says.
	if native(tcx, call.def_id, call.args) || tcx.trait_of_assoc(call.def_id).is_some() {
		return Ok(given);
	}
	let generic = tcx
		.fn_sig(call.def_id)
		.instantiate_identity()
		.skip_norm_wip()
		.skip_binder();
	let sig = b.t.sig(call.def_id, call.args);
	let mut any = false;
	for (i, (&g, &ty)) in std::iter::zip(generic.inputs(), sig.inputs()).enumerate() {
		// What std takes by a reference of its own, not a value it holds as it is.
		let (ty::Ref(..), ty::Ref(_, inner, mutability)) = (g.kind(), ty.kind()) else {
			continue;
		};
		// A reference to an array of arrays std takes as a `&T` [`super::lend::give`]s.
		if !(inner.is_slice() && of_arrays(*inner)) || i >= given.args.len() {
			continue;
		}
		any = true;
		let value = std::mem::replace(&mut given.args[i], Expr::undefined());
		given.args[i] = match mutability {
			ty::Mutability::Not => Expr::rs("arraySliceAsSlice", vec![value]),
			ty::Mutability::Mut => {
				let name = b.fresh("arrays");
				let copy = b.t.elem_copy(*inner, call.span);
				given.by.push(By {
					name: name.clone(),
					value,
					copy,
				});
				Expr::ident(name)
			}
		};
	}
	if any && generic.output().walk().any(|arg| arg.as_region().is_some()) {
		let path = tcx.def_path_str(call.def_id);
		return Err(b.err(
			call.span,
			format!("`{path}` of a slice of arrays, which lends part of it, which is not supported yet"),
		));
	}
	Ok(given)
}

/// What [`give`] makes of a call's arguments.
pub struct Given {
	/// The arguments std's mapping takes.
	pub args: Vec<Expr>,
	/// Each `&mut [[T; N]]` given by name.
	pub by: Vec<By>,
}

/// A `&mut [[T; N]]` [`give`] names in a call's arguments.
pub struct By {
	/// Its name in the arguments.
	pub name: String,
	/// The arrays.
	pub value: Expr,
	/// What copies an element of the arrays out, as [`super::Translator::elem_copy`] says.
	pub copy: Option<Expr>,
}

/// `e`, the call std's mapping made of [`Given::args`], a `ty`, after `stmts`, with each
/// name it binds the `rs.arraySliceBy` copies of its arrays.
pub fn wrap(e: Expr, ty: Type, stmts: Vec<Stmt>, by: Vec<By>) -> Expr {
	let mut stmts = Some(stmts);
	let mut e = e;
	for By { name, value, copy } in by.into_iter().rev() {
		let param = Param { name, ty: None };
		let f = match stmts.take().filter(|s| !s.is_empty()) {
			Some(mut body) => {
				body.push(StmtKind::Return(Some(e)).at(None));
				Expr::Arrow(vec![param], ty.clone(), body)
			}
			None => Expr::Lambda(vec![param], Box::new(e)),
		};
		e = Expr::rs("arraySliceBy", [vec![value, f], copy.into_iter().collect()].concat());
	}
	e
}
