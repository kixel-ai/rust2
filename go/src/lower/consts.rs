//! Constants and statics: rustc evaluates each. A constant's value is written at each
//! use, as Rust inlines it, and a static is one Go variable, which every use shares.

use rustc_abi::Size;
use rustc_hir::def::DefKind;
use rustc_hir::def_id::DefId;
use rustc_hir::def_id::LOCAL_CRATE;
use rustc_middle::mir::interpret::{AllocId, ConstAllocation, GlobalAlloc, Scalar, alloc_range};
use rustc_middle::mir::{self, ConstValue};
use rustc_middle::ty::{self, Ty};
use rustc_span::Span;

use super::body::Fx;

impl<'a, 'tcx> Fx<'a, 'tcx> {
	/// A constant operand's value.
	pub fn constant(&mut self, c: &mir::ConstOperand<'tcx>, span: Span) -> String {
		let ty = c.ty();
		match c.const_.eval(self.tcx, self.t_env(), span) {
			Ok(val) => self.const_value(val, ty, span),
			Err(_) => self.err(span, "a constant that fails to evaluate"),
		}
	}

	/// The value of a constant `val` of type `ty`.
	pub fn const_value(&mut self, val: ConstValue, ty: Ty<'tcx>, span: Span) -> String {
		let tcx = self.tcx;
		if ty.is_unit() {
			return "struct{}{}".into();
		}
		match ty.kind() {
			ty::FnDef(..) => return "struct{}{}".into(),
			ty::Closure(..) => return format!("{}{{}}", self.ty(ty, span)),
			_ => {}
		}
		// An aggregate, field by field.
		if matches!(ty.kind(), ty::Adt(..) | ty::Tuple(_) | ty::Array(..)) {
			if let ty::Adt(adt, _) = ty.kind()
				&& let Some(mapped) = super::library::mapped(tcx, adt.did())
				&& !mapped.is_runtime_layout()
			{
				return match mapped {
					super::library::Mapped::Runtime("struct{}") => "struct{}{}".into(),
					super::library::Mapped::Vec => format!("{}(nil)", self.ty(ty, span)),
					_ => self.err(span, format!("a constant of type `{ty}`")),
				};
			}
			let Some(d) = tcx.try_destructure_mir_constant_for_user_output(val, ty) else {
				return self.err(span, format!("a constant of type `{ty}` that can't be taken apart"));
			};
			let values: Vec<String> = d.fields.iter().map(|(v, t)| self.const_value(*v, *t, span)).collect();
			let go = self.ty(ty, span);
			return match ty.kind() {
				ty::Adt(adt, args) => {
					let variant = d.variant.unwrap_or(rustc_abi::FIRST_VARIANT);
					self.adt_lit(*adt, variant, *args, &values, &go, span)
				}
				_ => format!("{go}{{{}}}", values.join(", ")),
			};
		}
		match val {
			ConstValue::Scalar(Scalar::Int(int)) => match ty.kind() {
				ty::Bool => (int.to_bits_unchecked() != 0).to_string(),
				ty::Char => format!("rune({})", int.to_u32()),
				ty::Int(_) | ty::Uint(_) => self.int_lit(ty, int.to_bits_unchecked(), span),
				ty::RawPtr(..) if int.is_null() => format!("{}(nil)", self.ty(ty, span)),
				_ => self.err(span, format!("a scalar constant of type `{ty}`")),
			},
			ConstValue::Scalar(Scalar::Ptr(ptr, _)) => {
				let (prov, offset) = ptr.prov_and_relative_offset();
				if let GlobalAlloc::Static(def_id) = tcx.global_alloc(prov.alloc_id())
					&& tcx.static_mutability(def_id) == Some(rustc_hir::Mutability::Mut)
				{
					return self.err(span, format!("`{}`, a `static mut`,", tcx.def_path_str(def_id)));
				}
				let ty::Ref(_, inner, _) = ty.kind() else {
					return self.err(span, format!("a pointer constant of type `{ty}`"));
				};
				match tcx.global_alloc(prov.alloc_id()) {
					GlobalAlloc::Static(def_id)
						if !matches!(tcx.def_kind(def_id), DefKind::Static { nested: true, .. }) =>
					{
						let var = self.static_var(def_id, span);
						match offset.bytes() == 0 && *inner == self.static_ty(def_id) {
							true => format!("&{var}"),
							false => self.err(span, "a reference into part of a static"),
						}
					}
					GlobalAlloc::Memory(_) | GlobalAlloc::Static(_) => {
						let Some(alloc) = self.memory(prov.alloc_id(), span) else {
							return "struct{}{}".into();
						};
						if let ty::Array(elem, len) = inner.kind()
							&& let (Some(len), ty::Uint(ty::UintTy::U8)) = (len.try_to_target_usize(tcx), elem.kind())
						{
							let start = offset.bytes() as usize;
							let bytes = alloc
								.inner()
								.inspect_with_uninit_and_ptr_outside_interpreter(start..start + len as usize);
							let value = format!(
								"[{len}]uint8{{{}}}",
								bytes.iter().map(|b| b.to_string()).collect::<Vec<_>>().join(", ")
							);
							return format!(
								"&{}",
								self.shared(prov.alloc_id(), (offset.bytes(), len, ty), value, span)
							);
						}
						let value = self.const_value(
							ConstValue::Indirect {
								alloc_id: prov.alloc_id(),
								offset,
							},
							*inner,
							span,
						);
						match inner.kind() {
							ty::Str | ty::Slice(_) | ty::Dynamic(..) => value,
							_ => format!(
								"&{}",
								self.shared(prov.alloc_id(), (offset.bytes(), 0, ty), value, span)
							),
						}
					}
					_ => self.err(span, format!("a constant of type `{ty}` pointing into a function")),
				}
			}
			ConstValue::Slice { alloc_id, meta } => {
				if let Some(inner) = ty.builtin_deref(true)
					&& let Some(slice) = self.static_slice(alloc_id, Size::ZERO, meta, inner, span)
				{
					return slice;
				}
				match ty.builtin_deref(true).map(|t| t.kind()) {
					Some(ty::Str | ty::Slice(_)) => match self.slice_bytes(val, ty, span) {
						Some(bytes) => self.shared(alloc_id, (0, bytes.len() as u64, ty), bytes_lit(bytes), span),
						None => "struct{}{}".into(),
					},
					_ => self.err(span, format!("a slice constant of type `{ty}`")),
				}
			}
			ConstValue::ZeroSized => format!("{}{{}}", self.ty(ty, span)),
			ConstValue::Indirect { alloc_id, offset } => self.in_memory(alloc_id, offset, ty, span),
		}
	}
}

impl<'a, 'tcx> Fx<'a, 'tcx> {
	/// The package-level variable holding `value`, the constant memory `alloc` holds at
	/// `at`, an offset, length, and type: each use of one allocation is the same memory,
	/// as Rust's references to it are. A second reference into it at another offset or
	/// type is refused, since a Go variable apart from the first could not alias it.
	fn shared(&mut self, alloc: AllocId, at: (u64, u64, Ty<'tcx>), value: String, span: Span) -> String {
		if let Some((first, name)) = self.t.allocs.get(&alloc) {
			return match *first == at {
				true => name.clone(),
				false => self.err(
					span,
					"a reference into constant memory that another reference points into at another offset or type",
				),
			};
		}
		let name = self.t.fresh("constant");
		self.t.allocs.insert(alloc, (at, name.clone()));
		self.t.declare(LOCAL_CRATE, &name, format!("var {name} = {value}\n"));
		name
	}

	/// The memory of a constant allocation or a static, whose value rustc evaluated.
	fn memory(&mut self, alloc: AllocId, span: Span) -> Option<ConstAllocation<'tcx>> {
		let tcx = self.tcx;
		match tcx.global_alloc(alloc) {
			GlobalAlloc::Memory(memory) => Some(memory),
			GlobalAlloc::Static(def_id) if !tcx.is_foreign_item(def_id) => match tcx.eval_static_initializer(def_id) {
				Ok(memory) => Some(memory),
				Err(_) => {
					self.err(span, "a static that fails to evaluate");
					None
				}
			},
			_ => {
				self.err(span, "a constant pointing outside memory");
				None
			}
		}
	}

	/// The slice of `len` elements of `inner`, a slice or `str`, at `offset` in `alloc`, if
	/// that is a static: its Go variable's array sliced, which aliases it as Rust's does.
	fn static_slice(&mut self, alloc: AllocId, offset: Size, len: u64, inner: Ty<'tcx>, span: Span) -> Option<String> {
		let tcx = self.tcx;
		let GlobalAlloc::Static(def_id) = tcx.global_alloc(alloc) else {
			return None;
		};
		if matches!(tcx.def_kind(def_id), DefKind::Static { nested: true, .. }) {
			return None;
		}
		let var = self.static_var(def_id, span);
		let elem = match inner.kind() {
			ty::Str => tcx.types.u8,
			ty::Slice(elem) => *elem,
			_ => return Some(self.err(span, format!("a constant of type `{inner}` pointing into a static"))),
		};
		let size = tcx
			.layout_of(self.t_env().as_query_input(elem))
			.map_or(0, |layout| layout.size.bytes());
		Some(match self.static_ty(def_id).kind() {
			ty::Array(of, n)
				if *of == elem
					&& size > 0
					&& offset.bytes().is_multiple_of(size)
					&& n.try_to_target_usize(tcx)
						.is_some_and(|n| offset.bytes() / size + len <= n) =>
			{
				let from = offset.bytes() / size;
				format!("{var}[{from}:{}]", from + len)
			}
			_ => self.err(span, "a slice of a static of another type"),
		})
	}

	/// The type of a static.
	pub fn static_ty(&self, def_id: DefId) -> Ty<'tcx> {
		let ty = self.tcx.type_of(def_id).instantiate_identity().skip_norm_wip();
		self.tcx
			.normalize_erasing_regions(self.t_env(), ty::Unnormalized::new_wip(ty))
	}

	/// The Go variable of the static `def_id`, declared at its first use, with the value
	/// rustc evaluated for it: one place, which translated code and Go callers share, as
	/// Rust's uses of a static do. A static whose value reaches back to it is assigned in
	/// an `init`, since Go refuses a variable whose initializer refers to it. `span` is
	/// where it is used.
	pub fn static_var(&mut self, def_id: DefId, span: Span) -> String {
		let tcx = self.tcx;
		if let Some(name) = self.t.statics.get(&def_id) {
			if self.t.initializing.contains(&def_id) {
				self.t.cyclic.insert(def_id);
			}
			return name.clone();
		}
		let path = tcx.def_path_str(def_id);
		let ty = self.static_ty(def_id);
		if tcx.static_mutability(def_id) == Some(rustc_hir::Mutability::Mut) {
			return self.err(span, format!("`{path}`, a `static mut`,"));
		}
		if tcx.is_thread_local_static(def_id) {
			return self.err(span, format!("`{path}`, a thread-local static,"));
		}
		if tcx.is_foreign_item(def_id) {
			return self.err(span, format!("`{path}`, a static another language defines,"));
		}
		if !ty.is_freeze(tcx, self.t_env()) {
			return self.err(
				span,
				format!(
					"`{path}`, a static of `{ty}`, which has interior mutability Go callers would share unsynchronized,"
				),
			);
		}
		let name = match self.t.value_names.get(&def_id) {
			Some(name) => name.clone(),
			None => self.t.fresh(&super::names::static_base(tcx, def_id)),
		};
		self.t.statics.insert(def_id, name.clone());
		self.t.initializing.push(def_id);
		let go = self.ty(ty, span);
		let value = self.const_value(
			ConstValue::Indirect {
				alloc_id: tcx.reserve_and_set_static_alloc(def_id),
				offset: Size::ZERO,
			},
			ty,
			span,
		);
		self.t.initializing.pop();
		let decl = match self.t.cyclic.contains(&def_id) {
			false => format!("var {name} {go} = {value}\n"),
			true => format!("var {name} {go}\n\nfunc init() {{\n\t{name} = {value}\n}}\n"),
		};
		self.t
			.declare(def_id.krate, &name, format!("// {name} is `{path}`.\n{decl}"));
		name
	}

	/// The value of a scalar or a reference of type `ty` at `offset` in a constant's
	/// memory, as a promoted `&1` or `&"text"` holds it.
	fn in_memory(&mut self, alloc_id: AllocId, offset: Size, ty: Ty<'tcx>, span: Span) -> String {
		let tcx = self.tcx;
		match ty.kind() {
			ty::Ref(_, inner, _) if matches!(inner.kind(), ty::Str | ty::Slice(_)) => {
				let Some((to, start, len)) = self.fat_pointer(alloc_id, offset, ty, span) else {
					return "struct{}{}".into();
				};
				if let Some(slice) = self.static_slice(to, start, len, *inner, span) {
					return slice;
				}
				let at = (start.bytes(), len, ty);
				let value = match inner.kind() {
					ty::Str => match self.bytes(to, start, len, span) {
						Some(bytes) => bytes_lit(bytes),
						None => return "struct{}{}".into(),
					},
					// The elements as an array's, which the slice is of.
					_ => {
						let array = Ty::new_array(tcx, inner.sequence_element_type(tcx), len);
						let value = self.const_value(
							ConstValue::Indirect {
								alloc_id: to,
								offset: start,
							},
							array,
							span,
						);
						return format!("{}[:]", self.shared(to, at, value, span));
					}
				};
				self.shared(to, at, value, span)
			}
			ty::Bool | ty::Char | ty::Int(_) | ty::Uint(_) | ty::Ref(..) => {
				let Ok(layout) = tcx.layout_of(self.t_env().as_query_input(ty)) else {
					return self.err(span, format!("a constant of type `{ty}` without a layout"));
				};
				match self.read(alloc_id, offset, layout.size, ty.is_ref(), span) {
					Some(scalar) => self.const_value(ConstValue::Scalar(scalar), ty, span),
					None => self.err(span, format!("a constant of type `{ty}` in memory")),
				}
			}
			_ => self.err(span, format!("a constant of type `{ty}` in memory")),
		}
	}

	/// The scalar of `size` at `at` in `alloc`: a pointer, read with what it points to, or
	/// an integer, without.
	fn read(&mut self, alloc: AllocId, at: Size, size: Size, pointer: bool, span: Span) -> Option<Scalar> {
		let memory = self.memory(alloc, span)?;
		memory
			.inner()
			.read_scalar(&self.tcx, alloc_range(at, size), pointer)
			.ok()
	}

	/// What the reference `ty` to a `str` or slice at `offset` in `alloc` points to: an
	/// allocation, the offset in it, and the length.
	fn fat_pointer(&mut self, alloc: AllocId, offset: Size, ty: Ty<'tcx>, span: Span) -> Option<(AllocId, Size, u64)> {
		let ptr = self.tcx.data_layout.pointer_size();
		match (
			self.read(alloc, offset, ptr, true, span),
			self.read(alloc, offset + ptr, ptr, false, span),
		) {
			(Some(Scalar::Ptr(to, _)), Some(Scalar::Int(len))) => {
				let (prov, start) = to.prov_and_relative_offset();
				Some((prov.alloc_id(), start, len.to_target_usize(self.tcx)))
			}
			_ => {
				self.err(span, format!("a constant of type `{ty}` in memory"));
				None
			}
		}
	}

	/// The `len` bytes at `start` in `alloc`.
	fn bytes(&mut self, alloc: AllocId, start: Size, len: u64, span: Span) -> Option<&'tcx [u8]> {
		let memory = self.memory(alloc, span)?;
		let start = start.bytes() as usize;
		Some(
			memory
				.inner()
				.inspect_with_uninit_and_ptr_outside_interpreter(start..start + len as usize),
		)
	}

	/// The bytes a `&str` or `&[u8]` constant `val` of type `ty` points to, in constant
	/// memory or a static's.
	pub fn slice_bytes(&mut self, val: ConstValue, ty: Ty<'tcx>, span: Span) -> Option<&'tcx [u8]> {
		let (alloc, start, len) = match val {
			ConstValue::Slice { alloc_id, meta } => (alloc_id, Size::ZERO, meta),
			ConstValue::Indirect { alloc_id, offset } => self.fat_pointer(alloc_id, offset, ty, span)?,
			_ => {
				self.err(span, format!("a constant of type `{ty}` outside memory"));
				return None;
			}
		};
		self.bytes(alloc, start, len, span)
	}
}

/// Bytes as a Go `[]uint8` literal, through a string literal.
pub fn bytes_lit(bytes: &[u8]) -> String {
	format!("[]uint8({})", go_string(bytes))
}

/// Bytes as a Go string literal.
pub fn go_string(bytes: &[u8]) -> String {
	let mut s = String::from("\"");
	for &b in bytes {
		match b {
			b'"' => s.push_str("\\\""),
			b'\\' => s.push_str("\\\\"),
			0x20..=0x7e => s.push(b as char),
			_ => s.push_str(&format!("\\x{b:02x}")),
		}
	}
	s.push('"');
	s
}
