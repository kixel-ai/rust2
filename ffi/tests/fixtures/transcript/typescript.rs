//! The transcript as TypeScript: a module whose `script()` runs it through the web binding,
//! calling `tests/api.ts`'s own exports, which it imports as `hook`.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write;

use super::ir::{Access, Collection, Family, Int, Mutability, Receiver, Type};
use super::{
	Arm, BoundKind, Expr, FnId, Kind, Num, Op, Owner, Pat, Piece, Spec, Step, Transcript, bound, each_expr, unref, walk,
};

const LANGUAGE: &str = "TypeScript";

/// Helpers the script calls, each for a value TypeScript shapes apart from Rust. Each is
/// exported, so one a script leaves unused is no unused local.
const PRELUDE: &str = r#"
const _decoder = new TextDecoder();

/** Bytes as UTF-8 text, what is not replaced. */
export function _text(data: Uint8Array | rt.Bytes): string {
	return _decoder.decode(data.slice());
}

/** A string's length in UTF-8 bytes, as Rust's `len`. */
export function _utf8Len(text: string): number {
	return rt.encoded(text).length;
}

/** A string as Rust's `Debug` prints it. */
export function _quoted(s: string): string {
	let out = '"';
	for (const c of s) {
		switch (c) {
			case '"':
			case "\\":
				out += "\\" + c;
				break;
			case "\n":
				out += "\\n";
				break;
			case "\t":
				out += "\\t";
				break;
			case "\r":
				out += "\\r";
				break;
			default:
				out += c;
		}
	}
	return out + '"';
}

export function _list<T>(items: Iterable<T>, show: (item: T) => string): string {
	return `[${Array.from(items, show).join(", ")}]`;
}

export function _option<T>(value: T | undefined, show: (value: T) => string): string {
	return value === undefined ? "None" : `Some(${show(value)})`;
}

/** An option whose `None` is `rt.None`, shown as Debug does. */
export function _optionN<T>(value: T | rt.None, show: (value: T) => string): string {
	return value === rt.None ? "None" : `Some(${show(value as T)})`;
}

/** `show` of `value`, which it reads more than once, evaluated once. */
export function _with<T>(value: T, show: (value: T) => string): string {
	return show(value);
}

export function _bound<T>(
	b: { readonly tag: "Included"; readonly _0: T } | { readonly tag: "Excluded"; readonly _0: T } | { readonly tag: "Unbounded" },
	show: (value: T) => string,
): string {
	return b.tag === "Unbounded" ? "Unbounded" : `${b.tag}(${show(b._0)})`;
}

/** A `u64` in decimal: its form, or a block's `bigint`. */
export function _u64(v: rt.u64 | bigint): string {
	return typeof v === "bigint" ? `${v}` : `${rt.u64ToBig(v)}`;
}

export function _i64(v: rt.i64 | bigint): string {
	return typeof v === "bigint" ? `${v}` : `${rt.i64ToBig(v)}`;
}

export function _u128(v: rt.u128): string {
	return `${rt.u128ToBig(v)}`;
}

/** An integer in hex, zero-padded to `width` digits. */
export function _hex(v: bigint, width: number): string {
	return v.toString(16).padStart(width, "0");
}

/** The value of a result that must be `Ok`. */
export function _ok<T, E>(r: rt.Result<T, E>): T {
	if (!r.ok) throw new Error(`an unexpected Err: ${String(r.error)}`);
	return r.value;
}

/** The error of a result that must be `Err`. */
export function _err<T, E>(r: rt.Result<T, E>): E {
	if (r.ok) throw new Error("an unexpected Ok");
	return r.error;
}

/** The value of an option that must be `Some`. */
export function _some<T>(value: T | undefined): T {
	if (value === undefined) throw new Error("an unwrap of None");
	return value;
}

/** The value of an option whose `None` is `rt.None`, which must be `Some`. */
export function _someN<T>(value: T | rt.None): T {
	if (value === rt.None) throw new Error("an unwrap of None");
	return value as T;
}

export function _mapOr<T, U>(value: T | undefined, none: U, some: (value: T) => U): U {
	return value === undefined ? none : some(value);
}

export function _mapOrN<T, U>(value: T | rt.None, none: U, some: (value: T) => U): U {
	return value === rt.None ? none : some(value as T);
}

/** Its elements as a tuple, not an array. */
export function _tuple<T extends unknown[]>(...items: T): T {
	return items;
}

/** The floats of `bits`, each never a JavaScript number, whose NaN an engine may make its
 * own: Deno's V8 does, storing one in an array literal. */
export function _f64s(bits: bigint[]): Float64Array {
	return new Float64Array(new BigUint64Array(bits).buffer);
}

export function _f32s(bits: number[]): Float32Array {
	return new Float32Array(new Uint32Array(bits).buffer);
}

export function _f64(bits: bigint): number {
	return _f64s([bits])[0]!;
}

/** The bits of `value`, a float the script holds, which a block of them is built from. */
export function _f64Bits(value: number): bigint {
	return new BigUint64Array(new Float64Array([value]).buffer)[0]!;
}

export function _f32Bits(value: number): number {
	return new Uint32Array(new Float32Array([value]).buffer)[0]!;
}

/** The objects a statement made and lent, which Rust drops as the statement ends. */
const _temps: Disposable[] = [];

/** `value`, a temporary, disposed once its statement ends. */
export function _tmp<T extends Disposable>(value: T): T {
	_temps.push(value);
	return value;
}

/** How many temporaries are held: a match's arms free only those made after its scrutinee's. */
export function _held(): number {
	return _temps.length;
}

/** Dispose the statement's temporaries past the first `kept`, the last made first, as Rust
 * drops them. */
export function _free(kept = 0): void {
	while (_temps.length > kept) _temps.pop()![Symbol.dispose]();
}
"#;

/// The TypeScript module of `t`'s script.
pub(crate) fn emit(t: &Transcript) -> String {
	let mut e = Emitter {
		t,
		depth: 1,
		mutated: Vec::new(),
		fresh: 0,
		closures: 0,
		modules: BTreeSet::new(),
		vars: BTreeMap::new(),
		temps: false,
		marks: Vec::new(),
		places: Vec::new(),
	};
	mutated(&t.steps, t, &mut e.mutated);
	let body = e.steps(&t.steps);
	let mut out = String::from(
		"// Generated from fixtures/api/examples/transcript.rs by tests/fixtures/transcript.\n\n\
		import * as api from \"./web/index.ts\";\n\
		import * as rt from \"./web/rt.ts\";\n\
		import * as hook from \"./api.ts\";\n",
	);
	for module in &e.modules {
		let _ = writeln!(out, "import * as {module} from \"./web/{module}.ts\";");
	}
	out.push_str(PRELUDE);
	out.push_str("\nexport async function script(): Promise<void> {\n");
	out.push_str(&body);
	out.push_str("}\n");
	out
}

/// The locals a call takes `&mut` that are values, not objects: each is an `rt.Place`,
/// whose `v` the script reads.
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
	/// The locals each an `rt.Place`.
	mutated: Vec<String>,
	fresh: usize,
	/// How many closures the expression being written is inside, where nothing awaits.
	closures: usize,
	/// The binding's type modules the script names, each imported.
	modules: BTreeSet<String>,
	/// Each local's type.
	vars: BTreeMap<String, Type>,
	/// Whether the statement being written made a temporary to dispose as it ends.
	temps: bool,
	/// For each match being written whose scrutinee made temporaries, the local holding how
	/// many were held after it, which its arms' statements leave to the match's end.
	marks: Vec<(String, bool)>,
	/// The names a pattern being written binds as places: each element's fresh name, the
	/// local's, and its type.
	places: Vec<(String, String, Type)>,
}

/// JavaScript's reserved words, and what the script itself names, which no local may be.
const RESERVED: &str = "\
	break case catch class const continue debugger default delete do else enum export extends false \
	finally for function if import in instanceof new null return super switch this throw true try \
	typeof var void while with yield let static implements interface package private protected \
	public await arguments eval undefined NaN Infinity api rt hook console Array String Number \
	BigInt Symbol Math Error Uint8Array Uint16Array Uint32Array Int8Array Int16Array Int32Array \
	BigUint64Array BigInt64Array Float32Array Float64Array TextDecoder";

/// A local's name in TypeScript.
fn ident(name: &str) -> String {
	match RESERVED.split_whitespace().any(|r| r == name) {
		true => format!("{name}_"),
		false => name.to_string(),
	}
}

/// A Rust name in camel case, as the binding names a field or a function: `into_label` is
/// `intoLabel`, and `0` is `_0`.
fn camel(name: &str) -> String {
	if name.starts_with(|c: char| c.is_ascii_digit()) {
		return format!("_{name}");
	}
	let mut out = String::new();
	for (i, part) in name.split('_').filter(|p| !p.is_empty()).enumerate() {
		let mut chars = part.chars();
		let first = chars.next().unwrap();
		match i {
			0 => out.extend(first.to_lowercase()),
			_ => out.extend(first.to_uppercase()),
		}
		out.push_str(chars.as_str());
	}
	out
}

/// Names each distinct, as the binding keeps a record's fields apart: `foo` and `foo_`,
/// both `foo` in camel case, are `foo` and `foo2`.
fn distinct(names: Vec<String>) -> Vec<String> {
	let mut taken = BTreeSet::new();
	(names.iter())
		.map(|name| {
			let mut unique = name.clone();
			let mut n = 2;
			while taken.contains(&unique) || (unique != *name && names.contains(&unique)) {
				unique = format!("{name}{n}");
				n += 1;
			}
			taken.insert(unique.clone());
			unique
		})
		.collect()
}

/// A list of `ty` held as a typed array: its item, if so.
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

/// Whether `ty` is an integer the binding holds as `rt.u64`, `rt.i64`, or `rt.u128`, each
/// with its `bigint` conversion.
fn wide(ty: &Type) -> Option<&'static str> {
	match ty {
		Type::Int(Int::U64) | Type::NonZero(Int::U64) => Some("u64"),
		Type::Int(Int::I64) | Type::NonZero(Int::I64) => Some("i64"),
		Type::U128 => Some("u128"),
		_ => None,
	}
}

/// Whether `expr` is a literal, whose type TypeScript would infer wider than the binding's.
fn literal(expr: &Expr) -> bool {
	!matches!(
		expr.kind,
		Kind::Var(_)
			| Kind::Const(_)
			| Kind::Call(..)
			| Kind::Field(..)
			| Kind::Element(..)
			| Kind::Clone(_)
			| Kind::Unwrap(_)
			| Kind::UnwrapErr(_)
			| Kind::Local(..)
			| Kind::Binary(..)
			| Kind::IsSome(_)
			| Kind::IsNone(_)
			| Kind::IsErr(_)
			| Kind::Format(_)
			| Kind::Join { .. }
			| Kind::MapOr { .. }
			| Kind::Utf8(_)
			| Kind::Len(_)
			| Kind::Default
	)
}

impl Emitter<'_> {
	fn indent(&self) -> String {
		"\t".repeat(self.depth)
	}

	fn fresh(&mut self, what: &str) -> String {
		self.fresh += 1;
		format!("${what}{}", self.fresh)
	}

	fn line(&self, out: &mut String, line: &str) {
		let _ = writeln!(out, "{}{line}", self.indent());
	}

	/// The binding's module of the type `ty`, imported.
	fn module(&mut self, ty: &Type) -> String {
		let name = self.t.type_name(LANGUAGE, unref(ty).base()).to_string();
		self.modules.insert(name.clone());
		name
	}

	fn steps(&mut self, steps: &[Step]) -> String {
		let mut out = String::new();
		for step in steps {
			self.step(step, &mut out);
			// A closure's statements leave theirs to the statement that called it, whose own
			// the closure may still lend.
			if self.temps && self.closures == 0 {
				let free = match self.marks.last_mut() {
					Some((mark, used)) => {
						*used = true;
						format!("_free({mark});")
					}
					None => "_free();".to_string(),
				};
				self.line(&mut out, &free);
				self.temps = false;
			}
		}
		out
	}

	/// `v`, the TypeScript of `e`, which a call lends: a temporary object disposed as the
	/// statement ends, as Rust drops it.
	fn lent(&mut self, e: &Expr, v: String) -> String {
		let fresh = matches!(
			e.kind,
			Kind::Call(..) | Kind::Default | Kind::Clone(_) | Kind::Unwrap(_) | Kind::Local(..)
		);
		match (fresh, unref(&e.ty).base()) {
			(true, Type::Object(_)) => {
				self.temps = true;
				format!("_tmp({v})")
			}
			_ => v,
		}
	}

	fn step(&mut self, step: &Step, out: &mut String) {
		match step {
			Step::Let(pat, value) => {
				// A value's `&mut self` returns it as the call left it, before what Rust returns.
				if let Some((place, call)) = self.mut_receiver(value) {
					let ret = unref(&value.ty).clone();
					match (&ret, pat) {
						(Type::Unit, Pat::Ignore(_)) => self.line(out, &format!("{place} = {call};")),
						(Type::Unit, _) => panic!("a let of a value's &mut self method returning ()"),
						_ => {
							let t = self.fresh("t");
							self.line(out, &format!("const {t} = {call};"));
							self.line(out, &format!("{place} = {t}[0];"));
							let rest = match &ret {
								Type::Tuple(elements) if elements.len() > 1 => {
									let parts: Vec<String> =
										(1..=elements.len()).map(|i| format!("{t}[{i}]")).collect();
									format!("_tuple({})", parts.join(", "))
								}
								_ => format!("{t}[1]"),
							};
							self.bind(pat, &rest, &ret, false, out);
						}
					}
					return;
				}
				let v = self.expr(value);
				match pat {
					Pat::Name(name, ty) if self.mutated.contains(name) => {
						self.vars.insert(name.clone(), ty.clone());
						let ty = self.ts(ty);
						self.line(out, &format!("let {}: rt.Place<{ty}> = {{ v: {v} }};", ident(name)));
					}
					pat => {
						let annotate = literal(value);
						self.bind(pat, &v, &value.ty, annotate, out);
					}
				}
			}
			Step::Print(pieces) => {
				let v = self.pieces(pieces);
				self.line(out, &format!("console.log({v});"));
			}
			Step::Do(value) => {
				if let Some((place, call)) = self.mut_receiver(value) {
					return match unref(&value.ty) {
						Type::Unit => self.line(out, &format!("{place} = {call};")),
						_ => self.line(out, &format!("{place} = {call}[0];")),
					};
				}
				let v = self.expr(value);
				// What a language's own function does for its effect may be async, as a
				// language's futures are.
				let v = match (&value.kind, &value.ty, self.closures) {
					(Kind::Local(..), Type::Unit, 0) => format!("await {v}"),
					_ => v,
				};
				self.line(out, &format!("{v};"));
			}
			Step::Drop(name) => match self.vars.get(name).map(|ty| unref(ty).base()) {
				Some(Type::Object(_)) => self.line(out, &format!("{}[Symbol.dispose]();", ident(name))),
				ty => panic!("a drop of {name}, a {ty:?}"),
			},
			Step::Match { value, ok, err } => self.matched(value, ok, err, out),
		}
	}

	/// A call of a method taking a value's `&mut self` on a local: the local's place, and the
	/// call, which TypeScript returns the value from as the call left it.
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
		let place = match self.mutated.contains(name) {
			true => format!("{}.v", ident(name)),
			false => ident(name),
		};
		let call = self.call(*id, Some(receiver), args);
		Some((place, call))
	}

	/// `let pat = v`, of a value of `ty`, annotated with its type if `annotate`.
	fn bind(&mut self, pat: &Pat, v: &str, ty: &Type, annotate: bool, out: &mut String) {
		let mut names = Vec::new();
		bound(pat, &mut names);
		self.vars.extend(names);
		match pat {
			Pat::Ignore(_) => self.line(out, &format!("{v};")),
			Pat::Name(name, ty) if annotate => {
				let ty = self.ts(ty);
				self.line(out, &format!("let {}: {ty} = {v};", ident(name)));
			}
			pat => {
				let target = self.pat(pat, ty);
				self.line(out, &format!("let {target} = {v};"));
				for (element, name, ty) in std::mem::take(&mut self.places) {
					let ty = self.ts(&ty);
					self.line(
						out,
						&format!("let {}: rt.Place<{ty}> = {{ v: {element} }};", ident(&name)),
					);
				}
			}
		}
	}

	/// A pattern binding a closure's parameter: an ignored one kept in the list as an
	/// unused name, and a place left in [`Emitter::places`] for [`Emitter::arrow`].
	fn param(&mut self, pat: &Pat, ty: &Type) -> String {
		match pat {
			Pat::Ignore(_) => {
				self.fresh += 1;
				format!("_{}", self.fresh)
			}
			pat => self.pat(pat, ty),
		}
	}

	/// The statements binding each place a closure's parameters left, as its body begins.
	fn places(&mut self) -> Vec<String> {
		std::mem::take(&mut self.places)
			.into_iter()
			.map(|(element, name, ty)| {
				let ty = self.ts(&ty);
				format!("let {}: rt.Place<{ty}> = {{ v: {element} }};", ident(&name))
			})
			.collect()
	}

	/// An arrow function of `params` returning `body`, binding the places they left first.
	fn arrow(&mut self, params: &str, places: Vec<String>, body: String) -> String {
		match (places.is_empty(), body.starts_with('{')) {
			(true, true) => format!("({params}) => ({body})"),
			(true, false) => format!("({params}) => {body}"),
			(false, _) => format!("({params}) => {{ {} return {body}; }}", places.join(" ")),
		}
	}

	/// A pattern binding a value of `ty`: a `Bounds`'s by its start and end.
	fn pat(&mut self, pat: &Pat, ty: &Type) -> String {
		match pat {
			// A place, bound once the pattern has its element.
			Pat::Name(name, ty) if self.mutated.contains(name) => {
				self.vars.insert(name.clone(), ty.clone());
				let element = self.fresh("e");
				self.places.push((element.clone(), name.clone(), ty.clone()));
				element
			}
			Pat::Name(name, ty) => {
				self.vars.insert(name.clone(), ty.clone());
				ident(name)
			}
			Pat::Ignore(_) => String::new(),
			Pat::Tuple(pats) => match unref(ty).base() {
				Type::Bounds(int) => {
					let bound = Type::Bound(*int);
					let parts: Vec<String> = (pats.iter().zip(["start", "end"]))
						.filter(|(p, _)| !matches!(p, Pat::Ignore(_)))
						.map(|(p, key)| format!("{key}: {}", self.pat(p, &bound)))
						.collect();
					format!("{{ {} }}", parts.join(", "))
				}
				Type::Tuple(elements) => {
					let elements = elements.clone();
					let parts: Vec<String> = pats.iter().zip(&elements).map(|(p, ty)| self.pat(p, ty)).collect();
					format!("[{}]", parts.join(", "))
				}
				ty => panic!("a tuple pattern of {ty:?}"),
			},
		}
	}

	fn matched(&mut self, value: &Expr, ok: &Arm, err: &Arm, out: &mut String) {
		let v = self.expr(value);
		// The scrutinee's temporaries live to the match's end, past its arms' statements.
		let held = std::mem::take(&mut self.temps);
		let (test, ok_value, ok_ty, err_value, err_ty) = match &value.ty {
			Type::Option(inner) => {
				let o = self.fresh("o");
				self.line(out, &format!("const {o} = {v};"));
				(
					format!("{o} !== {}", none(&value.ty)),
					o,
					(**inner).clone(),
					String::new(),
					Type::Unit,
				)
			}
			Type::Result(ok_ty, err_ty) => {
				let r = self.fresh("r");
				self.line(out, &format!("const {r} = {v};"));
				(
					format!("{r}.ok"),
					format!("{r}.value"),
					(**ok_ty).clone(),
					format!("{r}.error"),
					(**err_ty).clone(),
				)
			}
			ty => panic!("a match on {ty:?}"),
		};
		let at = out.len();
		if held {
			let mark = self.fresh("m");
			self.marks.push((mark, false));
		}
		self.line(out, &format!("if ({test}) {{"));
		self.depth += 1;
		if let Some(pat) = &ok.pat {
			self.bind(pat, &ok_value, &ok_ty, false, out);
		}
		let body = self.steps(&ok.steps);
		out.push_str(&body);
		self.depth -= 1;
		self.line(out, "} else {");
		self.depth += 1;
		if let Some(pat) = &err.pat {
			self.bind(pat, &err_value, &err_ty, false, out);
		}
		let body = self.steps(&err.steps);
		out.push_str(&body);
		self.depth -= 1;
		self.line(out, "}");
		if held {
			if let Some((mark, true)) = self.marks.pop() {
				let held = format!("{}const {mark} = _held();\n", self.indent());
				out.insert_str(at, &held);
			}
			self.temps = true;
		}
	}

	/// The TypeScript type of a value of `ty`, as a literal of it takes it.
	fn ts(&mut self, ty: &Type) -> String {
		if let Some(item) = block(ty) {
			return typed_array(item).to_string();
		}
		match ty {
			Type::Unit => "undefined".to_string(),
			Type::Bool => "boolean".to_string(),
			ty if wide(ty).is_some() => format!("rt.{}", wide(ty).unwrap()),
			Type::Int(_) | Type::NonZero(_) | Type::F32 | Type::F64 | Type::Char => "number".to_string(),
			Type::Str | Type::String => "string".to_string(),
			Type::Slice | Type::Vec | Type::Bytes | Type::Array(_) => "Uint8Array".to_string(),
			Type::Path | Type::PathBuf => "rt.PathBuf".to_string(),
			Type::Duration => "rt.Duration".to_string(),
			Type::SocketAddr(Family::Any) => "rt.SocketAddr".to_string(),
			Type::SocketAddr(Family::V4) => "rt.SocketAddrV4".to_string(),
			Type::SocketAddr(Family::V6) => "rt.SocketAddrV6".to_string(),
			Type::IpAddr(Family::Any) => "rt.IpAddr".to_string(),
			Type::IpAddr(Family::V4) => "rt.Ipv4Addr".to_string(),
			Type::IpAddr(Family::V6) => "rt.Ipv6Addr".to_string(),
			Type::List(_, item) => {
				let item = self.ts(item);
				match item.contains(' ') {
					true => format!("({item})[]"),
					false => format!("{item}[]"),
				}
			}
			Type::Option(inner) if undefinable(inner) => format!("{} | rt.None", self.ts(inner)),
			Type::Option(inner) => format!("{} | undefined", self.ts(inner)),
			Type::Box(inner) | Type::Ref(_, inner) => self.ts(inner),
			Type::Record(_) | Type::Enum(_) | Type::Object(_) => {
				let module = self.module(ty);
				format!("{module}.{module}")
			}
			Type::Range(int) => {
				let int = self.ts(&Type::Int(*int));
				format!("{{ start: {int}; end: {int} }}")
			}
			Type::Bound(int) => {
				let int = self.ts(&Type::Int(*int));
				format!(
					"{{ readonly tag: \"Included\"; readonly _0: {int} }} | {{ readonly tag: \"Excluded\"; readonly _0: {int} }} | {{ readonly tag: \"Unbounded\" }}"
				)
			}
			Type::Bounds(int) => {
				let bound = self.ts(&Type::Bound(*int));
				format!("{{ start: {bound}; end: {bound} }}")
			}
			Type::Tuple(elements) => {
				let elements: Vec<String> = elements.iter().map(|e| self.ts(e)).collect();
				format!("[{}]", elements.join(", "))
			}
			Type::Result(ok, err) => {
				let (ok, err) = (self.ts(ok), self.ts(err));
				format!("rt.Result<{ok}, {err}>")
			}
			ty => panic!("the TypeScript type of {ty:?}"),
		}
	}

	/// A template literal of a format's pieces.
	fn pieces(&mut self, pieces: &[Piece]) -> String {
		let mut out = String::from("`");
		for piece in pieces {
			match piece {
				Piece::Text(text) => out.push_str(&template(text)),
				Piece::Value(e, spec) => {
					let v = self.expr(e);
					let v = self.lent(e, v);
					let shown = self.show(&v, &e.ty, *spec);
					let _ = write!(out, "${{{shown}}}");
				}
			}
		}
		out.push('`');
		out
	}

	/// `v`, a value of `ty`, as a string the way Rust formats it with `spec`.
	fn show(&mut self, v: &str, ty: &Type, spec: Spec) -> String {
		let ty = unref(ty).base().clone();
		match spec {
			Spec::Hex(width) => return format!("_hex({}, {width})", big(v, &ty)),
			Spec::Padded(width) => {
				return format!("String({}).padStart({width}, \"0\")", self.show(v, &ty, Spec::Display));
			}
			Spec::Debug => return self.debug(v, &ty),
			Spec::Display => {}
		}
		match &ty {
			Type::Bool => v.to_string(),
			Type::String | Type::Str => v.to_string(),
			Type::Char => format!("String.fromCodePoint({v})"),
			ty if wide(ty).is_some() => format!("_{}({v})", wide(ty).unwrap()),
			Type::Int(_) | Type::NonZero(_) => v.to_string(),
			Type::Object(_) | Type::Record(_) | Type::Enum(_) => self.text(v, &ty),
			ty => panic!("Display of {ty:?}"),
		}
	}

	/// A type's `Display`: its `Access::Text` method, else its `toString`.
	fn text(&mut self, v: &str, ty: &Type) -> String {
		let (owner, methods, traits) = match ty {
			Type::Record(i) => (
				Owner::Record(*i),
				&self.t.api.records[*i].methods,
				self.t.api.records[*i].traits,
			),
			Type::Enum(i) => (
				Owner::Enum(*i),
				&self.t.api.enums[*i].methods,
				self.t.api.enums[*i].traits,
			),
			Type::Object(i) => (
				Owner::Object(*i),
				&self.t.api.objects[*i].methods,
				self.t.api.objects[*i].traits,
			),
			ty => panic!("Display of {ty:?}"),
		};
		let module = self.module(ty);
		if let Some(index) = methods.iter().position(|m| m.access == Access::Text) {
			let name = self.t.name(LANGUAGE, FnId { owner, index });
			return format!("{module}.{name}({v})");
		}
		if !traits.display {
			panic!("Display of {}, which has none", self.t.rust(ty));
		}
		format!("{module}.toString({v})")
	}

	/// `v` as Rust's `Debug` prints a value of `ty`.
	fn debug(&mut self, v: &str, ty: &Type) -> String {
		let ty = unref(ty).base().clone();
		if let Some(item) = block(&ty) {
			let shown = match item {
				Type::Int(_) => "`${x}`",
				ty => panic!("Debug of a block of {ty:?}"),
			};
			return format!("_list({v}, (x) => {shown})");
		}
		match &ty {
			Type::Bool => format!("`${{{v}}}`"),
			ty if wide(ty).is_some() => format!("_{}({v})", wide(ty).unwrap()),
			Type::Int(_) | Type::NonZero(_) => format!("`${{{v}}}`"),
			Type::String | Type::Str => format!("_quoted({v})"),
			Type::Option(inner) => {
				// Typed, as TypeScript infers no `T` of `T | rt.None` apart from the symbol.
				let helper = match undefinable(inner) {
					true => format!("_optionN<{}>", self.ts(inner)),
					false => "_option".to_string(),
				};
				let x = self.fresh("x");
				let show = self.debug(&x, inner);
				format!("{helper}({v}, ({x}) => {show})")
			}
			Type::Vec | Type::Slice | Type::Bytes | Type::Array(_) => format!("_list({v}, (x) => `${{x}}`)"),
			Type::List(_, item) => {
				let x = self.fresh("x");
				let show = self.debug(&x, item);
				format!("_list({v}, ({x}) => {show})")
			}
			// Each element read from the tuple, evaluated once.
			Type::Tuple(elements) => {
				let x = self.fresh("x");
				let parts: Vec<String> = (elements.iter().enumerate())
					.map(|(i, e)| self.debug(&format!("{x}[{i}]"), e))
					.collect();
				let parts: Vec<String> = parts.iter().map(|p| format!("${{{p}}}")).collect();
				let tail = if parts.len() == 1 { ",)" } else { ")" };
				// Typed, as TypeScript infers an array literal's elements as one union.
				let tuple = self.ts(&ty);
				format!("_with<{tuple}>({v}, ({x}) => `({}{tail}`)", parts.join(", "))
			}
			Type::Bound(int) => {
				let x = self.fresh("x");
				let show = self.debug(&x, &Type::Int(*int));
				format!("_bound({v}, ({x}) => {show})")
			}
			Type::Record(_) | Type::Enum(_) | Type::Object(_) => {
				let traits = match &ty {
					Type::Record(i) => self.t.api.records[*i].traits,
					Type::Enum(i) => self.t.api.enums[*i].traits,
					Type::Object(i) => self.t.api.objects[*i].traits,
					_ => unreachable!(),
				};
				if !traits.debug {
					panic!("Debug of {}, which has none", self.t.rust(&ty));
				}
				let module = self.module(&ty);
				format!("{module}.debug({v})")
			}
			ty => panic!("Debug of {ty:?}"),
		}
	}

	fn expr(&mut self, e: &Expr) -> String {
		self.value(e, &e.ty)
	}

	/// The TypeScript of `e`, passed where `want` is taken.
	fn value(&mut self, e: &Expr, want: &Type) -> String {
		let v = self.bare(e, want);
		// An address of one family, where either is taken, is its variant.
		match (unref(&e.ty).base(), unref(want).base()) {
			(Type::SocketAddr(Family::V4), Type::SocketAddr(Family::Any))
			| (Type::IpAddr(Family::V4), Type::IpAddr(Family::Any)) => {
				format!("{{ tag: \"V4\", _0: {v} }}")
			}
			(Type::SocketAddr(Family::V6), Type::SocketAddr(Family::Any))
			| (Type::IpAddr(Family::V6), Type::IpAddr(Family::Any)) => {
				format!("{{ tag: \"V6\", _0: {v} }}")
			}
			_ => v,
		}
	}

	fn bare(&mut self, e: &Expr, want: &Type) -> String {
		match &e.kind {
			Kind::Unit => "undefined".to_string(),
			Kind::Bool(b) => b.to_string(),
			Kind::Int(n) => int(*n, unref(&e.ty).base()),
			Kind::Float(f) => float(*f),
			Kind::Char(c) => (*c as u32).to_string(),
			Kind::Str(s) => match unref(&e.ty).base() {
				Type::Path | Type::PathBuf => format!("rt.pathFrom({})", string(s)),
				_ => string(s),
			},
			Kind::Bytes(b) => bytes(b),
			Kind::Duration(secs, nanos) => {
				let secs = int(Num::new(i128::from(*secs)), &Type::Int(Int::U64));
				format!("{{ secs: {secs}, nanos: {nanos} }}")
			}
			Kind::Socket(addr) => {
				let inner = match addr {
					std::net::SocketAddr::V4(a) => {
						format!("{{ ip: {}, port: {} }}", octets(&a.ip().octets()), a.port())
					}
					std::net::SocketAddr::V6(a) => format!(
						"{{ ip: {}, port: {}, flowinfo: {}, scopeId: {} }}",
						octets(&a.ip().octets()),
						a.port(),
						a.flowinfo(),
						a.scope_id()
					),
				};
				match unref(&e.ty).base() {
					Type::SocketAddr(Family::Any) => format!("{{ tag: \"{}\", _0: {inner} }}", family(addr.is_ipv4())),
					_ => inner,
				}
			}
			Kind::Ip(ip) => {
				let inner = match ip {
					std::net::IpAddr::V4(v4) => octets(&v4.octets()),
					std::net::IpAddr::V6(v6) => octets(&v6.octets()),
				};
				match unref(&e.ty).base() {
					Type::IpAddr(Family::Any) => format!("{{ tag: \"{}\", _0: {inner} }}", family(ip.is_ipv4())),
					_ => inner,
				}
			}
			Kind::None => match unref(want).base() {
				Type::Option(inner) if undefinable(inner) => "rt.None".to_string(),
				_ => "undefined".to_string(),
			},
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
				let names = distinct(record.fields.iter().map(|f| camel(&f.name)).collect());
				let types: Vec<Type> = record.fields.iter().map(|f| f.ty.clone()).collect();
				let parts: Vec<String> = (fields.iter().zip(&types).zip(&names))
					.map(|((f, ty), name)| format!("{name}: {}", self.value(f, ty)))
					.collect();
				format!("{{ {} }}", parts.join(", "))
			}
			Kind::Variant(en, v, fields) => {
				let enumeration = &self.t.api.enums[*en];
				let variant = &enumeration.variants[*v];
				if enumeration.variants.iter().all(|v| v.fields.is_empty()) {
					return string(&variant.name);
				}
				let names = distinct(
					(variant.fields.iter())
						.map(|f| match camel(&f.name) {
							name if name == "tag" => "tag$".to_string(),
							name => name,
						})
						.collect(),
				);
				let types: Vec<Type> = variant.fields.iter().map(|f| f.ty.clone()).collect();
				let mut parts = vec![format!("tag: {}", string(&variant.name))];
				for ((f, ty), name) in fields.iter().zip(&types).zip(&names) {
					parts.push(format!("{name}: {}", self.value(f, ty)));
				}
				format!("{{ {} }}", parts.join(", "))
			}
			Kind::Bound(kind, value) => match (kind, value) {
				(BoundKind::Unbounded, _) => "{ tag: \"Unbounded\" }".to_string(),
				(BoundKind::Included, Some(v)) => format!("{{ tag: \"Included\", _0: {} }}", self.expr(v)),
				(BoundKind::Excluded, Some(v)) => format!("{{ tag: \"Excluded\", _0: {} }}", self.expr(v)),
				_ => unreachable!(),
			},
			Kind::Default => {
				let module = self.module(&e.ty);
				format!("{module}.default()")
			}
			Kind::Var(name) => match self.mutated.contains(name) {
				// Passed `&mut`, the place itself; read, its value.
				true if matches!(want, Type::Ref(Mutability::Mut, _)) => ident(name),
				true => format!("{}.v", ident(name)),
				false => ident(name),
			},
			Kind::Const(c) => {
				let key = format!("const {}", self.t.api.constants[*c].path.join("::"));
				format!("api.{}", self.t.named(LANGUAGE, &key))
			}
			Kind::Call(id, receiver, args) => {
				let call = self.call(*id, receiver.as_deref(), args);
				match self.t.function(*id).future {
					true if self.closures == 0 => format!("(await {call})"),
					true => panic!("a call of {} in a closure, which cannot await it", self.t.key(*id)),
					false => call,
				}
			}
			Kind::Field(base, i) => {
				let b = self.expr(base);
				let Type::Record(r) = unref(&base.ty).base() else {
					panic!("a field of {:?}", base.ty)
				};
				let names = distinct(self.t.api.records[*r].fields.iter().map(|f| camel(&f.name)).collect());
				format!("{b}.{}", names[*i])
			}
			Kind::Element(base, i) => {
				let b = self.expr(base);
				match unref(&base.ty).base() {
					Type::Range(_) => format!("{b}.{}", if *i == 0 { "start" } else { "end" }),
					_ => format!("{b}[{i}]"),
				}
			}
			Kind::Clone(inner) => {
				let v = self.expr(inner);
				match unref(&inner.ty).base() {
					ty @ Type::Object(_) => {
						let module = self.module(ty);
						format!("{module}.clone({v})")
					}
					ty @ Type::List(..) if block(ty).is_some() => format!("{v}.slice()"),
					Type::List(..) => format!("[...{v}]"),
					// A value, which nothing changes in place.
					_ => v,
				}
			}
			Kind::Unwrap(inner) => {
				let v = self.expr(inner);
				match unref(&inner.ty).base() {
					Type::Result(..) => format!("_ok({v})"),
					Type::Option(some) if undefinable(some) => {
						let some = (**some).clone();
						format!("_someN<{}>({v})", self.ts(&some))
					}
					_ => format!("_some({v})"),
				}
			}
			Kind::UnwrapErr(inner) => format!("_err({})", self.expr(inner)),
			Kind::IsErr(inner) => format!("!{}.ok", self.expr(inner)),
			Kind::IsSome(inner) => format!("({} !== {})", self.expr(inner), none(&inner.ty)),
			Kind::IsNone(inner) => format!("({} === {})", self.expr(inner), none(&inner.ty)),
			Kind::Binary(op, a, b) => self.binary(*op, a, b),
			Kind::Closure(params, steps, value) => {
				let Type::Closure(closure) = unref(&e.ty).base() else {
					panic!("a closure of {:?}", e.ty)
				};
				let types = closure.params.clone();
				self.closure(params, &types, steps, value)
			}
			Kind::Local(name, args) => {
				let args: Vec<String> = args.iter().map(|a| self.expr(a)).collect();
				match name.starts_with(|c: char| c.is_ascii_uppercase()) {
					// A unit struct: a value of the language's own class.
					true => format!("new hook.{name}({})", args.join(", ")),
					false => format!("hook.{}({})", camel(name), args.join(", ")),
				}
			}
			Kind::Format(pieces) => self.pieces(pieces),
			Kind::Join { items, pat, body, sep } => {
				let Type::List(_, item) = unref(&items.ty).base() else {
					panic!("a join over {:?}", items.ty)
				};
				let item = (**item).clone();
				let block = block(unref(&items.ty).base()).is_some();
				let mut items = self.expr(items);
				// A block's wide items are `bigint`s, which the body takes as the binding's form.
				if let (true, Some(wide)) = (block, wide(&item)) {
					items = format!("Array.from({items}, (v) => rt.{wide}FromBig(v))");
				}
				let target = self.param(pat, &item);
				let places = self.places();
				self.closures += 1;
				let body = self.expr(body);
				self.closures -= 1;
				let arrow = self.arrow(&target, places, body);
				format!("Array.from({items}, {arrow}).join({})", string(sep))
			}
			Kind::MapOr {
				option,
				none,
				pat,
				some,
			} => {
				let Type::Option(inner) = unref(&option.ty).base() else {
					panic!("a map_or of {:?}", option.ty)
				};
				let inner = (**inner).clone();
				let option = self.expr(option);
				let none = self.expr(none);
				let target = self.param(pat, &inner);
				let places = self.places();
				self.closures += 1;
				let some = self.expr(some);
				self.closures -= 1;
				let arrow = self.arrow(&target, places, some);
				let helper = match undefinable(&inner) {
					true => format!("_mapOrN<{}, {}>", self.ts(&inner), self.ts(&e.ty)),
					false => "_mapOr".to_string(),
				};
				format!("{helper}({option}, {none}, {arrow})")
			}
			Kind::Utf8(inner) => format!("_text({})", self.expr(inner)),
			Kind::Len(inner) => {
				let v = self.expr(inner);
				match unref(&inner.ty).base() {
					Type::String | Type::Str => format!("_utf8Len({v})"),
					_ => format!("{v}.length"),
				}
			}
		}
	}

	fn binary(&mut self, op: Op, a: &Expr, b: &Expr) -> String {
		let ty = unref(&a.ty).base().clone();
		let (x, y) = (self.expr(a), self.expr(b));
		let (x, y) = (self.lent(a, x), self.lent(b, y));
		match op {
			Op::Eq => self.equal(&x, &y, &ty),
			Op::Ne => format!("!{}", self.equal(&x, &y, &ty)),
			Op::And => format!("({x} && {y})"),
			Op::Lt | Op::Gt | Op::Add | Op::Sub | Op::Mul | Op::Rem => {
				let symbol = match op {
					Op::Lt => "<",
					Op::Gt => ">",
					Op::Add => "+",
					Op::Sub => "-",
					Op::Mul => "*",
					Op::Rem => "%",
					_ => unreachable!(),
				};
				match wide(&ty) {
					Some(_) => {
						let joined = format!("{} {symbol} {}", big(&x, &ty), big(&y, &ty));
						match op {
							Op::Lt | Op::Gt => format!("({joined})"),
							_ => format!("rt.{}FromBig({joined})", wide(&ty).unwrap()),
						}
					}
					None if matches!(ty, Type::Int(_) | Type::NonZero(_) | Type::F32 | Type::F64) => {
						format!("({x} {symbol} {y})")
					}
					None => panic!("{op:?} of {ty:?}"),
				}
			}
		}
	}

	/// Whether `x` equals `y`, each a value of `ty`, as Rust's `==` says.
	fn equal(&mut self, x: &str, y: &str, ty: &Type) -> String {
		if wide(ty).is_some() {
			return format!("({} === {})", big(x, ty), big(y, ty));
		}
		match ty {
			Type::Enum(i) if self.t.api.enums[*i].variants.iter().all(|v| v.fields.is_empty()) => {
				format!("({x} === {y})")
			}
			Type::Record(_) | Type::Enum(_) | Type::Object(_) => {
				let module = self.module(ty);
				format!("{module}.eq({x}, {y})")
			}
			Type::Bool | Type::Int(_) | Type::NonZero(_) | Type::Char | Type::String | Type::Str => {
				format!("({x} === {y})")
			}
			ty => panic!("== of {ty:?}"),
		}
	}

	fn tuple(&mut self, e: &Expr, elements: &[Expr], want: &Type) -> String {
		match unref(&e.ty).base() {
			Type::Range(_) | Type::Bounds(_) => {
				let (a, b) = (self.expr(&elements[0]), self.expr(&elements[1]));
				format!("{{ start: {a}, end: {b} }}")
			}
			_ => {
				let wants: Vec<Type> = match unref(want).base() {
					Type::Tuple(ts) => ts.clone(),
					_ => elements.iter().map(|e| e.ty.clone()).collect(),
				};
				let parts: Vec<String> = elements.iter().zip(&wants).map(|(e, t)| self.value(e, t)).collect();
				format!("[{}]", parts.join(", "))
			}
		}
	}

	fn list(&mut self, items: &[Expr], want: &Type) -> String {
		let ty = unref(want).base().clone();
		let Type::List(_, item) = &ty else {
			panic!("a list as {ty:?}")
		};
		let item = (**item).clone();
		let Some(element) = block(&ty) else {
			let parts: Vec<String> = items.iter().map(|e| self.value(e, &item)).collect();
			return format!("[{}]", parts.join(", "));
		};
		// A block's items: integers past 32 bits as `bigint`s, and floats as their bits, a
		// literal's spelled, another's read from the value the script holds.
		let mut each = |f: &dyn Fn(&Kind) -> Option<String>, convert: &str| -> Vec<String> {
			(items.iter())
				.map(|e| match f(&e.kind) {
					Some(literal) => literal,
					None => format!("{convert}({})", self.value(e, &item)),
				})
				.collect()
		};
		match element {
			Type::Int(int @ (Int::U64 | Int::I64)) => {
				let convert = format!("rt.{}ToBig", wide(element).unwrap());
				let parts = each(
					&|k| match k {
						Kind::Int(n) => Some(format!("{n}n")),
						_ => None,
					},
					&convert,
				);
				format!("new {}([{}])", typed_array(&Type::Int(*int)), parts.join(", "))
			}
			Type::F64 => {
				let parts = each(
					&|k| match k {
						Kind::Float(f) => Some(format!("{:#x}n", f.to_bits())),
						_ => None,
					},
					"_f64Bits",
				);
				format!("_f64s([{}])", parts.join(", "))
			}
			Type::F32 => {
				let parts = each(
					&|k| match k {
						Kind::Float(f) => Some(format!("{:#x}", (*f as f32).to_bits())),
						_ => None,
					},
					"_f32Bits",
				);
				format!("_f32s([{}])", parts.join(", "))
			}
			element => {
				let parts: Vec<String> = items.iter().map(|e| self.value(e, &item)).collect();
				format!("new {}([{}])", typed_array(element), parts.join(", "))
			}
		}
	}

	/// An arrow function: its parameters, of `types`, the steps it runs, and what it returns.
	fn closure(&mut self, params: &[Pat], types: &[Type], steps: &[Step], value: &Expr) -> String {
		let params: Vec<String> = params.iter().zip(types).map(|(p, ty)| self.param(p, ty)).collect();
		let params = params.join(", ");
		let places = self.places();
		self.closures += 1;
		let arrow = match steps.is_empty() {
			true => {
				let v = self.expr(value);
				self.arrow(&params, places, v)
			}
			false => {
				self.depth += 1;
				let mut body = String::new();
				for place in &places {
					self.line(&mut body, place);
				}
				body.push_str(&self.steps(steps));
				if !matches!(value.kind, Kind::Unit) {
					let v = self.expr(value);
					self.line(&mut body, &format!("return {v};"));
				}
				self.depth -= 1;
				format!("({params}) => {{\n{body}{}}}", self.indent())
			}
		};
		self.closures -= 1;
		arrow
	}

	/// A call of `id`: a function of the module, an associated function of a type, a
	/// method, or a field's getter or setter.
	fn call(&mut self, id: FnId, receiver: Option<&Expr>, args: &[Expr]) -> String {
		let f = self.t.function(id);
		let name = self.t.name(LANGUAGE, id).to_string();
		let params: Vec<Type> = f.params.iter().map(|p| p.ty.clone()).collect();
		let access = f.access;
		let owner = match self.t.owner(id) {
			Some(ty) => self.module(&ty),
			None => "api".to_string(),
		};
		let by_ref = matches!(f.receiver, Some(Receiver::Ref | Receiver::Mut));
		let mut values = Vec::new();
		if let Some(r) = receiver {
			let v = self.expr(r);
			values.push(if by_ref { self.lent(r, v) } else { v });
		}
		for (a, ty) in args.iter().zip(&params) {
			let v = self.value(a, ty);
			values.push(if matches!(ty, Type::Ref(..)) {
				self.lent(a, v)
			} else {
				v
			});
		}
		match access {
			Access::Const => format!("{owner}.{name}()"),
			_ => format!("{owner}.{name}({})", values.join(", ")),
		}
	}
}

/// Whether a value of `ty` can be `undefined`, so its `Option` keeps `None` apart as `rt.None`.
fn undefinable(ty: &Type) -> bool {
	matches!(ty.base(), Type::Unit | Type::Option(_))
}

/// The `None` of the option `ty`: `rt.None` where its `Some` may be `undefined`.
fn none(ty: &Type) -> &'static str {
	match unref(ty).base() {
		Type::Option(inner) if undefinable(inner) => "rt.None",
		_ => "undefined",
	}
}

/// The typed array holding a block's elements exactly.
fn typed_array(elem: &Type) -> &'static str {
	match elem {
		Type::Int(Int::U8) => "Uint8Array",
		Type::Int(Int::U16) => "Uint16Array",
		Type::Int(Int::U32) => "Uint32Array",
		Type::Int(Int::U64) => "BigUint64Array",
		Type::Int(Int::I8) => "Int8Array",
		Type::Int(Int::I16) => "Int16Array",
		Type::Int(Int::I32) => "Int32Array",
		Type::Int(Int::I64) => "BigInt64Array",
		Type::F32 => "Float32Array",
		Type::F64 => "Float64Array",
		other => panic!("{other:?} is no block"),
	}
}

/// `v`, a value of the integer type `ty`, as a `bigint`.
fn big(v: &str, ty: &Type) -> String {
	match wide(ty) {
		Some(wide) => format!("rt.{wide}ToBig({v})"),
		None => format!("BigInt({v})"),
	}
}

/// An integer literal of `ty`: a number while it is a safe integer, else its type's form.
fn int(n: Num, ty: &Type) -> String {
	const SAFE: u128 = (1 << 53) - 1;
	match ty {
		ty if wide(ty).is_some() => match n.magnitude <= SAFE {
			true => n.to_string(),
			false => format!("rt.{}FromBig({n}n)", wide(ty).unwrap()),
		},
		// The web binding's target is wasm32, whose pointer is 32 bits: a literal past them,
		// which no Rust for wasm32 spells, is the bound `MAX` or `MIN` is on a 64-bit host.
		Type::Int(Int::Usize) | Type::NonZero(Int::Usize) => match (n.negative, n.magnitude) {
			(false, m) if m <= u128::from(u32::MAX) => n.to_string(),
			(false, m) if m == u128::from(u64::MAX) => u32::MAX.to_string(),
			_ => panic!("{n} as wasm32's usize"),
		},
		Type::Int(Int::Isize) | Type::NonZero(Int::Isize) => {
			let v = if n.negative {
				-(n.magnitude as i128)
			} else {
				n.magnitude as i128
			};
			match v {
				v if i32::try_from(v).is_ok() => v.to_string(),
				v if v == i128::from(i64::MAX) => i32::MAX.to_string(),
				v if v == i128::from(i64::MIN) => i32::MIN.to_string(),
				_ => panic!("{n} as wasm32's isize"),
			}
		}
		Type::Int(_) | Type::NonZero(_) => n.to_string(),
		ty => panic!("an integer literal as {ty:?}"),
	}
}

/// A float literal: a NaN other than JavaScript's own, by its bits.
fn float(f: f64) -> String {
	match f {
		f if f.is_nan() && f.to_bits() != f64::NAN.to_bits() => format!("_f64({:#x}n)", f.to_bits()),
		f if f.is_nan() => "NaN".to_string(),
		f if f == f64::INFINITY => "Infinity".to_string(),
		f if f == f64::NEG_INFINITY => "-Infinity".to_string(),
		f => format!("{f:?}"),
	}
}

fn family(v4: bool) -> &'static str {
	if v4 { "V4" } else { "V6" }
}

/// An IP address's octets, as the binding holds one.
fn octets(b: &[u8]) -> String {
	format!("{{ octets: {} }}", bytes(b))
}

fn bytes(b: &[u8]) -> String {
	let parts: Vec<String> = b.iter().map(u8::to_string).collect();
	format!("new Uint8Array([{}])", parts.join(", "))
}

/// A JavaScript string literal.
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
			c if (c as u32) < 0x20 => {
				let _ = write!(out, "\\u{:04x}", c as u32);
			}
			c => out.push(c),
		}
	}
	out.push('"');
	out
}

/// Text inside a template literal.
fn template(s: &str) -> String {
	let mut out = String::new();
	for c in s.chars() {
		match c {
			'`' | '\\' | '$' => {
				out.push('\\');
				out.push(c);
			}
			'\n' => out.push_str("\\n"),
			'\t' => out.push_str("\\t"),
			'\r' => out.push_str("\\r"),
			c if (c as u32) < 0x20 => {
				let _ = write!(out, "\\u{:04x}", c as u32);
			}
			c => out.push(c),
		}
	}
	out
}
