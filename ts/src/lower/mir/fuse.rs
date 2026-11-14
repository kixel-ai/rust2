//! Iterator chains fused into their consumer: a `count`, `any`, `all`, `fold`, `sum`,
//! `product`, `for_each`, `find`, `position`, or `for` loop of `map`, `filter`,
//! `enumerate`, and `zip` adaptors over a slice's, a `Vec`'s, or a range's iterator, or
//! the `rev` of one, is one loop where it is read, not the `rs.Iter` objects the runtime
//! would chain. Each adaptor's `next` in the runtime is one function every chain in the
//! program calls through, which engines stop inlining once they have seen a few, and each
//! `zip` and `enumerate` makes a pair per item.
//!
//! The loop reads the leaves, which stay `rs.Iter`s, and calls the closures in the order
//! and as often as the runtime's adaptors would, on the state they would keep: an
//! `enumerate`'s count, the pairs left of a zip that std reads by index, and those a fold
//! of a `TrustedLen` zip reads by its size hint. It partially evaluates the runtime for
//! these leaves, whose `folded` and `indexed` are themselves and whose `find` reads by
//! `next`, so the readers each consumer and adaptor picks all read by `next`. A closure
//! whose body is one expression is written in place of its call, and a pair reaches it
//! as its parts, made whole only where something reads the tuple. A `for` loop's `next`
//! is written into the loop once the function is: [`loop_parts`] leaves the loop where
//! the chain ends and reads the item's parts, and [`inline_funs`] writes its closures in.

use rustc_data_structures::fx::{FxHashMap, FxHashSet};
use rustc_middle::mir::{self, Local, Operand, Rvalue, StatementKind, TerminatorKind};
use rustc_middle::ty::{self, Ty};
use rustc_span::Span;

use rust2_driver::cfg::Cond;

use super::Fx;
use super::step::peel;
use crate::lower::library;
use crate::ts::{Expr, Stmt, StmtKind, Type};

/// A consumer the loop replaces: the runtime function the mapping calls, and how many of
/// its arguments after the iterator are values it reads, before the flags and droppers.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Consumer {
	Count,
	Any,
	All,
	Fold,
	ForEach,
	Find,
	Position,
}

impl Consumer {
	fn of(name: &str) -> Option<Self> {
		Some(match name {
			"iterCount" => Self::Count,
			"iterAny" => Self::Any,
			"iterAll" => Self::All,
			"iterFold" | "iterSum" | "iterProduct" => Self::Fold,
			"iterForEach" => Self::ForEach,
			"iterFind" => Self::Find,
			"iterPosition" => Self::Position,
			_ => return None,
		})
	}

	/// Whether the runtime reads the iterator as `folded(true)` steps it, as std's `fold`
	/// does, rather than as `try_fold` does.
	fn folds(self) -> bool {
		matches!(self, Self::Count | Self::Fold | Self::ForEach)
	}
}

/// How a zip reads its sides.
#[derive(Clone)]
pub(super) enum Zipped {
	/// By `next`, each side's item and then the other's, dropping the first's by `drop` if
	/// the other has ended.
	Next { drop: Option<Expr> },
	/// By index, both being `TrustedRandomAccess`: as many pairs as the shorter has, counted
	/// by `index` up to `end`.
	Indexed { index: String, end: String },
	/// By a fold, both being `TrustedLen`: as many pairs as its size hint, `left` of them.
	Counted { left: String },
}

/// One node of a chain, its parts bound to variables.
#[derive(Clone)]
pub(super) enum Node {
	/// A slice's, a `Vec`'s, or a range's iterator, which stays an `rs.Iter`.
	Leaf {
		var: String,
		item: Type,
	},
	Map {
		inner: Box<Node>,
		f: Fun,
		item: Type,
	},
	Filter {
		inner: Box<Node>,
		f: Fun,
		drop: Option<Expr>,
		item: Type,
	},
	Enumerate {
		inner: Box<Node>,
		count: String,
		item: Type,
	},
	Zip {
		a: Box<Node>,
		b: Box<Node>,
		how: Zipped,
		item: Type,
	},
}

/// A closure or value a fused chain reads: what holds it, and, for a closure whose body is
/// one expression, the arrow, which the loop writes in place of each call.
#[derive(Clone)]
pub(super) struct Fun {
	var: Expr,
	arrow: Option<Expr>,
}

/// An item a fused chain reads: a value, or a tuple held as its parts, which only what
/// needs the whole tuple makes.
#[derive(Clone)]
enum Val {
	One(Expr),
	Tuple(Vec<Val>),
}

impl Val {
	fn whole(&self) -> Expr {
		match self {
			Val::One(e) => e.clone(),
			Val::Tuple(parts) => Expr::Array(parts.iter().map(Val::whole).collect()),
		}
	}

	/// Whether reading it again does nothing, as each variable or literal it holds is.
	fn pure(&self) -> bool {
		match self {
			Val::One(e) => e.pure(),
			Val::Tuple(parts) => parts.iter().all(Val::pure),
		}
	}
}

/// A chain as its mapping made it, before its parts are bound.
enum Shape<'tcx> {
	Leaf(Expr, Ty<'tcx>),
	/// Each closure with its type.
	Map(Box<Shape<'tcx>>, (Expr, Ty<'tcx>), Ty<'tcx>),
	Filter(Box<Shape<'tcx>>, (Expr, Ty<'tcx>), Option<Expr>, Ty<'tcx>),
	Enumerate(Box<Shape<'tcx>>, Ty<'tcx>),
	Zip(Box<Shape<'tcx>>, Box<Shape<'tcx>>, Trusted, Option<Expr>, Ty<'tcx>),
}

/// A zip's `rs.Trusted`.
enum Trusted {
	/// Read by index.
	Indexed,
	/// Read by `next`, but folded by its size hint.
	Bounded,
	/// Read by `next`.
	Next,
}

/// The adaptor `ty` is, by its path in std.
fn adaptor(tcx: ty::TyCtxt<'_>, ty: Ty<'_>) -> Option<&'static str> {
	let ty::Adt(adt, _) = ty.kind() else {
		return None;
	};
	let path = library::std_path(tcx, adt.did());
	let path: Vec<&str> = path.iter().map(String::as_str).collect();
	Some(match path.as_slice() {
		["iter", "adapters", "map", "Map"] => "map",
		["iter", "adapters", "filter", "Filter"] => "filter",
		["iter", "adapters", "enumerate", "Enumerate"] => "enumerate",
		["iter", "adapters", "zip", "Zip"] => "zip",
		_ => return None,
	})
}

/// `Some` of an argument that isn't `undefined`.
fn given(e: Option<&Expr>) -> Option<Expr> {
	e.filter(|e| !matches!(peel(e), Expr::Lit(l) if l == "undefined"))
		.cloned()
}

/// Whether `e` is absent, `undefined`, or `false`: a flag that is off.
fn off(e: Option<&Expr>) -> bool {
	e.is_none_or(|e| matches!(peel(e), Expr::Lit(l) if l == "undefined" || l == "false"))
}

impl<'tcx> Fx<'_, '_, '_, 'tcx> {
	/// Whether a call of `def_id` on `self_ty` is a consumer whose chain may fuse: a
	/// consumer of `Iterator` on one of the adaptors this fuses. The mapping's call decides
	/// the rest.
	pub(super) fn fuses(&self, def_id: rustc_hir::def_id::DefId, gargs: ty::GenericArgs<'tcx>) -> bool {
		let Some((owner, name)) = library::key(self.tcx, def_id) else {
			return false;
		};
		owner == "Iterator"
			&& matches!(
				name.as_str(),
				"count" | "any" | "all" | "fold" | "sum" | "product" | "for_each" | "find" | "position"
			)
			&& gargs.types().next().is_some_and(|ty| adaptor(self.tcx, ty).is_some())
	}

	/// The statements that compute `init`, a consumer the mapping made of a chain of type
	/// `ty`, as one loop, and what holds its value then, or `None` where it doesn't fuse.
	/// `acc` is the type of a fold's accumulator.
	pub(super) fn fuse(
		&mut self,
		init: &Expr,
		ty: Ty<'tcx>,
		acc: Option<Ty<'tcx>>,
		span: Span,
	) -> Option<(Vec<Stmt>, Expr)> {
		let Expr::Call(f, args) = peel(init) else {
			return None;
		};
		let Expr::Rs(runtime) = peel(f) else {
			return None;
		};
		let consumer = Consumer::of(runtime)?;
		let (chain, rest) = args.split_first()?;
		// The values it reads, and the flags after them, each of which must be off but a
		// fold's `owns`, which drops its closure as it ends.
		let (values, flags): (&[Expr], &[Expr]) = match (consumer, runtime.as_str()) {
			(Consumer::Count, _) => (&[], rest),
			(Consumer::Fold, _) => rest.split_at_checked(2)?,
			_ => rest.split_at_checked(1)?,
		};
		let owns = match (consumer, runtime.as_str()) {
			(Consumer::Fold, "iterFold") | (Consumer::ForEach, _) => {
				if !off(flags.first()) || flags.len() > 2 {
					return None;
				}
				!off(flags.get(1))
			}
			(Consumer::Fold, _) => {
				if !off(flags.first()) || flags.len() > 1 {
					return None;
				}
				false
			}
			(Consumer::Find, _) if flags.len() <= 2 => false,
			_ if !flags.is_empty() => return None,
			_ => false,
		};
		let shape = self.chain_shape(chain, ty)?;
		let mut out = Vec::new();
		let root = self.bind(shape, consumer.folds(), span, &mut out);
		// What it reads, held in order, then `sum`'s `add` and zero swapped into `fold`'s.
		let mut values: Vec<Fun> = values.iter().map(|v| self.fun(v.clone(), None, &mut out)).collect();
		if runtime != "iterFold" && consumer == Consumer::Fold {
			values.swap(0, 1);
		}
		let label = self.b.fresh("fused");
		let done = vec![StmtKind::Break(Some(label.clone())).at(None)];
		let mut body = Vec::new();
		let used = consumer != Consumer::Count;
		let item = self
			.read(&root, false, used, &done, &mut body)
			.unwrap_or(Val::One(Expr::undefined()));
		let result = match consumer {
			Consumer::Count => {
				let n = self.b.fresh("n");
				out.push(StmtKind::decl(true, &n, None, Some(Expr::lit("0"))).at(None));
				body.push(StmtKind::Expr(Expr::Postfix(Box::new(Expr::ident(&n)), "++")).at(None));
				Expr::ident(n)
			}
			Consumer::Any | Consumer::All => {
				let any = consumer == Consumer::Any;
				let found = self.b.fresh("found");
				out.push(StmtKind::decl(true, &found, None, Some(Expr::lit((!any).to_string()))).at(None));
				let test = apply(&values[0], vec![item], false);
				body.push(
					StmtKind::If {
						cond: if any { test } else { test.not() },
						then: vec![
							StmtKind::Expr(Expr::ident(&found).assign(Expr::lit(any.to_string()))).at(None),
							StmtKind::Break(Some(label.clone())).at(None),
						],
						els: Vec::new(),
					}
					.at(None),
				);
				Expr::ident(found)
			}
			Consumer::Fold => {
				let a = self.b.fresh("acc");
				let ty = acc.map(|acc| self.b.t.ty(acc, span));
				out.push(StmtKind::decl(true, &a, ty, Some(values[0].var.clone())).at(None));
				let step = apply(&values[1], vec![Val::One(Expr::ident(&a)), item], false);
				body.push(StmtKind::Expr(Expr::ident(&a).assign(step)).at(None));
				Expr::ident(a)
			}
			Consumer::ForEach => {
				let call = apply(&values[0], vec![item], true);
				if !call.pure() {
					body.push(StmtKind::Expr(call).at(None));
				}
				Expr::undefined()
			}
			Consumer::Find => {
				// `findItem`: what `f` rejects drops by the item's dropper.
				let drop = given(flags.first()).map(|d| self.part(d, "drop", None, &mut out));
				let found = self.b.fresh("found");
				let ty = Type::Union(vec![item_of(&root), Type::Rs("Done", Vec::new())]);
				out.push(StmtKind::decl(true, &found, Some(ty), Some(Expr::Rs("DONE".into()))).at(None));
				let mut then = vec![StmtKind::Expr(Expr::ident(&found).assign(item.whole())).at(None)];
				then.push(StmtKind::Break(Some(label.clone())).at(None));
				body.push(
					StmtKind::If {
						cond: apply(&values[0], vec![item.clone()], false),
						then,
						els: Vec::new(),
					}
					.at(None),
				);
				if let Some(drop) = drop {
					body.push(StmtKind::Expr(drop.call(vec![item.whole()])).at(None));
				}
				let none = flags.get(1).cloned();
				Expr::rs("iterToOption", [Expr::ident(found)].into_iter().chain(none).collect())
			}
			Consumer::Position => {
				let (i, at) = (self.b.fresh("i"), self.b.fresh("at"));
				out.push(StmtKind::decl(true, &i, None, Some(Expr::lit("0"))).at(None));
				let number = Type::Builtin("number");
				let maybe = Type::Union(vec![number, Type::Builtin("undefined")]);
				out.push(StmtKind::decl(true, &at, Some(maybe), Some(Expr::undefined())).at(None));
				body.push(
					StmtKind::If {
						cond: apply(&values[0], vec![item], false),
						then: vec![
							StmtKind::Expr(Expr::ident(&at).assign(Expr::ident(&i))).at(None),
							StmtKind::Break(Some(label.clone())).at(None),
						],
						els: Vec::new(),
					}
					.at(None),
				);
				body.push(StmtKind::Expr(Expr::Postfix(Box::new(Expr::ident(&i)), "++")).at(None));
				Expr::ident(at)
			}
		};
		out.push(
			StmtKind::While {
				label: Some(label),
				cond: Expr::lit("true"),
				body,
			}
			.at(None),
		);
		if owns {
			let f = values.last().expect("a fold's closure").var.clone();
			out.push(StmtKind::Expr(Expr::rs("drop", vec![f])).at(None));
		}
		prune(&mut out, &result);
		Some((out, result))
	}

	/// `e` replaced by what the statement just emitted to `out` declares it, if `e` names
	/// that variable and nothing else reads its local: a chain a consumer borrows is held
	/// in its own variable, though only the borrow reads it.
	pub(super) fn take_held(&mut self, e: &mut Expr, out: &mut Vec<Stmt>) {
		let Expr::Ident(name) = peel(e) else {
			return;
		};
		let Some(local) = self
			.local_named(name)
			.filter(|&l| self.uses[l] == 1 && self.defs[l] == 1)
		else {
			return;
		};
		let held = match out.last().map(|s| &s.kind) {
			Some(StmtKind::Decl {
				mutable: false,
				name: decl,
				init: Some(_),
				..
			}) => decl == name,
			Some(StmtKind::Expr(Expr::Assign(target, _))) => matches!(&**target, Expr::Ident(n) if n == name),
			_ => false,
		};
		if !held {
			return;
		}
		match out.pop().map(|s| s.kind) {
			Some(StmtKind::Decl { init: Some(init), .. }) => *e = init,
			Some(StmtKind::Expr(Expr::Assign(_, init))) => {
				// Declared up front, which it no longer needs.
				self.assigned[local] = false;
				*e = *init;
			}
			_ => unreachable!("checked above"),
		}
	}

	/// `e`, a chain of type `ty`, as its nodes, if each is one this fuses.
	fn chain_shape(&self, e: &Expr, ty: Ty<'tcx>) -> Option<Shape<'tcx>> {
		let tcx = self.tcx;
		let item = |ty: Ty<'tcx>| self.b.t.iter_item(ty);
		let rs = |f: &Expr, name: &str| matches!(peel(f), Expr::Rs(n) if n == name);
		let Some(kind) = adaptor(tcx, ty) else {
			return leaf(e).then(|| Shape::Leaf(e.clone(), item(ty).expect("an iterator's item")));
		};
		let ty::Adt(_, gargs) = ty.kind() else {
			return None;
		};
		let Expr::Call(f, args) = peel(e) else {
			return None;
		};
		let inner = gargs.type_at(0);
		match kind {
			"map" if rs(f, "iterMap") && args.len() == 2 => {
				let f = (args[1].clone(), gargs.type_at(1));
				Some(Shape::Map(Box::new(self.chain_shape(&args[0], inner)?), f, item(ty)?))
			}
			"filter" if rs(f, "iterFilter") && (2..=4).contains(&args.len()) && off(args.get(3)) => {
				Some(Shape::Filter(
					Box::new(self.chain_shape(&args[0], inner)?),
					(args[1].clone(), gargs.type_at(1)),
					given(args.get(2)),
					item(ty)?,
				))
			}
			"enumerate" if rs(f, "iterEnumerate") && args.len() == 1 => Some(Shape::Enumerate(
				Box::new(self.chain_shape(&args[0], inner)?),
				item(ty)?,
			)),
			"zip" if rs(f, "iterZip") && (2..=5).contains(&args.len()) => {
				let trusted = match args.get(2).map(peel) {
					None => Trusted::Next,
					Some(Expr::Lit(l)) if l == "undefined" => Trusted::Next,
					Some(Expr::Str(_)) => Trusted::Bounded,
					Some(Expr::Array(effects)) if effects.len() == 2 => Trusted::Indexed,
					_ => return None,
				};
				// What drops the second side's item is never called reading forward.
				Some(Shape::Zip(
					Box::new(self.chain_shape(&args[0], inner)?),
					Box::new(self.chain_shape(&args[1], gargs.type_at(1))?),
					trusted,
					given(args.get(3)),
					item(ty)?,
				))
			}
			_ => None,
		}
	}

	/// `shape`'s parts held in variables, in the order JS evaluates them where the chain
	/// is made, and the state each adaptor starts with as the runtime's is made.
	/// `folds` where the chain is read as a fold reads it, as each adaptor but `zip` passes
	/// to what it wraps.
	fn bind(&mut self, shape: Shape<'tcx>, folds: bool, span: Span, out: &mut Vec<Stmt>) -> Node {
		let number = || Type::Builtin("number");
		match shape {
			Shape::Leaf(e, item) => {
				let var = self.b.fresh("leaf");
				let item = self.b.t.ty(item, span);
				self.decl(false, &var, Type::Rs("Iter", vec![item.clone()]), e, out);
				Node::Leaf { var, item }
			}
			Shape::Map(inner, (f, f_ty), item) => {
				let inner = Box::new(self.bind(*inner, folds, span, out));
				let f_ty = self.b.t.ty(f_ty, span);
				let f = self.fun(f, Some(f_ty), out);
				let item = self.b.t.ty(item, span);
				Node::Map { inner, f, item }
			}
			Shape::Filter(inner, (f, f_ty), drop, item) => {
				let inner = Box::new(self.bind(*inner, folds, span, out));
				let item = self.b.t.ty(item, span);
				let f_ty = self.b.t.ty(f_ty, span);
				let f = self.fun(f, Some(f_ty), out);
				let drop = drop.map(|d| self.part(d, "drop", Some(dropper(&item)), out));
				Node::Filter { inner, f, drop, item }
			}
			Shape::Enumerate(inner, item) => {
				let inner = Box::new(self.bind(*inner, folds, span, out));
				let count = self.b.fresh("count");
				self.decl(true, &count, number(), Expr::lit("0"), out);
				let item = self.b.t.ty(item, span);
				Node::Enumerate { inner, count, item }
			}
			Shape::Zip(a, b, trusted, drop, item) => {
				let a = Box::new(self.bind(*a, false, span, out));
				let b = Box::new(self.bind(*b, false, span, out));
				let drop = drop.map(|d| self.part(d, "drop", Some(dropper(&item_of(&a))), out));
				let how = match trusted {
					// As `TrustedZipIter` is made: the shorter side's length.
					Trusted::Indexed => {
						let (index, end) = (self.b.fresh("index"), self.b.fresh("end"));
						let len = Expr::ident("Math").member("min").call(vec![self.len(&a), self.len(&b)]);
						self.decl(true, &index, number(), Expr::lit("0"), out);
						self.decl(false, &end, number(), len, out);
						Zipped::Indexed { index, end }
					}
					// As `ZipFold` starts: the pairs its size hint's upper bound gives.
					Trusted::Bounded if folds => {
						let left = self.b.fresh("left");
						let len = Expr::ident("Math").member("min").call(vec![upper(&a), upper(&b)]);
						self.decl(true, &left, number(), len, out);
						Zipped::Counted { left }
					}
					Trusted::Bounded | Trusted::Next => Zipped::Next { drop },
				};
				let item = self.b.t.ty(item, span);
				Node::Zip { a, b, how, item }
			}
		}
	}

	/// `name = init`: a variable declared here, or, for a chain `next` reads, up front, as
	/// `ty`, and assigned here.
	fn decl(&mut self, mutable: bool, name: &str, ty: Type, init: Expr, out: &mut Vec<Stmt>) {
		if self.fused_hoist {
			self.fused_decls.push((name.to_string(), ty));
			out.push(StmtKind::Expr(Expr::ident(name).assign(init)).at(None));
		} else {
			out.push(StmtKind::decl(mutable, name, None, Some(init)).at(None));
		}
	}

	/// `e`, a closure or value, held as [`Self::part`] holds it, with the arrow to write in
	/// place of its calls if it is one of one expression. A chain `next` reads is made
	/// apart from where it is read, so it calls its closures.
	fn fun(&mut self, e: Expr, ty: Option<Type>, out: &mut Vec<Stmt>) -> Fun {
		let arrow = Some(peel(&e))
			.filter(|e| !self.fused_hoist && inlinable(e, true).is_some())
			.cloned();
		let var = self.part(e, "f", ty, out);
		if self.fused_hoist
			&& let Expr::Ident(name) = peel(&var)
		{
			self.fused_funs.insert(name.clone());
		}
		Fun { var, arrow }
	}

	/// `e` held in a variable named for `base`, of type `ty`, unless it is one already or a
	/// literal. Only a chain `next` reads, whose variables are declared up front, needs `ty`.
	fn part(&mut self, e: Expr, base: &str, ty: Option<Type>, out: &mut Vec<Stmt>) -> Expr {
		if matches!(peel(&e), Expr::Ident(_) | Expr::Lit(_) | Expr::Rs(_) | Expr::Sym(_)) {
			return e;
		}
		let name = self.b.fresh(base);
		let ty = ty.unwrap_or(Type::Builtin("unknown"));
		self.decl(false, &name, ty, e, out);
		Expr::ident(name)
	}

	/// Find the chains only `next` reads, as a `for` loop's: a local of an adaptor this
	/// fuses, made by a call, that drops nothing JS sees, and that only `&mut` borrows read,
	/// each assigned once, borrowed by nothing, and read only as `next`'s receiver.
	pub(super) fn find_fused(&mut self) {
		let mir = self.mir;
		let tcx = self.tcx;
		let mut ok: FxHashSet<Local> = FxHashSet::default();
		for l in mir.local_decls.indices() {
			let ty = mir.local_decls[l].ty;
			if !self.is_arg(l)
				&& l != mir::RETURN_PLACE
				&& self.defs[l] == 1
				&& !self.stepped.contains(&l)
				&& adaptor(tcx, ty).is_some()
				&& !self.b.t.drops(ty)
			{
				ok.insert(l);
			}
		}
		if ok.is_empty() {
			return;
		}
		let mut reads: FxHashMap<Local, u32> = FxHashMap::default();
		let mut borrows: FxHashMap<Local, Local> = FxHashMap::default();
		let mut made = FxHashSet::default();
		let mut next_reads: FxHashMap<Local, u32> = FxHashMap::default();
		for data in mir.basic_blocks.iter() {
			for stmt in &data.statements {
				if let StatementKind::Assign(assign) = &stmt.kind
					&& let (to, Rvalue::Ref(_, mir::BorrowKind::Mut { .. }, from)) = &**assign
					&& let Some(from) = from.as_local()
					&& ok.contains(&from)
				{
					*reads.entry(from).or_default() += 1;
					match to.as_local() {
						Some(to) => {
							borrows.insert(to, from);
						}
						None => {
							ok.remove(&from);
						}
					}
				}
			}
			if let TerminatorKind::Call {
				func,
				args,
				destination,
				..
			} = &data.terminator().kind
			{
				if let Some(to) = destination.as_local() {
					made.insert(to);
				}
				if let ty::FnDef(def_id, _) = *func.ty(&mir.local_decls, tcx).kind()
					&& crate::lower::iter::is_next(tcx, def_id)
					&& destination.as_local().is_some_and(|l| self.next_results.contains(&l))
					&& let [arg] = &**args
					&& let Operand::Move(p) | Operand::Copy(p) = &arg.node
					&& let Some(recv) = p.as_local()
				{
					*next_reads.entry(recv).or_default() += 1;
				}
			}
		}
		let stepping = |fx: &Self, b: Local| {
			fx.defs[b] == 1 && !fx.borrowed[b] && next_reads.get(&b).copied().unwrap_or(0) == fx.uses[b]
		};
		// Made by a call, or moved from what a call made.
		for data in mir.basic_blocks.iter() {
			for stmt in &data.statements {
				if let StatementKind::Assign(assign) = &stmt.kind
					&& let (to, Rvalue::Use(Operand::Move(from), _)) = &**assign
					&& let (Some(to), Some(from)) = (to.as_local(), from.as_local())
					&& made.contains(&from)
				{
					made.insert(to);
				}
			}
		}
		ok.retain(|&l| made.contains(&l) && reads.get(&l).copied().unwrap_or(0) == self.uses[l]);
		ok.retain(|&l| borrows.iter().all(|(&b, &from)| from != l || stepping(self, b)));
		for (b, from) in borrows {
			if ok.contains(&from) {
				self.fused_borrows.insert(b, from);
			}
		}
		self.fused_loops = ok;
	}

	/// The statements making `local`, a chain only `next` reads, from `init`, the chain its
	/// mapping made: its parts held in variables, or, if it isn't one this fuses, the chain.
	pub(super) fn fuse_loop(&mut self, local: Local, init: Expr, span: Span) -> Vec<Stmt> {
		let ty = self.mir.local_decls[local].ty;
		let mut out = Vec::new();
		self.fused_hoist = true;
		let node = match self.chain_shape(&init, ty) {
			Some(shape) => self.bind(shape, false, span, &mut out),
			None => {
				let var = match self.var(local) {
					Some(name) => name.to_string(),
					None => self.b.fresh("chain"),
				};
				let item = self.b.t.iter_item(ty).expect("an iterator's item");
				let item = self.b.t.ty(item, span);
				self.decl(false, &var, Type::Rs("Iter", vec![item.clone()]), init, &mut out);
				Node::Leaf { var, item }
			}
		};
		self.fused_hoist = false;
		self.fused_nodes.insert(local, node);
		out
	}

	/// Make `local`, a chain only `next` reads, of `value` after `pre`: the chain is folded
	/// into one statement, whose parts `fuse_loop` then holds in variables.
	pub(super) fn make_fused(
		&mut self,
		local: Local,
		value: Expr,
		mut pre: Vec<Stmt>,
		span: Span,
		out: &mut Vec<Stmt>,
	) {
		let name = self.b.fresh("chain");
		pre.push(StmtKind::decl(false, &name, None, Some(value)).at(None));
		self.emit(pre, false, out);
		match out.pop() {
			Some(Stmt {
				kind: StmtKind::Decl {
					name: decl,
					init: Some(init),
					..
				},
				..
			}) if decl == name => {
				let mut init = init;
				self.take_held(&mut init, out);
				let stmts = self.fuse_loop(local, init, span);
				out.extend(stmts);
			}
			_ => {
				self.err(span, "a fused chain its statement didn't make");
			}
		}
	}

	/// `next` of the chain `recv` borrows, if it is fused, read from its parts into `out`:
	/// its item or `rs.DONE`.
	pub(super) fn fused_next(&mut self, recv: &Operand<'tcx>, span: Span, out: &mut Vec<Stmt>) -> Option<Expr> {
		let (Operand::Copy(p) | Operand::Move(p)) = recv else {
			return None;
		};
		let chain = *self.fused_borrows.get(&p.as_local()?)?;
		let Some(node) = self.fused_nodes.get(&chain).cloned() else {
			return Some(self.err(span, "`next` of a fused chain before it is made"));
		};
		// One the runtime makes, whose `next` is its own.
		if let Node::Leaf { var, .. } = &node {
			return Some(Expr::ident(var).member("next").call(Vec::new()));
		}
		let (r, label) = (self.b.fresh("next"), self.b.fresh("next"));
		self.fused_reads.insert(r.clone());
		let ty = Type::Union(vec![item_of(&node), Type::Rs("Done", Vec::new())]);
		out.push(StmtKind::Declare { name: r.clone(), ty }.at(None));
		let done = vec![
			StmtKind::Expr(Expr::ident(&r).assign(Expr::Rs("DONE".into()))).at(None),
			StmtKind::Break(Some(label.clone())).at(None),
		];
		let mut body = Vec::new();
		let item = self.read(&node, false, true, &done, &mut body).expect("a used item");
		body.push(StmtKind::Expr(Expr::ident(&r).assign(item.whole())).at(None));
		out.push(
			StmtKind::Block {
				label: Some(label),
				body,
			}
			.at(None),
		);
		Some(Expr::ident(r))
	}

	/// The length of a side of a zip read by index, as its `len` gives it.
	fn len(&self, node: &Node) -> Expr {
		match node {
			Node::Leaf { var, .. } => Expr::ident(var).member("len").call(Vec::new()),
			Node::Map { inner, .. } | Node::Enumerate { inner, .. } => self.len(inner),
			Node::Zip {
				how: Zipped::Indexed { index, end },
				..
			} => Expr::ident(end).binary("-", Expr::ident(index)),
			// Neither is `TrustedRandomAccess`.
			Node::Filter { .. } | Node::Zip { .. } => unreachable!("a side read by index"),
		}
	}

	/// The next item of `node`, read into `out`, which runs `done` where it has ended, or
	/// `None` where the item isn't `used`. `indexed` reads a side of a zip read by index,
	/// which never ends there.
	fn read(&mut self, node: &Node, indexed: bool, used: bool, done: &[Stmt], out: &mut Vec<Stmt>) -> Option<Val> {
		let expr = |e: Expr| StmtKind::Expr(e).at(None);
		match node {
			Node::Leaf { var, item } => {
				let next = Expr::ident(var).member("next").call(Vec::new());
				if indexed && !used {
					out.push(expr(next));
					return None;
				}
				let x = self.b.fresh("x");
				if indexed {
					out.push(StmtKind::decl(false, &x, None, Some(Expr::As(Box::new(next), item.clone()))).at(None));
				} else {
					out.push(StmtKind::decl(false, &x, None, Some(next)).at(None));
					out.push(ended(&x, done));
				}
				Some(Val::One(Expr::ident(x)))
			}
			Node::Map { inner, f, item } => {
				let x = self.read(inner, indexed, true, done, out)?;
				let mapped = apply(f, vec![x], false);
				if !used {
					if !mapped.pure() {
						out.push(expr(mapped));
					}
					return None;
				}
				// Of the closure's type, which an inlined body's own may be wider than.
				let y = self.b.fresh("y");
				out.push(StmtKind::decl(false, &y, Some(item.clone()), Some(mapped)).at(None));
				Some(Val::One(Expr::ident(y)))
			}
			Node::Filter { inner, f, drop, item } => {
				// `find` of what it wraps: each item until `f` keeps one, those it rejects
				// dropped, the one it keeps held, part by part, past the loop.
				let label = self.b.fresh("filter");
				let mut body = Vec::new();
				let x = self.read(inner, false, true, done, &mut body)?;
				let held = used.then(|| self.keep(&x, item, out));
				let mut then = Vec::new();
				if let Some(held) = &held {
					assign(held, &x, &mut then);
				}
				then.push(StmtKind::Break(Some(label.clone())).at(None));
				body.push(
					StmtKind::If {
						cond: apply(f, vec![x.clone()], false),
						then,
						els: Vec::new(),
					}
					.at(None),
				);
				if let Some(drop) = drop {
					body.push(expr(drop.clone().call(vec![x.whole()])));
				}
				out.push(
					StmtKind::While {
						label: Some(label),
						cond: Expr::lit("true"),
						body,
					}
					.at(None),
				);
				held
			}
			Node::Enumerate { inner, count, .. } => {
				let x = self.read(inner, indexed, used, done, out);
				let n = Expr::Postfix(Box::new(Expr::ident(count)), "++");
				let Some(x) = x else {
					out.push(expr(n));
					return None;
				};
				let i = self.b.fresh("i");
				out.push(StmtKind::decl(false, &i, None, Some(n)).at(None));
				Some(Val::Tuple(vec![Val::One(Expr::ident(i)), x]))
			}
			Node::Zip { a, b, how, .. } => {
				let (x, y) = match how {
					Zipped::Indexed { index, end } => {
						out.push(
							StmtKind::If {
								cond: Expr::ident(index).binary(">=", Expr::ident(end)),
								then: done.to_vec(),
								els: Vec::new(),
							}
							.at(None),
						);
						out.push(expr(Expr::Postfix(Box::new(Expr::ident(index)), "++")));
						let x = self.read(a, true, used, done, out);
						(x, self.read(b, true, used, done, out))
					}
					// `TrustedLen` sides, which never end before it.
					Zipped::Counted { left } => {
						out.push(
							StmtKind::If {
								cond: Expr::ident(left).binary("===", Expr::lit("0")),
								then: done.to_vec(),
								els: Vec::new(),
							}
							.at(None),
						);
						out.push(expr(Expr::Postfix(Box::new(Expr::ident(left)), "--")));
						let x = self.read(a, true, used, done, out);
						(x, self.read(b, true, used, done, out))
					}
					Zipped::Next { drop } => {
						// The first side's item drops where the second has ended.
						let x = self.read(a, false, used || drop.is_some(), done, out);
						let mut second = Vec::new();
						if let (Some(drop), Some(x)) = (drop, &x) {
							second.push(expr(drop.clone().call(vec![x.whole()])));
						}
						second.extend(done.iter().cloned());
						(x, self.read(b, false, used, &second, out))
					}
				};
				Some(Val::Tuple(vec![x?, y?]))
			}
		}
	}

	/// Variables, declared in `out`, to hold an item shaped as `x`, of type `ty`: one for
	/// each part of a tuple.
	fn keep(&mut self, x: &Val, ty: &Type, out: &mut Vec<Stmt>) -> Val {
		match (x, ty) {
			(Val::Tuple(parts), Type::Tuple(tys)) if parts.len() == tys.len() => {
				Val::Tuple(parts.iter().zip(tys).map(|(x, ty)| self.keep(x, ty, out)).collect())
			}
			_ => {
				let y = self.b.fresh("y");
				out.push(
					StmtKind::Declare {
						name: y.clone(),
						ty: ty.clone(),
					}
					.at(None),
				);
				Val::One(Expr::ident(y))
			}
		}
	}
}

/// The upper bound of `node`'s size hint, `Infinity` where it has none, as `ZipFold` reads
/// a zip's.
fn upper(node: &Node) -> Expr {
	let min = |a, b| Expr::ident("Math").member("min").call(vec![a, b]);
	match node {
		Node::Leaf { var, .. } => {
			let hint = Expr::ident(var).member("sizeHint").call(Vec::new());
			hint.index(Expr::lit("1")).binary("??", Expr::ident("Infinity"))
		}
		Node::Map { inner, .. } | Node::Filter { inner, .. } | Node::Enumerate { inner, .. } => upper(inner),
		Node::Zip {
			how: Zipped::Indexed { index, end },
			..
		} => Expr::ident(end).binary("-", Expr::ident(index)),
		Node::Zip {
			how: Zipped::Counted { left },
			..
		} => Expr::ident(left),
		Node::Zip { a, b, .. } => min(upper(a), upper(b)),
	}
}

/// Whether `e` makes a leaf: an iterator whose `folded`, `indexed`, and `find` are the
/// runtime's defaults, so each reads it by `next`, as a slice's, a `Vec`'s, a range's, or
/// the `rev` of one is.
fn leaf(e: &Expr) -> bool {
	let rs = |f: &Expr, name: &str| matches!(peel(f), Expr::Rs(n) if n == name);
	match peel(e) {
		Expr::Call(f, args) => {
			(rs(f, "iterSlice") || rs(f, "vecIntoIter")) && matches!(args.len(), 1 | 2)
				|| rs(f, "byteVecIntoIter") && args.len() == 1
				|| rs(f, "iterRev") && args.len() == 1 && leaf(&args[0])
		}
		Expr::New(f, args) => rs(f, "Range") && matches!(args.len(), 2 | 3),
		_ => false,
	}
}

/// The type of what `node` yields.
fn item_of(node: &Node) -> Type {
	match node {
		Node::Leaf { item, .. }
		| Node::Map { item, .. }
		| Node::Filter { item, .. }
		| Node::Enumerate { item, .. }
		| Node::Zip { item, .. } => item.clone(),
	}
}

/// What drops an `item`.
fn dropper(item: &Type) -> Type {
	Type::Func(vec![item.clone()], Box::new(Type::Builtin("void")))
}

/// `if (x === rs.DONE) { done }`.
fn ended(x: &str, done: &[Stmt]) -> Stmt {
	StmtKind::If {
		cond: Expr::ident(x).binary("===", Expr::Rs("DONE".into())),
		then: done.to_vec(),
		els: Vec::new(),
	}
	.at(None)
}

/// The parameters and body of `e`, an arrow whose body is one expression that makes no
/// function of its own and assigns no parameter, which a call can be replaced by.
/// `unread` takes one of one statement, as a call whose value nothing reads may be.
fn inlinable(e: &Expr, unread: bool) -> Option<(&[crate::ts::Param], &Expr)> {
	let (params, body) = match e {
		Expr::Lambda(params, body) => (params.as_slice(), &**body),
		Expr::Arrow(params, ret, body) => match body.as_slice() {
			[
				Stmt {
					kind: StmtKind::Return(Some(body)),
					..
				},
			] => (params.as_slice(), body),
			[
				Stmt {
					kind: StmtKind::Expr(body),
					..
				},
			] if unread && matches!(ret, Type::Builtin("undefined" | "void")) => (params.as_slice(), body),
			_ => return None,
		},
		_ => return None,
	};
	// One that assigns a parameter, or a part of one, has its argument to assign.
	let mut makes = false;
	walk(body, &mut |e| match e {
		Expr::Arrow(..) | Expr::Lambda(..) => makes = true,
		Expr::Assign(target, _) | Expr::Postfix(target, _) => {
			makes |= params.iter().any(|p| path(target, &p.name).is_some())
		}
		_ => {}
	});
	(!makes).then_some((params, body))
}

/// `f` of `args`: its body with each parameter replaced by its argument, where it has an
/// arrow and the arguments are variables or literals, none of whose tuples it would make
/// twice; else a call. `unread` where nothing reads its value.
fn apply(f: &Fun, args: Vec<Val>, unread: bool) -> Expr {
	if let Some((params, body)) = f.arrow.as_ref().and_then(|e| inlinable(e, unread))
		&& params.len() == args.len()
		&& args.iter().all(Val::pure)
	{
		let mut body = body.clone();
		let mut made = 0;
		for (param, arg) in params.iter().zip(&args) {
			let mut tuples = 0;
			put(&mut body, &param.name, arg, &mut tuples);
			made = made.max(tuples);
		}
		if made <= 1 {
			return body;
		}
	}
	f.var.clone().call(args.iter().map(Val::whole).collect())
}

/// `e` with each read of `name`, or of a part of it by a constant index, replaced by that
/// part of `arg`, counting in `tuples` each tuple it makes whole.
fn put(e: &mut Expr, name: &str, arg: &Val, tuples: &mut usize) {
	if let Some(path) = path(e, name) {
		let (mut at, mut rest) = (arg, path.as_slice());
		while let (Val::Tuple(parts), [i, tail @ ..]) = (at, rest)
			&& let Some(part) = parts.get(*i)
		{
			(at, rest) = (part, tail);
		}
		if matches!(at, Val::Tuple(_)) {
			*tuples += 1;
		}
		*e = (rest.iter()).fold(at.whole(), |e, i| e.index(Expr::lit(i.to_string())));
		return;
	}
	children(e, &mut |e| put(e, name, arg, tuples));
}

/// The constant indices `e` reads `name` at: none for `name` itself.
fn path(e: &Expr, name: &str) -> Option<Vec<usize>> {
	match e {
		Expr::Ident(n) => (n == name).then(Vec::new),
		Expr::Located(_, e) => path(e, name),
		Expr::Index(base, i) => {
			let Expr::Lit(i) = i.unlocated() else {
				return None;
			};
			let mut at = path(base, name)?;
			at.push(i.parse().ok()?);
			Some(at)
		}
		_ => None,
	}
}

/// Each statement that sets `held` to `x`, part by part.
fn assign(held: &Val, x: &Val, out: &mut Vec<Stmt>) {
	match (held, x) {
		(Val::Tuple(held), Val::Tuple(x)) => held.iter().zip(x).for_each(|(h, x)| assign(h, x, out)),
		(held, x) => out.push(StmtKind::Expr(held.whole().assign(x.whole())).at(None)),
	}
}

/// `stmts` without the variables of their own nothing reads, the values those hold still
/// computed where computing them does something: an inlined closure may skip part of an
/// item. `result` is what reads them after.
fn prune(stmts: &mut Vec<Stmt>, result: &Expr) {
	loop {
		let mut reads = FxHashMap::default();
		walk(result, &mut |e| count(e, &mut reads));
		stmts.iter().for_each(|s| walk_stmt(s, &mut |e| count(e, &mut reads)));
		let mut gone = FxHashSet::default();
		unread(stmts, &reads, &mut gone);
		if gone.is_empty() {
			return;
		}
		unset(stmts, &gone);
	}
}

/// Each variable `e` names, but as what an assignment sets.
fn count(e: &Expr, reads: &mut FxHashMap<String, isize>) {
	if let Expr::Ident(n) = e {
		*reads.entry(n.clone()).or_default() += 1;
	}
	if let Expr::Assign(target, _) = e
		&& let Expr::Ident(n) = target.unlocated()
	{
		*reads.entry(n.clone()).or_default() -= 1;
	}
}

/// `stmts` without each `const` and `let name!: T` that nothing `reads`, the names of the
/// latter added to `gone`.
fn unread(stmts: &mut Vec<Stmt>, reads: &FxHashMap<String, isize>, gone: &mut FxHashSet<String>) {
	let unread_name = |name: &String| reads.get(name).is_none_or(|&n| n <= 0);
	stmts.retain_mut(|stmt| match &mut stmt.kind {
		StmtKind::Decl {
			mutable: false,
			name,
			init: Some(init),
			..
		} if unread_name(name) => {
			if init.pure() || matches!(init.unlocated(), Expr::Arrow(..) | Expr::Lambda(..)) {
				gone.insert(name.clone());
				return false;
			}
			stmt.kind = StmtKind::Expr(init.clone());
			true
		}
		StmtKind::Declare { name, .. } if unread_name(name) => {
			gone.insert(name.clone());
			false
		}
		StmtKind::If { then, els, .. } => {
			unread(then, reads, gone);
			unread(els, reads, gone);
			true
		}
		StmtKind::While { body, .. } | StmtKind::Block { body, .. } | StmtKind::ForOf { body, .. } => {
			unread(body, reads, gone);
			true
		}
		_ => true,
	});
}

/// `stmts` without each assignment to a variable in `gone`, its value still computed where
/// computing it does something.
fn unset(stmts: &mut Vec<Stmt>, gone: &FxHashSet<String>) {
	stmts.retain_mut(|stmt| match &mut stmt.kind {
		StmtKind::Expr(Expr::Assign(target, value)) if matches!(target.unlocated(), Expr::Ident(n) if gone.contains(n)) =>
		{
			if value.pure() {
				return false;
			}
			stmt.kind = StmtKind::Expr((**value).clone());
			true
		}
		StmtKind::If { then, els, .. } => {
			unset(then, gone);
			unset(els, gone);
			true
		}
		StmtKind::While { body, .. } | StmtKind::Block { body, .. } | StmtKind::ForOf { body, .. } => {
			unset(body, gone);
			true
		}
		_ => true,
	});
}

/// `f` of `e` and of each expression in it, those of the functions it makes included.
fn walk(e: &Expr, f: &mut impl FnMut(&Expr)) {
	f(e);
	match e {
		Expr::Arrow(_, _, body) => body.iter().for_each(|s| walk_stmt(s, f)),
		Expr::Lambda(_, e)
		| Expr::Member(e, _)
		| Expr::Unary(_, e)
		| Expr::Postfix(e, _)
		| Expr::As(e, _)
		| Expr::Instantiate(e, _)
		| Expr::Located(_, e) => walk(e, f),
		Expr::Array(es) => es.iter().for_each(|e| walk(e, f)),
		Expr::Object(props) => props.iter().for_each(|(_, e)| walk(e, f)),
		Expr::Index(a, b) | Expr::Binary(a, _, b) | Expr::Assign(a, b) => {
			walk(a, f);
			walk(b, f);
		}
		Expr::Call(callee, args) | Expr::New(callee, args) => {
			walk(callee, f);
			args.iter().for_each(|e| walk(e, f));
		}
		Expr::Cond(a, b, c) => {
			walk(a, f);
			walk(b, f);
			walk(c, f);
		}
		Expr::Ident(_) | Expr::This | Expr::Sym(_) | Expr::Rs(_) | Expr::Lit(_) | Expr::Str(_) => {}
	}
}

/// [`walk`] of each expression in `stmt`.
fn walk_stmt(stmt: &Stmt, f: &mut impl FnMut(&Expr)) {
	match &stmt.kind {
		StmtKind::Expr(e) | StmtKind::Return(Some(e)) => walk(e, f),
		StmtKind::Decl { init, .. } => init.iter().for_each(|e| walk(e, f)),
		StmtKind::If { cond, then, els } => {
			walk(cond, f);
			then.iter().chain(els).for_each(|s| walk_stmt(s, f));
		}
		StmtKind::While { cond: e, body, .. } | StmtKind::ForOf { iter: e, body, .. } => {
			walk(e, f);
			body.iter().for_each(|s| walk_stmt(s, f));
		}
		StmtKind::Block { body, .. } => body.iter().for_each(|s| walk_stmt(s, f)),
		StmtKind::Switch {
			disc, cases, default, ..
		} => {
			walk(disc, f);
			for (case, body) in cases {
				walk(case, f);
				body.iter().for_each(|s| walk_stmt(s, f));
			}
			default.iter().for_each(|s| walk_stmt(s, f));
		}
		StmtKind::Declare { .. } | StmtKind::Return(None) | StmtKind::Break(_) | StmtKind::Continue(_) => {}
	}
}

/// `f` of each expression directly in `e`, not in a function it makes.
fn children(e: &mut Expr, f: &mut impl FnMut(&mut Expr)) {
	match e {
		Expr::Member(e, _)
		| Expr::Unary(_, e)
		| Expr::Postfix(e, _)
		| Expr::As(e, _)
		| Expr::Instantiate(e, _)
		| Expr::Located(_, e) => f(e),
		Expr::Array(es) => es.iter_mut().for_each(f),
		Expr::Object(props) => props.iter_mut().for_each(|(_, e)| f(e)),
		Expr::Index(a, b) | Expr::Binary(a, _, b) | Expr::Assign(a, b) => {
			f(a);
			f(b);
		}
		Expr::Call(callee, args) | Expr::New(callee, args) => {
			f(callee);
			args.iter_mut().for_each(f);
		}
		Expr::Cond(a, b, c) => {
			f(a);
			f(b);
			f(c);
		}
		Expr::Arrow(..)
		| Expr::Lambda(..)
		| Expr::Ident(_)
		| Expr::This
		| Expr::Sym(_)
		| Expr::Rs(_)
		| Expr::Lit(_)
		| Expr::Str(_) => {}
	}
}

/// Each `for` loop's read of a fused chain, `reads` naming what each `next` assigns,
/// written into the loop: where the chain ends, the loop leaves as its test of the item
/// would, and the loop reads the item's parts where it reads the item's fields, making
/// the tuple only where something reads it whole.
pub(super) fn loop_parts(stmts: &mut Vec<Stmt>, reads: &FxHashSet<String>) {
	if !reads.is_empty() {
		parts_in(stmts, reads, Jump::Other);
	}
}

/// What an unlabeled `break` in a list of statements leaves: a loop, by its label, which
/// one moved deeper names it by, or something else.
enum Jump<'a> {
	Loop(&'a mut Option<String>),
	Other,
}

impl Jump<'_> {
	fn reborrow(&mut self) -> Jump<'_> {
		match self {
			Jump::Loop(label) => Jump::Loop(label),
			Jump::Other => Jump::Other,
		}
	}
}

fn parts_in(stmts: &mut Vec<Stmt>, reads: &FxHashSet<String>, mut jump: Jump<'_>) {
	let mut k = 0;
	while k < stmts.len() {
		// What a read becomes reads no chain.
		if let Some(n) = take_apart(stmts, k, reads, &mut jump) {
			k += n;
			continue;
		}
		match &mut stmts[k].kind {
			StmtKind::If { then, els, .. } => {
				parts_in(then, reads, jump.reborrow());
				parts_in(els, reads, jump.reborrow());
			}
			StmtKind::Block { body, .. } => parts_in(body, reads, jump.reborrow()),
			StmtKind::While { label, body, .. } | StmtKind::ForOf { label, body, .. } => {
				parts_in(body, reads, Jump::Loop(label))
			}
			StmtKind::Switch { cases, default, .. } => {
				for (_, body) in cases {
					parts_in(body, reads, Jump::Other);
				}
				parts_in(default, reads, Jump::Other);
			}
			_ => {}
		}
		k += 1;
	}
}

/// Take apart the read at `stmts[k]`, if it is one: `let r!: T; label: { ... r = item; }`,
/// each end of the chain in it `r = rs.DONE; break label;`, then the loop's test, `const
/// d = r; if (d === rs.DONE) { exit }`, whose `exit` leaves. Each end runs `exit` instead,
/// and what reads `d` after reads the parts of `item`. How many statements replace them.
fn take_apart(stmts: &mut Vec<Stmt>, k: usize, reads: &FxHashSet<String>, jump: &mut Jump<'_>) -> Option<usize> {
	let StmtKind::Declare { name: r, ty: r_ty } = &stmts[k].kind else {
		return None;
	};
	if !reads.contains(r) {
		return None;
	}
	let StmtKind::Block {
		label: Some(label),
		body,
	} = &stmts.get(k + 1)?.kind
	else {
		return None;
	};
	// The local the loop's `match` reads, where it is held apart from `r`.
	let (d, ty, test) = match &stmts.get(k + 2)?.kind {
		StmtKind::Decl {
			mutable: false,
			name,
			ty,
			init: Some(init),
		} if matches!(peel(init), Expr::Ident(n) if n == r) => (name.clone(), ty.clone(), k + 3),
		_ => (r.clone(), Some(r_ty.clone()), k + 2),
	};
	let StmtKind::If { cond, then: exit, els } = &stmts.get(test)?.kind else {
		return None;
	};
	let tests = matches!(peel(cond), Expr::Binary(l, "===", done)
		if matches!(peel(l), Expr::Ident(n) if *n == d) && matches!(peel(done), Expr::Rs(x) if x == "DONE"));
	let leaves = matches!(
		exit.last().map(|s| &s.kind),
		Some(StmtKind::Break(_) | StmtKind::Continue(_) | StmtKind::Return(_))
	);
	if !tests
		|| !els.is_empty()
		|| !leaves
		|| exit.iter().any(loops)
		|| super::list_names(exit, &d) + super::list_names(exit, r) > 0
		|| jumps(exit, label) > 0
	{
		return None;
	}
	let mut body = body.clone();
	let Some(Stmt {
		kind: StmtKind::Expr(Expr::Assign(target, item)),
		..
	}) = body.pop()
	else {
		return None;
	};
	if !matches!(peel(&target), Expr::Ident(n) if n == r) {
		return None;
	}
	let item = val(&item)?;
	// Inside a loop of its own, as a filter's, `exit` names the loop it leaves.
	let (mut nested, mut n) = (false, 0);
	ends(&mut body, r, label, false, &mut |inner| {
		nested |= inner;
		n += 1;
		None
	});
	// Nothing else names `r` or `label`.
	if super::list_names(&body, r) != n || jumps(&body, label) != n {
		return None;
	}
	let unlabeled = exit.iter().any(unlabeled_jump);
	let named = match (nested && unlabeled, &*jump) {
		(false, _) => None,
		(true, Jump::Loop(own)) => Some(own.as_ref().unwrap_or(label).clone()),
		(true, Jump::Other) => return None,
	};
	let exit_in = match &named {
		Some(name) => exit.iter().map(|s| retarget(s.clone(), name)).collect(),
		None => exit.clone(),
	};
	ends(&mut body, r, label, false, &mut |inner| {
		Some(if inner { exit_in.clone() } else { exit.clone() })
	});
	let mut rest = stmts[test + 1..].to_vec();
	let (mut whole, mut bad) = (false, false);
	for stmt in &mut rest {
		stmt_exprs(stmt, &mut |e| take_parts(e, &d, &item, &mut whole, &mut bad));
	}
	if bad {
		return None;
	}
	if let (Some(name), Jump::Loop(own)) = (named, jump) {
		**own = Some(name);
	}
	if whole {
		body.push(StmtKind::decl(false, &d, ty.map(without_done), Some(item.whole())).at(None));
	}
	let n = body.len();
	stmts.truncate(k);
	stmts.extend(body);
	stmts.extend(rest);
	Some(n)
}

/// `item` held as the variables `next` made it of, if it is: a variable, or a tuple of
/// them.
fn val(item: &Expr) -> Option<Val> {
	match peel(item) {
		e @ Expr::Ident(_) => Some(Val::One(e.clone())),
		Expr::Array(parts) => parts.iter().map(val).collect::<Option<_>>().map(Val::Tuple),
		_ => None,
	}
}

/// `ty` without `rs.Done`.
fn without_done(ty: Type) -> Type {
	match ty {
		Type::Union(mut tys) => {
			tys.retain(|t| !matches!(t, Type::Rs(name, _) if *name == "Done"));
			match tys.len() {
				1 => tys.pop().unwrap(),
				_ => Type::Union(tys),
			}
		}
		ty => ty,
	}
}

/// Call `f` on each end of a read in `stmts`, `r = rs.DONE; break label;` ending a list,
/// with whether it is in a loop or `switch` of its own, replacing the two by what it gives.
fn ends(stmts: &mut Vec<Stmt>, r: &str, label: &str, inner: bool, f: &mut dyn FnMut(bool) -> Option<Vec<Stmt>>) {
	let end = matches!(stmts.as_slice(), [.., set, Stmt { kind: StmtKind::Break(Some(to)), .. }]
		if to == label && matches!(&set.kind, StmtKind::Expr(Expr::Assign(t, v))
			if matches!(peel(t), Expr::Ident(n) if n == r) && matches!(peel(v), Expr::Rs(x) if x == "DONE")));
	if end && let Some(with) = f(inner) {
		stmts.truncate(stmts.len() - 2);
		stmts.extend(with);
	}
	for stmt in stmts.iter_mut() {
		match &mut stmt.kind {
			StmtKind::If { then, els, .. } => {
				ends(then, r, label, inner, f);
				ends(els, r, label, inner, f);
			}
			StmtKind::Block { body, .. } => ends(body, r, label, inner, f),
			StmtKind::While { body, .. } | StmtKind::ForOf { body, .. } => ends(body, r, label, true, f),
			StmtKind::Switch { cases, default, .. } => {
				cases.iter_mut().for_each(|(_, body)| ends(body, r, label, true, f));
				ends(default, r, label, true, f);
			}
			_ => {}
		}
	}
}

/// Whether `stmt` holds a loop or a `switch`.
fn loops(stmt: &Stmt) -> bool {
	match &stmt.kind {
		StmtKind::While { .. } | StmtKind::ForOf { .. } | StmtKind::Switch { .. } => true,
		StmtKind::If { then, els, .. } => then.iter().chain(els).any(loops),
		StmtKind::Block { body, .. } => body.iter().any(loops),
		_ => false,
	}
}

/// Whether `stmt`, which holds no loop, has a `break` or `continue` without a label.
fn unlabeled_jump(stmt: &Stmt) -> bool {
	match &stmt.kind {
		StmtKind::Break(None) | StmtKind::Continue(None) => true,
		StmtKind::If { then, els, .. } => then.iter().chain(els).any(unlabeled_jump),
		StmtKind::Block { body, .. } => body.iter().any(unlabeled_jump),
		_ => false,
	}
}

/// `stmt`, which holds no loop, with each `break` and `continue` without a label naming
/// `label`.
fn retarget(mut stmt: Stmt, label: &str) -> Stmt {
	match &mut stmt.kind {
		StmtKind::Break(to @ None) | StmtKind::Continue(to @ None) => *to = Some(label.to_string()),
		StmtKind::If { then, els, .. } => {
			for s in then.iter_mut().chain(els.iter_mut()) {
				*s = retarget(s.clone(), label);
			}
		}
		StmtKind::Block { body, .. } => {
			for s in body.iter_mut() {
				*s = retarget(s.clone(), label);
			}
		}
		_ => {}
	}
	stmt
}

/// How many `break`s and `continue`s in `stmts` name `label`.
fn jumps(stmts: &[Stmt], label: &str) -> usize {
	stmts
		.iter()
		.map(|stmt| match &stmt.kind {
			StmtKind::Break(Some(to)) | StmtKind::Continue(Some(to)) => (to == label) as usize,
			StmtKind::If { then, els, .. } => jumps(then, label) + jumps(els, label),
			StmtKind::Block { body, .. } | StmtKind::While { body, .. } | StmtKind::ForOf { body, .. } => {
				jumps(body, label)
			}
			StmtKind::Switch { cases, default, .. } => {
				cases.iter().map(|(_, body)| jumps(body, label)).sum::<usize>() + jumps(default, label)
			}
			_ => 0,
		})
		.sum()
}

/// `e` with each read of `d`, or of a part of it by constant indices, replaced by that part
/// of `item`; `whole` where one reads a tuple of it, which stays a read of `d`, and `bad`
/// where something assigns `d` or a part of it, or names another variable `d`.
fn take_parts(e: &mut Expr, d: &str, item: &Val, whole: &mut bool, bad: &mut bool) {
	if let Some(at) = path(e, d) {
		let (mut part, mut rest) = (item, at.as_slice());
		while let (Val::Tuple(parts), [i, tail @ ..]) = (part, rest)
			&& let Some(p) = parts.get(*i)
		{
			(part, rest) = (p, tail);
		}
		match part {
			Val::One(v) => *e = rest.iter().fold(v.clone(), |e, i| e.index(Expr::lit(i.to_string()))),
			Val::Tuple(_) => *whole = true,
		}
		return;
	}
	match e {
		Expr::Assign(target, _) | Expr::Postfix(target, _) if path(target, d).is_some() => *bad = true,
		Expr::Arrow(params, _, body) => {
			*bad |= params.iter().any(|p| p.name == d);
			for stmt in body {
				stmt_exprs(stmt, &mut |e| take_parts(e, d, item, whole, bad));
			}
		}
		Expr::Lambda(params, body) => {
			*bad |= params.iter().any(|p| p.name == d);
			take_parts(body, d, item, whole, bad);
		}
		e => children(e, &mut |e| take_parts(e, d, item, whole, bad)),
	}
}

/// `f` of each expression `stmt` holds, and of those in its blocks, not in a function it
/// makes.
fn stmt_exprs(stmt: &mut Stmt, f: &mut dyn FnMut(&mut Expr)) {
	match &mut stmt.kind {
		StmtKind::Expr(e) | StmtKind::Return(Some(e)) | StmtKind::Decl { init: Some(e), .. } => f(e),
		StmtKind::If { cond, then, els } => {
			f(cond);
			then.iter_mut().chain(els.iter_mut()).for_each(|s| stmt_exprs(s, f));
		}
		StmtKind::While { cond: e, body, .. } | StmtKind::ForOf { iter: e, body, .. } => {
			f(e);
			body.iter_mut().for_each(|s| stmt_exprs(s, f));
		}
		StmtKind::Block { body, .. } => body.iter_mut().for_each(|s| stmt_exprs(s, f)),
		StmtKind::Switch {
			disc, cases, default, ..
		} => {
			f(disc);
			for (value, body) in cases {
				f(value);
				body.iter_mut().for_each(|s| stmt_exprs(s, f));
			}
			default.iter_mut().for_each(|s| stmt_exprs(s, f));
		}
		StmtKind::Return(None)
		| StmtKind::Decl { init: None, .. }
		| StmtKind::Declare { .. }
		| StmtKind::Break(_)
		| StmtKind::Continue(_) => {}
	}
}

/// Each call of a fused chain's closure whose body is one expression, `funs` naming what
/// holds each, written in place as a consumer's loop writes it. A `for` loop reads the
/// chain apart from where it is made, so only where each variable the closure names is
/// still the one it names there: nothing between declares another of its name.
pub(super) fn inline_funs(stmts: &mut Vec<Stmt>, funs: &FxHashSet<String>) {
	if funs.is_empty() {
		return;
	}
	let mut k = 0;
	while k < stmts.len() {
		if let Some((name, arrow, declared)) = made_fun(&stmts[k], funs)
			&& let Some((params, body)) = inlinable(&arrow, true)
		{
			let mut free = FxHashSet::default();
			walk(body, &mut |e| {
				if let Expr::Ident(n) = e {
					free.insert(n.clone());
				}
			});
			params.iter().for_each(|p| {
				free.remove(&p.name);
			});
			let mut names = FxHashSet::default();
			stmts[k + 1..].iter().for_each(|s| declares(s, &mut names));
			if free.is_disjoint(&names) {
				let fun = Fun {
					var: Expr::ident(&name),
					arrow: Some(arrow.clone()),
				};
				for stmt in &mut stmts[k + 1..] {
					calls(stmt, &name, &fun);
				}
				if declared && super::list_names(&stmts[k + 1..], &name) == 0 {
					stmts.remove(k);
					continue;
				}
			}
		}
		match &mut stmts[k].kind {
			StmtKind::If { then, els, .. } => {
				inline_funs(then, funs);
				inline_funs(els, funs);
			}
			StmtKind::Block { body, .. } | StmtKind::While { body, .. } | StmtKind::ForOf { body, .. } => {
				inline_funs(body, funs)
			}
			StmtKind::Switch { cases, default, .. } => {
				cases.iter_mut().for_each(|(_, body)| inline_funs(body, funs));
				inline_funs(default, funs);
			}
			_ => {}
		}
		k += 1;
	}
}

/// The closure `stmt` makes one of `funs` hold, and whether it declares it.
fn made_fun(stmt: &Stmt, funs: &FxHashSet<String>) -> Option<(String, Expr, bool)> {
	let (name, value, declared) = match &stmt.kind {
		StmtKind::Decl {
			mutable: false,
			name,
			init: Some(init),
			..
		} => (name.as_str(), init, true),
		StmtKind::Expr(Expr::Assign(target, value)) => match peel(target) {
			Expr::Ident(name) => (name.as_str(), &**value, false),
			_ => return None,
		},
		_ => return None,
	};
	let value = peel(value);
	(funs.contains(name) && matches!(value, Expr::Arrow(..) | Expr::Lambda(..)))
		.then(|| (name.to_string(), value.clone(), declared))
}

/// Each name `stmt` declares, those of the parameters of the functions it makes included.
fn declares(stmt: &Stmt, names: &mut FxHashSet<String>) {
	match &stmt.kind {
		StmtKind::Decl { name, .. } | StmtKind::Declare { name, .. } => {
			names.insert(name.clone());
		}
		StmtKind::ForOf { name, body, .. } => {
			names.insert(name.clone());
			body.iter().for_each(|s| declares(s, names));
		}
		StmtKind::If { then, els, .. } => then.iter().chain(els).for_each(|s| declares(s, names)),
		StmtKind::Block { body, .. } | StmtKind::While { body, .. } => body.iter().for_each(|s| declares(s, names)),
		StmtKind::Switch { cases, default, .. } => cases
			.iter()
			.flat_map(|(_, b)| b)
			.chain(default)
			.for_each(|s| declares(s, names)),
		_ => {}
	}
	walk_stmt(stmt, &mut |e| match e {
		Expr::Arrow(params, ..) | Expr::Lambda(params, _) => names.extend(params.iter().map(|p| p.name.clone())),
		_ => {}
	});
}

/// `stmt` with each call of `name`, not in a function it makes, written as `fun`'s body
/// where [`apply`] can.
fn calls(stmt: &mut Stmt, name: &str, fun: &Fun) {
	if let StmtKind::Expr(e) = &mut stmt.kind
		&& let Some(body) = call_of(e, name, fun, true)
	{
		*e = body;
		return;
	}
	stmt_exprs(stmt, &mut |e| call_in(e, name, fun));
}

fn call_in(e: &mut Expr, name: &str, fun: &Fun) {
	children(e, &mut |e| call_in(e, name, fun));
	if let Some(body) = call_of(e, name, fun, false) {
		*e = body;
	}
}

/// `fun`'s body in place of `e`, if `e` calls `name` and [`apply`] writes it in place.
fn call_of(e: &Expr, name: &str, fun: &Fun, unread: bool) -> Option<Expr> {
	let Expr::Call(f, args) = peel(e) else {
		return None;
	};
	if !matches!(peel(f), Expr::Ident(n) if n == name) {
		return None;
	}
	let body = apply(fun, args.iter().map(arg).collect(), unread);
	(!matches!(&body, Expr::Call(f, _) if matches!(peel(f), Expr::Ident(n) if n == name))).then_some(body)
}

/// An argument as a value: a tuple made in place as its parts.
fn arg(e: &Expr) -> Val {
	match peel(e) {
		Expr::Array(parts) => Val::Tuple(parts.iter().map(arg).collect()),
		_ => Val::One(e.clone()),
	}
}
