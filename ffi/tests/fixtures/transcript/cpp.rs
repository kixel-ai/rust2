//! The transcript as C++: a header `tests/api.cpp` includes after its namespace `hook`, which
//! holds its own functions `transcript.rs` defines, whose `transcript::script()` runs the
//! script through the generated header.
//!
//! C++ evaluates a call's arguments in no set order, but a braced list's in order, so each
//! line's values are a list, as Rust's are in order. A local moves at its last use, where
//! Rust moves it, and is copied, or cloned where C++ has no copy, at each use before.

use std::collections::BTreeSet;
use std::fmt::Write;

use super::ir::{Access, Family, Int, Mutability, Receiver, Type};
use super::{BoundKind, Expr, FnId, Kind, Op, Owner, Pat, Piece, Spec, Step, Transcript, each_expr, unref, walk};

const LANGUAGE: &str = "C++";

/// Helpers the script calls, each for a value C++ shapes apart from Rust.
const PRELUDE: &str = r#"// Generated from fixtures/api/examples/transcript.rs by tests/fixtures/transcript. Do not edit.
// tests/api.cpp includes it after its namespace `hook`, which holds the functions transcript.rs
// defines.

#include <array>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <initializer_list>
#include <iterator>
#include <optional>
#include <string>
#include <type_traits>
#include <utility>
#include <vector>

#include "api.hpp"

namespace rs {

[[noreturn]] inline void fail(const char *what) {
	std::fprintf(stderr, "%s\n", what);
	std::abort();
}

// A line of the transcript, its pieces evaluated in order, as a braced list's are.
inline void line(std::initializer_list<std::string> pieces) {
	std::string out;
	for (const auto &piece : pieces) {
		out += piece;
	}
	out += '\n';
	std::fwrite(out.data(), 1, out.size(), stdout);
}

inline std::string cat(std::initializer_list<std::string> pieces) {
	std::string out;
	for (const auto &piece : pieces) {
		out += piece;
	}
	return out;
}

// Bytes, or a string, as text.
template <class B>
std::string text(const B &bytes) {
	return std::string(reinterpret_cast<const char *>(std::data(bytes)), std::size(bytes));
}

inline std::string flag(bool value) {
	return value ? "true" : "false";
}

// Zeros before `digits` up to `width`.
inline std::string pad(std::string digits, int width) {
	if (static_cast<int>(digits.size()) < width) {
		digits.insert(0, static_cast<std::size_t>(width) - digits.size(), '0');
	}
	return digits;
}

template <class T>
std::string hex(T value, int width) {
	auto bits = static_cast<unsigned long long>(static_cast<std::make_unsigned_t<T>>(value));
	std::string digits;
	do {
		digits.insert(digits.begin(), "0123456789abcdef"[bits % 16]);
		bits /= 16;
	} while (bits != 0);
	return pad(digits, width);
}

inline std::string hex(api::u128 value, int width) {
	return pad(value.hi != 0 ? hex(value.hi, 0) + hex(value.lo, 16) : hex(value.lo, 0), width);
}

template <class T>
std::string padded(T value, int width) {
	std::string digits = std::to_string(value);
	if (digits[0] == '-') {
		return "-" + pad(digits.substr(1), width - 1);
	}
	return pad(digits, width);
}

// A string as Rust's `Debug` prints it.
template <class B>
std::string quoted(const B &bytes) {
	std::string out = "\"";
	for (char c : text(bytes)) {
		switch (c) {
		case '"':
		case '\\':
			out += '\\';
			out += c;
			break;
		case '\n':
			out += "\\n";
			break;
		case '\t':
			out += "\\t";
			break;
		case '\r':
			out += "\\r";
			break;
		case '\0':
			out += "\\0";
			break;
		default:
			if (static_cast<unsigned char>(c) < 0x20 || c == 0x7f) {
				char escaped[16];
				std::snprintf(escaped, sizeof escaped, "\\u{%x}", static_cast<unsigned>(static_cast<unsigned char>(c)));
				out += escaped;
			} else {
				out += c;
			}
		}
	}
	return out + "\"";
}

template <class T, class F>
std::string option(const std::optional<T> &value, F show) {
	return value ? "Some(" + show(*value) + ")" : "None";
}

template <class L, class F>
std::string list(const L &items, F show) {
	std::string out = "[";
	bool first = true;
	for (const auto &item : items) {
		out += first ? "" : ", ";
		out += show(item);
		first = false;
	}
	return out + "]";
}

// Bytes as Rust's `Debug` prints a slice of them.
template <class B>
std::string bytes(const B &bytes) {
	return list(bytes, [](std::uint8_t b) { return std::to_string(b); });
}

template <class B>
std::string bound(const B &bound) {
	switch (bound.kind) {
	case API_BOUND_INCLUDED:
		return "Included(" + std::to_string(bound.value) + ")";
	case API_BOUND_EXCLUDED:
		return "Excluded(" + std::to_string(bound.value) + ")";
	default:
		return "Unbounded";
	}
}

// `items.map(show).collect::<Vec<_>>().join(sep)`.
template <class R, class F>
std::string join(R &&items, const char *sep, F show) {
	std::string out;
	bool first = true;
	for (auto &&item : items) {
		out += first ? "" : sep;
		out += show(item);
		first = false;
	}
	return out;
}

// A vector of `items`, each moved in, which a braced list cannot.
template <class T, class... A>
std::vector<T> vec(A &&...items) {
	std::vector<T> out;
	out.reserve(sizeof...(items));
	(out.emplace_back(std::forward<A>(items)), ...);
	return out;
}

template <class T>
T some(std::optional<T> &&value) {
	if (!value) {
		fail("called `Option::unwrap()` on a `None` value");
	}
	return std::move(*value);
}

template <class T>
const T &some(const std::optional<T> &value) {
	if (!value) {
		fail("called `Option::unwrap()` on a `None` value");
	}
	return *value;
}

// An object a closure is lent: itself, or what a borrow lends.
template <class T>
T &lent(T &object) {
	return object;
}

template <class T>
T &lent(api::borrow<T> &object) {
	return *object.operator->();
}

template <class T>
T &lent(const api::borrow<T> &object) {
	return *object.operator->();
}

template <class T>
const T *ptr(const T &value) {
	return &value;
}

inline api::socket_addr socket(api::socket_addr_v4 v4) {
	api::socket_addr out{};
	out.addr.v4 = v4;
	return out;
}

inline api::socket_addr socket(api::socket_addr_v6 v6) {
	api::socket_addr out{};
	out.v6 = true;
	out.addr.v6 = v6;
	return out;
}

inline api::ip_addr ip(api::ipv4_addr v4) {
	api::ip_addr out{};
	out.ip.v4 = v4;
	return out;
}

inline api::ip_addr ip(api::ipv6_addr v6) {
	api::ip_addr out{};
	out.v6 = true;
	out.ip.v6 = v6;
	return out;
}

inline double f64(std::uint64_t bits) {
	double out;
	std::memcpy(&out, &bits, sizeof out);
	return out;
}

inline float f32(std::uint32_t bits) {
	float out;
	std::memcpy(&out, &bits, sizeof out);
	return out;
}

} // namespace rs
"#;

/// The C++ header of `t`'s script.
pub(crate) fn emit(t: &Transcript) -> String {
	let mut used = BTreeSet::new();
	for step in &t.steps {
		each_expr(step, &mut |e| {
			walk(e, &mut |e| {
				if let Kind::Var(name) = &e.kind {
					used.insert(name.clone());
				}
			})
		});
	}
	let mut e = Emitter {
		t,
		depth: 1,
		fresh: 0,
		level: 0,
		owned: Vec::new(),
		used,
	};
	let body = e.steps(&t.steps);
	let body = resolve(&body);
	format!("{PRELUDE}\nnamespace transcript {{\n\ninline void script() {{\n{body}}}\n\n}} // namespace transcript\n")
}

/// How a local is used where it appears, marked in the output until the last use of each
/// is known.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Use {
	/// Read in place.
	Read,
	/// Moved at its last use, and copied before it.
	Move,
	/// Moved at its last use, and cloned before it, as C++ has no copy.
	Clone,
	/// Cloned: a local a closure takes by value from outside it, which may run again.
	Cloned,
	/// Moved, as `drop` does.
	Drop,
}

impl Use {
	fn code(self) -> char {
		match self {
			Use::Read => 'R',
			Use::Move => 'M',
			Use::Clone => 'C',
			Use::Cloned => 'K',
			Use::Drop => 'D',
		}
	}
}

/// A local marked as used: resolved once the whole script is written.
fn marker(name: &str, how: Use) -> String {
	format!("\u{1}{}{}\u{2}", how.code(), ident(name))
}

/// `body` with each marked local resolved: moved at its last use, which C++ reads in text
/// order, as no closure the script defines runs after a later statement.
fn resolve(body: &str) -> String {
	let markers = || {
		let mut at = 0;
		std::iter::from_fn(move || {
			let start = at + body[at..].find('\u{1}')?;
			let end = start + body[start..].find('\u{2}').unwrap();
			at = end + 1;
			Some((start, end))
		})
	};
	let mut last = std::collections::BTreeMap::new();
	for (start, end) in markers() {
		last.insert(&body[start + 2..end], start);
	}
	let mut out = String::new();
	let mut at = 0;
	for (start, end) in markers() {
		out.push_str(&body[at..start]);
		let name = &body[start + 2..end];
		let is_last = last[name] == start;
		let resolved = match (&body[start + 1..start + 2], is_last) {
			("R", _) | ("M", false) => name.to_string(),
			("M", true) | ("C", true) | ("D", _) => format!("std::move({name})"),
			("C", false) | ("K", _) => format!("{name}.clone()"),
			(code, _) => unreachable!("a marker {code}"),
		};
		out.push_str(&resolved);
		at = end + 1;
	}
	out.push_str(&body[at..]);
	out
}

/// Where a value goes: what it may be written as, and whether a local there is moved.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Place {
	/// An argument of a call, of the type its parameter takes, which C++ converts to.
	Arg,
	/// A value of its own: a local, or what a closure returns.
	Own,
	/// A value of its own inside another, of the type its field or element takes, which C++
	/// converts to.
	Element,
	/// Read in place, such as an operand or what a line prints.
	Read,
}

struct Emitter<'t> {
	t: &'t Transcript,
	depth: usize,
	fresh: usize,
	/// How many closures deep the script is.
	level: usize,
	/// Each local the script owns, and how many closures deep it is: a closure deeper
	/// never moves it, as it may run again.
	owned: Vec<(String, usize)>,
	/// Each local some expression reads; any other is declared `[[maybe_unused]]`.
	used: BTreeSet<String>,
}

/// C++'s keywords, which no name may be.
const KEYWORDS: &[&str] = &[
	"alignas",
	"alignof",
	"and",
	"and_eq",
	"asm",
	"auto",
	"bitand",
	"bitor",
	"bool",
	"break",
	"case",
	"catch",
	"char",
	"char8_t",
	"char16_t",
	"char32_t",
	"class",
	"compl",
	"concept",
	"const",
	"consteval",
	"constexpr",
	"constinit",
	"const_cast",
	"continue",
	"co_await",
	"co_return",
	"co_yield",
	"decltype",
	"default",
	"delete",
	"do",
	"double",
	"dynamic_cast",
	"else",
	"enum",
	"explicit",
	"export",
	"extern",
	"false",
	"float",
	"for",
	"friend",
	"goto",
	"if",
	"inline",
	"int",
	"long",
	"mutable",
	"namespace",
	"new",
	"noexcept",
	"not",
	"not_eq",
	"nullptr",
	"operator",
	"or",
	"or_eq",
	"private",
	"protected",
	"public",
	"register",
	"reinterpret_cast",
	"requires",
	"return",
	"short",
	"signed",
	"sizeof",
	"static",
	"static_assert",
	"static_cast",
	"struct",
	"switch",
	"template",
	"this",
	"thread_local",
	"throw",
	"true",
	"try",
	"typedef",
	"typeid",
	"typename",
	"union",
	"unsigned",
	"using",
	"virtual",
	"void",
	"volatile",
	"wchar_t",
	"while",
	"xor",
	"xor_eq",
	// What the script names unqualified, and the standard library's macros.
	"api",
	"std",
	"rs",
	"hook",
	"transcript",
	"assert",
	"errno",
	"stdin",
	"stdout",
	"stderr",
	"major",
	"minor",
	"unix",
	"linux",
	"NULL",
	"EOF",
];

/// A Rust name in C++, as the generated header escapes one.
fn ident(name: &str) -> String {
	match name.chars().next() {
		Some(c) if c.is_ascii_digit() => format!("_{name}"),
		_ if KEYWORDS.contains(&name) => format!("{name}_"),
		_ => name.to_string(),
	}
}

/// A method named after a trait's, `_` apart from its class of the same name.
fn apart(bare: &str, class: &str) -> String {
	match bare == class {
		true => format!("{bare}_"),
		false => bare.to_string(),
	}
}

/// A Rust identifier in snake case, as the generated header names a variant.
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

fn cpp_int(int: Int) -> &'static str {
	match int {
		Int::U8 => "std::uint8_t",
		Int::U16 => "std::uint16_t",
		Int::U32 => "std::uint32_t",
		Int::U64 => "std::uint64_t",
		Int::Usize => "std::size_t",
		Int::I8 => "std::int8_t",
		Int::I16 => "std::int16_t",
		Int::I32 => "std::int32_t",
		Int::I64 => "std::int64_t",
		Int::Isize => "std::ptrdiff_t",
	}
}

/// An integer literal any C++ integer converts from exactly.
fn int_literal(n: super::Num) -> String {
	let m = n.magnitude;
	match (n.negative, m) {
		(false, m) if m <= i32::MAX as u128 => m.to_string(),
		(false, m) if m <= i64::MAX as u128 => format!("{m}LL"),
		(false, m) => format!("{m}ULL"),
		(true, m) if m <= i32::MAX as u128 => format!("-{m}"),
		(true, m) if m == 1 << 63 => "(-9223372036854775807LL - 1)".to_string(),
		(true, m) => format!("-{m}LL"),
	}
}

impl Emitter<'_> {
	fn indent(&self) -> String {
		"\t".repeat(self.depth)
	}

	fn fresh(&mut self, what: &str) -> String {
		self.fresh += 1;
		format!("{what}{}_", self.fresh)
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

	fn step(&mut self, step: &Step, out: &mut String) {
		match step {
			Step::Let(Pat::Tuple(pats), value) if matches!(&value.kind, Kind::Tuple(_)) && is_tuple(&value.ty) => {
				// A tuple built to be taken apart: each element bound in order.
				let Kind::Tuple(elements) = &value.kind else {
					unreachable!()
				};
				for (pat, element) in pats.iter().zip(elements) {
					self.step(&Step::Let(pat.clone(), element.clone()), out);
				}
			}
			Step::Let(pat, value) => {
				let v = self.value(value, &value.ty, Place::Own);
				self.bind(pat, &v, &value.ty, out);
			}
			Step::Print(pieces) => {
				let pieces = self.pieces(pieces);
				self.line(out, &format!("rs::line({{{pieces}}});"));
			}
			Step::Do(value) => {
				let v = self.value(value, &value.ty, Place::Read);
				match unref(&value.ty) {
					Type::Unit => self.line(out, &format!("{v};")),
					_ => self.line(out, &format!("(void){v};")),
				}
			}
			Step::Drop(name) => {
				let gone = self.fresh("dropped");
				self.line(
					out,
					&format!("{{ [[maybe_unused]] auto {gone} = {}; }}", marker(name, Use::Drop)),
				);
			}
			Step::Match { value, ok, err } => {
				let v = self.value(value, &value.ty, Place::Own);
				let tmp = self.fresh("matched");
				self.line(out, "{");
				self.depth += 1;
				self.line(out, &format!("auto {tmp} = {v};"));
				let (ok_value, ok_ty, err_value, err_ty) = match &value.ty {
					Type::Option(inner) => (
						format!("std::move(*{tmp})"),
						(**inner).clone(),
						String::new(),
						Type::Unit,
					),
					Type::Result(ok, err) => (
						format!("std::move(*{tmp})"),
						(**ok).clone(),
						format!("std::move({tmp}.error())"),
						(**err).clone(),
					),
					ty => panic!("a match on {ty:?}"),
				};
				self.line(out, &format!("if ({tmp}) {{"));
				self.depth += 1;
				if let Some(pat) = &ok.pat {
					self.bind(pat, &ok_value, &ok_ty, out);
				}
				let body = self.steps(&ok.steps);
				out.push_str(&body);
				self.depth -= 1;
				self.line(out, "} else {");
				self.depth += 1;
				if let Some(pat) = &err.pat {
					self.bind(pat, &err_value, &err_ty, out);
				}
				let body = self.steps(&err.steps);
				out.push_str(&body);
				self.depth -= 1;
				self.line(out, "}");
				self.depth -= 1;
				self.line(out, "}");
			}
		}
	}

	/// `[[maybe_unused]] ` if nothing reads `name`.
	fn unused(&self, name: &str) -> &'static str {
		match self.used.contains(name) {
			true => "",
			false => "[[maybe_unused]] ",
		}
	}

	/// `pat` bound to the value `v` of type `ty`, each name a local the script owns.
	fn bind(&mut self, pat: &Pat, v: &str, ty: &Type, out: &mut String) {
		match pat {
			Pat::Name(name, _) => {
				self.owned.push((name.clone(), self.level));
				self.line(out, &format!("{}auto {} = {v};", self.unused(name), ident(name)));
			}
			Pat::Ignore(_) => self.line(out, &format!("(void){v};")),
			Pat::Tuple(pats) => {
				let tmp = self.fresh("tuple");
				self.line(out, &format!("auto {tmp} = {v};"));
				let base = unref(ty).base();
				for (i, (pat, ty)) in pats.iter().zip(elements(base)).enumerate() {
					let element = match is_tuple(base) {
						true => format!("std::get<{i}>(std::move({tmp}))"),
						false => member(&tmp, i),
					};
					self.bind(pat, &element, &ty, out);
				}
			}
		}
	}

	/// `pat` bound to the value `v` of type `ty` by reference, each name a closure's.
	fn bind_ref(&mut self, pat: &Pat, v: &str, ty: &Type, out: &mut String) {
		match pat {
			Pat::Name(name, _) => {
				self.line(out, &format!("{}auto &&{} = {v};", self.unused(name), ident(name)));
			}
			Pat::Ignore(_) => {}
			Pat::Tuple(pats) => {
				let tmp = self.fresh("tuple");
				self.line(out, &format!("auto &&{tmp} = {v};"));
				let tys = elements(ty);
				for (i, pat) in pats.iter().enumerate() {
					let element = match is_tuple(unref(ty).base()) {
						true => format!("std::get<{i}>({tmp})"),
						false => member(&tmp, i),
					};
					self.bind_ref(pat, &element, &tys[i], out);
				}
			}
		}
	}

	/// The C++ of a format's pieces: the items of a braced list of strings.
	fn pieces(&mut self, pieces: &[Piece]) -> String {
		let mut parts = Vec::new();
		for piece in pieces {
			match piece {
				Piece::Text(text) => parts.push(string(text)),
				Piece::Value(e, spec) => {
					let v = self.value(e, &e.ty, Place::Read);
					parts.push(self.show(&v, &e.ty, *spec));
				}
			}
		}
		parts.join(", ")
	}

	/// `v`, a value of `ty`, as a `std::string` the way Rust formats it with `spec`.
	fn show(&mut self, v: &str, ty: &Type, spec: Spec) -> String {
		let ty = unref(ty).base();
		match spec {
			Spec::Hex(width) => return format!("rs::hex({v}, {width})"),
			Spec::Padded(width) => return format!("rs::padded({v}, {width})"),
			Spec::Debug => return self.debug(v, ty),
			Spec::Display => {}
		}
		match ty {
			Type::Bool => format!("rs::flag({v})"),
			Type::String | Type::Str => format!("rs::text({v})"),
			Type::Int(_) | Type::NonZero(_) => format!("std::to_string({v})"),
			Type::Object(i) => {
				let o = &self.t.api.objects[*i];
				let name = self.t.type_name(LANGUAGE, ty);
				match o.methods.iter().position(|m| m.access == Access::Text) {
					Some(index) => format!(
						"rs::text({v}.{}())",
						self.t.name(
							LANGUAGE,
							FnId {
								owner: Owner::Object(*i),
								index
							}
						)
					),
					None if o.traits.display => format!("rs::text({v}.{}())", apart("to_string", name)),
					None => panic!("Display of {name}, which has none"),
				}
			}
			Type::Record(i) if self.t.api.records[*i].traits.display => {
				format!("rs::text({v}.{}())", apart("to_string", self.t.type_name(LANGUAGE, ty)))
			}
			Type::Enum(i) if self.t.api.enums[*i].traits.display => {
				format!("rs::text(api::{}_to_string({v}))", self.t.type_name(LANGUAGE, ty))
			}
			ty => panic!("Display of {ty:?}"),
		}
	}

	/// `v` as Rust's `Debug` prints a value of `ty`.
	fn debug(&mut self, v: &str, ty: &Type) -> String {
		match unref(ty).base() {
			Type::Bool => format!("rs::flag({v})"),
			Type::Int(_) | Type::NonZero(_) => format!("std::to_string({v})"),
			Type::String | Type::Str => format!("rs::quoted({v})"),
			Type::Option(inner) => {
				let x = self.fresh("x");
				let show = self.debug(&x, inner);
				format!("rs::option({v}, [&](const auto &{x}) {{ return {show}; }})")
			}
			Type::Vec | Type::Slice | Type::Bytes | Type::Array(_) => format!("rs::bytes({v})"),
			Type::List(_, item) => {
				let x = self.fresh("x");
				let show = self.debug(&x, item);
				format!("rs::list({v}, [&](const auto &{x}) {{ return {show}; }})")
			}
			Type::Tuple(elements) => {
				let x = self.fresh("x");
				let parts: Vec<String> = (0..elements.len())
					.map(|i| self.debug(&format!("std::get<{i}>({x})"), &elements[i]))
					.collect();
				let tail = if parts.len() == 1 { ",)" } else { ")" };
				format!(
					"[&](const auto &{x}) {{ return rs::cat({{\"(\", {}, \"{tail}\"}}); }}({v})",
					parts.join(", \", \", ")
				)
			}
			Type::Bound(_) => format!("rs::bound({v})"),
			ty @ (Type::Record(_) | Type::Object(_)) => {
				format!("rs::text({v}.{}())", apart("debug", self.t.type_name(LANGUAGE, ty)))
			}
			ty @ Type::Enum(_) => format!("rs::text(api::{}_debug({v}))", self.t.type_name(LANGUAGE, ty)),
			ty => panic!("Debug of {ty:?}"),
		}
	}

	/// The C++ type of a value of `ty`, owning what it holds, as the header returns one.
	fn ctype(&self, ty: &Type) -> String {
		match ty {
			Type::Unit => "void".to_string(),
			Type::Bool => "bool".to_string(),
			Type::Int(int) | Type::NonZero(int) => cpp_int(*int).to_string(),
			Type::F32 => "float".to_string(),
			Type::F64 => "double".to_string(),
			Type::Char => "char32_t".to_string(),
			Type::Str | Type::String | Type::Slice | Type::Vec | Type::Bytes | Type::Path | Type::PathBuf => {
				"api::buf".to_string()
			}
			Type::Array(n) => format!("std::array<std::uint8_t, {n}>"),
			Type::U128 => "api::u128".to_string(),
			Type::Duration => "api::duration".to_string(),
			Type::SocketAddr(family) => format!("api::{}", addr(true, *family)),
			Type::IpAddr(family) => format!("api::{}", addr(false, *family)),
			Type::List(_, inner) => format!("std::vector<{}>", self.ctype(inner)),
			Type::Option(inner) => format!("std::optional<{}>", self.ctype(inner)),
			Type::Box(inner) | Type::Ref(_, inner) => self.ctype(inner),
			ty @ (Type::Record(_) | Type::Enum(_) | Type::Object(_)) => {
				format!("api::{}", self.t.type_name(LANGUAGE, ty))
			}
			Type::Range(int) => format!("api::range_{}", int.rust()),
			Type::Bound(int) => format!("api::bound_{}", int.rust()),
			Type::Bounds(int) => format!("api::bounds_{}", int.rust()),
			Type::Tuple(elements) => {
				let elements: Vec<String> = elements.iter().map(|e| self.ctype(e)).collect();
				format!("std::tuple<{}>", elements.join(", "))
			}
			ty => panic!("no C++ type of {ty:?}"),
		}
	}

	/// Whether C++ copies a value of `ty`, as it does every value but a handle's.
	fn copyable(&self, ty: &Type) -> bool {
		self.copyable_in(ty, &mut Vec::new())
	}

	fn copyable_in(&self, ty: &Type, seen: &mut Vec<Type>) -> bool {
		if seen.contains(ty) {
			return true;
		}
		seen.push(ty.clone());
		let api = &self.t.api;
		match ty {
			Type::Object(i) => api.objects[*i].shared,
			Type::Record(i) => {
				api.records[*i].opaque.is_some() || api.records[*i].fields.iter().all(|f| self.copyable_in(&f.ty, seen))
			}
			Type::Enum(i) => api.enums[*i]
				.variants
				.iter()
				.flat_map(|v| &v.fields)
				.all(|f| self.copyable_in(&f.ty, seen)),
			Type::Callback(..) | Type::Closure(_) | Type::Reader | Type::Future => false,
			Type::Option(inner) | Type::Box(inner) | Type::List(_, inner) => self.copyable_in(inner, seen),
			Type::Ref(_, inner) => self.copyable_in(inner, seen),
			Type::Tuple(elements) => elements.iter().all(|e| self.copyable_in(e, seen)),
			_ => true,
		}
	}

	/// Whether the parameter type `ty` is a pointer in C++, null for `None`.
	fn pointer(&self, ty: &Type) -> bool {
		match ty {
			Type::Option(inner) => match &**inner {
				Type::Ref(_, object) => matches!(**object, Type::Object(_)),
				Type::Object(i) => self.t.api.objects[*i].copy,
				_ => false,
			},
			Type::Ref(_, inner) | Type::Box(inner) => self.pointer(inner),
			_ => false,
		}
	}

	/// Whether a call reads a local passed for the parameter type `ty` in place, rather than
	/// taking it.
	fn reads(&self, ty: &Type) -> bool {
		match ty {
			Type::Ref(..) => true,
			// A `Copy` handle, which C++ takes by reference and copies out of.
			Type::Object(i) => self.t.api.objects[*i].copy,
			_ => false,
		}
	}

	/// A local `name` of type `ty`, used as `place` takes it.
	fn var(&self, name: &str, ty: &Type, place: Place, want: &Type) -> String {
		let read = match place {
			Place::Read => true,
			Place::Arg => self.reads(want),
			Place::Own | Place::Element => false,
		};
		if read {
			return marker(name, Use::Read);
		}
		let owned = self
			.owned
			.iter()
			.rev()
			.find(|(n, _)| n == name)
			.is_some_and(|(_, level)| *level == self.level);
		let copyable = self.copyable(unref(ty));
		let handle = matches!(unref(ty).base(), Type::Object(_));
		let how = match (owned, copyable, handle) {
			(true, true, _) => Use::Move,
			(true, false, true) => Use::Clone,
			(true, false, false) => Use::Move,
			(false, true, _) => Use::Read,
			(false, false, true) => Use::Cloned,
			(false, false, false) => panic!("a copy of {name}, whose {ty:?} C++ cannot copy"),
		};
		marker(name, how)
	}

	/// The C++ of `e` as `place` takes it, where a value of type `want` goes.
	fn value(&mut self, e: &Expr, want: &Type, place: Place) -> String {
		// An address of one family where either is taken, as `.into()` makes one.
		match (unref(&e.ty).base(), unref(want).base()) {
			(Type::SocketAddr(Family::V4 | Family::V6), Type::SocketAddr(Family::Any))
				if !matches!(e.kind, Kind::Socket(_)) =>
			{
				return format!("rs::socket({})", self.kind(e, &e.ty, place));
			}
			(Type::IpAddr(Family::V4 | Family::V6), Type::IpAddr(Family::Any)) if !matches!(e.kind, Kind::Ip(_)) => {
				return format!("rs::ip({})", self.kind(e, &e.ty, place));
			}
			_ => {}
		}
		self.kind(e, want, place)
	}

	fn kind(&mut self, e: &Expr, want: &Type, place: Place) -> String {
		match &e.kind {
			Kind::Unit => panic!("a unit value"),
			Kind::Bool(b) => b.to_string(),
			Kind::Int(n) => match unref(want).base() {
				Type::U128 => {
					let bits = match n.negative {
						true => panic!("a negative u128"),
						false => n.magnitude,
					};
					format!("api::u128{{{}ULL, {}ULL}}", bits as u64, (bits >> 64) as u64)
				}
				Type::Int(int) | Type::NonZero(int) if place == Place::Own => {
					format!("{}{{{}}}", cpp_int(*int), int_literal(*n))
				}
				_ => int_literal(*n),
			},
			Kind::Float(f) => match unref(want).base() {
				Type::F32 => format!("rs::f32({:#x})", (*f as f32).to_bits()),
				_ if !f.is_finite() || (*f == 0.0 && f.is_sign_negative()) => format!("rs::f64({:#x}ULL)", f.to_bits()),
				_ => format!("{f:?}"),
			},
			Kind::Char(c) => format!("char32_t{{{:#x}}}", *c as u32),
			Kind::Str(s) => match place {
				Place::Arg if !s.contains('\0') => string(s),
				_ => format!("api::buf(api::view({}, {}))", string(s), s.len()),
			},
			Kind::Bytes(b) => match unref(want).base() {
				Type::Array(n) => {
					let items: Vec<String> = b.iter().map(|b| b.to_string()).collect();
					format!("std::array<std::uint8_t, {n}>{{{{{}}}}}", items.join(", "))
				}
				_ if place == Place::Arg => format!("api::view({}, {})", bytes(b), b.len()),
				_ => format!("api::buf(api::view({}, {}))", bytes(b), b.len()),
			},
			Kind::Duration(secs, nanos) => format!(
				"api::duration{{{}, {nanos}}}",
				int_literal(super::Num::new(*secs as i128))
			),
			Kind::Socket(addr) => {
				let family = match unref(want).base() {
					Type::SocketAddr(family) => *family,
					ty => panic!("a socket address as {ty:?}"),
				};
				let inner = match addr {
					std::net::SocketAddr::V4(a) => {
						format!("api::socket_addr_v4{{{}, {}}}", ipv4(a.ip()), a.port())
					}
					std::net::SocketAddr::V6(a) => format!(
						"api::socket_addr_v6{{{}, {}, {}, {}}}",
						ipv6(a.ip()),
						a.port(),
						int_literal(super::Num::new(a.flowinfo() as i128)),
						int_literal(super::Num::new(a.scope_id() as i128))
					),
				};
				match (family, addr) {
					(Family::Any, _) => format!("rs::socket({inner})"),
					(Family::V4, std::net::SocketAddr::V4(_)) | (Family::V6, std::net::SocketAddr::V6(_)) => inner,
					_ => panic!("a socket address of another family than {family:?}"),
				}
			}
			Kind::Ip(ip) => {
				let family = match unref(want).base() {
					Type::IpAddr(family) => *family,
					ty => panic!("an IP address as {ty:?}"),
				};
				let inner = match ip {
					std::net::IpAddr::V4(ip) => ipv4(ip),
					std::net::IpAddr::V6(ip) => ipv6(ip),
				};
				match (family, ip) {
					(Family::Any, _) => format!("rs::ip({inner})"),
					(Family::V4, std::net::IpAddr::V4(_)) | (Family::V6, std::net::IpAddr::V6(_)) => inner,
					_ => panic!("an IP address of another family than {family:?}"),
				}
			}
			Kind::None => match place {
				Place::Arg if self.pointer(want) => "nullptr".to_string(),
				Place::Arg => "std::nullopt".to_string(),
				_ => format!("{}{{}}", self.ctype(&option_of(want, &e.ty))),
			},
			Kind::Some(inner) => {
				let option = option_of(want, &e.ty);
				let Type::Option(inner_want) = &option else {
					unreachable!()
				};
				if place == Place::Arg && self.pointer(want) {
					return match &inner.kind {
						Kind::Var(name) => format!("&{}", marker(name, Use::Read)),
						_ => format!("rs::ptr({})", self.value(inner, inner_want, Place::Read)),
					};
				}
				// An argument converts to the optional its parameter takes, unless what it holds
				// is optional too.
				match (place, unref(inner_want).base()) {
					(Place::Arg, Type::Option(_)) | (Place::Own | Place::Element | Place::Read, _) => {
						let v = self.value(inner, inner_want, Place::Element);
						format!("{}({v})", self.ctype(&option))
					}
					(Place::Arg, _) => self.value(inner, inner_want, Place::Arg),
				}
			}
			Kind::Tuple(elements) => match unref(&e.ty).base() {
				Type::Range(int) => {
					let (a, b) = (
						self.value(&elements[0], &Type::Int(*int), Place::Arg),
						self.value(&elements[1], &Type::Int(*int), Place::Arg),
					);
					format!("api::range_{}{{{a}, {b}}}", int.rust())
				}
				Type::Bounds(int) => {
					let (a, b) = (
						self.value(&elements[0], &Type::Bound(*int), Place::Arg),
						self.value(&elements[1], &Type::Bound(*int), Place::Arg),
					);
					format!("api::bounds_{}{{{a}, {b}}}", int.rust())
				}
				_ => {
					let tuple = match unref(want).base() {
						ty @ Type::Tuple(_) => ty.clone(),
						_ => unref(&e.ty).base().clone(),
					};
					let wants = elements_of(&tuple);
					let parts: Vec<String> = elements
						.iter()
						.zip(&wants)
						.map(|(e, t)| self.value(e, t, Place::Element))
						.collect();
					format!("{}({})", self.ctype(&tuple), parts.join(", "))
				}
			},
			Kind::List(items) => {
				let item = match unref(want).base() {
					Type::List(_, item) => (**item).clone(),
					_ => match unref(&e.ty).base() {
						Type::List(_, item) => (**item).clone(),
						ty => panic!("a list as {ty:?}"),
					},
				};
				let ty = self.ctype(&item);
				match items.is_empty() {
					true => format!("std::vector<{ty}>{{}}"),
					false => {
						let parts: Vec<String> = items.iter().map(|e| self.value(e, &item, Place::Element)).collect();
						format!("rs::vec<{ty}>({})", parts.join(", "))
					}
				}
			}
			Kind::Record(r, fields) => {
				let ty = Type::Record(*r);
				if self.t.api.records[*r].opaque.is_some() {
					panic!(
						"a literal of {}, which C++ makes only by its functions",
						self.t.type_name(LANGUAGE, &ty)
					);
				}
				let name = self.t.type_name(LANGUAGE, &ty).to_string();
				let field_types: Vec<Type> = self.t.api.records[*r].fields.iter().map(|f| f.ty.clone()).collect();
				let args: Vec<String> = fields
					.iter()
					.zip(&field_types)
					.map(|(f, ty)| self.value(f, ty, Place::Element))
					.collect();
				format!("api::{name}{{{}}}", args.join(", "))
			}
			Kind::Variant(en, v, fields) => {
				let name = self.t.type_name(LANGUAGE, &Type::Enum(*en)).to_string();
				let enumeration = &self.t.api.enums[*en];
				let variant = &enumeration.variants[*v];
				if enumeration.variants.iter().all(|v| v.fields.is_empty()) {
					return format!("api::{name}::{}", ident(&snake(&variant.name)));
				}
				let field_types: Vec<Type> = variant.fields.iter().map(|f| f.ty.clone()).collect();
				let variant_name = variant.name.clone();
				let args: Vec<String> = fields
					.iter()
					.zip(&field_types)
					.map(|(f, ty)| self.value(f, ty, Place::Element))
					.collect();
				format!("api::{name}_{variant_name}{{{}}}", args.join(", "))
			}
			Kind::Bound(kind, value) => {
				let Type::Bound(int) = unref(&e.ty) else {
					panic!("a bound of {:?}", e.ty)
				};
				let (kind, v) = match (kind, value) {
					(BoundKind::Unbounded, _) => ("UNBOUNDED", "0".to_string()),
					(BoundKind::Included, Some(v)) => ("INCLUDED", self.value(v, &Type::Int(*int), Place::Arg)),
					(BoundKind::Excluded, Some(v)) => ("EXCLUDED", self.value(v, &Type::Int(*int), Place::Arg)),
					_ => unreachable!(),
				};
				format!("api::bound_{}{{API_BOUND_{kind}, {v}}}", int.rust())
			}
			Kind::Default => {
				let ty = unref(&e.ty).base();
				let name = self.t.type_name(LANGUAGE, ty).to_string();
				match ty {
					Type::Enum(_) => format!("api::{name}_default()"),
					_ => format!("api::{name}::{}()", apart("default_", &name)),
				}
			}
			Kind::Var(name) => self.var(name, &e.ty, place, want),
			Kind::Const(c) => format!(
				"api::{}",
				self.t
					.named(LANGUAGE, &format!("const {}", self.t.api.constants[*c].path.join("::")))
			),
			Kind::Call(id, receiver, args) => self.call(*id, receiver.as_deref(), args),
			Kind::Field(base, i) => {
				let b = self.base(base, place);
				let Type::Record(r) = unref(&base.ty).base() else {
					panic!("a field of {:?}", base.ty)
				};
				format!("{b}.{}", ident(&self.t.api.records[*r].fields[*i].name))
			}
			Kind::Element(base, i) => {
				let b = self.base(base, place);
				match is_tuple(unref(&base.ty).base()) {
					true => format!("std::get<{i}>({b})"),
					false => member(&b, *i),
				}
			}
			Kind::Clone(inner) => {
				let v = self.value(inner, &inner.ty, Place::Read);
				match unref(&inner.ty).base() {
					// What a call returned is a value of its own already.
					ty if !self.copyable(ty) && matches!(inner.kind, Kind::Call(..) | Kind::Local(..)) => v,
					ty => {
						let ty = ty.clone();
						self.cloned(&v, &ty)
					}
				}
			}
			Kind::Unwrap(inner) => {
				let v = self.unwrapped(inner, place);
				match &inner.ty {
					Type::Result(..) => format!("{v}.value()"),
					_ => format!("rs::some({v})"),
				}
			}
			Kind::UnwrapErr(inner) => {
				let v = self.unwrapped(inner, place);
				format!("{v}.error()")
			}
			Kind::IsErr(inner) | Kind::IsNone(inner) => {
				format!("!{}.has_value()", self.value(inner, &inner.ty, Place::Read))
			}
			Kind::IsSome(inner) => format!("{}.has_value()", self.value(inner, &inner.ty, Place::Read)),
			Kind::Binary(op, a, b) => {
				let (a, b) = (self.value(a, &a.ty, Place::Read), self.value(b, &b.ty, Place::Read));
				let (op, arithmetic) = match op {
					Op::Eq => ("==", false),
					Op::Ne => ("!=", false),
					Op::Lt => ("<", false),
					Op::Gt => (">", false),
					Op::And => ("&&", false),
					Op::Add => ("+", true),
					Op::Sub => ("-", true),
					Op::Mul => ("*", true),
					Op::Rem => ("%", true),
				};
				match arithmetic {
					true => format!("static_cast<{}>({a} {op} {b})", self.ctype(unref(&e.ty))),
					false => format!("({a} {op} {b})"),
				}
			}
			Kind::Closure(params, steps, value) => self.closure(params, steps, value),
			Kind::Local(name, args) => match unref(&e.ty).base() {
				// A unit struct implementing a callback, owned by Rust once passed.
				Type::Callback(..) if args.is_empty() => format!("std::make_unique<hook::{name}>()"),
				_ => {
					let args: Vec<String> = args.iter().map(|a| self.value(a, &a.ty, Place::Arg)).collect();
					format!("hook::{name}({})", args.join(", "))
				}
			},
			Kind::Format(pieces) => format!("rs::cat({{{}}})", self.pieces(pieces)),
			Kind::Join { items, pat, body, sep } => {
				let items_v = self.value(items, &items.ty, Place::Read);
				let item_ty = match unref(&items.ty).base() {
					Type::List(_, item) => (**item).clone(),
					ty => panic!("a join over {ty:?}"),
				};
				let lambda = self.lambda(&[(pat.clone(), item_ty)], &[], body, true);
				format!("rs::join({items_v}, {}, {lambda})", string(sep))
			}
			Kind::MapOr {
				option,
				none,
				pat,
				some,
			} => {
				let o = self.value(option, &option.ty, Place::Read);
				let Type::Option(inner) = unref(&option.ty).base() else {
					panic!("map_or of {:?}", option.ty)
				};
				let inner = (**inner).clone();
				let text = matches!(unref(&e.ty).base(), Type::String | Type::Str);
				let normal = |v: String| match text {
					true => format!("rs::text({v})"),
					false => v,
				};
				let none_v = normal(self.value(none, &e.ty, Place::Own));
				let tmp = self.fresh("option");
				self.depth += 1;
				self.level += 1;
				let mut body = String::new();
				self.line(&mut body, &format!("auto &&{tmp} = {o};"));
				self.line(&mut body, &format!("if (!{tmp}) {{"));
				self.depth += 1;
				self.line(&mut body, &format!("return {none_v};"));
				self.depth -= 1;
				self.line(&mut body, "}");
				self.bind_ref(pat, &format!("*{tmp}"), &inner, &mut body);
				let some_v = normal(self.value(some, &e.ty, Place::Own));
				self.line(&mut body, &format!("return {some_v};"));
				self.level -= 1;
				self.depth -= 1;
				format!("[&]() {{\n{body}{}}}()", self.indent())
			}
			Kind::Utf8(inner) => format!("rs::text({})", self.value(inner, &inner.ty, Place::Read)),
			Kind::Len(inner) => format!("{}.size()", self.value(inner, &inner.ty, Place::Read)),
		}
	}

	/// Rust's `Clone` of `v`, a value of `ty`: a C++ copy, a handle's `clone()`, or a
	/// record's fields each cloned.
	fn cloned(&mut self, v: &str, ty: &Type) -> String {
		match ty {
			Type::Object(i) if !self.t.api.objects[*i].shared => {
				format!("{v}.{}()", apart("clone", self.t.type_name(LANGUAGE, ty)))
			}
			ty if self.copyable(ty) => v.to_string(),
			Type::Record(r) => {
				let x = self.fresh("x");
				let fields: Vec<(String, Type)> = self.t.api.records[*r]
					.fields
					.iter()
					.map(|f| (ident(&f.name), f.ty.clone()))
					.collect();
				let fields: Vec<String> = fields
					.iter()
					.map(|(name, ty)| self.cloned(&format!("{x}.{name}"), ty))
					.collect();
				format!(
					"[&](const auto &{x}) {{ return {}{{{}}}; }}({v})",
					self.ctype(ty),
					fields.join(", ")
				)
			}
			ty => panic!("a clone of {ty:?}, which C++ cannot copy"),
		}
	}

	/// The base of a field or an element: a local moves only where its field moves out.
	fn base(&mut self, base: &Expr, place: Place) -> String {
		match (&base.kind, place) {
			(Kind::Var(name), Place::Own | Place::Element) => {
				let owned = self
					.owned
					.iter()
					.rev()
					.find(|(n, _)| n == name)
					.is_some_and(|(_, level)| *level == self.level);
				marker(name, if owned { Use::Move } else { Use::Read })
			}
			_ => self.value(base, &base.ty, Place::Read),
		}
	}

	/// What an `unwrap` takes apart: a local moves out where it is the value's own.
	fn unwrapped(&mut self, inner: &Expr, place: Place) -> String {
		match place {
			Place::Read => self.value(inner, &inner.ty, Place::Read),
			_ => self.base(inner, Place::Own),
		}
	}

	/// A closure: a lambda taking each parameter by reference, which runs `steps`, then
	/// returns `value`.
	fn closure(&mut self, params: &[Pat], steps: &[super::Step], value: &Expr) -> String {
		let params: Vec<(Pat, Type)> = params.iter().map(|p| (p.clone(), pat_type(p))).collect();
		self.lambda(&params, steps, value, false)
	}

	/// A lambda of `params`, running `steps`, then returning `value`, as a string if `text`.
	fn lambda(&mut self, params: &[(Pat, Type)], steps: &[Step], value: &Expr, text: bool) -> String {
		self.level += 1;
		self.depth += 1;
		let mut body = String::new();
		let mut names = Vec::new();
		for (pat, ty) in params {
			match pat {
				// An object lent `&mut`, which C++ lends as a borrow.
				Pat::Name(name, Type::Ref(Mutability::Mut, inner)) if matches!(**inner, Type::Object(_)) => {
					let lent = self.fresh("lent");
					names.push(format!("auto &&{lent}"));
					self.line(
						&mut body,
						&format!("{}auto &{} = rs::lent({lent});", self.unused(name), ident(name)),
					);
				}
				Pat::Name(name, _) => names.push(format!("{}auto &&{}", self.unused(name), ident(name))),
				Pat::Ignore(_) => names.push("auto &&".to_string()),
				Pat::Tuple(_) => {
					let tuple = self.fresh("item");
					names.push(format!("auto &&{tuple}"));
					self.bind_ref(pat, &tuple, ty, &mut body);
				}
			}
		}
		let steps = self.steps(steps);
		body.push_str(&steps);
		let returned = match (&value.kind, text) {
			(Kind::Unit, _) => None,
			(_, true) => Some(format!("rs::text({})", self.value(value, &value.ty, Place::Own))),
			(_, false) => Some(self.value(value, &value.ty, Place::Own)),
		};
		self.depth -= 1;
		self.level -= 1;
		let params = names.join(", ");
		match (body.is_empty(), returned) {
			(true, Some(v)) if !v.contains('\n') => format!("[&]({params}) {{ return {v}; }}"),
			(_, returned) => {
				if let Some(v) = returned {
					self.depth += 1;
					self.line(&mut body, &format!("return {v};"));
					self.depth -= 1;
				}
				format!("[&]({params}) {{\n{body}{}}}", self.indent())
			}
		}
	}

	/// A call of `id`: a function of the namespace, an associated function of a type, a
	/// method, or a field's getter or setter.
	fn call(&mut self, id: FnId, receiver: Option<&Expr>, args: &[Expr]) -> String {
		let f = self.t.function(id);
		if f.future {
			panic!("a call of {}, a future, which only a hook awaits", self.t.key(id));
		}
		let name = self.t.name(LANGUAGE, id).to_string();
		let params: Vec<Type> = f.params.iter().map(|p| p.ty.clone()).collect();
		let receives = f.receiver;
		// C++ takes no unit.
		let args: Vec<String> = args
			.iter()
			.zip(&params)
			.filter(|(_, ty)| **ty != Type::Unit)
			.map(|(a, ty)| self.value(a, ty, Place::Arg))
			.collect();
		let args = args.join(", ");
		let owner = self.t.owner(id);
		match (id.owner, receiver) {
			(Owner::Free, _) => format!("api::{name}({args})"),
			// An enum's methods are functions of the namespace, taking it first.
			(Owner::Enum(_), Some(r)) => {
				let r = self.value(r, &r.ty, Place::Read);
				match args.is_empty() {
					true => format!("api::{name}({r})"),
					false => format!("api::{name}({r}, {args})"),
				}
			}
			(Owner::Enum(_), None) => format!("api::{name}({args})"),
			(_, Some(r)) => {
				// A handle's method taking `self` moves it out, unless it is `Copy`.
				let moves = match (receives, unref(&r.ty).base()) {
					(Some(Receiver::Owned), Type::Object(i)) => !self.t.api.objects[*i].copy,
					_ => false,
				};
				let r = match (&r.kind, moves) {
					(Kind::Var(name), true) => marker(name, Use::Move),
					_ => self.value(r, &r.ty, Place::Read),
				};
				format!("{r}.{name}({args})")
			}
			(_, None) => {
				let owner = owner.unwrap();
				format!("api::{}::{name}({args})", self.t.type_name(LANGUAGE, &owner))
			}
		}
	}
}

/// Whether `ty` is a tuple in C++, whose elements `std::get` reads, rather than a range's or
/// bounds' members.
fn is_tuple(ty: &Type) -> bool {
	matches!(ty, Type::Tuple(_))
}

/// The type a pattern binds.
fn pat_type(pat: &Pat) -> Type {
	match pat {
		Pat::Name(_, ty) | Pat::Ignore(ty) => ty.clone(),
		Pat::Tuple(pats) => Type::Tuple(pats.iter().map(pat_type).collect()),
	}
}

/// The `i`th member of a range or bounds.
fn member(v: &str, i: usize) -> String {
	match i {
		0 => format!("{v}.start"),
		1 => format!("{v}.end"),
		_ => panic!("a range's element {i}"),
	}
}

/// The types of the elements of a tuple, a range, or bounds.
fn elements(ty: &Type) -> Vec<Type> {
	elements_of(unref(ty).base())
}

fn elements_of(ty: &Type) -> Vec<Type> {
	match ty {
		Type::Tuple(elements) => elements.clone(),
		Type::Range(int) => vec![Type::Int(*int); 2],
		Type::Bounds(int) => vec![Type::Bound(*int); 2],
		ty => panic!("the elements of {ty:?}"),
	}
}

/// The option type of a `Some` or a `None` going where `want` is taken.
fn option_of(want: &Type, own: &Type) -> Type {
	match unref(want).base() {
		ty @ Type::Option(_) => ty.clone(),
		_ => match unref(own).base() {
			ty @ Type::Option(_) => ty.clone(),
			ty => panic!("an option as {ty:?}"),
		},
	}
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

fn ipv4(ip: &std::net::Ipv4Addr) -> String {
	let octets: Vec<String> = ip.octets().iter().map(|o| o.to_string()).collect();
	format!("api::ipv4_addr{{{{{}}}}}", octets.join(", "))
}

fn ipv6(ip: &std::net::Ipv6Addr) -> String {
	let octets: Vec<String> = ip.octets().iter().map(|o| o.to_string()).collect();
	format!("api::ipv6_addr{{{{{}}}}}", octets.join(", "))
}

/// A C++ string literal of `s`'s UTF-8, each byte past ASCII's printable ones escaped.
fn string(s: &str) -> String {
	bytes(s.as_bytes())
}

/// A C++ string literal of `b`, each byte past ASCII's printable ones in octal, which takes
/// no more than three digits.
fn bytes(b: &[u8]) -> String {
	let mut out = String::from("\"");
	for &byte in b {
		match byte {
			b'"' | b'\\' => {
				out.push('\\');
				out.push(byte as char);
			}
			b'\n' => out.push_str("\\n"),
			b'\t' => out.push_str("\\t"),
			b'\r' => out.push_str("\\r"),
			// `??` begins a trigraph before C++17.
			b'?' if out.ends_with('?') => out.push_str("\\?"),
			0x20..0x7f => out.push(byte as char),
			_ => {
				let _ = write!(out, "\\{byte:03o}");
			}
		}
	}
	out.push('"');
	out
}
