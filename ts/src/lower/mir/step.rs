//! Iterators stepped in place: a slice's or a `Vec`'s iterator, or a range of numbers,
//! that only `next` reads, as a `for` loop's is, is two variables, of the array and an
//! index or of the bounds, rather than an `rs.Iter`, which a bundler can't shake its
//! adaptors from. Variables, not an object of both: V8 took up to 1.25x as long over a
//! `BTreeMap` stepping an object's fields.
//!
//! A slice's `iter_mut` of values JS copies lends each item as its place. Where its `next`
//! results are read only by a switch on their variant and by moving the item into a
//! reference assigned nothing else, it is three variables, the array, an index, and the
//! end, and each item is its index: the item reads and writes the array there, as
//! `&mut v[i]` does, and one passed on whole is `rs.arrayAsPtr` of its element, as
//! `rs.iterSliceMut` would have lent it, rather than an `rs.Place` made for every item.

use rustc_data_structures::fx::{FxHashMap, FxHashSet};
use rustc_middle::mir::{self, Local, Operand, Rvalue, StatementKind, TerminatorKind};
use rustc_middle::ty::{self, Ty, TyCtxt};

use super::Fx;
use crate::lower::library::{self, Std};
use crate::ts::{Expr, Key, Stmt, StmtKind, Type};

/// The item type of a slice's or a `Vec`'s iterator, if `ty` is one.
fn array_iter<'tcx>(tcx: TyCtxt<'tcx>, ty: Ty<'tcx>) -> Option<Ty<'tcx>> {
	let ty::Adt(adt, args) = ty.kind() else {
		return None;
	};
	let path = library::std_path(tcx, adt.did());
	let path: Vec<&str> = path.iter().map(String::as_str).collect();
	match path.as_slice() {
		// A reference to an array is a slice of it made as the item is read.
		["slice", "iter", "Iter" | "IterMut"] if args.types().next().is_some_and(|t| t.is_array()) => None,
		["slice", "iter", "Iter" | "IterMut"] | ["vec", "into_iter", "IntoIter"] => args.types().next(),
		_ => None,
	}
}

/// Whether `ty` is what such an iterator is made from: a slice, an array, or a `Vec`.
fn array<'tcx>(tcx: TyCtxt<'tcx>, ty: Ty<'tcx>) -> bool {
	match ty.peel_refs().kind() {
		ty::Slice(_) | ty::Array(..) => true,
		ty::Adt(adt, _) => Std::of(tcx, adt.did()) == Some(Std::Vec),
		_ => false,
	}
}

impl<'tcx> Fx<'_, '_, '_, 'tcx> {
	/// Find the iterators to step in place: each made by a call from an array, or moved
	/// from another, and otherwise only borrowed for `next` into a protocol local and
	/// dropped. Each borrow is read only as `next`'s receiver.
	pub(super) fn find_stepped(&mut self) {
		let mir = self.mir;
		let tcx = self.tcx;
		let mut iters: Vec<Local> = mir
			.local_decls
			.indices()
			.filter(|&l| !self.is_arg(l) && l != mir::RETURN_PLACE && self.defs[l] == 1)
			.filter(|&l| {
				let ty = mir.local_decls[l].ty;
				// An element JS copies is lent as its place, which `rs.iterSliceMut` makes; one
				// lent whole as a place steps by its index, if its items allow it, below.
				let places = || self.b.t.iter_item(ty).is_some_and(|item| self.b.t.has_place(item));
				array_iter(tcx, ty).is_some() && (!places() || self.lends_places(l)) || super::small_range(tcx, ty)
			})
			.collect();
		if iters.is_empty() {
			return;
		}
		// What each reads that is allowed: borrows for `next`, drops, moves into another.
		let mut reads: FxHashMap<Local, u32> = FxHashMap::default();
		let mut made = FxHashSet::default();
		let mut moves: Vec<(Local, Local)> = Vec::new();
		let mut borrows: FxHashMap<Local, Local> = FxHashMap::default();
		let mut next_reads: FxHashMap<Local, u32> = FxHashMap::default();
		// Each `next`'s receiver and the local it assigns.
		let mut nexts: Vec<(Local, Local)> = Vec::new();
		for data in mir.basic_blocks.iter() {
			for stmt in &data.statements {
				let StatementKind::Assign(assign) = &stmt.kind else {
					continue;
				};
				let (place, rvalue) = &**assign;
				match rvalue {
					Rvalue::Ref(_, mir::BorrowKind::Mut { .. }, from) if from.as_local().is_some() => {
						let from = from.as_local().unwrap();
						*reads.entry(from).or_default() += 1;
						if let Some(to) = place.as_local() {
							borrows.insert(to, from);
						}
					}
					// A move into a field, such as a coroutine's for a local held across an
					// `.await`, needs the `rs.Iter` the field's type says, so it isn't counted.
					Rvalue::Use(Operand::Move(from), _)
						if let Some(from) = from.as_local()
							&& let Some(to) = place.as_local() =>
					{
						*reads.entry(from).or_default() += 1;
						moves.push((from, to));
					}
					Rvalue::Aggregate(..) if let Some(to) = place.as_local() => {
						made.insert(to);
					}
					_ => {}
				}
			}
			match &data.terminator().kind {
				// Counted as `uses` counts it: only a drop JS sees.
				TerminatorKind::Drop { place, .. }
					if let Some(local) = place.as_local()
						&& self.b.t.drops(mir.local_decls[local].ty) =>
				{
					*reads.entry(local).or_default() += 1;
				}
				TerminatorKind::Call {
					func,
					args,
					destination,
					..
				} => {
					let ty::FnDef(def_id, _) = *func.ty(&mir.local_decls, tcx).kind() else {
						continue;
					};
					let maker = library::key(tcx, def_id).is_some_and(|(o, n)| {
						matches!(
							(o.as_str(), n.as_str()),
							("IntoIterator", "into_iter") | ("slice", "iter" | "iter_mut")
						)
					});
					if maker
						&& let Some(to) = destination.as_local()
						&& let [arg] = &**args
					{
						match &arg.node {
							// A range, or an array's iterator, is its own iterator.
							Operand::Move(from)
								if let Some(from) = from.as_local()
									&& let ty = mir.local_decls[from].ty
									&& (super::small_range(tcx, ty) || array_iter(tcx, ty).is_some()) =>
							{
								*reads.entry(from).or_default() += 1;
								moves.push((from, to));
							}
							op if array(tcx, op.ty(&mir.local_decls, tcx)) => {
								made.insert(to);
							}
							_ => {}
						}
					}
					if crate::lower::iter::is_next(tcx, def_id)
						&& destination.as_local().is_some_and(|l| self.next_results.contains(&l))
						&& let [arg] = &**args
						&& let Operand::Copy(p) | Operand::Move(p) = &arg.node
						&& let Some(recv) = p.as_local()
					{
						*next_reads.entry(recv).or_default() += 1;
						nexts.push((recv, destination.local));
					}
				}
				_ => {}
			}
		}
		// A borrow only `next` reads, assigned once and borrowed by nothing.
		let stepping = |fx: &Self, borrow: Local| {
			fx.defs[borrow] == 1
				&& !fx.borrowed[borrow]
				&& next_reads.get(&borrow).copied().unwrap_or(0) == fx.uses[borrow]
		};
		let mut ok: FxHashSet<Local> = iters.drain(..).collect();
		ok.retain(|&l| reads.get(&l).copied().unwrap_or(0) == self.uses[l]);
		ok.retain(|&l| made.contains(&l) || moves.iter().any(|&(_, to)| to == l));
		ok.retain(|&l| borrows.iter().all(|(&b, &from)| from != l || stepping(self, b)));
		// One lending places steps by index only where each `next` it steps by lends items
		// that index allows.
		let mut lent: Vec<(Local, Local, Vec<Local>)> = Vec::new();
		ok.retain(|&l| {
			if !self.lends_places(l) {
				return true;
			}
			let mut found = Vec::new();
			for (&b, _) in borrows.iter().filter(|&(_, &from)| from == l) {
				for &(_, next) in nexts.iter().filter(|&&(recv, _)| recv == b) {
					match self.lent_items(next) {
						Some(items) => found.push((next, items)),
						None => return false,
					}
				}
			}
			lent.extend(found.into_iter().map(|(next, items)| (l, next, items)));
			true
		});
		// A move joins two iterators that must both step in place, or neither.
		loop {
			let before = ok.len();
			for &(from, to) in &moves {
				if ok.contains(&from) != ok.contains(&to) {
					ok.remove(&from);
					ok.remove(&to);
				}
			}
			if ok.len() == before {
				break;
			}
		}
		for (b, from) in borrows {
			if ok.contains(&from) {
				self.stepped_borrows.insert(b, from);
			}
		}
		for (iter, next, items) in lent {
			if ok.contains(&iter) {
				self.lent_nexts.insert(next);
				self.lent_items.extend(items.into_iter().map(|item| (item, iter)));
			}
		}
		// One moved into another and read nowhere else shares its variables: the other is
		// assigned only by the move, so neither is live while the other is.
		for &(from, to) in &moves {
			if ok.contains(&from) && self.uses[from] == 1 {
				self.stepped_into.insert(from, to);
			}
		}
		self.stepped = ok;
	}

	/// Whether `local` is a slice's iterator lending each item as its place, a `&mut` to a
	/// value JS copies.
	fn lends_places(&self, local: Local) -> bool {
		let ty = self.mir.local_decls[local].ty;
		let ty::Adt(adt, args) = ty.kind() else {
			return false;
		};
		let path = library::std_path(self.tcx, adt.did());
		path.iter().map(String::as_str).eq(["slice", "iter", "IterMut"])
			&& args
				.types()
				.next()
				.is_some_and(|elem| !elem.is_array() && self.b.t.needs_place(elem))
	}

	/// The locals the item `next`, a `next` result, lends is moved into, if that is all that
	/// reads it but a switch on its variant, and each is a reference read as the place it
	/// borrows, which an index into the array reads and writes as well.
	fn lent_items(&self, next: Local) -> Option<Vec<Local>> {
		use rustc_middle::mir::visit::{MutatingUseContext, NonMutatingUseContext, PlaceContext, Visitor};
		struct Reads {
			next: Local,
			/// The statements moving `Some`'s value out whole, and into which local.
			moves: FxHashMap<mir::Location, Local>,
			ok: bool,
		}
		impl<'tcx> Visitor<'tcx> for Reads {
			fn visit_place(&mut self, place: &mir::Place<'tcx>, cx: PlaceContext, at: mir::Location) {
				if place.local != self.next {
					return;
				}
				self.ok &= match place.projection.as_slice() {
					[] => matches!(
						cx,
						PlaceContext::NonUse(_)
							| PlaceContext::NonMutatingUse(NonMutatingUseContext::Inspect)
							| PlaceContext::MutatingUse(MutatingUseContext::Call)
					),
					[mir::ProjectionElem::Downcast(..), mir::ProjectionElem::Field(..)] => self.moves.contains_key(&at),
					_ => false,
				};
			}
		}
		let mir = self.mir;
		let mut moves = FxHashMap::default();
		for (block, data) in mir.basic_blocks.iter_enumerated() {
			for (statement_index, stmt) in data.statements.iter().enumerate() {
				if let StatementKind::Assign(assign) = &stmt.kind
					&& let (to, Rvalue::Use(Operand::Copy(from) | Operand::Move(from), _)) = &**assign
					&& from.local == next
					&& let [mir::ProjectionElem::Downcast(_, v), mir::ProjectionElem::Field(f, _)] =
						from.projection.as_slice()
					&& v.as_u32() == 1
					&& f.as_u32() == 0
					&& let Some(to) = to.as_local()
					&& self.alias(to)
				{
					moves.insert(mir::Location { block, statement_index }, to);
				}
			}
		}
		let mut reads = Reads { next, moves, ok: true };
		reads.visit_body(mir);
		reads.ok.then(|| reads.moves.into_values().collect())
	}

	/// The local whose variables `local` holds its fields in: the last it is moved into.
	pub(super) fn stepped_root(&self, mut local: Local) -> Local {
		while let Some(&to) = self.stepped_into.get(&local) {
			local = to;
		}
		local
	}

	/// The array iterator a `next` receiver `op` borrows, if it steps in place. A range's
	/// `next` steps one in place anyway.
	pub(super) fn stepped_array(&self, op: &Operand<'tcx>) -> bool {
		let (Operand::Copy(p) | Operand::Move(p)) = op else {
			return false;
		};
		p.as_local()
			.and_then(|b| self.stepped_borrows.get(&b))
			.is_some_and(|&l| array_iter(self.tcx, self.mir.local_decls[l].ty).is_some())
	}

	/// The keys of `local`'s fields: its array and index, and its end where it lends each
	/// item as that index, or its bounds.
	fn stepped_keys(&self, local: Local) -> &'static [&'static str] {
		match array_iter(self.tcx, self.mir.local_decls[local].ty) {
			Some(_) if self.lends_places(local) => &["s", "i", "e"],
			Some(_) => &["s", "i"],
			None => &["start", "end"],
		}
	}

	/// Name the variables `local` holds its fields in, unless it shares another's. A read
	/// of an index or a bound counts as one of memory, since `next` changes it in place; an
	/// array and an end change only where the iterator is made, which assigns them.
	pub(super) fn name_stepped(&mut self, local: Local, base: &str) {
		if self.stepped_root(local) != local {
			return;
		}
		let keys = self.stepped_keys(local);
		let vars = keys
			.iter()
			.map(|key| {
				let mut suffix = key.to_string();
				suffix[..1].make_ascii_uppercase();
				let name = self.b.fresh(&format!("{base}{suffix}"));
				if !matches!(*key, "s" | "e") {
					self.shared.insert(name.clone());
				}
				name
			})
			.collect::<Vec<_>>();
		if self.lends_places(local) {
			self.item_arrays.insert(vars[0].clone());
		}
		self.stepped_vars.insert(local, vars);
	}

	/// Give each stepped local that shares its variables those of the local it shares.
	pub(super) fn share_stepped(&mut self) {
		for &local in &self.stepped {
			let root = self.stepped_root(local);
			if root != local {
				let vars = self.stepped_vars[&root].clone();
				self.stepped_vars.insert(local, vars);
			}
		}
	}

	/// `local` read whole, if it is stepped in place: an object of its variables, which
	/// [`Self::set_stepped`] and [`stepped_field`] take apart again.
	pub(super) fn stepped_value(&self, local: Local) -> Option<Expr> {
		let vars = self.stepped_vars.get(&local)?;
		let keys = self.stepped_keys(local);
		let fields = keys.iter().zip(vars);
		Some(Expr::Object(
			fields.map(|(k, v)| (Key::Name((*k).into()), Expr::ident(v))).collect(),
		))
	}

	/// Assign `local`'s variables the fields of `value`, as [`Self::stepped_make`] made it.
	pub(super) fn set_stepped(&mut self, local: Local, value: Expr, out: &mut Vec<Stmt>) {
		let value = match value {
			Expr::Located(_, value) => *value,
			value => value,
		};
		// Anything else is refused already.
		let Expr::Object(fields) = value else {
			return;
		};
		self.assigned[local] = true;
		let vars = self.stepped_vars[&local].clone();
		for ((_, value), var) in fields.into_iter().zip(vars) {
			// A move from a local that shares the variables.
			if matches!(&value, Expr::Ident(name) if *name == var) {
				continue;
			}
			let stmt = StmtKind::Expr(Expr::ident(&var).assign(value)).at(None);
			self.emit(vec![stmt], false, out);
		}
	}

	/// An iterator stepped in place, from the `rs.Iter` its mapping makes: `{ s, i: 0 }`
	/// from `rs.iterSlice(s)`, or `rs.iterSlice(s, drop)` of items it owns, which its drop
	/// drops again, `{ s: v.items, i: 0 }` from `rs.vecIntoIter(v, drop)`, the view of `v`'s
	/// bytes from `rs.byteVecIntoIter(v)`, the array, start, and end of the slice of
	/// `rs.iterSliceMut(slice)`, held in `out` first unless it is a path, and `{ start, end }`
	/// from `new rs.Range(start, end)`. A moved one is already made.
	pub(super) fn stepped_make(&mut self, value: Expr, span: rustc_span::Span, out: &mut Vec<Stmt>) -> Expr {
		let rs = |f: &Expr, name: &str| matches!(peel(f), Expr::Rs(n) if n == name);
		let object = |fields: Vec<(&str, Expr)>| {
			Expr::Object(fields.into_iter().map(|(k, v)| (Key::Name(k.into()), v)).collect())
		};
		match value {
			Expr::Call(f, mut args) if rs(&f, "iterSlice") && matches!(args.len(), 1 | 2) => {
				args.truncate(1);
				object(vec![("s", args.pop().unwrap()), ("i", Expr::lit("0"))])
			}
			Expr::Call(f, mut args) if rs(&f, "vecIntoIter") && matches!(args.len(), 1 | 2) => {
				args.truncate(1);
				object(vec![("s", args.pop().unwrap().member("items")), ("i", Expr::lit("0"))])
			}
			Expr::Call(f, args) if rs(&f, "byteVecIntoIter") && args.len() == 1 => {
				object(vec![("s", Expr::rs("byteVecView", args)), ("i", Expr::lit("0"))])
			}
			Expr::Call(f, mut args) if rs(&f, "iterSliceMut") && args.len() == 1 => {
				// All of an array, or of a `Vec<u8>`'s bytes, held back or not, is read where
				// they are: the array or `ByteVec`, and whether it is a `ByteVec`. Any other
				// slice is held in a variable of its own.
				let whole = |e: &Expr| match peel(e) {
					Expr::Call(f, a) if a.len() == 1 && a[0].is_path() && rs(f, "sliceFrom") => {
						Some((a[0].clone(), false))
					}
					Expr::Call(f, a) if a.len() == 1 && a[0].is_path() && rs(f, "byteVecSlice") => {
						Some((a[0].clone(), true))
					}
					_ => None,
				};
				let mut slice = args.pop().unwrap();
				if let Expr::Ident(name) = peel(&slice)
					&& self
						.pending
						.iter()
						.any(|p| p.name == *name && whole(&p.value).is_some())
				{
					slice = self.fold_exprs(Vec::new(), vec![slice], false, out).remove(0);
				}
				let (s, i, e) = match whole(&slice) {
					Some((v, bytes)) => {
						let v = self.path(v, "items", out);
						let s = if bytes { v.clone().member("data") } else { v.clone() };
						(s, Expr::lit("0"), v.member("length"))
					}
					None => {
						let slice = self.path(slice, "slice", out);
						let start = slice.clone().member("start");
						let end = start.clone().binary("+", slice.clone().member("length"));
						(slice.member("data"), start, end)
					}
				};
				object(vec![("s", s), ("i", i), ("e", e)])
			}
			Expr::New(f, args) if rs(&f, "Range") && args.len() == 2 => {
				let [start, end] = <[Expr; 2]>::try_from(args).expect("checked above");
				object(vec![("start", start), ("end", end)])
			}
			Expr::Object(_) => value,
			// Refused already.
			Expr::Lit(lit) if lit == "undefined" => Expr::undefined(),
			_ => self.err(span, "an iterator stepped in place that its mapping didn't make"),
		}
	}

	/// The next item of `it`, stepped in place, or `rs.DONE`: the index of the next item of
	/// one that lends each as its index.
	pub(super) fn stepped_next(&self, it: &Expr) -> Expr {
		let s = stepped_field(it, "s");
		let i = stepped_field(it, "i");
		let (end, item) = match stepped_field_of(it, "e") {
			Some(e) => (e, Expr::Postfix(Box::new(i.clone()), "++")),
			None => (
				s.clone().member("length"),
				s.index(Expr::Postfix(Box::new(i.clone()), "++")),
			),
		};
		Expr::Cond(
			Box::new(i.binary("<", end)),
			Box::new(item),
			Box::new(Expr::Rs("DONE".into())),
		)
	}

	/// What drops a `Vec`'s iterator stepped in place, of type `ty`, as `it`: the items from
	/// its index on.
	pub(super) fn stepped_drop(&mut self, it: &Expr, ty: Ty<'tcx>, span: rustc_span::Span) -> Option<Expr> {
		let item = array_iter(self.tcx, ty)?;
		let dropper = self.b.t.drop_fn(item, span)?;
		Some(Expr::rs(
			"dropAll",
			vec![stepped_field(it, "s"), dropper, stepped_field(it, "i")],
		))
	}

	/// The types of a stepped local's variables: its array, index, and any end, or its
	/// bounds.
	pub(super) fn stepped_tys(&mut self, local: Local) -> Vec<Type> {
		let ty = self.mir.local_decls[local].ty;
		let span = self.local_span(local);
		match array_iter(self.tcx, ty) {
			// What `library::elements` makes of the slice, a view of bytes or an array, or
			// the array of the slice itself.
			Some(item) => {
				let array = match item.kind() {
					_ if crate::lower::byte(item) => Type::Builtin("Uint8Array"),
					_ => Type::Array(Box::new(self.b.t.held_ty(item, span))),
				};
				let n = self.stepped_keys(local).len();
				std::iter::once(array)
					.chain(std::iter::repeat_n(Type::Builtin("number"), n - 1))
					.collect()
			}
			None => {
				let ty::Adt(_, args) = ty.kind() else {
					unreachable!("a range is a struct")
				};
				let bound = self.b.t.held_ty(args.type_at(0), span);
				vec![bound.clone(), bound]
			}
		}
	}
}

/// The field `key` of an iterator stepped in place: its variable, from the object
/// [`Fx::stepped_value`] reads it as.
pub(super) fn stepped_field(it: &Expr, key: &str) -> Expr {
	match it {
		Expr::Object(_) => stepped_field_of(it, key).expect("a stepped iterator's field"),
		it => it.clone().member(key),
	}
}

/// The field `key` of `it`, an iterator stepped in place, if it has one.
fn stepped_field_of(it: &Expr, key: &str) -> Option<Expr> {
	let Expr::Object(fields) = it else {
		return None;
	};
	fields
		.iter()
		.find(|(k, _)| matches!(k, Key::Name(n) if n == key))
		.map(|(_, v)| v.clone())
}

/// Each loop over an array iterator stepped in place that only the loop steps, as
/// `for (const item of array)`: a `while (true)` whose first statements take the next
/// item and leave on `rs.DONE`. Nothing else names the index, so nothing sees where it
/// stopped, but for the `rs.dropAll` of a `Vec`'s items it owns right after the loop,
/// which drops nothing when the loop can only leave at the end.
pub(super) fn for_of(stmts: &mut Vec<Stmt>) {
	let mut names: FxHashMap<String, usize> = FxHashMap::default();
	for stmt in stmts.iter() {
		super::visit_stmt(stmt, &mut |e| {
			super::visit_expr(e, &mut |e| {
				if let Expr::Ident(name) = e {
					*names.entry(name.clone()).or_default() += 1;
				}
			})
		});
	}
	// The variables declared `= 0`, as an index starts.
	let mut zeros = FxHashSet::default();
	decls(stmts, &mut |name, init| {
		if matches!(init, Expr::Lit(l) if l == "0") {
			zeros.insert(name.to_string());
		}
	});
	let mut indexes = FxHashSet::default();
	loops(stmts, &names, &zeros, &mut indexes);
	if !indexes.is_empty() {
		drop_decls(stmts, &indexes);
	}
}

/// Call `f` on the name and value of each declaration in `stmts` that has one.
fn decls(stmts: &[Stmt], f: &mut dyn FnMut(&str, &Expr)) {
	for stmt in stmts {
		match &stmt.kind {
			StmtKind::Decl {
				name, init: Some(init), ..
			} => f(name, init),
			StmtKind::If { then, els, .. } => {
				decls(then, f);
				decls(els, f);
			}
			StmtKind::Block { body, .. } | StmtKind::While { body, .. } | StmtKind::ForOf { body, .. } => {
				decls(body, f)
			}
			StmtKind::Switch { cases, default, .. } => {
				cases.iter().for_each(|(_, body)| decls(body, f));
				decls(default, f);
			}
			_ => {}
		}
	}
}

/// Rewrite the loops in `stmts`, collecting the indexes they no longer read.
fn loops(
	stmts: &mut Vec<Stmt>,
	names: &FxHashMap<String, usize>,
	zeros: &FxHashSet<String>,
	indexes: &mut FxHashSet<String>,
) {
	let mut at = 0;
	while at < stmts.len() {
		let rest = stmts.get(at + 1).and_then(drop_rest);
		let stmt = &mut stmts[at];
		// The array and the index are declared, and named only by this loop's header, and
		// by a drop of what it leaves if it can only leave at the end.
		if let StmtKind::While { label, cond, body } = &mut stmt.kind
			&& matches!(cond, Expr::Lit(l) if l == "true")
			&& let Some((item, array, index)) = header(body, label.as_deref())
			&& let owned = (rest.as_ref().is_some_and(|(a, i)| *a == array && *i == index)
				&& !leaves(&body[2..], label.as_deref(), &mut Vec::new(), false))
			&& let uses = if owned { 3 } else { 2 }
			&& names.get(&array) == Some(&uses)
			&& names.get(&index) == Some(&uses)
			&& zeros.contains(&index)
		{
			let mut body: Vec<Stmt> = std::mem::take(body).into_iter().skip(2).collect();
			let mut name = item;
			// `const entry = item;` names the item, when nothing else reads it.
			if let Some(StmtKind::Decl {
				mutable: false,
				name: to,
				init: Some(Expr::Ident(from)),
				..
			}) = body.first().map(|s| &s.kind)
				&& *from == name
				&& super::list_names(&body, &name) == 1
			{
				name = to.clone();
				body.remove(0);
			}
			indexes.insert(index);
			stmt.kind = StmtKind::ForOf {
				label: label.take(),
				name,
				iter: Expr::ident(array),
				body,
			};
			if owned {
				stmts.remove(at + 1);
			}
		}
		let stmt = &mut stmts[at];
		// Any other loop over an iterator stepped in place tests for an item as its own
		// condition, `while (i < s.length) { const item = s[i++]; ... }`, which V8 and
		// JavaScriptCore step as a counted loop, where a test of the item against
		// `rs.DONE` costs several times as much.
		if let StmtKind::While { label, cond, body } = &mut stmt.kind
			&& matches!(cond, Expr::Lit(l) if l == "true")
			&& let Some((test, item)) = stepped_header(body, label.as_deref())
		{
			*cond = test;
			body.remove(1);
			if let StmtKind::Decl { ty, init, .. } = &mut body[0].kind {
				*init = Some(item);
				*ty = ty.take().map(|ty| match ty {
					Type::Union(mut tys) => {
						tys.retain(|t| !matches!(t, Type::Rs(name, _) if *name == "Done"));
						match tys.len() {
							1 => tys.pop().unwrap(),
							_ => Type::Union(tys),
						}
					}
					ty => ty,
				});
			}
		}
		let stmt = &mut stmts[at];
		at += 1;
		match &mut stmt.kind {
			StmtKind::If { then, els, .. } => {
				loops(then, names, zeros, indexes);
				loops(els, names, zeros, indexes);
			}
			StmtKind::Block { body, .. } | StmtKind::While { body, .. } | StmtKind::ForOf { body, .. } => {
				loops(body, names, zeros, indexes)
			}
			StmtKind::Switch { cases, default, .. } => {
				cases
					.iter_mut()
					.for_each(|(_, body)| loops(body, names, zeros, indexes));
				loops(default, names, zeros, indexes);
			}
			_ => {}
		}
	}
}

/// The array and index of `rs.dropAll(array, f, index)`, which drops what a loop over a
/// `Vec` it owns left.
fn drop_rest(stmt: &Stmt) -> Option<(String, String)> {
	let StmtKind::Expr(call) = &stmt.kind else {
		return None;
	};
	let Expr::Call(f, args) = peel(call) else {
		return None;
	};
	match (peel(f), args.as_slice()) {
		(Expr::Rs(name), [array, _, index]) if name == "dropAll" => match (peel(array), peel(index)) {
			(Expr::Ident(array), Expr::Ident(index)) => Some((array.clone(), index.clone())),
			_ => None,
		},
		_ => None,
	}
}

/// Whether `stmts`, in the loop labeled `label`, leave it other than at its end: a
/// `return`, or a `break` or `continue` to a statement around it. `inner` holds the labels
/// declared in between, and `nested` whether a loop or `switch` is, which an unlabeled
/// `break` leaves instead. A panic aborts, so it leaves nothing to drop.
fn leaves(stmts: &[Stmt], label: Option<&str>, inner: &mut Vec<String>, nested: bool) -> bool {
	stmts.iter().any(|stmt| match &stmt.kind {
		StmtKind::Return(_) => true,
		StmtKind::Break(None) => !nested,
		StmtKind::Break(Some(to)) => !inner.contains(to),
		StmtKind::Continue(Some(to)) => Some(to.as_str()) != label && !inner.contains(to),
		StmtKind::Continue(None) => false,
		StmtKind::If { then, els, .. } => leaves(then, label, inner, nested) || leaves(els, label, inner, nested),
		StmtKind::Block { label: own, body } => within(own, inner, |inner| leaves(body, label, inner, nested)),
		StmtKind::While { label: own, body, .. } | StmtKind::ForOf { label: own, body, .. } => {
			within(own, inner, |inner| leaves(body, label, inner, true))
		}
		StmtKind::Switch {
			label: own,
			cases,
			default,
			..
		} => within(own, inner, |inner| {
			cases.iter().any(|(_, body)| leaves(body, label, inner, true)) || leaves(default, label, inner, true)
		}),
		StmtKind::Expr(_) | StmtKind::Decl { .. } | StmtKind::Declare { .. } => false,
	})
}

/// `f` with `own`, a label, among the `inner` ones.
fn within(own: &Option<String>, inner: &mut Vec<String>, f: impl FnOnce(&mut Vec<String>) -> bool) -> bool {
	inner.extend(own.clone());
	let leaves = f(inner);
	if own.is_some() {
		inner.pop();
	}
	leaves
}

/// The item, array, and index of a loop that starts `const item = i < s.length ?
/// s[i++] : rs.DONE; if (item === rs.DONE) break;`, the `break` leaving it.
fn header(body: &[Stmt], label: Option<&str>) -> Option<(String, String, String)> {
	let [next, test, ..] = body else {
		return None;
	};
	let StmtKind::Decl {
		mutable: false,
		name: item,
		init: Some(init),
		..
	} = &next.kind
	else {
		return None;
	};
	let Expr::Cond(c, a, b) = peel(init) else {
		return None;
	};
	let (Expr::Binary(i, "<", len), Expr::Index(s, at), Expr::Rs(done)) = (peel(c), peel(a), peel(b)) else {
		return None;
	};
	let (Expr::Ident(index), Expr::Member(s2, Key::Name(length)), Expr::Ident(array), Expr::Postfix(i2, "++")) =
		(peel(i), peel(len), peel(s), peel(at))
	else {
		return None;
	};
	let same = |e: &Expr, name: &str| matches!(peel(e), Expr::Ident(n) if n == name);
	if done != "DONE" || length != "length" || !same(s2, array) || !same(i2, index) {
		return None;
	}
	let StmtKind::If { cond, then, els } = &test.kind else {
		return None;
	};
	let leaves = matches!(then.as_slice(), [Stmt { kind: StmtKind::Break(target), .. }]
		if target.is_none() || target.as_deref() == label);
	let tests = matches!(peel(cond), Expr::Binary(l, "===", r)
		if same(l, item) && matches!(peel(r), Expr::Rs(d) if d == "DONE"));
	(leaves && tests && els.is_empty()).then(|| (item.clone(), array.clone(), index.clone()))
}

/// The test and the item of a loop that starts `const item = test ? next : rs.DONE; if
/// (item === rs.DONE) break;`, the `break` leaving it, where `next` steps an iterator in
/// place: `s[i++]`, or `i++` of one lending each item as its index, neither ever `rs.DONE`.
fn stepped_header(body: &[Stmt], label: Option<&str>) -> Option<(Expr, Expr)> {
	let [next, test, ..] = body else {
		return None;
	};
	let StmtKind::Decl {
		mutable: false,
		name: item,
		init: Some(init),
		..
	} = &next.kind
	else {
		return None;
	};
	let Expr::Cond(c, a, b) = peel(init) else {
		return None;
	};
	let steps = |e: &Expr| matches!(peel(e), Expr::Postfix(i, "++") if matches!(peel(i), Expr::Ident(_)));
	let stepped = match peel(a) {
		Expr::Index(s, i) => matches!(peel(s), Expr::Ident(_)) && steps(i),
		e => steps(e),
	};
	if !stepped || !matches!(peel(b), Expr::Rs(done) if done == "DONE") {
		return None;
	}
	let StmtKind::If { cond, then, els } = &test.kind else {
		return None;
	};
	let leaves = matches!(then.as_slice(), [Stmt { kind: StmtKind::Break(target), .. }]
		if target.is_none() || target.as_deref() == label);
	let tests = matches!(peel(cond), Expr::Binary(l, "===", r)
		if matches!(peel(l), Expr::Ident(n) if n == item) && matches!(peel(r), Expr::Rs(d) if d == "DONE"));
	(leaves && tests && els.is_empty()).then(|| ((**c).clone(), (**a).clone()))
}

/// `e` without its source location.
pub(super) fn peel(e: &Expr) -> &Expr {
	match e {
		Expr::Located(_, e) => peel(e),
		e => e,
	}
}

/// Drop the declarations of `names`, which nothing reads any more.
fn drop_decls(stmts: &mut Vec<Stmt>, names: &FxHashSet<String>) {
	stmts.retain(|s| !matches!(&s.kind, StmtKind::Decl { name, .. } if names.contains(name)));
	for stmt in stmts.iter_mut() {
		match &mut stmt.kind {
			StmtKind::If { then, els, .. } => {
				drop_decls(then, names);
				drop_decls(els, names);
			}
			StmtKind::Block { body, .. } | StmtKind::While { body, .. } | StmtKind::ForOf { body, .. } => {
				drop_decls(body, names)
			}
			StmtKind::Switch { cases, default, .. } => {
				cases.iter_mut().for_each(|(_, body)| drop_decls(body, names));
				drop_decls(default, names);
			}
			_ => {}
		}
	}
}
