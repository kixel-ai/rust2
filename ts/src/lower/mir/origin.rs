//! Where a raw pointer a body makes points, as far as the body shows. JS keeps a value in
//! no array, and a shared slice of a byte in no array, apart from what holds them in Rust,
//! so a pointer leaving one, or measured against another, fails at run time; where the
//! body proves that it does, translation refuses it with a span instead. A pointer the
//! body takes from elsewhere, such as a field or a parameter, is left to the runtime.
//!
//! So is whether a pointer to a value JS copies is an `Arc`'s, the one place of its value,
//! or a borrow's, a place of its own: only an `Arc`'s is an identity, which `from_raw`
//! takes back to it and a cast to `*const ()` keeps. That cast is refused unless the body
//! shows an `Arc`'s, or null or dangling, since a place from elsewhere has no one identity.

use rustc_middle::mir::{
	self, BasicBlock, Local, Location, Operand, ProjectionElem, Rvalue, StatementKind, TerminatorKind,
};
use rustc_middle::ty::{self, GenericArgsRef};
use rustc_span::Spanned;

use super::{Fx, Slot};
use crate::lower::library::{self, Std};
use crate::lower::pointer::{self, Pointee};
use crate::ts::{Expr, Stmt};

/// The array JS keeps what a pointer points at in.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Array<'tcx> {
	/// An array of one, around the place a `&raw` here took, which each `&raw` makes anew.
	Lone(Location),
	/// The `Uint8Array` of one a byte variable, whose `&raw` the body takes, is kept in.
	Byte(Local),
	/// A shared slice JS copies, of a byte in no array, made by the call ending this block,
	/// with its length if the body fixes it.
	Copy(BasicBlock, Option<i128>),
	/// The array of a variable of this body's own, or of a field of one, at this place.
	Owned(mir::Place<'tcx>),
	/// A `str`'s UTF-8, which JS encodes anew at each `as_ptr`, the call ending this block.
	Text(BasicBlock),
}

impl<'tcx> Array<'tcx> {
	/// How many elements the array holds, if the body shows it.
	fn len(self) -> Option<i128> {
		match self {
			Array::Lone(_) | Array::Byte(_) => Some(1),
			Array::Copy(_, len) => len,
			Array::Owned(_) | Array::Text(_) => None,
		}
	}

	/// What the array is, as a refusal names it.
	fn what(self) -> &'static str {
		match self {
			Array::Lone(_) | Array::Byte(_) => "a value in no array",
			Array::Copy(..) => "a shared slice JS copies",
			Array::Owned(_) => "an array of this body's own",
			Array::Text(_) => "a `str`'s bytes",
		}
	}
}

/// Where a pointer points: into which array, at which index if the body fixes it.
#[derive(Clone, Copy)]
struct Origin<'tcx> {
	array: Array<'tcx>,
	index: Option<i128>,
}

impl<'tcx> Fx<'_, '_, '_, 'tcx> {
	/// Why the call of std's `def_id` would fail at run time where Rust's is defined, if the
	/// body proves it would: `offset_from` between pointers JS keeps in different arrays, or
	/// a pointer, slice, copy, or write reaching out of an array of one or a copied byte.
	/// From an array of one, every offset, count, length, and access, wrapping or not, must
	/// be one the body fixes and proves stays within it: past it, Rust may reach the
	/// neighbour the reference was borrowed beside, which JS keeps apart.
	pub(super) fn stray(
		&self,
		def_id: rustc_hir::def_id::DefId,
		gargs: GenericArgsRef<'tcx>,
		args: &[Spanned<Operand<'tcx>>],
	) -> Option<String> {
		if !library::is_std(self.tcx, def_id) {
			return None;
		}
		let (owner, name) = library::key(self.tcx, def_id)?;
		let arg = |i: usize| args.get(i).map(|a| &a.node);
		let pointee = gargs.types().next()?;
		if self.b.t.zero_sized(pointee) || self.b.t.pointee(pointee) != Pointee::Place {
			return None;
		}
		// Only a cast types memory from `alloc`, which has no bytes before or after.
		if matches!(pointee.kind(), ty::Uint(ty::UintTy::U8))
			&& matches!(owner.as_str(), "ptr" | "NonNull" | "raw")
			&& !matches!(name.as_str(), "cast" | "as_ptr" | "new" | "new_unchecked" | "is_null")
			&& args.iter().any(|a| self.allocated(&a.node))
		{
			return Some(super::alloc::BYTES.into());
		}
		let (unfixed, out) = (Self::unfixed, Self::out);
		match (owner.as_str(), name.as_str()) {
			("ptr" | "NonNull", "cast") => self.erased_place(arg(0)?, pointee, gargs.types().nth(1)?),
			("Arc" | "Rc", "from_raw" | "increment_strong_count" | "decrement_strong_count")
				if self.arc_pointer(arg(0)?) == Some(false) =>
			{
				Some(format!(
					"`{owner}::{name}` of a pointer to `{pointee}` a borrow made, whose JS value is a place of its own, which no `{owner}` gave"
				))
			}
			("ptr" | "NonNull", "offset_from" | "offset_from_unsigned") => {
				let (a, b) = (self.origin(arg(0)?)?, self.origin(arg(1)?)?);
				if let (Array::Text(_), Array::Text(_)) = (a.array, b.array) {
					return (a.array != b.array).then(|| {
						"`offset_from` between pointers of two `str::as_ptr` calls, each into an encoding of the text of its own, which JS keeps in different arrays".into()
					});
				}
				self.apart(a.array, b.array).then(|| {
					format!(
						"`offset_from` between a pointer into {} and one into {}, which JS keeps in different arrays",
						a.array.what(),
						b.array.what()
					)
				})
			}
			("ptr" | "NonNull", "add" | "sub" | "offset") => {
				let o = self.moved(self.origin(arg(0)?)?, name.as_str(), arg(1)?);
				out(o, None, "a pointer")
			}
			// A wrapping offset may leave any other array while nothing reads through it.
			("ptr" | "NonNull", "wrapping_add" | "wrapping_sub" | "wrapping_offset") => {
				let o = self.lone_origin(arg(0)?)?;
				out(self.moved(o, name.as_str(), arg(1)?), None, "a pointer")
			}
			// An access through a pointer, which the body shows one element at: both of a
			// swap's, and else the first argument, as the second is the value written.
			(
				"ptr" | "NonNull",
				"read" | "read_unaligned" | "read_volatile" | "write" | "write_unaligned" | "write_volatile"
				| "replace" | "swap" | "as_ref" | "as_mut" | "drop_in_place",
			) => [Some(arg(0)?), arg(1).filter(|_| name == "swap")]
				.into_iter()
				.flatten()
				.find_map(|p| self.access(self.lone_origin(p)?)),
			("raw", "from_raw_parts" | "from_raw_parts_mut")
			| ("ptr", "slice_from_raw_parts" | "slice_from_raw_parts_mut")
			| ("NonNull", "slice_from_raw_parts") => {
				let o = self.origin(arg(0)?)?;
				// A raw pointer to a slice of any other array reads nothing until a read
				// through it.
				if owner != "raw" && !matches!(o.array, Array::Lone(_)) {
					return None;
				}
				match self.const_int(arg(1)?) {
					Some(n) => out(o, Some(n), "a slice"),
					None => unfixed(o, "a slice of a length"),
				}
			}
			(
				"ptr",
				"copy"
				| "copy_nonoverlapping"
				| "copy_to"
				| "copy_to_nonoverlapping"
				| "copy_from"
				| "copy_from_nonoverlapping",
			)
			| ("NonNull", "copy_to" | "copy_to_nonoverlapping" | "copy_from" | "copy_from_nonoverlapping") => {
				let n = self.const_int(arg(2)?);
				[arg(0)?, arg(1)?].into_iter().find_map(|p| {
					let o = self.origin(p)?;
					match n {
						Some(_) => out(o, n, "a copy"),
						None => unfixed(o, "a copy of a count"),
					}
				})
			}
			("ptr" | "NonNull", "write_bytes") => {
				let o = self.origin(arg(0)?)?;
				match self.const_int(arg(2)?) {
					Some(n) => out(o, Some(n), "a write"),
					None => unfixed(o, "a write of a count"),
				}
			}
			_ => None,
		}
	}

	/// Why a read or write through the raw pointer `place` derefs would fail at run time, if
	/// the body shows that pointer off the element of an array of one.
	pub(super) fn beside(&self, place: mir::Place<'tcx>) -> Option<String> {
		let [ProjectionElem::Deref, ..] = place.projection.as_slice() else {
			return None;
		};
		// Every pointer to a value of no bytes is where it was, as `stray` leaves it.
		let ty::RawPtr(pointee, _) = *self.mir.local_decls[place.local].ty.kind() else {
			return None;
		};
		if self.b.t.zero_sized(pointee) || self.b.t.pointee(pointee) != Pointee::Place {
			return None;
		}
		self.access(self.lone_origin(&Operand::Copy(mir::Place::from(place.local)))?)
	}

	/// Why an access of the one element a pointer at `o` reads or writes reaches out of its
	/// array, if the body fixes where it is: one past it, as a pointer further out, or at
	/// an offset the body doesn't fix, is refused where it is made.
	fn access(&self, o: Origin<'tcx>) -> Option<String> {
		let len = o.array.len()?;
		(o.index? == len).then(|| Self::out(o, Some(1), "a read or write"))?
	}

	/// Where the pointer `op` points, if the body shows it in an array of one.
	fn lone_origin(&self, op: &Operand<'tcx>) -> Option<Origin<'tcx>> {
		self.origin(op).filter(|o| matches!(o.array, Array::Lone(_)))
	}

	/// Why an offset or a count the body doesn't fix, from `o`, may reach out of it: only
	/// from an array of one.
	fn unfixed(o: Origin<'tcx>, what: &str) -> Option<String> {
		matches!(o.array, Array::Lone(_)).then(|| {
			format!(
				"{what} the body doesn't fix, from {}, which JS keeps apart from what holds it in Rust",
				o.array.what()
			)
		})
	}

	/// Why `what`, at `o` and reaching `n` elements past it if a count, leaves `o`'s array.
	fn out(o: Origin<'tcx>, n: Option<i128>, what: &str) -> Option<String> {
		let Some(index) = o.index else {
			return Self::unfixed(o, &format!("{what} moved by an offset"));
		};
		let past = o.array.len().is_some_and(|len| index + n.unwrap_or(0) > len);
		(index < 0 || past).then(|| {
			format!(
				"{what} reaching past {}, which JS keeps apart from what holds it in Rust",
				o.array.what()
			)
		})
	}

	/// Whether JS keeps `a` and `b` in different arrays. Arrays of different variables are;
	/// of one variable's, those at different fields or constant indices are, where Rust's
	/// are one allocation.
	fn apart(&self, a: Array<'tcx>, b: Array<'tcx>) -> bool {
		let (Array::Owned(a), Array::Owned(b)) = (a, b) else {
			return a != b;
		};
		let index = |elem: ProjectionElem<Local, ty::Ty<'tcx>>| match elem {
			ProjectionElem::ConstantIndex {
				offset,
				from_end: false,
				..
			} => Some(i128::from(offset)),
			ProjectionElem::Index(local) => match self.def_rvalue(self.traced(&Operand::Copy(local.into()))?)? {
				Rvalue::Use(op, _) => self.const_int(op),
				_ => None,
			},
			_ => None,
		};
		a.local != b.local
			|| a.projection
				.iter()
				.zip(b.projection)
				.find(|(x, y)| x != y)
				.is_some_and(|pair| match pair {
					(ProjectionElem::Field(x, _), ProjectionElem::Field(y, _)) => x != y,
					(x, y) => index(x).zip(index(y)).is_some_and(|(x, y)| x != y),
				})
	}

	/// Where the pointer `op` points, if the body shows it.
	fn origin(&self, op: &Operand<'tcx>) -> Option<Origin<'tcx>> {
		let local = self.traced(op)?;
		if let Some((bb, i)) = self.def_at[local] {
			let rvalue = self.def_rvalue(local)?;
			return match rvalue {
				Rvalue::Use(op, _)
				| Rvalue::Cast(
					mir::CastKind::PtrToPtr
					| mir::CastKind::PointerCoercion(ty::adjustment::PointerCoercion::MutToConstPointer, _),
					op,
					_,
				) if self.same_pointee(op, local) => self.origin(op),
				Rvalue::RawPtr(mir::RawPtrKind::Const | mir::RawPtrKind::Mut, place) => self.raw_origin(
					*place,
					Location {
						block: bb,
						statement_index: i,
					},
				),
				_ => None,
			};
		}
		let (def_id, gargs, args) = self.def_call(local)?;
		let (owner, name) = library::key(self.tcx, def_id)?;
		match (owner.as_str(), name.as_str()) {
			("ptr", "from_ref" | "from_mut") | ("From", "from") | ("Into", "into")
				if crate::lower::pointer::from_reference(self.tcx, def_id, gargs) =>
			{
				let block = self.def_block(local)?;
				let at = Location {
					block,
					statement_index: self.mir.basic_blocks[block].statements.len(),
				};
				self.reference_origin(args[0].node.place()?, at)
			}
			("ptr" | "NonNull", "add" | "sub" | "offset") => {
				Some(self.moved(self.origin(&args[0].node)?, name.as_str(), &args[1].node))
			}
			// Only an array of one's, which every wrapping offset must stay within.
			("ptr" | "NonNull", "wrapping_add" | "wrapping_sub" | "wrapping_offset") => {
				Some(self.moved(self.lone_origin(&args[0].node)?, name.as_str(), &args[1].node))
			}
			("NonNull", "as_ptr" | "new_unchecked") | ("ptr" | "NonNull", "cast_const" | "cast_mut") => {
				self.origin(&args[0].node)
			}
			("ptr" | "NonNull", "cast")
				if self.b.t.uninit_inner(gargs.type_at(0)) == self.b.t.uninit_inner(gargs.type_at(1)) =>
			{
				self.origin(&args[0].node)
			}
			("slice" | "Vec", "as_ptr" | "as_mut_ptr") => self.start(&args[0].node),
			// The start of its own text's encoding, though a substring's bytes start past its
			// text's in Rust, so a move back before it is refused, as one off a copy is.
			("str", "as_ptr") => Some(Origin {
				array: Array::Text(self.def_block(local)?),
				index: Some(0),
			}),
			_ => None,
		}
	}

	/// Where a raw pointer to `place`, made at `at`, points, if the body shows it.
	fn raw_origin(&self, place: mir::Place<'tcx>, at: Location) -> Option<Origin<'tcx>> {
		if let Some(local) = self.raw_byte(place)
			&& matches!(self.slots[local], Slot::Byte(_))
		{
			return Some(Origin {
				array: Array::Byte(local),
				index: Some(0),
			});
		}
		match place.projection.as_slice() {
			// `&raw mut *p` is `p`.
			[ProjectionElem::Deref] if self.mir.local_decls[place.local].ty.is_raw_ptr() => {
				self.origin(&Operand::Copy(mir::Place::from(place.local)))
			}
			[rest @ .., ProjectionElem::Deref] => self.reference_origin(
				mir::Place {
					local: place.local,
					projection: self.tcx.mk_place_elems(rest),
				},
				at,
			),
			[
				array @ ..,
				ProjectionElem::Index(_) | ProjectionElem::ConstantIndex { .. },
			] => (!place.projection.contains(&ProjectionElem::Deref)).then(|| Origin {
				array: Array::Owned(mir::Place {
					local: place.local,
					projection: self.tcx.mk_place_elems(array),
				}),
				index: None,
			}),
			_ => Some(Origin {
				array: Array::Lone(at),
				index: Some(0),
			}),
		}
	}

	/// Where a raw pointer to what the reference `r` points at, made at `at`, points, as
	/// [`Self::behind_reference`] makes it: where the body shows `r` borrowed, and else at
	/// a place of its own, or, where `r` is an `rs.Place`, where that points.
	fn reference_origin(&self, r: mir::Place<'tcx>, at: Location) -> Option<Origin<'tcx>> {
		let ty = r.ty(&self.mir.local_decls, self.tcx).ty;
		// A reference to an `Arc`'s value points at that value's one place.
		if !ty.is_ref() || self.arc_of(&Operand::Copy(r)).is_some() {
			return None;
		}
		let pointee = self.tcx.mk_place_deref(r);
		let borrowed = self.through_aliases(pointee);
		if borrowed != pointee {
			return self.raw_origin(borrowed, at);
		}
		if self.indexed(r).is_some() {
			let (def_id, _, args) = self.def_call(r.as_local()?)?;
			let (owner, name) = library::key(self.tcx, def_id)?;
			if !matches!(
				(owner.as_str(), name.as_str()),
				("Index" | "IndexMut", "index" | "index_mut")
			) {
				return None;
			}
			let o = self.start(&args[0].node)?;
			return Some(Origin {
				index: o.index.zip(self.const_int(&args[1].node)).map(|(i, n)| i + n),
				..o
			});
		}
		if let Some(p) = self.reborrowed_pointer(r) {
			return self.origin(&Operand::Copy(p));
		}
		if self.b.t.place_pointee(ty).is_some() {
			return None;
		}
		Some(Origin {
			array: Array::Lone(at),
			index: Some(0),
		})
	}

	/// `o` moved by the offset `op` as `add`, `sub`, or `offset`, or a wrapping one, moves it.
	fn moved(&self, o: Origin<'tcx>, name: &str, op: &Operand<'tcx>) -> Origin<'tcx> {
		let n = self.const_int(op).map(|n| match name {
			"sub" | "wrapping_sub" => -n,
			_ => n,
		});
		Origin {
			index: o.index.zip(n).map(|(i, n)| i + n),
			..o
		}
	}

	/// Where the slice or `Vec`, or reference to one, `op` starts.
	fn start(&self, op: &Operand<'tcx>) -> Option<Origin<'tcx>> {
		let local = self.traced(op)?;
		if self.def_at[local].is_some() {
			return match self.def_rvalue(local)? {
				Rvalue::Use(op, _) | Rvalue::Cast(mir::CastKind::PointerCoercion(..), op, _) => self.start(op),
				Rvalue::Ref(_, _, place) | Rvalue::CopyForDeref(place) => match place.projection.as_slice() {
					[ProjectionElem::Deref] => self.start(&Operand::Copy(mir::Place::from(place.local))),
					projection
						if !projection.contains(&ProjectionElem::Deref)
							&& match place.ty(&self.mir.local_decls, self.tcx).ty.kind() {
								ty::Array(..) => true,
								ty::Adt(adt, _) => Std::of(self.tcx, adt.did()) == Some(Std::Vec),
								_ => false,
							} =>
					{
						Some(Origin {
							array: Array::Owned(*place),
							index: Some(0),
						})
					}
					_ => None,
				},
				_ => None,
			};
		}
		let (def_id, gargs, args) = self.def_call(local)?;
		// A shared slice of a byte in no array is a copy of it, as no `Uint8Array` can share
		// it, unless empty, which keeps the pointer. Any other slice is a view of where it is.
		match library::key(self.tcx, def_id)? {
			(owner, name) if owner == "raw" && matches!(name.as_str(), "from_raw_parts" | "from_raw_parts_mut") => {
				let o = self.origin(&args[0].node)?;
				let len = self.const_int(&args[1].node);
				match (crate::lower::byte(gargs.types().next()?), o.array) {
					(true, Array::Lone(_)) if len != Some(0) => Some(Origin {
						array: Array::Copy(self.def_block(local)?, len),
						index: Some(0),
					}),
					_ => Some(o),
				}
			}
			_ => None,
		}
	}

	/// The place holding the `Arc` or `Rc` whose value the shared reference `op` is, as
	/// `Deref` lends it, and which of the two it is, if the body shows it. The borrow keeps
	/// that place holding the same one while `op` lives.
	pub(super) fn arc_of(&self, op: &Operand<'tcx>) -> Option<(mir::Place<'tcx>, String)> {
		let local = self.traced(op)?;
		if self.def_at[local].is_some() {
			return match self.def_rvalue(local)? {
				Rvalue::Use(op, _) => self.arc_of(op),
				// `&*r`, of the reference `r`.
				Rvalue::Ref(_, mir::BorrowKind::Shared, place) => match place.projection.as_slice() {
					[ProjectionElem::Deref] if self.mir.local_decls[place.local].ty.is_ref() => {
						self.arc_of(&Operand::Copy(mir::Place::from(place.local)))
					}
					_ => None,
				},
				_ => None,
			};
		}
		let (def_id, gargs, args) = self.def_call(local)?;
		if library::key(self.tcx, def_id)? != ("Deref".into(), "deref".into()) {
			return None;
		}
		let ty::Adt(adt, _) = gargs.type_at(0).kind() else {
			return None;
		};
		if Std::of(self.tcx, adt.did()) != Some(Std::Arc) {
			return None;
		}
		let owner = self.tcx.item_name(adt.did()).to_string();
		Some((self.referent(&args.first()?.node)?, owner))
	}

	/// The place the shared reference `r` borrows, if the body shows it and that it holds the
	/// same value while the borrow lives: one a borrow here took, or what a reference it is
	/// given or assigns once points at.
	fn referent(&self, r: &Operand<'tcx>) -> Option<mir::Place<'tcx>> {
		let place = match self.traced(r) {
			Some(r) if self.def_at[r].is_none() => return Some(self.tcx.mk_place_deref(mir::Place::from(r))),
			Some(r) => match self.def_rvalue(r)? {
				Rvalue::Use(op, _) => return self.referent(op),
				Rvalue::Ref(_, _, place) => *place,
				_ => return None,
			},
			// A reference the function is given and never assigns.
			None => {
				let r = r.place()?.as_local()?;
				if !self.is_arg(r) || self.defs[r] != 0 {
					return None;
				}
				self.tcx.mk_place_deref(mir::Place::from(r))
			}
		};
		// Only through boxes, which the borrow keeps where they are, and a reference in a
		// variable the body never points elsewhere, by assigning or borrowing it, as it may
		// one the borrow went through.
		let l = place.local;
		let fixed = !self.borrowed[l]
			&& !self.mutated[l]
			&& match self.is_arg(l) {
				true => self.defs[l] == 0,
				false => self.defs[l] == 1,
			};
		let stays = place
			.iter_projections()
			.enumerate()
			.all(|(i, (base, elem))| match elem {
				ProjectionElem::Field(..) => true,
				ProjectionElem::Deref => {
					let ty = base.ty(&self.mir.local_decls, self.tcx).ty;
					ty.is_box() || (i == 0 && fixed && ty.is_ref())
				}
				_ => false,
			});
		stays.then_some(place)
	}

	/// The place the shared reference `r` borrows, as [`Self::referent`] has it, or the
	/// element `v[i]` it is where it is `&v[i]` of a `Vec` or slice by an index the body
	/// assigns once, which the borrow keeps where it is: two such borrows lend one element.
	fn lent(&self, r: &Operand<'tcx>) -> Option<mir::Place<'tcx>> {
		let indexed = self.traced(r).filter(|&l| self.def_at[l].is_none()).and_then(|l| {
			let (def_id, gargs, args) = self.def_call(l)?;
			let by_usize = gargs
				.types()
				.nth(1)
				.is_some_and(|ty| matches!(ty.kind(), ty::Uint(ty::UintTy::Usize)));
			if !by_usize || library::key(self.tcx, def_id)? != ("Index".into(), "index".into()) {
				return None;
			}
			let elem = match &args.get(1)?.node {
				Operand::Constant(_) => {
					let offset = u64::try_from(self.const_int(&args[1].node)?).ok()?;
					ProjectionElem::ConstantIndex {
						offset,
						min_length: offset + 1,
						from_end: false,
					}
				}
				op => {
					let i = op.place()?.as_local()?;
					let fixed = match self.is_arg(i) {
						true => self.defs[i] == 0,
						false => self.defs[i] == 1,
					};
					if !fixed || self.mutated[i] {
						return None;
					}
					ProjectionElem::Index(i)
				}
			};
			Some(self.lent(&args.first()?.node)?.project_deeper(&[elem], self.tcx))
		});
		indexed.or_else(|| self.referent(r))
	}

	/// The place of the `Bytes` whose storage the `Bytes` at `place` shares, as the body
	/// shows: through a move, or `slice` or `clone`, of one never assigned again,
	/// into a variable assigned once and never changed in place, which keeps that storage.
	fn storage(&self, place: mir::Place<'tcx>) -> mir::Place<'tcx> {
		let Some(local) = place.as_local() else {
			return place;
		};
		if self.defs[local] != 1 || self.is_arg(local) || self.mutated[local] {
			return place;
		}
		// A source never assigned again, and lent mutably only to `Bytes`' own methods, which
		// keep its storage, unlike `mem::swap` or `replace`.
		let fixed = |from: &mir::Place<'tcx>| {
			self.defs[from.local] == u32::from(!self.is_arg(from.local)) && self.only_own_edits(from.local)
		};
		if self.def_at[local].is_some() {
			return match self.def_rvalue(local) {
				Some(Rvalue::Use(Operand::Copy(from) | Operand::Move(from), _)) if fixed(from) => self.storage(*from),
				_ => place,
			};
		}
		let Some(bb) = self.def_block(local) else {
			return place;
		};
		let TerminatorKind::Call { func, args, .. } = &self.mir.basic_blocks[bb].terminator().kind else {
			return place;
		};
		let Some((def_id, gargs)) = func.const_fn_def() else {
			return place;
		};
		let shares = match library::key(self.tcx, def_id) {
			Some((owner, name)) => match (owner.as_str(), name.as_str()) {
				("Bytes", "slice") => library::inherent_std(self.tcx, def_id) == Some(Std::Bytes),
				("Clone", "clone") => gargs
					.types()
					.next()
					.is_some_and(|ty| self.b.t.std_of(ty) == Some(Std::Bytes)),
				_ => false,
			},
			None => false,
		};
		match args.first().filter(|_| shares).and_then(|a| self.referent(&a.node)) {
			Some(from) if fixed(&from) => self.storage(from),
			_ => place,
		}
	}

	/// Whether every `&mut` of the `Bytes` at `local` goes only to a method of `Bytes`, its own
	/// or a trait's, rather than to a generic function such as `mem::swap`.
	fn only_own_edits(&self, local: Local) -> bool {
		let lent: Vec<Local> = self
			.mir
			.basic_blocks
			.iter()
			.flat_map(|data| &data.statements)
			.filter_map(|stmt| match &stmt.kind {
				StatementKind::Assign(assign) => match &**assign {
					(to, Rvalue::Ref(_, mir::BorrowKind::Mut { .. }, place) | Rvalue::RawPtr(_, place))
						if place.local == local =>
					{
						Some(to.as_local())
					}
					_ => None,
				},
				_ => None,
			})
			.collect::<Option<_>>()
			.unwrap_or_else(|| vec![local]);
		lent.iter().all(|&r| {
			r != local
				&& self.mir.basic_blocks.iter().all(|data| {
					let read = |op: &Operand<'tcx>| op.place().is_some_and(|p| p.local == r);
					let in_stmt = data.statements.iter().any(|stmt| match &stmt.kind {
						StatementKind::Assign(assign) => match &assign.1 {
							Rvalue::Use(op, _) => read(op),
							Rvalue::Aggregate(_, ops) => ops.iter().any(read),
							Rvalue::Ref(_, _, p) | Rvalue::RawPtr(_, p) => p.local == r,
							_ => false,
						},
						_ => false,
					});
					let ok_call = match &data.terminator().kind {
						TerminatorKind::Call { func, args, .. } => {
							let rest = args.iter().skip(1).any(|a| read(&a.node));
							let own = |def_id, gargs: GenericArgsRef<'tcx>| {
								library::inherent_std(self.tcx, def_id) == Some(Std::Bytes)
									|| (self.tcx.trait_of_assoc(def_id).is_some()
										&& gargs
											.types()
											.next()
											.is_some_and(|ty| self.b.t.std_of(ty) == Some(Std::Bytes)))
							};
							!rest
								&& (!args.first().is_some_and(|a| read(&a.node))
									|| func.const_fn_def().is_some_and(|(def_id, gargs)| own(def_id, gargs)))
						}
						_ => true,
					};
					!in_stmt && ok_call
				})
		})
	}

	/// Whether the shared slice `subset` is of the storage of the `Bytes` the shared
	/// reference `bytes` is to, as the body shows: through the `Deref` or `as_ref`, or a slice
	/// of one, of that `Bytes` or one sharing its storage, as [`Self::storage`] has it. Only
	/// then does JS keep both in one buffer where Rust does.
	pub(super) fn bytes_of(&self, subset: &Operand<'tcx>, bytes: &Operand<'tcx>) -> bool {
		let Some(local) = self.traced(subset) else {
			return false;
		};
		if self.def_at[local].is_some() {
			return match self.def_rvalue(local) {
				Some(Rvalue::Use(op, _)) => self.bytes_of(op, bytes),
				// `&*s`, of the shared slice `s`.
				Some(Rvalue::Ref(_, mir::BorrowKind::Shared, place))
					if let [ProjectionElem::Deref] = place.projection.as_slice() =>
				{
					self.bytes_of(&Operand::Copy(mir::Place::from(place.local)), bytes)
				}
				_ => false,
			};
		}
		let Some((def_id, gargs, args)) = self.def_call(local) else {
			return false;
		};
		let Some(of) = args.first().map(|a| &a.node) else {
			return false;
		};
		let Some(self_ty) = gargs.types().next() else {
			return false;
		};
		match library::key(self.tcx, def_id) {
			Some((owner, name)) => match (owner.as_str(), name.as_str()) {
				("Index", "index") if self_ty.is_slice() => self.bytes_of(of, bytes),
				("Deref", "deref") | ("AsRef", "as_ref") if self.b.t.std_of(self_ty) == Some(Std::Bytes) => {
					let storage = |r| self.lent(r).map(|place| self.storage(place));
					storage(of).is_some_and(|place| storage(bytes) == Some(place))
				}
				_ => false,
			},
			None => false,
		}
	}

	/// `Arc::as_ptr` of the `Arc` or `Rc` whose value of a type JS copies the shared
	/// reference `r` is, if the body shows it: that value's one place, which `from_raw`
	/// takes back to it, rather than a place of its own as a pointer from a reference is.
	pub(super) fn arc_raw(&mut self, r: &Operand<'tcx>, span: rustc_span::Span, out: &mut Vec<Stmt>) -> Option<Expr> {
		let pointee = r.ty(&self.mir.local_decls, self.tcx).builtin_deref(true)?;
		if self.b.t.pointee(pointee) != Pointee::Place {
			return None;
		}
		let (place, owner) = self.arc_of(r)?;
		let arc = self.place_expr(place, span, out);
		let arc = match arc.unlocated() {
			Expr::Ident(_) => arc,
			_ => self.b.spill(arc, "arc", out),
		};
		let p = Expr::rs("arcPlace", vec![arc.clone()]);
		Some(Expr::rs("arcRaw", vec![arc, p, Expr::Str(owner)]))
	}

	/// Whether the raw pointer `op` to a value JS copies is an `Arc`'s or `Rc`'s, as
	/// `into_raw` or `as_ptr` gives, rather than a borrow's, if the body shows where it came
	/// from: an `Arc`'s is the one place of its value, and a borrow's a place of its own.
	pub(super) fn arc_pointer(&self, op: &Operand<'tcx>) -> Option<bool> {
		let local = self.traced(op)?;
		if self.def_at[local].is_some() {
			return match self.def_rvalue(local)? {
				Rvalue::Use(op, _)
				| Rvalue::Cast(
					mir::CastKind::PtrToPtr
					| mir::CastKind::PointerCoercion(ty::adjustment::PointerCoercion::MutToConstPointer, _),
					op,
					_,
				) if self.same_pointee(op, local) => self.arc_pointer(op),
				Rvalue::RawPtr(mir::RawPtrKind::Const | mir::RawPtrKind::Mut, place) => {
					match place.projection.as_slice() {
						[ProjectionElem::Deref] => {
							let base = Operand::Copy(mir::Place::from(place.local));
							match self.mir.local_decls[place.local].ty.is_raw_ptr() {
								true => self.arc_pointer(&base),
								false => Some(self.arc_of(&base).is_some()),
							}
						}
						_ => Some(false),
					}
				}
				_ => None,
			};
		}
		let (def_id, _, args) = self.def_call(local)?;
		let (owner, name) = library::key(self.tcx, def_id)?;
		match (owner.as_str(), name.as_str()) {
			("Arc" | "Rc", "into_raw" | "as_ptr") => Some(true),
			("ptr", "from_ref" | "from_mut") | ("From", "from") => Some(self.arc_of(&args.first()?.node).is_some()),
			("ptr" | "NonNull", "cast_const" | "cast_mut") | ("NonNull", "as_ptr" | "new_unchecked") => {
				self.arc_pointer(&args.first()?.node)
			}
			("ptr" | "NonNull", "cast") if self.same_pointee(&args.first()?.node, local) => {
				self.arc_pointer(&args.first()?.node)
			}
			("UnsafeCell", "get" | "raw_get") | ("Cell", "as_ptr") | ("MaybeUninit", "as_ptr" | "as_mut_ptr") => {
				Some(false)
			}
			_ => None,
		}
	}

	/// Why the cast of the raw pointer `op` to `from` to one to `to` can't be, if it erases
	/// one to a value JS copies to a `*const ()`, and the cast itself refuses no pointer to
	/// `from`: unless the body shows an `Arc`'s or `Rc`'s `into_raw` or `as_ptr` gave it, its
	/// value's one place, or that it is null or dangling, it has no one identity.
	pub(super) fn erased_place(&self, op: &Operand<'tcx>, from: ty::Ty<'tcx>, to: ty::Ty<'tcx>) -> Option<String> {
		let t = &self.b.t;
		let erases = t.identity_pointee(to) && !t.identity_pointee(from) && t.pointee(from) == Pointee::Place;
		if !erases || pointer::unerasable(self.b, from).is_some() || self.nowhere(op) {
			return None;
		}
		let pointee = t.uninit_inner(from);
		match self.arc_pointer(op) {
			Some(true) => None,
			Some(false) => Some(pointer::erased_place(pointee)),
			None => Some(format!(
				"a pointer to `{pointee}` cast to one to `()`, which only an `Arc`'s or `Rc`'s raw pointer keeps the place of, and the body doesn't show this one is"
			)),
		}
	}

	/// Whether the body shows the raw pointer `op` is null or dangling, at no memory.
	fn nowhere(&self, op: &Operand<'tcx>) -> bool {
		// A constant pointer of no allocation, an integer: null, or another, which a
		// constant's lowering refuses.
		if let Operand::Constant(c) = op {
			return (c.const_)
				.try_eval_scalar_int(self.tcx, self.b.t.typing_env_pub())
				.is_some();
		}
		let Some(local) = self.traced(op) else {
			return false;
		};
		if self.def_at[local].is_some() {
			return match self.def_rvalue(local) {
				Some(
					Rvalue::Use(op, _)
					| Rvalue::Cast(
						mir::CastKind::PtrToPtr
						| mir::CastKind::PointerCoercion(ty::adjustment::PointerCoercion::MutToConstPointer, _),
						op,
						_,
					),
				) => self.nowhere(op),
				_ => false,
			};
		}
		let Some((def_id, _, args)) = self.def_call(local) else {
			return false;
		};
		match library::key(self.tcx, def_id) {
			Some((owner, name)) => match (owner.as_str(), name.as_str()) {
				(
					"ptr",
					"null" | "null_mut" | "dangling" | "dangling_mut" | "without_provenance" | "without_provenance_mut",
				)
				| ("NonNull", "dangling") => true,
				("ptr" | "NonNull", "cast" | "cast_const" | "cast_mut") | ("NonNull", "as_ptr") => {
					args.first().is_some_and(|a| self.nowhere(&a.node))
				}
				_ => false,
			},
			None => false,
		}
	}

	/// The local `op` reads whole, if the body assigns it once.
	pub(super) fn traced(&self, op: &Operand<'tcx>) -> Option<Local> {
		let local = op.place()?.as_local()?;
		(self.defs[local] == 1 && !self.is_arg(local)).then_some(local)
	}

	/// Whether `op` is a pointer, raw or `NonNull`, to what `local` points at, through any
	/// `MaybeUninit`s or `ManuallyDrop`s, which a cast between keeps the pointer of.
	fn same_pointee(&self, op: &Operand<'tcx>, local: Local) -> bool {
		let pointee = |ty: ty::Ty<'tcx>| {
			let pointee = match ty.kind() {
				ty::Adt(adt, args) if Std::of(self.tcx, adt.did()) == Some(Std::NonNull) => Some(args.type_at(0)),
				_ => ty.builtin_deref(true),
			};
			pointee.map(|t| self.b.t.uninit_inner(t))
		};
		let from = op.ty(&self.mir.local_decls, self.tcx);
		pointee(from).is_some() && pointee(from) == pointee(self.mir.local_decls[local].ty)
	}

	/// The function, its generic arguments, and its arguments, of the call of std's that
	/// assigns `local`.
	pub(super) fn def_call(
		&self,
		local: Local,
	) -> Option<(
		rustc_hir::def_id::DefId,
		GenericArgsRef<'tcx>,
		&[Spanned<Operand<'tcx>>],
	)> {
		let bb = self.def_block(local)?;
		let TerminatorKind::Call { func, args, .. } = &self.mir.basic_blocks[bb].terminator().kind else {
			return None;
		};
		let ty::FnDef(def_id, gargs) = *func.ty(&self.mir.local_decls, self.tcx).kind() else {
			return None;
		};
		library::is_std(self.tcx, def_id).then_some((def_id, gargs.skip_binder(), args))
	}

	/// The block whose call assigns `local`.
	fn def_block(&self, local: Local) -> Option<BasicBlock> {
		self.mir
			.basic_blocks
			.iter_enumerated()
			.find_map(|(bb, data)| match &data.terminator().kind {
				TerminatorKind::Call { destination, .. } if destination.as_local() == Some(local) => Some(bb),
				_ => None,
			})
	}
}
