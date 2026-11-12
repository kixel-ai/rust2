//! Raw pointers, `NonNull`, `MaybeUninit`, and `ManuallyDrop`.
//!
//! A raw pointer is what a `&mut` to its pointee is where JS keeps the pointee where Rust
//! does: the object, a tuple's or the crate's own struct's, or the `rs.Slice` of a
//! `[T; N]`'s elements, an `rs.ByteSlice` of bytes, which may be part of a larger array or
//! a slice.
//! To anything else it is an `rs.Place` of the place holding the value, as a `&mut` to a
//! value JS copies is, which a `Vec` or an `Arc` also is here: its JS object is what it
//! owns, not where Rust keeps it. Null is `rs.NULL`, and a dangling pointer `rs.DANGLING`.
//!
//! An `rs.Place` into an array, slice, or `Vec` is the `rs.AtIndex` of its element, which
//! pointer arithmetic moves within the array. A pointer to an element JS keeps as an
//! object is that object, which knows no array it is in, so arithmetic on one is refused.
//!
//! A reference to a value JS copies is the value, not where it is kept, so a pointer made
//! from one is the place the body shows the reference borrowed, and else a place of its
//! own: of a shared reference's value, which nothing may write while the pointer lives,
//! or writing a whole value into the object a `&mut` is in place.
//!
//! A pointer to `()`, as std erases one, is an identity that compares as itself: null,
//! `rs.DANGLING_UNIT`, or a waker's data. One to a place of `()` or from a reference to
//! one is refused, as an `rs.Place` of it would be an identity of its own each borrow.
//!
//! A whole value written through a pointer to an object is copied into that object, so
//! every pointer to it sees the write. That goes one object deep, as `*r = v` does through
//! a `&mut`, so a pointer to a part kept as an object is refused wherever the crate may
//! give that part another object. One to a variable's object is refused wherever the
//! function may give the variable another after: by assigning it, by writing it whole
//! through an alias of a borrow of it, or by `mem::replace`, `swap`, or `take` of one. A
//! value read out is a copy of what the crate writes into in place, so a later write
//! leaves it be.
//!
//! A `MaybeUninit<T>` is a `T`: `undefined` where JS copies a `T` and it holds none yet,
//! or an object whose parts hold none yet where a pointer to it is the object. A
//! `ManuallyDrop<T>` is a `T` that nothing drops but its own `drop`. An `UnsafeCell<T>` is
//! an `rs.Cell`, which is the place a pointer to `T` is, or else holds the object it is.

use rustc_middle::ty::{self, Ty, TypeVisitableExt};
use rustc_span::Span;

use super::body::Body;
use super::library::{Call, Std, targ};
use super::{AdtRepr, ObjectKind, Translator};
use crate::ts::{Expr, Key, Stmt, StmtKind, Type};

/// What a type that [keeps cells](Translator::keeps_cells) holds.
pub enum KeptCells<'tcx> {
	/// A cell's value.
	Cell(Ty<'tcx>),
	/// Each field's type, and whether it keeps cells.
	Fields(Vec<(Ty<'tcx>, bool)>),
}

/// What a raw pointer to a type is in JS.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Pointee {
	/// An `rs.Place` of the value, which JS copies.
	Place,
	/// The object it points at.
	Object,
	/// An `rs.Slice` of the `[T; N]` it points at, or an `rs.ByteSlice` of bytes, as a
	/// reference to one is, which compares by where its elements are.
	Array,
	/// An `rs.RawSlice` of a `[T]`: the pointer to its first element and its length, which
	/// a read through it makes the slice of; of a `[[T; N]]`, an `rs.RawArraySlice`.
	Slice,
}

impl<'a, 'tcx> Translator<'a, 'tcx> {
	/// What a raw pointer to `ty` is.
	pub fn pointee(&self, ty: Ty<'tcx>) -> Pointee {
		// Only an instance decides what a parameter is.
		if self.polymorphic() && ty.has_param() {
			return Pointee::Object;
		}
		let ty = self.uninit_inner(ty);
		if ty.is_array() {
			return Pointee::Array;
		}
		if ty.is_slice() {
			return Pointee::Slice;
		}
		let object = match (self.object_kind(ty), ty.kind()) {
			(Some(ObjectKind::Array), ty::Tuple(_)) => true,
			// An async body passes its context as a pointer to it, and a waker's vtable is
			// one that is its identity.
			(Some(ObjectKind::Class), ty::Adt(adt, _)) => match Std::of(self.tcx, adt.did()) {
				Some(std) => matches!(std, Std::Context | Std::WakerVTable),
				None => self.bound_drops(adt.did()).is_none(),
			},
			// A `&mut` to one is the object too.
			(Some(ObjectKind::Class), ty::Dynamic(..)) => true,
			_ => false,
		};
		match object {
			true => Pointee::Object,
			false => Pointee::Place,
		}
	}

	/// The runtime's function `name` of the slice a pointer or reference to `ty`, a
	/// `[T; N]` through any `MaybeUninit`s, is: `byteSlice{name}` of bytes, `arraySlice{name}`
	/// of arrays, and else `slice{name}`.
	pub fn array_rs(&self, ty: Ty<'tcx>, name: &str) -> Expr {
		super::array_fn(self.uninit_inner(ty), name)
	}

	/// Whether a pointer to `ty` is an identity: one to `()`, through any `MaybeUninit`s.
	pub fn identity_pointee(&self, ty: Ty<'tcx>) -> bool {
		self.uninit_inner(ty).is_unit()
	}

	/// Why pointers to `ty` that are not null can't compare, if they can't: Rust leaves
	/// where a value of no bytes is unspecified, but a pointer to `()` is an identity. Notes
	/// the comparison of objects, for [`Self::taken`].
	pub fn incomparable(&mut self, ty: Ty<'tcx>) -> Option<String> {
		if self.identity_pointee(ty) {
			return None;
		}
		if self.zero_sized(ty) {
			return Some(format!(
				"comparing pointers to `{ty}`, which has no bytes and so no address of its own"
			));
		}
		if ty.is_trait() {
			return Some(format!(
				"comparing pointers to `{ty}`, each an object JS makes of its own as it unsizes"
			));
		}
		match self.pointee(ty) {
			// `from_ref`, `from_mut` of a variable, and `Option::as_slice` make an array of
			// one of their own for a value, so only longer arrays are always where Rust's are.
			Pointee::Array
				if let ty::Array(_, len) = self.uninit_inner(ty).kind()
					&& len.try_to_target_usize(self.tcx).is_none_or(|n| n == 1) =>
			{
				Some(format!(
					"comparing pointers to `{ty}`, an array of one JS may make of its own for a value, as `array::from_ref` does"
				))
			}
			Pointee::Object | Pointee::Array => {
				self.compare(ty);
				None
			}
			Pointee::Place => Some(format!(
				"comparing pointers to `{ty}`, each a place JS makes of its own"
			)),
			Pointee::Slice => Some(format!(
				"comparing pointers to `{ty}`, whose element pointers are places JS makes of their own"
			)),
		}
	}

	/// What tells whether two pointers to `ty` point at the same place where `===` can't:
	/// the slices' of an array.
	pub fn same_fn(&self, ty: Ty<'tcx>) -> Option<Expr> {
		(self.pointee(ty) == Pointee::Array).then(|| self.array_rs(ty, "Same"))
	}

	/// What a `MaybeUninit` or `ManuallyDrop` holds, through any number of them, or `ty`
	/// itself: JS keeps each as what it holds, and an `UninitSlice` as its slice.
	pub fn uninit_inner(&self, ty: Ty<'tcx>) -> Ty<'tcx> {
		let mut ty = super::stock::uninit_slice(self.tcx, ty).unwrap_or(ty);
		while let ty::Adt(adt, args) = ty.kind()
			&& matches!(Std::of(self.tcx, adt.did()), Some(Std::MaybeUninit | Std::ManuallyDrop))
		{
			ty = args.type_at(0);
		}
		ty
	}

	/// Whether a whole write of a `ty` through the pointer helpers below keeps each
	/// `rs.Cell` it holds, writing into it in place, so a pointer `get` gave into one before
	/// sees the write: a cell, or a tuple or a crate's struct a pointer is the object of, a
	/// field of which keeps cells. Any other part is given the new value's, one object deep.
	pub fn keeps_cells(&self, ty: Ty<'tcx>) -> bool {
		self.kept_cells(ty).is_some()
	}

	/// What a `ty` that [keeps cells](Self::keeps_cells) holds: a cell's value, or each
	/// field of a tuple or struct, and whether it keeps cells too.
	pub fn kept_cells(&self, ty: Ty<'tcx>) -> Option<KeptCells<'tcx>> {
		let (fields, cell) = self.kept_parts(ty)?;
		if cell {
			return Some(KeptCells::Cell(fields[0]));
		}
		let fields: Vec<_> = fields.into_iter().map(|f| (f, self.keeps_cells(f))).collect();
		(self.pointee(ty) == Pointee::Object && fields.iter().any(|&(_, kept)| kept))
			.then_some(KeptCells::Fields(fields))
	}

	/// What a write keeping cells reaches in `ty`, and whether `ty` is a cell: a cell's
	/// value, or each field of a tuple or a crate's struct.
	fn kept_parts(&self, ty: Ty<'tcx>) -> Option<(Vec<Ty<'tcx>>, bool)> {
		match ty.kind() {
			ty::Adt(adt, args) if Std::of(self.tcx, adt.did()) == Some(Std::Cell) => {
				Some((vec![args.type_at(0)], true))
			}
			ty::Tuple(tys) => Some((tys.to_vec(), false)),
			ty::Adt(adt, args) if adt.is_struct() && Std::of(self.tcx, adt.did()).is_none() && self.own(adt.did()) => {
				Some((adt.all_fields().map(|f| self.field_ty(f.did, args)).collect(), false))
			}
			_ => None,
		}
	}

	/// Whether a whole write of a `ty` would put other cells in place of those a
	/// `MaybeUninit` or `ManuallyDrop` in it holds, which a write keeps nowhere, so a pointer
	/// `get` gave into one would not see it: one of them holding a value that
	/// [keeps cells](Self::keeps_cells), or a part such a write reaches holding one.
	pub fn wraps_cells(&self, ty: Ty<'tcx>) -> bool {
		let inner = self.uninit_inner(ty);
		if inner != ty {
			return self.keeps_cells(inner) || self.wraps_cells(inner);
		}
		self.kept_parts(ty)
			.is_some_and(|(parts, _)| parts.into_iter().any(|part| self.wraps_cells(part)))
	}

	/// Whether only an instance decides what a pointer or reference to `ty` is, as in a
	/// generic type's definition: `ty` a type parameter, an array of one, either through
	/// `MaybeUninit`s, or a binding's generic type, which an instance decides is `Copy`.
	pub fn instance_pointee(&self, ty: Ty<'tcx>) -> bool {
		if self.polymorphic() || !ty.has_param() {
			return false;
		}
		let ty = self.uninit_inner(ty);
		match ty.kind() {
			ty::Array(elem, _) => self.open(*elem),
			ty::Adt(adt, _) => self.bound_drops(adt.did()).is_some(),
			_ => self.open(ty),
		}
	}

	/// The TypeScript type of a raw pointer or `NonNull` to `pointee`.
	pub fn pointer_ty(&mut self, pointee: Ty<'tcx>, span: Span) -> Type {
		if self.instance_pointee(pointee) {
			return Type::Rs("Ptr", vec![self.ty(pointee, span)]);
		}
		match self.pointee(pointee) {
			Pointee::Place => Type::Rs("Place", vec![self.ty(pointee, span)]),
			Pointee::Object => self.ty(pointee, span),
			Pointee::Array => match self.uninit_inner(pointee).kind() {
				ty::Array(elem, _) => self.slice_ty(*elem, span),
				_ => unreachable!("an array's pointee"),
			},
			Pointee::Slice => match self.uninit_inner(pointee).kind() {
				ty::Slice(elem) if let ty::Array(inner, _) = elem.kind() => {
					Type::Rs("RawArraySlice", vec![self.slice_ty(*inner, span)])
				}
				ty::Slice(elem) => Type::Rs("RawSlice", vec![self.ty(*elem, span)]),
				_ => unreachable!("a slice's pointee"),
			},
		}
	}

	/// Whether a `ty` has no bytes, so a write of one through any pointer, even a dangling
	/// one, writes nothing.
	pub fn zero_sized(&self, ty: Ty<'tcx>) -> bool {
		let env = self.typing_env_pub();
		self.tcx
			.layout_of(env.as_query_input(ty))
			.is_ok_and(|layout| layout.is_zst())
	}

	/// Whether a `ty` of no bytes is the value [`Self::shell`] makes of it: built of
	/// tuples, arrays, structs, and `()`, with no enum to tag.
	fn empty(&self, ty: Ty<'tcx>) -> bool {
		match ty.kind() {
			ty::Tuple(tys) => tys.iter().all(|t| self.empty(t)),
			ty::Array(elem, _) => self.empty(*elem),
			ty::Adt(adt, args) => match Std::of(self.tcx, adt.did()) {
				Some(Std::MaybeUninit | Std::ManuallyDrop) => self.empty(args.type_at(0)),
				Some(_) => false,
				None => adt.is_struct() && adt.all_fields().all(|f| self.empty(self.field_ty(f.did, args))),
			},
			_ => false,
		}
	}

	/// What `ty` points at, if it is a raw pointer that is an `rs.Place`.
	pub fn raw_place(&self, ty: Ty<'tcx>) -> Option<Ty<'tcx>> {
		match ty.kind() {
			ty::RawPtr(inner, _) if self.pointee(*inner) == Pointee::Place => Some(*inner),
			_ => None,
		}
	}

	/// A `MaybeUninit<ty>` that holds nothing yet: its parts kept as objects are there, so
	/// a pointer to one can initialize it, holding nothing yet themselves.
	pub fn uninit(&mut self, ty: Ty<'tcx>, span: Span) -> Expr {
		let ts_ty = self.ty(ty, span);
		let empty = self.shell(ty, span);
		Expr::As(Box::new(Expr::As(Box::new(empty), Type::Builtin("unknown"))), ts_ty)
	}

	/// What a `MaybeUninit<ty>` that holds nothing yet is, untyped.
	pub fn shell(&mut self, ty: Ty<'tcx>, span: Span) -> Expr {
		// A `MaybeUninit<u8>` is 0, as a typed array's bytes are before a write, so it is never
		// `undefined`.
		if !ty.is_integral() && super::byte(ty) {
			return Expr::lit("0");
		}
		let ty = self.uninit_inner(ty);
		match (self.pointee(ty), ty.kind()) {
			// A cell, so a pointer `raw_get` makes into it before a write sees the write,
			// which lands in it.
			(_, ty::Adt(adt, args)) if Std::of(self.tcx, adt.did()) == Some(Std::Cell) => Expr::New(
				Box::new(Expr::Rs("Cell".into())),
				vec![self.shell(args.type_at(0), span)],
			),
			(Pointee::Object, ty::Tuple(tys)) => Expr::Array(tys.iter().map(|t| self.shell(t, span)).collect()),
			(Pointee::Array, ty::Array(elem, len)) => {
				let len = Expr::lit(len.try_to_target_usize(self.tcx).unwrap_or(0).to_string());
				let cell = matches!(elem.kind(), ty::Adt(adt, _) if Std::of(self.tcx, adt.did()) == Some(Std::Cell));
				match (super::byte(*elem), self.pointee(*elem)) {
					(true, _) => Expr::New(Box::new(Expr::ident("Uint8Array")), vec![len]),
					(_, Pointee::Place) if !cell => Expr::New(Box::new(Expr::ident("Array")), vec![len]),
					(_, Pointee::Place | Pointee::Object | Pointee::Array | Pointee::Slice) => {
						let elem_ty = self.ty(*elem, span);
						let each = Expr::Arrow(
							Vec::new(),
							elem_ty,
							vec![StmtKind::Return(Some(self.uninit(*elem, span))).at(None)],
						);
						let length = Expr::Object(vec![(Key::Name("length".into()), len)]);
						Expr::ident("Array").member("from").call(vec![length, each])
					}
				}
			}
			(Pointee::Object, ty::Adt(adt, args)) => {
				let info = self.adt(*adt, span);
				match info.repr {
					AdtRepr::Struct { ctor, fields } => {
						// A class's constructor types each of its parameters.
						let plain = self.plain(info.sym);
						let values = (adt.non_enum_variant().fields.iter())
							.map(|f| {
								let t = self.field_ty(f.did, args);
								let value = match plain {
									true => self.shell(t, span),
									false => self.uninit(t, span),
								};
								self.field_value(value, adt.did(), f.did, args, span)
							})
							.collect();
						self.construct(info.sym, &ctor, &fields, values)
					}
					AdtRepr::Slots(_) => Expr::Object(Vec::new()),
					_ => unreachable!("only a struct or union is an object a pointer points at"),
				}
			}
			_ => Expr::undefined(),
		}
	}
}

/// Whether std's `def_id` with `args` makes a raw pointer to what its first argument, a
/// reference, points at: `UnsafeCell::get`, `MaybeUninit::as_ptr` and `as_mut_ptr`,
/// `ptr::from_ref` and `from_mut`, and `NonNull::from` and `into` of a reference to its
/// pointee, which only std's impl converts, as any other would overlap it.
pub fn from_reference<'tcx>(
	tcx: ty::TyCtxt<'tcx>,
	def_id: rustc_hir::def_id::DefId,
	args: ty::GenericArgsRef<'tcx>,
) -> bool {
	let Some((owner, name)) = super::library::key(tcx, def_id) else {
		return false;
	};
	match (owner.as_str(), name.as_str()) {
		("UnsafeCell", "get")
		| ("Cell", "as_ptr")
		| ("MaybeUninit", "as_ptr" | "as_mut_ptr")
		| ("ptr", "from_ref" | "from_mut") => true,
		("From", "from") | ("Into", "into") => {
			let (to, from) = match owner.as_str() {
				"From" => (args.type_at(0), args.type_at(1)),
				_ => (args.type_at(1), args.type_at(0)),
			};
			matches!(to.kind(), ty::Adt(adt, to) if Std::of(tcx, adt.did()) == Some(Std::NonNull)
				&& from.ref_mutability().is_some() && from.builtin_deref(true) == Some(to.type_at(0)))
		}
		_ => false,
	}
}

/// What the value std's `def_id` returns points at, if it is a raw pointer, `NonNull`,
/// `MaybeUninit`, `ManuallyDrop`, `UnsafeCell`, or `Box` function that runs none of the
/// crate's code and keeps what it is given nowhere but in what it returns.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Derives {
	/// Where its first argument points, as a whole.
	Same,
	/// Anywhere its arguments reach, if anywhere.
	Other,
	/// Wherever what it reads where its first argument points does.
	Loaded,
}

/// How std's `def_id` derives what it returns, if [`Derives`] describes it.
pub fn derives(tcx: ty::TyCtxt<'_>, def_id: rustc_hir::def_id::DefId) -> Option<Derives> {
	if !super::library::is_std(tcx, def_id) {
		return None;
	}
	let (owner, name) = super::library::key(tcx, def_id)?;
	match (owner.as_str(), name.as_str()) {
		("ptr", "cast" | "cast_const" | "cast_mut" | "as_ref" | "as_mut")
		| ("NonNull", "new" | "new_unchecked" | "as_ptr" | "as_ref" | "as_mut" | "cast" | "cast_const" | "cast_mut")
		| ("MaybeUninit", "as_ptr" | "as_mut_ptr" | "assume_init_ref" | "assume_init_mut" | "write")
		| ("Box", "into_raw" | "leak" | "from_raw" | "write") => Some(Derives::Same),
		(
			"ptr",
			"null" | "null_mut" | "dangling" | "dangling_mut" | "is_null" | "eq" | "add" | "sub" | "offset"
			| "wrapping_add" | "wrapping_sub" | "wrapping_offset" | "write" | "write_unaligned" | "write_volatile",
		)
		| ("NonNull", "dangling" | "add" | "sub" | "offset" | "write")
		| ("MaybeUninit", "new" | "uninit" | "assume_init")
		| ("ManuallyDrop", "new" | "into_inner")
		| (
			"UnsafeCell",
			"new" | "get" | "raw_get" | "get_mut" | "as_ref_unchecked" | "as_mut_unchecked" | "into_inner",
		)
		| ("Cell", "as_ptr" | "get_mut")
		| ("Box", "new") => Some(Derives::Other),
		("ptr", "read" | "read_unaligned" | "read_volatile")
		| ("NonNull", "read")
		| ("MaybeUninit", "assume_init_read")
		| ("ManuallyDrop", "take") => Some(Derives::Loaded),
		_ => None,
	}
}

/// Whether std's `def_id`, which [`derives`] describes, keeps its second argument where its
/// first points.
pub fn stores(tcx: ty::TyCtxt<'_>, def_id: rustc_hir::def_id::DefId) -> bool {
	matches!(
		super::library::key(tcx, def_id),
		Some((owner, name)) if matches!(owner.as_str(), "ptr" | "NonNull" | "MaybeUninit" | "Box")
			&& matches!(name.as_str(), "write" | "write_unaligned" | "write_volatile")
	)
}

/// Whether std's `def_id` returns memory nothing points into yet.
pub fn allocates(tcx: ty::TyCtxt<'_>, def_id: rustc_hir::def_id::DefId) -> bool {
	super::library::is_std(tcx, def_id)
		&& matches!(
			super::library::key(tcx, def_id),
			Some((owner, name)) if matches!(
				(owner.as_str(), name.as_str()),
				("alloc", "alloc" | "alloc_zeroed") | ("Box", "new" | "new_uninit" | "new_zeroed")
			)
		)
}

/// Which of std's `def_id`'s arguments are `&mut`s whose lowering may give what they
/// borrow another value, rather than writing into it, where one is an alias of a borrowed
/// place: the place is assigned again, as `mem::replace`, `clone_from`, and `clone_into`
/// assign it, and a whole write of what `slice::from_mut`, `array::from_mut`,
/// `Cell::from_mut`, or `UnsafeCell::from_mut` makes does. `mem::replace`'s value is no
/// such one.
pub fn rebinds(tcx: ty::TyCtxt<'_>, def_id: rustc_hir::def_id::DefId) -> &'static [usize] {
	match super::library::key(tcx, def_id) {
		Some((owner, name)) => match (owner.as_str(), name.as_str()) {
			("mem", "replace" | "take")
			| ("Clone", "clone_from")
			| ("raw" | "array" | "Cell" | "UnsafeCell", "from_mut") => &[0],
			("ToOwned", "clone_into") => &[1],
			("mem", "swap") => &[0, 1],
			_ => &[],
		},
		_ => &[],
	}
}

/// Whether std's `def_id` is one this module maps that lends a place: it makes the
/// `rs.Place` of the `&mut` it returns.
pub fn lends(tcx: ty::TyCtxt<'_>, def_id: rustc_hir::def_id::DefId) -> bool {
	matches!(
		super::library::key(tcx, def_id),
		Some((owner, name)) if matches!(
			(owner.as_str(), name.as_str()),
			("MaybeUninit", "as_mut_ptr" | "write" | "assume_init_mut" | "assume_init_drop")
				| ("ManuallyDrop", "drop" | "take")
				| ("UnsafeCell" | "Cell", "get_mut")
				| ("UnsafeCell", "as_mut_unchecked")
				| ("NonNull" | "ptr", "as_mut")
				| ("Box", "leak")
		)
	)
}

/// Whether std's `def_id` with `args` is one that only makes a pointer or points one
/// elsewhere, reads through it, or drops what it points at, as this module maps them, or
/// an atomic's, which holds a pointer as a value, so it gives no part another object.
pub fn keeps<'tcx>(tcx: ty::TyCtxt<'tcx>, def_id: rustc_hir::def_id::DefId, args: ty::GenericArgsRef<'tcx>) -> bool {
	let keeps = matches!(
		super::library::key(tcx, def_id),
		Some((owner, name)) if matches!(
			(owner.as_str(), name.as_str()),
			(
				"MaybeUninit",
				"new" | "uninit" | "as_ptr" | "as_mut_ptr" | "assume_init" | "assume_init_ref" | "assume_init_mut"
					| "assume_init_read" | "assume_init_drop"
			) | (
				"NonNull",
				"new" | "new_unchecked" | "dangling" | "as_ptr" | "as_ref" | "as_mut" | "cast" | "cast_const" | "cast_mut"
			) | (
				"ptr",
				"null" | "null_mut" | "dangling" | "dangling_mut" | "is_null" | "as_ref" | "as_mut" | "cast" | "cast_const"
					| "cast_mut" | "eq" | "read" | "read_unaligned" | "read_volatile" | "drop_in_place"
			) | ("Box", "into_raw" | "leak" | "from_raw")
				| ("ManuallyDrop", "new" | "into_inner" | "drop" | "take")
				| (
					"UnsafeCell",
					"new" | "get" | "raw_get" | "get_mut" | "as_ref_unchecked" | "as_mut_unchecked" | "into_inner"
				) | ("Cell", "as_ptr" | "get_mut")
		) || owner == "Atomic" && super::library::is_std(tcx, def_id)
	);
	keeps || from_reference(tcx, def_id, args)
}

/// Whether std's `def_id` of `args` is one a mapping takes the `rs.Place` a `&mut` is for,
/// rather than what it points at: a `MaybeUninit`'s or a `ManuallyDrop`'s, which it points
/// into, an `Arc`'s that `make_mut` replaces, a `String`'s that a drain edits as it drops,
/// a `Result`'s whose payload JS copies, which `iter_mut` lends as a place that writes the
/// whole `Result`, an iterator's that `by_ref` returns, what `borrow_mut` lends, the crate's
/// own iterator's that its `next` reads through, an `IoSlice`'s and a `&mut` to a slice of
/// them, which `advance` and `advance_slices` move, and a well-known crate's, such as a
/// `Bytes` that `Buf::advance` moves past its front.
fn takes_place<'tcx>(
	t: &Translator<'_, 'tcx>,
	def_id: rustc_hir::def_id::DefId,
	args: ty::GenericArgsRef<'tcx>,
) -> bool {
	matches!(
		super::library::key(t.tcx, def_id),
		Some((owner, name)) if matches!(owner.as_str(), "MaybeUninit" | "ManuallyDrop" | "IoSlice")
			|| matches!(owner.as_str(), "Arc" | "Rc") && name == "make_mut"
			|| (owner.as_str(), name.as_str()) == ("String", "drain")
			|| (owner.as_str(), name.as_str()) == ("Iterator", "by_ref")
			|| (owner.as_str(), name.as_str()) == ("BorrowMut", "borrow_mut")
			|| matches!(owner.as_str(), "Iterator" | "DoubleEndedIterator") && super::iter::held_own(t, args.type_at(0))
			|| (owner.as_str(), name.as_str()) == ("Result", "iter_mut") && t.needs_place(args.type_at(0))
			// The `&mut` to a hasher whose methods take it as it is.
			|| (owner.as_str(), name.as_str()) == ("Hash", "hash") && super::collections::called_hasher(t, args.type_at(1))
			|| owner == "Hasher" && super::collections::called_hasher(t, args.type_at(0))
	) || super::stock::is_stock(t.tcx, def_id)
}

/// Which of std's `def_id`'s parameters its mapping of `args` takes as what the `&mut` it
/// declares points at, rather than the `rs.Place` that `&mut` may be: not a type parameter
/// that one instantiates, as `Pin::new`'s pointer.
pub fn pointees<'tcx>(
	t: &Translator<'_, 'tcx>,
	def_id: rustc_hir::def_id::DefId,
	args: ty::GenericArgsRef<'tcx>,
) -> Vec<bool> {
	let takes_place = takes_place(t, def_id, args);
	let sig = t.tcx.fn_sig(def_id).instantiate_identity().skip_norm_wip();
	t.tcx
		.instantiate_bound_regions_with_erased(sig)
		.inputs()
		.iter()
		.map(|t| !takes_place && matches!(t.kind(), ty::Ref(_, _, rustc_ast::Mutability::Mut)))
		.collect()
}

/// A call of std's raw pointer, `NonNull`, `MaybeUninit`, `ManuallyDrop`, `UnsafeCell`, or
/// `Box` functions that make or take a raw pointer or what one points at: its mapping,
/// `None` if it has none, or its arguments back if it is no such call.
pub fn call<'tcx>(
	b: &mut Body<'_, '_, 'tcx>,
	call: &Call<'tcx>,
	owner: &str,
	name: &str,
	args: Vec<Expr>,
	out: &mut Vec<Stmt>,
) -> Result<Option<Expr>, Vec<Expr>> {
	let span = call.span;
	let gargs = call.args;
	let pointee = || targ(gargs, 0);
	let kind = |b: &Body<'_, '_, 'tcx>, ty: Ty<'tcx>| Some(b.t.pointee(ty));
	// A mapping holds a `[T; N]`, and hands out a reference to one, as the JS array, which a
	// pointer to it is an `rs.Slice` of, or an `rs.ByteSlice` of bytes.
	let array = |b: &Body<'_, '_, 'tcx>, ty: Ty<'tcx>| b.t.pointee(ty) == Pointee::Array;
	let to_pointer = |b: &Body<'_, '_, 'tcx>, e: Expr| match b.t.uninit_inner(pointee()).kind() {
		ty::Array(elem, _) => super::slice_items_from(e, *elem),
		_ => super::library::slice_from(e),
	};
	// A reference to the array a pointer points at: the slice the pointer is, or for one a
	// `MaybeUninit` or `ManuallyDrop` holds, whose reference is the array, as a reborrow of
	// `*p` makes it: a view of bytes, and else refused.
	let to_reference = |b: &mut Body<'_, '_, 'tcx>, p: Expr| {
		let ty = pointee();
		let inner = b.t.uninit_inner(ty);
		match inner {
			_ if inner == ty => p,
			_ if super::byte_array(inner) => Expr::rs("byteSliceView", vec![p]),
			_ => b.err(span, uninit_reborrow(ty)),
		}
	};
	// A raw pointer to a slice, from a reference to one, and the slice a read through it
	// makes, shared if nothing writes through it.
	let raw_of = |ty: Ty<'tcx>, s: Expr| match ty {
		_ if slice_of_bytes(ty) => Expr::rs("rawByteSliceOf", vec![s]),
		_ if slice_of_arrays(ty) => Expr::rs("rawArraySliceOf", vec![s]),
		_ => Expr::rs("rawSliceOf", vec![s]),
	};
	let deref = |ty: Ty<'tcx>, p: Expr, shared: bool| match slice_of_bytes(ty) {
		true => p.member("derefBytes").call(vec![Expr::lit(shared.to_string())]),
		false => p.member("deref").call(Vec::new()),
	};
	// `Arc::as_ptr` of `arc`, an `Arc` or an `Rc` as `of` says, of a value no trait object
	// is in: a pointer to a slice or an array it holds, and else the pointer `from_raw` and
	// the counts take back to it, made of `arc` spilled if `spill`. One to an object finds
	// it by that object, which each copy of its type then needs of its own, as a
	// comparison of pointers to it does.
	let arc_ptr = |b: &mut Body<'_, '_, 'tcx>, arc: Expr, of: &str, spill: bool, out: &mut Vec<Stmt>| {
		match kind(b, pointee()) {
			Some(Pointee::Slice) => return raw_of(pointee(), arc.member("value")),
			Some(Pointee::Array) => return to_pointer(b, arc.member("value")),
			_ => {}
		}
		let arc = match spill {
			true => b.spill(arc, "arc", out),
			false => arc,
		};
		let p = match kind(b, pointee()) {
			Some(Pointee::Place) => Expr::rs("arcPlace", vec![arc.clone()]),
			_ => {
				b.t.compare(pointee());
				arc.clone().member("value")
			}
		};
		Expr::rs("arcRaw", vec![arc, p, Expr::Str(of.into())])
	};
	let mut args = args.into_iter();
	let mut arg = || args.next().expect("an argument");
	let e = match (owner, name) {
		("ptr", "null" | "null_mut") => Expr::Rs("NULL".into()),
		// A pointer to `()` is an identity, which the `rs.Place` of a place or a reference
		// is not. A box of one holds nothing, so its pointer is dangling.
		("MaybeUninit", "as_ptr" | "as_mut_ptr") | ("UnsafeCell", "get" | "raw_get") | ("Cell", "as_ptr")
			if b.t.identity_pointee(pointee()) =>
		{
			b.err(
				span,
				"a raw pointer to a place of `()`, which has no address of its own",
			)
		}
		("ptr", "from_ref") if b.t.identity_pointee(pointee()) => b.err(
			span,
			"a raw pointer to a place of `()`, which has no address of its own",
		),
		("From", "from") if pointer_self(b, call).is_some_and(|t| b.t.identity_pointee(t)) => b.err(
			span,
			"a raw pointer to a place of `()`, which has no address of its own",
		),
		("Box", "into_raw" | "leak") if b.t.identity_pointee(pointee()) => Expr::Rs("DANGLING_UNIT".into()),
		// One to a value of no bytes points at one, which every read of it reads. One to
		// `()` is an identity, the same for every call, as its address is.
		("NonNull", "dangling") | ("ptr", "dangling" | "dangling_mut") if b.t.identity_pointee(pointee()) => {
			Expr::Rs("DANGLING_UNIT".into())
		}
		("NonNull", "dangling") | ("ptr", "dangling" | "dangling_mut") if b.t.zero_sized(pointee()) => {
			match (b.t.empty(pointee()), kind(b, pointee())) {
				(false, _) => b.err(
					span,
					format!(
						"a dangling pointer to `{}`, a value of no bytes made of more than structs and tuples",
						pointee()
					),
				),
				(true, Some(Pointee::Place)) => Expr::Object(vec![(Key::Name("v".into()), b.t.shell(pointee(), span))]),
				(true, Some(Pointee::Array)) => {
					let array = b.t.uninit(pointee(), span);
					to_pointer(b, array)
				}
				(true, _) => b.t.uninit(pointee(), span),
			}
		}
		("NonNull", "dangling") | ("ptr", "dangling" | "dangling_mut") => Expr::Rs("DANGLING".into()),
		("ptr" | "NonNull", "cast") => {
			let p = arg();
			cast(b, p, pointee(), targ(gargs, 1), span)
		}
		("ptr" | "NonNull", "addr" | "expose_provenance" | "with_addr" | "map_addr" | "without_provenance") => {
			b.err(span, "a pointer's address, which JS has no counterpart of")
		}
		// A raw pointer to a slice is null where its first element's is.
		("ptr", "is_null") if kind(b, pointee()) == Some(Pointee::Slice) => {
			arg().member("ptr").binary("===", Expr::Rs("NULL".into()))
		}
		// Only a pointer to a slice has these, generic over its element.
		("ptr" | "NonNull", "len") => arg().member("length"),
		("ptr" | "NonNull", "is_empty") => arg().member("length").binary("===", Expr::lit("0")),
		("ptr", "is_null") => arg().binary("===", Expr::Rs("NULL".into())),
		("NonNull", "as_ref" | "as_mut") if kind(b, pointee()) == Some(Pointee::Slice) => {
			deref(pointee(), arg(), name == "as_ref")
		}
		("NonNull", "as_mut") if array(b, pointee()) => {
			let p = arg();
			to_reference(b, p)
		}
		// A `&mut` to a `MaybeUninit` of a cell of a value JS copies is the place the cell is
		// kept in, but one to the cell is the cell.
		("MaybeUninit", "assume_init_mut")
			if b.t.needs_place(
				b.t.tcx
					.type_of(b.t.tcx.parent(call.def_id))
					.instantiate(b.t.tcx, gargs)
					.skip_norm_wip(),
			) && !b.t.needs_place(pointee()) =>
		{
			let ty = pointee();
			b.err(
				span,
				format!(
					"`MaybeUninit::assume_init_mut` of `{ty}`, whose mutable reference is the object, but the uninitialized value's is the place it is kept in"
				),
			)
		}
		// The same pointer, as another type of the same pointee.
		("ptr", "cast_const" | "cast_mut")
		| ("NonNull", "as_ptr" | "new_unchecked" | "cast_const" | "cast_mut")
		| ("MaybeUninit", "new" | "assume_init" | "assume_init_mut")
		| ("ManuallyDrop", "new" | "into_inner")
		| ("NonNull", "as_mut")
		| ("Clone", "clone")
			if owner != "Clone" || pointer_self(b, call).is_some() =>
		{
			arg()
		}
		// A shared reference to a value JS copies is the value, which no write changes.
		("From", "from") if let Some(ty) = pointer_self(b, call) => {
			let from = targ(gargs, 1);
			match (kind(b, ty), from.kind()) {
				(Some(Pointee::Place), ty::Ref(_, _, ty::Mutability::Not)) => {
					Expr::Object(vec![(crate::ts::Key::Name("v".into()), arg())])
				}
				// A reference to a `MaybeUninit` of an array is the JS array.
				(Some(Pointee::Array), _) if super::array_ref(from).is_none() => to_pointer(b, arg()),
				(Some(Pointee::Slice), _) => raw_of(ty, arg()),
				_ => arg(),
			}
		}
		// As `NonNull::from` makes one, of a `[u8; N]` given as the view of its bytes.
		("ptr", "from_ref" | "from_mut") if kind(b, pointee()) == Some(Pointee::Slice) => raw_of(pointee(), arg()),
		("ptr", "from_ref") => match kind(b, pointee()) {
			Some(Pointee::Place) => Expr::Object(vec![(crate::ts::Key::Name("v".into()), arg())]),
			Some(Pointee::Array) => to_pointer(b, arg()),
			_ => arg(),
		},
		("NonNull", "new") => {
			let p = b.spill(arg(), "pointer", out);
			let at = match kind(b, pointee()) {
				Some(Pointee::Slice) => p.clone().member("ptr"),
				_ => p.clone(),
			};
			Expr::Cond(
				Box::new(at.binary("===", Expr::Rs("NULL".into()))),
				Box::new(Expr::undefined()),
				Box::new(p),
			)
		}
		("ptr", "as_ref" | "as_mut") if kind(b, pointee()) == Some(Pointee::Slice) => {
			let p = b.spill(arg(), "pointer", out);
			Expr::Cond(
				Box::new(p.clone().member("ptr").binary("===", Expr::Rs("NULL".into()))),
				Box::new(Expr::undefined()),
				Box::new(deref(pointee(), p, name == "as_ref")),
			)
		}
		// `Option<&T>`: a shared reference to a value JS copies is the value.
		("ptr", "as_ref" | "as_mut") => {
			let p = b.spill(arg(), "pointer", out);
			let shared = name == "as_ref" && kind(b, pointee()) == Some(super::pointer::Pointee::Place);
			let (some, none) = match shared {
				true => (p.clone().member("v"), b.t.none(pointee())),
				// The `None` of an `Option` of the reference, which may be `rs.None` where the
				// reference is the array a `MaybeUninit` holds.
				false if array(b, pointee()) => {
					let lent = b.t.sig(call.def_id, gargs).output();
					let none = match lent.kind() {
						ty::Adt(_, args) => b.t.none(args.type_at(0)),
						_ => Expr::undefined(),
					};
					(to_reference(b, p.clone()), none)
				}
				false => (p.clone(), Expr::undefined()),
			};
			Expr::Cond(
				Box::new(p.binary("===", Expr::Rs("NULL".into()))),
				Box::new(none),
				Box::new(some),
			)
		}
		("NonNull", "as_ref") | ("MaybeUninit", "assume_init_ref") => match kind(b, pointee()) {
			Some(Pointee::Place) if owner == "NonNull" => arg().member("v"),
			Some(Pointee::Array) if owner == "NonNull" => {
				let p = arg();
				to_reference(b, p)
			}
			_ => arg(),
		},
		// An `rs.Arc` is its allocation, which has one place of its value: the pointer to a
		// value JS copies and to `()`, whose pointer is an identity. Otherwise the value is
		// where a reference to it points, a pointer to which is made as `ptr::from_ref`
		// makes one.
		("Arc" | "Rc", "as_ptr" | "into_raw" | "from_raw" | "increment_strong_count" | "decrement_strong_count")
		| ("Weak", "as_ptr")
			if pointee().is_trait() || super::dynamic::unsized_object(b.t.tcx, pointee()) =>
		{
			b.err(span, format!("a raw pointer to the `{}` an `Arc` holds", pointee()))
		}
		("Arc" | "Rc", "as_ptr") if matches!(kind(b, pointee()), Some(Pointee::Slice | Pointee::Array)) => {
			arc_ptr(b, arg(), owner, false, out)
		}
		// A raw pointer `from_raw` takes back to its `Arc` is one JS keeps the identity of.
		("Arc" | "Rc", "into_raw" | "from_raw" | "increment_strong_count" | "decrement_strong_count")
			if matches!(kind(b, pointee()), Some(Pointee::Slice | Pointee::Array)) =>
		{
			b.err(
				span,
				format!(
					"an `{owner}`'s raw pointer to `{}`, which JS makes anew each time",
					pointee()
				),
			)
		}
		("Arc" | "Rc", "as_ptr" | "into_raw") => arc_ptr(b, arg(), owner, true, out),
		// The pointer of the `Arc` it is of, as `Arc::as_ptr` gives it, whether or not any
		// strong one is left. One of `Weak::new`'s has no allocation, which std's dangling
		// address says. While `new_cyclic` makes the value, a value JS keeps as an object
		// isn't there to point at yet, so the translator refuses those, once it has seen
		// every `new_cyclic`.
		("Weak", "as_ptr") if kind(b, pointee()) == Some(Pointee::Slice) => b.err(
			span,
			format!(
				"`Weak::as_ptr` of `{}`, whose length JS keeps nowhere for a `Weak::new` unsized to one",
				pointee()
			),
		),
		("Weak", "as_ptr") => {
			let tcx = b.t.tcx;
			let imp = tcx.inherent_impl_of_assoc(call.def_id).expect("an inherent function");
			let of = match tcx.type_of(imp).skip_binder().kind() {
				ty::Adt(adt, _) if tcx.is_diagnostic_item(rustc_span::sym::ArcWeak, adt.did()) => "Arc",
				_ => "Rc",
			};
			if kind(b, pointee()) != Some(Pointee::Place) {
				b.t.weak_pointer(pointee(), of, span);
			}
			// A pointer to `()` is the `*const ()` every pointer there casts to.
			let dangling = match b.t.identity_pointee(pointee()) {
				true => "WEAK_DANGLING_UNIT",
				false => "WEAK_DANGLING",
			};
			let arc = b.spill(Expr::rs("weakArc", vec![arg()]), "arc", out);
			let p = arc_ptr(b, arc.clone(), of, false, out);
			Expr::Cond(
				Box::new(arc.binary("===", Expr::undefined())),
				Box::new(Expr::Rs(dangling.into())),
				Box::new(p),
			)
		}
		("Arc" | "Rc", "from_raw") => Expr::rs("arcFromRaw", vec![arg(), Expr::Str(owner.into())]),
		("Arc" | "Rc", "increment_strong_count") => {
			let arc = Expr::rs("arcFromRaw", vec![arg(), Expr::Str(owner.into())]);
			out.push(StmtKind::Expr(Expr::rs("arcClone", vec![arc])).at(None));
			Expr::undefined()
		}
		// The `Arc` `from_raw` takes back, dropped.
		("Arc" | "Rc", "decrement_strong_count") => {
			let tcx = b.t.tcx;
			let imp = tcx.inherent_impl_of_assoc(call.def_id).expect("an inherent function");
			let ty = tcx.type_of(imp).instantiate(tcx, gargs).skip_norm_wip();
			let arc = Expr::rs("arcFromRaw", vec![arg(), Expr::Str(owner.into())]);
			match b.t.drop_value(arc.clone(), ty, span) {
				Some(dropped) => dropped,
				None => Expr::rs("arcDrop", vec![arc]),
			}
		}
		// A pointer to what a box held, which no box holds any longer.
		("Box", "into_raw" | "leak") => match kind(b, pointee()) {
			Some(Pointee::Slice) if name == "into_raw" => raw_of(pointee(), arg()),
			Some(Pointee::Place) => Expr::Object(vec![(crate::ts::Key::Name("v".into()), arg())]),
			Some(Pointee::Array) if name == "into_raw" => to_pointer(b, arg()),
			_ => arg(),
		},
		("Box", "from_raw") => match kind(b, pointee()) {
			Some(Pointee::Slice) => deref(pointee(), arg(), false),
			Some(Pointee::Place) => arg().member("v"),
			// A box of an array is the array, all of which `into_raw` pointed at. One of no
			// bytes owns no allocation, so any pointer of any length makes one: a new array, as
			// a dangling pointer's is. Arrays `as_chunks` split from one flat array have no
			// array of their own to box.
			Some(Pointee::Array) => match b.t.uninit_inner(pointee()) {
				ty if super::byte_array(ty) => Expr::rs("byteSliceView", vec![arg()]),
				ty if b.t.zero_sized(ty) => match ty.kind() {
					ty::Array(elem, len) if b.t.empty(*elem) || len.try_to_target_usize(b.t.tcx) == Some(0) => {
						b.t.uninit(pointee(), span)
					}
					_ => b.err(
						span,
						format!("`Box::from_raw` of `{ty}`, a value of no bytes made of more than structs and tuples"),
					),
				},
				ty if super::array_slice::of_arrays(ty) => b.err(
					span,
					format!(
						"`Box::from_raw` of `{ty}`, which may be arrays `as_chunks` split from one flat array, which is not supported yet"
					),
				),
				_ => Expr::rs("boxArrayFromRaw", vec![arg()]),
			},
			_ => arg(),
		},
		// Refused here rather than in `read`, `replace`, and `swap`, which the safe `Cell`
		// methods share, as only an unsafe move leaves a pointer into the original.
		("ptr", "read" | "read_unaligned" | "read_volatile" | "replace" | "swap")
			if !b.t.zero_sized(pointee())
				&& let Some(refused) = unsplit(b, pointee(), span) =>
		{
			refused
		}
		("ptr", "read" | "read_unaligned" | "read_volatile") => {
			let p = arg();
			read(b, p, pointee(), span, out)
		}
		("ptr", "write" | "write_unaligned" | "write_volatile") => {
			let (p, value) = (arg(), arg());
			write(b, p, value, pointee(), span, out)
		}
		// A place of no bytes has nothing to read or write, and every value of its type is
		// the one it holds.
		("ptr", "replace") if b.t.zero_sized(pointee()) => {
			let _ = arg();
			arg()
		}
		("ptr", "swap") if b.t.zero_sized(pointee()) => Expr::undefined(),
		("ptr", "replace") => {
			let (p, value) = (arg(), arg());
			replace(b, p, value, pointee(), span, out)
		}
		("ptr", "swap") => {
			let (x, y) = (arg(), arg());
			swap(b, x, y, pointee(), span, out)
		}
		("ptr", "drop_in_place") => {
			let p = arg();
			let value = match kind(b, pointee()) {
				Some(Pointee::Place) => p.member("v"),
				Some(Pointee::Slice) => deref(pointee(), p, false),
				_ => p,
			};
			b.t.drop_value(value, pointee(), span).unwrap_or_else(Expr::undefined)
		}
		("ptr", "eq") => {
			let (x, y) = (arg(), arg());
			compare(b, x, y, pointee(), span)
		}
		// Only the addresses, which JS compares only between pointers to one type.
		("ptr", "addr_eq") if pointee() != targ(gargs, 1) => b.err(
			span,
			format!(
				"comparing the addresses of pointers to `{}` and to `{}`, which JS keeps apart",
				pointee(),
				targ(gargs, 1)
			),
		),
		("ptr", "addr_eq") => {
			let (x, y) = (arg(), arg());
			compare(b, x, y, pointee(), span)
		}
		("PartialEq", "eq" | "ne") if let Some(ty) = pointer_self(b, call) => {
			let (x, y) = (arg(), arg());
			let same = compare(b, x, y, ty, span);
			match name {
				"eq" => same,
				_ => same.not(),
			}
		}
		("MaybeUninit", "uninit") => b.t.uninit(Ty::new_maybe_uninit(b.t.tcx, pointee()), span),
		// A shared reference to a value JS copies is the value, which no write changes.
		("MaybeUninit", "as_ptr") if kind(b, pointee()) == Some(Pointee::Place) => {
			Expr::Object(vec![(crate::ts::Key::Name("v".into()), arg())])
		}
		("MaybeUninit", "as_ptr" | "as_mut_ptr") if array(b, pointee()) => to_pointer(b, arg()),
		("MaybeUninit", "as_ptr" | "as_mut_ptr") => arg(),
		("MaybeUninit", "write") => {
			let p = b.spill(arg(), "uninit", out);
			let value = arg();
			let write = match array(b, pointee()) {
				true => Expr::rs("ptrWrite", vec![p.clone(), value]),
				false => write(b, p.clone(), value, pointee(), span, out),
			};
			out.push(StmtKind::Expr(write).at(None));
			p
		}
		// The box, written into as `MaybeUninit::write` writes, so a pointer into it from
		// before still points into what it holds; one of a value JS copies is that value.
		("Box", "write") => {
			let boxed = arg();
			let value = arg();
			match kind(b, pointee()) {
				Some(Pointee::Place) => value,
				_ => {
					let p = b.spill(boxed, "uninit", out);
					let write = match array(b, pointee()) {
						true => Expr::rs("ptrWrite", vec![p.clone(), value]),
						false => write(b, p.clone(), value, pointee(), span, out),
					};
					out.push(StmtKind::Expr(write).at(None));
					p
				}
			}
		}
		// A shared reference to a value JS copies is the value.
		("MaybeUninit", "assume_init_read") if let Some(refused) = unsplit(b, pointee(), span) => refused,
		("MaybeUninit", "assume_init_read") if b.t.keeps_cells(pointee()) => copy_kept(b, arg(), pointee(), span, out),
		("MaybeUninit", "assume_init_read") => b.t.snapshot(arg(), pointee(), span),
		("MaybeUninit", "assume_init_drop") | ("ManuallyDrop", "drop") => {
			let p = arg();
			let value = match kind(b, pointee()) {
				Some(Pointee::Place) => p.member("v"),
				_ => p,
			};
			b.t.drop_value(value, pointee(), span).unwrap_or_else(Expr::undefined)
		}
		// What it holds, which it leaves for nothing to read or drop again.
		("ManuallyDrop", "take") => {
			let p = arg();
			if let Some(refused) = unsplit(b, pointee(), span) {
				return Ok(Some(refused));
			}
			let value = match kind(b, pointee()) {
				Some(Pointee::Place) => p.member("v"),
				_ => p,
			};
			match b.t.keeps_cells(pointee()) {
				true => copy_kept(b, value, pointee(), span, out),
				false => b.t.snapshot(value, pointee(), span),
			}
		}
		// An `rs.Cell`, a `Cell`'s too, which is the place a pointer to a value JS copies is,
		// and holds the object a pointer to any other is.
		("UnsafeCell", "new") => Expr::New(Box::new(Expr::Rs("Cell".into())), vec![arg()]),
		("UnsafeCell", "get") | ("Cell", "as_ptr") => {
			let cell = arg();
			cell_pointer(b, cell, pointee())
		}
		// `get` of the cell the pointer points at: the cell itself where a pointer to one is
		// the object, or else the cell in the `rs.Place` of where it is kept. Only a pointer
		// made of a known place gets here, so the cell is there.
		("UnsafeCell", "raw_get") => {
			let tcx = b.t.tcx;
			let cell = tcx
				.type_of(tcx.parent(call.def_id))
				.instantiate(tcx, gargs)
				.skip_norm_wip();
			let p = arg();
			let cell = match b.t.pointee(cell) {
				Pointee::Object => p,
				_ => p.member("v"),
			};
			cell_pointer(b, cell, pointee())
		}
		// `ptr::replace` of the pointer `get` gives.
		("UnsafeCell", "replace") if b.t.zero_sized(pointee()) => {
			let _ = arg();
			arg()
		}
		("UnsafeCell", "replace") => {
			let cell = arg();
			let p = cell_pointer(b, cell, pointee());
			let value = arg();
			replace(b, p, value, pointee(), span, out)
		}
		// A whole write through the `&mut` lands in the allocation, which a value read out
		// before could keep only as a copy, another allocation.
		("UnsafeCell" | "Cell", "get_mut") | ("UnsafeCell", "as_mut_unchecked") if b.t.boxed_object(pointee()) => {
			let ty = pointee();
			b.err(
				span,
				format!(
					"`{owner}::{name}` of `{ty}`, an object a whole write through the reference changes in place, so a value read out before would be a copy at another address"
				),
			)
		}
		// What `get` points at, lent as `get_mut` lends it.
		("UnsafeCell" | "Cell", "get_mut") | ("UnsafeCell", "as_mut_unchecked") if b.t.needs_place(pointee()) => arg(),
		// The pointer `get` gives to an array is the reference to it, which `lend` leaves as it
		// is; one a `MaybeUninit` holds is the array, which the cell holds.
		("UnsafeCell", "get_mut" | "as_ref_unchecked" | "as_mut_unchecked") | ("Cell", "get_mut")
			if pointee().is_array() =>
		{
			let cell = arg();
			cell_pointer(b, cell, pointee())
		}
		("UnsafeCell", "get_mut" | "as_ref_unchecked" | "as_mut_unchecked" | "into_inner") | ("Cell", "get_mut") => {
			arg().member("v")
		}
		// Into the UTF-8 of a `str`, which JS keeps as text: an encoding no write may reach.
		("str", "as_ptr") => Expr::rs("strAsPtr", vec![arg()]),
		// Pointers into an array: `rs.AtIndex`es, which a `Vec`'s and a slice's give.
		("slice" | "Vec", "as_ptr" | "as_mut_ptr") => {
			let elem = targ(gargs, 0);
			let s = arg();
			if let Some(why) = unmodeled(b, elem, "a pointer into") {
				return Ok(Some(b.err(span, why)));
			}
			match (owner, super::byte(elem)) {
				("slice", true) => Expr::rs("byteSliceAsPtr", vec![s]),
				("Vec", true) => Expr::rs("byteVecAsPtr", vec![s]),
				("slice", false) => Expr::rs("sliceAsPtr", vec![s]),
				// A `Vec`'s reaches its room, up to the capacity its size grows.
				_ => Expr::rs("vecAsPtr", vec![s, super::iter::size_of(b, elem, span)]),
			}
		}
		("ptr" | "NonNull", "add" | "sub" | "offset" | "wrapping_add" | "wrapping_sub" | "wrapping_offset") => {
			let (p, n) = (arg(), arg());
			if b.t.zero_sized(pointee()) {
				// Every pointer to a value of no bytes is where it was.
				out.push(StmtKind::Expr(n).at(None));
				p
			} else if let Some(why) = unmodeled(b, pointee(), "an offset of a pointer to") {
				b.err(span, why)
			} else {
				let n = match name {
					"sub" | "wrapping_sub" => Expr::unary("-", n),
					_ => n,
				};
				match name.starts_with("wrapping") {
					true => match super::iter::layout(b, pointee(), span) {
						Ok((size, align)) => Expr::rs(
							"ptrWrappingAdd",
							vec![p, n, Expr::lit(size.to_string()), Expr::lit(align.to_string())],
						),
						Err(e) => e,
					},
					false => Expr::rs("ptrAdd", vec![p, n]),
				}
			}
		}
		("ptr" | "NonNull", "offset_from" | "offset_from_unsigned") => {
			let (p, origin) = (arg(), arg());
			match unmodeled(b, pointee(), "`offset_from` of pointers to") {
				// std asserts that the pointee has bytes.
				_ if b.t.zero_sized(pointee()) => b.err(
					span,
					format!("`offset_from` of pointers to `{}`, which has no bytes", pointee()),
				),
				Some(why) => b.err(span, why),
				None => Expr::rs("ptrOffsetFrom", vec![p, origin]),
			}
		}
		(
			"ptr",
			"copy"
			| "copy_nonoverlapping"
			| "copy_to"
			| "copy_to_nonoverlapping"
			| "copy_from"
			| "copy_from_nonoverlapping",
		)
		| ("NonNull", "copy_to" | "copy_to_nonoverlapping" | "copy_from" | "copy_from_nonoverlapping") => {
			let (x, y, n) = (arg(), arg(), arg());
			let (src, dst) = match name {
				"copy_from" | "copy_from_nonoverlapping" => (y, x),
				_ => (x, y),
			};
			if b.t.zero_sized(pointee()) {
				out.push(StmtKind::Expr(n).at(None));
				Expr::undefined()
			} else if let Some(why) = unmodeled(b, pointee(), "a copy between pointers to") {
				b.err(span, why)
			} else if let Some(refused) = unsplit(b, pointee(), span) {
				refused
			} else {
				let mut args = vec![src, dst, n];
				args.extend(b.t.snapshot_fn(pointee(), span));
				Expr::rs("ptrCopy", args)
			}
		}
		("ptr" | "NonNull", "write_bytes") => {
			let (p, val, n) = (arg(), arg(), arg());
			if b.t.zero_sized(pointee()) {
				out.push(StmtKind::Expr(n).at(None));
				Expr::undefined()
			} else if super::byte(pointee()) {
				Expr::rs("ptrWriteBytes", vec![p, val, n])
			} else {
				// Only a byte is the byte it is set to: JS keeps any other element as a value.
				b.err(
					span,
					format!(
						"`write_bytes` of a pointer to `{}`, whose bytes JS keeps as a value",
						pointee()
					),
				)
			}
		}
		("raw", "from_raw_parts" | "from_raw_parts_mut")
		| ("ptr", "slice_from_raw_parts" | "slice_from_raw_parts_mut")
		| ("NonNull", "slice_from_raw_parts") => {
			let (p, len) = (arg(), arg());
			match unmodeled(b, pointee(), "a slice from a pointer to") {
				_ if b.t.zero_sized(pointee()) => {
					return Ok(Some(b.err(
						span,
						format!("a slice of `{}`, which has no bytes, from a pointer", pointee()),
					)));
				}
				Some(why) => return Ok(Some(b.err(span, why))),
				// A raw pointer to a slice is where it starts and how long it is, which
				// nothing checks until a read through it.
				None if owner != "raw" => Expr::New(Box::new(Expr::Rs("RawSlice".into())), vec![p, len]),
				None if super::byte(pointee()) => {
					let shared = name == "from_raw_parts";
					Expr::rs("ptrByteSlice", vec![p, len, Expr::lit(shared.to_string())])
				}
				None => Expr::rs("ptrSlice", vec![p, len]),
			}
		}
		("mem", "forget") => Expr::undefined(),
		_ => return Err(args.collect()),
	};
	Ok(Some(e))
}

/// Why a pointer to `ty` can't be erased to a `*const ()`, an identity that compares as
/// itself, if it can't: one JS reaches through a slice's places, or a view of bytes, is a
/// new one each borrow, and one to a value of no fixed size loses its size. Of a place,
/// only an `Arc`'s is the value's one, which the body must show.
pub fn unerasable<'tcx>(b: &Body<'_, '_, 'tcx>, ty: Ty<'tcx>) -> Option<String> {
	if b.t.polymorphic() && ty.has_param() {
		return None;
	}
	let erase = format!("a pointer to `{ty}` cast to one to `()`");
	if !ty.is_sized(b.t.tcx, b.t.typing_env_pub()) {
		return Some(format!("{erase}, which loses its size"));
	}
	if b.t.zero_sized(ty) {
		return Some(format!("{erase}, which has no bytes and so no address of its own"));
	}
	let what = match b.t.pointee(ty) {
		_ if b.t.erased(ty) != ty => return Some(format!("{erase}, which JS keeps as what it holds")),
		Pointee::Object => {
			return shares_start(b, ty).map(|part| {
				format!(
					"{erase}, whose address its `{part}` at its start shares, which JS keeps as an object of its own"
				)
			});
		}
		Pointee::Place => return None,
		Pointee::Slice => "a place",
		Pointee::Array => "a view",
	};
	Some(format!("{erase}, whose JS value is {what} of its own each borrow"))
}

/// Why a borrow's pointer to `ty`, which JS copies, can't be erased to a `*const ()`.
pub fn erased_place(ty: Ty<'_>) -> String {
	format!("a pointer to `{ty}` cast to one to `()`, whose JS value is a place of its own each borrow")
}

/// A part `ty` holds by value at its start, at any depth, which a pointer to is an object
/// of its own, or another type JS keeps as what it holds: at `ty`'s address in Rust, but
/// another identity in JS. Any field of an enum's or a union's may be at the start.
fn shares_start<'tcx>(b: &Body<'_, '_, 'tcx>, ty: Ty<'tcx>) -> Option<Ty<'tcx>> {
	let (tcx, env) = (b.t.tcx, b.t.typing_env_pub());
	let normalize = |t: Ty<'tcx>| {
		tcx.try_normalize_erasing_regions(env, ty::Unnormalized::new_wip(t))
			.unwrap_or(t)
	};
	let at_start = |i: usize| {
		tcx.layout_of(env.as_query_input(ty))
			.is_ok_and(|l| l.fields.offset(i) == rustc_abi::Size::ZERO)
	};
	let parts: Vec<Ty<'tcx>> = match *ty.kind() {
		ty::Array(elem, _) | ty::Pat(elem, _) => vec![elem],
		ty::Tuple(tys) => (tys.iter().enumerate())
			.filter(|&(i, _)| at_start(i))
			.map(|(_, t)| t)
			.collect(),
		ty::Adt(adt, args) if adt.is_struct() => (adt.non_enum_variant().fields.iter().enumerate())
			.filter(|&(i, _)| at_start(i))
			.map(|(_, f)| normalize(f.ty(tcx, args).skip_norm_wip()))
			.collect(),
		ty::Adt(adt, args) => adt
			.all_fields()
			.map(|f| normalize(f.ty(tcx, args).skip_norm_wip()))
			.collect(),
		_ => Vec::new(),
	};
	parts
		.into_iter()
		.filter(|&part| !b.t.zero_sized(part))
		.find_map(
			|part| match b.t.erased(part) != part || b.t.pointee(part) == Pointee::Object {
				true => Some(part),
				false => shares_start(b, part),
			},
		)
}

/// Why pointers to `ty` can't point into an array, if they can't: an element JS keeps as
/// an object or as bytes is what a pointer to it is, which knows no array it is in. Only an
/// instance decides what a parameter is.
fn unmodeled<'tcx>(b: &Body<'_, '_, 'tcx>, ty: Ty<'tcx>, what: &str) -> Option<String> {
	if b.t.polymorphic() && ty.has_param() {
		return None;
	}
	match b.t.pointee(ty) {
		Pointee::Place => None,
		Pointee::Object | Pointee::Array | Pointee::Slice => Some(format!(
			"{what} `{ty}`, which JS keeps as an object that knows no array it is in"
		)),
	}
}

/// The pointer `p` to `from`, as one to `to`: the same pointer, as one to what a
/// `MaybeUninit` holds or the other way, erased to or back from a `*const ()`, the pointer
/// to the first element of the array or slice it points at, or one to a slice as one to an
/// array of its elements.
pub fn cast<'tcx>(b: &mut Body<'_, '_, 'tcx>, p: Expr, from: Ty<'tcx>, to: Ty<'tcx>, span: Span) -> Expr {
	let (inner, into) = (b.t.uninit_inner(from), b.t.uninit_inner(to));
	// Bytes are bytes, initialized or not, so a slice of either is the same `rs.ByteSlice`.
	let byte_slice = |t: Ty<'tcx>| matches!(t.kind(), ty::Slice(e) if super::byte(*e));
	if inner == into || byte_slice(inner) && byte_slice(into) {
		return p;
	}
	// The type a pointer to it remembers, which a cast back must name, or fails at run time,
	// and where a dangling one is.
	let (tcx, env) = (b.t.tcx, b.t.typing_env_pub());
	let key = |t: Ty<'tcx>| Expr::Str(tcx.erase_and_anonymize_regions(t).to_string());
	let align = |t: Ty<'tcx>| {
		let align = tcx.layout_of(env.as_query_input(t)).map_or(1, |l| l.align.abi.bytes());
		Expr::lit(align.to_string())
	};
	if b.t.identity_pointee(into) {
		if let Some(why) = unerasable(b, inner) {
			return b.err(span, why);
		}
		let erase = match b.t.pointee(inner) {
			Pointee::Place => "arcErase",
			_ => "ptrErase",
		};
		return Expr::rs(erase, vec![p, key(inner), align(inner)]);
	}
	if b.t.identity_pointee(inner) {
		let restored = Expr::rs("ptrUnerase", vec![p, key(into), align(into)]);
		return Expr::As(Box::new(restored), b.t.pointer_ty(into, span));
	}
	// A pointer to a slice as one to an array of its elements, which reinterprets nothing:
	// `N` elements from its first, read through only where Rust reads.
	if let (ty::Slice(from_elem), ty::Array(elem, n)) = (inner.kind(), into.kind())
		&& (from_elem == elem || super::byte(*from_elem) && super::byte(*elem))
	{
		let Some(n) = n.try_to_target_usize(tcx) else {
			return b.err(
				span,
				format!("a cast from a pointer to `{from}` to one to `{to}`, of no fixed length"),
			);
		};
		let array = match byte_slice(inner) {
			true => "byteArray",
			false => "array",
		};
		return p.member(array).call(vec![Expr::lit(n.to_string())]);
	}
	let elem = match inner.kind() {
		ty::Array(elem, _) | ty::Slice(elem) if b.t.uninit_inner(*elem) == into => Some(*elem),
		_ => None,
	};
	let Some(elem) = elem else {
		return b.err(
			span,
			format!("a cast from a pointer to `{from}` to one to `{to}`, which reads the bytes of one type as another"),
		);
	};
	if let Some(why) = unmodeled(b, elem, "a pointer into an array of") {
		return b.err(span, why);
	}
	// A pointer to a slice holds the pointer to its first element, and one to an array is
	// the slice of its elements.
	match b.t.pointee(inner) {
		Pointee::Slice => p.member("ptr"),
		_ => b.t.array_rs(inner, "AsPtr").call(vec![p]),
	}
}

/// What a `PartialEq` or `Clone` call's `Self` points at, if it is a raw pointer or a
/// `NonNull`.
fn pointer_self<'tcx>(b: &Body<'_, '_, 'tcx>, call: &Call<'tcx>) -> Option<Ty<'tcx>> {
	raw_pointee(b.t.tcx, call.args.types().next()?)
}

/// What `ty` points at if it is a raw pointer or a `NonNull`, which compare by address.
pub fn raw_pointee<'tcx>(tcx: ty::TyCtxt<'tcx>, ty: Ty<'tcx>) -> Option<Ty<'tcx>> {
	match ty.kind() {
		ty::RawPtr(pointee, _) => Some(*pointee),
		ty::Adt(adt, args) if Std::of(tcx, adt.did()) == Some(Std::NonNull) => Some(args.type_at(0)),
		_ => None,
	}
}

/// The pointer an `rs.Cell` holding a `ty` gives to it, as `UnsafeCell::get` and
/// `Cell::as_ptr` do: the cell, the place of a value JS copies, or else the object, array,
/// or slice it holds.
pub(super) fn cell_pointer<'tcx>(b: &Body<'_, '_, 'tcx>, cell: Expr, ty: Ty<'tcx>) -> Expr {
	let pointee = b.t.pointee(ty);
	if pointee == Pointee::Place {
		return cell;
	}
	let inner = b.t.uninit_inner(ty);
	if pointee == Pointee::Array && !super::byte_array(inner) {
		// The slice `Cell::from_mut` made the cell of, or else of all of the array it holds.
		return Expr::rs("cellArray", vec![cell, super::array_fn(inner, "From")]);
	}
	let held = cell.member("v");
	match (pointee, inner.kind()) {
		(Pointee::Array, ty::Array(elem, _)) => super::slice_items_from(held, *elem),
		(Pointee::Slice, _) if slice_of_bytes(ty) => Expr::rs("rawByteSliceOf", vec![held]),
		(Pointee::Slice, _) if slice_of_arrays(ty) => Expr::rs("rawArraySliceOf", vec![held]),
		(Pointee::Slice, _) => Expr::rs("rawSliceOf", vec![held]),
		_ => held,
	}
}

/// The refusal of a move of a `ty` out of where it is kept, as `ptr::read` makes, that
/// would copy what JS can't split from the original as Rust's does.
fn unsplit<'tcx>(b: &mut Body<'_, '_, 'tcx>, ty: Ty<'tcx>, span: Span) -> Option<Expr> {
	let (held, why) = b.t.unsplit(ty)?;
	let what = match held == ty {
		true => String::new(),
		false => format!(", which holds a `{held}`"),
	};
	Some(b.err(
		span,
		format!("a read moving out a `{ty}`{what}, {why}, which no copy splits from the original as Rust's read does"),
	))
}

/// The value `p`, a pointer to `ty`, points at, as a value of its own.
pub(super) fn read<'tcx>(b: &mut Body<'_, '_, 'tcx>, p: Expr, ty: Ty<'tcx>, span: Span, out: &mut Vec<Stmt>) -> Expr {
	if b.t.keeps_cells(ty) {
		let at = pointed(b, p, ty);
		return copy_kept(b, at, ty, span, out);
	}
	let value = match b.t.pointee(ty) {
		// The value a pointer made here of a shared reference holds.
		Pointee::Place => match p.unlocated() {
			Expr::Object(members)
				if let [(Key::Name(v), value)] = members.as_slice()
					&& v == "v" =>
			{
				value.clone()
			}
			_ => p.member("v"),
		},
		Pointee::Object => p,
		Pointee::Array => {
			let ty = b.t.uninit_inner(ty);
			return b.t.array_copy(p, ty, span);
		}
		Pointee::Slice => return b.err(span, format!("a read of `{ty}`, which has no size")),
	};
	b.t.snapshot(value, ty, span)
}

/// `ptr::replace` of `p`, a pointer to `ty`, with `value`: what it held, as a value of its
/// own.
pub(super) fn replace<'tcx>(
	b: &mut Body<'_, '_, 'tcx>,
	p: Expr,
	value: Expr,
	ty: Ty<'tcx>,
	span: Span,
	out: &mut Vec<Stmt>,
) -> Expr {
	let p = b.spill(p, "pointer", out);
	let old = copied_out(b, p.clone(), ty, span, out);
	let old = b.spill(old, "old", out);
	let write = write(b, p, value, ty, span, out);
	out.push(StmtKind::Expr(write).at(None));
	old
}

/// What `p`, a pointer to `ty`, points at, as a value of its own that a whole write there
/// after leaves as it was. The write lands in the object or array a pointer is, so one is
/// copied one object deep, an array's elements to the depth `objectCopy` copies them, and an
/// array of arrays' as [`super::Translator::elem_copy`] says. One
/// a pointer is the place of is read as it is, keeping a `Box`'s allocation, as its write
/// puts another in the place.
fn copied_out<'tcx>(b: &mut Body<'_, '_, 'tcx>, p: Expr, ty: Ty<'tcx>, span: Span, out: &mut Vec<Stmt>) -> Expr {
	match b.t.pointee(ty) {
		_ if b.t.zero_sized(ty) || b.t.keeps_cells(ty) => read(b, p, ty, span, out),
		Pointee::Object => Expr::rs("objectCopy", vec![p]),
		Pointee::Array if super::array_slice::of_arrays(b.t.uninit_inner(ty)) => {
			let copy = b.t.elem_copy(b.t.uninit_inner(ty), span);
			Expr::rs("arraySliceCopy", std::iter::once(p).chain(copy).collect())
		}
		Pointee::Array if !super::byte_array(b.t.uninit_inner(ty)) => {
			let depth = b.t.copy_depth(b.t.uninit_inner(ty));
			Expr::rs("sliceCopy", std::iter::once(p).chain(depth).collect())
		}
		_ => read(b, p, ty, span, out),
	}
}

/// `ptr::swap` of `x` and `y`, pointers to `ty`.
pub(super) fn swap<'tcx>(
	b: &mut Body<'_, '_, 'tcx>,
	x: Expr,
	y: Expr,
	ty: Ty<'tcx>,
	span: Span,
	out: &mut Vec<Stmt>,
) -> Expr {
	let (x, y) = (b.spill(x, "x", out), b.spill(y, "y", out));
	let old = copied_out(b, x.clone(), ty, span, out);
	let old = b.spill(old, "old", out);
	let from_y = copied_out(b, y.clone(), ty, span, out);
	let into_x = write(b, x, from_y, ty, span, out);
	out.push(StmtKind::Expr(into_x).at(None));
	write(b, y, old, ty, span, out)
}

/// The write of `value`, a `ty`, where `p` points.
pub(super) fn write<'tcx>(
	b: &mut Body<'_, '_, 'tcx>,
	p: Expr,
	value: Expr,
	ty: Ty<'tcx>,
	span: Span,
	out: &mut Vec<Stmt>,
) -> Expr {
	if b.t.zero_sized(ty) {
		return Expr::undefined();
	}
	if b.t.wraps_cells(ty) {
		return wrapped_write(b, ty, span);
	}
	if b.t.keeps_cells(ty) {
		let at = match b.t.pointee(ty) {
			Pointee::Object => p,
			_ => b.spill(p, "pointer", out).member("v"),
		};
		write_kept_at(b, at, value, ty, span, out);
		return Expr::undefined();
	}
	match b.t.pointee(ty) {
		Pointee::Place => p.member("v").assign(value),
		Pointee::Object => Expr::rs("ptrWrite", vec![p, value]),
		Pointee::Array => b.t.array_rs(ty, "Set").call(vec![p, value]),
		Pointee::Slice => b.err(span, format!("a write of `{ty}`, which has no size")),
	}
}

/// The refusal of a whole write of a `ty` that [wraps cells](Translator::wraps_cells).
pub(super) fn wrapped_write<'tcx>(b: &mut Body<'_, '_, 'tcx>, ty: Ty<'tcx>, span: Span) -> Expr {
	b.err(
		span,
		format!(
			"a whole write of `{ty}`, which would put another cell in place of one a `MaybeUninit` or `ManuallyDrop` holds"
		),
	)
}

/// The JS value of the `ty` that `p` points at: a cell's `rs.Cell`, which a pointer to
/// it is the place of, or the object a pointer to it is.
fn pointed<'tcx>(b: &Body<'_, '_, 'tcx>, p: Expr, ty: Ty<'tcx>) -> Expr {
	match b.t.pointee(ty) {
		Pointee::Place => p.member("v"),
		_ => p,
	}
}

/// The write of `value`, a `ty` that keeps cells, where `at` names its JS value: into the
/// cells there in place, or whole where memory not written yet, as `ptr::write` may fill,
/// holds no cell to keep. `at` is read more than once.
pub(super) fn write_kept_at<'tcx>(
	b: &mut Body<'_, '_, 'tcx>,
	at: Expr,
	value: Expr,
	ty: Ty<'tcx>,
	span: Span,
	out: &mut Vec<Stmt>,
) {
	if b.t.pointee(ty) == Pointee::Object {
		return write_kept(b, at, value, ty, span, out);
	}
	let value = b.spill(value, "value", out);
	let mut kept = Vec::new();
	write_kept(b, at.clone(), value.clone(), ty, span, &mut kept);
	let whole = StmtKind::Expr(at.clone().assign(value)).at(None);
	let cond = at.binary("===", Expr::undefined());
	out.push(
		StmtKind::If {
			cond,
			then: vec![whole],
			els: kept,
		}
		.at(None),
	);
}

/// The write of `value` into `at`, the JS value of a `ty` that keeps cells, in place: a
/// cell's value written into the cell, as a write through its `get` would be, and a
/// tuple's or struct's other fields given `value`'s, as is a cell field of one not
/// written yet.
fn write_kept<'tcx>(b: &mut Body<'_, '_, 'tcx>, at: Expr, value: Expr, ty: Ty<'tcx>, span: Span, out: &mut Vec<Stmt>) {
	match b.t.kept_cells(ty) {
		Some(KeptCells::Cell(held)) if b.t.keeps_cells(held) => {
			write_kept(b, at.member("v"), value.member("v"), held, span, out)
		}
		Some(KeptCells::Cell(held)) => {
			let p = cell_pointer(b, at, held);
			let write = write(b, p, value.member("v"), held, span, out);
			out.push(StmtKind::Expr(write).at(None));
		}
		Some(KeptCells::Fields(fields)) => {
			let (at, value) = (b.spill(at, "object", out), b.spill(value, "value", out));
			let kept: Vec<_> = (fields.iter().enumerate())
				.filter(|(_, (_, kept))| *kept)
				.map(|(i, &(field, _))| {
					let part = object_field(b, at.clone(), ty, i, span);
					(i, field, b.spill(part, "cell", out))
				})
				.collect();
			out.push(StmtKind::Expr(Expr::rs("ptrWrite", vec![at.clone(), value.clone()])).at(None));
			for (i, field, cell) in kept {
				let part = object_field(b, at.clone(), ty, i, span);
				let mut then = vec![StmtKind::Expr(part.assign(cell.clone())).at(None)];
				let from = object_field(b, value.clone(), ty, i, span);
				write_kept(b, cell.clone(), from, field, span, &mut then);
				let cond = cell.binary("!==", Expr::undefined());
				out.push(
					StmtKind::If {
						cond,
						then,
						els: Vec::new(),
					}
					.at(None),
				);
			}
		}
		None => unreachable!("`{ty}` keeps no cells"),
	}
}

/// A copy of `at`, the JS value of a `ty` that keeps cells, with a cell of its own for
/// each it keeps, so a write into those in place leaves the copy be, and each other part
/// copied as a read copies it.
fn copy_kept<'tcx>(b: &mut Body<'_, '_, 'tcx>, at: Expr, ty: Ty<'tcx>, span: Span, out: &mut Vec<Stmt>) -> Expr {
	match b.t.kept_cells(ty) {
		Some(KeptCells::Cell(held)) => {
			let value = match (b.t.keeps_cells(held), b.t.pointee(held)) {
				(true, _) => copy_kept(b, at.member("v"), held, span, out),
				(false, Pointee::Object) if let Some(copy) = b.t.snapshot_fn(held, span) => {
					copy.call(vec![at.member("v")])
				}
				(false, _) => {
					let p = cell_pointer(b, at, held);
					copied_out(b, p, held, span, out)
				}
			};
			Expr::New(Box::new(Expr::Rs("Cell".into())), vec![value])
		}
		Some(KeptCells::Fields(fields)) => {
			let at = b.spill(at, "object", out);
			let copy = b.spill(Expr::rs("objectCopy", vec![at.clone()]), "copy", out);
			for (i, &(field, kept)) in fields.iter().enumerate() {
				let from = object_field(b, at.clone(), ty, i, span);
				let value = match kept {
					true => copy_kept(b, from, field, span, out),
					false => match b.t.snapshot_fn(field, span) {
						Some(copy) => copy.call(vec![from]),
						None => continue,
					},
				};
				let part = object_field(b, copy.clone(), ty, i, span);
				out.push(StmtKind::Expr(part.assign(value)).at(None));
			}
			copy
		}
		None => unreachable!("`{ty}` keeps no cells"),
	}
}

/// Field `i` of `object`, the JS object of a tuple or a crate's struct `ty`.
fn object_field<'tcx>(b: &mut Body<'_, '_, 'tcx>, object: Expr, ty: Ty<'tcx>, i: usize, span: Span) -> Expr {
	match ty.kind() {
		ty::Tuple(_) => object.index(Expr::lit(i.to_string())),
		ty::Adt(adt, _) => match b.t.adt(*adt, span).repr {
			AdtRepr::Struct { fields, .. } => Translator::read_field(object, fields[i]),
			_ => unreachable!("`{ty}` is a struct"),
		},
		_ => unreachable!("`{ty}` is a tuple or a struct"),
	}
}

/// Why a reborrow through a raw pointer of `ty`, an array a `MaybeUninit` or `ManuallyDrop`
/// holds of other than bytes, is refused: its reference is the array, which part of an array
/// has none of its own, so a shared one would be a copy at another address.
pub fn uninit_reborrow(ty: Ty<'_>) -> String {
	format!(
		"a reborrow of `{ty}` through a raw pointer, which may point into part of an array with no JS array of its own to lend, which is not supported yet"
	)
}

/// Whether `ty` is `[u8]`, one of `MaybeUninit<u8>`, or an `UninitSlice`, whose slices are
/// `rs.ByteSlice`s.
pub fn slice_of_bytes(ty: Ty<'_>) -> bool {
	match ty.kind() {
		ty::Slice(elem) => super::byte(*elem),
		// The context every lowering runs in, as for [`super::byte`].
		ty::Adt(adt, _) => ty::tls::with(|tcx| super::stock::is_uninit_slice(tcx, adt.did())),
		_ => false,
	}
}

/// Whether `ty`, a raw pointer's pointee, is a slice of arrays, which an
/// `rs.RawArraySlice` points at.
pub fn slice_of_arrays(ty: Ty<'_>) -> bool {
	matches!(ty.kind(), ty::Slice(elem) if elem.is_array())
}

/// Whether pointers `x` and `y` to `ty` point at the same place: the same object or
/// bytes, or null both.
pub fn compare<'tcx>(b: &mut Body<'_, '_, 'tcx>, x: Expr, y: Expr, ty: Ty<'tcx>, span: Span) -> Expr {
	let null = |e: &Expr| matches!(e, Expr::Rs(n) if n == "NULL");
	match b.t.incomparable(ty) {
		Some(why) if !null(&x) && !null(&y) => b.err(span, why),
		_ if !null(&x)
			&& !null(&y)
			&& let Some(same) = b.t.same_fn(ty) =>
		{
			same.call(vec![x, y])
		}
		// Each is a view of the struct it was made from.
		_ if !ty.is_trait() && super::dynamic::unsized_object(b.t.tcx, ty) => Expr::rs("tailSame", vec![x, y]),
		_ => x.binary("===", y),
	}
}
