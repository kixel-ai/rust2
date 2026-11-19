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

pub(crate) mod dart;
pub(crate) mod cpp;
pub(crate) mod ir;
mod parse;
pub(crate) mod go;
pub(crate) mod kotlin;
pub(crate) mod python;
pub(crate) mod swift;
pub(crate) mod typescript;

use std::collections::BTreeMap;
use std::path::Path;

use ir::{Access, Function, Type};

/// The transcript, resolved against an export.
pub(crate) struct Transcript {
	pub(crate) api: ir::Api,
	names: BTreeMap<String, BTreeMap<String, String>>,
	pub(crate) steps: Vec<Step>,
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
	Match { value: Expr, ok: Arm, err: Arm },
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
	Gt,
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
		};
		transcript.steps = parse::main(&transcript, &std::fs::read_to_string(source).unwrap());
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

	/// `language`'s name for each item, by its key.
	pub(crate) fn names(&self, language: &str) -> &BTreeMap<String, String> {
		&self.names[language]
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
