//! The transcript as Dart: a library whose `script()` runs it through the generated Dart,
//! calling `tests/api.dart`'s own functions, which it imports as `package:api_test/api.dart`.

use std::fmt::Write;

use super::ir::{Access, Collection, Int, Mutability, Receiver, Type};
use super::{
	Arm, BoundKind, Expr, FnId, Kind, Num, Op, Owner, Pat, Piece, Spec, Step, Transcript, each_expr, unref, walk,
};

const LANGUAGE: &str = "Dart";

/// Helpers the script calls, each for a value Dart shapes apart from Rust.
const PRELUDE: &str = r#"// Generated from fixtures/api/examples/transcript.rs by tests/fixtures/transcript.

import 'dart:convert';
import 'dart:typed_data';

import 'package:api/api.dart' as api;
import 'package:api_test/api.dart' as local;

/// A string as Rust's `Debug` prints it.
String _quoted(String s) {
  final out = StringBuffer('"');
  for (final c in s.runes) {
    switch (c) {
      case 0x22:
        out.write(r'\"');
      case 0x5c:
        out.write(r'\\');
      case 0x0a:
        out.write(r'\n');
      case 0x09:
        out.write(r'\t');
      case 0x0d:
        out.write(r'\r');
      case 0x00:
        out.write(r'\0');
      case < 0x20 || 0x7f:
        out.write('\\u{${c.toRadixString(16)}}');
      default:
        out.writeCharCode(c);
    }
  }
  out.write('"');
  return out.toString();
}

String _list<T>(Iterable<T> items, String Function(T) show) => '[${items.map(show).join(', ')}]';

/// An `Option` as Rust's `Debug` prints it. Unbounded, so it takes a `U64`, which is no
/// `Object`.
String _option<T>(T? value, String Function(T) show) => value == null ? 'None' : 'Some(${show(value as T)})';

Uint8List _bytes(List<int> bytes) => Uint8List.fromList(bytes);

/// The `f64` of `bits`.
double _f64(int bits) => (Uint64List(1)..[0] = bits).buffer.asFloat64List()[0];

/// Bytes as text, what is not UTF-8 replaced.
String _text(List<int> bytes) => utf8.decode(bytes, allowMalformed: true);

/// `value`, an object a statement made and only borrows, which Rust drops as the statement
/// ends: `_close` closes it then.
T _tmp<T>(List<Object?> temps, T value) {
  temps.add(value);
  return value;
}

/// Close each of a statement's temporaries, the last made first, as Rust drops them.
void _close(List<Object?> temps) {
  for (final value in temps.reversed) {
    (value as dynamic)?.close();
  }
}

T _some<T>(T? value) => value == null ? throw StateError('unwrap of None') : value as T;

/// A call's value, or what it threw that `caught` takes as its error.
sealed class _Result<T> {}

final class _Ok<T> extends _Result<T> {
  final T value;

  _Ok(this.value);
}

final class _Err<T> extends _Result<T> {
  final Object error;

  _Err(this.error);
}

_Result<T> _try<T>(T Function() call, bool Function(Object) caught) {
  try {
    return _Ok(call());
  } catch (e) {
    if (caught(e)) return _Err(e);
    rethrow;
  }
}

/// The error of a call that must fail.
E _err<E extends Object>(void Function() call) {
  try {
    call();
  } on E catch (e) {
    return e;
  }
  throw StateError('the call did not fail');
}

bool _fails<E extends Object>(void Function() call) {
  try {
    call();
  } on E {
    return true;
  }
  return false;
}

/// `value`, which Rust's arithmetic left in `lo` through `hi`, or else its overflow panic.
int _int(int value, int lo, int hi) =>
    value < lo || value > hi ? throw StateError('attempt to overflow') : value;

/// `value` as an `i64`, or else its overflow panic.
int _i64(BigInt value) => value.isValidInt ? value.toInt() : throw StateError('attempt to overflow');

/// `value` as a `u64`, or else its overflow panic.
api.U64 _u64(BigInt value) =>
    value.isNegative || value.bitLength > 64 ? throw StateError('attempt to overflow') : api.U64.fromBigInt(value);
"#;

/// The Dart library of `t`'s script.
pub(crate) fn emit(t: &Transcript) -> String {
	let mut e = Emitter {
		t,
		depth: 1,
		mutated: Vec::new(),
		locals: Vec::new(),
		closures: 0,
		fresh: 0,
		temps: Vec::new(),
	};
	mutated(&t.steps, t, &mut e.mutated);
	let body = e.steps(&t.steps);
	format!("{PRELUDE}\nFuture<void> script() async {{\n{body}}}\n")
}

/// The locals a call takes `&mut` that are values, not objects: each is an `api.Mut`,
/// whose `value` the script reads.
fn mutated(steps: &[Step], t: &Transcript, out: &mut Vec<String>) {
	let mut visit = |expr: &Expr| {
		walk(expr, &mut |e| {
			if let Kind::Call(id, _, args) = &e.kind {
				for (arg, param) in args.iter().zip(&t.function(*id).params) {
					if let (Kind::Var(name), Type::Ref(Mutability::Mut, inner)) = (&arg.kind, &param.ty)
						&& !matches!(**inner, Type::Object(_) | Type::Reader)
						&& !out.contains(name)
					{
						out.push(name.clone());
					}
				}
			}
		})
	};
	for step in steps {
		each_expr(step, &mut visit);
	}
}

struct Emitter<'t> {
	t: &'t Transcript,
	depth: usize,
	/// The locals each an `api.Mut`.
	mutated: Vec<String>,
	/// Each local bound so far, with its type.
	locals: Vec<(String, Type)>,
	/// How many closures deep the script is, where it awaits nothing.
	closures: usize,
	fresh: usize,
	/// Each statement's list of its temporaries, innermost last, and whether it holds any.
	temps: Vec<(String, bool)>,
}

/// Dart's reserved words, which no name may be.
const RESERVED: &[&str] = &[
	"assert", "await", "break", "case", "catch", "class", "const", "continue", "default", "do", "else", "enum",
	"extends", "false", "final", "finally", "for", "if", "in", "is", "new", "null", "rethrow", "return", "super",
	"switch", "this", "throw", "true", "try", "var", "void", "while", "with", "yield",
];

/// Dart's built-in and contextual words, which no local may be.
const BUILT_IN: &[&str] = &[
	"abstract",
	"as",
	"covariant",
	"deferred",
	"dynamic",
	"export",
	"extension",
	"external",
	"factory",
	"Function",
	"get",
	"implements",
	"import",
	"interface",
	"late",
	"library",
	"mixin",
	"operator",
	"part",
	"required",
	"set",
	"static",
	"typedef",
	"base",
	"sealed",
	"when",
	"on",
	"show",
	"hide",
	"async",
	"sync",
	"of",
];

/// What the script itself names, which no local of its may shadow.
const BUILTINS: &[&str] = &[
	"api",
	"local",
	"print",
	"utf8",
	"script",
	"identical",
	"int",
	"double",
	"bool",
	"num",
	"List",
	"String",
	"Object",
];

/// A local's name in Dart.
fn ident(name: &str) -> String {
	match RESERVED.contains(&name) || BUILT_IN.contains(&name) || BUILTINS.contains(&name) {
		true => format!("{name}_"),
		false => name.to_string(),
	}
}

/// A Rust identifier in snake case, as the exporter splits it: `TrackProducer` is
/// `track_producer`, and `WARN` is `warn`.
fn snake(name: &str) -> String {
	let chars: Vec<char> = name.chars().collect();
	let mut out = String::new();
	for (i, &c) in chars.iter().enumerate() {
		if c.is_uppercase() {
			let prev = i.checked_sub(1).map(|j| chars[j]);
			let next = chars.get(i + 1);
			let boundary = match prev {
				Some(p) if p.is_lowercase() || p.is_ascii_digit() => true,
				Some(p) if p.is_uppercase() => next.is_some_and(|n| n.is_lowercase()),
				_ => false,
			};
			if boundary && !out.ends_with('_') {
				out.push('_');
			}
			out.extend(c.to_lowercase());
		} else {
			out.push(c);
		}
	}
	out
}

/// A name in camel case, as Dart names members: `into_label` is `intoLabel`.
fn camel(name: &str) -> String {
	if name.starts_with(|c: char| c.is_ascii_digit()) {
		return format!("${name}");
	}
	let mut out = String::new();
	for (i, word) in snake(name).split('_').filter(|w| !w.is_empty()).enumerate() {
		let mut chars = word.chars();
		match (i, chars.next()) {
			(0, Some(first)) => out.extend(first.to_lowercase().chain(chars)),
			(_, Some(first)) => out.extend(first.to_uppercase().chain(chars)),
			_ => {}
		}
	}
	out
}

/// A member's name: camel case, with `_` past a reserved word or one of `taken`.
fn member(name: &str, taken: &[&str]) -> String {
	let name = camel(name);
	match RESERVED.contains(&name.as_str()) || taken.contains(&name.as_str()) {
		true => format!("{name}_"),
		false => name,
	}
}

/// A record's field, as the exporter names it.
fn field(name: &str) -> String {
	member(
		name,
		&[
			"hashCode",
			"noSuchMethod",
			"runtimeType",
			"toString",
			"compareTo",
			"partialCompareTo",
		],
	)
}

/// The Dart of an integer's name, as the runtime's classes end in it: `u64` is `U64`.
fn int_name(int: Int) -> String {
	let rust = int.rust();
	let mut chars = rust.chars();
	let first = chars.next().unwrap().to_ascii_uppercase();
	format!("{first}{}", chars.as_str())
}

/// Whether an integer crosses as `api.U64`.
fn wide(int: Int) -> bool {
	matches!(int, Int::U64 | Int::Usize)
}

/// The `dart:typed_data` list of a block's element, as the exporter picks one, or `None`
/// for a list of no block.
fn block(ty: &Type) -> Option<&'static str> {
	let Type::List(Collection::Vec | Collection::Slice | Collection::Box | Collection::Array(_), inner) = ty else {
		return None;
	};
	Some(match &**inner {
		Type::Int(Int::U8) => "Uint8List",
		Type::Int(Int::U16) => "Uint16List",
		Type::Int(Int::U32) => "Uint32List",
		// A `Uint64List` read as `U64`, which the binding passes as a `List<U64>`.
		Type::Int(Int::U64) => "<api.U64>",
		Type::Int(Int::I8) => "Int8List",
		Type::Int(Int::I16) => "Int16List",
		Type::Int(Int::I32) => "Int32List",
		Type::Int(Int::I64) => "Int64List",
		Type::F32 => "Float32List",
		Type::F64 => "Float64List",
		_ => return None,
	})
}

/// `ty` past references, boxes, and an `Option`.
fn plain(ty: &Type) -> &Type {
	match unref(ty).base() {
		Type::Option(inner) => plain(inner),
		ty => ty,
	}
}

impl Emitter<'_> {
	fn indent(&self) -> String {
		"  ".repeat(self.depth)
	}

	fn fresh(&mut self, what: &str) -> String {
		self.fresh += 1;
		format!("_{what}{}", self.fresh)
	}

	fn steps(&mut self, steps: &[Step]) -> String {
		let mut out = String::new();
		for step in steps {
			let (temps, body) = self.scoped(|e| {
				let mut out = String::new();
				e.step(step, &mut out);
				out
			});
			match temps {
				Some(temps) => {
					self.line(&mut out, &format!("final {temps} = <Object?>[];"));
					out.push_str(&body);
					self.line(&mut out, &format!("_close({temps});"));
				}
				None => out.push_str(&body),
			}
		}
		out
	}

	/// What `f` emits in a scope of its own temporaries, and the list holding them, if any.
	fn scoped<T>(&mut self, f: impl FnOnce(&mut Self) -> T) -> (Option<String>, T) {
		let name = self.fresh("d");
		self.temps.push((name, false));
		let v = f(self);
		let (name, used) = self.temps.pop().unwrap();
		(used.then_some(name), v)
	}

	/// `v`, the Dart of `e`, kept to close as its statement ends if it is an object only
	/// borrowed there that the statement made.
	fn temp(&mut self, v: String, e: &Expr) -> String {
		fn made(e: &Expr) -> bool {
			match &e.kind {
				Kind::Call(..) | Kind::Clone(_) | Kind::Default => true,
				Kind::Unwrap(inner) => made(inner),
				_ => false,
			}
		}
		if !made(e) || !matches!(plain(&e.ty), Type::Object(_)) {
			return v;
		}
		let (temps, used) = self.temps.last_mut().expect("a statement's temporaries");
		*used = true;
		format!("_tmp({temps}, {v})")
	}

	/// The Dart of `e`, an operand only borrowed.
	fn lent(&mut self, e: &Expr) -> String {
		let v = self.expr(e);
		self.temp(v, e)
	}

	fn line(&self, out: &mut String, line: &str) {
		let _ = writeln!(out, "{}{line}", self.indent());
	}

	/// A local's place: its `api.Mut`'s value, if it is one.
	fn place(&self, name: &str) -> String {
		match self.mutated.iter().any(|m| m == name) {
			true => format!("{}.value", ident(name)),
			false => ident(name),
		}
	}

	fn step(&mut self, step: &Step, out: &mut String) {
		match step {
			Step::Let(pat, value) => {
				// A value's `&mut self` returns it as the call left it, before what Rust returns.
				if let Some((name, call)) = self.mut_receiver(value) {
					match unref(&value.ty) {
						Type::Unit => {
							self.line(out, &format!("{name} = {call};"));
							return self.bind(pat, "null", &value.ty, out);
						}
						ty => {
							let tmp = self.fresh("t");
							self.line(out, &format!("final {tmp} = {call};"));
							self.line(out, &format!("{name} = {tmp}.$1;"));
							let rest = match ty {
								Type::Tuple(elements) if elements.len() > 1 => {
									let parts: Vec<String> =
										(0..elements.len()).map(|i| format!("{tmp}.${}", i + 2)).collect();
									format!("({})", parts.join(", "))
								}
								_ => format!("{tmp}.$2"),
							};
							return self.bind(pat, &rest, &value.ty, out);
						}
					}
				}
				let v = self.expr(value);
				match pat {
					Pat::Name(name, ty) if self.mutated.contains(name) => {
						self.locals.push((name.clone(), ty.clone()));
						self.line(out, &format!("final {} = api.Mut({v});", ident(name)));
					}
					pat => self.bind(pat, &v, &value.ty, out),
				}
			}
			Step::Print(pieces) => {
				let v = self.pieces(pieces);
				self.line(out, &format!("print({v});"));
			}
			Step::Do(value) => {
				if let Some((name, call)) = self.mut_receiver(value) {
					return match unref(&value.ty) {
						Type::Unit => self.line(out, &format!("{name} = {call};")),
						_ => self.line(out, &format!("{name} = ({call}).$1;")),
					};
				}
				let v = self.lent(value);
				self.line(out, &format!("{v};"));
			}
			Step::Drop(name) => {
				let ty = self.local(name).clone();
				match unref(&ty).base() {
					Type::Object(_) => self.line(out, &format!("{}.close();", self.place(name))),
					ty => panic!("Dart drops no {ty:?} but by the garbage collector"),
				}
			}
			Step::Match { value, ok, err } => self.matched(value, ok, err, out),
		}
	}

	fn local(&self, name: &str) -> &Type {
		match self.locals.iter().rev().find(|(n, _)| n == name) {
			Some((_, ty)) => ty,
			None => panic!("no local {name}"),
		}
	}

	/// A call of a method taking a value's `&mut self` on a local: the local's place, and
	/// the call, which Dart returns the value from as the call left it.
	fn mut_receiver(&mut self, value: &Expr) -> Option<(String, String)> {
		let Kind::Call(id, Some(receiver), args) = &value.kind else {
			return None;
		};
		let f = self.t.function(*id);
		if f.receiver != Some(Receiver::Mut) || !matches!(id.owner, Owner::Record(_)) {
			return None;
		}
		let Kind::Var(name) = &receiver.kind else {
			panic!("a value's &mut self method on anything but a local")
		};
		let call = self.call(*id, Some(receiver), args);
		Some((self.place(name), call))
	}

	/// `pat` bound to `v`, a value of `ty`: a `Bounds`, a class, by its fields.
	fn bind(&mut self, pat: &Pat, v: &str, ty: &Type, out: &mut String) {
		let mut names = Vec::new();
		super::bound(pat, &mut names);
		self.locals.extend(names);
		match pat {
			Pat::Ignore(_) if v == "null" => {}
			Pat::Ignore(_) => self.line(out, &format!("{v};")),
			Pat::Tuple(pats) if matches!(unref(ty).base(), Type::Bounds(_)) => {
				let tmp = self.fresh("b");
				self.line(out, &format!("final {tmp} = {v};"));
				for (pat, end) in pats.iter().zip(["start", "end"]) {
					let target = self.pat(pat);
					self.line(out, &format!("var {target} = {tmp}.{end};"));
				}
			}
			pat => {
				let target = self.pat(pat);
				self.line(out, &format!("var {target} = {v};"));
			}
		}
	}

	fn pat(&self, pat: &Pat) -> String {
		match pat {
			Pat::Name(name, _) => ident(name),
			Pat::Ignore(_) => "_".to_string(),
			Pat::Tuple(pats) => {
				let parts: Vec<String> = pats.iter().map(|p| self.pat(p)).collect();
				match parts.len() {
					1 => format!("({},)", parts[0]),
					_ => format!("({})", parts.join(", ")),
				}
			}
		}
	}

	/// An arm's steps, in a block of their own.
	fn arm(&mut self, arm: &Arm, payload: Option<(&str, &Type)>, out: &mut String) {
		self.depth += 1;
		if let (Some(pat), Some((v, ty))) = (&arm.pat, payload) {
			self.bind(pat, v, ty, out);
		}
		let body = self.steps(&arm.steps);
		out.push_str(&body);
		self.depth -= 1;
	}

	fn matched(&mut self, value: &Expr, ok: &Arm, err: &Arm, out: &mut String) {
		match &value.ty {
			Type::Option(inner) => {
				let v = self.expr(value);
				let tmp = self.fresh("o");
				self.line(out, &format!("final {tmp} = {v};"));
				self.line(out, &format!("if ({tmp} != null) {{"));
				self.arm(ok, Some((&tmp, inner)), out);
				self.line(out, "} else {");
				self.arm(err, None, out);
				self.line(out, "}");
			}
			Type::Result(ok_ty, err_ty) => {
				let caught = self.caught(err_ty);
				// Called in a closure, where the script awaits nothing.
				self.closures += 1;
				let v = self.expr(value);
				self.closures -= 1;
				let result = self.fresh("r");
				self.line(out, &format!("final {result} = _try(() => {v}, (e) => e is {caught});"));
				self.line(out, &format!("switch ({result}) {{"));
				let ok_value = self.fresh("v");
				self.line(out, &format!("  case _Ok(value: final {ok_value}):"));
				self.depth += 1;
				self.arm(ok, Some((&ok_value, ok_ty)), out);
				self.depth -= 1;
				let e = self.fresh("e");
				self.line(out, &format!("  case _Err(error: final {e}):"));
				self.depth += 1;
				let error = self.error_of(&format!("({e} as {caught})"), err_ty);
				self.arm(err, Some((&error, err_ty)), out);
				self.depth -= 1;
				self.line(out, "}");
			}
			ty => panic!("a match on {ty:?}"),
		}
	}

	/// The class a call failing with `err` throws: the error itself, if an exported type,
	/// or else the exception the binding names for it.
	fn caught(&self, err: &Type) -> String {
		match unref(err).base() {
			ty @ (Type::Record(_) | Type::Enum(_) | Type::Object(_)) => {
				format!("api.{}", self.t.type_name(LANGUAGE, ty))
			}
			ty => format!(
				"api.{}",
				self.t.named(LANGUAGE, &format!("error {}", self.error_key(ty)))
			),
		}
	}

	/// The error Rust returned, of the thrown `v`.
	fn error_of(&self, v: &str, err: &Type) -> String {
		match unref(err).base() {
			Type::Record(_) | Type::Enum(_) | Type::Object(_) => v.to_string(),
			_ => format!("{v}.error"),
		}
	}

	/// The key `api.json` names an error type's exception by.
	fn error_key(&self, ty: &Type) -> String {
		match unref(ty).base() {
			Type::String | Type::Str => "string".to_string(),
			Type::Bool => "bool".to_string(),
			Type::Int(int) => int.rust().to_string(),
			Type::Bounds(int) => format!("bounds_{}", int.rust()),
			Type::Range(int) => format!("range_{}", int.rust()),
			Type::List(_, item) => format!("vec_{}", self.error_key(item)),
			Type::Option(inner) => format!("option_{}", self.error_key(inner)),
			ty => panic!("Dart's exception of an error of {ty:?}"),
		}
	}

	/// The Dart of a format's pieces: a string literal, interpolating each value.
	fn pieces(&mut self, pieces: &[Piece]) -> String {
		let mut out = String::from("'");
		for piece in pieces {
			match piece {
				Piece::Text(text) => out.push_str(&escape(text)),
				Piece::Value(e, spec) => {
					let v = self.lent(e);
					let shown = self.show(&v, &e.ty, *spec);
					let _ = write!(out, "${{{shown}}}");
				}
			}
		}
		out.push('\'');
		out
	}

	/// `v`, a value of `ty`, as a `String` the way Rust formats it with `spec`.
	fn show(&mut self, v: &str, ty: &Type, spec: Spec) -> String {
		let ty = unref(ty).base();
		match spec {
			Spec::Hex(0) => return format!("{v}.toRadixString(16)"),
			Spec::Hex(width) => return format!("{v}.toRadixString(16).padLeft({width}, '0')"),
			Spec::Padded(width) => return format!("{}.padLeft({width}, '0')", self.decimal(v, ty)),
			Spec::Debug => return self.debug(v, ty),
			Spec::Display => {}
		}
		match ty {
			Type::Bool | Type::String | Type::Str | Type::Char => v.to_string(),
			Type::Int(_) | Type::U128 | Type::NonZero(_) => self.decimal(v, ty),
			Type::Object(_) | Type::Record(_) | Type::Enum(_) => match self.text_of(ty) {
				Some(text) => format!("{v}.{text}()"),
				None => format!("{v}.toString()"),
			},
			ty => panic!("Display of {ty:?}"),
		}
	}

	/// An integer's decimal digits: an `api.U64`'s unsigned.
	fn decimal(&self, v: &str, ty: &Type) -> String {
		match ty {
			Type::Int(int) | Type::NonZero(int) if wide(*int) => format!("{v}.toRadixString(10)"),
			Type::Int(_) | Type::NonZero(_) | Type::U128 => format!("{v}.toString()"),
			ty => panic!("the digits of {ty:?}"),
		}
	}

	/// The name of a type's `Access::Text` method, which Dart names apart from `toString`.
	fn text_of(&self, ty: &Type) -> Option<&str> {
		let Type::Object(i) = ty else { return None };
		let o = &self.t.api.objects[*i];
		let index = o.methods.iter().position(|m| m.access == Access::Text)?;
		Some(self.t.name(
			LANGUAGE,
			FnId {
				owner: Owner::Object(*i),
				index,
			},
		))
	}

	/// `v` as Rust's `Debug` prints a value of `ty`.
	fn debug(&mut self, v: &str, ty: &Type) -> String {
		match unref(ty).base() {
			Type::Bool => format!("{v}.toString()"),
			ty @ (Type::Int(_) | Type::U128 | Type::NonZero(_)) => self.decimal(v, ty),
			Type::String | Type::Str => format!("_quoted({v})"),
			Type::Option(inner) => {
				let x = self.fresh("x");
				let show = self.debug(&x, inner);
				format!("_option({v}, ({x}) => {show})")
			}
			Type::Vec | Type::Slice | Type::Bytes => format!("_list({v}, (x) => x.toString())"),
			Type::Array(_) => format!("_list({v}.toBytes(), (x) => x.toString())"),
			Type::List(_, item) => {
				let x = self.fresh("x");
				let show = self.debug(&x, item);
				format!("_list({v}, ({x}) => {show})")
			}
			Type::Tuple(elements) => {
				let parts: Vec<String> = (0..elements.len())
					.map(|i| format!("${{{}}}", self.debug(&format!("{v}.${}", i + 1), &elements[i])))
					.collect();
				let tail = if parts.len() == 1 { ",)" } else { ")" };
				format!("'({}{tail}'", parts.join(", "))
			}
			Type::Bound(int) => {
				let class = format!("api.Bound{}", int_name(*int));
				let value = self.decimal("value", &Type::Int(*int));
				format!(
					"switch ({v}) {{ {class}Included(:final value) => 'Included(${{{value}}})', {class}Excluded(:final value) => 'Excluded(${{{value}}})', {class}Unbounded() => 'Unbounded' }}"
				)
			}
			Type::Range(_) => format!("{v}.toString()"),
			Type::Record(_) | Type::Enum(_) | Type::Object(_) => format!("{v}.debug()"),
			ty => panic!("Debug of {ty:?}"),
		}
	}

	fn expr(&mut self, e: &Expr) -> String {
		self.value(e, &e.ty)
	}

	/// The Dart of `e`, passed where `want` is taken.
	fn value(&mut self, e: &Expr, want: &Type) -> String {
		match &e.kind {
			Kind::Unit => "null".to_string(),
			Kind::Bool(b) => b.to_string(),
			Kind::Int(n) => {
				let ty = match plain(&e.ty) {
					ty @ (Type::Int(_) | Type::NonZero(_) | Type::U128 | Type::F32 | Type::F64) => ty,
					_ => plain(want),
				};
				int(*n, ty)
			}
			Kind::Float(f) if !f.is_finite() || (*f == 0.0 && f.is_sign_negative()) => {
				format!("_f64(0x{:x})", f.to_bits())
			}
			Kind::Float(f) => format!("{f:?}"),
			Kind::Char(c) => string(&c.to_string()),
			Kind::Str(s) => match plain(want) {
				Type::Path | Type::PathBuf => format!("api.PathBuf({})", string(s)),
				_ => string(s),
			},
			Kind::Bytes(b) => {
				let bytes: Vec<String> = b.iter().map(|b| b.to_string()).collect();
				match plain(want) {
					Type::Array(n) => format!("api.ArrayU8x{n}([{}])", bytes.join(", ")),
					_ => format!("_bytes([{}])", bytes.join(", ")),
				}
			}
			Kind::Duration(secs, nanos) => {
				format!(
					"api.RustDuration({}, {nanos})",
					int(Num::new((*secs).into()), &Type::Int(Int::U64))
				)
			}
			Kind::Socket(std::net::SocketAddr::V4(a)) => {
				format!("api.SocketAddrV4({}, {})", ip(&std::net::IpAddr::V4(*a.ip())), a.port())
			}
			Kind::Socket(std::net::SocketAddr::V6(a)) => format!(
				"api.SocketAddrV6({}, {}, flowinfo: {}, scopeId: {})",
				ip(&std::net::IpAddr::V6(*a.ip())),
				a.port(),
				a.flowinfo(),
				a.scope_id()
			),
			Kind::Ip(addr) => ip(addr),
			Kind::None => "null".to_string(),
			Kind::Some(inner) => {
				let want = match unref(want).base() {
					Type::Option(inner) => (**inner).clone(),
					_ => inner.ty.clone(),
				};
				self.value(inner, &want)
			}
			Kind::Tuple(elements) => self.tuple(e, elements, want),
			Kind::List(items) => self.list(items, want),
			Kind::Record(r, fields) => {
				let record = &self.t.api.records[*r];
				assert!(
					record.opaque.is_none(),
					"Dart makes no opaque record but by its functions"
				);
				let name = self.t.type_name(LANGUAGE, &Type::Record(*r)).to_string();
				let tuple = record.tuple;
				let fields_of: Vec<(String, Type)> =
					record.fields.iter().map(|f| (field(&f.name), f.ty.clone())).collect();
				let args = self.fields(fields, &fields_of, tuple);
				format!("api.{name}({args})")
			}
			Kind::Variant(en, v, fields) => {
				let enumeration = &self.t.api.enums[*en];
				let variant = &enumeration.variants[*v];
				let name = self.t.type_name(LANGUAGE, &Type::Enum(*en)).to_string();
				if enumeration.variants.iter().all(|v| v.fields.is_empty()) {
					return format!(
						"api.{name}.{}",
						member(
							&snake(&variant.name),
							&["values", "index", "name", "compareTo", "partialCompareTo"]
						)
					);
				}
				let key = format!("type {} / variant {}", self.t.rust(&Type::Enum(*en)), variant.name);
				let class = self.t.named(LANGUAGE, &key).to_string();
				let tuple = variant.tuple;
				let fields_of: Vec<(String, Type)> =
					variant.fields.iter().map(|f| (field(&f.name), f.ty.clone())).collect();
				let args = self.fields(fields, &fields_of, tuple);
				format!("api.{class}({args})")
			}
			Kind::Bound(kind, value) => {
				let Type::Bound(int) = unref(&e.ty) else {
					panic!("a bound of {:?}", e.ty)
				};
				let class = format!("api.Bound{}", int_name(*int));
				match (kind, value) {
					(BoundKind::Unbounded, _) => format!("{class}Unbounded()"),
					(BoundKind::Included, Some(v)) => format!("{class}Included({})", self.value(v, &Type::Int(*int))),
					(BoundKind::Excluded, Some(v)) => format!("{class}Excluded({})", self.value(v, &Type::Int(*int))),
					_ => unreachable!(),
				}
			}
			Kind::Default => {
				let name = self.t.type_name(LANGUAGE, unref(&e.ty).base()).to_string();
				format!("api.{name}.default_()")
			}
			Kind::Var(name) => match self.mutated.contains(name) {
				// Passed `&mut`, the wrapper itself; read, its value.
				true if matches!(want, Type::Ref(Mutability::Mut, _)) => ident(name),
				_ => self.place(name),
			},
			Kind::Const(c) => format!(
				"api.{}",
				self.t
					.named(LANGUAGE, &format!("const {}", self.t.api.constants[*c].path.join("::")))
			),
			Kind::Call(id, receiver, args) => self.call(*id, receiver.as_deref(), args),
			Kind::Field(base, i) => {
				let b = self.expr(base);
				let Type::Record(r) = unref(&base.ty).base() else {
					panic!("a field of {:?}", base.ty)
				};
				format!("{b}.{}", field(&self.t.api.records[*r].fields[*i].name))
			}
			Kind::Element(base, i) => {
				let b = self.expr(base);
				match unref(&base.ty).base() {
					Type::Range(_) => format!("{b}.{}", if *i == 0 { "start" } else { "end" }),
					_ => format!("{b}.${}", i + 1),
				}
			}
			Kind::Clone(inner) => {
				let v = self.lent(inner);
				match unref(&inner.ty).base() {
					Type::Object(_) => format!("{v}.clone()"),
					ty @ Type::List(..) => match block(ty) {
						Some("<api.U64>") | None => format!("List.of({v})"),
						Some(list) => format!("{list}.fromList({v})"),
					},
					Type::Vec | Type::Slice | Type::Bytes => format!("Uint8List.fromList({v})"),
					// Every other value is immutable in Dart, so the value is its clone.
					_ => v,
				}
			}
			Kind::Unwrap(inner) => {
				let v = self.expr(inner);
				match &inner.ty {
					Type::Result(..) => v,
					_ => format!("_some({v})"),
				}
			}
			Kind::UnwrapErr(inner) => {
				let Type::Result(_, err) = &inner.ty else {
					panic!("unwrap_err of {:?}", inner.ty)
				};
				let caught = self.caught(err);
				self.closures += 1;
				let v = self.expr(inner);
				self.closures -= 1;
				self.error_of(&format!("_err<{caught}>(() => {v})"), err)
			}
			Kind::IsErr(inner) => {
				let Type::Result(_, err) = &inner.ty else {
					panic!("is_err of {:?}", inner.ty)
				};
				let caught = self.caught(err);
				self.closures += 1;
				let v = self.expr(inner);
				self.closures -= 1;
				format!("_fails<{caught}>(() => {v})")
			}
			Kind::IsSome(inner) => format!("({} != null)", self.lent(inner)),
			Kind::IsNone(inner) => format!("({} == null)", self.lent(inner)),
			Kind::Binary(op, a, b) => self.binary(*op, a, b, &e.ty),
			Kind::Closure(params, steps, value) => self.closure(params, steps, value),
			Kind::Local(name, args) => {
				let args: Vec<String> = args.iter().map(|a| self.expr(a)).collect();
				let name = match name.starts_with(char::is_uppercase) {
					true => name.clone(),
					false => camel(name),
				};
				let call = format!("local.{name}({})", args.join(", "));
				// A hook run as a step may be async: the script awaits it.
				match (unref(&e.ty), self.closures) {
					(Type::Unit, 0) => format!("await {call}"),
					(Type::Unit, _) => panic!("a hook {name} in a closure, which Dart cannot await"),
					_ => call,
				}
			}
			Kind::Format(pieces) => self.pieces(pieces),
			Kind::Join { items, pat, body, sep } => {
				let items = self.expr(items);
				let x = self.fresh("x");
				let target = self.pat(pat);
				self.closures += 1;
				let mut block = String::new();
				let body = self.returned(body, &mut block);
				self.closures -= 1;
				let block = block.lines().map(str::trim).collect::<Vec<_>>().join(" ");
				format!(
					"{items}.map(({x}) {{ var {target} = {x}; {block} return {body}; }}).join({})",
					string(sep)
				)
			}
			Kind::MapOr {
				option,
				none,
				pat,
				some,
			} => {
				let option = self.expr(option);
				let none = self.expr(none);
				let x = self.fresh("x");
				let target = self.pat(pat);
				self.closures += 1;
				let mut block = String::new();
				let some = self.returned(some, &mut block);
				self.closures -= 1;
				let block = block.lines().map(str::trim).collect::<Vec<_>>().join(" ");
				format!(
					"(switch ({option}) {{ final {x}? => (() {{ var {target} = {x}; {block} return {some}; }})(), null => {none} }})"
				)
			}
			Kind::Utf8(inner) => format!("_text({})", self.expr(inner)),
			Kind::Len(inner) => {
				let v = self.expr(inner);
				match unref(&inner.ty).base() {
					Type::String | Type::Str => format!("api.U64(utf8.encode({v}).length)"),
					_ => format!("api.U64({v}.length)"),
				}
			}
		}
	}

	/// A record's or a variant's fields as the arguments of its constructor: positional
	/// for a tuple's, or else named.
	fn fields(&mut self, values: &[Expr], fields: &[(String, Type)], tuple: bool) -> String {
		let args: Vec<String> = values
			.iter()
			.zip(fields)
			.map(|(v, (name, ty))| {
				let v = self.value(v, ty);
				match tuple {
					true => v,
					false => format!("{name}: {v}"),
				}
			})
			.collect();
		args.join(", ")
	}

	/// `a op b`, with Rust's overflow panic, of `ty`.
	fn binary(&mut self, op: Op, a: &Expr, b: &Expr, ty: &Type) -> String {
		let operand = unref(&a.ty).base().clone();
		let (a, b) = {
			let (va, vb) = (self.value(a, &operand), self.value(b, &operand));
			(self.temp(va, a), self.temp(vb, b))
		};
		let compare = |op: &str| format!("({a} {op} {b})");
		match op {
			Op::Eq => compare("=="),
			Op::Ne => compare("!="),
			Op::Lt => compare("<"),
			Op::Le => compare("<="),
			Op::Gt => compare(">"),
			Op::Ge => compare(">="),
			Op::Cmp => format!("{a}.compareTo({b})"),
			Op::And => compare("&&"),
			Op::Add | Op::Sub | Op::Mul | Op::Rem => {
				let Type::Int(int) = unref(ty).base() else {
					panic!("arithmetic of {ty:?}")
				};
				let sym = match op {
					Op::Add => "+",
					Op::Sub => "-",
					Op::Mul => "*",
					_ => "%",
				};
				match (int, op) {
					(int, _) if wide(*int) => {
						let big = match op {
							Op::Rem => format!("{a}.toBigInt().remainder({b}.toBigInt())"),
							_ => format!("{a}.toBigInt() {sym} {b}.toBigInt()"),
						};
						format!("_u64({big})")
					}
					// Rust's `%` truncates, as Dart's `remainder` does, never past either operand.
					(_, Op::Rem) => format!("{a}.remainder({b})"),
					(Int::I64 | Int::Isize, _) => format!("_i64(BigInt.from({a}) {sym} BigInt.from({b}))"),
					(int, _) => {
						let (lo, hi) = match int.signed() {
							true => (-(1i64 << (int.bits() - 1)), (1i64 << (int.bits() - 1)) - 1),
							false => (0, (1i64 << int.bits()) - 1),
						};
						format!("_int({a} {sym} {b}, {lo}, {hi})")
					}
				}
			}
		}
	}

	fn tuple(&mut self, e: &Expr, elements: &[Expr], want: &Type) -> String {
		match unref(&e.ty).base() {
			Type::Range(int) => {
				let item = Type::Int(*int);
				let (a, b) = (self.value(&elements[0], &item), self.value(&elements[1], &item));
				format!("api.Range{}({a}, {b})", int_name(*int))
			}
			Type::Bounds(int) => {
				let bound = Type::Bound(*int);
				let (a, b) = (self.value(&elements[0], &bound), self.value(&elements[1], &bound));
				format!("api.Bounds{}({a}, {b})", int_name(*int))
			}
			_ => {
				let wants: Vec<Type> = match unref(want).base() {
					Type::Tuple(ts) => ts.clone(),
					_ => elements.iter().map(|e| e.ty.clone()).collect(),
				};
				let parts: Vec<String> = elements.iter().zip(&wants).map(|(e, t)| self.value(e, t)).collect();
				match parts.len() {
					1 => format!("({},)", parts[0]),
					_ => format!("({})", parts.join(", ")),
				}
			}
		}
	}

	fn list(&mut self, items: &[Expr], want: &Type) -> String {
		let want = unref(want).base();
		let item = match want {
			Type::List(_, item) => (**item).clone(),
			ty => panic!("a list as {ty:?}"),
		};
		let parts: Vec<String> = items.iter().map(|e| self.value(e, &item)).collect();
		match block(want) {
			Some(typed @ "<api.U64>") => format!("{typed}[{}]", parts.join(", ")),
			Some(typed) => format!("{typed}.fromList([{}])", parts.join(", ")),
			None => format!("[{}]", parts.join(", ")),
		}
	}

	fn closure(&mut self, params: &[Pat], steps: &[Step], value: &Expr) -> String {
		let mut names = Vec::new();
		params.iter().for_each(|p| super::bound(p, &mut names));
		self.locals.extend(names);
		// Dart's parameters bind no tuples: each binds a fresh name, which the body takes apart.
		let mut binds = Vec::new();
		let params: Vec<String> = params
			.iter()
			.map(|p| match p {
				Pat::Tuple(_) => {
					let x = self.fresh("p");
					binds.push(format!("var {} = {x};", self.pat(p)));
					x
				}
				p => self.pat(p),
			})
			.collect();
		self.closures += 1;
		self.depth += 1;
		let mut body = String::new();
		for bind in &binds {
			self.line(&mut body, bind);
		}
		body.push_str(&self.steps(steps));
		let returned = match value.kind {
			Kind::Unit => None,
			_ => Some(self.returned(value, &mut body)),
		};
		self.depth -= 1;
		self.closures -= 1;
		match returned {
			Some(v) if body.is_empty() => format!("({}) => {v}", params.join(", ")),
			Some(v) => {
				self.depth += 1;
				self.line(&mut body, &format!("return {v};"));
				self.depth -= 1;
				format!("({}) {{\n{body}{}}}", params.join(", "), self.indent())
			}
			None => format!("({}) {{\n{body}{}}}", params.join(", "), self.indent()),
		}
	}

	/// The value a closure's body returns, in a scope of its own temporaries, which close
	/// before it returns.
	fn returned(&mut self, value: &Expr, body: &mut String) -> String {
		let (temps, v) = self.scoped(|e| e.expr(value));
		let Some(temps) = temps else { return v };
		let result = self.fresh("v");
		self.line(body, &format!("final {temps} = <Object?>[];"));
		self.line(body, &format!("final {result} = {v};"));
		self.line(body, &format!("_close({temps});"));
		result
	}

	/// A call of `id`: a function of the library, a constructor or a static member of a
	/// type, a method, or a field's getter or setter.
	fn call(&mut self, id: FnId, receiver: Option<&Expr>, args: &[Expr]) -> String {
		let f = self.t.function(id);
		assert!(!f.future, "a future of {}, which only a hook awaits", f.path.join("::"));
		let name = self.t.name(LANGUAGE, id).to_string();
		let params: Vec<Type> = f.params.iter().map(|p| p.ty.clone()).collect();
		let access = f.access;
		// A unit crosses as nothing, so Dart takes no parameter of it.
		let receives = f.receiver;
		let args: Vec<String> = (args.iter().zip(&params))
			.filter(|(_, ty)| !matches!(unref(ty), Type::Unit))
			.map(|(a, ty)| {
				let v = self.value(a, ty);
				match borrows(ty) {
					true => self.temp(v, a),
					false => v,
				}
			})
			.collect();
		let args = args.join(", ");
		let owner = self
			.t
			.owner(id)
			.map(|ty| format!("api.{}", self.t.type_name(LANGUAGE, &ty)));
		let (target, member) = match (receiver, name.split_once(' ')) {
			(Some(r), _) => {
				let v = self.expr(r);
				let v = match receives {
					Some(Receiver::Owned) => v,
					_ => self.temp(v, r),
				};
				(v, name.rsplit(' ').next().unwrap().to_string())
			}
			(None, Some(("new", ""))) => return format!("{}({args})", owner.expect("a constructor of a type")),
			(None, Some(("new" | "static", member))) => (owner.expect("a static member of a type"), member.to_string()),
			(None, None) => (owner.unwrap_or_else(|| "api".to_string()), name.clone()),
			(None, Some(_)) => panic!("a Dart name {name}"),
		};
		match access {
			Access::Field { set: false, .. } | Access::Const => format!("{target}.{member}"),
			_ => format!("{target}.{member}({args})"),
		}
	}
}

/// Whether a parameter of `ty` only borrows the object passed.
fn borrows(ty: &Type) -> bool {
	match ty {
		Type::Ref(_, inner) => matches!(plain(inner), Type::Object(_)),
		Type::Option(inner) => borrows(inner),
		_ => false,
	}
}

/// A Dart integer literal of `n`, a value of `ty`.
fn int(n: Num, ty: &Type) -> String {
	match ty {
		Type::Int(int) | Type::NonZero(int) if wide(*int) => match n.magnitude <= i64::MAX as u128 && !n.negative {
			true => format!("api.U64({})", n.magnitude),
			false => format!("api.U64.fromBits(0x{:x})", n.bits()),
		},
		Type::U128 => format!("BigInt.parse('{n}')"),
		Type::F32 | Type::F64 => format!("{n}.0"),
		_ if n.negative && n.magnitude == 1 << 63 => "-0x8000000000000000".to_string(),
		_ => n.to_string(),
	}
}

/// An IP address of its octets.
fn ip(addr: &std::net::IpAddr) -> String {
	let (class, octets) = match addr {
		std::net::IpAddr::V4(v4) => ("Ipv4Addr", v4.octets().to_vec()),
		std::net::IpAddr::V6(v6) => ("Ipv6Addr", v6.octets().to_vec()),
	};
	let octets: Vec<String> = octets.iter().map(|o| o.to_string()).collect();
	format!("api.{class}.fromOctets([{}])", octets.join(", "))
}

/// Text escaped for a single-quoted Dart string.
fn escape(s: &str) -> String {
	let mut out = String::new();
	for c in s.chars() {
		match c {
			'\'' | '\\' | '$' => {
				out.push('\\');
				out.push(c);
			}
			'\n' => out.push_str("\\n"),
			'\t' => out.push_str("\\t"),
			'\r' => out.push_str("\\r"),
			c if (c as u32) < 0x20 || c as u32 == 0x7f => {
				let _ = write!(out, "\\u{{{:x}}}", c as u32);
			}
			c => out.push(c),
		}
	}
	out
}

/// A Dart string literal.
fn string(s: &str) -> String {
	format!("'{}'", escape(s))
}
