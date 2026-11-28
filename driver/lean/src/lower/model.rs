//! Models of `std` calls that take closures or make iterators, and of the `Option`, slice,
//! integer, `Ord`, `Arc`, `BTreeMap`/`BTreeSet`, `VecDeque`, `Duration`, and kio `Instant`
//! functions beside them, and of the calls that make a lens.
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

/// An iterator's list, and whether its items are actions.
struct List {
	ex: Ex,
	deferred: bool,
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
			Some(t) if ["Iterator::", "Ord::", "PartialOrd::"].iter().any(|p| t.starts_with(p)) => t,
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
			"Iterator::map" => {
				let it = self.list(call, 0)?;
				let f = self.closure(&call.args[1].node, span)?;
				let text = match (it.deferred, f.monadic) {
					(false, false) => format!("{}.map {}", it.ex.paren(), f.f.paren()),
					(false, true) => format!("{}.map (Rust.defer {})", it.ex.paren(), f.f.paren()),
					(true, false) => format!("{}.map (Rust.deferMap {})", it.ex.paren(), f.f.paren()),
					(true, true) => format!("{}.map (Rust.deferBind {})", it.ex.paren(), f.f.paren()),
				};
				let deferred = it.deferred || f.monadic;
				self.same_list(call, deferred)?;
				compound(text, &[&it.ex, &f.f])
			}
			"Iterator::any" | "Iterator::all" => {
				let it = self.list_mut(call)?;
				let f = self.closure(&call.args[1].node, span)?;
				let name = if key == "Iterator::any" { "any" } else { "all" };
				match (it.deferred, f.monadic) {
					(false, false) => compound(format!("{}.{name} {}", it.ex.paren(), f.f.paren()), &[&it.ex, &f.f]),
					(false, true) => bound(compound(
						format!("List.{name}M {} {}", f.f.paren(), it.ex.paren()),
						&[&it.ex, &f.f],
					)),
					(true, monadic) => {
						let g = if monadic {
							f.f.paren()
						} else {
							format!("(pure ∘ {})", f.f.paren())
						};
						bound(compound(
							format!("List.{name}M (Rust.force {g}) {}", it.ex.paren()),
							&[&it.ex, &f.f],
						))
					}
				}
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
				let lift = |l: &List| {
					if deferred && !l.deferred {
						format!("({}.map Rust.ready)", l.ex.paren())
					} else {
						l.ex.paren()
					}
				};
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
				let it = self.list(call, 0)?;
				let into = call.gargs.type_at(1);
				let ty::Adt(def, _) = into.kind() else {
					return refuse(span, format!("collecting into `{into}` is not supported"));
				};
				match self.cx.std_name(def.did()).as_str() {
					"Vec" | "Arc" => {}
					_ => return refuse(span, format!("collecting into `{into}` is not supported")),
				}
				self.cx.ty(into, span)?;
				if it.deferred {
					let run = bound(compound(format!("Rust.run {}", it.ex.paren()), &[&it.ex]));
					compound(format!("{}.toArray", run.text), &[&run])
				} else {
					compound(format!("{}.toArray", it.ex.paren()), &[&it.ex])
				}
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
			// On an integer: `Duration`'s and kio `Instant`'s have models of their own.
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
				if tcx.erase_and_anonymize_regions(k) != tcx.erase_and_anonymize_regions(q) {
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
				if tcx.erase_and_anonymize_regions(k) != tcx.erase_and_anonymize_regions(q) {
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
					if tcx.erase_and_anonymize_regions(k) != tcx.erase_and_anonymize_regions(q) {
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
				self.mutate(&format!("{ns}.{f}"), call, ops)?;
				return Ok(true);
			}
			"VecDeque::pop_front" => {
				self.mutate("VecDeque.popFront", call, ops)?;
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
			"BTreeMap::keys" | "BTreeMap::values" | "BTreeMap::iter" | "BTreeSet::iter" => {
				let m = self.operand(&call.args[0].node, span)?;
				self.same_list(call, false)?;
				let text = match key {
					"BTreeMap::keys" => format!("{}.entries.map (·.1)", m.paren()),
					"BTreeMap::values" => format!("{}.entries.map (·.2)", m.paren()),
					"BTreeMap::iter" => format!("{}.entries", m.paren()),
					_ => format!("{}.items", m.paren()),
				};
				compound(text, &[&m])
			}
			"Duration::from_secs" | "Duration::from_millis" | "Duration::from_nanos" => {
				let n = self.operand(&call.args[0].node, span)?;
				let f = match key {
					"Duration::from_secs" => "fromSecs",
					"Duration::from_millis" => "fromMillis",
					_ => "fromNanos",
				};
				compound(format!("Rust.Duration.{f} {}", n.paren()), &[&n])
			}
			// `Instant - Duration`, which an older kio also has, would take the model's `Instant`.
			"Sub::sub for kio::Instant" if !matches!(call.args[1].node.ty(self.body, tcx).kind(), ty::Adt(d, _) if self.cx.kio_instant(d.did())) =>
			{
				return refuse(span, "subtracting a `Duration` from an `Instant` has no Lean model");
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
					if tcx.erase_and_anonymize_regions(k) != tcx.erase_and_anonymize_regions(q) {
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
	/// destination.
	fn mutate(&mut self, f: &str, call: &Call<'_, 'tcx>, ops: &mut Vec<Op>) -> R<()> {
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
		self.before(ops, None, false);
		ops.push(Op::Set {
			var: r,
			value: compound(text, &refs),
			bind: false,
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

	/// The closure operand `op` as a Lean function.
	fn closure(&mut self, op: &Operand<'tcx>, span: Span) -> R<Lambda> {
		let tcx = self.cx.tcx;
		let ty = op.ty(self.body, tcx);
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
			text += &format!(" ({} := {value})", structure::ident(param.name.as_str()));
		}
		text += &format!(" {}", env.paren());
		Ok(Lambda {
			f: atom_of(format!("({text})"), &[&env]),
			monadic,
		})
	}

	/// Argument `i` of `call`, an iterator taken by value.
	fn list(&mut self, call: &Call<'_, 'tcx>, i: usize) -> R<List> {
		let op = &call.args[i].node;
		let ty = op.ty(self.body, self.cx.tcx);
		let Some((_, deferred)) = self.cx.iterator(ty) else {
			return refuse(call.span, format!("`{ty}` has no Lean model"));
		};
		Ok(List {
			ex: self.operand(op, call.span)?,
			deferred,
		})
	}

	/// The iterator `call` borrows mutably as its first argument, which nothing reads after.
	fn list_mut(&mut self, call: &Call<'_, 'tcx>) -> R<List> {
		let place = self.mutable_arg(&call.args[0].node, call.span)?;
		let ty = place.ty(self.body, self.cx.tcx).ty;
		let Some((_, deferred)) = self.cx.iterator(ty) else {
			return refuse(call.span, format!("`{ty}` has no Lean model"));
		};
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

	/// What `op` iterates as a list, for `IntoIterator`: an array, a `Vec`, an `Option`, or an
	/// iterator.
	fn iterated(&mut self, op: &Operand<'tcx>, span: Span) -> R<List> {
		let ty = op.ty(self.body, self.cx.tcx);
		if let Some((_, deferred)) = self.cx.iterator(ty) {
			return Ok(List {
				ex: self.operand(op, span)?,
				deferred,
			});
		}
		let v = self.operand(op, span)?;
		let pure = match ty.kind() {
			ty::Array(..) => true,
			ty::Adt(def, _) => matches!(self.cx.std_name(def.did()).as_str(), "Vec" | "Option"),
			_ => false,
		};
		if !pure {
			return refuse(span, format!("iterating `{ty}` is not supported"));
		}
		Ok(List {
			ex: compound(format!("{}.toList", v.paren()), &[&v]),
			deferred: false,
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

/// The runtime's model of a two-argument `Duration` or kio `Instant` method.
fn duration_model(key: &str) -> Option<&'static str> {
	Some(match key {
		"Duration::checked_add" => "Duration.checkedAdd",
		"Duration::checked_sub" => "Duration.checkedSub",
		"Duration::checked_mul" => "Duration.checkedMul",
		"Duration::saturating_add" => "Duration.saturatingAdd",
		"Duration::saturating_sub" => "Duration.saturatingSub",
		"Duration::saturating_mul" => "Duration.saturatingMul",
		"kio::Instant::checked_add" => "kio.Instant.checkedAdd",
		"kio::Instant::checked_sub" => "kio.Instant.checkedSub",
		"kio::Instant::saturating_add" => "kio.Instant.saturatingAdd",
		"kio::Instant::saturating_sub" => "kio.Instant.saturatingSub",
		"kio::Instant::checked_duration_since" => "kio.Instant.checkedDurationSince",
		"kio::Instant::saturating_duration_since" | "kio::Instant::duration_since" | "Sub::sub for kio::Instant" => {
			"kio.Instant.saturatingDurationSince"
		}
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
