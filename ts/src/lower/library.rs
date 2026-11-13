//! Rust's standard library: its types' representations, and its functions as runtime
//! helpers or JS. A function missing here fails translation with its path.
//!
//! Functions are keyed by their owner and name: an inherent method by its type
//! (`slice::split_first`), a trait method by its trait (`From::from`), and a free
//! function by its module (`mem::take`).

use rustc_hir::def::DefKind;
use rustc_hir::def_id::DefId;
use rustc_middle::ty::{self, CallerBoundsIterator as _, GenericArgs, Instance, Ty, TyCtxt, TypingEnv};
use rustc_span::Span;

use super::body::Body;
use super::fmt;
use super::num::{self, Int, Operand, Overflow};
use super::{Translator, byte};
use crate::ts::{Expr, Stmt, StmtKind};

/// A type with a representation of its own: std's, or a well-known crate's that
/// [`super::stock`] maps.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Std {
	/// `T | undefined`.
	Option,
	/// `rs.Result<T, E>`.
	Result,
	/// `rs.ByteVec` for bytes, an array otherwise.
	Vec,
	/// A JS string.
	String,
	/// What it owns, or an `rs.Borrowed` of what it borrows, or for bytes an `rs.ByteVec`
	/// or an `rs.ByteSlice`.
	Cow,
	/// Its contents.
	Box,
	FromUtf8Error,
	Utf8Error,
	TryFromIntError,
	/// `rs.ParseIntError`: `{ kind }`, its `IntErrorKind`.
	ParseIntError,
	/// Why one of std's errors failed, as `rs.IntErrorKind` or `rs.CharErrorKind`: the
	/// variant's name.
	ErrorKind,
	/// `rs.ParseBoolError`.
	ParseBoolError,
	/// `rs.ParseCharError`: `{ kind }`, its `CharErrorKind`'s name.
	ParseCharError,
	/// `rs.FromUtf16Error`: `{ kind }`, its `FromUtf16ErrorKind`'s name.
	FromUtf16Error,
	/// `rs.CharTryFromError`: why `char::try_from` a `u32` failed.
	CharTryFromError,
	/// `rs.TryFromSliceError`: why `try_from` a slice into an array failed.
	TryFromSliceError,
	/// `rs.TryFromCharError`: why `u8::try_from` or `u16::try_from` a `char` failed.
	TryFromCharError,
	/// `rs.DecodeUtf16Error`: `{ code }`, the unpaired surrogate.
	DecodeUtf16Error,
	/// `rs.Utf8Chunk`: `{ valid, invalid }`, never changed once made.
	Utf8Chunk,
	ParseFloatError,
	TryFromFloatSecsError,
	/// `rs.Formatter`.
	Formatter,
	/// `rs.Arguments`: what `format_args!` makes.
	Arguments,
	/// `rs.FmtError`.
	FmtError,
	/// The `Debug` builders: `rs.DebugStruct` and the like.
	DebugStruct,
	DebugTuple,
	DebugList,
	/// What `assert_eq!` and `assert_ne!` compare by: its operator, as a string.
	AssertKind,
	/// `rs.Arc`, counted: an `Arc` or an `Rc`, which one thread can't tell apart.
	Arc,
	/// `rs.Weak`, of either.
	Weak,
	/// `rs.Mutex`, which is also its own guard.
	Mutex,
	MutexGuard,
	/// What it holds, a lock's guard or value.
	PoisonError,
	/// `rs.TryLockError<T>`, tagged as `Bound` is.
	TryLockError,
	/// `rs.RefCell`: a `RefCell` or an `RwLock`, which one thread can't tell apart, and
	/// also its own guard, shared or exclusive.
	RefCell,
	Ref,
	RefMut,
	/// `rs.BorrowError` and `rs.BorrowMutError`: why `try_borrow` and `try_borrow_mut` failed.
	BorrowError,
	BorrowMutError,
	/// `rs.Cell<T>`, a place holding its value: a `Cell`'s or an `UnsafeCell`'s.
	Cell,
	/// `rs.LocalKey<T>`: a thread-local, a module's value.
	LocalKey,
	/// `rs.Atomic<T>`, a place holding its `bool` or integer.
	Atomic,
	/// `rs.AtomicOrdering`: its discriminant.
	AtomicOrdering,
	/// `rs.OnceCell<T>` and `rs.OnceLock<T>`: the `Option<T>` each holds.
	OnceCell,
	OnceLock,
	/// `rs.Once`, an `rs.OnceLock` of `true`, and `rs.OnceState`, the `Once` it reports on.
	Once,
	OnceState,
	/// `rs.LazyCell<T>` and `rs.LazyLock<T>`: the function that makes the value, then the value.
	LazyCell,
	LazyLock,
	/// `rs.Waker`.
	Waker,
	/// `rs.WakerVTable`: a waker's vtable, of its four functions.
	WakerVTable,
	/// `rs.RawWaker`: a waker's data and vtable.
	RawWaker,
	/// `rs.Context`: its waker.
	Context,
	/// `rs.Poll<T>`: `T` itself, or `rs.Pending`.
	Poll,
	/// Its pointer, or the value `pin!` pins in a `PinMacroHelper`.
	Pin,
	/// `rs.PollFn`: `poll_fn`'s future, which owns its function.
	PollFn,
	/// `future::Pending`: `rs.NEVER`, a future never ready.
	Pending,
	/// `rs.Ready`: `future::ready`'s future, which owns its value until a poll takes it.
	Ready,
	/// `rs.Range`, an iterator with `start` and `end` fields.
	Range,
	/// `rs.RangeInclusive`.
	RangeInclusive,
	/// `rs.Peekable`, an iterator with `peek`.
	Peekable,
	/// What `?` branches on, as MIR spells it, and what else makes one: see [`Flow`].
	ControlFlow,
	/// `..end`, `start..`, and `..`, as MIR makes them to slice by: `{ end }`, `{ start }`,
	/// and `undefined`.
	RangeTo,
	RangeFrom,
	RangeFull,
	/// `..=end`: `{ end, inclusive: true }`.
	RangeToInclusive,
	/// `rs.RangeBound<T>`, tagged as the crate's own enums are: `{ tag: "Included", _0 }`.
	Bound,
	/// What it holds, which orders the other way.
	Reverse,
	/// Its integer, which is never 0.
	NonZero,
	/// What it holds, whose arithmetic wraps.
	Wrapping,
	/// What it holds, whose arithmetic saturates at its bounds.
	Saturating,
	/// What it holds, which it only marks as safe to unwind through.
	AssertUnwindSafe,
	/// A `format_args!` argument: a function writing its value, `rs.Fmt`.
	Argument,
	/// A raw pointer to its pointee, as `pointer` has it.
	NonNull,
	/// What it holds, or nothing yet, as `pointer` has it.
	MaybeUninit,
	/// What it holds, which it never drops itself.
	ManuallyDrop,
	/// No value: `never`.
	Infallible,
	/// `rs.HashMap`, and `rs.HashSet`, a map to `undefined`.
	HashMap,
	HashSet,
	/// `rs.BTreeMap`, and `rs.BTreeSet`, a map to `undefined`.
	BTreeMap,
	BTreeSet,
	/// `rs.VecDeque`, a ring buffer.
	VecDeque,
	/// `rs.LinkedList`, nodes linked both ways.
	LinkedList,
	/// `rs.BinaryHeap`, std's max-heap in an array, and `rs.PeekMut`, its greatest element lent.
	BinaryHeap,
	PeekMut,
	/// `rs.Entry`: a map's `Entry`, `OccupiedEntry`, or `VacantEntry`.
	Entry,
	/// `rs.HashTable`: hashbrown's `HashTable`, an `rs.HashMap` of its items by the hash its
	/// caller gives.
	HashTable,
	/// `rs.TableEntry`: a `HashTable`'s `Entry`, `OccupiedEntry`, `VacantEntry`, or `AbsentEntry`.
	TableEntry,
	/// `rs.Ordering`: its discriminant, -1, 0, or 1.
	Ordering,
	/// `rs.Hasher`: `DefaultHasher`, which hash collections hash their keys with.
	Hasher,
	/// `rs.RandomState`: the keys of the `DefaultHasher` it builds, never changed once made.
	RandomState,
	/// `BuildHasherDefault`, which holds nothing and builds its `Hasher` by `Default`.
	BuildHasherDefault,
	/// `rs.Duration`: its seconds and nanoseconds, never changed once made.
	Duration,
	/// `rs.TypeId`: rustc's 128-bit hash of the type, which std's `TypeId` holds at run time,
	/// as 32 hex digits.
	TypeId,
	/// `rs.Layout`: its size and alignment, never changed once made.
	Layout,
	/// `rs.Location`: a panic's file, line, and column, never changed once made.
	Location,
	/// `rs.PanicHookInfo`: what a panic hook is given, never changed once made.
	PanicHookInfo,
	/// `rs.LayoutError`.
	LayoutError,
	/// `rs.TryReserveError`: the layout whose allocation failed, if any, never changed once made.
	TryReserveError,
	/// `bytes::Bytes`: an `rs.ByteSlice`, as `&[u8]` is.
	Bytes,
	/// `io::IoSlice`: the `rs.ByteSlice` of the `&[u8]` it wraps.
	IoSlice,
	/// `bytes::BytesMut`: `rs.BytesMut`, a window of storage it shares with what it splits off.
	BytesMut,
	/// `rand::rngs::ThreadRng`: `rs.ThreadRng`, which holds no state.
	ThreadRng,
	/// wasm-bindgen's `JsValue`, `JsError`, or a type an import declares: the JS value itself.
	Js,
	/// wasm-bindgen's `ScopedClosure`, which `Closure` is: `rs.JsClosure`, the JS function
	/// over the closure it was made of, which throws once it drops.
	JsClosure,
	/// A handle of one of tokio's sync channels, or what one lends: a class of
	/// `rs/tokio.ts`'s, which [`super::tokio::Handle`] names.
	Sync,
}

impl Std {
	/// Whether it is one of std's errors, a `Utf8Chunk`, or a raw waker or its vtable, whose
	/// runtime fields have std's names and whose derived `PartialEq` lowers from its MIR.
	pub fn record(self) -> bool {
		matches!(
			self,
			Self::FromUtf8Error
				| Self::Utf8Error
				| Self::TryFromIntError
				| Self::ParseIntError
				| Self::ParseBoolError
				| Self::ParseCharError
				| Self::CharTryFromError
				| Self::TryFromSliceError
				| Self::TryFromCharError
				| Self::DecodeUtf16Error
				| Self::ParseFloatError
				| Self::TryFromFloatSecsError
				| Self::FmtError
				| Self::LayoutError
				| Self::Utf8Chunk
				| Self::WakerVTable
				| Self::RawWaker
		)
	}

	/// How its operators overflow, if it is a `Wrapping` or a `Saturating`.
	pub fn overflow(self) -> Option<Overflow> {
		match self {
			Self::Wrapping => Some(Overflow::Wrap),
			Self::Saturating => Some(Overflow::Saturate),
			_ => None,
		}
	}

	pub fn of(tcx: TyCtxt<'_>, did: DefId) -> Option<Self> {
		if !is_std(tcx, did) {
			if super::bindgen::represents(tcx, did) {
				return Some(Self::Js);
			}
			if super::bindgen::is_closure(tcx, did) {
				return Some(Self::JsClosure);
			}
			if super::bindgen::is_lazy(tcx, did) {
				return Some(Self::LazyCell);
			}
			return super::stock::of(tcx, did);
		}
		use rustc_span::sym;
		if tcx.is_diagnostic_item(sym::Option, did) {
			return Some(Self::Option);
		}
		if tcx.is_diagnostic_item(sym::Result, did) {
			return Some(Self::Result);
		}
		if tcx.is_diagnostic_item(sym::Vec, did) {
			return Some(Self::Vec);
		}
		if tcx.is_lang_item(did, rustc_attr_ir::LangItem::String) {
			return Some(Self::String);
		}
		if tcx.is_lang_item(did, rustc_attr_ir::LangItem::OwnedBox) {
			return Some(Self::Box);
		}
		if tcx.is_diagnostic_item(sym::Cow, did) {
			return Some(Self::Cow);
		}
		for (item, std) in [
			(sym::Arc, Self::Arc),
			(sym::ArcWeak, Self::Weak),
			(sym::Rc, Self::Arc),
			(sym::RcWeak, Self::Weak),
			(sym::Mutex, Self::Mutex),
			(sym::MutexGuard, Self::MutexGuard),
			(sym::RefCell, Self::RefCell),
			(sym::RwLock, Self::RefCell),
			(rustc_span::Symbol::intern("RefCellRef"), Self::Ref),
			(sym::RwLockReadGuard, Self::Ref),
			(rustc_span::Symbol::intern("RefCellRefMut"), Self::RefMut),
			(sym::RwLockWriteGuard, Self::RefMut),
			(sym::Cell, Self::Cell),
			(rustc_span::Symbol::intern("LocalKey"), Self::LocalKey),
			(rustc_span::Symbol::intern("Atomic"), Self::Atomic),
			(rustc_span::Symbol::intern("Waker"), Self::Waker),
			(rustc_span::Symbol::intern("PinMacroHelper"), Self::Pin),
			(rustc_span::Symbol::intern("HashMap"), Self::HashMap),
			(rustc_span::Symbol::intern("HashSet"), Self::HashSet),
			(rustc_span::Symbol::intern("BTreeMap"), Self::BTreeMap),
			(rustc_span::Symbol::intern("BTreeSet"), Self::BTreeSet),
			(rustc_span::Symbol::intern("VecDeque"), Self::VecDeque),
			(rustc_span::Symbol::intern("LinkedList"), Self::LinkedList),
			(rustc_span::Symbol::intern("BinaryHeap"), Self::BinaryHeap),
			(rustc_span::Symbol::intern("Duration"), Self::Duration),
		] {
			if tcx.is_diagnostic_item(item, did) {
				return Some(std);
			}
		}
		for (item, std) in [
			(rustc_attr_ir::LangItem::TypeId, Self::TypeId),
			(rustc_attr_ir::LangItem::UnsafeCell, Self::Cell),
			(rustc_attr_ir::LangItem::Context, Self::Context),
			(rustc_attr_ir::LangItem::ResumeTy, Self::Context),
			(rustc_attr_ir::LangItem::Poll, Self::Poll),
			(rustc_attr_ir::LangItem::Pin, Self::Pin),
			(rustc_attr_ir::LangItem::Range, Self::Range),
			(rustc_attr_ir::LangItem::RangeInclusiveStruct, Self::RangeInclusive),
			(rustc_attr_ir::LangItem::RangeTo, Self::RangeTo),
			(rustc_attr_ir::LangItem::RangeFrom, Self::RangeFrom),
			(rustc_attr_ir::LangItem::RangeFull, Self::RangeFull),
			(rustc_attr_ir::LangItem::RangeToInclusive, Self::RangeToInclusive),
		] {
			if tcx.is_lang_item(did, item) {
				return Some(std);
			}
		}
		match tcx.item_name(did).as_str() {
			"FromUtf8Error" => return Some(Self::FromUtf8Error),
			"FromUtf16Error" => return Some(Self::FromUtf16Error),
			"TryFromIntError" => return Some(Self::TryFromIntError),
			"ParseFloatError" => return Some(Self::ParseFloatError),
			"TryFromFloatSecsError" => return Some(Self::TryFromFloatSecsError),
			"PoisonError" => return Some(Self::PoisonError),
			"TryLockError" => return Some(Self::TryLockError),
			_ => {}
		}
		match std_path(tcx, did)
			.iter()
			.map(String::as_str)
			.collect::<Vec<_>>()
			.as_slice()
		{
			["str", "error", "Utf8Error"] => Some(Self::Utf8Error),
			["alloc", "layout", "Layout"] => Some(Self::Layout),
			["panic", "location", "Location"] => Some(Self::Location),
			["panic", "PanicHookInfo"] => Some(Self::PanicHookInfo),
			["alloc", "layout", "LayoutError"] => Some(Self::LayoutError),
			["collections", "TryReserveError"] => Some(Self::TryReserveError),
			["num", "error", "ParseIntError"] => Some(Self::ParseIntError),
			["num", "error", "IntErrorKind"]
			| ["char", "convert", "CharErrorKind"]
			| ["num", "float_parse", "FloatErrorKind"]
			| ["time", "TryFromFloatSecsErrorKind"] => Some(Self::ErrorKind),
			["str", "error", "ParseBoolError"] => Some(Self::ParseBoolError),
			["char", "convert", "ParseCharError"] => Some(Self::ParseCharError),
			["char", "convert", "CharTryFromError"] => Some(Self::CharTryFromError),
			["array", "TryFromSliceError"] => Some(Self::TryFromSliceError),
			["char", "TryFromCharError"] => Some(Self::TryFromCharError),
			["char", "decode", "DecodeUtf16Error"] => Some(Self::DecodeUtf16Error),
			["str", "lossy", "Utf8Chunk"] => Some(Self::Utf8Chunk),
			["io", "io_slice", "IoSlice"] => Some(Self::IoSlice),
			["fmt", "Formatter"] => Some(Self::Formatter),
			["fmt", "Arguments"] => Some(Self::Arguments),
			["fmt", "Error"] => Some(Self::FmtError),
			["fmt", "builders", "DebugStruct"] => Some(Self::DebugStruct),
			["fmt", "builders", "DebugTuple"] => Some(Self::DebugTuple),
			["fmt", "builders", "DebugList"] => Some(Self::DebugList),
			["panicking", "AssertKind"] => Some(Self::AssertKind),
			["iter", "adapters", "peekable", "Peekable"] => Some(Self::Peekable),
			["ops", "control_flow", "ControlFlow"] => Some(Self::ControlFlow),
			["fmt", "rt", "Argument"] => Some(Self::Argument),
			["ptr", "non_null", "NonNull"] => Some(Self::NonNull),
			["mem", "maybe_uninit", "MaybeUninit"] => Some(Self::MaybeUninit),
			["mem", "manually_drop", "ManuallyDrop"] => Some(Self::ManuallyDrop),
			["future", "poll_fn", "PollFn"] => Some(Self::PollFn),
			["future", "pending", "Pending"] => Some(Self::Pending),
			["future", "ready", "Ready"] => Some(Self::Ready),
			["task", "wake", "RawWakerVTable"] => Some(Self::WakerVTable),
			["task", "wake", "RawWaker"] => Some(Self::RawWaker),
			["convert", "Infallible"] => Some(Self::Infallible),
			// A thread-local on wasm32 is never destroyed, so `try_with` never fails.
			["thread", "local", "AccessError"] => Some(Self::Infallible),
			["cmp", "Ordering"] => Some(Self::Ordering),
			["sync", "atomic", "Ordering"] => Some(Self::AtomicOrdering),
			["cell", "once", "OnceCell"] => Some(Self::OnceCell),
			["cell", "BorrowError"] => Some(Self::BorrowError),
			["cell", "BorrowMutError"] => Some(Self::BorrowMutError),
			["sync", "once_lock", "OnceLock"] => Some(Self::OnceLock),
			// What a lock's guards map to, which guard as they do.
			["sync", "poison", "mutex", "MappedMutexGuard"] => Some(Self::MutexGuard),
			["sync", "poison", "rwlock", "MappedRwLockReadGuard"] => Some(Self::Ref),
			["sync", "poison", "rwlock", "MappedRwLockWriteGuard"] => Some(Self::RefMut),
			["sync", "once", "Once"] => Some(Self::Once),
			["sync", "once", "OnceState"] => Some(Self::OnceState),
			["cell", "lazy", "LazyCell"] => Some(Self::LazyCell),
			["sync", "lazy_lock", "LazyLock"] => Some(Self::LazyLock),
			["cmp", "Reverse"] => Some(Self::Reverse),
			["panic", "unwind_safe", "AssertUnwindSafe"] => Some(Self::AssertUnwindSafe),
			["collections", "binary_heap", "PeekMut"] => Some(Self::PeekMut),
			["ops", "range", "Bound"] => Some(Self::Bound),
			["num", "nonzero", "NonZero"] => Some(Self::NonZero),
			["num", "wrapping", "Wrapping"] => Some(Self::Wrapping),
			["num", "saturating", "Saturating"] => Some(Self::Saturating),
			["hash", "random", "DefaultHasher"] => Some(Self::Hasher),
			["hash", "random", "RandomState"] => Some(Self::RandomState),
			["hash", "BuildHasherDefault"] => Some(Self::BuildHasherDefault),
			["collections", "hash", "map", "Entry" | "OccupiedEntry" | "VacantEntry"]
			| [
				"collections",
				"btree",
				"map",
				"entry",
				"Entry" | "OccupiedEntry" | "VacantEntry",
			] => Some(Self::Entry),
			_ => None,
		}
	}
}

/// How a `ControlFlow<B, C>` is represented. What `?` branches on is the value it was
/// applied to, which breaks with what can only be its `Err` or `None`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flow {
	/// Breaking with a `Result<!, E>`: the `rs.Result<C, E>` `?` was applied to.
	Result,
	/// Breaking with an `Option<!>`: the `Option<C>` `?` was applied to.
	Option,
	/// Any other: an `rs.Result<C, B>`, `Ok` to continue and `Err` to break.
	Other,
}

impl Flow {
	/// The representation of a `ControlFlow` with `args`, its break's type first.
	pub fn of<'tcx>(tcx: TyCtxt<'tcx>, args: GenericArgs<'tcx>) -> Self {
		let ty::Adt(adt, residual) = args.type_at(0).kind() else {
			return Self::Other;
		};
		let uninhabited =
			|t: Ty<'tcx>| t.is_never() || matches!(t.kind(), ty::Adt(e, _) if e.is_enum() && e.variants().is_empty());
		match Std::of(tcx, adt.did()) {
			Some(Std::Result) if uninhabited(residual.type_at(0)) => Self::Result,
			Some(Std::Option) if uninhabited(residual.type_at(0)) => Self::Option,
			_ => Self::Other,
		}
	}

	/// The test that `value`, a `ControlFlow` with `args`, continues, or breaks.
	pub fn test<'tcx>(t: &Translator<'_, 'tcx>, args: GenericArgs<'tcx>, value: Expr, continues: bool) -> Expr {
		match (Self::of(t.tcx, args), continues) {
			(Self::Result | Self::Other, true) => value.member("ok"),
			(Self::Result | Self::Other, false) => value.member("ok").not(),
			(Self::Option, true) => value.binary("!==", t.none(args.type_at(1))),
			(Self::Option, false) => value.binary("===", t.none(args.type_at(1))),
		}
	}
}

/// Whether `ty` holds nothing, so its value is `undefined`: `()`, `PhantomData`,
/// `PhantomPinned`, `..`, or `BuildHasherDefault`, whose impls each compare equal and hash
/// nothing.
pub fn nothing<'tcx>(tcx: TyCtxt<'tcx>, ty: Ty<'tcx>) -> bool {
	match ty.kind() {
		ty::Tuple(tys) => tys.is_empty(),
		ty::Adt(adt, _) => {
			adt.is_phantom_data()
				|| pinned(tcx, adt.did())
				|| matches!(Std::of(tcx, adt.did()), Some(Std::RangeFull | Std::BuildHasherDefault))
		}
		_ => false,
	}
}

/// Whether `did` is `PhantomPinned`.
pub fn pinned(tcx: TyCtxt<'_>, did: DefId) -> bool {
	tcx.is_diagnostic_item(rustc_span::Symbol::intern("PhantomPinned"), did)
}

/// Whether `did` is `iter::Empty`, an `rs.Iter` that holds nothing.
pub fn empty_iter(tcx: TyCtxt<'_>, did: DefId) -> bool {
	is_std(tcx, did) && std_path(tcx, did) == ["iter", "sources", "empty", "Empty"]
}

/// Whether `did` is a list's `Iter`, `IterMut`, or `IntoIter`.
fn list_iter(tcx: TyCtxt<'_>, did: DefId) -> bool {
	is_std(tcx, did)
		&& matches!(
			std_path(tcx, did)
				.iter()
				.map(String::as_str)
				.collect::<Vec<_>>()
				.as_slice(),
			["collections", "linked_list", "Iter" | "IterMut" | "IntoIter"]
		)
}

/// Whether `Default::default()` of `ty` is an iterator that yields nothing.
pub fn empty_default(tcx: TyCtxt<'_>, ty: Ty<'_>) -> bool {
	matches!(ty.kind(), ty::Adt(adt, _) if empty_iter(tcx, adt.did()) || list_iter(tcx, adt.did()))
}

/// Whether a `ty`'s value is a JS string: a `str` or a `Box<str>`.
pub fn string<'tcx>(tcx: TyCtxt<'tcx>, ty: Ty<'tcx>) -> bool {
	match ty.kind() {
		ty::Str => true,
		ty::Adt(adt, args) => Std::of(tcx, adt.did()) == Some(Std::Box) && args.type_at(0).is_str(),
		_ => false,
	}
}

/// Whether `ty` is a `String`: an `rs.String`, whose text is its `v`.
pub fn owned_string<'tcx>(tcx: TyCtxt<'tcx>, ty: Ty<'tcx>) -> bool {
	matches!(ty.kind(), ty::Adt(adt, _) if Std::of(tcx, adt.did()) == Some(Std::String))
}

/// Whether `ty` is a `Cow<str>`: an `rs.StrCow`, the string it owns or the `rs.Borrowed`
/// one it borrows.
pub fn cow_str<'tcx>(tcx: TyCtxt<'tcx>, ty: Ty<'tcx>) -> bool {
	cow_borrowed(tcx, ty).is_some_and(|t| t.is_str())
}

/// Whether `ty` is text: a JS string, a `String`, or a `Cow<str>`, which [`text`] reads as one.
pub fn textual<'tcx>(tcx: TyCtxt<'tcx>, ty: Ty<'tcx>) -> bool {
	string(tcx, ty) || owned_string(tcx, ty) || cow_str(tcx, ty)
}

/// `value`, of type `ty`, as the JS string it is or, for a `String` or a `Cow<str>`, derefs to.
pub fn text<'tcx>(tcx: TyCtxt<'tcx>, ty: Ty<'tcx>, value: Expr) -> Expr {
	match () {
		_ if cow_str(tcx, ty) => Expr::rs("strCowStr", vec![value]),
		_ if owned_string(tcx, ty) => value.member("v"),
		_ => value,
	}
}

/// A `String` of `text`, a JS string, with room for exactly its bytes, as `to_owned` makes one.
pub fn string_from(text: Expr) -> Expr {
	Expr::rs("stringFrom", vec![text])
}

/// What a value of `ty` is in JS if `ty` wraps one value it is represented as: `Reverse<T>`'s,
/// `NonZero<T>`'s, `Wrapping<T>`'s, `Saturating<T>`'s, and `AssertUnwindSafe<T>`'s `T`, which
/// differ only in their order, range, arithmetic, and unwind safety, a pattern type's base,
/// the field of each of core's `niche_types` structs that hold one, and `bytes`'
/// `UninitSlice`'s `[MaybeUninit<u8>]`.
pub fn wrapped<'tcx>(tcx: TyCtxt<'tcx>, ty: Ty<'tcx>) -> Option<Ty<'tcx>> {
	if let Some(slice) = super::stock::uninit_slice(tcx, ty) {
		return Some(slice);
	}
	match ty.kind() {
		ty::Pat(base, _) => Some(*base),
		ty::Adt(adt, args) => match Std::of(tcx, adt.did()) {
			Some(Std::Reverse | Std::NonZero | Std::Wrapping | Std::Saturating | Std::AssertUnwindSafe) => {
				Some(args.type_at(0))
			}
			Some(_) => None,
			None => {
				let path = std_path(tcx, adt.did());
				if !(path.len() == 3 && path[0] == "num" && path[1] == "niche_types" && adt.is_struct()) {
					return None;
				}
				let field = adt.non_enum_variant().fields.iter().next()?;
				Some(field.ty(tcx, *args).skip_norm_wip())
			}
		},
		_ => None,
	}
}

/// What `ty` holds if it is an `AssertUnwindSafe`.
pub fn unwind_safe<'tcx>(tcx: TyCtxt<'tcx>, ty: Ty<'tcx>) -> Option<Ty<'tcx>> {
	match ty.kind() {
		ty::Adt(adt, args) if Std::of(tcx, adt.did()) == Some(Std::AssertUnwindSafe) => Some(args.type_at(0)),
		_ => None,
	}
}

/// A `Bound` or a `TryLockError` of type `ty`, tagged as the crate's own enums are: its
/// variant, by name, and the value it holds, if any.
pub fn tagged(variant: &str, value: Option<Expr>, ty: crate::ts::Type) -> Expr {
	let mut props = vec![(crate::ts::Key::Name("tag".into()), Expr::Str(variant.into()))];
	props.extend(value.map(|v| (crate::ts::Key::Name("_0".into()), v)));
	Expr::As(Box::new(Expr::Object(props)), ty)
}

/// `..end`, or `..=end` if `inclusive`: `{ end }`, or `{ end, inclusive: true }`.
pub fn range_to(end: Expr, inclusive: bool) -> Expr {
	let mut props = vec![(crate::ts::Key::Name("end".into()), end)];
	if inclusive {
		props.push((crate::ts::Key::Name("inclusive".into()), Expr::lit("true")));
	}
	Expr::Object(props)
}

/// What a `Box<[T]>` of type `ty` holds, `T`, or `None` for any other type.
pub fn box_slice<'tcx>(tcx: TyCtxt<'tcx>, ty: Ty<'tcx>) -> Option<Ty<'tcx>> {
	match ty.kind() {
		ty::Adt(adt, args) if Std::of(tcx, adt.did()) == Some(Std::Box) => match args.type_at(0).kind() {
			ty::Slice(elem) => Some(*elem),
			_ => None,
		},
		_ => None,
	}
}

/// `value`, a `&str` or a `Cow<str>` of type `ty`, as a `Cow<str>`, borrowing a `&str`.
pub fn str_cow<'tcx>(tcx: TyCtxt<'tcx>, ty: Ty<'tcx>, value: Expr) -> Expr {
	match cow_str(tcx, ty) {
		true => value,
		false => cow_made(tcx.types.str_, rustc_abi::FIRST_VARIANT, value),
	}
}

/// What a `Cow` of type `ty` borrows, a `[T]` or a `str`, or `None` for any other type.
pub fn cow_borrowed<'tcx>(tcx: TyCtxt<'tcx>, ty: Ty<'tcx>) -> Option<Ty<'tcx>> {
	match ty.kind() {
		// Its first argument is its lifetime.
		ty::Adt(adt, args) if Std::of(tcx, adt.did()) == Some(Std::Cow) => {
			let borrowed = args.types().next()?;
			(borrowed.is_slice() || borrowed.is_str()).then_some(borrowed)
		}
		_ => None,
	}
}

/// What a `Cow<[T]>` of type `ty` holds, `T`, or `None` for any other type.
pub fn cow_slice<'tcx>(tcx: TyCtxt<'tcx>, ty: Ty<'tcx>) -> Option<Ty<'tcx>> {
	match cow_borrowed(tcx, ty)?.kind() {
		ty::Slice(elem) => Some(*elem),
		_ => None,
	}
}

/// Whether `borrowed`, what a `Cow` borrows, is bytes, which it keeps as an `rs.ByteSlice`.
fn cow_bytes(borrowed: Ty<'_>) -> bool {
	matches!(borrowed.kind(), ty::Slice(elem) if byte(*elem))
}

/// What `cow`, a `Cow` of `borrowed`, derefs to: an `rs.ByteSlice` of bytes, an array, or
/// a string.
pub fn cow_deref(cow: Expr, borrowed: Ty<'_>) -> Expr {
	match borrowed.kind() {
		ty::Str => Expr::rs("strCowStr", vec![cow]),
		_ if cow_bytes(borrowed) => Expr::rs("byteCowSlice", vec![cow]),
		ty::Slice(elem) if elem.is_array() => Expr::rs("cowArraySlice", vec![cow, super::array_fn(*elem, "From")]),
		_ => Expr::rs("cowSlice", vec![cow]),
	}
}

/// A `Cow` of `borrowed` made by its variant `variant` of `value`: what it owns, or what
/// it borrows, wrapped in an `rs.Borrowed` unless it is bytes.
pub fn cow_made(borrowed: Ty<'_>, variant: rustc_abi::VariantIdx, value: Expr) -> Expr {
	match variant.index() == 0 && !cow_bytes(borrowed) {
		true => Expr::New(Box::new(Expr::Rs("Borrowed".into())), vec![value]),
		false => value,
	}
}

/// Whether `cow`, a `Cow` of `borrowed`, borrows: whether it is of the class its borrowed
/// variant is.
pub fn cow_borrows(cow: Expr, borrowed: Ty<'_>) -> Expr {
	let class = match cow_bytes(borrowed) {
		true => "ByteSlice",
		false => "Borrowed",
	};
	cow.binary("instanceof", Expr::Rs(class.into()))
}

/// The path of a std item within its crate, as `["slice", "iter", "Iter"]`; empty for
/// any other crate's.
pub fn std_path(tcx: TyCtxt<'_>, did: DefId) -> Vec<String> {
	if !is_std(tcx, did) {
		return Vec::new();
	}
	tcx.def_path(did)
		.data
		.iter()
		.filter_map(|d| d.data.get_opt_name())
		.map(|n| n.to_string())
		.collect()
}

pub fn is_std(tcx: TyCtxt<'_>, did: DefId) -> bool {
	matches!(tcx.crate_name(did.krate).as_str(), "core" | "alloc" | "std")
}

/// If `instance` is a method of std's `PartialEq`, `PartialOrd`, `Ord`, `Hash`, `AsRef`, or
/// `AsMut` impl for references, or of its `Hasher` or `fmt::Write` impl for a `&mut`, or its
/// `Hasher` impl for a box, which calls its referents' with the operands dereferenced, the
/// generic arguments of that call.
pub fn referent<'tcx>(
	tcx: TyCtxt<'tcx>,
	def_id: DefId,
	gargs: GenericArgs<'tcx>,
	instance: Instance<'tcx>,
) -> Option<GenericArgs<'tcx>> {
	let tr = tcx.trait_of_assoc(def_id)?;
	// A method the impl leaves to its trait's default, as `Ord::max` is, is not forwarded.
	let imp = tcx.impl_of_assoc(instance.def_id())?;
	if !is_std(tcx, imp) {
		return None;
	}
	let hasher = std_path(tcx, tr) == ["hash", "Hasher"];
	let operands = match tcx.get_diagnostic_name(tr).as_ref().map(|n| n.as_str()) {
		Some("PartialEq" | "PartialOrd") => 2,
		Some("Ord" | "Hash" | "FmtWrite" | "AsRef" | "AsMut") => 1,
		_ if hasher => 1,
		_ => return None,
	};
	let mut args = gargs.as_slice().to_vec();
	for arg in &mut args[..operands] {
		let ty = arg.as_type()?;
		*arg = match ty.kind() {
			ty::Ref(_, pointee, _) => (*pointee).into(),
			_ if hasher && let Some(held) = ty.boxed_ty() => held.into(),
			_ => return None,
		};
	}
	Some(tcx.mk_args(&args))
}

/// A comparison an impl of `PartialEq`, `PartialOrd`, or `Iterator` inherits from its
/// trait, as a derived one inherits all but `eq` and `partial_cmp`.
#[derive(Clone, Copy)]
pub enum Inherited {
	Ne,
	Lt,
	Le,
	Gt,
	Ge,
}

impl Inherited {
	/// The comparison `instance` is, if it is the trait's own default, and what it calls,
	/// with the same generic arguments and operands: the trait's `eq` or `partial_cmp`.
	pub fn of(tcx: TyCtxt<'_>, instance: Instance<'_>) -> Option<(Self, DefId)> {
		let ty::InstanceKind::Item(did) = instance.def else {
			return None;
		};
		let tr = tcx.trait_of_assoc(did)?;
		let inherited = match (tcx.get_diagnostic_name(tr)?.as_str(), tcx.item_name(did).as_str()) {
			("PartialEq" | "Iterator", "ne") => Self::Ne,
			("PartialOrd" | "Iterator", "lt") => Self::Lt,
			("PartialOrd" | "Iterator", "le") => Self::Le,
			("PartialOrd" | "Iterator", "gt") => Self::Gt,
			("PartialOrd" | "Iterator", "ge") => Self::Ge,
			_ => return None,
		};
		let via = match inherited {
			Self::Ne => "eq",
			_ => "partial_cmp",
		};
		let via = tcx
			.associated_items(tr)
			.filter_by_name_unhygienic(rustc_span::Symbol::intern(via))
			.next()?
			.def_id;
		Some((inherited, via))
	}

	/// The comparison, from what `via` returned: `!eq`, or whether the `partial_cmp` is
	/// `Some` and of the order wanted, as std's defaults test it.
	pub fn test(self, b: &mut Body<'_, '_, '_>, value: Expr, out: &mut Vec<Stmt>) -> Expr {
		let (op, order) = match self {
			Self::Ne => return value.not(),
			Self::Lt => return value.binary("===", Expr::lit("-1")),
			Self::Gt => return value.binary("===", Expr::lit("1")),
			Self::Le => ("<=", "0"),
			Self::Ge => (">=", "0"),
		};
		let value = match value.is_path() {
			true => value,
			false => b.spill(value, "order", out),
		};
		let some = value.clone().binary("!==", Expr::undefined());
		some.binary("&&", value.binary(op, Expr::lit(order)))
	}
}

/// Whether `Clone` of `ty` only copies: `ty` is `Copy`, and no `Clone` written by hand
/// runs for it or anything in it, as a derived or std `Clone` clones each field. `env`
/// holds the bounds of the body naming `ty`.
pub fn clone_copies<'tcx>(t: &super::Translator<'_, 'tcx>, env: TypingEnv<'tcx>, ty: Ty<'tcx>) -> bool {
	t.tcx.type_is_copy_modulo_regions(env, ty) && clone_only_copies(t, env, ty)
}

/// Whether `Clone` of `ty` is the value itself, as JS holds it: a string, a function, or a
/// closure that writes none of its captures, each of which only copies or clones so, as
/// nothing else reaches what a closure owns to write it.
pub fn clone_shares<'tcx>(t: &super::Translator<'_, 'tcx>, env: TypingEnv<'tcx>, ty: Ty<'tcx>) -> bool {
	match ty.kind() {
		ty::FnDef(..) => true,
		ty::Closure(..) | ty::CoroutineClosure(..) => {
			!super::mir::writes_captures(t.tcx, ty)
				&& (super::captured(ty).unwrap_or_default().iter())
					.all(|capture| clone_copies(t, env, capture) || clone_shares(t, env, capture))
		}
		_ => string(t.tcx, ty),
	}
}

/// Whether `Clone` of `ty`, a `Copy` type, only copies.
fn clone_only_copies<'tcx>(t: &super::Translator<'_, 'tcx>, env: TypingEnv<'tcx>, ty: Ty<'tcx>) -> bool {
	let tcx = t.tcx;
	match ty.kind() {
		ty::Bool | ty::Char | ty::Int(_) | ty::Uint(_) | ty::Float(_) | ty::Ref(..) | ty::FnPtr(..) | ty::Never => true,
		ty::Tuple(fields) => fields.iter().all(|field| clone_only_copies(t, env, field)),
		ty::Array(elem, _) => clone_only_copies(t, env, *elem),
		// A binding's `Clone` is its own.
		ty::Adt(adt, _) if t.bound_drops(adt.did()).is_some() => false,
		ty::Adt(adt, args) if !t.crates.translates(adt.did()) => args.types().all(|arg| clone_only_copies(t, env, arg)),
		ty::Adt(adt, args) => {
			let Some(clone) = tcx.lang_items().clone_fn() else {
				return false;
			};
			let derived = Instance::try_resolve(tcx, env, clone, tcx.mk_args(&[ty.into()]))
				.ok()
				.flatten()
				.and_then(|instance| tcx.impl_of_assoc(instance.def_id()))
				.is_some_and(|imp| tcx.is_automatically_derived(imp));
			derived
				&& adt
					.all_fields()
					.all(|field| clone_only_copies(t, env, super::copy::field_ty(tcx, env, field.did, *args)))
		}
		_ => false,
	}
}

/// The standard type an inherent method is of, such as `Range` for `Range::contains`.
pub fn inherent_std(tcx: TyCtxt<'_>, def_id: DefId) -> Option<Std> {
	let imp = tcx.inherent_impl_of_assoc(def_id)?;
	match tcx.type_of(imp).instantiate_identity().skip_norm_wip().kind() {
		ty::Adt(adt, _) => Std::of(tcx, adt.did()),
		_ => None,
	}
}

/// Whether std's `instance` lowers from its own MIR, as the crate's functions do, rather
/// than mapping here: a method of `bool`, `Option`, `Result`, `Poll`, or `LocalKey`, or
/// their `Clone`, or the derived `PartialEq::eq` of a `Result`, a `Poll`, or one of std's
/// errors, or a `Result`'s or a `Poll`'s derived `PartialOrd::partial_cmp` and `Ord::cmp`,
/// or `cmp`'s `max` and `min` and their `_by` and `_by_key` forms, or what
/// parses an integer, a `NonZero`, or a `bool`: `str::parse` of one, its `FromStr`,
/// `from_str_radix`, and what they call, `char::to_digit` and the radix's panic among it, and
/// `char::is_digit`, which calls it.
/// Not one taking a `&mut` or a raw pointer, whose writes the crate's analysis of what it
/// writes into would miss, nor an `unsafe` one, nor one mapped below as one expression or
/// as the runtime's panic.
pub fn from_mir<'tcx>(t: &Translator<'_, 'tcx>, env: TypingEnv<'tcx>, instance: Instance<'tcx>) -> bool {
	let tcx = t.tcx;
	let ty::InstanceKind::Item(did) = instance.def else {
		return false;
	};
	if !may_from_mir(tcx, did) {
		return false;
	}
	let Some(imp) = tcx.impl_of_assoc(did) else {
		return true;
	};
	let name = tcx.item_name(did);
	let self_ty = tcx.type_of(imp).instantiate_identity().skip_norm_wip();
	if self_ty.is_str() {
		let target = instance.args.type_at(0);
		return int(tcx, target) || target.is_bool();
	}
	if let Some(tr) = tcx.trait_impl_of_assoc(did).map(|imp| tcx.impl_trait_id(imp))
		&& tcx.is_lang_item(tr, rustc_attr_ir::LangItem::Clone)
	{
		return !clone_copies(t, env, super::copy::instantiate(tcx, env, self_ty, instance.args));
	}
	let std = match self_ty.kind() {
		ty::Adt(adt, _) => Std::of(tcx, adt.did()),
		_ => None,
	};
	// A `RefCell` key's `set` maps, as its body writes the cell in place.
	let refcell_set = std == Some(Std::LocalKey)
		&& name.as_str() == "set"
		&& matches!(super::copy::instantiate(tcx, env, self_ty, instance.args).kind(), ty::Adt(_, args)
			if matches!(args.type_at(0).kind(), ty::Adt(adt, _) if Std::of(tcx, adt.did()) == Some(Std::RefCell)));
	// A lock's `unwrap_or_else` maps, to unwrap its guard in place.
	let lock = std == Some(Std::Result)
		&& name.as_str() == "unwrap_or_else"
		&& matches!(instance.args.type_at(1).kind(), ty::Adt(adt, _) if Std::of(tcx, adt.did()) == Some(Std::PoisonError));
	!lock && !refcell_set
}

/// The generic arguments of the first function `callee` picks that std's body of `def_id`,
/// instantiated by `args`, calls: what that body passes on, such as an adaptor it wraps an
/// iterator in.
pub fn callee_args<'tcx>(
	t: &Translator<'_, 'tcx>,
	def_id: DefId,
	args: GenericArgs<'tcx>,
	callee: impl Fn(DefId) -> bool,
) -> Option<GenericArgs<'tcx>> {
	let tcx = t.tcx;
	let env = t.typing_env_pub();
	let instance = Instance::try_resolve(tcx, env, def_id, args).ok()??;
	let called = tcx.instance_mir(instance.def).basic_blocks.iter().find_map(|block| {
		let rustc_middle::mir::TerminatorKind::Call { func, .. } = &block.terminator().kind else {
			return None;
		};
		func.const_fn_def().filter(|&(f, _)| callee(f)).map(|(_, args)| args)
	})?;
	instance
		.try_instantiate_mir_and_normalize_erasing_regions(tcx, env, ty::EarlyBinder::bind(tcx, called))
		.ok()
}

/// Whether some instance of std's `did` may lower from its MIR, as [`from_mir`] decides
/// for each: what only the instance decides is left to it.
pub fn may_from_mir(tcx: TyCtxt<'_>, did: DefId) -> bool {
	if !is_std(tcx, did) {
		return false;
	}
	if tcx.def_kind(did) == DefKind::Fn {
		let path = std_path(tcx, did);
		return matches!(
			path.iter().map(String::as_str).collect::<Vec<_>>().as_slice(),
			["cmp", "max" | "min" | "max_by" | "min_by" | "max_by_key" | "min_by_key"]
				| ["num", "can_not_overflow"]
				// With the functions `const_panic!` nests in it.
				| ["num", "from_ascii_bytes_radix_panic", ..]
		);
	}
	if tcx.def_kind(did) != DefKind::AssocFn {
		return false;
	}
	let Some(imp) = tcx.impl_of_assoc(did) else {
		return false;
	};
	let name = tcx.item_name(did);
	let self_ty = tcx.type_of(imp).instantiate_identity().skip_norm_wip();
	let parses = match self_ty.kind() {
		_ if int(tcx, self_ty) => {
			matches!(
				name.as_str(),
				"from_str" | "from_str_radix" | "from_ascii_bytes_radix_impl"
			)
		}
		// Of an integer or a `bool`, which the instance says.
		ty::Str => name.as_str() == "parse",
		ty::Bool => name.as_str() == "from_str",
		ty::Char => matches!(name.as_str(), "to_digit" | "is_digit"),
		_ => false,
	};
	if parses {
		return true;
	}
	// A std error's or a `Utf8Chunk`'s derived `PartialEq`, which compares its fields.
	if let ty::Adt(adt, _) = self_ty.kind()
		&& Std::of(tcx, adt.did()).is_some_and(Std::record)
		&& tcx.is_automatically_derived(imp)
	{
		return name == rustc_span::sym::eq;
	}
	// `None` for `bool`'s.
	let std = match self_ty.kind() {
		ty::Adt(adt, _) => match Std::of(tcx, adt.did()) {
			std @ Some(Std::Option | Std::Result | Std::Poll | Std::LocalKey) => std,
			_ => return false,
		},
		ty::Bool => None,
		_ => return false,
	};
	let sig = tcx.fn_sig(did).instantiate_identity().skip_norm_wip().skip_binder();
	let writes = |ty: Ty<'_>| {
		ty.walk()
			.filter_map(|arg| arg.as_type())
			.any(|t| matches!(t.kind(), ty::Ref(_, _, ty::Mutability::Mut) | ty::RawPtr(..)))
	};
	if sig.safety().is_unsafe() || sig.inputs().iter().any(|&ty| writes(ty)) {
		return false;
	}
	if let Some(tr) = tcx.trait_impl_of_assoc(did).map(|imp| tcx.impl_trait_id(imp)) {
		// A `Result`'s or a `Poll`'s derived `PartialEq`, `PartialOrd`, and `Ord`, which compare
		// by variant and then what each holds by its own.
		if matches!(std, Some(Std::Result | Std::Poll)) {
			if tcx.is_lang_item(tr, rustc_attr_ir::LangItem::PartialEq) {
				return name == rustc_span::sym::eq;
			}
			if tcx.is_lang_item(tr, rustc_attr_ir::LangItem::PartialOrd) {
				return name == rustc_span::sym::partial_cmp;
			}
			if tcx.is_diagnostic_item(rustc_span::sym::Ord, tr) {
				return name == rustc_span::sym::cmp;
			}
		}
		// A `Clone` that only copies is the copy, which the instance says.
		return tcx.is_lang_item(tr, rustc_attr_ir::LangItem::Clone) && name == rustc_span::sym::clone;
	}
	!matches!(
		(std, name.as_str()),
		(
			Some(Std::Option),
			"unwrap" | "expect" | "is_some" | "is_none" | "as_ref" | "copied" | "cloned" | "iter" | "as_slice"
		) | (
			Some(Std::Result),
			"unwrap" | "expect" | "unwrap_err" | "is_ok" | "is_err" | "iter"
		) | (
			Some(Std::LocalKey),
			"with" | "try_with" | "initialize_with" | "with_borrow_mut"
		)
	)
}

/// An integer or a `NonZero` of one, which parse alike.
fn int<'tcx>(tcx: TyCtxt<'tcx>, ty: Ty<'tcx>) -> bool {
	match ty.kind() {
		ty::Int(_) | ty::Uint(_) => true,
		ty::Adt(adt, _) => Std::of(tcx, adt.did()) == Some(Std::NonZero),
		_ => false,
	}
}

/// The key `def_id` makes the value of, if it is what `thread_local!` declares in a key's
/// own initializer to: a `const { .. }` key's `const`, or a lazy key's function.
pub fn local_init(tcx: TyCtxt<'_>, def_id: DefId) -> Option<DefId> {
	let key = tcx.opt_parent(def_id)?;
	let is_key = tcx.def_kind(key) == DefKind::Const
		&& matches!(tcx.type_of(key).instantiate_identity().skip_norm_wip().kind(),
			ty::Adt(adt, _) if Std::of(tcx, adt.did()) == Some(Std::LocalKey));
	is_key.then_some(key)
}

pub struct Call<'tcx> {
	/// The function as written, before resolving a trait method to its impl.
	pub def_id: DefId,
	/// Its generic arguments: the impl's or trait's, then the function's.
	pub args: GenericArgs<'tcx>,
	pub span: Span,
}

/// What the mapping of `owner::name` may discard of the operands it takes by value, rather
/// than move into its result or a closure, as it returns: the types, by their index among
/// the call's type arguments, whose droppers its runtime function takes after its own
/// arguments, as `unwrap_or` drops its fallback, or a function it always calls once
/// through `FnOnce`, which that call consumes. `None` for any other mapping, which is
/// refused an operand that drops. Iterators refuse their own.
fn discards(owner: &str, name: &str) -> Option<&'static [usize]> {
	Some(match (owner, name) {
		("Option", "unwrap" | "expect")
		| ("Vec", "push" | "into_boxed_slice" | "into_flattened" | "leak")
		| ("slice", "into_vec")
		| ("Box", "new" | "pin")
		| ("mem", "replace" | "take" | "swap" | "forget")
		| ("ptr", "write" | "write_unaligned" | "write_volatile" | "replace")
		| ("MaybeUninit", "new" | "write" | "assume_init")
		| ("ManuallyDrop", "new" | "into_inner")
		| ("Box", "into_raw" | "leak" | "from_raw")
		| ("Arc" | "Rc", "into_raw")
		| ("hint", "must_use")
		| ("From", "from")
		| ("Into", "into")
		| ("TryFrom", "try_from")
		| ("TryInto", "try_into")
		// What a panic would drop, which aborts.
		| ("Result", "unwrap" | "expect" | "unwrap_err")
		| ("Vec", "insert")
		| ("RangeInclusive", "new")
		| ("panic", "panic_any" | "resume_unwind")
		| ("panicking", "begin_panic")
		// The hook it keeps, and the one it replaces, which `rs.setHook` drops.
		| ("panicking", "set_hook")
		// Each element moves into the closure.
		| ("array", "map") => &[],
		// `Vec<T>` drops each element `retain` rejects, `dedup` removes, and `truncate`,
		// `clear`, and a shrinking `resize` and `resize_with` cut, and the value `resize` then
		// has no slot for.
		(
			"Vec",
			"retain" | "retain_mut" | "dedup" | "dedup_by" | "dedup_by_key" | "truncate" | "clear" | "resize"
			| "resize_with",
		) => &[0],
		// A slice's `fill`, `fill_with`, and `clone_from_slice` drop each element they write
		// over, and `fill` its value if there is no slot for it.
		("slice", "fill" | "fill_with" | "clone_from_slice") => &[0],
		// `vec![x; 0]` drops `x`.
		("vec", "from_elem_global") => &[0],
		// `get_or_init::<T, F>` drops its function uncalled if the cell is set.
		("OnceCell" | "OnceLock", "get_or_init" | "get_or_try_init") => &[1],
		// A guard's map calls its function, filtered or not.
		_ if maps_guard(owner, name) => &[],
		// A guard it leaks or downgrades keeps its borrow or lock.
		("Ref" | "RefMut", "leak") | ("RwLockWriteGuard", "downgrade") => &[],
		("Once", "call_once" | "call_once_force") | ("panic", "catch_unwind") => &[0],
		("Arc" | "Rc", "new_cyclic") => &[1],
		// `pop_if::<F>` drops its test uncalled if there is no element.
		("Vec", "pop_if") => &[2],
		// `is_sorted_by::<F>` drops its comparator before the last item, as `all` takes it.
		("Iterator", "is_sorted_by") => &[1],
		("LocalKey", "with" | "try_with" | "initialize_with" | "with_borrow_mut") => &[1],
		("Cell", "update") | ("RefCell", "replace_with") => &[1],
		_ => return None,
	})
}

/// Whether the mapping of `owner::name` drops each closure it takes by value where std's
/// does: a consumer's as its fold ends, before the iterator, or as it finds the iterator
/// empty, and an entry's if it never calls it.
fn releases(owner: &str, name: &str) -> bool {
	matches!(
		(owner, name),
		(
			"Iterator",
			"for_each"
				| "fold"
				| "try_fold"
				| "try_for_each"
				| "partition"
				| "reduce"
				| "max_by"
				| "min_by"
				| "max_by_key"
				| "min_by_key"
				| "is_sorted_by_key"
				| "eq_by"
				| "cmp_by"
				| "partial_cmp_by"
		) | ("DoubleEndedIterator", "rfold" | "try_rfold")
			| ("Entry", "or_insert_with" | "or_insert_with_key" | "and_modify")
			| ("slice", "sort_by_cached_key")
	)
}

/// Whether `owner::name` is a guard's map, filtered, tried, or split, to parts of what it lends.
pub fn maps_guard(owner: &str, name: &str) -> bool {
	matches!(
		(owner, name),
		(
			"Ref"
				| "RefMut"
				| "MutexGuard"
				| "MappedMutexGuard"
				| "RwLockReadGuard"
				| "RwLockWriteGuard"
				| "MappedRwLockReadGuard"
				| "MappedRwLockWriteGuard",
			"map" | "filter_map" | "try_map" | "map_split",
		)
	)
}

/// Whether the mapping of `owner::name` tells an `Option`'s `None` from its `Some` where
/// the `None` is `rs.None`, as the runtime's functions that take `None` to be `undefined`
/// do not.
fn takes_none(owner: &str, name: &str) -> bool {
	none_last(owner, name)
		|| matches!(
			(owner, name),
			(
				"Option",
				"is_some" | "is_none" | "as_ref" | "copied" | "cloned" | "iter" | "iter_mut"
			) | (
				"Iterator",
				"next" | "nth" | "last" | "find" | "max" | "min" | "max_by_key" | "min_by_key"
			) | (
				"Iterator",
				"max_by" | "min_by" | "reduce" | "find_map" | "filter_map" | "map_while"
			) | ("Iterator", "try_fold" | "try_for_each" | "scan")
				| ("Peekable", "peek" | "next_if" | "next_if_eq")
				| (
					"OnceCell" | "OnceLock",
					"get" | "get_mut" | "set" | "get_or_init" | "get_or_try_init" | "take" | "into_inner"
				)
				| ("Arc" | "Rc", "into_inner")
				| ("DoubleEndedIterator", "next_back" | "nth_back" | "rfind" | "try_rfold")
				| ("dyn Any" | "dyn Error", "downcast_ref" | "downcast_mut")
				| ("ptr", "as_ref" | "as_mut")
				| ("from_fn", "from_fn")
				| ("successors", "successors")
				| ("Option", "as_slice" | "as_mut_slice")
		)
}

/// Whether the runtime function `owner::name` maps to takes the `None` of the `Option` it
/// makes or reads, where that is `rs.None`, last: after what drops its operands.
pub(super) fn none_last(owner: &str, name: &str) -> bool {
	matches!(
		(owner, name),
		("Option", "unwrap" | "expect")
			| ("slice", "first" | "last" | "get" | "first_mut" | "last_mut" | "get_mut")
			| ("Vec", "pop" | "pop_if")
			| (
				"VecDeque",
				"pop_front"
					| "pop_back"
					| "front"
					| "back"
					| "front_mut"
					| "back_mut"
					| "get"
					| "get_mut"
					| "remove"
					| "swap_remove_front"
					| "swap_remove_back"
			)
			| (
				"LinkedList",
				"pop_front" | "pop_back" | "front" | "back" | "front_mut" | "back_mut"
			)
			| ("BinaryHeap", "pop" | "peek")
			| ("HashSet", "get" | "take" | "replace")
			| ("HashTable", "find")
			| (
				"BTreeSet",
				"first" | "last" | "pop_first" | "pop_last" | "get" | "take" | "replace"
			)
			| ("HashMap" | "BTreeMap", "get" | "get_mut" | "insert" | "remove")
	)
}

/// The type of the first `Some` whose `Option`'s `None` is `rs.None`, of those in the
/// signature and bounds of the function `call` calls, over its type arguments: what the
/// mapping of a std function would make or read with `undefined` for `None`.
pub(super) fn sentinel_option<'tcx>(t: &Translator<'_, 'tcx>, call: &Call<'tcx>) -> Option<Ty<'tcx>> {
	let tcx = t.tcx;
	let sig = tcx.fn_sig(call.def_id).instantiate_identity().skip_norm_wip();
	let sig = tcx.instantiate_bound_regions_with_erased(sig);
	let bounds = tcx.param_env(call.def_id).caller_bounds().all_clauses(tcx).iter();
	let bound_args = bounds.flat_map(|clause| {
		let trait_args = clause.as_trait_clause().map(|c| c.skip_binder().trait_ref.args);
		let projection = clause.as_projection_clause().map(|c| c.skip_binder());
		let projection_args = projection.map(|p| p.projection_term.args);
		let term = projection.and_then(|p| p.term.as_type());
		(trait_args.into_iter().chain(projection_args))
			.flat_map(|args| args.types().collect::<Vec<_>>())
			.chain(term)
	});
	let env = t.typing_env_pub();
	sig.inputs_and_output
		.iter()
		.chain(bound_args)
		.flat_map(|ty| ty.walk().filter_map(|arg| arg.as_type()).collect::<Vec<_>>())
		.filter_map(|ty| match ty.kind() {
			ty::Adt(adt, args) if Std::of(tcx, adt.did()) == Some(Std::Option) => Some(args.type_at(0)),
			_ => None,
		})
		.map(|inner| {
			let inner = ty::EarlyBinder::bind(tcx, inner)
				.instantiate(tcx, call.args)
				.skip_norm_wip();
			let inner = tcx.erase_and_anonymize_regions(inner);
			tcx.try_normalize_erasing_regions(env, ty::Unnormalized::new_wip(inner))
				.unwrap_or(inner)
		})
		.find(|&inner| t.erases_to(inner).undefined)
}

/// What a std function does with the closures it calls that own a value that drops.
#[derive(Default)]
struct Closures<'tcx> {
	/// The first it would neither drop where std's does nor keep: refused.
	leaked: Option<Ty<'tcx>>,
	/// Those it drops as it returns, having kept nothing of them, in the order
	/// their parameters are declared.
	spent: Vec<Expr>,
}

/// What an argument of type `ty` that std calls holds, through any references, boxes, and
/// `AssertUnwindSafe`s, whether a reference lends it, and `value`, the argument, read as
/// that: a `&mut` to a function JS copies is its place, so std calls the function it holds.
pub fn called_through<'tcx>(
	t: &Translator<'_, 'tcx>,
	ty: Ty<'tcx>,
	mut value: Option<Expr>,
) -> (Ty<'tcx>, bool, Option<Expr>) {
	let (mut owned, mut lent) = (ty, false);
	loop {
		owned = match *owned.kind() {
			ty::Ref(_, inner, _) => {
				if t.place_pointee(owned).is_some() {
					value = value.map(|f| f.member("v"));
				}
				lent = true;
				inner
			}
			ty::Adt(adt, args) if matches!(Std::of(t.tcx, adt.did()), Some(Std::Box | Std::AssertUnwindSafe)) => {
				args.type_at(0)
			}
			_ => break,
		};
	}
	(owned, lent, value)
}

/// The closures `call` calls that own a value that drops, by what its mapping does with
/// each: kept by the iterator it returns, if `iter::holds_closure`; dropped by the mapping,
/// if `released`, as `releases` says; dropped as it returns, if `Fn` or `FnMut` call
/// it and nothing else it takes by value drops, or the mapping discards none of it, as then
/// nothing it drops is ordered against it; dropped where `discarded` says, by the type's
/// index among the call's type arguments; or else leaked. A closure that `Fn` or `FnMut`
/// call, given to one that calls it through `FnOnce`, becomes `rs.once` of it, which drops
/// it after the call, as rustc's shim does. A reference to one is the closure in JS, so an
/// iterator, which drops what it holds, keeps `rs.lent` of it instead.
fn called_closures<'tcx>(
	b: &mut Body<'_, '_, 'tcx>,
	call: &Call<'tcx>,
	discarded: Option<&[usize]>,
	released: bool,
	args: &mut [Expr],
) -> Closures<'tcx> {
	let tcx = b.t.tcx;
	let mut closures = Closures::default();
	let sig = tcx.fn_sig(call.def_id).instantiate_identity().skip_norm_wip();
	let sig = tcx.instantiate_bound_regions_with_erased(sig);
	let inputs = sig.inputs().to_vec();
	let output = sig.output();
	for (i, &input) in inputs.iter().enumerate() {
		let ty::Param(param) = *input.kind() else {
			continue;
		};
		let ty = call.args[param.index as usize].expect_ty();
		let Some(bound) = called_by(tcx, call.def_id, input) else {
			continue;
		};
		let value = (i < args.len()).then(|| std::mem::replace(&mut args[i], Expr::undefined()));
		let (owned, lent, value) = called_through(b.t, ty, value);
		if let Some(value) = value {
			args[i] = value;
		}
		let Some(called) = b.t.owned_fn(owned) else {
			continue;
		};
		if !b.t.drops(owned) {
			continue;
		}
		let mentions = |t: Ty<'tcx>| t.walk().any(|a| a.as_type() == Some(input));
		if mentions(output) {
			// An `rs.Iter` drops the closure it holds, an `rs.PollFn` one it owns, and an
			// `rs.LazyCell` or a `OnceWith` one it never calls, and one it calls through
			// `FnOnce` as it does.
			let holder = match output.kind() {
				ty::Adt(adt, args) if args.types().any(|a| a == input) => Some(adt.did()),
				_ => None,
			};
			let iter = holder.is_some_and(|did| super::iter::holds_closure(tcx, did));
			let holds = holder.and_then(|did| Std::of(tcx, did));
			let lazy = matches!(holds, Some(Std::LazyCell | Std::LazyLock))
				|| holder.is_some_and(|did| std_path(tcx, did) == ["iter", "sources", "once_with", "OnceWith"]);
			if lent && iter && i < args.len() {
				let f = std::mem::replace(&mut args[i], Expr::undefined());
				args[i] = Expr::rs("lent", vec![f]);
			} else if lazy && !lent && called != ty::ClosureKind::FnOnce && i < args.len() {
				let f = std::mem::replace(&mut args[i], Expr::undefined());
				args[i] = Expr::rs("once", vec![f]);
			} else if !lent && !iter && !lazy && holds != Some(Std::PollFn) {
				closures.leaked = closures.leaked.or(Some(ty));
			}
			continue;
		}
		// Its owner drops a borrowed closure.
		if lent {
			continue;
		}
		let index = call.args.as_slice()[..param.index as usize]
			.iter()
			.filter(|a| a.as_type().is_some())
			.count();
		let discards = released || discarded.is_some_and(|d| d.contains(&index));
		let kept = inputs.iter().enumerate().any(|(j, &t)| j != i && mentions(t));
		if bound != ty::ClosureKind::FnOnce && !kept && !discards && i < args.len() {
			let dropping = match discarded {
				Some([]) => None,
				_ => dropping_operand(b, call),
			};
			match dropping {
				None => closures.spent.push(args[i].clone()),
				Some(_) => closures.leaked = closures.leaked.or(Some(ty)),
			}
			continue;
		}
		if !discards {
			closures.leaked = closures.leaked.or(Some(ty));
			continue;
		}
		if bound == ty::ClosureKind::FnOnce && called != ty::ClosureKind::FnOnce && i < args.len() {
			let f = std::mem::replace(&mut args[i], Expr::undefined());
			args[i] = Expr::rs("once", vec![f]);
		}
	}
	closures
}

/// The trait `def_id` calls its parameter of type `input` by: `Fn`, then `FnMut`,
/// whichever it is bound by, or else `FnOnce`, or `None` if no function trait bounds it.
/// A `str` search's `Pattern` calls a closure by `FnMut`.
fn called_by<'tcx>(tcx: TyCtxt<'tcx>, def_id: DefId, input: Ty<'tcx>) -> Option<ty::ClosureKind> {
	tcx.param_env(def_id)
		.caller_bounds()
		.all_clauses(tcx)
		.iter()
		.filter_map(|p| p.as_trait_clause())
		.filter(|t| t.self_ty().skip_binder() == input)
		.filter_map(|t| match std_path(tcx, t.def_id()).as_slice() {
			[s, p, t] if s == "str" && p == "pattern" && t == "Pattern" => Some(ty::ClosureKind::FnMut),
			_ => tcx.fn_trait_kind_from_def_id(t.def_id()),
		})
		.min_by_key(|kind| match kind {
			ty::ClosureKind::Fn => 0,
			ty::ClosureKind::FnMut => 1,
			ty::ClosureKind::FnOnce => 2,
		})
}

/// The first type `call` takes by value that drops, but for a closure a function trait
/// bounds, which `called_closures` decides.
fn dropping_operand<'tcx>(b: &mut Body<'_, '_, 'tcx>, call: &Call<'tcx>) -> Option<Ty<'tcx>> {
	let tcx = b.t.tcx;
	let generic = tcx.fn_sig(call.def_id).instantiate_identity().skip_norm_wip();
	let generic = tcx.instantiate_bound_regions_with_erased(generic).inputs().to_vec();
	b.t.sig(call.def_id, call.args)
		.inputs()
		.iter()
		.copied()
		.enumerate()
		.find(|&(i, ty)| {
			let called = b.t.owned_fn(ty).is_some() && called_by(tcx, call.def_id, generic[i]).is_some();
			!called && b.t.drops(ty)
		})
		.map(|(_, ty)| ty)
}

/// Where std's `#[track_caller]` panic being mapped is, as `rs.panic` takes it after its
/// message, in a program with a panic hook: nothing in one without.
fn hook_at(b: &Body<'_, '_, '_>) -> Vec<Expr> {
	match b.t.hooked {
		true => vec![b.panic_at.clone().unwrap_or_else(Expr::undefined)],
		false => Vec::new(),
	}
}

/// `owner::name` for a standard library function.
pub fn key(tcx: TyCtxt<'_>, def_id: DefId) -> Option<(String, String)> {
	let name = tcx.item_name(def_id).to_string();
	if let Some(trait_id) = tcx.trait_of_assoc(def_id) {
		return Some((tcx.item_name(trait_id).to_string(), name));
	}
	if let Some(imp) = tcx.inherent_impl_of_assoc(def_id) {
		let self_ty = tcx.type_of(imp).instantiate_identity().skip_norm_wip();
		let owner = match self_ty.kind() {
			ty::Adt(adt, _) => tcx.item_name(adt.did()).to_string(),
			ty::Slice(_) => "slice".into(),
			ty::Array(..) => "array".into(),
			ty::Str => "str".into(),
			ty::Bool => "bool".into(),
			ty::Char => "char".into(),
			ty::Int(_) | ty::Uint(_) | ty::Float(_) => self_ty.to_string(),
			// `<*const T>::read` is `ptr::read`.
			ty::RawPtr(..) => "ptr".into(),
			ty::Dynamic(preds, ..) => format!("dyn {}", tcx.item_name(preds.principal_def_id()?)),
			_ => return None,
		};
		return Some((owner, name));
	}
	let parent = tcx.parent(def_id);
	let owner = tcx
		.opt_item_name(parent)
		.map_or_else(|| tcx.crate_name(def_id.krate).to_string(), |n| n.to_string());
	Some((owner, name))
}

pub fn call<'tcx>(b: &mut Body<'_, '_, 'tcx>, call: &Call<'tcx>, args: Vec<Expr>, out: &mut Vec<Stmt>) -> Option<Expr> {
	let tcx = b.t.tcx;
	let args = match super::stock::call(b, call, args, out) {
		Ok(e) => return e,
		Err(args) => args,
	};
	if !is_std(tcx, call.def_id) && !super::stock::as_std(tcx, call.def_id) {
		return None;
	}
	let (owner, name) = key(tcx, call.def_id)?;
	if !takes_none(&owner, &name)
		&& let Some(inner) = sentinel_option(b.t, call)
	{
		return Some(b.err(
			call.span,
			format!("`{owner}::{name}` of `Option<{inner}>`, whose `None` is `rs.None`, which is not supported yet"),
		));
	}
	let discarded = discards(&owner, &name);
	let mut args = args;
	let released = releases(&owner, &name);
	let closures = called_closures(b, call, discarded, released, &mut args);
	// What std lends of the `[u8; N]`s it holds, its closures are given and it returns as
	// slices of them.
	let lent = match super::lend::plan(b.t, call.def_id, call.args) {
		super::lend::Plan::Lend(lent) => Some(lent),
		_ => None,
	};
	for closure in lent.iter().flat_map(|lent| &lent.closures) {
		let holds = closure.held || b.t.drops(closure.ty);
		if let Some(value) = args.get_mut(closure.index) {
			let f = std::mem::replace(value, Expr::undefined());
			*value = super::lend::lend_closure(b, f, &closure.params, holds, call.span, out);
		}
	}
	let given = match super::array_slice::give(b, call, args) {
		Ok(given) => given,
		Err(e) => return Some(e),
	};
	let mut stmts = Vec::new();
	let mut e = mapping(b, call, owner.clone(), name.clone(), discarded, given.args, &mut stmts)?;
	if given.by.is_empty() {
		out.extend(stmts);
	} else {
		let ty = b.t.ty(b.t.sig(call.def_id, call.args).output(), call.span);
		e = super::array_slice::wrap(e, ty, stmts, given.by);
	}
	if let Some((generic, ty)) = lent.and_then(|lent| lent.output) {
		e = super::lend::lend(b, generic, ty, e, call.span);
	}
	if let Some(ty) = closures.leaked {
		return Some(b.err(
			call.span,
			format!(
				"passing `{ty}`, a closure that owns a value that drops, to `{owner}::{name}`, which is not supported yet"
			),
		));
	}
	// The last parameter drops first, as a function's do.
	for f in closures.spent.into_iter().rev() {
		e = Expr::rs("dropAfter", vec![f, Expr::Lambda(Vec::new(), Box::new(e))]);
	}
	Some(e)
}

/// The mapping of a call of std's `owner::name`, which may discard what `discarded` says.
fn mapping<'tcx>(
	b: &mut Body<'_, '_, 'tcx>,
	call: &Call<'tcx>,
	owner: String,
	name: String,
	discarded: Option<&'static [usize]>,
	args: Vec<Expr>,
	out: &mut Vec<Stmt>,
) -> Option<Expr> {
	let tcx = b.t.tcx;
	let span = call.span;
	let gargs = call.args;
	let args = match super::task::call(b, call, &owner, &name, args, out) {
		Ok(e) => return e,
		Err(args) => args,
	};
	let args = match super::dynamic::call(b, call, &owner, &name, args, out) {
		Ok(e) => return e,
		Err(args) => args,
	};
	let args = match super::pointer::call(b, call, &owner, &name, args, out) {
		Ok(e) => return e,
		Err(args) => args,
	};
	let args = match super::alloc::call(b, call, &owner, &name, args) {
		Ok(e) => return e,
		Err(args) => args,
	};
	let args = match super::time::call(b, call, &owner, &name, args) {
		Ok(e) => return e,
		Err(args) => args,
	};
	let mut args = match super::collections::call(b, call, &owner, &name, args, out) {
		Ok(e) => return e,
		Err(args) => args,
	};
	let dropping = match discarded {
		Some(_) => None,
		None => dropping_operand(b, call),
	};

	// Methods on integers.
	if let Some(int) = int_named(&owner) {
		return int_method(b, call, int, &name, args);
	}
	let float = match owner.as_str() {
		"f32" => Some(super::float::Float::F32),
		"f64" => Some(super::float::Float::F64),
		_ => None,
	};
	if let Some(float) = float {
		return super::float::method(float, &name, args);
	}

	let self_ty = || gargs.types().next();
	let byte_slice = |t: Ty<'tcx>| matches!(t.kind(), ty::Slice(e) if byte(*e));
	let range = matches!(inherent_std(tcx, call.def_id), Some(Std::Range | Std::RangeInclusive));

	let e = match (owner.as_str(), name.as_str()) {
		// An `AssertUnwindSafe` is what it holds.
		("From", "from") if unwind_safe(tcx, targ(gargs, 0)).is_some() => args.pop()?,
		("Default", "default") if let Some(held) = self_ty().and_then(|t| unwind_safe(tcx, t)) => {
			let default = super::collections::trait_fn(tcx, "Default", "default");
			return Some(b.call_def(default, tcx.mk_args(&[held.into()]), args, span, out));
		}

		// Conversions.
		("From", "from") => {
			// A `NonZero` converts as its integer.
			let (to, from) = (targ(gargs, 0), unwrapped(tcx, targ(gargs, 1)));
			let mut values = args;
			let value = values.pop()?;
			// `Some` or `Ready` of a value is the value.
			let some = matches!(to.kind(), ty::Adt(adt, args)
				if matches!(Std::of(tcx, adt.did()), Some(Std::Option | Std::Poll)) && args.type_at(0) == targ(gargs, 1));
			// `String::from(&String)` clones the `rs.String` object.
			if (to == from && !targ(gargs, 1).is_ref()) || some {
				return Some(value);
			}
			// Text into text: a `&mut str` is the `rs.Place` of its text, as a `String` is.
			let (raw, peeled) = (targ(gargs, 1), from);
			if textual(tcx, to) && textual(tcx, peeled) {
				let value = b.t.pointee_value(targ(gargs, 1), value);
				return Some(match () {
					// A `String` the `Cow<str>` owns, or one it borrows, or a clone of one.
					_ if owned_string(tcx, to) && cow_str(tcx, raw) => Expr::rs("strCowIntoOwned", vec![value]),
					_ if owned_string(tcx, to) && owned_string(tcx, peeled) => Expr::rs("stringClone", vec![value]),
					_ if owned_string(tcx, to) => string_from(text(tcx, peeled, value)),
					// A `Cow<str>` owns a `String` and borrows what a reference does.
					_ if cow_str(tcx, to) && !raw.is_ref() && owned_string(tcx, raw) => {
						cow_made(tcx.types.str_, rustc_abi::VariantIdx::from_u32(1), value)
					}
					_ if cow_str(tcx, to) => {
						cow_made(tcx.types.str_, rustc_abi::FIRST_VARIANT, text(tcx, peeled, value))
					}
					// A `Box<str>` is its text, a `String`'s once it shrinks to fit.
					_ => text(tcx, peeled, value),
				});
			}
			// A `Vec<u8>` of a `String`'s UTF-8, with its capacity, or of a `&str`'s.
			if b.t.std_of(to) == Some(Std::Vec) && textual(tcx, peeled) {
				let value = b.t.pointee_value(targ(gargs, 1), value);
				return Some(match !raw.is_ref() && owned_string(tcx, raw) {
					true => Expr::rs("stringIntoBytes", vec![value]),
					false => Expr::rs("strToBytes", vec![text(tcx, peeled, value)]),
				});
			}
			// A `Vec` is owned, and a reference to a slice, an array, or a `Vec` borrows its slice.
			if let Some(elem) = cow_slice(tcx, to) {
				let owned = b.t.std_of(targ(gargs, 1)) == Some(Std::Vec);
				let value = match owned {
					true => value,
					false => slice_of(value, targ(gargs, 1)),
				};
				return Some(cow_made(
					Ty::new_slice(tcx, elem),
					rustc_abi::VariantIdx::from_u32(owned.into()),
					value,
				));
			}
			if let Some(elem) = cow_slice(tcx, from)
				&& b.t.std_of(to) == Some(Std::Vec)
			{
				return Some(cow_cloned(b, "IntoOwned", value, elem, span));
			}
			// A `Vec` of a deque's items keeps its capacity.
			if let ty::Adt(_, vargs) = to.kind()
				&& b.t.std_of(to) == Some(Std::Vec)
				&& b.t.std_of(from) == Some(Std::VecDeque)
			{
				let v = Expr::rs("dequeToVec", vec![value]);
				return Some(match byte(vargs.type_at(0)) {
					true => Expr::rs("byteVecFromVec", vec![v]),
					false => v,
				});
			}
			// A `Vec` clones what a slice or an array borrows, as `to_vec` does.
			if let ty::Adt(_, vargs) = to.kind()
				&& b.t.std_of(to) == Some(Std::Vec)
				&& let ty::Ref(_, s, _) = targ(gargs, 1).kind()
				&& (s.is_slice() || s.is_array())
			{
				let slice = slice_of(value, targ(gargs, 1));
				return Some(slice_to_vec(b, slice, vargs.type_at(0), span));
			}
			// A boxed slice clones what it borrows, moves an array's or a `Vec`'s elements, owns
			// the `Vec` a `Cow` gives up, and takes a boxed text's bytes.
			if let Some(elem) = box_slice(tcx, to) {
				return match targ(gargs, 1).kind() {
					ty::Ref(_, s, _) if s.is_slice() => slice_boxed(b, value, elem, span),
					ty::Array(..) => Some(array_boxed(value, elem)),
					_ if b.t.std_of(from) == Some(Std::Vec) => Some(vec_boxed(value, elem)),
					_ if cow_slice(tcx, from).is_some() => {
						Some(vec_boxed(cow_cloned(b, "IntoOwned", value, elem, span), elem))
					}
					// A `Box<str>`'s bytes.
					_ if string(tcx, from) => Some(Expr::rs("stringAsBytes", vec![value])),
					_ => None,
				};
			}
			if let Some(elem) = box_slice(tcx, from)
				&& b.t.std_of(to) == Some(Std::Vec)
			{
				return Some(boxed_vec(value, elem));
			}
			// An `Arc` or an `Rc` of what a box holds, its trait object or a struct of one
			// included, which it drops with the last one. A box of a byte array holds a
			// `Uint8Array` of its own.
			if b.t.std_of(to) == Some(Std::Arc)
				&& let ty::Adt(adt, args) = from.kind()
				&& Std::of(tcx, adt.did()) == Some(Std::Box)
				&& (super::dynamic::unsized_object(tcx, args.type_at(0))
					|| args.type_at(0).is_sized(tcx, b.t.typing_env()))
				&& !super::byte_array(args.type_at(0))
			{
				return Some(Expr::New(Box::new(Expr::Rs("Arc".into())), vec![value]));
			}
			// An error, boxed as a `dyn Error`, which `?` converts it by. One of a string boxes
			// std's own private error type.
			let error = targ(gargs, 1);
			if let Some(object) = to.boxed_ty()
				&& super::dynamic::is_error_object(tcx, object)
				&& !textual(tcx, error.peel_refs())
			{
				return Some(b.object_of(value, Ty::new_box(tcx, error), to, span));
			}
			// A heap's `Vec` is its elements.
			if let ty::Adt(adt, args) = from.kind()
				&& Std::of(tcx, adt.did()) == Some(Std::BinaryHeap)
			{
				return Some(super::collections::heap_vec(value, byte(args.type_at(0))));
			}
			// Each `From` between integers, `char`, and floats, or from `bool`, is as `as`.
			let scalar = |t: Ty<'tcx>| Int::of(t).is_some() || t.is_char() || super::float::is(t);
			if !scalar(to) || !(scalar(from) || from.is_bool()) {
				return None;
			}
			let operand = Operand {
				expr: value,
				value: None,
			};
			num::cast(b, operand, from, to, span)
		}
		("Into", "into") => {
			let (from, to) = (targ(gargs, 0), targ(gargs, 1));
			let mut values = args;
			let value = values.pop()?;
			b.convert(value, from, to, span, out)
		}
		// A scalar value, but not a surrogate.
		("char", "from_u32") => Expr::rs("charFromU32", args),
		("TryFrom", "try_from") | ("TryInto", "try_into") => {
			let (to, from) = match owner.as_str() {
				"TryFrom" => (targ(gargs, 0), targ(gargs, 1)),
				_ => (targ(gargs, 1), targ(gargs, 0)),
			};
			// Where `From` holds, std's blanket impl converts by it and can't fail: its error
			// is `Infallible`, which is `!`.
			if let ty::Adt(_, result) = b.t.sig(call.def_id, call.args).output().kind()
				&& result.type_at(1).is_never()
			{
				let mut values = args;
				let value = b.convert(values.pop()?, from, to, span, out);
				return Some(Expr::rs("Ok", vec![value]));
			}
			// A boxed slice or a `Vec` of `N` elements is a boxed array of them, and else
			// itself again.
			if let Some(ty::Array(elem, n)) = to.boxed_ty().map(|t| t.kind())
				&& (box_slice(tcx, from) == Some(*elem) || b.t.std_of(from) == Some(Std::Vec))
			{
				let n = n.try_to_target_usize(tcx)?;
				let mut values = args;
				let from_value = match values.pop()? {
					v if v.is_path() => v,
					v => b.spill(v, "boxed", out),
				};
				let boxed = match b.t.std_of(from) {
					Some(Std::Vec) => vec_boxed(from_value.clone(), *elem),
					_ => from_value.clone(),
				};
				let length = match b.t.std_of(from) == Some(Std::Vec) && !byte(*elem) {
					true => from_value.clone().member("items").member("length"),
					false => from_value.clone().member("length"),
				};
				// The elements keep their buffer, as std keeps the allocation.
				let array = elements(boxed, Ty::new_slice(tcx, *elem));
				return Some(Expr::Cond(
					Box::new(length.binary("===", Expr::lit(n.to_string()))),
					Box::new(Expr::rs("Ok", vec![array])),
					Box::new(Expr::rs("Err", vec![from_value])),
				));
			}
			// A slice of `N` elements is the reference to the array of them it lends, where it holds them.
			if let (ty::Ref(_, array, _), ty::Ref(_, slice, _)) = (to.kind(), from.kind())
				&& let ty::Array(_, n) = array.kind()
				&& slice.is_slice()
			{
				let n = n.try_to_target_usize(tcx)?;
				let mut values = args;
				return Some(Expr::rs(
					"sliceTryAsArray",
					vec![values.pop()?, Expr::lit(n.to_string())],
				));
			}
			if to.is_char() && from == tcx.types.u32 {
				return Some(Expr::rs("charTryFrom", args));
			}
			if from.is_char() && (to == tcx.types.u8 || to == tcx.types.u16) {
				return Some(Expr::rs(format!("{to}TryFromChar"), args));
			}
			let (Some(int), Some(_)) = (Int::of(to), Int::of(from)) else {
				return None;
			};
			let values = args;
			int.method("tryFrom", values)
		}

		// A `Wrapping`'s or a `Saturating`'s operators, which wrap or saturate as its integer's
		// `wrapping_*` or `saturating_*` do.
		(_, _)
			if let Some(op) = operator(&owner)
				&& let Some((overflow, int)) = overflowing(tcx, targ(gargs, 0).peel_refs()) =>
		{
			let [l, r] = <[Expr; 2]>::try_from(args).ok()?;
			// A constant shift amount, which masks to a constant.
			let value = match &r {
				Expr::Lit(l) => l.parse().ok(),
				_ => None,
			};
			num::overflowing(
				b,
				overflow,
				op,
				Operand { expr: l, value: None },
				Operand { expr: r, value },
				int,
				span,
			)
		}
		("Neg", "neg") if let Some((overflow, int)) = overflowing(tcx, targ(gargs, 0).peel_refs()) => {
			let int = Int::of(int)?;
			int.method(&overflow.method("Sub"), vec![int.lit(0), args.pop()?])
		}
		("Not", "not") if let Some((_, int)) = overflowing(tcx, targ(gargs, 0).peel_refs()) => {
			num::unary(b, rustc_middle::mir::UnOp::Not, args.pop()?, int, span)
		}
		// Operators on primitives, called by path, and on references to them.
		(_, _)
			if let Some(op) = operator(&owner)
				&& primitive(targ(gargs, 0).peel_refs()) =>
		{
			let (lty, rty) = (targ(gargs, 0), targ(gargs, 1));
			let [l, r] = <[Expr; 2]>::try_from(args).ok()?;
			num::binary(
				b,
				op,
				Operand { expr: l, value: None },
				Operand { expr: r, value: None },
				lty,
				rty,
				span,
			)
		}
		("Neg", "neg") | ("Not", "not") if primitive(targ(gargs, 0).peel_refs()) => {
			let op = match name.as_str() {
				"neg" => rustc_middle::mir::UnOp::Neg,
				_ => rustc_middle::mir::UnOp::Not,
			};
			num::unary(b, op, args.pop()?, targ(gargs, 0), span)
		}
		// Every value of a type holding nothing is equal, and no `!` is ever compared.
		("PartialEq", "eq" | "ne")
			if nothing(tcx, unwrapped(tcx, targ(gargs, 0))) || unwrapped(tcx, targ(gargs, 0)).is_never() =>
		{
			Expr::lit(if name == "eq" { "true" } else { "false" })
		}
		("PartialEq", "eq" | "ne") | ("PartialOrd", "lt" | "le" | "gt" | "ge")
			if primitive(unwrapped(tcx, targ(gargs, 0))) =>
		{
			let js = match name.as_str() {
				"eq" => "===",
				"ne" => "!==",
				"lt" => "<",
				"le" => "<=",
				"gt" => ">",
				_ => ">=",
			};
			let values = args;
			let [l, r] = <[Expr; 2]>::try_from(values).ok()?;
			match Int::of(unwrapped(tcx, targ(gargs, 0))) {
				Some(int) => int.compare(js, l, r),
				None => l.binary(js, r),
			}
		}
		("Ord", "max" | "min") if let Some(int) = Int::of(unwrapped(tcx, targ(gargs, 0))) => {
			let values = args;
			match int.big() {
				true => int.method(name.as_str(), values),
				false => Expr::rs(if name == "max" { "max" } else { "min" }, values),
			}
		}
		("Ord", "clamp") if let Some(int) = Int::of(targ(gargs, 0)) => match int.big() {
			true => int.method("clamp", args),
			false => Expr::rs("clamp", args),
		},
		("Clone", "clone") => {
			let ty = targ(gargs, 0);
			let values = args;
			let value = values.into_iter().next()?;
			match ty.kind() {
				ty::Adt(adt, args) if Std::of(tcx, adt.did()) == Some(Std::Vec) && byte(args.type_at(0)) => {
					Expr::rs("byteVecFrom", vec![Expr::rs("byteVecView", vec![value])])
				}
				ty::Adt(..) if string(tcx, ty) => value,
				ty::Adt(..) if owned_string(tcx, ty) => Expr::rs("stringClone", vec![value]),
				ty::Adt(..) if cow_str(tcx, ty) => Expr::rs("strCowClone", vec![value]),
				ty::Adt(adt, args) if Std::of(tcx, adt.did()) == Some(Std::Vec) => {
					return Some(vec_clone(b, value.member("items"), args.type_at(0), span));
				}
				_ if let Some(elem) = cow_slice(tcx, ty) => cow_cloned(b, "Clone", value, elem, span),
				// Each element cloned in order, as `[T; N]`'s `Clone` does, through the
				// reference, which is a slice of it.
				ty::Array(elem, _) if !clone_copies(b.t, b.t.typing_env_pub(), ty) => {
					items_clone(b, value, *elem, span)
				}
				ty::Tuple(tys) if !clone_copies(b.t, b.t.typing_env_pub(), ty) => {
					let clones = tys.iter().map(|t| super::collections::clone_fn(b, t, span)).collect();
					Expr::rs("tupleClone", vec![value, Expr::Array(clones)])
				}
				_ if let Some(elem) = box_slice(tcx, ty) => return slice_boxed(b, value, elem, span),
				// A box, a `ManuallyDrop`, or a `Reverse` is what it holds, cloned.
				ty::Adt(adt, args)
					if matches!(
						Std::of(tcx, adt.did()),
						Some(Std::Box | Std::ManuallyDrop | Std::Reverse)
					) =>
				{
					let inner = args.type_at(0);
					if !inner.is_sized(tcx, b.t.typing_env_pub()) {
						return None;
					}
					match super::collections::clone_fn(b, inner, span) {
						Expr::Lit(l) if l == "undefined" => value,
						clone => clone.call(vec![value]),
					}
				}
				ty::Adt(adt, _) if Std::of(tcx, adt.did()) == Some(Std::FromUtf8Error) => {
					Expr::rs("fromUtf8ErrorClone", vec![value])
				}
				// Never changed once made.
				ty::Adt(adt, _)
					if matches!(
						Std::of(tcx, adt.did()),
						Some(Std::DecodeUtf16Error | Std::Utf8Chunk | Std::TryReserveError)
					) =>
				{
					value
				}
				// Another that holds nothing.
				ty::Adt(adt, _) if empty_iter(tcx, adt.did()) => b.zero_sized_std(ty, Vec::new(), span).ok()?,
				// Through the reference, a slice of the elements.
				ty::Array(..) => b.t.array_copy(value, ty, span),
				_ if b.t.std_iter(ty) => return super::iter::clone(b, ty, value, span),
				ty::Closure(def_id, _) | ty::CoroutineClosure(def_id, _)
					if !clone_shares(b.t, b.t.typing_env_pub(), ty) =>
				{
					b.t.clone_closure(*def_id);
					Expr::rs("closureClone", vec![value])
				}
				_ if clone_shares(b.t, b.t.typing_env_pub(), ty) => value,
				_ if clone_copies(b.t, b.t.typing_env_pub(), ty) => b.t.copied(value, ty, span),
				_ => return None,
			}
		}
		("Default", "default") => default_value(b, targ(gargs, 0), span, out)?,

		// Options, whose other combinators lower from core's MIR.
		("Option", "unwrap" | "expect") => rt2(args, "option", &name),
		("Option", "is_some") => args.pop()?.binary("!==", b.t.none(targ(gargs, 0))),
		("Option", "is_none") => args.pop()?.binary("===", b.t.none(targ(gargs, 0))),
		("Option", "as_ref" | "as_mut") => args.pop()?,
		// The payload, if any, as a slice of one, as `slice::from_ref` makes it.
		("Option", "as_slice") if byte(targ(gargs, 0)) => Expr::rs("byteOptionAsSlice", args),
		// Of an array, the arrays of the one it holds, if any.
		("Option", "as_slice") if targ(gargs, 0).is_array() => {
			args.push(super::array_fn(targ(gargs, 0), "From"));
			Expr::rs("optionAsArraySlice", args)
		}
		("Option", "as_slice") => {
			args.extend(b.t.none_arg(targ(gargs, 0)));
			Expr::rs("optionAsSlice", args)
		}
		// A box is what it holds.
		("Option", "as_deref_mut") if targ(gargs, 0).is_box() => args.pop()?,
		// `Option<&T>` or `Option<&mut T>` of the `T` it points at, through `T`'s `Clone`
		// where that does more than copy, as std's `cloned` does.
		("Option", "copied" | "cloned") => {
			let sig = b.t.sig(call.def_id, call.args);
			let (option, inner) = (sig.output(), targ(gargs, 0));
			let ty::Adt(_, of) = sig.inputs()[0].kind() else {
				return None;
			};
			let reference = of.type_at(0);
			let value = args.pop()?;
			let copies = name == "copied" || clone_copies(b.t, b.t.typing_env_pub(), inner);
			let place = b.t.place_pointee(reference).is_some();
			if copies && !place {
				return Some(b.t.copied(value, option, span));
			}
			let value = match value.is_path() {
				true => value,
				false => b.spill(value, "option", out),
			};
			let some = b.t.pointee_value(reference, value.clone());
			let some = match copies {
				true => b.t.copied(some, inner, span),
				false => {
					let clone = super::collections::trait_fn(tcx, "Clone", "clone");
					let mut body = Vec::new();
					let some = b.call_def(clone, tcx.mk_args(&[inner.into()]), vec![some], span, &mut body);
					match body.is_empty() {
						true => some,
						// Only once it is `Some`.
						false => {
							body.push(StmtKind::Return(Some(some.typed())).at(None));
							Expr::Arrow(Vec::new(), b.t.ret_ty(inner, span), body).call(Vec::new())
						}
					}
				}
			};
			let none = value.binary("===", b.t.none(reference));
			Expr::Cond(Box::new(none), Box::new(b.t.none(inner)), Box::new(some))
		}

		// Results, whose other combinators lower from core's MIR.
		("Result", "is_ok") => args.pop()?.member("ok"),
		("Result", "is_err") => args.pop()?.member("ok").not(),
		// Their panics show the other side with its `Debug`.
		("Result", "unwrap" | "expect" | "unwrap_err") => {
			let (method, shown) = match name.as_str() {
				"unwrap" => ("unwrap", targ(gargs, 1)),
				"expect" => ("expect", targ(gargs, 1)),
				_ => ("unwrapErr", targ(gargs, 0)),
			};
			let mut values = args;
			values.push(b.fmt_fn(fmt::Trait::Debug, shown, span));
			Expr::rs_of("result", method).call(values)
		}

		("ControlFlow", "is_continue" | "is_break") => Flow::test(b.t, gargs, args.pop()?, name == "is_continue"),

		// Ranges, which are their own iterators.
		// An integer range wraps within its type's bounds where std's `nth` does.
		("RangeInclusive", "new") if range => {
			let mut values = args;
			values.extend(self_ty().and_then(|t| range_step(tcx, t, true)));
			Expr::New(Box::new(Expr::Rs("RangeInclusive".into())), values)
		}
		("Range" | "RangeInclusive", "start" | "end") | ("RangeInclusive", "into_inner") if range => {
			let of = range_of(inherent_std(tcx, call.def_id)?)?;
			Expr::rs_of(of, &super::camel(&name)).call(args)
		}
		// Of bounds JS compares as Rust does, by `<` and `<=`, against an item of their type.
		("Range" | "RangeInclusive", "is_empty") | (_, "contains")
			if let Some(of) = inherent_std(tcx, call.def_id).and_then(range_of)
				&& let Some(bound) = compared(tcx, targ(gargs, 0))
				&& (name == "is_empty" || compared(tcx, targ(gargs, 1)) == Some(bound)) =>
		{
			Expr::rs_of(of, &super::camel(&name)).call(args)
		}
		("RangeBounds", "contains")
			if compared(tcx, targ(gargs, 1)).is_some_and(|bound| compared(tcx, targ(gargs, 2)) == Some(bound)) =>
		{
			let range = targ(gargs, 0).peel_refs();
			match b.t.std_of(range) {
				Some(Std::RangeFull) => Expr::lit("true"),
				Some(std) => Expr::rs_of(range_of(std)?, "contains").call(args),
				None if super::collections::plain_bounds(b.t, range) => Expr::rs("boundsContains", args),
				None => return None,
			}
		}

		// An `IoSlice` is the bytes it lends, which `advance` moves its place past the front of.
		("IoSlice", "new") => args.pop()?,
		("IoSlice", "advance") => Expr::rs("ioSliceAdvance", args),
		("IoSlice", "advance_slices") => Expr::rs("ioSliceAdvanceSlices", args),
		// A `NonZero` is its integer.
		("NonZero", "get") => args.pop()?,
		("NonZero", "new") => Expr::rs("nonZero", args),
		("NonZero", "new_unchecked") => {
			let check = Expr::Str("NonZero::new_unchecked requires the argument to be non-zero".into());
			Expr::rs("precondition", vec![Expr::rs("nonZero", args), check])
		}
		// Its integer's, of each width's own impl, which has no type argument to read the
		// integer from, but for the bits it always has. An `unchecked_` one aborts in std's
		// code, which its call's site says.
		("NonZero", _) => {
			let int = Int::of(unwrapped(tcx, impl_self(tcx, call.def_id)?))?;
			match name.as_str() {
				"highest_one" => {
					let zeros = int.method("leadingZeros", args);
					Expr::lit((int.bits - 1).to_string()).binary("-", zeros)
				}
				"lowest_one" => int.method("trailingZeros", args),
				_ => return int_method(b, call, int, &name, args),
			}
		}
		// A `Wrapping`'s and a `Saturating`'s are its integer's, of each width's own impl, but
		// that each overflows as its operators do.
		(_, _) if let Some((overflow, int)) = impl_self(tcx, call.def_id).and_then(|ty| overflowing(tcx, ty)) => {
			let name = match name.as_str() {
				"pow" | "abs" | "next_power_of_two" => format!("{}_{name}", overflow.prefix()),
				_ => name,
			};
			return int_method(b, call, Int::of(int)?, &name, args);
		}
		// `Bound<&T>::cloned::<T>`: what it holds, cloned.
		("Bound", "cloned") => {
			let value = args.pop()?;
			let bound = b.t.sig(call.def_id, gargs).output();
			match super::collections::clone_fn(b, targ(gargs, 0), span) {
				Expr::Lit(l) if l == "undefined" => b.t.copied(value, bound, span),
				clone => Expr::rs("boundMap", vec![value, clone]),
			}
		}
		("Bound", "as_ref") => args.pop()?,
		("RangeBounds", "start_bound" | "end_bound") => {
			let start = name == "start_bound";
			let range = targ(gargs, 0).peel_refs();
			let ts = b.t.ty(b.t.sig(call.def_id, gargs).output(), span);
			let r = args.pop()?;
			let (variant, field) = match (range.kind(), b.t.std_of(range)) {
				// `(Bound<T>, Bound<T>)`, whose bounds are its own.
				(ty::Tuple(_), _) => return Some(r.index(Expr::lit(if start { "0" } else { "1" }))),
				(_, Some(Std::Range)) if start => ("Included", Some("start")),
				(_, Some(Std::Range)) => ("Excluded", Some("end")),
				(_, Some(Std::RangeFrom)) if start => ("Included", Some("start")),
				(_, Some(Std::RangeTo)) if !start => ("Excluded", Some("end")),
				(_, Some(Std::RangeToInclusive)) if !start => ("Included", Some("end")),
				(_, Some(Std::RangeFrom | Std::RangeTo | Std::RangeToInclusive | Std::RangeFull)) => {
					("Unbounded", None)
				}
				// Exhausted, it ends before its end.
				(_, Some(Std::RangeInclusive)) => {
					let f = if start {
						"rangeInclusiveStartBound"
					} else {
						"rangeInclusiveEndBound"
					};
					return Some(Expr::As(Box::new(Expr::rs(f, vec![r])), ts));
				}
				_ => return None,
			};
			tagged(variant, field.map(|f| r.member(f)), ts)
		}

		// Slices, whose byte functions take an `rs.ByteSlice`.
		("slice", "len") => super::mir::slice_member(args.pop()?, "length"),
		("slice", "is_empty") => super::mir::slice_member(args.pop()?, "length").binary("===", Expr::lit("0")),
		("slice", "split_first" | "split_at" | "split_at_checked" | "first" | "get")
			if name != "get" || Int::of(targ(gargs, 1)).is_some() =>
		{
			rt2(args, slice_of_elem(targ(gargs, 0)), &super::camel(&name))
		}
		// An element JS copies is lent as its place, and an object as itself.
		("slice", "first_mut" | "last_mut" | "get_mut") if name != "get_mut" || Int::of(targ(gargs, 1)).is_some() => {
			let elem = targ(gargs, 0);
			let values = args;
			let name = match b.t.needs_place(elem) {
				true => name.as_str(),
				false => name.strip_suffix("_mut")?,
			};
			let of = match elem.is_array() {
				true => "arraySlice",
				false => "slice",
			};
			rt2(values, of, &super::camel(name))
		}
		("slice", "windows" | "chunks" | "chunks_exact" | "rchunks" | "rchunks_exact") => {
			rt2(args, slice_of_elem(targ(gargs, 0)), &super::camel(&name))
		}
		// An array slice of `N`-element arrays lent where the slice holds them, shared or mutable alike, of any slice.
		("slice", "as_chunks" | "as_rchunks" | "as_chunks_mut" | "as_rchunks_mut") => {
			let n = gargs.consts().next()?.try_to_target_usize(tcx)?;
			let mut values = args;
			values.push(Expr::lit(n.to_string()));
			rt2(
				values,
				"slice",
				&super::camel(name.strip_suffix("_mut").unwrap_or(&name)),
			)
		}
		// A part is a slice of the same storage, shared or mutable alike.
		("slice", "chunks_mut" | "chunks_exact_mut" | "rchunks_mut" | "rchunks_exact_mut") => rt2(
			args,
			slice_of_elem(targ(gargs, 0)),
			&super::camel(name.strip_suffix("_mut")?),
		),
		(
			"slice",
			"split"
			| "split_mut"
			| "rsplit"
			| "rsplit_mut"
			| "split_inclusive"
			| "split_inclusive_mut"
			| "splitn"
			| "splitn_mut"
			| "rsplitn"
			| "rsplitn_mut",
		) => rt2(
			args,
			slice_of_elem(targ(gargs, 0)),
			&super::camel(name.strip_suffix("_mut").unwrap_or(&name)),
		),
		("Split", "as_slice") => Expr::rs("splitAsSlice", args),
		("slice", "split_at_mut" | "split_at_mut_checked") => rt2(
			args,
			slice_of_elem(targ(gargs, 0)),
			&super::camel(&name.replace("_mut", "")),
		),
		// The element is lent as `first_mut` lends it.
		("slice", "split_first_mut" | "split_last_mut") => match byte(targ(gargs, 0)) {
			true => rt2(args, "byteSlice", &super::camel(&name)),
			false if targ(gargs, 0).is_array() => rt2(args, "arraySlice", &super::camel(name.strip_suffix("_mut")?)),
			false => {
				let mut values = args;
				values.push(Expr::lit(b.t.needs_place(targ(gargs, 0)).to_string()));
				rt2(values, "slice", &super::camel(&name))
			}
		},
		("ChunksExact" | "RChunksExact", "remainder") | ("ChunksExactMut" | "RChunksExactMut", "into_remainder") => {
			Expr::rs("partsRemainder", args)
		}
		// The first `N` elements are `get(..N)`, and split off, `split_at_checked(N)`.
		(
			"slice",
			"first_chunk"
			| "first_chunk_mut"
			| "last_chunk"
			| "last_chunk_mut"
			| "split_first_chunk"
			| "split_first_chunk_mut"
			| "split_last_chunk"
			| "split_last_chunk_mut",
		) => {
			let n = gargs.consts().next()?.try_to_target_usize(tcx)?;
			let mut values = args;
			let name = name.strip_suffix("_mut").unwrap_or(&name);
			if name.starts_with("first") {
				values.push(Expr::lit("0"));
			}
			values.push(Expr::lit(n.to_string()));
			let name = match name {
				"first_chunk" => "get_range",
				"split_first_chunk" => "split_at_checked",
				name => name,
			};
			rt2(values, slice_of_elem(targ(gargs, 0)), &super::camel(name))
		}
		// A slice or an array of the one value.
		("raw" | "array", "from_ref")
			if owner == "array" || std_path(tcx, call.def_id) == ["slice", "raw", "from_ref"] =>
		{
			let value = args.pop()?;
			match byte(targ(gargs, 0)) {
				true => Expr::rs(
					"byteSliceFrom",
					vec![Expr::ident("Uint8Array").member("of").call(vec![value])],
				),
				false => super::slice_items_from(Expr::Array(vec![value]), targ(gargs, 0)),
			}
		}
		// A slice or a `str` as itself.
		("AsRef", "as_ref") | ("AsMut", "as_mut") if targ(gargs, 0) == targ(gargs, 1) => args.pop()?,
		// The slice of all of an array, as `as_slice` has it.
		("AsRef", "as_ref") | ("AsMut", "as_mut") | ("Borrow", "borrow") | ("BorrowMut", "borrow_mut")
			if targ(gargs, 0).is_array() && targ(gargs, 1).is_slice() =>
		{
			args.pop()?
		}
		// A `Vec` as its slice, as `as_slice` has it, or as itself.
		("AsRef", "as_ref") | ("AsMut", "as_mut") | ("Borrow", "borrow") | ("BorrowMut", "borrow_mut")
			if let ty::Adt(_, vargs) = targ(gargs, 0).kind()
				&& b.t.std_of(targ(gargs, 0)) == Some(Std::Vec) =>
		{
			let v = args.pop()?;
			match targ(gargs, 1).is_slice() {
				true => vec_view(v, vargs.type_at(0)),
				false => v,
			}
		}
		// `Ord::cmp` of each element it compares with `x`, in std's order.
		("slice", "binary_search") => {
			let elem = targ(gargs, 0);
			let [s, x] = <[Expr; 2]>::try_from(args).ok()?;
			let cmp = super::collections::cmp_fn(b, elem, false, span);
			Expr::rs("sliceBinarySearch", vec![s, x, cmp])
		}
		// std's same search by the closure's `Ordering`, by `Ord::cmp` of the key it makes
		// with `b`, or by whether `pred` holds.
		("slice", "binary_search_by" | "binary_search_by_key" | "partition_point") => {
			let mut values = args;
			match name.as_str() {
				"binary_search_by" => Expr::rs("sliceBinarySearchBy", values),
				"partition_point" => Expr::rs("slicePartitionPoint", values),
				_ => {
					let key = targ(gargs, 1);
					values.push(super::collections::cmp_fn(b, key, false, span));
					let drop = b.t.dropper(key, span);
					Expr::rs("sliceBinarySearchByKey", super::iter::tail(values, vec![drop]))
				}
			}
		}
		// Each slot but the last a clone, as `clone_from` makes it, and the last the value;
		// a `TrivialClone`'s copies, as std specializes `fill` to.
		("slice", "fill") => {
			let elem = targ(gargs, 0);
			if byte(elem) {
				return Some(rt2(args, "byteSlice", "fill"));
			}
			let fns = clone_from_fns(b, elem, span);
			let mut values = args;
			// What clones one, which the runtime takes as `undefined` where it is the value.
			values.extend(fns.is_empty().then(Expr::undefined));
			// Its own droppers, before what clones over a slot.
			return Some(Expr::rs("sliceFill", [values, fns].concat()));
		}
		("slice", "fill_with" | "rotate_left" | "rotate_right") => match byte(targ(gargs, 0)) {
			true => rt2(args, "byteSlice", &super::camel(&name)),
			false => rt2(args, "slice", &super::camel(&name)),
		},
		// A `TrivialClone`'s `clone_from_slice` is std's `copy_from_slice`, panics included.
		("slice", "copy_from_slice" | "clone_from_slice")
			if name == "copy_from_slice" || super::iter::implements(b.t, targ(gargs, 0), "TrivialClone") =>
		{
			let elem = targ(gargs, 0);
			if byte(elem) {
				return Some(rt2(args, "byteSlice", "copyFromSlice"));
			}
			let mut values = args;
			values.extend(b.t.copy_fn(elem, span));
			Expr::rs("sliceCopyFromSlice", values)
		}
		// The slice it wrote, which JS holds as what a `MaybeUninit` holds.
		("slice", "write_copy_of_slice") => {
			let [dst, src] = <[Expr; 2]>::try_from(args).ok()?;
			let dst = b.spill(dst, "dst", out);
			let elem = targ(gargs, 0);
			let copy = match byte(elem) {
				true => rt2(vec![dst.clone(), src], "byteSlice", "copyFromSlice"),
				false => {
					let mut values = vec![dst.clone(), src];
					values.extend(b.t.copy_fn(elem, span));
					Expr::rs("sliceCopyFromSlice", values)
				}
			};
			out.push(StmtKind::Expr(copy).at(None));
			dst
		}
		// Each slot a clone, over nothing it drops, as a `MaybeUninit` holds nothing that does.
		("slice", "write_clone_of_slice") => {
			let elem = targ(gargs, 0);
			if byte(elem) {
				return Some(rt2(args, "byteSlice", "writeCloneOfSlice"));
			}
			let mut values = args;
			values.push(super::collections::clone_fn(b, elem, span));
			Expr::rs("sliceWriteCloneOfSlice", values)
		}
		// Of arrays, the arrays of the same arrays, which a slice of their `MaybeUninit`s holds.
		("slice", "assume_init_ref" | "assume_init_mut") if targ(gargs, 0).is_array() => Expr::rs(
			"arraySliceOfSlice",
			vec![args.pop()?, super::array_fn(targ(gargs, 0), "From")],
		),
		("slice", "assume_init_ref" | "assume_init_mut") => args.pop()?,
		("slice", "clone_from_slice") => {
			let mut values = args;
			let fns = clone_from_fns(b, targ(gargs, 0), span);
			// What clones one, which the runtime takes as `undefined` where it is the value.
			values.extend(fns.is_empty().then(Expr::undefined));
			return Some(Expr::rs("sliceCloneFromSlice", [values, fns].concat()));
		}
		("PartialEq", "eq") if byte_slice(targ(gargs, 0)) && byte_slice(targ(gargs, 1)) => {
			Expr::rs("byteSliceEq", args)
		}
		("slice", "to_vec") => slice_to_vec(b, args.pop()?, targ(gargs, 0), span),
		// A swap of arrays copies each out, as a write of it back reads it.
		("slice", "swap") if targ(gargs, 0).is_array() => {
			let mut values = args;
			values.push(b.t.elem_copy(targ(gargs, 0), span).unwrap_or_else(Expr::undefined));
			Expr::rs("arraySliceSwap", values)
		}
		("slice", "last" | "swap") => rt2(args, slice_of_elem(targ(gargs, 0)), &super::camel(&name)),
		("slice", "reverse") => match byte(targ(gargs, 0)) {
			true => Expr::rs("byteSliceView", args).method("reverse", Vec::new()),
			false => Expr::rs("sliceReverse", args),
		},
		("slice", "contains") => {
			let elem = targ(gargs, 0);
			let [s, x] = <[Expr; 2]>::try_from(args).ok()?;
			match byte(elem) {
				true => Expr::rs("byteSliceView", vec![s]).method("includes", vec![x]),
				false => {
					let eq = super::collections::eq_fn(b, elem, span);
					Expr::rs("sliceContains", vec![s, x, eq])
				}
			}
		}
		// Sorted as std's sorts do, by `PartialOrd::lt`, or by a comparator, or by `lt` of the
		// keys a function makes.
		(
			"slice",
			"sort" | "sort_unstable" | "sort_by" | "sort_unstable_by" | "sort_by_key" | "sort_unstable_by_key",
		) => {
			let elem = targ(gargs, 0);
			let mut values = args.into_iter();
			let s = values.next()?;
			let less = match name.as_str() {
				// A typed array sorts its numbers by value, and equal bytes are alike.
				"sort" | "sort_unstable" if byte(elem) => {
					return Some(Expr::rs("byteSliceView", vec![s]).method("sort", Vec::new()));
				}
				"sort" | "sort_unstable" => super::collections::test_fn(b, elem, "lt", span),
				"sort_by" | "sort_unstable_by" => Expr::rs("cmpLt", vec![values.next()?]),
				_ => {
					let key = targ(gargs, 1);
					if b.t.drops(key) {
						return Some(b.err(
							span,
							format!("`slice::{name}` by `{key}`, which drops, which is not supported yet"),
						));
					}
					let lt = super::collections::test_fn(b, key, "lt", span);
					Expr::rs("keyLt", vec![values.next()?, lt])
				}
			};
			let mut values = vec![s, less];
			values.extend(sort_of(b, elem, span));
			match name.contains("unstable") {
				true => {
					values.push(Expr::lit(b.t.is_copy(elem).to_string()));
					Expr::rs("sliceSortUnstable", values)
				}
				false => Expr::rs("sliceSort", values),
			}
		}
		// By `sort_unstable` of each key `f` makes paired with its index, a `usize`, as a
		// `u32` is no smaller on wasm32, `f` dropped once it made them, and then the keys
		// dropped.
		("slice", "sort_by_cached_key") => {
			let key = targ(gargs, 1);
			let mut values = args.into_iter();
			let s = values.next()?;
			let f = values.next()?;
			let pair = Ty::new_tup(tcx, &[key, tcx.types.usize]);
			let lt = super::collections::test_fn(b, pair, "lt", span);
			let [size, freeze] = sort_of(b, pair, span);
			let copy = Expr::lit(b.t.is_copy(key).to_string());
			let owns = Expr::lit(b.t.drops(targ(gargs, 2)).to_string());
			let drop = b.t.dropper(key, span);
			Expr::rs(
				"sliceSortByCachedKey",
				super::iter::tail(vec![s, f, lt, size, freeze, copy], vec![owns, drop]),
			)
		}
		// Whether each element and the next are in order, as `sort` would leave them: by
		// `PartialOrd::le` in std's chunks, or as the slice's iterator's `is_sorted_by` and
		// `is_sorted_by_key` test them, which std's are.
		("slice", "is_sorted") => {
			let elem = targ(gargs, 0);
			let mut values = args;
			values.push(super::collections::test_fn(b, elem, "le", span));
			Expr::rs("sliceIsSorted", values)
		}
		("slice", "is_sorted_by" | "is_sorted_by_key") => {
			let elem = targ(gargs, 0);
			let mut values = args.into_iter();
			let slice = elements(values.next()?, Ty::new_slice(tcx, elem));
			let items = super::iter::by_ref(Expr::rs("iterSlice", vec![slice]), elem);
			let f = values.next()?;
			let (items, compare, drop) = match name.as_str() {
				"is_sorted_by" => (items, f, Expr::undefined()),
				_ => {
					let key = targ(gargs, 2);
					let le = super::collections::test_fn(b, key, "le", span);
					(Expr::rs("iterMap", vec![items, f]), le, b.t.dropper(key, span))
				}
			};
			Expr::rs(
				"iterIsSortedBy",
				super::iter::tail(vec![items, compare], vec![Expr::undefined(), drop]),
			)
		}
		("slice", "into_vec") => boxed_vec(args.pop()?, targ(gargs, 0)),
		("Vec", "into_boxed_slice") => vec_boxed(args.pop()?, targ(gargs, 0)),
		// `vec![a, b]`'s array once `mir::vec` took the box away. A `Copy` aggregate's operand
		// may be shared with where it came from, which the vector must not change.
		("boxed", "box_assume_init_into_vec_unsafe") => {
			let elem = targ(gargs, 0);
			let value = args.pop()?;
			if byte(elem) {
				return Some(Expr::rs("byteVecFrom", vec![value]));
			}
			let items = match (b.t.copy_fn(elem, span), value) {
				(None, value) => value,
				(Some(_), Expr::Array(items)) => {
					Expr::Array(items.into_iter().map(|item| owned(b, item, elem, span)).collect())
				}
				(Some(copy), value) => value.member("map").call(vec![copy]),
			};
			Expr::rs("vecFrom", vec![items])
		}
		("Box", "new") => args.pop()?,
		// `len` values not yet written, which panics past what a `Vec` of them may hold.
		("Box", "new_uninit_slice") => {
			let elem = targ(gargs, 0);
			let len = args.pop()?;
			if byte(elem) {
				return Some(Expr::rs("byteSliceNewUninit", vec![len]));
			}
			let uninit = Ty::new_maybe_uninit(tcx, elem);
			let value = b.t.uninit(uninit, span);
			let each = Expr::Arrow(
				Vec::new(),
				b.t.ty(uninit, span),
				vec![StmtKind::Return(Some(value)).at(None)],
			);
			let size = super::iter::size_of(b, elem, span);
			Expr::rs("sliceNewUninit", vec![len, size, each])
		}
		// The slice, which JS holds as what its `MaybeUninit`s hold.
		("Box", "assume_init")
			if tcx
				.type_of(tcx.parent(call.def_id))
				.instantiate_identity()
				.skip_norm_wip()
				.boxed_ty()
				.is_some_and(|t| t.is_slice()) =>
		{
			match targ(gargs, 0).is_array() {
				true => Expr::rs(
					"arraySliceOfSlice",
					vec![args.pop()?, super::array_fn(targ(gargs, 0), "From")],
				),
				false => args.pop()?,
			}
		}
		// One value not yet written, as `MaybeUninit::uninit` makes it.
		("Box", "new_uninit") => b.t.uninit(Ty::new_maybe_uninit(tcx, targ(gargs, 0)), span),
		// What the box holds, which JS holds as what its `MaybeUninit` holds.
		("Box", "assume_init") => args.pop()?,
		// `vec![x; n]`: `n - 1` clones of `x`, then `x` itself, which the caller gives as its
		// own, as it does whatever std keeps, or `x` dropped where `n` is 0.
		("vec", "from_elem_global") => {
			let elem = targ(gargs, 0);
			let mut values = args;
			match byte(elem) {
				true => Expr::rs("byteVecOf", values),
				false => {
					values.push(super::collections::clone_fn(b, elem, span));
					Expr::rs("vecFromElem", values)
				}
			}
		}

		// Vectors.
		("Vec", "new") => vec_new(byte(targ(gargs, 0))),
		// What reads or sets a capacity is given the size of the elements, by which std's
		// grows it.
		("Vec", "with_capacity") if byte(targ(gargs, 0)) => Expr::rs("byteVecWithCapacity", args),
		("Vec", "capacity") if byte(targ(gargs, 0)) => args.pop()?.member("data").member("length"),
		("Vec", "reserve" | "reserve_exact" | "try_reserve" | "try_reserve_exact" | "shrink_to")
			if byte(targ(gargs, 0)) =>
		{
			rt2(args, "byteVec", &super::camel(&name))
		}
		("Vec", "shrink_to_fit") if byte(targ(gargs, 0)) => {
			Expr::rs("byteVecShrinkTo", vec![args.pop()?, Expr::lit("0")])
		}
		(
			"Vec",
			"with_capacity" | "capacity" | "reserve" | "reserve_exact" | "try_reserve" | "try_reserve_exact"
			| "shrink_to" | "shrink_to_fit",
		) => {
			let mut values = args;
			if name == "shrink_to_fit" {
				values.push(Expr::lit("0"));
			}
			values.push(super::iter::size_of(b, targ(gargs, 0), span));
			let f = match name.as_str() {
				"shrink_to_fit" => "shrinkTo".to_string(),
				_ => super::camel(&name),
			};
			rt2(values, "vec", &f)
		}
		("Vec", "push") => {
			let mut values = args;
			match byte(targ(gargs, 0)) {
				true => Expr::rs("byteVecPush", values),
				false => {
					let v = values.remove(0);
					v.member("items").method("push", values)
				}
			}
		}
		("Vec", "pop") if byte(targ(gargs, 0)) => Expr::rs("byteVecPop", args),
		("Vec", "pop") => Expr::rs("vecPop", args),
		// The last element is lent to the test as its place where JS copies it.
		("Vec", "pop_if") if byte(targ(gargs, 0)) => Expr::rs("byteVecPopIf", args),
		("Vec", "pop_if") if b.t.needs_place(targ(gargs, 0)) => Expr::rs("vecPopIfMut", args),
		("Vec", "pop_if") => Expr::rs("vecPopIf", args),
		("Vec", "len") => vec_items(args.pop()?, targ(gargs, 0)).member("length"),
		("Vec", "is_empty") => vec_items(args.pop()?, targ(gargs, 0))
			.member("length")
			.binary("===", Expr::lit("0")),
		("Vec", "truncate") if byte(targ(gargs, 0)) => Expr::rs("byteVecTruncate", args),
		("Vec", "clear") if byte(targ(gargs, 0)) => Expr::rs("byteVecClear", args),
		("Vec", "extend_from_slice") if byte(targ(gargs, 0)) => Expr::rs("byteVecExtendFromSlice", args),
		("Vec", "extend_from_slice") => {
			let mut values = args;
			values.push(super::iter::size_of(b, targ(gargs, 0), span));
			let clone = super::collections::clone_fn(b, targ(gargs, 0), span);
			if !matches!(&clone, Expr::Lit(l) if l == "undefined") {
				values.push(clone);
			}
			Expr::rs("vecExtendFromSlice", values)
		}
		("Vec", "insert" | "remove" | "swap_remove" | "append" | "resize" | "resize_with" | "split_off")
			if byte(targ(gargs, 0)) =>
		{
			rt2(args, "byteVec", &super::camel(&name))
		}
		// Each reserves as std's does, by the size of the elements, and `resize` clones its
		// value for each slot but the last.
		("Vec", "append" | "resize" | "resize_with") => {
			let elem = targ(gargs, 0);
			let mut values = args;
			values.push(super::iter::size_of(b, elem, span));
			if name == "resize" {
				values.push(super::collections::clone_fn(b, elem, span));
			}
			rt2(values, "vec", &super::camel(&name))
		}
		("Vec", "extend_from_within") => {
			let elem = targ(gargs, 0);
			let ranged = super::collections::bounds(b, call, args.pop()?);
			args.push(ranged.bounds.clone());
			return Some(ranged.after(match byte(elem) {
				true => Expr::rs("byteVecExtendFromWithin", args),
				false => {
					args.push(super::iter::size_of(b, elem, span));
					args.push(super::collections::clone_fn(b, elem, span));
					Expr::rs("vecExtendFromWithin", args)
				}
			}));
		}
		("Vec", "insert" | "remove" | "swap_remove" | "truncate" | "split_off") => {
			rt2(args, "vec", &super::camel(&name))
		}
		("Vec", "clear") => {
			let mut values = args;
			values.push(Expr::lit("0"));
			Expr::rs("vecTruncate", values)
		}
		// What each takes by value is a closure, which it drops once done. An element JS
		// copies is lent to the closure as its place where it takes a `&mut`.
		("Vec", "retain" | "retain_mut" | "dedup" | "dedup_by" | "dedup_by_key") => {
			if let Some(ty) = dropping_operand(b, call) {
				return Some(b.err(
					span,
					format!("passing `{ty}`, which drops, to `{owner}::{name}`, which is not supported yet"),
				));
			}
			let elem = targ(gargs, 0);
			let of = match byte(elem) {
				true => "byteVec",
				false => "vec",
			};
			let lend = match b.t.needs_place(elem) {
				true => "Mut",
				false => "",
			};
			let mut values = args;
			let f = match name.as_str() {
				"retain" => "retain".to_string(),
				"retain_mut" => format!("retain{lend}"),
				"dedup" if byte(elem) => "dedup".to_string(),
				"dedup" => {
					values.push(super::collections::eq_fn(b, elem, span));
					"dedupBy".to_string()
				}
				"dedup_by" => format!("dedupBy{lend}"),
				_ => {
					// `Vec<T, A>::dedup_by_key::<F, K>`.
					let key = targ(gargs, 3);
					if b.t.drops(key) {
						return Some(b.err(
							span,
							format!("`Vec::{name}` by `{key}`, which drops, which is not supported yet"),
						));
					}
					values.push(super::collections::eq_fn(b, key, span));
					format!("dedupByKey{lend}")
				}
			};
			rt2(values, of, &f)
		}
		// Of a range, which `Vec<T, A>::drain::<R>` takes, as the other operand is the vector.
		("Vec", "drain") => {
			let ranged = super::collections::bounds(b, call, args.pop()?);
			args.push(ranged.bounds.clone());
			return Some(ranged.after(match byte(targ(gargs, 0)) {
				true => Expr::rs("byteVecDrain", args),
				false => {
					let dropper = b.t.dropper(targ(gargs, 0), span);
					Expr::rs("vecDrain", super::iter::tail(args, vec![dropper]))
				}
			}));
		}
		// A drain that fills its gap from the replacing items, which it drops last. std's
		// makes the drain, dropping the range, before it makes the items an iterator.
		("Vec", "splice") => {
			// `Vec<T, A>::splice::<R, I>`.
			let (elem, source) = (targ(gargs, 0), targ(gargs, 3));
			let iter = super::iter::iter_of(b.t, source)?;
			// std's reads the items through their own `by_ref`, which an override may give
			// another iterator in place of.
			let by_ref = super::collections::trait_fn(tcx, "Iterator", "by_ref");
			if let Ok(Some(instance)) =
				Instance::try_resolve(tcx, b.t.typing_env_pub(), by_ref, tcx.mk_args(&[iter.into()]))
				&& instance.def_id() != by_ref
			{
				return Some(b.err(
					span,
					format!("`Vec::splice` of `{iter}`, whose own `by_ref` std's calls, which is not supported yet"),
				));
			}
			let items = args.pop()?;
			let ranged = super::collections::bounds(b, call, args.pop()?);
			args.push(ranged.bounds.clone());
			let drain = match byte(elem) {
				true => Expr::rs("byteVecSplice", args),
				false => {
					args.push(super::iter::size_of(b, elem, span));
					let dropper = b.t.dropper(elem, span);
					Expr::rs("vecSplice", super::iter::tail(args, vec![dropper]))
				}
			};
			let items = super::iter::into_iter(b, source, items, span)?;
			// The drain, the first argument, which JS evaluates first.
			let drain = ranged.after(drain);
			let trusted = Expr::lit(super::iter::implements(b.t, iter, "TrustedLen").to_string());
			let drops = super::iter::iter_drops(b, iter, span);
			return Some(Expr::rs(
				"spliceFrom",
				super::iter::tail(vec![drain, items], vec![trusted, drops]),
			));
		}
		// The closure, which it drops as it drops, takes the place of each element JS copies.
		("Vec", "extract_if") => {
			let elem = targ(gargs, 0);
			let f = args.pop()?;
			let ranged = super::collections::bounds(b, call, args.pop()?);
			args.extend([ranged.bounds.clone(), f]);
			let extract = match (byte(elem), b.t.needs_place(elem)) {
				(true, _) => "byteVecExtractIf",
				(false, true) => "vecExtractIfMut",
				(false, false) => "vecExtractIf",
			};
			return Some(ranged.after(Expr::rs(extract, args)));
		}
		// Its elements' elements, with room for as many as its capacity held arrays.
		// `Vec<[T; N], A>::into_flattened`.
		("Vec", "into_flattened") => {
			let elem = targ(gargs, 0);
			let n = gargs.consts().next()?;
			let Some(n) = n.try_to_target_usize(tcx) else {
				return Some(b.err(
					span,
					format!("`Vec::into_flattened` of arrays of `{n}`, which is not known"),
				));
			};
			let mut values = args;
			values.push(Expr::lit(n.to_string()));
			return Some(match byte(elem) {
				true => Expr::rs("byteVecIntoFlattened", values),
				false => {
					values.push(super::iter::size_of(b, elem, span));
					Expr::rs("vecIntoFlattened", values)
				}
			});
		}
		// The room past a byte vector's length, a slice of its storage.
		("Vec", "spare_capacity_mut") if byte(targ(gargs, 0)) => Expr::rs("byteVecSpare", args),
		("Vec", "set_len") if byte(targ(gargs, 0)) => Expr::rs("byteVecSetLen", args),
		// The room past any other's, which the runtime keeps apart from its items, given
		// the size its capacity grows by and what a slot holds before a write.
		("Vec", "spare_capacity_mut" | "set_len") => {
			let elem = targ(gargs, 0);
			let uninit = Ty::new_maybe_uninit(tcx, elem);
			let value = b.t.uninit(uninit, span);
			let each = Expr::Arrow(
				Vec::new(),
				b.t.ty(uninit, span),
				vec![StmtKind::Return(Some(value)).at(None)],
			);
			let mut values = args;
			values.push(super::iter::size_of(b, elem, span));
			values.push(each);
			let f = match name.as_str() {
				"set_len" => "vecSetLen",
				_ => "vecSpare",
			};
			Expr::rs(f, values)
		}
		// Its elements, which nothing drops.
		("Vec", "leak") => {
			let v = args.pop()?;
			vec_view(v, targ(gargs, 0))
		}
		("Vec", "as_slice" | "as_mut_slice") => {
			let v = args.pop()?;
			vec_view(v, targ(gargs, 0))
		}
		// A box is what it holds.
		("AsRef", "as_ref") | ("AsMut", "as_mut") if b.t.std_of(targ(gargs, 0)) == Some(Std::Box) => args.pop()?,
		("Deref", "deref") | ("DerefMut", "deref_mut") => {
			let ty = targ(gargs, 0);
			let v = args.pop()?;
			match ty.kind() {
				ty::Adt(adt, vargs) => match Std::of(tcx, adt.did()) {
					Some(Std::Vec) => vec_view(v, vargs.type_at(0)),
					// A `String` is the place of its text, as a `&mut str` of it is.
					Some(Std::String) if name == "deref" => v.member("v"),
					Some(Std::String | Std::Box | Std::ManuallyDrop | Std::AssertUnwindSafe | Std::IoSlice) => v,
					Some(Std::Cow) if let Some(borrowed) = cow_borrowed(tcx, ty) => cow_deref(v, borrowed),
					// The greatest element of a heap, which `DerefMut` marks to sift once lent.
					Some(Std::PeekMut) => {
						let p = match v.is_path() {
							true => v,
							false => b.spill(v, "peek", out),
						};
						if name == "deref_mut" {
							out.push(StmtKind::Expr(Expr::rs("peekMutDerefMut", vec![p.clone()])).at(None));
						}
						p.member("v")
					}
					_ => return None,
				},
				_ => return None,
			}
		}

		// Strings.
		// A `Box<str>`'s bytes are their own, as `as_bytes` gives them.
		("String", "as_bytes") => Expr::rs("stringAsBytes", vec![args.pop()?.member("v")]),
		("str", "as_bytes" | "into_boxed_bytes") => Expr::rs("stringAsBytes", args),
		("String", "from_utf8") => Expr::rs("stringFromUtf8", args),
		// Its bytes, with its capacity, as the vector it is.
		("String", "into_bytes") => Expr::rs("stringIntoBytes", args),
		("String", "from_utf8_lossy") => Expr::rs("stringFromUtf8Lossy", args),
		// UTF-8 by their contract, which the lossy decoder reads as the text.
		("converts" | "str", "from_utf8_unchecked") => Expr::rs("strFromUtf8Unchecked", args),
		("String", "from_utf8_unchecked") => Expr::rs("stringFromUtf8Unchecked", args),
		("FromUtf8Error", "as_bytes") => Expr::rs("byteVecSlice", vec![args.pop()?.member("bytes")]),
		("FromUtf8Error", "into_bytes") => args.pop()?.member("bytes"),
		("FromUtf8Error", "utf8_error") => args.pop()?.member("error"),
		("slice", "utf8_chunks") => Expr::rs("byteSliceUtf8Chunks", args),
		("Utf8Chunk", "valid" | "invalid") => args.pop()?.member(&name),
		(
			"String",
			"from_utf16" | "from_utf16_lossy" | "from_utf16le" | "from_utf16le_lossy" | "from_utf16be"
			| "from_utf16be_lossy",
		) => Expr::rs_of("string", &super::camel(&name)).call(args),
		// `str::from_utf8` is `core::str::converts::from_utf8`, or `str`'s own.
		("converts" | "str", "from_utf8") => Expr::rs("strFromUtf8", args),
		("Utf8Error", "valid_up_to") => args.pop()?.member("validUpTo"),
		("Utf8Error", "error_len") => args.pop()?.member("errorLen"),
		("String", "len") => args.pop()?.member("len"),
		("String", "capacity") => args.pop()?.member("cap"),
		("str", "len") => Expr::rs("stringLen", args),
		// A `Box<str>` is its text, as a `String`'s is once it shrinks to fit, and a `String`
		// of one has room for exactly its bytes.
		("String", "as_str" | "into_boxed_str") => args.pop()?.member("v"),
		("str", "into_string") => string_from(args.pop()?),
		("String", "new") => Expr::rs("stringNew", vec![]),
		// `String + &str`, which pushes onto the moved `String` as `+=` does.
		("Add", "add") if owned_string(tcx, targ(gargs, 0)) => Expr::rs("stringAdd", args),
		("Add", "add") if cow_str(tcx, targ(gargs, 0)) => {
			let rhs = str_cow(tcx, targ(gargs, 1), args.pop()?);
			Expr::rs("strCowAdd", vec![args.pop()?, rhs])
		}
		("String", "with_capacity") => Expr::rs("stringWithCapacity", args),
		// Of the place the `&mut` is, which it writes as it drops, and of a range, as `Vec::drain`.
		("String", "drain") => {
			let ranged = super::collections::bounds(b, call, args.pop()?);
			let s = args.pop()?;
			let s = match s.is_path() {
				true => s,
				false => b.spill(s, "s", out),
			};
			let len = s.clone().member("len");
			return Some(Expr::rs("stringDrain", vec![s, ranged.within(len)]));
		}
		("Drain", "as_str") => Expr::rs("stringDrainAsStr", args),
		("ParseIntError", "kind") => args.pop()?.member("kind"),
		// `str::parse` of a float, by std's grammar rather than `Number`'s.
		("str", "parse") | ("FromStr", "from_str") if let Some(float) = super::float::Float::of(targ(gargs, 0)) => {
			Expr::rs_of(float.name(), "fromStr").call(args)
		}
		// A `char`'s steps through its chars by `&mut`, which no body of std's may.
		("str", "parse") | ("FromStr", "from_str") if targ(gargs, 0).is_char() => Expr::rs("charFromStr", args),
		("String", "is_empty") => args.pop()?.member("len").binary("===", Expr::lit("0")),
		("str", "is_empty") => args.pop()?.member("length").binary("===", Expr::lit("0")),
		("str", "chars") => Expr::rs("strChars", args),
		("str", "bytes") => Expr::rs("strBytes", args),
		("str", "is_char_boundary") => Expr::rs("strIsCharBoundary", args),
		// Searches by a pattern.
		("str", "split" | "rsplit" | "split_terminator" | "rsplit_terminator" | "split_inclusive")
		| ("str", "matches" | "rmatches" | "match_indices" | "rmatch_indices" | "split_once" | "rsplit_once")
		| ("str", "find" | "rfind" | "contains" | "starts_with" | "ends_with" | "strip_prefix" | "strip_suffix")
		| ("str", "trim_matches" | "trim_start_matches" | "trim_end_matches") => {
			let [s, p] = <[Expr; 2]>::try_from(args).ok()?;
			Expr::rs_of("str", &super::camel(&name)).call(vec![s, pattern(b.t, targ(gargs, 0), p)?])
		}
		("str", "splitn" | "rsplitn") => {
			let [s, n, p] = <[Expr; 3]>::try_from(args).ok()?;
			Expr::rs_of("str", &name).call(vec![s, n, pattern(b.t, targ(gargs, 0), p)?])
		}
		// `replacen`'s count follows what replaces each match. std's `replace` reserves by
		// the pattern's length if it is a `char`, a `&str`, or a `&String`.
		("str", "replace" | "replacen") => {
			let p = targ(gargs, 0);
			let mut args = args.into_iter();
			let (s, pat) = (args.next()?, args.next()?);
			let mut values = [vec![s, pattern(b.t, p, pat)?], args.collect()].concat();
			if name == "replace" {
				let utf8 = p.is_char() || matches!(p.kind(), ty::Ref(_, t, _) if t.is_str() || owned_string(tcx, *t));
				values.push(Expr::lit(utf8.to_string()));
			}
			Expr::rs(if name == "replace" { "strReplace" } else { "strReplacen" }, values)
		}
		// Searches by whitespace, as std's are by `char::is_whitespace`.
		("char", "is_whitespace") => Expr::rs("charIsWhitespace", args),
		// By std's own Unicode tables, which the engine's may trail.
		(
			"char",
			"is_alphabetic" | "is_lowercase" | "is_uppercase" | "is_alphanumeric" | "is_control" | "is_numeric"
			| "to_lowercase" | "to_uppercase" | "escape_unicode" | "escape_debug" | "escape_default" | "len_utf8"
			| "len_utf16" | "encode_utf16" | "from_digit",
		) => Expr::rs_of("char", &super::camel(&name)).call(args),
		(
			"str",
			"to_lowercase" | "to_uppercase" | "escape_unicode" | "escape_debug" | "escape_default" | "encode_utf16",
		) => Expr::rs_of("str", &super::camel(&name)).call(args),
		// `char::decode_utf16` of what it is given, either as a method or as `char`'s function.
		("char", "decode_utf16") => {
			let mut values = args;
			let it = super::iter::into_iter(b, targ(gargs, 0), values.pop()?, span)?;
			Expr::rs("charDecodeUtf16", vec![it])
		}
		("DecodeUtf16Error", "unpaired_surrogate") => args.pop()?.member("code"),
		("str", "trim" | "trim_start" | "trim_end") => {
			let f = format!("{}_matches", name);
			Expr::rs_of("str", &super::camel(&f)).call(vec![args.pop()?, Expr::Rs("charIsWhitespace".into())])
		}
		("str", "char_indices" | "lines" | "split_whitespace" | "split_ascii_whitespace") => {
			Expr::rs_of("str", &super::camel(&name)).call(args)
		}
		// A slice of strings concatenated, or joined by a `&str`.
		("slice", "concat" | "join")
			if textual(tcx, targ(gargs, 0).peel_refs()) && targ(gargs, 1).peel_refs().is_str() =>
		{
			let mut values = args;
			values[0] = elements(values[0].clone(), Ty::new_slice(tcx, targ(gargs, 0)));
			if cow_str(tcx, targ(gargs, 0).peel_refs()) {
				values[0] = values[0].clone().member("map").call(vec![Expr::Rs("strCowStr".into())]);
			}
			if name == "concat" {
				values.push(Expr::Str(String::new()));
			}
			Expr::rs("strsJoin", values)
		}
		("slice", "concat" | "join") => {
			return slices_join(b, targ(gargs, 0), targ(gargs, 1), name == "join", args, span);
		}
		("char", _) if let Some(method) = ascii(&name) => Expr::rs(method, args),
		("ascii", "escape_default") => Expr::rs("escapeAscii", args),
		// Those of `[u8]`, a `Vec<u8>`'s by `Deref`, and of `str`, a `String`'s, but `str`'s
		// `make_ascii_*`, which MIR lowers as a byte's and a char's.
		(
			"slice" | "str",
			"is_ascii"
			| "eq_ignore_ascii_case"
			| "to_ascii_uppercase"
			| "to_ascii_lowercase"
			| "trim_ascii_start"
			| "trim_ascii_end"
			| "trim_ascii",
		)
		| ("slice", "escape_ascii" | "make_ascii_uppercase" | "make_ascii_lowercase") => {
			let of = if owner == "slice" { "byteSlice" } else { "str" };
			let value = Expr::rs_of(of, &super::camel(&name)).call(args);
			// A copy of the bytes, as `to_vec` makes it.
			match (owner.as_str(), name.as_str()) {
				("str", "to_ascii_uppercase" | "to_ascii_lowercase") => string_from(value),
				_ => value,
			}
		}
		// The `String` a `Cow<str>` owns, or one of the text it borrows.
		("Cow", "into_owned") if targ(gargs, 0).is_str() => Expr::rs("strCowIntoOwned", args),
		("AsRef", "as_ref")
			if textual(tcx, targ(gargs, 0)) && !cow_str(tcx, targ(gargs, 0)) && targ(gargs, 1).is_str() =>
		{
			text(tcx, targ(gargs, 0), args.pop()?)
		}
		("AsRef", "as_ref")
			if (string(tcx, targ(gargs, 0)) || owned_string(tcx, targ(gargs, 0))) && byte_slice(targ(gargs, 1)) =>
		{
			Expr::rs("stringAsBytes", vec![text(tcx, targ(gargs, 0), args.pop()?)])
		}
		// A `Cow<[T]>` borrows its slice, and its `Vec` is its own or a clone of what it
		// borrows, so it drops nothing.
		("Cow", "into_owned") if let ty::Slice(elem) = targ(gargs, 0).kind() => {
			return Some(cow_cloned(b, "IntoOwned", args.pop()?, *elem, span));
		}
		("AsRef", "as_ref") | ("Borrow", "borrow") if let Some(borrowed) = cow_borrowed(tcx, targ(gargs, 0)) => {
			cow_deref(args.pop()?, borrowed)
		}
		// A `String` with room for exactly its bytes, as `str`'s `to_owned` makes it, and its
		// `to_string`, through as many references as std's impls name.
		("ToOwned", "to_owned") if self_ty()?.is_str() => string_from(args.pop()?),
		("ToString", "to_string") if self_ty()?.peel_refs().is_str() && refs(self_ty()?) <= 12 => {
			string_from(b.t.pointee_value(self_ty()?, args.pop()?))
		}
		// std's `ToOwned` for a `T: Clone` is its `clone`.
		("ToOwned", "to_owned") if self_ty()?.is_sized(tcx, b.t.typing_env_pub()) => {
			let clone = super::collections::trait_fn(tcx, "Clone", "clone");
			return Some(b.call_def(clone, tcx.mk_args(&[self_ty()?.into()]), args, span, out));
		}

		// Panics, which format their message as std does.
		("panicking", "panic_fmt") => {
			let message = args.pop()?;
			match literal(&message) {
				Some(s) => Expr::rs("panic", [vec![Expr::Str(s)], hook_at(b)].concat()),
				None => Expr::rs("panicFmt", vec![message]),
			}
		}
		("panicking", "panic") => Expr::rs("panic", [args, hook_at(b)].concat()),
		// `panic!()`, `unreachable!()`, `unimplemented!()`, and `todo!()` without a message.
		("panicking", "panic_default_msg" | "unreachable_default_msg" | "unimplemented_default_msg" | "todo_default_msg") => {
			let message = match name.as_str() {
				"panic_default_msg" => "explicit panic",
				"unreachable_default_msg" => "internal error: entered unreachable code",
				"unimplemented_default_msg" => "not implemented",
				_ => "not yet implemented",
			};
			Expr::rs("panic", [vec![Expr::Str(message.into())], hook_at(b)].concat())
		}
		// The harness writes a payload of text, and `Box<dyn Any>` for any other. A hook reads
		// the payload itself: a `String`, a `&str`, or the `dyn Any` object of any other.
		("panic", "panic_any") | ("panicking", "begin_panic") => {
			let payload = targ(gargs, 0);
			let owned = b.t.std_of(payload) == Some(Std::String);
			match (
				matches!(payload.kind(), ty::Ref(_, s, _) if s.is_str()) || owned,
				b.t.hooked,
			) {
				(true, true) => {
					let at = b.panic_at.clone().unwrap_or_else(Expr::undefined);
					let text = args.pop()?;
					match owned {
						true => Expr::rs("panic", vec![text.clone().member("v"), at, text]),
						false => Expr::rs("panic", vec![text, at]),
					}
				}
				(true, false) if owned => Expr::rs("panic", vec![args.pop()?.member("v")]),
				(true, false) => Expr::rs("panic", args),
				(false, true) => {
					let class = b.t.object_class(
						payload,
						super::dynamic::any_object(tcx)?,
						super::dynamic::Held::Value,
						span,
					)?;
					Expr::rs("panicAny", vec![Expr::New(Box::new(Expr::Sym(class.sym)), args)])
				}
				(false, false) => Expr::rs("panic", vec![Expr::Str("Box<dyn Any>".into())]),
			}
		}
		// A panic aborts, so `catch_unwind` never catches one. Its call consumes the function,
		// so nothing drops it.
		("panic", "catch_unwind") => return Some(Expr::result(true, args.pop()?.call(Vec::new()))),
		("panic", "resume_unwind") => Expr::rs("resumeUnwind", Vec::new()),
		// The location the call passes, as a `#[track_caller]` function's call does.
		("Location", _) if inherent_std(tcx, call.def_id) != Some(Std::Location) => return None,
		("Location", "caller") => {
			let instance = Instance::try_resolve(tcx, b.t.typing_env_pub(), call.def_id, call.args).ok()??;
			b.caller_arg(instance, span)
		}
		("Location", "file" | "line") => args.pop()?.member(name.as_str()),
		("Location", "column") => args.pop()?.member("col"),
		// Once a program sets a hook, each panic passes where it is, and std's own code's
		// panics are where the toolchain's source has them.
		("panicking", "set_hook") => {
			b.t.hook();
			let sites = super::caller::sites(b.t, span);
			Expr::rs("setHook", vec![args.pop()?, sites])
		}
		("panicking", "take_hook") => Expr::rs("takeHook", Vec::new()),
		("PanicHookInfo", "payload") => {
			let object = b.t.sig(call.def_id, gargs).output().builtin_deref(true)?;
			let class = |b: &mut Body<'_, '_, 'tcx>, ty| {
				let class = b.t.object_class(ty, object, super::dynamic::Held::Value, span)?;
				Some(Expr::Sym(class.sym))
			};
			let str = class(b, Ty::new_static_str(tcx))?;
			let string = tcx.lang_items().string()?;
			let string = class(b, Ty::new_adt(tcx, tcx.adt_def(string), ty::GenericArgs::empty()))?;
			Expr::rs("hookPayload", vec![args.pop()?, str, string])
		}
		("PanicHookInfo", "payload_as_str") => args.pop()?.member("text"),
		("PanicHookInfo", "location") => args.pop()?.member("location"),
		// A panic aborts.
		("PanicHookInfo", "can_unwind") => Expr::lit("false"),
		("panicking", "panic_explicit") => {
			Expr::rs("panic", [vec![Expr::Str("explicit panic".into())], hook_at(b)].concat())
		}
		("panicking", "panic_display" | "unreachable_display") => {
			let value = args.pop()?;
			let value = b.fmt_value(fmt::Trait::Display, targ(gargs, 0), value, span, out);
			let mut message = vec![value];
			if name == "unreachable_display" {
				message.insert(0, Expr::Str("internal error: entered unreachable code: ".into()));
			}
			Expr::rs("panicFmt", vec![Expr::Array(message)])
		}
		// `assert_eq!` and `assert_ne!`: the operator, both sides, and any message.
		("panicking", "assert_failed") => {
			let [kind, left, right, message] = <[Expr; 4]>::try_from(args).ok()?;
			let left = b.fmt_value(fmt::Trait::Debug, targ(gargs, 0), left, span, out);
			let right = b.fmt_value(fmt::Trait::Debug, targ(gargs, 1), right, span, out);
			let mut values = vec![kind, left, right];
			if !matches!(&message, Expr::Lit(text) if text == "undefined") {
				values.push(message);
			}
			Expr::rs("assertFailed", values)
		}

		// Formatting.
		("hint", "must_use") => args.pop()?,
		// The size and alignment wasm32 lays the type out in, which translation targets.
		("mem", "size_of") => {
			let env = b.t.typing_env_pub();
			let layout = tcx.layout_of(env.as_query_input(targ(gargs, 0))).ok()?;
			Expr::lit(layout.size.bytes().to_string())
		}
		("mem", "align_of") => {
			let env = b.t.typing_env_pub();
			let layout = tcx.layout_of(env.as_query_input(targ(gargs, 0))).ok()?;
			Expr::lit(layout.align.bytes().to_string())
		}
		// rustc's answers for the instance, which a JS value that drops nothing, such as a
		// `String`'s, leaves as they are.
		("mem", "needs_drop") => {
			let needs = targ(gargs, 0).needs_drop(tcx, b.t.typing_env_pub());
			Expr::lit(needs.to_string())
		}
		("any", "type_name" | "type_name_of_val") => Expr::Str(rustc_const_eval::util::type_name(tcx, targ(gargs, 0))),
		// What wasm32 lays out the value in: its type's, or a slice's elements' or a `str`'s
		// bytes'. A trait object's is its vtable's, which is left to the refusal of any other call.
		("mem", "size_of_val" | "align_of_val") => {
			let env = b.t.typing_env_pub();
			let ty = targ(gargs, 0);
			let value = args.pop()?;
			let (elem, len) = match ty.kind() {
				ty::Slice(elem) => (*elem, Some(super::mir::slice_member(value, "length"))),
				ty::Str => (tcx.types.u8, Some(Expr::rs("stringLen", vec![value]))),
				_ if ty.is_sized(tcx, env) => (ty, None),
				_ => return None,
			};
			let layout = tcx.layout_of(env.as_query_input(elem)).ok()?;
			let size = layout.size.bytes();
			match (name.as_str(), len) {
				("align_of_val", _) => Expr::lit(layout.align.bytes().to_string()),
				(_, None) => Expr::lit(size.to_string()),
				(_, Some(len)) if size == 1 => len,
				(_, Some(len)) => len.binary("*", Expr::lit(size.to_string())),
			}
		}
		("convert", "identity") => args.pop()?,
		// A reference to an array is a slice of it already.
		("array", "as_slice" | "as_mut_slice") => args.pop()?,
		("array", "from_fn") => {
			let n = gargs.consts().next()?.try_to_target_usize(tcx)?;
			let f = match targ(gargs, 0).kind() {
				_ if byte(targ(gargs, 0)) => "bytesFromFn",
				_ => "arrayFromFn",
			};
			Expr::rs(f, vec![Expr::lit(n.to_string()), args.pop()?])
		}
		("array", "map") => {
			let f = match targ(gargs, 2).kind() {
				_ if byte(targ(gargs, 2)) => "bytesMap",
				_ => "arrayMap",
			};
			Expr::rs(f, args)
		}
		// Each element lent as `first_mut` lends it: an element JS copies as its place, an
		// array as the slice of it, and an object as itself.
		("array", "each_ref" | "each_mut") => {
			let elem = targ(gargs, 0);
			let s = args.pop()?;
			if name == "each_mut" && b.t.needs_place(elem) {
				return Some(Expr::rs("slicePlaces", vec![s]));
			}
			if elem.is_array() {
				return Some(Expr::rs("arraySliceEach", vec![s]));
			}
			let a = match byte(elem) {
				true => Expr::rs("byteSliceView", vec![s]),
				false => slice_items(s),
			};
			match elem.kind() {
				ty::Array(..) => Expr::rs("arrayMap", vec![a, b.t.array_rs(elem, "From")]),
				_ => Expr::rs("arrayFrom", vec![a]),
			}
		}
		// Text alone is `to_owned`, as std's `as_str` gives it.
		("fmt", "format") => {
			let parts = args.pop()?;
			match literal(&parts) {
				Some(s) => string_from(Expr::Str(s)),
				None => Expr::rs("fmtFormat", vec![parts]),
			}
		}
		("Display" | "Debug" | "LowerHex" | "UpperHex" | "Octal" | "Binary" | "LowerExp" | "UpperExp", "fmt") => {
			let tr = fmt::Trait::of(tcx, tcx.trait_of_assoc(call.def_id)?)?;
			let [value, f] = <[Expr; 2]>::try_from(args).ok()?;
			fmt::std_impl(b, tr, targ(gargs, 0), value, f, span, out)?
		}
		// `Error`'s own `source`, which a type that has none keeps, and std's for a reference
		// or a box, which asks what it points at, its JS value.
		("Error", "source")
			if tcx
				.trait_of_assoc(call.def_id)
				.is_some_and(|tr| super::dynamic::is_error(tcx, tr)) =>
		{
			let self_ty = targ(gargs, 0);
			if let Some(inner) = self_ty.builtin_deref(true).filter(|_| !self_ty.is_raw_ptr()) {
				return Some(b.call_def(call.def_id, tcx.mk_args(&[inner.into()]), args, span, out));
			}
			let instance = Instance::try_resolve(tcx, b.t.typing_env_pub(), call.def_id, gargs).ok()??;
			if instance.def_id() != call.def_id {
				return None;
			}
			let sig = b.t.sig(call.def_id, gargs);
			match sig.output().kind() {
				ty::Adt(_, option) => b.t.none(option.type_at(0)),
				_ => return None,
			}
		}
		// std's own for a `String` and a `Cow<str>`, and for a `char`, a `bool`, and an
		// unsigned integer, have room for exactly their bytes, and for a signed integer, for
		// its most digits and a sign if it has one; `fmt::Arguments`' is `format`. Any other
		// grows by what its `Display` writes.
		("ToString", "to_string") => {
			let ty = targ(gargs, 0);
			let value = args.pop()?;
			let size = match ty.kind() {
				ty::Adt(..) if textual(tcx, ty) && !string(tcx, ty) => return Some(string_from(text(tcx, ty, value))),
				ty::Adt(..) if b.t.std_of(ty) == Some(Std::Arguments) => {
					return Some(Expr::rs("fmtFormat", vec![value]));
				}
				ty::Char | ty::Bool | ty::Uint(_) => Some(0),
				ty::Int(int) => Some(match int.bit_width().unwrap_or(32) {
					8 => 3,
					16 => 5,
					32 => 10,
					64 => 19,
					_ => 39,
				}),
				_ => None,
			};
			let value = b.fmt_value(fmt::Trait::Display, ty, value, span, out);
			Expr::rs(
				"fmtToString",
				[
					vec![value],
					size.map(|n: u32| Expr::lit(n.to_string())).into_iter().collect(),
				]
				.concat(),
			)
		}
		// `fmt::Write`'s, of a `String`, a `Formatter`, or the crate's own writer.
		("Write", "write_str" | "write_char" | "write_fmt")
			if tcx
				.trait_of_assoc(call.def_id)
				.is_some_and(|tr| tcx.is_diagnostic_item(rustc_span::Symbol::intern("FmtWrite"), tr)) =>
		{
			let args = <[Expr; 2]>::try_from(args).ok()?;
			let literal = literal(&args[1]);
			b.fmt_write(call.def_id, targ(gargs, 0), args, literal, span, out)?
		}
		("Formatter", "write_fmt") => {
			let [f, parts] = <[Expr; 2]>::try_from(args).ok()?;
			match literal(&parts) {
				Some(s) => Expr::rs("fmtWriteStr", vec![f, Expr::Str(s)]),
				None => Expr::rs("fmtWriteFmt", vec![f, parts]),
			}
		}
		// Each takes its receiver first: `rs.fmtPad(f, s)`, `rs.debugStructField(b, name, value)`.
		("Formatter", "write_str" | "pad" | "alternate" | "width" | "debug_struct" | "debug_tuple" | "debug_list")
		| ("DebugStruct" | "DebugTuple", "field" | "finish" | "finish_non_exhaustive")
		| ("DebugList", "entry" | "finish" | "finish_non_exhaustive") => {
			let of = match owner.as_str() {
				"Formatter" => "fmt",
				"DebugStruct" => "debugStruct",
				"DebugTuple" => "debugTuple",
				_ => "debugList",
			};
			Expr::rs_of(of, &super::camel(&name)).call(args)
		}
		// Derived `Debug`, whose names are the type's then its fields', space-separated:
		// `debug_struct_field2_finish(f, "Point x y", &x, &y)`, past five fields of a slice.
		("Formatter", "debug_struct_fields_finish") => {
			let [f, names, values] = <[Expr; 3]>::try_from(args).ok()?;
			Expr::rs("fmtDebugFields", vec![f, names, slice_items(values)])
		}
		("Formatter", n) if n.starts_with("debug_struct_field") => {
			let mut args = args.into_iter();
			let (f, names) = (args.next()?, args.next()?);
			Expr::rs("fmtDebugFields", vec![f, names, Expr::Array(args.collect())])
		}
		("Formatter", "debug_c_like_enum_write_str") => Expr::rs("fmtDebugEnum", args),
		_ => return super::iter::call(b, call, &owner, &name, args),
	};
	if let Some(ty) = dropping {
		return Some(b.err(
			span,
			format!("passing `{ty}`, which drops, to `{owner}::{name}`, which is not supported yet"),
		));
	}
	// What drops each operand the mapping may discard, then `rs.None` where it is one.
	let lost = discarded.unwrap_or_default().iter().map(|&n| targ(gargs, n));
	let Some(mut droppers) = b.t.droppers(lost, span) else {
		return Some(Expr::undefined());
	};
	let none = match none_last(&owner, &name) {
		true => sentinel_option(b.t, call).and_then(|inner| b.t.none_arg(inner)),
		false => None,
	};
	if let Some(none) = none {
		droppers.resize(discarded.unwrap_or_default().len(), Expr::undefined());
		droppers.push(none);
	}
	Some(match e {
		Expr::Call(f, mut args) if !droppers.is_empty() => {
			args.extend(droppers);
			Expr::Call(f, args)
		}
		e => e,
	})
}

/// A number from its big-endian bytes, each a value: shifted and or-ed.
pub fn be_word(int: Int, values: Vec<Expr>) -> Expr {
	let last = values.len() - 1;
	let mut word: Option<Expr> = None;
	let mut top = false;
	for (i, value) in values.into_iter().enumerate() {
		if matches!(&value, Expr::Lit(l) if l == "0") {
			continue;
		}
		top |= i == 0;
		let shift = 8 * (last - i);
		let term = match shift {
			0 => value,
			_ => value.binary("<<", Expr::lit(shift.to_string())),
		};
		word = Some(match word {
			Some(word) => word.binary("|", term),
			None => term,
		});
	}
	let Some(word) = word else {
		return Expr::lit("0");
	};
	// JS's `|` and `<<` make an `i32`, so only a `u32` with its top byte set and the
	// narrower signed types need their bits read again.
	match (int.signed, int.bits) {
		(false, 32) if top => int.wrap(word),
		(true, bits) if bits < 32 => int.wrap(word),
		_ => word,
	}
}

/// The `n`th type among a call's generic arguments, skipping lifetimes.
pub fn targ<'tcx>(gargs: GenericArgs<'tcx>, n: usize) -> Ty<'tcx> {
	gargs.types().nth(n).expect("a type argument")
}

/// `value`, a `ty`, as a value of its own: copied if it is a `Copy` aggregate that is not
/// made right here.
fn owned<'tcx>(b: &mut Body<'_, '_, 'tcx>, value: Expr, ty: Ty<'tcx>, span: Span) -> Expr {
	fn made(e: &Expr) -> bool {
		match e {
			Expr::Array(_) | Expr::Object(_) | Expr::New(..) => true,
			Expr::As(e, _) | Expr::Located(_, e) => made(e),
			_ => false,
		}
	}
	match made(&value) {
		true => value,
		false => b.t.copied(value, ty, span),
	}
}

/// The `Box<[elem]>` that `clone_from` of `source` leaves in place of `slot`: `slot`, each
/// element cloned over its own as `clone_from_slice` clones them, where the lengths match,
/// and else a clone of `source`, made before `slot`'s elements drop.
pub(super) fn box_slice_clone_from<'tcx>(
	b: &mut Body<'_, '_, 'tcx>,
	slot: Expr,
	source: Expr,
	elem: Ty<'tcx>,
	span: Span,
) -> Expr {
	if byte(elem) {
		return Expr::rs("byteBoxSliceCloneFrom", vec![slot, source]);
	}
	let (name, fns) = box_slice_clone_from_fns(b, elem, span);
	Expr::rs(name, [vec![slot, source], fns].concat())
}

/// The runtime's `clone_from` of a `Box<[elem]>` of other than bytes, and what it takes
/// after the box it writes over and its source: of arrays, the arrays' own, which makes arrays
/// of a clone by `array_fn`'s `From`.
fn box_slice_clone_from_fns<'tcx>(b: &mut Body<'_, '_, 'tcx>, elem: Ty<'tcx>, span: Span) -> (&'static str, Vec<Expr>) {
	let fns = clone_from_fns(b, elem, span);
	match elem.is_array() {
		true => (
			"arraySliceBoxCloneFrom",
			[vec![super::array_fn(elem, "From")], fns].concat(),
		),
		false => ("boxSliceCloneFrom", fns),
	}
}

/// What `clone_from` of `source` leaves over `old`, a `ty` whose `clone_from` is std's own
/// over what it holds, as `clone_from_clones` says it is not: `old` itself, written in
/// place, for all but an `Option`, a `Result`, and a `Cow`. Refused for any other `clone_from`.
pub(super) fn clone_from<'tcx>(b: &mut Body<'_, '_, 'tcx>, old: Expr, source: Expr, ty: Ty<'tcx>, span: Span) -> Expr {
	match std_clone_from(b, ty, span) {
		Ok((name, fns)) => Expr::rs(name, [vec![old, source], fns].concat()),
		Err(refused) => refused,
	}
}

/// An `rs.CloneFrom` of a `ty` whose `clone_from` is not `*self = source.clone()`: the
/// crate's own, or std's over what it holds.
fn clone_from_fn<'tcx>(b: &mut Body<'_, '_, 'tcx>, ty: Ty<'tcx>, span: Span) -> Expr {
	let tcx = b.t.tcx;
	let own = unboxed(b.t, ty);
	let clone_from = super::collections::trait_fn(tcx, "Clone", "clone_from");
	let args = tcx.mk_args(&[own.into()]);
	if b.t.translated(clone_from, args) {
		let name = match b.t.needs_place(own) {
			true => "cloneFromPlaceFn",
			false => "cloneFromFn",
		};
		return Expr::rs(name, vec![b.fn_value(clone_from, args, span)]);
	}
	match std_clone_from(b, ty, span) {
		Ok((name, fns)) => Expr::rs(format!("{name}Fn"), fns),
		Err(refused) => refused,
	}
}

/// The runtime's `clone_from` of a `ty` whose `clone_from` is std's own over what it holds,
/// and what it takes after the value it writes over and its source.
fn std_clone_from<'tcx>(
	b: &mut Body<'_, '_, 'tcx>,
	ty: Ty<'tcx>,
	span: Span,
) -> Result<(&'static str, Vec<Expr>), Expr> {
	let own = unboxed(b.t, ty);
	if let ty::Array(elem, _) = own.kind() {
		return Ok(("arrayCloneFrom", clone_from_fns(b, *elem, span)));
	}
	if let Some(elem) = box_slice(b.t.tcx, own) {
		return Ok(box_slice_clone_from_fns(b, elem, span));
	}
	let refused = |b: &mut Body<'_, '_, 'tcx>| {
		b.err(
			span,
			format!("`clone_from` of `{ty}`, whose `clone_from` is its own, which is not supported yet"),
		)
	};
	// A `String`'s bytes clone into its buffer, as an owned `Cow<str>`'s do over an owned one's.
	if owned_string(b.t.tcx, own) {
		return Ok(("stringCloneFrom", vec![]));
	}
	if cow_str(b.t.tcx, own) {
		return Ok(("strCowCloneFrom", vec![]));
	}
	// A `Cow<[T]>`'s: an owned one's `Vec` clones into itself over an owned one's.
	if let Some(elem) = cow_slice(b.t.tcx, own) {
		return Ok(match elem.kind() {
			_ if byte(elem) => ("byteCowCloneFrom", vec![]),
			_ => {
				let size = super::iter::size_of(b, elem, span);
				("cowCloneFrom", [vec![size], clone_from_fns(b, elem, span)].concat())
			}
		});
	}
	let ty::Adt(_, args) = own.kind() else {
		return Err(refused(b));
	};
	let first = args.types().next();
	match (b.t.std_of(own), first) {
		(Some(Std::Vec), Some(elem)) if byte(elem) => Ok(("byteVecCloneFrom", vec![])),
		// A heap's is its vector's.
		(Some(Std::Vec | Std::BinaryHeap), Some(elem)) => {
			let name = match b.t.std_of(own) {
				Some(Std::Vec) => "vecCloneFrom",
				_ => "heapCloneFrom",
			};
			let size = super::iter::size_of(b, elem, span);
			Ok((name, [vec![size], clone_from_fns(b, elem, span)].concat()))
		}
		// Cleared, then extended by clones, in the buffer it keeps.
		(Some(Std::VecDeque), Some(elem)) => {
			let fns = vec![super::collections::clone_fn(b, elem, span), b.t.dropper(elem, span)];
			Ok(("dequeCloneFrom", trimmed(fns)))
		}
		(Some(Std::LinkedList), Some(elem)) => Ok(("listCloneFrom", clone_from_fns(b, elem, span))),
		// Of two `Some`s, what one holds clones over the other's by its own `clone_from`.
		(Some(Std::Option), Some(inner)) => {
			let clone = super::collections::clone_fn(b, inner, span);
			let drop = b.t.dropper(inner, span);
			let over = clone_from_fn(b, inner, span);
			let mut fns = vec![clone, drop, over];
			fns.extend(b.t.none_arg(inner));
			Ok(("optionCloneFrom", fns))
		}
		// Of two `Ok`s or two `Err`s, what one holds clones over the other's, as an
		// element's does.
		(Some(Std::Result), Some(ok)) => {
			let mut fns = clone_from_fns(b, ok, span);
			fns.resize_with(3, Expr::undefined);
			fns.extend(clone_from_fns(b, args.type_at(1), span));
			Ok(("resultCloneFrom", trimmed(fns)))
		}
		(Some(Std::RefCell), Some(inner)) => Ok(("refCellCloneFrom", clone_from_fns(b, inner, span))),
		(Some(Std::Waker), _) => Ok(("wakerCloneFrom", vec![])),
		_ => Err(refused(b)),
	}
}

/// What the runtime's clones of `elem`s over others by their `clone_from` take after their
/// own arguments: what clones one, a copy for a `TrivialClone`; what drops one; and its
/// `rs.CloneFrom` where its `clone_from` is not `*self = source.clone()`.
pub(super) fn clone_from_fns<'tcx>(b: &mut Body<'_, '_, 'tcx>, elem: Ty<'tcx>, span: Span) -> Vec<Expr> {
	if super::iter::implements(b.t, elem, "TrivialClone") {
		return b.t.copy_fn(elem, span).into_iter().collect();
	}
	let mut fns = vec![super::collections::clone_fn(b, elem, span), b.t.dropper(elem, span)];
	if !clone_from_clones(b.t, elem) {
		fns.push(clone_from_fn(b, elem, span));
	}
	trimmed(fns)
}

/// What `ty` holds through std's boxes of sized values and `Reverse`s, each its value in
/// JS, whose `clone_from` is that of what they hold.
pub(super) fn unboxed<'tcx>(t: &Translator<'_, 'tcx>, mut ty: Ty<'tcx>) -> Ty<'tcx> {
	loop {
		ty = match ty.kind() {
			ty::Adt(_, args) if t.std_of(ty) == Some(Std::Reverse) => args.type_at(0),
			_ if let Some(inner) = ty.boxed_ty()
				&& inner.is_sized(t.tcx, t.typing_env_pub()) =>
			{
				inner
			}
			_ => return ty,
		};
	}
}

/// Whether `clone_from` of `ty` clones, and drops what it writes over, as
/// `*self = source.clone()` would: `Clone`'s own, and std's own over such types, where it
/// only reuses memory, if nothing drops or it is an `Option`'s, or a box's of one. A
/// vector's, a deque's, and a heap's keep their buffer's capacity, which a clone's would not.
pub(super) fn clone_from_clones<'tcx>(t: &mut Translator<'_, 'tcx>, ty: Ty<'tcx>) -> bool {
	let tcx = t.tcx;
	let ty = unboxed(t, ty);
	let clone_from = super::collections::trait_fn(tcx, "Clone", "clone_from");
	let resolved = Instance::try_resolve(tcx, t.typing_env_pub(), clone_from, tcx.mk_args(&[ty.into()]));
	let Some(instance) = resolved.ok().flatten() else {
		return false;
	};
	if instance.def_id() == clone_from {
		return true;
	}
	let inner: Vec<Ty<'tcx>> = match (t.std_of(ty), ty.kind()) {
		(Some(Std::Vec | Std::VecDeque | Std::BinaryHeap), _) => return false,
		// A hash table's and a B-tree set's clone each element anew, by its `clone`. A `String`
		// keeps its buffer, as an owned `Cow<str>`'s does, and a `Box<str>` is a JS string.
		(Some(Std::HashMap | Std::HashSet | Std::HashTable | Std::BTreeSet), _) => vec![],
		(Some(Std::String), _) => return false,
		_ if cow_str(tcx, ty) => return false,
		_ if string(tcx, ty) => vec![],
		(_, ty::Adt(adt, args)) if is_std(tcx, adt.did()) => args.types().collect(),
		(_, ty::Array(elem, _)) => vec![*elem],
		_ => return false,
	};
	(t.std_of(ty) == Some(Std::Option) || !t.drops(ty)) && inner.into_iter().all(|arg| clone_from_clones(t, arg))
}

/// `concat` of a slice of `item`s, or with `join`, its `join` by `by`, a `&T` or a
/// `&[T]`, where each item borrows a slice of `T`s: a `Vec<T>` of clones of their elements
/// and the separator's between them, in order. `None` for an item with no slice.
fn slices_join<'tcx>(
	b: &mut Body<'_, '_, 'tcx>,
	item: Ty<'tcx>,
	by: Ty<'tcx>,
	join: bool,
	mut args: Vec<Expr>,
	span: Span,
) -> Option<Expr> {
	let sep = match join {
		true => args.pop()?,
		false => Expr::undefined(),
	};
	// A `&T` separator is a slice of one.
	let (elem, one) = match by.builtin_deref(true) {
		Some(by) if join => match by.kind() {
			ty::Slice(elem) => (*elem, false),
			_ => (by, true),
		},
		_ => (by, false),
	};
	let bytes = byte(elem);
	let sep = match (one, bytes) {
		(true, true) => Expr::rs(
			"byteSliceFrom",
			vec![Expr::ident("Uint8Array").member("of").call(vec![sep])],
		),
		// An array separator is the array slice of it alone, whose elements the join clones as
		// they are, so each `Clone` reads the separator's own.
		(true, false) if let ty::Array(_, len) = elem.kind() => {
			let n = len.try_to_target_usize(b.t.tcx).unwrap_or(0);
			let one = Expr::rs("arraySliceOne", vec![sep, Expr::lit(n.to_string())]);
			Expr::rs("arraySliceAsSlice", vec![one])
		}
		(true, false) => Expr::rs("sliceFrom", vec![Expr::Array(vec![sep])]),
		// An array slice, as the slice of its arrays.
		(false, false) if join && elem.is_array() => Expr::rs("arraySliceAsSlice", vec![sep]),
		(false, _) => sep,
	};
	let items = args.pop()?;
	// What reads an item as its slice, unless it is one.
	let rs = |f: &str| Some(Expr::Rs(f.into()));
	let slice = match item.kind() {
		ty::Array(..) => rs(if bytes { "byteSliceFrom" } else { "sliceFrom" }),
		// An array slice, as a slice of its arrays.
		ty::Ref(_, s, _) if super::array_slice::of_arrays(*s) => rs("arraySliceAsSlice"),
		ty::Ref(_, s, _) if s.is_slice() => None,
		_ if b.t.std_of(item) == Some(Std::Vec) => rs(if bytes { "byteVecSlice" } else { "vecSlice" }),
		_ if let Some(elem) = cow_slice(b.t.tcx, item)
			&& elem.is_array() =>
		{
			let runs = cow_deref(Expr::ident("c"), Ty::new_slice(b.t.tcx, elem));
			let param = crate::ts::Param {
				name: "c".into(),
				ty: None,
			};
			Some(Expr::Lambda(
				vec![param],
				Box::new(Expr::rs("arraySliceAsSlice", vec![runs])),
			))
		}
		_ if cow_slice(b.t.tcx, item).is_some() => rs(if bytes { "byteCowSlice" } else { "cowSlice" }),
		_ if box_slice(b.t.tcx, item).is_some_and(|elem| elem.is_array()) => rs("arraySliceAsSlice"),
		_ if box_slice(b.t.tcx, item).is_some() => None,
		_ => return None,
	};
	let slice = slice.unwrap_or_else(Expr::undefined);
	if bytes {
		return Some(Expr::rs("byteSlicesJoin", trimmed(vec![items, sep, slice])));
	}
	let size = super::iter::size_of(b, elem, span);
	let clone = super::collections::clone_fn(b, elem, span);
	Some(Expr::rs("slicesJoin", trimmed(vec![items, size, sep, slice, clone])))
}

/// `args` without the `undefined`s that end them, which a runtime function's optional
/// parameters take as left out.
fn trimmed(mut args: Vec<Expr>) -> Vec<Expr> {
	while matches!(args.last(), Some(Expr::Lit(l)) if l == "undefined") {
		args.pop();
	}
	args
}

/// `cow`, a `Cow<[elem]>`, through the runtime's `cow{op}`, `IntoOwned` or `Clone`, which
/// clones what it borrows, or for `Clone` what it owns, by each item's `Clone`, into a
/// `Vec` with room for exactly them.
pub fn cow_cloned<'tcx>(b: &mut Body<'_, '_, 'tcx>, op: &str, cow: Expr, elem: Ty<'tcx>, span: Span) -> Expr {
	if byte(elem) {
		return Expr::rs(format!("byteCow{op}"), vec![cow]);
	}
	let clone = super::collections::clone_fn(b, elem, span);
	Expr::rs(format!("cow{op}"), trimmed(vec![cow, clone]))
}

/// `to_vec` of `slice`, of `elem`s: a `Vec` of clones of them.
pub(super) fn slice_to_vec<'tcx>(b: &mut Body<'_, '_, 'tcx>, slice: Expr, elem: Ty<'tcx>, span: Span) -> Expr {
	match elem.kind() {
		_ if byte(elem) => Expr::rs("byteSliceToByteVec", vec![slice]),
		_ => Expr::rs("vecFrom", vec![items_clone(b, slice, elem, span)]),
	}
}

/// A clone of the elements of `slice`, an `rs.Slice`, as an array of their own: a copy of
/// them if each element's clone is itself, or else each element cloned.
fn items_clone<'tcx>(b: &mut Body<'_, '_, 'tcx>, slice: Expr, elem: Ty<'tcx>, span: Span) -> Expr {
	match super::collections::clone_fn(b, elem, span) {
		Expr::Lit(l) if l == "undefined" && elem.is_array() => Expr::rs("arraySliceCopy", vec![slice]),
		Expr::Lit(l) if l == "undefined" => Expr::rs("sliceToArray", vec![slice]),
		clone => items_of(slice, elem).method("map", vec![clone]),
	}
}

/// A clone of `value`, the JS array of a `Vec` of `elem` other than bytes, as a `Vec` with
/// room for exactly its elements, as std's allocates one.
fn vec_clone<'tcx>(b: &mut Body<'_, '_, 'tcx>, value: Expr, elem: Ty<'tcx>, span: Span) -> Expr {
	Expr::rs("vecFrom", vec![slice_clone(b, value, elem, span)])
}

/// A clone of `value`, the JS array of a `Vec` of `elem` other than bytes, as an array: a
/// copy of it if each element's clone is itself, or else each element cloned.
fn slice_clone<'tcx>(b: &mut Body<'_, '_, 'tcx>, value: Expr, elem: Ty<'tcx>, span: Span) -> Expr {
	match super::collections::clone_fn(b, elem, span) {
		Expr::Lit(l) if l == "undefined" => value.method("slice", Vec::new()),
		clone => value.method("map", vec![clone]),
	}
}

/// A `Box<[elem]>` of clones of the elements of `slice`, as std's `From<&[T]>` and `Clone`
/// make one.
fn slice_boxed<'tcx>(b: &mut Body<'_, '_, 'tcx>, slice: Expr, elem: Ty<'tcx>, span: Span) -> Option<Expr> {
	match elem.kind() {
		_ if byte(elem) => Some(Expr::rs(
			"byteSliceFrom",
			vec![Expr::rs("byteSliceToArray", vec![slice])],
		)),
		_ => Some(super::slice_items_from(items_clone(b, slice, elem, span), elem)),
	}
}

/// The `Box<[elem]>` that a `Vec<elem>`, `v`, becomes: its elements, as many as it holds.
pub(super) fn vec_boxed(v: Expr, elem: Ty<'_>) -> Expr {
	vec_view(v, elem)
}

/// The `Vec<elem>` that a `Box<[elem]>`, `boxed`, becomes, as `into_vec` makes it, with room
/// for exactly its elements.
fn boxed_vec(boxed: Expr, elem: Ty<'_>) -> Expr {
	match elem.kind() {
		_ if byte(elem) => Expr::rs("byteSliceToByteVec", vec![boxed]),
		_ => Expr::rs("vecFrom", vec![items_of(boxed, elem)]),
	}
}

/// The `Box<[elem]>` that `From<[elem; N]>` moves `array` into: its elements.
fn array_boxed(array: Expr, elem: Ty<'_>) -> Expr {
	super::slice_items_from(array, elem)
}

/// The operator a binary operator trait, `Add` or another, applies.
pub fn operator(owner: &str) -> Option<rustc_middle::mir::BinOp> {
	use rustc_middle::mir::BinOp;
	Some(match owner {
		"Add" => BinOp::Add,
		"Sub" => BinOp::Sub,
		"Mul" => BinOp::Mul,
		"Div" => BinOp::Div,
		"Rem" => BinOp::Rem,
		"BitAnd" => BinOp::BitAnd,
		"BitOr" => BinOp::BitOr,
		"BitXor" => BinOp::BitXor,
		"Shl" => BinOp::Shl,
		"Shr" => BinOp::Shr,
		_ => return None,
	})
}

/// How the operators of `ty` overflow and the integer it holds, if it is a `Wrapping` or a
/// `Saturating` of an integer.
pub fn overflowing<'tcx>(tcx: TyCtxt<'tcx>, ty: Ty<'tcx>) -> Option<(Overflow, Ty<'tcx>)> {
	let ty::Adt(adt, args) = ty.kind() else {
		return None;
	};
	let overflow = Std::of(tcx, adt.did())?.overflow()?;
	Int::of(args.type_at(0)).map(|_| (overflow, args.type_at(0)))
}

/// A `str` method's pattern as the runtime takes it, an `rs.Pattern`: a `char`'s text, a
/// string or a predicate as it is, or a predicate for an array or slice of chars. `None`
/// for any other pattern. A predicate behind references and boxes, a closure, a function,
/// or a `dyn FnMut`, is the predicate, as `called_closures` reads it.
fn pattern<'tcx>(t: &Translator<'_, 'tcx>, ty: Ty<'tcx>, p: Expr) -> Option<Expr> {
	let (called, _, _) = called_through(t, ty, None);
	match ty.kind() {
		ty::Char => Some(char_text(p)),
		_ if matches!(called.kind(), ty::Closure(..) | ty::FnDef(..) | ty::FnPtr(..)) || t.dyn_fn(called).is_some() => {
			Some(p)
		}
		ty::Ref(..) if is_text(t, ty) => Some(match owned_string(t.tcx, ty.peel_refs()) {
			true => p.member("v"),
			false => p,
		}),
		// Any of the chars of an array or slice.
		ty::Array(c, _) | ty::Slice(c) if c.is_char() => Some(Expr::rs("patternChars", vec![elements(p, ty)])),
		ty::Ref(_, inner, _) if matches!(inner.kind(), ty::Array(c, _) | ty::Slice(c) if c.is_char()) => {
			Some(Expr::rs("patternChars", vec![elements(p, ty)]))
		}
		_ => None,
	}
}

/// The name of std's searcher of a `str` method's pattern of type `ty`, which `Debug`
/// writes, as `pattern` reads it, or `None` for any other.
pub fn searcher<'tcx>(t: &Translator<'_, 'tcx>, ty: Ty<'tcx>) -> Option<&'static str> {
	let (called, _, _) = called_through(t, ty, None);
	Some(match ty.kind() {
		ty::Char => "CharSearcher",
		_ if matches!(called.kind(), ty::Closure(..) | ty::FnDef(..) | ty::FnPtr(..)) || t.dyn_fn(called).is_some() => {
			"CharPredicateSearcher"
		}
		ty::Ref(..) if is_text(t, ty) => "StrSearcher",
		ty::Array(c, _) if c.is_char() => "CharArraySearcher",
		ty::Ref(_, inner, _) => match inner.kind() {
			ty::Array(c, _) if c.is_char() => "CharArrayRefSearcher",
			ty::Slice(c) if c.is_char() => "CharSliceSearcher",
			_ => return None,
		},
		_ => return None,
	})
}

/// Whether `ty` is a `str` or a `String`, behind any references: a JS string.
fn is_text<'tcx>(t: &Translator<'_, 'tcx>, ty: Ty<'tcx>) -> bool {
	let ty = ty.peel_refs();
	ty.is_str() || t.std_of(ty) == Some(Std::String)
}

/// A `char`'s text: a literal for a constant, or `String.fromCodePoint(c)`.
pub fn char_text(c: Expr) -> Expr {
	if let Expr::Lit(n) = &c
		&& let Some(c) = n.parse().ok().and_then(char::from_u32)
	{
		return Expr::Str(c.to_string());
	}
	Expr::ident("String").member("fromCodePoint").call(vec![c])
}

/// How many references `ty` is through.
fn refs(mut ty: Ty<'_>) -> usize {
	let mut n = 0;
	while let ty::Ref(_, inner, _) = ty.kind() {
		ty = *inner;
		n += 1;
	}
	n
}

/// `rs.<of><Name>(args)`.
fn rt2(args: Vec<Expr>, of: &str, name: &str) -> Expr {
	Expr::rs_of(of, name).call(args)
}

/// What the runtime's functions of a std range type start with.
fn range_of(std: Std) -> Option<&'static str> {
	match std {
		Std::Range => Some("range"),
		Std::RangeInclusive => Some("rangeInclusive"),
		Std::RangeFrom => Some("rangeFrom"),
		Std::RangeTo | Std::RangeToInclusive => Some("rangeTo"),
		_ => None,
	}
}

/// The `Step` a range of `ty`s is made with: a `char`'s, which skips the surrogates, an
/// unsigned `NonZero`'s, which stops at either end where its integer's wraps, and a wide
/// integer's, which steps past 2^53, or any integer's where `every` integer needs it to
/// step within its type.
pub fn range_step<'tcx>(tcx: TyCtxt<'tcx>, ty: Ty<'tcx>, every: bool) -> Option<Expr> {
	if let ty::Adt(adt, args) = ty.kind()
		&& Std::of(tcx, adt.did()) == Some(Std::NonZero)
	{
		let int = Int::of(args.type_at(0)).filter(|int| !int.signed)?;
		let step = vec![int.func("step"), int.lit(int.max() as i128)];
		return (every || int.big()).then(|| Expr::rs("nonZeroStep", step));
	}
	match Int::of(ty) {
		Some(int) => (every || int.big()).then(|| int.func("step")),
		None => ty.is_char().then(|| Expr::Rs("charStep".into())),
	}
}

fn int_named(name: &str) -> Option<Int> {
	let (signed, bits, pointer) = match name {
		"u8" => (false, 8, false),
		"u16" => (false, 16, false),
		"u32" => (false, 32, false),
		"u64" => (false, 64, false),
		"u128" => (false, 128, false),
		"usize" => (false, 32, true),
		"i8" => (true, 8, false),
		"i16" => (true, 16, false),
		"i32" => (true, 32, false),
		"i64" => (true, 64, false),
		"i128" => (true, 128, false),
		"isize" => (true, 32, true),
		_ => return None,
	};
	Some(Int { bits, signed, pointer })
}

/// The runtime function of `u8`'s or `char`'s ASCII method `name`, which the two share:
/// std's char ones are its byte ones' for an ASCII char, and past ASCII none holds or
/// changes either. Their `make_ascii_*` write where the `&mut` borrows, as MIR lowers them.
fn ascii(name: &str) -> Option<String> {
	match name {
		"is_ascii"
		| "is_ascii_alphabetic"
		| "is_ascii_uppercase"
		| "is_ascii_lowercase"
		| "is_ascii_alphanumeric"
		| "is_ascii_digit"
		| "is_ascii_hexdigit"
		| "is_ascii_punctuation"
		| "is_ascii_graphic"
		| "is_ascii_whitespace"
		| "is_ascii_control"
		| "to_ascii_uppercase"
		| "to_ascii_lowercase"
		| "eq_ignore_ascii_case"
		| "escape_ascii" => Some(super::camel(name)),
		_ => None,
	}
}

/// The type whose inherent impl holds `def_id`.
fn impl_self(tcx: TyCtxt<'_>, def_id: DefId) -> Option<Ty<'_>> {
	let imp = tcx.inherent_impl_of_assoc(def_id)?;
	Some(tcx.type_of(imp).instantiate_identity().skip_norm_wip())
}

/// The integer methods whose runtime function serves every width, `intCheckedPow` for
/// `checked_pow`, given the type's width and sign after the method's arguments. A
/// `_signed` or `_unsigned` form is its plain one's, which takes any integer.
const SHARED: &[&str] = &[
	"abs",
	"abs_diff",
	"bit_width",
	"borrowing_sub",
	"carrying_add",
	"carrying_mul",
	"carrying_mul_add",
	"checked_abs",
	"checked_add",
	"checked_div",
	"checked_div_euclid",
	"checked_ilog",
	"checked_ilog10",
	"checked_ilog2",
	"checked_isqrt",
	"checked_neg",
	"checked_next_multiple_of",
	"checked_next_power_of_two",
	"checked_pow",
	"checked_rem",
	"checked_rem_euclid",
	"checked_shl",
	"checked_shr",
	"checked_signed_diff",
	"checked_sub",
	"count_ones",
	"count_zeros",
	"div_euclid",
	"funnel_shl",
	"funnel_shr",
	"highest_one",
	"ilog",
	"ilog10",
	"ilog2",
	"is_power_of_two",
	"isolate_highest_one",
	"isolate_lowest_one",
	"isqrt",
	"lowest_one",
	"midpoint",
	"next_multiple_of",
	"next_power_of_two",
	"overflowing_abs",
	"overflowing_add",
	"overflowing_div",
	"overflowing_div_euclid",
	"overflowing_mul",
	"overflowing_neg",
	"overflowing_pow",
	"overflowing_rem",
	"overflowing_rem_euclid",
	"overflowing_shl",
	"overflowing_shr",
	"overflowing_sub",
	"pow",
	"rem_euclid",
	"reverse_bits",
	"rotate_left",
	"rotate_right",
	"saturating_abs",
	"saturating_add",
	"saturating_pow",
	"saturating_sub",
	"signum",
	"strict_add",
	"strict_neg",
	"strict_sub",
	"swap_bytes",
	"trailing_ones",
	"unbounded_shl",
	"unbounded_shr",
	"unsigned_abs",
	"wrapping_abs",
	"wrapping_add",
	"wrapping_div_euclid",
	"wrapping_neg",
	"wrapping_next_power_of_two",
	"wrapping_pow",
	"wrapping_rem_euclid",
	"wrapping_shl",
	"wrapping_shr",
	"wrapping_sub",
];

/// Of [`SHARED`], what only an unsigned type has, which a signed one's of the same name,
/// where it has one, does otherwise.
const UNSIGNED: &[&str] = &[
	"bit_width",
	"borrowing_sub",
	"carrying_add",
	"carrying_mul",
	"carrying_mul_add",
	"checked_next_multiple_of",
	"checked_next_power_of_two",
	"checked_signed_diff",
	"funnel_shl",
	"funnel_shr",
	"is_power_of_two",
	"next_multiple_of",
	"next_power_of_two",
	"wrapping_next_power_of_two",
];

/// A call of `int`'s inherent method `name` with `args`, or of a `NonZero`'s, a
/// `Wrapping`'s, or a `Saturating`'s that is its integer's: `None` for one the runtime
/// lacks.
fn int_method<'tcx>(
	b: &mut Body<'_, '_, 'tcx>,
	call: &Call<'tcx>,
	int: Int,
	name: &str,
	mut args: Vec<Expr>,
) -> Option<Expr> {
	let tcx = b.t.tcx;
	let method = match name {
		"from_be_bytes" | "from_le_bytes" | "from_ne_bytes" => "fromBeBytes",
		"to_be_bytes" | "to_le_bytes" | "to_ne_bytes" => "toBeBytes",
		"leading_ones" => "leadingOnes",
		"leading_zeros" => "leadingZeros",
		"trailing_zeros" => "trailingZeros",
		"checked_add" => "checkedAdd",
		"checked_sub" => "checkedSub",
		"checked_mul" => "checkedMul",
		"saturating_add" => "saturatingAdd",
		"saturating_sub" => "saturatingSub",
		"saturating_mul" => "saturatingMul",
		// Only a signed one can overflow, at `MIN / -1`.
		"saturating_div" | "wrapping_div" if int.signed => &super::camel(name),
		"wrapping_rem" if int.signed => "wrappingRem",
		"saturating_div" | "wrapping_div" => "div",
		"wrapping_rem" => "rem",
		"saturating_neg" => return Some(int.method("saturatingSub", [vec![int.lit(0)], args].concat())),
		"wrapping_add" => "wrappingAdd",
		"wrapping_sub" => "wrappingSub",
		"wrapping_mul" => "wrappingMul",
		"div_ceil" => "divCeil",
		"is_multiple_of" => "isMultipleOf",
		_ if int.bits == 8
			&& !int.signed
			&& let Some(method) = ascii(name) =>
		{
			return Some(Expr::rs(method, args));
		}
		// The operator's, which panics with the message each of these does.
		"strict_add" => "add",
		"strict_sub" => "sub",
		"strict_mul" => "mul",
		"strict_div" => "div",
		"strict_rem" => "rem",
		"strict_shl" => "shl",
		"strict_shr" => "shr",
		// Its checked form, which aborts where it gives `None` as std's check of the
		// precondition does in a debug build, since Rust defines nothing past it.
		_ if let Some(op) = name.strip_prefix("unchecked_") => {
			let checked = match op {
				"add" | "sub" | "mul" | "neg" | "shl" | "shr" => {
					int_method(b, call, int, &format!("checked_{op}"), args)?
				}
				"funnel_shl" | "funnel_shr" if !int.signed => {
					args.extend([Expr::lit(int.bits.to_string()), Expr::lit("false")]);
					Expr::rs(super::camel(&format!("int_checked_{op}")), args)
				}
				_ => return None,
			};
			let check = Expr::Str(format!("{}::{name} cannot overflow", int.rs()));
			return Some(Expr::rs("precondition", vec![checked, check]));
		}
		// wasm32 is little-endian.
		"to_le" | "from_le" => return args.pop(),
		"max_value" => return Some(int.lit(int.max() as i128)),
		"min_value" => return Some(int.lit(int.min())),
		"is_positive" => return Some(int.compare(">", args.pop()?, int.lit(0))),
		"is_negative" => return Some(int.compare("<", args.pop()?, int.lit(0))),
		"cast_signed" | "cast_unsigned" => {
			let sig = b.t.sig(call.def_id, call.args);
			let (from, to) = (unwrapped(tcx, sig.inputs()[0]), unwrapped(tcx, sig.output()));
			let value = Operand {
				expr: args.pop()?,
				value: None,
			};
			return Some(num::cast(b, value, from, to, call.span));
		}
		_ => {
			let shared = match name {
				"to_be" | "from_be" => "swap_bytes",
				// Each panics as its plain form does.
				"strict_pow" | "strict_abs" | "strict_div_euclid" | "strict_rem_euclid" => &name["strict_".len()..],
				_ => name
					.strip_suffix("_signed")
					.or_else(|| name.strip_suffix("_unsigned"))
					.unwrap_or(name),
			};
			if !SHARED.contains(&shared) || (int.signed && UNSIGNED.contains(&shared)) {
				return None;
			}
			args.push(Expr::lit(int.bits.to_string()));
			args.push(Expr::lit(int.signed.to_string()));
			let e = Expr::rs(super::camel(&format!("int_{shared}")), args);
			// What gives another type's integer is any integer to the runtime.
			return Some(match shared {
				"abs_diff" | "unsigned_abs" | "checked_signed_diff" => {
					let output = b.t.sig(call.def_id, call.args).output();
					Expr::As(Box::new(e), b.t.ty(output, call.span))
				}
				_ => e,
			});
		}
	};
	// wasm32 is little-endian, so the native bytes are the little-endian ones: the
	// big-endian bytes, the other way around. The caller may still hold the array it
	// reads, so it reverses a copy; the bytes it makes are its own.
	let args = match name {
		"from_le_bytes" | "from_ne_bytes" => args
			.into_iter()
			.map(|a| a.method("slice", Vec::new()).method("reverse", Vec::new()))
			.collect(),
		_ => args,
	};
	let e = int.method(method, args);
	Some(match name {
		"to_le_bytes" | "to_ne_bytes" => e.method("reverse", Vec::new()),
		_ => e,
	})
}

/// What a JS value of `ty` is through references and `NonZero`, which is its integer.
fn unwrapped<'tcx>(tcx: TyCtxt<'tcx>, ty: Ty<'tcx>) -> Ty<'tcx> {
	let ty = ty.peel_refs();
	match ty.kind() {
		ty::Adt(adt, args) if Std::of(tcx, adt.did()) == Some(Std::NonZero) => args.type_at(0),
		_ => ty,
	}
}

/// The primitive JS compares a range's bounds of type `ty` as, through shared references
/// and a `NonZero`: not through a `&mut`, which may be an `rs.Place`.
fn compared<'tcx>(tcx: TyCtxt<'tcx>, mut ty: Ty<'tcx>) -> Option<Ty<'tcx>> {
	while let ty::Ref(_, inner, ty::Mutability::Not) = ty.kind() {
		ty = *inner;
	}
	let ty = (!ty.is_ref()).then(|| unwrapped(tcx, ty))?;
	primitive(ty).then_some(ty)
}

pub fn primitive(ty: Ty<'_>) -> bool {
	matches!(ty.kind(), ty::Bool | ty::Int(_) | ty::Uint(_) | ty::Char) || super::float::is(ty)
}

/// The prefix of the runtime's functions of a slice of `elem`: `byteSlice` of bytes, and
/// `arraySlice` of arrays.
fn slice_of_elem(elem: Ty<'_>) -> &'static str {
	match elem.kind() {
		_ if byte(elem) => "byteSlice",
		ty::Array(..) => "arraySlice",
		_ => "slice",
	}
}

/// A `Vec` of `elem`s as a slice: an `rs.ByteSlice` of a byte vector, or an `rs.Slice` of
/// all of an `rs.Vec`'s items.
pub fn vec_view(v: Expr, elem: Ty<'_>) -> Expr {
	match elem.kind() {
		_ if byte(elem) => Expr::rs("byteVecSlice", vec![v]),
		_ => super::slice_items_from(v.member("items"), elem),
	}
}

/// What holds a `Vec` of `elem`s' length: an `rs.ByteVec` itself, or an `rs.Vec`'s items.
fn vec_items(v: Expr, elem: Ty<'_>) -> Expr {
	match byte(elem) {
		true => v,
		false => v.member("items"),
	}
}

/// A new empty `Vec`, of bytes if `bytes`.
pub fn vec_new(bytes: bool) -> Expr {
	Expr::New(
		Box::new(Expr::Rs(if bytes { "ByteVec" } else { "Vec" }.into())),
		Vec::new(),
	)
}

/// The elements of a slice, array, or `Vec`, a `ty`, as a JS array or `Uint8Array`, for
/// what only reads them: a view of a byte slice's or byte vector's, the elements of any
/// other slice, and a `Vec`'s items, which the runtime's iterators and formatting index.
pub fn elements(v: Expr, ty: Ty<'_>) -> Expr {
	match ty.peel_refs().kind() {
		ty::Slice(elem) if byte(*elem) => Expr::rs("byteSliceView", vec![v]),
		// The arrays of a slice of arrays, as `rs.arraySliceItems` reads them.
		ty::Slice(elem) | ty::Array(elem, _) if elem.is_array() && (ty.is_ref() || ty.is_slice()) => items_of(v, *elem),
		ty::Slice(_) => slice_items(v),
		_ if super::bytes_ref(ty) => Expr::rs("byteSliceView", vec![v]),
		_ if super::array_ref(ty).is_some() => slice_items(v),
		ty::Adt(adt, args) if adt.is_box() => elements(v, args.type_at(0)),
		ty::Adt(_, args) if args.types().next().is_some_and(byte) => Expr::rs("byteVecView", vec![v]),
		// A `Vec`'s items.
		ty::Adt(..) => v.member("items"),
		_ => v,
	}
}

/// The elements of `s`, an `rs.Slice`, as a JS array, as `rs.sliceItems` reads them: all of
/// an array is the array.
pub fn slice_items(s: Expr) -> Expr {
	array_of(s, "sliceItems")
}

/// The elements of `s`, a slice of `elem`s, as [`slice_items`] reads them: of arrays, the
/// arrays of its arrays, as `rs.arraySliceItems` reads them.
pub fn items_of(s: Expr, elem: Ty<'_>) -> Expr {
	match elem.is_array() {
		true => array_of(s, "arraySliceItems"),
		false => slice_items(s),
	}
}

/// `rs.{read}` of `s`, an `rs.Slice`: the array of an `rs.sliceFrom`.
fn array_of(s: Expr, read: &str) -> Expr {
	match s {
		Expr::Call(f, mut args) if matches!(&*f, Expr::Rs(n) if n == "sliceFrom") && args.len() == 1 => {
			args.pop().unwrap()
		}
		// All of the arrays of a slice of arrays, which only a read takes as they are.
		Expr::Call(f, mut args) if matches!(&*f, Expr::Rs(n) if n == "arraySliceFrom") && read == "arraySliceItems" => {
			args.swap_remove(0)
		}
		s => Expr::rs(read, vec![s]),
	}
}

/// All of `items`, a JS array, as an `rs.Slice`, as `rs.sliceFrom` makes one: the elements
/// `rs.sliceItems` reads of a slice are that slice.
pub fn slice_from(items: Expr) -> Expr {
	match items {
		Expr::Call(f, mut args) if matches!(&*f, Expr::Rs(n) if n == "sliceItems") && args.len() == 1 => {
			args.pop().unwrap()
		}
		items => Expr::rs("sliceFrom", vec![items]),
	}
}

/// The slice of a slice, array, or `Vec`, a `ty`: an `rs.ByteSlice` of bytes, an `rs.ArraySlice`
/// of arrays, and an `rs.Slice` of other values, which the runtime's slice functions read
/// and write.
pub fn slice_of(v: Expr, ty: Ty<'_>) -> Expr {
	match ty.peel_refs().kind() {
		ty::Slice(_) => v,
		_ if super::array_ref(ty).is_some() => v,
		ty::Array(elem, _) => super::slice_items_from(v, *elem),
		ty::Adt(_, args) => vec_view(v, args.type_at(0)),
		_ => v,
	}
}

/// What std's sorts pick their small sorts, partitions, and scratch by, of an `elem`: its
/// size on wasm32 and whether it is `Freeze`.
pub fn sort_of<'tcx>(b: &mut Body<'_, '_, 'tcx>, elem: Ty<'tcx>, span: Span) -> [Expr; 2] {
	let freeze = elem.is_freeze(b.t.tcx, b.t.typing_env_pub());
	[super::iter::size_of(b, elem, span), Expr::lit(freeze.to_string())]
}

trait MethodCall {
	fn method(self, name: &str, args: Vec<Expr>) -> Expr;
}

impl MethodCall for Expr {
	fn method(self, name: &str, args: Vec<Expr>) -> Expr {
		self.member(name).call(args)
	}
}

/// `Default::default()` for a standard type, of its parts' defaults where it has parts.
pub fn default_value<'tcx>(b: &mut Body<'_, '_, 'tcx>, ty: Ty<'tcx>, span: Span, out: &mut Vec<Stmt>) -> Option<Expr> {
	let tcx = b.t.tcx;
	let part = |b: &mut Body<'_, '_, 'tcx>, ty: Ty<'tcx>, out: &mut Vec<Stmt>| {
		let f = super::collections::trait_fn(tcx, "Default", "default");
		b.call_def(f, tcx.mk_args(&[ty.into()]), Vec::new(), span, out)
	};
	Some(match ty.kind() {
		_ if nothing(tcx, ty) => Expr::undefined(),
		ty::Adt(adt, _) if empty_iter(tcx, adt.did()) => b.zero_sized_std(ty, Vec::new(), span).ok()?,
		// A list's iterators, over an empty list.
		ty::Adt(adt, _) if list_iter(tcx, adt.did()) => Expr::rs("listIter", vec![Expr::rs("listNew", Vec::new())]),
		ty::Tuple(parts) => Expr::Array(parts.iter().map(|t| part(b, t, out)).collect()),
		ty::Array(elem, n) => {
			let n = n.try_to_target_usize(tcx)?;
			match elem.kind() {
				_ if byte(*elem) => return Some(Expr::rs("bytes", vec![Expr::lit(n.to_string())])),
				// Whatever its element, which needn't have a default.
				_ if n == 0 => return Some(Expr::Array(Vec::new())),
				_ => {}
			}
			// Each element its own, as an atomic must be.
			let mut body = Vec::new();
			let item = part(b, *elem, &mut body);
			body.push(StmtKind::Return(Some(item.typed())).at(None));
			let item_ty = b.t.ty(*elem, span);
			Expr::ident("Array").member("from").call(vec![
				Expr::Object(vec![(crate::ts::Key::Name("length".into()), Expr::lit(n.to_string()))]),
				Expr::Arrow(Vec::new(), item_ty, body),
			])
		}
		ty::Bool => Expr::lit("false"),
		ty::Int(_) | ty::Uint(_) => Int::of(ty)?.lit(0),
		_ if let Some(float) = super::float::Float::of(ty) => float.lit(0.0),
		ty::Char => Expr::lit("0"),
		ty::Ref(_, inner, _) => match inner.kind() {
			ty::Slice(elem) if byte(*elem) => Expr::rs(
				"byteSliceFrom",
				vec![Expr::New(Box::new(Expr::ident("Uint8Array")), vec![Expr::lit("0")])],
			),
			ty::Slice(elem) => super::slice_items_from(Expr::Array(Vec::new()), *elem),
			ty::Str => Expr::Str(String::new()),
			_ => return None,
		},
		ty::Adt(adt, args) => match Std::of(tcx, adt.did())? {
			Std::Option => b.t.none(args.type_at(0)),
			Std::Vec => vec_new(byte(args.type_at(0))),
			Std::String => Expr::rs("stringNew", vec![]),
			// An owned empty `String` or `Vec`.
			Std::Cow if cow_str(tcx, ty) => Expr::rs("stringNew", vec![]),
			Std::Cow if let Some(elem) = cow_slice(tcx, ty) => vec_new(byte(elem)),
			Std::Duration => super::time::zero(),
			Std::Atomic => Expr::New(
				Box::new(Expr::Rs("Atomic".into())),
				vec![default_value(b, args.type_at(0), span, out)?],
			),
			Std::OnceCell | Std::OnceLock => super::task::once_new(b, ty),
			// An empty slice or text, its own, or an `Arc`'s.
			std @ (Std::Box | Std::Arc) if !args.type_at(0).is_sized(tcx, b.t.typing_env_pub()) => {
				let empty = default_value(
					b,
					Ty::new_imm_ref(tcx, tcx.lifetimes.re_erased, args.type_at(0)),
					span,
					out,
				)?;
				match std {
					Std::Arc => Expr::New(Box::new(Expr::Rs("Arc".into())), vec![empty]),
					_ => empty,
				}
			}
			Std::Box | Std::Wrapping | Std::Saturating => part(b, args.type_at(0), out),
			Std::Arc => Expr::New(Box::new(Expr::Rs("Arc".into())), vec![part(b, args.type_at(0), out)]),
			Std::Mutex => Expr::New(Box::new(Expr::Rs("Mutex".into())), vec![part(b, args.type_at(0), out)]),
			Std::RefCell => Expr::New(
				Box::new(Expr::Rs("RefCell".into())),
				vec![part(b, args.type_at(0), out)],
			),
			Std::Cell => Expr::New(Box::new(Expr::Rs("Cell".into())), vec![part(b, args.type_at(0), out)]),
			Std::Weak => Expr::New(Box::new(Expr::Rs("Weak".into())), Vec::new()),
			_ => return None,
		},
		_ => return None,
	})
}

/// The text of a `fmt::Arguments` with only literal pieces, which writes it as is.
pub fn literal(parts: &Expr) -> Option<String> {
	let Expr::Array(parts) = parts else {
		return None;
	};
	parts
		.iter()
		.map(|part| match part {
			Expr::Str(s) => Some(s.as_str()),
			_ => None,
		})
		.collect()
}
