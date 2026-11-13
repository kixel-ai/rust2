//! What may hold a raw pointer, so that rustc's liveness of those locals bounds where it
//! may still be read: the pointer made at the statement being lowered, or every pointer
//! into memory a write fills, which nothing points into yet if none may be read after.
//! Std's code given one, as its iterators are, keeps it only in what it returns if it can
//! run none of the crate's code: each of the crate's types it is given has no drop glue
//! and meets only bounds that call nothing or that std's own impls, inert in turn, meet.
//! Its code over a type parameter puts a value of it only where an argument naming that
//! parameter, or a bound relating the two, gave one, so the part of what it returns that
//! holds another parameter's values holds none, as a zip's other half does not.

use std::cell::RefCell;

use rustc_data_structures::either::Either;
use rustc_data_structures::fx::{FxHashMap, FxHashSet, FxIndexSet};
use rustc_hir::def_id::DefId;
use rustc_index::IndexVec;
use rustc_index::bit_set::DenseBitSet;
use rustc_middle::mir::visit::{PlaceContext, Visitor};
use rustc_middle::mir::{
	self, AggregateKind, BorrowKind, Local, Location, Operand, Place, ProjectionElem, RawPtrKind, Rvalue,
	StatementKind, TerminatorKind,
};
use rustc_middle::traits::ImplSource;
use rustc_middle::ty::{self, GenericArgs, Instance, Interner as _, Ty, TypeVisitableExt, TypingEnv};
use rustc_mir_dataflow::impls::MaybeLiveLocals;
use rustc_mir_dataflow::{Analysis, ResultsCursor};
use rustc_span::Spanned;

use super::{Fx, own};
use crate::lower::Translator;
use crate::lower::copy::{field_ty, instantiate};
use crate::lower::library::Std;
use crate::lower::pointer::{self, Derives};

impl<'tcx> Fx<'_, '_, '_, 'tcx> {
	/// The locals that may hold the raw pointer the statement being lowered makes, or what
	/// is made of it, as [`Trace::holders`] finds them. `None` if one may escape where
	/// liveness can't follow it.
	pub(super) fn holders(&self) -> Option<DenseBitSet<Local>> {
		let made = match self.mir.stmt_at(self.at) {
			Either::Left(stmt) => match &stmt.kind {
				StatementKind::Assign(assign) => assign.0,
				_ => return None,
			},
			Either::Right(term) => match &term.kind {
				TerminatorKind::Call { destination, .. } => *destination,
				_ => return None,
			},
		};
		if !own(made) {
			return None;
		}
		let memo = RefCell::default();
		let held = Trace::new(self.b.t, self.mir, self.b.t.typing_env_pub(), &memo).holders(
			made.local,
			Holds::Pointers,
			true,
			ty::List::empty(),
		);
		held.escapes.is_empty().then_some(held.all)
	}
}

/// A body's raw pointers, followed from where they are made.
pub(in crate::lower) struct Trace<'x, 'a, 'tcx> {
	t: &'x Translator<'a, 'tcx>,
	mir: &'x mir::Body<'tcx>,
	/// Where the body's trait bounds come from.
	env: TypingEnv<'tcx>,
	/// What each of the crate's functions does with the pointers it is given, as found.
	memo: &'x RefCell<Memo<'tcx>>,
	/// Each local's whole assignments, and whether anything else may change it.
	defs: RefCell<Option<IndexVec<Local, Defs<'x, 'tcx>>>>,
}

/// What each of the crate's functions does with the pointers it is given, and which of
/// std's run none of the crate's code.
#[derive(Default)]
pub(in crate::lower) struct Memo<'tcx> {
	/// Whether a function may keep any of the given parameters anywhere but in its result,
	/// with `loads` as [`Trace::holders`] takes it.
	kept: FxHashMap<(DefId, Vec<Local>, bool), bool>,
	/// Where each function's result may point: where its parameters do, or at a constant
	/// or a thread-local; or `None` if anywhere else.
	returned: FxHashMap<DefId, Option<Vec<Source<'tcx>>>>,
	/// Whether std's function or impl, given these arguments, is [`Trace::inert`].
	inert: FxHashMap<(DefId, GenericArgs<'tcx>), bool>,
}

/// The locals that may hold a pointer, and where one may escape them.
pub(in crate::lower) struct Held<'tcx> {
	pub all: DenseBitSet<Local>,
	/// What each of `all` holds none of the pointers in, as a [`Mask`].
	masks: FxHashMap<Local, Mask<'tcx>>,
	/// Those that may hold a pointer to a part of what one points at, or to anything else
	/// made of it, rather than only to its whole.
	pub parts: DenseBitSet<Local>,
	/// Those, not among `parts`, that may hold pointers std's code or indexing made of one
	/// instead, as an iterator over its elements does: each of a type that tells what it
	/// points at, a whole value of that type or of one holding it.
	elems: DenseBitSet<Local>,
	/// The local the pointers point to, if they are made of it rather than held in it.
	value: Option<Local>,
	/// Whether a value of a type parameter may itself be one of the pointers, as [`Holds`]
	/// tells.
	params: bool,
	/// Whether a value read through a holder holds the pointers too, as [`Trace::holders`]
	/// takes `loads`.
	loads: bool,
	/// Without `loads`, those that may point at a holder, or at memory one was put in,
	/// through which a read does hold the pointers.
	behind: DenseBitSet<Local>,
	/// Where one may escape where liveness can't follow it: into memory, a closure, an
	/// async body's state (which is memory), or a call that may keep it.
	pub escapes: Vec<Location>,
}

/// A local's whole assignments, the locals its mutable borrows are, and whether anything
/// else may change it.
#[derive(Default)]
struct Defs<'x, 'tcx> {
	whole: Vec<Def<'x, 'tcx>>,
	lent: Vec<Local>,
	other: bool,
}

#[derive(Clone, Copy)]
enum Def<'x, 'tcx> {
	Assign(&'x Rvalue<'tcx>),
	Call(&'x Operand<'tcx>, &'x [Spanned<Operand<'tcx>>]),
}

/// What the root of [`Trace::holders`] holds.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(in crate::lower) enum Holds {
	/// The pointers, each in a value whose type shows one. Neither std's code nor a body
	/// generic over a type parameter makes a value of it out of a pointer, but by reading
	/// one through a pointer cast to point at it.
	Pointers,
	/// The pointers, a type parameter's value among them, as a parameter of a function
	/// generic over it may be one.
	Any,
	/// What the pointers point at, which are made of it.
	Value,
}

/// How much of what a pointer was made for one may point at, finest last.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Level {
	Whole,
	/// As [`Held::elems`].
	Elems,
	Part,
}

/// Types whose values, wherever what a local holds or reaches has one, hold none of the
/// pointers followed and reach none of them: where std's code generic over a type
/// parameter keeps values of it that no argument holding one gave it.
type Mask<'tcx> = &'tcx ty::List<Ty<'tcx>>;

/// Where a pointer may have come from.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Source<'tcx> {
	/// Memory the call assigning this local, or changing it through a borrow, may have
	/// allocated, reached only through what the local holds outside the [`Mask`].
	Fresh(Local, Mask<'tcx>),
	/// A borrow of this local or a part of it.
	Local(Local),
	/// This parameter.
	Param(Local),
	/// A constant or a thread-local, which no local is.
	Static,
}

/// The memory a write fills.
#[derive(Clone, Copy)]
pub(in crate::lower) enum Filled {
	/// Where this local points.
	Through(Local),
	/// This local or a part of it.
	Local(Local),
}

impl<'x, 'a, 'tcx> Trace<'x, 'a, 'tcx> {
	pub fn new(
		t: &'x Translator<'a, 'tcx>,
		mir: &'x mir::Body<'tcx>,
		env: TypingEnv<'tcx>,
		memo: &'x RefCell<Memo<'tcx>>,
	) -> Self {
		Self {
			t,
			mir,
			env,
			memo,
			defs: RefCell::new(None),
		}
	}

	/// The locals that may hold the pointer `root` holds, or one to `root` itself, as
	/// `holds` tells, or what is made of it: a copy, a cast, a reference or pointer made
	/// through it, a value read through it that could hold one, or what std or the crate
	/// returns given one. Without `loads`, a value read through a pointer that points at no
	/// holder, as std's `ptr::read` reads one, holds none of them: what the pointer points
	/// at holds it only once it is put in memory, which is an escape. Only a pointer read
	/// after counts then, not one into memory a holder reaches. `root` holds none of them in
	/// what `mask` names.
	pub fn holders(&self, root: Local, holds: Holds, loads: bool, mask: Mask<'tcx>) -> Held<'tcx> {
		let mir = self.mir;
		let mut held = Held {
			all: DenseBitSet::new_empty(mir.local_decls.len()),
			masks: FxHashMap::from_iter([(root, mask)]),
			parts: DenseBitSet::new_empty(mir.local_decls.len()),
			elems: DenseBitSet::new_empty(mir.local_decls.len()),
			value: (holds == Holds::Value).then_some(root),
			params: holds == Holds::Any,
			loads,
			behind: DenseBitSet::new_empty(mir.local_decls.len()),
			escapes: Vec::new(),
		};
		held.all.insert(root);
		let mut grew = true;
		while grew {
			grew = false;
			held.escapes.clear();
			for (block, data) in mir.basic_blocks.iter_enumerated() {
				for (statement_index, stmt) in data.statements.iter().enumerate() {
					let at = Location { block, statement_index };
					match &stmt.kind {
						StatementKind::Assign(assign) => {
							let (to, rvalue) = &**assign;
							if !reads(&held.all, |v| v.visit_rvalue(rvalue, at)) || held.loaded(rvalue) {
								continue;
							}
							if let Rvalue::Aggregate(kind, _) = rvalue
								&& matches!(
									**kind,
									AggregateKind::Closure(..)
										| AggregateKind::Coroutine(..)
										| AggregateKind::CoroutineClosure(..)
								) {
								held.escapes.push(at);
								continue;
							}
							let level = match rvalue {
								Rvalue::Cast(_, op, ty) => {
									self.retyped(level(rvalue, &held), op.ty(&mir.local_decls, self.t.tcx), *ty)
								}
								_ => level(rvalue, &held),
							};
							// What a value read out of a holder, or a pointer to one, reaches is
							// what the holder does; a cast reads memory as another type.
							let mask = match rvalue {
								Rvalue::Use(Operand::Copy(p) | Operand::Move(p), _)
								| Rvalue::CopyForDeref(p)
								| Rvalue::Ref(_, _, p)
								| Rvalue::RawPtr(_, p) => held.mask(p.local),
								_ => ty::List::empty(),
							};
							match self.held_in(*to, level, mask, &mut held) {
								Some(new) => grew |= new,
								None => held.escapes.push(at),
							}
							// A pointer to a holder, through which a read holds the pointers.
							let borrowed =
								matches!(rvalue, Rvalue::Ref(_, _, p) | Rvalue::RawPtr(_, p) if !p.is_indirect());
							if !held.loads && (borrowed || reads(&held.behind, |v| v.visit_rvalue(rvalue, at))) {
								grew |= held.behind.insert(to.local);
							}
						}
						StatementKind::Intrinsic(_) if reads(&held.all, |v| v.visit_statement(stmt, at)) => {
							held.escapes.push(at);
						}
						_ => {}
					}
				}
				let at = mir.terminator_loc(block);
				let term = data.terminator();
				match &term.kind {
					TerminatorKind::Call {
						func,
						args,
						destination,
						..
					} => {
						let given: Vec<usize> = (0..args.len())
							.filter(|&i| reads_operand(&args[i].node, &held.all))
							.collect();
						if given.is_empty() && !reads_operand(func, &held.all) {
							continue;
						}
						let returned = destination.ty(&mir.local_decls, self.t.tcx).ty;
						let derives = match *func.ty(&mir.local_decls, self.t.tcx).kind() {
							ty::FnDef(def_id, _) => pointer::derives(self.t.tcx, def_id),
							_ => None,
						};
						let behind = given.iter().any(|&i| held.behind_arg(&args[i].node));
						if !held.loads && !behind && derives == Some(Derives::Loaded) {
							continue;
						}
						match self.call(func, args, returned, &given, &held) {
							Some((level, mask)) => match self.held_in(*destination, level, mask, &mut held) {
								Some(new) => grew |= new,
								None => held.escapes.push(at),
							},
							None => held.escapes.push(at),
						}
						// What else std or the crate returns may point at memory it put a
						// pointer in, as `Box::new` does.
						if !held.loads && (behind || derives != Some(Derives::Same)) {
							grew |= held.behind.insert(destination.local);
						}
					}
					TerminatorKind::Drop { place, .. }
						if held.all.contains(place.local)
							&& self.drops_own_code(place.ty(&mir.local_decls, self.t.tcx).ty) =>
					{
						held.escapes.push(at);
					}
					TerminatorKind::TailCall { .. }
					| TerminatorKind::InlineAsm { .. }
					| TerminatorKind::Yield { .. }
						if reads(&held.all, |v| v.visit_terminator(term, at)) =>
					{
						held.escapes.push(at);
					}
					_ => {}
				}
			}
		}
		held
	}

	/// Whether `to`, given a pointer at `level` of what it points at, outside what `mask`
	/// names, newly holds it; `None` if that is memory, reached through a pointer. A value
	/// of a type parameter, which a pointer is only as [`Holds`] tells, holds one only if
	/// read as a part.
	fn held_in(&self, to: Place<'tcx>, level: Level, mask: Mask<'tcx>, held: &mut Held<'tcx>) -> Option<bool> {
		let ty = to.ty(&self.mir.local_decls, self.t.tcx).ty;
		if !self.carries(ty) || level != Level::Part && !held.params && !self.shows_pointer(ty) || self.masked(ty, mask)
		{
			return Some(false);
		}
		if !own(to) {
			return None;
		}
		// A local assigned in part may be of a type the mask names.
		let mask = match to.projection.is_empty() {
			true if held.all.contains(to.local) => self.meet(held.mask(to.local), mask),
			true => mask,
			false => ty::List::empty(),
		};
		let mut new = held.all.insert(to.local);
		new |= held.masks.insert(to.local, mask) != Some(mask);
		// A local assigned in part holds whatever else it held, too.
		match level {
			_ if !to.projection.is_empty() => new |= held.parts.insert(to.local),
			Level::Part => new |= held.parts.insert(to.local),
			Level::Elems => new |= held.elems.insert(to.local),
			Level::Whole => {}
		}
		Some(new)
	}

	/// What a call of `func` given `args`, of which those at `given` hold pointers, returns
	/// as a `returned`: a pointer at what level of what one points at, outside what the
	/// [`Mask`] names; `None` if it may keep one anywhere else.
	fn call(
		&self,
		func: &Operand<'tcx>,
		args: &[Spanned<Operand<'tcx>>],
		returned: Ty<'tcx>,
		given: &[usize],
		held: &Held<'tcx>,
	) -> Option<(Level, Mask<'tcx>)> {
		if reads_operand(func, &held.all) {
			return None;
		}
		let ty::FnDef(def_id, gargs) = *func.ty(&self.mir.local_decls, self.t.tcx).kind() else {
			return None;
		};
		// What a given argument holds, or a part of it if it is no local's own value.
		let arg = |i: usize| match args[i].node.place() {
			Some(p) if !p.is_indirect() => held.of(p.local),
			_ => Level::Part,
		};
		if let Some(derives) = pointer::derives(self.t.tcx, def_id) {
			if pointer::stores(self.t.tcx, def_id) && given.iter().any(|&i| i > 0) {
				return None;
			}
			let level = match derives {
				Derives::Same if given == [0] => {
					self.retyped(arg(0), args[0].node.ty(&self.mir.local_decls, self.t.tcx), returned)
				}
				_ => Level::Part,
			};
			return Some((level, ty::List::empty()));
		}
		if let Some(did) = self.read_fn(def_id, gargs.skip_binder()) {
			let params = given.iter().map(|&i| Local::from_usize(i + 1)).collect();
			let loads = held.loads || given.iter().any(|&i| held.behind_arg(&args[i].node));
			return (!self.keeps(did, params, loads)).then_some((Level::Part, ty::List::empty()));
		}
		let tys: Vec<Ty<'tcx>> = given
			.iter()
			.map(|&i| args[i].node.ty(&self.mir.local_decls, self.t.tcx))
			.collect();
		self.leaves(def_id, gargs.skip_binder(), args, &tys).then(|| {
			let masks: Vec<_> = given
				.iter()
				.map(|&i| {
					(
						i,
						args[i].node.place().map_or(ty::List::empty(), |p| held.mask(p.local)),
					)
				})
				.collect();
			let level = given.iter().map(|&i| arg(i)).fold(Level::Elems, Level::max);
			(
				level,
				self.returned_mask(def_id, gargs.skip_binder(), args.len(), &masks),
			)
		})
	}

	/// `level`, of a pointer of type `from` made one of type `to`: a part where it pointed at
	/// [`Level::Elems`] and now points at another type, which its type no longer tells.
	fn retyped(&self, level: Level, from: Ty<'tcx>, to: Ty<'tcx>) -> Level {
		let tcx = self.t.tcx;
		let pointee = |ty: Ty<'tcx>| {
			let pointee = match *ty.kind() {
				ty::Ref(_, pointee, _) | ty::RawPtr(pointee, _) => pointee,
				ty::Adt(adt, args) if ty.is_box() || Std::of(tcx, adt.did()) == Some(Std::NonNull) => args.type_at(0),
				_ => return None,
			};
			Some(storage(tcx, self.peel(pointee)))
		};
		match (level, pointee(from), pointee(to)) {
			(Level::Elems, Some(from), Some(to)) if from != to => Level::Part,
			_ => level,
		}
	}

	/// The crate's function a call of `def_id` with `args` runs, if its body translates and
	/// takes its arguments as it declares them, which a closure's doesn't.
	fn read_fn(&self, def_id: DefId, args: ty::GenericArgs<'tcx>) -> Option<DefId> {
		let tcx = self.t.tcx;
		let instance = Instance::try_resolve(tcx, self.env, def_id, args).ok()??;
		let ty::InstanceKind::Item(did) = instance.def else {
			return None;
		};
		(self.t.lowers(self.env, instance)
			&& !crate::lower::library::is_std(tcx, did)
			&& !tcx.is_closure_like(did)
			&& tcx.def_kind(did).is_fn_like()
			&& tcx.is_mir_available(did))
		.then_some(did)
	}

	/// Whether the crate's function `did` may keep what its `params` hold anywhere but in
	/// its result, with `loads` as [`Self::holders`] takes it.
	fn keeps(&self, did: DefId, params: Vec<Local>, loads: bool) -> bool {
		let key = (did, params, loads);
		if let Some(&kept) = self.memo.borrow().kept.get(&key) {
			return kept;
		}
		// Until found, as for a function that calls itself, it may.
		self.memo.borrow_mut().kept.insert(key.clone(), true);
		let callee = self.callee(did);
		let kept = key.1.iter().any(|&param| {
			!callee
				.holders(param, Holds::Any, key.2, ty::List::empty())
				.escapes
				.is_empty()
		});
		self.memo.borrow_mut().kept.insert(key, kept);
		kept
	}

	/// Where the crate's function `did`'s result may point: where each [`Source::Param`]
	/// does, or at a [`Source::Static`]; or `None` if anywhere else.
	fn returned(&self, did: DefId) -> Option<Vec<Source<'tcx>>> {
		if let Some(returned) = self.memo.borrow().returned.get(&did) {
			return returned.clone();
		}
		self.memo.borrow_mut().returned.insert(did, None);
		let returned = self.callee(did).origins(mir::RETURN_PLACE, None).and_then(|sources| {
			sources
				.into_iter()
				.map(|source| match source {
					Source::Param(_) | Source::Static => Some(source),
					Source::Fresh(..) | Source::Local(_) => None,
				})
				.collect()
		});
		self.memo.borrow_mut().returned.insert(did, returned.clone());
		returned
	}

	/// The trace of the crate's function `did`'s body.
	fn callee(&self, did: DefId) -> Trace<'x, 'a, 'tcx> {
		let tcx = self.t.tcx;
		Trace::new(
			self.t,
			tcx.optimized_mir(did),
			TypingEnv::post_analysis(tcx, did),
			self.memo,
		)
	}

	/// Whether a call of std's `def_id` with `args`, of which the holders are `given`, keeps
	/// none of them anywhere but its result: std's code, calling none of the crate's, with
	/// no argument reaching somewhere that could take one in, as `Vec::push` does.
	fn leaves(
		&self,
		def_id: DefId,
		gargs: ty::GenericArgs<'tcx>,
		args: &[Spanned<Operand<'tcx>>],
		given: &[Ty<'tcx>],
	) -> bool {
		self.runs_std(def_id, gargs, args)
			&& !args.iter().any(|a| {
				let t = a.node.ty(&self.mir.local_decls, self.t.tcx);
				given.iter().any(|&h| self.t.stores(self.env, t, h))
			})
	}

	/// Whether a call of `def_id` with `args` runs std's code, calling none of the crate's.
	fn runs_std(&self, def_id: DefId, gargs: ty::GenericArgs<'tcx>, args: &[Spanned<Operand<'tcx>>]) -> bool {
		if self.translated(def_id, gargs) {
			return false;
		}
		match Instance::try_resolve(self.t.tcx, self.env, def_id, gargs) {
			Ok(Some(instance)) if matches!(instance.def, ty::InstanceKind::Item(_)) => {
				let did = instance.def_id();
				crate::lower::library::is_std(self.t.tcx, did)
					&& self.inert(did, instance.args)
					&& !self.opaque_sig(did)
			}
			_ => !gargs
				.types()
				.chain(args.iter().map(|a| a.node.ty(&self.mir.local_decls, self.t.tcx)))
				.any(|t| self.runs_own_code(t)),
		}
	}

	/// Whether std's function or impl `did`, given `args`, runs none of the crate's code: each
	/// argument that could reach it has no drop glue, and each bound of `did` that one meets
	/// calls nothing, as `Sized` or `Send`, or is met by std's own impl, inert in turn. Code
	/// generic over a type calls its methods only through its bounds, so an iterator over
	/// slots of the crate's type, bound by none, runs none of it.
	fn inert(&self, did: DefId, args: GenericArgs<'tcx>) -> bool {
		let tcx = self.t.tcx;
		let key = (did, args);
		if let Some(&inert) = self.memo.borrow().inert.get(&key) {
			return inert;
		}
		// Until found, as for an impl whose bounds come back to it, it may not be.
		self.memo.borrow_mut().inert.insert(key, false);
		let inert = args.types().all(|t| !self.drops_own_code(t))
			&& tcx
				.clauses_of(did)
				.instantiate(tcx, args)
				.clauses
				.into_iter()
				.all(|clause| {
					let Some(clause) = clause.skip_norm_wip().as_trait_clause() else {
						return true;
					};
					let trait_ref = tcx.instantiate_bound_regions_with_erased(clause).trait_ref;
					if !trait_ref.args.types().any(|t| self.runs_own_code(t))
						|| calls_nothing(tcx, trait_ref.def_id)
						|| tcx.is_lang_item(trait_ref.def_id, rustc_attr_ir::LangItem::Copy) && copies(tcx, did)
					{
						return true;
					}
					let Ok(trait_ref) =
						tcx.try_normalize_erasing_regions(self.env, ty::Unnormalized::new_wip(trait_ref))
					else {
						return false;
					};
					match tcx.codegen_select_candidate(self.env.as_query_input(trait_ref)) {
						Ok(ImplSource::UserDefined(imp)) => {
							crate::lower::library::is_std(tcx, imp.impl_def_id) && self.inert(imp.impl_def_id, imp.args)
						}
						_ => false,
					}
				});
		self.memo.borrow_mut().inert.insert(key, inert);
		inert
	}

	/// Whether a call of `def_id` with `args` is `vec![x; n]` of a value that shows no pointer
	/// and whose drop runs none of the crate's code. It runs the crate's code only as its
	/// element's `Clone`, which it gives the value it moves into the vector last, never the
	/// memory it returns, and which returns no pointer std's code could keep.
	fn repeats(&self, def_id: DefId, args: ty::GenericArgs<'tcx>) -> bool {
		let tcx = self.t.tcx;
		crate::lower::library::is_std(tcx, def_id)
			&& matches!(crate::lower::library::key(tcx, def_id), Some((owner, name)) if owner == "vec" && name == "from_elem_global")
			&& !self.shows_pointer(args.type_at(0))
			&& !self.drops_own_code(args.type_at(0))
	}

	/// Whether std's function `did` takes or returns what could run code that its generic
	/// arguments don't tell: a trait object or a function pointer.
	fn opaque_sig(&self, did: DefId) -> bool {
		let sig = self
			.t
			.tcx
			.fn_sig(did)
			.instantiate_identity()
			.skip_norm_wip()
			.skip_binder();
		sig.inputs_and_output.iter().any(|ty| {
			ty.walk().filter_map(|arg| arg.as_type()).any(|t| {
				matches!(
					t.kind(),
					ty::Dynamic(..) | ty::FnPtr(..) | ty::Closure(..) | ty::CoroutineClosure(..) | ty::Coroutine(..)
				)
			})
		})
	}

	/// Whether a call of `def_id` with `args` runs a function whose body translates.
	fn translated(&self, def_id: DefId, args: ty::GenericArgs<'tcx>) -> bool {
		match Instance::try_resolve(self.t.tcx, self.env, def_id, args) {
			Ok(Some(instance)) => self.t.lowers(self.env, instance) || self.t.dispatches(instance),
			_ => false,
		}
	}

	/// Whether std's code handed a value of `ty` may call the crate's on it, which could
	/// keep what it is given: through the crate's own types, a trait object, a closure, or
	/// a function pointer.
	fn runs_own_code(&self, ty: Ty<'tcx>) -> bool {
		self.t.reaches(self.env, ty, |t| self.own_code(t))
	}

	/// Whether dropping a value of `ty` may run the crate's code: it has drop glue and holds
	/// what [`Self::runs_own_code`] counts that has drop glue of its own in the body's
	/// environment, which a `Copy` type, the crate's or a type parameter bound by it, has not.
	fn drops_own_code(&self, ty: Ty<'tcx>) -> bool {
		let tcx = self.t.tcx;
		ty.needs_drop(tcx, self.env)
			&& self
				.t
				.reaches(self.env, ty, |t| t.needs_drop(tcx, self.env) && self.own_code(t))
	}

	/// Whether `ty` itself, not what it holds, is what [`Self::runs_own_code`] counts.
	fn own_code(&self, ty: Ty<'tcx>) -> bool {
		match ty.kind() {
			ty::Adt(adt, _) => self.t.own(adt.did()),
			ty::FnDef(def_id, args) => self.translated(*def_id, args.skip_binder()),
			ty::Dynamic(..)
			| ty::Closure(..)
			| ty::CoroutineClosure(..)
			| ty::Coroutine(..)
			| ty::FnPtr(..)
			| ty::Param(_)
			| ty::Alias(..) => true,
			_ => false,
		}
	}

	/// Whether a value of `ty` could hold a pointer or a reference, in any field of any type.
	pub fn carries(&self, ty: Ty<'tcx>) -> bool {
		self.holds_pointer(ty, true)
	}

	/// Whether code reading a value of `ty` could get a pointer or a reference out of it: as
	/// [`Self::carries`], but for one a type parameter's value holds, which code generic
	/// over it gets only as a whole value of that parameter, never a pointer itself.
	fn shows_pointer(&self, ty: Ty<'tcx>) -> bool {
		self.holds_pointer(ty, false)
	}

	/// Whether safe code could put a pointer or a reference in a value of `ty`: as
	/// [`Self::carries`], but std's types hold only what their type arguments do, as the
	/// pointers of their own are private.
	pub fn takes_pointer(&self, ty: Ty<'tcx>) -> bool {
		self.holds_pointer_as(ty, true, true)
	}

	/// Whether a value of `ty` points at memory safe code could put a pointer or a
	/// reference in, as [`Self::takes_pointer`] tells, through a pointer it holds.
	pub fn points_at_taker(&self, ty: Ty<'tcx>) -> bool {
		let tcx = self.t.tcx;
		let mut seen = FxHashSet::default();
		let mut stack = vec![ty];
		while let Some(ty) = stack.pop() {
			if !seen.insert(ty) {
				continue;
			}
			match ty.kind() {
				ty::Bool
				| ty::Char
				| ty::Int(_)
				| ty::Uint(_)
				| ty::Float(_)
				| ty::Str
				| ty::Never
				| ty::FnDef(..)
				| ty::FnPtr(..) => {}
				ty::Ref(_, inner, _) | ty::RawPtr(inner, _) if self.takes_pointer(*inner) => return true,
				ty::Ref(..) | ty::RawPtr(..) => {}
				ty::Array(elem, _) | ty::Slice(elem) | ty::Pat(elem, _) => stack.push(*elem),
				ty::Tuple(tys) => stack.extend(tys.iter()),
				ty::Closure(_, args) => stack.extend(args.as_closure().upvar_tys()),
				// A raw pointer of std's points as one does.
				ty::Adt(adt, args) if raw_carrier(tcx, adt.did()) => {
					if args.types().any(|t| self.takes_pointer(t)) {
						return true;
					}
				}
				ty::Adt(adt, args) if crate::lower::library::is_std(tcx, adt.did()) => stack.extend(args.types()),
				ty::Adt(adt, args) => {
					stack.extend(adt.all_fields().map(|field| field_ty(tcx, self.env, field.did, *args)))
				}
				_ => return true,
			}
		}
		false
	}

	/// Whether a value of `ty` could hold a raw pointer, which names no region of the
	/// reference it was made from: in a field, behind a reference, in a std type's type
	/// argument or a [`raw_carrier`], in a type parameter's value, or in a trait object's.
	pub fn hides_raw_pointer(&self, ty: Ty<'tcx>) -> bool {
		let tcx = self.t.tcx;
		let mut seen = FxHashSet::default();
		let mut stack = vec![ty];
		while let Some(ty) = stack.pop() {
			if !seen.insert(ty) {
				continue;
			}
			match ty.kind() {
				ty::Bool
				| ty::Char
				| ty::Int(_)
				| ty::Uint(_)
				| ty::Float(_)
				| ty::Str
				| ty::Never
				| ty::FnDef(..)
				| ty::FnPtr(..) => {}
				ty::Ref(_, inner, _) | ty::Array(inner, _) | ty::Slice(inner) | ty::Pat(inner, _) => stack.push(*inner),
				ty::Tuple(tys) => stack.extend(tys.iter()),
				ty::Closure(_, args) => stack.extend(args.as_closure().upvar_tys()),
				ty::Adt(adt, _) if raw_carrier(tcx, adt.did()) => return true,
				ty::Adt(adt, args) if crate::lower::library::is_std(tcx, adt.did()) => stack.extend(args.types()),
				ty::Adt(adt, args) => {
					stack.extend(adt.all_fields().map(|field| field_ty(tcx, self.env, field.did, *args)))
				}
				_ => return true,
			}
		}
		false
	}

	/// Whether a value of `ty` could hold a pointer or a reference, in any field of any
	/// type, and with `params`, in a type parameter's value.
	fn holds_pointer(&self, ty: Ty<'tcx>, params: bool) -> bool {
		self.holds_pointer_as(ty, params, false)
	}

	/// [`Self::holds_pointer`], or with `std_args`, [`Self::takes_pointer`].
	fn holds_pointer_as(&self, ty: Ty<'tcx>, params: bool, std_args: bool) -> bool {
		let mut seen = FxHashSet::default();
		let mut stack = vec![ty];
		while let Some(ty) = stack.pop() {
			if !seen.insert(ty) {
				continue;
			}
			match ty.kind() {
				ty::Bool
				| ty::Char
				| ty::Int(_)
				| ty::Uint(_)
				| ty::Float(_)
				| ty::Str
				| ty::Never
				| ty::FnDef(..)
				| ty::FnPtr(..) => {}
				ty::Array(elem, _) | ty::Slice(elem) | ty::Pat(elem, _) => stack.push(*elem),
				ty::Tuple(tys) => stack.extend(tys.iter()),
				ty::Adt(adt, args) if std_args && crate::lower::library::is_std(self.t.tcx, adt.did()) => {
					if args.regions().next().is_some() || raw_carrier(self.t.tcx, adt.did()) {
						return true;
					}
					stack.extend(args.types());
				}
				ty::Adt(adt, args) => stack.extend(
					adt.all_fields()
						.map(|field| field_ty(self.t.tcx, self.env, field.did, *args)),
				),
				ty::Param(_) if !params => {}
				_ => return true,
			}
		}
		false
	}

	/// Whether a write at `at` of a `filled` value fills memory nothing points into yet:
	/// where it may run, no pointer into the memory has escaped, and every local that may
	/// hold one and be read after, but the call's own result `made`, holds only pointers to
	/// the whole of what it was made for, which the write fills in place and gives only its
	/// parts other objects, or, as one made by std's code or indexing, to no part of a
	/// `filled` value.
	pub fn fresh(
		&self,
		filled: Filled,
		ty: Ty<'tcx>,
		at: Location,
		made: Option<Local>,
		live: &mut ResultsCursor<'x, 'tcx, MaybeLiveLocals>,
	) -> bool {
		let sources = match filled {
			Filled::Through(pointer) => match self.origins(pointer, Some(ty)) {
				Some(sources) => sources,
				None => return false,
			},
			Filled::Local(local) => FxIndexSet::from_iter([Source::Local(local)]),
		};
		sources.into_iter().all(|source| {
			let held = match source {
				Source::Fresh(root, mask) => self.holders(root, Holds::Pointers, true, mask),
				Source::Local(root) => self.holders(root, Holds::Value, true, ty::List::empty()),
				Source::Param(_) | Source::Static => return false,
			};
			if held.escapes.iter().any(|&escape| self.reaches(escape, at)) {
				return false;
			}
			live.seek_before_primary_effect(at);
			let live = live.get();
			let parts = held
				.parts
				.iter()
				.any(|local| Some(local) != made && live.contains(local));
			!parts
				&& held.elems.iter().all(|local| {
					Some(local) == made
						|| !live.contains(local)
						|| !self.points_inside(self.mir.local_decls[local].ty, ty, held.mask(local))
				})
		})
	}

	/// Whether a value of `ty` may hold a pointer into a part of a `filled` value, as its
	/// type tells: to a type a `filled` value holds in place, outside what `mask` names.
	fn points_inside(&self, ty: Ty<'tcx>, filled: Ty<'tcx>, mask: Mask<'tcx>) -> bool {
		let tcx = self.t.tcx;
		let filled = self.peel(filled);
		let parts = self.inline_parts(filled);
		self.points_to(ty, mask, |pointee| {
			pointee != filled
				&& parts
					.as_ref()
					.is_none_or(|parts| parts.contains(&storage(tcx, pointee)))
		})
	}

	/// Whether a value of `ty` may hold a pointer, or one behind it, to a value `to` holds
	/// for, as what it points at is peeled, outside what `mask` names; or one its type
	/// doesn't tell. A vector's buffer is its own allocation, never a part of another value,
	/// so only what its elements hold may point at one.
	fn points_to(&self, ty: Ty<'tcx>, mask: Mask<'tcx>, to: impl Fn(Ty<'tcx>) -> bool) -> bool {
		let mut seen = FxHashSet::default();
		let mut stack = vec![ty];
		while let Some(ty) = stack.pop() {
			if !seen.insert(ty) || self.masked(ty, mask) {
				continue;
			}
			match ty.kind() {
				ty::Ref(_, pointee, _) | ty::RawPtr(pointee, _) => {
					let pointee = self.peel(*pointee);
					if to(pointee) {
						return true;
					}
					stack.push(pointee);
				}
				ty::Adt(adt, args)
					if crate::lower::library::std_path(self.t.tcx, adt.did()) == ["raw_vec", "RawVec"] =>
				{
					stack.extend(args.types())
				}
				_ => match self.fields(ty) {
					Some(fields) => stack.extend(fields),
					None => return true,
				},
			}
		}
		false
	}

	/// Whether code given a shared reference to a value of `ty` may change what it holds or
	/// reaches: through an `UnsafeCell` in a field, behind a pointer, or in a type argument,
	/// as a hash map's buffer holds its entries; or in one its type doesn't tell. A type
	/// parameter's value holds none the body can read a pointer out of but through the
	/// crate's code, which [`Self::origins`] follows on its own.
	fn mutable_through_shared(&self, ty: Ty<'tcx>) -> bool {
		let tcx = self.t.tcx;
		let mut seen = FxHashSet::default();
		let mut stack = vec![ty];
		while let Some(ty) = stack.pop() {
			if !seen.insert(ty) {
				continue;
			}
			match ty.kind() {
				ty::Adt(adt, _) if tcx.is_lang_item(adt.did(), rustc_attr_ir::LangItem::UnsafeCell) => return true,
				ty::Adt(_, args) => stack.extend(args.types()),
				ty::Ref(_, pointee, _) | ty::RawPtr(pointee, _) => {
					stack.push(*pointee);
					continue;
				}
				_ => {}
			}
			match self.fields(ty) {
				Some(fields) => stack.extend(fields),
				None => return true,
			}
		}
		false
	}

	/// Whether a value of `ty` may be or hold a `filled` value in place.
	fn holds_in_place(&self, ty: Ty<'tcx>, filled: Ty<'tcx>) -> bool {
		let tcx = self.t.tcx;
		let (ty, filled) = (self.peel(ty), storage(tcx, self.peel(filled)));
		storage(tcx, ty) == filled || self.inline_parts(ty).is_none_or(|parts| parts.contains(&filled))
	}

	/// The types a value of `ty` holds in place, as storage, but `ty` itself; `None` if it
	/// may hold one its type doesn't tell.
	fn inline_parts(&self, ty: Ty<'tcx>) -> Option<FxHashSet<Ty<'tcx>>> {
		let tcx = self.t.tcx;
		let mut parts = FxHashSet::default();
		let mut stack = self.fields(ty)?;
		while let Some(part) = stack.pop() {
			if matches!(part.kind(), ty::Ref(..) | ty::RawPtr(..)) {
				continue;
			}
			let part = self.peel(part);
			if parts.insert(storage(tcx, part)) {
				stack.extend(self.fields(part)?);
			}
		}
		Some(parts)
	}

	/// The types a value of `ty` holds in place, through std's types too; `None` if its type
	/// doesn't tell, as a trait object's. A type parameter's value holds none the body or
	/// std's code can see: code generic over it sees inside only through its bounds, which
	/// run the crate's code, and std's code that reinterprets memory is refused.
	fn fields(&self, ty: Ty<'tcx>) -> Option<Vec<Ty<'tcx>>> {
		Some(match ty.kind() {
			ty::Param(_) => Vec::new(),
			ty::Bool
			| ty::Char
			| ty::Int(_)
			| ty::Uint(_)
			| ty::Float(_)
			| ty::Str
			| ty::Never
			| ty::FnDef(..)
			| ty::FnPtr(..)
			| ty::Ref(..)
			| ty::RawPtr(..) => Vec::new(),
			ty::Array(elem, _) | ty::Slice(elem) | ty::Pat(elem, _) => vec![*elem],
			ty::Tuple(tys) => tys.to_vec(),
			ty::Adt(adt, args) => adt
				.all_fields()
				.map(|field| field_ty(self.t.tcx, self.env, field.did, *args))
				.collect(),
			_ => return None,
		})
	}

	/// Whether `mask` names `ty`, as storage JS keeps alike.
	fn masked(&self, ty: Ty<'tcx>, mask: Mask<'tcx>) -> bool {
		mask.iter().any(|m| self.same(m, ty))
	}

	/// Whether values of `a` and `b` are kept as the same storage.
	fn same(&self, a: Ty<'tcx>, b: Ty<'tcx>) -> bool {
		let tcx = self.t.tcx;
		storage(tcx, self.peel(a)) == storage(tcx, self.peel(b))
	}

	/// The [`Mask`] of `tys`.
	fn mask(&self, tys: impl IntoIterator<Item = Ty<'tcx>>) -> Mask<'tcx> {
		let tys: FxIndexSet<_> = tys.into_iter().collect();
		self.t.tcx.mk_type_list_from_iter(tys.into_iter())
	}

	/// The types both masks name.
	fn meet(&self, a: Mask<'tcx>, b: Mask<'tcx>) -> Mask<'tcx> {
		self.mask(a.iter().filter(|t| b.contains(t)))
	}

	/// `mask`, of a value read out of `place` that may hold the pointers, or with `deep`
	/// reach them, as of `place`'s local: less the types of what holds the value there, with
	/// those of what lies beside it but any the value holds, or with `deep` reaches.
	fn narrowed(&self, place: Place<'tcx>, mask: Mask<'tcx>, deep: bool) -> Mask<'tcx> {
		let tcx = self.t.tcx;
		let mut outer = Vec::new();
		let mut beside = Vec::new();
		for (base, elem) in place.iter_projections() {
			let base = base.ty(&self.mir.local_decls, tcx);
			outer.push(base.ty);
			match (elem, base.ty.kind()) {
				(ProjectionElem::Field(field, _), ty::Tuple(tys)) => beside.extend(
					tys.iter()
						.enumerate()
						.filter(|&(i, _)| i != field.index())
						.map(|(_, t)| t),
				),
				(ProjectionElem::Field(field, _), ty::Adt(adt, args)) if !adt.is_union() => {
					let variant = adt.variant(base.variant_index.unwrap_or(rustc_abi::FIRST_VARIANT));
					beside.extend(
						variant
							.fields
							.iter_enumerated()
							.filter(|&(i, _)| i != field)
							.map(|(_, f)| field_ty(tcx, self.env, f.did, *args)),
					);
				}
				(ProjectionElem::Downcast(_, at), ty::Adt(adt, args)) => beside.extend(
					adt.variants()
						.iter_enumerated()
						.filter(|&(i, _)| i != at)
						.flat_map(|(_, v)| v.fields.iter().map(|f| field_ty(tcx, self.env, f.did, *args))),
				),
				(
					ProjectionElem::Index(_) | ProjectionElem::ConstantIndex { .. } | ProjectionElem::Subslice { .. },
					_,
				) => {}
				_ => return ty::List::empty(),
			}
		}
		let value = place.ty(&self.mir.local_decls, tcx).ty;
		let outer = |m: Ty<'tcx>| outer.iter().any(|&o| self.same(o, m));
		let held = |m: Ty<'tcx>| self.shows(value, None, |_| false, deep, |t| self.same(t, m));
		self.mask(
			mask.iter()
				.filter(|&m| !outer(m))
				.chain(beside.into_iter().filter(|&m| !outer(m) && !held(m))),
		)
	}

	/// The body a call of std's `def_id` with `args`, given `count` arguments, runs, over its
	/// own generics; `None` if it takes another count, as a closure's call does.
	fn generic(&self, def_id: DefId, args: GenericArgs<'tcx>, count: usize) -> Option<Generic<'tcx>> {
		let tcx = self.t.tcx;
		let instance = Instance::try_resolve(tcx, self.env, def_id, args).ok()??;
		let ty::InstanceKind::Item(did) = instance.def else {
			return None;
		};
		// As its own bounds normalize it, so that `Self::Item` names what it is made of.
		let sig = tcx.instantiate_bound_regions_with_erased(tcx.fn_sig(did).instantiate_identity().skip_norm_wip());
		let sig = tcx
			.try_normalize_erasing_regions(TypingEnv::post_analysis(tcx, did), ty::Unnormalized::new_wip(sig))
			.unwrap_or(sig);
		if sig.inputs().len() != count {
			return None;
		}
		let mut generic = Generic {
			args: instance.args,
			inputs: sig.inputs(),
			output: sig.output(),
			bounds: Vec::new(),
		};
		for clause in tcx.clauses_of(did).instantiate_identity(tcx).clauses {
			let tys: Vec<Ty<'tcx>> = match clause.kind().skip_binder() {
				ty::ClauseKind::Trait(bound) => bound.trait_ref.args.types().collect(),
				ty::ClauseKind::Projection(bound) => {
					bound.projection_term.args.types().chain(bound.term.as_type()).collect()
				}
				_ => continue,
			};
			let mut named: FxHashSet<u32> = tys.iter().flat_map(|&t| params(t)).collect();
			if tys
				.iter()
				.any(|&t| self.shows(t, Some(&generic), |_| true, true, pointer))
			{
				named.insert(OPEN);
			}
			if named.len() > 1 {
				generic.bounds.push(named);
			}
		}
		Some(generic)
	}

	/// What of the result of a call of std's `def_id` with `args`, given `count` arguments
	/// of which those at `given` hold the pointers outside what their mask names, holds none
	/// of them: where it keeps values of type parameters none of those places name.
	fn returned_mask(
		&self,
		def_id: DefId,
		args: GenericArgs<'tcx>,
		count: usize,
		given: &[(usize, Mask<'tcx>)],
	) -> Mask<'tcx> {
		let Some(g) = self.generic(def_id, args, count) else {
			return ty::List::empty();
		};
		let mut held: FxHashSet<u32> = given
			.iter()
			.flat_map(|&(i, mask)| self.named(g.inputs[i], &g, mask))
			.collect();
		if given
			.iter()
			.any(|&(i, _)| self.shows(g.inputs[i], Some(&g), node, true, pointer))
		{
			held.insert(OPEN);
		}
		let held = related(&g, held);
		let from: Vec<_> = given.iter().map(|&(i, _)| g.inputs[i]).collect();
		let passed = given.iter().map(|&(_, mask)| mask).reduce(|a, b| self.meet(a, b));
		self.split(
			g.output,
			&g,
			|t| node(t) && params(t).all(|p| !held.contains(&p)),
			&from,
			passed.unwrap_or(ty::List::empty()),
		)
	}

	/// The arguments of a call of std's `def_id` with `args`, given `count`, that may reach
	/// the pointers its result, or what the argument `into` points at, holds outside what
	/// `mask` names, each with what of it holds none of them: those that name a type
	/// parameter whose values it keeps there. Every argument, whole, where std's own parts
	/// of it may hold them.
	fn reached_masks(
		&self,
		def_id: DefId,
		args: GenericArgs<'tcx>,
		count: usize,
		mask: Mask<'tcx>,
		into: Option<usize>,
	) -> Vec<Option<Mask<'tcx>>> {
		let all = vec![Some(ty::List::empty()); count];
		let Some(g) = self.generic(def_id, args, count) else {
			return all;
		};
		let made = match into {
			Some(at) => match g.inputs[at].builtin_deref(true) {
				Some(pointee) => pointee,
				None => return all,
			},
			None => g.output,
		};
		if self.shows(made, Some(&g), node, true, pointer) {
			return all;
		}
		let needed = related(&g, self.named(made, &g, mask));
		if needed.contains(&OPEN) {
			return all;
		}
		let unneeded = |t: Ty<'tcx>| node(t) && params(t).all(|p| !needed.contains(&p));
		g.inputs
			.iter()
			.map(|&input| {
				nodes(input)
					.into_iter()
					.any(|t| !unneeded(t))
					.then(|| self.split(input, &g, unneeded, &[made], mask))
			})
			.collect()
	}

	/// What of a value of `ty`, over `g`'s generics, holds none of the pointers: the type,
	/// as `g`'s arguments make it, of each place of type parameters `cut` names, and each of
	/// `passed`, which `from` holds none of them in, that the value holds only in the place
	/// of a type parameter `from` names whole and no bound relates to another. Each where no
	/// other place the value holds or reaches has a value of it.
	fn split(
		&self,
		ty: Ty<'tcx>,
		g: &Generic<'tcx>,
		cut: impl Fn(Ty<'tcx>) -> bool,
		from: &[Ty<'tcx>],
		passed: Mask<'tcx>,
	) -> Mask<'tcx> {
		let kept = |t: Ty<'tcx>| {
			matches!(t.kind(), ty::Param(p) if !g.bounds.iter().any(|b| b.contains(&p.index)))
				&& from.iter().any(|&f| nodes(f).contains(&t))
		};
		let cuts = nodes(ty).into_iter().filter(|&t| cut(t)).map(|t| self.at(t, g));
		let cuts: Vec<_> = cuts
			.filter(|&m| !self.shows(ty, Some(g), &cut, true, |t| self.same(t, m)))
			.collect();
		let passed = passed
			.iter()
			.filter(|&m| !self.shows(ty, Some(g), |t| cut(t) || kept(t), true, |t| self.same(t, m)));
		self.mask(cuts.into_iter().chain(passed))
	}

	/// The type parameters of `g` whose values a value of `ty`, over its generics, holds in
	/// a place whose type, as `g`'s arguments make it, `mask` doesn't name.
	fn named(&self, ty: Ty<'tcx>, g: &Generic<'tcx>, mask: Mask<'tcx>) -> FxHashSet<u32> {
		nodes(ty)
			.into_iter()
			.filter(|&t| !self.masked(self.at(t, g), mask))
			.flat_map(params)
			.collect()
	}

	/// `ty`, over `g`'s generics, as `g`'s arguments make it.
	fn at(&self, ty: Ty<'tcx>, g: &Generic<'tcx>) -> Ty<'tcx> {
		instantiate(self.t.tcx, self.env, ty, g.args)
	}

	/// Whether a value of `ty`, over `generic`'s generics if any, holds, or with `behind`
	/// reaches, a value whose type, as the call's arguments make it, `found` names, outside
	/// the places of type parameters `cut` names; or one its type doesn't tell.
	fn shows(
		&self,
		ty: Ty<'tcx>,
		generic: Option<&Generic<'tcx>>,
		cut: impl Fn(Ty<'tcx>) -> bool,
		behind: bool,
		found: impl Fn(Ty<'tcx>) -> bool,
	) -> bool {
		let tcx = self.t.tcx;
		let mut seen = FxHashSet::default();
		// Each type, and whether it is over the call's generics rather than the body's.
		let mut stack = vec![(ty, generic.is_some())];
		while let Some((ty, over)) = stack.pop() {
			if !seen.insert((ty, over)) {
				continue;
			}
			let (ty, over) = match generic {
				Some(g) if over && (node(ty) || !ty.has_param()) => {
					if node(ty) && cut(ty) {
						continue;
					}
					(self.at(ty, g), false)
				}
				_ => (ty, over),
			};
			if found(match generic {
				Some(g) if over => self.at(ty, g),
				_ => ty,
			}) {
				return true;
			}
			match ty.kind() {
				ty::Ref(_, pointee, _) | ty::RawPtr(pointee, _) => {
					if behind {
						stack.push((*pointee, over));
					}
				}
				ty::Adt(adt, args) if over => {
					stack.extend(
						adt.all_fields()
							.map(|f| (tcx.type_of(f.did).instantiate(tcx, args).skip_norm_wip(), true)),
					);
				}
				_ => {
					if behind
						&& let ty::Adt(adt, args) = ty.kind()
						&& crate::lower::library::std_path(tcx, adt.did()) == ["raw_vec", "RawVec"]
					{
						stack.extend(args.types().map(|t| (t, false)));
					}
					match self.fields(ty) {
						Some(fields) => stack.extend(fields.into_iter().map(|t| (t, over))),
						None => return true,
					}
				}
			}
		}
		false
	}

	/// `ty`, or what it holds if JS keeps that as it: a `MaybeUninit`'s or a
	/// `ManuallyDrop`'s payload.
	fn peel(&self, mut ty: Ty<'tcx>) -> Ty<'tcx> {
		while let ty::Adt(adt, args) = ty.kind()
			&& matches!(
				Std::of(self.t.tcx, adt.did()),
				Some(Std::MaybeUninit | Std::ManuallyDrop)
			) {
			ty = args.type_at(0);
		}
		ty
	}

	/// Rustc's liveness of the body's locals.
	pub fn liveness(&self) -> ResultsCursor<'x, 'tcx, MaybeLiveLocals> {
		MaybeLiveLocals
			.iterate_to_fixpoint(self.t.tcx, self.mir, None)
			.into_results_cursor(self.mir)
	}

	/// Whether code at `to` may run after code at `from`, or is it.
	fn reaches(&self, from: Location, to: Location) -> bool {
		if from.block == to.block && from.statement_index <= to.statement_index {
			return true;
		}
		let blocks = &self.mir.basic_blocks;
		let mut seen = DenseBitSet::new_empty(blocks.len());
		let mut stack: Vec<_> = blocks[from.block].terminator().successors().collect();
		while let Some(block) = stack.pop() {
			if block == to.block {
				return true;
			}
			if seen.insert(block) {
				stack.extend(blocks[block].terminator().successors());
			}
		}
		false
	}

	/// Whether a pointer `local` holds may point into one of the body's own locals, as
	/// [`Self::origins`] finds where it came from, or can't tell.
	pub fn may_point_into_locals(&self, local: Local) -> bool {
		self.origins(local, None)
			.is_none_or(|sources| sources.iter().any(|source| matches!(source, Source::Local(_))))
	}

	/// Where the pointers `local` may hold, to a `filled` value if known, may have come
	/// from, following copies, casts, pointers made through them, and calls of std or the
	/// crate that return where they point, or of std's that run none of the crate's code,
	/// which may return what their arguments reach, or memory they allocate, and change what
	/// their mutable borrows of a local hold; `None` if from anywhere else, such as memory.
	fn origins(&self, local: Local, filled: Option<Ty<'tcx>>) -> Option<FxIndexSet<Source<'tcx>>> {
		let defs = self.defs();
		let defs = defs.as_ref().expect("defs are found");
		let tcx = self.t.tcx;
		let empty = ty::List::empty();
		let mut sources = FxIndexSet::default();
		let mut seen = FxHashSet::default();
		// Each local, whether what it points to may hold the pointers too, and what of it
		// holds none of them.
		let mut stack = vec![(local, false, empty)];
		// The arguments of a call of std's, which may reach the pointers it returns, each
		// outside what its mask names.
		let reached = |args: &[Spanned<Operand<'tcx>>],
		               masks: Vec<Option<Mask<'tcx>>>,
		               stack: &mut Vec<(Local, bool, Mask<'tcx>)>| {
			for (arg, mask) in args.iter().zip(masks) {
				if let Some(mask) = mask
					&& self.carries(arg.node.ty(&self.mir.local_decls, tcx))
				{
					stack.push((direct(arg.node.place()?)?, true, mask));
				}
			}
			Some(())
		};
		while let Some((local, deep, mask)) = stack.pop() {
			let ty = self.mir.local_decls[local].ty;
			if !seen.insert((local, deep, mask)) || deep && !self.shows_pointer(ty) || self.masked(ty, mask) {
				continue;
			}
			if defs[local].other {
				return None;
			}
			if local != mir::RETURN_PLACE && local.as_usize() <= self.mir.arg_count {
				sources.insert(Source::Param(local));
			}
			for &lent in &defs[local].lent {
				for (def_id, gargs, args) in self.lent_to(lent)? {
					// What std's code puts there may be memory it allocated, as `Vec::push` does.
					sources.insert(Source::Fresh(local, mask));
					let at = args
						.iter()
						.position(|a| a.node.place().and_then(|p| p.as_local()) == Some(lent));
					reached(
						args,
						self.reached_masks(def_id, gargs, args.len(), mask, at),
						&mut stack,
					)?;
				}
			}
			for def in &defs[local].whole {
				match *def {
					Def::Assign(
						Rvalue::Use(Operand::Constant(_), _)
						| Rvalue::Cast(_, Operand::Constant(_), _)
						| Rvalue::ThreadLocalRef(_),
					) => _ = sources.insert(Source::Static),
					Def::Assign(Rvalue::Use(op, _)) => {
						let place = op.place()?;
						stack.push((direct(place)?, deep, self.narrowed(place, mask, deep)))
					}
					Def::Assign(Rvalue::Cast(_, op, _)) => stack.push((direct(op.place()?)?, deep, empty)),
					Def::Assign(Rvalue::CopyForDeref(place)) => {
						stack.push((direct(*place)?, deep, self.narrowed(*place, mask, deep)))
					}
					Def::Assign(Rvalue::Ref(_, _, place) | Rvalue::RawPtr(_, place)) => {
						match place.projection.split_first() {
							Some((ProjectionElem::Deref, rest)) if !rest.contains(&ProjectionElem::Deref) => {
								stack.push((place.local, deep, empty))
							}
							_ if !place.is_indirect() => {
								// What a local reaches may point into it only if it can hold one.
								if !deep
									|| filled
										.is_none_or(|f| self.holds_in_place(self.mir.local_decls[place.local].ty, f))
								{
									sources.insert(Source::Local(place.local));
								}
								if deep {
									stack.push((place.local, true, self.narrowed(*place, mask, true)));
								}
							}
							_ => return None,
						}
					}
					Def::Assign(_) => return None,
					Def::Call(func, args) => {
						let ty::FnDef(def_id, gargs) = *func.ty(&self.mir.local_decls, tcx).kind() else {
							return None;
						};
						if pointer::allocates(tcx, def_id) {
							sources.insert(Source::Fresh(local, mask));
						} else if let Some(derives) = pointer::derives(tcx, def_id) {
							if derives == Derives::Loaded {
								return None;
							}
							for arg in args {
								if self.carries(arg.node.ty(&self.mir.local_decls, tcx)) {
									stack.push((direct(arg.node.place()?)?, deep, empty));
								}
							}
						} else if let Some(did) = self.read_fn(def_id, gargs.skip_binder()) {
							for source in self.returned(did)? {
								match source {
									Source::Param(param) => {
										stack.push((direct(args[param.as_usize() - 1].node.place()?)?, deep, empty))
									}
									_ => _ = sources.insert(source),
								}
							}
						} else if self.runs_std(def_id, gargs.skip_binder(), args)
							|| self.repeats(def_id, gargs.skip_binder())
						{
							sources.insert(Source::Fresh(local, mask));
							let masks = self.reached_masks(def_id, gargs.skip_binder(), args.len(), mask, None);
							reached(args, masks, &mut stack)?;
						} else {
							return None;
						}
					}
				}
			}
		}
		Some(sources)
	}

	/// Each call `lent`, a mutable borrow of a local, is given to, with its arguments; `None`
	/// if it is used anywhere else, or given to a call that may run the crate's code or keep
	/// it where code could change the local through it after: in another argument or in what
	/// the call returns.
	#[allow(
		clippy::type_complexity,
		reason = "a call's function, its generic arguments, and its arguments"
	)]
	fn lent_to(&self, lent: Local) -> Option<Vec<(DefId, GenericArgs<'tcx>, &'x [Spanned<Operand<'tcx>>])>> {
		let (mir, tcx) = (self.mir, self.t.tcx);
		let ty = mir.local_decls[lent].ty;
		let borrowed = ty.builtin_deref(true)?;
		let mut calls = Vec::new();
		let mut given = 0;
		for data in mir.basic_blocks.iter() {
			if let TerminatorKind::Call {
				func,
				args,
				destination,
				..
			} = &data.terminator().kind
			{
				let n = args
					.iter()
					.filter(|a| a.node.place().and_then(|p| p.as_local()) == Some(lent))
					.count();
				if n > 0 {
					let ty::FnDef(def_id, gargs) = *func.ty(&mir.local_decls, tcx).kind() else {
						return None;
					};
					let returned = destination.ty(&mir.local_decls, tcx).ty;
					if pointer::derives(tcx, def_id).is_some()
						|| !self.leaves(def_id, gargs.skip_binder(), args, &[ty])
						|| self.points_to(returned, ty::List::empty(), |pointee| {
							self.holds_in_place(pointee, borrowed)
						}) {
						return None;
					}
					given += n;
					calls.push((def_id, gargs.skip_binder(), &args[..]));
				}
			}
		}
		let mut uses = Uses { local: lent, count: 0 };
		uses.visit_body(mir);
		(uses.count == given).then_some(calls)
	}

	/// The body's [`Defs`], found the first time.
	fn defs(&self) -> std::cell::Ref<'_, Option<IndexVec<Local, Defs<'x, 'tcx>>>> {
		if self.defs.borrow().is_none() {
			let mir = self.mir;
			let mut defs: IndexVec<Local, Defs<'x, 'tcx>> =
				IndexVec::from_fn_n(|_| Defs::default(), mir.local_decls.len());
			let assigned = |to: &Place<'tcx>, def: Def<'x, 'tcx>, defs: &mut IndexVec<Local, Defs<'x, 'tcx>>| match to
				.as_local()
			{
				Some(local) => defs[local].whole.push(def),
				None if !to.is_indirect() => defs[to.local].other = true,
				None => {}
			};
			for data in mir.basic_blocks.iter() {
				for stmt in &data.statements {
					match &stmt.kind {
						StatementKind::Assign(assign) => {
							let (to, rvalue) = &**assign;
							assigned(to, Def::Assign(rvalue), &mut defs);
							// A local something borrows may change through the borrow, which
							// `origins` follows where it is a local's mutable borrow of all of it.
							match rvalue {
								Rvalue::Ref(_, BorrowKind::Mut { .. }, place)
								| Rvalue::RawPtr(RawPtrKind::Mut, place)
									if let Some(borrowed) = place.as_local()
										&& let Some(to) = to.as_local() =>
								{
									defs[borrowed].lent.push(to)
								}
								// A shared borrow changes nothing but through an `UnsafeCell`.
								Rvalue::Ref(_, BorrowKind::Shared | BorrowKind::Fake(_), place)
									if !self.mutable_through_shared(place.ty(&mir.local_decls, self.t.tcx).ty) => {}
								Rvalue::Ref(_, _, place) | Rvalue::RawPtr(_, place) if !place.is_indirect() => {
									defs[place.local].other = true
								}
								_ => {}
							}
						}
						StatementKind::SetDiscriminant { place, .. } if !place.is_indirect() => {
							defs[place.local].other = true
						}
						_ => {}
					}
				}
				match &data.terminator().kind {
					TerminatorKind::Call {
						func,
						args,
						destination,
						..
					} => assigned(destination, Def::Call(func, args), &mut defs),
					TerminatorKind::InlineAsm { .. }
					| TerminatorKind::Yield { .. }
					| TerminatorKind::TailCall { .. } => defs.iter_mut().for_each(|d| d.other = true),
					_ => {}
				}
			}
			*self.defs.borrow_mut() = Some(defs);
		}
		self.defs.borrow()
	}
}

/// What a local of `rvalue`, which reads one of the holders, holds pointers to: what a
/// holder it copies or casts does, or a field of it such as an `Option`'s payload; what a
/// reference or pointer made of one or through it does; or an element of that, by index.
fn level(rvalue: &Rvalue<'_>, held: &Held) -> Level {
	match rvalue {
		Rvalue::Use(op, _) | Rvalue::Cast(_, op, _) => match op.place() {
			Some(p) if !p.is_indirect() => held.of(p.local),
			_ => Level::Part,
		},
		Rvalue::CopyForDeref(place) if !place.is_indirect() => held.of(place.local),
		Rvalue::Ref(_, _, place) | Rvalue::RawPtr(_, place) => {
			let (base, rest) = match place.projection.split_first() {
				Some((ProjectionElem::Deref, rest)) => (held.of(place.local), rest),
				_ => (held.at(place.local), place.projection.as_slice()),
			};
			let index = |elem: &ProjectionElem<_, _>| {
				matches!(
					elem,
					ProjectionElem::Index(_) | ProjectionElem::ConstantIndex { .. } | ProjectionElem::Subslice { .. }
				)
			};
			match rest {
				[] => base,
				_ if rest.iter().all(index) => base.max(Level::Elems),
				_ => Level::Part,
			}
		}
		_ => Level::Part,
	}
}

impl<'tcx> Held<'tcx> {
	/// What the pointers `local` holds are in none of, as a [`Mask`].
	fn mask(&self, local: Local) -> Mask<'tcx> {
		self.masks.get(&local).copied().unwrap_or(ty::List::empty())
	}

	/// Whether `rvalue`, which reads a holder, only reads a value through one that points at
	/// no holder, which then holds none of the pointers without `loads`.
	fn loaded(&self, rvalue: &Rvalue<'_>) -> bool {
		let place = match rvalue {
			Rvalue::Use(op, _) | Rvalue::Cast(_, op, _) => op.place(),
			Rvalue::CopyForDeref(place) => Some(*place),
			_ => None,
		};
		!self.loads && place.is_some_and(|p| p.is_indirect() && !self.behind.contains(p.local))
	}

	/// Whether the argument `op` may point at a holder, as [`Self::behind`] says.
	fn behind_arg(&self, op: &Operand<'_>) -> bool {
		!self.loads && op.place().is_some_and(|p| self.behind.contains(p.local))
	}

	/// What the pointers `local` holds point at; any read of the value the pointers are made
	/// of is a part of it.
	fn of(&self, local: Local) -> Level {
		match self.value == Some(local) {
			true => Level::Part,
			false => self.at(local),
		}
	}

	/// What a pointer to `local` itself points at, of the value the pointers are made of:
	/// its whole, if it is that value.
	fn at(&self, local: Local) -> Level {
		if self.parts.contains(local) {
			Level::Part
		} else if self.elems.contains(local) {
			Level::Elems
		} else {
			Level::Whole
		}
	}
}

/// A call of std's code over type parameters, as the body it runs declares it.
struct Generic<'tcx> {
	/// The arguments for the body's generics.
	args: GenericArgs<'tcx>,
	/// Its parameters' types, over its generics.
	inputs: &'tcx [Ty<'tcx>],
	/// Its result's type, over its generics.
	output: Ty<'tcx>,
	/// The type parameters each of its bounds names, with [`OPEN`] for a type it names that
	/// shows a pointer, where it names two: code may make a value of one out of another.
	bounds: Vec<FxHashSet<u32>>,
}

/// What stands, in [`Generic::bounds`], for a pointer of std's own a bound names.
const OPEN: u32 = u32::MAX;

/// Whether `ty`, over a call's generics, is the place of a value of its type parameters,
/// which std's code over them gets only from the places of its arguments that name them,
/// or from what their bounds give it: a type parameter, or a projection of them.
fn node(ty: Ty<'_>) -> bool {
	matches!(ty.kind(), ty::Param(_)) || matches!(ty.kind(), ty::Alias(..)) && ty.has_param()
}

/// The places of type parameters `ty` names, as [`node`] finds them, outermost.
fn nodes(ty: Ty<'_>) -> Vec<Ty<'_>> {
	let mut nodes = Vec::new();
	let mut walk = ty.walk();
	while let Some(arg) = walk.next() {
		if let Some(t) = arg.as_type()
			&& node(t)
		{
			walk.skip_current_subtree();
			nodes.push(t);
		}
	}
	nodes
}

/// The type parameters `ty` names.
fn params(ty: Ty<'_>) -> impl Iterator<Item = u32> {
	ty.walk().filter_map(|arg| match arg.as_type()?.kind() {
		ty::Param(p) => Some(p.index),
		_ => None,
	})
}

/// `named`, with each type parameter a bound of `g` relates to one of them.
fn related(g: &Generic<'_>, mut named: FxHashSet<u32>) -> FxHashSet<u32> {
	loop {
		let before = named.len();
		for bound in &g.bounds {
			if bound.iter().any(|p| named.contains(p)) {
				named.extend(bound);
			}
		}
		if named.len() == before {
			return named;
		}
	}
}

/// Whether `ty` is a pointer itself.
fn pointer(ty: Ty<'_>) -> bool {
	matches!(ty.kind(), ty::Ref(..) | ty::RawPtr(..))
}

/// Whether a bound of `trait_id` lets code call nothing: it and its supertraits have no
/// methods, as `Sized` and `Send`.
fn calls_nothing(tcx: ty::TyCtxt<'_>, trait_id: DefId) -> bool {
	tcx.supertrait_def_ids(trait_id).all(|t| {
		tcx.associated_items(t)
			.in_definition_order()
			.all(|item| !matches!(item.kind, ty::AssocKind::Fn { .. }))
	})
}

/// Whether std's `did` is a `copied` or of the `Copied` iterator, which copy what a `Copy`
/// bound gives them rather than run its `Clone`, unlike `Extend<&T>` for a set, which
/// clones under the same bound.
fn copies(tcx: ty::TyCtxt<'_>, did: DefId) -> bool {
	let path = crate::lower::library::std_path(tcx, did);
	path.last().is_some_and(|name| name == "copied")
		|| path.starts_with(&["iter".into(), "adapters".into(), "copied".into()])
}

/// `ty` as storage, which an array shares with a slice of its element.
fn storage<'tcx>(tcx: ty::TyCtxt<'tcx>, ty: Ty<'tcx>) -> Ty<'tcx> {
	match ty.kind() {
		ty::Array(elem, _) => Ty::new_slice(tcx, *elem),
		_ => ty,
	}
}

/// The local `place` is, read as a whole or through fields of it, with no pointer on its
/// path.
fn direct(place: Place<'_>) -> Option<Local> {
	(!place.is_indirect()).then_some(place.local)
}

/// Whether `op` reads one of `holders`.
fn reads_operand(op: &Operand<'_>, holders: &DenseBitSet<Local>) -> bool {
	op.place().is_some_and(|p| holders.contains(p.local))
}

/// Whether what `visit` visits reads one of `holders`.
fn reads(holders: &DenseBitSet<Local>, visit: impl FnOnce(&mut Reads<'_>)) -> bool {
	let mut reads = Reads { holders, found: false };
	visit(&mut reads);
	reads.found
}

/// How many times what it visits uses `local`, but to assign all of it.
struct Uses {
	local: Local,
	count: usize,
}

impl<'tcx> Visitor<'tcx> for Uses {
	fn visit_local(&mut self, local: Local, context: PlaceContext, _: Location) {
		use rustc_middle::mir::visit::MutatingUseContext;
		if local == self.local
			&& !matches!(
				context,
				PlaceContext::NonUse(_)
					| PlaceContext::MutatingUse(MutatingUseContext::Store | MutatingUseContext::Call)
			) {
			self.count += 1;
		}
	}
}

/// Whether what it visits reads one of the holders.
struct Reads<'a> {
	holders: &'a DenseBitSet<Local>,
	found: bool,
}

impl<'tcx> Visitor<'tcx> for Reads<'_> {
	fn visit_local(&mut self, local: Local, _: PlaceContext, _: Location) {
		self.found |= self.holders.contains(local);
	}
}

/// Whether `did` is one of std's types that hold a raw pointer safe code puts in and gets
/// back, unlike std's others, whose pointers are private.
fn raw_carrier(tcx: ty::TyCtxt<'_>, did: DefId) -> bool {
	matches!(
		crate::lower::library::std_path(tcx, did).as_slice(),
		[.., m, name] if matches!(
			(m.as_str(), name.as_str()),
			("non_null", "NonNull")
				| ("atomic", "AtomicPtr")
				| ("wake", "RawWaker" | "Waker" | "LocalWaker" | "Context")
		)
	)
}
