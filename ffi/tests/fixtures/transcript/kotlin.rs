//! The transcript as Kotlin: a file whose `transcript()` runs it through the generated
//! Kotlin, calling `tests/api.kt`'s own functions, which share its default package.

use std::fmt::Write;

use super::ir::{Access, Collection, Int, Mutability, Receiver, Type};
use super::{Arm, BoundKind, Expr, FnId, Kind, Num, Op, Owner, Pat, Piece, Spec, Step, Transcript, unref};

const LANGUAGE: &str = "Kotlin";

/// Helpers the script calls, each for a value Kotlin shapes apart from Rust.
const PRELUDE: &str = r#"// Generated from fixtures/api/examples/transcript.rs by tests/fixtures/transcript. Do not edit.

@file:OptIn(ExperimentalUnsignedTypes::class)

/** A call's value, or the exception of [exception] it threw, as a `Result`. */
private fun <T> _try(exception: Class<out Throwable>, call: () -> T): Result<T> = try {
	Result.success(call())
} catch (e: Throwable) {
	if (!exception.isInstance(e)) throw e
	Result.failure(e)
}

/** The exception of [exception] a call that must fail threw. */
private fun <X : Throwable> _thrown(exception: Class<X>, call: () -> Any?): X {
	val thrown = _try(exception, call).exceptionOrNull() ?: throw AssertionError("the call did not fail")
	return exception.cast(thrown)
}

/** A string as Rust's `Debug` prints it. */
private fun _quoted(s: String): String {
	val out = StringBuilder("\"")
	for (c in s) {
		when (c) {
			'"', '\\' -> out.append('\\').append(c)
			'\n' -> out.append("\\n")
			'\t' -> out.append("\\t")
			'\r' -> out.append("\\r")
			'\u0000' -> out.append("\\0")
			else -> if (c.code < 0x20 || c.code == 0x7f) out.append("\\u{${c.code.toString(16)}}") else out.append(c)
		}
	}
	return out.append('"').toString()
}

/** A char, which Kotlin holds as its code point, as text. */
private fun _char(c: Int): String = String(Character.toChars(c))

private fun _bound(kind: api.Bound, value: Any): String = when (kind) {
	api.Bound.INCLUDED -> "Included($value)"
	api.Bound.EXCLUDED -> "Excluded($value)"
	api.Bound.UNBOUNDED -> "Unbounded"
}

/** Bytes as Rust's `Debug` prints a list of them. */
private fun _bytes(bytes: ByteArray): String = bytes.joinToString(", ", "[", "]") { (it.toInt() and 0xff).toString() }

private fun <T> _option(value: T?, show: (T) -> String): String = if (value == null) "None" else "Some(${show(value)})"
"#;

/// The Kotlin file of `t`'s script.
pub(crate) fn emit(t: &Transcript) -> String {
	let mut hooks = Vec::new();
	for step in &t.steps {
		each_expr(step, &mut |e| {
			walk(e, &mut |e| {
				if let Kind::Local(name, _) = &e.kind
					&& !hooks.contains(&local(name))
				{
					hooks.push(local(name));
				}
			})
		});
	}
	let mut e = Emitter {
		t,
		depth: 1,
		mutated: Vec::new(),
		vars: Vec::new(),
		hooks,
		fresh: 0,
	};
	mutated(&t.steps, t, &mut e.mutated, &mut e.vars);
	let mut out = String::from(PRELUDE);
	out.push_str("\nfun transcript() {\n");
	let body = e.steps(&t.steps);
	out.push_str(&body);
	out.push_str("}\n");
	out
}

/// The locals a call takes `&mut` that are values, not objects, each a `Mut` whose `value`
/// the script reads; and those a value's `&mut self` method is called on, each a `var` the
/// method's return replaces.
fn mutated(steps: &[Step], t: &Transcript, out: &mut Vec<String>, vars: &mut Vec<String>) {
	let mut visit = |expr: &Expr| {
		walk(expr, &mut |e| {
			if let Kind::Call(id, receiver, args) = &e.kind {
				let f = t.function(*id);
				for (arg, param) in args.iter().zip(params(t, *id)) {
					if let Kind::Var(name) = &arg.kind
						&& written(&param)
						&& !out.contains(name)
					{
						out.push(name.clone());
					}
				}
				if let Some(receiver) = receiver
					&& let Kind::Var(name) = &receiver.kind
					&& f.receiver == Some(Receiver::Mut)
					&& matches!(id.owner, Owner::Record(_))
					&& !vars.contains(name)
				{
					vars.push(name.clone());
				}
			}
		})
	};
	for step in steps {
		each_expr(step, &mut visit);
	}
}

/// The types of `id`'s parameters Kotlin passes: all but a JNI env, which the binding passes.
fn params(t: &Transcript, id: FnId) -> Vec<Type> {
	(t.function(id).params.iter())
		.filter(|p| !matches!(&p.ty, Type::Jni(v) if v == "Env"))
		.map(|p| p.ty.clone())
		.collect()
}

/// Whether a parameter of `ty` is a `Mut` Rust writes back through.
fn written(ty: &Type) -> bool {
	matches!(ty, Type::Ref(Mutability::Mut, inner) if !matches!(**inner, Type::Object(_) | Type::Reader))
}

/// Each expression a step holds, its arms' and closures' steps included.
fn each_expr(step: &Step, f: &mut dyn FnMut(&Expr)) {
	match step {
		Step::Let(_, e) | Step::Do(e) => f(e),
		Step::Print(pieces) => pieces.iter().for_each(|p| {
			if let Piece::Value(e, _) = p {
				f(e)
			}
		}),
		Step::Drop(_) => {}
		Step::Match { value, ok, err } => {
			f(value);
			ok.steps.iter().chain(&err.steps).for_each(|s| each_expr(s, f));
		}
	}
}

/// `f` on `expr` and each expression inside it.
fn walk(expr: &Expr, f: &mut dyn FnMut(&Expr)) {
	f(expr);
	let mut inner = |e: &Expr| walk(e, f);
	match &expr.kind {
		Kind::Some(e)
		| Kind::Field(e, _)
		| Kind::Element(e, _)
		| Kind::Clone(e)
		| Kind::Unwrap(e)
		| Kind::UnwrapErr(e) => inner(e),
		Kind::IsSome(e) | Kind::IsNone(e) | Kind::IsErr(e) | Kind::Utf8(e) | Kind::Len(e) => inner(e),
		Kind::Tuple(es) | Kind::List(es) | Kind::Record(_, es) | Kind::Variant(_, _, es) | Kind::Local(_, es) => {
			es.iter().for_each(inner)
		}
		Kind::Call(_, r, args) => r.iter().map(|b| &**b).chain(args).for_each(inner),
		Kind::Binary(_, a, b) => {
			inner(a);
			inner(b);
		}
		Kind::Bound(_, Some(e)) => inner(e),
		Kind::Closure(_, steps, value) => {
			steps.iter().for_each(|s| each_expr(s, &mut |e| walk(e, f)));
			walk(value, f);
		}
		Kind::Format(pieces) => pieces.iter().for_each(|p| {
			if let Piece::Value(e, _) = p {
				walk(e, f)
			}
		}),
		Kind::Join { items, body, .. } => {
			walk(items, f);
			walk(body, f);
		}
		Kind::MapOr { option, none, some, .. } => {
			walk(option, f);
			walk(none, f);
			walk(some, f);
		}
		_ => {}
	}
}

struct Emitter<'t> {
	t: &'t Transcript,
	depth: usize,
	/// The locals each a `Mut`.
	mutated: Vec<String>,
	/// The locals each a `var`, which a value's `&mut self` method replaces.
	vars: Vec<String>,
	/// The names of `tests/api.kt`'s functions the script calls, which no local may shadow.
	hooks: Vec<String>,
	fresh: usize,
}

/// Kotlin's hard keywords, which no name may be.
const KEYWORDS: &[&str] = &[
	"as",
	"break",
	"class",
	"continue",
	"do",
	"else",
	"false",
	"for",
	"fun",
	"if",
	"in",
	"interface",
	"is",
	"null",
	"object",
	"package",
	"return",
	"super",
	"this",
	"throw",
	"true",
	"try",
	"typealias",
	"typeof",
	"val",
	"var",
	"when",
	"while",
];

/// What the script names by its root, which no local may shadow.
const ROOTS: &[&str] = &["api", "kotlin", "kotlinx", "java", "it", "field", "value"];

/// The Kotlin name of a function `transcript.rs` defines, or of a unit struct it does.
fn local(name: &str) -> String {
	match name.starts_with(|c: char| c.is_ascii_uppercase()) {
		true => name.to_string(),
		false => camel(name),
	}
}

impl Emitter<'_> {
	/// A local's name in Kotlin.
	fn ident(&self, name: &str) -> String {
		match KEYWORDS.contains(&name) || ROOTS.contains(&name) || self.hooks.iter().any(|h| h == name) {
			true => format!("{name}_"),
			false => name.to_string(),
		}
	}

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
					return match (unref(&value.ty), pat) {
						(Type::Unit, _) => self.line(out, &format!("{name} = {call}")),
						(ty, pat) => {
							let tmp = self.fresh("t");
							self.line(out, &format!("val {tmp} = {call}"));
							self.line(out, &format!("{name} = {tmp}.first"));
							let rest = match ty {
								Type::Tuple(elements) if elements.len() > 1 => {
									let parts: Vec<String> = (0..elements.len())
										.map(|i| format!("{tmp}.{}", ordinal(i + 1)))
										.collect();
									let class = self.tuple_class(elements);
									format!("{class}({})", parts.join(", "))
								}
								_ => format!("{tmp}.second"),
							};
							self.bind(pat, &rest, out)
						}
					};
				}
				let v = self.expr(value, out);
				match pat {
					Pat::Ignore(_) => {
						let v = self.dropped(&v, &value.ty);
						self.line(out, &v)
					}
					Pat::Name(name, _) if self.mutated.contains(name) => {
						let name = self.ident(name);
						self.line(out, &format!("val {name} = api.Mut({v})"))
					}
					pat => self.bind(pat, &v, out),
				}
			}
			Step::Print(pieces) => {
				let v = self.pieces(pieces, out);
				self.line(out, &format!("println({v})"));
			}
			Step::Do(value) => {
				if let Some((name, call)) = self.mut_receiver(value) {
					return match unref(&value.ty) {
						Type::Unit => self.line(out, &format!("{name} = {call}")),
						_ => self.line(out, &format!("{name} = {call}.first")),
					};
				}
				let v = self.expr(value, out);
				let v = self.dropped(&v, &value.ty);
				self.line(out, &v);
			}
			Step::Drop(name) => {
				let name = self.ident(name);
				self.line(out, &format!("{name}.close()"))
			}
			Step::Match { value, ok, err } => self.matched(value, ok, err, out),
		}
	}

	/// `v`, a value of `ty` the script drops: a `Result`'s error too, which Kotlin throws.
	fn dropped(&self, v: &str, ty: &Type) -> String {
		match ty {
			Type::Result(_, err) => format!("_try({}::class.java) {{ {v} }}", self.exception(err)),
			_ => v.to_string(),
		}
	}

	/// A call of a method taking a value's `&mut self` on a local: the local's name, and
	/// the call, which Kotlin returns the value from as the call left it.
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
		Some((self.ident(name), call))
	}

	/// `pat` bound to `v`: a name, a `var` if a method replaces it, or a tuple's elements,
	/// destructured where Kotlin can and read one by one where it nests.
	fn bind(&mut self, pat: &Pat, v: &str, out: &mut String) {
		match pat {
			Pat::Name(name, _) => {
				let keyword = if self.vars.contains(name) { "var" } else { "val" };
				let name = self.ident(name);
				self.line(out, &format!("{keyword} {name} = {v}"));
			}
			Pat::Ignore(_) => self.line(out, v),
			Pat::Tuple(pats) if pats.iter().all(|p| self.flat(p)) => {
				let names: Vec<String> = pats.iter().map(|p| self.pat(p)).collect();
				self.line(out, &format!("val ({}) = {v}", names.join(", ")));
			}
			Pat::Tuple(pats) => {
				let tmp = self.fresh("t");
				self.line(out, &format!("val {tmp} = {v}"));
				for (i, p) in pats.iter().enumerate() {
					if !matches!(p, Pat::Ignore(_)) {
						self.bind(p, &format!("{tmp}.component{}()", i + 1), out);
					}
				}
			}
		}
	}

	/// Whether `pat` binds in a destructuring declaration: a name no method replaces, or `_`.
	fn flat(&self, pat: &Pat) -> bool {
		match pat {
			Pat::Name(name, _) => !self.vars.contains(name) && !self.mutated.contains(name),
			Pat::Ignore(_) => true,
			Pat::Tuple(_) => false,
		}
	}

	fn pat(&self, pat: &Pat) -> String {
		match pat {
			Pat::Name(name, _) => self.ident(name),
			Pat::Ignore(_) => "_".to_string(),
			Pat::Tuple(pats) => {
				let parts: Vec<String> = pats.iter().map(|p| self.pat(p)).collect();
				format!("({})", parts.join(", "))
			}
		}
	}

	/// A lambda's parameter binding `pat`, and the statements its body begins with to bind
	/// what Kotlin cannot destructure there.
	fn param(&mut self, pat: &Pat) -> (String, String) {
		match pat {
			Pat::Tuple(pats) if !pats.iter().all(|p| self.flat(p)) => {
				let tmp = self.fresh("p");
				let mut body = String::new();
				self.depth += 1;
				self.bind(pat, &tmp, &mut body);
				self.depth -= 1;
				(tmp, body)
			}
			pat => (self.pat(pat), String::new()),
		}
	}

	fn matched(&mut self, value: &Expr, ok: &Arm, err: &Arm, out: &mut String) {
		match &value.ty {
			Type::Option(_) => {
				let v = self.expr(value, out);
				let tmp = self.fresh("o");
				self.line(out, &format!("val {tmp} = {v}"));
				self.line(out, &format!("if ({tmp} != null) {{"));
				self.arm(ok, &tmp, out);
				self.line(out, "} else {");
				self.arm(err, "null", out);
				self.line(out, "}");
			}
			Type::Result(_, err_ty) => {
				let exception = self.exception(err_ty);
				let v = self.expr(value, out);
				let tmp = self.fresh("r");
				self.line(out, &format!("val {tmp} = _try({exception}::class.java) {{ {v} }}"));
				self.line(out, &format!("if ({tmp}.isSuccess) {{"));
				self.arm(ok, &format!("{tmp}.getOrThrow()"), out);
				self.line(out, "} else {");
				self.arm(err, &format!("({tmp}.exceptionOrNull() as {exception}).error"), out);
				self.line(out, "}");
			}
			ty => panic!("a match on {ty:?}"),
		}
	}

	fn arm(&mut self, arm: &Arm, payload: &str, out: &mut String) {
		self.depth += 1;
		if let Some(pat) = &arm.pat {
			self.bind(pat, payload, out);
		}
		let body = self.steps(&arm.steps);
		out.push_str(&body);
		self.depth -= 1;
	}

	/// The exception a call failing with `err` throws.
	fn exception(&self, err: &Type) -> String {
		let name = match unref(err).base() {
			Type::String | Type::Str => self.t.named(LANGUAGE, "error string").to_string(),
			Type::Bounds(int) => self
				.t
				.named(LANGUAGE, &format!("error bounds_{}", int.rust()))
				.to_string(),
			ty @ (Type::Record(_) | Type::Enum(_) | Type::Object(_)) => {
				format!("{}Exception", self.t.type_name(LANGUAGE, ty))
			}
			ty => panic!("an error of {ty:?}"),
		};
		format!("api.{name}")
	}

	/// The Kotlin of a format's pieces: a string template.
	fn pieces(&mut self, pieces: &[Piece], out: &mut String) -> String {
		let mut template = String::from("\"");
		for piece in pieces {
			match piece {
				Piece::Text(text) => template.push_str(&escape(text)),
				Piece::Value(e, spec) => {
					let v = self.expr(e, out);
					let shown = self.show(&v, &e.ty, *spec);
					let _ = write!(template, "${{{shown}}}");
				}
			}
		}
		template.push('"');
		template
	}

	/// `v`, a value of `ty`, as Kotlin shows it the way Rust formats it with `spec`.
	fn show(&mut self, v: &str, ty: &Type, spec: Spec) -> String {
		let ty = unref(ty).base();
		match spec {
			Spec::Hex(width) => {
				assert!(
					matches!(ty, Type::Int(int) | Type::NonZero(int) if !int.signed()) || *ty == Type::U128,
					"hex of {ty:?}"
				);
				let hex = format!("{v}.toString(16)");
				return match width {
					0 => hex,
					width => format!("{hex}.padStart({width}, '0')"),
				};
			}
			Spec::Padded(width) => {
				assert!(
					matches!(ty, Type::Int(int) | Type::NonZero(int) if !int.signed()) || *ty == Type::U128,
					"zero-padded {ty:?}"
				);
				return format!("{v}.toString().padStart({width}, '0')");
			}
			Spec::Debug => return self.debug(v, ty),
			Spec::Display => {}
		}
		match ty {
			Type::Bool | Type::String | Type::Str | Type::Int(_) | Type::U128 | Type::NonZero(_) => v.to_string(),
			Type::Char => format!("_char({v})"),
			Type::Object(_) | Type::Record(_) | Type::Enum(_) => match self.text_of(ty) {
				Some(text) => format!("{v}.{text}()"),
				None => v.to_string(),
			},
			ty => panic!("Display of {ty:?}"),
		}
	}

	/// The name of a type's `Access::Text` method, which Kotlin names apart from `toString`.
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
			Type::Bool | Type::Int(_) | Type::U128 | Type::NonZero(_) => format!("{v}.toString()"),
			Type::String | Type::Str => format!("_quoted({v})"),
			Type::Option(inner) => {
				let x = self.fresh("x");
				let show = self.debug(&x, inner);
				format!("_option({v}) {{ {x} -> {show} }}")
			}
			Type::Vec | Type::Slice | Type::Bytes | Type::Array(_) => format!("_bytes({v}.toByteArray())"),
			Type::List(_, item) => {
				let x = self.fresh("x");
				let show = self.debug(&x, item);
				format!("{v}.joinToString(\", \", \"[\", \"]\") {{ {x} -> {show} }}")
			}
			Type::Tuple(elements) => {
				let x = self.fresh("x");
				let parts: Vec<String> = (0..elements.len())
					.map(|i| {
						let shown = self.debug(&format!("{x}.{}", ordinal(i)), &elements[i]);
						format!("${{{shown}}}")
					})
					.collect();
				let tail = if parts.len() == 1 { ",)" } else { ")" };
				format!("{v}.let {{ {x} -> \"({}{tail}\" }}", parts.join(", "))
			}
			Type::Bound(_) => format!("{v}.let {{ _bound(it.kind, it.value) }}"),
			Type::Record(_) | Type::Enum(_) | Type::Object(_) => format!("{v}.debug()"),
			ty => panic!("Debug of {ty:?}"),
		}
	}

	fn expr(&mut self, e: &Expr, out: &mut String) -> String {
		self.value(e, &e.ty, out)
	}

	/// The Kotlin of `e`, passed where `want` is taken.
	fn value(&mut self, e: &Expr, want: &Type, out: &mut String) -> String {
		match &e.kind {
			Kind::Unit => "Unit".to_string(),
			Kind::Bool(b) => b.to_string(),
			Kind::Int(n) => {
				let ty = match unref(&e.ty).base() {
					ty @ (Type::Int(_) | Type::NonZero(_) | Type::U128) => ty,
					_ => unref(want).base(),
				};
				int(*n, ty)
			}
			Kind::Float(f) => match unref(want).base() {
				Type::F32 => float32(*f as f32),
				_ => float64(*f),
			},
			Kind::Char(c) => format!("0x{:x}", *c as u32),
			Kind::Str(s) => match unref(want).base() {
				Type::Path | Type::PathBuf => format!("api.PathBuf({})", string(s)),
				_ => string(s),
			},
			Kind::Bytes(b) => match unref(want).base() {
				Type::Array(n) => format!("api.ArrayU8x{n}({})", byte_array(b)),
				_ => format!("api.Bytes({})", byte_array(b)),
			},
			Kind::Duration(secs, nanos) => format!("api.Duration({secs}uL, {nanos}u)"),
			Kind::Socket(std::net::SocketAddr::V4(a)) => {
				format!(
					"api.SocketAddrV4({}, {})",
					ipv4(a.ip()),
					int(Num::new(a.port().into()), &Type::Int(Int::U16))
				)
			}
			Kind::Socket(std::net::SocketAddr::V6(a)) => format!(
				"api.SocketAddrV6({}, {}, {}u, {}u)",
				ipv6(a.ip()),
				int(Num::new(a.port().into()), &Type::Int(Int::U16)),
				a.flowinfo(),
				a.scope_id()
			),
			Kind::Ip(std::net::IpAddr::V4(ip)) => ipv4(ip),
			Kind::Ip(std::net::IpAddr::V6(ip)) => ipv6(ip),
			Kind::None => "null".to_string(),
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
				if variant.fields.is_empty() {
					return format!("api.{name}.{}", variant.name);
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
				let Type::Bound(int_ty) = unref(&e.ty) else {
					panic!("a bound of {:?}", e.ty)
				};
				let (kind, v) = match (kind, value) {
					(BoundKind::Unbounded, _) => ("UNBOUNDED", int(Num::new(0), &Type::Int(*int_ty))),
					(BoundKind::Included, Some(v)) => ("INCLUDED", self.value(v, &Type::Int(*int_ty), out)),
					(BoundKind::Excluded, Some(v)) => ("EXCLUDED", self.value(v, &Type::Int(*int_ty), out)),
					_ => unreachable!(),
				};
				format!("api.Bound{}(api.Bound.{kind}, {v})", int_name(*int_ty))
			}
			Kind::Default => {
				let name = self.t.type_name(LANGUAGE, unref(&e.ty).base()).to_string();
				format!("api.{name}.default()")
			}
			Kind::Var(name) => match self.mutated.contains(name) {
				// Passed `&mut`, the `Mut` itself; read, its value.
				true if written(want) => self.ident(name),
				true => format!("{}.value", self.ident(name)),
				false => self.ident(name),
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
				format!("{b}.{}", field_name(&self.t.api.records[*r].fields[*i].name))
			}
			Kind::Element(base, i) => {
				let b = self.expr(base, out);
				match unref(&base.ty).base() {
					Type::Range(_) | Type::Bounds(_) => format!("{b}.{}", if *i == 0 { "start" } else { "end" }),
					Type::Tuple(_) => format!("{b}.{}", ordinal(*i)),
					ty => panic!("an element of {ty:?}"),
				}
			}
			Kind::Clone(inner) => {
				let v = self.expr(inner, out);
				match unref(&inner.ty).base() {
					Type::Object(_) => format!("{v}.clone()"),
					Type::List(..) if block(unref(&inner.ty).base()).is_none() => format!("{v}.toList()"),
					// Records, bytes, and the rest are immutable, so the value is its clone.
					Type::Record(_) | Type::Enum(_) | Type::Bytes | Type::String | Type::Int(_) | Type::Bool => v,
					ty => panic!("a clone of {ty:?}"),
				}
			}
			Kind::Unwrap(inner) => {
				let v = self.expr(inner, out);
				match &inner.ty {
					Type::Result(..) => v,
					_ => format!("{v}!!"),
				}
			}
			Kind::UnwrapErr(inner) => {
				let Type::Result(_, err) = &inner.ty else {
					panic!("unwrap_err of {:?}", inner.ty)
				};
				let exception = self.exception(err);
				let v = self.expr(inner, out);
				format!("_thrown({exception}::class.java) {{ {v} }}.error")
			}
			Kind::IsErr(inner) => {
				let Type::Result(_, err) = &inner.ty else {
					panic!("is_err of {:?}", inner.ty)
				};
				let exception = self.exception(err);
				let v = self.expr(inner, out);
				format!("_try({exception}::class.java) {{ {v} }}.isFailure")
			}
			Kind::IsSome(inner) => format!("({} != null)", self.expr(inner, out)),
			Kind::IsNone(inner) => format!("({} == null)", self.expr(inner, out)),
			Kind::Binary(op, a, b) => {
				let ty = unref(&a.ty).base().clone();
				let (a, b) = (self.value(a, &ty, out), self.value(b, &ty, out));
				let sym = match op {
					Op::Eq => "==",
					Op::Ne => "!=",
					Op::Lt => "<",
					Op::Gt => ">",
					Op::Add => "+",
					Op::Sub => "-",
					Op::Mul => "*",
					Op::Rem => "%",
					Op::And => "&&",
				};
				let v = format!("({a} {sym} {b})");
				// Kotlin widens arithmetic on a byte or a short to an `Int`, which Rust keeps.
				match (op, &ty) {
					(Op::Add | Op::Sub | Op::Mul | Op::Rem, Type::Int(int)) => match narrow(*int) {
						Some(to) => format!("{v}.{to}()"),
						None => v,
					},
					_ => v,
				}
			}
			Kind::Closure(params, steps, value) => self.closure(params, steps, value),
			Kind::Local(name, args) => {
				let args: Vec<String> = args.iter().map(|a| self.expr(a, out)).collect();
				format!("{}({})", local(name), args.join(", "))
			}
			Kind::Format(pieces) => self.pieces(pieces, out),
			Kind::Join { items, pat, body, sep } => {
				let items = self.expr(items, out);
				let (param, bound) = self.param(pat);
				let body = self.lambda_body(&bound, body);
				format!("{items}.joinToString({}) {{ {param} ->{body}}}", string(sep))
			}
			Kind::MapOr {
				option,
				none,
				pat,
				some,
			} => {
				let option = self.expr(option, out);
				let none = self.expr(none, out);
				let Pat::Name(..) = pat else {
					panic!("map_or binding a pattern")
				};
				let param = self.pat(pat);
				let some = self.expr(some, out);
				format!("{option}.let {{ {param} -> if ({param} == null) {none} else {some} }}")
			}
			Kind::Utf8(inner) => format!("{}.toByteArray().decodeToString()", self.expr(inner, out)),
			Kind::Len(inner) => {
				let v = self.expr(inner, out);
				match unref(&inner.ty).base() {
					Type::String | Type::Str => format!("{v}.encodeToByteArray().size.toULong()"),
					_ => format!("{v}.size.toULong()"),
				}
			}
		}
	}

	/// A lambda's body after its parameters: the statements `bound` binds, then `value`.
	fn lambda_body(&mut self, bound: &str, value: &Expr) -> String {
		let mut pre = String::new();
		self.depth += 1;
		let v = self.expr(value, &mut pre);
		self.depth -= 1;
		match (bound.is_empty() && pre.is_empty(), &value.kind) {
			(true, _) => format!(" {v} "),
			(false, Kind::Unit) => format!("\n{bound}{pre}{}", self.indent()),
			(false, _) => format!("\n{bound}{pre}{}\t{v}\n{}", self.indent(), self.indent()),
		}
	}

	fn tuple(&mut self, e: &Expr, elements: &[Expr], want: &Type, out: &mut String) -> String {
		match unref(&e.ty).base() {
			Type::Range(int_ty) => {
				let ty = Type::Int(*int_ty);
				let (a, b) = (self.value(&elements[0], &ty, out), self.value(&elements[1], &ty, out));
				format!("api.Range{}({a}, {b})", int_name(*int_ty))
			}
			Type::Bounds(int_ty) => {
				let ty = Type::Bound(*int_ty);
				let (a, b) = (self.value(&elements[0], &ty, out), self.value(&elements[1], &ty, out));
				format!("api.Bounds{}({a}, {b})", int_name(*int_ty))
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
				let class = self.tuple_class(&wants);
				format!("{class}({})", parts.join(", "))
			}
		}
	}

	/// The Kotlin class of a tuple of `elements`: `Pair` and `Triple`, unless an element
	/// compares by its contents, which theirs cannot, else the binding's `TupleN`.
	fn tuple_class(&self, elements: &[Type]) -> String {
		let contents = elements.iter().any(|e| holds_block(e) || self.holds_float(e));
		match elements.len() {
			2 | 3 if contents => format!("api.Tuple{}", elements.len()),
			2 => "Pair".to_string(),
			3 => "Triple".to_string(),
			n => format!("api.Tuple{n}"),
		}
	}

	fn holds_float(&self, ty: &Type) -> bool {
		self.floats(ty, &mut Vec::new())
	}

	fn floats(&self, ty: &Type, seen: &mut Vec<(bool, usize)>) -> bool {
		match ty {
			Type::F32 | Type::F64 => true,
			Type::Option(inner) | Type::Box(inner) | Type::Ref(_, inner) | Type::List(_, inner) => {
				self.floats(inner, seen)
			}
			Type::Tuple(elements) => elements.iter().any(|e| self.floats(e, seen)),
			Type::Record(i) if !seen.contains(&(false, *i)) => {
				seen.push((false, *i));
				self.t.api.records[*i].fields.iter().any(|f| self.floats(&f.ty, seen))
			}
			Type::Enum(i) if !seen.contains(&(true, *i)) => {
				seen.push((true, *i));
				(self.t.api.enums[*i].variants.iter())
					.flat_map(|v| &v.fields)
					.any(|f| self.floats(&f.ty, seen))
			}
			_ => false,
		}
	}

	fn list(&mut self, items: &[Expr], want: &Type, out: &mut String) -> String {
		let list = unref(want).base();
		let Type::List(_, item) = list else {
			panic!("a list as {list:?}")
		};
		let item = (**item).clone();
		let parts: Vec<String> = items.iter().map(|e| self.value(e, &item, out)).collect();
		match block(list) {
			Some(element) => format!("{}({})", array_of(element), parts.join(", ")),
			None => format!("listOf({})", parts.join(", ")),
		}
	}

	/// A closure as a lambda, its steps before the value it returns.
	fn closure(&mut self, params: &[Pat], steps: &[Step], value: &Expr) -> String {
		let mut bound = String::new();
		let mut names = Vec::new();
		for p in params {
			let (name, binds) = self.param(p);
			names.push(name);
			bound.push_str(&binds);
		}
		self.depth += 1;
		bound.push_str(&self.steps(steps));
		self.depth -= 1;
		let body = self.lambda_body(&bound, value);
		match names.is_empty() {
			true => format!("{{{body}}}"),
			false => format!("{{ {} ->{body}}}", names.join(", ")),
		}
	}

	/// A call of `id`: a function of the package, an associated function of a type, a
	/// method, or a field's getter or setter.
	fn call(&mut self, id: FnId, receiver: Option<&Expr>, args: &[Expr], out: &mut String) -> String {
		let f = self.t.function(id);
		let name = self.t.name(LANGUAGE, id).to_string();
		let (access, future) = (f.access, f.future);
		let params = params(self.t, id);
		assert_eq!(
			args.len(),
			params.len(),
			"a call of {} with {} arguments",
			self.t.key(id),
			args.len()
		);
		let args: Vec<String> = args.iter().zip(&params).map(|(a, ty)| self.value(a, ty, out)).collect();
		let target = match (receiver, self.t.owner(id)) {
			(Some(r), _) => self.expr(r, out),
			(None, Some(ty)) => format!("api.{}", self.t.type_name(LANGUAGE, &ty)),
			(None, None) => "api".to_string(),
		};
		// A handle made for the call is a temporary, which Rust drops once the statement
		// ends, ending any borrow it holds: Kotlin closes it once the call returns.
		let temporary = receiver.is_some_and(temporary) && f.receiver != Some(Receiver::Owned);
		let (target, close) = match temporary {
			true => {
				let tmp = self.fresh("h");
				(tmp.clone(), Some((target, tmp)))
			}
			false => (target, None),
		};
		let call = match access {
			Access::Field { set: false, .. } | Access::Const => format!("{target}.{name}"),
			_ => format!("{target}.{name}({})", args.join(", ")),
		};
		let call = match close {
			Some((handle, tmp)) => format!("{handle}.use {{ {tmp} -> {call} }}"),
			None => call,
		};
		match future {
			true => format!("kotlinx.coroutines.runBlocking {{ {call} }}"),
			false => call,
		}
	}
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

/// Whether `ty` holds a list Kotlin keeps as a primitive array anywhere but in a record or
/// an enum, which compare their own.
fn holds_block(ty: &Type) -> bool {
	match ty {
		_ if block(ty).is_some() => true,
		Type::Option(inner) | Type::Box(inner) | Type::Ref(_, inner) | Type::List(_, inner) => holds_block(inner),
		Type::Tuple(elements) => elements.iter().any(holds_block),
		_ => false,
	}
}

/// The element of a list Kotlin holds as a primitive array: a `Vec`, slice, boxed slice,
/// or array of a fixed-width integer or a float.
fn block(ty: &Type) -> Option<&Type> {
	match ty {
		Type::List(Collection::Vec | Collection::Slice | Collection::Box | Collection::Array(_), inner) => {
			match &**inner {
				Type::Int(Int::Usize | Int::Isize) => None,
				Type::Int(_) | Type::F32 | Type::F64 => Some(inner),
				_ => None,
			}
		}
		_ => None,
	}
}

/// The function making a primitive array of `element`s.
fn array_of(element: &Type) -> &'static str {
	match element {
		Type::Int(Int::U8) => "ubyteArrayOf",
		Type::Int(Int::I8) => "byteArrayOf",
		Type::Int(Int::U16) => "ushortArrayOf",
		Type::Int(Int::I16) => "shortArrayOf",
		Type::Int(Int::U32) => "uintArrayOf",
		Type::Int(Int::I32) => "intArrayOf",
		Type::Int(Int::U64) => "ulongArrayOf",
		Type::Int(Int::I64) => "longArrayOf",
		Type::F32 => "floatArrayOf",
		Type::F64 => "doubleArrayOf",
		ty => panic!("an array of {ty:?}"),
	}
}

/// An integer literal of `ty`, typed as Kotlin holds it wherever it stands.
fn int(n: Num, ty: &Type) -> String {
	let int = match ty {
		Type::Int(int) | Type::NonZero(int) => *int,
		Type::U128 => return format!("java.math.BigInteger(\"{n}\")"),
		ty => panic!("an integer as {ty:?}"),
	};
	match int {
		Int::U8 => format!("{n}.toUByte()"),
		Int::U16 => format!("{n}.toUShort()"),
		Int::U32 => format!("{n}u"),
		Int::U64 | Int::Usize => format!("{n}uL"),
		Int::I8 => format!("({n}).toByte()"),
		Int::I16 => format!("({n}).toShort()"),
		Int::I32 if n == Num::new(i32::MIN.into()) => "Int.MIN_VALUE".to_string(),
		Int::I32 => format!("({n})"),
		Int::I64 | Int::Isize if n == Num::new(i64::MIN.into()) => "Long.MIN_VALUE".to_string(),
		Int::I64 | Int::Isize => format!("({n}L)"),
	}
}

/// What Kotlin's arithmetic on `int`, which it widens, converts back with.
fn narrow(int: Int) -> Option<&'static str> {
	match int {
		Int::U8 => Some("toUByte"),
		Int::U16 => Some("toUShort"),
		Int::I8 => Some("toByte"),
		Int::I16 => Some("toShort"),
		_ => None,
	}
}

fn float64(f: f64) -> String {
	match f.is_finite() && !(f == 0.0 && f.is_sign_negative()) {
		true => format!("{f:?}"),
		false => format!("Double.fromBits(0x{:x}uL.toLong())", f.to_bits()),
	}
}

fn float32(f: f32) -> String {
	match f.is_finite() && !(f == 0.0 && f.is_sign_negative()) {
		true => format!("{f:?}f"),
		false => format!("Float.fromBits(0x{:x}u.toInt())", f.to_bits()),
	}
}

fn ipv4(ip: &std::net::Ipv4Addr) -> String {
	format!("api.Ipv4Addr({}u)", u32::from(*ip))
}

fn ipv6(ip: &std::net::Ipv6Addr) -> String {
	format!("api.Ipv6Addr({})", byte_array(&ip.octets()))
}

/// A field's Kotlin name: in camel case, quoted if a keyword.
fn field_name(name: &str) -> String {
	let name = camel(name);
	match KEYWORDS.contains(&name.as_str()) {
		true => format!("`{name}`"),
		false => name,
	}
}

/// A snake-case name in camel case: `into_label` is `intoLabel`, and `0` is `_0`.
fn camel(name: &str) -> String {
	if name.starts_with(|c: char| c.is_ascii_digit()) {
		return format!("_{name}");
	}
	let mut out = String::new();
	let mut up = false;
	for c in name.chars() {
		match c {
			'_' if !out.is_empty() => up = true,
			c if up => {
				out.extend(c.to_uppercase());
				up = false;
			}
			c => out.push(c),
		}
	}
	out
}

/// The tuple element `k` names, from 0.
fn ordinal(k: usize) -> String {
	const ORDINALS: [&str; 12] = [
		"first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth", "eleventh",
		"twelfth",
	];
	ORDINALS
		.get(k)
		.map_or_else(|| format!("item{}", k + 1), |o| o.to_string())
}

/// A plain enum's entry: its variant's name in upper snake case.
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

/// Text escaped for a Kotlin string literal or template.
fn escape(s: &str) -> String {
	let mut out = String::new();
	for c in s.chars() {
		match c {
			'"' | '\\' | '$' => {
				out.push('\\');
				out.push(c);
			}
			'\n' => out.push_str("\\n"),
			'\t' => out.push_str("\\t"),
			'\r' => out.push_str("\\r"),
			c if (c as u32) < 0x20 || c as u32 == 0x7f => {
				let _ = write!(out, "\\u{:04x}", c as u32);
			}
			c => out.push(c),
		}
	}
	out
}

/// A Kotlin string literal.
fn string(s: &str) -> String {
	format!("\"{}\"", escape(s))
}

/// A Kotlin `ByteArray` of `b`: its text encoded where it is printable UTF-8.
fn byte_array(b: &[u8]) -> String {
	match std::str::from_utf8(b) {
		Ok(text) if !text.is_empty() && text.chars().all(|c| (' '..='~').contains(&c)) => {
			format!("{}.encodeToByteArray()", string(text))
		}
		_ => {
			let parts: Vec<String> = b.iter().map(|&byte| (byte as i8).to_string()).collect();
			format!("byteArrayOf({})", parts.join(", "))
		}
	}
}
