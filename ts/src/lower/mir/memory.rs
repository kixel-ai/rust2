//! Values read from the memory rustc evaluated: a `static`'s, and a `const`'s that holds a
//! cell or has no value tree.
//!
//! An atomic is a new `rs.Atomic` of the value read, a one-time cell the runtime's, empty,
//! and a lazy one the runtime's over its function, as const evaluation leaves them. A
//! `MaybeUninit` is what it holds or nothing yet, as [`crate::lower::pointer`] has it, and
//! an `UnsafeCell` a new `rs.Cell` of its contents, which a `Cell` is too.

use rustc_abi::{BackendRepr, Primitive, Size};
use rustc_middle::mir::interpret::{GlobalAlloc, InitChunk, Scalar, alloc_range};
use rustc_middle::mir::{ConstValue, DestructuredConstant};
use rustc_middle::ty::{self, Ty};
use rustc_span::Span;

use super::Fx;
use crate::lower::library::Std;
use crate::ts::Expr;

impl<'tcx> Fx<'_, '_, '_, 'tcx> {
	/// `val`, a `ty` that is one of std's cells, or `None` if `ty` is no such cell.
	pub(super) fn cell_value(&mut self, val: ConstValue, ty: Ty<'tcx>, span: Span) -> Option<Expr> {
		let tcx = self.tcx;
		let ty::Adt(adt, args) = ty.kind() else {
			return None;
		};
		let e = match Std::of(tcx, adt.did()) {
			Some(Std::Cell) => match self.fields(val, ty).as_deref() {
				// A `Cell`'s field is its `UnsafeCell`, which is the same `rs.Cell`.
				Some([(value, inner)]) if !tcx.is_lang_item(adt.did(), rustc_attr_ir::LangItem::UnsafeCell) => {
					self.const_value(*value, *inner, span)
				}
				Some([(value, inner)]) => {
					let value = self.const_value(*value, *inner, span);
					Expr::New(Box::new(Expr::Rs("Cell".into())), vec![value])
				}
				_ => self.unreadable(ty, span),
			},
			Some(Std::Atomic) => {
				let inner = args.type_at(0);
				match self.scalar(val, inner) {
					Some(scalar) => {
						let value = self.const_value(ConstValue::Scalar(scalar), inner, span);
						Expr::New(Box::new(Expr::Rs("Atomic".into())), vec![value])
					}
					None => self.unreadable(ty, span),
				}
			}
			// Const evaluation can't fill one, which only a call made at run time does.
			Some(Std::OnceCell) => match self.variant_through(val, ty) {
				Some(variant) if variant == "None" => crate::lower::task::once_new(self.b, ty),
				_ => self.unreadable(ty, span),
			},
			Some(Std::OnceLock) => match self.fields(val, ty).as_deref() {
				Some([(once, once_ty), ..]) if self.incomplete(*once, *once_ty) => {
					crate::lower::task::once_new(self.b, ty)
				}
				_ => self.unreadable(ty, span),
			},
			Some(Std::Once) if self.incomplete(val, ty) => crate::lower::task::once_unrun(),
			Some(Std::Once) => self.unreadable(ty, span),
			// Its function, which the first `force` runs.
			// wasm-bindgen's, around a lazy of its own: the function, wherever it keeps it.
			Some(Std::LazyCell)
				if let ty::Adt(adt, _) = ty.kind()
					&& crate::lower::bindgen::is_lazy(tcx, adt.did()) =>
			{
				match self.lazy_init(val, ty) {
					Some(init) => self.lazy("LazyCell", init, span),
					None => self.unreadable(ty, span),
				}
			}
			Some(Std::LazyCell) => match self.variant_fields(val, ty) {
				Some((variant, fields)) if variant == "Uninit" => self.lazy("LazyCell", fields[0], span),
				_ => self.unreadable(ty, span),
			},
			Some(Std::LazyLock) => match self.fields(val, ty).as_deref() {
				Some([(once, once_ty), (data, data_ty)]) if self.incomplete(*once, *once_ty) => {
					// A union of the value and the function, `UnsafeCell`'s field, all at its start.
					match self.fields(*data, *data_ty).as_deref() {
						Some([(data, _)]) => self.lazy("LazyLock", (*data, args.type_at(1)), span),
						_ => self.unreadable(ty, span),
					}
				}
				_ => self.unreadable(ty, span),
			},
			Some(Std::MaybeUninit) => {
				let inner = args.type_at(0);
				match self.initialized(val, inner) {
					Some(true) => self.const_value(val, inner, span),
					Some(false) => self.b.t.uninit(ty, span),
					None => self.unreadable(ty, span),
				}
			}
			Some(_) | None => return None,
		};
		Some(e)
	}

	/// A new `rs.LazyCell` or `rs.LazyLock`, `class`, that `f`, an `f_ty`, initializes.
	fn lazy(&mut self, class: &str, (f, f_ty): (ConstValue, Ty<'tcx>), span: Span) -> Expr {
		let f = self.const_value(f, f_ty, span);
		Expr::New(Box::new(Expr::Rs(class.into())), vec![f])
	}

	/// The function a lazy `val`, a `ty`, has yet to run, in whichever of its fields holds it.
	fn lazy_init(&self, val: ConstValue, ty: Ty<'tcx>) -> Option<(ConstValue, Ty<'tcx>)> {
		match ty.kind() {
			ty::FnPtr(..) | ty::FnDef(..) => Some((val, ty)),
			ty::Adt(..) => self
				.destructure(val, ty)?
				.fields
				.iter()
				.find_map(|&(v, t)| self.lazy_init(v, t)),
			_ => None,
		}
	}

	/// The refusal of a `ty` in a state translation can't read, as a `OnceLock` const
	/// evaluation can't have filled.
	fn unreadable(&mut self, ty: Ty<'tcx>, span: Span) -> Expr {
		self.err(
			span,
			format!("a constant of type `{ty}` in a state translation can't read from memory"),
		)
	}

	/// Each field of `val`, a `ty` that rustc can take apart, and its type.
	fn fields(&self, val: ConstValue, ty: Ty<'tcx>) -> Option<Vec<(ConstValue, Ty<'tcx>)>> {
		self.destructure(val, ty).map(|d| d.fields.to_vec())
	}

	fn destructure(&self, val: ConstValue, ty: Ty<'tcx>) -> Option<DestructuredConstant<'tcx>> {
		self.tcx.try_destructure_mir_constant_for_user_output(val, ty)
	}

	/// The name of the variant `val`, an enum `ty`, holds, and its fields.
	fn variant_fields(&self, val: ConstValue, ty: Ty<'tcx>) -> Option<(String, Vec<(ConstValue, Ty<'tcx>)>)> {
		let (val, ty) = self.through(val, ty)?;
		let ty::Adt(adt, _) = ty.kind() else {
			return None;
		};
		let d = self.destructure(val, ty)?;
		Some((adt.variant(d.variant?).name.to_string(), d.fields.to_vec()))
	}

	/// The name of the variant of the enum `val` holds through structs of one field each.
	fn variant_through(&self, val: ConstValue, ty: Ty<'tcx>) -> Option<String> {
		self.variant_fields(val, ty).map(|(name, _)| name)
	}

	/// The enum `val`, a `ty`, holds through structs of one field each, such as a `Once`'s
	/// state in its `Cell`.
	fn through(&self, mut val: ConstValue, mut ty: Ty<'tcx>) -> Option<(ConstValue, Ty<'tcx>)> {
		loop {
			let ty::Adt(adt, _) = ty.kind() else {
				return None;
			};
			if adt.is_enum() {
				return Some((val, ty));
			}
			if !adt.is_struct() {
				return None;
			}
			match self.fields(val, ty)?.as_slice() {
				[(v, t)] => (val, ty) = (*v, *t),
				_ => return None,
			}
		}
	}

	/// Whether `val`, a `Once`, has not run, as only a call at run time makes one run.
	fn incomplete(&self, val: ConstValue, ty: Ty<'tcx>) -> bool {
		self.variant_through(val, ty).is_some_and(|v| v == "Incomplete")
	}

	/// The scalar `val`, a `ty` whose layout is one, holds, read from memory if it is there.
	pub(super) fn scalar(&self, val: ConstValue, ty: Ty<'tcx>) -> Option<Scalar> {
		let tcx = self.tcx;
		match val {
			ConstValue::Scalar(scalar) => Some(scalar),
			ConstValue::Indirect { alloc_id, offset } => {
				let env = self.b.t.typing_env_pub();
				let layout = tcx.layout_of(env.as_query_input(ty)).ok()?;
				let BackendRepr::Scalar(scalar) = layout.backend_repr else {
					return None;
				};
				let GlobalAlloc::Memory(alloc) = tcx.global_alloc(alloc_id) else {
					return None;
				};
				let pointer = matches!(scalar.primitive(), Primitive::Pointer(_));
				alloc
					.inner()
					.read_scalar(&tcx, alloc_range(offset, layout.size), pointer)
					.ok()
			}
			_ => None,
		}
	}

	/// Whether the `ty` at the start of `val` holds a value, `false` if every byte of it is
	/// uninitialized, as `MaybeUninit::uninit()` leaves it.
	fn initialized(&self, val: ConstValue, ty: Ty<'tcx>) -> Option<bool> {
		let tcx = self.tcx;
		let ConstValue::Indirect { alloc_id, offset } = val else {
			return Some(true);
		};
		let env = self.b.t.typing_env_pub();
		let size = tcx.layout_of(env.as_query_input(ty)).ok()?.size;
		if size == Size::ZERO {
			return Some(true);
		}
		let GlobalAlloc::Memory(alloc) = tcx.global_alloc(alloc_id) else {
			return None;
		};
		let mut chunks = alloc
			.inner()
			.init_mask()
			.range_as_init_chunks(alloc_range(offset, size));
		Some(chunks.any(|c| matches!(c, InitChunk::Init(_))))
	}
}
