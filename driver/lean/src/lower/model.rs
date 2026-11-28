//! Models of `std` calls that take closures or make iterators, and of the `Option`, slice,
//! integer, `Ord`, `Arc`, `BTreeMap`/`BTreeSet`, `VecDeque`, and `Duration` functions beside
//! them, and of the calls that make a lens.
//!
//! An iterator is the list of its items ([`Cx::iterator`]): pure, or each item the action
//! that makes it when a closure that may panic maps it, so a consumer meets a panic only at
//! the item Rust's laziness meets it at.

use super::*;

/// What a call's model reads of it.
pub(super) struct Call<'a, 'tcx> {
	pub callee: DefId,
	pub gargs: GenericArgsRef<'tcx>,
	pub args: &'a [rustc_span::Spanned<Operand<'tcx>>],
	pub destination: Place<'tcx>,
	pub span: Span,
	/// The block the call ends.
	pub block: BasicBlock,
}

/// A closure as a Lean function: its definition applied to what it captures, and whether
/// it runs in `Rust.M`.
struct Lambda {
	f: Ex,
	monadic: bool,
}

impl Lambda {
	/// The function as one into `Rust.M`.
	fn action(&self) -> String {
		match self.monadic {
			true => self.f.paren(),
			false => format!("(pure ∘ {})", self.f.paren()),
		}
	}
}

/// An iterator's list, and whether its items are lazy.
struct List {
	ex: Ex,
	deferred: bool,
}

impl List {
	/// The list as lazy items, lifting made ones.
	fn lazy(&self) -> String {
		match self.deferred {
			true => self.ex.paren(),
			false => format!("(Rust.lazy {})", self.ex.paren()),
		}
	}

	/// Every item made, in order, as a consumer that reads each makes them.
	fn made(self) -> Ex {
		match self.deferred {
			true => bound(compound(format!("Rust.run {}", self.ex.paren()), &[&self.ex])),
			false => self.ex,
		}
	}
}

/// The element type of a `std` collection type.
fn element(ty: Ty<'_>) -> Ty<'_> {
	match ty.kind() {
		ty::Adt(_, args) => args.type_at(0),
		_ => ty,
	}
}

impl<'tcx> Fx<'_, 'tcx> {
	/// Lower `call` if a model here covers it: `false` if none does.
	pub(super) fn model(&mut self, key: &str, call: &Call<'_, 'tcx>, ops: &mut Vec<Op>) -> R<bool> {
		let tcx = self.cx.tcx;
		let span = call.span;
		// A std trait's method by the trait, whichever impl runs it.
		let trait_key = tcx
			.trait_of_assoc(call.callee)
			.filter(|t| self.cx.is_std(*t))
			.map(|t| format!("{}::{}", tcx.item_name(t), tcx.item_name(call.callee)));
		let key = match trait_key.as_deref() {
			Some(t)
				if ["Iterator::", "IntoIterator::", "Ord::", "PartialOrd::"]
					.iter()
					.any(|p| t.starts_with(p)) =>
			{
				t
			}
			_ => key,
		};
		// `&a + b` on integers, which std forwards to `a + b`, overflow check and all.
		if let Some(op @ ("Add::add" | "Sub::sub" | "Mul::mul")) = trait_key.as_deref()
			&& let [a, b] = call.args
			&& let (ta, tb) = (a.node.ty(self.body, tcx), b.node.ty(self.body, tcx))
			&& ta.peel_refs().is_integral()
			&& ta.peel_refs() == tb.peel_refs()
			&& (ta.is_ref() || tb.is_ref())
		{
			// std checks it as the calling crate's build does, which is the package's alone.
			if !self.did.is_local() {
				return refuse(
					span,
					"`&a + b` in a dependency, whose overflow checks the translation cannot see, is not supported",
				);
			}
			let x = self.operand(&a.node, span)?;
			let y = self.operand(&b.node, span)?;
			let (x, y) = strip_lits(x, y);
			let sym = match op {
				"Add::add" => "+",
				"Sub::sub" => "-",
				_ => "*",
			};
			let value = if tcx.sess.overflow_checks() {
				bound(compound(format!("{} {sym}? {}", x.paren(), y.paren()), &[&x, &y]))
			} else {
				compound(format!("{} {sym} {}", x.paren(), y.paren()), &[&x, &y])
			};
			self.assign(call.destination, value, span, ops)?;
			return Ok(true);
		}
		let value = match key {
			"Iterator::map" | "Iterator::filter" | "Iterator::filter_map" => {
				let it = self.list(call, 0)?;
				let f = self.closure(&call.args[1].node, span)?;
				if f.monadic {
					self.no_runs(call)?;
				}
				let deferred = it.deferred || f.monadic;
				self.same_list(call, deferred)?;
				let (eager, lazy) = match key {
					"Iterator::map" => ("map", "lazyMap"),
					"Iterator::filter" => ("filter", "lazyFilter"),
					_ => ("filterMap", "lazyFilterMap"),
				};
				let text = match deferred {
					false => format!("{}.{eager} {}", it.ex.paren(), f.f.paren()),
					true => format!("Rust.{lazy} {} {}", f.action(), it.lazy()),
				};
				compound(text, &[&it.ex, &f.f])
			}
			"Iterator::flat_map" | "Iterator::flatten" => {
				let it = self.list(call, 0)?;
				let (f, made) = match key {
					"Iterator::flat_map" => (Some(self.closure(&call.args[1].node, span)?), call.gargs.type_at(1)),
					_ => {
						let source = call.args[0].node.ty(self.body, tcx);
						let Some((item, _)) = self.cx.iterator(source) else {
							return refuse(span, format!("`{source}` has no Lean model"));
						};
						(None, item)
					}
				};
				let Some((items, inner)) = self.cx.iterable(made) else {
					return refuse(span, format!("iterating `{made}` is not supported"));
				};
				let monadic = f.as_ref().is_some_and(|f| f.monadic);
				// An item's lazy items would be made in the action that makes the item.
				if inner && (it.deferred || monadic) {
					return refuse(
						span,
						"flattening iterators whose items may panic, from items or a closure that may panic, is not supported",
					);
				}
				if monadic {
					self.no_runs(call)?;
				}
				self.same_list(call, it.deferred || monadic || inner)?;
				let into = match items {
					Items::Same => None,
					Items::Range(into) | Items::Collection(_, into) => Some(into),
				};
				let text = match (&f, into) {
					_ if it.deferred || monadic => {
						let f = f.as_ref().map_or("pure".into(), Lambda::action);
						format!("Rust.lazyFlatMap {f} {} {}", into.unwrap_or("id"), it.lazy())
					}
					(None, None) => format!("{}.flatten", it.ex.paren()),
					(None, Some(into)) => format!("{}.flatMap {into}", it.ex.paren()),
					(Some(f), None) => format!("{}.flatMap {}", it.ex.paren(), f.f.paren()),
					(Some(f), Some(into)) => format!("{}.flatMap ({into} ∘ {})", it.ex.paren(), f.f.paren()),
				};
				match &f {
					Some(f) => compound(text, &[&it.ex, &f.f]),
					None => compound(text, &[&it.ex]),
				}
			}
			"Iterator::peekable" => {
				let it = self.list(call, 0)?;
				self.same_list(call, it.deferred)?;
				it.ex
			}
			"Peekable::peek" => {
				let place = self.mutable_arg(&call.args[0].node, span)?;
				let ty = place.ty(self.body, tcx).ty;
				let Some((_, deferred)) = self.cx.iterator(ty) else {
					return refuse(span, format!("`{ty}` has no Lean model"));
				};
				self.whole(ty, span)?;
				let f = if deferred { "lazyPeek" } else { "listPeek" };
				self.mutate(f, deferred, call, ops)?;
				return Ok(true);
			}
			"Iterator::any" | "Iterator::all" | "Iterator::find" | "Iterator::find_map" => {
				let it = self.list_mut(call)?;
				let f = self.closure(&call.args[1].node, span)?;
				let (eager, monadic, lazy) = match key {
					"Iterator::any" => ("any", "List.anyM", "Rust.lazyAny"),
					"Iterator::all" => ("all", "List.allM", "Rust.lazyAll"),
					"Iterator::find" => ("find?", "List.findM?", "Rust.lazyFind"),
					_ => ("findSome?", "List.findSomeM?", "Rust.lazyFindMap"),
				};
				match (it.deferred, f.monadic) {
					(false, false) => compound(format!("{}.{eager} {}", it.ex.paren(), f.f.paren()), &[&it.ex, &f.f]),
					(false, true) => bound(compound(
						format!("{monadic} {} {}", f.f.paren(), it.ex.paren()),
						&[&it.ex, &f.f],
					)),
					(true, _) => bound(compound(
						format!("{lazy} {} {}", f.action(), it.ex.paren()),
						&[&it.ex, &f.f],
					)),
				}
			}
			"Iterator::next" => {
				let place = self.mutable_arg(&call.args[0].node, span)?;
				let ty = place.ty(self.body, tcx).ty;
				if self.cx.iterator(ty).is_some() {
					self.whole(ty, span)?;
				}
				let (f, monadic) = match self.cx.iterator(ty) {
					Some((_, false)) => ("listNext", false),
					Some((_, true)) => ("listNextM", true),
					// An integer range; a `char` range's stepping skips the surrogates, which
					// `rangeNext` does not model.
					None if matches!(ty.kind(), ty::Adt(d, a) if self.cx.ops_range(d.did()) == Some("Range")
						&& a.type_at(0).is_integral()) =>
					{
						("rangeNext", false)
					}
					None => return refuse(span, format!("`{ty}` has no Lean model")),
				};
				self.mutate(f, monadic, call, ops)?;
				return Ok(true);
			}
			// The blanket impl for an iterator, a range, or a translated iterator, which is itself;
			// a collection's, which is the list of its items.
			"IntoIterator::into_iter" => {
				let op = &call.args[0].node;
				let from = op.ty(self.body, tcx);
				let into = call.destination.ty(self.body, tcx).ty;
				if tcx.erase_and_anonymize_regions(from) == tcx.erase_and_anonymize_regions(into) {
					self.operand(op, span)?
				} else {
					let it = self.iterated(op, span)?;
					self.same_list(call, it.deferred)?;
					it.ex
				}
			}
			"Iterator::rev" => {
				let it = self.list(call, 0)?;
				self.same_list(call, it.deferred)?;
				let text = match it.deferred {
					false => format!("{}.reverse", it.ex.paren()),
					true => format!("Rust.lazyRev {}", it.ex.paren()),
				};
				compound(text, &[&it.ex])
			}
			"Iterator::take_while" => {
				self.whole(call.args[0].node.ty(self.body, tcx), span)?;
				let it = self.list(call, 0)?;
				let f = self.closure(&call.args[1].node, span)?;
				// Which items it keeps is decided as a consumer reaches them, which a list of items
				// made ahead cannot wait for when deciding may panic.
				if it.deferred || f.monadic {
					return refuse(
						span,
						"`take_while` where an item or the closure may panic is not supported",
					);
				}
				self.same_list(call, false)?;
				compound(format!("{}.takeWhile {}", it.ex.paren(), f.f.paren()), &[&it.ex, &f.f])
			}
			"Iterator::count" => match self.drain(call)? {
				Some((step, it)) => bound(compound(
					format!(
						"Rust.drainFold {step} (fun n _ => pure (n + 1)) (0 : UInt64) {}",
						it.paren()
					),
					&[&it],
				)),
				None => {
					let it = self.consumed(call, false)?;
					// Counting makes every item, as Rust's `fold` does.
					let items = it.made();
					compound(format!("{}.length.toUInt64", items.paren()), &[&items])
				}
			},
			"str::len" | "str::is_empty" | "str::as_bytes" => {
				let s = self.operand(&call.args[0].node, span)?;
				let text = match key {
					"str::len" => format!("Rust.strLen {}", s.paren()),
					"str::is_empty" => format!("{}.isEmpty", s.paren()),
					_ => format!("Rust.asBytes {}", s.paren()),
				};
				compound(text, &[&s])
			}
			"str::find" | "str::starts_with" => {
				let pat = call.gargs.type_at(0);
				let s = self.operand(&call.args[0].node, span)?;
				let p = self.operand(&call.args[1].node, span)?;
				let text = match (key, pat.kind()) {
					("str::find", ty::Char) => format!("Rust.strFind {} {}", s.paren(), p.paren()),
					("str::starts_with", ty::Char) => format!("{}.toList.head? == some {}", s.paren(), p.paren()),
					("str::starts_with", ty::Ref(_, t, _)) if t.is_str() => {
						format!("{}.isPrefixOf {}", p.paren(), s.paren())
					}
					_ => return refuse(span, format!("`{key}` of a `{pat}` pattern has no Lean model")),
				};
				compound(text, &[&s, &p])
			}
			"Index::index for str" => {
				let s = self.operand(&call.args[0].node, span)?;
				let range = call.gargs.type_at(1);
				let r = self.operand(&call.args[1].node, span)?;
				let ty::Adt(def, _) = range.kind() else {
					return refuse(span, format!("indexing a `str` by `{range}` is not supported"));
				};
				let text = match self.cx.ops_range(def.did()).unwrap_or_default() {
					"RangeTo" => format!("Rust.strSliceTo {} {}", s.paren(), r.paren()),
					"RangeFrom" => format!("Rust.strSliceFrom {} {}", s.paren(), r.paren()),
					"Range" => format!("Rust.strSlice {} {}.1 {}.2", s.paren(), r.paren(), r.paren()),
					"RangeFull" if !r.monadic => {
						self.assign(call.destination, s, span, ops)?;
						return Ok(true);
					}
					_ => return refuse(span, format!("indexing a `str` by `{range}` is not supported")),
				};
				bound(compound(text, &[&s, &r]))
			}
			"slice::get" => {
				let index = call.gargs.type_at(1);
				if !matches!(index.kind(), ty::Uint(ty::UintTy::Usize)) {
					return refuse(span, format!("`slice::get` by `{index}` is not supported"));
				}
				let s = self.operand(&call.args[0].node, span)?;
				let i = self.operand(&call.args[1].node, span)?;
				compound(format!("{}[{}.toNat]?", s.paren(), i.paren()), &[&s, &i])
			}
			"Iterator::copied" | "Iterator::cloned" => {
				let item = self.cx.iterator(call.gargs.type_at(0)).map(|(t, _)| t);
				if key.ends_with("cloned") {
					let t = item.map(|t| t.peel_refs());
					if !t.is_some_and(|t| self.clones(t)) {
						return refuse(span, "cloning these items may not copy them");
					}
				}
				let it = self.list(call, 0)?;
				self.same_list(call, it.deferred)?;
				it.ex
			}
			"Iterator::chain" => {
				let a = self.list(call, 0)?;
				let b = self.iterated(&call.args[1].node, span)?;
				let deferred = a.deferred || b.deferred;
				self.same_list(call, deferred)?;
				let lift = |l: &List| if deferred { l.lazy() } else { l.ex.paren() };
				compound(format!("{} ++ {}", lift(&a), lift(&b)), &[&a.ex, &b.ex])
			}
			"Iterator::enumerate" => {
				let it = self.list(call, 0)?;
				if it.deferred {
					return refuse(span, "enumerating items that may panic is not supported");
				}
				self.same_list(call, false)?;
				compound(format!("Rust.enumerate {}", it.ex.paren()), &[&it.ex])
			}
			"Iterator::collect" => {
				let it = self.drained(call)?;
				let into = call.gargs.type_at(1);
				let ty::Adt(def, args) = into.kind() else {
					return refuse(span, format!("collecting into `{into}` is not supported"));
				};
				let set = match self.cx.std_name(def.did()).as_str() {
					"Vec" | "Arc" => false,
					"BTreeSet" => {
						self.ordered(args.type_at(0), span)?;
						true
					}
					_ => return refuse(span, format!("collecting into `{into}` is not supported")),
				};
				self.cx.ty(into, span)?;
				let items = it.made();
				match set {
					false => compound(format!("{}.toArray", items.paren()), &[&items]),
					true => compound(format!("Rust.BTreeSet.ofList {}", items.paren()), &[&items]),
				}
			}
			"Iterator::sum" => {
				let sum = call.gargs.type_at(1);
				// An integer's own `Sum`, of itself or references to it, which std writes.
				let item = self.cx.item(call.gargs.type_at(0));
				if !sum.is_integral() || item.is_none_or(|t| t.peel_refs() != sum) {
					return refuse(span, format!("summing into `{sum}` is not supported"));
				}
				// std checks it as the calling crate's build does, which is the package's alone.
				if !self.did.is_local() {
					return refuse(
						span,
						"`sum` in a dependency, whose overflow checks the translation cannot see, is not supported",
					);
				}
				let checked = tcx.sess.overflow_checks();
				let add = match checked {
					true => "(· +? ·)",
					false => "(fun a b => pure (a + b))",
				};
				match self.drain(call)? {
					Some((step, it)) => bound(compound(
						format!("Rust.drainFold {step} {add} (Rust.Scalar.ofInt 0) {}", it.paren()),
						&[&it],
					)),
					None => {
						let it = self.consumed(call, checked)?;
						match (it.deferred, checked) {
							(false, true) => bound(compound(format!("Rust.sum {}", it.ex.paren()), &[&it.ex])),
							(false, false) => compound(format!("Rust.sumWrapping {}", it.ex.paren()), &[&it.ex]),
							(true, _) => bound(compound(
								format!("Rust.lazyFold {add} (Rust.Scalar.ofInt 0) {}", it.ex.paren()),
								&[&it.ex],
							)),
						}
					}
				}
			}
			"Iterator::min_by_key" => {
				let f = self.closure(&call.args[1].node, span)?;
				self.ordered(call.gargs.type_at(1), span)?;
				match self.drain(call)? {
					Some((step, it)) => bound(compound(
						format!("Rust.drainMinByKey {step} {} {}", f.action(), it.paren()),
						&[&it, &f.f],
					)),
					None => {
						let it = self.consumed(call, f.monadic)?;
						match it.deferred || f.monadic {
							false => compound(
								format!("Rust.minByKey {} {}", f.f.paren(), it.ex.paren()),
								&[&it.ex, &f.f],
							),
							true => bound(compound(
								format!("Rust.lazyMinByKey {} {}", f.action(), it.lazy()),
								&[&it.ex, &f.f],
							)),
						}
					}
				}
			}
			"Iterator::min" => {
				let Some(item) = self.cx.item(call.gargs.type_at(0)) else {
					return refuse(span, "`min` of this iterator is not supported");
				};
				self.ordered(item, span)?;
				match self.drain(call)? {
					Some((step, it)) => bound(compound(
						format!("Rust.drainMinByKey {step} pure {}", it.paren()),
						&[&it],
					)),
					None => {
						let it = self.consumed(call, false)?;
						match it.deferred {
							false => compound(format!("Rust.minByKey id {}", it.ex.paren()), &[&it.ex]),
							true => bound(compound(format!("Rust.lazyMinByKey pure {}", it.ex.paren()), &[&it.ex])),
						}
					}
				}
			}
			"std::iter::once" | "core::iter::once" => {
				let x = self.operand(&call.args[0].node, span)?;
				self.same_list(call, false)?;
				atom_of(format!("[{}]", x.text), &[&x])
			}
			"RangeInclusive::new" => {
				let a = self.operand(&call.args[0].node, span)?;
				let b = self.operand(&call.args[1].node, span)?;
				self.cx.ty(call.destination.ty(self.body, tcx).ty, span)?;
				atom_of(format!("({}, {}, false)", a.text, b.text), &[&a, &b])
			}
			// Whether out-of-order bounds panic depends on whether the map has held an entry, even
			// one since removed, which its model does not record.
			"BTreeMap::range" | "BTreeSet::range" => {
				return refuse(
					span,
					format!(
						"`{key}` is not supported, since whether it panics depends on whether the collection has ever held an item"
					),
				);
			}
			"Extend::extend for BTreeSet" | "Extend::extend for Vec" => {
				// Extending by references to what the collection holds clones each item.
				let item = call.gargs.type_at(1);
				if let ty::Ref(_, t, _) = item.kind()
					&& element(call.gargs.type_at(0)) != item
					&& !self.clones(*t)
				{
					return refuse(span, "cloning these items may not copy them");
				}
				let place = self.mutable_arg(&call.args[0].node, span)?;
				let source = &call.args[1].node;
				self.through(source.ty(self.body, tcx), span, false)?;
				let it = self.iterated(source, span)?;
				let cur = self.read_place(place, span)?;
				let items = it.made();
				let new = match key {
					"Extend::extend for Vec" => {
						compound(format!("{} ++ {}.toArray", cur.paren(), items.paren()), &[&cur, &items])
					}
					_ => {
						self.ordered(element(call.gargs.type_at(0)), span)?;
						compound(format!("{}.extend {}", cur.paren(), items.paren()), &[&cur, &items])
					}
				};
				self.assign(place, new, span, ops)?;
				return Ok(true);
			}
			"From::from for BTreeSet" if matches!(call.args[0].node.ty(self.body, tcx).kind(), ty::Array(..)) => {
				self.ordered(element(call.gargs.type_at(0)), span)?;
				let a = self.operand(&call.args[0].node, span)?;
				compound(format!("Rust.BTreeSet.ofList {}.toList", a.paren()), &[&a])
			}
			"slice::iter" => {
				let s = self.operand(&call.args[0].node, span)?;
				self.same_list(call, false)?;
				compound(format!("{}.toList", s.paren()), &[&s])
			}
			"slice::windows" => {
				let s = self.operand(&call.args[0].node, span)?;
				let n = self.operand(&call.args[1].node, span)?;
				self.same_list(call, false)?;
				bound(compound(format!("Rust.windows {} {}", s.paren(), n.paren()), &[&s, &n]))
			}
			"slice::first" => {
				let s = self.operand(&call.args[0].node, span)?;
				compound(format!("{}[0]?", s.paren()), &[&s])
			}
			"slice::last" => {
				let s = self.operand(&call.args[0].node, span)?;
				compound(format!("{}.back?", s.paren()), &[&s])
			}
			"slice::contains" => {
				let t = call.gargs.type_at(0);
				if !self.cx.structural_eq(t) {
					return refuse(span, format!("`PartialEq` of `{t}` is not structural equality"));
				}
				let s = self.operand(&call.args[0].node, span)?;
				let x = self.operand(&call.args[1].node, span)?;
				compound(format!("{}.contains {}", s.paren(), x.paren()), &[&s, &x])
			}
			"Index::index for Vec" | "Index::index for slice" => {
				let s = self.operand(&call.args[0].node, span)?;
				let range = call.gargs.type_at(1);
				let ty::Adt(def, _) = range.kind() else {
					return refuse(span, format!("indexing by `{range}` is not supported"));
				};
				let r = self.operand(&call.args[1].node, span)?;
				let text = match self.cx.ops_range(def.did()).unwrap_or_default() {
					"RangeTo" => format!("Rust.sliceTo {} {}", s.paren(), r.paren()),
					"RangeFrom" => format!("Rust.sliceFrom {} {}", s.paren(), r.paren()),
					"Range" => format!("Rust.slice {} {}.1 {}.2", s.paren(), r.paren(), r.paren()),
					"RangeFull" => {
						if r.monadic {
							return refuse(span, "a `..` index that may panic is not supported");
						}
						self.assign(call.destination, s, span, ops)?;
						return Ok(true);
					}
					_ => return refuse(span, format!("indexing by `{range}` is not supported")),
				};
				bound(compound(text, &[&s, &r]))
			}
			"Option::is_some_and" => {
				let o = self.operand(&call.args[0].node, span)?;
				let f = self.closure(&call.args[1].node, span)?;
				if f.monadic {
					bound(compound(
						format!("Rust.isSomeAndM {} {}", o.paren(), f.f.paren()),
						&[&o, &f.f],
					))
				} else {
					compound(format!("Option.any {} {}", f.f.paren(), o.paren()), &[&o, &f.f])
				}
			}
			"Option::map" => {
				let o = self.operand(&call.args[0].node, span)?;
				let f = self.closure(&call.args[1].node, span)?;
				if f.monadic {
					bound(compound(
						format!("Rust.mapM {} {}", o.paren(), f.f.paren()),
						&[&o, &f.f],
					))
				} else {
					compound(format!("{}.map {}", o.paren(), f.f.paren()), &[&o, &f.f])
				}
			}
			"Option::expect" | "Option::unwrap" => {
				let o = self.operand(&call.args[0].node, span)?;
				let msg = if key == "Option::unwrap" {
					atom(lean_string("called `Option::unwrap()` on a `None` value"))
				} else {
					self.operand(&call.args[1].node, span)?
				};
				bound(compound(
					format!("Rust.expect {} {}", o.paren(), msg.paren()),
					&[&o, &msg],
				))
			}
			"Option::unwrap_or" => {
				let o = self.operand(&call.args[0].node, span)?;
				let v = self.operand(&call.args[1].node, span)?;
				compound(format!("{}.getD {}", o.paren(), v.paren()), &[&o, &v])
			}
			"Option::ok_or" => {
				let o = self.operand(&call.args[0].node, span)?;
				let e = self.operand(&call.args[1].node, span)?;
				compound(format!("Rust.okOr {} {}", o.paren(), e.paren()), &[&o, &e])
			}
			"Option::unwrap_or_default" => {
				let o = self.operand(&call.args[0].node, span)?;
				let d = self.default_value(call.gargs.type_at(0), span)?;
				if d.monadic {
					bound(compound(
						format!("Rust.getDM {} (fun _ => do pure {})", o.paren(), d.text),
						&[&o, &d],
					))
				} else {
					compound(format!("{}.getD {}", o.paren(), d.paren()), &[&o, &d])
				}
			}
			"std::mem::replace" | "core::mem::replace" => {
				let place = self.mutable_arg(&call.args[0].node, span)?;
				let cur = self.read_place(place, span)?;
				let v = self.operand(&call.args[1].node, span)?;
				self.assign(call.destination, cur, span, ops)?;
				self.assign(place, v, span, ops)?;
				return Ok(true);
			}
			"std::mem::take" | "core::mem::take" => {
				let place = self.mutable_arg(&call.args[0].node, span)?;
				let cur = self.read_place(place, span)?;
				let d = self.default_value(call.gargs.type_at(0), span)?;
				self.assign(call.destination, cur, span, ops)?;
				self.assign(place, d, span, ops)?;
				return Ok(true);
			}
			"Option::copied" => self.operand(&call.args[0].node, span)?,
			"Result::ok" => {
				let r = self.operand(&call.args[0].node, span)?;
				compound(format!("{}.toOption", r.paren()), &[&r])
			}
			// Between integers, failing with a `TryFromIntError`, which holds nothing.
			k if k.starts_with("TryFrom::try_from for ")
				&& call.args[0].node.ty(self.body, tcx).is_integral()
				&& matches!(call.destination.ty(self.body, tcx).ty.kind(),
					ty::Adt(_, a) if a.type_at(0).is_integral()
						&& matches!(a.type_at(1).kind(), ty::Adt(e, _) if self.cx.std_name(e.did()) == "TryFromIntError")) =>
			{
				let ty::Adt(_, a) = call.destination.ty(self.body, tcx).ty.kind() else {
					unreachable!("matched")
				};
				let to = self.cx.ty(a.type_at(0), span)?;
				let v = self.operand(&call.args[0].node, span)?;
				compound(format!("Rust.tryFrom {to} {}", v.paren()), &[&v])
			}
			"Option::cloned" => {
				let t = call.gargs.type_at(0);
				if !self.clones(t) {
					return refuse(span, format!("cloning `{t}` may not copy it"));
				}
				self.operand(&call.args[0].node, span)?
			}
			"bool::then_some" => {
				let b = self.operand(&call.args[0].node, span)?;
				let v = self.operand(&call.args[1].node, span)?;
				// A bind in `v` runs whatever `b` is, as Rust evaluates `v` first, so it is an
				// argument rather than inside a branch, where Lean cannot lift it.
				let text = match v.monadic {
					true => format!("(if {} then some · else none) {}", b.text, v.paren()),
					false => format!("if {} then some {} else none", b.text, v.paren()),
				};
				compound(text, &[&b, &v])
			}
			"Ord::cmp" if call.gargs.type_at(0).is_integral() || call.gargs.type_at(0).is_bool() => {
				let a = self.operand(&call.args[0].node, span)?;
				let b = self.operand(&call.args[1].node, span)?;
				let (a, b) = strip_lits(a, b);
				compound(format!("compare {} {}", a.paren(), b.paren()), &[&a, &b])
			}
			"Ord::min" | "Ord::max" if call.gargs.type_at(0).is_integral() => {
				let a = self.operand(&call.args[0].node, span)?;
				let b = self.operand(&call.args[1].node, span)?;
				let (a, b) = strip_lits(a, b);
				let f = if key == "Ord::min" { "min" } else { "max" };
				compound(format!("{f} {} {}", a.paren(), b.paren()), &[&a, &b])
			}
			// On any other type whose order Lean derives alike.
			"Ord::cmp" | "Ord::min" | "Ord::max" => {
				self.ordered(call.gargs.type_at(0), span)?;
				let a = self.operand(&call.args[0].node, span)?;
				let b = self.operand(&call.args[1].node, span)?;
				let f = match key {
					"Ord::cmp" => "compare",
					"Ord::min" => "Rust.ordMin",
					_ => "Rust.ordMax",
				};
				compound(format!("{f} {} {}", a.paren(), b.paren()), &[&a, &b])
			}
			"PartialOrd::lt" | "PartialOrd::le" | "PartialOrd::gt" | "PartialOrd::ge" => {
				let (t, rhs) = (call.gargs.type_at(0), call.gargs.type_at(1));
				if tcx.erase_and_anonymize_regions(t) != tcx.erase_and_anonymize_regions(rhs) {
					return refuse(span, format!("comparing `{t}` with `{rhs}` is not supported"));
				}
				self.ordered(t, span)?;
				let a = self.operand(&call.args[0].node, span)?;
				let b = self.operand(&call.args[1].node, span)?;
				let (a, b) = strip_lits(a, b);
				let rel = match key {
					"PartialOrd::lt" => "= .lt",
					"PartialOrd::le" => "≠ .gt",
					"PartialOrd::gt" => "= .gt",
					_ => "≠ .lt",
				};
				let prop = format!("compare {} {} {rel}", a.paren(), b.paren());
				let mut ex = compound(format!("decide ({prop})"), &[&a, &b]);
				ex.prop = Some(prop);
				ex
			}
			// On an integer: `Duration`'s have models of their own.
			k if (k.ends_with("::saturating_add") || k.ends_with("::saturating_sub"))
				&& call.args[0].node.ty(self.body, tcx).is_integral() =>
			{
				let a = self.operand(&call.args[0].node, span)?;
				let b = self.operand(&call.args[1].node, span)?;
				let (a, b) = strip_lits(a, b);
				let f = if k.ends_with("add") {
					"saturatingAdd"
				} else {
					"saturatingSub"
				};
				compound(format!("Rust.{f} {} {}", a.paren(), b.paren()), &[&a, &b])
			}
			k if k.starts_with("From::from for ") => {
				let to = self
					.cx
					.tcx
					.fn_sig(call.callee)
					.instantiate(tcx, call.gargs)
					.skip_norm_wip();
				let to = to.skip_binder().output();
				let from = call.args[0].node.ty(self.body, tcx);
				let v = self.operand(&call.args[0].node, span)?;
				let to_lean = self.cx.ty(to, span)?;
				if from.is_bool() && to.is_integral() {
					compound(format!("if {} then (1 : {to_lean}) else 0", v.text), &[&v])
				} else if from.is_integral() && to.is_integral() {
					// `From` between integers only widens, so a cast is exact.
					compound(format!("Rust.cast {to_lean} {}", v.paren()), &[&v])
				} else {
					return Ok(false);
				}
			}
			"Arc::new" | "Deref::deref for Arc" => self.operand(&call.args[0].node, span)?,
			"BTreeMap::new" | "BTreeSet::new" => {
				let ty = self.cx.ty(call.destination.ty(self.body, tcx).ty, span)?;
				let ns = if key.starts_with("BTreeMap") {
					"BTreeMap"
				} else {
					"BTreeSet"
				};
				atom(format!("(Rust.{ns}.empty : {ty})"))
			}
			"BTreeMap::get" | "BTreeMap::get_key_value" | "BTreeMap::contains_key" | "BTreeSet::contains" => {
				let n = call.gargs.len();
				let (k, q) = (call.gargs.type_at(0), call.gargs.type_at(n - 1));
				if !self.same_key(k, q) {
					return refuse(span, format!("looking up `{k}` by `{q}` is not supported"));
				}
				self.ordered(k, span)?;
				let m = self.operand(&call.args[0].node, span)?;
				let x = self.operand(&call.args[1].node, span)?;
				let f = match key {
					"BTreeMap::get" => "get",
					"BTreeMap::get_key_value" => "getKeyValue",
					"BTreeMap::contains_key" => "containsKey",
					_ => "contains",
				};
				compound(format!("{}.{f} {}", m.paren(), x.paren()), &[&m, &x])
			}
			"Index::index for BTreeMap" => {
				let ty::Adt(_, margs) = call.gargs.type_at(0).kind() else {
					return refuse(span, "indexing this map is not supported");
				};
				// `&Q`: only the lookup's own borrow comes off, since a key may be a reference too.
				let q = match call.gargs.type_at(1).kind() {
					ty::Ref(_, q, _) => *q,
					_ => call.gargs.type_at(1),
				};
				let k = margs.type_at(0);
				if !self.same_key(k, q) {
					return refuse(span, format!("looking up `{k}` by `{q}` is not supported"));
				}
				self.ordered(k, span)?;
				let m = self.operand(&call.args[0].node, span)?;
				let x = self.operand(&call.args[1].node, span)?;
				bound(compound(
					format!("Rust.BTreeMap.index {} {}", m.paren(), x.paren()),
					&[&m, &x],
				))
			}
			"BTreeMap::insert"
			| "BTreeMap::remove"
			| "BTreeMap::pop_first"
			| "BTreeSet::insert"
			| "BTreeSet::remove"
			| "BTreeSet::pop_first" => {
				let (ns, method) = key.split_once("::").expect("keys have a method");
				let k = call.gargs.type_at(0);
				// A lookup by a borrowed form of the key, such as `str` for `String`, has no model.
				if method == "remove" {
					let q = call.gargs.type_at(call.gargs.len() - 1);
					if !self.same_key(k, q) {
						return refuse(span, format!("looking up `{k}` by `{q}` is not supported"));
					}
				}
				if method != "pop_first" {
					self.ordered(k, span)?;
				}
				let f = match method {
					"insert" => "insert",
					"remove" => "remove",
					_ => "popFirst",
				};
				self.mutate(&format!("{ns}.{f}"), false, call, ops)?;
				return Ok(true);
			}
			"VecDeque::pop_front" => {
				self.mutate("VecDeque.popFront", false, call, ops)?;
				return Ok(true);
			}
			"VecDeque::new" => {
				let ty = self.cx.ty(call.destination.ty(self.body, tcx).ty, span)?;
				atom(format!("(Rust.VecDeque.empty : {ty})"))
			}
			"VecDeque::push_back" => {
				let place = self.mutable_arg(&call.args[0].node, span)?;
				let x = self.operand(&call.args[1].node, span)?;
				let cur = self.read_place(place, span)?;
				let new = compound(format!("{}.pushBack {}", cur.paren(), x.paren()), &[&cur, &x]);
				self.assign(place, new, span, ops)?;
				return Ok(true);
			}
			"VecDeque::is_empty" | "VecDeque::len" | "VecDeque::iter" => {
				let d = self.operand(&call.args[0].node, span)?;
				let text = match key {
					"VecDeque::is_empty" => format!("{}.items.isEmpty", d.paren()),
					"VecDeque::len" => format!("{}.items.length.toUInt64", d.paren()),
					_ => {
						self.same_list(call, false)?;
						format!("{}.items", d.paren())
					}
				};
				compound(text, &[&d])
			}
			"BTreeSet::first" => {
				let s = self.operand(&call.args[0].node, span)?;
				compound(format!("{}.items.head?", s.paren()), &[&s])
			}
			"BTreeMap::keys"
			| "BTreeMap::values"
			| "BTreeMap::into_keys"
			| "BTreeMap::into_values"
			| "BTreeMap::iter"
			| "BTreeSet::iter" => {
				let m = self.operand(&call.args[0].node, span)?;
				self.same_list(call, false)?;
				let text = match key {
					"BTreeMap::keys" | "BTreeMap::into_keys" => format!("{}.entries.map (·.1)", m.paren()),
					"BTreeMap::values" | "BTreeMap::into_values" => format!("{}.entries.map (·.2)", m.paren()),
					"BTreeMap::iter" => format!("{}.entries", m.paren()),
					_ => format!("{}.items", m.paren()),
				};
				compound(text, &[&m])
			}
			k if unary_model(k).is_some() => {
				let n = self.operand(&call.args[0].node, span)?;
				let f = unary_model(k).expect("matched");
				compound(format!("{f} {}", n.paren()), &[&n])
			}
			k if duration_model(k).is_some() => {
				let a = self.operand(&call.args[0].node, span)?;
				let b = self.operand(&call.args[1].node, span)?;
				let f = duration_model(k).expect("matched");
				compound(format!("Rust.{f} {} {}", a.paren(), b.paren()), &[&a, &b])
			}
			"BTreeMap::len" | "BTreeSet::len" => {
				let m = self.operand(&call.args[0].node, span)?;
				compound(format!("{}.len", m.paren()), &[&m])
			}
			"BTreeMap::is_empty" | "BTreeSet::is_empty" => {
				let m = self.operand(&call.args[0].node, span)?;
				let field = if key.starts_with("BTreeMap") {
					"entries"
				} else {
					"items"
				};
				compound(format!("{}.{field}.isEmpty", m.paren()), &[&m])
			}
			_ => return Ok(false),
		};
		self.assign(call.destination, value, span, ops)?;
		Ok(true)
	}

	/// Lower `call` if it makes a lens, or a value holding one: `false` if no model does.
	/// `get_mut` and `entry` point at the value at a key of a map, and `get_or_insert_default`
	/// at what an `Option` holds; `expect` and `unwrap` take the `&mut` out of an `Option`.
	pub(super) fn lens_model(&mut self, key: &str, call: &Call<'_, 'tcx>, ops: &mut Vec<Op>) -> R<bool> {
		let tcx = self.cx.tcx;
		let span = call.span;
		let dest = call.destination;
		if !dest.projection.is_empty() {
			return Ok(false);
		}
		match key {
			"BTreeMap::get_mut" | "BTreeMap::entry" => {
				let k = call.gargs.type_at(0);
				if key == "BTreeMap::get_mut" {
					let q = call.gargs.type_at(call.gargs.len() - 1);
					if !self.same_key(k, q) {
						return refuse(span, format!("looking up `{k}` by `{q}` is not supported"));
					}
				}
				self.ordered(k, span)?;
				let base = self.mutable_arg(&call.args[0].node, span)?;
				let lean_k = self.cx.ty(k, span)?;
				let x = self.operand(&call.args[1].node, span)?;
				let x = self.capture(x, lean_k, ops);
				let m = self.read_place(base, span)?;
				let value = compound(format!("{}.get {}", m.paren(), x.paren()), &[&m, &x]);
				self.hold(
					dest.local,
					Lens {
						base,
						steps: vec![Proj::At(x)],
					},
					span,
				)?;
				self.assign(dest, value, span, ops)?;
			}
			"Entry::or_default" | "Entry::or_insert" | "Entry::or_insert_with" | "VacantEntry::insert" => {
				let Some(lens) = self.carrier_arg(&call.args[0].node) else {
					return refuse(span, "this map entry is not supported");
				};
				let [Proj::At(k)] = lens.steps.as_slice() else {
					return refuse(span, "this map entry is not supported");
				};
				let k = k.clone();
				let m = self.read_place(lens.base, span)?;
				// The value an absent key gets, as a thunk, since Rust makes it only then.
				let (v, monadic) = match key {
					"Entry::or_default" => {
						let ty = dest.ty(self.body, tcx).ty.peel_refs();
						let d = self.default_value(ty, span)?;
						let m = d.monadic;
						(
							if m {
								compound(format!("do pure {}", d.text), &[&d])
							} else {
								d
							},
							m,
						)
					}
					"Entry::or_insert_with" => {
						let f = self.closure(&call.args[1].node, span)?;
						(f.f, f.monadic)
					}
					// Made whether the key is there or not, as Rust evaluates it first.
					_ => {
						let v = self.operand(&call.args[1].node, span)?;
						let ty = call.args[1].node.ty(self.body, tcx);
						(self.eager(v, ty, span, ops)?, false)
					}
				};
				let new = match (key, monadic) {
					("VacantEntry::insert", _) => compound(
						format!("{}.setAt {} {}", m.paren(), k.paren(), v.paren()),
						&[&m, &k, &v],
					),
					(_, true) => bound(compound(
						format!(
							"Rust.BTreeMap.orInsertWithM {} {} (fun _ => {})",
							m.paren(),
							k.paren(),
							v.text
						),
						&[&m, &k, &v],
					)),
					(_, false) => compound(
						format!("{}.orInsertWith {} (fun _ => {})", m.paren(), k.paren(), v.text),
						&[&m, &k, &v],
					),
				};
				self.assign(lens.base, new, span, ops)?;
				self.lenses.insert(dest.local, lens);
			}
			"Option::get_or_insert_default" => {
				let base = self.mutable_arg(&call.args[0].node, span)?;
				let ty = dest.ty(self.body, tcx).ty.peel_refs();
				let d = self.default_value(ty, span)?;
				let o = self.read_place(base, span)?;
				let new = if d.monadic {
					bound(compound(
						format!("Rust.getOrInsertWithM {} (fun _ => do pure {})", o.paren(), d.text),
						&[&o, &d],
					))
				} else {
					compound(format!("some ({}.getD {})", o.paren(), d.paren()), &[&o, &d])
				};
				self.assign(base, new, span, ops)?;
				self.lenses.insert(
					dest.local,
					Lens {
						base,
						steps: vec![Proj::Some],
					},
				);
			}
			"Option::expect" | "Option::unwrap" if matches!(self.mutable.get(&dest.local), Some(Mutable::Lens)) => {
				let Some(lens) = self.carrier_arg(&call.args[0].node) else {
					return refuse(span, "this mutable reference is not supported");
				};
				let o = self.operand(&call.args[0].node, span)?;
				let msg = if key == "Option::unwrap" {
					atom(lean_string("called `Option::unwrap()` on a `None` value"))
				} else {
					// Made whether the value is there or not, as Rust evaluates it first.
					let m = self.operand(&call.args[1].node, span)?;
					let ty = call.args[1].node.ty(self.body, tcx);
					self.eager(m, ty, span, ops)?
				};
				let cond = compound(format!("{}.isSome", o.paren()), &[&o, &msg]);
				self.before(ops, None, true);
				ops.push(Op::Check { cond, msg: msg.text });
				self.lenses.insert(dest.local, lens);
			}
			_ => return Ok(false),
		}
		Ok(true)
	}

	/// Lower a collection's method that changes it through `&mut self` as the runtime's `f`,
	/// which returns the new collection and the result, for the borrowed place and the
	/// destination, in `Rust.M` if `monadic`.
	fn mutate(&mut self, f: &str, monadic: bool, call: &Call<'_, 'tcx>, ops: &mut Vec<Op>) -> R<()> {
		let span = call.span;
		let place = self.mutable_arg(&call.args[0].node, span)?;
		let cur = self.read_place(place, span)?;
		let mut reads = vec![cur];
		for a in &call.args[1..] {
			reads.push(self.operand(&a.node, span)?);
		}
		let mut text = format!("Rust.{f}");
		for r in &reads {
			text += &format!(" {}", r.paren());
		}
		let refs: Vec<&Ex> = reads.iter().collect();
		let r = self.var("r", String::new(), false);
		self.before(ops, None, monadic);
		let value = compound(text, &refs);
		ops.push(Op::Set {
			var: r,
			value: Ex { monadic, ..value },
			bind: monadic,
		});
		for (i, target) in [place, call.destination].into_iter().enumerate() {
			let mut ex = atom(tuple_field(&self.vars[r].name, i, 2));
			ex.reads.insert(r);
			let (var, update) = self.write(target, ex, span)?;
			self.before(ops, Some(var), false);
			push_set(ops, var, update);
			self.written.insert(var);
		}
		Ok(())
	}

	/// Whether a lookup of a `K` key by a `Q` finds what a lookup by the same key does: `Q` is
	/// `K`, or `str` for moq-net's `Path`, whose `Borrow<str>` is the string the model holds.
	fn same_key(&self, k: Ty<'tcx>, q: Ty<'tcx>) -> bool {
		let tcx = self.cx.tcx;
		tcx.erase_and_anonymize_regions(k) == tcx.erase_and_anonymize_regions(q)
			|| (self.cx.moq_path_ty(k) && q.is_str())
	}

	/// Refuse a key type whose `Ord` the translation's `Ord` may not match.
	fn ordered(&self, ty: Ty<'tcx>, span: Span) -> R<()> {
		if !self.cx.structural_ord(ty) {
			return refuse(span, format!("`Ord` of `{ty}` may not match a Lean order"));
		}
		// A parameter's Lean instance comes from its own `Ord` bound.
		let ord = self.cx.tcx.get_diagnostic_item(rustc_span::sym::Ord);
		let unbound = ty.walk().filter_map(|a| a.as_type()).find(
			|t| matches!(t.kind(), ty::Param(p) if !ord.is_some_and(|o| self.cx.param_bounds(self.did, p.index).contains(&o))),
		);
		match unbound {
			Some(t) => refuse(span, format!("ordering `{t}` needs an `Ord` bound on it")),
			None => Ok(()),
		}
	}

	/// The closure operand `op` as a Lean function, or a function item passed in its place.
	fn closure(&mut self, op: &Operand<'tcx>, span: Span) -> R<Lambda> {
		let tcx = self.cx.tcx;
		let ty = op.ty(self.body, tcx);
		if let ty::FnDef(f, args) = *ty.kind() {
			return self.fn_value(f, args.skip_binder(), span);
		}
		let ty::Closure(did, cargs) = *ty.kind() else {
			return refuse(
				span,
				format!("passing `{ty}` where a closure is taken is not supported"),
			);
		};
		// Refuses a closure that changes what it captures.
		self.cx.ty(ty, span)?;
		// One that takes a `&mut`, such as `Option::map` on `get_mut`'s result, returns what it
		// wrote beside its result, which no model takes.
		if self.cx.holds_mut(cargs.as_closure().sig().skip_binder().inputs()[0]) {
			return refuse(span, "a closure taking a mutable reference is not supported");
		}
		let env = self.operand(op, span)?;
		let monadic = self.cx.shapes.get(&did).is_none_or(|s| s.monadic || s.failed);
		let mut text = self.cx.fn_name(did);
		for (param, arg) in all_params(tcx, did).iter().zip(cargs.as_closure().parent_args()) {
			let value = match (&param.kind, arg.as_type(), arg.as_const()) {
				(ty::GenericParamDefKind::Type { .. }, Some(t), _) => self.cx.ty(t, span)?,
				(ty::GenericParamDefKind::Const { .. }, _, Some(c)) => self.cx.const_value(c, span)?,
				_ => continue,
			};
			text += &format!(" ({} := {value})", self.cx.param(did.krate, param.name));
		}
		text += &format!(" {}", env.paren());
		Ok(Lambda {
			f: atom_of(format!("({text})"), &[&env]),
			monadic,
		})
	}

	/// Function item `f` passed as a value, as a Lean function: a translated function, a
	/// constructor, or a `std` function a model applies alike.
	fn fn_value(&mut self, f: DefId, args: GenericArgsRef<'tcx>, span: Span) -> R<Lambda> {
		let tcx = self.cx.tcx;
		let sig = tcx.instantiate_bound_regions_with_erased(tcx.fn_sig(f).instantiate(tcx, args).skip_norm_wip());
		if sig.inputs().iter().any(|t| self.cx.holds_mut(*t)) {
			return refuse(span, "passing a function taking a mutable reference is not supported");
		}
		// What it returns, refused if it has no model, and queued if it is translated.
		self.cx.ty(sig.output(), span)?;
		if let DefKind::Ctor(of, rustc_hir::def::CtorKind::Fn) = tcx.def_kind(f) {
			let variant = tcx.parent(f);
			let adt = match of {
				rustc_hir::def::CtorOf::Struct => variant,
				rustc_hir::def::CtorOf::Variant => tcx.parent(variant),
			};
			let name = tcx.item_name(variant);
			let ctor = match (self.cx.translated(adt), self.cx.std_name(adt).as_str(), name.as_str()) {
				(true, _, _) if tcx.adt_def(adt).is_struct() => format!("{}.mk", self.cx.adt_name(adt)),
				(true, _, name) => format!("{}.{}", self.cx.adt_name(adt), structure::ident(name)),
				(false, "Option", "Some") => "some".into(),
				(false, "Result", "Ok") => "Except.ok".into(),
				(false, "Result", "Err") => "Except.error".into(),
				_ => {
					return refuse(
						span,
						format!("passing `{}` as a function has no Lean model", tcx.def_path_str(f)),
					);
				}
			};
			return Ok(Lambda {
				f: atom(ctor),
				monadic: false,
			});
		}
		if let Some((callee, gargs)) = resolve_local(tcx, self.env, f, args) {
			let text = self.callee_text(callee, gargs, span)?;
			return Ok(Lambda {
				f: atom(format!("({text})")),
				monadic: self.cx.shapes.get(&callee).is_none_or(|s| s.monadic || s.failed),
			});
		}
		let did = ty::Instance::try_resolve(tcx, self.env, f, args)
			.ok()
			.flatten()
			.map_or(f, |i| i.def_id());
		let key = self.cx.std_key(did);
		match unary_model(&key).or_else(|| passed_model(&key)) {
			Some(model) => Ok(Lambda {
				f: atom(model.into()),
				monadic: false,
			}),
			None => refuse(
				span,
				format!("passing `{}` as a function has no Lean model", tcx.def_path_str(f)),
			),
		}
	}

	/// Argument `i` of `call`, an iterator taken by value: a list, or an integer range, which
	/// becomes the list of its items.
	fn list(&mut self, call: &Call<'_, 'tcx>, i: usize) -> R<List> {
		let op = &call.args[i].node;
		let ty = op.ty(self.body, self.cx.tcx);
		match self.cx.iterable(ty) {
			Some((items @ (Items::Same | Items::Range(_)), deferred)) => {
				let v = self.operand(op, call.span)?;
				Ok(List {
					ex: items_of(items, v),
					deferred,
				})
			}
			_ => refuse(call.span, format!("`{ty}` has no Lean model")),
		}
	}

	/// The iterator a consumer that reads every item takes as its first argument: a list, or
	/// an iterator written in a translated crate, whose `next` makes them.
	fn drained(&mut self, call: &Call<'_, 'tcx>) -> R<List> {
		match self.drain(call)? {
			Some((step, it)) => Ok(List {
				ex: bound(compound(format!("Rust.drain {step} {}", it.paren()), &[&it])),
				deferred: false,
			}),
			None => self.consumed(call, false),
		}
	}

	/// The list of the iterator a consumer that reads every item takes as its first argument,
	/// where the consumer `panics` partway if it may.
	fn consumed(&mut self, call: &Call<'_, 'tcx>, panics: bool) -> R<List> {
		self.through(call.args[0].node.ty(self.body, self.cx.tcx), call.span, panics)?;
		self.list(call, 0)
	}

	/// The `next` of an iterator written in a translated crate that consumer `call` reads every
	/// item of, as a Lean function into `Rust.M`, and the iterator: `None` for another iterator.
	fn drain(&mut self, call: &Call<'_, 'tcx>) -> R<Option<(String, Ex)>> {
		let tcx = self.cx.tcx;
		let Some((next, args)) = super::drained_next(tcx, self.env, call.callee, call.gargs) else {
			return Ok(None);
		};
		// The consumer reaches `next` through `fold`, `size_hint`, and the like, so it calls
		// only `next` where the impl writes nothing else.
		if let Some(other) = super::beside_next(tcx, next) {
			return refuse(
				call.span,
				format!("an iterator whose impl writes `{other}` beside `next` is not supported"),
			);
		}
		let it = self.operand(&call.args[0].node, call.span)?;
		let text = self.callee_text(next, args, call.span)?;
		let step = match self.cx.shapes.get(&next).is_none_or(|s| s.monadic || s.failed) {
			true => format!("({text})"),
			false => format!("(pure ∘ ({text}))"),
		};
		Ok(Some((step, it)))
	}

	/// Refuse a closure that may panic over `call`'s iterator, if that may make several items
	/// at once.
	fn no_runs(&self, call: &Call<'_, 'tcx>) -> R<()> {
		let ty = call.args[0].node.ty(self.body, self.cx.tcx);
		match self.cx.runs(ty) {
			true => refuse(
				call.span,
				"a closure that may panic over a flat map's items, which may panic, is not supported",
			),
			false => Ok(()),
		}
	}

	/// The iterator `call` borrows mutably as its first argument, which nothing reads after.
	fn list_mut(&mut self, call: &Call<'_, 'tcx>) -> R<List> {
		let place = self.mutable_arg(&call.args[0].node, call.span)?;
		let ty = place.ty(self.body, self.cx.tcx).ty;
		let Some((_, deferred)) = self.cx.iterator(ty) else {
			return refuse(call.span, format!("`{ty}` has no Lean model"));
		};
		self.whole(ty, call.span)?;
		if !place.projection.is_empty()
			|| self.uses[place.local.as_usize()] != 1
			|| self.reruns(call.block, place.local)
		{
			return refuse(
				call.span,
				"an iterator read after it is partly consumed is not supported",
			);
		}
		Ok(List {
			ex: self.read_place(place, call.span)?,
			deferred,
		})
	}

	/// Refuse a consumer that may stop before the end of an iterator of type `ty` over a wide
	/// integer range, whose list makes every item however far Rust steps it.
	fn whole(&self, ty: Ty<'tcx>, span: Span) -> R<()> {
		match self.cx.ranged(ty) {
			true => refuse(
				span,
				"stopping early in an iterator over a wide integer range is not supported, since its list makes every item",
			),
			false => Ok(()),
		}
	}

	/// Refuse a consumer that reads every item of an iterator of type `ty` over a wide integer
	/// range where an item, or the consumer itself if it `panics`, may panic partway, which its
	/// list would reach only after making every item.
	fn through(&self, ty: Ty<'tcx>, span: Span, panics: bool) -> R<()> {
		let deferred = self.cx.iterator(ty).is_some_and(|(_, d)| d);
		match self.cx.ranged(ty) && (deferred || panics) {
			true => refuse(
				span,
				"a panic partway through an iterator over a wide integer range is not supported, since its list makes every item first",
			),
			false => Ok(()),
		}
	}

	/// Whether block `bb` can run again before `local` is set anew, as in a loop consuming an
	/// iterator made before it.
	fn reruns(&self, bb: BasicBlock, local: Local) -> bool {
		let blocks = &self.body.basic_blocks;
		let mut seen = HashSet::new();
		let mut stack: Vec<BasicBlock> = blocks[bb].terminator().successors().collect();
		while let Some(b) = stack.pop() {
			if b == bb {
				return true;
			}
			if !seen.insert(b) {
				continue;
			}
			let data = &blocks[b];
			let sets = data.statements.iter().any(|s| match &s.kind {
				StatementKind::Assign(a) => a.0.local == local && a.0.projection.is_empty(),
				StatementKind::StorageLive(l) | StatementKind::StorageDead(l) => *l == local,
				_ => false,
			}) || matches!(&data.terminator().kind,
				TerminatorKind::Call { destination, .. } if destination.local == local && destination.projection.is_empty());
			if !sets {
				stack.extend(data.terminator().successors());
			}
		}
		false
	}

	/// What `op` iterates as a list, for `IntoIterator`: an iterator, or an integer range, an
	/// array, a slice, a `Vec`, an `Option`, a `VecDeque`, a `BTreeMap`, or a `BTreeSet`, or a
	/// reference to one.
	fn iterated(&mut self, op: &Operand<'tcx>, span: Span) -> R<List> {
		let ty = op.ty(self.body, self.cx.tcx);
		let Some((items, deferred)) = self.cx.iterable(ty) else {
			return refuse(span, format!("iterating `{ty}` is not supported"));
		};
		let v = self.operand(op, span)?;
		Ok(List {
			ex: items_of(items, v),
			deferred,
		})
	}

	/// Refuse a call whose result is an iterator of another shape than the model makes.
	fn same_list(&mut self, call: &Call<'_, 'tcx>, deferred: bool) -> R<()> {
		let ty = call.destination.ty(self.body, self.cx.tcx).ty;
		match self.cx.iterator(ty) {
			Some((_, d)) if d == deferred => Ok(()),
			_ => refuse(call.span, format!("`{ty}` has no Lean model")),
		}
	}
}

/// The list of the items of `v`, which `items` iterates.
fn items_of(items: Items, v: Ex) -> Ex {
	match items {
		Items::Same => v,
		Items::Range(f) => compound(format!("{f} {}", v.paren()), &[&v]),
		Items::Collection(method, _) => compound(format!("{}{method}", v.paren()), &[&v]),
	}
}

/// The runtime's model of a two-argument `Duration` method.
fn duration_model(key: &str) -> Option<&'static str> {
	Some(match key {
		"Duration::checked_add" => "Duration.checkedAdd",
		"Duration::checked_sub" => "Duration.checkedSub",
		"Duration::checked_mul" => "Duration.checkedMul",
		"Duration::saturating_add" => "Duration.saturatingAdd",
		"Duration::saturating_sub" => "Duration.saturatingSub",
		"Duration::saturating_mul" => "Duration.saturatingMul",
		_ => return None,
	})
}

/// The runtime's model of a one-argument `std` function that cannot panic, as a Lean function,
/// which a call applies and a function argument passes as it is.
pub(super) fn unary_model(key: &str) -> Option<&'static str> {
	Some(match key {
		"Duration::from_secs" => "Rust.Duration.fromSecs",
		"Duration::from_millis" => "Rust.Duration.fromMillis",
		"Duration::from_nanos" => "Rust.Duration.fromNanos",
		"Duration::as_nanos" => "Rust.Duration.asNanos",
		"Duration::as_micros" => "Rust.Duration.asMicros",
		"Duration::as_millis" => "Rust.Duration.asMillis",
		_ => return None,
	})
}

/// The runtime's model of a one-argument `std` method that cannot panic, as a Lean function a
/// function argument passes, such as `flat_map(BTreeMap::keys)`; a call has a model of its own.
pub(super) fn passed_model(key: &str) -> Option<&'static str> {
	Some(match key {
		"BTreeMap::keys" => "Rust.BTreeMap.keys",
		"BTreeMap::values" => "Rust.BTreeMap.values",
		"BTreeMap::len" => "Rust.BTreeMap.len",
		"BTreeMap::is_empty" => "Rust.BTreeMap.isEmpty",
		_ => return None,
	})
}

/// `s` as a Lean string literal.
pub(super) fn lean_string(s: &str) -> String {
	let mut out = String::from("\"");
	for c in s.chars() {
		match c {
			'"' => out += "\\\"",
			'\\' => out += "\\\\",
			'\n' => out += "\\n",
			'\t' => out += "\\t",
			c if (c as u32) < 0x20 || c == '\u{7f}' => out += &format!("\\x{:02x}", c as u32),
			c => out.push(c),
		}
	}
	out.push('"');
	out
}
