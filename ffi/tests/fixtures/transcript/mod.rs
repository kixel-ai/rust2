//! The api fixture's transcript as data: `fixtures/api/examples/transcript.rs`'s `main`,
//! a typed subset of Rust, parsed and resolved against the export's `api.json`, then
//! written as each language's script, whose output must match the native one line for line.
//!
//! The subset is what every binding can say alike: `let`, `println!`, calls of the API's
//! functions and methods, field reads and writes, `match` on a `Result` or an `Option`,
//! `for` over a literal array, which unrolls, closures, and literals of the API's types.
//! A call of a function `transcript.rs` defines itself is each language's own, hand-written
//! beside its script under the same name: what tests a language's idiom, such as its
//! futures, or prints a value no binding shapes alike, such as `hex`. Anything else is an
//! error naming its line.

pub(crate) mod ir;
pub(crate) mod python;

use std::collections::BTreeMap;
use std::path::Path;

use ir::{Access, Collection, Function, Int, Mutability, Type};
use syn::spanned::Spanned;

/// The transcript, resolved against an export.
pub(crate) struct Transcript {
	pub(crate) api: ir::Api,
	names: BTreeMap<String, BTreeMap<String, String>>,
	pub(crate) steps: Vec<Step>,
	/// The functions `transcript.rs` defines, each a language's own: what each returns.
	pub(crate) locals: BTreeMap<String, Type>,
}

/// A statement of the script.
#[derive(Clone)]
pub(crate) enum Step {
	Let(Pat, Expr),
	Print(Vec<Piece>),
	/// An expression whose value is dropped, such as a call made for its effect.
	Do(Expr),
	/// `drop(name)`: the local's value drops here, not at the end of the script.
	Drop(String),
	/// `place.field = value`: a field's setter.
	Set(Expr, Expr),
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

/// What a `let`, an arm, or a closure's parameter binds.
#[derive(Clone)]
pub(crate) enum Pat {
	Name(String, Type),
	Tuple(Vec<Pat>),
	Ignore(Type),
}

/// A piece of a `println!`.
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
	/// `{:0Nx}`: lowercase hex, zero-padded to `N` digits.
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

#[derive(Clone)]
pub(crate) enum Kind {
	Unit,
	Bool(bool),
	Int(Num),
	/// A float's bits, an `f32`'s widened.
	Float(f64),
	Char(char),
	Str(String),
	Bytes(Vec<u8>),
	None,
	Some(Box<Expr>),
	Tuple(Vec<Expr>),
	/// The items of a collection, a map's as tuples of a key and its value.
	List(Vec<Expr>),
	/// A record built from its fields, in its order.
	Record(usize, Vec<Expr>),
	/// An enum's variant, from its fields.
	Variant(usize, usize, Vec<Expr>),
	/// A record's `Default`, or an object's.
	Default,
	Var(String),
	/// A constant of the IR's `constants`.
	Const(usize),
	/// A call: the receiver of a method taking `self`, then its arguments.
	Call(FnId, Option<Box<Expr>>, Vec<Expr>),
	/// A record's field, by index.
	Field(Box<Expr>, usize),
	/// A tuple's element.
	Element(Box<Expr>, usize),
	/// `Clone::clone`.
	Clone(Box<Expr>),
	/// `.unwrap()` of an `Option` or a `Result`, or `.expect(..)`.
	Unwrap(Box<Expr>),
	/// `.unwrap_err()` or `.err().unwrap()` of a `Result`.
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
		let text = std::fs::read_to_string(source).unwrap();
		let file = syn::parse_file(&text).unwrap_or_else(|e| panic!("{}: {e}", at(e.span())));
		let mut transcript = Transcript {
			api: lock.api,
			names: lock.names,
			steps: Vec::new(),
			locals: BTreeMap::new(),
		};
		let mut main = None;
		let mut units = Vec::new();
		for item in &file.items {
			match item {
				syn::Item::Fn(f) if f.sig.ident == "main" => main = Some(f),
				syn::Item::Fn(f) => {
					let ty = match &f.sig.output {
						syn::ReturnType::Default => Type::Unit,
						syn::ReturnType::Type(_, ty) => local_type(ty),
					};
					transcript.locals.insert(f.sig.ident.to_string(), ty);
				}
				syn::Item::Struct(s) if matches!(s.fields, syn::Fields::Unit) => units.push(s.ident.to_string()),
				_ => {}
			}
		}
		let main = main.expect("transcript.rs has a main");
		let mut scope = Scope {
			t: &transcript,
			vars: Vec::new(),
			units,
		};
		let steps = scope.block(&main.block.stmts);
		transcript.steps = steps;
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
			Type::Record(i) => named(&self.api.records[*i].path, &self.api.records[*i].args),
			Type::Enum(i) => named(&self.api.enums[*i].path, &self.api.enums[*i].args),
			Type::Object(i) => named(&self.api.objects[*i].path, &self.api.objects[*i].args),
			Type::Const(ir::Literal::Int(_, v)) => v.to_string(),
			Type::Const(ir::Literal::Bool(v)) => v.to_string(),
			other => panic!("no type argument is {other:?}"),
		}
	}

	/// The key `api.json`'s names give `id`, such as `fn api::Counter::new`.
	pub(crate) fn key(&self, id: FnId) -> String {
		let f = self.function(id);
		let owner = match id.owner {
			Owner::Free => None,
			Owner::Record(i) => Some(self.rust(&Type::Record(i))),
			Owner::Enum(i) => Some(self.rust(&Type::Enum(i))),
			Owner::Object(i) => Some(self.rust(&Type::Object(i))),
		};
		let name = f.path.last().unwrap();
		let mut path = match owner {
			Some(owner) => format!("{owner}::{name}"),
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

/// `file:line:column` of a span in `transcript.rs`.
fn at(span: proc_macro2::Span) -> String {
	let start = span.start();
	format!("transcript.rs:{}:{}", start.line, start.column + 1)
}

/// Fail at `node`'s span: a construct the subset lacks, or one the API does not have.
fn fail(node: &dyn Spanned, message: impl std::fmt::Display) -> ! {
	panic!("{}: {message}", at(node.span()))
}

/// The type of a local function's return, which only names the standard library's.
fn local_type(ty: &syn::Type) -> Type {
	let syn::Type::Path(path) = ty else {
		fail(ty, "a local function returns a named type")
	};
	let name = path.path.segments.last().unwrap().ident.to_string();
	match name.as_str() {
		"String" => Type::String,
		"bool" => Type::Bool,
		"u8" => Type::Int(Int::U8),
		"u32" => Type::Int(Int::U32),
		"u64" => Type::Int(Int::U64),
		"usize" => Type::Int(Int::Usize),
		"i64" => Type::Int(Int::I64),
		// What a hook makes is the API's, typed where it is used.
		_ => fail(ty, format!("a local function returning {name}")),
	}
}

/// A local in scope: its type, or a loop's literal, typed at each use.
enum Var {
	Typed(String, Type),
	Alias(String, syn::Expr),
}

struct Scope<'t> {
	t: &'t Transcript,
	vars: Vec<Var>,
	/// The unit structs `transcript.rs` defines: each language's own implementation.
	units: Vec<String>,
}

impl Scope<'_> {
	fn block(&mut self, stmts: &[syn::Stmt]) -> Vec<Step> {
		let depth = self.vars.len();
		let mut steps = Vec::new();
		for stmt in stmts {
			self.stmt(stmt, &mut steps);
		}
		self.vars.truncate(depth);
		steps
	}

	fn lookup(&self, name: &str) -> Option<&Var> {
		self.vars.iter().rev().find(|v| match v {
			Var::Typed(n, _) | Var::Alias(n, _) => n == name,
		})
	}

	fn stmt(&mut self, stmt: &syn::Stmt, steps: &mut Vec<Step>) {
		match stmt {
			syn::Stmt::Local(local) => {
				let Some(init) = &local.init else {
					fail(local, "a let without a value")
				};
				if init.diverge.is_some() {
					fail(local, "a let-else");
				}
				let expected = match &local.pat {
					syn::Pat::Type(typed) => Some(self.annotated(&typed.ty)),
					_ => None,
				};
				let value = match &expected {
					Some(ty) => self.check(&init.expr, ty),
					None => self.infer(&init.expr),
				};
				let pat = self.bind(&local.pat, &value.ty);
				steps.push(Step::Let(pat, value));
			}
			syn::Stmt::Macro(m) => steps.push(self.mac(&m.mac)),
			syn::Stmt::Expr(expr, _) => self.expr_stmt(expr, steps),
			syn::Stmt::Item(item) => fail(item, "an item inside main"),
		}
	}

	fn expr_stmt(&mut self, expr: &syn::Expr, steps: &mut Vec<Step>) {
		match expr {
			syn::Expr::Macro(m) => steps.push(self.mac(&m.mac)),
			syn::Expr::ForLoop(f) => {
				let syn::Expr::Array(items) = &*f.expr else {
					fail(&f.expr, "a for loop over anything but an array literal")
				};
				let syn::Pat::Ident(name) = &*f.pat else {
					fail(&f.pat, "a for loop binding anything but a name")
				};
				for item in &items.elems {
					self.vars.push(Var::Alias(name.ident.to_string(), item.clone()));
					let body = self.block(&f.body.stmts);
					self.vars.pop();
					steps.extend(body);
				}
			}
			syn::Expr::Match(m) => steps.push(self.matched(m)),
			syn::Expr::Assign(a) => {
				let place = self.infer(&a.left);
				let value = self.check(&a.right, &place.ty);
				steps.push(Step::Set(place, value));
			}
			syn::Expr::Call(call) if path_is(&call.func, "drop") => {
				let [arg] = call.args.iter().collect::<Vec<_>>()[..] else {
					fail(call, "drop of one local")
				};
				let syn::Expr::Path(p) = arg else {
					fail(arg, "drop of anything but a local")
				};
				let name = p.path.get_ident().unwrap().to_string();
				let Some(Var::Typed(..)) = self.lookup(&name) else {
					fail(arg, format!("drop of {name}, which is no local"))
				};
				steps.push(Step::Drop(name));
			}
			expr => steps.push(Step::Do(self.infer(expr))),
		}
	}

	/// A `let`'s pattern, binding its names to the parts of `ty`.
	fn bind(&mut self, pat: &syn::Pat, ty: &Type) -> Pat {
		match pat {
			syn::Pat::Type(typed) => self.bind(&typed.pat, ty),
			syn::Pat::Ident(ident) => {
				let name = ident.ident.to_string();
				self.vars.push(Var::Typed(name.clone(), ty.clone()));
				Pat::Name(name, ty.clone())
			}
			syn::Pat::Wild(_) => Pat::Ignore(ty.clone()),
			syn::Pat::Tuple(tuple) => {
				let Type::Tuple(elements) = unref(ty) else {
					fail(pat, format!("a tuple pattern of {ty:?}"))
				};
				if elements.len() != tuple.elems.len() {
					fail(pat, "a tuple pattern of another length");
				}
				let elements = elements.clone();
				Pat::Tuple(tuple.elems.iter().zip(&elements).map(|(p, ty)| self.bind(p, ty)).collect())
			}
			pat => fail(pat, "a pattern other than a name, `_`, or a tuple"),
		}
	}

	/// The type an annotation on a `let` names.
	fn annotated(&self, ty: &syn::Type) -> Type {
		let syn::Type::Path(path) = ty else {
			fail(ty, "an annotation other than a path")
		};
		let last = path.path.segments.last().unwrap();
		match last.ident.to_string().as_str() {
			"Vec" if matches!(&last.arguments, syn::PathArguments::AngleBracketed(args) if matches!(args.args.first(), Some(syn::GenericArgument::Type(syn::Type::Path(p))) if p.path.is_ident("String"))) => {
				Type::List(Collection::Vec, Box::new(Type::String))
			}
			_ => self.type_of_path(&path.path).unwrap_or_else(|| fail(ty, "an annotation of a type the API lacks")),
		}
	}

	/// The API's type a path names, such as `api::Batch` or `std::net::SocketAddr`.
	fn type_of_path(&self, path: &syn::Path) -> Option<Type> {
		let names: Vec<String> = path.segments.iter().map(|s| s.ident.to_string()).collect();
		let last = names.last()?.as_str();
		let api = &self.t.api;
		let matches = |p: &[String]| p.last().map(String::as_str) == Some(last) && suffix(p, &names);
		let found: Vec<Type> = (api.records.iter().enumerate())
			.filter(|(_, r)| matches(&r.path))
			.map(|(i, _)| Type::Record(i))
			.chain((api.enums.iter().enumerate()).filter(|(_, e)| matches(&e.path)).map(|(i, _)| Type::Enum(i)))
			.chain((api.objects.iter().enumerate()).filter(|(_, o)| matches(&o.path)).map(|(i, _)| Type::Object(i)))
			.collect();
		match (found.as_slice(), last) {
			([one], _) => Some(one.clone()),
			// A generic type at its default, or its one instance.
			([first, ..], _) if found.iter().all(|t| self.t.rust(t).split('<').next() == self.t.rust(first).split('<').next()) => {
				let plain = found.iter().find(|t| !self.t.rust(t).contains('<'));
				Some(plain.unwrap_or(first).clone())
			}
			(_, "SocketAddr") => Some(Type::SocketAddr(ir::Family::Any)),
			(_, "Duration") => Some(Type::Duration),
			(_, "Bytes") => Some(Type::Bytes),
			_ => None,
		}
	}

	fn mac(&mut self, mac: &syn::Macro) -> Step {
		if !mac.path.is_ident("println") {
			fail(mac, "a macro other than println!");
		}
		let args = mac
			.parse_body_with(syn::punctuated::Punctuated::<syn::Expr, syn::Token![,]>::parse_terminated)
			.unwrap_or_else(|e| fail(mac, e));
		let mut args = args.into_iter();
		let Some(syn::Expr::Lit(syn::ExprLit {
			lit: syn::Lit::Str(format),
			..
		})) = args.next()
		else {
			fail(mac, "println! without a format string")
		};
		let args: Vec<syn::Expr> = args.collect();
		// The arguments are parsed out of the macro's tokens, so they live as long as the
		// transcript does by leaking: a test process parses one file once.
		let args: &[syn::Expr] = Box::leak(args.into_boxed_slice());
		let pieces = self.format(&format, args);
		Step::Print(pieces)
	}

	/// The pieces of a format string and its arguments.
	fn format(&mut self, lit: &syn::LitStr, args: &[syn::Expr]) -> Vec<Piece> {
		let text = lit.value();
		let mut pieces = Vec::new();
		let mut literal = String::new();
		let mut next = 0;
		let mut chars = text.chars().peekable();
		while let Some(c) = chars.next() {
			match c {
				'{' if chars.peek() == Some(&'{') => {
					chars.next();
					literal.push('{');
				}
				'}' if chars.peek() == Some(&'}') => {
					chars.next();
					literal.push('}');
				}
				'{' => {
					let mut inner = String::new();
					for c in chars.by_ref() {
						if c == '}' {
							break;
						}
						inner.push(c);
					}
					let (name, spec) = inner.split_once(':').unwrap_or((&inner, ""));
					let spec = match spec {
						"" => Spec::Display,
						"?" => Spec::Debug,
						s if s.starts_with('0') && s.ends_with('x') => Spec::Hex(s[1..s.len() - 1].parse().unwrap()),
						s if s.starts_with('0') => Spec::Padded(s[1..].parse().unwrap()),
						s => fail(lit, format!("the format {s}")),
					};
					let value = if name.is_empty() {
						let arg = args.get(next).unwrap_or_else(|| fail(lit, "too few arguments"));
						next += 1;
						self.infer(arg)
					} else {
						let expr: &syn::Expr = Box::leak(Box::new(syn::parse_str::<syn::Expr>(name).unwrap()));
						self.infer_at(expr, lit)
					};
					if !literal.is_empty() {
						pieces.push(Piece::Text(std::mem::take(&mut literal)));
					}
					pieces.push(Piece::Value(value, spec));
				}
				c => literal.push(c),
			}
		}
		if next != args.len() {
			fail(lit, "too many arguments");
		}
		if !literal.is_empty() {
			pieces.push(Piece::Text(literal));
		}
		pieces
	}

	/// `infer`, failing at `at`'s span, for an expression parsed from a format string.
	fn infer_at(&mut self, expr: &syn::Expr, at: &dyn Spanned) -> Expr {
		let syn::Expr::Path(p) = expr else { fail(at, "an inline format argument other than a name") };
		let name = p.path.segments.last().unwrap().ident.to_string();
		if self.lookup(&name).is_none() && self.constant(&p.path).is_none() {
			fail(at, format!("{name} is no local or constant"));
		}
		self.infer(expr)
	}

	fn matched(&mut self, m: &syn::ExprMatch) -> Step {
		let value = self.infer(&m.expr);
		let (ok_ty, err_ty, option) = match &value.ty {
			Type::Result(ok, err) => ((**ok).clone(), Some((**err).clone()), false),
			Type::Option(inner) => ((**inner).clone(), None, true),
			ty => fail(&m.expr, format!("a match on {ty:?}")),
		};
		let mut ok = None;
		let mut err = None;
		for arm in &m.arms {
			let (variant, inner) = match &arm.pat {
				syn::Pat::TupleStruct(ts) => {
					let name = ts.path.segments.last().unwrap().ident.to_string();
					let [inner] = ts.elems.iter().collect::<Vec<_>>()[..] else {
						fail(ts, "a variant of one field")
					};
					(name, Some(inner))
				}
				syn::Pat::Ident(i) if i.ident == "None" => ("None".to_string(), None),
				syn::Pat::Path(p) if p.path.is_ident("None") => ("None".to_string(), None),
				pat => fail(pat, "an arm other than Ok, Err, Some, or None"),
			};
			let depth = self.vars.len();
			let (slot, ty) = match (variant.as_str(), option) {
				("Ok", false) | ("Some", true) => (&mut ok, Some(&ok_ty)),
				("Err", false) => (&mut err, err_ty.as_ref()),
				("None", true) => (&mut err, None),
				_ => fail(&arm.pat, format!("{variant} in a match on {:?}", value.ty)),
			};
			let pat = inner.map(|p| self.bind(p, ty.unwrap()));
			let mut steps = Vec::new();
			match &*arm.body {
				syn::Expr::Block(b) => {
					for stmt in &b.block.stmts {
						self.stmt(stmt, &mut steps);
					}
				}
				body => self.expr_stmt(body, &mut steps),
			}
			self.vars.truncate(depth);
			*slot = Some(Arm { pat, steps });
		}
		let (Some(ok), Some(err)) = (ok, err) else {
			fail(m, "a match without both arms")
		};
		Step::Match { value, ok, err }
	}

	fn constant(&self, path: &syn::Path) -> Option<usize> {
		let names: Vec<String> = path.segments.iter().map(|s| s.ident.to_string()).collect();
		(self.t.api.constants.iter()).position(|c| suffix(&c.path, &names) && c.path.last() == names.last())
	}

	/// An expression whose type follows from itself.
	fn infer(&mut self, expr: &syn::Expr) -> Expr {
		match expr {
			syn::Expr::Paren(p) => self.infer(&p.expr),
			syn::Expr::Reference(r) => self.infer(&r.expr),
			syn::Expr::Group(g) => self.infer(&g.expr),
			syn::Expr::Path(p) => {
				if let Some(name) = p.path.get_ident().map(|i| i.to_string()) {
					match self.lookup(&name) {
						Some(Var::Typed(_, ty)) => return Expr::new(Kind::Var(name), ty.clone()),
						Some(Var::Alias(_, expr)) => {
							let expr = expr.clone();
							return self.infer(&expr);
						}
						None => {}
					}
					if self.units.contains(&name) {
						fail(p, format!("{name}, a value of a local type, where nothing types it"));
					}
				}
				if let Some(c) = self.constant(&p.path) {
					let ty = match self.t.api.constants[c].value {
						ir::Literal::Bool(_) => Type::Bool,
						ir::Literal::Int(int, _) => Type::Int(int),
					};
					return Expr::new(Kind::Const(c), ty);
				}
				// A constant the IR keeps as a function, such as `api::BOXED`.
				if let Some(id) = self.function(&p.path, None, None) {
					let f = self.t.function(id);
					if f.access == Access::Const {
						return Expr::new(Kind::Call(id, None, Vec::new()), f.ok.clone());
					}
				}
				// A plain enum's variant.
				if let Some(e) = self.variant(&p.path, &[]) {
					return e;
				}
				fail(p, "a path that names no local, constant, or variant")
			}
			syn::Expr::Lit(lit) => match &lit.lit {
				syn::Lit::Str(s) => Expr::new(Kind::Str(s.value()), Type::Ref(Mutability::Not, Box::new(Type::Str))),
				syn::Lit::Bool(b) => Expr::new(Kind::Bool(b.value), Type::Bool),
				syn::Lit::ByteStr(b) => Expr::new(Kind::Bytes(b.value()), Type::Ref(Mutability::Not, Box::new(Type::Slice))),
				syn::Lit::Int(i) if !i.suffix().is_empty() => {
					let int = int_named(i.suffix()).unwrap_or_else(|| fail(i, "an integer suffix"));
					self.check(expr, &Type::Int(int))
				}
				lit => fail(lit, "a literal whose type nothing gives"),
			},
			syn::Expr::Call(call) => self.call(call, None),
			syn::Expr::MethodCall(m) => self.method(m, None),
			syn::Expr::Field(f) => self.field(f),
			syn::Expr::Struct(s) => self.structure(s, None),
			syn::Expr::Binary(b) => self.binary(b),
			syn::Expr::Unary(u) if matches!(u.op, syn::UnOp::Deref(_)) => self.infer(&u.expr),
			syn::Expr::Tuple(t) => {
				let elements: Vec<Expr> = t.elems.iter().map(|e| self.infer(e)).collect();
				let ty = Type::Tuple(elements.iter().map(|e| e.ty.clone()).collect());
				Expr::new(Kind::Tuple(elements), ty)
			}
			expr => fail(expr, "an expression outside the transcript's subset"),
		}
	}

	/// An expression that must be of type `ty`, such as an argument: literals take their
	/// type from it, and Rust's own conversions between the forms of a value, such as
	/// `.to_string()` or `&`, are the value itself.
	fn check(&mut self, expr: &syn::Expr, ty: &Type) -> Expr {
		let base = unref(ty);
		match expr {
			syn::Expr::Paren(p) => return self.check(&p.expr, ty),
			syn::Expr::Group(g) => return self.check(&g.expr, ty),
			syn::Expr::Reference(r) => return self.check(&r.expr, ty),
			syn::Expr::Unary(u) if matches!(u.op, syn::UnOp::Deref(_)) => return self.check(&u.expr, ty),
			// `&b"lo"[..]`: the whole slice.
			syn::Expr::Index(i) if matches!(&*i.index, syn::Expr::Range(r) if r.start.is_none() && r.end.is_none()) => {
				return self.check(&i.expr, ty);
			}
			syn::Expr::Path(p) if p.path.get_ident().is_some() => {
				let name = p.path.get_ident().unwrap().to_string();
				if let Some(Var::Alias(_, alias)) = self.lookup(&name) {
					let alias = alias.clone();
					return self.check(&alias, ty);
				}
				if self.units.contains(&name) {
					return Expr::new(Kind::Local(name, Vec::new()), ty.clone());
				}
				if name == "None" {
					return match base {
						Type::Option(_) => Expr::new(Kind::None, ty.clone()),
						_ => fail(expr, format!("None as {ty:?}")),
					};
				}
			}
			syn::Expr::Path(p) if path_is(expr, "Vec::new") || path_is(expr, "String::new") || path_is(expr, "Bytes::new") => {
				let _ = p;
				return self.empty(expr, ty);
			}
			syn::Expr::MethodCall(m) if CONVERSIONS.contains(&m.method.to_string().as_str()) && m.args.is_empty() => {
				return self.check(&m.receiver, ty);
			}
			// `[..].into_iter().map(..).collect()`, as a map's or a set's items are built.
			syn::Expr::MethodCall(m) if m.method == "collect" => return self.collected(m, ty),
			syn::Expr::Call(call) => {
				let func = &*call.func;
				let one = call.args.len() == 1;
				if one && ["String::from", "Bytes::from_static", "Box::new", "PathBuf::from", "Path::new", "Bytes::from"].iter().any(|p| path_is(func, p)) {
					return self.check(&call.args[0], ty);
				}
				if one && path_is(func, "Some") {
					let Type::Option(inner) = base else {
						fail(expr, format!("Some as {ty:?}"))
					};
					let inner = (**inner).clone();
					let value = self.check(&call.args[0], &inner);
					return Expr::new(Kind::Some(Box::new(value)), ty.clone());
				}
				if call.args.is_empty() && (path_is(func, "Default::default") || path_is(func, "Box::default")) {
					return self.empty(expr, ty);
				}
				if let syn::Expr::Path(p) = func
					&& let Some(name) = p.path.get_ident()
					&& self.t.locals.contains_key(&name.to_string())
				{
					let args = call.args.iter().map(|a| self.infer(a)).collect();
					return Expr::new(Kind::Local(name.to_string(), args), ty.clone());
				}
				return self.call(call, Some(ty));
			}
			syn::Expr::Macro(m) if m.mac.path.is_ident("vec") => {
				let items = m
					.mac
					.parse_body_with(syn::punctuated::Punctuated::<syn::Expr, syn::Token![,]>::parse_terminated)
					.unwrap_or_else(|e| fail(m, e));
				let items: &[syn::Expr] = Box::leak(items.into_iter().collect::<Vec<_>>().into_boxed_slice());
				return self.items(expr, items.iter(), ty);
			}
			syn::Expr::Array(a) => return self.items(expr, a.elems.iter(), ty),
			syn::Expr::Lit(lit) => return self.literal(lit, ty),
			syn::Expr::Unary(u) if matches!(u.op, syn::UnOp::Neg(_)) => {
				if let syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Int(i), .. }) = &*u.expr {
					let mut e = self.literal(&syn::ExprLit { attrs: Vec::new(), lit: syn::Lit::Int((*i).clone()) }, ty);
					if let Kind::Int(n) = &mut e.kind {
						n.negative = true;
					}
					if let Kind::Float(f) = &mut e.kind {
						*f = -*f;
					}
					return e;
				}
			}
			syn::Expr::Tuple(t) => {
				let Type::Tuple(elements) = base else {
					fail(expr, format!("a tuple as {ty:?}"))
				};
				if elements.len() != t.elems.len() {
					fail(expr, "a tuple of another length");
				}
				let elements = elements.clone();
				let values = t.elems.iter().zip(&elements).map(|(e, ty)| self.check(e, ty)).collect();
				return Expr::new(Kind::Tuple(values), ty.clone());
			}
			syn::Expr::Range(r) => return self.range(r, ty),
			syn::Expr::Closure(c) => return self.closure(c, ty),
			syn::Expr::Struct(s) => return self.structure(s, Some(ty)),
			syn::Expr::MethodCall(m) => return self.method(m, Some(ty)),
			_ => {}
		}
		let value = self.infer(expr);
		if !fits(&value.ty, ty) {
			fail(expr, format!("{:?} where {ty:?} is taken", value.ty));
		}
		value
	}

	/// An empty collection, string, or a type's default.
	fn empty(&mut self, expr: &syn::Expr, ty: &Type) -> Expr {
		match unref(ty) {
			Type::String | Type::Str => Expr::new(Kind::Str(String::new()), ty.clone()),
			Type::Bytes | Type::Vec | Type::Slice => Expr::new(Kind::Bytes(Vec::new()), ty.clone()),
			Type::List(..) => Expr::new(Kind::List(Vec::new()), ty.clone()),
			Type::Box(inner) if matches!(**inner, Type::List(..)) => Expr::new(Kind::List(Vec::new()), ty.clone()),
			Type::Option(_) => Expr::new(Kind::None, ty.clone()),
			Type::Record(_) | Type::Object(_) => Expr::new(Kind::Default, ty.clone()),
			other => fail(expr, format!("an empty {other:?}")),
		}
	}

	/// The items of a list literal, as a list of `ty` or its bytes.
	fn items<'e>(&mut self, expr: &syn::Expr, items: impl Iterator<Item = &'e syn::Expr>, ty: &Type) -> Expr {
		match unref(ty) {
			Type::List(_, item) => {
				let item = (**item).clone();
				let items = items.map(|e| self.check(e, &item)).collect();
				Expr::new(Kind::List(items), ty.clone())
			}
			Type::Vec | Type::Slice | Type::Bytes | Type::Array(_) => {
				let bytes = items
					.map(|e| match self.check(e, &Type::Int(Int::U8)).kind {
						Kind::Int(n) => n.magnitude as u8,
						_ => fail(e, "a byte other than a literal"),
					})
					.collect();
				Expr::new(Kind::Bytes(bytes), ty.clone())
			}
			other => fail(expr, format!("a list as {other:?}")),
		}
	}

	/// `items.into_iter().map(..).collect()` as `ty`: the items, each mapped.
	fn collected(&mut self, m: &syn::ExprMethodCall, ty: &Type) -> Expr {
		let mut receiver = &*m.receiver;
		let mut map = None;
		loop {
			match receiver {
				syn::Expr::MethodCall(inner) if CONVERSIONS.contains(&inner.method.to_string().as_str()) || inner.method == "into_iter" => {
					receiver = &inner.receiver;
				}
				syn::Expr::MethodCall(inner) if inner.method == "map" && map.is_none() => {
					map = Some(&inner.args[0]);
					receiver = &inner.receiver;
				}
				_ => break,
			}
		}
		let syn::Expr::Array(array) = receiver else {
			fail(m, "a collect of anything but an array literal")
		};
		let Type::List(_, item) = unref(ty) else {
			fail(m, format!("a collect as {ty:?}"))
		};
		let item = (**item).clone();
		let Some(map) = map else {
			return self.items(receiver, array.elems.iter(), ty);
		};
		// The map is a closure of a tuple pattern: each item, bound by it, is its body.
		let syn::Expr::Closure(closure) = map else {
			fail(map, "a map of anything but a closure")
		};
		let [syn::Pat::Tuple(pat)] = closure.inputs.iter().collect::<Vec<_>>()[..] else {
			fail(closure, "a map's closure of one tuple pattern")
		};
		let items = array
			.elems
			.iter()
			.map(|element| {
				let syn::Expr::Tuple(values) = element else {
					fail(element, "an item other than a tuple")
				};
				let depth = self.vars.len();
				for (p, value) in pat.elems.iter().zip(&values.elems) {
					let syn::Pat::Ident(name) = p else { fail(p, "a name") };
					self.vars.push(Var::Alias(name.ident.to_string(), value.clone()));
				}
				let value = self.check(&closure.body, &item);
				self.vars.truncate(depth);
				value
			})
			.collect();
		Expr::new(Kind::List(items), ty.clone())
	}

	fn literal(&mut self, lit: &syn::ExprLit, ty: &Type) -> Expr {
		let base = unref(ty);
		match (&lit.lit, base) {
			(syn::Lit::Str(s), Type::Str | Type::String | Type::Path | Type::PathBuf) => Expr::new(Kind::Str(s.value()), ty.clone()),
			(syn::Lit::ByteStr(b), Type::Slice | Type::Vec | Type::Bytes | Type::Array(_)) => Expr::new(Kind::Bytes(b.value()), ty.clone()),
			(syn::Lit::Byte(b), Type::Int(Int::U8)) => Expr::new(Kind::Int(Num { negative: false, magnitude: b.value().into() }), ty.clone()),
			(syn::Lit::Bool(b), Type::Bool) => Expr::new(Kind::Bool(b.value), ty.clone()),
			(syn::Lit::Char(c), Type::Char) => Expr::new(Kind::Char(c.value()), ty.clone()),
			(syn::Lit::Int(i), Type::Int(_) | Type::U128 | Type::NonZero(_)) => {
				let magnitude: u128 = i.base10_parse().unwrap_or_else(|e| fail(i, e));
				Expr::new(Kind::Int(Num { negative: false, magnitude }), ty.clone())
			}
			(syn::Lit::Float(f), Type::F32 | Type::F64) => Expr::new(Kind::Float(f.base10_parse().unwrap_or_else(|e| fail(f, e))), ty.clone()),
			(syn::Lit::Int(i), Type::F32 | Type::F64) => Expr::new(Kind::Float(i.base10_parse().unwrap_or_else(|e| fail(i, e))), ty.clone()),
			(lit, _) => fail(lit, format!("this literal as {ty:?}")),
		}
	}

	fn range(&mut self, r: &syn::ExprRange, ty: &Type) -> Expr {
		match unref(ty) {
			Type::Range(int) => {
				let int = Type::Int(*int);
				let (Some(start), Some(end)) = (&r.start, &r.end) else {
					fail(r, "a Range without both ends")
				};
				if matches!(r.limits, syn::RangeLimits::Closed(_)) {
					fail(r, "an inclusive range as a Range");
				}
				let elements = vec![self.check(start, &int), self.check(end, &int)];
				Expr::new(Kind::Tuple(elements), ty.clone())
			}
			Type::Bounds(int) => {
				// The start and end bounds, each a `Bound`'s variant.
				let int = *int;
				let bound = |s: &mut Self, e: Option<&syn::Expr>, kind: &str| -> Expr {
					let ty = Type::Bound(int);
					match e {
						None => Expr::new(Kind::Str("Unbounded".to_string()), ty),
						Some(e) => {
							let v = s.check(e, &Type::Int(int));
							Expr::new(Kind::Tuple(vec![Expr::new(Kind::Str(kind.to_string()), Type::Str), v]), ty)
						}
					}
				};
				let end_kind = match r.limits {
					syn::RangeLimits::HalfOpen(_) => "Excluded",
					syn::RangeLimits::Closed(_) => "Included",
				};
				let start = bound(self, r.start.as_deref(), "Included");
				let end = bound(self, r.end.as_deref(), end_kind);
				Expr::new(Kind::Tuple(vec![start, end]), ty.clone())
			}
			other => fail(r, format!("a range as {other:?}")),
		}
	}

	fn closure(&mut self, c: &syn::ExprClosure, ty: &Type) -> Expr {
		let Type::Closure(closure) = unref(ty) else {
			fail(c, format!("a closure as {ty:?}"))
		};
		let closure = (**closure).clone();
		if closure.params.len() != c.inputs.len() {
			fail(c, "a closure of another arity");
		}
		let depth = self.vars.len();
		let params = c.inputs.iter().zip(&closure.params).map(|(p, ty)| self.bind(p, ty)).collect();
		let mut steps = Vec::new();
		let value = match &*c.body {
			syn::Expr::Block(b) => {
				let (last, init) = b.block.stmts.split_last().unwrap_or_else(|| fail(b, "an empty closure"));
				for stmt in init {
					self.stmt(stmt, &mut steps);
				}
				match last {
					syn::Stmt::Expr(e, None) => self.check(e, &closure.ok),
					stmt => {
						self.stmt(stmt, &mut steps);
						Expr::new(Kind::Unit, Type::Unit)
					}
				}
			}
			syn::Expr::Macro(m) => {
				steps.push(self.mac(&m.mac));
				Expr::new(Kind::Unit, Type::Unit)
			}
			body => self.check(body, &closure.ok),
		};
		self.vars.truncate(depth);
		Expr::new(Kind::Closure(params, steps, Box::new(value)), ty.clone())
	}

	fn binary(&mut self, b: &syn::ExprBinary) -> Expr {
		let op = match b.op {
			syn::BinOp::Eq(_) => Op::Eq,
			syn::BinOp::Ne(_) => Op::Ne,
			syn::BinOp::Lt(_) => Op::Lt,
			syn::BinOp::Gt(_) => Op::Gt,
			syn::BinOp::Add(_) => Op::Add,
			syn::BinOp::Sub(_) => Op::Sub,
			syn::BinOp::Mul(_) => Op::Mul,
			syn::BinOp::Rem(_) => Op::Rem,
			syn::BinOp::And(_) => Op::And,
			_ => fail(b, "this operator"),
		};
		// The side whose type follows from itself types the other.
		let (left, right) = match literalish(&b.left) {
			false => {
				let left = self.infer(&b.left);
				let right = self.check(&b.right, &left.ty.clone());
				(left, right)
			}
			true => {
				let right = self.infer(&b.right);
				let left = self.check(&b.left, &right.ty.clone());
				(left, right)
			}
		};
		let ty = match op {
			Op::Eq | Op::Ne | Op::Lt | Op::Gt | Op::And => Type::Bool,
			_ => unref(&left.ty).clone(),
		};
		Expr::new(Kind::Binary(op, Box::new(left), Box::new(right)), ty)
	}

	fn field(&mut self, f: &syn::ExprField) -> Expr {
		let base = self.infer(&f.base);
		match &f.member {
			syn::Member::Unnamed(i) => {
				let Type::Tuple(elements) = unref(&base.ty) else {
					fail(f, format!("an element of {:?}", base.ty))
				};
				let ty = elements.get(i.index as usize).unwrap_or_else(|| fail(f, "no such element")).clone();
				Expr::new(Kind::Element(Box::new(base), i.index as usize), ty)
			}
			syn::Member::Named(name) => {
				let name = name.to_string();
				let owner = unref(&base.ty).clone();
				if let Type::Record(r) = owner
					&& let Some(i) = self.t.api.records[r].fields.iter().position(|f| f.name == name)
				{
					let ty = self.t.api.records[r].fields[i].ty.clone();
					return Expr::new(Kind::Field(Box::new(base), i), ty);
				}
				// A getter: an object's field, or an opaque value's.
				let id = self.method_of(&owner, &name, |f| matches!(f.access, Access::Field { set: false, .. }));
				let Some(id) = id else {
					fail(f, format!("no field {name} on {owner:?}"))
				};
				let ty = self.t.function(id).ok.clone();
				Expr::new(Kind::Call(id, Some(Box::new(base)), Vec::new()), ty)
			}
		}
	}

	/// The method `name` of `owner`'s type for which `which` holds.
	fn method_of(&self, owner: &Type, name: &str, which: impl Fn(&Function) -> bool) -> Option<FnId> {
		let (owner, methods) = match unref(owner) {
			Type::Record(i) => (Owner::Record(*i), &self.t.api.records[*i].methods),
			Type::Enum(i) => (Owner::Enum(*i), &self.t.api.enums[*i].methods),
			Type::Object(i) => (Owner::Object(*i), &self.t.api.objects[*i].methods),
			_ => return None,
		};
		let index = methods.iter().position(|f| f.path.last().map(String::as_str) == Some(name) && which(f))?;
		Some(FnId { owner, index })
	}

	/// A struct literal: a record's, or an enum's struct variant's.
	fn structure(&mut self, s: &syn::ExprStruct, ty: Option<&Type>) -> Expr {
		if let Some(e) = self.variant_struct(s) {
			return e;
		}
		let named = self.type_of_path(&s.path);
		// A generic record's instance: the one the expected type names, else the one whose
		// fields the literal's types fit.
		let candidates: Vec<usize> = match (ty.map(unref), &named) {
			(Some(Type::Record(i)), _) => vec![*i],
			(_, Some(Type::Record(i))) => {
				let base = self.t.api.records[*i].path.clone();
				(0..self.t.api.records.len()).filter(|r| self.t.api.records[*r].path == base).collect()
			}
			_ => fail(s, "a struct literal of no record"),
		};
		for (n, r) in candidates.iter().enumerate() {
			let record = &self.t.api.records[*r];
			let fields: Vec<(String, Type)> = record.fields.iter().map(|f| (f.name.clone(), f.ty.clone())).collect();
			if fields.len() != s.fields.len() {
				fail(s, "a record literal without each field");
			}
			// Try each instance in turn, keeping the first whose fields all fit.
			let last = n + 1 == candidates.len();
			let fits_all = last
				|| s.fields.iter().all(|fv| {
					let syn::Member::Named(name) = &fv.member else { return false };
					let Some((_, ty)) = fields.iter().find(|(n, _)| name == n) else { return false };
					self.fits_expr(&fv.expr, ty)
				});
			if !fits_all {
				continue;
			}
			let mut values = Vec::new();
			for (name, ty) in &fields {
				let fv = s
					.fields
					.iter()
					.find(|fv| matches!(&fv.member, syn::Member::Named(n) if n == name))
					.unwrap_or_else(|| fail(s, format!("no field {name}")));
				values.push(self.check(&fv.expr, ty));
			}
			return Expr::new(Kind::Record(*r, values), Type::Record(*r));
		}
		fail(s, "a record literal fitting no instance")
	}

	/// Whether `expr` may be of type `ty`, judged by what it plainly is without checking it:
	/// a local's type, a struct literal's, or a suffixed literal's. What it cannot judge fits.
	fn fits_expr(&self, expr: &syn::Expr, ty: &Type) -> bool {
		let guess = match expr {
			syn::Expr::Reference(r) => return self.fits_expr(&r.expr, ty),
			syn::Expr::Paren(p) => return self.fits_expr(&p.expr, ty),
			syn::Expr::Path(p) => match p.path.get_ident().and_then(|i| self.lookup(&i.to_string())) {
				Some(Var::Typed(_, ty)) => Some(ty.clone()),
				Some(Var::Alias(_, expr)) => return self.fits_expr(&expr.clone(), ty),
				None => None,
			},
			syn::Expr::Struct(s) => self.type_of_path(&s.path),
			syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Int(i), .. }) if !i.suffix().is_empty() => int_named(i.suffix()).map(Type::Int),
			_ => None,
		};
		match (guess, unref(ty)) {
			(None, _) => true,
			// A generic record's literal names its type but not the instance.
			(Some(Type::Record(a)), Type::Record(b)) => self.t.api.records[a].path == self.t.api.records[*b].path,
			(Some(have), want) => fits(&have, want),
		}
	}

	fn variant_struct(&mut self, s: &syn::ExprStruct) -> Option<Expr> {
		let (e, v) = self.find_variant(&s.path)?;
		let fields: Vec<(String, Type)> =
			(self.t.api.enums[e].variants[v].fields.iter()).map(|f| (f.name.clone(), f.ty.clone())).collect();
		let values = fields
			.iter()
			.map(|(name, ty)| {
				let fv = s
					.fields
					.iter()
					.find(|fv| matches!(&fv.member, syn::Member::Named(n) if n == name))
					.unwrap_or_else(|| fail(s, format!("no field {name}")));
				self.check(&fv.expr, ty)
			})
			.collect();
		Some(Expr::new(Kind::Variant(e, v, values), Type::Enum(e)))
	}

	fn find_variant(&self, path: &syn::Path) -> Option<(usize, usize)> {
		let names: Vec<String> = path.segments.iter().map(|s| s.ident.to_string()).collect();
		let (variant, enum_path) = names.split_last()?;
		if enum_path.is_empty() {
			return None;
		}
		let e = (self.t.api.enums.iter()).position(|e| e.path.last() == enum_path.last() && suffix(&e.path, enum_path))?;
		let v = self.t.api.enums[e].variants.iter().position(|v| &v.name == variant)?;
		Some((e, v))
	}

	/// A unit variant at `path`, or a tuple variant of `args`.
	fn variant(&mut self, path: &syn::Path, args: &[syn::Expr]) -> Option<Expr> {
		let (e, v) = self.find_variant(path)?;
		let fields: Vec<Type> = self.t.api.enums[e].variants[v].fields.iter().map(|f| f.ty.clone()).collect();
		if fields.len() != args.len() {
			fail(path, "a variant of another arity");
		}
		let values = args.iter().zip(&fields).map(|(a, ty)| self.check(a, ty)).collect();
		Some(Expr::new(Kind::Variant(e, v, values), Type::Enum(e)))
	}

	/// A free function or associated function at `path`, whose instance, for a generic one,
	/// is the one `turbofish` names or else the first whose parameters `args` fit.
	fn function(&mut self, path: &syn::Path, args: Option<&[syn::Expr]>, expected: Option<&Type>) -> Option<FnId> {
		let names: Vec<String> = path.segments.iter().map(|s| s.ident.to_string()).collect();
		let (name, owner_path) = names.split_last()?;
		let api = &self.t.api;
		let mut candidates: Vec<FnId> = Vec::new();
		for (i, f) in api.functions.iter().enumerate() {
			if f.path.last() == Some(name) && suffix(&f.path, &names) {
				candidates.push(FnId { owner: Owner::Free, index: i });
			}
		}
		if !owner_path.is_empty() {
			let owned = |p: &[String]| p.last() == owner_path.last() && suffix(p, owner_path);
			for (i, r) in api.records.iter().enumerate() {
				if owned(&r.path) {
					for (j, f) in r.methods.iter().enumerate() {
						if f.path.last() == Some(name) && f.receiver.is_none() {
							candidates.push(FnId { owner: Owner::Record(i), index: j });
						}
					}
				}
			}
			for (i, o) in api.objects.iter().enumerate() {
				if owned(&o.path) {
					for (j, f) in o.methods.iter().enumerate() {
						if f.path.last() == Some(name) && f.receiver.is_none() {
							candidates.push(FnId { owner: Owner::Object(i), index: j });
						}
					}
				}
			}
			for (i, e) in api.enums.iter().enumerate() {
				if owned(&e.path) {
					for (j, f) in e.methods.iter().enumerate() {
						if f.path.last() == Some(name) && f.receiver.is_none() {
							candidates.push(FnId { owner: Owner::Enum(i), index: j });
						}
					}
				}
			}
		}
		// The turbofish's arguments narrow the instances.
		let turbofish: Vec<Type> = path
			.segments
			.iter()
			.filter_map(|s| match &s.arguments {
				syn::PathArguments::AngleBracketed(a) => Some(a),
				_ => None,
			})
			.flat_map(|a| &a.args)
			.filter_map(|a| match a {
				syn::GenericArgument::Type(syn::Type::Path(p)) => self.type_of_path(&p.path).or_else(|| int_named(&p.path.get_ident()?.to_string()).map(Type::Int)),
				_ => None,
			})
			.collect();
		self.pick(path, candidates, args, expected, &turbofish)
	}

	/// The candidate whose parameters `args` fit and whose return `expected` fits.
	fn pick(&mut self, at: &dyn Spanned, candidates: Vec<FnId>, args: Option<&[syn::Expr]>, expected: Option<&Type>, turbofish: &[Type]) -> Option<FnId> {
		let candidates: Vec<FnId> = candidates
			.into_iter()
			.filter(|id| {
				let f = self.t.function(*id);
				let instance = f.instance.as_ref().map(|i| i.named.as_slice()).unwrap_or(&[]);
				turbofish.iter().all(|t| instance.contains(t))
					&& args.is_none_or(|a| a.len() == f.params.len())
					&& expected.is_none_or(|e| fits(&f.ok, e) || f.err.is_some())
			})
			.collect();
		if candidates.len() <= 1 {
			return candidates.first().copied();
		}
		let Some(args) = args else {
			fail(at, "an ambiguous function")
		};
		let fitting: Vec<FnId> = candidates
			.iter()
			.copied()
			.filter(|id| {
				let params: Vec<Type> = self.t.function(*id).params.iter().map(|p| p.ty.clone()).collect();
				args.iter().zip(&params).all(|(a, ty)| self.fits_expr(a, ty))
			})
			.collect();
		match fitting.as_slice() {
			[one] => Some(*one),
			[] => fail(at, "no instance fits these arguments"),
			// Instances alike in every parameter: the first, as Rust's inference would pick
			// no other for the same arguments.
			[first, ..] => Some(*first),
		}
	}

	/// A call of a path: the API's function, a variant, `Some`, or a local function.
	fn call(&mut self, call: &syn::ExprCall, expected: Option<&Type>) -> Expr {
		let syn::Expr::Path(p) = &*call.func else {
			fail(call, "a call of anything but a path")
		};
		let args: Vec<syn::Expr> = call.args.iter().cloned().collect();
		let args: &[syn::Expr] = Box::leak(args.into_boxed_slice());
		if let Some(name) = p.path.get_ident().map(|i| i.to_string())
			&& let Some(ty) = self.t.locals.get(&name)
		{
			let ty = ty.clone();
			let args = args.iter().map(|a| self.infer(a)).collect();
			return Expr::new(Kind::Local(name, args), ty);
		}
		if let Some(e) = self.variant(&p.path, args) {
			return e;
		}
		let Some(id) = self.function(&p.path, Some(args), expected) else {
			fail(call, "a call of no function the API exports")
		};
		self.apply(call, id, None, args)
	}

	/// The call of `id` on `receiver`, with `args` checked against its parameters.
	fn apply(&mut self, at: &dyn Spanned, id: FnId, receiver: Option<Expr>, args: &[syn::Expr]) -> Expr {
		let params: Vec<Type> = self.t.function(id).params.iter().map(|p| p.ty.clone()).collect();
		if params.len() != args.len() {
			fail(at, format!("{} takes {} arguments", self.t.key(id), params.len()));
		}
		let args = args.iter().zip(&params).map(|(a, ty)| self.check(a, ty)).collect();
		let f = self.t.function(id);
		let ty = match &f.err {
			Some(err) => Type::Result(Box::new(f.ok.clone()), Box::new(err.clone())),
			None => f.ok.clone(),
		};
		Expr::new(Kind::Call(id, receiver.map(Box::new), args), ty)
	}

	fn method(&mut self, m: &syn::ExprMethodCall, expected: Option<&Type>) -> Expr {
		let name = m.method.to_string();
		let args: Vec<syn::Expr> = m.args.iter().cloned().collect();
		let args: &[syn::Expr] = Box::leak(args.into_boxed_slice());
		let receiver = self.infer(&m.receiver);
		let rty = unref(&receiver.ty).clone();
		let only = |s: &Self, kind: fn(Box<Expr>) -> Kind, ty: Type| {
			if !args.is_empty() {
				fail(m, format!("{name} with arguments"));
			}
			let _ = s;
			Expr::new(kind(Box::new(receiver.clone())), ty)
		};
		match (name.as_str(), &rty) {
			("unwrap" | "expect", Type::Option(inner) | Type::Result(inner, _)) => {
				let inner = (**inner).clone();
				return Expr::new(Kind::Unwrap(Box::new(receiver)), inner);
			}
			("unwrap_err", Type::Result(_, err)) => {
				let err = (**err).clone();
				return only(self, Kind::UnwrapErr, err);
			}
			("is_some", Type::Option(_)) => return only(self, Kind::IsSome, Type::Bool),
			("is_none", Type::Option(_)) => return only(self, Kind::IsNone, Type::Bool),
			("is_err", Type::Result(..)) => return only(self, Kind::IsErr, Type::Bool),
			("err", Type::Result(_, err)) => {
				// `.err().unwrap()`, the only use the subset has.
				let err = (**err).clone();
				return Expr::new(Kind::UnwrapErr(Box::new(receiver)), Type::Option(Box::new(err)));
			}
			("clone", _) if args.is_empty() => {
				let ty = receiver.ty.clone();
				return Expr::new(Kind::Clone(Box::new(receiver)), ty);
			}
			(conversion, _) if CONVERSIONS.contains(&conversion) && args.is_empty() => {
				// `.into()` of a value to the type the binding returns it as already.
				let _ = expected;
				return receiver;
			}
			_ => {}
		}
		let id = self.method_of(&rty, &name, |f| f.receiver.is_some() && f.access == Access::Call);
		let id = match id {
			Some(id) => id,
			None => {
				// A generic method's instances.
				let candidates = self.methods_named(&rty, &name);
				let turbofish: Vec<Type> = (m.turbofish.iter().flat_map(|t| &t.args))
					.filter_map(|a| match a {
						syn::GenericArgument::Type(syn::Type::Path(p)) => self.type_of_path(&p.path),
						_ => None,
					})
					.collect();
				self.pick(m, candidates, Some(args), expected, &turbofish)
					.unwrap_or_else(|| fail(m, format!("no method {name} on {}", self.describe(&rty))))
			}
		};
		self.apply(m, id, Some(receiver), args)
	}

	fn methods_named(&self, owner: &Type, name: &str) -> Vec<FnId> {
		let (owner, methods) = match unref(owner) {
			Type::Record(i) => (Owner::Record(*i), &self.t.api.records[*i].methods),
			Type::Enum(i) => (Owner::Enum(*i), &self.t.api.enums[*i].methods),
			Type::Object(i) => (Owner::Object(*i), &self.t.api.objects[*i].methods),
			_ => return Vec::new(),
		};
		(methods.iter().enumerate())
			.filter(|(_, f)| f.path.last().map(String::as_str) == Some(name) && f.receiver.is_some() && f.access == Access::Call)
			.map(|(index, _)| FnId { owner, index })
			.collect()
	}

	fn describe(&self, ty: &Type) -> String {
		match ty {
			Type::Record(_) | Type::Enum(_) | Type::Object(_) => self.t.rust(ty),
			ty => format!("{ty:?}"),
		}
	}
}

/// Conversions that leave a value as the binding has it.
const CONVERSIONS: &[&str] = &["to_string", "to_vec", "to_owned", "into", "as_str", "as_ref", "as_slice", "as_deref"];

/// Whether `path`'s segments end with `names`' segments, ignoring the `api` crate's root.
fn suffix(path: &[String], names: &[String]) -> bool {
	let names: Vec<&String> = names.iter().filter(|n| *n != "api").collect();
	names.len() <= path.len() && path[path.len() - names.len()..].iter().zip(&names).all(|(a, b)| a == *b)
}

/// Whether `expr` is the path `path`, such as `String::from`.
fn path_is(expr: &syn::Expr, path: &str) -> bool {
	let syn::Expr::Path(p) = expr else { return false };
	let names: Vec<String> = p.path.segments.iter().map(|s| s.ident.to_string()).collect();
	let want: Vec<&str> = path.split("::").collect();
	names.len() >= want.len() && names[names.len() - want.len()..].iter().zip(&want).all(|(a, b)| a == b)
}

/// Whether `expr` takes its type from the other side of an operator, as a literal does.
fn literalish(expr: &syn::Expr) -> bool {
	match expr {
		syn::Expr::Lit(_) => true,
		syn::Expr::Unary(u) => literalish(&u.expr),
		syn::Expr::Paren(p) => literalish(&p.expr),
		_ => false,
	}
}

fn int_named(name: &str) -> Option<Int> {
	Some(match name {
		"u8" => Int::U8,
		"u16" => Int::U16,
		"u32" => Int::U32,
		"u64" => Int::U64,
		"usize" => Int::Usize,
		"i8" => Int::I8,
		"i16" => Int::I16,
		"i32" => Int::I32,
		"i64" => Int::I64,
		"isize" => Int::Isize,
		_ => return None,
	})
}

/// `ty` behind any references.
pub(crate) fn unref(ty: &Type) -> &Type {
	match ty {
		Type::Ref(_, inner) => unref(inner),
		ty => ty,
	}
}

/// Whether a value of `have` passes where `want` is taken: the same type, behind any
/// references or boxes, or a string or bytes in any of their forms.
fn fits(have: &Type, want: &Type) -> bool {
	let (have, want) = (have.base(), want.base());
	let text = |t: &Type| matches!(t, Type::Str | Type::String);
	let bytes = |t: &Type| matches!(t, Type::Slice | Type::Vec | Type::Bytes);
	let path = |t: &Type| matches!(t, Type::Path | Type::PathBuf);
	match (have, want) {
		(Type::Option(a), Type::Option(b)) => fits(a, b),
		(Type::List(_, a), Type::List(_, b)) => fits(a, b),
		(Type::Tuple(a), Type::Tuple(b)) => a.len() == b.len() && a.iter().zip(b).all(|(a, b)| fits(a, b)),
		(a, b) => a == b || (text(a) && text(b)) || (bytes(a) && bytes(b)) || (path(a) && path(b)),
	}
}
