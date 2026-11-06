//! Addresses read as integers: `p as usize`, `addr`, and `expose_provenance`.
//!
//! Rust promises of an allocation's address only that it is aligned and that no other
//! live allocation overlaps it, so `rs.addr` numbers each object as an allocator would
//! place it, on its first read, while JS keeps the object. A pointer's address is then
//! exact wherever its identity is, as [`Translator::unidentified`] decides for comparing
//! pointers, with one more condition: Rust fixes how far apart two values in one
//! allocation are, as the fields of a struct or the elements of a `Vec` are, which the
//! objects JS keeps apart can't show. So an address is read only of a type no allocation
//! holds beside another whose address is read, nor holds many of. Nothing makes a pointer
//! of an address, which JS has no memory at.

use rustc_abi::{FieldsShape, Variants};
use rustc_data_structures::fx::{FxHashMap, FxHashSet};
use rustc_middle::ty::layout::{LayoutCx, TyAndLayout};
use rustc_middle::ty::{self, Ty, TyCtxt, TypeVisitableExt};
use rustc_span::Span;

use super::Translator;
use super::body::Body;
use super::library::Std;
use super::pointer::Pointee;
use crate::ts::Expr;

/// The address of `p`, a pointer to `ty`, as a `usize`.
pub fn address<'tcx>(b: &mut Body<'_, '_, 'tcx>, p: Expr, ty: Ty<'tcx>, span: Span) -> Expr {
	match read(b, ty, span) {
		Ok(layout) => Expr::rs("addr", [p].into_iter().chain(layout).collect()),
		Err(refused) => refused,
	}
}

/// The size and alignment `rs.addr` reads the address of a pointer to `ty` by, with the
/// alignment of where the program keeps one, or the
/// refusal of a type whose address JS has no counterpart of.
pub fn read<'tcx>(b: &mut Body<'_, '_, 'tcx>, ty: Ty<'tcx>, span: Span) -> Result<[Expr; 3], Expr> {
	let (tcx, env) = (b.t.tcx, b.t.typing_env_pub());
	// Only an instance decides what a parameter is, and is refused on its own.
	if !(b.t.polymorphic() && ty.has_param()) {
		let why = match b.t.pointee(ty) {
			_ if b.t.zero_sized(ty) => Some("which has no bytes and so no address of its own"),
			Pointee::Array => Some("a view JS makes of its own of an array that may be in a larger one"),
			_ if b.t.erased(ty) != ty => Some("which JS keeps as what it holds"),
			_ => b.t.unidentified(ty),
		};
		let what = format!("the address of a pointer to `{ty}`");
		if let Some(why) = why {
			return Err(b.err(span, format!("{what}, {why}")));
		}
		let read = b.t.sited(span, what.clone());
		b.t.addresses.push((ty, read.0, read.1));
		let layout = tcx.layout_of(env.as_query_input(ty)).ok();
		let size = layout.map_or(0, |l| l.size.bytes());
		let own = layout.map_or(1, |l| l.align.abi.bytes());
		return match b.t.alignment(ty) {
			Ok(kept) => Ok([size, own, kept].map(|n| Expr::lit(n.to_string()))),
			Err(holder) => Err(b.err(
				span,
				format!(
					"{what}, which `{holder}` holds at an offset its alignment fixes the address of, which JS can't show"
				),
			)),
		};
	}
	let layout = tcx.layout_of(env.as_query_input(ty)).ok();
	let size = layout.map_or(0, |l| l.size.bytes());
	let align = layout.map_or(1, |l| l.align.abi.bytes());
	Ok([size, align, align].map(|n| Expr::lit(n.to_string())))
}

impl<'a, 'tcx> Translator<'a, 'tcx> {
	/// Note `Arc::new_cyclic` of a `ty`, whose value JS has none of until its closure
	/// returns, so a `Weak::as_ptr` of one has nothing to point at.
	pub fn cyclic(&mut self, ty: Ty<'tcx>) {
		self.cyclic.insert(self.tcx.erase_and_anonymize_regions(ty));
	}

	/// Note `Weak::as_ptr` of a `Weak<ty>` at `span`, refused once translation settles if
	/// the crate makes a `ty` by `Arc::new_cyclic`.
	pub fn weak_pointer(&mut self, ty: Ty<'tcx>, span: Span) {
		let msg = format!(
			"`Weak::as_ptr` of a `Weak<{ty}>`, which `Arc::new_cyclic` makes and JS has no value of until its closure returns"
		);
		let (span, msg) = self.sited(span, msg);
		self.weak_pointers
			.push((self.tcx.erase_and_anonymize_regions(ty), span, msg));
	}

	/// The alignment `rs.addr` numbers the addresses of a `ty` by: the largest of its own
	/// and each allocation's that holds one at an offset that alignment keeps, as an
	/// `Arc`'s counts before its value do, or else the type of an allocation that holds one
	/// where its alignment fixes the address apart from it.
	fn alignment(&mut self, ty: Ty<'tcx>) -> Result<u64, Ty<'tcx>> {
		let tcx = self.tcx;
		let ty = super::copy::storage(tcx, ty);
		if let Some(&found) = self.alignments.get(&ty) {
			return found;
		}
		let env = ty::TypingEnv::fully_monomorphized();
		let own = tcx.layout_of(env.as_query_input(ty)).map_or(1, |l| l.align.abi.bytes());
		let mut placed = Placed {
			tcx,
			cx: LayoutCx::new(tcx, env),
			target: ty,
			own,
			align: own,
			roots: self.writes.locals(tcx).map(|t| (t, None)).collect(),
			seen: FxHashSet::default(),
		};
		let found = placed.run();
		self.alignments.insert(ty, found);
		found
	}

	/// Refuse each address read of a type an allocation holds beside another whose address
	/// is read, or many of, and each `Weak::as_ptr` of a value `Arc::new_cyclic` makes.
	pub(super) fn refuse_addresses(&mut self) {
		for (ty, span, msg) in std::mem::take(&mut self.weak_pointers) {
			if self.cyclic.contains(&ty) {
				self.refuse(span, msg);
			}
		}
		let reads = std::mem::take(&mut self.addresses);
		if reads.is_empty() {
			return;
		}
		let tcx = self.tcx;
		let addressed: FxHashSet<Ty<'tcx>> = reads.iter().map(|&(ty, ..)| super::copy::storage(tcx, ty)).collect();
		let shared = shared(tcx, &addressed, self.writes.locals(tcx));
		for (ty, span, msg) in reads {
			if let Some(&(holder, many)) = shared.get(&super::copy::storage(tcx, ty)) {
				let held = match many {
					true => "many of",
					false => "beside another value whose address is read,",
				};
				let why = format!(
					"{msg}, which `{holder}` holds {held} in one allocation, as far apart as Rust fixes and JS can't show"
				);
				self.refuse(span, why);
			}
		}
	}
}

/// Each of `addressed` that an allocation holds beside another, or many of, by the type
/// it holds them in and whether it holds many, of the allocations `locals` hold or point at.
fn shared<'tcx>(
	tcx: TyCtxt<'tcx>,
	addressed: &FxHashSet<Ty<'tcx>>,
	locals: impl Iterator<Item = Ty<'tcx>>,
) -> FxHashMap<Ty<'tcx>, (Ty<'tcx>, bool)> {
	let mut held = Held::new(tcx, addressed);
	held.roots.extend(locals.map(|ty| (ty, false)));
	let mut seen = FxHashSet::default();
	let mut shared = FxHashMap::default();
	while let Some((ty, many)) = held.roots.pop() {
		if !seen.insert((ty, many)) {
			continue;
		}
		let n = held.count(ty);
		if n < 2 && !(many && n == 1) {
			continue;
		}
		for &each in addressed {
			let one = FxHashSet::from_iter([each]);
			if Held::new(tcx, &one).count(ty) > 0 {
				shared
					.entry(each)
					.or_insert((ty, many || matches!(ty.kind(), ty::Array(..) | ty::Slice(_))));
			}
		}
	}
	shared
}

/// How many values of the `addressed` types a value holds, and the allocations it points
/// at, each with whether it may hold many.
struct Held<'s, 'tcx> {
	tcx: TyCtxt<'tcx>,
	addressed: &'s FxHashSet<Ty<'tcx>>,
	counts: FxHashMap<Ty<'tcx>, u8>,
	roots: Vec<(Ty<'tcx>, bool)>,
}

impl<'s, 'tcx> Held<'s, 'tcx> {
	fn new(tcx: TyCtxt<'tcx>, addressed: &'s FxHashSet<Ty<'tcx>>) -> Self {
		Held {
			tcx,
			addressed,
			counts: FxHashMap::default(),
			roots: Vec::new(),
		}
	}

	/// How many values of the addressed types a `ty` holds by value, up to 2, which is
	/// many. A std type holds its type arguments as its kind says, and the crate's own
	/// raw pointers each point at one value, as no arithmetic moves one to an object.
	fn count(&mut self, ty: Ty<'tcx>) -> u8 {
		let tcx = self.tcx;
		let env = ty::TypingEnv::fully_monomorphized();
		let ty = tcx
			.try_normalize_erasing_regions(env, ty::Unnormalized::new_wip(ty))
			.unwrap_or(ty);
		if let Some(&n) = self.counts.get(&ty) {
			return n;
		}
		self.counts.insert(ty, 0);
		let own = u8::from(self.addressed.contains(&super::copy::storage(tcx, ty)));
		let parts = match *ty.kind() {
			ty::Array(elem, len) => match (len.try_to_target_usize(tcx), self.count(elem)) {
				(Some(0), _) | (_, 0) => 0,
				(Some(1), n) => n,
				_ => 2,
			},
			ty::Slice(elem) => 2 * self.count(elem).min(1),
			ty::Pat(elem, _) => self.count(elem),
			ty::Tuple(tys) => total(tys.iter().map(|t| self.count(t))),
			ty::Ref(_, pointee, _) | ty::RawPtr(pointee, _) => self.point(pointee, false),
			ty::Adt(adt, args) if super::library::is_std(tcx, adt.did()) => match Std::of(tcx, adt.did()) {
				// An allocation of one value, or a pointer to one.
				Some(Std::Box | Std::Arc | Std::Weak | Std::NonNull) => self.point(args.type_at(0), false),
				_ if adt.is_phantom_data() => 0,
				Some(Std::Range | Std::RangeInclusive) => 2 * self.count(args.type_at(0)).min(1),
				// Each of its type arguments, at most one of each, by value.
				_ if by_value(tcx, adt) => total(args.types().map(|t| self.count(t))),
				// A collection, or anything else of std's, may hold many of each in one
				// allocation of its own.
				_ => {
					self.roots.extend(args.types().map(|t| (t, true)));
					0
				}
			},
			ty::Adt(adt, args) => {
				let mut counts = adt.variants().iter().map(|v| {
					v.fields
						.iter()
						.map(|f| self.count(f.ty(tcx, args).skip_norm_wip()))
						.collect::<Vec<u8>>()
				});
				match adt.is_union() {
					// Its fields overlap, as an enum's variants do.
					true => counts.next().into_iter().flatten().max().unwrap_or(0),
					false => counts.map(|c| total(c.into_iter())).max().unwrap_or(0),
				}
			}
			ty::Closure(_, args) => total(args.as_closure().upvar_tys().iter().map(|t| self.count(t))),
			ty::CoroutineClosure(_, args) => {
				total(args.as_coroutine_closure().upvar_tys().iter().map(|t| self.count(t)))
			}
			// Its captures, and the locals it keeps across an `.await`.
			ty::Coroutine(def_id, args) => {
				let upvars = total(args.as_coroutine().upvar_tys().iter().map(|t| self.count(t)));
				let saved = match tcx.coroutine_layout(def_id, args) {
					Ok(layout) => total(layout.field_tys.iter().map(|f| {
						let saved = ty::EarlyBinder::bind(tcx, f.ty);
						self.count(tcx.instantiate_and_normalize_erasing_regions(args, env, saved))
					})),
					Err(_) => 0,
				};
				upvars.saturating_add(saved)
			}
			_ => 0,
		};
		let n = own.saturating_add(parts).min(2);
		self.counts.insert(ty, n);
		n
	}

	/// A pointer to `pointee`, an allocation of its own, which holds nothing beside the
	/// pointer: of many where `many` says it may be.
	fn point(&mut self, pointee: Ty<'tcx>, many: bool) -> u8 {
		self.roots.push((pointee, many));
		0
	}
}

/// Where the program's allocations hold values of the `target` type: at what offset
/// from where each starts, aligned as its type is, or as an `Arc`'s allocation is.
struct Placed<'tcx> {
	tcx: TyCtxt<'tcx>,
	cx: LayoutCx<'tcx>,
	target: Ty<'tcx>,
	/// The target's own alignment.
	own: u64,
	/// The largest alignment of an allocation holding one at an offset it keeps.
	align: u64,
	/// The allocations still to look in, each with the offset of its value and the
	/// alignment of the allocation, if it holds more before it, as an `Arc`'s does.
	roots: Vec<(Ty<'tcx>, Option<(u64, u64)>)>,
	seen: FxHashSet<(Ty<'tcx>, Option<(u64, u64)>)>,
}

impl<'tcx> Placed<'tcx> {
	fn run(&mut self) -> Result<u64, Ty<'tcx>> {
		let env = ty::TypingEnv::fully_monomorphized();
		while let Some((ty, header)) = self.roots.pop() {
			let ty = (self.tcx)
				.try_normalize_erasing_regions(env, ty::Unnormalized::new_wip(ty))
				.unwrap_or(ty);
			if !self.seen.insert((ty, header)) {
				continue;
			}
			// A slice's elements are each at a multiple of their own alignment.
			let ty = match ty.kind() {
				ty::Slice(elem) => *elem,
				ty::Str | ty::Dynamic(..) => continue,
				_ => ty,
			};
			let Ok(layout) = self.tcx.layout_of(env.as_query_input(ty)) else {
				continue;
			};
			let (offset, align) = header.unwrap_or((0, layout.align.abi.bytes()));
			self.visit(layout, offset, align, ty)?;
		}
		Ok(self.align)
	}

	/// Look for the target in `layout`, `offset` bytes into an allocation of `holder`
	/// aligned to `align`.
	fn visit(&mut self, layout: TyAndLayout<'tcx>, offset: u64, align: u64, holder: Ty<'tcx>) -> Result<(), Ty<'tcx>> {
		let tcx = self.tcx;
		if super::copy::storage(tcx, layout.ty) == self.target && align > self.own {
			match offset % align {
				0 => self.align = self.align.max(align),
				_ => return Err(holder),
			}
		}
		match *layout.ty.kind() {
			ty::Ref(_, pointee, _) | ty::RawPtr(pointee, _) => self.roots.push((pointee, None)),
			ty::Adt(adt, args) if super::library::is_std(tcx, adt.did()) => match Std::of(tcx, adt.did()) {
				Some(Std::Box | Std::NonNull) => self.roots.push((args.type_at(0), None)),
				// The value after the counts, at an offset of its own alignment.
				Some(Std::Arc | Std::Weak) => {
					let value = args.type_at(0);
					let env = ty::TypingEnv::fully_monomorphized();
					if let Ok(inner) = tcx.layout_of(env.as_query_input(value)) {
						let pointer = tcx.data_layout.pointer_size().bytes();
						let align = inner.align.abi.bytes();
						let at = (2 * pointer).div_ceil(align) * align;
						let whole = align.max(tcx.data_layout.pointer_align().abi.bytes());
						self.roots.push((value, Some((at, whole))));
					}
				}
				_ if adt.is_phantom_data() => {}
				_ if by_value(tcx, adt) => self.fields(layout, offset, align, holder)?,
				_ => self.roots.extend(args.types().map(|t| (t, None))),
			},
			ty::Coroutine(..) => {
				let held = super::copy::by_value(tcx, ty::TypingEnv::fully_monomorphized(), layout.ty)
					.any(|part| super::copy::storage(tcx, part) == self.target);
				if held && layout.align.abi.bytes() > self.own {
					return Err(layout.ty);
				}
				self.fields(layout, offset, align, holder)?;
			}
			ty::Adt(..) | ty::Tuple(_) | ty::Array(..) | ty::Closure(..) | ty::CoroutineClosure(..) | ty::Pat(..) => {
				self.fields(layout, offset, align, holder)?
			}
			_ => {}
		}
		Ok(())
	}

	/// Look in each field of each variant of `layout`, and in the first two elements of an
	/// array, whose others are where those two say.
	fn fields(&mut self, layout: TyAndLayout<'tcx>, offset: u64, align: u64, holder: Ty<'tcx>) -> Result<(), Ty<'tcx>> {
		let variants = match &layout.variants {
			Variants::Multiple { variants, .. } => {
				variants.indices().map(|v| layout.for_variant(&self.cx, v)).collect()
			}
			Variants::Single { .. } => vec![layout],
			Variants::Empty => Vec::new(),
		};
		for variant in variants {
			let count = match variant.fields {
				FieldsShape::Array { count, .. } => count.min(2) as usize,
				_ => variant.fields.count(),
			};
			for i in 0..count {
				let field = variant.field(&self.cx, i);
				self.visit(field, offset + variant.fields.offset(i).bytes(), align, holder)?;
			}
		}
		Ok(())
	}
}

/// Whether std's `adt` holds each of its type arguments by value, at most one of each, as
/// [`Held::count`] counts them.
fn by_value(tcx: TyCtxt<'_>, adt: ty::AdtDef<'_>) -> bool {
	adt.is_unsafe_cell()
		|| matches!(
			Std::of(tcx, adt.did()),
			Some(
				Std::Option
					| Std::Result
					| Std::Cow
					| Std::Cell
					| Std::RefCell
					| Std::Mutex
					| Std::OnceCell
					| Std::OnceLock
					| Std::LazyCell
					| Std::LazyLock
					| Std::Poll
					| Std::Ready
					| Std::Pin
					| Std::ControlFlow
					| Std::Bound
					| Std::Range
					| Std::RangeInclusive
					| Std::RangeTo
					| Std::RangeFrom
					| Std::RangeToInclusive
					| Std::Reverse
					| Std::Wrapping
					| Std::Saturating
					| Std::AssertUnwindSafe
					| Std::MaybeUninit
					| Std::ManuallyDrop
			)
		)
}

/// The sum of `counts`, up to many.
fn total(counts: impl Iterator<Item = u8>) -> u8 {
	counts.fold(0, u8::saturating_add)
}
