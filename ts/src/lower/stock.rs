//! Well-known crates that translation maps itself, as it maps std, so a consumer binds
//! none of them: `bytes`' `Bytes`, which is the `rs.ByteSlice` a `&[u8]` is, and
//! `BytesMut`, which is `rs.BytesMut`, `hashbrown`'s tables, which std's `HashMap` and
//! `HashSet` are, and `rand`'s thread-local generator, which draws from the platform's
//! cryptographic source, and tokio's sync channels, which [`super::tokio`] maps. The rest of `bytes`, such as most of `Buf`'s
//! provided methods of the crate's own buffers, translates from its MIR. `tracing`'s
//! events are left out where they are written, by [`super::Translator::ignored_site`].

use rustc_hir::def::DefKind;
use rustc_hir::def_id::{CrateNum, DefId};
use rustc_middle::ty::{self, Instance, Ty, TyCtxt, TypeVisitableExt};

use super::body::Body;
use super::library::{self, Call, Std, key, owned_string, string, targ};
use super::num::Int;
use crate::ts::{Expr, Stmt, StmtKind};

/// The type `did` is, if it is one of these crates' that has a representation of its own.
pub fn of(tcx: TyCtxt<'_>, did: DefId) -> Option<Std> {
	if !is_stock(tcx, did) {
		return None;
	}
	if super::tokio::Handle::of(tcx, did).is_some() {
		return Some(Std::Sync);
	}
	match (tcx.crate_name(did.krate).as_str(), tcx.opt_item_name(did)?.as_str()) {
		("bytes", "Bytes") => Some(Std::Bytes),
		("bytes", "BytesMut") => Some(Std::BytesMut),
		("rand", "ThreadRng") => Some(Std::ThreadRng),
		("hashbrown", "HashMap") => Some(Std::HashMap),
		("hashbrown", "HashSet") => Some(Std::HashSet),
		("hashbrown", "HashTable") => Some(Std::HashTable),
		// foldhash's, which hashbrown's are, seed each state at random, as std's do, so
		// which hash a key gets is never observable.
		("hashbrown", "DefaultHashBuilder") => Some(Std::RandomState),
		("hashbrown", "DefaultHasher") => Some(Std::Hasher),
		("hashbrown", "Entry" | "OccupiedEntry" | "VacantEntry" | "AbsentEntry") => {
			match tcx.opt_item_name(tcx.parent(did))?.as_str() {
				"map" => (tcx.item_name(did).as_str() != "AbsentEntry").then_some(Std::Entry),
				"table" => Some(Std::TableEntry),
				_ => None,
			}
		}
		_ => None,
	}
}

/// The `[MaybeUninit<u8>]` that `ty` wraps if it is `bytes`' `UninitSlice`, which is
/// `#[repr(transparent)]` over it, so JS keeps it as that slice's `rs.ByteSlice`, which
/// views the storage of the buffer that lends it.
pub fn uninit_slice<'tcx>(tcx: TyCtxt<'tcx>, ty: Ty<'tcx>) -> Option<Ty<'tcx>> {
	match ty.kind() {
		ty::Adt(adt, _) if is_uninit_slice(tcx, adt.did()) => {
			Some(Ty::new_slice(tcx, Ty::new_maybe_uninit(tcx, tcx.types.u8)))
		}
		_ => None,
	}
}

/// Whether `did` is `bytes`' `UninitSlice`.
pub fn is_uninit_slice(tcx: TyCtxt<'_>, did: DefId) -> bool {
	is_stock(tcx, did)
		&& tcx.crate_name(did.krate).as_str() == "bytes"
		&& tcx
			.opt_item_name(did)
			.is_some_and(|name| name.as_str() == "UninitSlice")
}

/// Whether `did` is an item of `hashbrown`, whose tables are std's, so its functions map
/// as std's do.
pub fn as_std(tcx: TyCtxt<'_>, did: DefId) -> bool {
	is_stock(tcx, did) && tcx.crate_name(did.krate).as_str() == "hashbrown"
}

/// Whether `did` is an item of one of these crates, rather than of the crate translated,
/// whatever its name.
pub fn is_stock(tcx: TyCtxt<'_>, did: DefId) -> bool {
	!did.is_local() && is_stock_crate(tcx, did.krate)
}

/// Whether `krate`, a dependency, is one of these crates, whose functions map, or translate
/// only where [`maps`] says they don't.
pub fn is_stock_crate(tcx: TyCtxt<'_>, krate: CrateNum) -> bool {
	matches!(tcx.crate_name(krate).as_str(), "bytes" | "hashbrown" | "rand" | "tokio")
}

/// Whether `krate` is one of these crates of which nothing translates: `rand`, whose
/// generators would have to be its own, and `hashbrown`, whose tables are the runtime's.
pub fn is_opaque_crate(tcx: TyCtxt<'_>, krate: CrateNum) -> bool {
	matches!(tcx.crate_name(krate).as_str(), "hashbrown" | "rand")
}

/// Whether `did`, an item of `bytes`, has a mapping in place of its MIR: `Bytes`,
/// `BytesMut`, and their functions and impls, and the crate's impls of std's shared byte slices
/// and vectors. The mapping of a function stands in for it or refuses it. The impls that
/// forward through a reference or a `Box`, and the crate's other types and functions,
/// translate, as do its impls for a `VecDeque<u8>` and a `&mut [u8]`.
pub fn mapped(tcx: TyCtxt<'_>, did: DefId) -> bool {
	if tcx.crate_name(did.krate).as_str() == "tokio" {
		return super::tokio::mapped(tcx, did);
	}
	if tcx.crate_name(did.krate).as_str() != "bytes" {
		return false;
	}
	match tcx
		.opt_parent(did)
		.filter(|&imp| matches!(tcx.def_kind(imp), DefKind::Impl { .. }))
	{
		Some(imp) => mapped_self(tcx, tcx.type_of(imp).instantiate_identity().skip_norm_wip()),
		None => of(tcx, did).is_some(),
	}
}

/// Whether `instance`, a provided method of `Buf` or `BufMut`, maps for its `Self`, which
/// [`mapped`] can't tell from the method alone: one the mapping names, of a buffer whose
/// impl maps or that a `&mut` or a `Box` writes through, and `copy_to_bytes` of any
/// other, whose `BytesMut::put` reads a capacity. The others lower from their MIR, as for
/// the crate's own buffers.
pub fn maps<'tcx>(tcx: TyCtxt<'tcx>, instance: Instance<'tcx>) -> bool {
	let did = instance.def_id();
	if tcx.crate_name(did.krate).as_str() != "bytes" || tcx.trait_of_assoc(did).is_none() {
		return false;
	}
	let name = tcx.item_name(did);
	let ty = instance.args.type_at(0);
	let get = Get::of(name.as_str());
	match mapped_self(tcx, ty) {
		true => BUF.contains(&name.as_str()) || BUF_MUT.contains(&name.as_str()) || get.is_some_and(|g| g.fallible),
		false if Buffer::through(tcx, ty).is_some() => BUF_MUT.contains(&name.as_str()),
		false => name.as_str() == "copy_to_bytes" || get.is_some(),
	}
}

/// `Buf`'s methods that the mapping names for its buffers.
const BUF: &[&str] = &[
	"remaining",
	"chunk",
	"advance",
	"has_remaining",
	"copy_to_slice",
	"copy_to_bytes",
	"get_u8",
	"get_i8",
	"get_u16",
	"get_u16_le",
	"get_u16_ne",
	"get_i16",
	"get_i16_le",
	"get_i16_ne",
	"get_u32",
	"get_u32_le",
	"get_u32_ne",
	"get_i32",
	"get_i32_le",
	"get_i32_ne",
	"get_u64",
	"get_u64_le",
	"get_u64_ne",
	"get_i64",
	"get_i64_le",
	"get_i64_ne",
	"get_u128",
	"get_u128_le",
	"get_u128_ne",
	"get_i128",
	"get_i128_le",
	"get_i128_ne",
	"get_f32",
	"get_f32_le",
	"get_f32_ne",
	"get_f64",
	"get_f64_le",
	"get_f64_ne",
	"get_uint",
	"get_uint_le",
	"get_uint_ne",
	"get_int",
	"get_int_le",
	"get_int_ne",
];

/// `BufMut`'s methods that the mapping names for its buffers.
const BUF_MUT: &[&str] = &[
	"remaining_mut",
	"has_remaining_mut",
	"put",
	"put_slice",
	"put_bytes",
	"put_u8",
	"put_i8",
	"put_u16",
	"put_u16_le",
	"put_u16_ne",
	"put_i16",
	"put_i16_le",
	"put_i16_ne",
	"put_u32",
	"put_u32_le",
	"put_u32_ne",
	"put_i32",
	"put_i32_le",
	"put_i32_ne",
	"put_u64",
	"put_u64_le",
	"put_u64_ne",
	"put_i64",
	"put_i64_le",
	"put_i64_ne",
	"put_u128",
	"put_u128_le",
	"put_u128_ne",
	"put_i128",
	"put_i128_le",
	"put_i128_ne",
	"put_f32",
	"put_f32_le",
	"put_f32_ne",
	"put_f64",
	"put_f64_le",
	"put_f64_ne",
	"put_uint",
	"put_uint_le",
	"put_uint_ne",
	"put_int",
	"put_int_le",
	"put_int_ne",
];

/// A buffer that the mapping of `Buf` or `BufMut` reads or writes.
#[derive(Clone, Copy, PartialEq)]
enum Buffer {
	/// A `Bytes`, whose `&mut` is an `rs.Place`.
	Bytes,
	BytesMut,
	/// A `&[u8]`, whose `&mut` is an `rs.Place` too.
	Slice,
	/// A `Vec<u8>`, which `BufMut` writes.
	Vec,
}

impl Buffer {
	fn of<'tcx>(tcx: TyCtxt<'tcx>, ty: Ty<'tcx>) -> Option<Self> {
		match ty.kind() {
			ty::Adt(adt, args) => match library::Std::of(tcx, adt.did())? {
				Std::Bytes => Some(Self::Bytes),
				Std::BytesMut => Some(Self::BytesMut),
				Std::Vec if is_u8(args.type_at(0)) => Some(Self::Vec),
				_ => None,
			},
			ty::Ref(_, inner, ty::Mutability::Not) if matches!(inner.kind(), ty::Slice(e) if is_u8(*e)) => {
				Some(Self::Slice)
			}
			_ => None,
		}
	}

	/// The buffer that `ty`, `&mut`s and `Box`es of a `Vec<u8>` or a `BytesMut`, writes
	/// through, whose impls forward all but some of `BufMut`'s provided methods.
	fn through<'tcx>(tcx: TyCtxt<'tcx>, ty: Ty<'tcx>) -> Option<Self> {
		let inner = match ty.kind() {
			ty::Ref(_, inner, ty::Mutability::Mut) => *inner,
			ty::Adt(adt, args) if library::Std::of(tcx, adt.did()) == Some(Std::Box) => args.type_at(0),
			_ => return None,
		};
		match Self::of(tcx, inner) {
			Some(buffer @ (Self::Vec | Self::BytesMut)) if !inner.has_param() => Some(buffer),
			Some(_) => None,
			None => Self::through(tcx, inner),
		}
	}
}

/// Whether `ty`, the `Self` of a function of `bytes`, has a representation of its own or
/// is std's, so a mapping stands in for its impls: not a type parameter, through which an
/// impl forwards, nor the crate's own buffer or another crate's, nor a `VecDeque<u8>`,
/// whose impl reads the slices the runtime's deque has, nor a `&mut [u8]`, whose impl
/// writes its front and moves past it, which translate.
fn mapped_self<'tcx>(tcx: TyCtxt<'tcx>, ty: Ty<'tcx>) -> bool {
	match ty.kind() {
		ty::Adt(adt, _) => library::Std::of(tcx, adt.did()).is_some_and(|std| std != Std::VecDeque) && !ty.has_param(),
		ty::Ref(_, inner, ty::Mutability::Not) => matches!(inner.kind(), ty::Slice(e) if is_u8(*e)),
		_ => false,
	}
}

/// A call of one of these crates' functions, or of a std trait's on one of their types:
/// its mapping, `None` if it has none, or its arguments back if it is no such call.
pub fn call<'tcx>(
	b: &mut Body<'_, '_, 'tcx>,
	call: &Call<'tcx>,
	args: Vec<Expr>,
	out: &mut Vec<Stmt>,
) -> Result<Option<Expr>, Vec<Expr>> {
	let tcx = b.t.tcx;
	let Some((owner, name)) = key(tcx, call.def_id) else {
		return Err(args);
	};
	let self_std = call.args.types().next().and_then(|t| b.t.std_of(t.peel_refs()));
	if as_std(tcx, call.def_id) {
		return Err(args);
	}
	if is_stock(tcx, call.def_id) {
		return Ok(match tcx.crate_name(call.def_id.krate).as_str() {
			"bytes" => bytes(b, call, &owner, &name, args, out),
			"tokio" => super::tokio::call(b, call, args, out),
			_ => rand(b, call, &owner, &name, args, out),
		});
	}
	if self_std == Some(Std::Sync) {
		return super::tokio::trait_call(b, call, &owner, &name, args, out);
	}
	let mut args = args.into_iter();
	let mut arg = || args.next().expect("an argument");
	Ok(match (owner.as_str(), name.as_str(), self_std) {
		// A clone of a `Bytes` counts its storage once more, and a `ThreadRng` holds nothing.
		("Clone", "clone", Some(Std::Bytes)) => Some(Expr::rs("bytesClone", vec![arg()])),
		// A `Bytes` is the slice it derefs to.
		("Clone", "clone", Some(Std::ThreadRng))
		| ("Deref", "deref", Some(Std::Bytes))
		| ("AsRef", "as_ref", Some(Std::Bytes))
		| ("Borrow", "borrow", Some(Std::Bytes)) => Some(arg()),
		// A `BytesMut` lends a slice of its storage, and its clone copies it.
		("Deref", "deref", Some(Std::BytesMut))
		| ("DerefMut", "deref_mut", Some(Std::BytesMut))
		| ("AsRef", "as_ref", Some(Std::BytesMut))
		| ("AsMut", "as_mut", Some(Std::BytesMut))
		| ("Borrow", "borrow", Some(Std::BytesMut))
		| ("BorrowMut", "borrow_mut", Some(Std::BytesMut)) => Some(view(arg())),
		("Clone", "clone", Some(Std::BytesMut)) => Some(Expr::rs("bytesMutFrom", vec![view(arg())])),
		("Default", "default", Some(Std::Bytes)) => Some(Expr::rs("bytesNew", vec![])),
		("Default", "default", Some(Std::BytesMut)) => Some(Expr::rs("bytesMutWithCapacity", vec![Expr::lit("0")])),
		("Default", "default", Some(Std::ThreadRng)) => Some(Expr::rs("randRng", vec![])),
		("PartialEq", "eq", _) => {
			let (l, r) = (targ(call.args, 0).peel_refs(), targ(call.args, 1).peel_refs());
			let stock = |t: Ty<'tcx>| matches!(b.t.std_of(t), Some(Std::Bytes | Std::BytesMut));
			if !stock(l) && !stock(r) {
				return Err(args.collect());
			}
			let (a, c) = (arg(), arg());
			match (as_slice(b, l, a), as_slice(b, r, c)) {
				(Some(a), Some(c)) => Some(Expr::rs("byteSliceEq", vec![a, c])),
				_ => None,
			}
		}
		("From", "from", _) => return from(b, call, args.collect()),
		_ => return Err(args.collect()),
	})
}

/// The bytes of `value`, a `ty`, as an `rs.ByteSlice`, if it is a run of bytes.
fn as_slice<'tcx>(b: &Body<'_, '_, 'tcx>, ty: Ty<'tcx>, value: Expr) -> Option<Expr> {
	Some(match ty.kind() {
		ty::Slice(e) if is_u8(*e) => value,
		ty::Array(e, _) if is_u8(*e) => Expr::rs("byteSliceFrom", vec![value]),
		_ if string(b.t.tcx, ty) => Expr::rs("stringAsBytes", vec![value]),
		_ if owned_string(b.t.tcx, ty) => Expr::rs("stringAsBytes", vec![value.member("v")]),
		_ => match b.t.std_of(ty)? {
			Std::Bytes => value,
			Std::BytesMut => view(value),
			Std::Vec if targ_is_u8(ty) => Expr::rs("byteVecSlice", vec![value]),
			_ => return None,
		},
	})
}

/// The bytes of a `BytesMut`.
fn view(value: Expr) -> Expr {
	Expr::rs("bytesMutView", vec![value])
}

/// `From::from` into or out of a `Bytes` or a `BytesMut`.
fn from<'tcx>(b: &mut Body<'_, '_, 'tcx>, call: &Call<'tcx>, args: Vec<Expr>) -> Result<Option<Expr>, Vec<Expr>> {
	let tcx = b.t.tcx;
	let (to, from) = (targ(call.args, 0), targ(call.args, 1));
	let byte_vec = |t: Ty<'tcx>| b.t.std_of(t) == Some(Std::Vec) && targ_is_u8(t);
	let mut args = args.into_iter();
	let f = match (b.t.std_of(to), b.t.std_of(from)) {
		(Some(Std::Bytes), Some(Std::BytesMut)) => "bytesMutFreeze",
		(Some(Std::Bytes), _) if byte_vec(from) => "bytesFromVec",
		// A `&'static str`'s bytes are static, and a `String`'s are its vector, with its capacity.
		(Some(Std::Bytes), _) if from.is_ref() && string(tcx, from.peel_refs()) => "stringAsBytes",
		(Some(Std::Bytes), _) if owned_string(tcx, from) => {
			let bytes = Expr::rs("stringIntoBytes", args.collect());
			return Ok(Some(Expr::rs("bytesFromVec", vec![bytes])));
		}
		(Some(Std::Bytes), _) if is_bytes_ref(from) => "bytesStatic",
		(Some(Std::Bytes), _) => return Ok(None),
		(Some(Std::BytesMut), _) => {
			let bytes = match as_slice(b, from.peel_refs(), args.next().expect("an operand")) {
				Some(bytes) if from.is_ref() => Expr::rs("bytesMutFrom", vec![bytes]),
				Some(bytes) if b.t.std_of(from) == Some(Std::Bytes) => Expr::rs("bytesMutFromBytes", vec![bytes]),
				_ => return Ok(None),
			};
			return Ok(Some(bytes));
		}
		(_, Some(Std::Bytes)) if byte_vec(to) => "bytesToByteVec",
		(_, Some(Std::BytesMut)) if byte_vec(to) => "bytesMutToByteVec",
		(_, Some(Std::Bytes | Std::BytesMut)) => return Ok(None),
		_ => return Err(args.collect()),
	};
	Ok(Some(Expr::rs(f, args.collect())))
}

/// A function of `bytes`: `Bytes`'s or `BytesMut`'s own, or a method of `Buf` or `BufMut`
/// on a buffer the mapping reads or writes.
fn bytes<'tcx>(
	b: &mut Body<'_, '_, 'tcx>,
	call: &Call<'tcx>,
	owner: &str,
	name: &str,
	args: Vec<Expr>,
	out: &mut Vec<Stmt>,
) -> Option<Expr> {
	let tcx = b.t.tcx;
	match super::library::inherent_std(tcx, call.def_id) {
		Some(Std::Bytes) => return bytes_own(b, call, name, args, out),
		Some(Std::BytesMut) => return bytes_mut(b, call, name, args, out),
		_ => {}
	}
	let ty = targ(call.args, 0);
	// What its MIR reads through a pointer cast: any buffer's `try_get_*` of a fixed size,
	// and its `get_*` of one the mapping doesn't name.
	if owner == "Buf"
		&& let Some(get) = Get::of(name)
		&& (get.fallible || Buffer::of(tcx, ty).is_none())
	{
		return provided_get(b, call, ty, get, args);
	}
	if owner == "BufMut"
		&& let Some(buffer) = Buffer::through(tcx, ty)
	{
		return buf_mut_through(b, call, ty, buffer, name, args, out);
	}
	let Some(buffer) = Buffer::of(tcx, ty) else {
		return match (owner, name) {
			("Buf", "copy_to_bytes") => provided_copy_to_bytes(b, call, args),
			_ => None,
		};
	};
	match owner {
		"Buf" => buf(b, call, buffer, name, args),
		"BufMut" => buf_mut(b, call, buffer, false, name, args, out),
		_ => None,
	}
}

/// A function of `Bytes`: a `&mut self` one takes its `rs.Place`.
fn bytes_own<'tcx>(
	b: &mut Body<'_, '_, 'tcx>,
	call: &Call<'tcx>,
	name: &str,
	args: Vec<Expr>,
	out: &mut Vec<Stmt>,
) -> Option<Expr> {
	let tcx = b.t.tcx;
	let mut args = args.into_iter();
	Some(match name {
		"new" => Expr::rs("bytesNew", vec![]),
		// A `&'static [u8]` never changes, so the slice is the `Bytes`, which counts no storage.
		"from_static" => Expr::rs("bytesStatic", vec![args.next()?]),
		"copy_from_slice" => Expr::rs("bytesCopyFromSlice", vec![args.next()?]),
		"len" => args.next()?.member("length"),
		"is_empty" => args.next()?.member("length").binary("===", Expr::lit("0")),
		// The other ranges' bounds are read where the range is made, by the body's lowering.
		"slice" if b.t.std_of(targ(call.args, 0)) == Some(Std::RangeInclusive) => {
			Expr::rs("bytesSliceInclusive", vec![args.next()?, args.next()?])
		}
		"slice_ref" => Expr::rs("bytesSliceRef", vec![args.next()?, args.next()?]),
		"is_unique" => Expr::rs("bytesIsUnique", vec![args.next()?]),
		"try_into_mut" => Expr::rs("bytesTryIntoMut", vec![args.next()?]),
		"split_to" => Expr::rs("bytesSplitTo", vec![args.next()?, args.next()?]),
		"split_off" => Expr::rs("bytesSplitOff", vec![args.next()?, args.next()?]),
		"truncate" => Expr::rs("bytesTruncate", vec![args.next()?, args.next()?]),
		"clear" => Expr::rs("bytesTruncate", vec![args.next()?, Expr::lit("0")]),
		// The owner is read once, as `bytes` reads it, and one that drops is kept in the
		// storage, which drops it with the last `Bytes` that shares it.
		"from_owner" => {
			let owner = targ(call.args, 0);
			let drop = match b.t.drops(owner) {
				true => Some(b.t.drop_fn(owner, call.span)?),
				false => None,
			};
			let value = args.next()?;
			let value = match drop {
				Some(_) => b.spill(value, "owner", out),
				None => value,
			};
			let as_ref = trait_fn(tcx, rustc_span::sym::AsRef)?;
			let slice = Ty::new_slice(tcx, tcx.types.u8);
			let bytes = b.call_def(
				as_ref,
				tcx.mk_args(&[owner.into(), slice.into()]),
				vec![value.clone()],
				call.span,
				out,
			);
			match drop {
				Some(drop) => Expr::rs("bytesFromOwner", vec![value, bytes, drop]),
				None => Expr::rs("bytesStatic", vec![bytes]),
			}
		}
		_ => return None,
	})
}

/// A function of `BytesMut`.
fn bytes_mut(
	b: &mut Body<'_, '_, '_>,
	_call: &Call<'_>,
	name: &str,
	args: Vec<Expr>,
	out: &mut Vec<Stmt>,
) -> Option<Expr> {
	let mut args = args.into_iter();
	let mut arg = || args.next();
	let f = match name {
		"new" => return Some(Expr::rs("bytesMutWithCapacity", vec![Expr::lit("0")])),
		"with_capacity" => "bytesMutWithCapacity",
		"zeroed" => "bytesMutZeroed",
		"len" => return Some(arg()?.member("length")),
		"is_empty" => return Some(arg()?.member("length").binary("===", Expr::lit("0"))),
		"capacity" => return Some(arg()?.member("cap")),
		"freeze" => "bytesMutFreeze",
		"split_to" => "bytesMutSplitTo",
		"split_off" => "bytesMutSplitOff",
		"try_reclaim" => "bytesMutTryReclaim",
		"try_unsplit" => "bytesMutTryUnsplit",
		"split" => {
			let this = b.spill(arg()?, "buf", out);
			return Some(Expr::rs("bytesMutSplitTo", vec![this.clone(), this.member("length")]));
		}
		"truncate" => "bytesMutTruncate",
		"clear" => return Some(Expr::rs("bytesMutTruncate", vec![arg()?, Expr::lit("0")])),
		"resize" => "bytesMutResize",
		"reserve" => "bytesMutReserve",
		"extend_from_slice" => "bytesMutExtend",
		"unsplit" => "bytesMutUnsplit",
		"spare_capacity_mut" => "bytesMutSpare",
		"set_len" => "bytesMutSetLen",
		_ => return None,
	};
	Some(Expr::rs(f, args.collect()))
}

/// A method of `Buf` on a buffer the mapping reads.
fn buf<'tcx>(
	b: &mut Body<'_, '_, 'tcx>,
	call: &Call<'tcx>,
	buffer: Buffer,
	name: &str,
	args: Vec<Expr>,
) -> Option<Expr> {
	let mut args = args.into_iter();
	let this = args.next()?;
	// What reads past the front: a `Bytes`'s or a `&[u8]`'s place, which `bytes` reads
	// alike but to advance and copy out, or a `BytesMut`.
	let prefix = match buffer {
		Buffer::Bytes | Buffer::Slice => Some("bytes"),
		Buffer::BytesMut => Some("bytesMut"),
		Buffer::Vec => None,
	};
	let rs = |f: &str, args: Vec<Expr>| Some(Expr::rs(format!("{}{f}", prefix?), args));
	Some(match name {
		"remaining" => this.member("length"),
		"has_remaining" => this.member("length").binary("!==", Expr::lit("0")),
		"chunk" if buffer == Buffer::BytesMut => view(this),
		"chunk" => this,
		"advance" if buffer == Buffer::Slice => Expr::rs("byteSliceAdvance", vec![this, args.next()?]),
		"advance" => rs("Advance", vec![this, args.next()?])?,
		"copy_to_slice" => rs("CopyToSlice", vec![this, args.next()?])?,
		"copy_to_bytes" if buffer == Buffer::BytesMut => Expr::rs(
			"bytesMutFreeze",
			vec![Expr::rs("bytesMutSplitTo", vec![this, args.next()?])],
		),
		"copy_to_bytes" if buffer == Buffer::Slice => Expr::rs("byteSliceCopyToBytes", vec![this, args.next()?]),
		"copy_to_bytes" => rs("SplitTo", vec![this, args.next()?])?,
		_ => {
			let (ty, le) = scalar(name.strip_prefix("get_")?);
			let out = b.t.sig(call.def_id, call.args).output();
			let le = Expr::lit(le.to_string());
			match ty {
				"uint" | "int" => {
					let int = Int::of(out)?;
					let (size, signed) = (Expr::lit("8"), Expr::lit(int.signed.to_string()));
					let read = rs("ReadInt", vec![this, size, args.next()?, signed, le])?;
					int.method("fromBeBytes", vec![read])
				}
				"f32" => Expr::rs("f32FromBeBytes", vec![rs("Read", vec![this, Expr::lit("4"), le])?]),
				"f64" => Expr::rs("f64FromBeBytes", vec![rs("Read", vec![this, Expr::lit("8"), le])?]),
				_ => {
					let int = Int::of(out)?;
					let n = Expr::lit((int.bits / 8).to_string());
					if int.big() {
						int.method("fromBeBytes", vec![rs("Read", vec![this, n, le])?])
					} else {
						let word = rs("ReadWord", vec![this, n, le])?;
						let word = if int.signed { int.wrap(word) } else { word };
						Expr::As(Box::new(word), int.ty())
					}
				}
			}
		}
	})
}

/// A provided method of `BufMut` that `&mut`s and `Box`es of `buffer` don't forward, on
/// `ty`, one of them: the buffer's own method, but for `put` and `put_bytes`.
fn buf_mut_through<'tcx>(
	b: &mut Body<'_, '_, 'tcx>,
	call: &Call<'tcx>,
	mut ty: Ty<'tcx>,
	buffer: Buffer,
	name: &str,
	mut args: Vec<Expr>,
	out: &mut Vec<Stmt>,
) -> Option<Expr> {
	let tcx = b.t.tcx;
	// `self`'s value, down to a reference to the buffer, which is the buffer's object.
	let input = *b.t.sig(call.def_id, call.args).inputs().first()?;
	let mut this = b.t.pointee_value(input, args.first()?.clone());
	loop {
		let inner = match ty.kind() {
			ty::Ref(_, inner, _) => *inner,
			_ => ty.boxed_ty()?,
		};
		if Buffer::of(tcx, inner).is_some() {
			break;
		}
		this = b.t.pointee_value(ty, this);
		ty = inner;
	}
	args[0] = this;
	buf_mut(b, call, buffer, true, name, args, out)
}

/// A method of `BufMut` on a buffer the mapping writes, `through` a `&mut` or a `Box` if
/// it is a provided one they don't forward, whose `put` and `put_bytes` grow the buffer
/// by `chunk_mut`'s 64 bytes at a time rather than by what they write.
fn buf_mut<'tcx>(
	b: &mut Body<'_, '_, 'tcx>,
	call: &Call<'tcx>,
	buffer: Buffer,
	through: bool,
	name: &str,
	args: Vec<Expr>,
	out: &mut Vec<Stmt>,
) -> Option<Expr> {
	let mut args = args.into_iter();
	let this = args.next()?;
	let prefix = match buffer {
		Buffer::BytesMut => "bytesMut",
		Buffer::Vec => "byteVec",
		Buffer::Bytes | Buffer::Slice => return None,
	};
	let rs = |f: &str, args: Vec<Expr>| Expr::rs(format!("{prefix}{f}"), args);
	// A vector's or a `BytesMut`'s length is at most `isize::MAX`.
	let isize_max = Expr::lit("0x7fffffff");
	Some(match name {
		"remaining_mut" => Expr::As(
			Box::new(isize_max.binary("-", this.member("length"))),
			Int::of(b.t.tcx.types.usize)?.ty(),
		),
		"has_remaining_mut" => this.member("length").binary("<", isize_max),
		"advance_mut" => rs("AdvanceMut", vec![this, args.next()?]),
		"chunk_mut" => rs("ChunkMut", vec![this]),
		"put_slice" if buffer == Buffer::BytesMut => Expr::rs("bytesMutExtend", vec![this, args.next()?]),
		"put_slice" => Expr::rs("byteVecExtendFromSlice", vec![this, args.next()?]),
		"put" => put(b, call, buffer, through, this, args.next()?, out)?,
		"put_bytes" if through => rs("PutFillChunked", vec![this, args.next()?, args.next()?]),
		"put_bytes" => rs("PutFill", vec![this, args.next()?, args.next()?]),
		_ => {
			let (ty, le) = scalar(name.strip_prefix("put_")?);
			let input = *b.t.sig(call.def_id, call.args).inputs().get(1)?;
			let value = args.next()?;
			let le = Expr::lit(le.to_string());
			match ty {
				"uint" | "int" => {
					let be = Int::of(input)?.method("toBeBytes", vec![value]);
					rs("PutInt", vec![this, be, args.next()?, le])
				}
				"f32" => rs("PutBytes", vec![this, Expr::rs("f32ToBeBytes", vec![value]), le]),
				"f64" => rs("PutBytes", vec![this, Expr::rs("f64ToBeBytes", vec![value]), le]),
				_ => {
					let int = Int::of(input)?;
					match int.big() {
						true => rs("PutBytes", vec![this, int.method("toBeBytes", vec![value]), le]),
						false => rs("PutWord", vec![this, value, Expr::lit((int.bits / 8).to_string()), le]),
					}
				}
			}
		}
	})
}

/// `BufMut::put` of a `Buf` into a buffer the mapping writes, `through` a `&mut` or a
/// `Box` by the provided body: all of it, read whole from a buffer the mapping reads, and
/// otherwise through its own methods, as `bytes` reads it, then dropped.
fn put<'tcx>(
	b: &mut Body<'_, '_, 'tcx>,
	call: &Call<'tcx>,
	buffer: Buffer,
	through: bool,
	this: Expr,
	src: Expr,
	out: &mut Vec<Stmt>,
) -> Option<Expr> {
	let tcx = b.t.tcx;
	let ty = targ(call.args, 1);
	let source = Buffer::of(tcx, ty);
	// Into a `BytesMut` of no capacity, `bytes` copies it out whole and drops it, then takes
	// the copy's storage if no other handle holds it, so the runtime drops it there.
	if buffer == Buffer::BytesMut && !through {
		let f = match source {
			Some(Buffer::Bytes) => "bytesMutPutStorage",
			Some(Buffer::BytesMut) => "bytesMutPutMut",
			Some(Buffer::Slice) => "bytesMutPutSlice",
			_ => {
				let mut values = vec![this, held(b, ty, src)];
				for name in ["has_remaining", "remaining", "chunk", "advance", "copy_to_bytes"] {
					values.push(buf_method_mut(b, call, ty, name)?);
				}
				if b.t.drops(ty) {
					values.push(dropper(b, ty, call.span)?);
				}
				return Some(Expr::rs("bytesMutPutBuf", values));
			}
		};
		return Some(Expr::rs(f, vec![this, src]));
	}
	// Elsewhere it drops once all of it is written.
	let src = match b.t.drops(ty) {
		true => b.spill(src, "src", out),
		false => src,
	};
	let bytes = match source {
		Some(Buffer::BytesMut) => Some(view(src.clone())),
		Some(Buffer::Bytes | Buffer::Slice) => Some(src.clone()),
		_ => None,
	};
	let written = match bytes {
		Some(bytes) => {
			let f = match buffer {
				Buffer::BytesMut => "bytesMutPutChunked",
				_ if through => "byteVecPutChunked",
				_ => "byteVecExtendFromSlice",
			};
			Expr::rs(f, vec![this, bytes])
		}
		None => {
			let mut values = vec![this, held(b, ty, src.clone())];
			for name in ["has_remaining", "remaining", "chunk", "advance"] {
				values.push(buf_method_mut(b, call, ty, name)?);
			}
			let f = match (buffer, through) {
				(Buffer::BytesMut, _) => "bytesMutPutBufChunked",
				(_, true) => "byteVecPutBufChunked",
				(_, false) => "byteVecPutBuf",
			};
			Expr::rs(f, values)
		}
	};
	match b.t.drop_value(src, ty, call.span) {
		Some(dropped) => {
			out.push(StmtKind::Expr(written).at(None));
			Some(dropped)
		}
		None => Some(written),
	}
}

/// `src`, a `ty` moved in, as `Buf`'s methods take it: a value JS copies, such as a `&mut`
/// to a buffer, in a place of its own.
fn held<'tcx>(b: &Body<'_, '_, 'tcx>, ty: Ty<'tcx>, src: Expr) -> Expr {
	match b.t.needs_place(ty) {
		true => Expr::Object(vec![(crate::ts::Key::Name("v".into()), src)]),
		false => src,
	}
}

/// What drops a `ty` that [`held`] holds.
fn dropper<'tcx>(b: &mut Body<'_, '_, 'tcx>, ty: Ty<'tcx>, span: rustc_span::Span) -> Option<Expr> {
	if !b.t.needs_place(ty) {
		return b.t.drop_fn(ty, span);
	}
	let param = crate::ts::Param {
		name: "p".into(),
		ty: Some(crate::ts::Type::Rs("Place", vec![b.t.ty(ty, span)])),
	};
	let dropped = b.t.drop_value(Expr::ident("p").member("v"), ty, span)?;
	Some(Expr::Lambda(vec![param], Box::new(dropped)))
}

/// `Buf::copy_to_bytes` as `bytes` provides it, of a buffer the mapping doesn't read: it
/// puts a `Take` of the buffer into a new `BytesMut` of the capacity asked for, which
/// copies chunk by chunk through the buffer's own methods, as the runtime does.
fn provided_copy_to_bytes<'tcx>(b: &mut Body<'_, '_, 'tcx>, call: &Call<'tcx>, args: Vec<Expr>) -> Option<Expr> {
	let ty = targ(call.args, 0);
	if b.t.needs_place(ty) {
		let msg = format!("`Buf::copy_to_bytes` of `{ty}`, a value JS copies, which is not supported");
		return Some(b.err(call.span, msg));
	}
	let mut args = args.into_iter();
	let (this, len) = (args.next()?, args.next()?);
	let (remaining, chunk, advance) = (
		buf_method(b, call, ty, "remaining")?,
		buf_method(b, call, ty, "chunk")?,
		buf_method(b, call, ty, "advance")?,
	);
	Some(Expr::rs("bufCopyToBytes", vec![this, len, remaining, chunk, advance]))
}

/// A provided `get_*` or `try_get_*` of `Buf` of an integer of a fixed size.
#[derive(Clone, Copy)]
struct Get {
	/// `try_get_*`, which gives a `TryGetError` where `get_*` panics.
	fallible: bool,
	le: bool,
}

impl Get {
	fn of(name: &str) -> Option<Self> {
		let (fallible, rest) = match name.strip_prefix("try_get_") {
			Some(rest) => (true, rest),
			None => (false, name.strip_prefix("get_")?),
		};
		let (int, le) = scalar(rest);
		["u16", "i16", "u32", "i32", "u64", "i64", "u128", "i128"]
			.contains(&int)
			.then_some(Self { fallible, le })
	}
}

/// `get` of the buffer `ty` as `bytes` provides it: `rs.bufGet` or `rs.bufTryGet` of its own
/// methods, which read its chunk where it holds the integer, or else copy it out.
fn provided_get<'tcx>(
	b: &mut Body<'_, '_, 'tcx>,
	call: &Call<'tcx>,
	ty: Ty<'tcx>,
	get: Get,
	args: Vec<Expr>,
) -> Option<Expr> {
	let this = args.into_iter().next()?;
	let int = Int::of(match get.fallible {
		true => {
			let ty::Adt(_, result) = b.t.sig(call.def_id, call.args).output().kind() else {
				return None;
			};
			result.type_at(0)
		}
		false => b.t.sig(call.def_id, call.args).output(),
	})?;
	let mut values = vec![
		this,
		Expr::lit((int.bits / 8).to_string()),
		Expr::lit(get.le.to_string()),
		int.func("fromBeBytes"),
	];
	for name in ["remaining", "chunk", "advance", "copy_to_slice"] {
		values.push(buf_method_mut(b, call, ty, name)?);
	}
	let f = match get.fallible {
		true => "bufTryGet",
		false => "bufGet",
	};
	Some(Expr::rs(f, values))
}

/// `Buf`'s method `name` for `ty`, taking the buffer as a `&mut` to it is: through the
/// `rs.Place` of one JS copies for a `&self` method.
fn buf_method_mut<'tcx>(b: &mut Body<'_, '_, 'tcx>, call: &Call<'tcx>, ty: Ty<'tcx>, name: &str) -> Option<Expr> {
	let f = buf_method(b, call, ty, name)?;
	if !b.t.needs_place(ty) || matches!(name, "advance" | "copy_to_slice" | "copy_to_bytes") {
		return Some(f);
	}
	let param = crate::ts::Param {
		name: "p".into(),
		ty: Some(crate::ts::Type::Rs("Place", vec![b.t.ty(ty, call.span)])),
	};
	Some(Expr::Lambda(
		vec![param],
		Box::new(f.call(vec![Expr::ident("p").member("v")])),
	))
}

/// The function value of `Buf`'s method `name` for the buffer `ty`, of the crate `call` is to.
fn buf_method<'tcx>(b: &mut Body<'_, '_, 'tcx>, call: &Call<'tcx>, ty: Ty<'tcx>, name: &str) -> Option<Expr> {
	let tcx = b.t.tcx;
	let buf = tcx
		.traits(call.def_id.krate)
		.iter()
		.copied()
		.find(|&tr| tcx.item_name(tr).as_str() == "Buf")?;
	let f = tcx
		.associated_items(buf)
		.in_definition_order()
		.find(|i| i.is_fn() && i.name().as_str() == name)?
		.def_id;
	Some(b.fn_value(f, tcx.mk_args(&[ty.into()]), call.span))
}

/// A `get_` or `put_` method's scalar, and whether its bytes are little-endian, as
/// wasm32's native order is.
fn scalar(name: &str) -> (&str, bool) {
	match name.strip_suffix("_le").or_else(|| name.strip_suffix("_ne")) {
		Some(ty) => (ty, true),
		None => (name, false),
	}
}

/// A function of `rand`: `rng`, or a method of its `Rng` or `RngExt` on a `ThreadRng`.
fn rand<'tcx>(
	b: &mut Body<'_, '_, 'tcx>,
	call: &Call<'tcx>,
	owner: &str,
	name: &str,
	args: Vec<Expr>,
	out: &mut Vec<Stmt>,
) -> Option<Expr> {
	let tcx = b.t.tcx;
	if tcx.def_kind(call.def_id) == DefKind::Fn && matches!(name, "rng" | "thread_rng") {
		return Some(Expr::rs("randRng", vec![]));
	}
	if !matches!(owner, "Rng" | "RngExt") || b.t.std_of(targ(call.args, 0).peel_refs()) != Some(Std::ThreadRng) {
		return None;
	}
	// The generator holds no state, so only the arguments after it are read.
	let mut args = args.into_iter().skip(1);
	let ty = targ(call.args, 1);
	let width = |int: Int| (Expr::lit(int.bits.to_string()), Expr::lit(int.signed.to_string()));
	Some(match name {
		"random" if ty.is_bool() => Expr::rs("randBool", vec![]),
		"random" => {
			let int = Int::of(ty)?;
			let (bits, signed) = width(int);
			Expr::As(Box::new(Expr::rs("randInt", vec![bits, signed])), int.ty())
		}
		"random_range" => {
			let int = Int::of(ty)?;
			let (bits, signed) = width(int);
			let range = args.next()?;
			let value = match b.t.std_of(targ(call.args, 2))? {
				Std::Range => {
					let range = b.spill(range, "range", out);
					let (start, end) = (range.clone().member("start"), range.member("end"));
					Expr::rs("randRange", vec![start, end, bits, signed])
				}
				Std::RangeTo => Expr::rs("randRange", vec![int.lit(0), range.member("end"), bits, signed]),
				Std::RangeInclusive => Expr::rs("randRangeInclusive", vec![range, bits, signed]),
				// `rand` samples `..=b` only of unsigned integers.
				Std::RangeToInclusive => Expr::rs("randRangeToInclusive", vec![range.member("end"), bits]),
				_ => return None,
			};
			Expr::As(Box::new(value), int.ty())
		}
		_ => return None,
	})
}

/// The one function of the trait `item` names, such as `AsRef::as_ref`.
fn trait_fn(tcx: TyCtxt<'_>, item: rustc_span::Symbol) -> Option<DefId> {
	let tr = tcx.get_diagnostic_item(item)?;
	tcx.associated_items(tr)
		.in_definition_order()
		.find(|i| i.is_fn())
		.map(|i| i.def_id)
}

fn is_u8(ty: Ty<'_>) -> bool {
	matches!(ty.kind(), ty::Uint(ty::UintTy::U8))
}

fn targ_is_u8(ty: Ty<'_>) -> bool {
	matches!(ty.kind(), ty::Adt(_, args) if args.types().next().is_some_and(|t| matches!(t.kind(), ty::Uint(ty::UintTy::U8))))
}

/// Whether `ty` is `&[u8]`.
fn is_bytes_ref(ty: Ty<'_>) -> bool {
	matches!(ty.kind(), ty::Ref(_, inner, _) if matches!(inner.kind(), ty::Slice(e) if matches!(e.kind(), ty::Uint(ty::UintTy::U8))))
}
