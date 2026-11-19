//! The transcript as Swift: a file of the test's own module whose `transcript()` runs it
//! through the generated Swift, calling `tests/api.swift`'s own functions.
//!
//! Swift's binding shapes a call by rules the exporter applies, which this mirrors from the
//! IR alone: which calls throw, which objects are noncopyable structs, which methods mutate,
//! and which getters are properties. A call that can throw is `try!`, as the script expects
//! each to succeed; a `match` on a `Result` catches the error it throws.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write;

use super::ir::{Access, Api, Family, Int, Mutability, Receiver, Type};
use super::{Arm, BoundKind, Expr, FnId, Kind, Op, Owner, Pat, Piece, Spec, Step, Transcript, bound, unref};

const LANGUAGE: &str = "Swift";

/// Helpers the script calls, each for a value Swift prints apart from Rust.
const PRELUDE: &str = r#"// Generated from fixtures/api/examples/transcript.rs by tests/fixtures/transcript.

import Api
import BasicContainers

/// A string as Rust's `Debug` prints it.
func _quoted(_ s: String) -> String {
	var out = "\""
	for c in s.unicodeScalars {
		switch c {
		case "\"": out += "\\\""
		case "\\": out += "\\\\"
		case "\n": out += "\\n"
		case "\t": out += "\\t"
		case "\r": out += "\\r"
		case "\0": out += "\\0"
		case _ where c.value < 0x20 || c.value == 0x7f: out += "\\u{\(String(c.value, radix: 16))}"
		default: out.unicodeScalars.append(c)
		}
	}
	return out + "\""
}

/// An optional as Rust's `Debug` prints it, its value as `show` does.
func _option<T>(_ value: T?, _ show: (T) -> String) -> String {
	switch value {
	case .some(let value): return "Some(\(show(value)))"
	case .none: return "None"
	}
}

/// A list as Rust's `Debug` prints it, each item as `show` does.
func _list<T>(_ items: [T], _ show: (T) -> String) -> String {
	"[" + items.map(show).joined(separator: ", ") + "]"
}

/// Each of `items`, borrowed, as `show` prints it.
func _each<T: ~Copyable>(_ items: borrowing UniqueArray<T>, _ show: (borrowing T) -> String) -> [String] {
	var out: [String] = []
	for i in 0..<items.count {
		out.append(show(items[i]))
	}
	return out
}

/// `value`, given to `body`, as a value read more than once.
func _with<T, R>(_ value: T, _ body: (T) -> R) -> R {
	body(value)
}

/// An integer's lowercase hex digits, zero-padded to `width`.
func _hex<T: BinaryInteger>(_ value: T, _ width: Int) -> String {
	let digits = String(value, radix: 16)
	return String(repeating: "0", count: max(0, width - digits.count)) + digits
}

/// A `U128`'s lowercase hex digits, zero-padded to `width`.
func _hex(_ value: U128, _ width: Int) -> String {
	let low = String(value.low, radix: 16)
	let digits = value.high == 0 ? low : String(value.high, radix: 16) + String(repeating: "0", count: 16 - low.count) + low
	return String(repeating: "0", count: max(0, width - digits.count)) + digits
}

/// An integer's decimal digits, zero-padded to `width`.
func _padded<T: BinaryInteger>(_ value: T, _ width: Int) -> String {
	let digits = String(value)
	return String(repeating: "0", count: max(0, width - digits.count)) + digits
}

/// The error of a call that must fail with an `E`.
func _err<T: ~Copyable, E: Swift.Error>(_: E.Type, _ body: () throws -> T) -> E {
	do {
		_ = try body()
	} catch let error as E {
		return error
	} catch {
		fatalError("\(error)")
	}
	fatalError("the call did not fail")
}

/// Whether a call fails with an `E`.
func _fails<T: ~Copyable, E: Swift.Error>(_: E.Type, _ body: () throws -> T) -> Bool {
	do {
		_ = try body()
		return false
	} catch is E {
		return true
	} catch {
		fatalError("\(error)")
	}
}
"#;

/// The Swift file of `t`'s script.
pub(crate) fn emit(t: &Transcript) -> String {
	let model = Model::new(&t.api);
	let mut e = Emitter {
		t,
		model,
		depth: 1,
		fresh: 0,
		threw: false,
		at_root: false,
		mutated: BTreeSet::new(),
		optional: BTreeSet::new(),
		aliases: BTreeMap::new(),
		bounds: BTreeSet::new(),
	};
	for step in &t.steps {
		e.mutations(step);
	}
	let body = e.steps(&t.steps);
	let mut out = String::from(PRELUDE);
	for int in &e.bounds {
		let name = format!("Bound{}", int_name(*int));
		let _ = write!(
			out,
			"\n/// A `Bound` as Rust's `Debug` prints it.\nfunc _bound{0}(_ b: Api.{name}) -> String {{\n\tswitch b {{\n\tcase .included(let v): return \"Included(\\(v))\"\n\tcase .excluded(let v): return \"Excluded(\\(v))\"\n\tcase .unbounded: return \"Unbounded\"\n\t}}\n}}\n",
			int_name(*int)
		);
	}
	out.push_str("\n/// The transcript's script, through the generated Swift.\nfunc transcript() {\n");
	out.push_str(&body);
	out.push_str("}\n");
	out
}

/// How Swift's binding shapes the API, by the exporter's rules.
struct Model {
	/// The objects that are noncopyable structs: each but those `Copy` or thrown.
	structs: BTreeSet<usize>,
	/// The objects whose handles a call can fail a check of.
	checked: Vec<bool>,
}

impl Model {
	fn new(api: &Api) -> Model {
		let functions = || {
			(api.records.iter().flat_map(|r| &r.methods))
				.chain(api.enums.iter().flat_map(|e| &e.methods))
				.chain(api.objects.iter().flat_map(|o| &o.methods))
				.chain(&api.functions)
		};
		let thrown: BTreeSet<usize> = functions()
			.filter_map(|f| match f.err.as_ref().map(Type::base) {
				Some(Type::Object(i)) => Some(*i),
				_ => None,
			})
			.collect();
		let structs = (0..api.objects.len())
			.filter(|i| !api.objects[*i].copy && !thrown.contains(i))
			.collect();
		Model {
			structs,
			checked: checked(api),
		}
	}

	/// Whether Swift's value of `ty` is noncopyable: it holds an object's struct.
	fn unique(&self, ty: &Type) -> bool {
		match ty {
			Type::Object(i) => self.structs.contains(i),
			Type::Option(inner) | Type::Box(inner) | Type::Ref(_, inner) | Type::List(_, inner) => self.unique(inner),
			Type::Tuple(elements) => elements.iter().any(|e| self.unique(e)),
			_ => false,
		}
	}

	/// Whether `ty` is an `Option<&T>` of a struct, which Swift passes by an overload with it
	/// and one without.
	fn lent_option(&self, ty: &Type) -> bool {
		matches!(ty, Type::Option(inner) if matches!(&**inner, Type::Ref(Mutability::Not, object) if self.unique(object)))
	}

	/// Whether a parameter of `ty` is `inout`: a value Rust writes back, a reader, or a struct
	/// borrowed mutably.
	fn inout(&self, ty: &Type) -> bool {
		let written = matches!(ty, Type::Ref(Mutability::Mut, inner) if !matches!(**inner, Type::Object(_) | Type::Reader));
		let reader = matches!(ty, Type::Ref(_, inner) if **inner == Type::Reader);
		let lent = self.unique(ty)
			&& match ty {
				Type::Ref(Mutability::Mut, _) => true,
				Type::Option(inner) => matches!(**inner, Type::Ref(Mutability::Mut, _)),
				_ => false,
			};
		written || reader || lent
	}

	/// Whether the std traits of `ty` can fail a check, so throw.
	fn traits_throw(&self, api: &Api, ty: &Type) -> bool {
		match ty {
			Type::Object(i) => self.checked[*i],
			ty => {
				let mut handles = Vec::new();
				api.handles(ty, &mut handles, &mut Vec::new());
				handles.iter().any(|&i| self.checked[i])
			}
		}
	}

	/// Whether a call of `id` can fail a check of a handle it borrows or moves.
	fn status(&self, t: &Transcript, id: FnId) -> bool {
		let f = t.function(id);
		let mut handles = Vec::new();
		let mut seen = Vec::new();
		if f.receiver.is_some() {
			match id.owner {
				Owner::Object(i) => handles.push(i),
				Owner::Record(i) => t.api.handles(&Type::Record(i), &mut handles, &mut seen),
				Owner::Enum(i) => t.api.handles(&Type::Enum(i), &mut handles, &mut seen),
				Owner::Free => {}
			}
		}
		for param in &f.params {
			t.api.handles(&param.ty, &mut handles, &mut seen);
		}
		handles.iter().any(|&i| self.checked[i])
	}

	/// Whether a call of `id` throws: a future, a `Result`, a check that can fail, or a
	/// target the library may lack.
	fn throws(&self, t: &Transcript, id: FnId) -> bool {
		let f = t.function(id);
		f.future || f.err.is_some() || self.status(t, id) || !t.api.everywhere(f.on)
	}

	/// Whether `id` is a `mutating` method: a value's or a struct's `&mut self`.
	fn mutating(&self, t: &Transcript, id: FnId) -> bool {
		match (t.function(id).receiver, id.owner) {
			(Some(Receiver::Mut), Owner::Record(_) | Owner::Enum(_)) => true,
			(Some(Receiver::Mut), Owner::Object(i)) => self.structs.contains(&i),
			_ => false,
		}
	}

	/// Whether `id` is a property rather than a method.
	fn property(&self, t: &Transcript, id: FnId) -> bool {
		let f = t.function(id);
		let throws = self.throws(t, id);
		match (f.access, f.receiver) {
			(Access::Field { set: false, .. }, Some(_)) => {
				let ok = match &f.ok {
					Type::Tuple(elements) if elements.len() == 1 => &elements[0],
					ok => ok,
				};
				let getter = !f.future && f.err.is_none() && t.api.everywhere(f.on) && !self.unique(ok);
				f.params.is_empty() && !self.mutating(t, id) && (!throws || getter)
			}
			(Access::Const, None) => f.params.is_empty() && !throws,
			_ => false,
		}
	}
}

/// Whether a call can fail a check of each object's handle, as the exporter's `checked`
/// decides: its value is not `Sync`, a call borrows it exclusively or moves it, or a handle
/// to it is lent.
fn checked(api: &Api) -> Vec<bool> {
	fn param(api: &Api, ty: &Type, owned: bool, checked: &mut [bool]) {
		match ty {
			Type::Object(i) if owned && !api.objects[*i].copy => checked[*i] = true,
			Type::Ref(Mutability::Mut, inner) => {
				if let Type::Object(i) = inner.base() {
					checked[*i] = true;
				}
				param(api, inner, true, checked);
			}
			Type::Ref(Mutability::Not, inner) => param(api, inner, false, checked),
			Type::Option(inner) | Type::Box(inner) | Type::List(_, inner) => param(api, inner, owned, checked),
			Type::Tuple(elements) => elements.iter().for_each(|ty| param(api, ty, owned, checked)),
			Type::Closure(closure) => closure.params.iter().for_each(|ty| lent(api, ty, checked)),
			Type::Record(_) | Type::Enum(_) if owned => lent(api, ty, checked),
			_ => {}
		}
	}
	fn lent(api: &Api, ty: &Type, checked: &mut [bool]) {
		let mut handles = Vec::new();
		api.handles(ty, &mut handles, &mut Vec::new());
		handles.into_iter().for_each(|i| checked[i] = true);
	}
	let mut checked: Vec<bool> = api.objects.iter().map(|o| !o.sync).collect();
	let functions = (api.records.iter().flat_map(|r| &r.methods))
		.chain(api.enums.iter().flat_map(|e| &e.methods))
		.chain(&api.functions)
		.map(|f| (None, f))
		.chain((api.objects.iter().enumerate()).flat_map(|(i, o)| o.methods.iter().map(move |f| (Some(i), f))));
	for (owner, f) in functions {
		match (owner, f.receiver) {
			(Some(i), Some(Receiver::Mut)) => checked[i] = true,
			(Some(i), Some(Receiver::Owned)) if !api.objects[i].copy => checked[i] = true,
			_ => {}
		}
		f.params.iter().for_each(|p| param(api, &p.ty, true, &mut checked));
		if f.lends {
			lent(api, &f.ok, &mut checked);
		}
		if f.lends_mut
			&& let Type::Object(j) = f.ok
		{
			checked[j] = true;
		}
	}
	for method in api.callbacks.iter().flat_map(|c| &c.methods) {
		method.params.iter().for_each(|p| lent(api, &p.ty, &mut checked));
		lent(api, &method.ok, &mut checked);
		method.err.iter().for_each(|err| lent(api, err, &mut checked));
	}
	checked
}

struct Emitter<'t> {
	t: &'t Transcript,
	model: Model,
	depth: usize,
	fresh: usize,
	/// Whether the expression being written throws, so its root is `try!`.
	threw: bool,
	/// Whether the call about to be written is a statement's whole value.
	at_root: bool,
	/// The locals Swift changes, each a `var`.
	mutated: BTreeSet<String>,
	/// The locals Swift holds optional, each read unwrapped: a class Rust drops early, or a
	/// struct passed as an optional `inout`.
	optional: BTreeSet<String>,
	/// The names a closure's tuple pattern binds, each read as the element it names.
	aliases: BTreeMap<String, String>,
	/// The integers whose `Bound` the script prints.
	bounds: BTreeSet<Int>,
}

/// Swift's keywords, which a local's name is quoted as.
const KEYWORDS: &[&str] = &[
	"as", "break", "case", "catch", "class", "continue", "default", "defer", "do", "else", "enum", "extension", "fallthrough",
	"false", "for", "func", "guard", "if", "import", "in", "init", "inout", "internal", "is", "let", "nil", "operator",
	"private", "protocol", "public", "repeat", "rethrows", "return", "self", "static", "struct", "subscript", "super",
	"switch", "throw", "throws", "true", "try", "var", "where", "while",
];

/// A name as Swift spells it, quoted if a keyword.
fn kw(name: &str) -> String {
	match KEYWORDS.contains(&name) {
		true => format!("`{name}`"),
		false => name.to_string(),
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
		for l in line.lines() {
			let _ = writeln!(out, "{}{l}", self.indent());
		}
	}

	/// Record which locals Swift changes or holds optional.
	fn mutations(&mut self, step: &Step) {
		let visit = |e: &Expr, this: &mut Self| {
			walk(e, &mut |e| {
				let Kind::Call(id, receiver, args) = &e.kind else { return };
				if this.model.mutating(this.t, *id)
					&& let Some(Kind::Var(name)) = receiver.as_ref().map(|r| &r.kind)
				{
					this.mutated.insert(name.clone());
				}
				for (arg, param) in args.iter().zip(&this.t.function(*id).params) {
					if !this.model.inout(&param.ty) {
						continue;
					}
					match &arg.kind {
						Kind::Var(name) => {
							this.mutated.insert(name.clone());
						}
						Kind::Some(inner) => match &inner.kind {
							Kind::Var(name) => {
								this.mutated.insert(name.clone());
								this.optional.insert(name.clone());
							}
							_ => panic!("an optional &mut of anything but a local"),
						},
						_ => {}
					}
				}
			})
		};
		match step {
			Step::Drop(name) => {
				// A class goes once Swift's last reference does, so Rust's early drop is Swift's
				// release of an optional's.
				if !self.dropped_struct(name) {
					self.optional.insert(name.clone());
					self.mutated.insert(name.clone());
				}
			}
			Step::Let(_, e) | Step::Do(e) => visit(e, self),
			Step::Print(pieces) => pieces.iter().for_each(|p| {
				if let Piece::Value(e, _) = p {
					visit(e, self)
				}
			}),
			Step::Match { value, ok, err } => {
				visit(value, self);
				ok.steps.iter().chain(&err.steps).for_each(|s| self.mutations(s));
			}
		}
		// The steps of each closure.
		let mut closures = Vec::new();
		each_expr(step, &mut |e| {
			walk(e, &mut |e| {
				if let Kind::Closure(_, steps, _) = &e.kind {
					closures.extend(steps.iter().cloned());
				}
			})
		});
		for step in &closures {
			self.mutations(step);
		}
	}

	/// Whether the local `name` Rust drops early is a struct, which Swift consumes.
	fn dropped_struct(&self, name: &str) -> bool {
		let mut types = Vec::new();
		let mut find = |steps: &[Step]| {
			for step in steps {
				let mut bound_names = Vec::new();
				if let Step::Let(pat, _) = step {
					bound(pat, &mut bound_names);
				}
				types.extend(bound_names.into_iter().filter(|(n, _)| n == name).map(|(_, ty)| ty));
			}
		};
		find(&self.t.steps);
		let ty = types.first().unwrap_or_else(|| panic!("a drop of {name}, which no let binds"));
		self.model.unique(ty)
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
			Step::Let(pat, value) => {
				let literal = matches!(&value.kind, Kind::Tuple(_)) && !matches!(unref(&value.ty).base(), Type::Range(_) | Type::Bounds(_));
				match (pat, &value.kind) {
					// A tuple literal binds each element, so each may be its own `let` or `var`.
					(Pat::Tuple(pats), Kind::Tuple(elements)) if literal => {
						for (pat, element) in pats.iter().zip(elements) {
							self.step(&Step::Let(pat.clone(), element.clone()), out);
						}
					}
					(Pat::Name(name, _), _) => {
						self.at_root = matches!(value.kind, Kind::Call(..));
						let v = self.root(value, &value.ty, out);
						self.at_root = false;
						let typed = match self.optional.contains(name) {
							true => format!(": {}?", self.sw(&value.ty)),
							false if literalish(value) => format!(": {}", self.sw(&value.ty)),
							false => String::new(),
						};
						let keyword = self.keyword(name);
						self.line(out, &format!("{keyword} {}{typed} = {v}", kw(name)));
					}
					(pat, _) => {
						self.at_root = matches!(value.kind, Kind::Call(..));
						let v = self.root(value, &value.ty, out);
						self.at_root = false;
						self.bind(pat, &v, &value.ty, out);
					}
				}
			}
			Step::Print(pieces) => {
				let v = self.pieces(pieces, out);
				self.line(out, &format!("print({v})"));
			}
			Step::Do(value) => {
				self.at_root = matches!(value.kind, Kind::Call(..));
				let v = self.root(value, &value.ty, out);
				self.at_root = false;
				match unref(&value.ty) {
					Type::Unit => self.line(out, &v),
					_ => self.line(out, &format!("_ = {v}")),
				}
			}
			Step::Drop(name) => match self.optional.contains(name) {
				true => self.line(out, &format!("{} = nil", kw(name))),
				false => self.line(out, &format!("_ = consume {}", kw(name))),
			},
			Step::Match { value, ok, err } => self.matched(value, ok, err, out),
		}
	}

	/// The keyword declaring the local `name`.
	fn keyword(&self, name: &str) -> &'static str {
		match self.mutated.contains(name) || self.optional.contains(name) {
			true => "var",
			false => "let",
		}
	}

	/// `pat` bound to `v`, a value of `ty`.
	fn bind(&mut self, pat: &Pat, v: &str, ty: &Type, out: &mut String) {
		match pat {
			Pat::Name(name, _) => {
				let keyword = self.keyword(name);
				let typed = match self.optional.contains(name) {
					true => format!(": {}?", self.sw(ty)),
					false => String::new(),
				};
				self.line(out, &format!("{keyword} {}{typed} = {v}", kw(name)));
			}
			Pat::Ignore(_) => self.line(out, &format!("_ = {v}")),
			Pat::Tuple(pats) => {
				let parts = self.parts(pats, ty);
				// A tuple of one is its element.
				if let [(pat, _, _)] = &parts[..] {
					return self.bind(pat, v, &parts[0].2, out);
				}
				let tmp = self.fresh("t");
				self.line(out, &format!("let {tmp} = {v}"));
				for (pat, element, ty) in parts {
					self.bind(&pat, &format!("{tmp}{element}"), &ty, out);
				}
			}
		}
	}

	/// The parts of a tuple pattern of `ty`: each pattern, the member that reads its value,
	/// and its type.
	fn parts(&self, pats: &[Pat], ty: &Type) -> Vec<(Pat, String, Type)> {
		match unref(ty).base() {
			Type::Bounds(int) => vec![
				(pats[0].clone(), ".start".to_string(), Type::Bound(*int)),
				(pats[1].clone(), ".end".to_string(), Type::Bound(*int)),
			],
			Type::Tuple(elements) => {
				let unique = self.model.unique(unref(ty).base());
				(pats.iter().zip(elements).enumerate())
					.map(|(n, (p, ty))| {
						let member = match unique {
							true => format!("._{n}"),
							false => format!(".{n}"),
						};
						(p.clone(), member, ty.clone())
					})
					.collect()
			}
			ty => panic!("a tuple pattern of {ty:?}"),
		}
	}

	/// `pat`, the parameter of a closure, as Swift names it, each name a tuple pattern binds
	/// read as the element it names.
	fn param(&mut self, pat: &Pat) -> String {
		match pat {
			Pat::Name(name, _) => kw(name),
			Pat::Ignore(_) => "_".to_string(),
			Pat::Tuple(_) => {
				let tmp = self.fresh("p");
				self.alias(pat, &tmp);
				tmp
			}
		}
	}

	fn alias(&mut self, pat: &Pat, v: &str) {
		match pat {
			Pat::Name(name, _) => {
				self.aliases.insert(name.clone(), v.to_string());
			}
			Pat::Ignore(_) => {}
			Pat::Tuple(pats) => {
				let ty = pat_type(pat);
				let parts = self.parts(pats, &ty);
				if let [(pat, _, _)] = &parts[..] {
					return self.alias(pat, v);
				}
				for (pat, element, _) in parts {
					self.alias(&pat, &format!("{v}{element}"));
				}
			}
		}
	}

	/// The Swift of `e` as a whole expression: `try!` if any call it makes throws.
	fn root(&mut self, e: &Expr, want: &Type, out: &mut String) -> String {
		self.rooted("try! ", out, |this, out| this.value(e, want, out))
	}

	/// `write`'s expression, prefixed with `prefix` if it throws.
	fn rooted(&mut self, prefix: &str, out: &mut String, write: impl FnOnce(&mut Self, &mut String) -> String) -> String {
		let saved = std::mem::replace(&mut self.threw, false);
		let v = write(self, out);
		match std::mem::replace(&mut self.threw, saved) {
			true => format!("{prefix}{v}"),
			false => v,
		}
	}

	fn matched(&mut self, value: &Expr, ok: &Arm, err: &Arm, out: &mut String) {
		match &value.ty {
			Type::Option(inner) => {
				let v = self.root(value, &value.ty, out);
				self.line(out, &format!("switch {v} {{"));
				let binding = match &ok.pat {
					Some(Pat::Name(name, _)) => format!("{} {}", self.keyword(name), kw(name)),
					Some(Pat::Ignore(_)) | None => "_".to_string(),
					Some(Pat::Tuple(_)) => format!("let {}", self.fresh("o")),
				};
				self.line(out, &format!("case .some({binding}):"));
				self.depth += 1;
				let mut body = String::new();
				if let Some(pat @ Pat::Tuple(_)) = &ok.pat {
					let tmp = binding.trim_start_matches("let ").to_string();
					self.bind(pat, &tmp, inner, &mut body);
				}
				body.push_str(&self.steps(&ok.steps));
				self.body(body, out);
				self.depth -= 1;
				self.line(out, "case .none:");
				self.depth += 1;
				let body = self.steps(&err.steps);
				self.body(body, out);
				self.depth -= 1;
				self.line(out, "}");
			}
			Type::Result(ok_ty, err_ty) => {
				let (exception, wrapped) = self.exception(err_ty);
				self.line(out, "do {");
				self.depth += 1;
				let mut body = String::new();
				let v = self.rooted("try ", &mut body, |this, out| this.value(value, &value.ty, out));
				match (&ok.pat, unref(ok_ty)) {
					(_, Type::Unit) => self.line(&mut body, &v),
					(Some(pat), _) => self.bind(pat, &v, ok_ty, &mut body),
					(None, _) => self.line(&mut body, &format!("_ = {v}")),
				}
				body.push_str(&self.steps(&ok.steps));
				out.push_str(&body);
				self.depth -= 1;
				let e = self.fresh("e");
				self.line(out, &format!("}} catch let {e} as {exception} {{"));
				self.depth += 1;
				let mut body = String::new();
				if let Some(pat) = &err.pat {
					let error = match wrapped {
						true => format!("{e}.error"),
						false => e.clone(),
					};
					self.bind(pat, &error, err_ty, &mut body);
				}
				body.push_str(&self.steps(&err.steps));
				self.body(body, out);
				self.depth -= 1;
				self.line(out, "} catch {");
				self.depth += 1;
				self.line(out, "fatalError(\"\\(error)\")");
				self.depth -= 1;
				self.line(out, "}");
			}
			ty => panic!("a match on {ty:?}"),
		}
	}

	/// A case's body, which Swift requires hold a statement.
	fn body(&self, body: String, out: &mut String) {
		match body.is_empty() {
			true => self.line(out, "break"),
			false => out.push_str(&body),
		}
	}

	/// The type a call failing with `err` throws, and whether it wraps the error in its
	/// `error`.
	fn exception(&self, err: &Type) -> (String, bool) {
		match unref(err).base() {
			ty @ (Type::Record(_) | Type::Enum(_) | Type::Object(_)) => (self.type_name(ty), false),
			_ => {
				let key = format!("error {}", self.spell(&owned(err)));
				(format!("Api.{}", self.t.named(LANGUAGE, &key)), true)
			}
		}
	}

	/// A type as the export keys it: its C name, past the prefix, which C++ names it by.
	fn key(&self, ty: &Type) -> String {
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
			Type::Option(inner) => format!("option_{}", self.key(inner)),
			Type::List(_, inner) => format!("vec_{}", self.key(inner)),
			Type::Record(_) | Type::Enum(_) | Type::Object(_) => self.t.type_name("C++", ty).to_string(),
			Type::Range(int) => format!("range_{}", int.rust()),
			Type::Bound(int) => format!("bound_{}", int.rust()),
			Type::Bounds(int) => format!("bounds_{}", int.rust()),
			Type::Ref(_, inner) | Type::Box(inner) => self.key(inner),
			other => panic!("no key spells {other:?}"),
		}
	}

	/// A type by its key, each tuple spelled as Rust spells it.
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

	/// The Swift name of a record, enum, or object, qualified by the module.
	fn type_name(&self, ty: &Type) -> String {
		format!("Api.{}", self.t.type_name(LANGUAGE, ty))
	}

	/// The Swift type of `ty`.
	fn sw(&self, ty: &Type) -> String {
		match ty {
			Type::Unit => "Void".to_string(),
			Type::Bool => "Bool".to_string(),
			Type::Int(int) => sw_int(*int).to_string(),
			Type::NonZero(int) => format!("NonZero<{}>", sw_int(*int)),
			Type::U128 => "U128".to_string(),
			Type::F32 => "Float".to_string(),
			Type::F64 => "Double".to_string(),
			Type::Char => "Unicode.Scalar".to_string(),
			Type::Str | Type::String => "String".to_string(),
			Type::Slice | Type::Vec | Type::Bytes => "Bytes".to_string(),
			Type::Array(n) => format!("Api.ArrayU8x{n}"),
			Type::Path | Type::PathBuf => "PathBuf".to_string(),
			Type::Duration => "RustDuration".to_string(),
			Type::SocketAddr(Family::Any) => "SocketAddr".to_string(),
			Type::SocketAddr(Family::V4) => "SocketAddrV4".to_string(),
			Type::SocketAddr(Family::V6) => "SocketAddrV6".to_string(),
			Type::IpAddr(Family::Any) => "IpAddr".to_string(),
			Type::IpAddr(Family::V4) => "Ipv4Addr".to_string(),
			Type::IpAddr(Family::V6) => "Ipv6Addr".to_string(),
			Type::List(_, item) if self.model.unique(item) => format!("UniqueArray<{}>", self.sw(item)),
			Type::List(_, item) => format!("[{}]", self.sw(item)),
			Type::Option(inner) => format!("{}?", self.sw(inner)),
			Type::Box(inner) | Type::Ref(_, inner) => self.sw(inner),
			Type::Record(_) | Type::Enum(_) | Type::Object(_) => self.type_name(ty),
			Type::Callback(..) => panic!("a local of a callback"),
			Type::Range(int) => format!("Api.Range{}", int_name(*int)),
			Type::Bound(int) => format!("Api.Bound{}", int_name(*int)),
			Type::Bounds(int) => format!("Api.Bounds{}", int_name(*int)),
			Type::Tuple(elements) => match &elements[..] {
				[element] => self.sw(element),
				_ if self.model.unique(ty) => self.tuple_name(elements),
				elements => {
					let elements: Vec<String> = elements.iter().map(|e| self.sw(e)).collect();
					format!("({})", elements.join(", "))
				}
			},
			ty => panic!("a local of {ty:?}"),
		}
	}

	/// The struct of a tuple holding a struct, which a Swift tuple cannot.
	fn tuple_name(&self, elements: &[Type]) -> String {
		let key = format!("tuple {}", self.spell(&owned(&Type::Tuple(elements.to_vec()))));
		format!("Api.{}", self.t.named(LANGUAGE, &key))
	}

	/// The Swift of a format's pieces: a string literal. A piece that needs statements first
	/// has each piece before it written to a local, so they run in order.
	fn pieces(&mut self, pieces: &[Piece], out: &mut String) -> String {
		let mut parts = String::new();
		for piece in pieces {
			match piece {
				Piece::Text(text) => parts.push_str(&escape(text)),
				Piece::Value(e, spec) => {
					let mut pre = String::new();
					let mut v = self.rooted("try! ", &mut pre, |this, pre| {
						let v = this.value(e, &e.ty, pre);
						this.show(&v, &e.ty, *spec)
					});
					// A string's interpolation is one line: a closure of several is a local's.
					if v.contains('\n') {
						let tmp = self.fresh("v");
						self.line(&mut pre, &format!("let {tmp} = {v}"));
						v = tmp;
					}
					if !pre.is_empty() && !parts.is_empty() {
						let tmp = self.fresh("s");
						self.line(out, &format!("let {tmp} = \"{parts}\""));
						parts = format!("\\({tmp})");
					}
					out.push_str(&pre);
					let _ = write!(parts, "\\({v})");
				}
			}
		}
		format!("\"{parts}\"")
	}

	/// `v`, a value of `ty`, as Swift interpolates it the way Rust formats it with `spec`.
	fn show(&mut self, v: &str, ty: &Type, spec: Spec) -> String {
		let ty = unref(ty).base();
		match (spec, ty) {
			(Spec::Hex(width), Type::Int(int)) if !int.signed() => return format!("_hex({v}, {width})"),
			(Spec::Hex(width), Type::U128) => return format!("_hex({v}, {width})"),
			(Spec::Padded(width), Type::Int(_)) => return format!("_padded({v}, {width})"),
			(Spec::Debug, _) => return self.debug(v, ty),
			(Spec::Display, _) => {}
			(spec, ty) => panic!("{spec:?} of {ty:?}"),
		}
		match ty {
			Type::Bool | Type::Int(_) | Type::U128 | Type::NonZero(_) | Type::String | Type::Str | Type::Char => v.to_string(),
			Type::Object(i) if !self.t.api.objects[*i].traits.display => {
				let o = &self.t.api.objects[*i];
				let Some(index) = o.methods.iter().position(|m| m.access == Access::Text) else {
					panic!("Display of {}, which has none", self.t.rust(ty))
				};
				let id = FnId {
					owner: Owner::Object(*i),
					index,
				};
				self.threw |= self.model.throws(self.t, id);
				format!("{}.{}()", postfix(v), self.t.name(LANGUAGE, id))
			}
			Type::Object(_) | Type::Record(_) | Type::Enum(_) => {
				self.threw |= self.model.traits_throw(&self.t.api, ty);
				format!("{}.description", postfix(v))
			}
			ty => panic!("Display of {ty:?}"),
		}
	}

	/// `x` as Rust's `Debug` prints a value of `ty`, in a closure of its own: `try!` if it
	/// throws.
	fn debug_in(&mut self, x: &str, ty: &Type) -> String {
		let mut pre = String::new();
		self.rooted("try! ", &mut pre, |this, _| this.debug(x, ty))
	}

	/// `v` as Rust's `Debug` prints a value of `ty`.
	fn debug(&mut self, v: &str, ty: &Type) -> String {
		match unref(ty).base() {
			Type::Bool | Type::Int(_) | Type::U128 | Type::NonZero(_) => v.to_string(),
			Type::String | Type::Str => format!("_quoted({v})"),
			Type::Vec | Type::Slice | Type::Bytes | Type::Array(_) => format!("Array({v})"),
			Type::Option(inner) => {
				let x = self.fresh("x");
				let show = self.debug_in(&x, inner);
				format!("_option({v}) {{ {x} in \"\\({show})\" }}")
			}
			Type::List(_, item) if self.model.unique(item) => {
				let x = self.fresh("x");
				let show = self.debug_in(&x, item);
				format!("\"[\" + _each({v}) {{ {x} in \"\\({show})\" }}.joined(separator: \", \") + \"]\"")
			}
			Type::List(_, item) => {
				let x = self.fresh("x");
				let show = self.debug_in(&x, item);
				format!("_list({v}) {{ {x} in \"\\({show})\" }}")
			}
			Type::Tuple(elements) if elements.len() == 1 => format!("\"(\\({}),)\"", self.debug(v, &elements[0])),
			Type::Tuple(elements) => {
				let x = self.fresh("x");
				let unique = self.model.unique(ty);
				let parts: Vec<String> = (elements.iter().enumerate())
					.map(|(n, ty)| {
						let element = match unique {
							true => format!("{x}._{n}"),
							false => format!("{x}.{n}"),
						};
						format!("\\({})", self.debug_in(&element, ty))
					})
					.collect();
				format!("_with({v}) {{ {x} in \"({})\" }}", parts.join(", "))
			}
			Type::Bound(int) => {
				self.bounds.insert(*int);
				format!("_bound{}({v})", int_name(*int))
			}
			ty @ (Type::Record(_) | Type::Enum(_) | Type::Object(_)) => {
				self.threw |= self.model.traits_throw(&self.t.api, ty);
				format!("{}.debugDescription", postfix(v))
			}
			ty => panic!("Debug of {ty:?}"),
		}
	}

	/// The Swift of `e`, passed where `want` is taken: an address of one family widened to
	/// either's, as Rust's `into` does.
	fn value(&mut self, e: &Expr, want: &Type, out: &mut String) -> String {
		let v = self.value_of(e, want, out);
		match (unref(&e.ty).base(), unref(want).base()) {
			(Type::SocketAddr(Family::V4), Type::SocketAddr(Family::Any)) => format!("SocketAddr.v4({v})"),
			(Type::SocketAddr(Family::V6), Type::SocketAddr(Family::Any)) => format!("SocketAddr.v6({v})"),
			(Type::IpAddr(Family::V4), Type::IpAddr(Family::Any)) => format!("IpAddr.v4({v})"),
			(Type::IpAddr(Family::V6), Type::IpAddr(Family::Any)) => format!("IpAddr.v6({v})"),
			_ => v,
		}
	}

	fn value_of(&mut self, e: &Expr, want: &Type, out: &mut String) -> String {
		match &e.kind {
			Kind::Unit => "()".to_string(),
			Kind::Bool(b) => b.to_string(),
			Kind::Int(n) => match unref(want).base() {
				Type::U128 => {
					let v = n.magnitude;
					format!("U128(high: {}, low: {})", (v >> 64) as u64, v as u64)
				}
				Type::NonZero(int) => {
					self.threw = true;
					format!("NonZero<{}>({n})", sw_int(*int))
				}
				_ => n.to_string(),
			},
			Kind::Float(f) => {
				let f32 = matches!(unref(want).base(), Type::F32);
				match (f32, f.is_finite() && !(*f == 0.0 && f.is_sign_negative())) {
					(false, true) => format!("{f:?}"),
					(true, true) => format!("Float({f:?})"),
					(false, false) => format!("Double(bitPattern: {:#x})", f.to_bits()),
					(true, false) => format!("Float(bitPattern: {:#x})", (*f as f32).to_bits()),
				}
			}
			Kind::Char(c) => format!("Unicode.Scalar({:#x} as UInt32)!", *c as u32),
			Kind::Str(s) => match unref(want).base() {
				Type::Path | Type::PathBuf => format!("PathBuf(\"{}\")", escape(s)),
				_ => format!("\"{}\"", escape(s)),
			},
			Kind::Bytes(b) => match unref(want).base() {
				Type::Array(n) => format!("Api.ArrayU8x{n}({})!", byte_list(b)),
				_ => format!("Bytes({})", byte_list(b)),
			},
			Kind::Duration(secs, nanos) => {
				self.threw = true;
				format!("RustDuration(secs: {secs}, nanos: {nanos})")
			}
			Kind::Socket(addr) => {
				let (v4, v6) = match addr {
					std::net::SocketAddr::V4(a) => (Some(format!("SocketAddrV4(ip: {}, port: {})", ipv4(*a.ip()), a.port())), None),
					std::net::SocketAddr::V6(a) => (
						None,
						Some(format!(
							"SocketAddrV6(ip: {}, port: {}, flowinfo: {}, scopeId: {})",
							ipv6(*a.ip()),
							a.port(),
							a.flowinfo(),
							a.scope_id()
						)),
					),
				};
				match (unref(want).base(), v4, v6) {
					(Type::SocketAddr(Family::Any), Some(v), _) => format!("SocketAddr.v4({v})"),
					(Type::SocketAddr(Family::Any), _, Some(v)) => format!("SocketAddr.v6({v})"),
					(Type::SocketAddr(Family::V4), Some(v), _) | (Type::SocketAddr(Family::V6), _, Some(v)) => v,
					(ty, ..) => panic!("a socket address as {ty:?}"),
				}
			}
			Kind::Ip(ip) => {
				let (v, family) = match ip {
					std::net::IpAddr::V4(ip) => (ipv4(*ip), "v4"),
					std::net::IpAddr::V6(ip) => (ipv6(*ip), "v6"),
				};
				match unref(want).base() {
					Type::IpAddr(Family::Any) => format!("IpAddr.{family}({v})"),
					_ => v,
				}
			}
			Kind::None => "nil".to_string(),
			Kind::Some(inner) => {
				let inner_want = match unref(want).base() {
					Type::Option(inner) => (**inner).clone(),
					_ => inner.ty.clone(),
				};
				let v = self.value(inner, &inner_want, out);
				// Some of what Swift holds optional already nests, else a nil would be its none.
				match optional(&inner_want) {
					true => format!(".some({v})"),
					false => v,
				}
			}
			Kind::Tuple(elements) => self.tuple(e, elements, want, out),
			Kind::List(items) => self.list(items, want, out),
			Kind::Record(r, fields) => {
				let record = &self.t.api.records[*r];
				let name = self.type_name(&Type::Record(*r));
				let labels: Vec<Option<String>> = (record.fields.iter())
					.map(|f| (!record.tuple).then(|| field_name(&f.name)))
					.collect();
				let types: Vec<Type> = record.fields.iter().map(|f| f.ty.clone()).collect();
				let args = self.labelled(fields, &types, &labels, out);
				format!("{name}({args})")
			}
			Kind::Variant(en, v, fields) => {
				let name = self.type_name(&Type::Enum(*en));
				let variant = &self.t.api.enums[*en].variants[*v];
				let case = member(&snake(&variant.name));
				if variant.fields.is_empty() {
					return format!("{name}.{case}");
				}
				let labels: Vec<Option<String>> = (variant.fields.iter())
					.map(|f| (!variant.tuple).then(|| field_name(&f.name)))
					.collect();
				let types: Vec<Type> = variant.fields.iter().map(|f| f.ty.clone()).collect();
				let args = self.labelled(fields, &types, &labels, out);
				format!("{name}.{case}({args})")
			}
			Kind::Bound(kind, value) => match (kind, value) {
				(BoundKind::Unbounded, _) => format!("{}.unbounded", self.sw(unref(&e.ty))),
				(BoundKind::Included, Some(v)) => format!("{}.included({})", self.sw(unref(&e.ty)), self.expr(v, out)),
				(BoundKind::Excluded, Some(v)) => format!("{}.excluded({})", self.sw(unref(&e.ty)), self.expr(v, out)),
				_ => unreachable!(),
			},
			Kind::Default => {
				let ty = unref(&e.ty).base();
				let on = match ty {
					Type::Record(i) => self.t.api.records[*i].on,
					Type::Enum(i) => self.t.api.enums[*i].on,
					Type::Object(i) => self.t.api.objects[*i].on,
					ty => panic!("a default of {ty:?}"),
				};
				self.threw |= !self.t.api.everywhere(on);
				format!("{}.default()", self.type_name(ty))
			}
			Kind::Var(name) => self.var(name),
			Kind::Const(c) => format!(
				"Api.{}",
				self.t.named(LANGUAGE, &format!("const {}", self.t.api.constants[*c].path.join("::")))
			),
			Kind::Call(id, receiver, args) => self.call(*id, receiver.as_deref(), args, out),
			Kind::Field(base, i) => {
				let b = self.expr(base, out);
				let Type::Record(r) = unref(&base.ty).base() else { panic!("a field of {:?}", base.ty) };
				format!("{}.{}", postfix(&b), field_name(&self.t.api.records[*r].fields[*i].name))
			}
			Kind::Element(base, i) => {
				let b = self.expr(base, out);
				match unref(&base.ty).base() {
					Type::Range(_) => format!("{}.{}", postfix(&b), if *i == 0 { "start" } else { "end" }),
					Type::Tuple(elements) if elements.len() == 1 => b,
					ty if self.model.unique(ty) => format!("{}._{i}", postfix(&b)),
					_ => format!("{}.{i}", postfix(&b)),
				}
			}
			Kind::Clone(inner) => {
				let v = self.expr(inner, out);
				match unref(&inner.ty).base() {
					ty @ Type::Object(i) if self.t.api.objects[*i].traits.clone => {
						self.threw |= self.model.traits_throw(&self.t.api, ty);
						format!("{}.clone()", postfix(&v))
					}
					// What a call returns is the caller's own already.
					ty if self.model.unique(ty) && matches!(inner.kind, Kind::Call(..)) => v,
					ty if self.model.unique(ty) => panic!("a clone of {ty:?}, which Swift cannot copy"),
					_ => v,
				}
			}
			Kind::Unwrap(inner) => {
				let v = self.expr(inner, out);
				match &inner.ty {
					Type::Result(..) => v,
					_ => format!("{}!", postfix(&v)),
				}
			}
			Kind::UnwrapErr(inner) => {
				let Type::Result(_, err) = &inner.ty else { panic!("unwrap_err of {:?}", inner.ty) };
				let (exception, wrapped) = self.exception(err);
				let call = self.attempt(inner);
				match wrapped {
					true => format!("_err({exception}.self) {{ {call} }}.error"),
					false => format!("_err({exception}.self) {{ {call} }}"),
				}
			}
			Kind::IsErr(inner) => {
				let Type::Result(_, err) = &inner.ty else { panic!("is_err of {:?}", inner.ty) };
				let (exception, _) = self.exception(err);
				let call = self.attempt(inner);
				format!("_fails({exception}.self) {{ {call} }}")
			}
			Kind::IsSome(inner) => format!("({} != nil)", self.expr(inner, out)),
			Kind::IsNone(inner) => format!("({} == nil)", self.expr(inner, out)),
			Kind::Binary(op, a, b) => {
				if matches!(op, Op::Eq | Op::Ne) {
					let ty = unref(&a.ty).base();
					if matches!(ty, Type::Record(_) | Type::Enum(_) | Type::Object(_)) {
						self.threw |= self.model.traits_throw(&self.t.api, ty);
					}
				}
				let (a, b) = (self.expr(a, out), self.expr(b, out));
				let op = match op {
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
				format!("({a} {op} {b})")
			}
			Kind::Closure(params, steps, value) => self.closure(params, steps, value),
			Kind::Local(name, args) => {
				let args: Vec<String> = args.iter().map(|a| self.expr(a, out)).collect();
				match name.starts_with(|c: char| c.is_uppercase()) {
					true => format!("{name}({})", args.join(", ")),
					false => format!("{}({})", camel(name), args.join(", ")),
				}
			}
			Kind::Format(pieces) => self.pieces(pieces, out),
			Kind::Join { items, pat, body, sep } => {
				let item = match unref(&items.ty).base() {
					Type::List(_, item) => (**item).clone(),
					ty => panic!("a join over {ty:?}"),
				};
				let v = self.expr(items, out);
				let x = self.param(pat);
				let body = self.closure_value(body);
				match self.model.unique(&item) {
					true => format!("_each({v}) {{ {x} in {body} }}.joined(separator: \"{}\")", escape(sep)),
					false => format!("{}.map {{ {x} in {body} }}.joined(separator: \"{}\")", postfix(&v), escape(sep)),
				}
			}
			Kind::MapOr { option, none, pat, some } => {
				let option = self.expr(option, out);
				let none = self.expr(none, out);
				let x = self.param(pat);
				let some = self.closure_value(some);
				format!("({}.map {{ {x} in {some} }} ?? {none})", postfix(&option))
			}
			Kind::Utf8(inner) => format!("String(decoding: {}, as: UTF8.self)", self.expr(inner, out)),
			Kind::Len(inner) => {
				let v = self.expr(inner, out);
				match unref(&inner.ty).base() {
					Type::String | Type::Str => format!("{}.utf8.count", postfix(&v)),
					_ => format!("{}.count", postfix(&v)),
				}
			}
		}
	}

	fn expr(&mut self, e: &Expr, out: &mut String) -> String {
		self.value(e, &e.ty, out)
	}

	/// A local, read: unwrapped if Swift holds it optional, or the element a tuple pattern
	/// names.
	fn var(&self, name: &str) -> String {
		if let Some(alias) = self.aliases.get(name) {
			return alias.clone();
		}
		match self.optional.contains(name) {
			true => format!("{}!", kw(name)),
			false => kw(name),
		}
	}

	/// The call `inner` makes, which throws its error, for a closure that catches it.
	fn attempt(&mut self, inner: &Expr) -> String {
		let mut pre = String::new();
		let call = self.rooted("try ", &mut pre, |this, pre| this.expr(inner, pre));
		assert!(pre.is_empty(), "a failing call needing statements first");
		call
	}

	/// Arguments labelled as `labels` say, each passed where its type is taken.
	fn labelled(&mut self, values: &[Expr], types: &[Type], labels: &[Option<String>], out: &mut String) -> String {
		let args: Vec<String> = (values.iter().zip(types).zip(labels))
			.map(|((v, ty), label)| {
				let v = self.value(v, ty, out);
				match label {
					Some(label) => format!("{}: {v}", unquote(label)),
					None => v,
				}
			})
			.collect();
		args.join(", ")
	}

	fn tuple(&mut self, e: &Expr, elements: &[Expr], want: &Type, out: &mut String) -> String {
		match unref(&e.ty).base() {
			Type::Range(_) | Type::Bounds(_) => {
				let name = self.sw(unref(&e.ty).base());
				let (a, b) = (self.expr(&elements[0], out), self.expr(&elements[1], out));
				format!("{name}(start: {a}, end: {b})")
			}
			_ => {
				let wants: Vec<Type> = match unref(want).base() {
					Type::Tuple(ts) => ts.clone(),
					_ => elements.iter().map(|e| e.ty.clone()).collect(),
				};
				let parts: Vec<String> = elements.iter().zip(&wants).map(|(e, t)| self.value(e, t, out)).collect();
				match parts.len() {
					1 => parts.into_iter().next().unwrap(),
					_ if self.model.unique(&Type::Tuple(wants.clone())) => {
						format!("{}({})", self.tuple_name(&wants), parts.join(", "))
					}
					_ => format!("({})", parts.join(", ")),
				}
			}
		}
	}

	fn list(&mut self, items: &[Expr], want: &Type, out: &mut String) -> String {
		let item = match unref(want).base() {
			Type::List(_, item) => (**item).clone(),
			ty => panic!("a list as {ty:?}"),
		};
		if !self.model.unique(&item) {
			let parts: Vec<String> = items.iter().map(|e| self.value(e, &item, out)).collect();
			return format!("[{}]", parts.join(", "));
		}
		let sw = self.sw(&item);
		if items.is_empty() {
			return format!("UniqueArray<{sw}>()");
		}
		// A `UniqueArray` has no literal: each item is appended, in order.
		let list = self.fresh("a");
		self.line(out, &format!("var {list} = UniqueArray<{sw}>()"));
		for e in items {
			let mut pre = String::new();
			let v = self.rooted("try! ", &mut pre, |this, pre| this.value(e, &item, pre));
			out.push_str(&pre);
			self.line(out, &format!("{list}.append({v})"));
		}
		list
	}

	/// A closure's value, as its own root.
	fn closure_value(&mut self, value: &Expr) -> String {
		let mut pre = String::new();
		let v = self.root(value, &value.ty, &mut pre);
		assert!(pre.is_empty(), "a closure's value needing statements first");
		v
	}

	fn closure(&mut self, params: &[Pat], steps: &[Step], value: &Expr) -> String {
		let params: Vec<String> = params.iter().map(|p| self.param(p)).collect();
		let head = match params.is_empty() {
			true => String::new(),
			false => format!(" {} in", params.join(", ")),
		};
		self.depth += 1;
		let mut body = self.steps(steps);
		if !matches!(value.kind, Kind::Unit) {
			let v = self.root(value, &value.ty, &mut body);
			match body.is_empty() {
				true => {
					self.depth -= 1;
					return format!("{{{head} {v} }}");
				}
				false => self.line(&mut body, &format!("return {v}")),
			}
		}
		self.depth -= 1;
		format!("{{{head}\n{body}{}}}", self.indent())
	}

	/// A call of `id`: a function of the module, an associated function of a type, a method,
	/// or a field's getter or setter.
	fn call(&mut self, id: FnId, receiver: Option<&Expr>, args: &[Expr], out: &mut String) -> String {
		let at_root = std::mem::take(&mut self.at_root);
		let f = self.t.function(id);
		let key = self.t.key(id);
		assert!(!f.future, "a call of the future {key}, which only a hook awaits");
		// A fresh implementation of a callback, passed to a call within an expression: the call
		// is its own statement, so Swift's reference to it goes before the rest runs.
		let fresh = args.iter().zip(&f.params).any(|(a, p)| matches!(p.ty.base(), Type::Callback(..)) && matches!(a.kind, Kind::Local(..)));
		if fresh && !at_root {
			let call = self.rooted("try! ", out, |this, out| {
				this.at_root = true;
				this.call(id, receiver, args, out)
			});
			let tmp = self.fresh("h");
			self.line(out, &format!("let {tmp} = {call}"));
			return tmp;
		}
		self.threw |= self.model.throws(self.t, id);
		let omitted: Vec<usize> = (args.iter().zip(&f.params).enumerate())
			.filter(|(_, (a, p))| self.model.lent_option(&p.ty) && matches!(a.kind, Kind::None))
			.map(|(k, _)| k)
			.collect();
		let omitted_list: Vec<String> = omitted.iter().map(usize::to_string).collect();
		let name = self
			.t
			.named(LANGUAGE, &format!("{key} / omitted {}", omitted_list.join(",")))
			.to_string();
		let target = match receiver {
			Some(r) => {
				let v = self.expr(r, out);
				let lvalue = matches!(r.kind, Kind::Var(_)) && !self.aliases.contains_key(var_name(r));
				match self.model.mutating(self.t, id) && !lvalue {
					// A mutating method of a value no local holds: one holds it for the call.
					true => {
						let tmp = self.fresh("m");
						let sw = self.sw(&r.ty);
						self.line(out, &format!("var {tmp}: {sw} = {v}"));
						tmp
					}
					false => postfix(&v),
				}
			}
			None => match self.t.owner(id) {
				Some(ty) => self.type_name(&ty),
				None => "Api".to_string(),
			},
		};
		let labels: Vec<String> = f.params.iter().map(|p| camel(&p.name)).collect();
		let params: Vec<_> = f.params.iter().map(|p| p.ty.clone()).collect();
		let mut parts = Vec::new();
		for (k, (arg, ty)) in args.iter().zip(&params).enumerate() {
			if omitted.contains(&k) {
				continue;
			}
			if self.model.lent_option(ty) {
				let (Type::Option(inner), Kind::Some(value)) = (ty, &arg.kind) else {
					panic!("an optional borrow passed for {key} other than Some or None")
				};
				let v = self.value(value, inner, out);
				parts.push(format!("{}: {v}", labels[k]));
			} else if self.model.inout(ty) {
				parts.push(self.inout(arg, ty, out));
			} else {
				parts.push(self.value(arg, ty, out));
			}
		}
		let property = self.model.property(self.t, id);
		match f.access {
			Access::Field { set: false, .. } | Access::Const if property => format!("{target}.{name}"),
			_ => format!("{target}.{name}({})", parts.join(", ")),
		}
	}

	/// An argument passed `inout`: a local, or a nil one holds.
	fn inout(&mut self, arg: &Expr, ty: &Type, out: &mut String) -> String {
		match &arg.kind {
			Kind::Var(name) => format!("&{}", kw(name)),
			Kind::Some(inner) => match &inner.kind {
				Kind::Var(name) => format!("&{}", kw(name)),
				_ => panic!("an optional &mut of anything but a local"),
			},
			Kind::None => {
				let tmp = self.fresh("n");
				let sw = self.sw(ty);
				self.line(out, &format!("var {tmp}: {sw} = nil"));
				format!("&{tmp}")
			}
			_ => panic!("a &mut of anything but a local"),
		}
	}
}

/// The local `e` reads.
fn var_name(e: &Expr) -> &str {
	match &e.kind {
		Kind::Var(name) => name,
		_ => "",
	}
}

/// Whether `e`'s Swift is a literal whose type Swift would infer apart from Rust's, so its
/// local is annotated.
fn literalish(e: &Expr) -> bool {
	match &e.kind {
		Kind::Int(_) | Kind::Float(_) | Kind::None | Kind::Some(_) | Kind::List(_) | Kind::Str(_) => true,
		Kind::Tuple(elements) => elements.iter().any(literalish),
		_ => false,
	}
}

/// Whether Swift's value of `ty` is optional: an `Option`, or a tuple of one of one.
fn optional(ty: &Type) -> bool {
	match unref(ty).base() {
		Type::Option(_) => true,
		Type::Tuple(elements) if elements.len() == 1 => optional(&elements[0]),
		_ => false,
	}
}

/// The type a pattern binds, as its names' types say.
fn pat_type(pat: &Pat) -> Type {
	match pat {
		Pat::Name(_, ty) | Pat::Ignore(ty) => ty.clone(),
		Pat::Tuple(pats) => Type::Tuple(pats.iter().map(pat_type).collect()),
	}
}

/// `ty` without its references, as a key names it.
fn owned(ty: &Type) -> Type {
	match ty {
		Type::Ref(_, inner) => owned(inner),
		Type::Tuple(elements) => Type::Tuple(elements.iter().map(owned).collect()),
		Type::Option(inner) => Type::Option(Box::new(owned(inner))),
		Type::Box(inner) => Type::Box(Box::new(owned(inner))),
		Type::List(c, inner) => Type::List(*c, Box::new(owned(inner))),
		ty => ty.clone(),
	}
}

/// `v` as the receiver of a member: parenthesized unless a postfix expression already.
fn postfix(v: &str) -> String {
	let mut depth = 0i32;
	let mut quoted = false;
	let mut escaped = false;
	for c in v.chars() {
		match c {
			_ if escaped => escaped = false,
			'\\' if quoted => escaped = true,
			'"' => quoted = !quoted,
			'(' | '[' | '{' if !quoted => depth += 1,
			')' | ']' | '}' if !quoted => depth -= 1,
			' ' if !quoted && depth == 0 => return format!("({v})"),
			_ => {}
		}
	}
	match v.starts_with(['&', '-']) {
		true => format!("({v})"),
		false => v.to_string(),
	}
}

/// Each expression a step holds, its arms' steps included.
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

/// `f` on `expr` and each expression inside it, but not a closure's steps.
fn walk(expr: &Expr, f: &mut dyn FnMut(&Expr)) {
	f(expr);
	let mut inner = |e: &Expr| walk(e, f);
	match &expr.kind {
		Kind::Some(e) | Kind::Field(e, _) | Kind::Element(e, _) | Kind::Clone(e) | Kind::Unwrap(e) | Kind::UnwrapErr(e) => inner(e),
		Kind::IsSome(e) | Kind::IsNone(e) | Kind::IsErr(e) | Kind::Utf8(e) | Kind::Len(e) => inner(e),
		Kind::Tuple(es) | Kind::List(es) | Kind::Record(_, es) | Kind::Variant(_, _, es) | Kind::Local(_, es) => es.iter().for_each(inner),
		Kind::Call(_, r, args) => r.iter().map(|b| &**b).chain(args).for_each(inner),
		Kind::Binary(_, a, b) => {
			inner(a);
			inner(b);
		}
		Kind::Bound(_, Some(e)) => inner(e),
		Kind::Closure(_, _, value) => walk(value, f),
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

/// The Swift integer type of `int`.
fn sw_int(int: Int) -> &'static str {
	match int {
		Int::U8 => "UInt8",
		Int::U16 => "UInt16",
		Int::U32 => "UInt32",
		Int::U64 => "UInt64",
		Int::Usize => "UInt",
		Int::I8 => "Int8",
		Int::I16 => "Int16",
		Int::I32 => "Int32",
		Int::I64 => "Int64",
		Int::Isize => "Int",
	}
}

/// An integer's name in a type's: `U64`.
fn int_name(int: Int) -> String {
	let rust = int.rust();
	let mut chars = rust.chars();
	let first = chars.next().unwrap().to_ascii_uppercase();
	format!("{first}{}", chars.as_str())
}

/// A Rust name split at its case changes, in snake case: `NotFound` is `not_found`.
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

/// A snake-case name in camel case: `into_label` is `intoLabel`, and `0` is `_0`.
fn camel(name: &str) -> String {
	if name.starts_with(|c: char| c.is_ascii_digit()) {
		return format!("_{name}");
	}
	let mut out = String::new();
	for (i, word) in snake(name).split('_').filter(|s| !s.is_empty()).enumerate() {
		let mut chars = word.chars();
		match (i, chars.next()) {
			(0, Some(first)) => out.extend(first.to_lowercase().chain(chars)),
			(_, Some(first)) => out.extend(first.to_uppercase().chain(chars)),
			_ => {}
		}
	}
	out
}

/// A member's Swift name: in camel case, quoted if a keyword, and `init` taking `_`.
fn member(snake: &str) -> String {
	let name = camel(snake);
	match name.as_str() {
		"init" => "init_".to_string(),
		name => kw(name),
	}
}

/// A field's Swift name.
fn field_name(name: &str) -> String {
	member(name)
}

/// A name without the backticks quoting a keyword.
fn unquote(name: &str) -> String {
	name.trim_matches('`').to_string()
}

fn ipv4(ip: std::net::Ipv4Addr) -> String {
	let [a, b, c, d] = ip.octets();
	format!("Ipv4Addr({a}, {b}, {c}, {d})")
}

fn ipv6(ip: std::net::Ipv6Addr) -> String {
	let bits = u128::from(ip);
	format!("Ipv6Addr(bits: U128(high: {:#x}, low: {:#x}))", (bits >> 64) as u64, bits as u64)
}

/// Bytes as a Swift array literal.
fn byte_list(b: &[u8]) -> String {
	let items: Vec<String> = b.iter().map(|b| format!("{b:#04x}")).collect();
	format!("[{}] as [UInt8]", items.join(", "))
}

/// Text as the inside of a Swift string literal.
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
			c if (c as u32) < 0x20 || c as u32 == 0x7f => {
				let _ = write!(out, "\\u{{{:x}}}", c as u32);
			}
			c => out.push(c),
		}
	}
	out
}
