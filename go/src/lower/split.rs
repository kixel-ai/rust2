//! Enums split into their parts. Go keeps a value of over four words in memory, zeroing
//! it first when it holds a pointer and copying it in and out of every call, so an enum
//! that large is its tag and its variants' fields, each a Go variable, parameter, and
//! result of its own. A part over four words stays whole, behind a pointer: a function
//! takes it through one it only reads through, and returns it through one its caller
//! passes, which it writes only when it returns that variant, so `?` hands an error on in
//! its caller's storage rather than copying it at each call and return.

use std::rc::Rc;

use rustc_abi::{FieldIdx, VariantIdx};
use rustc_middle::ty::util::IntTypeExt;
use rustc_middle::ty::{self, Ty};
use rustc_span::Span;

use super::library::{self, Mapped};
use super::ty::variant_field;
use super::{Translator, names};

/// The most words Go keeps a value of in registers.
const REGISTERS: usize = 4;

/// An enum's parts: its tag, then each variant's fields in order.
pub struct Split<'tcx> {
	/// The Go type of the whole enum.
	pub go: String,
	pub parts: Vec<Part<'tcx>>,
}

pub struct Part<'tcx> {
	/// Its variable's suffix: `tag`, or its variant's and field's indices, `v1_0`, which no
	/// two parts share as two names joined could.
	pub name: String,
	/// Its selector in the whole value: `.tag`, `.Ok.F0`.
	pub path: String,
	/// The Go field of its variant, and its own: `Ok` and `F0`.
	label: String,
	field_name: String,
	pub ty: Ty<'tcx>,
	pub go: String,
	/// The variant and field it is, or `None` for the tag.
	pub field: Option<(VariantIdx, FieldIdx)>,
	/// Whether it is over four words, which a function returns through a pointer.
	pub big: bool,
}

impl<'tcx> Split<'tcx> {
	/// The part of variant `variant`'s field `field`.
	pub fn part(&self, variant: VariantIdx, field: FieldIdx) -> usize {
		self.parts
			.iter()
			.position(|p| p.field == Some((variant, field)))
			.expect("a variant's field is a part")
	}

	/// The results of a function returning the enum: its parts but those it writes
	/// through pointers, as `func` writes them after its parameters.
	pub fn results(&self) -> String {
		let small: Vec<&str> = self.parts.iter().filter(|p| !p.big).map(|p| p.go.as_str()).collect();
		format!(" ({})", small.join(", "))
	}

	/// The whole enum from its parts' values, as a Go composite literal.
	pub fn rebuild(&self, values: &[String]) -> String {
		let mut fields = vec![format!("tag: {}", values[0])];
		let mut i = 1;
		while i < self.parts.len() {
			let (variant, _) = self.parts[i].field.expect("only the first part is the tag");
			let start = i;
			while i < self.parts.len() && self.parts[i].field.is_some_and(|(v, _)| v == variant) {
				i += 1;
			}
			let parts = &self.parts[start..i];
			let types: Vec<String> = parts.iter().map(|p| format!("{} {}", p.field_name, p.go)).collect();
			fields.push(format!(
				"{}: struct{{ {} }}{{{}}}",
				parts[0].label,
				types.join("; "),
				values[start..i].join(", ")
			));
		}
		format!("({}{{{}}})", self.go, fields.join(", "))
	}
}

impl<'tcx> Translator<'tcx> {
	/// The parts of `ty`, if Go is to hold it split: an enum of over four words, laid out
	/// as the translation lays out any enum.
	pub fn split(&mut self, ty: Ty<'tcx>, span: Span) -> Option<Rc<Split<'tcx>>> {
		if let Some(split) = self.splits.get(&ty) {
			return split.clone();
		}
		// A part's Go type reaching the enum again, as `fn(E) -> E` held in `E`, would be
		// a Go function type holding itself, which only a named type can.
		if self.splitting.contains(&ty) {
			self.err(
				span,
				format!(
					"`{ty}`, an enum held as its parts that a function pointer in it takes or returns, is not supported"
				),
			);
			return None;
		}
		self.splitting.push(ty);
		let split = self.find_split(ty, span).map(Rc::new);
		self.splitting.pop();
		self.splits.insert(ty, split.clone());
		split
	}

	fn find_split(&mut self, ty: Ty<'tcx>, span: Span) -> Option<Split<'tcx>> {
		let tcx = self.tcx;
		let ty::Adt(adt, args) = *ty.kind() else {
			return None;
		};
		if !adt.is_enum()
			|| library::mapped(tcx, adt.did()).is_some_and(|m| !m.is_runtime_layout())
			|| self.words(ty) <= REGISTERS
		{
			return None;
		}
		let tag = adt.repr().discr_type().to_ty(tcx);
		let mut parts = vec![Part {
			name: "tag".into(),
			path: ".tag".into(),
			label: String::new(),
			field_name: String::new(),
			ty: tag,
			go: self.ty(tag, span),
			field: None,
			big: false,
		}];
		for (v, variant) in adt.variants().iter_enumerated() {
			let label = variant_field(variant.name.as_str());
			for (f, field) in variant.fields.iter_enumerated() {
				let fty = self.field_of(field, args);
				let name = names::field(field, f.as_usize());
				parts.push(Part {
					name: format!("v{}_{}", v.as_usize(), f.as_usize()),
					path: format!(".{label}.{name}"),
					label: label.clone(),
					field_name: name,
					ty: fty,
					go: self.ty(fty, span),
					field: Some((v, f)),
					big: self.words(fty) > REGISTERS,
				});
			}
		}
		Some(Split {
			go: self.ty(ty, span),
			parts,
		})
	}

	/// The Go types of a signature's parameters, out-pointers last, and its results, as
	/// `func` writes them after its parameters.
	pub fn sig_types(&mut self, inputs: &[Ty<'tcx>], output: Ty<'tcx>, span: Span) -> (Vec<String>, String) {
		let mut params = Vec::new();
		for input in inputs {
			match self.split(*input, span) {
				Some(split) => params.extend(split.parts.iter().map(|p| match p.big {
					true => format!("*{}", p.go),
					false => p.go.clone(),
				})),
				None => params.push(self.ty(*input, span)),
			}
		}
		if Self::no_value(output) {
			return (params, String::new());
		}
		let Some(split) = self.split(output, span) else {
			return (params, format!(" {}", self.ty(output, span)));
		};
		params.extend(split.parts.iter().filter(|p| p.big).map(|p| format!("*{}", p.go)));
		(params, split.results())
	}

	/// A field's monomorphic type.
	fn field_of(&self, field: &ty::FieldDef, args: ty::GenericArgsRef<'tcx>) -> Ty<'tcx> {
		let fty = field.ty(self.tcx, args).skip_norm_wip();
		self.tcx
			.normalize_erasing_regions(self.env(), ty::Unnormalized::new_wip(fty))
	}

	/// About how many machine words Go holds a value of `ty` in: an integer or a pointer
	/// one, a slice three, an aggregate the sum of its fields, an enum its tag and every
	/// variant's fields side by side, and an array of more than one element, which Go
	/// never holds in registers, more than four.
	fn words(&self, ty: Ty<'tcx>) -> usize {
		let tcx = self.tcx;
		let sum = |tys: &mut dyn Iterator<Item = Ty<'tcx>>| tys.map(|t| self.words(t)).sum::<usize>();
		match *ty.kind() {
			ty::Never | ty::FnDef(..) => 0,
			ty::Ref(_, inner, _) | ty::RawPtr(inner, _) => match inner.kind() {
				ty::Str | ty::Slice(_) => 3,
				_ => 1,
			},
			ty::Tuple(tys) => sum(&mut tys.iter()),
			ty::Array(elem, len) => match len.try_to_target_usize(tcx) {
				Some(0) => 0,
				Some(1) => self.words(elem),
				_ => REGISTERS + 1,
			},
			ty::Closure(_, args) => sum(&mut args.as_closure().upvar_tys().iter()),
			ty::Adt(adt, args) => match library::mapped(tcx, adt.did()) {
				Some(Mapped::Vec) => 3,
				Some(Mapped::SliceIter) => 3,
				Some(Mapped::Box) => self.words(Ty::new_imm_ref(tcx, tcx.lifetimes.re_erased, args.type_at(0))),
				Some(Mapped::Transparent) => self.words(args.type_at(0)),
				Some(Mapped::Runtime("struct{}" | "fmtError") | Mapped::Refused(_)) => 0,
				// The formatter and its builders, which no enum holds.
				Some(Mapped::Runtime(_)) if !adt.is_enum() => REGISTERS + 1,
				_ => {
					let tag = usize::from(adt.is_enum());
					tag + sum(&mut adt.all_fields().map(|f| self.field_of(f, args)))
				}
			},
			_ => 1,
		}
	}
}
