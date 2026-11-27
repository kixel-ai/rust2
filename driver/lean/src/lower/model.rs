//! Models of `std` calls that take closures or make iterators, and of the `Option`, slice,
//! integer, `Ord`, `Arc`, and `BTreeMap`/`BTreeSet` functions beside them.
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
			Some(t) if t.starts_with("Iterator::") || t.starts_with("Ord::") => t,
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
			k if k.ends_with("::saturating_add") || k.ends_with("::saturating_sub") => {
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
			"BTreeMap::get" | "BTreeMap::contains_key" | "BTreeSet::contains" => {
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
					"BTreeMap::contains_key" => "containsKey",
					_ => "contains",
				};
				compound(format!("{}.{f} {}", m.paren(), x.paren()), &[&m, &x])
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

	/// Refuse a key type whose `Ord` the translation's `Ord` may not match.
	fn ordered(&self, ty: Ty<'tcx>, span: Span) -> R<()> {
		if self.cx.structural_ord(ty) {
			Ok(())
		} else {
			refuse(span, format!("`Ord` of `{ty}` may not match a Lean order"))
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
