//! `transcript.rs` read into steps: each expression typed against the API, bidirectionally,
//! so a literal takes the type of the parameter it is passed to.

use std::collections::BTreeMap;

use syn::spanned::Spanned;

use super::ir::{Access, Collection, Family, Function, Int, Literal, Mutability, Type};
use super::{Arm, BoundKind, Expr, FnId, Kind, Num, Op, Owner, Pat, Piece, Spec, Step, Transcript, unref};

/// The steps of `source`'s `main`, and the parameters' types of each function it defines
/// that `main` calls.
pub(super) fn main(t: &Transcript, source: &str) -> (Vec<Step>, BTreeMap<String, Vec<Type>>) {
	let file = syn::parse_file(source).unwrap_or_else(|e| panic!("{}: {e}", at(e.span())));
	let mut scope = Scope {
		t,
		vars: Vec::new(),
		units: Vec::new(),
		locals: BTreeMap::new(),
		params: BTreeMap::new(),
		uses: BTreeMap::new(),
		taken: BTreeMap::new(),
	};
	let mut main = None;
	for item in &file.items {
		match item {
			syn::Item::Use(u) => scope.import(&u.tree, &mut Vec::new()),
			syn::Item::Struct(s) if matches!(s.fields, syn::Fields::Unit) => scope.units.push(s.ident.to_string()),
			syn::Item::Static(s) => scope.vars.push(Var::Alias(s.ident.to_string(), (*s.expr).clone())),
			syn::Item::Const(c) => scope.vars.push(Var::Alias(c.ident.to_string(), (*c.expr).clone())),
			_ => {}
		}
	}
	for item in &file.items {
		if let syn::Item::Fn(f) = item {
			if f.sig.ident == "main" {
				main = Some(f);
				continue;
			}
			scope.locals.insert(f.sig.ident.to_string(), f.sig.clone());
		}
	}
	let main = main.expect("transcript.rs has a main");
	let steps = scope.block(&main.block.stmts);
	(steps, scope.params)
}

/// `transcript.rs:line:column` of a span.
fn at(span: proc_macro2::Span) -> String {
	let start = span.start();
	format!("transcript.rs:{}:{}", start.line, start.column + 1)
}

/// Fail at `node`'s span: a construct the subset lacks, or one the API does not have.
fn fail(node: &dyn Spanned, message: impl std::fmt::Display) -> ! {
	panic!("{}: {message}", at(node.span()))
}

/// A local in scope: its unique name and type, or a loop's literal, typed at each use.
enum Var {
	Typed(String, String, Type),
	Alias(String, syn::Expr),
}

struct Scope<'t> {
	t: &'t Transcript,
	vars: Vec<Var>,
	/// The unit structs `transcript.rs` defines: each language's own implementation.
	units: Vec<String>,
	/// The functions `transcript.rs` defines, each a language's own: what each returns,
	/// typed where it is called.
	locals: BTreeMap<String, syn::Signature>,
	/// The parameters' types of each function `transcript.rs` defines that `main` calls,
	/// which a language whose own functions are typed declares them by.
	params: BTreeMap<String, Vec<Type>>,
	/// What each name a `use` brings in stands for.
	uses: BTreeMap<String, Vec<String>>,
	/// How many locals took each name, so each binding gets its own.
	taken: BTreeMap<String, usize>,
}

/// Conversions that leave a value as the binding has it.
const CONVERSIONS: &[&str] = &["to_string", "to_vec", "to_owned", "into", "as_ref", "into_iter", "iter", "as_deref"];

impl Scope<'_> {
	fn import(&mut self, tree: &syn::UseTree, prefix: &mut Vec<String>) {
		match tree {
			syn::UseTree::Path(p) => {
				prefix.push(p.ident.to_string());
				self.import(&p.tree, prefix);
				prefix.pop();
			}
			syn::UseTree::Name(n) => {
				let mut path = prefix.clone();
				path.push(n.ident.to_string());
				self.uses.insert(n.ident.to_string(), path);
			}
			syn::UseTree::Group(g) => g.items.iter().for_each(|t| self.import(t, prefix)),
			tree => fail(tree, "a use other than of names"),
		}
	}

	/// A path's segments, its first expanded through the `use`s.
	fn segments(&self, path: &syn::Path) -> Vec<String> {
		let mut names: Vec<String> = path.segments.iter().map(|s| s.ident.to_string()).collect();
		if let Some(full) = self.uses.get(&names[0]) {
			names.splice(0..1, full.iter().cloned());
		}
		names
	}

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
			Var::Typed(n, ..) | Var::Alias(n, _) => n == name,
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
				let value = match &local.pat {
					syn::Pat::Type(typed) => {
						let ty = self.ty(&typed.ty);
						self.check(&init.expr, &ty)
					}
					_ => self.infer(&init.expr),
				};
				let pat = self.bind(&local.pat, &value.ty);
				steps.push(Step::Let(pat, value));
			}
			syn::Stmt::Macro(m) => steps.push(Step::Print(self.print(&m.mac))),
			syn::Stmt::Expr(expr, _) => self.expr_stmt(expr, steps),
			syn::Stmt::Item(item) => fail(item, "an item inside main"),
		}
	}

	fn expr_stmt(&mut self, expr: &syn::Expr, steps: &mut Vec<Step>) {
		match expr {
			syn::Expr::Macro(m) => steps.push(Step::Print(self.print(&m.mac))),
			syn::Expr::Block(b) => steps.extend(self.block(&b.block.stmts)),
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
			syn::Expr::Assign(a) => steps.push(Step::Do(self.assign(a))),
			syn::Expr::Call(call) if path_is(&call.func, "drop") => {
				let [syn::Expr::Path(p)] = call.args.iter().collect::<Vec<_>>()[..] else {
					fail(call, "drop of anything but a local")
				};
				let name = p.path.get_ident().unwrap().to_string();
				let Some(Var::Typed(_, unique, _)) = self.lookup(&name) else {
					fail(p, format!("drop of {name}, which is no local"))
				};
				steps.push(Step::Drop(unique.clone()));
			}
			expr => steps.push(Step::Do(self.infer(expr))),
		}
	}

	/// `place.field = value`: the field's setter, called on the place's receiver.
	fn assign(&mut self, a: &syn::ExprAssign) -> Expr {
		let syn::Expr::Field(field) = &*a.left else {
			fail(&a.left, "an assignment to anything but a field")
		};
		let syn::Member::Named(name) = &field.member else {
			fail(field, "an assignment to an element")
		};
		let receiver = self.infer(&field.base);
		let Some(id) = self.method_of(&receiver.ty, &name.to_string(), |f| matches!(f.access, Access::Field { set: true, .. })) else {
			fail(field, format!("no setter of {name} on {}", self.describe(&receiver.ty)))
		};
		let ty = self.t.function(id).params[0].ty.clone();
		let value = self.check(&a.right, &ty);
		Expr::new(Kind::Call(id, Some(Box::new(receiver)), vec![value]), Type::Unit)
	}

	/// A unique name for a local named `name`.
	fn unique(&mut self, name: &str) -> String {
		let n = self.taken.entry(name.to_string()).or_insert(0);
		*n += 1;
		match *n {
			1 => name.to_string(),
			n => format!("{name}_{n}"),
		}
	}

	/// A pattern, binding its names to the parts of `ty`.
	fn bind(&mut self, pat: &syn::Pat, ty: &Type) -> Pat {
		match pat {
			syn::Pat::Type(typed) => self.bind(&typed.pat, ty),
			syn::Pat::Ident(ident) => {
				let name = ident.ident.to_string();
				let unique = self.unique(&name);
				self.vars.push(Var::Typed(name, unique.clone(), ty.clone()));
				Pat::Name(unique, ty.clone())
			}
			syn::Pat::Wild(_) => Pat::Ignore(ty.clone()),
			syn::Pat::Reference(r) => self.bind(&r.pat, ty),
			syn::Pat::Tuple(tuple) => {
				// A `Bounds` is its start and end `Bound`.
				let bounds = match unref(ty).base() {
					Type::Bounds(int) => Some(Type::Tuple(vec![Type::Bound(*int), Type::Bound(*int)])),
					_ => None,
				};
				let Type::Tuple(elements) = bounds.as_ref().unwrap_or(unref(ty).base()) else {
					fail(pat, format!("a tuple pattern of {}", self.describe(ty)))
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

	/// The type a Rust type names: the API's, or the standard library's it maps.
	fn ty(&self, ty: &syn::Type) -> Type {
		match ty {
			syn::Type::Reference(r) => {
				let mutability = if r.mutability.is_some() { Mutability::Mut } else { Mutability::Not };
				Type::Ref(mutability, Box::new(self.ty(&r.elem)))
			}
			syn::Type::Paren(p) => self.ty(&p.elem),
			syn::Type::TraitObject(t) => {
				let Some(syn::TypeParamBound::Trait(bound)) = t.bounds.first() else {
					fail(t, "a trait object of no trait")
				};
				self.type_of_path(&bound.path).unwrap_or_else(|| fail(t, "a trait object the API lacks"))
			}
			syn::Type::Tuple(t) if t.elems.is_empty() => Type::Unit,
			syn::Type::Tuple(t) => Type::Tuple(t.elems.iter().map(|e| self.ty(e)).collect()),
			syn::Type::Array(a) => {
				let len = match &a.len {
					syn::Expr::Lit(syn::ExprLit {
						lit: syn::Lit::Int(i), ..
					}) => i.base10_parse().unwrap(),
					len => fail(len, "an array length other than a literal"),
				};
				match self.ty(&a.elem) {
					Type::Int(Int::U8) => Type::Array(len),
					item => Type::List(Collection::Array(len), Box::new(item)),
				}
			}
			syn::Type::Slice(s) => match self.ty(&s.elem) {
				Type::Int(Int::U8) => Type::Slice,
				item => Type::List(Collection::Slice, Box::new(item)),
			},
			syn::Type::Path(p) => {
				let last = p.path.segments.last().unwrap();
				let arg = || match &last.arguments {
					syn::PathArguments::AngleBracketed(a) => match a.args.first() {
						Some(syn::GenericArgument::Type(t)) => self.ty(t),
						_ => fail(a, "a type argument"),
					},
					_ => fail(last, "a type without its argument"),
				};
				let name = last.ident.to_string();
				if let Some(int) = int_named(&name) {
					return Type::Int(int);
				}
				match name.as_str() {
					"bool" => Type::Bool,
					"char" => Type::Char,
					"f32" => Type::F32,
					"f64" => Type::F64,
					"u128" => Type::U128,
					"str" => Type::Str,
					"String" => Type::String,
					"Bytes" => Type::Bytes,
					"Duration" => Type::Duration,
					"Path" => Type::Path,
					"PathBuf" => Type::PathBuf,
					"SocketAddr" => Type::SocketAddr(Family::Any),
					"SocketAddrV4" => Type::SocketAddr(Family::V4),
					"SocketAddrV6" => Type::SocketAddr(Family::V6),
					"IpAddr" => Type::IpAddr(Family::Any),
					"Ipv4Addr" => Type::IpAddr(Family::V4),
					"Ipv6Addr" => Type::IpAddr(Family::V6),
					"Option" => Type::Option(Box::new(arg())),
					"Box" => Type::Box(Box::new(arg())),
					"Vec" => match arg() {
						Type::Int(Int::U8) => Type::Vec,
						item => Type::List(Collection::Vec, Box::new(item)),
					},
					_ => self.type_of_path(&p.path).unwrap_or_else(|| fail(ty, "a type the API lacks")),
				}
			}
			ty => fail(ty, "this type"),
		}
	}

	/// The API's record, enum, or object a path names: the one whose path or shown path it
	/// is, else the only one of its last segment, a generic one at its default.
	fn type_of_path(&self, path: &syn::Path) -> Option<Type> {
		let names = self.segments(path);
		let api = &self.t.api;
		let types: Vec<(Type, &Vec<String>, bool)> = (api.records.iter().enumerate())
			.map(|(i, r)| (Type::Record(i), &r.path, r.args.is_empty()))
			.chain(api.enums.iter().enumerate().map(|(i, e)| (Type::Enum(i), &e.path, e.args.is_empty())))
			.chain(api.objects.iter().enumerate().map(|(i, o)| (Type::Object(i), &o.path, o.args.is_empty())))
			.collect();
		let pick = |found: Vec<&(Type, &Vec<String>, bool)>| -> Option<Type> {
			match found.as_slice() {
				[] => None,
				[one] => Some(one.0.clone()),
				// A generic type: the instance at its defaults, which takes no arguments, else its
				// first, which a literal's fields narrow.
				many => Some(many.iter().find(|t| t.2).unwrap_or(&many[0]).0.clone()),
			}
		};
		let exact: Vec<_> = types.iter().filter(|t| *t.1 == names).collect();
		if !exact.is_empty() {
			return pick(exact);
		}
		pick(types.iter().filter(|t| t.1.last() == names.last()).collect())
	}

	fn print(&mut self, mac: &syn::Macro) -> Vec<Piece> {
		if !mac.path.is_ident("println") {
			fail(mac, "a macro other than println!");
		}
		self.format(mac)
	}

	/// The pieces of a `println!` or `format!`'s format string and its arguments.
	fn format(&mut self, mac: &syn::Macro) -> Vec<Piece> {
		let args = mac
			.parse_body_with(syn::punctuated::Punctuated::<syn::Expr, syn::Token![,]>::parse_terminated)
			.unwrap_or_else(|e| fail(mac, e));
		let mut args = args.into_iter();
		let Some(syn::Expr::Lit(syn::ExprLit {
			lit: syn::Lit::Str(lit), ..
		})) = args.next()
		else {
			fail(mac, "a format without its string")
		};
		let args: Vec<syn::Expr> = args.collect();
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
					let inner: String = chars.by_ref().take_while(|c| *c != '}').collect();
					let (name, spec) = inner.split_once(':').unwrap_or((&inner, ""));
					let spec = match spec {
						"" => Spec::Display,
						"?" => Spec::Debug,
						"x" => Spec::Hex(0),
						s if s.starts_with('0') && s.ends_with('x') => Spec::Hex(s[1..s.len() - 1].parse().unwrap()),
						s if s.starts_with('0') => Spec::Padded(s[1..].parse().unwrap()),
						s => fail(&lit, format!("the format {s}")),
					};
					let value = if name.is_empty() {
						let arg = args.get(next).unwrap_or_else(|| fail(&lit, "too few arguments"));
						next += 1;
						self.infer(arg)
					} else {
						let ident = syn::Ident::new(name, lit.span());
						let path = syn::Expr::Path(syn::ExprPath {
							attrs: Vec::new(),
							qself: None,
							path: ident.into(),
						});
						self.infer(&path)
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
			fail(&lit, "too many arguments");
		}
		if !literal.is_empty() {
			pieces.push(Piece::Text(literal));
		}
		pieces
	}

	fn matched(&mut self, m: &syn::ExprMatch) -> Step {
		let value = self.infer(&m.expr);
		let (ok_ty, err_ty) = match &value.ty {
			Type::Result(ok, err) => ((**ok).clone(), Some((**err).clone())),
			Type::Option(inner) => ((**inner).clone(), None),
			ty => fail(&m.expr, format!("a match on {}", self.describe(ty))),
		};
		let option = err_ty.is_none();
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
				("Ok", false) | ("Some", true) => (&mut ok, Some(ok_ty.clone())),
				("Err", false) => (&mut err, err_ty.clone()),
				("None", true) => (&mut err, None),
				_ => fail(&arm.pat, format!("{variant} in this match")),
			};
			let pat = inner.map(|p| self.bind(p, ty.as_ref().unwrap()));
			let mut steps = Vec::new();
			match &*arm.body {
				syn::Expr::Block(b) => b.block.stmts.iter().for_each(|s| self.stmt(s, &mut steps)),
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

	/// The constant at `path`, if the API has one.
	fn constant(&self, path: &syn::Path) -> Option<usize> {
		let names = self.segments(path);
		(self.t.api.constants.iter()).position(|c| c.path == names)
	}

	/// An expression whose type follows from itself.
	fn infer(&mut self, expr: &syn::Expr) -> Expr {
		match expr {
			syn::Expr::Paren(p) => self.infer(&p.expr),
			syn::Expr::Reference(r) => self.infer(&r.expr),
			syn::Expr::Group(g) => self.infer(&g.expr),
			syn::Expr::Unary(u) if matches!(u.op, syn::UnOp::Deref(_)) => self.infer(&u.expr),
			syn::Expr::Path(p) => {
				if let Some(name) = p.path.get_ident().map(|i| i.to_string()) {
					match self.lookup(&name) {
						Some(Var::Typed(_, unique, ty)) => return Expr::new(Kind::Var(unique.clone()), ty.clone()),
						Some(Var::Alias(_, expr)) => {
							let expr = expr.clone();
							return self.infer(&expr);
						}
						None => {}
					}
				}
				if let Some(c) = self.constant(&p.path) {
					let ty = match self.t.api.constants[c].value {
						Literal::Bool(_) => Type::Bool,
						Literal::Int(int, _) => Type::Int(int),
					};
					return Expr::new(Kind::Const(c), ty);
				}
				// A constant the IR keeps as a function, such as `api::BOXED`.
				if let Some(id) = self.function(&p.path, &[], None)
					&& self.t.function(id).access == Access::Const
				{
					return Expr::new(Kind::Call(id, None, Vec::new()), self.t.function(id).ok.clone());
				}
				if let Some(e) = self.variant(&p.path, &[], None) {
					return e;
				}
				fail(p, "a path that names no local, constant, or variant")
			}
			syn::Expr::Lit(lit) => match &lit.lit {
				syn::Lit::Str(s) => Expr::new(Kind::Str(s.value()), Type::Ref(Mutability::Not, Box::new(Type::Str))),
				syn::Lit::Bool(b) => Expr::new(Kind::Bool(b.value), Type::Bool),
				syn::Lit::ByteStr(b) => Expr::new(Kind::Bytes(b.value()), Type::Ref(Mutability::Not, Box::new(Type::Slice))),
				syn::Lit::Int(i) if !i.suffix().is_empty() => {
					let int = int_named(i.suffix()).unwrap_or_else(|| fail(i, "this integer suffix"));
					self.check(expr, &Type::Int(int))
				}
				lit => fail(lit, "a literal whose type nothing gives"),
			},
			syn::Expr::Call(call) => self.call(call, None),
			syn::Expr::MethodCall(m) => self.method(m, None),
			syn::Expr::Field(f) => self.field(f),
			syn::Expr::Struct(s) => self.structure(s, None),
			syn::Expr::Binary(b) => self.binary(b),
			syn::Expr::Macro(m) if m.mac.path.is_ident("format") => {
				Expr::new(Kind::Format(self.format(&m.mac)), Type::String)
			}
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
		let base = unref(ty).base().clone();
		if let Some(value) = self.constant_of(expr, &base) {
			return Expr::new(value, ty.clone());
		}
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
				if self.lookup(&name).is_none() && self.units.contains(&name) {
					return Expr::new(Kind::Local(name, Vec::new()), ty.clone());
				}
				if name == "None" {
					let Type::Option(_) = base else {
						fail(expr, format!("None as {}", self.describe(ty)))
					};
					return Expr::new(Kind::None, ty.clone());
				}
			}
			// A foreign enum's constant, which the IR keeps as a variant: tracing's `Level`,
			// and `LevelFilter`, an optional level whose `OFF` is none.
			syn::Expr::Path(p) if matches!(base, Type::Enum(_) | Type::Option(_)) => {
				if let Some(e) = self.variant(&p.path, &[], Some(&base)) {
					return Expr::new(e.kind, ty.clone());
				}
			}
			syn::Expr::MethodCall(m) if m.method == "parse" || (m.method == "unwrap" && matches!(&*m.receiver, syn::Expr::MethodCall(r) if r.method == "parse")) => {
				return self.parsed(m, ty);
			}
			syn::Expr::MethodCall(m) if CONVERSIONS.contains(&m.method.to_string().as_str()) && m.args.is_empty() && !self.api_method(m) => {
				return self.check(&m.receiver, ty);
			}
			// `[..].into_iter().map(..).collect()`, as a map's or a set's items are built.
			syn::Expr::MethodCall(m) if m.method == "collect" => return self.collected(m, ty),
			syn::Expr::Call(call) => {
				let func = &*call.func;
				let one = call.args.len() == 1;
				let passed = ["String::from", "Bytes::from_static", "Box::new", "PathBuf::from", "Path::new", "Bytes::from"];
				if one && passed.iter().any(|p| path_is(func, p)) {
					return self.check(&call.args[0], ty);
				}
				if one && path_is(func, "Some") {
					let Type::Option(inner) = &base else {
						fail(expr, format!("Some as {}", self.describe(ty)))
					};
					let value = self.check(&call.args[0], inner);
					return Expr::new(Kind::Some(Box::new(value)), ty.clone());
				}
				let empties = ["Default::default", "Box::default", "Vec::new", "String::new", "Bytes::new"];
				if call.args.is_empty() && empties.iter().any(|p| path_is(func, p)) {
					return self.empty(expr, ty);
				}
				if let syn::Expr::Path(p) = func
					&& let Some(name) = p.path.get_ident()
					&& self.locals.contains_key(&name.to_string())
				{
					self.local_params(&name.to_string());
					let args = call.args.iter().map(|a| self.infer(a)).collect();
					return Expr::new(Kind::Local(name.to_string(), args), ty.clone());
				}
				if let Some(bound) = self.bound(call, &base) {
					return bound;
				}
				return self.call(call, Some(ty));
			}
			syn::Expr::Macro(m) if m.mac.path.is_ident("vec") => {
				let items = m
					.mac
					.parse_body_with(syn::punctuated::Punctuated::<syn::Expr, syn::Token![,]>::parse_terminated)
					.unwrap_or_else(|e| fail(m, e));
				let items: Vec<syn::Expr> = items.into_iter().collect();
				return self.items(expr, &items, ty);
			}
			syn::Expr::Array(a) => {
				let items: Vec<syn::Expr> = a.elems.iter().cloned().collect();
				return self.items(expr, &items, ty);
			}
			syn::Expr::Lit(lit) => return self.literal(lit, ty),
			syn::Expr::Tuple(t) => {
				if let Type::Bounds(int) = base {
					let elements = t.elems.iter().map(|e| self.check(e, &Type::Bound(int))).collect();
					return Expr::new(Kind::Tuple(elements), ty.clone());
				}
				let Type::Tuple(elements) = &base else {
					fail(expr, format!("a tuple as {}", self.describe(ty)))
				};
				if elements.len() != t.elems.len() {
					fail(expr, "a tuple of another length");
				}
				let values = t.elems.iter().zip(elements).map(|(e, ty)| self.check(e, ty)).collect();
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
			fail(expr, format!("{} where {} is taken", self.describe(&value.ty), self.describe(ty)));
		}
		value
	}

	/// Record the parameters' types of the function `name` that `transcript.rs` defines.
	fn local_params(&mut self, name: &str) {
		let inputs = self.locals[name].inputs.clone();
		let params = (inputs.iter())
			.map(|input| match input {
				syn::FnArg::Typed(typed) => self.ty(&typed.ty),
				input => fail(input, "a function of transcript.rs taking self"),
			})
			.collect();
		self.params.insert(name.to_string(), params);
	}

	/// Whether `m` calls a method of the API's, not a conversion of the same name.
	fn api_method(&self, m: &syn::ExprMethodCall) -> bool {
		let Some(ty) = self.peek(&m.receiver) else { return false };
		!self.methods_named(&ty, &m.method.to_string()).is_empty()
	}

	/// The type of `expr` where it plainly has one, without typing it: a local's.
	fn peek(&self, expr: &syn::Expr) -> Option<Type> {
		match expr {
			syn::Expr::Reference(r) => self.peek(&r.expr),
			syn::Expr::Paren(p) => self.peek(&p.expr),
			syn::Expr::Path(p) => match self.lookup(&p.path.get_ident()?.to_string())? {
				Var::Typed(_, _, ty) => Some(ty.clone()),
				Var::Alias(..) => None,
			},
			_ => None,
		}
	}

	/// A value of a standard type spelled by its constructors or constants, which only
	/// literals feed: an integer past a literal's reach, a float, a duration, an address.
	fn constant_of(&self, expr: &syn::Expr, ty: &Type) -> Option<Kind> {
		match ty {
			Type::Int(_) | Type::U128 | Type::NonZero(_) => match expr {
				syn::Expr::Lit(_) => None,
				expr => const_int(expr).map(Kind::Int),
			},
			Type::F32 | Type::F64 => const_float(expr).map(Kind::Float),
			Type::Char if path_is(expr, "char::MAX") => Some(Kind::Char(char::MAX)),
			Type::Duration => const_duration(expr).map(|(s, n)| Kind::Duration(s, n)),
			Type::SocketAddr(_) => match const_net(expr)? {
				Net::Socket(addr) => Some(Kind::Socket(addr)),
				Net::Text(text) => Some(Kind::Socket(text.parse().unwrap_or_else(|e| fail(expr, e)))),
				Net::Ip(_) => fail(expr, "an IP address as a socket address"),
			},
			Type::IpAddr(_) => match const_net(expr)? {
				Net::Ip(ip) => Some(Kind::Ip(ip)),
				Net::Text(text) => Some(Kind::Ip(text.parse().unwrap_or_else(|e| fail(expr, e)))),
				Net::Socket(_) => fail(expr, "a socket address as an IP address"),
			},
			_ => None,
		}
	}

	/// `Bound::Included(v)`, `Bound::Excluded(v)`, as a `Bound`.
	fn bound(&mut self, call: &syn::ExprCall, ty: &Type) -> Option<Expr> {
		let Type::Bound(int) = ty else { return None };
		let kind = if path_is(&call.func, "Bound::Included") {
			BoundKind::Included
		} else if path_is(&call.func, "Bound::Excluded") {
			BoundKind::Excluded
		} else {
			return None;
		};
		let value = self.check(&call.args[0], &Type::Int(*int));
		Some(Expr::new(Kind::Bound(kind, Some(Box::new(value))), ty.clone()))
	}

	/// `"text".parse()` as `ty`: an address the generator parses, or the API's parse.
	fn parsed(&mut self, m: &syn::ExprMethodCall, ty: &Type) -> Expr {
		let (parse, unwrapped) = match &*m.receiver {
			syn::Expr::MethodCall(r) if m.method == "unwrap" => (r, true),
			_ => (m, false),
		};
		let target = match unwrapped {
			true => ty.clone(),
			false => match unref(ty) {
				Type::Result(ok, _) => (**ok).clone(),
				_ => fail(m, "a parse not unwrapped"),
			},
		};
		let id = self
			.method_of(&target, "parse", |f| f.access == Access::Parse)
			.unwrap_or_else(|| fail(m, format!("no parse of {}", self.describe(&target))));
		let text = self.check(&parse.receiver, &Type::Ref(Mutability::Not, Box::new(Type::Str)));
		let call = self.apply_checked(id, None, vec![text]);
		match unwrapped {
			true => Expr::new(Kind::Unwrap(Box::new(call)), target),
			false => call,
		}
	}

	/// An empty collection, string, or a type's default.
	fn empty(&mut self, expr: &syn::Expr, ty: &Type) -> Expr {
		match unref(ty).base() {
			Type::String | Type::Str => Expr::new(Kind::Str(String::new()), ty.clone()),
			Type::Bytes | Type::Vec | Type::Slice => Expr::new(Kind::Bytes(Vec::new()), ty.clone()),
			Type::List(..) => Expr::new(Kind::List(Vec::new()), ty.clone()),
			Type::Option(_) => Expr::new(Kind::None, ty.clone()),
			Type::Record(_) | Type::Object(_) => Expr::new(Kind::Default, ty.clone()),
			other => fail(expr, format!("an empty {}", self.describe(other))),
		}
	}

	/// The items of a list literal, as a list of `ty` or its bytes.
	fn items(&mut self, expr: &syn::Expr, items: &[syn::Expr], ty: &Type) -> Expr {
		match unref(ty).base() {
			Type::List(_, item) => {
				let item = (**item).clone();
				let items = items.iter().map(|e| self.check(e, &item)).collect();
				Expr::new(Kind::List(items), ty.clone())
			}
			Type::Vec | Type::Slice | Type::Bytes | Type::Array(_) => {
				let bytes = items
					.iter()
					.map(|e| match self.check(e, &Type::Int(Int::U8)).kind {
						Kind::Int(n) => n.magnitude as u8,
						_ => fail(e, "a byte other than a literal"),
					})
					.collect();
				Expr::new(Kind::Bytes(bytes), ty.clone())
			}
			other => fail(expr, format!("a list as {}", self.describe(other))),
		}
	}

	/// `items.into_iter().map(..).collect()` as `ty`: the items, each mapped, of an array
	/// literal, which a map's closure of a tuple pattern binds by name.
	fn collected(&mut self, m: &syn::ExprMethodCall, ty: &Type) -> Expr {
		let mut receiver = &*m.receiver;
		let mut map = None;
		loop {
			match receiver {
				syn::Expr::MethodCall(inner) if CONVERSIONS.contains(&inner.method.to_string().as_str()) => {
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
		let items: Vec<syn::Expr> = array.elems.iter().cloned().collect();
		let Some(map) = map else {
			return self.items(receiver, &items, ty);
		};
		let Type::List(_, item) = unref(ty) else {
			fail(m, format!("a collect as {}", self.describe(ty)))
		};
		let syn::Expr::Closure(closure) = map else {
			fail(map, "a map of anything but a closure")
		};
		let [syn::Pat::Tuple(pat)] = closure.inputs.iter().collect::<Vec<_>>()[..] else {
			fail(closure, "a map's closure of one tuple pattern")
		};
		let values = items
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
				let value = self.check(&closure.body, item);
				self.vars.truncate(depth);
				value
			})
			.collect();
		Expr::new(Kind::List(values), ty.clone())
	}

	fn literal(&mut self, lit: &syn::ExprLit, ty: &Type) -> Expr {
		let kind = match (&lit.lit, unref(ty).base()) {
			(syn::Lit::Str(s), Type::Str | Type::String | Type::Path | Type::PathBuf) => Kind::Str(s.value()),
			(syn::Lit::ByteStr(b), Type::Slice | Type::Vec | Type::Bytes | Type::Array(_)) => Kind::Bytes(b.value()),
			(syn::Lit::Byte(b), Type::Int(Int::U8)) => Kind::Int(Num::new(b.value().into())),
			(syn::Lit::Bool(b), Type::Bool) => Kind::Bool(b.value),
			(syn::Lit::Char(c), Type::Char) => Kind::Char(c.value()),
			(syn::Lit::Int(i), Type::Int(_) | Type::U128 | Type::NonZero(_)) => Kind::Int(Num {
				negative: false,
				magnitude: i.base10_parse().unwrap_or_else(|e| fail(i, e)),
			}),
			(syn::Lit::Float(f), Type::F32 | Type::F64) => Kind::Float(f.base10_parse().unwrap_or_else(|e| fail(f, e))),
			(syn::Lit::Int(i), Type::F32 | Type::F64) => Kind::Float(i.base10_parse().unwrap_or_else(|e| fail(i, e))),
			(lit, _) => fail(lit, format!("this literal as {}", self.describe(ty))),
		};
		Expr::new(kind, ty.clone())
	}

	fn range(&mut self, r: &syn::ExprRange, ty: &Type) -> Expr {
		let closed = matches!(r.limits, syn::RangeLimits::Closed(_));
		match unref(ty) {
			Type::Range(int) => {
				let int = Type::Int(*int);
				let (Some(start), Some(end), false) = (&r.start, &r.end, closed) else {
					fail(r, "a Range other than start..end")
				};
				let elements = vec![self.check(start, &int), self.check(end, &int)];
				Expr::new(Kind::Tuple(elements), ty.clone())
			}
			Type::Bounds(int) => {
				let bound = Type::Bound(*int);
				let mut end = |s: &mut Self, e: Option<&syn::Expr>, kind: BoundKind| match e {
					None => Expr::new(Kind::Bound(BoundKind::Unbounded, None), bound.clone()),
					Some(e) => {
						let v = s.check(e, &Type::Int(*int));
						Expr::new(Kind::Bound(kind, Some(Box::new(v))), bound.clone())
					}
				};
				let start = end(self, r.start.as_deref(), BoundKind::Included);
				let last = end(self, r.end.as_deref(), if closed { BoundKind::Included } else { BoundKind::Excluded });
				Expr::new(Kind::Tuple(vec![start, last]), ty.clone())
			}
			other => fail(r, format!("a range as {}", self.describe(other))),
		}
	}

	fn closure(&mut self, c: &syn::ExprClosure, ty: &Type) -> Expr {
		let Type::Closure(closure) = unref(ty).base() else {
			fail(c, format!("a closure as {}", self.describe(ty)))
		};
		let closure = (**closure).clone();
		if closure.params.len() != c.inputs.len() {
			fail(c, "a closure of another arity");
		}
		let depth = self.vars.len();
		let params = c.inputs.iter().zip(&closure.params).map(|(p, ty)| self.bind(p, ty)).collect();
		let (steps, value) = self.body(&c.body, &closure.ok);
		self.vars.truncate(depth);
		Expr::new(Kind::Closure(params, steps, Box::new(value)), ty.clone())
	}

	/// A closure's body: the steps it runs, and its value, of type `ok`.
	fn body(&mut self, body: &syn::Expr, ok: &Type) -> (Vec<Step>, Expr) {
		let mut steps = Vec::new();
		let value = match body {
			syn::Expr::Block(b) => {
				let (last, init) = b.block.stmts.split_last().unwrap_or_else(|| fail(b, "an empty closure"));
				init.iter().for_each(|s| self.stmt(s, &mut steps));
				match last {
					syn::Stmt::Expr(e, None) => self.check(e, ok),
					stmt => {
						self.stmt(stmt, &mut steps);
						Expr::new(Kind::Unit, Type::Unit)
					}
				}
			}
			syn::Expr::Macro(m) if m.mac.path.is_ident("println") => {
				steps.push(Step::Print(self.print(&m.mac)));
				Expr::new(Kind::Unit, Type::Unit)
			}
			body => self.check(body, ok),
		};
		(steps, value)
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
				let want = unref(&left.ty).clone();
				(left, self.check(&b.right, &want))
			}
			true => {
				let right = self.infer(&b.right);
				let want = unref(&right.ty).clone();
				(self.check(&b.left, &want), right)
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
		let owner = unref(&base.ty).base().clone();
		match (&f.member, &owner) {
			(syn::Member::Unnamed(i), Type::Tuple(elements)) => {
				let ty = elements.get(i.index as usize).unwrap_or_else(|| fail(f, "no such element")).clone();
				Expr::new(Kind::Element(Box::new(base), i.index as usize), ty)
			}
			(syn::Member::Named(name), Type::Range(int)) if name == "start" || name == "end" => {
				let at = usize::from(name == "end");
				Expr::new(Kind::Element(Box::new(base), at), Type::Int(*int))
			}
			(syn::Member::Named(name), _) => {
				let name = name.to_string();
				if let Type::Record(r) = owner
					&& let Some(i) = self.t.api.records[r].fields.iter().position(|f| f.name == name)
				{
					let ty = self.t.api.records[r].fields[i].ty.clone();
					return Expr::new(Kind::Field(Box::new(base), i), ty);
				}
				// A getter: an object's field, or an opaque value's.
				let Some(id) = self.method_of(&owner, &name, |f| matches!(f.access, Access::Field { set: false, .. })) else {
					fail(f, format!("no field {name} on {}", self.describe(&owner)))
				};
				let ty = self.t.function(id).ok.clone();
				Expr::new(Kind::Call(id, Some(Box::new(base)), Vec::new()), ty)
			}
			_ => fail(f, format!("an element of {}", self.describe(&owner))),
		}
	}

	/// The method `name` of `owner`'s type for which `which` holds.
	fn method_of(&self, owner: &Type, name: &str, which: impl Fn(&Function) -> bool) -> Option<FnId> {
		let (owner, methods) = self.methods(owner)?;
		let index = methods.iter().position(|f| f.path.last().map(String::as_str) == Some(name) && which(f))?;
		Some(FnId { owner, index })
	}

	fn methods(&self, ty: &Type) -> Option<(Owner, &Vec<Function>)> {
		Some(match unref(ty).base() {
			Type::Record(i) => (Owner::Record(*i), &self.t.api.records[*i].methods),
			Type::Enum(i) => (Owner::Enum(*i), &self.t.api.enums[*i].methods),
			Type::Object(i) => (Owner::Object(*i), &self.t.api.objects[*i].methods),
			_ => return None,
		})
	}

	/// A struct literal: a record's, or an enum's struct variant's.
	fn structure(&mut self, s: &syn::ExprStruct, ty: Option<&Type>) -> Expr {
		if let Some((e, v)) = self.find_variant(&s.path, None) {
			let fields: Vec<(String, Type)> =
				(self.t.api.enums[e].variants[v].fields.iter()).map(|f| (f.name.clone(), f.ty.clone())).collect();
			let values = self.fields(s, &fields);
			return Expr::new(Kind::Variant(e, v, values), Type::Enum(e));
		}
		let named = self.type_of_path(&s.path);
		// A generic record's instance: the one the expected type names, else the first whose
		// fields the literal's plainly fit.
		let candidates: Vec<usize> = match (ty.map(|t| unref(t).base()), &named) {
			(Some(Type::Record(i)), _) => vec![*i],
			(_, Some(Type::Record(i))) => {
				let path = &self.t.api.records[*i].path;
				(0..self.t.api.records.len()).filter(|r| self.t.api.records[*r].path == *path).collect()
			}
			_ => fail(s, "a struct literal of no record"),
		};
		let r = candidates
			.iter()
			.copied()
			.find(|r| {
				(self.t.api.records[*r].fields.iter()).all(|f| {
					(s.fields.iter())
						.find(|fv| matches!(&fv.member, syn::Member::Named(n) if *n == f.name))
						.is_some_and(|fv| self.fits_expr(&fv.expr, &f.ty))
				})
			})
			.unwrap_or_else(|| fail(s, "a record literal fitting no instance"));
		let fields: Vec<(String, Type)> = (self.t.api.records[r].fields.iter()).map(|f| (f.name.clone(), f.ty.clone())).collect();
		if fields.len() != s.fields.len() {
			fail(s, "a record literal without each field");
		}
		let values = self.fields(s, &fields);
		Expr::new(Kind::Record(r, values), Type::Record(r))
	}

	fn fields(&mut self, s: &syn::ExprStruct, fields: &[(String, Type)]) -> Vec<Expr> {
		fields
			.iter()
			.map(|(name, ty)| {
				let fv = (s.fields.iter())
					.find(|fv| matches!(&fv.member, syn::Member::Named(n) if n == name))
					.unwrap_or_else(|| fail(s, format!("no field {name}")));
				self.check(&fv.expr, ty)
			})
			.collect()
	}

	/// Whether `expr` may be of type `ty`, judged by what it plainly is without checking it:
	/// a local's type, a struct literal's, or a suffixed literal's. What it cannot judge fits.
	fn fits_expr(&self, expr: &syn::Expr, ty: &Type) -> bool {
		let guess = match expr {
			syn::Expr::Reference(r) => return self.fits_expr(&r.expr, ty),
			syn::Expr::Paren(p) => return self.fits_expr(&p.expr, ty),
			syn::Expr::Path(p) => match p.path.get_ident().and_then(|i| self.lookup(&i.to_string())) {
				Some(Var::Typed(_, _, have)) => return fits(have, ty),
				Some(Var::Alias(_, expr)) => return self.fits_expr(&expr.clone(), ty),
				None => None,
			},
			syn::Expr::Struct(s) => self.type_of_path(&s.path),
			syn::Expr::Lit(syn::ExprLit {
				lit: syn::Lit::Int(i), ..
			}) if !i.suffix().is_empty() => int_named(i.suffix()).map(Type::Int),
			syn::Expr::Call(c) if path_is(&c.func, "NonZero::new") => return matches!(unref(ty), Type::NonZero(_)),
			syn::Expr::Call(c) if path_is(&c.func, "Bytes::from_static") => Some(Type::Bytes),
			syn::Expr::Lit(syn::ExprLit {
				lit: syn::Lit::Str(_), ..
			}) => Some(Type::Str),
			expr if const_duration(expr).is_some() => Some(Type::Duration),
			syn::Expr::MethodCall(m) if m.method == "unwrap" => return self.fits_expr(&m.receiver, ty),
			_ => None,
		};
		match (guess, unref(ty).base()) {
			(None, _) => true,
			// A generic record's literal names its type but not the instance.
			(Some(Type::Record(a)), Type::Record(b)) => self.t.api.records[a].path == self.t.api.records[*b].path,
			(Some(have), want) => fits(&have, want),
		}
	}

	/// The enum and variant `path` names: by name, or for a foreign enum, as the constant
	/// of the same name in capitals, such as tracing's `Level::WARN`.
	fn find_variant(&self, path: &syn::Path, ty: Option<&Type>) -> Option<(usize, usize)> {
		let names = self.segments(path);
		let (variant, enum_path) = names.split_last()?;
		let enum_path = enum_path.to_vec();
		let found = |e: &super::ir::Enum| {
			e.path == enum_path || (e.path.last() == enum_path.last() && !enum_path.is_empty()) || e.foreign && ty.is_some()
		};
		let (e, en) = (self.t.api.enums.iter().enumerate()).find(|(i, e)| {
			found(e) && ty.is_none_or(|t| matches!(unref(t), Type::Enum(j) if j == i) || matches!(unref(t), Type::Option(inner) if **inner == Type::Enum(*i)))
		})?;
		let v = (en.variants.iter()).position(|v| &v.name == variant || (en.foreign && v.name.to_uppercase() == *variant))?;
		Some((e, v))
	}

	/// A unit variant at `path`, or a tuple variant of `args`, as `ty` if given.
	fn variant(&mut self, path: &syn::Path, args: &[syn::Expr], ty: Option<&Type>) -> Option<Expr> {
		// `LevelFilter::OFF`, the level filter that is none.
		if let Some(Type::Option(_)) = ty
			&& path.segments.last().is_some_and(|s| s.ident == "OFF")
		{
			return Some(Expr::new(Kind::None, ty.unwrap().clone()));
		}
		let (e, v) = self.find_variant(path, ty)?;
		let fields: Vec<Type> = self.t.api.enums[e].variants[v].fields.iter().map(|f| f.ty.clone()).collect();
		if fields.len() != args.len() {
			fail(path, "a variant of another arity");
		}
		let values = args.iter().zip(&fields).map(|(a, ty)| self.check(a, ty)).collect();
		let value = Expr::new(Kind::Variant(e, v, values), Type::Enum(e));
		Some(match ty {
			Some(Type::Option(_)) => Expr::new(Kind::Some(Box::new(value)), ty.unwrap().clone()),
			_ => value,
		})
	}

	/// The functions `path` names: free functions, or associated functions of a type,
	/// including methods called by their path with the receiver first.
	fn functions(&self, path: &syn::Path) -> Vec<FnId> {
		let names = self.segments(path);
		let Some((name, owner_path)) = names.split_last() else { return Vec::new() };
		let api = &self.t.api;
		let mut exact = Vec::new();
		let mut loose = Vec::new();
		for (i, f) in api.functions.iter().enumerate() {
			if f.path == names {
				exact.push(FnId { owner: Owner::Free, index: i });
			} else if f.path.last() == Some(name) && names.len() == 1 {
				loose.push(FnId { owner: Owner::Free, index: i });
			}
		}
		if !owner_path.is_empty() {
			let owners = (api.records.iter().enumerate())
				.map(|(i, r)| (Owner::Record(i), &r.path, &r.methods))
				.chain(api.enums.iter().enumerate().map(|(i, e)| (Owner::Enum(i), &e.path, &e.methods)))
				.chain(api.objects.iter().enumerate().map(|(i, o)| (Owner::Object(i), &o.path, &o.methods)));
			for (owner, path, methods) in owners {
				let list = match () {
					_ if *path == owner_path => &mut exact,
					_ if path.last() == owner_path.last() => &mut loose,
					_ => continue,
				};
				for (index, f) in methods.iter().enumerate() {
					if f.path.last() == Some(name) && matches!(f.access, Access::Call | Access::Const | Access::Parse) {
						list.push(FnId { owner, index });
					}
				}
			}
		}
		if exact.is_empty() { loose } else { exact }
	}

	/// The function at `path` taking `args`, whose instance, for a generic one, is the one
	/// the path's type arguments name, else the first whose parameters `args` plainly fit.
	fn function(&self, path: &syn::Path, args: &[syn::Expr], expected: Option<&Type>) -> Option<FnId> {
		let turbofish: Vec<Type> = (path.segments.iter())
			.filter_map(|s| match &s.arguments {
				syn::PathArguments::AngleBracketed(a) => Some(a),
				_ => None,
			})
			.flat_map(|a| &a.args)
			.filter_map(|a| match a {
				syn::GenericArgument::Type(t) => Some(self.ty(t)),
				_ => None,
			})
			.collect();
		self.pick(self.functions(path), args, expected, &turbofish)
	}

	/// The candidate whose parameters `args` fit and whose return `expected` fits.
	fn pick(&self, candidates: Vec<FnId>, args: &[syn::Expr], expected: Option<&Type>, turbofish: &[Type]) -> Option<FnId> {
		let fitting: Vec<FnId> = candidates
			.into_iter()
			.filter(|id| {
				let f = self.t.function(*id);
				let named: Vec<Type> = match &f.instance {
					Some(i) if !i.named.is_empty() => i.named.clone(),
					_ => match self.t.owner(*id).map(|o| unref(&o).clone()) {
						Some(Type::Record(r)) => self.t.api.records[r].args.clone(),
						Some(Type::Object(o)) => self.t.api.objects[o].args.clone(),
						_ => Vec::new(),
					},
				};
				let receiver = usize::from(f.receiver.is_some());
				let params: Vec<&Type> = f.params.iter().map(|p| &p.ty).collect();
				let args: Vec<&syn::Expr> = args.iter().filter(|a| !is_unit(a)).collect();
				turbofish.iter().zip(&named).all(|(t, n)| t == n)
					&& args.len() == params.len() + receiver
					&& args[receiver..].iter().zip(&params).all(|(a, ty)| self.fits_expr(a, ty))
					&& expected.is_none_or(|e| f.err.is_some() || fits(&f.ok, e) || matches!(unref(e), Type::Result(..)))
			})
			.collect();
		fitting.first().copied()
	}

	/// A call of a path: the API's function, a variant, or a local function.
	fn call(&mut self, call: &syn::ExprCall, expected: Option<&Type>) -> Expr {
		let syn::Expr::Path(p) = &*call.func else {
			fail(call, "a call of anything but a path")
		};
		let args: Vec<syn::Expr> = call.args.iter().cloned().collect();
		if let Some(name) = p.path.get_ident().map(|i| i.to_string())
			&& let Some(signature) = self.locals.get(&name)
		{
			let output = signature.output.clone();
			self.local_params(&name);
			let ty = match output {
				syn::ReturnType::Default => Type::Unit,
				syn::ReturnType::Type(_, ty) => self.ty(&ty),
			};
			let args = args.iter().map(|a| self.infer(a)).collect();
			return Expr::new(Kind::Local(name, args), ty);
		}
		if path_is(&call.func, "String::from_utf8_lossy") {
			let value = self.infer(&args[0]);
			return Expr::new(Kind::Utf8(Box::new(value)), Type::String);
		}
		if let Some(e) = self.variant(&p.path, &args, None) {
			return e;
		}
		// A type's `Default`, which the IR keeps among its traits.
		if p.path.segments.last().is_some_and(|s| s.ident == "default") && args.is_empty() {
			let mut owner = p.path.clone();
			owner.segments.pop();
			owner.segments.pop_punct();
			if let Some(ty) = self.type_of_path(&owner) {
				return Expr::new(Kind::Default, ty);
			}
		}
		let Some(id) = self.function(&p.path, &args, expected) else {
			fail(call, format!("a call of {}, which the API does not export", self.segments(&p.path).join("::")))
		};
		let args: Vec<syn::Expr> = args.into_iter().filter(|a| !is_unit(a)).collect();
		match self.t.function(id).receiver {
			// A method called by its path, its receiver first.
			Some(_) => {
				let receiver = self.infer(&args[0]);
				self.apply(call, id, Some(receiver), &args[1..])
			}
			None => self.apply(call, id, None, &args),
		}
	}

	/// The call of `id` on `receiver`, with `args` checked against its parameters.
	fn apply(&mut self, at: &dyn Spanned, id: FnId, receiver: Option<Expr>, args: &[syn::Expr]) -> Expr {
		let params: Vec<Type> = self.t.function(id).params.iter().map(|p| p.ty.clone()).collect();
		let args: Vec<&syn::Expr> = args.iter().filter(|a| !is_unit(a)).collect();
		if params.len() != args.len() {
			fail(at, format!("{} takes {} arguments", self.t.key(id), params.len()));
		}
		let args = args.iter().zip(&params).map(|(a, ty)| self.check(a, ty)).collect();
		self.apply_checked(id, receiver, args)
	}

	fn apply_checked(&self, id: FnId, receiver: Option<Expr>, args: Vec<Expr>) -> Expr {
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
		match name.as_str() {
			"join" if matches!(m.args.first(), Some(syn::Expr::Lit(_))) => return self.join(m),
			"parse" => {
				let Some(ty) = (m.turbofish.iter().flat_map(|t| &t.args)).find_map(|a| match a {
					syn::GenericArgument::Type(t) => Some(self.ty(t)),
					_ => None,
				}) else {
					fail(m, "a parse without the type it makes")
				};
				let f = self.method_of(&ty, "parse", |f| f.access == Access::Parse);
				let err = f.and_then(|id| self.t.function(id).err.clone()).unwrap_or(Type::String);
				return self.parsed(m, &Type::Result(Box::new(ty), Box::new(err)));
			}
			// `String::from_utf8(..).unwrap()`, of bytes the transcript knows are text.
			"unwrap" if matches!(&*m.receiver, syn::Expr::Call(c) if path_is(&c.func, "String::from_utf8")) => {
				let syn::Expr::Call(c) = &*m.receiver else { unreachable!() };
				let value = self.infer(&c.args[0]);
				return Expr::new(Kind::Utf8(Box::new(value)), Type::String);
			}
			// `.err().unwrap()`, the error of a call that must fail.
			"unwrap" if matches!(&*m.receiver, syn::Expr::MethodCall(e) if e.method == "err" && e.args.is_empty()) => {
				let syn::Expr::MethodCall(e) = &*m.receiver else { unreachable!() };
				let value = self.infer(&e.receiver);
				let Type::Result(_, err) = &value.ty else {
					fail(e, "err() of no Result")
				};
				let err = (**err).clone();
				return Expr::new(Kind::UnwrapErr(Box::new(value)), err);
			}
			_ => {}
		}
		let receiver = self.infer(&m.receiver);
		let rty = unref(&receiver.ty).clone();
		let only = |kind: fn(Box<Expr>) -> Kind, ty: Type, receiver: Expr| {
			if !args.is_empty() {
				fail(m, format!("{name} with arguments"));
			}
			Expr::new(kind(Box::new(receiver)), ty)
		};
		// The API's own method of the name first, then the standard library's.
		let api = self.methods_named(&rty, &name);
		if api.is_empty() {
			match (name.as_str(), rty.base()) {
				("unwrap" | "expect", Type::Option(inner) | Type::Result(inner, _)) => {
					let inner = (**inner).clone();
					return Expr::new(Kind::Unwrap(Box::new(receiver)), inner);
				}
				("unwrap_err", Type::Result(_, err)) => {
					let err = (**err).clone();
					return only(Kind::UnwrapErr, err, receiver);
				}
				("is_some", Type::Option(_)) => return only(Kind::IsSome, Type::Bool, receiver),
				("is_none", Type::Option(_)) => return only(Kind::IsNone, Type::Bool, receiver),
				("is_err", Type::Result(..)) => return only(Kind::IsErr, Type::Bool, receiver),
				("len", Type::List(..) | Type::Vec | Type::Slice | Type::Bytes | Type::String | Type::Str) => {
					return only(Kind::Len, Type::Int(Int::Usize), receiver);
				}
				("clone", _) => {
					let ty = receiver.ty.clone();
					return only(Kind::Clone, ty, receiver);
				}
				// A `NonZero`'s value, which crosses as its integer.
				("get", Type::NonZero(_)) => return receiver,
				("collect", Type::Vec | Type::Slice | Type::Bytes) => return Expr::new(receiver.kind, Type::Vec),
				// A list collected again: the items, now a `Vec` of them.
				("collect", Type::List(_, item)) => {
					let item = item.clone();
					return Expr::new(receiver.kind, Type::List(Collection::Vec, item));
				}
				("map_or", Type::Option(inner)) => {
					let inner = (**inner).clone();
					return self.map_or(m, receiver, &inner);
				}
				(conversion, _) if CONVERSIONS.contains(&conversion) && args.is_empty() => {
					let _ = expected;
					return receiver;
				}
				_ => fail(m, format!("no method {name} on {}", self.describe(&rty))),
			}
		}
		let turbofish: Vec<Type> = (m.turbofish.iter().flat_map(|t| &t.args))
			.filter_map(|a| match a {
				syn::GenericArgument::Type(t) => Some(self.ty(t)),
				_ => None,
			})
			.collect();
		let mut with_receiver = vec![(*m.receiver).clone()];
		with_receiver.extend(args.iter().cloned());
		let id = self
			.pick(api, &with_receiver, expected, &turbofish)
			.unwrap_or_else(|| fail(m, format!("no instance of {name} on {} fits", self.describe(&rty))));
		self.apply(m, id, Some(receiver), &args)
	}

	/// `option.map_or(none, |v| some)`.
	fn map_or(&mut self, m: &syn::ExprMethodCall, option: Expr, inner: &Type) -> Expr {
		let [none, syn::Expr::Closure(c)] = m.args.iter().collect::<Vec<_>>()[..] else {
			fail(m, "a map_or other than of a default and a closure")
		};
		let none = self.infer(none);
		let [p] = c.inputs.iter().collect::<Vec<_>>()[..] else {
			fail(c, "a map_or closure of one parameter")
		};
		let depth = self.vars.len();
		let pat = self.bind(p, inner);
		let ty = none.ty.clone();
		let some = self.check(&c.body, &ty);
		self.vars.truncate(depth);
		Expr::new(
			Kind::MapOr {
				option: Box::new(option),
				none: Box::new(none),
				pat,
				some: Box::new(some),
			},
			ty,
		)
	}

	/// `items.map(|pat| body).collect::<Vec<_>>().join(sep)`.
	fn join(&mut self, m: &syn::ExprMethodCall) -> Expr {
		let [syn::Expr::Lit(syn::ExprLit {
			lit: syn::Lit::Str(sep), ..
		})] = m.args.iter().collect::<Vec<_>>()[..]
		else {
			fail(m, "a join other than on a string literal")
		};
		let syn::Expr::MethodCall(collect) = &*m.receiver else {
			fail(m, "a join of anything but a collect")
		};
		let syn::Expr::MethodCall(map) = &*collect.receiver else {
			fail(collect, "a collect of anything but a map")
		};
		if collect.method != "collect" || map.method != "map" {
			fail(m, "a join of anything but .map(..).collect()");
		}
		let [syn::Expr::Closure(c)] = map.args.iter().collect::<Vec<_>>()[..] else {
			fail(map, "a map of anything but a closure")
		};
		let items = self.infer(&map.receiver);
		let Type::List(_, item) = unref(&items.ty).base() else {
			fail(map, format!("a map over {}", self.describe(&items.ty)))
		};
		let item = (**item).clone();
		let [p] = c.inputs.iter().collect::<Vec<_>>()[..] else {
			fail(c, "a map's closure of one parameter")
		};
		let depth = self.vars.len();
		let pat = self.bind(p, &item);
		let body = self.check(&c.body, &Type::String);
		self.vars.truncate(depth);
		Expr::new(
			Kind::Join {
				items: Box::new(items),
				pat,
				body: Box::new(body),
				sep: sep.value(),
			},
			Type::String,
		)
	}

	fn methods_named(&self, owner: &Type, name: &str) -> Vec<FnId> {
		let Some((owner, methods)) = self.methods(owner) else { return Vec::new() };
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

/// Whether `expr` is `()`, an argument the IR drops.
fn is_unit(expr: &syn::Expr) -> bool {
	matches!(expr, syn::Expr::Tuple(t) if t.elems.is_empty())
}

/// Whether `expr` is the path ending in `path`, such as `String::from`.
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

/// An integer spelled by literals, constants such as `u64::MAX`, shifts, sums, and casts,
/// or `NonZero::new(..).unwrap()` of one.
fn const_int(expr: &syn::Expr) -> Option<Num> {
	let num = |v: i128| Some(Num::new(v));
	match expr {
		syn::Expr::Lit(syn::ExprLit {
			lit: syn::Lit::Int(i), ..
		}) => Some(Num {
			negative: false,
			magnitude: i.base10_parse().ok()?,
		}),
		syn::Expr::Paren(p) => const_int(&p.expr),
		syn::Expr::Cast(c) => const_int(&c.expr),
		syn::Expr::Unary(u) if matches!(u.op, syn::UnOp::Neg(_)) => {
			let n = const_int(&u.expr)?;
			Some(Num {
				negative: !n.negative,
				..n
			})
		}
		syn::Expr::Binary(b) => {
			let (l, r) = (const_int(&b.left)?, const_int(&b.right)?);
			let (l, r) = (l.magnitude, r.magnitude);
			match b.op {
				syn::BinOp::Shl(_) => Some(Num {
					negative: false,
					magnitude: l << r,
				}),
				syn::BinOp::Add(_) => Some(Num {
					negative: false,
					magnitude: l + r,
				}),
				_ => None,
			}
		}
		syn::Expr::MethodCall(m) if m.method == "unwrap" => match &*m.receiver {
			syn::Expr::Call(c) if path_is(&c.func, "NonZero::new") => const_int(&c.args[0]),
			_ => None,
		},
		syn::Expr::Path(p) => {
			let names: Vec<String> = p.path.segments.iter().map(|s| s.ident.to_string()).collect();
			let [ty, name] = &names[..] else { return None };
			let bits = |signed: bool, bits: u32| match (name.as_str(), signed) {
				("MAX", false) => Some(Num {
					negative: false,
					magnitude: if bits == 128 { u128::MAX } else { (1u128 << bits) - 1 },
				}),
				("MAX", true) => num((1i128 << (bits - 1)) - 1),
				("MIN", false) => num(0),
				("MIN", true) => Some(Num {
					negative: true,
					magnitude: 1u128 << (bits - 1),
				}),
				_ => None,
			};
			match ty.as_str() {
				"u128" => bits(false, 128),
				"char" if name == "MAX" => num(char::MAX as i128),
				ty => {
					let int = int_named(ty)?;
					bits(int.signed(), int.bits())
				}
			}
		}
		_ => None,
	}
}

/// A float spelled by a literal, `f64::NAN`, `f64::from_bits(..)`, or a negation of one.
fn const_float(expr: &syn::Expr) -> Option<f64> {
	match expr {
		syn::Expr::Lit(syn::ExprLit {
			lit: syn::Lit::Float(f), ..
		}) => f.base10_parse().ok(),
		syn::Expr::Lit(syn::ExprLit {
			lit: syn::Lit::Int(i), ..
		}) => i.base10_parse().ok(),
		syn::Expr::Unary(u) if matches!(u.op, syn::UnOp::Neg(_)) => const_float(&u.expr).map(|f| -f),
		syn::Expr::Path(p) if p.path.segments.last()?.ident == "NAN" => Some(f64::NAN),
		syn::Expr::Call(c) if path_is(&c.func, "f64::from_bits") => Some(f64::from_bits(const_int(&c.args[0])?.bits())),
		_ => None,
	}
}

/// A `Duration` spelled by its constructors or constants: its seconds and nanoseconds.
fn const_duration(expr: &syn::Expr) -> Option<(u64, u32)> {
	match expr {
		syn::Expr::Path(p) => match p.path.segments.last()?.ident.to_string().as_str() {
			"ZERO" => Some((0, 0)),
			"MAX" => Some((u64::MAX, 999_999_999)),
			_ => None,
		},
		syn::Expr::Call(c) => {
			let v = const_int(c.args.first()?)?.magnitude as u64;
			let syn::Expr::Path(p) = &*c.func else { return None };
			match p.path.segments.last()?.ident.to_string().as_str() {
				"from_secs" => Some((v, 0)),
				"from_millis" => Some((v / 1000, (v % 1000) as u32 * 1_000_000)),
				"from_nanos" => Some((v / 1_000_000_000, (v % 1_000_000_000) as u32)),
				_ => None,
			}
		}
		_ => None,
	}
}

/// An address spelled by literals: parsed from text, or built by `std::net`'s
/// constructors and constants.
enum Net {
	Text(String),
	Socket(std::net::SocketAddr),
	Ip(std::net::IpAddr),
}

fn const_net(expr: &syn::Expr) -> Option<Net> {
	use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddrV4, SocketAddrV6};
	let int = |e: &syn::Expr| const_int(e).map(|n| n.magnitude);
	match expr {
		syn::Expr::Paren(p) => const_net(&p.expr),
		syn::Expr::MethodCall(m) if m.method == "into" => const_net(&m.receiver),
		syn::Expr::MethodCall(m) if m.method == "unwrap" => match &*m.receiver {
			syn::Expr::MethodCall(p) if p.method == "parse" => match &*p.receiver {
				syn::Expr::Lit(syn::ExprLit {
					lit: syn::Lit::Str(s), ..
				}) => Some(Net::Text(s.value())),
				_ => None,
			},
			_ => None,
		},
		syn::Expr::Path(p) => {
			let names: Vec<String> = p.path.segments.iter().map(|s| s.ident.to_string()).collect();
			let [.., ty, name] = &names[..] else { return None };
			Some(Net::Ip(match (ty.as_str(), name.as_str()) {
				("Ipv4Addr", "BROADCAST") => IpAddr::V4(Ipv4Addr::BROADCAST),
				("Ipv4Addr", "LOCALHOST") => IpAddr::V4(Ipv4Addr::LOCALHOST),
				("Ipv6Addr", "LOCALHOST") => IpAddr::V6(Ipv6Addr::LOCALHOST),
				_ => return None,
			}))
		}
		syn::Expr::Call(c) => {
			let args: Vec<&syn::Expr> = c.args.iter().collect();
			if path_is(&c.func, "Ipv4Addr::new") {
				let o: Vec<u8> = args.iter().map(|a| int(a).map(|v| v as u8)).collect::<Option<_>>()?;
				return Some(Net::Ip(IpAddr::V4(Ipv4Addr::new(o[0], o[1], o[2], o[3]))));
			}
			if path_is(&c.func, "Ipv6Addr::new") {
				let s: Vec<u16> = args.iter().map(|a| int(a).map(|v| v as u16)).collect::<Option<_>>()?;
				return Some(Net::Ip(IpAddr::V6(Ipv6Addr::new(s[0], s[1], s[2], s[3], s[4], s[5], s[6], s[7]))));
			}
			let ip = |e: &syn::Expr| match const_net(e)? {
				Net::Ip(ip) => Some(ip),
				_ => None,
			};
			if path_is(&c.func, "SocketAddrV4::new") {
				let IpAddr::V4(v4) = ip(args[0])? else { return None };
				return Some(Net::Socket(SocketAddrV4::new(v4, int(args[1])? as u16).into()));
			}
			if path_is(&c.func, "SocketAddrV6::new") {
				let IpAddr::V6(v6) = ip(args[0])? else { return None };
				let addr = SocketAddrV6::new(v6, int(args[1])? as u16, int(args[2])? as u32, int(args[3])? as u32);
				return Some(Net::Socket(addr.into()));
			}
			None
		}
		_ => None,
	}
}

/// Whether a value of `have` passes where `want` is taken: the same type, behind any
/// references or boxes, a string or bytes in any of their forms, or an address of one
/// family where any is.
pub(super) fn fits(have: &Type, want: &Type) -> bool {
	let (have, want) = (unref(have).base(), unref(want).base());
	let text = |t: &Type| matches!(t, Type::Str | Type::String);
	let bytes = |t: &Type| matches!(t, Type::Slice | Type::Vec | Type::Bytes | Type::Array(_));
	let path = |t: &Type| matches!(t, Type::Path | Type::PathBuf);
	match (have, want) {
		(Type::Option(a), Type::Option(b)) => fits(a, b),
		(Type::List(_, a), Type::List(_, b)) => fits(a, b),
		(Type::Tuple(a), Type::Tuple(b)) => a.len() == b.len() && a.iter().zip(b).all(|(a, b)| fits(a, b)),
		(Type::SocketAddr(_), Type::SocketAddr(Family::Any)) | (Type::IpAddr(_), Type::IpAddr(Family::Any)) => true,
		(a, b) => a == b || (text(a) && text(b)) || (bytes(a) && bytes(b)) || (path(a) && path(b)),
	}
}
