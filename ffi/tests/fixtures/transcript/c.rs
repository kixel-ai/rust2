//! The transcript as C: a file `tests/api.c` includes after its own functions, whose
//! `script()` runs the transcript through the C ABI, calling the functions `api.c` defines
//! under the names `transcript.rs` gives them.
//!
//! C frees what Rust drops, when Rust drops it: a temporary at the end of its statement, a
//! local at the end of its scope or at its `drop`, and a local moved into a call once that
//! call returns. A value C builds from literals is a view, which Rust copies and never frees,
//! so only what Rust returned is ever freed; one passed back by value is passed as a view
//! of itself, which leaves it the script's to free.

use std::collections::HashMap;
use std::fmt::Write;

use super::ir::{Access, Closure, Family, Int, Mutability, Receiver, Type};
use super::{Arm, BoundKind, Expr, FnId, Kind, Num, Op, Pat, Piece, Spec, Step, Transcript, unref};

const LANGUAGE: &str = "C";

/// Helpers the script calls, each for what C has no expression of: text built as Rust
/// formats it, bytes spelled by their length, and floats by their bits.
const PRELUDE: &str = r#"/* Generated from fixtures/api/examples/transcript.rs by tests/fixtures/transcript. */

#include <stdarg.h>

#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wunused-variable"
#pragma GCC diagnostic ignored "-Wunused-but-set-variable"
#pragma GCC diagnostic ignored "-Wunused-parameter"
#pragma GCC diagnostic ignored "-Wunused-function"

/* Text a line or a format! builds. */
typedef struct {
	char *ptr;
	size_t len;
	size_t cap;
} tr_text;

static void tr_put(tr_text *t, const char *s, size_t n) {
	if (t->len + n + 1 > t->cap) {
		t->cap = (t->len + n + 1) * 2;
		t->ptr = realloc(t->ptr, t->cap);
		if (!t->ptr) {
			abort();
		}
	}
	if (n) {
		memcpy(t->ptr + t->len, s, n);
	}
	t->len += n;
}

static void tr_str(tr_text *t, const char *s) {
	tr_put(t, s, strlen(s));
}

static void tr_fmt(tr_text *t, const char *fmt, ...) {
	char small[64];
	va_list args;
	va_start(args, fmt);
	int n = vsnprintf(small, sizeof small, fmt, args);
	va_end(args);
	if (n < (int)sizeof small) {
		tr_put(t, small, (size_t)n);
		return;
	}
	char *big = malloc((size_t)n + 1);
	va_start(args, fmt);
	vsnprintf(big, (size_t)n + 1, fmt, args);
	va_end(args);
	tr_put(t, big, (size_t)n);
	free(big);
}

static void tr_buf(tr_text *t, PREFIX_buf b) {
	tr_put(t, (const char *)b.ptr, b.len);
}

/* A string as Rust's Debug prints it. */
static void tr_quoted(tr_text *t, PREFIX_buf b) {
	tr_str(t, "\"");
	for (size_t i = 0; i < b.len; i++) {
		unsigned char c = b.ptr[i];
		switch (c) {
		case '"':
			tr_str(t, "\\\"");
			break;
		case '\\':
			tr_str(t, "\\\\");
			break;
		case '\n':
			tr_str(t, "\\n");
			break;
		case '\r':
			tr_str(t, "\\r");
			break;
		case '\t':
			tr_str(t, "\\t");
			break;
		case 0:
			tr_str(t, "\\0");
			break;
		default:
			if (c < 0x20 || c == 0x7f) {
				tr_fmt(t, "\\u{%x}", c);
			} else {
				tr_put(t, (const char *)&c, 1);
			}
		}
	}
	tr_str(t, "\"");
}

/* A char's UTF-8. */
static void tr_char(tr_text *t, uint32_t c) {
	char out[4];
	size_t n;
	if (c < 0x80) {
		out[0] = (char)c;
		n = 1;
	} else if (c < 0x800) {
		out[0] = (char)(0xc0 | (c >> 6));
		out[1] = (char)(0x80 | (c & 0x3f));
		n = 2;
	} else if (c < 0x10000) {
		out[0] = (char)(0xe0 | (c >> 12));
		out[1] = (char)(0x80 | ((c >> 6) & 0x3f));
		out[2] = (char)(0x80 | (c & 0x3f));
		n = 3;
	} else {
		out[0] = (char)(0xf0 | (c >> 18));
		out[1] = (char)(0x80 | ((c >> 12) & 0x3f));
		out[2] = (char)(0x80 | ((c >> 6) & 0x3f));
		out[3] = (char)(0x80 | (c & 0x3f));
		n = 4;
	}
	tr_put(t, out, n);
}

/* A u128 in hex, zero-padded to width digits. */
static void tr_hex128(tr_text *t, PREFIX_u128 v, int width) {
	char digits[33];
	if (v.hi) {
		snprintf(digits, sizeof digits, "%llx%016llx", (unsigned long long)v.hi, (unsigned long long)v.lo);
	} else {
		snprintf(digits, sizeof digits, "%llx", (unsigned long long)v.lo);
	}
	for (int pad = width - (int)strlen(digits); pad > 0; pad--) {
		tr_str(t, "0");
	}
	tr_str(t, digits);
}

/* The line, written whole, then freed. */
static void tr_line(tr_text *t) {
	tr_put(t, "\n", 1);
	fwrite(t->ptr, 1, t->len, stdout);
	free(t->ptr);
}

/* The text as a buffer Rust owns, the text freed. */
static PREFIX_buf tr_done(tr_text *t) {
	PREFIX_buf b = PREFIX_buf_copy((PREFIX_buf){(const uint8_t *)(t->ptr ? t->ptr : ""), t->len, NULL});
	free(t->ptr);
	return b;
}

/* A buffer as a view, which Rust copies rather than takes. */
static PREFIX_buf tr_view(PREFIX_buf b) {
	b.owner = NULL;
	return b;
}

static double tr_f64(uint64_t bits) {
	double f;
	memcpy(&f, &bits, sizeof f);
	return f;
}

static float tr_f32(uint32_t bits) {
	float f;
	memcpy(&f, &bits, sizeof f);
	return f;
}

/* What an unwrap of None or of an error does in Rust. */
static void tr_fail(const char *what) {
	fprintf(stderr, "%s\n", what);
	abort();
}

/* The items of an empty array, which is no None. */
static _Alignas(max_align_t) const unsigned char tr_empty[64];
"#;

/// The C file of `t`'s script, `header` the export's C header it calls.
pub(crate) fn emit(t: &Transcript, header: &str) -> String {
	let prefix = t.named(LANGUAGE, "prefix").to_string();
	let mut e = Emitter {
		t,
		upper: prefix.to_uppercase(),
		declared: declared(header, &prefix),
		p: prefix,
		defs: String::new(),
		out: String::new(),
		depth: 1,
		fresh: 0,
		frames: Vec::new(),
		scopes: vec![Scope::default()],
	};
	for step in &t.steps {
		e.step(step);
	}
	e.drop_scope();
	let mut out = PRELUDE.replace("PREFIX", &e.p);
	out.push('\n');
	out.push_str(&e.defs);
	out.push_str("static void script(void) {\n");
	out.push_str(&e.out);
	out.push_str("}\n\n#pragma GCC diagnostic pop\n");
	out
}

/// Each function `header` declares, by its name: how many parameters it takes, and
/// whether its last is a status.
fn declared(header: &str, prefix: &str) -> HashMap<String, (usize, bool)> {
	let status = format!("{prefix}_status *");
	let mut out = HashMap::new();
	for line in header.lines() {
		let skip = line.starts_with(|c: char| c.is_whitespace() || c == '#' || c == '/' || c == '*')
			|| line.starts_with("typedef")
			|| !line.ends_with(");");
		if skip {
			continue;
		}
		let Some(open) = line.find('(') else { continue };
		let name: String = {
			let before: Vec<char> = line[..open].chars().collect();
			let start = before
				.iter()
				.rposition(|c| !(c.is_alphanumeric() || *c == '_'))
				.map_or(0, |i| i + 1);
			before[start..].iter().collect()
		};
		let params = &line[open + 1..line.len() - 2];
		let mut parts = Vec::new();
		let (mut depth, mut start) = (0, 0);
		for (i, c) in params.char_indices() {
			match c {
				'(' => depth += 1,
				')' => depth -= 1,
				',' if depth == 0 => {
					parts.push(params[start..i].trim());
					start = i + 1;
				}
				_ => {}
			}
		}
		parts.push(params[start..].trim());
		let count = if parts == ["void"] { 0 } else { parts.len() };
		let statused = parts.last().is_some_and(|p| p.starts_with(&status));
		out.insert(name, (count, statused));
	}
	out
}

/// What the script frees once its owner goes: `key` names it, so a move out finds it.
#[derive(Clone)]
struct Owned {
	key: String,
	code: Vec<String>,
}

#[derive(Default)]
struct Scope {
	locals: Vec<(String, Val)>,
	owned: Vec<Owned>,
}

/// A C value: its expression, its Rust type, and its C type.
#[derive(Clone)]
struct Val {
	c: String,
	ty: Type,
	cty: String,
	form: Form,
	/// Whether `c` names a place, so it may be read again without running anything.
	named: bool,
	/// Whether `c` has an address.
	lv: bool,
}

#[derive(Clone)]
enum Form {
	Plain,
	/// A tuple as its elements, each its own value, as a call returns one through its outs.
	Tuple(Vec<Val>),
	/// A call's `Result`: whether it succeeded, its value, and its error.
	Result {
		flag: String,
		ok: Box<Val>,
		err: Box<Val>,
	},
}

impl Val {
	fn new(c: impl Into<String>, ty: Type, cty: impl Into<String>) -> Val {
		Val {
			c: c.into(),
			ty,
			cty: cty.into(),
			form: Form::Plain,
			named: false,
			lv: false,
		}
	}

	/// A value a variable or another place holds.
	fn place(c: impl Into<String>, ty: Type, cty: impl Into<String>) -> Val {
		Val {
			named: true,
			lv: true,
			..Val::new(c, ty, cty)
		}
	}

	fn unit() -> Val {
		Val::new("", Type::Unit, "void")
	}
}

struct Emitter<'t> {
	t: &'t Transcript,
	p: String,
	upper: String,
	declared: HashMap<String, (usize, bool)>,
	/// The closures' functions, before the script that passes them.
	defs: String,
	out: String,
	depth: usize,
	fresh: usize,
	/// The temporaries of each statement being written, freed at its end.
	frames: Vec<Vec<Owned>>,
	scopes: Vec<Scope>,
}

impl Emitter<'_> {
	fn line(&mut self, line: &str) {
		let _ = writeln!(self.out, "{}{line}", "\t".repeat(self.depth));
	}

	fn fresh(&mut self, what: &str) -> String {
		self.fresh += 1;
		format!("tr_{what}{}", self.fresh)
	}

	// Ownership.

	/// Start a statement: what it makes is freed at its end.
	fn begin(&mut self) {
		self.frames.push(Vec::new());
	}

	fn end(&mut self) {
		let frame = self.frames.pop().expect("a statement");
		for owned in frame.iter().rev() {
			for code in &owned.code {
				self.line(code);
			}
		}
	}

	/// A block of its own scope, opened by `head`.
	fn open(&mut self, head: &str) {
		self.line(&format!("{head} {{"));
		self.depth += 1;
		self.scopes.push(Scope::default());
		self.frames.push(Vec::new());
	}

	fn close(&mut self) {
		self.end();
		self.drop_scope();
		self.depth -= 1;
		self.line("}");
	}

	/// Free what the innermost scope owns, the last first, as Rust drops its locals.
	fn drop_scope(&mut self) {
		let scope = self.scopes.pop().expect("a scope");
		for owned in scope.owned.iter().rev() {
			for code in &owned.code {
				self.line(code);
			}
		}
	}

	/// `c`, of type `ty`, freed at the end of this statement.
	fn temp(&mut self, c: &str, ty: &Type) {
		if self.owns(ty) {
			let code = self.free(c, ty);
			self.frames.last_mut().expect("a statement").push(Owned {
				key: c.to_string(),
				code,
			});
		}
	}

	/// Take `key` out of this statement's temporaries: whether it was one.
	fn take(&mut self, key: &str) -> bool {
		let Some(frame) = self.frames.last_mut() else {
			return false;
		};
		match frame.iter().position(|o| o.key == key) {
			Some(i) => {
				frame.remove(i);
				true
			}
			None => false,
		}
	}

	/// The local `name`.
	fn lookup(&self, name: &str) -> Option<&Val> {
		self.scopes
			.iter()
			.rev()
			.find_map(|s| s.locals.iter().rev().find(|(n, _)| n == name).map(|(_, v)| v))
	}

	fn local(&self, name: &str) -> Val {
		self.lookup(name).unwrap_or_else(|| panic!("no local {name}")).clone()
	}

	/// What a scope owns under `key`, taken out.
	fn take_owned(&mut self, key: &str) -> Option<Owned> {
		for scope in self.scopes.iter_mut().rev() {
			if let Some(i) = scope.owned.iter().position(|o| o.key == key) {
				return Some(scope.owned.remove(i));
			}
		}
		None
	}

	/// The local `name` moved into a call: freed once the call's statement ends.
	fn consume(&mut self, name: &str) {
		let val = self.local(name);
		let keys: Vec<String> = match &val.form {
			Form::Tuple(parts) => parts.iter().map(|p| p.c.clone()).collect(),
			_ => vec![val.c.clone()],
		};
		for key in keys {
			if let Some(owned) = self.take_owned(&key) {
				self.frames.last_mut().expect("a statement").push(owned);
			}
		}
	}

	/// The local `name`, which a call wrote back through its `&mut`, owns what Rust wrote.
	fn rewritten(&mut self, name: &str) {
		let val = self.local(name);
		if !self.owns(&val.ty) || self.scopes.iter().any(|s| s.owned.iter().any(|o| o.key == val.c)) {
			return;
		}
		assert!(!self.holds_handle(&val.ty), "a &mut of a local holding a handle");
		let code = self.free(&val.c, &val.ty);
		let scope = (self.scopes.iter_mut().rev())
			.find(|s| s.locals.iter().any(|(n, _)| n == name))
			.expect("the local's scope");
		scope.owned.push(Owned { key: val.c, code });
	}

	// Types.

	fn owns(&self, ty: &Type) -> bool {
		self.owns_in(ty, &mut Vec::new())
	}

	fn owns_in(&self, ty: &Type, seen: &mut Vec<Type>) -> bool {
		let api = &self.t.api;
		match ty {
			Type::Str | Type::String | Type::Slice | Type::Vec | Type::Bytes | Type::Path | Type::PathBuf => true,
			Type::List(..) | Type::Object(_) => true,
			Type::Option(inner) | Type::Box(inner) | Type::Ref(_, inner) => self.owns_in(inner, seen),
			Type::Tuple(elements) => elements.iter().any(|e| self.owns_in(e, seen)),
			Type::Record(i) if !seen.contains(ty) => {
				seen.push(ty.clone());
				api.records[*i].opaque.is_none() && api.records[*i].fields.iter().any(|f| self.owns_in(&f.ty, seen))
			}
			Type::Enum(i) if !seen.contains(ty) => {
				seen.push(ty.clone());
				(api.enums[*i].variants.iter().flat_map(|v| &v.fields)).any(|f| self.owns_in(&f.ty, seen))
			}
			_ => false,
		}
	}

	/// Whether a value of `ty` holds bytes Rust would take if passed by value.
	fn viewed(&self, ty: &Type) -> bool {
		let api = &self.t.api;
		match ty {
			Type::Str | Type::String | Type::Slice | Type::Vec | Type::Bytes | Type::Path | Type::PathBuf => true,
			Type::Option(inner) | Type::Box(inner) | Type::Ref(_, inner) => self.viewed(inner),
			Type::Tuple(elements) => elements.iter().any(|e| self.viewed(e)),
			Type::Record(i) => {
				api.records[*i].opaque.is_none() && api.records[*i].fields.iter().any(|f| self.viewed(&f.ty))
			}
			Type::Enum(i) => (api.enums[*i].variants.iter().flat_map(|v| &v.fields)).any(|f| self.viewed(&f.ty)),
			_ => false,
		}
	}

	fn holds_handle(&self, ty: &Type) -> bool {
		let mut handles = Vec::new();
		self.handles(ty, &mut handles, &mut Vec::new());
		!handles.is_empty()
	}

	fn handles(&self, ty: &Type, out: &mut Vec<usize>, seen: &mut Vec<Type>) {
		let api = &self.t.api;
		match ty {
			Type::Object(i) => out.push(*i),
			Type::Option(inner) | Type::Box(inner) | Type::Ref(_, inner) | Type::List(_, inner) => {
				self.handles(inner, out, seen)
			}
			Type::Tuple(elements) => elements.iter().for_each(|e| self.handles(e, out, seen)),
			Type::Record(i) if !seen.contains(ty) => {
				seen.push(ty.clone());
				api.records[*i]
					.fields
					.iter()
					.for_each(|f| self.handles(&f.ty, out, seen));
			}
			Type::Enum(i) if !seen.contains(ty) => {
				seen.push(ty.clone());
				(api.enums[*i].variants.iter().flat_map(|v| &v.fields)).for_each(|f| self.handles(&f.ty, out, seen));
			}
			_ => {}
		}
	}

	/// Whether a local of `ty` stays usable after Rust passes it by value.
	fn copy(&self, ty: &Type) -> bool {
		let api = &self.t.api;
		match ty {
			Type::Object(i) => api.objects[*i].copy,
			Type::Str | Type::String | Type::Slice | Type::Vec | Type::Bytes | Type::Path | Type::PathBuf => false,
			Type::List(..) | Type::Callback(..) | Type::Closure(_) => false,
			Type::Option(inner) | Type::Box(inner) => self.copy(inner),
			Type::Ref(..) => true,
			Type::Tuple(elements) => elements.iter().all(|e| self.copy(e)),
			Type::Record(i) => api.records[*i].fields.iter().all(|f| self.copy(&f.ty)),
			Type::Enum(i) => (api.enums[*i].variants.iter().flat_map(|v| &v.fields)).all(|f| self.copy(&f.ty)),
			_ => true,
		}
	}

	/// The C name of a record, enum, object, or callback.
	fn type_name(&self, ty: &Type) -> String {
		match ty {
			Type::Callback(i, _) => {
				let path = self.t.api.callbacks[*i].path.join("::");
				self.t.named(LANGUAGE, &format!("type {path}")).to_string()
			}
			ty => self.t.type_name(LANGUAGE, ty).to_string(),
		}
	}

	/// A type's name after the prefix, as the structs built from it are named.
	fn key(&self, ty: &Type) -> String {
		let strip = |name: String| name[self.p.len() + 1..].to_string();
		match ty {
			Type::Unit => "unit".to_string(),
			Type::Bool => "bool".to_string(),
			Type::Str | Type::String => "string".to_string(),
			Type::Slice | Type::Vec | Type::Bytes => "bytes".to_string(),
			Type::Path | Type::PathBuf => "path".to_string(),
			Type::Int(int) => int.rust().to_string(),
			Type::NonZero(int) => format!("non_zero_{}", int.rust()),
			Type::F32 => "f32".to_string(),
			Type::F64 => "f64".to_string(),
			Type::Char => "char".to_string(),
			Type::U128 => "u128".to_string(),
			Type::Duration => "duration".to_string(),
			Type::Array(n) => format!("array_u8_{n}"),
			Type::SocketAddr(family) => addr(true, *family).to_string(),
			Type::IpAddr(family) => addr(false, *family).to_string(),
			Type::Option(inner) => format!("option_{}", self.key(inner)),
			Type::List(_, inner) => format!("vec_{}", self.key(inner)),
			Type::Tuple(_) => strip(
				self.tuple_struct(ty)
					.unwrap_or_else(|| panic!("C has no struct of {ty:?}")),
			),
			Type::Record(_) | Type::Enum(_) | Type::Object(_) => strip(self.type_name(ty)),
			Type::Range(int) => format!("range_{}", int.rust()),
			Type::Bound(int) => format!("bound_{}", int.rust()),
			Type::Bounds(int) => format!("bounds_{}", int.rust()),
			Type::Ref(_, inner) | Type::Box(inner) => self.key(inner),
			other => panic!("no C struct holds {other:?}"),
		}
	}

	/// A tuple type as its struct's name is keyed: its elements' keys, a tuple's spelled.
	fn spell(&self, ty: &Type) -> String {
		match ty {
			Type::Tuple(elements) => {
				let elements: Vec<String> = elements.iter().map(|e| self.spell(e)).collect();
				match elements.len() {
					1 => format!("({},)", elements[0]),
					_ => format!("({})", elements.join(", ")),
				}
			}
			ty => self.key(ty),
		}
	}

	/// The C struct of tuple type `ty`, if the header has one.
	fn tuple_struct(&self, ty: &Type) -> Option<String> {
		let ty = owned(unref(ty));
		let Type::Tuple(_) = ty.base() else { return None };
		self.t
			.find(LANGUAGE, &format!("tuple {}", self.spell(ty.base())))
			.map(str::to_string)
	}

	/// The C type of a value of `ty`.
	fn cty(&self, ty: &Type) -> String {
		let p = &self.p;
		match ty {
			Type::Unit => "void".to_string(),
			Type::Bool => "bool".to_string(),
			Type::Int(int) | Type::NonZero(int) => c_int(*int).to_string(),
			Type::F32 => "float".to_string(),
			Type::F64 => "double".to_string(),
			Type::Char => "uint32_t".to_string(),
			Type::Str | Type::String | Type::Slice | Type::Vec | Type::Bytes | Type::Path | Type::PathBuf => {
				format!("{p}_buf")
			}
			Type::U128 => format!("{p}_u128"),
			Type::Duration => format!("{p}_duration"),
			Type::Array(n) => format!("{p}_array_u8_{n}"),
			Type::SocketAddr(family) => format!("{p}_{}", addr(true, *family)),
			Type::IpAddr(family) => format!("{p}_{}", addr(false, *family)),
			Type::List(_, inner) => format!("{p}_vec_{}", self.key(inner)),
			Type::Option(inner) if nullable(inner) => self.cty(inner),
			Type::Option(inner) => format!("{p}_option_{}", self.key(inner)),
			Type::Box(inner) => self.cty(inner),
			Type::Ref(m, inner) => match &**inner {
				Type::Object(_) if *m == Mutability::Not => format!("const {} *", self.type_name(inner)),
				inner => self.cty(inner),
			},
			Type::Record(_) | Type::Enum(_) | Type::Callback(..) => self.type_name(ty),
			Type::Object(_) => format!("{} *", self.type_name(ty)),
			Type::Range(_) | Type::Bound(_) | Type::Bounds(_) => format!("{p}_{}", self.key(ty)),
			Type::Tuple(_) => self
				.tuple_struct(ty)
				.unwrap_or_else(|| panic!("C has no struct of {ty:?}")),
			other => panic!("no C value is a {other:?}"),
		}
	}

	/// Whether a C value of `ty` is a struct, which `{0}` zeroes, rather than a scalar.
	fn is_struct(&self, ty: &Type) -> bool {
		match ty {
			Type::Unit | Type::Bool | Type::Int(_) | Type::NonZero(_) | Type::F32 | Type::F64 | Type::Char => false,
			Type::Object(_) => false,
			Type::Enum(i) => !self.t.api.enums[*i].variants.iter().all(|v| v.fields.is_empty()),
			Type::Option(inner) if nullable(inner) => self.is_struct(inner),
			Type::Ref(_, inner) | Type::Box(inner) => self.is_struct(inner),
			_ => true,
		}
	}

	fn zero(&self, ty: &Type) -> &'static str {
		if self.is_struct(ty) { "{0}" } else { "0" }
	}

	/// What frees `c`, an owned value of `ty`.
	fn free(&self, c: &str, ty: &Type) -> Vec<String> {
		let p = &self.p;
		match ty {
			Type::Str | Type::String | Type::Slice | Type::Vec | Type::Bytes | Type::Path | Type::PathBuf => {
				vec![format!("{p}_buf_free({c});")]
			}
			Type::List(_, inner) => vec![format!("{p}_vec_{}_free({c});", self.key(inner))],
			Type::Object(_) => vec![format!("{}_free({c});", self.type_name(ty))],
			Type::Ref(_, inner) | Type::Box(inner) => self.free(c, inner),
			Type::Option(inner) if nullable(inner) => self.free(c, inner),
			Type::Option(inner) if self.owns(inner) => {
				let inner = self.free(&format!("{c}.value"), inner).join(" ");
				vec![format!("if ({c}.some) {{ {inner} }}")]
			}
			Type::Record(_) | Type::Enum(_) if self.owns(ty) => vec![format!("{}_free(&{c});", self.type_name(ty))],
			Type::Tuple(elements) => (elements.iter().enumerate())
				.flat_map(|(i, e)| self.free(&format!("{c}._{i}"), e))
				.collect(),
			_ => Vec::new(),
		}
	}

	/// `c`, a place holding a value of `ty`, as a view that leaves Rust nothing to take.
	fn view(&self, c: &str, ty: &Type) -> String {
		if !self.viewed(ty) {
			return c.to_string();
		}
		let api = &self.t.api;
		match ty {
			Type::Str | Type::String | Type::Slice | Type::Vec | Type::Bytes | Type::Path | Type::PathBuf => {
				format!("tr_view({c})")
			}
			Type::Ref(_, inner) | Type::Box(inner) => self.view(c, inner),
			Type::Option(inner) if nullable(inner) => self.view(c, inner),
			Type::Option(inner) => format!(
				"({}){{.some = {c}.some, .value = {}}}",
				self.cty(ty),
				self.view(&format!("{c}.value"), inner)
			),
			Type::Record(i) => {
				let key = format!("type {}", self.t.rust(ty));
				let fields: Vec<String> = (api.records[*i].fields.iter())
					.map(|f| {
						let name = self.t.named(LANGUAGE, &format!("{key} / field {}", f.name));
						format!(".{name} = {}", self.view(&format!("{c}.{name}"), &f.ty))
					})
					.collect();
				format!("({}){{{}}}", self.cty(ty), fields.join(", "))
			}
			Type::Tuple(elements) => {
				let fields: Vec<String> = (elements.iter().enumerate())
					.map(|(i, e)| format!("._{i} = {}", self.view(&format!("{c}._{i}"), e)))
					.collect();
				format!("({}){{{}}}", self.cty(ty), fields.join(", "))
			}
			ty => panic!("a {ty:?} holding bytes passed by value"),
		}
	}

	/// Whether the header declares `name` taking a status last, checking it declares it.
	fn statused(&self, name: &str) -> bool {
		self.declared
			.get(name)
			.unwrap_or_else(|| panic!("the C header declares no {name}"))
			.1
	}

	/// The call of `name` with `args`, a status appended if it takes one.
	fn invoke(&self, name: &str, mut args: Vec<String>) -> String {
		if self.statused(name) {
			args.push("NULL".to_string());
		}
		let (count, _) = self.declared[name];
		assert_eq!(
			count,
			args.len(),
			"{name} takes {count} parameters, not ({})",
			args.join(", ")
		);
		format!("{name}({})", args.join(", "))
	}

	// Values.

	/// `v` where its address is taken: bound to a variable unless it is a place.
	fn place(&mut self, v: Val) -> Val {
		if v.lv && v.named {
			return v;
		}
		let name = self.fresh("p");
		self.line(&format!("{} {name} = {};", v.cty, v.c));
		Val {
			c: name,
			named: true,
			lv: true,
			..v
		}
	}

	/// A call's result, bound to a fresh variable, freed at the statement's end if owned.
	fn bound(&mut self, call: String, ty: &Type) -> Val {
		if *ty == Type::Unit {
			self.line(&format!("{call};"));
			return Val::unit();
		}
		let name = self.fresh("t");
		let cty = self.cty(ty);
		self.line(&format!("{cty} {name} = {call};"));
		self.temp(&name, ty);
		Val::place(name, ty.clone(), cty)
	}

	fn expr(&mut self, e: &Expr) -> Val {
		self.value(e, &e.ty)
	}

	/// The C argument of `e`, passed where `want` is taken.
	fn arg(&mut self, e: &Expr, want: &Type) -> String {
		let v = self.value(e, want);
		self.pass(v, want)
	}

	/// `v` as an argument where `want` is taken: a tuple of elements in its struct, an
	/// address of one family as any, an array as bytes, and what Rust would take as a view.
	fn pass(&mut self, v: Val, want: &Type) -> String {
		let w = unref(want).base().clone();
		let p = self.p.clone();
		match v.form {
			Form::Tuple(parts) => {
				let elements = match &w {
					Type::Tuple(elements) => elements.clone(),
					Type::Bounds(int) => vec![Type::Bound(*int), Type::Bound(*int)],
					w => panic!("a tuple passed as {w:?}"),
				};
				let fields: Vec<String> = match &w {
					Type::Bounds(_) => vec!["start".to_string(), "end".to_string()],
					_ => (0..parts.len()).map(|i| format!("_{i}")).collect(),
				};
				let values: Vec<String> = (parts.into_iter().zip(&elements).zip(fields))
					.map(|((part, ty), field)| format!(".{field} = {}", self.pass(part, ty)))
					.collect();
				return format!("({}){{{}}}", self.cty(&w), values.join(", "));
			}
			Form::Result { .. } => panic!("a Result passed as {w:?}"),
			Form::Plain => {}
		}
		let have = unref(&v.ty).base().clone();
		match (&have, &w) {
			(Type::SocketAddr(f), Type::SocketAddr(Family::Any)) if *f != Family::Any => {
				let six = *f == Family::V6;
				let member = if six { "v6" } else { "v4" };
				return format!("({p}_socket_addr){{.v6 = {six}, .addr.{member} = {}}}", v.c);
			}
			(Type::IpAddr(f), Type::IpAddr(Family::Any)) if *f != Family::Any => {
				let six = *f == Family::V6;
				let member = if six { "v6" } else { "v4" };
				return format!("({p}_ip_addr){{.v6 = {six}, .ip.{member} = {}}}", v.c);
			}
			(Type::Array(n), w) if is_buf(w) => {
				let v = self.place(v);
				return format!("({p}_buf){{{}.items, {n}, NULL}}", v.c);
			}
			_ => {}
		}
		let mut c = match v.named && self.viewed(&v.ty) {
			true => self.view(&v.c, &v.ty),
			false => v.c.clone(),
		};
		let wc = self.cty(want);
		if v.cty.starts_with("const ") && !wc.starts_with("const ") && wc.ends_with('*') {
			c = format!("({wc}){c}");
		}
		c
	}

	/// The C of `e`, built as `want` where it is a literal.
	fn value(&mut self, e: &Expr, want: &Type) -> Val {
		let t = self.t;
		let p = self.p.clone();
		let base = unref(want).base().clone();
		match &e.kind {
			Kind::Unit => Val::unit(),
			Kind::Bool(b) => Val::new(b.to_string(), Type::Bool, "bool"),
			Kind::Int(n) => self.int(*n, &base),
			Kind::Float(f) => match base {
				Type::F32 => Val::new(format!("tr_f32(0x{:x}u)", (*f as f32).to_bits()), Type::F32, "float"),
				Type::F64 => Val::new(format!("tr_f64(0x{:x}ULL)", f.to_bits()), Type::F64, "double"),
				ty => panic!("a float as {ty:?}"),
			},
			Kind::Char(c) => Val::new(format!("0x{:x}u", *c as u32), Type::Char, "uint32_t"),
			Kind::Str(s) => self.bytes(s.as_bytes(), want),
			Kind::Bytes(b) => match base {
				Type::Array(n) => {
					assert_eq!(n, b.len(), "an array literal of another length");
					let items: Vec<String> = b.iter().map(|b| b.to_string()).collect();
					let cty = format!("{p}_array_u8_{n}");
					Val {
						lv: true,
						..Val::new(
							format!("({cty}){{.items = {{{}}}}}", items.join(", ")),
							base.clone(),
							cty,
						)
					}
				}
				_ => self.bytes(b, want),
			},
			Kind::Duration(secs, nanos) => {
				let cty = format!("{p}_duration");
				Val::new(
					format!("({cty}){{.secs = {secs}ULL, .nanos = {nanos}u}}"),
					Type::Duration,
					cty,
				)
			}
			Kind::Socket(addr) => self.socket(addr, &base),
			Kind::Ip(ip) => self.ip(ip, &base),
			Kind::None => {
				let Type::Option(inner) = &base else {
					panic!("None as {base:?}")
				};
				let cty = self.cty(&base);
				match nullable(inner) && !self.is_struct(inner) {
					true => Val::new("NULL", base.clone(), cty),
					false => Val::new(format!("({cty}){{0}}"), base.clone(), cty),
				}
			}
			Kind::Some(inner) => {
				let Type::Option(iw) = &base else {
					panic!("Some as {base:?}")
				};
				let iw = (**iw).clone();
				if nullable(&iw) {
					let v = self.value(inner, &iw);
					let c = self.pass(v, &iw);
					return Val::new(c, base.clone(), self.cty(&base));
				}
				let v = self.value(inner, &iw);
				let c = self.pass(v, &iw);
				let cty = self.cty(&base);
				Val {
					lv: true,
					..Val::new(format!("({cty}){{.some = true, .value = {c}}}"), base.clone(), cty)
				}
			}
			Kind::Tuple(elements) => match &base {
				Type::Range(int) => {
					let int = Type::Int(*int);
					let (a, b) = (self.arg(&elements[0], &int), self.arg(&elements[1], &int));
					let cty = self.cty(&base);
					Val::new(format!("({cty}){{.start = {a}, .end = {b}}}"), base.clone(), cty)
				}
				Type::Bounds(int) => {
					let bound = Type::Bound(*int);
					let (a, b) = (self.arg(&elements[0], &bound), self.arg(&elements[1], &bound));
					let cty = self.cty(&base);
					Val::new(format!("({cty}){{.start = {a}, .end = {b}}}"), base.clone(), cty)
				}
				_ => {
					let wants: Vec<Type> = match &base {
						Type::Tuple(ws) => ws.clone(),
						_ => elements.iter().map(|e| e.ty.clone()).collect(),
					};
					let parts: Vec<Val> = elements.iter().zip(&wants).map(|(e, w)| self.value(e, w)).collect();
					let ty = Type::Tuple(wants);
					let cty = self.tuple_struct(&ty).unwrap_or_default();
					Val {
						form: Form::Tuple(parts),
						..Val::new("", ty, cty)
					}
				}
			},
			Kind::List(items) => {
				let Type::List(_, item) = &base else {
					panic!("a list as {base:?}")
				};
				let item = (**item).clone();
				let cty = self.cty(&base);
				let elem = self.cty(&owned(&item));
				let len = items.len();
				let ptr = match items.is_empty() {
					true => match &item.base() {
						Type::Object(_) => format!("({elem}const *)(const void *)tr_empty"),
						_ => format!("(const {elem} *)(const void *)tr_empty"),
					},
					false => {
						let values: Vec<String> = items.iter().map(|e| self.arg(e, &item)).collect();
						format!("({elem}[]){{{}}}", values.join(", "))
					}
				};
				Val {
					lv: true,
					..Val::new(format!("({cty}){{{ptr}, {len}, NULL}}"), base.clone(), cty)
				}
			}
			Kind::Record(r, fields) => {
				let record = &t.api.records[*r];
				assert!(
					record.opaque.is_none(),
					"a literal of {}, which only Rust makes",
					t.rust(&Type::Record(*r))
				);
				let ty = Type::Record(*r);
				let key = format!("type {}", t.rust(&ty));
				let values: Vec<String> = (fields.iter().zip(&record.fields))
					.map(|(v, f)| {
						let name = t.named(LANGUAGE, &format!("{key} / field {}", f.name)).to_string();
						format!(".{name} = {}", self.arg(v, &f.ty))
					})
					.collect();
				let cty = self.cty(&ty);
				Val {
					lv: true,
					..Val::new(format!("({cty}){{{}}}", values.join(", ")), ty, cty)
				}
			}
			Kind::Variant(en, v, fields) => {
				let ty = Type::Enum(*en);
				let enumeration = &t.api.enums[*en];
				let variant = &enumeration.variants[*v];
				let key = format!("type {}", t.rust(&ty));
				let constant = t
					.named(LANGUAGE, &format!("{key} / variant {}", variant.name))
					.to_string();
				let cty = self.cty(&ty);
				if enumeration.variants.iter().all(|v| v.fields.is_empty()) {
					return Val::new(constant, ty, cty);
				}
				if fields.is_empty() {
					return Val {
						lv: true,
						..Val::new(format!("({cty}){{.tag = {constant}}}"), ty, cty)
					};
				}
				let member = t
					.named(LANGUAGE, &format!("{key} / member {}", variant.name))
					.to_string();
				let values: Vec<String> = (fields.iter().zip(&variant.fields))
					.map(|(v, f)| {
						let name = t.named(
							LANGUAGE,
							&format!("{key} / variant {} / field {}", variant.name, f.name),
						);
						let name = name.to_string();
						format!(".{name} = {}", self.arg(v, &f.ty))
					})
					.collect();
				Val {
					lv: true,
					..Val::new(
						format!("({cty}){{.tag = {constant}, .{member} = {{{}}}}}", values.join(", ")),
						ty,
						cty,
					)
				}
			}
			Kind::Bound(kind, value) => {
				let Type::Bound(int) = base else {
					panic!("a bound as {base:?}")
				};
				let cty = self.cty(&base);
				let (kind, v) = match (kind, value) {
					(BoundKind::Unbounded, _) => ("UNBOUNDED", "0".to_string()),
					(BoundKind::Included, Some(v)) => ("INCLUDED", self.arg(v, &Type::Int(int))),
					(BoundKind::Excluded, Some(v)) => ("EXCLUDED", self.arg(v, &Type::Int(int))),
					_ => unreachable!(),
				};
				let upper = self.upper.clone();
				Val::new(
					format!("({cty}){{.kind = {upper}_BOUND_{kind}, .value = {v}}}"),
					base.clone(),
					cty,
				)
			}
			Kind::Default => {
				let ty = base.clone();
				let name = format!("{}_default", self.type_name(&ty));
				let call = self.invoke(&name, Vec::new());
				self.bound(call, &ty)
			}
			Kind::Var(name) => self.local(name),
			Kind::Const(c) => {
				let constant = &t.api.constants[*c];
				let name = t.named(LANGUAGE, &format!("const {}", constant.path.join("::")));
				let ty = match constant.value {
					super::ir::Literal::Bool(_) => Type::Bool,
					super::ir::Literal::Int(int, _) => Type::Int(int),
				};
				let cty = self.cty(&ty);
				Val::new(name, ty, cty)
			}
			Kind::Call(id, receiver, args) => self.call(*id, receiver.as_deref(), args),
			Kind::Field(record, i) => {
				let b = self.expr(record);
				let b = match b.named {
					true => b,
					false => self.place(b),
				};
				let Type::Record(r) = unref(&b.ty).base() else {
					panic!("a field of {:?}", b.ty)
				};
				let field = &t.api.records[*r].fields[*i];
				let key = format!("type {} / field {}", t.rust(&Type::Record(*r)), field.name);
				let name = t.named(LANGUAGE, &key);
				Val::place(format!("{}.{name}", b.c), field.ty.clone(), self.cty(&field.ty))
			}
			Kind::Element(tuple, i) => {
				let b = self.expr(tuple);
				match (&b.form, unref(&b.ty).base()) {
					(Form::Tuple(parts), _) => parts[*i].clone(),
					(_, Type::Range(int)) => {
						let field = if *i == 0 { "start" } else { "end" };
						Val::new(format!("{}.{field}", b.c), Type::Int(*int), c_int(*int))
					}
					(_, Type::Tuple(elements)) => {
						let ty = elements[*i].clone();
						let b = self.place(b);
						Val::place(format!("{}._{i}", b.c), ty.clone(), self.cty(&ty))
					}
					(_, ty) => panic!("an element of {ty:?}"),
				}
			}
			Kind::Clone(inner) => {
				let v = self.expr(inner);
				match unref(&v.ty).base() {
					Type::Object(_) => {
						let ty = owned(unref(&v.ty));
						let name = format!("{}_clone", self.type_name(&ty));
						let call = self.invoke(&name, vec![v.c.clone()]);
						self.bound(call, &ty)
					}
					// A value C holds is read where it is: its clone is the same view.
					_ => v,
				}
			}
			Kind::Unwrap(inner) => {
				let v = self.expr(inner);
				self.unwrap(v)
			}
			Kind::UnwrapErr(inner) => {
				let v = self.expr(inner);
				let Form::Result { flag, err, .. } = &v.form else {
					panic!("unwrap_err of {:?}", v.ty)
				};
				self.line(&format!("if ({flag}) tr_fail(\"unwrap_err of an Ok\");"));
				let err = (**err).clone();
				if self.take(flag) {
					self.temp(&err.c, &err.ty);
				}
				err
			}
			Kind::IsErr(inner) => {
				let v = self.expr(inner);
				let Form::Result { flag, .. } = &v.form else {
					panic!("is_err of {:?}", v.ty)
				};
				Val::new(format!("(!{flag})"), Type::Bool, "bool")
			}
			Kind::IsSome(inner) => {
				let v = self.expr(inner);
				let v = self.named(v);
				Val::new(format!("({})", self.is_some(&v)), Type::Bool, "bool")
			}
			Kind::IsNone(inner) => {
				let v = self.expr(inner);
				let v = self.named(v);
				Val::new(format!("(!({}))", self.is_some(&v)), Type::Bool, "bool")
			}
			Kind::Binary(op, a, b) => self.binary(*op, a, b, &e.ty),
			Kind::Closure(..) => panic!("a closure anywhere but an argument"),
			Kind::Local(name, args) => {
				let params = t.locals.get(name).cloned().unwrap_or_default();
				assert_eq!(params.len(), args.len(), "{name} takes {} arguments", params.len());
				let args: Vec<String> = args.iter().zip(&params).map(|(a, p)| self.arg(a, p)).collect();
				let call = format!("{name}({})", args.join(", "));
				self.bound(call, &e.ty)
			}
			Kind::Format(pieces) => {
				let text = self.text(pieces);
				let name = self.fresh("t");
				self.line(&format!("{p}_buf {name} = tr_done(&{text});"));
				self.temp(&name, &Type::String);
				Val::place(name, Type::String, format!("{p}_buf"))
			}
			Kind::Join { items, pat, body, sep } => self.join(items, pat, body, sep),
			Kind::MapOr {
				option,
				none,
				pat,
				some,
			} => self.map_or(option, none, pat, some, &e.ty),
			Kind::Utf8(inner) => {
				let v = self.expr(inner);
				Val { ty: Type::String, ..v }
			}
			Kind::Len(inner) => {
				let v = self.expr(inner);
				let v = self.named(v);
				Val::new(format!("{}.len", v.c), Type::Int(Int::Usize), "size_t")
			}
		}
	}

	/// `v`, bound to a variable unless it names one, so it may be read again.
	fn named(&mut self, v: Val) -> Val {
		match v.named || !matches!(v.form, Form::Plain) {
			true => v,
			false => self.place(v),
		}
	}

	fn int(&self, n: Num, ty: &Type) -> Val {
		let p = &self.p;
		match ty {
			Type::Int(int) | Type::NonZero(int) => {
				let cty = c_int(*int);
				let m = n.magnitude;
				let c = match (n.negative, int.signed()) {
					(true, _) if m == 1u128 << (int.bits() - 1) => format!("(({cty})(-{}LL - 1))", m - 1),
					(true, _) => format!("(({cty})-{m}LL)"),
					(false, true) => format!("(({cty}){m}LL)"),
					(false, false) => format!("(({cty}){m}ULL)"),
				};
				Val::new(c, ty.clone(), cty)
			}
			Type::U128 => {
				let m = n.magnitude;
				let cty = format!("{p}_u128");
				let c = format!("({cty}){{.lo = {}ULL, .hi = {}ULL}}", m as u64, (m >> 64) as u64);
				Val {
					lv: true,
					..Val::new(c, Type::U128, cty)
				}
			}
			Type::F32 => Val::new(
				format!("tr_f32(0x{:x}u)", (n.magnitude as f32).to_bits()),
				Type::F32,
				"float",
			),
			Type::F64 => Val::new(
				format!("tr_f64(0x{:x}ULL)", (n.magnitude as f64).to_bits()),
				Type::F64,
				"double",
			),
			ty => panic!("an integer as {ty:?}"),
		}
	}

	/// Bytes, or a string's, as a view of a literal.
	fn bytes(&self, b: &[u8], want: &Type) -> Val {
		let p = &self.p;
		let cty = format!("{p}_buf");
		let ty = match unref(want).base() {
			ty if is_buf(ty) => ty.clone(),
			ty => panic!("bytes as {ty:?}"),
		};
		Val {
			lv: true,
			..Val::new(
				format!("({cty}){{(const uint8_t *){}, {}, NULL}}", literal(b), b.len()),
				ty,
				cty,
			)
		}
	}

	fn octets(octets: &[u8]) -> String {
		let o: Vec<String> = octets.iter().map(|o| o.to_string()).collect();
		format!("{{{{{}}}}}", o.join(", "))
	}

	fn socket(&self, addr: &std::net::SocketAddr, want: &Type) -> Val {
		let Type::SocketAddr(family) = want else {
			panic!("a socket address as {want:?}")
		};
		let v4 =
			|a: &std::net::SocketAddrV4| format!("{{.ip = {}, .port = {}}}", Self::octets(&a.ip().octets()), a.port());
		let v6 = |a: &std::net::SocketAddrV6| {
			format!(
				"{{.ip = {}, .port = {}, .flowinfo = {}u, .scope_id = {}u}}",
				Self::octets(&a.ip().octets()),
				a.port(),
				a.flowinfo(),
				a.scope_id()
			)
		};
		let cty = self.cty(want);
		let c = match (family, addr) {
			(Family::Any, std::net::SocketAddr::V4(a)) => format!("({cty}){{.v6 = false, .addr.v4 = {}}}", v4(a)),
			(Family::Any, std::net::SocketAddr::V6(a)) => format!("({cty}){{.v6 = true, .addr.v6 = {}}}", v6(a)),
			(Family::V4, std::net::SocketAddr::V4(a)) => format!("({cty}){}", v4(a)),
			(Family::V6, std::net::SocketAddr::V6(a)) => format!("({cty}){}", v6(a)),
			(family, addr) => panic!("{addr} as a socket address of {family:?}"),
		};
		Val {
			lv: true,
			..Val::new(c, want.clone(), cty)
		}
	}

	fn ip(&self, ip: &std::net::IpAddr, want: &Type) -> Val {
		let Type::IpAddr(family) = want else {
			panic!("an IP address as {want:?}")
		};
		let cty = self.cty(want);
		let c = match (family, ip) {
			(Family::Any, std::net::IpAddr::V4(a)) => {
				format!("({cty}){{.v6 = false, .ip.v4 = {}}}", Self::octets(&a.octets()))
			}
			(Family::Any, std::net::IpAddr::V6(a)) => {
				format!("({cty}){{.v6 = true, .ip.v6 = {}}}", Self::octets(&a.octets()))
			}
			(Family::V4, std::net::IpAddr::V4(a)) => format!("({cty}){}", Self::octets(&a.octets())),
			(Family::V6, std::net::IpAddr::V6(a)) => format!("({cty}){}", Self::octets(&a.octets())),
			(family, ip) => panic!("{ip} as an IP address of {family:?}"),
		};
		Val {
			lv: true,
			..Val::new(c, want.clone(), cty)
		}
	}

	/// Whether `v`, an option, is `Some`.
	fn is_some(&self, v: &Val) -> String {
		let Type::Option(inner) = unref(&v.ty).base() else {
			panic!("an option of {:?}", v.ty)
		};
		match (nullable(inner), inner.base()) {
			(true, Type::Object(_)) => format!("{} != NULL", v.c),
			(true, _) => format!("{}.ptr != NULL", v.c),
			(false, _) => format!("{}.some", v.c),
		}
	}

	/// The value `v`, an option known to be `Some`, holds.
	fn inner(&self, v: &Val) -> Val {
		let Type::Option(inner) = unref(&v.ty).base() else {
			panic!("an option of {:?}", v.ty)
		};
		let inner = (**inner).clone();
		let cty = self.cty(&inner);
		match nullable(&inner) {
			true => Val {
				ty: inner,
				cty,
				..v.clone()
			},
			false => Val::place(format!("{}.value", v.c), inner, cty),
		}
	}

	fn unwrap(&mut self, v: Val) -> Val {
		match &v.form {
			Form::Result { flag, ok, .. } => {
				self.line(&format!("if (!{flag}) tr_fail(\"unwrap of an Err\");"));
				let ok = (**ok).clone();
				if self.take(flag) {
					match &ok.form {
						Form::Tuple(parts) => {
							for part in parts {
								self.temp(&part.c, &part.ty);
							}
						}
						_ => self.temp(&ok.c, &ok.ty),
					}
				}
				ok
			}
			_ => {
				let v = self.named(v);
				let some = self.is_some(&v);
				self.line(&format!("if (!({some})) tr_fail(\"unwrap of None\");"));
				let inner = self.inner(&v);
				if self.take(&v.c) {
					self.temp(&inner.c, &inner.ty);
				}
				inner
			}
		}
	}

	fn binary(&mut self, op: Op, a: &Expr, b: &Expr, ty: &Type) -> Val {
		if op == Op::And {
			let a = self.expr(a);
			let r = self.fresh("b");
			self.line(&format!("bool {r} = {};", a.c));
			self.open(&format!("if ({r})"));
			let b = self.expr(b);
			self.line(&format!("{r} = {};", b.c));
			self.close();
			return Val::new(r, Type::Bool, "bool");
		}
		let (a, b) = (self.expr(a), self.expr(b));
		let operand = unref(&a.ty).base().clone();
		let c = match op {
			Op::Eq | Op::Ne => {
				let equal = match &operand {
					Type::Record(_) => Some(true),
					Type::Enum(_) => Some(true),
					Type::Object(_) => Some(false),
					_ => None,
				};
				let eq = match equal {
					Some(by_address) => {
						let name = format!("{}_eq", self.type_name(&operand));
						let (a, b) = match by_address {
							true => (self.place(a), self.place(b)),
							false => (a, b),
						};
						let amp = if by_address { "&" } else { "" };
						let call = self.invoke(&name, vec![format!("{amp}{}", a.c), format!("{amp}{}", b.c)]);
						self.bound(call, &Type::Bool).c
					}
					None => format!("({} == {})", a.c, b.c),
				};
				match op {
					Op::Eq => eq,
					_ => format!("(!{eq})"),
				}
			}
			Op::Lt => format!("({} < {})", a.c, b.c),
			Op::Gt => format!("({} > {})", a.c, b.c),
			Op::Add | Op::Sub | Op::Mul | Op::Rem => {
				let symbol = match op {
					Op::Add => "+",
					Op::Sub => "-",
					Op::Mul => "*",
					_ => "%",
				};
				format!("(({})({} {symbol} {}))", self.cty(ty), a.c, b.c)
			}
			Op::And => unreachable!(),
		};
		let cty = self.cty(ty);
		Val::new(c, ty.clone(), cty)
	}

	/// A call of `id`: a function, an associated function, a method, or a field's getter or
	/// setter, its result bound to a variable, and each local it moves freed after it.
	fn call(&mut self, id: FnId, receiver: Option<&Expr>, args: &[Expr]) -> Val {
		let t = self.t;
		let f = t.function(id);
		assert!(
			!f.future,
			"{} makes a future, which only a function of api.c polls",
			t.key(id)
		);
		let name = t.name(LANGUAGE, id).to_string();
		let mut cargs = Vec::new();
		let mut moved = Vec::new();
		if let Some(r) = receiver {
			let owner = t.owner(id).expect("a method's type");
			let v = self.expr(r);
			let by_value = f.receiver == Some(Receiver::Owned);
			if by_value
				&& !self.copy(&owner)
				&& let Kind::Var(local) = &r.kind
			{
				moved.push(local.clone());
			}
			match owner {
				Type::Object(i) => {
					let constant = f.receiver == Some(Receiver::Ref) || (by_value && t.api.objects[i].copy);
					let c = match !constant && v.cty.starts_with("const ") {
						true => format!("({} *){}", self.type_name(&owner), v.c),
						false => v.c,
					};
					cargs.push(c);
				}
				_ => {
					if f.receiver == Some(Receiver::Mut) {
						assert!(
							v.named && v.lv,
							"{}: a value's &mut self on anything but a place",
							t.key(id)
						);
					}
					let v = self.place(v);
					cargs.push(format!("&{}", v.c));
				}
			}
		}
		let mut written = Vec::new();
		let mut rewritten = Vec::new();
		for (a, param) in args.iter().zip(&f.params) {
			match &param.ty {
				Type::Closure(closure) => {
					let (function, data) = self.closure(a, closure);
					cargs.push(function);
					cargs.push(data);
					if closure.kept {
						cargs.push("NULL".to_string());
					}
				}
				ty if written_back(ty) => {
					let Kind::Var(local) = &a.kind else {
						panic!("{}: a &mut of anything but a local", t.key(id))
					};
					let v = self.expr(a);
					assert!(matches!(v.form, Form::Plain), "a &mut of a tuple's elements");
					cargs.push(v.c.clone());
					written.push(format!("&{}", v.c));
					rewritten.push(local.clone());
				}
				ty => {
					cargs.push(self.arg(a, ty));
					if let Kind::Var(local) = &a.kind
						&& !matches!(ty, Type::Ref(..))
						&& !matches!(ty, Type::Option(inner) if matches!(**inner, Type::Ref(..)))
						&& !self.copy(&self.local(local).ty)
					{
						moved.push(local.clone());
					}
				}
			}
		}
		cargs.extend(written);
		let result = self.returned(&name, f.ok.clone(), f.err.clone(), cargs);
		for local in rewritten {
			self.rewritten(&local);
		}
		for local in moved {
			self.consume(&local);
		}
		result
	}

	/// The call of `name` with `args`, returning `ok`, or failing with `err`, as the C
	/// function does: a value returned, or written through outs.
	fn returned(&mut self, name: &str, ok: Type, err: Option<Type>, mut args: Vec<String>) -> Val {
		if err.is_none() && !matches!(ok, Type::Tuple(_)) {
			let ty = owned(&ok);
			let call = self.invoke(name, args);
			return self.bound(call, &ty);
		}
		let parts: Vec<Type> = match &ok {
			Type::Unit => Vec::new(),
			Type::Tuple(elements) => elements.iter().map(owned).collect(),
			ok => vec![owned(ok)],
		};
		let mut outs = Vec::new();
		for ty in &parts {
			let out = self.out(ty);
			args.push(format!("&{}", out.c));
			outs.push(out);
		}
		let ok_ty = owned(&ok);
		let ok_val = match &ok {
			Type::Unit => Val::unit(),
			Type::Tuple(_) => {
				let cty = self.tuple_struct(&ok_ty).unwrap_or_default();
				Val {
					form: Form::Tuple(outs.clone()),
					..Val::new("", ok_ty.clone(), cty)
				}
			}
			_ => outs[0].clone(),
		};
		let Some(err) = err else {
			let call = self.invoke(name, args);
			self.line(&format!("{call};"));
			for out in &outs {
				self.temp(&out.c, &out.ty);
			}
			return ok_val;
		};
		let err_ty = owned(&err);
		let err_val = self.out(&err_ty);
		args.push(format!("&{}", err_val.c));
		let flag = self.fresh("ok");
		let call = self.invoke(name, args);
		self.line(&format!("bool {flag} = {call};"));
		let ok_free: Vec<String> = outs.iter().flat_map(|o| self.free(&o.c, &o.ty)).collect();
		let err_free = self.free(&err_val.c, &err_ty);
		if !ok_free.is_empty() || !err_free.is_empty() {
			let code = format!(
				"if ({flag}) {{ {} }} else {{ {} }}",
				ok_free.join(" "),
				err_free.join(" ")
			);
			self.frames.last_mut().expect("a statement").push(Owned {
				key: flag.clone(),
				code: vec![code],
			});
		}
		Val {
			form: Form::Result {
				flag,
				ok: Box::new(ok_val),
				err: Box::new(err_val),
			},
			..Val::new("", Type::Result(Box::new(ok_ty), Box::new(err_ty)), "")
		}
	}

	/// A zeroed variable a call writes a value of `ty` through.
	fn out(&mut self, ty: &Type) -> Val {
		let name = self.fresh("o");
		let cty = self.cty(ty);
		let zero = self.zero(ty);
		self.line(&format!("{cty} {name} = {zero};"));
		Val::place(name, ty.clone(), cty)
	}

	/// The function a closure is, and the data passed with it: each local it captures is
	/// read through a static of its own, so a closure Rust keeps past the call reads it too.
	fn closure(&mut self, e: &Expr, closure: &Closure) -> (String, String) {
		let Kind::Closure(params, steps, value) = &e.kind else {
			panic!("a closure's parameter passed anything but a closure")
		};
		assert!(
			closure.err.is_none(),
			"a closure returning a Result, which only api.c writes"
		);
		let n = self.fresh("closure");
		let mut used = Vec::new();
		for step in steps {
			each_expr(step, &mut |e| walk(e, &mut |e| vars(e, &mut used)));
		}
		walk(value, &mut |e| vars(e, &mut used));
		let captured: Vec<(String, Val)> = (used.iter())
			.filter_map(|name| self.lookup(name).map(|v| (name.clone(), v.clone())))
			.collect();
		let env = format!("{n}_env");
		if !captured.is_empty() {
			let fields: Vec<String> = (captured.iter())
				.map(|(name, v)| {
					assert!(
						matches!(v.form, Form::Plain) && v.lv,
						"a closure capturing {name}, which is no place"
					);
					format!("\t{} *v_{name};\n", v.cty)
				})
				.collect();
			let _ = writeln!(self.defs, "static struct {{\n{}}} {env};\n", fields.concat());
			for (name, v) in &captured {
				self.line(&format!("{env}.v_{name} = &{};", v.c));
			}
		}
		let ret = self.cty(&closure.ok);
		// The closure's body, written apart.
		let out = std::mem::take(&mut self.out);
		let depth = std::mem::replace(&mut self.depth, 1);
		let frames = std::mem::take(&mut self.frames);
		let mut scope = Scope::default();
		for (name, v) in &captured {
			scope.locals.push((
				name.clone(),
				Val::place(format!("(*{env}.v_{name})"), v.ty.clone(), v.cty.clone()),
			));
		}
		let scopes = std::mem::replace(&mut self.scopes, vec![scope]);
		let mut decls = vec!["void *data".to_string()];
		let mut tuples = Vec::new();
		for (pat, ty) in params.iter().zip(&closure.params) {
			let cty = self.cty(ty);
			let name = match pat {
				Pat::Name(name, _) => {
					let c = format!("v_{name}");
					self.scopes[0]
						.locals
						.push((name.clone(), Val::place(c.clone(), ty.clone(), cty.clone())));
					c
				}
				Pat::Ignore(_) => self.fresh("unused"),
				Pat::Tuple(_) => {
					let c = self.fresh("arg");
					tuples.push((pat.clone(), Val::place(c.clone(), ty.clone(), cty.clone())));
					c
				}
			};
			decls.push(format!("{cty} {name}"));
		}
		for (pat, v) in tuples {
			self.begin();
			self.bind(&pat, v);
			self.end();
		}
		self.steps(steps);
		if !matches!(value.kind, Kind::Unit) {
			self.begin();
			let v = self.value(value, &closure.ok);
			let c = match is_buf(unref(&closure.ok).base()) {
				// A buffer it returns is Rust's, one buf_copy made.
				true if self.take(&v.c) => v.c.clone(),
				true => format!("{}_buf_copy(tr_view({}))", self.p, v.c),
				false => {
					assert!(!self.owns(&closure.ok), "a closure returning a {:?}", closure.ok);
					self.pass(v, &closure.ok)
				}
			};
			self.line(&format!("{ret} tr_ret = {c};"));
			self.end();
			self.drop_scope();
			self.line("return tr_ret;");
		} else {
			self.drop_scope();
		}
		let body = std::mem::replace(&mut self.out, out);
		self.depth = depth;
		self.frames = frames;
		self.scopes = scopes;
		let _ = writeln!(self.defs, "static {ret} {n}({}) {{\n{body}}}\n", decls.join(", "));
		(n, "NULL".to_string())
	}

	/// The text a format's pieces make: each value is read first, as Rust's macros do, then
	/// each is written.
	fn text(&mut self, pieces: &[Piece]) -> String {
		let values: Vec<Option<(Val, Spec)>> = (pieces.iter())
			.map(|piece| match piece {
				Piece::Text(_) => None,
				Piece::Value(e, spec) => Some((self.expr(e), *spec)),
			})
			.collect();
		let text = self.fresh("text");
		self.line(&format!("tr_text {text} = {{0}};"));
		for (piece, value) in pieces.iter().zip(values) {
			match (piece, value) {
				(Piece::Text(s), _) => self.line(&format!("tr_put(&{text}, {}, {});", literal(s.as_bytes()), s.len())),
				(_, Some((v, spec))) => self.show(&text, v, spec),
				_ => unreachable!(),
			}
		}
		text
	}

	/// Append `v` to `text` as Rust formats it with `spec`.
	fn show(&mut self, text: &str, v: Val, spec: Spec) {
		let ty = unref(&v.ty).base().clone();
		match (spec, &ty) {
			(Spec::Hex(width), Type::Int(_) | Type::NonZero(_)) => self.line(&format!(
				"tr_fmt(&{text}, \"%0*llx\", {width}, (unsigned long long){});",
				v.c
			)),
			(Spec::Hex(width), Type::U128) => self.line(&format!("tr_hex128(&{text}, {}, {width});", v.c)),
			(Spec::Padded(width), Type::Int(int) | Type::NonZero(int)) => {
				let (format, cast) = number(*int);
				let format = format.replace('%', &format!("%0{width}"));
				self.line(&format!("tr_fmt(&{text}, \"{format}\", ({cast}){});", v.c))
			}
			(Spec::Display, _) => self.display(text, v),
			(Spec::Debug, _) => self.debug(text, v),
			(spec, ty) => panic!("{spec:?} of {ty:?}"),
		}
	}

	fn display(&mut self, text: &str, v: Val) {
		let ty = unref(&v.ty).base().clone();
		match &ty {
			Type::Bool => self.line(&format!("tr_str(&{text}, {} ? \"true\" : \"false\");", v.c)),
			Type::Int(int) | Type::NonZero(int) => {
				let (format, cast) = number(*int);
				self.line(&format!("tr_fmt(&{text}, \"{format}\", ({cast}){});", v.c))
			}
			Type::Char => self.line(&format!("tr_char(&{text}, {});", v.c)),
			ty if is_buf(ty) => self.line(&format!("tr_buf(&{text}, {});", v.c)),
			Type::Record(_) | Type::Enum(_) | Type::Object(_) => {
				let s = match self.text_of(&ty) {
					Some(id) => {
						let f = self.t.function(id);
						let name = self.t.name(LANGUAGE, id).to_string();
						let (ok, err) = (f.ok.clone(), f.err.clone());
						self.returned(&name, ok, err, vec![v.c.clone()])
					}
					None => self.traited(&v, "to_string", &Type::String),
				};
				self.line(&format!("tr_buf(&{text}, {});", s.c))
			}
			ty => panic!("Display of {ty:?}"),
		}
	}

	/// The method an object's text is, which a binding names its Display.
	fn text_of(&self, ty: &Type) -> Option<FnId> {
		let Type::Object(i) = ty else { return None };
		let index = self.t.api.objects[*i]
			.methods
			.iter()
			.position(|m| m.access == Access::Text)?;
		Some(FnId {
			owner: super::Owner::Object(*i),
			index,
		})
	}

	/// The call of a std trait's function the header declares for `v`'s type, by `suffix`.
	fn traited(&mut self, v: &Val, suffix: &str, ret: &Type) -> Val {
		let ty = unref(&v.ty).base().clone();
		let name = format!("{}_{suffix}", self.type_name(&ty));
		let arg = match ty {
			Type::Object(_) => v.c.clone(),
			_ => {
				let v = self.place(v.clone());
				format!("&{}", v.c)
			}
		};
		let call = self.invoke(&name, vec![arg]);
		self.bound(call, ret)
	}

	fn debug(&mut self, text: &str, v: Val) {
		if let Form::Tuple(parts) = &v.form {
			return self.debug_tuple(text, parts.clone());
		}
		let ty = unref(&v.ty).base().clone();
		match &ty {
			Type::Bool | Type::Int(_) | Type::NonZero(_) => self.display(text, v),
			Type::String | Type::Str | Type::Path | Type::PathBuf => {
				self.line(&format!("tr_quoted(&{text}, {});", v.c))
			}
			Type::Option(_) => {
				let v = self.named(v);
				let some = self.is_some(&v);
				let inner = self.inner(&v);
				self.open(&format!("if ({some})"));
				self.line(&format!("tr_str(&{text}, \"Some(\");"));
				self.debug(text, inner);
				self.line(&format!("tr_str(&{text}, \")\");"));
				self.close();
				self.open("else");
				self.line(&format!("tr_str(&{text}, \"None\");"));
				self.close();
			}
			Type::Vec | Type::Slice | Type::Bytes | Type::Array(_) => {
				let v = self.named(v);
				let (items, len) = match ty {
					Type::Array(n) => (format!("{}.items", v.c), n.to_string()),
					_ => (format!("{}.ptr", v.c), format!("{}.len", v.c)),
				};
				let i = self.fresh("i");
				self.line(&format!("tr_str(&{text}, \"[\");"));
				self.open(&format!("for (size_t {i} = 0; {i} < {len}; {i}++)"));
				self.line(&format!(
					"tr_fmt(&{text}, {i} ? \", %u\" : \"%u\", (unsigned){items}[{i}]);"
				));
				self.close();
				self.line(&format!("tr_str(&{text}, \"]\");"));
			}
			Type::List(_, item) => {
				let v = self.named(v);
				let i = self.fresh("i");
				self.line(&format!("tr_str(&{text}, \"[\");"));
				self.open(&format!("for (size_t {i} = 0; {i} < {}.len; {i}++)", v.c));
				self.line(&format!("if ({i}) tr_str(&{text}, \", \");"));
				let item = owned(item);
				let cty = self.cty(&item);
				self.debug(text, Val::place(format!("{}.ptr[{i}]", v.c), item, cty));
				self.close();
				self.line(&format!("tr_str(&{text}, \"]\");"));
			}
			Type::Tuple(elements) => {
				let v = self.place(v);
				let parts: Vec<Val> = (elements.iter().enumerate())
					.map(|(i, e)| Val::place(format!("{}._{i}", v.c), e.clone(), self.cty(e)))
					.collect();
				self.debug_tuple(text, parts);
			}
			Type::Bound(_) => {
				let v = self.named(v);
				let upper = self.upper.clone();
				for (kind, name) in [("INCLUDED", "Included"), ("EXCLUDED", "Excluded")] {
					self.line(&format!(
						"if ({}.kind == {upper}_BOUND_{kind}) tr_fmt(&{text}, \"{name}(%llu)\", (unsigned long long){}.value);",
						v.c, v.c
					));
				}
				self.line(&format!(
					"if ({}.kind == {upper}_BOUND_UNBOUNDED) tr_str(&{text}, \"Unbounded\");",
					v.c
				));
			}
			Type::Record(_) | Type::Enum(_) | Type::Object(_) => {
				let s = self.traited(&v, "debug", &Type::String);
				self.line(&format!("tr_buf(&{text}, {});", s.c));
			}
			ty => panic!("Debug of {ty:?}"),
		}
	}

	fn debug_tuple(&mut self, text: &str, parts: Vec<Val>) {
		self.line(&format!("tr_str(&{text}, \"(\");"));
		let n = parts.len();
		for (i, part) in parts.into_iter().enumerate() {
			if i > 0 {
				self.line(&format!("tr_str(&{text}, \", \");"));
			}
			self.debug(text, part);
		}
		let tail = if n == 1 { ",)" } else { ")" };
		self.line(&format!("tr_str(&{text}, \"{tail}\");"));
	}

	fn join(&mut self, items: &Expr, pat: &Pat, body: &Expr, sep: &str) -> Val {
		let list = self.expr(items);
		let list = self.named(list);
		let Type::List(_, item) = unref(&list.ty).base() else {
			panic!("a join over {:?}", list.ty)
		};
		let item = owned(item);
		let text = self.fresh("text");
		self.line(&format!("tr_text {text} = {{0}};"));
		let i = self.fresh("i");
		self.open(&format!("for (size_t {i} = 0; {i} < {}.len; {i}++)", list.c));
		self.line(&format!(
			"if ({i}) tr_put(&{text}, {}, {});",
			literal(sep.as_bytes()),
			sep.len()
		));
		let cty = self.cty(&item);
		self.bind(pat, Val::place(format!("{}.ptr[{i}]", list.c), item, cty));
		let b = self.expr(body);
		self.line(&format!("tr_buf(&{text}, {});", b.c));
		self.close();
		let name = self.fresh("t");
		let p = &self.p;
		let cty = format!("{p}_buf");
		self.line(&format!("{cty} {name} = tr_done(&{text});"));
		self.temp(&name, &Type::String);
		Val::place(name, Type::String, cty)
	}

	fn map_or(&mut self, option: &Expr, none: &Expr, pat: &Pat, some: &Expr, ty: &Type) -> Val {
		let opt = self.expr(option);
		let opt = self.named(opt);
		let name = self.fresh("t");
		let cty = self.cty(ty);
		let zero = self.zero(ty);
		self.line(&format!("{cty} {name} = {zero};"));
		let is_some = self.is_some(&opt);
		let inner = self.inner(&opt);
		self.open(&format!("if ({is_some})"));
		self.bind(pat, inner);
		let v = self.expr(some);
		let c = self.keep(v);
		self.line(&format!("{name} = {c};"));
		self.close();
		self.open("else");
		let v = self.expr(none);
		let c = self.keep(v);
		self.line(&format!("{name} = {c};"));
		self.close();
		self.temp(&name, ty);
		Val::place(name, ty.clone(), cty)
	}

	/// `v` as a value that outlives this statement: a temporary taken from it, or a copy.
	fn keep(&mut self, v: Val) -> String {
		if self.take(&v.c) {
			return v.c;
		}
		match unref(&v.ty).base() {
			ty if is_buf(ty) => format!("{}_buf_copy(tr_view({}))", self.p, v.c),
			ty if !self.owns(ty) => v.c,
			ty => panic!("a {ty:?} kept past its statement"),
		}
	}

	// Patterns and steps.

	fn bind(&mut self, pat: &Pat, v: Val) {
		match pat {
			Pat::Ignore(_) => {}
			Pat::Name(name, _) => self.bind_name(name, v),
			Pat::Tuple(pats) => {
				let parts = self.parts(v);
				assert_eq!(parts.len(), pats.len(), "a tuple pattern of another length");
				for (pat, part) in pats.iter().zip(parts) {
					self.bind(pat, part);
				}
			}
		}
	}

	/// The elements of `v`, a tuple or a pair of bounds, each owned if `v` was.
	fn parts(&mut self, v: Val) -> Vec<Val> {
		if let Form::Tuple(parts) = v.form {
			return parts;
		}
		let v = self.named(v);
		let owned = self.take(&v.c);
		let parts: Vec<Val> = match unref(&v.ty).base() {
			Type::Bounds(int) => ["start", "end"]
				.iter()
				.map(|f| Val::place(format!("{}.{f}", v.c), Type::Bound(*int), self.cty(&Type::Bound(*int))))
				.collect(),
			Type::Tuple(elements) => (elements.iter().enumerate())
				.map(|(i, e)| Val::place(format!("{}._{i}", v.c), e.clone(), self.cty(e)))
				.collect(),
			ty => panic!("the elements of {ty:?}"),
		};
		if owned {
			for part in &parts {
				self.temp(&part.c, &part.ty);
			}
		}
		parts
	}

	fn bind_name(&mut self, name: &str, v: Val) {
		let c = format!("v_{name}");
		let local = match &v.form {
			Form::Result { .. } => panic!("{name} bound to a Result"),
			Form::Tuple(parts) => match self.tuple_struct(&v.ty) {
				Some(tn) => {
					let mut owned = false;
					for part in parts {
						owned |= self.take(&part.c);
					}
					let fields: Vec<String> = (parts.iter().enumerate())
						.map(|(i, p)| format!("._{i} = {}", p.c))
						.collect();
					self.line(&format!("{tn} {c} = ({tn}){{{}}};", fields.join(", ")));
					let local = Val::place(c.clone(), v.ty.clone(), tn);
					if owned {
						self.own(&c, &v.ty);
					}
					local
				}
				None => {
					let mut locals = Vec::new();
					for (i, part) in parts.iter().enumerate() {
						let pc = format!("{c}_{i}");
						self.line(&format!("{} {pc} = {};", part.cty, part.c));
						if self.take(&part.c) {
							self.own(&pc, &part.ty);
						}
						locals.push(Val::place(pc, part.ty.clone(), part.cty.clone()));
					}
					Val {
						form: Form::Tuple(locals),
						..Val::new("", v.ty.clone(), "")
					}
				}
			},
			Form::Plain => {
				self.line(&format!("{} {c} = {};", v.cty, v.c));
				if self.take(&v.c) {
					self.own(&c, &v.ty);
				} else if self.take_owned(&v.c).is_some() {
					// A local moved to another: the new one owns it.
					let code = self.free(&c, &v.ty);
					self.scopes
						.last_mut()
						.unwrap()
						.owned
						.push(Owned { key: c.clone(), code });
				} else if self.owns(&v.ty) {
					// A value C built of temporaries, which live as long as it does.
					let frame = std::mem::take(self.frames.last_mut().expect("a statement"));
					self.scopes.last_mut().unwrap().owned.extend(frame);
				}
				Val::place(c, v.ty.clone(), v.cty.clone())
			}
		};
		self.scopes.last_mut().unwrap().locals.push((name.to_string(), local));
	}

	/// The local `c` of `ty` owned by the innermost scope.
	fn own(&mut self, c: &str, ty: &Type) {
		let code = self.free(c, ty);
		if !code.is_empty() {
			self.scopes.last_mut().unwrap().owned.push(Owned {
				key: c.to_string(),
				code,
			});
		}
	}

	fn steps(&mut self, steps: &[Step]) {
		for step in steps {
			self.step(step);
		}
	}

	fn step(&mut self, step: &Step) {
		match step {
			Step::Let(pat, value) => {
				self.begin();
				let v = self.expr(value);
				self.bind(pat, v);
				self.end();
			}
			Step::Print(pieces) => {
				self.begin();
				let text = self.text(pieces);
				self.line(&format!("tr_line(&{text});"));
				self.end();
			}
			Step::Do(value) => {
				self.begin();
				let v = self.expr(value);
				if matches!(v.form, Form::Plain) && !v.c.is_empty() && !v.named {
					self.line(&format!("(void)({});", v.c));
				}
				self.end();
			}
			Step::Drop(name) => {
				let v = self.local(name);
				let keys: Vec<String> = match &v.form {
					Form::Tuple(parts) => parts.iter().map(|p| p.c.clone()).collect(),
					_ => vec![v.c.clone()],
				};
				for key in keys {
					if let Some(owned) = self.take_owned(&key) {
						for code in owned.code {
							self.line(&code);
						}
					}
				}
			}
			Step::Match { value, ok, err } => {
				self.begin();
				let v = self.expr(value);
				let (cond, ok_val, err_val) = match &v.form {
					Form::Result { flag, ok, err } => (flag.clone(), Some((**ok).clone()), Some((**err).clone())),
					_ => {
						let v = self.named(v);
						(self.is_some(&v), Some(self.inner(&v)), None)
					}
				};
				self.arm(&format!("if ({cond})"), ok, ok_val);
				self.arm("else", err, err_val);
				self.end();
			}
		}
	}

	fn arm(&mut self, head: &str, arm: &Arm, value: Option<Val>) {
		self.open(head);
		if let (Some(pat), Some(v)) = (&arm.pat, value) {
			self.begin();
			self.bind(pat, v);
			self.end();
		}
		self.steps(&arm.steps);
		self.close();
	}
}

/// Each local `e` reads.
fn vars(e: &Expr, out: &mut Vec<String>) {
	if let Kind::Var(name) = &e.kind
		&& !out.contains(name)
	{
		out.push(name.clone());
	}
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

/// `ty` as Rust returns it to C: each borrow a clone, as the C ABI returns one.
fn owned(ty: &Type) -> Type {
	match ty {
		Type::Ref(_, inner) => owned(inner),
		Type::Option(inner) => Type::Option(Box::new(owned(inner))),
		Type::Box(inner) => Type::Box(Box::new(owned(inner))),
		Type::List(c, inner) => Type::List(*c, Box::new(owned(inner))),
		Type::Tuple(elements) => Type::Tuple(elements.iter().map(owned).collect()),
		ty => ty.clone(),
	}
}

/// Whether `ty` crosses as a buffer.
fn is_buf(ty: &Type) -> bool {
	matches!(
		ty.base(),
		Type::Str | Type::String | Type::Slice | Type::Vec | Type::Bytes | Type::Path | Type::PathBuf
	)
}

/// Whether `Option<ty>` crosses as `ty`, a null pointer its `None`.
fn nullable(ty: &Type) -> bool {
	matches!(ty.base(), Type::Object(_) | Type::List(..)) || is_buf(ty)
}

/// Whether Rust writes a parameter of `ty` back through an out, as a `&mut` of a value.
fn written_back(ty: &Type) -> bool {
	matches!(ty, Type::Ref(Mutability::Mut, inner) if !matches!(**inner, Type::Object(_) | Type::Reader))
}

fn addr(socket: bool, family: Family) -> &'static str {
	match (socket, family) {
		(true, Family::Any) => "socket_addr",
		(true, Family::V4) => "socket_addr_v4",
		(true, Family::V6) => "socket_addr_v6",
		(false, Family::Any) => "ip_addr",
		(false, Family::V4) => "ipv4_addr",
		(false, Family::V6) => "ipv6_addr",
	}
}

fn c_int(int: Int) -> &'static str {
	match int {
		Int::U8 => "uint8_t",
		Int::U16 => "uint16_t",
		Int::U32 => "uint32_t",
		Int::U64 => "uint64_t",
		Int::Usize => "size_t",
		Int::I8 => "int8_t",
		Int::I16 => "int16_t",
		Int::I32 => "int32_t",
		Int::I64 => "int64_t",
		Int::Isize => "ptrdiff_t",
	}
}

/// The `printf` format of an integer, and the type it is cast to for it.
fn number(int: Int) -> (&'static str, &'static str) {
	match int.signed() {
		true => ("%lld", "long long"),
		false => ("%llu", "unsigned long long"),
	}
}

/// A C string literal of `bytes`, each byte past printable ASCII in octal, which takes at
/// most three digits, so no digit after one joins it.
fn literal(bytes: &[u8]) -> String {
	let mut out = String::from("\"");
	for &b in bytes {
		match b {
			b'"' | b'\\' => {
				out.push('\\');
				out.push(b as char);
			}
			// `??` begins a trigraph.
			b'?' => out.push_str("\\?"),
			0x20..0x7f => out.push(b as char),
			_ => {
				let _ = write!(out, "\\{b:03o}");
			}
		}
	}
	out.push('"');
	out
}
