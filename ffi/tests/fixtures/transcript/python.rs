//! The transcript as Python: a module whose `script()` runs it through the generated
//! Python, calling `tests/api.py`'s own functions, which it imports as `__main__`.

use std::fmt::Write;

use super::ir::{Access, Collection, Int, Mutability, Receiver, Type};
use super::{Arm, BoundKind, Expr, FnId, Kind, Op, Owner, Pat, Piece, Spec, Step, Transcript, each_expr, unref, walk};

const LANGUAGE: &str = "Python";

/// Helpers the script calls, each for a value Python shapes apart from Rust.
const PRELUDE: &str = r#"# Generated from fixtures/api/examples/transcript.rs by tests/fixtures/transcript.

import array
import copy
import ipaddress
import struct

import __main__ as local
import api


def _flag(value):
	return "true" if value else "false"


def _text(view):
	"""Bytes as text, releasing a view Rust lent."""
	if isinstance(view, memoryview):
		with view:
			return bytes(view).decode()
	return bytes(view).decode()


def _bytes(view):
	if isinstance(view, memoryview):
		with view:
			return bytes(view)
	return bytes(view)


def _quoted(s):
	"""A string as Rust's `Debug` prints it."""
	out = '"'
	for c in s:
		match c:
			case '"' | "\\":
				out += "\\" + c
			case "\n":
				out += "\\n"
			case "\t":
				out += "\\t"
			case "\r":
				out += "\\r"
			case c:
				out += c
	return out + '"'


def _bound(b):
	match b.kind:
		case api.Bound.INCLUDED:
			return f"Included({b.value})"
		case api.Bound.EXCLUDED:
			return f"Excluded({b.value})"
		case api.Bound.UNBOUNDED:
			return "Unbounded"


def _list(items, show):
	return "[" + ", ".join(show(item) for item in items) + "]"


def _option(value, show):
	return "None" if value is None else f"Some({show(value)})"


def _err(call, exception):
	"""The error of a call that must fail."""
	try:
		call()
	except exception as e:
		return e.error
	raise AssertionError("the call did not fail")


def _fails(call, exception):
	try:
		call()
	except exception:
		return True
	return False


def _cmp(a, b):
	"""Rust's `Ord` of two values, through the comparisons Python has."""
	return (a > b) - (a < b)


def _f64(bits):
	return struct.unpack("<d", bits.to_bytes(8, "little"))[0]


def _some(value):
	assert value is not None, "unwrap of None"
	return value
"#;

/// The Python module of `t`'s script.
pub(crate) fn emit(t: &Transcript) -> String {
	let mut e = Emitter {
		t,
		out: String::new(),
		depth: 1,
		mutated: Vec::new(),
		fresh: 0,
	};
	mutated(&t.steps, t, &mut e.mutated);
	e.out.push_str(PRELUDE);
	e.out.push_str("\n\ndef script():\n");
	let body = e.steps(&t.steps);
	e.out.push_str(&body);
	e.out
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
	out: String,
	depth: usize,
	/// The locals each an `api.Mut`.
	mutated: Vec<String>,
	fresh: usize,
}

/// Python's keywords, which no name may be.
const KEYWORDS: &[&str] = &[
	"from", "in", "is", "not", "and", "or", "None", "True", "False", "lambda", "def", "del", "pass", "with", "yield",
	"async", "await", "class", "global", "import", "return", "try", "except", "raise", "while", "for", "if", "else",
	"elif", "break", "continue", "as", "assert", "finally", "nonlocal", "match", "case", "type",
];

/// What the script itself calls, which no local of its may shadow.
const BUILTINS: &[&str] = &[
	"list",
	"bytes",
	"str",
	"len",
	"range",
	"iter",
	"format",
	"print",
	"repr",
	"copy",
	"array",
	"struct",
	"ipaddress",
	"api",
	"local",
	"id",
	"hex",
	"min",
	"max",
	"sum",
	"next",
	"map",
	"filter",
	"bool",
	"int",
	"float",
	"tuple",
	"set",
	"dict",
	"object",
	"input",
	"open",
	"round",
	"all",
	"any",
	"zip",
	"slice",
	"vars",
	"bin",
	"oct",
	"ord",
	"chr",
	"abs",
	"hash",
	"callable",
];

/// A local's name in Python.
fn ident(name: &str) -> String {
	match KEYWORDS.contains(&name) || BUILTINS.contains(&name) {
		true => format!("{name}_"),
		false => name.to_string(),
	}
}

impl Emitter<'_> {
	fn indent(&self) -> String {
		"\t".repeat(self.depth)
	}

	fn fresh(&mut self, what: &str) -> String {
		self.fresh += 1;
		format!("_{what}{}", self.fresh)
	}

	fn steps(&mut self, steps: &[Step]) -> String {
		let mut out = String::new();
		for step in steps {
			self.step(step, &mut out);
		}
		if out.is_empty() {
			let _ = writeln!(out, "{}pass", self.indent());
		}
		out
	}

	fn line(&self, out: &mut String, line: &str) {
		let _ = writeln!(out, "{}{line}", self.indent());
	}

	fn step(&mut self, step: &Step, out: &mut String) {
		match step {
			Step::Let(pat, value) => {
				// A value's `&mut self` returns it as the call left it, before what Rust returns.
				if let Some((name, call)) = self.mut_receiver(value) {
					let target = match pat {
						Pat::Ignore(_) => "_".to_string(),
						pat => self.pat(pat),
					};
					let line = match unref(&value.ty) {
						Type::Unit => format!("{name} = {call}"),
						Type::Tuple(elements) if elements.len() > 1 => format!("{name}, *{target} = {call}"),
						_ => format!("{name}, {target} = {call}"),
					};
					let line = match pat {
						Pat::Tuple(_) => {
							let tmp = self.fresh("t");
							self.line(out, &format!("{name}, {tmp} = {call}"));
							format!("{target} = {tmp}")
						}
						_ => line,
					};
					return self.line(out, &line);
				}
				let v = self.expr(value, out);
				match pat {
					Pat::Name(name, _) if self.mutated.contains(name) => {
						self.line(out, &format!("{} = api.Mut({v})", ident(name)))
					}
					pat => self.bind(pat, &v, &value.ty, out),
				}
			}
			Step::Print(pieces) => {
				let v = self.pieces(pieces, out);
				self.line(out, &format!("print({v})"));
			}
			Step::Do(value) => {
				if let Some((name, call)) = self.mut_receiver(value) {
					return match unref(&value.ty) {
						Type::Unit => self.line(out, &format!("{name} = {call}")),
						_ => self.line(out, &format!("{name}, _ = {call}")),
					};
				}
				let v = self.expr(value, out);
				self.line(out, &v);
			}
			Step::Drop(name) => self.line(out, &format!("del {}", ident(name))),
			Step::Match { value, ok, err } => self.matched(value, ok, err, out),
		}
	}

	/// A call of a method taking a value's `&mut self` on a local: the local's name, and
	/// the call, which Python returns the value from as the call left it.
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
		let mut scratch = String::new();
		let call = self.call(*id, Some(receiver), args, &mut scratch);
		assert!(
			scratch.is_empty(),
			"a value's &mut self method taking a closure of steps"
		);
		Some((ident(name), call))
	}

	/// `pat` bound to `v`, a value of `ty`: bounds, which Python keeps as a record, by its
	/// fields.
	fn bind(&mut self, pat: &Pat, v: &str, ty: &Type, out: &mut String) {
		match pat {
			Pat::Tuple(pats) if matches!(unref(ty).base(), Type::Bounds(_)) => {
				let (a, b) = (self.pat(&pats[0]), self.pat(&pats[1]));
				self.line(out, &format!("{a}, {b} = {v}.start, {v}.end"));
			}
			pat => {
				let target = self.pat(pat);
				self.line(out, &format!("{target} = {v}"));
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

	fn matched(&mut self, value: &Expr, ok: &Arm, err: &Arm, out: &mut String) {
		match &value.ty {
			Type::Option(inner) => {
				let v = self.expr(value, out);
				let tmp = self.fresh("o");
				self.line(out, &format!("{tmp} = {v}"));
				self.line(out, &format!("if {tmp} is not None:"));
				self.depth += 1;
				if let Some(pat) = &ok.pat {
					self.bind(pat, &tmp, inner, out);
				}
				let body = self.steps(&ok.steps);
				out.push_str(&body);
				self.depth -= 1;
				self.line(out, "else:");
				self.depth += 1;
				let body = self.steps(&err.steps);
				out.push_str(&body);
				self.depth -= 1;
			}
			Type::Result(ok_ty, err_ty) => {
				let exception = self.exception(err_ty);
				self.line(out, "try:");
				self.depth += 1;
				let mut pre = String::new();
				let v = self.expr(value, &mut pre);
				out.push_str(&pre);
				let ok_value = self.fresh("v");
				self.line(out, &format!("{ok_value} = {v}"));
				self.depth -= 1;
				let e = self.fresh("e");
				self.line(out, &format!("except api.{exception} as {e}:"));
				self.depth += 1;
				if let Some(pat) = &err.pat {
					self.bind(pat, &format!("{e}.error"), err_ty, out);
				}
				let body = self.steps(&err.steps);
				out.push_str(&body);
				self.depth -= 1;
				self.line(out, "else:");
				self.depth += 1;
				if let Some(pat) = &ok.pat {
					self.bind(pat, &ok_value, ok_ty, out);
				}
				let body = self.steps(&ok.steps);
				out.push_str(&body);
				self.depth -= 1;
			}
			ty => panic!("a match on {ty:?}"),
		}
	}

	/// The exception a call failing with `err` raises.
	fn exception(&self, err: &Type) -> String {
		format!("{}Exception", self.error_name(err))
	}

	fn error_name(&self, ty: &Type) -> String {
		match unref(ty).base() {
			Type::String | Type::Str => "String".to_string(),
			Type::Bounds(int) => format!("Bounds{}", int_name(*int)),
			Type::List(_, item) => format!("List{}", self.error_name(item)),
			ty @ (Type::Record(_) | Type::Enum(_) | Type::Object(_)) => self.t.type_name(LANGUAGE, ty).to_string(),
			ty => panic!("an error of {ty:?}"),
		}
	}

	/// The Python of a format's pieces: a string expression.
	fn pieces(&mut self, pieces: &[Piece], out: &mut String) -> String {
		let mut parts = Vec::new();
		for piece in pieces {
			match piece {
				Piece::Text(text) => parts.push(string(text)),
				Piece::Value(e, spec) => {
					let v = self.expr(e, out);
					parts.push(self.show(&v, &e.ty, *spec));
				}
			}
		}
		match parts.len() {
			0 => "\"\"".to_string(),
			_ => parts.join(" + "),
		}
	}

	/// `v`, a value of `ty`, as a `str` the way Rust formats it with `spec`.
	fn show(&mut self, v: &str, ty: &Type, spec: Spec) -> String {
		let ty = unref(ty).base();
		match spec {
			Spec::Hex(0) => return format!("format({v}, \"x\")"),
			Spec::Hex(width) => return format!("format({v}, \"0{width}x\")"),
			Spec::Padded(width) => return format!("format({v}, \"0{width}\")"),
			Spec::Debug => return self.debug(v, ty),
			Spec::Display => {}
		}
		match ty {
			Type::Bool => format!("_flag({v})"),
			Type::String | Type::Str | Type::Char => v.to_string(),
			Type::Int(_) | Type::U128 | Type::NonZero(_) => format!("str({v})"),
			Type::Object(_) | Type::Record(_) | Type::Enum(_) => match self.text_of(ty) {
				Some(text) => format!("{v}.{text}()"),
				None => format!("str({v})"),
			},
			ty => panic!("Display of {ty:?}"),
		}
	}

	/// The name of a type's `Access::Text` method, which Python names its `Display`.
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
			Type::Bool => format!("_flag({v})"),
			Type::Int(_) | Type::U128 | Type::NonZero(_) => format!("str({v})"),
			Type::String | Type::Str => format!("_quoted({v})"),
			Type::Option(inner) => {
				let x = self.fresh("x");
				let show = self.debug(&x, inner);
				format!("_option({v}, lambda {x}: {show})")
			}
			Type::Vec | Type::Slice | Type::Bytes | Type::Array(_) => format!("str(list(_bytes({v})))"),
			Type::List(_, item) => {
				let x = self.fresh("x");
				let show = self.debug(&x, item);
				format!("_list({v}, lambda {x}: {show})")
			}
			Type::Tuple(elements) => {
				let parts: Vec<String> = (0..elements.len())
					.map(|i| self.debug(&format!("{v}[{i}]"), &elements[i]))
					.collect();
				let tail = if parts.len() == 1 { ",)" } else { ")" };
				format!("\"(\" + {} + \"{tail}\"", parts.join(" + \", \" + "))
			}
			Type::Bound(_) => format!("_bound({v})"),
			Type::Record(_) | Type::Enum(_) | Type::Object(_) => format!("repr({v})"),
			ty => panic!("Debug of {ty:?}"),
		}
	}

	fn expr(&mut self, e: &Expr, out: &mut String) -> String {
		self.value(e, &e.ty, out)
	}

	/// The Python of `e`, passed where `want` is taken.
	fn value(&mut self, e: &Expr, want: &Type, out: &mut String) -> String {
		match &e.kind {
			Kind::Unit => "None".to_string(),
			Kind::Bool(b) => if *b { "True" } else { "False" }.to_string(),
			Kind::Int(n) => n.to_string(),
			Kind::Float(f) if !f.is_finite() || (*f == 0.0 && f.is_sign_negative()) => {
				format!("_f64({:#x})", f.to_bits())
			}
			Kind::Float(f) => format!("{f:?}"),
			Kind::Char(c) => string(&c.to_string()),
			Kind::Str(s) => string(s),
			Kind::Bytes(b) => match unref(want).base() {
				Type::Array(n) => format!("api.ArrayU8x{n}({})", bytes(b)),
				_ => bytes(b),
			},
			Kind::Duration(secs, nanos) => format!("api.Duration({secs}, {nanos})"),
			Kind::Socket(std::net::SocketAddr::V4(a)) => format!("(\"{}\", {})", a.ip(), a.port()),
			Kind::Socket(std::net::SocketAddr::V6(a)) => {
				format!("(\"{}\", {}, {}, {})", a.ip(), a.port(), a.flowinfo(), a.scope_id())
			}
			Kind::Ip(std::net::IpAddr::V4(ip)) => format!("ipaddress.IPv4Address(\"{ip}\")"),
			Kind::Ip(std::net::IpAddr::V6(ip)) => format!("ipaddress.IPv6Address(\"{ip}\")"),
			Kind::None => "None".to_string(),
			Kind::Some(inner) => {
				let want = match unref(want).base() {
					Type::Option(inner) => (**inner).clone(),
					_ => inner.ty.clone(),
				};
				self.value(inner, &want, out)
			}
			Kind::Tuple(elements) => self.tuple(e, elements, want, out),
			Kind::List(items) => self.list(items, want, out),
			Kind::Record(r, fields) => {
				let name = self.t.type_name(LANGUAGE, &Type::Record(*r)).to_string();
				let field_types: Vec<Type> = self.t.api.records[*r].fields.iter().map(|f| f.ty.clone()).collect();
				let args: Vec<String> = fields
					.iter()
					.zip(&field_types)
					.map(|(f, ty)| self.value(f, ty, out))
					.collect();
				format!("api.{name}({})", args.join(", "))
			}
			Kind::Variant(en, v, fields) => {
				let name = self.t.type_name(LANGUAGE, &Type::Enum(*en)).to_string();
				let enumeration = &self.t.api.enums[*en];
				let variant = &enumeration.variants[*v];
				if enumeration.variants.iter().all(|v| v.fields.is_empty()) {
					return format!("api.{name}.{}", upper(&variant.name));
				}
				let field_types: Vec<Type> = variant.fields.iter().map(|f| f.ty.clone()).collect();
				let variant_name = variant.name.clone();
				let args: Vec<String> = fields
					.iter()
					.zip(&field_types)
					.map(|(f, ty)| self.value(f, ty, out))
					.collect();
				format!("api.{name}.{variant_name}({})", args.join(", "))
			}
			Kind::Bound(kind, value) => {
				let Type::Bound(int) = unref(&e.ty) else {
					panic!("a bound of {:?}", e.ty)
				};
				let (kind, v) = match (kind, value) {
					(BoundKind::Unbounded, _) => ("UNBOUNDED", "0".to_string()),
					(BoundKind::Included, Some(v)) => ("INCLUDED", self.expr(v, out)),
					(BoundKind::Excluded, Some(v)) => ("EXCLUDED", self.expr(v, out)),
					_ => unreachable!(),
				};
				format!("api.Bound{}(api.Bound.{kind}, {v})", int_name(*int))
			}
			Kind::Default => {
				let name = self.t.type_name(LANGUAGE, unref(&e.ty).base()).to_string();
				format!("api.{name}.default()")
			}
			Kind::Var(name) => match self.mutated.contains(name) {
				// Passed `&mut`, the wrapper itself; read, its value.
				true if matches!(want, Type::Ref(Mutability::Mut, _)) => ident(name),
				true => format!("{}.value", ident(name)),
				false => ident(name),
			},
			Kind::Const(c) => format!(
				"api.{}",
				self.t
					.named(LANGUAGE, &format!("const {}", self.t.api.constants[*c].path.join("::")))
			),
			Kind::Call(id, receiver, args) => self.call(*id, receiver.as_deref(), args, out),
			Kind::Field(base, i) => {
				let b = self.expr(base, out);
				let Type::Record(r) = unref(&base.ty).base() else {
					panic!("a field of {:?}", base.ty)
				};
				format!("{b}.{}", self.t.api.records[*r].fields[*i].name)
			}
			Kind::Element(base, i) => {
				let b = self.expr(base, out);
				match unref(&base.ty).base() {
					Type::Range(_) => format!("{b}.{}", if *i == 0 { "start" } else { "stop" }),
					_ => format!("{b}[{i}]"),
				}
			}
			Kind::Clone(inner) => {
				let v = self.expr(inner, out);
				match unref(&inner.ty).base() {
					Type::Object(_) => format!("{v}.clone()"),
					Type::List(..) => format!("list({v})"),
					_ => format!("copy.copy({v})"),
				}
			}
			Kind::Unwrap(inner) => {
				let v = self.expr(inner, out);
				match &inner.ty {
					Type::Result(..) => v,
					_ => format!("_some({v})"),
				}
			}
			Kind::UnwrapErr(inner) => {
				let Type::Result(_, err) = &inner.ty else {
					panic!("unwrap_err of {:?}", inner.ty)
				};
				let exception = self.exception(err);
				let v = self.expr(inner, out);
				format!("_err(lambda: {v}, api.{exception})")
			}
			Kind::IsErr(inner) => {
				let Type::Result(_, err) = &inner.ty else {
					panic!("is_err of {:?}", inner.ty)
				};
				let exception = self.exception(err);
				let v = self.expr(inner, out);
				format!("_fails(lambda: {v}, api.{exception})")
			}
			Kind::IsSome(inner) => format!("({} is not None)", self.expr(inner, out)),
			Kind::IsNone(inner) => format!("({} is None)", self.expr(inner, out)),
			Kind::Binary(op, a, b) => {
				let (a, b) = (self.expr(a, out), self.expr(b, out));
				let op = match op {
					Op::Cmp => return format!("_cmp({a}, {b})"),
					Op::Eq => "==",
					Op::Ne => "!=",
					Op::Lt => "<",
					Op::Le => "<=",
					Op::Gt => ">",
					Op::Ge => ">=",
					Op::Add => "+",
					Op::Sub => "-",
					Op::Mul => "*",
					Op::Rem => "%",
					Op::And => "and",
				};
				format!("({a} {op} {b})")
			}
			Kind::Closure(params, steps, value) => self.closure(params, steps, value, out),
			Kind::Local(name, args) => {
				let args: Vec<String> = args.iter().map(|a| self.expr(a, out)).collect();
				format!("local.{name}({})", args.join(", "))
			}
			Kind::Format(pieces) => self.pieces(pieces, out),
			Kind::Join { items, pat, body, sep } => {
				let items = self.expr(items, out);
				let target = self.pat(pat);
				let body = self.expr(body, out);
				format!("{}.join({body} for {target} in {items})", string(sep))
			}
			Kind::MapOr {
				option,
				none,
				pat,
				some,
			} => {
				let option = self.expr(option, out);
				let none = self.expr(none, out);
				let target = self.pat(pat);
				let some = self.expr(some, out);
				format!("(lambda {target}: {none} if {target} is None else {some})({option})")
			}
			Kind::Utf8(inner) => format!("_text({})", self.expr(inner, out)),
			Kind::Len(inner) => format!("len({})", self.expr(inner, out)),
		}
	}

	fn tuple(&mut self, e: &Expr, elements: &[Expr], want: &Type, out: &mut String) -> String {
		match unref(&e.ty).base() {
			Type::Range(_) => {
				let (a, b) = (self.expr(&elements[0], out), self.expr(&elements[1], out));
				format!("range({a}, {b})")
			}
			Type::Bounds(int) => {
				let (a, b) = (self.expr(&elements[0], out), self.expr(&elements[1], out));
				format!("api.Bounds{}({a}, {b})", int_name(*int))
			}
			_ => {
				let wants: Vec<Type> = match unref(want).base() {
					Type::Tuple(ts) => ts.clone(),
					_ => elements.iter().map(|e| e.ty.clone()).collect(),
				};
				let parts: Vec<String> = elements
					.iter()
					.zip(&wants)
					.map(|(e, t)| self.value(e, t, out))
					.collect();
				match parts.len() {
					1 => format!("({},)", parts[0]),
					_ => format!("({})", parts.join(", ")),
				}
			}
		}
	}

	fn list(&mut self, items: &[Expr], want: &Type, out: &mut String) -> String {
		let (collection, item) = match unref(want).base() {
			Type::List(c, item) => (*c, (**item).clone()),
			ty => panic!("a list as {ty:?}"),
		};
		let parts: Vec<String> = items.iter().map(|e| self.value(e, &item, out)).collect();
		let code = match &item {
			Type::Int(int) => Some(array_code(*int)),
			Type::F64 => Some("d"),
			Type::F32 => Some("f"),
			_ => None,
		};
		match (code, collection) {
			(Some(code), _) => format!("array.array(\"{code}\", [{}])", parts.join(", ")),
			(None, Collection::Iter) => format!("iter([{}])", parts.join(", ")),
			_ => format!("[{}]", parts.join(", ")),
		}
	}

	fn closure(&mut self, params: &[Pat], steps: &[Step], value: &Expr, out: &mut String) -> String {
		let params: Vec<String> = params.iter().map(|p| self.pat(p)).collect();
		if steps.is_empty() {
			let mut pre = String::new();
			let v = self.expr(value, &mut pre);
			if pre.is_empty() {
				return format!("(lambda {}: {v})", params.join(", "));
			}
		}
		// A closure of steps: a function defined before the statement that passes it.
		let name = self.fresh("f");
		self.line(out, &format!("def {name}({}):", params.join(", ")));
		self.depth += 1;
		let mut body = self.steps(steps);
		if !matches!(value.kind, Kind::Unit) {
			let v = self.expr(value, &mut body);
			let _ = writeln!(body, "{}return {v}", self.indent());
		}
		out.push_str(&body);
		self.depth -= 1;
		name
	}

	/// A call of `id`: a function of the module, an associated function of a type, a
	/// method, or a field's getter or setter.
	fn call(&mut self, id: FnId, receiver: Option<&Expr>, args: &[Expr], out: &mut String) -> String {
		let f = self.t.function(id);
		let name = self.t.name(LANGUAGE, id).to_string();
		let params: Vec<Type> = f.params.iter().map(|p| p.ty.clone()).collect();
		let access = f.access;
		let args: Vec<String> = args.iter().zip(&params).map(|(a, ty)| self.value(a, ty, out)).collect();
		let owner = match self.t.owner(id) {
			Some(ty) => format!("api.{}", self.t.type_name(LANGUAGE, &ty)),
			None => "api".to_string(),
		};
		let target = match receiver {
			Some(r) => self.expr(r, out),
			None => owner,
		};
		match access {
			Access::Field { set: false, .. } | Access::Const => format!("{target}.{name}"),
			_ => format!("{target}.{name}({})", args.join(", ")),
		}
	}
}

fn upper(name: &str) -> String {
	let mut out = String::new();
	for (i, c) in name.chars().enumerate() {
		if c.is_uppercase() && i > 0 {
			out.push('_');
		}
		out.push(c.to_ascii_uppercase());
	}
	out
}

fn int_name(int: Int) -> String {
	let rust = int.rust();
	let mut chars = rust.chars();
	let first = chars.next().unwrap().to_ascii_uppercase();
	format!("{first}{}", chars.as_str())
}

fn array_code(int: Int) -> &'static str {
	match int {
		Int::U8 => "B",
		Int::U16 => "H",
		Int::U32 => "I",
		Int::U64 | Int::Usize => "Q",
		Int::I8 => "b",
		Int::I16 => "h",
		Int::I32 => "i",
		Int::I64 | Int::Isize => "q",
	}
}

/// A Python string literal.
fn string(s: &str) -> String {
	let mut out = String::from("\"");
	for c in s.chars() {
		match c {
			'"' | '\\' => {
				out.push('\\');
				out.push(c);
			}
			'\n' => out.push_str("\\n"),
			'\t' => out.push_str("\\t"),
			'\r' => out.push_str("\\r"),
			c if (c as u32) < 0x20 || (0xd800..0xe000).contains(&(c as u32)) => {
				let _ = write!(out, "\\U{:08x}", c as u32);
			}
			c => out.push(c),
		}
	}
	out.push('"');
	out
}

/// A Python bytes literal.
fn bytes(b: &[u8]) -> String {
	let mut out = String::from("b\"");
	for &byte in b {
		match byte {
			b'"' | b'\\' => {
				out.push('\\');
				out.push(byte as char);
			}
			0x20..0x7f => out.push(byte as char),
			_ => {
				let _ = write!(out, "\\x{byte:02x}");
			}
		}
	}
	out.push('"');
	out
}
