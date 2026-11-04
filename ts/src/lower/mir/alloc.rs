//! Casts of pointers to memory from `alloc`, as [`crate::lower::alloc`] models it: one from
//! `*mut u8` to `*mut T` the body shows came from `alloc` or `realloc` makes the memory a JS
//! array of `T`s, and one back to `*mut u8` is the block, only as what `dealloc` or
//! `realloc` takes. Any other cast between them reads the bytes of one type as another,
//! which stays refused.

use rustc_middle::mir::{self, Local, Operand, ProjectionElem, Rvalue, TerminatorKind};
use rustc_middle::ty::{self, Ty};
use rustc_span::{Span, Spanned};

use super::Fx;
use crate::lower::library;
use crate::lower::pointer::Pointee;
use crate::ts::{Expr, Stmt};

fn byte(ty: Ty<'_>) -> bool {
	matches!(ty.kind(), ty::Uint(ty::UintTy::U8))
}

/// Why the bytes of memory from `alloc` are refused.
pub(super) const BYTES: &str =
	"a byte of memory from `alloc`, which JS keeps as the values a cast of its pointer types it as";

impl<'tcx> Fx<'_, '_, '_, 'tcx> {
	/// The cast of the pointer `op` from one to `from` to one to `to`, if it is a cast of
	/// memory from `alloc` between bytes and values JS copies.
	pub(super) fn alloc_cast(
		&mut self,
		op: &Operand<'tcx>,
		from: Ty<'tcx>,
		to: Ty<'tcx>,
		span: Span,
		out: &mut Vec<Stmt>,
	) -> Option<Expr> {
		let (f, elem) = match (byte(from), byte(to)) {
			(true, false) if self.typed(to) && self.allocated(op) => ("allocAs", to),
			(false, true) if self.typed(from) && self.freed(op) => ("allocBytes", from),
			_ => return None,
		};
		let size = crate::lower::iter::size_of(self.b, elem, span);
		// The type, not only its size: JS keeps `u32`s and `i32`s, for one, as the same numbers.
		// A `MaybeUninit` is what it holds, as a cast between them keeps the pointer.
		let key = self.alloc_key(elem);
		// One body casting the memory to two types reads one's bytes as the other's; a
		// pointer from elsewhere fails at run time.
		if f == "allocAs"
			&& let Some(other) = self
				.alloc_root(op)
				.and_then(|root| self.alloc_types(root).find(|t| *t != key))
		{
			return Some(self.err(
				span,
				format!(
					"memory from `alloc` cast to pointers to `{key}` and to `{other}`, which reads the bytes of one type as another"
				),
			));
		}
		let p = self.operand(op, span, out);
		Some(Expr::rs(f, vec![p, size, Expr::Str(key.to_string())]))
	}

	/// Whether memory from `alloc` can be typed as `ty`s: values JS copies, of some bytes.
	fn typed(&self, ty: Ty<'tcx>) -> bool {
		!byte(ty)
			&& !self.b.t.zero_sized(ty)
			&& self.b.t.pointee(ty) == Pointee::Place
			&& ty.is_sized(self.tcx, self.b.t.typing_env_pub())
	}

	/// The type a cast to pointers to `ty` types memory from `alloc` as.
	fn alloc_key(&self, ty: Ty<'tcx>) -> Ty<'tcx> {
		self.b.t.erased(self.b.t.uninit_inner(ty))
	}

	/// The types each cast in the body of a pointer to the memory `alloc` returned into
	/// `root` types it as.
	fn alloc_types(&self, root: Local) -> impl Iterator<Item = Ty<'tcx>> + '_ {
		let (decls, tcx) = (&self.mir.local_decls, self.tcx);
		let casts = self.mir.basic_blocks.iter().flat_map(move |data| {
			let assigns = data.statements.iter().filter_map(move |stmt| match &stmt.kind {
				mir::StatementKind::Assign(assign) => match &assign.1 {
					Rvalue::Cast(mir::CastKind::PtrToPtr, op, to) => {
						Some((op, op.ty(decls, tcx).builtin_deref(true)?, to.builtin_deref(true)?))
					}
					_ => None,
				},
				_ => None,
			});
			let call = || {
				let TerminatorKind::Call { func, args, .. } = &data.terminator().kind else {
					return None;
				};
				let ty::FnDef(def_id, gargs) = *func.ty(decls, tcx).kind() else {
					return None;
				};
				let (owner, name) = library::key(tcx, def_id)?;
				if !library::is_std(tcx, def_id) || !matches!(owner.as_str(), "ptr" | "NonNull") || name != "cast" {
					return None;
				}
				let mut types = gargs.skip_binder().types();
				Some((&args.first()?.node, types.next()?, types.next()?))
			};
			assigns.chain(call())
		});
		casts
			.filter(move |&(op, from, to)| byte(from) && self.typed(to) && self.alloc_root(op) == Some(root))
			.map(|(_, _, to)| self.alloc_key(to))
	}

	/// A call of `cast` on a pointer, if it is a cast of memory from `alloc`, as [`Self::alloc_cast`] has it.
	pub(super) fn alloc_cast_call(
		&mut self,
		def_id: rustc_hir::def_id::DefId,
		gargs: ty::GenericArgsRef<'tcx>,
		args: &[Spanned<Operand<'tcx>>],
		span: Span,
		out: &mut Vec<Stmt>,
	) -> Option<Expr> {
		let (owner, name) = library::key(self.tcx, def_id)?;
		if !library::is_std(self.tcx, def_id) || !matches!(owner.as_str(), "ptr" | "NonNull") || name != "cast" {
			return None;
		}
		let mut types = gargs.types();
		let (from, to) = (types.next()?, types.next()?);
		self.alloc_cast(&args.first()?.node, from, to, span, out)
	}

	/// Why `place` can't be read or written, if it is a byte of memory from `alloc`, as the
	/// body shows, which JS keeps as values once a cast types it, and never as bytes.
	pub(super) fn alloc_bytes(&self, place: mir::Place<'tcx>) -> Option<&'static str> {
		let [ProjectionElem::Deref, ..] = place.projection.as_slice() else {
			return None;
		};
		let ty::RawPtr(pointee, _) = self.mir.local_decls[place.local].ty.kind() else {
			return None;
		};
		(matches!(pointee.kind(), ty::Uint(ty::UintTy::U8))
			&& self.allocated(&Operand::Copy(mir::Place::from(place.local))))
		.then_some(BYTES)
	}

	/// Whether the pointer `op` is one `alloc` or `realloc` returned, as the body shows.
	pub(super) fn allocated(&self, op: &Operand<'tcx>) -> bool {
		self.alloc_root(op).is_some()
	}

	/// The local `alloc` returned the pointer `op` into, as the body shows, or `realloc`
	/// did, which keeps the type of the memory it grows when that is in the body too.
	fn alloc_root(&self, op: &Operand<'tcx>) -> Option<Local> {
		let local = self.traced(op)?;
		if self.def_at[local].is_some() {
			return match self.def_rvalue(local) {
				Some(Rvalue::Use(op, _) | Rvalue::Cast(mir::CastKind::PtrToPtr, op, _)) => match op.place() {
					// A payload out of an `Option`, a `Result`, or `?`'s `ControlFlow`.
					Some(place)
						if let [ProjectionElem::Downcast(..), ProjectionElem::Field(..)] =
							place.projection.as_slice() =>
					{
						self.alloc_root(&Operand::Copy(mir::Place::from(place.local)))
					}
					_ => self.alloc_root(op),
				},
				_ => None,
			};
		}
		let (def_id, _, args) = self.def_call(local)?;
		let (owner, name) = library::key(self.tcx, def_id)?;
		match (owner.as_str(), name.as_str()) {
			("alloc", "alloc") => library::is_std(self.tcx, def_id).then_some(local),
			("alloc", "realloc") if library::is_std(self.tcx, def_id) => self.alloc_root(&args[0].node).or(Some(local)),
			("NonNull", "new" | "new_unchecked" | "as_ptr")
			| ("ptr" | "NonNull", "cast" | "cast_const" | "cast_mut")
			| ("Option", "ok_or" | "ok_or_else" | "unwrap" | "expect" | "unwrap_unchecked")
			| ("Result", "unwrap" | "expect" | "unwrap_unchecked")
			| ("Try", "branch") => self.alloc_root(&args[0].node),
			_ => None,
		}
	}

	/// Whether the cast of the pointer `op` gives only what `dealloc` or `realloc` takes.
	fn freed(&self, op: &Operand<'tcx>) -> bool {
		self.mir.basic_blocks.iter().any(|data| {
			let TerminatorKind::Call { func, args, .. } = &data.terminator().kind else {
				return false;
			};
			let ty::FnDef(def_id, _) = *func.ty(&self.mir.local_decls, self.tcx).kind() else {
				return false;
			};
			let freeing = library::is_std(self.tcx, def_id)
				&& matches!(
					library::key(self.tcx, def_id),
					Some((owner, name)) if owner == "alloc" && matches!(name.as_str(), "dealloc" | "realloc")
				);
			freeing && args.first().is_some_and(|arg| self.cast_of(&arg.node, op))
		})
	}

	/// Whether `arg` is read only here, and is the cast of `op` alone, by `as` or `cast`,
	/// and a `NonNull`'s through `as_ptr`.
	fn cast_of(&self, arg: &Operand<'tcx>, op: &Operand<'tcx>) -> bool {
		let Some(local): Option<Local> = arg.place().and_then(|p| p.as_local()) else {
			return false;
		};
		if self.uses[local] != 1 || self.traced(arg).is_none() {
			return false;
		}
		if self.def_at[local].is_some() {
			return matches!(self.def_rvalue(local), Some(Rvalue::Cast(mir::CastKind::PtrToPtr, cast, _)) if std::ptr::eq(cast, op));
		}
		let Some((def_id, _, args)) = self.def_call(local) else {
			return false;
		};
		match library::key(self.tcx, def_id) {
			Some((owner, name)) => match (owner.as_str(), name.as_str()) {
				("ptr" | "NonNull", "cast") => std::ptr::eq(&args[0].node, op),
				("NonNull", "as_ptr") => self.cast_of(&args[0].node, op),
				_ => false,
			},
			None => false,
		}
	}
}
