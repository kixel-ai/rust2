//! Formatting: `format_args!`, the formatting traits, and std's impls of them.
//!
//! The runtime ports `core::fmt`, so translated output matches native Rust's byte for
//! byte. A placeholder's value becomes `(f) => ...`, a function writing it to a
//! formatter, and `fmt::Arguments` an array of literal pieces and such functions. Each
//! call goes to the impl of the value's type directly, as monomorphization allows.

use rustc_hir::def_id::DefId;
use rustc_middle::ty::{self, Ty, TyCtxt};
use rustc_span::Span;

use super::body::Body;
use super::library::{self, Std};
use super::num::Int;
use crate::ts::{Expr, Key, Param, Stmt, StmtKind, Type};

/// A formatting trait.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Trait {
	Display,
	Debug,
	LowerHex,
	UpperHex,
	Octal,
	Binary,
	LowerExp,
	UpperExp,
}

impl Trait {
	/// The trait `did` is, if it is one of these.
	pub fn of(tcx: TyCtxt<'_>, did: DefId) -> Option<Self> {
		match library::std_path(tcx, did)
			.iter()
			.map(String::as_str)
			.collect::<Vec<_>>()
			.as_slice()
		{
			["fmt", "Display"] => Some(Self::Display),
			["fmt", "Debug"] => Some(Self::Debug),
			["fmt", "LowerHex"] => Some(Self::LowerHex),
			["fmt", "UpperHex"] => Some(Self::UpperHex),
			["fmt", "Octal"] => Some(Self::Octal),
			["fmt", "Binary"] => Some(Self::Binary),
			["fmt", "LowerExp"] => Some(Self::LowerExp),
			["fmt", "UpperExp"] => Some(Self::UpperExp),
			_ => None,
		}
	}

	fn name(self) -> &'static str {
		match self {
			Self::Display => "Display",
			Self::Debug => "Debug",
			Self::LowerHex => "LowerHex",
			Self::UpperHex => "UpperHex",
			Self::Octal => "Octal",
			Self::Binary => "Binary",
			Self::LowerExp => "LowerExp",
			Self::UpperExp => "UpperExp",
		}
	}

	/// The trait's `fmt` method.
	fn method(self, tcx: TyCtxt<'_>) -> Option<DefId> {
		let module = tcx.parent(tcx.get_diagnostic_item(rustc_span::sym::Debug)?);
		let child = tcx
			.module_children(module)
			.iter()
			.find(|child| child.ident.as_str() == self.name())?;
		let trait_id = child.res.opt_def_id()?;
		let method = tcx
			.associated_items(trait_id)
			.in_definition_order()
			.find(|item| item.is_fn())?;
		Some(method.def_id)
	}
}

/// What a `format_args!` argument's constructor, `Argument::new_display` and the like,
/// makes: a value written by a trait, or `None` for a width or precision, as in `{:1$}`.
/// Otherwise, the construct outside the subset that it is.
pub fn argument(name: &str) -> Result<Option<Trait>, &'static str> {
	Ok(Some(match name {
		"new_display" => Trait::Display,
		"new_debug" => Trait::Debug,
		"new_lower_hex" => Trait::LowerHex,
		"new_upper_hex" => Trait::UpperHex,
		"new_octal" => Trait::Octal,
		"new_binary" => Trait::Binary,
		"new_lower_exp" => Trait::LowerExp,
		"new_upper_exp" => Trait::UpperExp,
		"from_usize" => return Ok(None),
		"new_pointer" => return Err("`Pointer` formatting, as in `{:p}`,"),
		_ => return Err("an unexpected `format_args!` lowering"),
	}))
}

/// A formatter's type, for the parameters of the functions written here.
fn formatter() -> Type {
	Type::Rs("Formatter", Vec::new())
}

impl<'b, 'a, 'tcx> Body<'b, 'a, 'tcx> {
	/// The formatting trait of a `&dyn Debug` or the like.
	pub fn fmt_object(&self, ty: Ty<'tcx>) -> Option<Trait> {
		let ty::Ref(_, inner, _) = ty.kind() else {
			return None;
		};
		let ty::Dynamic(preds, ..) = inner.kind() else {
			return None;
		};
		Trait::of(self.t.tcx, preds.principal_def_id()?)
	}

	/// `value`, a `ty`, written by `tr` to `f`, through the impl for `ty`.
	pub fn fmt_call(&mut self, tr: Trait, ty: Ty<'tcx>, value: Expr, f: Expr, span: Span) -> Expr {
		let tcx = self.t.tcx;
		let Some(method) = tr.method(tcx) else {
			return self.err(span, format!("no `{}::fmt`", tr.name()));
		};
		let args = tcx.mk_args(&[ty.into()]);
		let mut pre = Vec::new();
		let call = self.call_def(method, args, vec![value, f], span, &mut pre);
		match pre.is_empty() {
			true => call,
			false => self.err(span, format!("`{}` of `{ty}` needs statements", tr.name())),
		}
	}

	/// A function writing a `ty` by `tr`: `(value, f) => ...`, or the runtime's own.
	pub fn fmt_fn(&mut self, tr: Trait, ty: Ty<'tcx>, span: Span) -> Expr {
		let body = self.fmt_call(tr, ty, Expr::ident("value"), Expr::ident("f"), span);
		if let Expr::Call(callee, args) = &body
			&& let [Expr::Ident(value), Expr::Ident(f)] = args.as_slice()
			&& value == "value"
			&& f == "f"
			&& runtime(callee)
		{
			return (**callee).clone();
		}
		let params = vec![
			Param {
				name: "value".into(),
				ty: Some(self.t.ty(ty, span)),
			},
			Param {
				name: "f".into(),
				ty: Some(formatter()),
			},
		];
		Expr::Lambda(params, Box::new(body))
	}

	/// `value`, a `ty`, ready to be written by `tr`: `(f) => ...`. It is read now, once,
	/// as a placeholder's or a `&dyn Debug`'s value is.
	pub fn fmt_value(&mut self, tr: Trait, ty: Ty<'tcx>, value: Expr, span: Span, out: &mut Vec<Stmt>) -> Expr {
		let value = match value.is_path() {
			true => value,
			false => self.spill(value, "arg", out),
		};
		self.thunk(tr, ty, value, span)
	}

	/// `(f) => ...` writing `value`, which must be [`pure`].
	fn thunk(&mut self, tr: Trait, ty: Ty<'tcx>, value: Expr, span: Span) -> Expr {
		let f = match mentions(&value, "f") {
			true => self.fresh("f"),
			false => "f".to_string(),
		};
		let body = self.fmt_call(tr, ty, value, Expr::ident(&f), span);
		Expr::Lambda(
			vec![Param {
				name: f,
				ty: Some(formatter()),
			}],
			Box::new(body),
		)
	}
}

impl<'tcx> Body<'_, '_, 'tcx> {
	/// `fmt::Write`'s `def_id`, `write_str`, `write_char`, or `write_fmt`, of `w`, a
	/// `String`, a `Formatter`, or the crate's own writer, of `receiver`, the `&mut` to it,
	/// and `arg`, a `fmt::Arguments` whose only piece is `literal` if it is one. A
	/// provided method of the crate's impl writes through its `write_str`, and its
	/// `write_fmt` through the runtime's port of `fmt::write`, which calls its `write_str`
	/// and `write_char` as std does. `None` for any other writer.
	pub fn fmt_write(
		&mut self,
		def_id: DefId,
		w: Ty<'tcx>,
		[receiver, arg]: [Expr; 2],
		literal: Option<String>,
		span: Span,
		out: &mut Vec<Stmt>,
	) -> Option<Expr> {
		let tcx = self.t.tcx;
		let name = tcx.item_name(def_id);
		let name = name.as_str();
		let wargs = tcx.mk_args(&[w.into()]);
		let write_str = super::collections::trait_fn(tcx, "FmtWrite", "write_str");
		let std = self.t.std_of(w);
		if !matches!(std, Some(Std::String | Std::Formatter)) && !self.t.translated(write_str, wargs) {
			return None;
		}
		if std == Some(Std::String) {
			return Some(self.string_write(name, receiver, arg, literal, out));
		}
		// The crate's impl writes this method itself.
		if self.t.translated(def_id, wargs) {
			return Some(self.call_def(def_id, wargs, vec![receiver, arg], span, out));
		}
		if std == Some(Std::Formatter) {
			return Some(match (name, literal) {
				("write_char", _) => Expr::rs("fmtWriteChar", vec![receiver, arg]),
				("write_fmt", Some(s)) => Expr::rs("fmtWriteStr", vec![receiver, Expr::Str(s)]),
				("write_fmt", None) => Expr::rs("fmtWriteFmt", vec![receiver, arg]),
				_ => Expr::rs("fmtWriteStr", vec![receiver, arg]),
			});
		}
		// The provided `write_fmt` writes a literal with one `write_str`, as std's does.
		let text = match (name, literal) {
			("write_char", _) => library::char_text(arg),
			("write_fmt", Some(s)) => Expr::Str(s),
			("write_fmt", None) => {
				let write_char = super::collections::trait_fn(tcx, "FmtWrite", "write_char");
				// Read in each function it writes through.
				let receiver = match receiver.is_path() {
					true => receiver,
					false => self.spill(receiver, "w", out),
				};
				let (s, c) = (self.fresh("s"), self.fresh("c"));
				let (s_arg, c_arg) = (Expr::ident(&s), Expr::ident(&c));
				let mut pre = Vec::new();
				let str_call = self.call_def(write_str, wargs, vec![receiver.clone(), s_arg], span, &mut pre);
				// std's provided `write_char` writes the char's UTF-8 through `write_str`.
				let char_call = match self.t.translated(write_char, wargs) {
					true => self.call_def(write_char, wargs, vec![receiver, c_arg], span, &mut pre),
					false => self.call_def(
						write_str,
						wargs,
						vec![receiver, library::char_text(c_arg)],
						span,
						&mut pre,
					),
				};
				if !pre.is_empty() {
					return Some(self.err(span, format!("`write_str` of `{w}` needs statements")));
				}
				let lambda = |name: String, body: Expr| Expr::Lambda(vec![Param { name, ty: None }], Box::new(body));
				return Some(Expr::rs(
					"fmtWrite",
					vec![lambda(s, str_call), lambda(c, char_call), arg],
				));
			}
			_ => arg,
		};
		Some(self.call_def(write_str, wargs, vec![receiver, text], span, out))
	}

	/// `fmt::Write`'s `name` of the `String` `s`, writing `arg`, a `fmt::Arguments` whose only
	/// piece is `literal` if it is one, in place: what the call returns. What a placeholder
	/// wrote before an error stays written.
	pub fn string_write(
		&mut self,
		name: &str,
		s: Expr,
		arg: Expr,
		literal: Option<String>,
		out: &mut Vec<Stmt>,
	) -> Expr {
		let pushed = match (name, literal) {
			("write_char", _) => Expr::rs("stringPush", vec![s, arg]),
			("write_fmt", Some(text)) => Expr::rs("stringPushStr", vec![s, Expr::Str(text)]),
			("write_fmt", None) => return Expr::rs("fmtWriteString", vec![s, arg]),
			_ => Expr::rs("stringPushStr", vec![s, arg]),
		};
		out.push(StmtKind::Expr(pushed).at(None));
		Expr::Rs("OK".into())
	}
}

/// `f.debug_struct(name).finish_non_exhaustive()`: the name and ` { .. }`, each a `write_str`.
fn non_exhaustive(f: Expr, name: &str) -> Expr {
	let builder = Expr::rs("fmtDebugStruct", vec![f, Expr::Str(name.into())]);
	Expr::rs("debugStructFinishNonExhaustive", vec![builder])
}

/// Whether `e` is a runtime function, which reads no `this`.
fn runtime(e: &Expr) -> bool {
	matches!(e, Expr::Rs(_))
}

/// Whether the path `e` reads the variable `name`.
fn mentions(e: &Expr, name: &str) -> bool {
	match e {
		Expr::Ident(ident) => ident == name,
		Expr::Member(base, _) | Expr::Located(_, base) => mentions(base, name),
		Expr::Index(base, index) => mentions(base, name) || mentions(index, name),
		_ => false,
	}
}

/// Whether reading `e` again gives the same value and does nothing else: a path, or an
/// element or a field of one, such as what a tuple's guard lends.
fn pure(e: &Expr) -> bool {
	match e {
		Expr::Index(base, index) => pure(base) && index.is_constant(),
		Expr::Member(base, _) | Expr::Located(_, base) => pure(base),
		e => e.is_path(),
	}
}

/// std's impl of `tr` for `ty`, writing `value` to `f`; `None` if it has none here.
pub fn std_impl<'tcx>(
	b: &mut Body<'_, '_, 'tcx>,
	tr: Trait,
	ty: Ty<'tcx>,
	value: Expr,
	f: Expr,
	span: Span,
	out: &mut Vec<Stmt>,
) -> Option<Expr> {
	let tcx = b.t.tcx;
	let rs = |name: &str, args: Vec<Expr>| Expr::rs_of("fmt", name).call(args);
	let text = matches!(tr, Trait::Display | Trait::Debug);
	Some(match ty.kind() {
		// A reference to an array is a slice of it, written as a slice is.
		ty::Ref(_, inner, _) if let ty::Array(elem, _) = inner.kind() => {
			b.fmt_call(tr, Ty::new_slice(tcx, *elem), value, f, span)
		}
		ty::Ref(_, inner, _) => {
			let value = b.t.pointee_value(ty, value);
			b.fmt_call(tr, *inner, value, f, span)
		}
		ty::Int(_) | ty::Uint(_) => {
			let method = match tr {
				Trait::Display => "display",
				Trait::Debug => "debug",
				Trait::LowerHex => "lowerHex",
				Trait::UpperHex => "upperHex",
				Trait::Octal => "octal",
				Trait::Binary => "binary",
				Trait::LowerExp => "lowerExp",
				Trait::UpperExp => "upperExp",
			};
			Int::of(ty)?.method(method, vec![value, f])
		}
		_ if let Some(float) = super::float::Float::of(ty) => {
			let method = match tr {
				Trait::Display => "display",
				Trait::Debug => "debug",
				Trait::LowerExp => "lowerExp",
				Trait::UpperExp => "upperExp",
				_ => return None,
			};
			Expr::rs_of(float.name(), method).call(vec![value, f])
		}
		// `!`, and `Infallible`, which is it: never written, as none exists.
		ty::Never if text => Expr::rs("unreachable", vec![value]),
		ty::Bool if text => rs("bool", vec![value, f]),
		ty::Char if tr == Trait::Display => rs("charDisplay", vec![value, f]),
		ty::Char if tr == Trait::Debug => rs("charDebug", vec![value, f]),
		ty::Str if tr == Trait::Display => Expr::rs("fmtPad", vec![f, value]),
		ty::Str if tr == Trait::Debug => rs("str", vec![value, f]),
		ty::Tuple(tys) if tys.is_empty() && tr == Trait::Debug => rs("unit", vec![value, f]),
		ty::Tuple(tys) if tr == Trait::Debug => {
			// Each element's function reads the tuple.
			let value = match pure(&value) {
				true => value,
				false => b.spill(value, "arg", out),
			};
			let fields = tys
				.iter()
				.enumerate()
				.map(|(i, t)| b.thunk(tr, t, value.clone().index(Expr::lit(i.to_string())), span))
				.collect();
			rs("tuple", vec![Expr::Array(fields), f])
		}
		ty::Array(elem, _) | ty::Slice(elem) if tr == Trait::Debug => {
			let item = b.fmt_fn(tr, *elem, span);
			rs("list", vec![library::elements(value, ty), f, item])
		}
		// A `&dyn Debug` is already a function writing its value.
		ty::Dynamic(preds, ..) if preds.principal_def_id().and_then(|did| Trait::of(tcx, did)) == Some(tr) => {
			value.call(vec![f])
		}
		// Its name and no fields, unpadded, whatever it holds.
		ty::Dynamic(..) if tr == Trait::Debug && super::dynamic::is_any_object(tcx, ty) => non_exhaustive(f, "Any"),
		// Unpadded, naming its type as `type_name` does, a piece at a time as its `write!` does.
		ty::Adt(adt, args) if tr == Trait::Debug && adt.is_phantom_data() => {
			let name = rustc_const_eval::util::type_name(tcx, args.type_at(0));
			let pieces = ["PhantomData<", &name, ">"].map(|s| Expr::Str(s.to_string()));
			rs("writeFmt", vec![f, Expr::Array(pieces.into())])
		}
		// Each its name alone, unpadded.
		ty::Adt(adt, _)
			if tr == Trait::Debug
				&& (library::pinned(tcx, adt.did())
					|| library::empty_iter(tcx, adt.did())
					|| matches!(
						Std::of(tcx, adt.did()),
						Some(Std::Pending | Std::PollFn | Std::BuildHasherDefault)
					)) =>
		{
			rs("writeStr", vec![f, Expr::Str(tcx.item_name(adt.did()).to_string())])
		}
		// An ASCII escape writes what it has left, unpadded, and `Debug` only its name.
		ty::Adt(adt, _)
			if matches!(
				library::std_path(tcx, adt.did())
					.iter()
					.map(String::as_str)
					.collect::<Vec<_>>()
					.as_slice(),
				["ascii", "EscapeDefault"] | ["slice", "ascii", "EscapeAscii"]
			) =>
		{
			match tr {
				Trait::Display => Expr::rs("fmtEscapeAscii", vec![value, f]),
				Trait::Debug => non_exhaustive(f, tcx.item_name(adt.did()).as_str()),
				_ => return None,
			}
		}
		// A char's case map or escape writes what it has left, unpadded, and `Debug` that in
		// std's derived fields.
		ty::Adt(adt, _)
			if matches!(
				library::std_path(tcx, adt.did())
					.iter()
					.map(String::as_str)
					.collect::<Vec<_>>()
					.as_slice(),
				[
					"char",
					"ToLowercase" | "ToUppercase" | "EscapeUnicode" | "EscapeDebug" | "EscapeDefault"
				]
			) =>
		{
			match tr {
				Trait::Display => Expr::rs("fmtCharsDisplay", vec![value, f]),
				Trait::Debug => {
					let name = Expr::Str(tcx.item_name(adt.did()).to_string());
					Expr::rs("fmtCharsDebug", vec![value, f, name])
				}
				_ => return None,
			}
		}
		// A `str`'s escape writes what it has left, unpadded.
		ty::Adt(adt, _)
			if tr == Trait::Display
				&& matches!(
					library::std_path(tcx, adt.did())
						.iter()
						.map(String::as_str)
						.collect::<Vec<_>>()
						.as_slice(),
					["str", "iter", "EscapeUnicode" | "EscapeDebug" | "EscapeDefault"]
				) =>
		{
			Expr::rs("fmtStrEscapeDisplay", vec![value, f])
		}
		ty::Adt(adt, args)
			if tr == Trait::Debug
				&& let Some(e) = iter_debug(b, adt.did(), args, &value, &f, span) =>
		{
			e
		}
		ty::Adt(adt, args)
			if tr == Trait::Debug
				&& let Some(e) = adaptor_debug(b, adt.did(), args, &value, &f, span) =>
		{
			e
		}
		ty::Adt(adt, args) => match (Std::of(tcx, adt.did())?, tr) {
			(Std::String, Trait::Display) => Expr::rs("fmtPad", vec![f, value.member("v")]),
			(Std::String, Trait::Debug) => rs("str", vec![value.member("v"), f]),
			// A `NonZero`, a `Wrapping`, and a `Saturating` write their integer, by each trait.
			(Std::Box | Std::NonZero | Std::Wrapping | Std::Saturating, _) => {
				b.fmt_call(tr, args.type_at(0), value, f, span)
			}
			// An `IoSlice` writes the bytes it lends.
			(Std::IoSlice, Trait::Debug) => {
				let bytes = Ty::new_imm_ref(tcx, tcx.lifetimes.re_erased, Ty::new_slice(tcx, tcx.types.u8));
				b.fmt_call(tr, bytes, value, f, span)
			}
			// A `Cow` writes what it derefs to.
			(Std::Cow, _) if let Some(borrowed) = library::cow_borrowed(tcx, ty) => {
				b.fmt_call(tr, borrowed, library::cow_deref(value, borrowed), f, span)
			}
			(Std::Arc, Trait::Display | Trait::Debug) => {
				b.fmt_call(tr, args.type_at(0), value.member("value"), f, span)
			}
			// A guard writes what it lends; its first argument is its lifetime.
			(Std::MutexGuard | Std::Ref | Std::RefMut, Trait::Display | Trait::Debug) => {
				let held = args.types().next()?;
				b.fmt_call(tr, held, value.member("value"), f, span)
			}
			// Each bound is written with the range's options.
			(
				std @ (Std::Range | Std::RangeInclusive | Std::RangeFrom | Std::RangeTo | Std::RangeToInclusive),
				Trait::Debug,
			) => {
				let bound = b.fmt_fn(tr, args.type_at(0), span);
				let of = match std {
					Std::Range => "range",
					Std::RangeInclusive => "rangeInclusive",
					Std::RangeFrom => "rangeFrom",
					Std::RangeTo => "rangeTo",
					_ => "rangeToInclusive",
				};
				rs(of, vec![value, f, bound])
			}
			(Std::RangeFull, Trait::Debug) => rs("rangeFull", vec![value, f]),
			(Std::Bound, Trait::Debug) => {
				let held = b.fmt_fn(tr, args.type_at(0), span);
				rs("bound", vec![value, f, held])
			}
			(Std::Ordering, Trait::Debug) => rs("ordering", vec![value, f]),
			// Erased to what it holds.
			(std @ (Std::Reverse | Std::AssertUnwindSafe), Trait::Debug) => {
				let held = b.fmt_value(tr, args.type_at(0), value, span, out);
				let name = format!("{std:?}");
				rs("debugTupleFields", vec![f, Expr::Str(name), Expr::Array(vec![held])])
			}
			(Std::ControlFlow, Trait::Debug) => {
				let go = b.fmt_fn(tr, args.type_at(1), span);
				let stop = b.fmt_fn(tr, args.type_at(0), span);
				match library::Flow::of(tcx, args) {
					library::Flow::Other => rs("controlFlow", vec![value, f, go, stop]),
					library::Flow::Result => rs("controlFlow", vec![value, f, go, stop, Expr::lit("true")]),
					library::Flow::Option => {
						let none = b.t.none_arg(args.type_at(1));
						rs(
							"controlFlowOption",
							[vec![value, f, go, stop], none.into_iter().collect()].concat(),
						)
					}
				}
			}
			(Std::Vec, Trait::Debug) => {
				let item = b.fmt_fn(tr, args.type_at(0), span);
				rs("list", vec![library::elements(value, ty), f, item])
			}
			(Std::BinaryHeap, Trait::Debug) => {
				let item = b.fmt_fn(tr, args.type_at(0), span);
				rs("list", vec![value.member("items"), f, item])
			}
			// Each writes its items as its iterator reads them: a deque from its head.
			(Std::VecDeque, Trait::Debug) => {
				let item = b.fmt_fn(tr, args.type_at(0), span);
				rs("list", vec![Expr::rs("dequeIter", vec![value]), f, item])
			}
			(Std::LinkedList, Trait::Debug) => {
				let item = b.fmt_fn(tr, args.type_at(0), span);
				rs("list", vec![Expr::rs("listIter", vec![value]), f, item])
			}
			// Its items, as a set.
			(Std::HashTable, Trait::Debug) => {
				let item = b.fmt_fn(tr, args.type_at(0), span);
				rs("set", vec![Expr::rs("hashValues", vec![value]), f, item])
			}
			(std @ (Std::HashSet | Std::BTreeSet), Trait::Debug) => {
				let item = b.fmt_fn(tr, args.type_at(0), span);
				let keys = if std == Std::HashSet { "hashKeys" } else { "btreeKeys" };
				rs("set", vec![Expr::rs(keys, vec![value]), f, item])
			}
			(std @ (Std::HashMap | Std::BTreeMap), Trait::Debug) => {
				let key = b.fmt_fn(tr, args.type_at(0), span);
				let val = b.fmt_fn(tr, args.type_at(1), span);
				let entries = if std == Std::HashMap { "hashIter" } else { "btreeIter" };
				rs("map", vec![Expr::rs(entries, vec![value]), f, key, val])
			}
			(Std::Option, Trait::Debug) => {
				let some = b.fmt_fn(tr, args.type_at(0), span);
				let none = b.t.none_arg(args.type_at(0));
				rs("option", [vec![value, f, some], none.into_iter().collect()].concat())
			}
			(Std::Result, Trait::Debug) => {
				let ok = b.fmt_fn(tr, args.type_at(0), span);
				let err = b.fmt_fn(tr, args.type_at(1), span);
				rs("result", vec![value, f, ok, err])
			}
			(Std::Poll, Trait::Debug) => {
				let ready = b.fmt_fn(tr, args.type_at(0), span);
				let pending = b.t.pending_arg(args.type_at(0));
				rs("poll", [vec![value, f, ready], pending.into_iter().collect()].concat())
			}
			(Std::Waker | Std::Context | Std::WakerVTable | Std::RawWaker, Trait::Debug) => b.err(
				span,
				format!("`Debug` of `{ty}`, which writes addresses, which JS has no counterpart of"),
			),
			// The `Option` it holds until a poll takes its value.
			(Std::Ready, Trait::Debug) => {
				let held = b.fmt_fn(tr, args.type_at(0), span);
				value.member("debug").call(vec![f, held])
			}
			(Std::Arguments, Trait::Display | Trait::Debug) => rs("arguments", vec![value, f]),
			(Std::FmtError, Trait::Debug) => rs("errorDebug", vec![value, f]),
			(Std::PoisonError, Trait::Debug) => rs("poisonErrorDebug", vec![value, f]),
			(Std::PoisonError, Trait::Display) => rs("poisonErrorDisplay", vec![value, f]),
			(Std::FmtError, Trait::Display) => rs("errorDisplay", vec![value, f]),
			(Std::TryFromIntError, Trait::Debug) => rs("tryFromIntErrorDebug", vec![value, f]),
			(Std::TryFromIntError, Trait::Display) => rs("tryFromIntErrorDisplay", vec![value, f]),
			(Std::ParseIntError, Trait::Debug) => rs("parseIntErrorDebug", vec![value, f]),
			(Std::ParseIntError, Trait::Display) => rs("parseIntErrorDisplay", vec![value, f]),
			(Std::ErrorKind, Trait::Debug) => rs("writeStr", vec![f, value]),
			(Std::CharTryFromError, Trait::Debug) => rs("charTryFromErrorDebug", vec![value, f]),
			(Std::CharTryFromError, Trait::Display) => rs("charTryFromErrorDisplay", vec![value, f]),
			(Std::TryFromCharError, Trait::Debug) => rs("tryFromCharErrorDebug", vec![value, f]),
			(Std::TryFromCharError, Trait::Display) => rs("tryFromCharErrorDisplay", vec![value, f]),
			(Std::DecodeUtf16Error, Trait::Debug) => rs("decodeUtf16ErrorDebug", vec![value, f]),
			(Std::DecodeUtf16Error, Trait::Display) => rs("decodeUtf16ErrorDisplay", vec![value, f]),
			(Std::ParseBoolError, Trait::Debug) => rs("parseBoolErrorDebug", vec![value, f]),
			(Std::ParseBoolError, Trait::Display) => rs("parseBoolErrorDisplay", vec![value, f]),
			(Std::BorrowError, Trait::Debug) => Expr::rs("borrowErrorDebug", vec![value, f]),
			(Std::BorrowError, Trait::Display) => Expr::rs("borrowErrorDisplay", vec![value, f]),
			(Std::BorrowMutError, Trait::Debug) => Expr::rs("borrowMutErrorDebug", vec![value, f]),
			(Std::BorrowMutError, Trait::Display) => Expr::rs("borrowMutErrorDisplay", vec![value, f]),
			// What each holds, held as a guard would while it is written, or what keeps it from
			// being read. An `RwLock` shares a `RefCell`'s representation, and an `UnsafeCell` a
			// `Cell`'s.
			(Std::Cell, Trait::Debug) if tcx.is_lang_item(adt.did(), rustc_attr_ir::LangItem::UnsafeCell) => {
				non_exhaustive(f, "UnsafeCell")
			}
			// What `get` gives once its name is written, as std reads it.
			(Std::Cell, Trait::Debug) => {
				let held = b.fmt_fn(tr, args.type_at(0), span);
				let value = b.t.copied(value.member("v"), args.type_at(0), span);
				Expr::rs("cellDebug", vec![Expr::Lambda(Vec::new(), Box::new(value)), f, held])
			}
			(std @ (Std::RefCell | Std::Mutex), Trait::Debug) => {
				let debug = match std {
					Std::RefCell if tcx.is_diagnostic_item(rustc_span::sym::RwLock, adt.did()) => "rwLockDebug",
					Std::RefCell => "refCellDebug",
					_ => "mutexDebug",
				};
				let held = b.fmt_fn(tr, args.type_at(0), span);
				Expr::rs(debug, vec![value, f, held])
			}
			// What it holds, or `<uninit>`.
			(std @ (Std::OnceCell | Std::OnceLock), Trait::Debug) => {
				let held = b.fmt_fn(tr, args.type_at(0), span);
				let none = b.t.none_arg(args.type_at(0));
				let name = Expr::Str(format!("{std:?}"));
				Expr::rs(
					"onceDebug",
					[vec![value, f, name, held], none.into_iter().collect()].concat(),
				)
			}
			// Its value, or `<uninit>` before it is forced and while its function runs.
			(std @ (Std::LazyCell | Std::LazyLock), Trait::Debug) => {
				let held = b.fmt_fn(tr, args.type_at(0), span);
				value.member("debug").call(vec![f, Expr::Str(format!("{std:?}")), held])
			}
			(Std::ParseCharError, Trait::Debug) => rs("parseCharErrorDebug", vec![value, f]),
			(Std::ParseCharError, Trait::Display) => rs("parseCharErrorDisplay", vec![value, f]),
			(Std::ParseFloatError, Trait::Debug) => rs("parseFloatErrorDebug", vec![value, f]),
			(Std::ParseFloatError, Trait::Display) => rs("parseFloatErrorDisplay", vec![value, f]),
			(Std::TryFromFloatSecsError, Trait::Debug) => rs("tryFromFloatSecsErrorDebug", vec![value, f]),
			(Std::TryFromFloatSecsError, Trait::Display) => rs("tryFromFloatSecsErrorDisplay", vec![value, f]),
			(Std::FromUtf8Error, Trait::Debug) => rs("fromUtf8ErrorDebug", vec![value, f]),
			(Std::FromUtf8Error, Trait::Display) => rs("fromUtf8ErrorDisplay", vec![value, f]),
			(Std::FromUtf16Error, Trait::Debug) => rs("fromUtf16ErrorDebug", vec![value, f]),
			(Std::FromUtf16Error, Trait::Display) => rs("fromUtf16ErrorDisplay", vec![value, f]),
			(Std::Utf8Error, Trait::Debug) => rs("utf8ErrorDebug", vec![value, f]),
			(Std::Utf8Error, Trait::Display) => rs("utf8ErrorDisplay", vec![value, f]),
			(Std::Utf8Chunk, Trait::Debug) => rs("utf8ChunkDebug", vec![value, f]),
			(Std::Duration, Trait::Debug) => Expr::rs("durationDebug", vec![value, f]),
			(Std::TypeId, Trait::Debug) => rs("typeIdDebug", vec![value, f]),
			(Std::Layout, Trait::Debug) => rs("layoutDebug", vec![value, f]),
			(Std::Location, Trait::Debug) => rs("locationDebug", vec![value, f]),
			(Std::Location, Trait::Display) => rs("locationDisplay", vec![value, f]),
			(Std::PanicHookInfo, Trait::Display) => rs("panicHookInfoDisplay", vec![value, f]),
			(Std::LayoutError, Trait::Debug) => rs("layoutErrorDebug", vec![value, f]),
			(Std::LayoutError, Trait::Display) => rs("layoutErrorDisplay", vec![value, f]),
			(Std::TryReserveError, Trait::Debug) => rs("tryReserveErrorDebug", vec![value, f]),
			(Std::TryReserveError, Trait::Display) => rs("tryReserveErrorDisplay", vec![value, f]),
			(Std::Sync, Trait::Debug) => return super::tokio::debug(b, ty, value, f, span),
			(Std::JsClosure, Trait::Debug) => f.member("out").member("writeStr").call(vec![Expr::Str("Closure { ... }".into())]),
			(Std::Bytes, Trait::Debug) => Expr::rs("bytesDebug", vec![value, f]),
			(Std::BytesMut, Trait::Debug) => Expr::rs("bytesDebug", vec![Expr::rs("bytesMutView", vec![value]), f]),
			_ => return None,
		},
		_ => return None,
	})
}

/// `Debug` of one of std's iterators that lists the items it has left, as std writes
/// them, or `None` for another type. A deque's `Iter` and `IterMut` write the two slices of
/// its ring buffer they have left, and its `Drain` its counts; an `Option`'s iterator the
/// item it has left as an `Option`, and a slice's parts what is left of the slice.
fn iter_debug<'tcx>(
	b: &mut Body<'_, '_, 'tcx>,
	did: DefId,
	args: ty::GenericArgsRef<'tcx>,
	value: &Expr,
	f: &Expr,
	span: Span,
) -> Option<Expr> {
	/// What std writes around the list: a tuple struct's name, or a struct's and its field's.
	enum Wrap<'a> {
		Tuple(&'a str),
		Struct(&'a str, &'a str),
	}
	/// What it lists: a type argument, the map entry of the first two, or a `BTreeSet`'s
	/// entry of its key and `SetValZST`.
	enum Item {
		Arg(usize),
		Entry,
		SetVal,
		Byte,
	}
	use {Item::*, Wrap::*};
	let tcx = b.t.tcx;
	let path = library::std_path(tcx, did);
	let path: Vec<&str> = path.iter().map(String::as_str).collect();
	let listed = |wraps: &[Wrap]| {
		let wraps = (wraps.iter()).map(|wrap| match *wrap {
			Tuple(name) => Expr::Str(name.into()),
			Struct(name, field) => Expr::Array(vec![Expr::Str(name.into()), Expr::Str(field.into())]),
		});
		Expr::Array(wraps.collect())
	};
	let call = |name: &str, rest: Vec<Expr>| Expr::rs(name, [vec![value.clone(), f.clone()], rest].concat());
	let write = |text: String| Some(Expr::rs("fmtWriteStr", vec![f.clone(), Expr::Str(text)]));
	// The item it has left, if any, as std's `Option` inside `wraps`: a reference as it is
	// held, a `&mut` to a value JS copies its place.
	let option = |b: &mut Body<'_, '_, 'tcx>, wraps: &[Wrap]| {
		let item = b.t.iter_item(Ty::new_adt(tcx, tcx.adt_def(did), args))?;
		let debug = b.fmt_fn(Trait::Debug, item, span);
		Some(call("fmtIterOption", vec![debug, listed(wraps)]))
	};
	// What writes a slice of its parts.
	let slice =
		|b: &mut Body<'_, '_, 'tcx>| Some(b.fmt_fn(Trait::Debug, Ty::new_slice(tcx, args.types().next()?), span));
	let (item, wraps): (Item, &[Wrap]) = match path.as_slice() {
		["str", "iter", "Chars"] => return Some(Expr::rs("fmtChars", vec![value.clone(), f.clone()])),
		["str", "iter", "CharIndices"] => return Some(call("fmtCharIndices", Vec::new())),
		// Its searcher's state, by the name of std's searcher of its pattern.
		[
			"str",
			"iter",
			name @ ("Split" | "RSplit" | "SplitTerminator" | "RSplitTerminator" | "SplitN" | "RSplitN"
			| "SplitInclusive" | "Matches" | "RMatches" | "MatchIndices" | "RMatchIndices"),
		] => {
			let searcher = library::searcher(b.t, args.types().next()?)?;
			return Some(call(
				"fmtStrSearch",
				vec![Expr::Str((*name).into()), Expr::Str(searcher.into())],
			));
		}
		[
			"str",
			"iter",
			name @ ("Lines" | "SplitWhitespace" | "SplitAsciiWhitespace"),
		] => {
			return Some(call("fmtStrSearch", vec![Expr::Str((*name).into())]));
		}
		["string", "Drain"] => return Some(call("fmtStringDrain", Vec::new())),
		["str", "lossy", "Utf8Chunks"] => return Some(Expr::rs("fmtUtf8Chunks", vec![value.clone(), f.clone()])),
		// Without its fields, which neither `#` nor a width changes.
		["str", "iter", "EncodeUtf16"]
		| ["iter", "sources", "repeat_with", "RepeatWith"]
		| ["collections", "hash", "map" | "set", "ExtractIf"] => {
			return Some(non_exhaustive(f.clone(), tcx.item_name(did).as_str()));
		}
		// The item it reads next.
		["collections", "linked_list", "ExtractIf"] => {
			let debug = b.fmt_fn(Trait::Debug, args.types().next()?, span);
			return Some(call("fmtListExtractIf", vec![debug]));
		}
		// The entry it reads next, of a set its key.
		["collections", "btree", map @ ("map" | "set"), "ExtractIf"] => {
			let mut types = args.types();
			let debug = b.fmt_fn(Trait::Debug, types.next()?, span);
			let value = match *map == "map" {
				true => vec![b.fmt_fn(Trait::Debug, types.next()?, span)],
				false => Vec::new(),
			};
			return Some(call("fmtBTreeExtractIf", [vec![debug], value].concat()));
		}
		["iter", "sources", "from_fn", "FromFn"] => return write("FromFn".into()),
		["iter", "sources", "once_with", "OnceWith"] => return Some(call("fmtOnceWith", Vec::new())),
		["iter", "sources", "repeat", "Repeat"] | ["iter", "sources", "repeat_n", "RepeatN"] => {
			let debug = b.fmt_fn(Trait::Debug, args.types().next()?, span);
			return Some(call(&format!("fmt{}", tcx.item_name(did)), vec![debug]));
		}
		["iter", "sources", "successors", "Successors"] => {
			let debug = b.fmt_fn(Trait::Debug, args.types().next()?, span);
			return Some(call("fmtSuccessors", vec![debug]));
		}
		["iter", "sources", "once", "Once"] => {
			return option(
				b,
				&[
					Struct("Once", "inner"),
					Struct("IntoIter", "inner"),
					Struct("Item", "opt"),
				],
			);
		}
		["option", name @ ("Iter" | "IterMut" | "IntoIter")] => {
			return option(b, &[Struct(name, "inner"), Struct("Item", "opt")]);
		}
		["result", name @ ("Iter" | "IterMut" | "IntoIter")] => return option(b, &[Struct(name, "inner")]),
		[
			"slice",
			"iter",
			name @ ("Windows" | "Chunks" | "ChunksExact" | "RChunks" | "RChunksExact"),
		] => {
			let slice = slice(b)?;
			return Some(call("fmtParts", vec![slice, Expr::Str((*name).into())]));
		}
		[
			"slice",
			"iter",
			"ChunksMut" | "ChunksExactMut" | "RChunksMut" | "RChunksExactMut",
		] => {
			let name = tcx.item_name(did);
			return Some(b.err(
				span,
				format!("`Debug` of a `{name}`, whose slice std writes as an address, which JS has no counterpart of"),
			));
		}
		// A `splitn`'s is a `GenericSplitN` of the split it is named for.
		[
			"slice",
			"iter",
			name @ ("Split" | "SplitMut" | "SplitInclusive" | "SplitInclusiveMut" | "RSplit" | "RSplitMut" | "SplitN"
			| "SplitNMut" | "RSplitN" | "RSplitNMut"),
		] => {
			let split = name.replacen('N', "", 1);
			let n = (split != *name).then(|| Expr::Str((*name).into()));
			let taken = Expr::lit(name.ends_with("Mut").to_string());
			let rest = [slice(b)?, Expr::Str(split), taken].into_iter().chain(n);
			return Some(call("fmtSplit", rest.collect()));
		}
		// The way std picked to make it, and what that reads.
		[
			"collections",
			"btree",
			"set",
			name @ ("Difference" | "Intersection" | "SymmetricDifference" | "Union"),
		] => {
			let debug = b.fmt_fn(Trait::Debug, args.types().next()?, span);
			return Some(call("fmtBTreeSetOp", vec![debug, Expr::Str((*name).into())]));
		}
		// The list of what a clone of it yields.
		[
			"collections",
			"hash",
			"set",
			"Difference" | "Intersection" | "SymmetricDifference" | "Union",
		] => {
			let debug = b.fmt_fn(Trait::Debug, args.types().next()?, span);
			return Some(call("fmtIterClone", vec![debug]));
		}
		[
			"collections",
			"vec_deque",
			"iter" | "iter_mut",
			name @ ("Iter" | "IterMut"),
		] => {
			let debug = b.fmt_fn(Trait::Debug, args.types().next()?, span);
			let call = vec![value.clone(), f.clone(), debug, Expr::Str((*name).into())];
			return Some(Expr::rs("fmtDequeIter", call));
		}
		["vec", "extract_if", "ExtractIf"] => {
			let debug = b.fmt_fn(Trait::Debug, args.types().next()?, span);
			return Some(Expr::rs("fmtVecExtractIf", vec![value.clone(), f.clone(), debug]));
		}
		["collections", "vec_deque", "drain", "Drain"] => {
			let len = b.fmt_fn(Trait::Debug, tcx.types.usize, span);
			return Some(Expr::rs("fmtDequeDrain", vec![value.clone(), f.clone(), len]));
		}
		// The list it has left, then its length.
		["collections", "linked_list", name @ ("Iter" | "IterMut")] => {
			let debug = b.fmt_fn(Trait::Debug, args.types().next()?, span);
			let len = b.fmt_fn(Trait::Debug, tcx.types.usize, span);
			let call = vec![value.clone(), f.clone(), debug, len, Expr::Str((*name).into())];
			return Some(Expr::rs("fmtListIter", call));
		}
		["str", "iter", "Bytes"] => (Byte, &[Tuple("Bytes"), Struct("Copied", "it"), Tuple("Iter")]),
		["slice", "iter", "Iter"]
		| ["collections", "binary_heap", "Iter"]
		| ["collections", "btree", "set", "Iter"] => (Arg(0), &[Tuple("Iter")]),
		["slice", "iter", "IterMut"] => (Arg(0), &[Tuple("IterMut")]),
		["vec", "into_iter", "IntoIter"]
		| ["array", "iter", "IntoIter"]
		| ["collections", "binary_heap", "IntoIter"]
		| ["collections", "vec_deque", "into_iter", "IntoIter"]
		| ["collections", "linked_list", "IntoIter"] => (Arg(0), &[Tuple("IntoIter")]),
		["vec", "drain", "Drain"] => (Arg(0), &[Tuple("Drain")]),
		["collections", "binary_heap", "Drain"] => (Arg(0), &[Struct("Drain", "iter"), Tuple("Drain")]),
		["collections", "btree", "set", "Range"] => (SetVal, &[Struct("Range", "iter")]),
		["collections", "btree", "set", "IntoIter"] => (SetVal, &[Struct("IntoIter", "iter")]),
		[
			"collections",
			"btree",
			"map",
			"Iter" | "IterMut" | "Range" | "RangeMut" | "IntoIter",
		]
		| ["collections", "hash", "map", "Iter" | "IterMut" | "IntoIter" | "Drain"] => (Entry, &[]),
		["collections", "btree" | "hash", "map", "Keys" | "IntoKeys"]
		| ["collections", "hash", "set", "Iter" | "IntoIter" | "Drain"] => (Arg(0), &[]),
		[
			"collections",
			"btree" | "hash",
			"map",
			"Values" | "ValuesMut" | "IntoValues",
		] => (Arg(1), &[]),
		_ => return None,
	};
	let arg = |n: usize| args.types().nth(n);
	let debug = match item {
		Arg(n) => b.fmt_fn(Trait::Debug, arg(n)?, span),
		Entry => b.fmt_fn(Trait::Debug, Ty::new_tup(tcx, &[arg(0)?, arg(1)?]), span),
		SetVal => Expr::rs("fmtSetVal", vec![b.fmt_fn(Trait::Debug, arg(0)?, span)]),
		Byte => b.fmt_fn(Trait::Debug, tcx.types.u8, span),
	};
	let mut rest = vec![debug];
	if !wraps.is_empty() {
		rest.push(listed(wraps));
	}
	Some(call("fmtIter", rest))
}

/// `Debug` of one of std's iterator adaptors, as std's derived or written impl writes it:
/// its fields in order, each iterator it wraps by that iterator's own `Debug`, and none of
/// its closures; or `None` for another type.
fn adaptor_debug<'tcx>(
	b: &mut Body<'_, '_, 'tcx>,
	did: DefId,
	args: ty::GenericArgsRef<'tcx>,
	value: &Expr,
	f: &Expr,
	span: Span,
) -> Option<Expr> {
	let tcx = b.t.tcx;
	let path = library::std_path(tcx, did);
	let path: Vec<&str> = path.iter().map(String::as_str).collect();
	let arg = |n: usize| args.types().nth(n);
	let call = |name: &str, rest: Vec<Expr>| Expr::rs(name, [vec![value.clone(), f.clone()], rest].concat());
	let name = |name: &str| Expr::Str(name.into());
	// What writes an iterator it wraps, as the adaptor holds it and `rs.iterHeld` reads it.
	let wrapped = |b: &mut Body<'_, '_, 'tcx>, ty: Ty<'tcx>| {
		let (ty, _) = super::iter::peel_iter(b, ty, Expr::undefined());
		b.fmt_fn(Trait::Debug, ty, span)
	};
	Some(match path.as_slice() {
		[
			"iter",
			"adapters",
			_,
			adaptor @ ("Map" | "Filter" | "FilterMap" | "Inspect" | "MapWhile" | "Rev"),
		] => call("fmtAdaptor", vec![name(adaptor), wrapped(b, arg(0)?)]),
		// A `copied` that JS reads its items as is is its iterator itself.
		["iter", "adapters", "copied", "Copied"]
			if let ty::Ref(_, item, _) = b.t.iter_item(arg(0)?)?.kind()
				&& super::iter::copies_itself(b.t, *item) =>
		{
			call("fmtAround", vec![name("Copied"), name("it"), wrapped(b, arg(0)?)])
		}
		["iter", "adapters", _, adaptor @ ("Copied" | "Cloned")] => {
			let inner = wrapped(b, arg(0)?);
			call("fmtAdaptor", vec![name(adaptor), inner, name("it")])
		}
		["iter", "adapters", "enumerate", "Enumerate"] => call("fmtEnumerate", vec![wrapped(b, arg(0)?)]),
		// Its drain, of what it has not yielded, and the iterator it fills the gap from.
		["vec", "splice", "Splice"] => {
			let debug = b.fmt_fn(Trait::Debug, b.t.iter_item(arg(0)?)?, span);
			call("fmtVecSplice", vec![debug, wrapped(b, arg(0)?)])
		}
		["iter", "adapters", _, adaptor @ ("Skip" | "Take")] => {
			call("fmtCounted", vec![name(adaptor), wrapped(b, arg(0)?)])
		}
		["iter", "adapters", _, adaptor @ ("SkipWhile" | "TakeWhile")] => {
			call("fmtFlagged", vec![name(adaptor), wrapped(b, arg(0)?)])
		}
		["iter", "adapters", "scan", "Scan"] => {
			let inner = wrapped(b, arg(0)?);
			call("fmtScan", vec![inner, b.fmt_fn(Trait::Debug, arg(1)?, span)])
		}
		["iter", "adapters", "peekable", "Peekable"] => {
			let inner = wrapped(b, arg(0)?);
			let item = b.t.iter_item(arg(0)?)?;
			call("fmtPeekable", vec![inner, b.fmt_fn(Trait::Debug, item, span)])
		}
		["iter", "adapters", "step_by", "StepBy"] => call("fmtStepBy", vec![wrapped(b, arg(0)?)]),
		["iter", "adapters", "chain", "Chain"] => {
			let a = wrapped(b, arg(0)?);
			call("fmtChain", vec![a, wrapped(b, arg(1)?)])
		}
		// std's specializes it to write neither of two it reads by index.
		["iter", "adapters", "zip", "Zip"] => {
			let (a, other) = (arg(0)?, arg(1)?);
			match [a, other]
				.iter()
				.all(|&ty| super::iter::implements(b.t, ty, "TrustedRandomAccessNoCoerce"))
			{
				true => call("fmtZip", Vec::new()),
				false => {
					let a = wrapped(b, a);
					call("fmtZip", vec![a, wrapped(b, other)])
				}
			}
		}
		["iter", "adapters", "cycle", "Cycle"] => call("fmtCycle", vec![wrapped(b, arg(0)?)]),
		["iter", "adapters", "fuse", "Fuse"] => call("fmtFuse", vec![wrapped(b, arg(0)?)]),
		// Its `FlattenCompat`'s source and the iterator of each item, as its field's type
		// names them.
		["iter", "adapters", "flatten", adaptor @ ("Flatten" | "FlatMap")] => {
			let field = tcx.adt_def(did).non_enum_variant().fields.iter().next()?;
			let ty::Adt(_, compat) = b.t.field_ty(field.did, args).kind() else {
				return None;
			};
			let source = wrapped(b, compat.type_at(0));
			let inner = wrapped(b, compat.type_at(1));
			call("fmtFlatten", vec![name(adaptor), source, inner])
		}
		["char", "decode", "DecodeUtf16"] => call("fmtDecodeUtf16", vec![wrapped(b, arg(0)?)]),
		// A `str`'s escape is the `flat_map` of its chars it holds.
		[
			"str",
			"iter",
			escape @ ("EscapeDebug" | "EscapeDefault" | "EscapeUnicode"),
		] => {
			let field = tcx.adt_def(did).non_enum_variant().fields.iter().next()?;
			let inner = b.fmt_fn(Trait::Debug, b.t.field_ty(field.did, args), span);
			call("fmtAround", vec![name(escape), name("inner"), inner])
		}
		_ => return None,
	})
}

/// One part of a `format_args!` template.
pub enum Piece {
	Lit(String),
	/// A placeholder writing the `index`th argument.
	Arg {
		index: usize,
		spec: Spec,
	},
}

/// A placeholder's options, from `FormattingOptions`' flags.
pub struct Spec {
	fill: char,
	align: Option<char>,
	plus: bool,
	alternate: bool,
	zero: bool,
	debug_hex: Option<char>,
	width: Option<Count>,
	precision: Option<Count>,
}

/// A width or precision: in the template, or the `index`th argument's.
#[derive(Clone, Copy)]
pub enum Count {
	Is(u16),
	Arg(usize),
}

impl Spec {
	/// The runtime's `Spec` object, or `None` for the defaults. `arg` reads an argument.
	pub fn object(&self, arg: impl Fn(usize) -> Expr) -> Option<Expr> {
		let count = |count: Count| match count {
			Count::Is(n) => Expr::lit(n.to_string()),
			Count::Arg(index) => Expr::rs("fmtCountOf", vec![arg(index)]),
		};
		let mut props = Vec::new();
		if self.fill != ' ' {
			props.push(("fill", Expr::Str(self.fill.to_string())));
		}
		if let Some(align) = self.align {
			props.push(("align", Expr::Str(align.to_string())));
		}
		if self.plus {
			props.push(("plus", Expr::lit("true")));
		}
		if self.alternate {
			props.push(("alternate", Expr::lit("true")));
		}
		if self.zero {
			props.push(("zero", Expr::lit("true")));
		}
		if let Some(hex) = self.debug_hex {
			props.push(("debugHex", Expr::Str(hex.to_string())));
		}
		if let Some(width) = self.width {
			props.push(("width", count(width)));
		}
		if let Some(precision) = self.precision {
			props.push(("precision", count(precision)));
		}
		let props: Vec<(Key, Expr)> = props.into_iter().map(|(k, v)| (Key::Name(k.into()), v)).collect();
		(!props.is_empty()).then_some(Expr::Object(props))
	}
}

// `FormattingOptions`' flags, as `core::fmt` lays them out.
const SIGN_PLUS: u32 = 1 << 21;
const ALTERNATE: u32 = 1 << 23;
const ZERO_PAD: u32 = 1 << 24;
const DEBUG_LOWER_HEX: u32 = 1 << 25;
const DEBUG_UPPER_HEX: u32 = 1 << 26;
const WIDTH: u32 = 1 << 27;
const PRECISION: u32 = 1 << 28;
const FILL: u32 = (1 << 21) - 1;
const DEFAULT_FLAGS: u32 = ' ' as u32 | 3 << 29;

/// Decode a template as `core::fmt::write` reads it: literal pieces, and placeholders
/// with their argument and options. The `-` flag is dropped, as std's impls ignore it.
pub fn decode(template: &[u8]) -> Result<Vec<Piece>, &'static str> {
	const TRUNCATED: &str = "a truncated template";
	let byte = |i: usize| template.get(i).copied().ok_or(TRUNCATED);
	let u16_at = |i: usize| Ok::<_, &str>(u16::from_le_bytes([byte(i)?, byte(i + 1)?]));
	let lit = |start: usize, len: usize| {
		let bytes = template.get(start..start + len).ok_or(TRUNCATED)?;
		String::from_utf8(bytes.to_vec()).map_err(|_| "a template piece that is not UTF-8")
	};
	let mut pieces = Vec::new();
	let mut i = 0;
	let mut next = 0;
	loop {
		let n = byte(i)?;
		i += 1;
		match n {
			0 => return Ok(pieces),
			1..=0x7f => {
				pieces.push(Piece::Lit(lit(i, n.into())?));
				i += usize::from(n);
			}
			0x80 => {
				let len = usize::from(u16_at(i)?);
				pieces.push(Piece::Lit(lit(i + 2, len)?));
				i += 2 + len;
			}
			0xc0..=0xff => {
				let mut flags = DEFAULT_FLAGS;
				let mut width = 0;
				let mut precision = 0;
				if n & 1 != 0 {
					flags = u32::from_le_bytes([byte(i)?, byte(i + 1)?, byte(i + 2)?, byte(i + 3)?]);
					i += 4;
				}
				if n & 2 != 0 {
					width = u16_at(i)?;
					i += 2;
				}
				if n & 4 != 0 {
					precision = u16_at(i)?;
					i += 2;
				}
				if n & 8 != 0 {
					next = usize::from(u16_at(i)?);
					i += 2;
				}
				// An indirect count is the index of the argument that holds it.
				let count = |set: u32, indirect: u8, value: u16| {
					(flags & set != 0).then_some(match n & indirect != 0 {
						true => Count::Arg(value.into()),
						false => Count::Is(value),
					})
				};
				let spec = Spec {
					fill: char::from_u32(flags & FILL).ok_or("a fill that is not a char")?,
					align: match flags >> 29 & 3 {
						0 => Some('<'),
						1 => Some('>'),
						2 => Some('^'),
						_ => None,
					},
					plus: flags & SIGN_PLUS != 0,
					alternate: flags & ALTERNATE != 0,
					zero: flags & ZERO_PAD != 0,
					debug_hex: match flags {
						f if f & DEBUG_LOWER_HEX != 0 => Some('x'),
						f if f & DEBUG_UPPER_HEX != 0 => Some('X'),
						_ => None,
					},
					width: count(WIDTH, 16, width),
					precision: count(PRECISION, 32, precision),
				};
				pieces.push(Piece::Arg { index: next, spec });
				next += 1;
			}
			_ => return Err("an unknown template byte"),
		}
	}
}
