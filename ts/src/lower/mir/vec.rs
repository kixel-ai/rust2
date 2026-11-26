//! `vec![a, b]`, which the pinned nightly expands to
//! `box_assume_init_into_vec_unsafe(write_box_via_move(Box::new_uninit(), [a, b]))`: MIR
//! makes an uninitialized box, writes the array through the box's pointer, and turns the
//! box into a `Vec`. A box is its contents in JS, and there is nothing to make before
//! them, so the write becomes the box's local's one assignment, of the array itself.

use rustc_middle::mir::{
	self, BasicBlock, Local, Operand, Place, ProjectionElem, Rvalue, StatementKind, TerminatorKind,
};
use rustc_middle::ty::{Ty, TyCtxt};

use crate::lower::library::{self, Std};

/// Rewrite each `vec![a, b]` in `body` so its box's locals hold the array. One whose MIR
/// has another shape is left as it is, and refused where it is lowered.
pub(crate) fn literals<'tcx>(tcx: TyCtxt<'tcx>, body: &mut mir::Body<'tcx>) {
	let Some(into_vec) = tcx.get_diagnostic_item(rustc_span::Symbol::intern("box_assume_init_into_vec_unsafe")) else {
		return;
	};
	let boxes: Vec<Local> = body
		.basic_blocks
		.iter()
		.filter_map(|data| match &data.terminator().kind {
			TerminatorKind::Call { func, args, .. } if func.const_fn_def().is_some_and(|(f, _)| f == into_vec) => {
				match &args[..] {
					[arg] => arg.node.place()?.as_local(),
					_ => None,
				}
			}
			_ => None,
		})
		.collect();
	for boxed in boxes {
		if let Some(literal) = Literal::find(tcx, body, boxed) {
			literal.rewrite(body);
		}
	}
}

/// Where one `vec![a, b]`'s MIR makes its box, takes its pointer, and writes the array.
struct Literal<'tcx> {
	/// The locals that hold the box: the one made first, then each it moves to.
	boxes: Vec<Local>,
	/// The block whose call makes the box, and where it goes on to.
	made: (BasicBlock, BasicBlock),
	/// The statement that takes the box's pointer.
	pointer: (BasicBlock, usize),
	/// The statement that writes the array through it, and the array's type.
	write: (BasicBlock, usize),
	array: Ty<'tcx>,
}

impl<'tcx> Literal<'tcx> {
	/// The literal whose box `last` holds as `box_assume_init_into_vec_unsafe` takes it.
	fn find(tcx: TyCtxt<'tcx>, body: &mir::Body<'tcx>, last: Local) -> Option<Self> {
		let new_uninit = |func: &Operand<'tcx>| {
			func.const_fn_def().is_some_and(|(f, _)| {
				tcx.item_name(f).as_str() == "new_uninit" && library::inherent_std(tcx, f) == Some(Std::Box)
			})
		};
		// Each statement of the body, where it is.
		let statements = || {
			body.basic_blocks.iter_enumerated().flat_map(|(bb, data)| {
				data.statements
					.iter()
					.enumerate()
					.filter_map(move |(i, stmt)| match &stmt.kind {
						StatementKind::Assign(assign) => Some((bb, i, &assign.0, &assign.1)),
						_ => None,
					})
			})
		};
		// Back from `last` through the moves between its locals, to the one made first.
		let mut boxes = vec![last];
		loop {
			let to = *boxes.last()?;
			let mut from = None;
			for (_, _, place, rvalue) in statements() {
				if place.as_local() == Some(to)
					&& let Rvalue::Use(Operand::Move(source), _) = rvalue
				{
					from = only(from, source.as_local()?)?;
				}
			}
			match from {
				Some(from) if !boxes.contains(&from) => boxes.push(from),
				Some(_) => return None,
				None => break,
			}
		}
		boxes.reverse();
		let first = boxes[0];
		let mut made = None;
		for (bb, data) in body.basic_blocks.iter_enumerated() {
			if let TerminatorKind::Call {
				func,
				destination,
				target: Some(target),
				..
			} = &data.terminator().kind
				&& destination.as_local() == Some(first)
				&& new_uninit(func)
			{
				made = only(made, (bb, *target))?;
			}
		}
		let mut pointer = None;
		for (bb, i, place, rvalue) in statements() {
			if let Rvalue::Cast(mir::CastKind::BoxDerefTransmute, Operand::Copy(from) | Operand::Move(from), _) = rvalue
				&& from.local == first
			{
				pointer = only(pointer, (place.as_local()?, bb, i))?;
			}
		}
		let (ptr, pointer_bb, pointer_i) = pointer?;
		let mut write = None;
		for (bb, i, place, _) in statements() {
			if place.local == ptr
				&& let [
					ProjectionElem::Deref,
					ProjectionElem::Field(..),
					ProjectionElem::Field(_, array),
				] = place.projection.as_slice()
			{
				write = only(write, (bb, i, *array))?;
			}
		}
		let (write_bb, write_i, array) = write?;
		Some(Self {
			boxes,
			made: made?,
			pointer: (pointer_bb, pointer_i),
			write: (write_bb, write_i),
			array,
		})
	}

	fn rewrite(self, body: &mut mir::Body<'tcx>) {
		for &local in &self.boxes {
			body.local_decls[local].ty = self.array;
		}
		let blocks = body.basic_blocks.as_mut();
		let (made, target) = self.made;
		blocks[made].terminator_mut().kind = TerminatorKind::goto(target);
		blocks[self.pointer.0].statements[self.pointer.1].make_nop(true);
		if let StatementKind::Assign(assign) = &mut blocks[self.write.0].statements[self.write.1].kind {
			assign.0 = Place::from(self.boxes[0]);
		}
		// Dropping the box only frees it, which JS leaves to its collector.
		for data in blocks.iter_mut() {
			let term = data.terminator_mut();
			if let TerminatorKind::Drop { place, target, .. } = &term.kind
				&& place.as_local().is_some_and(|local| self.boxes.contains(&local))
			{
				term.kind = TerminatorKind::goto(*target);
			}
		}
	}
}

/// `found`, unless `seen` already holds one: then the pattern is not `vec!`'s.
fn only<T>(seen: Option<T>, found: T) -> Option<Option<T>> {
	match seen {
		Some(_) => None,
		None => Some(Some(found)),
	}
}
