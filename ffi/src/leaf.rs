//! Which calls are leaves, by their MIR: a body that runs no code but bodies proven the same
//! way and core's intrinsics that compute alone, in a bounded number of steps, as
//! [`crate::abi::Abi::leaf`] needs.

use std::cell::RefCell;

use rustc_attr_ir::LangItem;
use rustc_data_structures::fx::FxHashMap;
use rustc_hir::def::DefKind;
use rustc_hir::def_id::DefId;
use rustc_middle::mir::visit::Visitor;
use rustc_middle::mir::{self, BasicBlock, Location, NonDivergingIntrinsic, Rvalue, StatementKind, TerminatorKind};
use rustc_middle::ty::{self, Instance, InstanceKind, ShimKind, Ty, TyCtxt, TypeVisitableExt, TypingEnv};
use rustc_span::DUMMY_SP;

/// The flag each crate compiled for an export's target adds to rustc's: all of its MIR, which
/// `cargo check` encodes none of. Measured on moq's stand-in, it costs the export about 9%
/// of its CPU time and 5% of its peak memory (`bench/ffi/README.md`'s "The export's build
/// cost").
const FLAG: &str = "-Zalways-encode-mir";

/// Add [`FLAG`] to rustc's `args` for a crate of the export's target, not a build script or
/// proc macro, which cargo compiles without `--target`. Cargo's query of the compiler, whose
/// answer it hashes into every unit's fingerprint, names it, so a target directory keeps
/// nothing compiled without it.
pub(crate) fn flags(args: &[String], added: &mut Vec<String>) {
	if args[1..] == ["-vV"] {
		println!("rust2-ffi-leaf: {FLAG}");
	}
	if args.iter().any(|arg| arg == "--target") {
		added.push(FLAG.to_string());
	}
}

/// The proofs of a crate's leaves.
pub(crate) struct Leaves<'tcx> {
	tcx: TyCtxt<'tcx>,
	/// Whether each instance met is a leaf: `None` while its walk is open.
	seen: RefCell<FxHashMap<Instance<'tcx>, Option<bool>>>,
	/// How many walks are open, which the crate's recursion limit bounds.
	depth: std::cell::Cell<usize>,
}

impl<'tcx> Leaves<'tcx> {
	pub(crate) fn new(tcx: TyCtxt<'tcx>) -> Self {
		Self {
			tcx,
			seen: Default::default(),
			depth: Default::default(),
		}
	}

	/// Whether a call of `def_id` at `args`, which name no parameter, is a leaf.
	pub(crate) fn call(&self, def_id: DefId, args: ty::GenericArgs<'tcx>) -> bool {
		let tcx = self.tcx;
		if args.has_param() {
			return false;
		}
		let args = tcx.erase_and_anonymize_regions(args);
		match Instance::try_resolve(tcx, TypingEnv::fully_monomorphized(), def_id, args) {
			Ok(Some(instance)) => self.instance(instance),
			_ => false,
		}
	}

	/// Whether `<ty as Deref>::deref`, or with `mutable` `<ty as DerefMut>::deref_mut`, is a
	/// leaf, as the glue calls it to reach a member of the type `ty` derefs to.
	pub(crate) fn deref(&self, ty: Ty<'tcx>, mutable: bool) -> bool {
		let tcx = self.tcx;
		let lang = tcx.lang_items();
		let tr = match mutable {
			true => lang.deref_mut_trait(),
			false => lang.deref_trait(),
		};
		// Each trait's one method.
		let method = tr.and_then(|tr| tcx.associated_items(tr).in_definition_order().find(|i| i.is_fn()));
		method.is_some_and(|m| self.call(m.def_id, tcx.mk_args(&[ty.into()])))
	}

	fn instance(&self, instance: Instance<'tcx>) -> bool {
		if let Some(&found) = self.seen.borrow().get(&instance) {
			// An open walk met again is recursion, which no bound on its steps holds.
			return found.unwrap_or(false);
		}
		let depth = self.depth.get();
		if !self.tcx.recursion_limit().value_within_limit(depth) {
			return false;
		}
		self.seen.borrow_mut().insert(instance, None);
		self.depth.set(depth + 1);
		let found = self.body(instance);
		self.depth.set(depth);
		self.seen.borrow_mut().insert(instance, Some(found));
		found
	}

	fn body(&self, instance: Instance<'tcx>) -> bool {
		let tcx = self.tcx;
		match instance.def {
			InstanceKind::Intrinsic(def_id) => return computes(tcx.item_name(def_id).as_str()),
			InstanceKind::Item(def_id) => {
				// A constructor only builds its value.
				if matches!(tcx.def_kind(def_id), DefKind::Ctor(..)) {
					return true;
				}
				// A foreign function, or one whose crate kept no MIR, as std's that is
				// neither generic nor inline: what it runs is unknown.
				if !tcx.is_mir_available(def_id) {
					return false;
				}
			}
			// Drop glue of a type with nothing to drop.
			InstanceKind::Shim(ShimKind::DropGlue(_, None)) => return true,
			// A trait object's method, an LLVM intrinsic, and a thread local's initializer:
			// what each runs is unknown, or may be anything.
			InstanceKind::Virtual(..)
			| InstanceKind::LlvmIntrinsic(_)
			| InstanceKind::Shim(ShimKind::ThreadLocal(_)) => return false,
			// Every other shim's MIR rustc builds here, which the walk follows.
			InstanceKind::Shim(_) => {}
		}
		let body = tcx.instance_mir(instance.def);
		let blocks = &body.basic_blocks;
		// A loop, of a lock's spin or a length the caller picks, bounds no number of steps.
		let mut order = vec![usize::MAX; blocks.len()];
		for (at, &block) in blocks.reverse_postorder().iter().enumerate() {
			order[block.index()] = at;
		}
		let reached = |block: BasicBlock| order[block.index()] != usize::MAX;
		let env = TypingEnv::fully_monomorphized();
		let mono = |ty: Ty<'tcx>| {
			instance.try_instantiate_mir_and_normalize_erasing_regions(tcx, env, ty::EarlyBinder::bind(tcx, ty))
		};
		for (block, data) in blocks.iter_enumerated() {
			// Cleanup runs only as a call or a drop unwinds, which none a leaf makes does.
			if data.is_cleanup || !reached(block) {
				continue;
			}
			let terminator = data.terminator();
			let forward = terminator
				.successors()
				.all(|to| blocks[to].is_cleanup || order[to.index()] > order[block.index()]);
			if !forward {
				return false;
			}
			let ok = match &terminator.kind {
				TerminatorKind::Goto { .. }
				| TerminatorKind::SwitchInt { .. }
				| TerminatorKind::Return
				| TerminatorKind::Unreachable
				| TerminatorKind::FalseEdge { .. }
				| TerminatorKind::FalseUnwind { .. } => true,
				TerminatorKind::Call { func, args, .. } | TerminatorKind::TailCall { func, args, .. } => {
					match mono(func.ty(body, tcx)).map(|ty| *ty.kind()) {
						Ok(ty::FnDef(def_id, generics)) => {
							// A copy or fill of as many values as the caller picks is a loop of
							// its own.
							let sized = tcx.intrinsic(def_id).is_some_and(|i| {
								matches!(i.name.as_str(), "copy" | "copy_nonoverlapping" | "write_bytes")
							});
							let counted = !sized || args.get(2).is_some_and(|count| count.node.constant().is_some());
							counted
								&& generics
									.no_bound_vars()
									.is_some_and(|generics| self.call(def_id, generics))
						}
						// A function pointer's callee is unknown.
						_ => false,
					}
				}
				TerminatorKind::Drop { place, .. } => match mono(place.ty(body, tcx).ty) {
					Ok(ty) => self.drop(ty),
					Err(_) => false,
				},
				// An assertion that fails panics, which runs the panic hook.
				TerminatorKind::Assert { .. }
				| TerminatorKind::InlineAsm { .. }
				| TerminatorKind::Yield { .. }
				| TerminatorKind::CoroutineDrop
				| TerminatorKind::UnwindResume
				| TerminatorKind::UnwindTerminate(_) => false,
			};
			// The same copy, lowered to a statement.
			let copies = data.statements.iter().any(|statement| {
				matches!(
					&statement.kind,
					StatementKind::Intrinsic(intrinsic)
						if matches!(&**intrinsic, NonDivergingIntrinsic::CopyNonOverlapping(copy) if copy.count.constant().is_none())
				)
			});
			if !ok || copies {
				return false;
			}
		}
		// A static is shared state beyond the handle's borrow, which is a leaf's only one.
		let mut statics = Statics { tcx, found: false };
		for (block, data) in blocks.iter_enumerated() {
			if !data.is_cleanup && reached(block) {
				statics.visit_basic_block_data(block, data);
			}
		}
		!statics.found
	}

	/// Whether dropping a value of `ty` is a leaf: nothing to drop, or drop glue proven so.
	fn drop(&self, ty: Ty<'tcx>) -> bool {
		let tcx = self.tcx;
		if !ty.needs_drop(tcx, TypingEnv::fully_monomorphized()) {
			return true;
		}
		let glue = tcx.require_lang_item(LangItem::DropGlue, DUMMY_SP);
		self.call(glue, tcx.mk_args(&[ty.into()]))
	}
}

/// Whether core's intrinsic `name` only computes: it runs no other code, waits on nothing,
/// and cannot panic. Any other, such as `catch_unwind`, is no leaf. A copy or fill is one
/// only of a count fixed at its call.
fn computes(name: &str) -> bool {
	name.starts_with("atomic_")
		|| matches!(
			name,
			"add_with_overflow"
				| "aggregate_raw_ptr"
				| "align_of"
				| "align_of_val"
				| "arith_offset"
				| "assume"
				| "bitreverse"
				| "black_box"
				| "bswap"
				| "carrying_mul_add"
				| "cold_path"
				| "copy"
				| "copy_nonoverlapping"
				| "ctlz"
				| "ctlz_nonzero"
				| "ctpop"
				| "cttz"
				| "cttz_nonzero"
				| "discriminant_value"
				| "disjoint_bitor"
				| "exact_div"
				| "forget"
				| "likely"
				| "mul_with_overflow"
				| "needs_drop"
				| "offset"
				| "overflow_checks"
				| "ptr_guaranteed_cmp"
				| "ptr_mask"
				| "ptr_metadata"
				| "ptr_offset_from"
				| "ptr_offset_from_unsigned"
				| "read_via_copy"
				| "rotate_left"
				| "rotate_right"
				| "saturating_add"
				| "saturating_sub"
				| "select_unpredictable"
				| "size_of"
				| "size_of_val"
				| "slice_get_unchecked"
				| "sub_with_overflow"
				| "three_way_compare"
				| "transmute"
				| "transmute_unchecked"
				| "typed_swap_nonoverlapping"
				| "ub_checks"
				| "unchecked_add"
				| "unchecked_div"
				| "unchecked_mul"
				| "unchecked_rem"
				| "unchecked_shl"
				| "unchecked_shr"
				| "unchecked_sub"
				| "unlikely"
				| "unreachable"
				| "variant_count"
				| "wrapping_add"
				| "wrapping_mul"
				| "wrapping_sub"
				| "write_bytes"
				| "write_via_move"
		)
}

/// Finds a use of a static, or of a thread local.
struct Statics<'tcx> {
	tcx: TyCtxt<'tcx>,
	found: bool,
}

impl<'tcx> Visitor<'tcx> for Statics<'tcx> {
	fn visit_const_operand(&mut self, constant: &mir::ConstOperand<'tcx>, _: Location) {
		self.found |= constant.check_static_ptr(self.tcx).is_some();
	}

	fn visit_rvalue(&mut self, rvalue: &Rvalue<'tcx>, location: Location) {
		self.found |= matches!(rvalue, Rvalue::ThreadLocalRef(_));
		self.super_rvalue(rvalue, location);
	}
}
