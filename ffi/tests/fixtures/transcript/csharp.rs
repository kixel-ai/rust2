//! The transcript as C#: a partial class `Script` whose `Transcript()` runs it through the
//! generated C#, calling `tests/api.cs`'s own methods, which the other half of the class holds.

use std::fmt::Write;

use super::ir::{Access, Int, Mutability, Order, Receiver, Type};
use super::{Arm, BoundKind, Expr, FnId, Kind, Num, Op, Owner, Pat, Piece, Spec, Step, Transcript, unref};

const LANGUAGE: &str = "C#";

/// Helpers the script calls, each for a value C# shapes apart from Rust.
const PRELUDE: &str = r#"// Generated from fixtures/api/examples/transcript.rs by tests/fixtures/transcript. Do not edit.
#nullable enable
// Rust compares a value to itself, to show `PartialEq` is reflexive.
#pragma warning disable CS1718
// `api` aliases the namespace as Rust spells the crate.
#pragma warning disable CS8981

using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;
using Api;
using api = Api;

static partial class Script
{
	/// A call's value, or the exception of `X` it threw.
	readonly struct _Result<T, X>
		where X : Exception
	{
		public readonly T Value;
		public readonly X? Error;

		public _Result(T value, X? error)
		{
			Value = value;
			Error = error;
		}

		public bool Failed => Error != null;

		public X Err => Error ?? throw new InvalidOperationException("the call did not fail");
	}

	static class _Catch<X>
		where X : Exception
	{
		public static _Result<T, X> Try<T>(Func<T> call)
		{
			try
			{
				return new _Result<T, X>(call(), null);
			}
			catch (X e)
			{
				return new _Result<T, X>(default!, e);
			}
		}

		public static _Result<byte, X> Try(Action call)
		{
			try
			{
				call();
				return new _Result<byte, X>(0, null);
			}
			catch (X e)
			{
				return new _Result<byte, X>(0, e);
			}
		}
	}

	/// What `call` returns of `handle`, which it disposes after, as Rust drops a temporary.
	static R _Use<T, R>(T handle, Func<T, R> call)
		where T : IDisposable
	{
		using (handle)
		{
			return call(handle);
		}
	}

	static void _Use<T>(T handle, Action<T> call)
		where T : IDisposable
	{
		using (handle)
		{
			call(handle);
		}
	}

	static T _Block<T>(ValueTask<T> task) => task.AsTask().GetAwaiter().GetResult();

	static void _Block(ValueTask task) => task.AsTask().GetAwaiter().GetResult();

	static string _Bool(bool b) => b ? "true" : "false";

	/// A string as Rust's `Debug` prints it.
	static string _Quoted(string s)
	{
		var out_ = new System.Text.StringBuilder("\"");
		foreach (var c in s)
		{
			switch (c)
			{
				case '"':
				case '\\':
					out_.Append('\\').Append(c);
					break;
				case '\n':
					out_.Append("\\n");
					break;
				case '\t':
					out_.Append("\\t");
					break;
				case '\r':
					out_.Append("\\r");
					break;
				case '\0':
					out_.Append("\\0");
					break;
				default:
					if (c < 0x20 || c == 0x7f)
					{
						out_.Append("\\u{").Append(((int)c).ToString("x")).Append('}');
					}
					else
					{
						out_.Append(c);
					}
					break;
			}
		}
		return out_.Append('"').ToString();
	}

	/// Bytes as Rust's `Debug` prints a list of them.
	static string _Bytes(api.Bytes bytes) => "[" + string.Join(", ", bytes.ToArray()) + "]";

	static bool _BytesEq(api.Bytes a, api.Bytes b) => a == b;

	static string _List<T>(IEnumerable<T> items, Func<T, string> show) => "[" + string.Join(", ", items.Select(show)) + "]";

	static string _OptionS<T>(T? value, Func<T, string> show)
		where T : struct => value is T v ? "Some(" + show(v) + ")" : "None";

	static string _OptionC<T>(T? value, Func<T, string> show)
		where T : class => value is T v ? "Some(" + show(v) + ")" : "None";

	/// Bytes as text, what is not UTF-8 replaced.
	static string _Text(api.Bytes bytes) => System.Text.Encoding.UTF8.GetString(bytes.ToArray());

	static Exception _None() => new InvalidOperationException("unwrap of None");
"#;

/// The C# file of `t`'s script.
pub(crate) fn emit(t: &Transcript) -> String {
	let mut e = Emitter { t, depth: 2, fresh: 0 };
	let body = e.steps(&t.steps);
	format!("{PRELUDE}\n\tpublic static void Transcript()\n\t{{\n{body}\t}}\n}}\n")
}

struct Emitter<'t> {
	t: &'t Transcript,
	depth: usize,
	fresh: usize,
}

/// C#'s keywords, which a local takes with `@`.
const KEYWORDS: &[&str] = &[
	"abstract",
	"as",
	"base",
	"bool",
	"break",
	"byte",
	"case",
	"catch",
	"char",
	"checked",
	"class",
	"const",
	"continue",
	"decimal",
	"default",
	"delegate",
	"do",
	"double",
	"else",
	"enum",
	"event",
	"explicit",
	"extern",
	"false",
	"finally",
	"fixed",
	"float",
	"for",
	"foreach",
	"goto",
	"if",
	"implicit",
	"in",
	"int",
	"interface",
	"internal",
	"is",
	"lock",
	"long",
	"namespace",
	"new",
	"null",
	"object",
	"operator",
	"out",
	"override",
	"params",
	"private",
	"protected",
	"public",
	"readonly",
	"ref",
	"return",
	"sbyte",
	"sealed",
	"short",
	"sizeof",
	"stackalloc",
	"static",
	"string",
	"struct",
	"switch",
	"this",
	"throw",
	"true",
	"try",
	"typeof",
	"uint",
	"ulong",
	"unchecked",
	"unsafe",
	"ushort",
	"using",
	"virtual",
	"void",
	"volatile",
	"while",
	"var",
	"value",
];

/// A local's name in C#: a keyword with `@`, and the namespace's alias apart.
fn ident(name: &str) -> String {
	match name {
		"api" => "api_".to_string(),
		name if KEYWORDS.contains(&name) => format!("@{name}"),
		name => name.to_string(),
	}
}

/// The C# name of a function `transcript.rs` defines: Pascal case.
fn hook(name: &str) -> String {
	pascal(name)
}

fn pascal(name: &str) -> String {
	let mut out = String::new();
	let mut up = true;
	for c in name.chars() {
		match c {
			'_' => up = true,
			c if up => {
				out.extend(c.to_uppercase());
				up = false;
			}
			c => out.push(c),
		}
	}
	out
}

/// C#'s integer type of `int`.
fn int_cs(int: Int) -> &'static str {
	match int {
		Int::U8 => "byte",
		Int::U16 => "ushort",
		Int::U32 => "uint",
		Int::U64 => "ulong",
		Int::Usize => "nuint",
		Int::I8 => "sbyte",
		Int::I16 => "short",
		Int::I32 => "int",
		Int::I64 => "long",
		Int::Isize => "nint",
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

	fn line(&self, out: &mut String, line: &str) {
		let _ = writeln!(out, "{}{line}", self.indent());
	}

	fn steps(&mut self, steps: &[Step]) -> String {
		let mut out = String::new();
		for step in steps {
			self.step(step, &mut out);
		}
		out
	}

	/// The C# type of `ty`, as the binding holds a value of it.
	fn cs(&self, ty: &Type) -> String {
		match ty {
			Type::Unit => "void".to_string(),
			Type::Bool => "bool".to_string(),
			Type::Int(int) => int_cs(*int).to_string(),
			Type::NonZero(int) => format!("api.NonZero{}", pascal(int.rust())),
			Type::F32 => "float".to_string(),
			Type::F64 => "double".to_string(),
			Type::Char => "api.Scalar".to_string(),
			Type::Str | Type::String => "api.Str".to_string(),
			Type::Slice | Type::Vec | Type::Bytes => "api.Bytes".to_string(),
			Type::Path | Type::PathBuf => "api.PathBuf".to_string(),
			Type::U128 => "api.U128".to_string(),
			Type::Duration => "api.Duration".to_string(),
			Type::Array(n) => format!("api.ArrayU8x{n}"),
			Type::SocketAddr(family) => format!("api.{}", addr(true, *family)),
			Type::IpAddr(family) => format!("api.{}", addr(false, *family)),
			Type::List(_, inner) => format!("{}[]", self.cs(inner)),
			Type::Waker => "api.Waker".to_string(),
			Type::Future => "Func<System.Threading.CancellationToken, Task>".to_string(),
			Type::Reader => "api.ISeekableReader".to_string(),
			Type::Option(inner) => format!("{}?", self.cs(inner)),
			Type::Box(inner) | Type::Ref(_, inner) => self.cs(inner),
			ty @ (Type::Record(_) | Type::Enum(_) | Type::Object(_)) => {
				format!("api.{}", self.t.type_name(LANGUAGE, ty))
			}
			Type::Range(int) => format!("api.Range{}", pascal(int.rust())),
			Type::Bound(int) => format!("api.Bound{}", pascal(int.rust())),
			Type::Bounds(int) => format!("api.Bounds{}", pascal(int.rust())),
			Type::Tuple(elements) => match &elements[..] {
				[element] => format!("ValueTuple<{}>", self.cs(element)),
				elements => {
					let elements: Vec<String> = elements.iter().map(|e| self.cs(e)).collect();
					format!("({})", elements.join(", "))
				}
			},
			Type::Closure(closure) => {
				let params: Vec<String> = closure.params.iter().map(|p| self.cs(p)).collect();
				match &closure.ok {
					Type::Unit if params.is_empty() => "Action".to_string(),
					Type::Unit => format!("Action<{}>", params.join(", ")),
					ok => {
						let mut all = params;
						all.push(self.cs(ok));
						format!("Func<{}>", all.join(", "))
					}
				}
			}
			Type::Result(ok, _) => self.cs(ok),
			ty => panic!("C# holds no {ty:?} in the script"),
		}
	}

	/// Whether C# holds a value of `ty` as a value type, whose option is a `Nullable`.
	fn is_value(&self, ty: &Type) -> bool {
		match ty {
			Type::Object(_) | Type::List(..) | Type::Callback(..) | Type::Closure(_) | Type::Future | Type::Reader => {
				false
			}
			Type::Waker => false,
			Type::Enum(i) => self.t.api.enums[*i].variants.iter().all(|v| v.fields.is_empty()),
			Type::Box(inner) | Type::Ref(_, inner) => self.is_value(inner),
			_ => true,
		}
	}

	fn step(&mut self, step: &Step, out: &mut String) {
		match step {
			Step::Let(pat, value) => {
				let v = self.expr(value, out);
				match pat {
					Pat::Ignore(_) => {
						let v = self.dropped(&v, &value.ty);
						self.line(out, &v)
					}
					pat => self.bind(pat, &v, &value.ty, out),
				}
			}
			Step::Print(pieces) => {
				let v = self.pieces(pieces, out);
				self.line(out, &format!("Console.WriteLine({v});"));
			}
			Step::Do(value) => {
				let v = self.expr(value, out);
				let v = self.dropped(&v, &value.ty);
				self.line(out, &v);
			}
			Step::Drop(name) => {
				let name = ident(name);
				self.line(out, &format!("{name}.Dispose();"))
			}
			Step::Match { value, ok, err } => self.matched(value, ok, err, out),
		}
	}

	/// The statement of `v`, a value of `ty` the script drops: a `Result`'s error too, which C# throws.
	fn dropped(&self, v: &str, ty: &Type) -> String {
		match ty {
			Type::Result(_, err) => format!("_Catch<{}>.Try(() => {v});", self.exception(err)),
			_ if statement(v) => format!("{v};"),
			_ => format!("_ = {v};"),
		}
	}

	/// `pat` bound to `v`, a value of `ty`.
	fn bind(&mut self, pat: &Pat, v: &str, ty: &Type, out: &mut String) {
		match pat {
			Pat::Name(name, _) => {
				let name = ident(name);
				self.line(out, &format!("var {name} = {v};"));
			}
			Pat::Ignore(_) => {
				let v = self.dropped(v, ty);
				self.line(out, &v)
			}
			Pat::Tuple(pats) => {
				let tmp = self.fresh("t");
				self.line(out, &format!("var {tmp} = {v};"));
				let base = unref(ty).base().clone();
				for (i, p) in pats.iter().enumerate() {
					if matches!(p, Pat::Ignore(_)) {
						continue;
					}
					let (element, ty) = match &base {
						Type::Bounds(int) => ([".Start", ".End"][i].to_string(), Type::Bound(*int)),
						Type::Range(int) => ([".Start", ".End"][i].to_string(), Type::Int(*int)),
						Type::Tuple(elements) => (format!(".Item{}", i + 1), elements[i].clone()),
						ty => panic!("a tuple pattern of {ty:?}"),
					};
					self.bind(p, &format!("{tmp}{element}"), &ty, out);
				}
			}
		}
	}

	fn matched(&mut self, value: &Expr, ok: &Arm, err: &Arm, out: &mut String) {
		match &value.ty {
			Type::Option(inner) => {
				let v = self.expr(value, out);
				let tmp = self.fresh("o");
				self.line(out, &format!("if ({v} is {{}} {tmp})"));
				self.line(out, "{");
				self.arm(ok, &tmp, inner, out);
				self.line(out, "}");
				self.line(out, "else");
				self.line(out, "{");
				self.arm(err, "null", &Type::Unit, out);
				self.line(out, "}");
			}
			Type::Result(ok_ty, err_ty) => {
				let exception = self.exception(err_ty);
				let v = self.expr(value, out);
				let tmp = self.fresh("r");
				self.line(out, &format!("var {tmp} = _Catch<{exception}>.Try(() => {v});"));
				self.line(out, &format!("if (!{tmp}.Failed)"));
				self.line(out, "{");
				self.arm(ok, &format!("{tmp}.Value"), ok_ty, out);
				self.line(out, "}");
				self.line(out, "else");
				self.line(out, "{");
				self.arm(err, &format!("{tmp}.Err.Error"), err_ty, out);
				self.line(out, "}");
			}
			ty => panic!("a match on {ty:?}"),
		}
	}

	fn arm(&mut self, arm: &Arm, payload: &str, ty: &Type, out: &mut String) {
		self.depth += 1;
		if let Some(pat) = &arm.pat {
			self.bind(pat, payload, ty, out);
		}
		let body = self.steps(&arm.steps);
		out.push_str(&body);
		self.depth -= 1;
	}

	/// The exception a call failing with `err` throws.
	fn exception(&self, err: &Type) -> String {
		format!("api.{}", self.t.error(LANGUAGE, err))
	}

	/// The C# of a format's pieces: an interpolated string, as an `api.Str`.
	fn pieces(&mut self, pieces: &[Piece], out: &mut String) -> String {
		let mut template = String::from("$\"");
		for piece in pieces {
			match piece {
				Piece::Text(text) => template.push_str(&escape(text).replace('{', "{{").replace('}', "}}")),
				Piece::Value(e, spec) => {
					let v = self.expr(e, out);
					let shown = self.show(&v, &e.ty, *spec);
					let _ = write!(template, "{{({shown})}}");
				}
			}
		}
		template.push('"');
		template
	}

	/// `v`, a value of `ty`, as C# shows it the way Rust formats it with `spec`.
	fn show(&mut self, v: &str, ty: &Type, spec: Spec) -> String {
		let ty = unref(ty).base();
		match spec {
			Spec::Hex(width) => {
				return match ty {
					Type::U128 => format!("{v}.ToHex({width})"),
					Type::Int(Int::Usize) => format!("((ulong){v}).ToString(\"x{width}\")"),
					Type::NonZero(_) => format!("{v}.Value.ToString(\"x{width}\")"),
					Type::Int(int) if !int.signed() => format!("{v}.ToString(\"x{width}\")"),
					ty => panic!("hex of {ty:?}"),
				};
			}
			Spec::Padded(width) => {
				assert!(
					matches!(ty, Type::Int(int) | Type::NonZero(int) if !int.signed()) || *ty == Type::U128,
					"zero-padded {ty:?}"
				);
				return format!("{v}.ToString().PadLeft({width}, '0')");
			}
			Spec::Debug => return self.debug(v, ty),
			Spec::Display => {}
		}
		match ty {
			Type::Bool => format!("_Bool({v})"),
			Type::String | Type::Str | Type::Int(_) | Type::U128 | Type::NonZero(_) | Type::Char => v.to_string(),
			Type::Object(_) | Type::Record(_) | Type::Enum(_) => match self.text_of(ty) {
				Some(text) => format!("{v}.{text}()"),
				None if self.plain(ty) => format!("{v}.{}()", self.t.member(LANGUAGE, ty, "Display")),
				None => v.to_string(),
			},
			ty => panic!("Display of {ty:?}"),
		}
	}

	/// Whether `ty` is an enum without data, a C# `enum`.
	fn plain(&self, ty: &Type) -> bool {
		matches!(ty, Type::Enum(i) if self.t.api.enums[*i].variants.iter().all(|v| v.fields.is_empty()))
	}

	/// The name of a type's `Access::Text` method, which C# names apart from `ToString`.
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
			Type::Bool => format!("_Bool({v})"),
			Type::Int(_) | Type::U128 | Type::NonZero(_) => format!("{v}.ToString()"),
			Type::String | Type::Str => format!("_Quoted({v}.ToString())"),
			Type::Option(inner) => {
				let x = self.fresh("x");
				let show = self.debug(&x, inner);
				let helper = match self.is_value(inner) {
					true => "_OptionS",
					false => "_OptionC",
				};
				format!("{helper}({v}, {x} => {show})")
			}
			Type::Vec | Type::Slice | Type::Bytes => format!("_Bytes({v})"),
			Type::Array(_) => format!("_Bytes({v}.ToArray())"),
			Type::List(_, item) => {
				let x = self.fresh("x");
				let show = self.debug(&x, item);
				format!("_List({v}, {x} => {show})")
			}
			Type::Tuple(elements) => {
				let x = self.fresh("x");
				let parts: Vec<String> = (0..elements.len())
					.map(|i| {
						let shown = self.debug(&format!("{x}.Item{}", i + 1), &elements[i]);
						format!("{{({shown})}}")
					})
					.collect();
				let tail = if parts.len() == 1 { ",)" } else { ")" };
				let tuple = self.cs(unref(ty).base());
				format!(
					"((Func<{tuple}, string>)({x} => $\"({}{tail}\"))({v})",
					parts.join(", ")
				)
			}
			Type::Bound(_) => format!("{v}.ToString()"),
			Type::Range(_) => format!("$\"{{{v}.Start}}..{{{v}.End}}\""),
			ty @ (Type::Record(_) | Type::Enum(_) | Type::Object(_)) => {
				format!("{v}.{}()", self.t.member(LANGUAGE, ty, "Debug"))
			}
			ty => panic!("Debug of {ty:?}"),
		}
	}

	fn expr(&mut self, e: &Expr, out: &mut String) -> String {
		self.value(e, &e.ty, out)
	}

	/// The C# of `e`, passed where `want` is taken.
	fn value(&mut self, e: &Expr, want: &Type, out: &mut String) -> String {
		match &e.kind {
			Kind::Unit => "default".to_string(),
			Kind::Bool(b) => b.to_string(),
			Kind::Int(n) => {
				let ty = match unref(&e.ty).base() {
					ty @ (Type::Int(_) | Type::NonZero(_) | Type::U128 | Type::F32 | Type::F64) => ty,
					_ => match unref(want).base() {
						Type::Option(inner) => unref(inner).base(),
						ty => ty,
					},
				};
				int(*n, ty)
			}
			Kind::Float(f) => match unref(want).base() {
				Type::F32 => float32(*f as f32),
				_ => float64(*f),
			},
			Kind::Char(c) => format!("new api.Scalar(0x{:x})", *c as u32),
			Kind::Str(s) => match unref(want).base() {
				Type::Path | Type::PathBuf => format!("new api.PathBuf({})", string(s)),
				Type::Option(inner) if matches!(unref(inner).base(), Type::Path | Type::PathBuf) => {
					format!("new api.PathBuf({})", string(s))
				}
				_ => format!("((api.Str){})", string(s)),
			},
			Kind::Bytes(b) => {
				let bytes: Vec<String> = b.iter().map(|b| b.to_string()).collect();
				let array = format!("new byte[] {{ {} }}", bytes.join(", "));
				let want = match unref(want).base() {
					Type::Option(inner) => unref(inner).base(),
					ty => ty,
				};
				match want {
					Type::Array(n) => format!("new api.ArrayU8x{n}({array})"),
					_ => format!("((api.Bytes){array})"),
				}
			}
			Kind::Duration(secs, nanos) => format!("new api.Duration({secs}UL, {nanos}U)"),
			Kind::Socket(addr) => {
				let made = match addr {
					std::net::SocketAddr::V4(a) => format!("new api.SocketAddrV4({}, {})", ipv4(a.ip()), a.port()),
					std::net::SocketAddr::V6(a) => format!(
						"new api.SocketAddrV6({}, {}, {}U, {}U)",
						ipv6(a.ip()),
						a.port(),
						a.flowinfo(),
						a.scope_id()
					),
				};
				match unref(&e.ty).base() {
					Type::SocketAddr(super::ir::Family::Any) => format!("((api.SocketAddr){made})"),
					_ => made,
				}
			}
			Kind::Ip(ip) => {
				let made = match ip {
					std::net::IpAddr::V4(ip) => ipv4(ip),
					std::net::IpAddr::V6(ip) => ipv6(ip),
				};
				match unref(&e.ty).base() {
					Type::IpAddr(super::ir::Family::Any) => format!("((api.IpAddr){made})"),
					_ => made,
				}
			}
			Kind::None => {
				let ty = match unref(want).base() {
					ty @ Type::Option(_) => ty.clone(),
					_ => e.ty.clone(),
				};
				format!("(({})null)", self.cs(&ty))
			}
			Kind::Some(inner) => {
				let inner_want = match unref(want).base() {
					Type::Option(inner) => (**inner).clone(),
					_ => inner.ty.clone(),
				};
				let v = self.value(inner, &inner_want, out);
				// Typed as the option, so a local of it may later hold `None`.
				let ty = match unref(want).base() {
					ty @ Type::Option(_) => ty.clone(),
					_ => e.ty.clone(),
				};
				format!("(({}){v})", self.cs(&ty))
			}
			Kind::Tuple(elements) => self.tuple(e, elements, want, out),
			Kind::List(items) => self.list(items, want, &e.ty, out),
			Kind::Record(r, fields) => {
				let name = self.t.type_name(LANGUAGE, &Type::Record(*r)).to_string();
				let field_types: Vec<Type> = self.t.api.records[*r].fields.iter().map(|f| f.ty.clone()).collect();
				let args: Vec<String> = (fields.iter().zip(&field_types))
					.map(|(f, ty)| self.value(f, ty, out))
					.collect();
				format!("new api.{name}({})", args.join(", "))
			}
			Kind::Variant(en, v, fields) => {
				let ty = Type::Enum(*en);
				let variant = &self.t.api.enums[*en].variants[*v];
				let name = self.t.type_name(LANGUAGE, &ty).to_string();
				let member = self.t.variant(LANGUAGE, &ty, &variant.name).to_string();
				if self.plain(&ty) {
					return format!("api.{name}.{member}");
				}
				let field_types: Vec<Type> = variant.fields.iter().map(|f| f.ty.clone()).collect();
				let args: Vec<String> = (fields.iter().zip(&field_types))
					.map(|(f, ty)| self.value(f, ty, out))
					.collect();
				format!("new api.{name}.{member}({})", args.join(", "))
			}
			Kind::Bound(kind, value) => {
				let Type::Bound(int_ty) = unref(&e.ty) else {
					panic!("a bound of {:?}", e.ty)
				};
				let class = format!("api.Bound{}", pascal(int_ty.rust()));
				match (kind, value) {
					(BoundKind::Unbounded, _) => format!("{class}.Unbounded"),
					(BoundKind::Included, Some(v)) => {
						format!("{class}.Included({})", self.value(v, &Type::Int(*int_ty), out))
					}
					(BoundKind::Excluded, Some(v)) => {
						format!("{class}.Excluded({})", self.value(v, &Type::Int(*int_ty), out))
					}
					_ => unreachable!(),
				}
			}
			Kind::Default => {
				let ty = unref(&e.ty).base();
				let name = self.t.type_name(LANGUAGE, ty);
				let default = self.t.member(LANGUAGE, ty, "Default");
				match self.plain(ty) {
					true => format!("api.{name}Extensions.{default}()"),
					false => format!("api.{name}.{default}()"),
				}
			}
			Kind::Var(name) => match written(want) {
				true => format!("ref {}", ident(name)),
				false => ident(name),
			},
			Kind::Const(c) => format!(
				"api.Lib.{}",
				self.t
					.named(LANGUAGE, &format!("const {}", self.t.api.constants[*c].path.join("::")))
			),
			Kind::Call(id, receiver, args) => self.call(*id, receiver.as_deref(), args, out),
			Kind::Field(base, i) => {
				let b = self.expr(base, out);
				let ty = unref(&base.ty).base();
				let Type::Record(r) = ty else {
					panic!("a field of {:?}", base.ty)
				};
				format!(
					"{b}.{}",
					self.t.field(LANGUAGE, ty, &self.t.api.records[*r].fields[*i].name)
				)
			}
			Kind::Element(base, i) => {
				let b = self.expr(base, out);
				match unref(&base.ty).base() {
					Type::Range(_) | Type::Bounds(_) => format!("{b}.{}", if *i == 0 { "Start" } else { "End" }),
					Type::Tuple(_) => format!("{b}.Item{}", i + 1),
					ty => panic!("an element of {ty:?}"),
				}
			}
			Kind::Clone(inner) => {
				let v = self.expr(inner, out);
				match unref(&inner.ty).base() {
					ty @ Type::Object(_) => format!("{v}.{}()", self.t.member(LANGUAGE, ty, "Clone")),
					Type::List(..) => format!("{v}.ToArray()"),
					// Records, bytes, and the rest are values C# copies, so the value is its clone.
					_ => v,
				}
			}
			Kind::Unwrap(inner) => {
				let v = self.expr(inner, out);
				match &inner.ty {
					Type::Result(..) => v,
					_ => {
						let x = self.fresh("u");
						format!("({v} is {{}} {x} ? {x} : throw _None())")
					}
				}
			}
			Kind::UnwrapErr(inner) => {
				let Type::Result(_, err) = &inner.ty else {
					panic!("unwrap_err of {:?}", inner.ty)
				};
				let exception = self.exception(err);
				let v = self.expr(inner, out);
				format!("_Catch<{exception}>.Try(() => {v}).Err.Error")
			}
			Kind::IsErr(inner) => {
				let Type::Result(_, err) = &inner.ty else {
					panic!("is_err of {:?}", inner.ty)
				};
				let exception = self.exception(err);
				let v = self.expr(inner, out);
				format!("_Catch<{exception}>.Try(() => {v}).Failed")
			}
			Kind::IsSome(inner) => format!("({} != null)", self.expr(inner, out)),
			Kind::IsNone(inner) => format!("({} == null)", self.expr(inner, out)),
			Kind::Binary(op, a, b) => self.binary(*op, a, b, &e.ty, out),
			Kind::Closure(params, steps, value) => {
				let ty = match unref(want).base() {
					ty @ Type::Closure(_) => ty.clone(),
					_ => e.ty.clone(),
				};
				self.closure(params, steps, value, &ty)
			}
			Kind::Local(name, args) => {
				if name.starts_with(|c: char| c.is_ascii_uppercase()) {
					return format!("new {name}()");
				}
				let params = self.t.locals.get(name).cloned().unwrap_or_default();
				let args: Vec<String> = (args.iter().enumerate())
					.map(|(k, a)| {
						let ty = params.get(k).cloned().unwrap_or_else(|| a.ty.clone());
						self.value(a, &ty, out)
					})
					.collect();
				format!("{}({})", hook(name), args.join(", "))
			}
			Kind::Format(pieces) => format!("((api.Str){})", self.pieces(pieces, out)),
			Kind::Join { items, pat, body, sep } => {
				let items = self.expr(items, out);
				let lambda = self.lambda(std::slice::from_ref(pat), &[], body);
				format!("((api.Str)string.Join({}, {items}.Select({lambda})))", string(sep))
			}
			Kind::MapOr {
				option,
				none,
				pat,
				some,
			} => {
				let option = self.expr(option, out);
				let none = self.expr(none, out);
				let Pat::Name(name, _) = pat else {
					panic!("map_or binding a pattern")
				};
				let x = ident(name);
				let some = self.expr(some, out);
				format!("({option} is {{}} {x} ? {some} : {none})")
			}
			Kind::Utf8(inner) => format!("((api.Str)_Text({}))", self.expr(inner, out)),
			Kind::Len(inner) => format!("((nuint){}.Length)", self.expr(inner, out)),
		}
	}

	/// `a op b`, with Rust's overflow panic, of `ty`.
	fn binary(&mut self, op: Op, a: &Expr, b: &Expr, ty: &Type, out: &mut String) -> String {
		let operand = unref(&a.ty).base().clone();
		let (a, b) = (self.value(a, &operand, out), self.value(b, &operand, out));
		let partial = self.t.api.traits(&operand).is_some_and(|t| t.ord == Order::Partial);
		let plain = self.plain(&operand);
		// A partial order has no `CompareTo`, only `PartialCompareTo`, null when unordered.
		if (partial || plain)
			&& let Some(test) = match op {
				Op::Lt => Some("== -1"),
				Op::Le => Some("<= 0"),
				Op::Gt => Some("== 1"),
				Op::Ge => Some(">= 0"),
				_ => None,
			} {
			let member = self.t.member(LANGUAGE, &operand, "PartialOrd");
			return format!("({a}.{member}({b}) {test})");
		}
		let sym = match op {
			Op::Cmp => {
				let cmp = match operand {
					Type::Record(_) | Type::Enum(_) | Type::Object(_) => self.t.member(LANGUAGE, &operand, "Ord"),
					_ => "CompareTo",
				};
				return format!("((sbyte)Math.Sign({a}.{cmp}({b})))");
			}
			Op::Eq | Op::Ne => {
				let not = if op == Op::Ne { "!" } else { "" };
				return match &operand {
					Type::Vec | Type::Slice | Type::Bytes => format!("{not}_BytesEq({a}, {b})"),
					Type::List(..) => format!("{not}{a}.SequenceEqual({b})"),
					_ => format!("({a} {} {b})", if op == Op::Ne { "!=" } else { "==" }),
				};
			}
			Op::Lt => "<",
			Op::Le => "<=",
			Op::Gt => ">",
			Op::Ge => ">=",
			Op::Add => "+",
			Op::Sub => "-",
			Op::Mul => "*",
			Op::Rem => "%",
			Op::And => "&&",
		};
		match (op, unref(ty).base()) {
			(Op::Add | Op::Sub | Op::Mul | Op::Rem, Type::Int(int)) => {
				format!("checked(({})({a} {sym} {b}))", int_cs(*int))
			}
			_ => format!("({a} {sym} {b})"),
		}
	}

	fn tuple(&mut self, e: &Expr, elements: &[Expr], want: &Type, out: &mut String) -> String {
		match unref(&e.ty).base() {
			Type::Range(int_ty) => {
				let ty = Type::Int(*int_ty);
				let (a, b) = (self.value(&elements[0], &ty, out), self.value(&elements[1], &ty, out));
				format!("new api.Range{}({a}, {b})", pascal(int_ty.rust()))
			}
			Type::Bounds(int_ty) => {
				let ty = Type::Bound(*int_ty);
				let (a, b) = (self.value(&elements[0], &ty, out), self.value(&elements[1], &ty, out));
				format!("new api.Bounds{}({a}, {b})", pascal(int_ty.rust()))
			}
			_ => {
				let wants: Vec<Type> = match unref(want).base() {
					Type::Tuple(ts) => ts.clone(),
					Type::Option(inner) => match unref(inner).base() {
						Type::Tuple(ts) => ts.clone(),
						_ => elements.iter().map(|e| e.ty.clone()).collect(),
					},
					_ => elements.iter().map(|e| e.ty.clone()).collect(),
				};
				let parts: Vec<String> = (elements.iter().zip(&wants))
					.map(|(e, t)| self.value(e, t, out))
					.collect();
				let ty = self.cs(&Type::Tuple(wants));
				match parts.len() {
					1 => format!("new {ty}({})", parts[0]),
					_ => format!("(({ty})({}))", parts.join(", ")),
				}
			}
		}
	}

	fn list(&mut self, items: &[Expr], want: &Type, ty: &Type, out: &mut String) -> String {
		let list = match unref(want).base() {
			list @ Type::List(..) => list,
			Type::Option(inner) => unref(inner).base(),
			_ => unref(ty).base(),
		};
		let Type::List(_, item) = list else {
			panic!("a list as {list:?}")
		};
		let item = (**item).clone();
		let parts: Vec<String> = items.iter().map(|e| self.value(e, &item, out)).collect();
		format!("new {}[] {{ {} }}", self.cs(&item), parts.join(", "))
	}

	/// A lambda of `params` running `steps`, then returning `value`.
	fn lambda(&mut self, params: &[Pat], steps: &[Step], value: &Expr) -> String {
		let mut body = String::new();
		self.depth += 1;
		let mut names = Vec::new();
		for p in params {
			match p {
				Pat::Name(name, _) => names.push(ident(name)),
				Pat::Ignore(_) => names.push(self.fresh("p")),
				Pat::Tuple(_) => {
					let x = self.fresh("p");
					let ty = match p {
						Pat::Tuple(pats) => Type::Tuple(pats.iter().map(pat_type).collect()),
						_ => unreachable!(),
					};
					self.bind(p, &x, &ty, &mut body);
					names.push(x);
				}
			}
		}
		body.push_str(&self.steps(steps));
		let v = match value.kind {
			Kind::Unit => None,
			_ => Some(self.expr(value, &mut body)),
		};
		self.depth -= 1;
		let params = format!("({})", names.join(", "));
		match (body.is_empty(), v) {
			(true, Some(v)) => format!("{params} => {v}"),
			(_, v) => {
				let mut text = format!("{params} =>\n{}{{\n{body}", self.indent());
				if let Some(v) = v {
					let _ = writeln!(text, "{}\treturn {v};", self.indent());
				}
				let _ = write!(text, "{}}}", self.indent());
				text
			}
		}
	}

	/// A closure as a lambda of its delegate's type.
	fn closure(&mut self, params: &[Pat], steps: &[Step], value: &Expr, ty: &Type) -> String {
		let delegate = self.cs(ty);
		let lambda = self.lambda(params, steps, value);
		format!("(({delegate})({lambda}))")
	}

	/// A call of `id`: a function of the library, a static member of a type, a method, or a
	/// field's getter or setter.
	fn call(&mut self, id: FnId, receiver: Option<&Expr>, args: &[Expr], out: &mut String) -> String {
		let f = self.t.function(id);
		let name = self.t.name(LANGUAGE, id).to_string();
		let (access, future) = (f.access, f.future);
		let params: Vec<Type> = f.params.iter().map(|p| p.ty.clone()).collect();
		assert_eq!(
			args.len(),
			params.len(),
			"a call of {} with {} arguments",
			self.t.key(id),
			args.len()
		);
		let args: Vec<String> = (args.iter().zip(&params))
			.map(|(a, ty)| {
				// Bytes a parameter takes are a span, of an array or of the memory a value holds.
				match (ty.base(), &a.kind) {
					_ if written(ty) => self.value(a, ty, out),
					(Type::Slice | Type::Vec | Type::Bytes, Kind::Bytes(b)) => {
						let bytes: Vec<String> = b.iter().map(|b| b.to_string()).collect();
						format!("new byte[] {{ {} }}", bytes.join(", "))
					}
					(Type::Slice | Type::Vec | Type::Bytes, _) => {
						format!("{}.Span", self.value(a, ty, out))
					}
					_ => self.value(a, ty, out),
				}
			})
			.collect();
		let owner = self.t.owner(id);
		let plain = owner.as_ref().is_some_and(|o| self.plain(o));
		let target = match (receiver, &owner) {
			(Some(r), _) => self.expr(r, out),
			(None, Some(ty)) if plain => format!("api.{}Extensions", self.t.type_name(LANGUAGE, ty)),
			(None, Some(ty)) => format!("api.{}", self.t.type_name(LANGUAGE, ty)),
			(None, None) => "api.Lib".to_string(),
		};
		// A handle made for the call is a temporary, which Rust drops once the statement
		// ends, ending any borrow it holds: C# disposes it once the call returns.
		let temporary = receiver.is_some_and(temporary) && f.receiver != Some(Receiver::Owned);
		let (target, close) = match temporary {
			true => {
				let tmp = self.fresh("h");
				(tmp.clone(), Some((target, tmp)))
			}
			false => (target, None),
		};
		let value = !matches!(owner, Some(Type::Object(_)));
		let property = match access {
			Access::Field { set: false, .. } => {
				f.params.is_empty() && !future && !plain && !(value && f.receiver == Some(Receiver::Mut))
			}
			Access::Field { set: true, .. } => f.params.len() == 1 && !future && !plain,
			Access::Const => f.params.is_empty() && !future,
			_ => false,
		};
		let call = match (access, property) {
			(Access::Field { set: true, .. }, true) => format!("{target}.{name} = {}", args[0]),
			(_, true) => format!("{target}.{name}"),
			_ => format!("{target}.{name}({})", args.join(", ")),
		};
		let call = match close {
			Some((handle, tmp)) => format!("_Use({handle}, {tmp} => {call})"),
			None => call,
		};
		match future {
			true => format!("_Block({call})"),
			false => call,
		}
	}
}

/// The type a pattern binds, as far as a lambda's tuple parameter needs it.
fn pat_type(pat: &Pat) -> Type {
	match pat {
		Pat::Name(_, ty) | Pat::Ignore(ty) => ty.clone(),
		Pat::Tuple(pats) => Type::Tuple(pats.iter().map(pat_type).collect()),
	}
}

/// Whether C# takes `v` alone as a statement: a call, an assignment, or a `new`.
fn statement(v: &str) -> bool {
	v.ends_with(')') && !v.starts_with('(') || v.contains(" = ")
}

/// Whether a parameter of `ty` is a value Rust writes back through, a `ref` in C#.
fn written(ty: &Type) -> bool {
	matches!(ty, Type::Ref(Mutability::Mut, inner) if !matches!(**inner, Type::Object(_) | Type::Reader | Type::Callback(..)))
}

/// Whether `e` is a handle no local holds: an object a call returned, unwrapped or not.
fn temporary(e: &Expr) -> bool {
	let called = match &e.kind {
		Kind::Call(..) => true,
		Kind::Unwrap(inner) => matches!(inner.kind, Kind::Call(..)),
		_ => false,
	};
	called && matches!(unref(&e.ty).base(), Type::Object(_))
}

/// The runtime's class of an address type, `socket` for a socket's.
fn addr(socket: bool, family: super::ir::Family) -> &'static str {
	use super::ir::Family;
	match (socket, family) {
		(true, Family::Any) => "SocketAddr",
		(true, Family::V4) => "SocketAddrV4",
		(true, Family::V6) => "SocketAddrV6",
		(false, Family::Any) => "IpAddr",
		(false, Family::V4) => "Ipv4Addr",
		(false, Family::V6) => "Ipv6Addr",
	}
}

/// An integer literal of `ty`, typed as C# holds it wherever it stands.
fn int(n: Num, ty: &Type) -> String {
	let int = match ty {
		Type::Int(int) => *int,
		Type::NonZero(int) => return format!("new api.NonZero{}({})", pascal(int.rust()), int_of(n, *int)),
		Type::U128 => {
			let v = n.magnitude;
			return format!("new api.U128({}UL, {}UL)", (v >> 64) as u64, v as u64);
		}
		Type::F32 => return format!("{n}F"),
		Type::F64 => return format!("{n}.0"),
		ty => panic!("an integer as {ty:?}"),
	};
	int_of(n, int)
}

fn int_of(n: Num, int: Int) -> String {
	match int {
		Int::U8 => format!("(byte){n}"),
		Int::U16 => format!("(ushort){n}"),
		Int::U32 => format!("{n}U"),
		Int::U64 => format!("{n}UL"),
		Int::Usize => format!("unchecked((nuint){n}UL)"),
		Int::I8 => format!("(sbyte)({n})"),
		Int::I16 => format!("(short)({n})"),
		Int::I32 if n == Num::new(i32::MIN.into()) => "int.MinValue".to_string(),
		Int::I32 => format!("({n})"),
		Int::I64 | Int::Isize if n == Num::new(i64::MIN.into()) => format!("unchecked(({})long.MinValue)", int_cs(int)),
		Int::I64 => format!("({n}L)"),
		Int::Isize => format!("unchecked((nint)({n}L))"),
	}
}

fn float64(f: f64) -> String {
	match f.is_finite() && !(f == 0.0 && f.is_sign_negative()) {
		true => format!("{f:?}"),
		false => format!("BitConverter.Int64BitsToDouble(unchecked((long)0x{:x}UL))", f.to_bits()),
	}
}

fn float32(f: f32) -> String {
	match f.is_finite() && !(f == 0.0 && f.is_sign_negative()) {
		true => format!("{f:?}F"),
		false => format!("BitConverter.Int32BitsToSingle(unchecked((int)0x{:x}U))", f.to_bits()),
	}
}

fn ipv4(ip: &std::net::Ipv4Addr) -> String {
	format!("new api.Ipv4Addr({}U)", u32::from(*ip))
}

fn ipv6(ip: &std::net::Ipv6Addr) -> String {
	let octets: Vec<String> = ip.octets().iter().map(|o| o.to_string()).collect();
	format!("new api.Ipv6Addr(new byte[] {{ {} }})", octets.join(", "))
}

/// Text escaped for a C# string literal.
fn escape(s: &str) -> String {
	let mut out = String::new();
	for c in s.chars() {
		match c {
			'"' | '\\' => {
				out.push('\\');
				out.push(c);
			}
			'\n' => out.push_str("\\n"),
			'\t' => out.push_str("\\t"),
			'\r' => out.push_str("\\r"),
			'\0' => out.push_str("\\0"),
			c if (c as u32) < 0x20 || c as u32 == 0x7f => {
				let _ = write!(out, "\\u{:04x}", c as u32);
			}
			c if (c as u32) > 0xffff => {
				let _ = write!(out, "\\U{:08x}", c as u32);
			}
			c => out.push(c),
		}
	}
	out
}

/// A C# string literal.
fn string(s: &str) -> String {
	format!("\"{}\"", escape(s))
}
