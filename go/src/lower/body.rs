//! One function instance's MIR as a Go function: each local a variable, but a temporary
//! folded into its reader (`fold`) and a large enum a variable per part (`split`), each
//! declared where it is used, and its blocks structured into loops, `if`s, and blocks by
//! [`cfg`], as rust2-ts structures them, so a small function stays within Go's inlining
//! budget. A panic is Go's, with native Rust's message, and runs no cleanup, as
//! `panic = "abort"` doesn't: the MIR's cleanup blocks are left out.

use std::collections::{HashMap, HashSet};
use std::fmt::Write;
use std::rc::Rc;

use rust2_driver::cfg;
use rustc_hir::def::DefKind;
use rustc_middle::mir::visit::{MutatingUseContext, NonMutatingUseContext, PlaceContext, Visitor};
use rustc_middle::mir::{
	self, AggregateKind, AssertKind, BasicBlock, BinOp, CastKind, Local, Location, Operand, Place, PlaceElem, Rvalue,
	StatementKind, TerminatorKind, UnOp,
};
use rustc_middle::ty::adjustment::PointerCoercion;
use rustc_middle::ty::util::IntTypeExt;
use rustc_middle::ty::{self, EarlyBinder, Instance, InstanceKind, Ty, TyCtxt};
use rustc_span::Span;

use super::Translator;
use super::fold::{self, Does, Effect, Held, Pending, Temps};
use super::library::{self, Mapped};
use super::names;
use super::split::Split;
use super::stmt::{self, Cond, Stmt};
use super::ty::variant_field;

/// How a block leaves.
type Exit = cfg::Exit<Cond>;

/// Translate one function instance, or `None` after reporting what it refuses.
pub fn function<'tcx>(t: &mut Translator<'tcx>, instance: Instance<'tcx>, reached: Span) -> Option<String> {
	let mir = instance_mir(t, instance, reached)?;
	let name = t.funcs[&instance].clone();
	let mut fx = Fx::new(t, &mir, reached, instance.def_id().is_local());
	let text = fx.function(&name, instance);
	fx.ok.then_some(text)
}

/// The locals of enums Go holds split, as their parts: those never borrowed whole nor
/// reached but through a variant's field, but for a temporary that folds into its reader,
/// which a call does not assign.
fn split_locals<'tcx>(
	t: &mut Translator<'tcx>,
	mir: &mir::Body<'tcx>,
	temps: &Temps,
) -> HashMap<Local, Rc<Split<'tcx>>> {
	struct Reach<'a, 'tcx> {
		split: &'a mut HashMap<Local, Rc<Split<'tcx>>>,
		args: usize,
	}
	impl<'tcx> Visitor<'tcx> for Reach<'_, 'tcx> {
		fn visit_place(&mut self, place: &Place<'tcx>, ctx: PlaceContext, _: Location) {
			let borrow = matches!(
				ctx,
				PlaceContext::NonMutatingUse(
					NonMutatingUseContext::SharedBorrow
						| NonMutatingUseContext::FakeBorrow
						| NonMutatingUseContext::RawBorrow
				) | PlaceContext::MutatingUse(MutatingUseContext::Borrow | MutatingUseContext::RawBorrow)
			);
			let parts = match place.projection.as_slice() {
				[] => !borrow,
				[PlaceElem::Downcast(..), PlaceElem::Field(..), ..] => true,
				_ => false,
			};
			// What an argument's big parts point at is its caller's, which it only reads.
			let changed = place.local.as_usize() >= 1
				&& place.local.as_usize() <= self.args
				&& matches!(ctx, PlaceContext::MutatingUse(m) if m != MutatingUseContext::Drop);
			if !parts || changed {
				self.split.remove(&place.local);
			}
		}
	}
	let called: HashSet<Local> = (mir.basic_blocks.iter())
		.filter_map(|data| match &data.terminator().kind {
			TerminatorKind::Call { destination, .. } => destination.as_local(),
			_ => None,
		})
		.collect();
	let mut split = HashMap::new();
	for (local, decl) in mir.local_decls.iter_enumerated() {
		if temps.foldable.contains(&local) && !called.contains(&local) {
			continue;
		}
		if let Some(parts) = t.split(decl.ty, decl.source_info.span) {
			split.insert(local, parts);
		}
	}
	if let Some(spread) = mir.spread_arg {
		split.remove(&spread);
	}
	let mut reach = Reach {
		split: &mut split,
		args: mir.arg_count,
	};
	for (bb, data) in mir.basic_blocks.iter_enumerated() {
		if !data.is_cleanup {
			reach.visit_basic_block_data(bb, data);
		}
	}
	split
}

/// The MIR of `instance`, with its generic arguments in place.
fn instance_mir<'tcx>(t: &mut Translator<'tcx>, instance: Instance<'tcx>, reached: Span) -> Option<mir::Body<'tcx>> {
	let tcx = t.tcx;
	let def_id = instance.def_id();
	let body = match instance.def {
		InstanceKind::Item(_) if !tcx.is_mir_available(def_id) => {
			t.err(
				reached,
				format!("calls `{}`, which has no MIR and no binding", tcx.def_path_str(def_id)),
			);
			return None;
		}
		InstanceKind::Item(_) if tcx.def_kind(def_id) == DefKind::Const => tcx.mir_for_ctfe(def_id).clone(),
		_ => tcx.instance_mir(instance.def).clone(),
	};
	match instance.try_instantiate_mir_and_normalize_erasing_regions(tcx, t.env(), EarlyBinder::bind(tcx, body)) {
		Ok(body) => Some(body),
		Err(_) => {
			t.err(reached, format!("`{instance}` does not normalize"));
			None
		}
	}
}

pub struct Fx<'a, 'tcx> {
	pub t: &'a mut Translator<'tcx>,
	pub tcx: TyCtxt<'tcx>,
	mir: &'a mir::Body<'tcx>,
	/// What first reached this function, which a refusal in another crate's points at too.
	reached: Span,
	/// Whether this is the translated crate's own function.
	local: bool,
	/// Locals the body reads, which Go requires of every variable.
	used: HashSet<Local>,
	/// Locals the body names at all, which are the ones it declares.
	mentioned: HashSet<Local>,
	/// Where each `tracing` event in the body is invoked, which translation leaves out.
	sites: Vec<Span>,
	temps: Temps,
	held: Held,
	/// The locals Go holds as their parts.
	split: HashMap<Local, Rc<Split<'tcx>>>,
	/// The parts of split locals the body reads.
	part_used: HashSet<(Local, usize)>,
	/// The variables a whole value is spread from, and their Go types.
	scratch: Vec<(String, String)>,
	/// Whether the operand being lowered may fold in a temporary: false where Go would
	/// evaluate it twice, late, or not at all.
	fold_ok: bool,
	/// Whether the terminator being translated goes on to a block only it goes to, which
	/// a call's result can wait into.
	carry: bool,
	ok: bool,
}

/// A place as Go reads and writes it, and its Rust type.
pub struct GoPlace<'tcx> {
	pub expr: String,
	pub ty: Ty<'tcx>,
}

impl<'a, 'tcx> Fx<'a, 'tcx> {
	/// Lowering in `mir`, reached from `reached`, of the translated crate if `local`. An
	/// item outside any body, such as a constant's value, is lowered in an empty one.
	pub fn new(t: &'a mut Translator<'tcx>, mir: &'a mir::Body<'tcx>, reached: Span, local: bool) -> Self {
		let mut temps = Temps::new(mir);
		let split = split_locals(t, mir, &temps);
		temps.foldable.retain(|l| !split.contains_key(l));
		Self {
			tcx: t.tcx,
			t,
			mir,
			reached,
			local,
			used: HashSet::new(),
			mentioned: HashSet::new(),
			sites: Vec::new(),
			temps,
			held: Held::default(),
			split,
			part_used: HashSet::new(),
			scratch: Vec::new(),
			fold_ok: true,
			carry: false,
			ok: true,
		}
	}

	/// Report a construct translation refuses, here and where it was reached from.
	pub fn err(&mut self, span: Span, msg: impl Into<String>) -> String {
		let msg = format!("{} is not supported", msg.into());
		self.ok = false;
		self.t.failed = true;
		if !self.t.reported.insert((span, msg.clone())) {
			return "struct{}{}".into();
		}
		let msg = format!("rust2-go: {msg}");
		let mut diag = self.tcx.dcx().struct_span_err(span, msg);
		if !self.local {
			diag = diag.with_span_note(self.reached, "reached from here");
		}
		diag.emit();
		"struct{}{}".into()
	}

	pub fn ty(&mut self, ty: Ty<'tcx>, span: Span) -> String {
		self.t.ty(ty, span)
	}

	fn local_name(local: Local) -> String {
		format!("_{}", local.as_usize())
	}

	fn function(&mut self, name: &str, instance: Instance<'tcx>) -> String {
		let mir = self.mir;
		let span = mir.span;
		let mut params = Vec::new();
		let mut prologue = String::new();
		for arg in mir.args_iter() {
			let ty = mir.local_decls[arg].ty;
			let name = Self::local_name(arg);
			if Some(arg) == mir.spread_arg {
				// rust-call's tuple arrives as its fields, as every caller spreads it.
				let ty::Tuple(tys) = ty.kind() else {
					self.err(span, "a spread argument that is not a tuple");
					continue;
				};
				let mut fields = Vec::new();
				for (i, field) in tys.iter().enumerate() {
					let go = self.ty(field, span);
					params.push(format!("{name}_{i} {go}"));
					fields.push(format!("{name}_{i}"));
				}
				let go = self.ty(ty, span);
				let value = match tys.is_empty() {
					true => "struct{}{}".to_string(),
					false => format!("{go}{{{}}}", fields.join(", ")),
				};
				writeln!(prologue, "\tvar {name} {go} = {value}").unwrap();
				self.used.insert(arg);
			} else if let Some(split) = self.t.split(ty, span) {
				let mut values = Vec::new();
				for p in &split.parts {
					let part = format!("{name}_{}", p.name);
					match p.big {
						true => {
							params.push(format!("{part} *{}", p.go));
							values.push(format!("(*{part})"));
						}
						false => {
							params.push(format!("{part} {}", p.go));
							values.push(part);
						}
					}
				}
				// Borrowed whole or changed, so a variable of its own, as what a big part
				// points at is its caller's.
				if !self.split.contains_key(&arg) {
					writeln!(prologue, "\tvar {name} {} = {}", split.go, split.rebuild(&values)).unwrap();
				}
			} else {
				let go = self.ty(ty, span);
				params.push(format!("{name} {go}"));
			}
		}
		let ret_ty = mir.return_ty();
		let ret = match (Translator::no_value(ret_ty), self.t.split(ret_ty, span)) {
			(true, _) => String::new(),
			(false, Some(split)) => {
				for p in split.parts.iter().filter(|p| p.big) {
					params.push(format!("_0_{} *{}", p.name, p.go));
				}
				split.results()
			}
			(false, None) => format!(" {}", self.ty(ret_ty, span)),
		};
		self.sites = self.tracing_sites();
		// Only the blocks the translation goes to, from the first: no cleanup, and no block
		// of a left-out event.
		let succs = mir.basic_blocks.indices().map(|bb| self.successors(bb)).collect();
		let mut body = cfg::structure(&cfg::Graph { succs }, self, &mut 0);
		if ret.is_empty() {
			cfg::strip_return(&mut body);
		}
		cfg::hoist_else(&mut body);
		cfg::tidy_loops(&mut body);
		let mut vars = Vec::new();
		for local in mir.local_decls.indices().skip(1 + mir.arg_count) {
			self.declare(local, &mut vars);
		}
		self.declare(mir::RETURN_PLACE, &mut vars);
		for (name, go) in &self.scratch {
			vars.push((name.clone(), format!("\tvar {name} {go}\n")));
		}
		stmt::declare(&mut body, vars);
		let mut decls = String::new();
		for arg in mir.args_iter() {
			if !self.used.contains(&arg) && !self.split.contains_key(&arg) {
				writeln!(decls, "\t_ = {}", Self::local_name(arg)).unwrap();
			}
		}
		let mut out = format!(
			"// {name} is `{instance}`.\nfunc {name}({}){ret} {{\n",
			params.join(", ")
		);
		out.push_str(&prologue);
		out.push_str(&decls);
		stmt::print(&body, &mut HashSet::new(), &mut out);
		out.push_str("}\n");
		out
	}

	/// The blocks `bb` goes on to, as [`cfg::Lower::node`] says it leaves.
	fn successors(&self, bb: BasicBlock) -> Vec<usize> {
		let term = self.mir.basic_blocks[bb].terminator();
		let succs = match self.in_site(term.source_info.span) {
			Some(site) => self.site_exit(bb, site).into_iter().collect(),
			None => match &term.kind {
				TerminatorKind::Goto { target, .. }
				| TerminatorKind::Drop { target, .. }
				| TerminatorKind::Assert { target, .. }
				| TerminatorKind::Call {
					target: Some(target), ..
				} => vec![*target],
				TerminatorKind::FalseEdge { real_target, .. } | TerminatorKind::FalseUnwind { real_target, .. } => {
					vec![*real_target]
				}
				TerminatorKind::SwitchInt { targets, .. } => {
					let (arms, default) = self.switch_targets(targets);
					arms.into_iter().map(|(_, t)| t).chain([default]).collect()
				}
				_ => Vec::new(),
			},
		};
		let mut out: Vec<usize> = Vec::new();
		for bb in succs {
			if !out.contains(&bb.as_usize()) {
				out.push(bb.as_usize());
			}
		}
		out
	}

	/// Emit the block `bb`'s statements and say how it leaves, holding back for the block
	/// it goes on to what only that block reads.
	fn block(&mut self, bb: BasicBlock, out: &mut Vec<Stmt>) -> Exit {
		let mir = self.mir;
		let data = &mir.basic_blocks[bb];
		let mut text = String::new();
		for (i, stmt) in data.statements.iter().enumerate() {
			if self.in_site(stmt.source_info.span).is_none() {
				self.statement(
					stmt,
					Location {
						block: bb,
						statement_index: i,
					},
					&mut text,
				);
			}
		}
		let term = data.terminator();
		let (exit, carried) = match self.in_site(term.source_info.span) {
			Some(site) => {
				self.begin(&fold::Access::default(), Effect::default(), true, &mut text);
				match self.site_exit(bb, site) {
					Some(exit) => (Exit::Goto(exit.as_usize()), false),
					None => {
						self.err(site, "a `tracing` event whose expansion has no single exit");
						(Exit::End, false)
					}
				}
			}
			None => {
				let to = fold::carried(mir, bb, term);
				let acc = self.temps.access(|v| v.visit_terminator(term, mir.terminator_loc(bb)));
				let effect = match term.kind {
					TerminatorKind::Call { .. } => Effect {
						writes: true,
						panics: true,
					},
					TerminatorKind::Assert { .. } => Effect {
						writes: false,
						panics: true,
					},
					_ => Effect::default(),
				};
				self.begin(&acc, effect, to.is_none(), &mut text);
				self.carry = to.is_some();
				let mut tail = String::new();
				let (exit, result) = self.terminator(term, &mut tail);
				self.carry = false;
				let mut does = self.end(&mut text);
				text.push_str(&tail);
				if let Some((local, expr)) = result {
					does.join(&acc.does());
					does.effect = true;
					self.held.pending.push(Pending { local, expr, does });
				}
				(exit, to.is_some())
			}
		};
		assert!(
			carried || self.held.pending.is_empty(),
			"a temporary held past the end of its block"
		);
		if !text.is_empty() {
			out.push(Stmt::Lines(text));
		}
		if let TerminatorKind::Return = term.kind {
			let mut text = String::new();
			let value = self.returned(&mut text);
			if !text.is_empty() {
				out.push(Stmt::Lines(text));
			}
			out.push(Stmt::Return(value));
		}
		exit
	}

	/// Where each `tracing` event the body holds is invoked: the outermost of its macros,
	/// which expand to one another. Translated code installs no subscriber, and without one
	/// an event evaluates none of its fields and does nothing, so leaving it out is exact,
	/// as it is in rust2-ts.
	fn tracing_sites(&self) -> Vec<Span> {
		let mut sites = Vec::new();
		let spans = self.mir.basic_blocks.iter().flat_map(|data| {
			data.statements
				.iter()
				.map(|s| s.source_info.span)
				.chain(std::iter::once(data.terminator().source_info.span))
		});
		for mut span in spans {
			let mut site = None;
			while span.from_expansion() {
				let data = span.ctxt().outer_expn_data();
				if let Some(def_id) = data.macro_def_id
					&& self.tcx.crate_name(def_id.krate).as_str() == "tracing"
					&& matches!(
						self.tcx.item_name(def_id).as_str(),
						"event" | "trace" | "debug" | "info" | "warn" | "error"
					) {
					site = Some(data.call_site);
				}
				span = data.call_site;
			}
			if let Some(site) = site
				&& !sites.contains(&site)
			{
				sites.push(site);
			}
		}
		sites
	}

	/// The `tracing` event whose invocation holds `span`, its arguments included.
	fn in_site(&self, span: Span) -> Option<Span> {
		let root = span.source_callsite();
		self.sites.iter().copied().find(|site| site.contains(root))
	}

	/// The one block an event's blocks, from `bb`'s on, go on to.
	fn site_exit(&self, bb: BasicBlock, site: Span) -> Option<BasicBlock> {
		let blocks = &self.mir.basic_blocks;
		let mut seen = HashSet::from([bb]);
		let mut stack = vec![bb];
		let mut exits = HashSet::new();
		while let Some(next) = stack.pop() {
			for succ in blocks[next].terminator().successors() {
				if blocks[succ].is_cleanup || !seen.insert(succ) {
					continue;
				}
				match self.in_site(blocks[succ].terminator().source_info.span) == Some(site) {
					true => stack.push(succ),
					false => {
						exits.insert(succ);
					}
				}
			}
		}
		let mut exits = exits.into_iter();
		match (exits.next(), exits.next()) {
			(Some(exit), None) => Some(exit),
			_ => None,
		}
	}

	/// Declare a local the body names, and use it if nothing reads it, as Go requires:
	/// each variable's name and its declaration.
	fn declare(&mut self, local: Local, out: &mut Vec<(String, String)>) {
		let ty = self.mir.local_decls[local].ty;
		if ty.is_never() || !self.mentioned.contains(&local) {
			return;
		}
		let name = Self::local_name(local);
		if let Some(split) = self.split.get(&local).cloned() {
			for (i, p) in split.parts.iter().enumerate() {
				// A part the function returns through a pointer is that parameter.
				if local == mir::RETURN_PLACE && p.big {
					continue;
				}
				let part = format!("{name}_{}", p.name);
				let mut decl = format!("\tvar {part} {}\n", p.go);
				if !self.part_used.contains(&(local, i)) {
					writeln!(decl, "\t_ = {part}").unwrap();
				}
				out.push((part, decl));
			}
			return;
		}
		let go = self.ty(ty, self.mir.local_decls[local].source_info.span);
		let mut decl = format!("\tvar {name} {go}\n");
		if !self.used.contains(&local) {
			writeln!(decl, "\t_ = {name}").unwrap();
		}
		out.push((name, decl));
	}

	/// Before a statement that reads and writes as `acc` and does `effect`: emit what it
	/// can't fold in nor leave held back, as [`Held::begin`] decides.
	fn begin(&mut self, acc: &fold::Access, effect: Effect, ends: bool, out: &mut String) {
		self.held.begin(acc, effect, ends, out);
		self.mentioned.extend(self.held.emitted.drain(..));
	}

	/// After the statement: emit what it read but did not fold in, before it, and give
	/// what it folded in does.
	fn end(&mut self, out: &mut String) -> Does {
		let does = self.held.end(out);
		for local in self.held.emitted.drain(..) {
			self.mentioned.insert(local);
			self.used.insert(local);
		}
		does
	}

	/// The local `place` assigns `rvalue` to, if it is a temporary to hold back. A Go
	/// constant is not, as Go would fold it into its reader's arithmetic, refusing an
	/// overflow Rust wraps, nor a trait object, which can be `nil`.
	fn holds(&self, place: Place<'tcx>, rvalue: &Rvalue<'tcx>) -> Option<Local> {
		let local = place.as_local().filter(|l| self.temps.foldable.contains(l))?;
		let constant = match rvalue {
			Rvalue::Use(Operand::Constant(_), _) => true,
			Rvalue::Discriminant(place) => {
				!matches!(place.ty(self.mir, self.tcx).ty.kind(), ty::Adt(adt, _) if adt.is_enum())
			}
			Rvalue::Cast(CastKind::PointerCoercion(PointerCoercion::Unsize, _), _, to) => {
				matches!(to.builtin_deref(true).map(|t| t.kind()), Some(ty::Dynamic(..)))
			}
			_ => false,
		};
		(!constant).then_some(local)
	}

	fn statement(&mut self, stmt: &mir::Statement<'tcx>, loc: Location, out: &mut String) {
		let span = stmt.source_info.span;
		match &stmt.kind {
			StatementKind::Assign(assign) => {
				let (place, rvalue) = &**assign;
				let acc = self.temps.access(|v| v.visit_statement(stmt, loc));
				self.begin(&acc, Effect::default(), false, out);
				let ty = place.ty(self.mir, self.tcx).ty;
				if place.projection.is_empty()
					&& let Some(split) = self.split.get(&place.local).cloned()
				{
					let (pre, values) = self.split_rvalue(&split, rvalue, ty, span);
					self.end(out);
					out.push_str(&pre);
					let parts: Vec<(String, String)> = values
						.into_iter()
						.map(|(i, value)| (self.part(place.local, i), value))
						.collect();
					assign_all(&parts, out);
					return;
				}
				let value = self.rvalue(rvalue, ty, span);
				match self.holds(*place, rvalue) {
					Some(local) => {
						let mut does = self.end(out);
						does.join(&acc.does());
						self.held.pending.push(Pending {
							local,
							expr: value,
							does,
						});
					}
					None => {
						let dest = self.dest(*place, span);
						self.end(out);
						writeln!(out, "\t{dest} = {value}").unwrap();
					}
				}
			}
			StatementKind::SetDiscriminant { place, variant_index }
				if place.projection.is_empty() && self.split.contains_key(&place.local) =>
			{
				let acc = self.temps.access(|v| v.visit_statement(stmt, loc));
				self.begin(&acc, Effect::default(), false, out);
				self.end(out);
				let ty::Adt(adt, _) = self.mir.local_decls[place.local].ty.kind() else {
					unreachable!("a split local is an enum")
				};
				let discr = self.discr(*adt, *variant_index, span);
				let tag = self.part(place.local, 0);
				writeln!(out, "\t{tag} = {discr}").unwrap();
			}
			StatementKind::SetDiscriminant { place, variant_index } => {
				let acc = self.temps.access(|v| v.visit_statement(stmt, loc));
				self.begin(&acc, Effect::default(), false, out);
				let place = self.place(**place, span);
				self.end(out);
				let ty::Adt(adt, _) = place.ty.kind() else {
					self.err(span, "a discriminant of a type that is no enum");
					return;
				};
				let discr = self.discr(*adt, *variant_index, span);
				writeln!(out, "\t{}.tag = {discr}", place.expr).unwrap();
			}
			StatementKind::StorageLive(_)
			| StatementKind::StorageDead(_)
			| StatementKind::Nop
			| StatementKind::FakeRead(_)
			| StatementKind::AscribeUserType(..)
			| StatementKind::Coverage(_)
			| StatementKind::PlaceMention(_)
			| StatementKind::ConstEvalCounter
			| StatementKind::BackwardIncompatibleDropHint { .. } => {}
			StatementKind::Intrinsic(intrinsic) => match &**intrinsic {
				mir::NonDivergingIntrinsic::Assume(_) => {}
				mir::NonDivergingIntrinsic::CopyNonOverlapping(_) => {
					self.err(span, "`copy_nonoverlapping`");
				}
			},
		}
	}

	/// A place written to: its local is no read of it.
	fn dest(&mut self, place: Place<'tcx>, span: Span) -> String {
		if let Some(split) = self.split.get(&place.local).cloned()
			&& let [PlaceElem::Downcast(_, variant), PlaceElem::Field(field, _)] = place.projection.as_slice()
		{
			return self.part(place.local, split.part(*variant, *field));
		}
		let used = self.used.contains(&place.local);
		let expr = self.place(place, span).expr;
		if place.projection.is_empty() && !used {
			self.used.remove(&place.local);
		}
		expr
	}

	/// The parts `rvalue`, of the enum `split`, assigns, each by its index, and what
	/// must be emitted first: the variant an aggregate makes, or else every part.
	fn split_rvalue(
		&mut self,
		split: &Split<'tcx>,
		rvalue: &Rvalue<'tcx>,
		ty: Ty<'tcx>,
		span: Span,
	) -> (String, Vec<(usize, String)>) {
		match rvalue {
			Rvalue::Aggregate(kind, ops) if let AggregateKind::Adt(did, variant, ..) = **kind => {
				let adt = self.tcx.adt_def(did);
				let mut values = vec![(0, self.discr(adt, variant, span))];
				for (field, op) in ops.iter_enumerated() {
					values.push((split.part(variant, field), self.operand(op, span)));
				}
				(String::new(), values)
			}
			Rvalue::Use(Operand::Copy(place) | Operand::Move(place), _) | Rvalue::CopyForDeref(place) => (
				String::new(),
				self.parts_of(*place, split, span).into_iter().enumerate().collect(),
			),
			_ => {
				let value = self.rvalue(rvalue, ty, span);
				let (scratch, pre) = self.spill(split, value);
				(pre, scratch.into_iter().enumerate().collect())
			}
		}
	}

	/// Each part of `place`, a value of the enum `split`.
	fn parts_of(&mut self, place: Place<'tcx>, split: &Split<'tcx>, span: Span) -> Vec<String> {
		if place.projection.is_empty() && self.split.contains_key(&place.local) {
			return (0..split.parts.len()).map(|i| self.read_part(place.local, i)).collect();
		}
		// Read once per part.
		let fold_ok = std::mem::replace(&mut self.fold_ok, false);
		let whole = self.place(place, span).expr;
		self.fold_ok = fold_ok;
		split.parts.iter().map(|p| format!("{whole}{}", p.path)).collect()
	}

	/// Each part of `op`, a value of the enum `split`, as an argument: a big part by a
	/// pointer the callee only reads through, to its storage where nothing else can
	/// change it while the callee runs, or else to a copy made into `out` first.
	fn arg_parts(&mut self, op: &Operand<'tcx>, split: &Split<'tcx>, span: Span, out: &mut String) -> Vec<String> {
		let (values, own) = match op {
			Operand::Copy(place) | Operand::Move(place) => (
				self.parts_of(*place, split, span),
				// As a temporary held back is, once assigned.
				!place.is_indirect() && !self.temps.borrowed.contains(&place.local),
			),
			_ => {
				let whole = self.operand(op, span);
				(
					split.parts.iter().map(|p| format!("({whole}){}", p.path)).collect(),
					false,
				)
			}
		};
		(split.parts.iter().zip(values))
			.map(|(p, value)| match (p.big, own) {
				(false, _) => value,
				(true, true) => format!("&{value}"),
				(true, false) => {
					let name = format!("_s{}", self.scratch.len());
					self.scratch.push((name.clone(), p.go.clone()));
					writeln!(out, "\t{name} = {value}").unwrap();
					format!("&{name}")
				}
			})
			.collect()
	}

	/// `value`, a whole value of the enum `split`, assigned to a scratch variable: its
	/// parts, and the assignment.
	fn spill(&mut self, split: &Split<'tcx>, value: String) -> (Vec<String>, String) {
		let name = format!("_s{}", self.scratch.len());
		self.scratch.push((name.clone(), split.go.clone()));
		let parts = split.parts.iter().map(|p| format!("{name}{}", p.path)).collect();
		(parts, format!("\t{name} = {value}\n"))
	}

	/// The parts of the enum `split` a call returning it into `place` assigns, and the
	/// pointers it writes the rest through.
	fn dest_parts(&mut self, place: Place<'tcx>, split: &Split<'tcx>, span: Span) -> (Vec<String>, Vec<String>) {
		let (mut lhs, mut outs) = (Vec::new(), Vec::new());
		if place.projection.is_empty() && self.split.contains_key(&place.local) {
			for (i, p) in split.parts.iter().enumerate() {
				match p.big {
					false => lhs.push(self.part(place.local, i)),
					// The return place's is its caller's storage, handed on.
					true => outs.push(format!("&{}", self.read_part(place.local, i))),
				}
			}
			return (lhs, outs);
		}
		let fold_ok = std::mem::replace(&mut self.fold_ok, false);
		let whole = self.place(place, span).expr;
		self.fold_ok = fold_ok;
		for p in &split.parts {
			match p.big {
				false => lhs.push(format!("{whole}{}", p.path)),
				true => outs.push(format!("&{whole}{}", p.path)),
			}
		}
		(lhs, outs)
	}

	/// A place's Go expression.
	pub fn place(&mut self, place: Place<'tcx>, span: Span) -> GoPlace<'tcx> {
		let local = place.local;
		let ty = self.mir.local_decls[local].ty;
		let (mut expr, mut pty, skip) = match self.split.get(&local).cloned() {
			Some(split) => match place.projection.as_slice() {
				[] => {
					let values: Vec<String> = (0..split.parts.len()).map(|i| self.read_part(local, i)).collect();
					return GoPlace {
						expr: split.rebuild(&values),
						ty,
					};
				}
				[PlaceElem::Downcast(_, variant), PlaceElem::Field(field, _), ..] => {
					let i = split.part(*variant, *field);
					(self.read_part(local, i), mir::PlaceTy::from_ty(split.parts[i].ty), 2)
				}
				_ => unreachable!("a split local is reached through its variants' fields"),
			},
			None => {
				let expr = match self.held.take(local, self.fold_ok) {
					fold::Take::Folded(expr) => expr,
					// Read as its variable once the statement has assigned it.
					fold::Take::Refused => Self::local_name(local),
					fold::Take::Var => {
						self.used.insert(local);
						self.mentioned.insert(local);
						Self::local_name(local)
					}
				};
				(expr, mir::PlaceTy::from_ty(ty), 0)
			}
		};
		for elem in place.projection.iter().skip(skip) {
			let base = pty.ty;
			match elem {
				PlaceElem::Deref => match base.kind() {
					ty::Ref(_, inner, _) | ty::RawPtr(inner, _) => {
						// A slice, a `str`, or a trait object is its fat pointer.
						if !matches!(inner.kind(), ty::Slice(_) | ty::Str | ty::Dynamic(..)) {
							expr = format!("(*{expr})");
						}
					}
					_ => {
						return GoPlace {
							expr: self.err(span, format!("a dereference of `{base}`")),
							ty: base,
						};
					}
				},
				PlaceElem::Field(field, _) => match base.kind() {
					// What a box and `MaybeUninit` hold is where they are.
					ty::Adt(adt, _)
						if matches!(
							library::mapped(self.tcx, adt.did()),
							Some(Mapped::Box | Mapped::Transparent)
						) && field.as_usize() == usize::from(adt.is_union()) => {}
					ty::Adt(adt, _) if library::mapped(self.tcx, adt.did()).is_some_and(|m| !m.is_runtime_layout()) => {
						return GoPlace {
							expr: self.err(
								span,
								format!("a field of `{base}`, which Go holds as a type of its own"),
							),
							ty: base,
						};
					}
					ty::Adt(adt, _) => {
						let variant = pty.variant_index.unwrap_or(rustc_abi::FIRST_VARIANT);
						let def = adt.variant(variant);
						let name = names::field(&def.fields[field], field.as_usize());
						expr = match adt.is_enum() {
							true => format!("{expr}.{}.{name}", variant_field(def.name.as_str())),
							false => format!("{expr}.{name}"),
						};
					}
					ty::Tuple(_) | ty::Closure(..) => expr = format!("{expr}.F{}", field.as_usize()),
					_ => {
						return GoPlace {
							expr: self.err(span, format!("a field of `{base}`")),
							ty: base,
						};
					}
				},
				PlaceElem::Downcast(..)
				| PlaceElem::OpaqueCast(_)
				| PlaceElem::UnwrapUnsafeBinder(_)
				| PlaceElem::PhantomDeref => {}
				PlaceElem::Index(index) => {
					self.used.insert(index);
					self.mentioned.insert(index);
					expr = format!("{expr}[{}]", Self::local_name(index));
				}
				PlaceElem::ConstantIndex {
					offset,
					from_end: false,
					..
				} => expr = format!("{expr}[{offset}]"),
				PlaceElem::ConstantIndex {
					offset, from_end: true, ..
				} => expr = format!("{expr}[len({expr})-{offset}]"),
				PlaceElem::Subslice {
					from,
					to,
					from_end: true,
				} if matches!(base.kind(), ty::Slice(_)) => expr = format!("{expr}[{from}:len({expr})-{to}]"),
				// Part of an array, as an array: the place a pointer to it converted from a slice
				// of it is, which aliases the array as Rust's does.
				PlaceElem::Subslice { from, to, from_end } => {
					let ty::Array(elem, len) = base.kind() else {
						return GoPlace {
							expr: self.err(span, format!("a subslice of `{base}`")),
							ty: base,
						};
					};
					let len = len.try_to_target_usize(self.tcx).unwrap_or(0);
					let end = if from_end { len - to } else { to };
					let elem = self.ty(*elem, span);
					expr = format!("(*(*[{}]{elem})({expr}[{from}:{end}]))", end - from);
				}
			}
			pty = pty.projection_ty(self.tcx, elem);
		}
		GoPlace { expr, ty: pty.ty }
	}

	pub fn operand(&mut self, op: &Operand<'tcx>, span: Span) -> String {
		match op {
			Operand::Copy(place) | Operand::Move(place) => self.place(*place, span).expr,
			Operand::Constant(c) => self.constant(c, span),
			Operand::RuntimeChecks(_) => "false".into(),
		}
	}

	fn operand_ty(&self, op: &Operand<'tcx>) -> Ty<'tcx> {
		op.ty(self.mir, self.tcx)
	}

	/// The value of an enum's tag for `variant`, as a Go literal of its type.
	pub fn discr(&mut self, adt: ty::AdtDef<'tcx>, variant: rustc_abi::VariantIdx, span: Span) -> String {
		let tcx = self.tcx;
		let discr_ty = adt.repr().discr_type().to_ty(tcx);
		let value = adt.discriminant_for_variant(tcx, variant).val;
		self.int_lit(discr_ty, value, span)
	}

	/// An integer of type `ty` from its bits, as a typed Go literal.
	pub fn int_lit(&mut self, ty: Ty<'tcx>, bits: u128, span: Span) -> String {
		let go = self.ty(ty, span);
		let size = int_bits(self.tcx, ty);
		let text = match ty.kind() {
			ty::Int(_) => {
				let shift = 128 - size;
				(((bits << shift) as i128) >> shift).to_string()
			}
			ty::Bool => return (bits != 0).to_string(),
			_ => (bits & (u128::MAX >> (128 - size))).to_string(),
		};
		format!("{go}({text})")
	}

	fn rvalue(&mut self, rvalue: &Rvalue<'tcx>, dest_ty: Ty<'tcx>, span: Span) -> String {
		let tcx = self.tcx;
		match rvalue {
			Rvalue::Use(op, _) => self.operand(op, span),
			Rvalue::CopyForDeref(place) => self.place(*place, span).expr,
			Rvalue::Repeat(op, _) => {
				// Read once per element, in a function literal.
				self.fold_ok = false;
				let v = self.operand(op, span);
				self.fold_ok = true;
				let go = self.ty(dest_ty, span);
				format!("func() (a {go}) {{ for i := range a {{ a[i] = {v} }}; return }}()")
			}
			Rvalue::Ref(_, _, place) | Rvalue::RawPtr(_, place) => {
				let place = self.place(*place, span);
				match place.ty.kind() {
					ty::Slice(_) | ty::Str | ty::Dynamic(..) => place.expr,
					_ => format!("&{}", place.expr),
				}
			}
			Rvalue::Discriminant(place) if place.projection.is_empty() && self.split.contains_key(&place.local) => {
				let tag = self.read_part(place.local, 0);
				let split = &self.split[&place.local];
				match split.parts[0].ty == dest_ty {
					true => tag,
					false => format!("{}({tag})", self.ty(dest_ty, span)),
				}
			}
			Rvalue::Discriminant(place) => {
				let place = self.place(*place, span);
				match place.ty.kind() {
					ty::Adt(adt, _) if adt.is_enum() => {
						let tag_ty = adt.repr().discr_type().to_ty(tcx);
						let go = self.ty(dest_ty, span);
						match tag_ty == dest_ty {
							true => format!("{}.tag", place.expr),
							false => format!("{go}({}.tag)", place.expr),
						}
					}
					_ => format!("{}(0)", self.ty(dest_ty, span)),
				}
			}
			Rvalue::Cast(kind, op, to) => self.cast(*kind, op, *to, span),
			Rvalue::BinaryOp(op, ops) => self.binary(*op, &ops.0, &ops.1, dest_ty, span),
			Rvalue::UnaryOp(op, operand) => {
				let v = self.runtime(operand, span);
				let from = self.operand_ty(operand);
				match op {
					UnOp::Not if from.is_bool() => format!("!{v}"),
					UnOp::Not => format!("^{v}"),
					UnOp::Neg => format!("-{v}"),
					UnOp::PtrMetadata => match from.builtin_deref(true).map(|t| t.kind()) {
						Some(ty::Slice(_) | ty::Str) => format!("uint(len({v}))"),
						_ if dest_ty.is_unit() => "struct{}{}".into(),
						_ => self.err(span, format!("the metadata of `{from}`")),
					},
				}
			}
			Rvalue::Aggregate(kind, ops) => self.aggregate(kind, ops, dest_ty, span),
			Rvalue::ThreadLocalRef(_) => self.err(span, "a thread-local"),
			Rvalue::Reborrow(..) => self.err(span, "reborrowing by a `Reborrow` impl"),
			Rvalue::WrapUnsafeBinder(op, _) => self.operand(op, span),
		}
	}

	fn aggregate(
		&mut self,
		kind: &AggregateKind<'tcx>,
		ops: &rustc_index::IndexSlice<rustc_abi::FieldIdx, Operand<'tcx>>,
		dest_ty: Ty<'tcx>,
		span: Span,
	) -> String {
		let values: Vec<String> = ops.iter().map(|op| self.operand(op, span)).collect();
		let go = self.ty(dest_ty, span);
		match kind {
			AggregateKind::Array(_) | AggregateKind::Closure(..) => format!("{go}{{{}}}", values.join(", ")),
			AggregateKind::Tuple if values.is_empty() => "struct{}{}".into(),
			AggregateKind::Tuple => format!("{go}{{{}}}", values.join(", ")),
			AggregateKind::Adt(did, variant, args, _, active) => {
				let adt = self.tcx.adt_def(*did);
				if library::mapped(self.tcx, *did) == Some(Mapped::Transparent) {
					// The value it holds, or Go's zero value for `MaybeUninit::uninit()`.
					return match (adt.is_union(), active) {
						(true, Some(field)) if field.as_usize() == 0 => format!("*new({go})"),
						_ => values[0].clone(),
					};
				}
				if active.is_some() {
					return self.err(span, "a union");
				}
				self.adt_lit(adt, *variant, args, &values, &go, span)
			}
			_ => self.err(span, format!("the aggregate `{kind:?}`")),
		}
	}

	/// An ADT's value of `variant` from its fields' values, as a Go composite literal of
	/// its type `go`.
	pub fn adt_lit(
		&mut self,
		adt: ty::AdtDef<'tcx>,
		variant: rustc_abi::VariantIdx,
		args: ty::GenericArgsRef<'tcx>,
		values: &[String],
		go: &str,
		span: Span,
	) -> String {
		if let Some(mapped) = library::mapped(self.tcx, adt.did())
			&& !mapped.is_runtime_layout()
		{
			return match mapped {
				Mapped::Runtime("struct{}") => "struct{}{}".into(),
				_ => self.err(
					span,
					format!("making a `{}` from its fields", self.tcx.def_path_str(adt.did())),
				),
			};
		}
		let def = adt.variant(variant);
		let fields: Vec<String> = def
			.fields
			.iter()
			.enumerate()
			.zip(values)
			.map(|((i, f), v)| format!("{}: {v}", names::field(f, i)))
			.collect();
		if !adt.is_enum() {
			return format!("{go}{{{}}}", fields.join(", "));
		}
		let discr = self.discr(adt, variant, span);
		if fields.is_empty() {
			return format!("{go}{{tag: {discr}}}");
		}
		let mut types = Vec::new();
		for (i, f) in def.fields.iter().enumerate() {
			let fty = f.ty(self.tcx, args).skip_norm_wip();
			let fty = self
				.tcx
				.normalize_erasing_regions(self.t.env(), ty::Unnormalized::new_wip(fty));
			let fgo = self.ty(fty, span);
			types.push(format!("{} {fgo}", names::field(f, i)));
		}
		format!(
			"{go}{{tag: {discr}, {}: struct{{ {} }}{{{}}}}}",
			variant_field(def.name.as_str()),
			types.join("; "),
			values.join(", ")
		)
	}

	/// A Go function literal of `body`, run at once, returning a value of `ty` as
	/// [`Fx::variant`] gives it: a bound call of it.
	pub fn returning(&mut self, ty: Ty<'tcx>, body: String, span: Span) -> library::Bound {
		match self.t.split(ty, span) {
			Some(split) => library::Bound::Parts(format!("func(){} {{ {body} }}()", split.results())),
			None => library::Bound::Expr(format!("func() {} {{ {body} }}()", self.ty(ty, span))),
		}
	}

	/// The value of the enum `ty`'s variant `variant` of `fields`, as a function literal
	/// [`Fx::returning`] it returns: whole, or as its parts where Go holds it split.
	pub fn variant(&mut self, ty: Ty<'tcx>, variant: usize, fields: &[String], span: Span) -> String {
		let ty::Adt(adt, args) = *ty.kind() else {
			return self.err(span, format!("a variant of `{ty}`, which is no enum"));
		};
		let variant = rustc_abi::VariantIdx::from_usize(variant);
		let Some(split) = self.t.split(ty, span) else {
			let go = self.ty(ty, span);
			return self.adt_lit(adt, variant, args, fields, &go, span);
		};
		let mut values = Vec::new();
		for p in split.parts.iter().filter(|p| !p.big) {
			values.push(match p.field {
				None => self.discr(adt, variant, span),
				Some((v, f)) if v == variant => fields[f.as_usize()].clone(),
				Some(_) => format!("*new({})", p.go),
			});
		}
		match split.parts.iter().any(|p| p.big) {
			true => self.err(
				span,
				format!("a bound call returning `{ty}`, which has a part of over four words"),
			),
			false => values.join(", "),
		}
	}

	pub fn t_env(&self) -> ty::TypingEnv<'tcx> {
		self.t.env()
	}

	pub fn mir_decls(&self) -> &'a mir::LocalDecls<'tcx> {
		&self.mir.local_decls
	}

	/// An operand Go evaluates at run time: a constant made a value through `val`, since
	/// Go refuses an expression of constants that overflows, where Rust's wraps.
	pub fn runtime(&mut self, op: &Operand<'tcx>, span: Span) -> String {
		let v = self.operand(op, span);
		match op {
			Operand::Constant(_) => format!("val({v})"),
			_ => v,
		}
	}

	fn cast(&mut self, kind: CastKind, op: &Operand<'tcx>, to: Ty<'tcx>, span: Span) -> String {
		let from = self.operand_ty(op);
		// A trait object reads the value when it formats, in a function literal.
		self.fold_ok = !matches!(to.builtin_deref(true).map(|t| t.kind()), Some(ty::Dynamic(..)));
		let v = self.runtime(op, span);
		self.fold_ok = true;
		let go = self.ty(to, span);
		match kind {
			CastKind::IntToInt => match from.kind() {
				ty::Bool => format!("boolTo[{go}]({v})"),
				ty::Adt(adt, _) if adt.is_enum() => format!("{go}({v}.tag)"),
				_ => format!("{go}({v})"),
			},
			CastKind::PtrToPtr | CastKind::Transmute | CastKind::Subtype | CastKind::BoxDerefTransmute => {
				let from_go = self.ty(from, span);
				match from_go == go {
					true => v,
					false => self.err(span, format!("a cast from `{from}` to `{to}`")),
				}
			}
			CastKind::PointerCoercion(coercion, _) => match coercion {
				PointerCoercion::Unsize => {
					let inner = to.builtin_deref(true);
					match inner.map(|t| t.kind()) {
						Some(ty::Slice(_)) => format!("{v}[:]"),
						Some(ty::Dynamic(..)) => self.dyn_value(from, to, v, span),
						_ => self.err(span, format!("an unsizing of `{from}` to `{to}`")),
					}
				}
				PointerCoercion::ReifyFnPointer(_) => {
					let ty::FnDef(def_id, args) = *from.kind() else {
						return self.err(span, "a function pointer to a value that is no function");
					};
					match Instance::resolve_for_fn_ptr(self.tcx, self.t.env(), def_id, args.skip_binder()) {
						Some(instance) => self.t.func(instance, span),
						None => self.err(span, "a function pointer that does not resolve"),
					}
				}
				PointerCoercion::MutToConstPointer
				| PointerCoercion::ArrayToPointer
				| PointerCoercion::UnsafeFnPointer => v,
				_ => self.err(span, format!("the coercion `{coercion:?}`")),
			},
			_ => self.err(span, format!("the cast `{kind:?}` from `{from}` to `{to}`")),
		}
	}

	/// A trait object of `Display` or `Debug` made from a reference to a value of `from`'s
	/// pointee: the runtime's `fmtDyn`, calling the impl's `fmt`.
	fn dyn_value(&mut self, from: Ty<'tcx>, to: Ty<'tcx>, v: String, span: Span) -> String {
		let tcx = self.tcx;
		let (Some(pointee), Some(ty::Dynamic(preds, ..))) =
			(from.builtin_deref(true), to.builtin_deref(true).map(|t| t.kind()))
		else {
			return self.err(span, format!("a trait object made from `{from}`"));
		};
		let Some(trait_id) = preds.principal_def_id() else {
			return self.err(span, format!("a trait object made from `{from}`"));
		};
		let name = tcx.item_name(trait_id).to_string();
		match library::fmt_call(self, &name, pointee, &v, span) {
			Some(call) => format!("fmtDyn({call})"),
			None => "nil".into(),
		}
	}

	/// `fmt::Result`, which formatting functions return.
	pub fn fmt_result_ty(&self) -> Ty<'tcx> {
		let tcx = self.tcx;
		let result = tcx
			.get_diagnostic_item(rustc_span::sym::Result)
			.expect("std's `Result`");
		let error = self.fmt_error_ty().expect("`fmt::Error`");
		Ty::new_adt(
			tcx,
			tcx.adt_def(result),
			tcx.mk_args(&[tcx.types.unit.into(), error.into()]),
		)
	}

	fn fmt_error_ty(&self) -> Option<Ty<'tcx>> {
		let tcx = self.tcx;
		let core = tcx
			.crates(())
			.iter()
			.copied()
			.find(|&c| tcx.crate_name(c).as_str() == "core")?;
		let fmt = super::names::resolve_in(tcx, core, &["fmt", "Error"])?;
		Some(tcx.type_of(fmt).instantiate_identity().skip_norm_wip())
	}

	/// `&mut Formatter`, which formatting functions take.
	pub fn formatter_ref_ty(&self) -> Ty<'tcx> {
		let tcx = self.tcx;
		let core = tcx
			.crates(())
			.iter()
			.copied()
			.find(|&c| tcx.crate_name(c).as_str() == "core")
			.expect("core");
		let f = super::names::resolve_in(tcx, core, &["fmt", "Formatter"]).expect("`fmt::Formatter`");
		let ty = Ty::new_adt(tcx, tcx.adt_def(f), tcx.mk_args(&[tcx.lifetimes.re_erased.into()]));
		Ty::new_mut_ref(tcx, tcx.lifetimes.re_erased, ty)
	}

	fn binary(&mut self, op: BinOp, a: &Operand<'tcx>, b: &Operand<'tcx>, dest_ty: Ty<'tcx>, span: Span) -> String {
		let ty = self.operand_ty(a);
		// Go folds two constants, refusing an overflow, and refuses a constant divisor of
		// zero or a negative shift, where Rust panics before or wraps at run time.
		let both = matches!((a, b), (Operand::Constant(_), Operand::Constant(_)));
		let x = match both {
			true => self.runtime(a, span),
			false => self.operand(a, span),
		};
		// Go's `&&` and `||`, which booleans' operators lower to, may not evaluate it.
		self.fold_ok = !ty.is_bool();
		let y = match matches!(
			op,
			BinOp::Div | BinOp::Rem | BinOp::Shl | BinOp::Shr | BinOp::ShlUnchecked | BinOp::ShrUnchecked
		) {
			true => self.runtime(b, span),
			false => self.operand(b, span),
		};
		self.fold_ok = true;
		let signed = ty.is_signed();
		let bits = match ty.is_integral() {
			true => int_bits(self.tcx, ty),
			false => 0,
		};
		if ty.is_bool() {
			return match op {
				BinOp::BitAnd => format!("({x} && {y})"),
				BinOp::BitOr => format!("({x} || {y})"),
				BinOp::BitXor | BinOp::Ne => format!("({x} != {y})"),
				BinOp::Eq => format!("({x} == {y})"),
				BinOp::Lt => format!("(!{x} && {y})"),
				BinOp::Gt => format!("({x} && !{y})"),
				BinOp::Le => format!("(!{x} || {y})"),
				BinOp::Ge => format!("({x} || !{y})"),
				_ => self.err(span, format!("`{op:?}` of `bool`")),
			};
		}
		if !(ty.is_integral() || ty.is_char() || ty.is_raw_ptr() || ty.is_ref()) {
			return self.err(span, format!("`{op:?}` of `{ty}`"));
		}
		// Go compares a pointer only by address, and a slice not at all.
		if let Some(pointee) = ty.builtin_deref(true)
			&& (!pointee.is_sized(self.tcx, self.t.env()) || !matches!(op, BinOp::Eq | BinOp::Ne))
		{
			return self.err(span, format!("`{op:?}` of `{ty}`"));
		}
		let sign = if signed { "S" } else { "U" };
		match op {
			BinOp::Add | BinOp::AddUnchecked => format!("({x} + {y})"),
			BinOp::Sub | BinOp::SubUnchecked => format!("({x} - {y})"),
			BinOp::Mul | BinOp::MulUnchecked => format!("({x} * {y})"),
			BinOp::Div => format!("({x} / {y})"),
			BinOp::Rem => format!("({x} % {y})"),
			BinOp::BitAnd => format!("({x} & {y})"),
			BinOp::BitOr => format!("({x} | {y})"),
			BinOp::BitXor => format!("({x} ^ {y})"),
			// MIR's shifts take the amount modulo the width, which Go's do not.
			BinOp::Shl => format!("({x} << (uint({y}) & {}))", bits - 1),
			BinOp::Shr => format!("({x} >> (uint({y}) & {}))", bits - 1),
			BinOp::ShlUnchecked => format!("({x} << uint({y}))"),
			BinOp::ShrUnchecked => format!("({x} >> uint({y}))"),
			BinOp::Eq => format!("({x} == {y})"),
			BinOp::Ne => format!("({x} != {y})"),
			BinOp::Lt => format!("({x} < {y})"),
			BinOp::Le => format!("({x} <= {y})"),
			BinOp::Gt => format!("({x} > {y})"),
			BinOp::Ge => format!("({x} >= {y})"),
			BinOp::Cmp => format!("cmp3({x}, {y})"),
			BinOp::AddWithOverflow => format!("add{sign}({x}, {y})"),
			BinOp::SubWithOverflow => format!("sub{sign}({x}, {y})"),
			BinOp::MulWithOverflow => format!("mul{sign}({x}, {y})"),
			_ => {
				let _ = dest_ty;
				self.err(span, format!("`{op:?}` of `{ty}`"))
			}
		}
	}

	/// Translate a terminator and say where it goes, giving a call's result if it is held
	/// back instead.
	fn terminator(&mut self, term: &mir::Terminator<'tcx>, out: &mut String) -> (Exit, Option<(Local, String)>) {
		let span = term.source_info.span;
		let exit = match &term.kind {
			TerminatorKind::Goto { target, .. } => Exit::Goto(target.as_usize()),
			TerminatorKind::SwitchInt { discr, targets } => self.switch(discr, targets, span, out),
			TerminatorKind::Return => Exit::End,
			TerminatorKind::Unreachable => {
				writeln!(out, "\tpanic(\"unreachable\")").unwrap();
				Exit::End
			}
			TerminatorKind::UnwindResume | TerminatorKind::UnwindTerminate(_) => {
				writeln!(out, "\tpanic(\"unwind\")").unwrap();
				Exit::End
			}
			TerminatorKind::Drop { place, target, .. } => {
				let ty = place.ty(self.mir, self.tcx).ty;
				if let Some(why) = library::drop_refused(self.tcx, self.t.env(), ty) {
					self.err(span, format!("dropping `{ty}`, {why}"));
				}
				Exit::Goto(target.as_usize())
			}
			TerminatorKind::Call {
				func,
				args,
				destination,
				target,
				fn_span,
				..
			} => {
				let args: Vec<&Operand<'tcx>> = args.iter().map(|a| &a.node).collect();
				let start = out.len();
				let held = self.call(func, &args, *destination, *fn_span, out);
				let exit = match target {
					Some(target) => Exit::Goto(target.as_usize()),
					None if out[start..].trim_start().starts_with("panic(") => Exit::End,
					None => {
						writeln!(out, "\tpanic(\"unreachable\")").unwrap();
						Exit::End
					}
				};
				return (exit, held);
			}
			TerminatorKind::Assert {
				cond,
				expected,
				msg,
				target,
				..
			} => {
				let c = self.operand(cond, span);
				let check = if *expected { format!("!{c}") } else { c };
				let msg = self.assert_message(msg, span);
				writeln!(out, "\tif {check} {{\n\t\tpanic({msg})\n\t}}").unwrap();
				Exit::Goto(target.as_usize())
			}
			TerminatorKind::FalseEdge { real_target, .. } | TerminatorKind::FalseUnwind { real_target, .. } => {
				Exit::Goto(real_target.as_usize())
			}
			kind => {
				self.err(span, format!("the terminator `{:?}`", kind.name()));
				Exit::End
			}
		};
		(exit, None)
	}

	/// A `switchInt` as tests in order, each target's values one test, and the target if
	/// none holds, or a `goto` where every value goes there. Its discriminant is read once per test, so it folds in a held-back value
	/// only where there is one test or the value does nothing but read.
	fn switch(&mut self, discr: &Operand<'tcx>, targets: &mir::SwitchTargets, span: Span, out: &mut String) -> Exit {
		let ty = self.operand_ty(discr);
		let (arms, default) = self.switch_targets(targets);
		// The local whose value a test of it, or of a field of it, may fold in.
		let held = discr.place().map(|p| p.local);
		if arms.is_empty() {
			if let Some(local) = held {
				self.held.discard(local, out);
			}
			return Exit::Goto(default.as_usize());
		}
		self.fold_ok = arms.len() < 2 || held.is_none_or(|l| !self.held.effects(l));
		let v = self.operand(discr, span);
		self.fold_ok = true;
		// `if v` goes on where `0` doesn't.
		if let (ty::Bool, &[(0, zero)]) = (ty.kind(), arms.as_slice()) {
			return Exit::Branch(vec![(Cond(v), default.as_usize())], zero.as_usize());
		}
		let mut tests: Vec<(Vec<String>, BasicBlock)> = Vec::new();
		for (value, target) in arms {
			let test = match ty.kind() {
				ty::Bool if value == 0 => {
					let mut c = Cond(v.clone());
					cfg::Cond::negate(&mut c);
					c.0
				}
				ty::Bool => v.clone(),
				ty::Char => format!("{v} == {value}"),
				_ => format!("{v} == {}", self.int_lit(ty, value, span)),
			};
			match tests.iter_mut().find(|(_, t)| *t == target) {
				Some((tests, _)) => tests.push(test),
				None => tests.push((vec![test], target)),
			}
		}
		let tests = tests
			.into_iter()
			.map(|(tests, target)| (Cond(tests.join(" || ")), target.as_usize()))
			.collect();
		Exit::Branch(tests, default.as_usize())
	}

	/// A `switchInt`'s values and their targets, and the target if none matches. Where no
	/// value is left, as an exhaustive `match` has it, the last value's target is that;
	/// and a value going there is no test.
	fn switch_targets(&self, targets: &mir::SwitchTargets) -> (Vec<(u128, BasicBlock)>, BasicBlock) {
		let mut arms: Vec<(u128, BasicBlock)> = targets.iter().collect();
		let mut default = targets.otherwise();
		let data = &self.mir.basic_blocks[default];
		if matches!(data.terminator().kind, TerminatorKind::Unreachable)
			&& self.empty(data)
			&& let Some((_, last)) = arms.pop()
		{
			default = last;
		}
		let default = self.forward(default);
		arms.retain_mut(|(_, target)| {
			*target = self.forward(*target);
			*target != default
		});
		(arms, default)
	}

	/// Where `bb` goes, past blocks that do nothing but go on, such as a drop that does
	/// nothing, which a drop flag's test goes to.
	fn forward(&self, mut bb: BasicBlock) -> BasicBlock {
		for _ in 0..self.mir.basic_blocks.len() {
			let data = &self.mir.basic_blocks[bb];
			if !self.empty(data) || self.in_site(data.terminator().source_info.span).is_some() {
				break;
			}
			bb = match &data.terminator().kind {
				TerminatorKind::Goto { target, .. } => *target,
				TerminatorKind::Drop { place, target, .. }
					if library::drop_refused(self.tcx, self.t.env(), place.ty(self.mir, self.tcx).ty).is_none() =>
				{
					*target
				}
				_ => break,
			};
		}
		bb
	}

	/// Whether `data` has no statement that does anything.
	fn empty(&self, data: &mir::BasicBlockData<'tcx>) -> bool {
		data.statements.iter().all(|s| {
			matches!(
				s.kind,
				StatementKind::StorageLive(_) | StatementKind::StorageDead(_) | StatementKind::Nop
			)
		})
	}

	/// What a `return` gives: nothing, the return place, or its parts but those written
	/// through pointers, which a return place held whole writes into `out` first.
	fn returned(&mut self, out: &mut String) -> Option<String> {
		let ty = self.mir.return_ty();
		if Translator::no_value(ty) {
			return None;
		}
		let ret = mir::RETURN_PLACE;
		let Some(split) = self.t.split(ty, self.mir.span) else {
			self.used.insert(ret);
			self.mentioned.insert(ret);
			return Some(Self::local_name(ret));
		};
		let mut values = Vec::new();
		for (i, p) in split.parts.iter().enumerate() {
			match (self.split.contains_key(&ret), p.big) {
				(true, true) => {}
				(true, false) => values.push(self.read_part(ret, i)),
				(false, big) => {
					self.used.insert(ret);
					self.mentioned.insert(ret);
					match big {
						true => writeln!(out, "\t*_0_{} = _0{}", p.name, p.path).unwrap(),
						false => values.push(format!("_0{}", p.path)),
					}
				}
			}
		}
		Some(values.join(", "))
	}

	/// Part `i` of the split local `local`, as Go reads or writes it.
	fn part(&mut self, local: Local, i: usize) -> String {
		self.mentioned.insert(local);
		let p = &self.split[&local].parts[i];
		match p.big && self.pointed(local) {
			true => format!("(*{}_{})", Self::local_name(local), p.name),
			false => format!("{}_{}", Self::local_name(local), p.name),
		}
	}

	/// Whether the split local `local` holds its big parts through pointers: the return
	/// place and the arguments, whose callers hold them.
	fn pointed(&self, local: Local) -> bool {
		local.as_usize() <= self.mir.arg_count
	}

	/// Part `i` of the split local `local`, read.
	fn read_part(&mut self, local: Local, i: usize) -> String {
		self.part_used.insert((local, i));
		self.part(local, i)
	}

	/// The panic of a failed assertion, with native Rust's message.
	fn assert_message(&mut self, msg: &AssertKind<Operand<'tcx>>, span: Span) -> String {
		// Evaluated only on failure.
		self.fold_ok = false;
		let text = self.assert_text(msg, span);
		self.fold_ok = true;
		text
	}

	fn assert_text(&mut self, msg: &AssertKind<Operand<'tcx>>, span: Span) -> String {
		let text = match msg {
			AssertKind::BoundsCheck { len, index } => {
				let len = self.operand(len, span);
				let index = self.operand(index, span);
				return format!("panicBounds({len}, {index})");
			}
			AssertKind::Overflow(op, ..) => match op {
				BinOp::Add => "attempt to add with overflow",
				BinOp::Sub => "attempt to subtract with overflow",
				BinOp::Mul => "attempt to multiply with overflow",
				BinOp::Div => "attempt to divide with overflow",
				BinOp::Rem => "attempt to calculate the remainder with overflow",
				BinOp::Shl => "attempt to shift left with overflow",
				BinOp::Shr => "attempt to shift right with overflow",
				_ => return self.err(span, format!("the assertion `{msg:?}`")),
			},
			AssertKind::OverflowNeg(_) => "attempt to negate with overflow",
			AssertKind::DivisionByZero(_) => "attempt to divide by zero",
			AssertKind::RemainderByZero(_) => "attempt to calculate the remainder with a divisor of zero",
			_ => return self.err(span, format!("the assertion `{msg:?}`")),
		};
		format!("rustPanic({text:?})")
	}

	/// Translate a call, giving its result if it is held back instead of assigned.
	fn call(
		&mut self,
		func: &Operand<'tcx>,
		args: &[&Operand<'tcx>],
		dest: Place<'tcx>,
		span: Span,
		out: &mut String,
	) -> Option<(Local, String)> {
		let tcx = self.tcx;
		let dest_ty = dest.ty(self.mir, tcx).ty;
		// Held back into the block only this call goes to.
		let held = dest
			.as_local()
			.filter(|l| self.carry && self.temps.foldable.contains(l) && !Translator::no_value(dest_ty));
		let ret = self.t.split(dest_ty, span);
		let func_ty = self.operand_ty(func);
		let (callee, spread) = match *func_ty.kind() {
			ty::FnDef(def_id, gargs) => {
				let instance = match Instance::try_resolve(tcx, self.t.env(), def_id, gargs.skip_binder()) {
					Ok(Some(instance)) => instance,
					_ => {
						self.err(
							span,
							format!("a call of `{}` that does not resolve", tcx.def_path_str(def_id)),
						);
						return None;
					}
				};
				// A binding may use an argument twice, late, or not at all.
				self.fold_ok = false;
				let bound = library::call(self, instance, args, dest_ty, span);
				self.fold_ok = true;
				if let Some(text) = bound {
					let text = match text {
						library::Bound::Expr(e) if Translator::no_value(dest_ty) => e,
						// Not a scalar, which may be a Go constant.
						library::Bound::Expr(e)
							if let Some(local) = held
								&& !dest_ty.is_scalar() =>
						{
							return Some((local, e));
						}
						library::Bound::Expr(e)
							if dest.projection.is_empty()
								&& let Some(split) = self.split.get(&dest.local).cloned() =>
						{
							let (values, pre) = self.spill(&split, e);
							out.push_str(&pre);
							let parts: Vec<(String, String)> = (values.into_iter().enumerate())
								.map(|(i, value)| (self.part(dest.local, i), value))
								.collect();
							assign_all(&parts, out);
							return None;
						}
						library::Bound::Expr(e) => format!("{} = {e}", self.dest(dest, span)),
						library::Bound::Parts(e) => {
							let split = ret.as_ref().expect("a bound call returns parts of a split enum");
							let (lhs, _) = self.dest_parts(dest, split, span);
							format!("{} = {e}", lhs.join(", "))
						}
						library::Bound::Stmt(s) => s,
					};
					writeln!(out, "\t{text}").unwrap();
					return None;
				}
				match instance.def {
					InstanceKind::Intrinsic(_) => {
						let name = tcx.item_name(instance.def_id());
						self.err(span, format!("the intrinsic `{name}`"));
						return None;
					}
					InstanceKind::Virtual(..) => {
						self.err(span, "a call through a trait object");
						return None;
					}
					InstanceKind::Item(_) if !tcx.is_mir_available(instance.def_id()) => {
						self.err(span, format!("`{instance}`, which has no MIR and no binding"));
						return None;
					}
					_ => {}
				}
				let rust_call = tcx.fn_sig(def_id).skip_binder().abi() == rustc_abi::ExternAbi::RustCall;
				(self.t.func(instance, span), rust_call)
			}
			ty::FnPtr(..) => (self.operand(func, span), false),
			_ => {
				self.err(span, format!("a call of `{func_ty}`"));
				return None;
			}
		};
		let mut values = Vec::new();
		for (i, arg) in args.iter().enumerate() {
			if spread && i + 1 == args.len() {
				// Spread into its fields, each a read of it.
				self.fold_ok = false;
				let v = self.operand(arg, span);
				self.fold_ok = true;
				let ty::Tuple(tys) = *self.operand_ty(arg).kind() else {
					self.err(span, "a rust-call whose last argument is no tuple");
					return None;
				};
				values.extend((0..tys.len()).map(|f| format!("{v}.F{f}")));
			} else if let Some(split) = self.t.split(self.operand_ty(arg), span) {
				values.extend(self.arg_parts(arg, &split, span, out));
			} else {
				values.push(self.operand(arg, span));
			}
		}
		if let Some(split) = ret {
			let (lhs, outs) = self.dest_parts(dest, &split, span);
			values.extend(outs);
			writeln!(out, "\t{} = {callee}({})", lhs.join(", "), values.join(", ")).unwrap();
			return None;
		}
		let call = format!("{callee}({})", values.join(", "));
		if let Some(local) = held {
			return Some((local, call));
		}
		match Translator::no_value(dest_ty) {
			true => writeln!(out, "\t{call}").unwrap(),
			false => {
				let dest = self.dest(dest, span);
				writeln!(out, "\t{dest} = {call}").unwrap()
			}
		}
		None
	}
}

/// Assign each value to its place at once, as Go evaluates every value of an assignment
/// before it assigns any, which a composite literal's fields would be.
fn assign_all(parts: &[(String, String)], out: &mut String) {
	let (lhs, rhs): (Vec<&str>, Vec<&str>) = parts.iter().map(|(l, r)| (l.as_str(), r.as_str())).unzip();
	writeln!(out, "\t{} = {}", lhs.join(", "), rhs.join(", ")).unwrap();
}

/// The width of an integer type, or a `char`'s or a `bool`'s, in bits.
pub fn int_bits<'tcx>(tcx: TyCtxt<'tcx>, ty: Ty<'tcx>) -> u32 {
	match ty.kind() {
		ty::Int(i) => i.bit_width().unwrap_or(tcx.data_layout.pointer_size().bits()) as u32,
		ty::Uint(u) => u.bit_width().unwrap_or(tcx.data_layout.pointer_size().bits()) as u32,
		ty::Char => 32,
		ty::Bool => 8,
		_ => 64,
	}
}

impl cfg::Lower<Stmt> for Fx<'_, '_> {
	fn node(&mut self, node: usize, out: &mut Vec<Stmt>) -> Exit {
		self.block(BasicBlock::from_usize(node), out)
	}

	fn flush(&mut self, out: &mut Vec<Stmt>) {
		let mut text = String::new();
		self.begin(&fold::Access::default(), Effect::default(), true, &mut text);
		if !text.is_empty() {
			out.push(Stmt::Lines(text));
		}
	}
}
