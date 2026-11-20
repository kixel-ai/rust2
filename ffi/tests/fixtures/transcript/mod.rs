//! The api fixture's transcript as data: `fixtures/api/examples/transcript.rs`'s `main`,
//! a typed subset of Rust, read and resolved against the export's `api.json`, then
//! written as each language's script, whose output must match the native one line for line.
//!
//! The subset is what every binding can say alike: `let`, `println!` and `format!`, calls
//! of the API's functions and methods, field reads and writes, `match` on a `Result` or an
//! `Option`, `for` over an array literal, which unrolls, closures, `.map(..).collect()`
//! joined into a string, and literals of the API's types and of the standard library's it
//! maps. A call of a function `transcript.rs` defines itself is each language's own,
//! hand-written beside its script under the same name: what tests a language's idiom, such
//! as its futures, or prints a value no binding shapes alike, such as `hex`. Anything else
//! fails, naming its line.

pub(crate) mod c;
pub(crate) mod cpp;
pub(crate) mod dart;
pub(crate) mod go;
pub(crate) mod ir;
pub(crate) mod kotlin;
mod parse;
pub(crate) mod python;
pub(crate) mod swift;
pub(crate) mod typescript;

use std::collections::BTreeMap;
use std::path::Path;

use ir::{Access, Family, Function, Type};

/// The transcript, resolved against an export.
pub(crate) struct Transcript {
	pub(crate) api: ir::Api,
	names: BTreeMap<String, BTreeMap<String, String>>,
	pub(crate) steps: Vec<Step>,
	/// The parameters' types of each function `transcript.rs` defines that the script calls,
	/// by its name.
	pub(crate) locals: BTreeMap<String, Vec<Type>>,
}

/// A statement of the script.
#[derive(Clone)]
pub(crate) enum Step {
	Let(Pat, Expr),
	Print(Vec<Piece>),
	/// An expression whose value is dropped, such as a call made for its effect, a setter's
	/// included.
	Do(Expr),
	/// `drop(name)`: the local's value drops here, not at the end of its scope.
	Drop(String),
	/// A `match` on a `Result` or an `Option`, `ok` on `Ok` or `Some`.
	Match {
		value: Expr,
		ok: Arm,
		err: Arm,
	},
}

/// A `match` arm: the pattern binding the payload, if any, and the steps it runs.
#[derive(Clone)]
pub(crate) struct Arm {
	pub(crate) pat: Option<Pat>,
	pub(crate) steps: Vec<Step>,
}

/// What a `let`, an arm, or a closure's parameter binds. Each name is unique in the
/// script, so a language that cannot shadow need not rename.
#[derive(Clone)]
pub(crate) enum Pat {
	Name(String, Type),
	Tuple(Vec<Pat>),
	Ignore(Type),
}

/// A piece of a `println!` or a `format!`.
#[derive(Clone)]
pub(crate) enum Piece {
	Text(String),
	Value(Expr, Spec),
}

/// How a value prints.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum Spec {
	Display,
	Debug,
	/// `{:x}`, or `{:0Nx}` zero-padded to `N` digits.
	Hex(usize),
	/// `{:0N}`: decimal, zero-padded to `N` digits.
	Padded(usize),
}

/// A typed expression.
#[derive(Clone)]
pub(crate) struct Expr {
	pub(crate) kind: Kind,
	pub(crate) ty: Type,
}

/// An integer literal of any width: its magnitude, and whether it is negative.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) struct Num {
	pub(crate) negative: bool,
	pub(crate) magnitude: u128,
}

impl Num {
	pub(crate) fn new(v: i128) -> Num {
		Num {
			negative: v < 0,
			magnitude: v.unsigned_abs(),
		}
	}

	/// Its two's complement bits, truncated to 64.
	pub(crate) fn bits(self) -> u64 {
		let v = self.magnitude as u64;
		if self.negative { v.wrapping_neg() } else { v }
	}
}

impl std::fmt::Display for Num {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match self.negative {
			true => write!(f, "-{}", self.magnitude),
			false => write!(f, "{}", self.magnitude),
		}
	}
}

/// What owns a function: nothing, or the type whose method it is.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum Owner {
	Free,
	Record(usize),
	Enum(usize),
	Object(usize),
}

/// A function of the API: its owner and its index there.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) struct FnId {
	pub(crate) owner: Owner,
	pub(crate) index: usize,
}

/// A binary operator.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum Op {
	Eq,
	Ne,
	Lt,
	Le,
	Gt,
	Ge,
	/// `a.cmp(&b) as i8`: Rust's `Ord`, -1, 0, or 1.
	Cmp,
	Add,
	Sub,
	Mul,
	Rem,
	And,
}

/// A `std::ops::Bound`'s variant.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum BoundKind {
	Included,
	Excluded,
	Unbounded,
}

#[derive(Clone)]
pub(crate) enum Kind {
	Unit,
	Bool(bool),
	Int(Num),
	/// A float, an `f32`'s widened.
	Float(f64),
	Char(char),
	/// A string, or a path spelled as one.
	Str(String),
	Bytes(Vec<u8>),
	/// A `Duration`'s seconds and nanoseconds.
	Duration(u64, u32),
	Socket(std::net::SocketAddr),
	Ip(std::net::IpAddr),
	None,
	Some(Box<Expr>),
	/// A tuple, or a `Range`'s start and end, or a `Bounds`'s two `Bound`s.
	Tuple(Vec<Expr>),
	/// The items of a collection, a map's as tuples of a key and its value.
	List(Vec<Expr>),
	/// A record built from its fields, in its order.
	Record(usize, Vec<Expr>),
	/// An enum's variant, from its fields.
	Variant(usize, usize, Vec<Expr>),
	Bound(BoundKind, Option<Box<Expr>>),
	/// A type's `Default`.
	Default,
	Var(String),
	/// A constant of the IR's `constants`.
	Const(usize),
	/// A call: the receiver of a method taking `self`, then its arguments.
	Call(FnId, Option<Box<Expr>>, Vec<Expr>),
	/// A record's field, by index.
	Field(Box<Expr>, usize),
	/// A tuple's element, or a `Range`'s start (0) or end (1).
	Element(Box<Expr>, usize),
	/// `Clone::clone`.
	Clone(Box<Expr>),
	/// `.unwrap()` or `.expect(..)` of an `Option` or a `Result`.
	Unwrap(Box<Expr>),
	/// `.err().unwrap()` of a `Result`.
	UnwrapErr(Box<Expr>),
	IsSome(Box<Expr>),
	IsNone(Box<Expr>),
	IsErr(Box<Expr>),
	Binary(Op, Box<Expr>, Box<Expr>),
	/// A closure: its parameters, the steps it runs, and what it returns.
	Closure(Vec<Pat>, Vec<Step>, Box<Expr>),
	/// A call of a function `transcript.rs` defines, or a value of a unit struct it does,
	/// each a language's own.
	Local(String, Vec<Expr>),
	/// `format!`
	Format(Vec<Piece>),
	/// `items.map(|pat| body).collect::<Vec<_>>().join(sep)`, over a list or an iterator.
	Join {
		items: Box<Expr>,
		pat: Pat,
		body: Box<Expr>,
		sep: String,
	},
	/// `option.map_or(none, |pat| some)`
	MapOr {
		option: Box<Expr>,
		none: Box<Expr>,
		pat: Pat,
		some: Box<Expr>,
	},
	/// Bytes as UTF-8 text: `String::from_utf8_lossy`, or `String::from_utf8(..).unwrap()`.
	Utf8(Box<Expr>),
	/// `.len()` of bytes, a string, or a list.
	Len(Box<Expr>),
}

impl Expr {
	fn new(kind: Kind, ty: Type) -> Expr {
		Expr { kind, ty }
	}
}

impl Transcript {
	/// The transcript of `source`, resolved against the export's `api.json`.
	pub(crate) fn load(lock: &Path, source: &Path) -> Transcript {
		let lock: ir::Lock = serde_json::from_str(&std::fs::read_to_string(lock).unwrap()).unwrap();
		let mut transcript = Transcript {
			api: lock.api,
			names: lock.names,
			steps: Vec::new(),
			locals: BTreeMap::new(),
		};
		let (steps, locals) = parse::main(&transcript, &std::fs::read_to_string(source).unwrap());
		transcript.steps = steps;
		transcript.locals = locals;
		transcript
	}

	/// The function `id` names.
	pub(crate) fn function(&self, id: FnId) -> &Function {
		match id.owner {
			Owner::Free => &self.api.functions[id.index],
			Owner::Record(i) => &self.api.records[i].methods[id.index],
			Owner::Enum(i) => &self.api.enums[i].methods[id.index],
			Owner::Object(i) => &self.api.objects[i].methods[id.index],
		}
	}

	/// The type owning `id`, if any.
	pub(crate) fn owner(&self, id: FnId) -> Option<Type> {
		match id.owner {
			Owner::Free => None,
			Owner::Record(i) => Some(Type::Record(i)),
			Owner::Enum(i) => Some(Type::Enum(i)),
			Owner::Object(i) => Some(Type::Object(i)),
		}
	}

	/// The Rust path of a type, with its arguments, as `api.json`'s names key it.
	pub(crate) fn rust(&self, ty: &Type) -> String {
		let named = |path: &[String], args: &[Type]| match args {
			[] => path.join("::"),
			args => {
				let args: Vec<String> = args.iter().map(|a| self.rust(a)).collect();
				format!("{}<{}>", path.join("::"), args.join(", "))
			}
		};
		match ty {
			Type::Unit => "()".to_string(),
			Type::Bool => "bool".to_string(),
			Type::Int(int) => int.rust().to_string(),
			Type::NonZero(int) => format!("std::num::NonZero<{}>", int.rust()),
			Type::F32 => "f32".to_string(),
			Type::F64 => "f64".to_string(),
			Type::Char => "char".to_string(),
			Type::String => "String".to_string(),
			Type::Bytes => "bytes::Bytes".to_string(),
			Type::Duration => "std::time::Duration".to_string(),
			Type::Record(i) => {
				let r = &self.api.records[*i];
				match &r.shown {
					Some(shown) => shown.join("::"),
					None => named(&r.path, &r.args),
				}
			}
			Type::Enum(i) => {
				let e = &self.api.enums[*i];
				match &e.shown {
					Some(shown) => shown.join("::"),
					None => named(&e.path, &e.args),
				}
			}
			Type::Object(i) => {
				let o = &self.api.objects[*i];
				match &o.shown {
					Some(shown) => shown.join("::"),
					None => named(&o.path, &o.args),
				}
			}
			Type::Const(ir::Literal::Int(_, v)) => v.to_string(),
			Type::Const(ir::Literal::Bool(v)) => v.to_string(),
			other => panic!("no type argument is {other:?}"),
		}
	}

	/// The key `api.json`'s names give `id`, such as `fn api::Counter::new`.
	pub(crate) fn key(&self, id: FnId) -> String {
		let f = self.function(id);
		let name = f.path.last().unwrap();
		let mut path = match self.owner(id) {
			Some(owner) => format!("{}::{name}", self.rust(&owner)),
			None => f.path.join("::"),
		};
		if let Some(instance) = &f.instance
			&& !instance.named.is_empty()
		{
			let args: Vec<String> = instance.named.iter().map(|a| self.rust(a)).collect();
			path = format!("{path}<{}>", args.join(", "));
		}
		let kind = match f.access {
			Access::Field { set: false, .. } => "get",
			Access::Field { set: true, .. } => "set",
			_ => "fn",
		};
		format!("{kind} {path}")
	}

	/// `language`'s name for `id`.
	pub(crate) fn name(&self, language: &str, id: FnId) -> &str {
		self.named(language, &self.key(id))
	}

	/// `language`'s name for the type `ty`.
	pub(crate) fn type_name(&self, language: &str, ty: &Type) -> &str {
		self.named(language, &format!("type {}", self.rust(ty)))
	}

	/// The key the export spells `ty` by inside another's, as `vec_u16` holds `u16`: its C
	/// name past the prefix.
	pub(crate) fn spelling(&self, ty: &Type) -> String {
		let strip = |name: &str| {
			let prefix = self.named("C", "prefix");
			name[prefix.len() + 1..].to_string()
		};
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
			Type::Option(inner) => format!("option_{}", self.spelling(inner)),
			Type::List(_, inner) => format!("vec_{}", self.spelling(inner)),
			Type::Tuple(elements) => strip(self.tuple("C", elements)),
			Type::Record(_) | Type::Enum(_) | Type::Object(_) => strip(self.type_name("C", ty)),
			Type::Range(int) => format!("range_{}", int.rust()),
			Type::Bound(int) => format!("bound_{}", int.rust()),
			Type::Bounds(int) => format!("bounds_{}", int.rust()),
			Type::Ref(_, inner) | Type::Box(inner) => self.spelling(inner),
			other => panic!("no key spells {other:?}"),
		}
	}

	/// The key the export spells `ty` by alone, each tuple as Rust spells it, past any
	/// reference.
	pub(crate) fn spell(&self, ty: &Type) -> String {
		match owned(ty) {
			Type::Tuple(elements) => {
				let elements: Vec<String> = elements.iter().map(|e| self.spell(e)).collect();
				match elements.len() {
					1 => format!("({},)", elements[0]),
					_ => format!("({})", elements.join(", ")),
				}
			}
			ty => self.spelling(&ty),
		}
	}

	/// `language`'s name for the tuple of `elements`, where it names one.
	pub(crate) fn find_tuple(&self, language: &str, elements: &[Type]) -> Option<&str> {
		self.find(language, &format!("tuple {}", self.spell(&Type::Tuple(elements.to_vec()))))
	}

	/// `language`'s name for the tuple of `elements`.
	pub(crate) fn tuple(&self, language: &str, elements: &[Type]) -> &str {
		self.named(language, &format!("tuple {}", self.spell(&Type::Tuple(elements.to_vec()))))
	}

	/// `language`'s name for the exception an error of `ty` throws as.
	pub(crate) fn error(&self, language: &str, ty: &Type) -> &str {
		self.named(language, &format!("error {}", self.spell(ty)))
	}

	/// `language`'s name for the field `field` of the record `ty`.
	pub(crate) fn field(&self, language: &str, ty: &Type, field: &str) -> &str {
		self.named(language, &format!("type {} / field {field}", self.rust(unref(ty).base())))
	}

	/// `language`'s name for the variant `variant` of the enum `ty`.
	pub(crate) fn variant(&self, language: &str, ty: &Type, variant: &str) -> &str {
		self.named(language, &format!("type {} / variant {variant}", self.rust(unref(ty).base())))
	}

	/// `language`'s name for the field `field` of the data variant `variant` of `ty`.
	pub(crate) fn variant_field(&self, language: &str, ty: &Type, variant: &str, field: &str) -> &str {
		let key = format!("type {} / variant {variant} / field {field}", self.rust(unref(ty).base()));
		self.named(language, &key)
	}

	/// `language`'s name for the member of the std trait `t`, such as `Clone`, on `ty`.
	pub(crate) fn member(&self, language: &str, ty: &Type, t: &str) -> &str {
		self.named(language, &format!("type {} / {t}", self.rust(unref(ty).base())))
	}

	/// `language`'s name for each item, by its key.
	pub(crate) fn names(&self, language: &str) -> &BTreeMap<String, String> {
		&self.names[language]
	}

	/// `language`'s name for the item `key` names, if it names one.
	pub(crate) fn find(&self, language: &str, key: &str) -> Option<&str> {
		self.names[language].get(key).map(String::as_str)
	}

	/// `language`'s name for the item `key` names, such as `const api::LIMIT`.
	pub(crate) fn named(&self, language: &str, key: &str) -> &str {
		let names = &self.names[language];
		if let Some(name) = names.get(key) {
			return name;
		}
		// Swift keys an overload by its labels after the path.
		let prefix = format!("{key} / ");
		match names.range(prefix.clone()..).next() {
			Some((k, name)) if k.starts_with(&prefix) => name,
			_ => panic!("{language} names no {key}"),
		}
	}
}

/// `ty` without its references, as a key names it, and as Rust returns it to C: each
/// borrow a clone.
pub(crate) fn owned(ty: &Type) -> Type {
	match ty {
		Type::Ref(_, inner) => owned(inner),
		Type::Option(inner) => Type::Option(Box::new(owned(inner))),
		Type::Box(inner) => Type::Box(Box::new(owned(inner))),
		Type::List(c, inner) => Type::List(*c, Box::new(owned(inner))),
		Type::Tuple(elements) => Type::Tuple(elements.iter().map(owned).collect()),
		ty => ty.clone(),
	}
}

/// The key of an address type, `socket` for a socket's.
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

/// `ty` behind any references.
pub(crate) fn unref(ty: &Type) -> &Type {
	match ty {
		Type::Ref(_, inner) => unref(inner),
		ty => ty,
	}
}

/// Each name a step binds, in order, a closure's and an arm's included.
pub(crate) fn bound(pat: &Pat, out: &mut Vec<(String, Type)>) {
	match pat {
		Pat::Name(name, ty) => out.push((name.clone(), ty.clone())),
		Pat::Tuple(pats) => pats.iter().for_each(|p| bound(p, out)),
		Pat::Ignore(_) => {}
	}
}

/// Each expression a step holds, its arms' and closures' steps included.
pub(crate) fn each_expr(step: &Step, f: &mut dyn FnMut(&Expr)) {
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
pub(crate) fn walk(expr: &Expr, f: &mut dyn FnMut(&Expr)) {
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
