//! Handing a `Vec<u8>`'s storage between Go code and the translation. The runtime grows
//! storage it owns without zeroing it and recycles it once outgrown, telling it apart by
//! its shape (`bytesOwn` in `runtime/rt.go`). Go code reaches storage only through a root
//! function, so the root hands Go's storage in off that shape, and its own out clipped to
//! its length. In, it also copies each container an argument owns, a `Box`'s pointee or a
//! `Vec`'s elements, by value or behind `&mut`, so the translation never writes storage of
//! its own where Go code may still hold the container, which a move, a grow, or a
//! replaced `Box` would leave behind. Out, it clips its result and what its arguments
//! reach behind `&mut`, the places Go code keeps.

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
	/// Out of a root, in its result: each `Vec<u8>` of own storage, and each byte slice,
	/// is clipped to its length.
	Out,
	/// Out of a root, in an argument: as `Out`, but only behind a reference, where Go's
	/// places are, since what the argument owns is the root's copy.
	Back,
}

impl Hand {
	/// The way of what lies behind a pointer, which `owns` it if a `Box`.
	fn behind(self, owns: bool) -> Self {
		match self {
			Hand::Back if !owns => Hand::Out,
			hand => hand,
		}
	}
}

fn is_u8(ty: Ty<'_>) -> bool {
	matches!(ty.kind(), ty::Uint(ty::UintTy::U8))
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
				Some(Mapped::Vec) => match is_u8(args.type_at(0)) {
					true => hand != Hand::Back,
					false => self.hands_in(args.type_at(0), hand, seen),
				},
				Some(Mapped::Box) => self.hands_behind(args.type_at(0), true, true, hand, seen),
				Some(Mapped::SliceIter) => hand == Hand::Out && is_u8(args.type_at(0)),
				Some(Mapped::Transparent) => self.hands_in(args.type_at(0), hand, seen),
				Some(Mapped::Option | Mapped::Result) | None => adt
					.all_fields()
					.any(|field| self.hands_in(self.mono_field_ty(field, args), hand, seen)),
				Some(Mapped::Runtime(_) | Mapped::Refused(_)) => false,
			},
			ty::Ref(_, inner, m) | ty::RawPtr(inner, m) => self.hands_behind(inner, m.is_mut(), false, hand, seen),
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

	/// `hands` of a pointer to `inner`, through which the translation may write if `mutable`,
	/// and which `owns` it if a `Box`.
	fn hands_behind(
		&self,
		inner: Ty<'tcx>,
		mutable: bool,
		owns: bool,
		hand: Hand,
		seen: &mut Vec<(Ty<'tcx>, Hand)>,
	) -> bool {
		// The pointer is where `hand` is; what it points to is behind it.
		let next = hand.behind(owns);
		match *inner.kind() {
			ty::Str => hand == Hand::Out,
			ty::Slice(elem) => (hand == Hand::Out && is_u8(elem)) || (mutable && self.hands_in(elem, next, seen)),
			ty::Dynamic(..) => false,
			_ => mutable && self.hands_in(inner, next, seen),
		}
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
			Hand::Back => "Back",
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
				Some(Mapped::SliceIter) => body.push("clip(&p.s)".into()),
				Some(Mapped::Transparent) => {
					let f = self.hand(args.type_at(0), hand, span);
					body.push(format!("{f}(p)"));
				}
				_ => self.hand_adt(adt, args, hand, &mut body, span),
			},
			ty::Ref(_, inner, m) | ty::RawPtr(inner, m) => {
				self.hand_behind(inner, m.is_mut(), false, hand, "p", &mut body, span)
			}
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
			Hand::Back => "back to Go in a root function's argument",
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
		let next = hand.behind(owns);
		match *inner.kind() {
			ty::Str => body.push(format!("clip({p})")),
			ty::Slice(elem) => {
				if hand == Hand::Out && is_u8(elem) {
					body.push(format!("clip({p})"));
				}
				if mutable && self.hands(elem, next) {
					if copy {
						body.push(format!("*{p} = append((*{p})[:0:0], *{p}...)"));
					}
					let f = self.hand(elem, next, span);
					body.push(format!("for i := range *{p} {{ {f}(&(*{p})[i]) }}"));
				}
			}
			_ => {
				let f = self.hand(inner, next, span);
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
	/// Refuse a function pointer or trait object in the signature of a root that reaches
	/// byte storage: Go code could pass a function of its own, which the translation would
	/// hand its storage to, or run while a place Go holds keeps storage of its own.
	pub fn refuse_callback(&mut self, sig: &[Ty<'tcx>], span: Span) {
		let find = |pick: &dyn Fn(Ty<'tcx>, &Self) -> bool| {
			let mut seen = Vec::new();
			sig.iter().find_map(|t| self.find_part(*t, &mut seen, pick))
		};
		if find(&|t, this| bytes(this.tcx, t)).is_none() {
			return;
		}
		if let Some(f) = find(&|t, _| matches!(t.kind(), ty::FnPtr(..) | ty::Dynamic(..))) {
			self.err(
				span,
				format!("`{f}`, a callback in the signature of a root that reaches byte storage, is not supported"),
			);
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
