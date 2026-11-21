//! The transcript as Go: `transcript.go`, a file of `tests/api.go`'s package `main`, whose
//! `script()` runs it through the generated Go, calling `api.go`'s own functions.
//!
//! Each call is bound to a local before the statement that reads it, so the calls run in
//! Rust's order whatever Go's evaluation order, and an error Rust's call cannot return
//! panics there. A handle a statement makes and only lends, as a receiver or a reference,
//! closes at the statement's end, where Rust drops the temporary.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write;

use super::ir::{Access, Family, Int, Mutability, Receiver, Type};
use super::{Arm, BoundKind, Expr, FnId, Kind, Op, Pat, Piece, Spec, Step, Transcript, each_expr, unref, walk};

const LANGUAGE: &str = "Go";

/// The module path the export's Go packages are under, as `tests/fixtures.rs` lays it out.
const MODULE: &str = "api";

/// Helpers the script calls, each for a value Go shapes apart from Rust, or a check Rust
/// makes that Go does not.
const PRELUDE: &str = r#"
// must_ is v, or a panic on err.
func must_[T any](v T, err error) T {
	if err != nil {
		panic(err)
	}
	return v
}

// done_ panics on err.
func done_(err error) {
	if err != nil {
		panic(err)
	}
}

// ptr_ is a pointer to a copy of v, as an `Option` holds `Some(v)`.
func ptr_[T any](v T) *T {
	return &v
}

// some_ is what v points to, as `unwrap` of an `Option`.
func some_[T any](v *T) T {
	if v == nil {
		panic("called `Option::unwrap()` on a `None` value")
	}
	return *v
}

// present_ is the handle v, as `unwrap` of an `Option` of one.
func present_[T any](v *T) *T {
	if v == nil {
		panic("called `Option::unwrap()` on a `None` value")
	}
	return v
}

// option_ is v as Rust's Debug prints an `Option`, its value by show.
func option_[T any](v *T, show func(T) string) string {
	if v == nil {
		return "None"
	}
	return "Some(" + show(*v) + ")"
}

// optionHandle_ is v as Rust's Debug prints an `Option` of a handle, by show.
func optionHandle_[T any](v *T, show func(*T) string) string {
	if v == nil {
		return "None"
	}
	return "Some(" + show(v) + ")"
}

// list_ is items as Rust's Debug prints a slice, each by show.
func list_[T any](items []T, show func(T) string) string {
	shown := make([]string, len(items))
	for i, item := range items {
		shown[i] = show(item)
	}
	return "[" + strings.Join(shown, ", ") + "]"
}

// debugBytes_ is b as Rust's Debug prints bytes.
func debugBytes_(b []byte) string {
	return list_(b, func(b byte) string { return fmt.Sprint(b) })
}

// debugString_ is s as Rust's Debug prints a string.
func debugString_(s string) string {
	var out strings.Builder
	out.WriteByte('"')
	for _, r := range s {
		switch {
		case r == '"' || r == '\\':
			out.WriteByte('\\')
			out.WriteRune(r)
		case r == '\n':
			out.WriteString(`\n`)
		case r == '\r':
			out.WriteString(`\r`)
		case r == '\t':
			out.WriteString(`\t`)
		case r == 0:
			out.WriteString(`\0`)
		case !unicode.IsPrint(r) && r != ' ':
			fmt.Fprintf(&out, `\u{%x}`, r)
		default:
			out.WriteRune(r)
		}
	}
	out.WriteByte('"')
	return out.String()
}

// bound_ is a bound as Rust's Debug prints it.
func bound_[T any](kind rt.Bound, value T) string {
	switch kind {
	case rt.BoundIncluded:
		return fmt.Sprintf("Included(%v)", value)
	case rt.BoundExcluded:
		return fmt.Sprintf("Excluded(%v)", value)
	}
	return "Unbounded"
}

// errOf_ is err as the error type T Rust's call returns, or a panic on any other.
func errOf_[T error](err error) T {
	var t T
	if !errors.As(err, &t) {
		panic(fmt.Sprintf("an error other than %T: %v", t, err))
	}
	return t
}

// isErr_ is whether err is one of T, as `is_err`, panicking on any other error.
func isErr_[T error](err error) bool {
	if err == nil {
		return false
	}
	errOf_[T](err)
	return true
}

// ignore_ drops an error of T, as `let _ =` drops a `Result`, panicking on any other.
func ignore_[T error](err error) {
	if err != nil {
		errOf_[T](err)
	}
}

// lossy_ is b as text, as `String::from_utf8_lossy`: each longest prefix of a valid
// sequence that does not complete one is a replacement character.
func lossy_(b []byte) string {
	var out strings.Builder
	for len(b) > 0 {
		r, n := utf8.DecodeRune(b)
		if r != utf8.RuneError || n > 1 {
			out.Write(b[:n])
			b = b[n:]
			continue
		}
		need := 0
		switch {
		case b[0] >= 0xc2 && b[0] <= 0xdf:
			need = 1
		case b[0] >= 0xe0 && b[0] <= 0xef:
			need = 2
		case b[0] >= 0xf0 && b[0] <= 0xf4:
			need = 3
		}
		lo, hi := byte(0x80), byte(0xbf)
		switch b[0] {
		case 0xe0:
			lo = 0xa0
		case 0xed:
			hi = 0x9f
		case 0xf0:
			lo = 0x90
		case 0xf4:
			hi = 0x8f
		}
		n = 1
		for n <= need && n < len(b) && b[n] >= lo && b[n] <= hi {
			n++
			lo, hi = 0x80, 0xbf
		}
		out.WriteRune(utf8.RuneError)
		b = b[n:]
	}
	return out.String()
}

// hex128_ is v in lowercase hex, zero-padded to width digits.
func hex128_(v rt.U128, width int) string {
	s := v.Big().Text(16)
	return strings.Repeat("0", max(0, width-len(s))) + s
}

// integer_ is any of Go's integers, each of a Rust integer's.
type integer_ interface {
	~int | ~int8 | ~int16 | ~int32 | ~int64 | ~uint | ~uint8 | ~uint16 | ~uint32 | ~uint64
}

// add_ is a + b, panicking on overflow as Rust's checked build does.
func add_[T integer_](a, b T) T {
	c := a + b
	if (b > 0 && c < a) || (b < 0 && c > a) {
		panic("attempt to add with overflow")
	}
	return c
}

// sub_ is a - b, panicking on overflow as Rust's checked build does.
func sub_[T integer_](a, b T) T {
	c := a - b
	if (b > 0 && c > a) || (b < 0 && c < a) {
		panic("attempt to subtract with overflow")
	}
	return c
}

// least_ is whether a is its signed type's least value, never for an unsigned one.
func least_[T integer_](a T) bool {
	return a < 0 && a == 0-a
}

// mul_ is a * b, panicking on overflow as Rust's checked build does.
func mul_[T integer_](a, b T) T {
	c := a * b
	minus := T(0) - 1
	if a != 0 && (c/a != b || (a < 0 && a == minus && least_(b)) || (b < 0 && b == minus && least_(a))) {
		panic("attempt to multiply with overflow")
	}
	return c
}

// rem_ is a % b, panicking on overflow as Rust's checked build does.
func rem_[T integer_](a, b T) T {
	if b < 0 && b == T(0)-1 && least_(a) {
		panic("attempt to calculate the remainder with overflow")
	}
	return a % b
}
"#;

/// The Go file of `t`'s script.
pub(crate) fn emit(t: &Transcript) -> String {
	let mut e = Emitter {
		t,
		depth: 1,
		fresh: 0,
		imports: BTreeSet::new(),
		used: BTreeSet::new(),
		reserved: BTreeSet::new(),
		handles: BTreeMap::new(),
		drops: Vec::new(),
	};
	let mut used = BTreeSet::new();
	let mut locals = BTreeSet::new();
	for step in &t.steps {
		reads(step, &mut used, &mut locals);
	}
	e.used = used;
	e.reserved = RESERVED.iter().map(|s| s.to_string()).collect();
	e.reserved.extend(locals.iter().map(|name| local(name)));
	for name in t
		.names(LANGUAGE)
		.iter()
		.filter_map(|(key, name)| key.strip_prefix("crate ").map(|_| name))
	{
		e.reserved.insert(name.to_string());
	}
	for package in [
		"fmt", "strings", "errors", "unicode", "utf8", "math", "context", "slices", "rt", "io",
	] {
		e.reserved.insert(package.to_string());
	}
	for package in ["errors", "fmt", "strings", "unicode", "unicode/utf8"] {
		e.imports.insert(package.to_string());
	}
	e.rt();
	let mut body = String::new();
	e.steps(&t.steps, &mut body);
	let imports: String = e.imports.iter().map(|i| format!("\t\"{i}\"\n")).collect();
	format!(
		"// Code generated from fixtures/api/examples/transcript.rs by tests/fixtures/transcript. DO NOT EDIT.\n\npackage main\n\nimport (\n{imports})\n\n// script runs the transcript through the generated Go.\nfunc script() {{\n{body}}}\n{PRELUDE}"
	)
}

/// The names of locals `step` reads, and of the functions and unit structs of
/// `transcript.rs` it calls or makes.
fn reads(step: &Step, used: &mut BTreeSet<String>, locals: &mut BTreeSet<String>) {
	if let Step::Drop(name) = step {
		used.insert(name.clone());
	}
	each_expr(step, &mut |expr| {
		walk(expr, &mut |e| match &e.kind {
			Kind::Var(name) => {
				used.insert(name.clone());
			}
			Kind::Local(name, _) => {
				locals.insert(name.clone());
			}
			_ => {}
		})
	});
	let mut inner = |steps: &[Step]| steps.iter().for_each(|s| reads(s, used, locals));
	if let Step::Match { ok, err, .. } = step {
		inner(&ok.steps);
		inner(&err.steps);
	}
}

/// Go's keywords and the predeclared names the script calls, which no local may take.
const RESERVED: &[&str] = &[
	"break",
	"case",
	"chan",
	"const",
	"continue",
	"default",
	"defer",
	"else",
	"fallthrough",
	"for",
	"func",
	"go",
	"goto",
	"if",
	"import",
	"interface",
	"map",
	"package",
	"range",
	"return",
	"select",
	"struct",
	"switch",
	"type",
	"var",
	"any",
	"bool",
	"byte",
	"error",
	"false",
	"float32",
	"float64",
	"int",
	"int8",
	"int16",
	"int32",
	"int64",
	"len",
	"nil",
	"rune",
	"string",
	"true",
	"uint",
	"uint8",
	"uint16",
	"uint32",
	"uint64",
	"append",
	"make",
	"max",
	"min",
	"new",
	"panic",
	"script",
	"main",
];

/// The Go name of a function `transcript.rs` defines: in camel case, unexported. A unit
/// struct keeps its name.
fn local(name: &str) -> String {
	let mut out = String::new();
	let mut upper = false;
	for c in name.chars() {
		match c {
			'_' => upper = true,
			c if upper => {
				out.extend(c.to_uppercase());
				upper = false;
			}
			c => out.push(c),
		}
	}
	out
}

/// How a call's error is handled where it is bound.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Failure {
	/// Any error panics, as Rust's call cannot fail.
	Panic,
	/// The error is bound beside the values, for the script to read.
	Bind,
}

/// A call, made: its Go values, of each type, and its error, if bound.
struct Made {
	values: Vec<String>,
	types: Vec<Type>,
	/// Whether the values are a tuple's elements, as Go returns a tuple, even one of one.
	tuple: bool,
	error: Option<String>,
}

struct Emitter<'t> {
	t: &'t Transcript,
	depth: usize,
	fresh: usize,
	/// The packages the script imports, by path.
	imports: BTreeSet<String>,
	/// The locals the script reads, which a binding of any other leaves `_`.
	used: BTreeSet<String>,
	/// Names no local may take.
	reserved: BTreeSet<String>,
	/// The temporaries holding a handle no use has taken yet, each by its type.
	handles: BTreeMap<String, Type>,
	/// What closes each handle the current statement only lent, at its end.
	drops: Vec<String>,
}

impl Emitter<'_> {
	fn indent(&self) -> String {
		"\t".repeat(self.depth)
	}

	fn line(&self, out: &mut String, line: &str) {
		for l in line.lines() {
			let _ = writeln!(out, "{}{l}", self.indent());
		}
	}

	fn fresh(&mut self, what: &str) -> String {
		self.fresh += 1;
		format!("_{what}{}", self.fresh)
	}

	/// A local's name in Go.
	fn ident(&self, name: &str) -> String {
		match self.reserved.contains(name) {
			true => format!("{name}_"),
			false => name.to_string(),
		}
	}

	/// A package of the standard library, imported.
	fn std(&mut self, path: &str) -> String {
		self.imports.insert(path.to_string());
		path.rsplit('/').next().unwrap().to_string()
	}

	/// The runtime's package, imported.
	fn rt(&mut self) -> String {
		self.imports.insert(format!("{MODULE}/rt"));
		"rt".to_string()
	}

	/// The package of `krate`, imported.
	fn package(&mut self, krate: &str) -> String {
		let name = self.t.named(LANGUAGE, &format!("crate {krate}")).to_string();
		self.imports.insert(format!("{MODULE}/{name}"));
		name
	}

	fn steps(&mut self, steps: &[Step], out: &mut String) {
		for step in steps {
			let outer = std::mem::take(&mut self.drops);
			self.step(step, out);
			let drops = std::mem::replace(&mut self.drops, outer);
			for drop in drops.iter().rev() {
				self.line(out, drop);
			}
		}
	}

	fn step(&mut self, step: &Step, out: &mut String) {
		match step {
			Step::Let(pat, value) => {
				if let Type::Result(_, err) = &value.ty {
					// `let _ =` of a `Result`, which drops its error.
					let Pat::Ignore(_) = pat else {
						panic!("a let binding a Result whole")
					};
					let made = self.made(value, Failure::Bind, false, out);
					let error = self.error_type(err);
					self.line(out, &format!("ignore_[{error}]({})", made.error.unwrap()));
					return;
				}
				let made = self.spread(value, out);
				self.bind_made(pat, &made, out);
			}
			Step::Print(pieces) => {
				let v = self.pieces(pieces, out);
				let fmt = self.std("fmt");
				self.line(out, &format!("{fmt}.Println({v})"));
			}
			Step::Do(value) => self.discard(value, out),
			Step::Drop(name) => match unref(&self.local_type(name)).base() {
				ty @ Type::Object(_) => {
					let close = self.close(&self.ident(name), ty);
					self.line(out, &close)
				}
				ty if !holds(self.t, ty) => self.line(out, &format!("_ = {}", self.ident(name))),
				ty => panic!("a drop of {ty:?}, which holds a handle"),
			},
			Step::Match { value, ok, err } => self.matched(value, ok, err, out),
		}
	}

	/// The type the script binds `name` to.
	fn local_type(&self, name: &str) -> Type {
		fn find(steps: &[Step], name: &str) -> Option<Type> {
			let named = |pat: &Pat| {
				let mut bound = Vec::new();
				super::bound(pat, &mut bound);
				bound.into_iter().find(|(n, _)| n == name).map(|(_, ty)| ty)
			};
			steps.iter().find_map(|step| match step {
				Step::Let(pat, _) => named(pat),
				Step::Match { ok, err, .. } => [ok, err]
					.into_iter()
					.find_map(|arm| arm.pat.as_ref().and_then(named).or_else(|| find(&arm.steps, name))),
				_ => None,
			})
		}
		find(&self.t.steps, name).unwrap_or_else(|| panic!("no local {name}"))
	}

	/// An expression evaluated for its effect: what it returns, dropped, each handle closed.
	fn discard(&mut self, value: &Expr, out: &mut String) {
		if let Type::Result(_, err) = &value.ty {
			let made = self.made(value, Failure::Bind, false, out);
			let error = self.error_type(err);
			self.line(out, &format!("ignore_[{error}]({})", made.error.unwrap()));
			return;
		}
		match &value.kind {
			Kind::Call(..) => {
				let made = self.made(value, Failure::Panic, true, out);
				for (v, ty) in made.values.iter().zip(&made.types) {
					self.dropped(v, ty, out);
				}
			}
			Kind::Local(..) if value.ty == Type::Unit => {
				let call = self.local_call(value, out);
				self.line(out, &call);
			}
			_ => {
				let v = self.expr(value, out);
				self.dropped(&v, &value.ty, out);
			}
		}
	}

	/// The statement closing `v`, a handle of `ty` or an optional one, as Rust drops it.
	fn close(&self, v: &str, ty: &Type) -> String {
		match unref(ty).base() {
			ty @ Type::Object(_) => format!("done_({v}.{}())", self.t.member(LANGUAGE, ty, "Drop")),
			Type::Option(inner) => format!("if {v} != nil {{\n\t{}\n}}", self.close(v, inner)),
			ty => panic!("a close of {ty:?}"),
		}
	}

	/// The value `v` of `ty`, which nothing reads: a handle closes, as Rust drops it.
	fn dropped(&mut self, v: &str, ty: &Type, out: &mut String) {
		self.handles.remove(v);
		let line = match unref(ty).base() {
			Type::Object(_) => self.close(v, ty),
			Type::Option(inner) if matches!(inner.base(), Type::Object(_)) => self.close(v, ty),
			_ => format!("_ = {v}"),
		};
		self.line(out, &line);
	}

	/// `v`, a value read only by reference: a temporary handle closes at the statement's end.
	fn lent(&mut self, v: &str) {
		if let Some(ty) = self.handles.remove(v) {
			let close = self.close(v, &ty);
			self.drops.push(close);
		}
	}

	/// `v`, a value moved: a temporary handle is the callee's.
	fn moved(&mut self, v: &str) {
		self.handles.remove(v);
	}

	/// `e`'s values: each of a call's, which may be many, or else `e`'s one.
	fn spread(&mut self, e: &Expr, out: &mut String) -> Made {
		match &e.kind {
			Kind::Call(..) => self.made(e, Failure::Panic, true, out),
			Kind::Unwrap(inner) if matches!(inner.ty, Type::Result(..)) => self.made(inner, Failure::Panic, true, out),
			_ => {
				let v = self.expr(e, out);
				Made {
					values: vec![v],
					types: vec![e.ty.clone()],
					tuple: false,
					error: None,
				}
			}
		}
	}

	/// `pat` bound to what `made` holds.
	fn bind_made(&mut self, pat: &Pat, made: &Made, out: &mut String) {
		match made.values.len() {
			1 if !made.tuple => self.bind(pat, &made.values[0], &made.types[0], out),
			_ => match pat {
				Pat::Tuple(pats) => {
					assert_eq!(pats.len(), made.values.len(), "a tuple pattern of a call's values");
					for ((p, v), ty) in pats.iter().zip(&made.values).zip(&made.types) {
						self.bind(p, v, ty, out);
					}
				}
				pat => {
					let v = self.pack(&made.values, &made.types);
					let ty = Type::Tuple(made.types.clone());
					self.bind(pat, &v, &ty, out);
				}
			},
		}
	}

	/// `pat` bound to `v`, a value of `ty`.
	fn bind(&mut self, pat: &Pat, v: &str, ty: &Type, out: &mut String) {
		match pat {
			Pat::Name(name, _) if self.used.contains(name) => {
				let name = self.ident(name);
				self.line(out, &format!("{name} := {v}"));
			}
			Pat::Name(..) | Pat::Ignore(_) => self.dropped(v, ty, out),
			Pat::Tuple(pats) => {
				let parts = self.parts(v, ty);
				assert_eq!(parts.len(), pats.len(), "a tuple pattern of another length");
				for (p, (part, ty)) in pats.iter().zip(parts) {
					self.bind(p, &part, &ty, out);
				}
			}
		}
	}

	/// The elements of `v`, a tuple or bounds of `ty`.
	fn parts(&mut self, v: &str, ty: &Type) -> Vec<(String, Type)> {
		match unref(ty).base() {
			Type::Bounds(int) => vec![
				(format!("{v}.Start"), Type::Bound(*int)),
				(format!("{v}.End"), Type::Bound(*int)),
			],
			Type::Tuple(elements) if elements.len() == 1 => vec![(v.to_string(), elements[0].clone())],
			Type::Tuple(elements) => (elements.iter().enumerate())
				.map(|(i, ty)| (format!("{v}.V{i}"), ty.clone()))
				.collect(),
			ty => panic!("the elements of {ty:?}"),
		}
	}

	/// Values of each of `types` as one tuple.
	fn pack(&mut self, values: &[String], types: &[Type]) -> String {
		match values.len() {
			0 => panic!("no value of a call returning none"),
			1 => values[0].clone(),
			n => {
				let rt = self.rt();
				let tys: Vec<String> = types.iter().map(|t| self.go(t)).collect();
				let fields: Vec<String> = values.iter().enumerate().map(|(i, v)| format!("V{i}: {v}")).collect();
				format!("{rt}.Tuple{n}[{}]{{{}}}", tys.join(", "), fields.join(", "))
			}
		}
	}

	fn matched(&mut self, value: &Expr, ok: &Arm, err: &Arm, out: &mut String) {
		match unref(&value.ty).base() {
			Type::Option(inner) => {
				let v = self.expr(value, out);
				self.lent(&v);
				self.line(out, &format!("if {v} != nil {{"));
				self.depth += 1;
				if let Some(pat) = &ok.pat {
					let some = self.deref(&v, inner);
					self.bind(pat, &some, inner, out);
				}
				self.steps(&ok.steps, out);
				self.depth -= 1;
				self.line(out, "} else {");
				self.depth += 1;
				self.steps(&err.steps, out);
				self.depth -= 1;
				self.line(out, "}");
			}
			Type::Result(_, err_ty) => {
				let made = self.made(value, Failure::Bind, true, out);
				let error = made.error.clone().unwrap();
				self.line(out, &format!("if {error} == nil {{"));
				self.depth += 1;
				if let Some(pat) = &ok.pat {
					self.bind_made(pat, &made, out);
				}
				self.steps(&ok.steps, out);
				self.depth -= 1;
				self.line(out, "} else {");
				self.depth += 1;
				if let Some(pat) = &err.pat {
					let payload = self.payload(&error, err_ty);
					self.bind(pat, &payload, err_ty, out);
				}
				self.steps(&err.steps, out);
				self.depth -= 1;
				self.line(out, "}");
			}
			ty => panic!("a match on {ty:?}"),
		}
	}

	/// What `v`, an `Option` of `inner`, holds: a handle is its own pointer.
	fn deref(&self, v: &str, inner: &Type) -> String {
		match inner.base() {
			Type::Object(_) => v.to_string(),
			_ => format!("(*{v})"),
		}
	}

	/// The Go type of the error a call failing with `err` returns.
	fn error_type(&mut self, err: &Type) -> String {
		match unref(err).base() {
			ty @ (Type::Record(_) | Type::Enum(_) | Type::Object(_)) => self.go(ty),
			ty => {
				let named = self.t.error(LANGUAGE, ty).to_string();
				let (krate, name) = named.split_once("::").unwrap();
				let package = match krate {
					"" => self.rt(),
					krate => self.package(krate),
				};
				format!("{package}.{name}")
			}
		}
	}

	/// The Rust value of `error`, a Go error of a call failing with `err`.
	fn payload(&mut self, error: &str, err: &Type) -> String {
		let ty = self.error_type(err);
		match unref(err).base() {
			Type::Record(_) | Type::Enum(_) | Type::Object(_) => format!("errOf_[{ty}]({error})"),
			_ => format!("errOf_[{ty}]({error}).Value"),
		}
	}

	/// The Go of a format's pieces: a string expression.
	fn pieces(&mut self, pieces: &[Piece], out: &mut String) -> String {
		let mut parts = Vec::new();
		for piece in pieces {
			match piece {
				Piece::Text(text) => parts.push(string(text)),
				Piece::Value(e, spec) => {
					let v = self.expr(e, out);
					let shown = self.show(&v, &e.ty, *spec, out);
					self.lent(&v);
					parts.push(shown);
				}
			}
		}
		match parts.len() {
			0 => "\"\"".to_string(),
			_ => parts.join(" + "),
		}
	}

	/// `v`, a value of `ty`, as a string the way Rust formats it with `spec`.
	fn show(&mut self, v: &str, ty: &Type, spec: Spec, out: &mut String) -> String {
		let ty = unref(ty).base().clone();
		let fmt = self.std("fmt");
		match spec {
			Spec::Hex(width) => {
				let width = match width {
					0 => String::new(),
					w => format!("0{w}"),
				};
				return match &ty {
					Type::U128 => format!("hex128_({v}, {})", if width.is_empty() { "0" } else { &width[1..] }),
					Type::Int(int) | Type::NonZero(int) if int.signed() => {
						format!("{fmt}.Sprintf(\"%{width}x\", uint{}({v}))", int.bits())
					}
					Type::Int(_) | Type::NonZero(_) => format!("{fmt}.Sprintf(\"%{width}x\", {v})"),
					ty => panic!("hex of {ty:?}"),
				};
			}
			Spec::Padded(width) => return format!("{fmt}.Sprintf(\"%0{width}d\", {v})"),
			Spec::Debug => return self.debug(v, &ty),
			Spec::Display => {}
		}
		match &ty {
			Type::Bool | Type::Int(_) | Type::NonZero(_) => format!("{fmt}.Sprint({v})"),
			Type::String | Type::Str | Type::Path | Type::PathBuf => v.to_string(),
			Type::Char => format!("string({v})"),
			Type::U128 => format!("{v}.Big().String()"),
			Type::Object(i) if !self.t.api.objects[*i].traits.display => {
				let index = (self.t.api.objects[*i].methods.iter())
					.position(|m| m.access == Access::Text)
					.unwrap_or_else(|| panic!("Display of {}, which has none", self.t.rust(&ty)));
				let id = FnId {
					owner: super::Owner::Object(*i),
					index,
				};
				let made = self.call_with(id, Some(v.to_string()), Vec::new(), Failure::Panic, true, out);
				made.values[0].clone()
			}
			ty @ (Type::Object(_) | Type::Record(_) | Type::Enum(_)) => {
				format!("{v}.{}()", self.t.member(LANGUAGE, ty, "Display"))
			}
			ty => panic!("Display of {ty:?}"),
		}
	}

	/// `v` as Rust's `Debug` prints a value of `ty`.
	fn debug(&mut self, v: &str, ty: &Type) -> String {
		let fmt = self.std("fmt");
		match unref(ty).base() {
			Type::Bool | Type::Int(_) | Type::NonZero(_) => format!("{fmt}.Sprint({v})"),
			Type::U128 => format!("{v}.Big().String()"),
			Type::String | Type::Str | Type::Path | Type::PathBuf => format!("debugString_({v})"),
			Type::Option(inner) => {
				let x = self.fresh("x");
				let show = self.debug(&x, inner);
				let go = self.go(inner);
				match inner.base() {
					Type::Object(_) => format!("optionHandle_({v}, func({x} {go}) string {{ return {show} }})"),
					_ => format!("option_({v}, func({x} {go}) string {{ return {show} }})"),
				}
			}
			Type::Vec | Type::Slice | Type::Bytes => format!("debugBytes_({v}.Copy())"),
			Type::Array(_) => format!("debugBytes_({v}[:])"),
			Type::List(_, item) => {
				let x = self.fresh("x");
				let show = self.debug(&x, item);
				let go = self.go(item);
				format!("list_({v}, func({x} {go}) string {{ return {show} }})")
			}
			Type::Tuple(elements) => {
				let elements = elements.clone();
				let parts: Vec<String> = match elements.len() {
					1 => vec![self.debug(v, &elements[0])],
					_ => (elements.iter().enumerate())
						.map(|(i, ty)| self.debug(&format!("{v}.V{i}"), ty))
						.collect(),
				};
				let tail = if parts.len() == 1 { ",)" } else { ")" };
				format!("\"(\" + {} + \"{tail}\"", parts.join(" + \", \" + "))
			}
			Type::Bound(_) => format!("bound_({v}.Kind, {v}.Value)"),
			ty @ (Type::Record(_) | Type::Enum(_) | Type::Object(_)) => {
				format!("{v}.{}()", self.t.member(LANGUAGE, ty, "Debug"))
			}
			ty => panic!("Debug of {ty:?}"),
		}
	}

	fn expr(&mut self, e: &Expr, out: &mut String) -> String {
		self.value(e, &e.ty, out)
	}

	/// The Go of `e`, passed where `want` is taken: a call's value is bound before the
	/// statement, each in Rust's order.
	fn value(&mut self, e: &Expr, want: &Type, out: &mut String) -> String {
		let base = unref(want).base().clone();
		match &e.kind {
			Kind::Unit => panic!("a value of ()"),
			Kind::Bool(b) => b.to_string(),
			Kind::Int(n) => match &base {
				Type::U128 => {
					let rt = self.rt();
					let v = n.magnitude;
					assert!(!n.negative, "a negative u128");
					format!("{rt}.U128{{Hi: {:#x}, Lo: {:#x}}}", (v >> 64) as u64, v as u64)
				}
				Type::Int(int) | Type::NonZero(int) => format!("{}({n})", go_int(*int)),
				ty => match unref(&e.ty).base() {
					Type::Int(int) | Type::NonZero(int) => format!("{}({n})", go_int(*int)),
					_ => panic!("an integer as {ty:?}"),
				},
			},
			Kind::Float(f) => {
				let math = self.std("math");
				match &base {
					Type::F32 => format!("{math}.Float32frombits({:#x})", (*f as f32).to_bits()),
					_ => format!("{math}.Float64frombits({:#x})", f.to_bits()),
				}
			}
			Kind::Char(c) => format!("rune({:#x})", *c as u32),
			Kind::Str(s) => string(s),
			Kind::Bytes(b) => match &base {
				Type::Array(n) => {
					let items: Vec<String> = b.iter().map(|b| format!("{b:#04x}")).collect();
					format!("[{n}]byte{{{}}}", items.join(", "))
				}
				_ => {
					let rt = self.rt();
					format!("{rt}.NewBytes([]byte({}))", bytes(b))
				}
			},
			Kind::Duration(secs, nanos) => {
				let rt = self.rt();
				match (secs, nanos) {
					(0, 0) => format!("{rt}.Duration{{}}"),
					_ => format!("must_({rt}.NewDuration({secs}, {nanos}))"),
				}
			}
			Kind::Socket(addr) => self.socket(addr, &base),
			Kind::Ip(ip) => self.ip(ip, &base),
			Kind::None => {
				let ty = match &base {
					Type::Option(_) => base.clone(),
					_ => unref(&e.ty).base().clone(),
				};
				format!("({})(nil)", self.go(&ty))
			}
			Kind::Some(inner) => {
				let want = match &base {
					Type::Option(inner) => (**inner).clone(),
					_ => match unref(&e.ty).base() {
						Type::Option(inner) => (**inner).clone(),
						ty => panic!("Some as {ty:?}"),
					},
				};
				let v = self.value(inner, &want, out);
				match want.base() {
					Type::Object(_) => v,
					_ => format!("ptr_({v})"),
				}
			}
			Kind::Tuple(elements) => self.tuple(e, elements, &base, out),
			Kind::List(items) => {
				let item = match &base {
					Type::List(_, item) => (**item).clone(),
					ty => panic!("a list as {ty:?}"),
				};
				let go = self.go(&item);
				let parts: Vec<String> = items.iter().map(|i| self.value(i, &item, out)).collect();
				for (p, i) in parts.iter().zip(items) {
					let _ = i;
					self.moved(p);
				}
				format!("[]{go}{{{}}}", parts.join(", "))
			}
			Kind::Record(r, fields) => {
				let ty = Type::Record(*r);
				let name = self.go(&ty);
				let record = &self.t.api.records[*r];
				let named: Vec<(String, Type)> = (record.fields.iter())
					.map(|f| (self.t.field(LANGUAGE, &ty, &f.name).to_string(), f.ty.clone()))
					.collect();
				let values: Vec<String> = (fields.iter().zip(&named))
					.map(|(f, (name, ty))| {
						let v = self.value(f, ty, out);
						self.moved(&v);
						format!("{name}: {v}")
					})
					.collect();
				format!("{name}{{{}}}", values.join(", "))
			}
			Kind::Variant(en, v, fields) => {
				let ty = Type::Enum(*en);
				let enumeration = &self.t.api.enums[*en];
				let variant = &enumeration.variants[*v];
				let name = self.t.variant(LANGUAGE, &ty, &variant.name).to_string();
				let package = self.package(&enumeration.home.krate.clone());
				if enumeration.variants.iter().all(|v| v.fields.is_empty()) {
					return format!("{package}.{name}");
				}
				let named: Vec<(String, Type)> = (variant.fields.iter())
					.map(|f| {
						let name = self.t.variant_field(LANGUAGE, &ty, &variant.name, &f.name);
						(name.to_string(), f.ty.clone())
					})
					.collect();
				let values: Vec<String> = (fields.iter().zip(&named))
					.map(|(f, (name, ty))| {
						let v = self.value(f, ty, out);
						self.moved(&v);
						format!("{name}: {v}")
					})
					.collect();
				format!("{package}.{name}{{{}}}", values.join(", "))
			}
			Kind::Bound(kind, value) => {
				let Type::Bound(_) = &base else {
					panic!("a bound as {base:?}")
				};
				let rt = self.rt();
				let ty = self.go(&base);
				match (kind, value) {
					(BoundKind::Unbounded, _) => format!("{ty}{{Kind: {rt}.BoundUnbounded}}"),
					(BoundKind::Included, Some(v)) => {
						let int = bound_int(&base);
						let v = self.value(v, &int, out);
						format!("{ty}{{Kind: {rt}.BoundIncluded, Value: {v}}}")
					}
					(BoundKind::Excluded, Some(v)) => {
						let int = bound_int(&base);
						let v = self.value(v, &int, out);
						format!("{ty}{{Kind: {rt}.BoundExcluded, Value: {v}}}")
					}
					_ => unreachable!(),
				}
			}
			Kind::Default => {
				let ty = match &base {
					Type::Record(_) | Type::Object(_) => base.clone(),
					_ => unref(&e.ty).base().clone(),
				};
				let name = self.t.member(LANGUAGE, &ty, "Default").to_string();
				let krate = self.home(&ty);
				let package = self.package(&krate);
				let call = format!("{package}.{name}()");
				let made = self.hoist(&call, &[ty], false, Failure::Panic, true, out);
				made.values[0].clone()
			}
			Kind::Const(c) => {
				let constant = &self.t.api.constants[*c];
				let name = self
					.t
					.named(LANGUAGE, &format!("const {}", constant.path.join("::")))
					.to_string();
				let package = self.package(&constant.home.krate.clone());
				format!("{package}.{name}")
			}
			Kind::Closure(params, steps, value) => self.closure(params, steps, value, &base, out),
			Kind::Local(..) if e.ty == Type::Unit => panic!("a value of a function returning nothing"),
			Kind::Local(name, args) if args.is_empty() && name.starts_with(char::is_uppercase) => format!("{name}{{}}"),
			Kind::Format(pieces) => self.pieces(pieces, out),
			_ => {
				let v = self.computed(e, out);
				self.coerce(&v, &e.ty, &base)
			}
		}
	}

	/// The Go of `e`, an expression no literal spells, as a value of its own type.
	fn computed(&mut self, e: &Expr, out: &mut String) -> String {
		match &e.kind {
			Kind::Var(name) => self.ident(name),
			Kind::Call(..) => {
				let made = self.made(e, Failure::Panic, true, out);
				self.pack(&made.values, &made.types)
			}
			Kind::Local(..) => {
				let call = self.local_call(e, out);
				let v = self.fresh("t");
				self.line(out, &format!("{v} := {call}"));
				v
			}
			Kind::Field(base, i) => {
				let b = self.expr(base, out);
				let Type::Record(r) = unref(&base.ty).base() else {
					panic!("a field of {:?}", base.ty)
				};
				let field = &self.t.api.records[*r].fields[*i].name;
				let name = self.t.field(LANGUAGE, &Type::Record(*r), field).to_string();
				format!("{b}.{name}")
			}
			Kind::Element(base, i) => {
				let b = self.expr(base, out);
				match unref(&base.ty).base() {
					Type::Range(_) => format!("{b}.{}", if *i == 0 { "Start" } else { "End" }),
					ty => self.parts(&b, &ty.clone()).swap_remove(*i).0,
				}
			}
			Kind::Clone(inner) => {
				let v = self.expr(inner, out);
				match unref(&inner.ty).base() {
					Type::Object(_) => {
						self.lent(&v);
						let made = self.hoist(
							&format!("{v}.{}()", self.t.member(LANGUAGE, &inner.ty, "Clone")),
							std::slice::from_ref(&inner.ty),
							true,
							Failure::Panic,
							true,
							out,
						);
						made.values[0].clone()
					}
					Type::List(..) => format!("{}.Clone({v})", self.std("slices")),
					_ => v,
				}
			}
			Kind::Unwrap(inner) => match unref(&inner.ty).base() {
				Type::Result(..) => {
					let made = self.made(inner, Failure::Panic, true, out);
					self.pack(&made.values, &made.types)
				}
				Type::Option(item) => {
					let v = self.expr(inner, out);
					match item.base() {
						Type::Object(_) => {
							self.moved(&v);
							let made = self.hoist(
								&format!("present_({v})"),
								&[(**item).clone()],
								false,
								Failure::Panic,
								true,
								out,
							);
							made.values[0].clone()
						}
						_ => format!("some_({v})"),
					}
				}
				ty => panic!("unwrap of {ty:?}"),
			},
			Kind::UnwrapErr(inner) => {
				let Type::Result(_, err) = &inner.ty else {
					panic!("unwrap_err of {:?}", inner.ty)
				};
				let made = self.made(inner, Failure::Bind, false, out);
				self.payload(&made.error.unwrap(), err)
			}
			Kind::IsErr(inner) => {
				let Type::Result(_, err) = &inner.ty else {
					panic!("is_err of {:?}", inner.ty)
				};
				let made = self.made(inner, Failure::Bind, false, out);
				let error = self.error_type(err);
				format!("isErr_[{error}]({})", made.error.unwrap())
			}
			Kind::IsSome(inner) | Kind::IsNone(inner) => {
				let v = self.expr(inner, out);
				let test = format!(
					"({v} {} nil)",
					if matches!(e.kind, Kind::IsSome(_)) { "!=" } else { "==" }
				);
				self.lent(&v);
				test
			}
			Kind::Binary(op, a, b) => self.binary(*op, a, b, out),
			Kind::Join { items, pat, body, sep } => {
				let items_v = self.expr(items, out);
				self.lent(&items_v);
				let Type::List(_, item) = unref(&items.ty).base() else {
					panic!("a join over {:?}", items.ty)
				};
				let item = (**item).clone();
				let parts = self.fresh("j");
				let x = self.fresh("x");
				self.line(out, &format!("var {parts} []string"));
				self.line(out, &format!("for _, {x} := range {items_v} {{"));
				self.depth += 1;
				let outer = std::mem::take(&mut self.drops);
				self.bind(pat, &x, &item, out);
				let v = self.expr(body, out);
				self.line(out, &format!("{parts} = append({parts}, {v})"));
				let drops = std::mem::replace(&mut self.drops, outer);
				for drop in drops.iter().rev() {
					self.line(out, drop);
				}
				self.depth -= 1;
				self.line(out, "}");
				format!("{}.Join({parts}, {})", self.std("strings"), string(sep))
			}
			Kind::MapOr {
				option,
				none,
				pat,
				some,
			} => {
				let option_v = self.expr(option, out);
				let Type::Option(inner) = unref(&option.ty).base() else {
					panic!("map_or of {:?}", option.ty)
				};
				let inner = (**inner).clone();
				let none_v = self.value(none, &e.ty, out);
				let result = self.fresh("m");
				self.line(out, &format!("{result} := {none_v}"));
				self.line(out, &format!("if {option_v} != nil {{"));
				self.depth += 1;
				let outer = std::mem::take(&mut self.drops);
				let some_v = self.deref(&option_v, &inner);
				self.bind(pat, &some_v, &inner, out);
				let v = self.value(some, &e.ty, out);
				self.line(out, &format!("{result} = {v}"));
				let drops = std::mem::replace(&mut self.drops, outer);
				for drop in drops.iter().rev() {
					self.line(out, drop);
				}
				self.depth -= 1;
				self.line(out, "}");
				self.lent(&option_v);
				result
			}
			Kind::Utf8(inner) => {
				let v = self.expr(inner, out);
				match unref(&inner.ty).base() {
					Type::Array(_) => format!("lossy_({v}[:])"),
					Type::Vec | Type::Slice | Type::Bytes => format!("lossy_({v}.Copy())"),
					ty => panic!("text of {ty:?}"),
				}
			}
			Kind::Len(inner) => {
				let v = self.expr(inner, out);
				match unref(&inner.ty).base() {
					Type::Vec | Type::Slice | Type::Bytes => format!("uint({v}.Len())"),
					Type::Array(_) | Type::String | Type::Str | Type::List(..) => format!("uint(len({v}))"),
					ty => panic!("len of {ty:?}"),
				}
			}
			kind => panic!("{} has no Go for this expression", describe(kind)),
		}
	}

	/// `v`, a value of `have`, as one of `want`: an address of one family where any is
	/// taken, or an array of bytes where bytes are.
	fn coerce(&mut self, v: &str, have: &Type, want: &Type) -> String {
		match (unref(have).base(), want) {
			(Type::SocketAddr(Family::V4), Type::SocketAddr(Family::Any)) => {
				format!("{}.SocketAddrFromV4({v})", self.rt())
			}
			(Type::SocketAddr(Family::V6), Type::SocketAddr(Family::Any)) => {
				format!("{}.SocketAddrFromV6({v})", self.rt())
			}
			(Type::IpAddr(Family::V4), Type::IpAddr(Family::Any)) => format!("{}.IpAddrFromV4({v})", self.rt()),
			(Type::IpAddr(Family::V6), Type::IpAddr(Family::Any)) => format!("{}.IpAddrFromV6({v})", self.rt()),
			(Type::Array(_), Type::Slice | Type::Vec | Type::Bytes) => format!("{}.NewBytes({v}[:])", self.rt()),
			_ => v.to_string(),
		}
	}

	fn socket(&mut self, addr: &std::net::SocketAddr, want: &Type) -> String {
		let rt = self.rt();
		let (value, family) = match addr {
			std::net::SocketAddr::V4(a) => {
				let ip: Vec<String> = a.ip().octets().iter().map(|o| o.to_string()).collect();
				(
					format!(
						"{rt}.SocketAddrV4{{IP: {rt}.Ipv4Addr{{{}}}, Port: {}}}",
						ip.join(", "),
						a.port()
					),
					"V4",
				)
			}
			std::net::SocketAddr::V6(a) => {
				let ip: Vec<String> = a.ip().octets().iter().map(|o| o.to_string()).collect();
				(
					format!(
						"{rt}.SocketAddrV6{{IP: {rt}.Ipv6Addr{{{}}}, Port: {}, FlowInfo: {}, ScopeID: {}}}",
						ip.join(", "),
						a.port(),
						a.flowinfo(),
						a.scope_id()
					),
					"V6",
				)
			}
		};
		match want {
			Type::SocketAddr(Family::Any) => format!("{rt}.SocketAddrFrom{family}({value})"),
			_ => value,
		}
	}

	fn ip(&mut self, ip: &std::net::IpAddr, want: &Type) -> String {
		let rt = self.rt();
		let (value, family) = match ip {
			std::net::IpAddr::V4(ip) => {
				let o: Vec<String> = ip.octets().iter().map(|o| o.to_string()).collect();
				(format!("{rt}.Ipv4Addr{{{}}}", o.join(", ")), "V4")
			}
			std::net::IpAddr::V6(ip) => {
				let o: Vec<String> = ip.octets().iter().map(|o| o.to_string()).collect();
				(format!("{rt}.Ipv6Addr{{{}}}", o.join(", ")), "V6")
			}
		};
		match want {
			Type::IpAddr(Family::Any) => format!("{rt}.IpAddrFrom{family}({value})"),
			_ => value,
		}
	}

	fn tuple(&mut self, e: &Expr, elements: &[Expr], base: &Type, out: &mut String) -> String {
		let ty = match base {
			Type::Range(_) | Type::Bounds(_) | Type::Tuple(_) => base.clone(),
			_ => unref(&e.ty).base().clone(),
		};
		match &ty {
			Type::Range(int) | Type::Bounds(int) => {
				let element = match ty {
					Type::Range(_) => Type::Int(*int),
					_ => Type::Bound(*int),
				};
				let go = self.go(&ty);
				let (a, b) = (
					self.value(&elements[0], &element, out),
					self.value(&elements[1], &element, out),
				);
				format!("{go}{{Start: {a}, End: {b}}}")
			}
			Type::Tuple(types) => {
				let values: Vec<String> = (elements.iter().zip(types))
					.map(|(e, t)| self.value(e, t, out))
					.collect();
				for v in &values {
					self.moved(v);
				}
				self.pack(&values, types)
			}
			ty => panic!("a tuple as {ty:?}"),
		}
	}

	fn binary(&mut self, op: Op, a: &Expr, b: &Expr, out: &mut String) -> String {
		if op == Op::And {
			// `&&` evaluates its right side only once its left holds.
			let left = self.expr(a, out);
			let result = self.fresh("b");
			self.line(out, &format!("{result} := {left}"));
			self.line(out, &format!("if {result} {{"));
			self.depth += 1;
			let outer = std::mem::take(&mut self.drops);
			let right = self.expr(b, out);
			self.line(out, &format!("{result} = {right}"));
			let drops = std::mem::replace(&mut self.drops, outer);
			for drop in drops.iter().rev() {
				self.line(out, drop);
			}
			self.depth -= 1;
			self.line(out, "}");
			return result;
		}
		let ty = unref(&a.ty).base().clone();
		let (x, y) = (self.expr(a, out), self.value(b, &ty, out));
		match op {
			Op::Eq | Op::Ne => {
				let not = if op == Op::Ne { "!" } else { "" };
				match &ty {
					Type::Record(_) | Type::Enum(_) | Type::Object(_) => {
						self.lent(&x);
						self.lent(&y);
						let fails = matches!(ty, Type::Object(_)) || fails(self.t, &ty);
						let made = self.hoist(
							&format!("{x}.{}({y})", self.t.member(LANGUAGE, &ty, "PartialEq")),
							&[Type::Bool],
							fails,
							Failure::Panic,
							true,
							out,
						);
						format!("{not}{}", made.values[0])
					}
					Type::Bool
					| Type::Int(_)
					| Type::NonZero(_)
					| Type::Char
					| Type::F32
					| Type::F64
					| Type::String
					| Type::Str => {
						format!("({x} {} {y})", if op == Op::Eq { "==" } else { "!=" })
					}
					ty => panic!("== of {ty:?}"),
				}
			}
			// A value of the API orders through `PartialCompare`, whose false is unordered, and
			// with `Ord` through `Compare`.
			Op::Lt | Op::Le | Op::Gt | Op::Ge | Op::Cmp
				if matches!(ty, Type::Record(_) | Type::Enum(_) | Type::Object(_)) =>
			{
				self.lent(&x);
				self.lent(&y);
				let fails = matches!(ty, Type::Object(_)) || fails(self.t, &ty);
				if op == Op::Cmp {
					let made = self.hoist(
						&format!("{x}.{}({y})", self.t.member(LANGUAGE, &ty, "Ord")),
						&[Type::Int(Int::I8)],
						fails,
						Failure::Panic,
						true,
						out,
					);
					return made.values[0].clone();
				}
				let made = self.hoist(
					&format!("{x}.{}({y})", self.t.member(LANGUAGE, &ty, "PartialOrd")),
					&[Type::Int(Int::I8), Type::Bool],
					fails,
					Failure::Panic,
					true,
					out,
				);
				let sym = match op {
					Op::Lt => "<",
					Op::Le => "<=",
					Op::Gt => ">",
					_ => ">=",
				};
				format!("({} && {} {sym} 0)", made.values[1], made.values[0])
			}
			Op::Lt => format!("({x} < {y})"),
			Op::Le => format!("({x} <= {y})"),
			Op::Gt => format!("({x} > {y})"),
			Op::Ge => format!("({x} >= {y})"),
			Op::Cmp => unreachable!("`cmp` of a value of the API"),
			Op::Add | Op::Sub | Op::Mul | Op::Rem => {
				let Type::Int(_) = ty else {
					panic!("arithmetic on {ty:?}")
				};
				let f = match op {
					Op::Add => "add_",
					Op::Sub => "sub_",
					Op::Mul => "mul_",
					_ => "rem_",
				};
				format!("{f}({x}, {y})")
			}
			Op::And => unreachable!(),
		}
	}

	/// A closure, as a function literal bound before the statement that passes it.
	fn closure(&mut self, params: &[Pat], steps: &[Step], value: &Expr, want: &Type, out: &mut String) -> String {
		let Type::Closure(closure) = want else {
			panic!("a closure as {want:?}")
		};
		if closure.err.is_some() {
			panic!("a closure returning a Result, which Go returns beside an error");
		}
		let types: Vec<String> = closure.params.iter().map(|ty| self.go_param(ty)).collect();
		let mut body = String::new();
		self.depth += 1;
		let mut names = Vec::new();
		for (pat, ty) in params.iter().zip(&closure.params) {
			match pat {
				Pat::Name(name, _) if self.used.contains(name) => names.push(self.ident(name)),
				Pat::Name(..) | Pat::Ignore(_) => names.push("_".to_string()),
				Pat::Tuple(_) => {
					let p = self.fresh("p");
					self.bind(pat, &p, ty, &mut body);
					names.push(p);
				}
			}
		}
		let outer = std::mem::take(&mut self.drops);
		self.steps(steps, &mut body);
		let result = match closure.ok {
			Type::Unit => String::new(),
			ref ok => {
				let ok = ok.clone();
				let v = self.value(value, &ok, &mut body);
				let drops = std::mem::take(&mut self.drops);
				for drop in drops.iter().rev() {
					self.line(&mut body, drop);
				}
				self.line(&mut body, &format!("return {v}"));
				format!(" {}", self.go(&ok))
			}
		};
		self.drops = outer;
		self.depth -= 1;
		let params: Vec<String> = names.iter().zip(&types).map(|(n, t)| format!("{n} {t}")).collect();
		let f = self.fresh("f");
		self.line(out, &format!("{f} := func({}){result} {{", params.join(", ")));
		out.push_str(&body);
		self.line(out, "}");
		f
	}

	/// A call of a function `transcript.rs` defines, each argument as its own type.
	fn local_call(&mut self, e: &Expr, out: &mut String) -> String {
		let Kind::Local(name, args) = &e.kind else {
			unreachable!()
		};
		let args: Vec<String> = args.iter().map(|a| self.expr(a, out)).collect();
		for a in &args {
			self.lent(a);
		}
		format!("{}({})", local(name), args.join(", "))
	}

	/// The crate a record, enum, or object belongs to.
	fn home(&self, ty: &Type) -> String {
		let api = &self.t.api;
		match ty {
			Type::Record(i) => api.records[*i].home.krate.clone(),
			Type::Enum(i) => api.enums[*i].home.krate.clone(),
			Type::Object(i) => api.objects[*i].home.krate.clone(),
			Type::Callback(i, _) => api.callbacks[*i].home.krate.clone(),
			ty => panic!("{ty:?} belongs to no crate"),
		}
	}

	/// A call expression `e`, made: bound before the statement, its error handled as
	/// `failure` says, its values `kept` or else left `_`.
	fn made(&mut self, e: &Expr, failure: Failure, kept: bool, out: &mut String) -> Made {
		let Kind::Call(id, receiver, args) = &e.kind else {
			panic!("{} is no call", describe(&e.kind))
		};
		let f = self.t.function(*id);
		let receiver = receiver.as_ref().map(|r| {
			let v = self.expr(r, out);
			match f.receiver {
				Some(Receiver::Owned) => self.moved(&v),
				_ => self.lent(&v),
			}
			v
		});
		let mut values = Vec::new();
		for (arg, param) in args.iter().zip(&f.params) {
			if written(&param.ty) {
				// A value passed for `&mut`, which the call writes back through its pointer.
				let Kind::Var(name) = &arg.kind else {
					panic!("a value passed for &mut that is no local")
				};
				values.push(format!("&{}", self.ident(name)));
				continue;
			}
			let v = self.value(arg, &param.ty, out);
			match param.ty {
				Type::Ref(..) => self.lent(&v),
				_ => self.moved(&v),
			}
			values.push(v);
		}
		self.call_with(*id, receiver, values, failure, kept, out)
	}

	/// The call of `id` on `receiver` with `args`, each already Go, made.
	fn call_with(
		&mut self,
		id: FnId,
		receiver: Option<String>,
		mut args: Vec<String>,
		failure: Failure,
		kept: bool,
		out: &mut String,
	) -> Made {
		let f = self.t.function(id);
		let name = self.t.name(LANGUAGE, id).to_string();
		if f.future {
			let context = self.std("context");
			args.insert(0, format!("{context}.Background()"));
		}
		let target = match receiver {
			Some(r) => r,
			None => self.package(&f.home.krate.clone()),
		};
		let call = format!("{target}.{name}({})", args.join(", "));
		let types: Vec<Type> = match &f.ok {
			Type::Unit => Vec::new(),
			Type::Tuple(elements) => elements.clone(),
			ok => vec![ok.clone()],
		};
		let errs = self.errs(id);
		let mut made = self.hoist(&call, &types, errs, failure, kept, out);
		made.tuple = matches!(f.ok, Type::Tuple(_));
		made
	}

	/// Whether the Go of `id` returns an error beside its values: Rust's own, a future's
	/// context's, or a check of what it takes failing, as of a closed handle or text Rust's
	/// type does not hold, or of an item the target lacks.
	fn errs(&self, id: FnId) -> bool {
		let f = self.t.function(id);
		let receiver = f.receiver.is_some() && self.t.owner(id).is_some_and(|o| fails(self.t, &o));
		f.err.is_some()
			|| f.future
			|| receiver
			|| f.params.iter().any(|p| fails(self.t, &p.ty))
			|| !self.t.api.everywhere(f.on)
	}

	/// `call`, of values of `types` and an error if `errs`, bound before the statement.
	fn hoist(
		&mut self,
		call: &str,
		types: &[Type],
		errs: bool,
		failure: Failure,
		kept: bool,
		out: &mut String,
	) -> Made {
		let values: Vec<String> = types
			.iter()
			.map(|_| match kept {
				true => self.fresh("t"),
				false => "_".to_string(),
			})
			.collect();
		let mut made = Made {
			values: values.clone(),
			types: types.to_vec(),
			tuple: false,
			error: None,
		};
		match (values.len(), errs, failure) {
			(0, false, _) => self.line(out, call),
			(0, true, Failure::Panic) => self.line(out, &format!("done_({call})")),
			(1, true, Failure::Panic) if kept => self.line(out, &format!("{} := must_({call})", values[0])),
			(_, false, _) if kept => self.line(out, &format!("{} := {call}", values.join(", "))),
			(_, false, _) => self.line(out, &format!("{} = {call}", values.join(", "))),
			(_, true, failure) => {
				let error = self.fresh("e");
				let mut names = values.clone();
				names.push(error.clone());
				self.line(out, &format!("{} := {call}", names.join(", ")));
				match failure {
					Failure::Panic => self.line(out, &format!("done_({error})")),
					Failure::Bind => made.error = Some(error),
				}
			}
		}
		for (v, ty) in made.values.iter().zip(types) {
			if v != "_" && holds_handle(ty) {
				self.handles.insert(v.clone(), unref(ty).base().clone());
			}
		}
		made
	}

	/// The Go type of `ty`, as the export spells it.
	fn go(&mut self, ty: &Type) -> String {
		match ty {
			Type::Bool => "bool".to_string(),
			Type::Int(int) | Type::NonZero(int) => go_int(*int).to_string(),
			Type::U128 => format!("{}.U128", self.rt()),
			Type::F32 => "float32".to_string(),
			Type::F64 => "float64".to_string(),
			Type::Char => "rune".to_string(),
			Type::Str | Type::String | Type::Path | Type::PathBuf => "string".to_string(),
			Type::Slice | Type::Vec | Type::Bytes => format!("{}.Bytes", self.rt()),
			Type::Array(n) => format!("[{n}]byte"),
			Type::Duration => format!("{}.Duration", self.rt()),
			Type::SocketAddr(family) => {
				let name = match family {
					Family::Any => "SocketAddr",
					Family::V4 => "SocketAddrV4",
					Family::V6 => "SocketAddrV6",
				};
				format!("{}.{name}", self.rt())
			}
			Type::IpAddr(family) => {
				let name = match family {
					Family::Any => "IpAddr",
					Family::V4 => "Ipv4Addr",
					Family::V6 => "Ipv6Addr",
				};
				format!("{}.{name}", self.rt())
			}
			Type::List(_, inner) => format!("[]{}", self.go(inner)),
			Type::Option(inner) => match inner.base() {
				Type::Object(_) => self.go(inner),
				_ => format!("*{}", self.go(inner)),
			},
			Type::Box(inner) | Type::Ref(_, inner) => self.go(inner),
			Type::Record(_) | Type::Enum(_) => self.named_type(ty),
			Type::Object(_) => format!("*{}", self.named_type(ty)),
			Type::Callback(i, _) => {
				let path = self.t.api.callbacks[*i].path.join("::");
				let name = self.t.named(LANGUAGE, &format!("type {path}")).to_string();
				let package = self.package(&self.home(ty));
				format!("{package}.{name}")
			}
			Type::Closure(closure) => {
				let params: Vec<String> = closure.params.iter().map(|p| self.go_param(p)).collect();
				let params = params.join(", ");
				match (&closure.ok, closure.err.is_some()) {
					(Type::Unit, false) => format!("func({params})"),
					(Type::Unit, true) => format!("func({params}) error"),
					(ok, false) => format!("func({params}) {}", self.go(ok)),
					(ok, true) => format!("func({params}) ({}, error)", self.go(ok)),
				}
			}
			Type::Range(int) => format!("{}.Range{}", self.rt(), pascal_int(*int)),
			Type::Bound(int) => format!("{}.Bound{}", self.rt(), pascal_int(*int)),
			Type::Bounds(int) => format!("{}.Bounds{}", self.rt(), pascal_int(*int)),
			Type::Tuple(elements) => match &elements[..] {
				[element] => self.go(element),
				elements => {
					let types: Vec<String> = elements.iter().map(|e| self.go(e)).collect();
					format!("{}.Tuple{}[{}]", self.rt(), elements.len(), types.join(", "))
				}
			},
			Type::Waker => "func()".to_string(),
			Type::Future => format!("func({}.Context)", self.std("context")),
			Type::Reader => format!("{}.ReadSeeker", self.std("io")),
			ty => panic!("no Go type is {ty:?}"),
		}
	}

	/// The Go type of a parameter Rust lends a closure: bytes are a `[]byte`, copied.
	fn go_param(&mut self, ty: &Type) -> String {
		match ty.base() {
			Type::Slice | Type::Vec | Type::Bytes => "[]byte".to_string(),
			Type::Option(inner) if matches!(inner.base(), Type::Slice | Type::Vec | Type::Bytes) => {
				"*[]byte".to_string()
			}
			_ => self.go(ty),
		}
	}

	/// A record's, enum's, or object's name, qualified by its package.
	fn named_type(&mut self, ty: &Type) -> String {
		let name = self.t.type_name(LANGUAGE, ty).to_string();
		let package = self.package(&self.home(ty));
		format!("{package}.{name}")
	}
}

/// Whether a parameter of `ty` takes a value the call writes back, through a pointer.
fn written(ty: &Type) -> bool {
	matches!(ty, Type::Ref(Mutability::Mut, inner) if !matches!(**inner, Type::Object(_) | Type::Reader))
}

/// What an expression kind is, for a panic naming it.
fn describe(kind: &Kind) -> String {
	let text = format!("{:?}", std::mem::discriminant(kind));
	text
}

/// Whether a value of `ty` is a handle, or an `Option` of one, which closes once dropped.
fn holds_handle(ty: &Type) -> bool {
	match unref(ty).base() {
		Type::Object(_) => true,
		Type::Option(inner) => matches!(inner.base(), Type::Object(_)),
		_ => false,
	}
}

/// Whether a value of `ty` holds a handle anywhere.
fn holds(t: &Transcript, ty: &Type) -> bool {
	let api = &t.api;
	match ty {
		Type::Object(_) => true,
		Type::Option(inner) | Type::Box(inner) | Type::Ref(_, inner) | Type::List(_, inner) => holds(t, inner),
		Type::Tuple(elements) => elements.iter().any(|e| holds(t, e)),
		Type::Record(i) => api.records[*i].fields.iter().any(|f| holds(t, &f.ty)),
		Type::Enum(i) => (api.enums[*i].variants.iter()).any(|v| v.fields.iter().any(|f| holds(t, &f.ty))),
		_ => false,
	}
}

/// Whether passing a value of `ty` can fail a Go call before it is made: it holds a
/// handle, which may be closed, or text, a char, a path, or a `NonZero`, whose Go types
/// hold values Rust's do not.
fn fails(t: &Transcript, ty: &Type) -> bool {
	let api = &t.api;
	match ty {
		Type::Object(_) | Type::Str | Type::String | Type::Char | Type::Path | Type::PathBuf | Type::NonZero(_) => true,
		Type::Option(inner) | Type::Box(inner) | Type::Ref(_, inner) | Type::List(_, inner) => fails(t, inner),
		Type::Tuple(elements) => elements.iter().any(|e| fails(t, e)),
		Type::Record(i) => api.records[*i].fields.iter().any(|f| fails(t, &f.ty)),
		Type::Enum(i) => (api.enums[*i].variants.iter()).any(|v| v.fields.iter().any(|f| fails(t, &f.ty))),
		_ => false,
	}
}

/// The integer a bound of `ty` holds.
fn bound_int(ty: &Type) -> Type {
	match ty {
		Type::Bound(int) => Type::Int(*int),
		ty => panic!("a bound as {ty:?}"),
	}
}

fn go_int(int: Int) -> &'static str {
	match int {
		Int::U8 => "uint8",
		Int::U16 => "uint16",
		Int::U32 => "uint32",
		Int::U64 => "uint64",
		Int::Usize => "uint",
		Int::I8 => "int8",
		Int::I16 => "int16",
		Int::I32 => "int32",
		Int::I64 => "int64",
		Int::Isize => "int",
	}
}

fn pascal_int(int: Int) -> String {
	let rust = int.rust();
	let mut chars = rust.chars();
	let first = chars.next().unwrap().to_ascii_uppercase();
	format!("{first}{}", chars.as_str())
}

/// A Go string literal.
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
			c if (c as u32) < 0x20 || c as u32 == 0x7f => {
				let _ = write!(out, "\\x{:02x}", c as u32);
			}
			c => out.push(c),
		}
	}
	out.push('"');
	out
}

/// A Go string literal of bytes, any byte escaped.
fn bytes(b: &[u8]) -> String {
	let mut out = String::from("\"");
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
