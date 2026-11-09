//! Casts of pointers to memory from `alloc`, as [`crate::lower::alloc`] models it: one from
//! `*mut u8` to `*mut T` the body shows came from `alloc` or `realloc` makes the memory a JS
//! array of `T`s, and one back to `*mut u8` is the block, only as what `dealloc` or
//! `realloc` takes. Any other cast between them reads the bytes of one type as another,
//! which stays refused, and so does a call of `alloc` or `realloc` whose byte pointer the
//! body passes on elsewhere, so no pointer to a block's bytes leaves the body that made it.

use rustc_middle::mir::{self, Local, Operand, ProjectionElem, RETURN_PLACE, Rvalue, StatementKind, TerminatorKind};
use rustc_middle::ty::{self, Ty};
use rustc_span::{Span, Spanned};

use super::Fx;
use crate::lower::library;
use crate::lower::pointer::Pointee;
use crate::ts::{Expr, Stmt};

fn byte(ty: Ty<'_>) -> bool {
	matches!(ty.kind(), ty::Uint(ty::UintTy::U8))
}

/// Whether std's `owner::name` gives the byte pointer its first argument holds on, as a
/// pointer, a `NonNull`, or the payload of an `Option`, a `Result`, or `?`'s `ControlFlow`.
fn step(owner: &str, name: &str) -> bool {
	matches!(
		(owner, name),
		("NonNull", "new" | "new_unchecked" | "as_ptr")
			| ("ptr" | "NonNull", "cast" | "cast_const" | "cast_mut")
			| (
				"Option",
				"ok_or" | "ok_or_else" | "unwrap" | "expect" | "unwrap_unchecked"
			)
			| ("Result", "unwrap" | "expect" | "unwrap_unchecked")
			| ("Try", "branch")
	)
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
			(owner, name) if step(owner, name) => self.alloc_root(&args[0].node),
			_ => None,
		}
	}

	/// Why the call of std's `def_id`, if `alloc` or `realloc`, can't be: the body passes the
	/// byte pointer it returns into `dest` on elsewhere than a cast to a pointer to values JS
	/// copies, `dealloc`, `realloc`, or the steps [`step`] names, such as into a field, out
	/// of the body, or to any other function; or, of `realloc`, casts it to a pointer to a type
	/// other than the one the memory it grows was cast from. A byte of the memory read or
	/// written here is refused where it is.
	pub(super) fn alloc_leak(
		&self,
		def_id: rustc_hir::def_id::DefId,
		args: &[Spanned<Operand<'tcx>>],
		dest: mir::Place<'tcx>,
	) -> Option<String> {
		if !library::is_std(self.tcx, def_id) {
			return None;
		}
		let (owner, name) = library::key(self.tcx, def_id)?;
		if owner != "alloc" || !matches!(name.as_str(), "alloc" | "realloc") {
			return None;
		}
		let Some(types) = dest.as_local().and_then(|local| self.alloc_uses(local)) else {
			return Some(format!(
				"`{name}` whose pointer the body passes on as bytes, other than to a cast to a pointer to values, `dealloc`, or `realloc`, which JS keeps as the values a cast types the memory as"
			));
		};
		// Memory from an `alloc` in the body, whose casts `alloc_cast` checks against each other.
		if name != "realloc" || self.alloc_root(&args.first()?.node).is_some() {
			return None;
		}
		let from = self.freed_type(&args.first()?.node)?;
		let to = types.into_iter().find(|t| *t != from)?;
		Some(format!(
			"`realloc` of memory cast from a pointer to `{from}` whose result is cast to one to `{to}`, which reads the bytes of one type as another"
		))
	}

	/// The types the casts in the body of the byte pointer in `root` type it as, if the body
	/// passes it on no further than [`Self::alloc_leak`] allows.
	fn alloc_uses(&self, root: Local) -> Option<Vec<Ty<'tcx>>> {
		let (decls, tcx) = (&self.mir.local_decls, self.tcx);
		// Whether a value of `ty` may hold the pointer: one to bytes, or what holds one.
		let holds = |ty: Ty<'tcx>| {
			ty.walk().any(|arg| {
				arg.as_type().is_some_and(|t| match t.kind() {
					ty::RawPtr(pointee, _) => byte(*pointee),
					ty::Adt(adt, args) => {
						library::Std::of(tcx, adt.did()) == Some(library::Std::NonNull) && byte(args.type_at(0))
					}
					_ => false,
				})
			})
		};
		// Whether `op` reads what `local` holds, or a payload of it, but not through it.
		let reads = |op: &Operand<'tcx>, local: Local| {
			op.place()
				.is_some_and(|p| p.local == local && !p.projection.contains(&ProjectionElem::Deref))
		};
		let mut types = Vec::new();
		let mut seen = vec![root];
		let mut todo = vec![root];
		while let Some(local) = todo.pop() {
			// Out of the body, as what it returns or into what it was given.
			if local == RETURN_PLACE || self.is_arg(local) {
				return None;
			}
			let mut carry = |to: mir::Place<'tcx>| -> Option<()> {
				if !holds(to.ty(decls, tcx).ty) {
					return Some(());
				}
				let to = to.as_local()?;
				if !seen.contains(&to) {
					seen.push(to);
					todo.push(to);
				}
				Some(())
			};
			for data in self.mir.basic_blocks.iter() {
				for stmt in &data.statements {
					let StatementKind::Assign(assign) = &stmt.kind else {
						continue;
					};
					let (dest, rvalue) = &**assign;
					match rvalue {
						Rvalue::Cast(mir::CastKind::PtrToPtr, op, to) if reads(op, local) => {
							match to.builtin_deref(true) {
								Some(pointee) if !byte(pointee) => types.push(self.alloc_key(pointee)),
								_ => carry(*dest)?,
							}
						}
						Rvalue::Use(op, _) | Rvalue::Cast(_, op, _) if reads(op, local) => carry(*dest)?,
						Rvalue::Ref(_, _, place) | Rvalue::RawPtr(_, place)
							if place.local == local && !place.projection.contains(&ProjectionElem::Deref) =>
						{
							return None;
						}
						Rvalue::Aggregate(_, ops) if ops.iter().any(|op| reads(op, local)) => return None,
						_ => {}
					}
				}
				let TerminatorKind::Call {
					func,
					args,
					destination,
					..
				} = &data.terminator().kind
				else {
					continue;
				};
				let Some(at) = args.iter().position(|a| reads(&a.node, local)) else {
					continue;
				};
				let ty::FnDef(def_id, gargs) = *func.ty(decls, tcx).kind() else {
					return None;
				};
				let (owner, name) = library::key(tcx, def_id).filter(|_| library::is_std(tcx, def_id))?;
				match (owner.as_str(), name.as_str()) {
					("alloc", "dealloc" | "realloc") if at == 0 => {}
					("ptr" | "NonNull", "is_null") => {}
					("ptr" | "NonNull", "cast") => match gargs.skip_binder().types().nth(1) {
						Some(to) if !byte(to) => types.push(self.alloc_key(to)),
						_ => carry(*destination)?,
					},
					(owner, name) if at == 0 && step(owner, name) => carry(*destination)?,
					_ => return None,
				}
			}
		}
		Some(types)
	}

	/// The type of the values the pointer `op`, which `dealloc` or `realloc` takes, was cast
	/// from in the body, as `alloc_key` names it.
	fn freed_type(&self, op: &Operand<'tcx>) -> Option<Ty<'tcx>> {
		let local = self.traced(op)?;
		if self.def_at[local].is_some() {
			return match self.def_rvalue(local)? {
				Rvalue::Use(op, _) => self.freed_type(op),
				Rvalue::Cast(mir::CastKind::PtrToPtr, op, _) => {
					let from = op.ty(&self.mir.local_decls, self.tcx).builtin_deref(true)?;
					(!byte(from)).then(|| self.alloc_key(from))
				}
				_ => None,
			};
		}
		let (def_id, gargs, args) = self.def_call(local)?;
		match library::key(self.tcx, def_id)? {
			(owner, name) if matches!(owner.as_str(), "ptr" | "NonNull") && name == "cast" => {
				let from = gargs.types().next()?;
				match byte(from) {
					true => self.freed_type(&args.first()?.node),
					false => Some(self.alloc_key(from)),
				}
			}
			(owner, name) if owner == "NonNull" && name == "as_ptr" => self.freed_type(&args.first()?.node),
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
