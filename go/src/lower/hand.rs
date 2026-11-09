//! Handing a `Vec<u8>`'s storage between Go code and the translation. The runtime grows
//! storage it owns without zeroing it and recycles it once outgrown, telling it apart by
//! its shape (`bytesOwn` in `runtime/rt.go`). Go code reaches storage only through a root
//! function, so the root hands Go's storage in off that shape, and its own out clipped to
//! its length. In, it also copies each container an argument owns, a `Box`'s pointee or a
//! `Vec`'s elements, by value or behind `&mut`, so the translation never writes storage of
//! its own where Go code may still hold the container, which a move, a grow, or a
//! replaced `Box` would leave behind. Out, it clips its result and what its `&mut`
//! arguments reach, the places Go code keeps. A root refuses what would let a place Go
//! holds leave those walks, or see what handing in copied (`refuse_unhandable`).

use rustc_hir::def_id::LOCAL_CRATE;
use rustc_middle::ty::util::IntTypeExt;
use rustc_middle::ty::{self, AdtDef, Ty, TyCtxt, TypingEnv, Unnormalized};
use rustc_span::Span;

use super::body::int_bits;
use super::library::{self, Mapped};
use super::ty::variant_field;
use super::{Translator, names};

/// Which way a walk hands storage, and so what it does with each buffer it reaches.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Hand {
	/// Into a root: each `Vec<u8>` of Go's is taken off the shape of own storage, and each
	/// container holding one is copied.
	In,
	/// Out of a root, in its result or behind a `&mut` argument: each `Vec<u8>` of own
	/// storage, and each byte slice, is clipped to its length.
	Out,
}

fn is_u8(ty: Ty<'_>) -> bool {
	matches!(ty.kind(), ty::Uint(ty::UintTy::U8))
}

/// Whether a walk this way goes behind a pointer, which is `mutable` if the translation
/// may write through it: handing in, only where the translation may grow or replace what
/// Go passed; handing out, behind any, since Go code gets what the result reaches once
/// the call is done, shared or not.
fn reaches(mutable: bool, hand: Hand) -> bool {
	mutable || hand == Hand::Out
}

impl<'tcx> Translator<'tcx> {
	/// Whether a walk handing a value of `ty` this way has anything to do.
	pub fn hands(&self, ty: Ty<'tcx>, hand: Hand) -> bool {
		self.hands_in(ty, hand, &mut Vec::new())
	}

	/// `hands` of `ty` inside the walks in `seen`: one met again, as a recursive type's
	/// is, adds nothing to its first.
	fn hands_in(&self, ty: Ty<'tcx>, hand: Hand, seen: &mut Vec<(Ty<'tcx>, Hand)>) -> bool {
		if seen.contains(&(ty, hand)) {
			return false;
		}
		seen.push((ty, hand));
		let tcx = self.tcx;
		let found = match *ty.kind() {
			ty::Adt(adt, args) => match library::mapped(tcx, adt.did()) {
				Some(Mapped::Vec) => is_u8(args.type_at(0)) || self.hands_in(args.type_at(0), hand, seen),
				Some(Mapped::Box) => self.hands_behind(args.type_at(0), true, hand, seen),
				// What it holds is shared, so only a result's is handed.
				Some(Mapped::SliceIter) => {
					hand == Hand::Out && self.hands_behind(self.iter_slice(args), false, hand, seen)
				}
				Some(Mapped::Transparent) => self.hands_in(args.type_at(0), hand, seen),
				Some(Mapped::Option | Mapped::Result) | None => adt
					.all_fields()
					.any(|field| self.hands_in(self.mono_field_ty(field, args), hand, seen)),
				Some(Mapped::Runtime(_) | Mapped::Refused(_)) => false,
			},
			ty::Ref(_, inner, m) => self.hands_behind(inner, m.is_mut(), hand, seen),
			ty::Tuple(tys) => tys.iter().any(|t| self.hands_in(t, hand, seen)),
			ty::Array(elem, _) => self.hands_in(elem, hand, seen),
			ty::Closure(_, args) => args
				.as_closure()
				.upvar_tys()
				.iter()
				.any(|t| self.hands_in(t, hand, seen)),
			_ => false,
		};
		seen.pop();
		found
	}

	/// `hands` of a pointer to `inner`, through which the translation may write if `mutable`.
	fn hands_behind(&self, inner: Ty<'tcx>, mutable: bool, hand: Hand, seen: &mut Vec<(Ty<'tcx>, Hand)>) -> bool {
		let reach = reaches(mutable, hand);
		match *inner.kind() {
			ty::Str => hand == Hand::Out,
			ty::Slice(elem) => (hand == Hand::Out && is_u8(elem)) || (reach && self.hands_in(elem, hand, seen)),
			ty::Dynamic(..) => false,
			_ => reach && self.hands_in(inner, hand, seen),
		}
	}

	/// Whether a root hands back to Go what its argument of `ty` reaches: what lies behind
	/// a `&mut`, the only place of Go's the translation can write, since a reference
	/// anywhere else in an argument is refused. A slice or `str` it points to is the
	/// root's own copy of the slice, so only its elements are Go's.
	pub fn hands_back(&self, ty: Ty<'tcx>) -> bool {
		let ty::Ref(_, inner, ty::Mutability::Mut) = *ty.kind() else {
			return false;
		};
		match *inner.kind() {
			ty::Str => false,
			ty::Slice(elem) => self.hands(elem, Hand::Out),
			_ => self.hands(inner, Hand::Out),
		}
	}

	/// The slice a `slice::Iter` with `args` walks, whose first is the slice's lifetime.
	fn iter_slice(&self, args: ty::GenericArgsRef<'tcx>) -> Ty<'tcx> {
		let elem = args.types().next().expect("a slice iterator's element type");
		Ty::new_slice(self.tcx, elem)
	}

	/// A field's type at `args`.
	fn mono_field_ty(&self, field: &ty::FieldDef, args: ty::GenericArgsRef<'tcx>) -> Ty<'tcx> {
		let fty = field.ty(self.tcx, args).skip_norm_wip();
		self.tcx
			.normalize_erasing_regions(TypingEnv::fully_monomorphized(), Unnormalized::new_wip(fty))
	}

	/// The Go function that walks a value of `ty` behind a pointer, handing its storage
	/// this way, declared if new. Only for a `ty` that `hands`.
	pub fn hand(&mut self, ty: Ty<'tcx>, hand: Hand, span: Span) -> String {
		if let Some(name) = self.hand_fns.get(&(ty, hand)) {
			return name.clone();
		}
		let tcx = self.tcx;
		let way = match hand {
			Hand::In => "In",
			Hand::Out => "Out",
		};
		let name = self.fresh(&format!("hand{way}_{}", names::ty_short(tcx, ty)));
		self.hand_fns.insert((ty, hand), name.clone());
		let go = self.ty(ty, span);
		let mut body = Vec::new();
		match *ty.kind() {
			ty::Adt(adt, args) => match library::mapped(tcx, adt.did()) {
				Some(Mapped::Vec) => {
					let elem = args.type_at(0);
					if is_u8(elem) {
						body.push(match hand {
							Hand::In => "bytesEnter(p)".into(),
							_ => "bytesEscape(p)".into(),
						});
					} else {
						if hand == Hand::In {
							body.push("*p = append((*p)[:0:0], *p...)".into());
						}
						let f = self.hand(elem, hand, span);
						body.push(format!("for i := range *p {{ {f}(&(*p)[i]) }}"));
					}
				}
				Some(Mapped::Box) => self.hand_behind(args.type_at(0), true, true, hand, "p", &mut body, span),
				Some(Mapped::SliceIter) => {
					let slice = self.iter_slice(args);
					self.hand_behind(slice, false, false, hand, "&p.s", &mut body, span)
				}
				Some(Mapped::Transparent) => {
					let f = self.hand(args.type_at(0), hand, span);
					body.push(format!("{f}(p)"));
				}
				_ => self.hand_adt(adt, args, hand, &mut body, span),
			},
			ty::Ref(_, inner, m) => self.hand_behind(inner, m.is_mut(), false, hand, "p", &mut body, span),
			ty::Tuple(tys) => {
				for (i, t) in tys.iter().enumerate() {
					if self.hands(t, hand) {
						let f = self.hand(t, hand, span);
						body.push(format!("{f}(&p.F{i})"));
					}
				}
			}
			ty::Array(elem, _) => {
				let f = self.hand(elem, hand, span);
				body.push(format!("for i := range p {{ {f}(&p[i]) }}"));
			}
			ty::Closure(_, args) => {
				for (i, t) in args.as_closure().upvar_tys().iter().enumerate() {
					if self.hands(t, hand) {
						let f = self.hand(t, hand, span);
						body.push(format!("{f}(&p.F{i})"));
					}
				}
			}
			_ => unreachable!("a walk of `{ty}`, which hands nothing"),
		}
		let what = match hand {
			Hand::In => "into a root function",
			Hand::Out => "out of a root function",
		};
		let text = format!(
			"// {name} hands the storage of a `{ty}` {what}.\nfunc {name}(p *{go}) {{\n\t{}\n}}\n",
			body.join("\n\t")
		);
		self.out
			.entry(LOCAL_CRATE)
			.or_default()
			.funcs
			.push((name.clone(), text));
		name
	}

	/// The walk of a pointer to `inner`, at `p`, a pointer to the pointer, or to the
	/// slice of an unsized `inner`, which a `Box` `owns` and so copies in.
	#[expect(clippy::too_many_arguments, reason = "the walk's position, as hands_behind takes it")]
	fn hand_behind(
		&mut self,
		inner: Ty<'tcx>,
		mutable: bool,
		owns: bool,
		hand: Hand,
		p: &str,
		body: &mut Vec<String>,
		span: Span,
	) {
		let copy = owns && hand == Hand::In;
		match *inner.kind() {
			ty::Str => body.push(format!("clip({p})")),
			ty::Slice(elem) => {
				if hand == Hand::Out && is_u8(elem) {
					body.push(format!("clip({p})"));
				}
				if reaches(mutable, hand) && self.hands(elem, hand) {
					if copy {
						body.push(format!("*{p} = append((*{p})[:0:0], *{p}...)"));
					}
					let f = self.hand(elem, hand, span);
					body.push(format!("for i := range *{p} {{ {f}(&(*{p})[i]) }}"));
				}
			}
			_ => {
				let f = self.hand(inner, hand, span);
				match copy {
					true => body.push(format!("if *{p} != nil {{ c := **{p}; *{p} = &c; {f}(&c) }}")),
					false => body.push(format!("if *{p} != nil {{ {f}(*{p}) }}")),
				}
			}
		}
	}

	/// The walk of a struct's fields, or of an enum's variant it holds. Handing out also
	/// zeroes the variants it does not hold, whose fields may be what was moved out.
	fn hand_adt(
		&mut self,
		adt: AdtDef<'tcx>,
		args: ty::GenericArgsRef<'tcx>,
		hand: Hand,
		body: &mut Vec<String>,
		span: Span,
	) {
		let tcx = self.tcx;
		for (v, variant) in adt.variants().iter_enumerated() {
			let mut walks = Vec::new();
			for (i, field) in variant.fields.iter().enumerate() {
				let fty = self.mono_field_ty(field, args);
				if !self.hands(fty, hand) {
					continue;
				}
				let f = self.hand(fty, hand, span);
				let field = names::field(field, i);
				walks.push(match adt.is_enum() {
					true => format!("{f}(&p.{}.{field})", variant_field(variant.name.as_str())),
					false => format!("{f}(&p.{field})"),
				});
			}
			if walks.is_empty() {
				continue;
			}
			if !adt.is_enum() {
				body.extend(walks);
				continue;
			}
			let discr_ty = adt.repr().discr_type().to_ty(tcx);
			let bits = adt.discriminant_for_variant(tcx, v).val;
			let size = int_bits(tcx, discr_ty);
			let value = match discr_ty.kind() {
				ty::Int(_) => (((bits << (128 - size)) as i128) >> (128 - size)).to_string(),
				_ => (bits & (u128::MAX >> (128 - size))).to_string(),
			};
			let tag = format!("{}({value})", self.ty(discr_ty, span));
			let mut text = format!("if p.tag == {tag} {{ {} }}", walks.join("; "));
			if hand == Hand::Out {
				text.push_str(&format!(
					" else {{ zero(&p.{}) }}",
					variant_field(variant.name.as_str())
				));
			}
			body.push(text);
		}
	}
}

impl<'tcx> Translator<'tcx> {
	/// Refuse what a root that reaches byte storage cannot hand over: a function pointer or
	/// trait object, which Go code could fill with a function that keeps what the
	/// translation hands it, or reads a place Go holds while it keeps storage of its own; a
	/// raw pointer, which could see that handing in copied what it points into; and a
	/// reference anywhere in an argument but its outermost type, which the translation
	/// could move out of what handing back walks, leaving the place of Go's it points to
	/// with storage of its own.
	pub fn refuse_unhandable(&mut self, inputs: &[Ty<'tcx>], output: Ty<'tcx>, span: Span) {
		let find = |tys: &[Ty<'tcx>], pick: &dyn Fn(Ty<'tcx>, &Self) -> bool| {
			let mut seen = Vec::new();
			tys.iter().find_map(|t| self.find_part(*t, &mut seen, pick))
		};
		let sig: Vec<Ty<'tcx>> = inputs.iter().copied().chain([output]).collect();
		if find(&sig, &|t, this| bytes(this.tcx, t)).is_none() {
			return;
		}
		let mut refused = Vec::new();
		if let Some(f) = find(&sig, &|t, _| matches!(t.kind(), ty::FnPtr(..) | ty::Dynamic(..))) {
			refused.push(format!(
				"`{f}`, a callback in the signature of a root that reaches byte storage,"
			));
		}
		if let Some(p) = find(&sig, &|t, _| t.is_raw_ptr()) {
			refused.push(format!(
				"`{p}`, a raw pointer in the signature of a root that reaches byte storage,"
			));
		}
		let below: Vec<Ty<'tcx>> = inputs
			.iter()
			.map(|t| match *t.kind() {
				ty::Ref(_, inner, _) => inner,
				_ => *t,
			})
			.collect();
		if let Some(r) = find(&below, &|t, _| t.is_ref()) {
			refused.push(format!(
				"`{r}`, a reference inside an argument of a root that reaches byte storage,"
			));
		}
		for msg in refused {
			self.err(span, format!("{msg} is not supported"));
		}
	}

	/// The first type `pick` takes among `ty` and every type it is made of: fields, generic
	/// arguments, pointees, elements, captures, and a function pointer's parameters and result.
	fn find_part(
		&self,
		ty: Ty<'tcx>,
		seen: &mut Vec<Ty<'tcx>>,
		pick: &dyn Fn(Ty<'tcx>, &Self) -> bool,
	) -> Option<Ty<'tcx>> {
		if seen.contains(&ty) {
			return None;
		}
		seen.push(ty);
		if pick(ty, self) {
			return Some(ty);
		}
		let tcx = self.tcx;
		let parts: Vec<Ty<'tcx>> = match *ty.kind() {
			ty::Adt(adt, args) => {
				let mut parts: Vec<Ty<'tcx>> = args.types().collect();
				if library::mapped(tcx, adt.did()).is_none() {
					parts.extend(adt.all_fields().map(|f| self.mono_field_ty(f, args)));
				}
				parts
			}
			ty::Ref(_, inner, _) | ty::RawPtr(inner, _) | ty::Slice(inner) | ty::Array(inner, _) => vec![inner],
			ty::Tuple(tys) => tys.to_vec(),
			ty::Closure(_, args) => args.as_closure().upvar_tys().to_vec(),
			ty::FnPtr(..) => {
				let sig = tcx.instantiate_bound_regions_with_erased(ty.fn_sig(tcx));
				sig.inputs_and_output.to_vec()
			}
			_ => Vec::new(),
		};
		parts.into_iter().find_map(|t| self.find_part(t, seen, pick))
	}
}

/// Whether `ty` is byte storage: `str`, `[u8]`, or `Vec<u8>`.
fn bytes<'tcx>(tcx: TyCtxt<'tcx>, ty: Ty<'tcx>) -> bool {
	match ty.kind() {
		ty::Str => true,
		ty::Slice(elem) => is_u8(*elem),
		ty::Adt(adt, args) => library::mapped(tcx, adt.did()) == Some(Mapped::Vec) && is_u8(args.type_at(0)),
		_ => false,
	}
}
