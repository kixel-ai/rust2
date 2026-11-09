//! Each temporary assigned once and read once, folded into the expression that reads it,
//! as rust2-ts folds them: a Go variable fewer, which Go would zero at entry and copy in
//! and out. A temporary is held back as its value's text until the statement that reads
//! it, along its block and on into a block only it goes to, and emitted as an assignment
//! wherever folding it would reorder an effect.

use std::collections::HashSet;
use std::fmt::Write;

use rustc_middle::mir::visit::{MutatingUseContext, NonMutatingUseContext, PlaceContext, Visitor};
use rustc_middle::mir::{self, BasicBlock, Local, Location, Place, PlaceElem};

/// Which locals can be held back: those assigned whole once and read once, and never
/// borrowed nor changed in place, in the blocks translation emits.
pub struct Temps {
	pub foldable: HashSet<Local>,
	/// Locals something borrows, which a call or a write through a pointer can change.
	pub borrowed: HashSet<Local>,
}

impl Temps {
	pub fn new(mir: &mir::Body<'_>) -> Self {
		let mut count = Count {
			reads: vec![0; mir.local_decls.len()],
			defs: vec![0; mir.local_decls.len()],
			blocked: HashSet::new(),
			borrowed: HashSet::new(),
		};
		for (bb, data) in mir.basic_blocks.iter_enumerated() {
			if data.is_cleanup {
				continue;
			}
			for (i, stmt) in data.statements.iter().enumerate() {
				count.visit_statement(
					stmt,
					Location {
						block: bb,
						statement_index: i,
					},
				);
			}
			count.visit_terminator(data.terminator(), mir.terminator_loc(bb));
		}
		let foldable = mir
			.local_decls
			.indices()
			.skip(1 + mir.arg_count)
			.filter(|l| {
				count.reads[l.as_usize()] == 1
					&& count.defs[l.as_usize()] == 1
					&& !count.blocked.contains(l)
					&& !mir.local_decls[*l].ty.is_never()
			})
			.collect();
		Self {
			foldable,
			borrowed: count.borrowed,
		}
	}

	/// What `visit` reaches, given to a [`Collect`].
	pub fn access(&self, visit: impl FnOnce(&mut Collect<'_>)) -> Access {
		let mut collect = Collect {
			borrowed: &self.borrowed,
			acc: Access::default(),
		};
		visit(&mut collect);
		collect.acc
	}
}

struct Count {
	reads: Vec<u32>,
	defs: Vec<u32>,
	/// Locals changed in place, borrowed, or used as an index.
	blocked: HashSet<Local>,
	borrowed: HashSet<Local>,
}

impl<'tcx> Visitor<'tcx> for Count {
	fn visit_place(&mut self, place: &Place<'tcx>, ctx: PlaceContext, _: Location) {
		let local = place.local;
		for elem in place.projection {
			if let PlaceElem::Index(index) = elem {
				self.blocked.insert(index);
			}
		}
		if matches!(
			ctx,
			PlaceContext::NonUse(_) | PlaceContext::MutatingUse(MutatingUseContext::Drop)
		) {
			return;
		}
		// Through a pointer: the pointer is read, whatever is done to what it points at.
		if place.projection.iter().any(|e| e == PlaceElem::Deref) {
			self.reads[local.as_usize()] += 1;
			return;
		}
		match Use::of(ctx) {
			Use::Read => {
				self.reads[local.as_usize()] += 1;
				// Read twice in Go, or a part of an array that Go must address.
				if place.projection.iter().any(|e| {
					matches!(
						e,
						PlaceElem::ConstantIndex { from_end: true, .. } | PlaceElem::Subslice { .. }
					)
				}) {
					self.blocked.insert(local);
				}
			}
			Use::Store
				if place.projection.is_empty()
					&& matches!(
						ctx,
						PlaceContext::MutatingUse(MutatingUseContext::Store | MutatingUseContext::Call)
					) =>
			{
				self.defs[local.as_usize()] += 1;
			}
			Use::Store => {
				self.blocked.insert(local);
			}
			Use::Borrow => {
				self.blocked.insert(local);
				self.borrowed.insert(local);
			}
		}
	}
}

/// How a place is used, apart from through a pointer.
enum Use {
	Read,
	Store,
	Borrow,
}

impl Use {
	fn of(ctx: PlaceContext) -> Self {
		match ctx {
			PlaceContext::NonMutatingUse(
				NonMutatingUseContext::SharedBorrow
				| NonMutatingUseContext::FakeBorrow
				| NonMutatingUseContext::RawBorrow,
			)
			| PlaceContext::MutatingUse(MutatingUseContext::Borrow | MutatingUseContext::RawBorrow) => Use::Borrow,
			PlaceContext::NonMutatingUse(_) | PlaceContext::NonUse(_) => Use::Read,
			PlaceContext::MutatingUse(_) => Use::Store,
		}
	}
}

/// What one statement or terminator reads and writes, outside its own effect.
#[derive(Default)]
pub struct Access {
	/// Locals it reads, a pointer it reads through included.
	pub reads: Vec<Local>,
	/// Whether it reads memory a call or a write through a pointer could change: through
	/// a pointer, or a local something borrows.
	pub memory: bool,
	/// Locals it assigns, whole or in part.
	pub writes: Vec<Local>,
	/// Whether it writes such memory.
	pub writes_memory: bool,
}

impl Access {
	/// What a value computed from its reads does, before what it folds in.
	pub fn does(&self) -> Does {
		Does {
			effect: false,
			memory: self.memory,
			reads: self.reads.clone(),
		}
	}
}

pub struct Collect<'a> {
	borrowed: &'a HashSet<Local>,
	acc: Access,
}

impl<'tcx> Visitor<'tcx> for Collect<'_> {
	fn visit_place(&mut self, place: &Place<'tcx>, ctx: PlaceContext, _: Location) {
		let acc = &mut self.acc;
		let local = place.local;
		for elem in place.projection {
			if let PlaceElem::Index(index) = elem {
				acc.reads.push(index);
				acc.memory |= self.borrowed.contains(&index);
			}
		}
		if matches!(
			ctx,
			PlaceContext::NonUse(_) | PlaceContext::MutatingUse(MutatingUseContext::Drop)
		) {
			return;
		}
		let derefs = place.projection.iter().filter(|e| *e == PlaceElem::Deref).count();
		let borrowed = self.borrowed.contains(&local);
		let usage = Use::of(ctx);
		if derefs > 0 {
			acc.reads.push(local);
			acc.memory |= borrowed;
			// Each pointer but the last is read from memory, and the last too unless the
			// place is only addressed or written.
			acc.memory |= derefs > 1 || matches!(usage, Use::Read);
			acc.writes_memory |= matches!(usage, Use::Store);
			return;
		}
		match usage {
			Use::Read => {
				acc.reads.push(local);
				acc.memory |= borrowed;
			}
			Use::Store => {
				acc.writes.push(local);
				acc.writes_memory |= borrowed;
			}
			Use::Borrow => {}
		}
	}
}

/// What evaluating a temporary's value does, as far as moving it goes.
#[derive(Clone, Default)]
pub struct Does {
	/// Whether it calls something, which can change memory and panic.
	pub effect: bool,
	/// Whether it reads memory a call or a write through a pointer could change.
	pub memory: bool,
	/// The locals it reads, which an assignment to must not pass it.
	pub reads: Vec<Local>,
}

impl Does {
	pub fn join(&mut self, other: &Does) {
		self.effect |= other.effect;
		self.memory |= other.memory;
		self.reads.extend_from_slice(&other.reads);
	}

	/// Whether the two can run in either order.
	fn commutes(&self, other: &Does) -> bool {
		!(self.effect && (other.effect || other.memory) || other.effect && self.memory)
	}
}

/// A temporary's value, held back to fold into the one expression that reads it.
pub struct Pending {
	pub local: Local,
	pub expr: String,
	pub does: Does,
}

/// What a statement or terminator does besides reading and writing its places.
#[derive(Clone, Copy, Default)]
pub struct Effect {
	/// A call, which can change memory.
	pub writes: bool,
	/// A call or an assertion.
	pub panics: bool,
}

/// The temporaries held back, in the order their values were computed, and those the
/// statement being emitted folds in.
#[derive(Default)]
pub struct Held {
	pub pending: Vec<Pending>,
	/// The locals of temporaries the statement being emitted may fold in.
	folding: HashSet<Local>,
	/// What the temporaries folded in so far do.
	folded: Does,
	/// The locals of temporaries emitted as assignments, which need declaring.
	pub emitted: Vec<Local>,
}

impl Held {
	/// Before emitting a statement that does `acc` and `effect`: emit into `out` each
	/// temporary that can't wait past it, and those before them that can't run after
	/// them, and let it fold in the rest it reads. If `ends`, control goes on to code
	/// emitted elsewhere, so nothing waits.
	pub fn begin(&mut self, acc: &Access, effect: Effect, ends: bool, out: &mut String) {
		let read: Vec<bool> = self.pending.iter().map(|p| acc.reads.contains(&p.local)).collect();
		let mut flush: Vec<bool> = self
			.pending
			.iter()
			.zip(&read)
			.map(|(p, &is_read)| {
				let d = &p.does;
				match is_read {
					// Go may evaluate the statement's other operands before a call folded
					// among them, where Rust evaluated the call first.
					true => {
						d.effect
							&& (acc.memory
								|| self
									.pending
									.iter()
									.zip(&read)
									.any(|(q, &r)| r && q.local != p.local && q.does.memory))
					}
					false => {
						ends || effect.writes && (d.effect || d.memory)
							|| effect.panics && d.effect
							|| d.effect && (acc.memory || acc.writes_memory)
							|| d.memory && acc.writes_memory
							|| d.reads.iter().any(|r| acc.writes.contains(r))
					}
				}
			})
			.collect();
		// Emitted ones run first, then those folded in, then those still held: one computed
		// earlier that now runs after another must not care.
		let rank = |flush: &[bool], i: usize| match (flush[i], read[i]) {
			(true, _) => 0,
			(false, true) => 1,
			(false, false) => 2,
		};
		let mut changed = true;
		while changed {
			changed = false;
			for i in 0..flush.len() {
				for j in 0..i {
					if rank(&flush, j) > rank(&flush, i) && !self.pending[j].does.commutes(&self.pending[i].does) {
						flush[j] = true;
						changed = true;
					}
				}
			}
		}
		let pending = std::mem::take(&mut self.pending);
		for (i, p) in pending.into_iter().enumerate() {
			if flush[i] {
				self.assign(p, out);
			} else {
				if read[i] {
					self.folding.insert(p.local);
				}
				self.pending.push(p);
			}
		}
		self.folded = Does::default();
	}

	/// Whether the value held back for `local` does more than read.
	pub fn effects(&self, local: Local) -> bool {
		self.pending.iter().any(|p| p.local == local && p.does.effect)
	}

	/// Drop the value held back for `local`, which the statement being emitted no longer
	/// reads, emitting it into `out` only if it does more than read.
	pub fn discard(&mut self, local: Local, out: &mut String) {
		self.folding.remove(&local);
		if let Some(i) = self.pending.iter().position(|p| p.local == local) {
			let p = self.pending.remove(i);
			if p.does.effect {
				writeln!(out, "\t_ = {}", p.expr).unwrap();
			}
		}
	}

	/// How the statement being emitted reads `local`: as its value, parenthesized unless
	/// it is an operand already, if it folds it in. If `allowed` is false, where Go would
	/// evaluate it twice, late, or not at all, it is emitted before the statement instead.
	pub fn take(&mut self, local: Local, allowed: bool) -> Take {
		if !self.folding.contains(&local) {
			return Take::Var;
		}
		if !allowed {
			return Take::Refused;
		}
		self.folding.remove(&local);
		let i = self
			.pending
			.iter()
			.position(|p| p.local == local)
			.expect("a temporary being folded is held");
		let p = self.pending.remove(i);
		self.folded.join(&p.does);
		Take::Folded(match operand(&p.expr) {
			true => p.expr,
			false => format!("({})", p.expr),
		})
	}

	/// After emitting the statement: emit into `out` each temporary it read but did not
	/// fold in, and give what those it folded in do.
	pub fn end(&mut self, out: &mut String) -> Does {
		let folding = std::mem::take(&mut self.folding);
		let pending = std::mem::take(&mut self.pending);
		for p in pending {
			match folding.contains(&p.local) {
				true => self.assign(p, out),
				false => self.pending.push(p),
			}
		}
		std::mem::take(&mut self.folded)
	}

	fn assign(&mut self, p: Pending, out: &mut String) {
		writeln!(out, "\t_{} = {}", p.local.as_usize(), p.expr).unwrap();
		self.emitted.push(p.local);
	}
}

/// How a statement reads a local.
pub enum Take {
	/// As the value held back for it.
	Folded(String),
	/// As its variable, which the value held back for it is assigned to first.
	Refused,
	/// As its variable.
	Var,
}

/// Whether Go reads `expr` as one operand wherever it stands: a name, a call, a
/// conversion, or a selector or index of one, with nothing outside brackets but names
/// and dots. A composite literal is not, as it would end an `if`'s condition.
pub fn operand(expr: &str) -> bool {
	let mut depth = 0usize;
	let mut chars = expr.chars();
	while let Some(c) = chars.next() {
		match c {
			'(' | '[' => depth += 1,
			')' | ']' => depth = depth.saturating_sub(1),
			'"' => {
				while let Some(c) = chars.next() {
					match c {
						'\\' => {
							chars.next();
						}
						'"' => break,
						_ => {}
					}
				}
			}
			c if depth == 0 && !(c.is_ascii_alphanumeric() || c == '_' || c == '.') => return false,
			_ => {}
		}
	}
	!expr.is_empty() && !expr.starts_with(|c: char| c.is_ascii_digit())
}

/// The block `bb`'s terminator goes on to alone, if only it goes there, so what it holds
/// back can wait into that block.
pub fn carried(mir: &mir::Body<'_>, bb: BasicBlock, term: &mir::Terminator<'_>) -> Option<BasicBlock> {
	use mir::TerminatorKind as K;
	let target = match &term.kind {
		K::Goto { target, .. } | K::Drop { target, .. } | K::Assert { target, .. } => *target,
		K::Call {
			target: Some(target), ..
		} => *target,
		K::FalseEdge { real_target, .. } | K::FalseUnwind { real_target, .. } => *real_target,
		_ => return None,
	};
	(target != bb && target != mir::START_BLOCK && mir.basic_blocks.predecessors()[target].as_slice() == [bb])
		.then_some(target)
}

#[cfg(test)]
mod tests {
	use super::operand;

	#[test]
	fn operands() {
		for expr in ["_1", "(*_1).buf", "f(x, y + 1)", "T(x)", "_2.Some.F0[3]", "uint8(_3)"] {
			assert!(operand(expr), "{expr}");
		}
		for expr in [
			"&x",
			"-x",
			"x + 1",
			"T{a: 1}",
			"len(*x) == 0",
			"func() T { return x }()",
			"1",
		] {
			assert!(!operand(expr), "{expr}");
		}
	}
}
