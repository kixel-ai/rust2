//! Slices of one byte's place. A `&mut [u8]` is an `rs.ByteSlice`, a view of a
//! `Uint8Array`, while a `&mut u8` is an `rs.Place` wherever the byte is. So
//! `slice::from_mut`, `array::from_mut`, and `Option::as_mut_slice` of one view the bytes it
//! is in, or for a byte in no array of bytes, such as a field's, copy it into a buffer of
//! its own, its mirror, an `rs.ByteMirror`. Rust reaches the byte only through the slice
//! while the slice may still be used, so the mirror is written back after each write
//! through a local that may hold the slice, or what is made of it, and each call or drop
//! given one, whenever its byte changed since the last write-back. Once nothing uses the slice, the mirror no longer
//! changes, so a later write-back writes nothing. A slice that may be kept where no local of
//! the body holds it, such as in memory a parameter points at, a coroutine's state, or what
//! the function returns, is refused, as is one an earlier run of the call made that may still
//! be used.

use rustc_data_structures::fx::{FxHashMap, FxHashSet};
use rustc_index::bit_set::DenseBitSet;
use rustc_middle::mir::visit::Visitor;
use rustc_middle::mir::{self, BasicBlock, Local, Location, Operand, TerminatorKind};
use rustc_middle::ty::{self, Ty};
use rustc_mir_dataflow::Analysis;
use rustc_mir_dataflow::impls::MaybeLiveLocals;

use super::holders::Trace;
use super::{Fx, Slot};
use crate::lower::library;
use crate::ts::{Expr, Stmt, StmtKind};

/// Why a slice of one byte's place kept where no variable of the function holds it is
/// refused.
const KEPT: &str = "a slice of one byte's place, which JS copies, kept where no variable of the function holds it, such as in memory a parameter points at or in what the function returns, so the copy is never written back";

/// Why a slice of one byte's place that may still be used once its call makes another is
/// refused.
const AGAIN: &str = "a slice of one byte's place, which JS copies, that may still be used once the call makes another, which the variable written back from holds instead";

/// The slices of one byte's place a body makes, and where each mirror is written back.
#[derive(Default)]
pub(super) struct Mirrors {
	/// By the block whose call makes one: how it is held.
	made: FxHashMap<BasicBlock, Made>,
	/// The blocks whose call makes one of a byte the body shows is in an array of bytes,
	/// whose slice is a view of it.
	pub views: FxHashSet<BasicBlock>,
	/// The mirrors to write back after each statement.
	after: FxHashMap<Location, Vec<String>>,
	/// The mirrors to write back as each block starts, after a call or drop that may have
	/// written them, by that block or one that only goes on to it.
	entry: FxHashMap<BasicBlock, Vec<String>>,
}

/// How a slice of one byte's place a call makes is held.
enum Made {
	/// Refused, for this reason.
	Refused(&'static str),
	/// Nothing writes through it, so its mirror, if any, needs no variable to be written back.
	Unwritten,
	/// In this variable, which its mirror, if any, is written back from.
	Var(String),
}

impl<'tcx> Fx<'_, '_, '_, 'tcx> {
	/// Whether `func` makes a slice of the one byte a `&mut` points at, or an `Option` holds.
	pub(super) fn byte_slice_call(&self, func: &Operand<'tcx>) -> bool {
		let tcx = self.tcx;
		let ty::FnDef(def_id, gargs) = *func.ty(&self.mir.local_decls, tcx).kind() else {
			return false;
		};
		if !library::is_std(tcx, def_id) || !gargs.skip_binder().types().next().is_some_and(crate::lower::byte) {
			return false;
		}
		match library::key(tcx, def_id) {
			Some((owner, name)) => match (owner.as_str(), name.as_str()) {
				("raw", "from_mut") => library::std_path(tcx, def_id) == ["slice", "raw", "from_mut"],
				("array", "from_mut") | ("Option", "as_mut_slice") => true,
				_ => false,
			},
			None => false,
		}
	}

	/// Each call making a slice of one byte's place, its mirror's variable, and where that
	/// is written back.
	pub(super) fn find_mirrors(&mut self) {
		let mir = self.mir;
		let mut live = None;
		let mut mirrors = Mirrors::default();
		for (bb, data) in mir.basic_blocks.iter_enumerated() {
			let TerminatorKind::Call {
				func,
				args,
				destination,
				..
			} = &data.terminator().kind
			else {
				continue;
			};
			if data.is_cleanup || !self.byte_slice_call(func) {
				continue;
			}
			if self.byte_in_array(&args[0].node) {
				mirrors.views.insert(bb);
				continue;
			}
			let site = mir.terminator_loc(bb);
			let Some(held) = self.slice_holders(*destination) else {
				mirrors.made.insert(bb, Made::Refused(KEPT));
				continue;
			};
			// What may still hold the slice an earlier run of the call made, which the one
			// variable can't keep apart from this run's. Only a call in a loop runs again
			// with this body's variables.
			if self.in_loop(bb) {
				let live = live.get_or_insert_with(|| {
					MaybeLiveLocals
						.iterate_to_fixpoint(self.tcx, mir, None)
						.into_results_cursor(mir)
				});
				live.seek_after_primary_effect(site);
				if held.iter().any(|h| live.get().contains(h)) {
					mirrors.made.insert(bb, Made::Refused(AGAIN));
					continue;
				}
			}
			let (mut after, mut entry) = (Vec::new(), Vec::new());
			for (block, data) in mir.basic_blocks.iter_enumerated() {
				if data.is_cleanup {
					continue;
				}
				// A write through what may hold it.
				for (statement_index, stmt) in data.statements.iter().enumerate() {
					if let mir::StatementKind::Assign(assign) = &stmt.kind
						&& held.contains(assign.0.local)
						&& assign.0.is_indirect()
					{
						after.push(Location { block, statement_index });
					}
				}
				// A call or drop given what may hold it, which may write through it.
				let term = data.terminator();
				if block == bb
					|| !matches!(term.kind, TerminatorKind::Call { .. } | TerminatorKind::Drop { .. })
					|| !reads(&held, |v| v.visit_terminator(term, Location::START))
				{
					continue;
				}
				entry.extend(term.successors().filter(|&to| !mir.basic_blocks[to].is_cleanup));
			}
			if after.is_empty() && entry.is_empty() {
				mirrors.made.insert(bb, Made::Unwritten);
				continue;
			}
			let name = self.b.fresh("mirror");
			for at in after {
				mirrors.after.entry(at).or_default().push(name.clone());
			}
			for to in entry {
				let due = mirrors.entry.entry(to).or_default();
				if !due.contains(&name) {
					due.push(name.clone());
				}
			}
			mirrors.made.insert(bb, Made::Var(name));
		}
		// A call that returns its result as is returns before its target block starts, so
		// one with a mirror due there returns through that block instead.
		self.returning
			.retain(|at| match &mir.basic_blocks[at.block].terminator().kind {
				TerminatorKind::Call { target: Some(to), .. }
					if at.statement_index == mir[at.block].statements.len() =>
				{
					!mirrors.entry.contains_key(to)
				}
				_ => true,
			});
		self.mirrors = mirrors;
	}

	/// Whether `op`, the `&mut u8` a slice of one is made of, is an alias of a byte the body
	/// shows is in an array of bytes: an element of one, by a place or by `index_mut` of a
	/// `Vec`, a slice, or an array of bytes, or a byte variable kept in a buffer of its own.
	fn byte_in_array(&self, op: &Operand<'tcx>) -> bool {
		let tcx = self.tcx;
		let Some(local) = op.place().and_then(|p| p.as_local()) else {
			return false;
		};
		let bytes = |ty: ty::Ty<'tcx>| match ty.kind() {
			ty::Slice(elem) | ty::Array(elem, _) => crate::lower::byte(*elem),
			ty::Adt(_, args) => self.b.t.std_of(ty) == Some(library::Std::Vec) && crate::lower::byte(args.type_at(0)),
			_ => false,
		};
		let mut local = local;
		for _ in 0..self.mir.local_decls.len() {
			if self.defs[local] != 1 {
				return false;
			}
			let Some(rvalue) = self.def_rvalue(local) else {
				// By `index_mut` of an element.
				return self.mir.basic_blocks.iter().any(|data| {
					let TerminatorKind::Call {
						func,
						args,
						destination,
						..
					} = &data.terminator().kind
					else {
						return false;
					};
					let ty::FnDef(def_id, gargs) = *func.ty(&self.mir.local_decls, tcx).kind() else {
						return false;
					};
					destination.as_local() == Some(local)
						&& library::key(tcx, def_id).is_some_and(|(o, n)| o == "IndexMut" && n == "index_mut")
						&& gargs.skip_binder().types().next().is_some_and(bytes)
						&& args
							.get(1)
							.is_some_and(|i| i.node.ty(&self.mir.local_decls, tcx).is_usize())
				});
			};
			// A reference lowers to an alias of its place only where its slot is one; what
			// `index_mut` returns, to one of the element it lends wherever it is read once.
			let mir::Rvalue::Ref(_, _, place) = rvalue else {
				return false;
			};
			if !matches!(self.slots[local], Slot::Alias(_)) {
				return false;
			}
			// Through a reborrow `&mut *r`.
			if place.projection.as_slice() == [mir::ProjectionElem::Deref] {
				local = place.local;
				continue;
			}
			return match place.iter_projections().last() {
				None => matches!(self.slots[place.local], Slot::Byte(_)),
				Some((base, mir::ProjectionElem::Index(_) | mir::ProjectionElem::ConstantIndex { .. })) => {
					bytes(base.ty(&self.mir.local_decls, tcx).ty)
				}
				Some(_) => false,
			};
		}
		false
	}

	/// The view of the byte the alias `local` names, if [`Self::byte_in_array`] found it in
	/// an array of bytes for the call ending the block being lowered.
	pub(super) fn byte_view(&self, local: Local) -> Option<Expr> {
		if !self.mirrors.views.contains(&self.at.block) {
			return None;
		}
		let Slot::Alias(Some(target)) = &self.slots[local] else {
			return None;
		};
		let Expr::Index(bytes, at) = target.unlocated() else {
			return None;
		};
		let slice = Expr::New(
			Box::new(Expr::Rs("ByteSlice".into())),
			vec![(**bytes).clone(), (**at).clone(), Expr::lit("1")],
		);
		Some(slice)
	}

	/// The locals that may hold the slice the call assigning `made` makes, or what is made of
	/// it; `None` if one may be kept where liveness can't follow it: in memory no local of
	/// this body owns, a closure or coroutine's state, or what the function returns. A call
	/// given one may put it in any result that could take a pointer, whatever its regions,
	/// since unsafe code may make one up; in an argument's memory only where borrow checking
	/// lets it, as [`Self::call_keeps`] reads that from its signature.
	fn slice_holders(&self, made: mir::Place<'tcx>) -> Option<DenseBitSet<Local>> {
		let mir = self.mir;
		let tcx = self.tcx;
		let memo = Default::default();
		let trace = Trace::new(self.b.t, mir, self.b.t.typing_env_pub(), &memo);
		let mut held = DenseBitSet::new_empty(mir.local_decls.len());
		// Whether `place`, given one, is a newly found holder's: by a value read out of a
		// holder, or with `taken`, by what a call returns or writes through one of its
		// arguments, which it can make only into a value that could take it.
		let keep = |held: &mut DenseBitSet<Local>, place: mir::Place<'tcx>, taken: bool| -> Option<bool> {
			let ty = place.ty(&mir.local_decls, tcx).ty;
			if !(match taken {
				true => trace.takes_pointer(ty),
				false => trace.carries(ty),
			}) {
				return Some(false);
			}
			let local = self.owner(place).filter(|&l| l != mir::RETURN_PLACE)?;
			Some(held.insert(local))
		};
		held.insert(self.owner(made).filter(|&l| l != mir::RETURN_PLACE)?);
		let mut grew = true;
		while grew {
			grew = false;
			for data in mir.basic_blocks.iter() {
				if data.is_cleanup {
					continue;
				}
				for stmt in &data.statements {
					match &stmt.kind {
						mir::StatementKind::Assign(assign) => {
							let (to, rvalue) = &**assign;
							if !reads(&held, |v| v.visit_rvalue(rvalue, Location::START)) {
								continue;
							}
							if let mir::Rvalue::Aggregate(kind, _) = rvalue
								&& matches!(
									**kind,
									mir::AggregateKind::Coroutine(..) | mir::AggregateKind::CoroutineClosure(..)
								) {
								return None;
							}
							grew |= keep(&mut held, *to, false)?;
						}
						mir::StatementKind::Intrinsic(_)
							if reads(&held, |v| v.visit_statement(stmt, Location::START)) =>
						{
							return None;
						}
						_ => {}
					}
				}
				let term = data.terminator();
				match &term.kind {
					TerminatorKind::Call {
						func,
						args,
						destination,
						..
					} => {
						let given: Vec<usize> = (0..args.len())
							.filter(|&i| args[i].node.place().is_some_and(|p| held.contains(p.local)))
							.collect();
						if given.is_empty() && !func.place().is_some_and(|p| held.contains(p.local)) {
							continue;
						}
						grew |= keep(&mut held, *destination, true)?;
						for j in self.call_keeps(func, args.len(), &given, |ty| trace.hides_raw_pointer(ty)) {
							// What the argument points at may be given one, if this body owns
							// it; memory reached through more pointers can't be followed.
							let ty = args[j].node.ty(&mir.local_decls, tcx);
							match (args[j].node.place(), ty.builtin_deref(true)) {
								(Some(p), Some(pointee)) if !trace.points_at_taker(pointee) => {
									grew |= keep(&mut held, tcx.mk_place_deref(p), true)?;
								}
								_ if trace.points_at_taker(ty) => return None,
								_ => {}
							}
						}
					}
					TerminatorKind::Yield { .. }
					| TerminatorKind::TailCall { .. }
					| TerminatorKind::InlineAsm { .. }
						if reads(&held, |v| v.visit_terminator(term, Location::START)) =>
					{
						return None;
					}
					_ => {}
				}
			}
		}
		Some(held)
	}

	/// The local of this body's own that holds `place`: its local, or through a reference
	/// made here once to a place of one, the local of that place; `None` for memory no local
	/// of this body owns, as through a parameter.
	fn owner(&self, place: mir::Place<'tcx>) -> Option<Local> {
		let mut place = place;
		for _ in 0..self.mir.local_decls.len() {
			let Some(at) = place.projection.iter().position(|e| e == mir::ProjectionElem::Deref) else {
				return Some(place.local);
			};
			if at != 0 || self.defs[place.local] != 1 {
				return None;
			}
			let (mir::Rvalue::Ref(_, _, to) | mir::Rvalue::RawPtr(_, to)) = self.def_rvalue(place.local)? else {
				return None;
			};
			place = to.project_deeper(&place.projection[1..], self.tcx);
		}
		None
	}

	/// Whether block `bb` may run again after it runs, past cleanup blocks, which abort.
	fn in_loop(&self, bb: BasicBlock) -> bool {
		let blocks = &self.mir.basic_blocks;
		let mut seen = DenseBitSet::new_empty(blocks.len());
		let mut stack: Vec<_> = blocks[bb].terminator().successors().collect();
		while let Some(block) = stack.pop() {
			if block == bb {
				return true;
			}
			if !blocks[block].is_cleanup && seen.insert(block) {
				stack.extend(blocks[block].terminator().successors());
			}
		}
		false
	}

	/// The arguments whose memory a call of `func`, given what may hold a slice in each
	/// argument at `given`, may put it in, as its signature lets borrow checking allow. A
	/// region the function's own signature binds, a late-bound one, is apart from every
	/// other: the slice reaches only an argument whose type names one of the regions of an
	/// argument holding it. A region or a type the caller picks may be shared with any
	/// other, so then any argument may take it. A raw pointer made from the slice names no
	/// region at all, so an argument pointing at memory `hides` says could hold one may
	/// take it too. This trusts the callee's unsafe code not to make up a lifetime that
	/// lets it keep the slice in an argument's memory, as it is trusted not to keep a raw
	/// pointer to it where no argument points, such as in a thread-local.
	fn call_keeps(
		&self,
		func: &Operand<'tcx>,
		n: usize,
		given: &[usize],
		hides: impl Fn(Ty<'tcx>) -> bool,
	) -> Vec<usize> {
		let tcx = self.tcx;
		let everything = (0..n).filter(|j| !given.contains(j)).collect();
		let ty::FnDef(def_id, _) = *func.ty(&self.mir.local_decls, tcx).kind() else {
			return everything;
		};
		let sig = tcx.fn_sig(def_id).instantiate_identity().skip_norm_wip().skip_binder();
		let mut regions = Vec::new();
		for &i in given {
			let Some(input) = sig.inputs().get(i) else {
				return everything;
			};
			for arg in input.walk() {
				match arg.kind() {
					ty::GenericArgKind::Lifetime(r) => match r.kind() {
						ty::ReBound(ty::BoundVarIndexKind::Bound(ty::INNERMOST), br) => regions.push(br.var),
						ty::ReStatic | ty::ReBound(..) => {}
						_ => return everything,
					},
					ty::GenericArgKind::Type(t) if matches!(t.kind(), ty::Param(_) | ty::Alias(..) | ty::Infer(_)) => {
						return everything;
					}
					_ => {}
				}
			}
		}
		let names = |ty: Ty<'tcx>| {
			ty.walk().any(|arg| {
				matches!(arg.kind(), ty::GenericArgKind::Lifetime(r)
					if matches!(r.kind(), ty::ReBound(ty::BoundVarIndexKind::Bound(ty::INNERMOST), br) if regions.contains(&br.var)))
			})
		};
		(0..n)
			.filter(|j| {
				!given.contains(j)
					&& sig
						.inputs()
						.get(*j)
						.is_none_or(|&t| names(t) || t.builtin_deref(true).is_some_and(&hides))
			})
			.collect()
	}

	/// The slice of the byte at `p`, an `rs.Place`, that the call ending the block being
	/// lowered makes, as its mirror's variable holds it.
	pub(super) fn byte_slice_of(&mut self, p: Expr, span: rustc_span::Span) -> Expr {
		match self.mirrors.made.get(&self.at.block) {
			Some(Made::Var(name)) => Expr::ident(name).assign(Expr::rs("byteMirror", vec![p])).member("s"),
			Some(Made::Unwritten) => Expr::rs("byteMirror", vec![p]).member("s"),
			Some(&Made::Refused(why)) => self.err(span, why),
			None => self.err(span, "a slice of one byte's place no call makes"),
		}
	}

	/// Write back the mirrors due as block `bb` starts.
	pub(super) fn mirror_entry(&mut self, bb: BasicBlock, out: &mut Vec<Stmt>) {
		let mut names: Vec<String> = Vec::new();
		for (&from, due) in &self.mirrors.entry {
			if self.forward(from) == bb {
				names.extend(due.iter().filter(|n| !names.contains(n)).cloned().collect::<Vec<_>>());
			}
		}
		names.sort();
		self.mirror_flush(names, out);
	}

	/// Write back the mirrors due after the statement at `at`.
	pub(super) fn mirror_after(&mut self, at: Location, out: &mut Vec<Stmt>) {
		let names = self.mirrors.after.get(&at).cloned().unwrap_or_default();
		self.mirror_flush(names, out);
	}

	fn mirror_flush(&mut self, names: Vec<String>, out: &mut Vec<Stmt>) {
		let stmts = names
			.into_iter()
			.map(|name| StmtKind::Expr(Expr::rs("byteMirrorFlush", vec![Expr::ident(&name)])).at(None))
			.collect::<Vec<_>>();
		if !stmts.is_empty() {
			self.emit(stmts, false, out);
		}
	}

	/// The variables of the body's mirrors.
	pub(super) fn mirror_names(&self) -> Vec<String> {
		let mut names: Vec<String> = (self.mirrors.made.values())
			.filter_map(|made| match made {
				Made::Var(name) => Some(name.clone()),
				_ => None,
			})
			.collect();
		names.sort();
		names
	}
}

/// Whether what `visit` visits uses one of `held`, but to assign all of it.
fn reads(held: &DenseBitSet<Local>, visit: impl FnOnce(&mut Reads<'_>)) -> bool {
	let mut reads = Reads { held, found: false };
	visit(&mut reads);
	reads.found
}

/// Finds a use of any of `held` but a whole assignment.
struct Reads<'h> {
	held: &'h DenseBitSet<Local>,
	found: bool,
}

impl<'tcx> Visitor<'tcx> for Reads<'_> {
	fn visit_local(&mut self, local: Local, cx: mir::visit::PlaceContext, _: Location) {
		use mir::visit::{MutatingUseContext, PlaceContext};
		let stored = matches!(
			cx,
			PlaceContext::NonUse(_) | PlaceContext::MutatingUse(MutatingUseContext::Store | MutatingUseContext::Call)
		);
		self.found |= !stored && self.held.contains(local);
	}
}
