//! The C++ header: header-only classes over the C header's declarations, from C++17, with
//! exceptions and RTTI on or off.
//!
//! Each name is its C name without the prefix, in a namespace of the prefix, escaped as a
//! name is: `moq_track` is `moq::track`, and a method is its C name without its type's (`moq_track_close` is
//! `moq::track::close`). C names are unique, so these are too, once a keyword, or a member
//! named as its class, a field, or a trait's member, takes a trailing `_`, a name C++
//! reserves drops its `__`s (`__all__` is `all_`), and one an escape makes another's takes
//! `2` (`delete` beside `delete_` is `delete_2`), since `__` is reserved; so does a parameter
//! named as a member or as anything in the namespace, which it would hide. An object is a class owning
//! its handle through a `std::unique_ptr`, so it moves, or where `Clone` makes a value no
//! call tells apart but by an `Arc`'s or `Rc`'s count, a `std::shared_ptr` its copies
//! share; a record a struct, and a `Copy` type with private fields a class over its C
//! struct's bytes, which only Rust makes or reads; an enum an
//! `enum class`, or for one with data a `std::variant` of a struct per variant, with its
//! methods free functions; a closure, a future, or a reader C++ implements a template
//! parameter. The runtime they share, [`runtime/rt.hpp`](../runtime/rt.hpp), holds the
//! handles, bytes, results, wakers, futures, and readers.
//!
//! The items are split into a header per crate, each including the shared `rt.hpp`, which
//! holds the runtime and declares every type, and the headers of the crates its items name,
//! all before it in dependency order: a class declares each of its methods, and the header
//! of the crate the method belongs to, which an instance makes a later one, defines it.
//! `<prefix>.hpp` includes them all.

use std::collections::{BTreeMap, BTreeSet};

use crate::abi::{self, Abi, Ty};
use crate::c::decl;
use crate::doc;
use crate::ir::{Closure, Family, Function, Int, Literal, Order, Receiver, Traits, Type};

const RUNTIME: &str = include_str!("../runtime/rt.hpp");

/// The generated C++ headers and their public names.
pub(crate) struct Output {
	/// Each header by its path under `include/`.
	pub(crate) files: Vec<(String, String)>,
	pub(crate) names: BTreeMap<String, String>,
}

/// A comparison operator: its signature, its body, and whether it is C++20's alone.
struct Operator {
	head: String,
	body: String,
	cpp20: bool,
}

impl Operator {
	/// `text` inside the guard of a C++20 operator.
	fn guarded(&self, text: String) -> String {
		match self.cpp20 {
			true => format!("#if defined(__cpp_lib_three_way_comparison)\n{text}\n#endif"),
			false => text,
		}
	}

	/// Its definition, inline as a friend defined in its class is.
	fn define(&self, prefix: &str) -> String {
		self.guarded(format!("{prefix}{} {{\n{}}}", self.head, self.body))
	}
}

/// The headers for `abi`: the umbrella `<prefix>.hpp`, the shared `<prefix>/rt.hpp`, and a
/// `<prefix>/<package>.hpp` per crate; or the members two items name alike.
pub(crate) fn headers(abi: &Abi<'_>, packages: &str) -> Result<Output, String> {
	let p = &abi.prefix;
	let mut h = Header {
		locked: BTreeMap::new(),
		abi,
		// Escaped as a name is, since a library may be named as a keyword or a macro: `class`.
		ns: escape(p),
		krate: String::new(),
		files: crate::go::packages(abi),
		out: String::new(),
		defs: String::new(),
		hashes: String::new(),
		defined: BTreeSet::new(),
		globals: runtime_names().map(str::to_string).collect(),
		names: BTreeMap::new(),
		errors: Vec::new(),
	};
	// Every name the namespace declares, by its C name and spelled as its Rust: those of C's
	// own, and each constant's, whose C name is in capitals, so a constant `a_b` and a
	// function `a::b` stay two.
	let declared: BTreeMap<String, String> = (abi.declared().into_iter())
		.filter(|c| c.strip_prefix(p.as_str()).is_some_and(|rest| rest.starts_with('_')))
		.map(|c| (c.to_string(), c[p.len() + 1..].to_string()))
		.chain(abi.api.constants.iter().map(|constant| {
			(
				abi.constant(&constant.path),
				abi.name(&constant.path)[p.len() + 1..].to_string(),
			)
		}))
		.collect();
	let rust: Vec<String> = declared.values().cloned().collect();
	let runtime: BTreeSet<String> = (runtime_names().map(str::to_string))
		.chain(fixed_aliases(abi).into_iter().map(|(alias, _)| alias))
		.collect();
	h.names = declared
		.into_keys()
		.zip(named(&rust, |n| runtime.contains(n)))
		.collect();
	// A C++ class is named by its C name alone.
	let api = abi.api;
	let classes = (0..api.records.len())
		.map(|i| abi.record(i))
		.chain((0..api.enums.len()).map(|i| abi.enumeration(i)))
		.chain((0..api.objects.len()).map(|i| abi.object(i)))
		.chain((0..api.callbacks.len()).map(|i| abi.callback(i)))
		.map(|c| {
			let name = h.name(&c);
			(c, name)
		})
		.collect();
	h.locked = crate::lock::types(abi, &classes);
	// Each name a trait or a name escaping alike can move, so a change that moves one, such
	// as a reorder or a derived `Debug` beside a field `debug`, breaks the lock.
	for (i, record) in api.records.iter().enumerate() {
		for (field, name) in record.fields.iter().zip(h.fields(i).0) {
			crate::lock::field(&mut h.locked, &record.shown, &field.name, &name);
		}
	}
	for (i, e) in api.enums.iter().enumerate() {
		if e.plain() {
			for (v, value) in e.variants.iter().zip(h.values(i)) {
				crate::lock::variant(&mut h.locked, &e.shown, &v.name, &value);
			}
		}
		for v in &e.variants {
			for (field, name) in v.fields.iter().zip(field_names(&v.fields)) {
				crate::lock::variant_field(&mut h.locked, &e.shown, &v.name, &field.name, &name);
			}
		}
	}
	let mut headers = vec![(format!("{p}/rt.hpp"), h.shared(packages))];
	let guard = format!("{}_HPP", p.to_uppercase());
	let mut umbrella = format!(
		"// Generated by rust2-ffi: C++ over the C ABI of the pub API of {packages}, every crate's\n// header. Do not edit.\n\n#ifndef {guard}\n#define {guard}\n\n#include \"{p}/rt.hpp\"\n"
	);
	for krate in &abi.api.crates {
		let name = h.files[krate].clone();
		headers.push((format!("{p}/{name}.hpp"), h.krate(krate, packages)));
		umbrella.push_str(&format!("#include \"{p}/{name}.hpp\"\n"));
	}
	umbrella.push_str(&format!("\n#endif // {guard}\n"));
	headers.insert(0, (format!("{p}.hpp"), umbrella));
	// Each link once every name is final, and the headers' own uses of what they deprecate
	// kept from warning, which a consumer's still do.
	let deprecates = headers.iter().any(|(_, text)| text.contains("[[deprecated"));
	for (_, text) in &mut headers {
		let render = |link: &doc::Link<'_>| {
			let target = h.link(link.target)?;
			Some(match link.text {
				Some(text) => format!("@ref {} \"{text}\"", target.trim_end_matches("()")),
				None => target,
			})
		};
		*text = doc::links(text, api, render);
		if deprecates {
			*text = quiet(text);
		}
	}
	match h.errors.is_empty() {
		true => Ok(Output {
			files: headers,
			names: h.locked,
		}),
		false => Err(h.errors.join("\n")),
	}
}

/// A method its class declares, rather than a free function: an enum's methods take it first.
#[derive(Clone, Copy)]
struct Member<'t> {
	/// Its name in the class.
	name: &'t str,
	/// The class's other names, which a parameter named alike would hide.
	taken: &'t [String],
}

struct Header<'a> {
	locked: BTreeMap<String, String>,
	abi: &'a Abi<'a>,
	ns: String,
	/// The crate whose header this writes, or none for the shared one.
	krate: String,
	/// Each crate's header's name, by crate.
	files: BTreeMap<String, String>,
	/// The declarations, in the order C++ needs them.
	out: String,
	/// The definitions, after every type is complete.
	defs: String,
	/// `std::hash` of each class Rust hashes, after the namespace.
	hashes: String,
	/// The value types written so far, so each is written after what it holds.
	defined: BTreeSet<String>,
	/// The names declared in the namespace, which a parameter named alike would hide.
	globals: BTreeSet<String>,
	/// The C++ name of each C name the API declares, kept apart from each other's and the
	/// runtime's, as escaping may make two alike (`__all__` and `all_`).
	names: BTreeMap<String, String>,
	errors: Vec<String>,
}

impl<'a> Header<'a> {
	fn line(&mut self, line: impl AsRef<str>) {
		self.out.push_str(line.as_ref());
		self.out.push('\n');
	}

	fn def(&mut self, line: impl AsRef<str>) {
		self.defs.push_str(line.as_ref());
		self.defs.push('\n');
	}

	/// A C name without the prefix, as C++ names it.
	fn name(&self, c: &str) -> String {
		match self.names.get(c) {
			Some(name) => name.clone(),
			None => escape(&c[self.abi.prefix.len() + 1..]),
		}
	}

	/// A C name's C++ name, qualified.
	fn q(&self, c: &str) -> String {
		format!("::{}::{}", self.ns, self.name(c))
	}

	/// The namespace's `detail`, qualified.
	fn d(&self) -> String {
		format!("::{}::detail", self.ns)
	}

	/// The names of `==`'s two arguments, apart from every name in the namespace, which
	/// they would hide.
	fn operands(&self) -> (String, String) {
		let a = parameter("a", |n| self.globals.contains(n));
		let b = parameter("b", |n| self.globals.contains(n) || n == a);
		(a, b)
	}

	/// The comparison operators of a type ordered by `ord`, each taking `params`, whose
	/// body runs the first of what `call` gives of a C suffix and returns through the
	/// second, the C call: `<` and its kin from `partial_cmp`, as Rust's are, and from
	/// C++20 `<=>`, from `cmp` with `Ord` and else from `partial_cmp`.
	fn orders(&self, ord: Order, params: &str, call: &dyn Fn(&str) -> (String, String)) -> Vec<Operator> {
		let d = self.d();
		let (pre, partial) = call("partial_cmp");
		let mut ops: Vec<Operator> = [
			("<", "less"),
			("<=", "less_equal"),
			(">", "greater"),
			(">=", "greater_equal"),
		]
		.into_iter()
		.map(|(op, test)| Operator {
			head: format!("bool operator{op}({params})"),
			body: format!("{pre}\treturn {d}::{test}({partial});\n"),
			cpp20: false,
		})
		.collect();
		let (ordering, suffix) = match ord {
			Order::Total => ("weak_ordering", "cmp"),
			_ => ("partial_ordering", "partial_cmp"),
		};
		let (pre, compare) = call(suffix);
		ops.push(Operator {
			head: format!("::std::{ordering} operator<=>({params})"),
			body: format!("{pre}\treturn {d}::{ordering}({compare});\n"),
			cpp20: true,
		});
		ops
	}

	/// `name`, declared in the namespace.
	fn global(&mut self, name: &str) -> String {
		self.globals.insert(name.to_string());
		name.to_string()
	}

	/// The C++ type of `ty` as a value: a field, or what a function returns, owning what
	/// it holds.
	fn owned(&self, ty: &Type) -> String {
		let abi = self.abi;
		match ty {
			Type::Const(_) => unreachable!("a const argument is no value"),
			Type::Jni(_) => unreachable!("only Kotlin exports what takes a JNI value"),
			Type::Array(n) => format!("::std::array<::std::uint8_t, {n}>"),
			Type::Unit => "void".to_string(),
			Type::Bool => "bool".to_string(),
			Type::Int(int) | Type::NonZero(int) => cpp_int(*int).to_string(),
			Type::F32 => "float".to_string(),
			Type::F64 => "double".to_string(),
			Type::Char => "char32_t".to_string(),
			Type::Str | Type::String | Type::Slice | Type::Vec | Type::Bytes | Type::Path | Type::PathBuf => {
				format!("::{}::buf", self.ns)
			}
			Type::U128 => format!("::{}::u128", self.ns),
			Type::Duration => format!("::{}::duration", self.ns),
			Type::SocketAddr(family) => format!("::{}::{}", self.ns, crate::abi::addr(true, *family)),
			Type::IpAddr(family) => format!("::{}::{}", self.ns, crate::abi::addr(false, *family)),
			Type::List(_, inner) => format!("::std::vector<{}>", self.owned(inner)),
			Type::Waker => format!("::{}::waker", self.ns),
			Type::Option(inner) => format!("::std::optional<{}>", self.owned(inner)),
			Type::Box(inner) | Type::Ref(_, inner) => self.owned(inner),
			Type::Record(i) => self.q(&abi.record(*i)),
			Type::Enum(i) => self.q(&abi.enumeration(*i)),
			Type::Object(i) => self.q(&abi.object(*i)),
			Type::Callback(i, _) => format!("::std::unique_ptr<{}>", self.q(&abi.callback(*i))),
			Type::Closure(..) | Type::Future | Type::Reader => unreachable!("{ty:?} is a template parameter"),
			Type::Range(_) | Type::Bound(_) | Type::Bounds(_) => self.q(&abi.range(ty)),
			Type::Tuple(elements) => {
				let elements: Vec<String> = elements.iter().map(|e| self.owned(e)).collect();
				format!("::std::tuple<{}>", elements.join(", "))
			}
		}
	}

	/// The C++ type of a parameter of type `ty`.
	fn param(&self, ty: &Type) -> String {
		let abi = self.abi;
		// A value Rust writes back, through the caller's own.
		if let Type::Ref(_, inner) = ty
			&& ty.written()
		{
			return format!("{} &", self.owned(inner));
		}
		// Passed by value, so a `buf` Rust made moves back without a copy.
		if Abi::is_buf(ty) {
			return format!("::{}::input", self.ns);
		}
		match ty {
			Type::Option(inner) if Abi::is_buf(inner) => format!("::std::optional<::{}::input>", self.ns),
			// Numbers lent in place, which Rust copies if it keeps them.
			Type::Option(inner) if inner.base().block().is_some() => {
				format!("::std::optional<{}>", self.slice(inner.base()))
			}
			Type::List(..) if ty.block().is_some() => self.slice(ty),
			// Moved out of, each handle as a handle taken by value is.
			Type::Option(inner) if matches!(inner.base(), Type::List(_, elem) if moves(abi, elem)) => {
				format!("::std::optional<{}>", self.items(inner.base()))
			}
			Type::Option(inner) if matches!(inner.base(), Type::List(..)) => {
				format!("const ::std::optional<{}> &", self.items(inner.base()))
			}
			// Moved out of, each handle as a handle taken by value is.
			Type::List(_, inner) if moves(abi, inner) => self.items(ty),
			Type::List(..) => format!("const {} &", self.items(ty)),
			Type::Option(inner) => match &**inner {
				// Lent, or copied out: a pointer, null for none.
				Type::Ref(m, object) if matches!(**object, Type::Object(_)) => {
					let constant = match m {
						crate::ir::Mutability::Not => "const ",
						crate::ir::Mutability::Mut => "",
					};
					format!("{constant}{} *", self.owned(object))
				}
				Type::Object(i) if abi.api.objects[*i].copy => format!("const {} *", self.owned(inner)),
				// Moved out of, as a handle taken by value is.
				_ if moves(abi, inner) => format!("::std::optional<{}>", self.held(inner)),
				_ => format!("const ::std::optional<{}> &", self.held(inner)),
			},
			Type::Waker => format!("const ::{}::waker &", self.ns),
			Type::Ref(m, inner) if matches!(**inner, Type::Object(_)) => match m {
				crate::ir::Mutability::Not => format!("const {} &", self.owned(inner)),
				crate::ir::Mutability::Mut => format!("{} &", self.owned(inner)),
			},
			Type::Ref(_, inner) | Type::Box(inner) => self.param(inner),
			Type::Object(i) if abi.api.objects[*i].copy => format!("const {} &", self.owned(ty)),
			Type::Record(_) | Type::Array(_) | Type::Range(_) | Type::Bound(_) | Type::Bounds(_) => {
				format!("const {} &", self.owned(ty))
			}
			Type::Enum(i) if !abi.api.enums[*i].plain() => format!("const {} &", self.owned(ty)),
			// Moved out of, each handle as a handle taken by value is.
			Type::Tuple(_) if moves(abi, ty) => self.held(ty),
			Type::Tuple(_) => format!("const {} &", self.held(ty)),
			ty => self.owned(ty),
		}
	}

	/// The C++ type a `Vec` parameter of type `ty` takes: a vector, or, when its items hold
	/// views, the runtime's `items` lent or `taken` when they move handles, each of which
	/// also takes a returned vector of `buf`s as it is.
	fn items(&self, ty: &Type) -> String {
		let Type::List(_, inner) = ty else {
			unreachable!("{ty:?} is no Vec")
		};
		let held = self.held(inner);
		match (held == self.owned(inner), moves(self.abi, inner)) {
			(true, _) => format!("::std::vector<{held}>"),
			(false, true) => format!("::{}::taken<{held}>", self.ns),
			(false, false) => format!("::{}::items<{held}>", self.ns),
		}
	}

	/// The `slice` a parameter of the block `ty` takes.
	fn slice(&self, ty: &Type) -> String {
		let elem = ty.block().expect("a block");
		format!("::{}::slice<{}>", self.ns, self.owned(elem))
	}

	/// The C++ type a parameter's `Vec`, `Option`, or tuple holds a `ty` as: bytes as a view.
	fn held(&self, ty: &Type) -> String {
		match ty {
			ty if Abi::is_buf(ty) => format!("::{}::view", self.ns),
			Type::Tuple(elements) => {
				let elements: Vec<String> = elements.iter().map(|e| self.held(e)).collect();
				format!("::std::tuple<{}>", elements.join(", "))
			}
			ty => self.owned(ty),
		}
	}

	/// Whether lowering a `ty` lowers a `Vec`, whose array lives in the arena `a_` until
	/// the call returns.
	fn arena(&self, ty: &Type) -> bool {
		let api = self.abi.api;
		match ty {
			// A block crosses in place.
			Type::List(..) if ty.block().is_some() => false,
			Type::List(..) => true,
			Type::Option(inner) | Type::Box(inner) | Type::Ref(_, inner) => self.arena(inner),
			Type::Tuple(elements) => elements.iter().any(|e| self.arena(e)),
			Type::Record(i) => api.records[*i].fields.iter().any(|f| self.arena(&f.ty)),
			Type::Enum(i) => (api.enums[*i].variants.iter()).any(|v| v.fields.iter().any(|f| self.arena(&f.ty))),
			_ => false,
		}
	}

	/// The arena a value's `lower` takes, if it lowers a `Vec`.
	fn arena_param(&self, ty: &Type) -> String {
		match self.arena(ty) {
			true => format!(", {}::arena &a_", self.d()),
			false => String::new(),
		}
	}

	/// The status a record's or enum's trait function takes, if a handle in it is checked:
	/// null, so a failed check aborts, as each other call of C++'s does.
	fn value_status(&self, ty: &Type) -> &'static str {
		match self.abi.value_status(ty) {
			true => ", nullptr",
			false => "",
		}
	}

	/// The arena a body lowering each of `tys` declares, if one needs it.
	fn scope<'t>(&self, tys: impl IntoIterator<Item = &'t Type>) -> String {
		match tys.into_iter().any(|ty| self.arena(ty)) {
			true => format!(
				"	{}::arena a_;
",
				self.d()
			),
			false => String::new(),
		}
	}

	/// Whether [`Self::arg`] takes Rust's bytes from a parameter of type `ty`.
	fn takes(ty: &Type) -> bool {
		match ty {
			_ if Abi::is_buf(ty) => true,
			Type::Option(inner) => Abi::is_buf(inner),
			Type::Ref(_, inner) | Type::Box(inner) => matches!(inner.base(), Type::Option(o) if Abi::is_buf(o)),
			_ => false,
		}
	}

	/// The C value of parameter `e` of type `ty`: a buffer's `input`, or a value passed for
	/// `&mut`, hands Rust's bytes over.
	fn arg(&self, ty: &Type, e: &str) -> String {
		let d = self.d();
		match ty {
			_ if Abi::is_buf(ty) => format!("{d}::access::take({e})"),
			Type::Option(inner) if Abi::is_buf(inner) => format!("({e} ? {d}::access::take(*{e}) : {d}::none())"),
			Type::Ref(_, inner) | Type::Box(inner) if matches!(inner.base(), Type::Option(o) if Abi::is_buf(o)) => {
				self.arg(inner.base(), e)
			}
			_ => self.lower(ty, e),
		}
	}

	/// The C value of the C++ value `e` of type `ty`, as a parameter or a field passes it.
	fn lower(&self, ty: &Type, e: &str) -> String {
		let abi = self.abi;
		let d = self.d();
		let ns = &self.ns;
		if Abi::is_buf(ty) {
			return format!("{d}::access::raw(::{ns}::view({e}))");
		}
		match ty {
			Type::Const(_) => unreachable!("a const argument is no value"),
			Type::Jni(_) => unreachable!("only Kotlin exports what takes a JNI value"),
			Type::Array(n) => format!("{d}::lower_array<::{}>({e})", abi.array(*n)),
			Type::Unit => unreachable!("no value is unit"),
			Type::Bool | Type::Int(_) | Type::NonZero(_) | Type::F32 | Type::F64 => e.to_string(),
			Type::Char => format!("static_cast<uint32_t>({e})"),
			Type::U128
			| Type::Duration
			| Type::SocketAddr(_)
			| Type::IpAddr(_)
			| Type::Range(_)
			| Type::Bound(_)
			| Type::Bounds(_) => e.to_string(),
			Type::Waker => format!("{d}::give({d}::access::raw({e}))"),
			Type::List(_, inner) if ty.block().is_some() => format!("{d}::block<::{}>({e})", abi.list(inner)),
			Type::List(_, inner) => {
				let x = format!("x{}_", lists(inner));
				format!(
					"a_.list<::{}>({e}, [&](auto &{x}) {{ return {}; }})",
					abi.list(inner),
					self.lower(inner, &x)
				)
			}
			Type::Option(inner) => match inner.base() {
				Type::Object(_) => format!("({e} ? {} : nullptr)", self.lower(inner, &format!("*{e}"))),
				base @ Type::List(_, elem) => format!(
					"({e} ? {} : ::{}{{}})",
					self.lower(base, &format!("(*{e})")),
					abi.list(elem)
				),
				// Through a view, so an empty buffer C++ made crosses as `Some`, not as null.
				_ if Abi::is_buf(inner) => format!("({e} ? {d}::access::raw(::{ns}::view(*{e})) : {d}::none())"),
				_ => {
					let c = abi.option(inner);
					let value = self.lower(inner, &format!("(*{e})"));
					format!("({e} ? ::{c}{{true, {value}}} : ::{c}{{}})")
				}
			},
			Type::Ref(_, inner) if matches!(inner.base(), Type::Object(_)) => format!("{d}::access::raw({e})"),
			Type::Box(inner) | Type::Ref(_, inner) => self.lower(inner, e),
			Type::Record(_) | Type::Enum(_) if self.arena(ty) => format!("{d}::lower({e}, a_)"),
			Type::Record(_) => format!("{d}::lower({e})"),
			Type::Enum(i) => match abi.api.enums[*i].plain() {
				true => format!("static_cast<::{}>({e})", abi.enumeration(*i)),
				false => format!("{d}::lower({e})"),
			},
			// A `Copy` value is copied out, and any other moved out.
			Type::Object(i) if abi.api.objects[*i].copy => format!("{d}::access::raw({e})"),
			Type::Object(_) => format!("{d}::access::own({e})"),
			Type::Callback(..) => format!("{d}::lower(::std::move({e}))"),
			Type::Closure(..) | Type::Future | Type::Reader => {
				unreachable!("{ty:?} lowers through its template parameter")
			}
			Type::Str | Type::String | Type::Slice | Type::Vec | Type::Bytes | Type::Path | Type::PathBuf => {
				unreachable!("a buffer is above")
			}
			Type::Tuple(elements) => {
				let fields: Vec<String> = (0..elements.len())
					.map(|n| self.lower(&elements[n], &format!("::std::get<{n}>({e})")))
					.collect();
				format!("::{}{{{}}}", abi.tuple(elements), fields.join(", "))
			}
		}
	}

	/// The C++ value of the C value `e` of type `ty`, which it takes ownership of. `e` is
	/// named, so it may be read more than once.
	fn lift(&self, ty: &Type, e: &str) -> String {
		let abi = self.abi;
		let d = self.d();
		if Abi::is_buf(ty) {
			return format!("{d}::access::adopt({e})");
		}
		match ty {
			Type::Bool | Type::Int(_) | Type::NonZero(_) | Type::F32 | Type::F64 => e.to_string(),
			Type::Char => format!("static_cast<char32_t>({e})"),
			Type::U128
			| Type::Duration
			| Type::SocketAddr(_)
			| Type::IpAddr(_)
			| Type::Range(_)
			| Type::Bound(_)
			| Type::Bounds(_) => e.to_string(),
			Type::List(_, inner) if ty.block().is_some() => format!(
				"{d}::lift_block<{}>({e}, &::{})",
				self.owned(inner),
				abi::join(&abi.list(inner), "free")
			),
			Type::List(_, inner) => {
				let c = format!("c{}_", lists(inner));
				format!(
					"{d}::lift_list<{}>({e}, [&](const auto &{c}) {{ return {}; }}, &::{})",
					self.owned(inner),
					self.lift(inner, &c),
					abi::join(&abi.list(inner), "free")
				)
			}
			Type::Option(inner) => {
				let owned = self.owned(inner);
				match inner.base() {
					Type::Object(_) => {
						format!("({e} ? ::std::optional<{owned}>({d}::access::wrap<{owned}>({e})) : ::std::nullopt)")
					}
					Type::List(..) => format!(
						"({e}.ptr ? ::std::optional<{owned}>({}) : ::std::nullopt)",
						self.lift(inner, e)
					),
					_ if Abi::is_buf(inner) => format!("{d}::access::adopt_option({e})"),
					_ => format!(
						"({e}.some ? ::std::optional<{owned}>({}) : ::std::nullopt)",
						self.lift(inner, &format!("{e}.value"))
					),
				}
			}
			Type::Box(inner) | Type::Ref(_, inner) => self.lift(inner, e),
			Type::Record(_) => format!("{d}::lift({e})"),
			Type::Enum(i) => match abi.api.enums[*i].plain() {
				true => format!("static_cast<{}>({e})", self.owned(ty)),
				false => format!("{d}::lift({e})"),
			},
			Type::Object(_) => format!("{d}::access::wrap<{}>({e})", self.owned(ty)),
			Type::Array(n) => format!("{d}::lift_array<{n}>({e})"),
			Type::Waker => format!("::{}::waker({d}::returned({e}))", self.ns),
			Type::Tuple(elements) => {
				let lifted: Vec<String> = (elements.iter().enumerate())
					.map(|(n, ty)| self.lift(ty, &format!("{e}._{n}")))
					.collect();
				format!("{}({})", self.owned(ty), lifted.join(", "))
			}
			other => unreachable!("{other:?} is never returned"),
		}
	}

	/// The C++ type of a callback's parameter of type `ty`, lent for the call.
	fn lent(&self, ty: &Type) -> String {
		match ty {
			ty if Abi::is_buf(ty) => format!("::{}::view", self.ns),
			Type::Option(inner) => format!("::std::optional<{}>", self.lent(inner)),
			ty => self.owned(ty),
		}
	}

	/// The C++ value of a callback's C parameter `e` of type `ty`, lent for the call.
	fn lift_lent(&self, ty: &Type, e: &str) -> String {
		match ty {
			ty if Abi::is_buf(ty) => format!("::{}::view({e}.ptr, {e}.len)", self.ns),
			Type::Option(inner) if Abi::is_buf(inner) => format!(
				"({e}.ptr ? ::std::optional<::{ns}::view>(::{ns}::view({e}.ptr, {e}.len)) : ::std::nullopt)",
				ns = self.ns
			),
			Type::Option(inner) => format!(
				"({e}.some ? ::std::optional<{}>({}) : ::std::nullopt)",
				self.lent(inner),
				self.lift_lent(inner, &format!("{e}.value"))
			),
			ty => self.lift(ty, e),
		}
	}

	/// The shared header: the runtime, and every type declared, so any crate's signature
	/// may name any other's, as a method belonging to a later crate's instance does.
	fn shared(&mut self, packages: &str) -> String {
		let abi = self.abi;
		let api = abi.api;
		let p = abi.prefix.clone();
		let ns = self.ns.clone();
		let guard = format!("{}_HPP_RT", p.to_uppercase());
		self.line(format!(
			"// Generated by rust2-ffi: the runtime of the C++ over the C ABI of the pub API of {packages},\n// and every type it declares. Do not edit.\n"
		));
		self.line(format!("#ifndef {guard}\n#define {guard}\n"));
		self.line(format!("#include \"../{p}.h\"\n"));
		for include in [
			"array",
			"atomic",
			"condition_variable",
			"cstddef",
			"cstdint",
			"cstdio",
			"cstdlib",
			"cstring",
			"functional",
			"initializer_list",
			"ios",
			"memory",
			"mutex",
			"optional",
			"string_view",
			"thread",
			"tuple",
			"type_traits",
			"utility",
			"variant",
			"vector",
		] {
			self.line(format!("#include <{include}>"));
		}
		self.line(
			"#if defined(__cpp_impl_coroutine) && __cpp_impl_coroutine >= 201902L && __has_include(<coroutine>)\n#include <coroutine>\n#endif\n#if defined(__cpp_impl_three_way_comparison) && __cpp_impl_three_way_comparison >= 201907L && __has_include(<compare>)\n#include <compare>\n#endif\n",
		);
		self.line(format!("namespace {ns} {{\n"));
		// Whether a call makes a value that is not `Send`, whose thread an executor drains.
		let drains = match self.abi.api.unsent() {
			true => format!(
				"inline constexpr bool drains = true;\n\ninline void c_thread_wake(c_waker wake) noexcept {{\n\t::{p}_thread_wake(wake);\n}}\n\ninline void c_thread_drain() noexcept {{\n\t::{p}_thread_drain();\n}}\n\n"
			),
			false => "inline constexpr bool drains = false;\n\ninline void c_thread_wake(c_waker) noexcept {}\n\ninline void c_thread_drain() noexcept {}\n\n".to_string(),
		};
		self.line(format!(
			"namespace detail {{\n\nusing c_buf = ::{p}_buf;\nusing c_waker = ::{p}_waker;\nusing c_foreign_future = ::{p}_foreign_future;\nusing c_foreign_reader = ::{p}_foreign_reader;\n\ninline void c_buf_free(c_buf buf) noexcept {{\n\t::{p}_buf_free(buf);\n}}\n\ninline c_buf c_buf_copy(c_buf view) noexcept {{\n\treturn ::{p}_buf_copy(view);\n}}\n\ninline c_buf c_buf_clone(c_buf buf) noexcept {{\n\treturn ::{p}_buf_clone(buf);\n}}\n\n{drains}}} // namespace detail\n"
		));
		self.out.push_str(RUNTIME);
		self.line(format!(
			"\nnamespace detail {{\n\n// Checked at load, before any other call: a header and a library from different builds\n// must not mix.\ninline const bool contract = ::{p}_contract() == {upper}_CONTRACT ||\n\t(fail(\"{p}/rt.hpp and the library are from different builds\"), false);\n\n}} // namespace detail\n",
			upper = p.to_uppercase()
		));
		let mut aliases = fixed_aliases(abi);
		aliases.extend(abi.ranges.keys().map(|name| (self.name(name), name.clone())));
		for (alias, c) in aliases {
			let alias = self.global(&alias);
			self.line(format!("using {alias} = ::{c};"));
		}
		self.line("");

		// Every type declared, so any signature may name it; a plain enum is complete at
		// once, as C++ gives one with no underlying type `int`'s.
		for i in (0..api.enums.len()).filter(|&i| api.enums[i].plain()) {
			let name = self.global(&self.name(&abi.enumeration(i)));
			self.line(format!("enum class {name};"));
		}

		// Every class and struct, declared so any signature may name it.
		for (i, record) in api.records.iter().enumerate() {
			let name = self.global(&self.name(&abi.record(i)));
			// The key its definition uses, which MSVC's C4099 and Clang's
			// `-Wmismatched-tags` require match.
			match record.opaque {
				Some(_) => self.line(format!("class {name};")),
				None => self.line(format!("struct {name};")),
			}
		}
		for i in 0..api.objects.len() {
			let name = self.global(&self.name(&abi.object(i)));
			self.line(format!("class {name};"));
		}
		for i in 0..api.callbacks.len() {
			let name = self.global(&self.name(&abi.callback(i)));
			self.line(format!("class {name};"));
		}
		for (i, e) in api.enums.iter().enumerate().filter(|(_, e)| !e.plain()) {
			let name = abi.enumeration(i);
			let payloads: Vec<String> = e
				.variants
				.iter()
				.map(|v| self.global(&self.name(&abi::payload(&name, &v.name))))
				.collect();
			for payload in &payloads {
				self.line(format!("struct {payload};"));
			}
			let alias = self.global(&self.name(&name));
			self.line(format!(
				"{}using {alias}{} = ::std::variant<{}>;",
				docs(&e.docs, ""),
				suffixed(e.deprecated.as_deref()),
				payloads.join(", ")
			));
		}
		self.line("");

		// Each future's type, by the operations it polls with, which its crate's header
		// defines.
		for (function, owner) in &self.functions() {
			let export = abi.export(function, owner.as_ref());
			if let Some(abi::Future { handle, .. }) = &export.future {
				let name = self.global(&self.name(handle));
				self.line(format!(
					"namespace detail {{\nstruct {name}_op;\n}}\nusing {name} = ::{ns}::future<detail::{name}_op>;"
				));
			}
		}
		self.line("");
		// The constants each crate's header defines, which a parameter named alike would hide.
		for constant in &api.constants {
			self.global(&self.name(&abi.constant(&constant.path)));
		}
		self.line(format!("}} // namespace {ns}\n"));
		self.line(format!("#endif // {guard}"));
		std::mem::take(&mut self.out)
	}

	/// The header of `krate`: what belongs to it, after the headers of the crates it names.
	fn krate(&mut self, krate: &str, packages: &str) -> String {
		let abi = self.abi;
		let api = abi.api;
		let p = abi.prefix.clone();
		let ns = self.ns.clone();
		self.krate = krate.to_string();
		// Named by the crate's place, not its name, which upper case could make the runtime's
		// `RT` or another crate's.
		let n = api
			.crates
			.iter()
			.position(|k| k == krate)
			.expect("each crate is listed");
		let guard = format!("{}_HPP_CRATE{n}", p.to_uppercase());
		self.line(format!(
			"// Generated by rust2-ffi: C++ over the C ABI of the pub API of the crate {krate} in\n// {packages}. Do not edit.\n"
		));
		self.line(format!("#ifndef {guard}\n#define {guard}\n"));
		self.line("#include \"rt.hpp\"");
		for named in self.includes(krate) {
			self.line(format!("#include \"{}.hpp\"", self.files[&named]));
		}
		self.line(format!("\nnamespace {ns} {{\n"));
		let home = |h: &crate::ir::Home| h.krate == krate;

		for (i, e) in api.enums.iter().enumerate().filter(|(_, e)| e.plain() && home(&e.home)) {
			let name = self.name(&abi.enumeration(i));
			let variants: Vec<String> = (e.variants.iter())
				.zip(self.values(i))
				.map(|(v, value)| format!("\t{value} = {},", v.discriminant))
				.collect();
			self.line(format!(
				"{}enum class{} {name} {{\n{}\n}};\n",
				docs(&e.docs, ""),
				suffixed(e.deprecated.as_deref()),
				variants.join("\n")
			));
		}

		let constants: Vec<_> = api.constants.iter().filter(|c| home(&c.home)).collect();
		for constant in &constants {
			let ty = match constant.value {
				Literal::Bool(_) => "bool",
				Literal::Int(int, _) => cpp_int(int),
			};
			let name = self.name(&abi.constant(&constant.path));
			self.locked
				.insert(format!("const {}", constant.path.join("::")), name.clone());
			self.line(format!(
				"{}{}inline constexpr {ty} {name} = {};",
				docs(&constant.docs, ""),
				prefixed(constant.deprecated.as_deref()),
				abi.constant(&constant.path)
			));
		}
		if !constants.is_empty() {
			self.line("");
		}

		for i in (0..api.objects.len()).filter(|&i| home(&api.objects[i].home)) {
			self.object(i);
		}
		for i in (0..api.callbacks.len()).filter(|&i| home(&api.callbacks[i].home)) {
			self.callback(i);
		}

		// Value types, each after the values it holds.
		for i in (0..api.records.len()).filter(|&i| home(&api.records[i].home)) {
			self.value(&Type::Record(i));
		}
		for (i, e) in api.enums.iter().enumerate().filter(|(_, e)| home(&e.home)) {
			match e.plain() {
				true => self.enumeration(i),
				false => self.value(&Type::Enum(i)),
			}
		}

		self.conversions();
		for (function, owner) in &self.functions() {
			if home(&function.home) {
				self.future(function, owner.as_ref());
			}
		}

		let defs = std::mem::take(&mut self.defs);
		self.out.push_str(&defs);
		// The methods of another crate's types that belong here, as an instance naming this
		// crate's types does: declared in their class, defined here.
		for (i, object) in api.objects.iter().enumerate().filter(|(_, o)| !home(&o.home)) {
			let owner = Ty::Handle {
				name: abi.object(i),
				constant: false,
			};
			let taken = self.object_taken(i);
			let names = self.methods(&object.methods, &owner, &taken);
			for (method, name) in object.methods.iter().zip(&names).filter(|(m, _)| home(&m.home)) {
				self.function(method, Some(&owner), Some(Member { name, taken: &taken }));
			}
		}
		for (i, record) in api.records.iter().enumerate().filter(|(_, r)| !home(&r.home)) {
			let owner = Ty::Struct(abi.record(i));
			let (_, taken) = self.fields(i);
			let names = self.methods(&record.methods, &owner, &taken);
			for (method, name) in record.methods.iter().zip(&names).filter(|(m, _)| home(&m.home)) {
				self.function(method, Some(&owner), Some(Member { name, taken: &taken }));
			}
		}
		for (i, e) in api.enums.iter().enumerate().filter(|(_, e)| !home(&e.home)) {
			let owner = abi.ty(&Type::Enum(i));
			for method in e.methods.iter().filter(|m| home(&m.home)) {
				self.function(method, Some(&owner), None);
			}
		}
		for function in api.functions.iter().filter(|f| home(&f.home)) {
			self.function(function, None, None);
		}
		let defs = std::mem::take(&mut self.defs);
		self.out.push_str(&defs);
		self.line(format!("}} // namespace {ns}\n"));
		let hashes = std::mem::take(&mut self.hashes);
		self.out.push_str(&hashes);
		self.line(format!("#endif // {guard}"));
		std::mem::take(&mut self.out)
	}

	/// The other crates whose types what belongs to `krate` names, each before it in
	/// dependency order, as [`crate::ir::Home`] places an instance.
	fn includes(&mut self, krate: &str) -> BTreeSet<String> {
		let api = self.abi.api;
		let mut tys: Vec<&Type> = Vec::new();
		let mut owners: Vec<Type> = Vec::new();
		for (i, r) in api.records.iter().enumerate() {
			if r.home.krate == krate {
				tys.extend(r.fields.iter().map(|f| &f.ty));
			}
			if r.methods.iter().any(|m| m.home.krate == krate) {
				owners.push(Type::Record(i));
			}
		}
		for (i, e) in api.enums.iter().enumerate() {
			if e.home.krate == krate {
				tys.extend(e.variants.iter().flat_map(|v| &v.fields).map(|f| &f.ty));
			}
			if e.methods.iter().any(|m| m.home.krate == krate) {
				owners.push(Type::Enum(i));
			}
		}
		for (i, o) in api.objects.iter().enumerate() {
			if o.methods.iter().any(|m| m.home.krate == krate) {
				owners.push(Type::Object(i));
			}
		}
		let methods = (api.records.iter().flat_map(|r| &r.methods))
			.chain(api.enums.iter().flat_map(|e| &e.methods))
			.chain(api.objects.iter().flat_map(|o| &o.methods))
			.chain(&api.functions)
			.filter(|f| f.home.krate == krate);
		let callbacks = (api.callbacks.iter())
			.filter(|c| c.home.krate == krate)
			.flat_map(|c| &c.methods);
		for function in methods.chain(callbacks) {
			tys.extend(function.params.iter().map(|p| &p.ty));
			tys.push(&function.ok);
			tys.extend(&function.err);
		}
		let rank = |k: &str| api.crates.iter().position(|c| c == k);
		let mut named = BTreeSet::new();
		for ty in tys.into_iter().chain(&owners) {
			crate::ir::each(ty, &mut |ty| {
				let home = match ty {
					Type::Record(i) => &api.records[*i].home,
					Type::Enum(i) => &api.enums[*i].home,
					Type::Object(i) => &api.objects[*i].home,
					Type::Callback(i, _) => &api.callbacks[*i].home,
					_ => return,
				};
				if home.krate != krate {
					named.insert(home.krate.clone());
				}
			});
		}
		for later in named.iter().filter(|k| rank(k) > rank(krate)) {
			self.errors.push(format!(
				"the crate {krate} names the types of {later}, a crate after it, which rust2-ffi should place with them"
			));
		}
		named
	}

	/// Every exported function, with the C type of its owner if a method.
	fn functions(&self) -> Vec<(&'a Function, Option<Ty>)> {
		let abi = self.abi;
		let api = abi.api;
		let mut functions = Vec::new();
		for (i, record) in api.records.iter().enumerate() {
			let owner = Ty::Struct(abi.record(i));
			functions.extend(record.methods.iter().map(|m| (m, Some(owner.clone()))));
		}
		for (i, e) in api.enums.iter().enumerate() {
			let owner = abi.ty(&Type::Enum(i));
			functions.extend(e.methods.iter().map(|m| (m, Some(owner.clone()))));
		}
		for (i, object) in api.objects.iter().enumerate() {
			let owner = Ty::Handle {
				name: abi.object(i),
				constant: false,
			};
			functions.extend(object.methods.iter().map(|m| (m, Some(owner.clone()))));
		}
		functions.extend(api.functions.iter().map(|f| (f, None)));
		functions
	}

	/// The member names of `methods`, of the type `owner`, each its C name without its type's,
	/// kept apart from `taken` and each other as [`named`] keeps them.
	fn methods(&self, methods: &[Function], owner: &Ty, taken: &[String]) -> Vec<String> {
		let c = match owner {
			Ty::Struct(n) | Ty::Handle { name: n, .. } => n,
			_ => unreachable!("a class's methods are a record's or an object's"),
		};
		let bare: Vec<String> = (methods.iter())
			.map(|method| {
				let call = self.abi.export(method, Some(owner)).call.name;
				match call
					.strip_prefix(crate::abi::stem(c))
					.and_then(|rest| rest.strip_prefix('_'))
				{
					Some(rest) => rest.to_string(),
					None => call[self.abi.prefix.len() + 1..].to_string(),
				}
			})
			.collect();
		named(&bare, |n| taken.iter().any(|t| t == n))
	}

	/// The record `i`'s field names, kept apart from the members its traits export; and every
	/// name its methods keep apart from: those, its class's, its traits' as an object's would
	/// be, as they always have, and an opaque value's bytes `c_`.
	fn fields(&self, i: usize) -> (Vec<String>, Vec<String>) {
		let record = &self.abi.api.records[i];
		let name = self.name(&self.abi.record(i));
		let made = Self::made(record.traits, &name);
		// A record's traits give it no `clone` or `hash`, which only an object's give.
		let emitted = Traits {
			clone: false,
			hash: false,
			..record.traits
		};
		let rust: Vec<String> = record.fields.iter().map(|f| f.name.clone()).collect();
		let members = Self::made(emitted, &name);
		let fields = named(&rust, |n| members.iter().any(|t| t == n));
		let mut taken = fields.clone();
		taken.extend(made);
		taken.push(name);
		if record.opaque.is_some() {
			taken.push("c_".to_string());
		}
		(fields, taken)
	}

	/// The names the methods of the object `i` keep apart from: its traits' members, its
	/// class's, and its handle `raw_`.
	fn object_taken(&self, i: usize) -> Vec<String> {
		let name = self.name(&self.abi.object(i));
		let mut taken = Self::made(self.abi.api.objects[i].traits, &name);
		taken.push(name);
		taken.push("raw_".to_string());
		taken
	}

	/// The values of the plain enum `i`, each its variant's name in snake case.
	fn values(&self, i: usize) -> Vec<String> {
		let rust: Vec<String> = (self.abi.api.enums[i].variants.iter())
			.map(|v| abi::snake(&v.name))
			.collect();
		named(&rust, |_| false)
	}

	/// The members `traits` exports on the class `class`, which a method's name keeps apart
	/// from: an inherent `default`, which C renames `default_` beside the trait's, is
	/// otherwise the trait's `default_` too, as C++ escapes the keyword.
	fn made(traits: Traits, class: &str) -> Vec<String> {
		let names = [
			(traits.clone, "clone"),
			(traits.hash, "hash"),
			(traits.display, "to_string"),
			(traits.debug, "debug"),
			(traits.default, "default_"),
		];
		(names.into_iter())
			.filter(|(made, _)| *made)
			.map(|(_, bare)| apart(bare, class))
			.collect()
	}

	/// Check that no two members of `class`, the Rust type `rust`, share a name.
	fn distinct(&mut self, class: &str, rust: &str, members: &[String]) {
		let mut seen = BTreeSet::new();
		for member in members {
			if !seen.insert(member) {
				self.errors.push(format!(
					"two members of `{rust}` are both `{}::{class}::{member}` in C++, which rust2-ffi should keep apart",
					self.ns
				));
			}
		}
	}

	fn object(&mut self, i: usize) {
		let abi = self.abi;
		let object = &abi.api.objects[i];
		let c = abi.object(i);
		let c_ = crate::abi::stem(&c);
		let name = self.name(&c);
		let t = object.traits;
		let text = format!("{}\n\n{}", abi.api.rust(&object.path, &object.args), object.docs);
		let d = self.d();
		// A failed check of a trait's borrow aborts with its message, as C++ throws nothing.
		let status = match abi.traits_status(i) {
			true => ", nullptr",
			false => "",
		};
		// Copies share a handle only where no call tells that apart from a clone, but by an
		// `Arc`'s or `Rc`'s count, which counts one for every copy.
		let handle = match (object.shared, status.is_empty()) {
			(true, true) => format!("{d}::shared<::{c}, &::{c_}_free, &::{c_}_clone, &::{c_}_borrowed>"),
			(true, false) => {
				self.line(format!(
					"namespace detail {{\n\ninline ::{c} *{c_}_clone(const ::{c} *raw) noexcept {{\n\treturn ::{c_}_clone(raw, nullptr);\n}}\n\n}} // namespace detail\n"
				));
				format!("{d}::shared<::{c}, &::{c_}_free, &{d}::{c_}_clone, &::{c_}_borrowed>")
			}
			(false, _) => format!("{d}::unique<::{c}, &::{c_}_free>"),
		};
		let mut body = vec![
			format!("\t{handle} raw_;\n"),
			format!("\texplicit {name}(decltype(raw_) raw) noexcept : raw_(::std::move(raw)) {{}}\n"),
			format!("\tfriend struct {d}::access;\n"),
			"public:".to_string(),
			// A move leaves the source consumed, which Clang's `-Wconsumed` then refuses to call.
			format!("\tRUST2_FFI_UNCONSUMED {name}({name} &&other RUST2_FFI_MOVED_FROM) noexcept = default;"),
			format!("\t{name} &operator=({name} &&other) noexcept = default;"),
		];
		if object.shared {
			body.push(format!("\tRUST2_FFI_UNCONSUMED {name}(const {name} &other) = default;"));
			body.push(format!("\t{name} &operator=(const {name} &other) = default;"));
		}
		let mut members = Vec::new();
		if t.clone && !object.shared {
			let clone = apart("clone", &name);
			crate::lock::trait_member(&mut self.locked, &object.shown, "Clone", &clone);
			body.push(format!(
				"\t/// Rust's `Clone`, a value of its own: copies sharing one value could be told apart\n\t/// from clones, so it does not copy.\n\tRUST2_FFI_CALLABLE {name} {clone}() const;"
			));
			self.def(format!(
				"inline {name} {name}::{clone}() const {{\n\treturn {d}::access::wrap<{name}>(::{c_}_clone(raw_.get(){status}));\n}}\n"
			));
			members.push(clone);
		}
		if t.eq {
			let (a, b) = self.operands();
			body.push(format!(
				"\tfriend bool operator==(const {name} &{a}, const {name} &{b}) {{\n\t\treturn ::{c_}_eq({a}.raw_.get(), {b}.raw_.get(){status});\n\t}}\n\tfriend bool operator!=(const {name} &{a}, const {name} &{b}) {{\n\t\treturn !({a} == {b});\n\t}}"
			));
		}
		if t.ord != Order::None {
			let (a, b) = self.operands();
			let ops = self.orders(t.ord, &format!("const {name} &{a}, const {name} &{b}"), &|suffix| {
				(
					String::new(),
					format!("::{c_}_{suffix}({d}::access::raw({a}), {d}::access::raw({b}){status})"),
				)
			});
			// Friends, as `==` is, but for C++20's `<=>`: a class that differs between
			// standards breaks the one-definition rule, so it lives in the namespace.
			for op in ops {
				match op.cpp20 {
					true => self.def(format!("{}\n", op.define("inline "))),
					false => body.push(format!(
						"\tfriend {} {{\n\t{}\n\t}}",
						op.head,
						op.body.trim_end().replace('\n', "\n\t")
					)),
				}
			}
		}
		if t.hash {
			let hash = apart("hash", &name);
			crate::lock::trait_member(&mut self.locked, &object.shown, "Hash", &hash);
			body.push(format!(
				"\t/// Rust's `Hash`, agreeing with `==`, as `std::hash` hashes it.\n\tRUST2_FFI_CALLABLE ::std::uint64_t {hash}() const;"
			));
			self.def(format!(
				"inline ::std::uint64_t {name}::{hash}() const {{\n\treturn ::{c_}_hash(raw_.get(){status});\n}}\n"
			));
			members.push(hash.clone());
			let q = format!("::{}::{name}", self.ns);
			self.hashes.push_str(&format!(
				"namespace std {{\ntemplate <>\nstruct hash<{q}> {{\n\tsize_t operator()(const {q} &value) const noexcept {{\n\t\treturn static_cast<size_t>(value.{hash}());\n\t}}\n}};\n}} // namespace std\n\n"
			));
		}
		self.traits(&object.shown, &c, t, Ok(status), &mut body, &mut members);
		let owner = Ty::Handle {
			name: c.clone(),
			constant: false,
		};
		let taken = self.object_taken(i);
		for (method, name) in object.methods.iter().zip(self.methods(&object.methods, &owner, &taken)) {
			let (decl, member) = self.function(
				method,
				Some(&owner),
				Some(Member {
					name: &name,
					taken: &taken,
				}),
			);
			body.push(decl);
			members.push(member);
		}
		self.distinct(&name, &abi.api.rust(&object.path, &object.args), &members);
		self.line(format!(
			"{}class{} RUST2_FFI_CONSUMABLE {name} {{\n{}\n}};\n",
			docs(text.trim(), ""),
			suffixed(object.deprecated.as_deref()),
			body.join("\n")
		));
	}

	/// The members a type's std traits export beyond its operators: an object's, which
	/// holds a handle and passes each call `Ok` of its status argument, or a record's, of
	/// type `Err`, which is lowered. Each is recorded as a member of the type at `shown`.
	fn traits(
		&mut self,
		shown: &[String],
		c: &str,
		t: Traits,
		of: Result<&str, &Type>,
		body: &mut Vec<String>,
		members: &mut Vec<String>,
	) {
		let c_ = crate::abi::stem(c);
		let name = &self.name(c);
		let ns = self.ns.clone();
		let d = self.d();
		let (value, status) = match of {
			Ok(status) => (None, status),
			Err(ty) => (Some(ty), self.value_status(ty)),
		};
		let object = value.is_none();
		let callable = match object {
			true => "RUST2_FFI_CALLABLE ",
			false => "",
		};
		let (this, lower) = match value {
			None => ("raw_.get()", String::new()),
			Some(ty) => (
				"&c_self_",
				format!("{}\tauto c_self_ = {};\n", self.scope([ty]), self.lower(ty, "*this")),
			),
		};
		if t.display {
			let member = apart("to_string", name);
			crate::lock::trait_member(&mut self.locked, shown, "Display", &member);
			body.push(format!("\t{callable}::{ns}::buf {member}() const;"));
			self.def(format!(
				"inline ::{ns}::buf {name}::{member}() const {{\n{lower}\treturn {d}::access::adopt(::{c_}_to_string({this}{status}));\n}}\n"
			));
			members.push(member);
		}
		if t.debug {
			let member = apart("debug", name);
			crate::lock::trait_member(&mut self.locked, shown, "Debug", &member);
			body.push(format!("\t{callable}::{ns}::buf {member}() const;"));
			self.def(format!(
				"inline ::{ns}::buf {name}::{member}() const {{\n{lower}\treturn {d}::access::adopt(::{c_}_debug({this}{status}));\n}}\n"
			));
			members.push(member);
		}
		if t.default {
			let member = apart("default_", name);
			crate::lock::trait_member(&mut self.locked, shown, "Default", &member);
			body.push(format!("\tstatic {name} {member}();"));
			members.push(member.clone());
			let make = match object {
				true => format!("{d}::access::wrap<{name}>(c_ret_)"),
				false => format!("{d}::lift(c_ret_)"),
			};
			self.def(format!(
				"inline {name} {name}::{member}() {{\n\tauto c_ret_ = ::{c_}_default();\n\treturn {make};\n}}\n"
			));
		}
	}

	fn callback(&mut self, i: usize) {
		let abi = self.abi;
		let callback = &abi.api.callbacks[i];
		let c = abi.callback(i);
		let c_ = crate::abi::stem(&c);
		let name = self.name(&c);
		let text = format!(
			"{}: implemented in C++. Rust deletes it once it drops the implementation, and may\ncall its methods from any thread.\n\n{}",
			callback.path.join("::"),
			callback.docs
		);
		let mut body = vec!["public:".to_string(), format!("\tvirtual ~{name}() = default;")];
		let mut members = Vec::new();
		let mut trampolines = vec![format!(
			"\t\tstatic void free(void *data) noexcept {{\n\t\t\tdelete static_cast<::{}::{name} *>(data);\n\t\t}}",
			self.ns
		)];
		let mut table = vec!["&t::free".to_string()];
		let rust: Vec<String> = callback.methods.iter().map(|m| m.name().to_string()).collect();
		let names = named(&rust, |n| n == name);
		for (n, (method, member)) in callback.methods.iter().zip(names).enumerate() {
			crate::lock::function(&mut self.locked, method, &member);
			members.push(member.clone());
			let constant = match method.receiver {
				Some(Receiver::Mut) => "",
				_ => " const",
			};
			let rust: Vec<String> = method.params.iter().map(|p| p.name.clone()).collect();
			let params: Vec<String> = (method.params.iter())
				.zip(named(&rust, |_| false))
				.map(|(p, param)| declare(&self.lent(&p.ty), &param))
				.collect();
			body.push(format!(
				"{}\t{}virtual {} {member}({}){constant} = 0;",
				docs(&self.function_docs(method, doc::sections(&method.docs).body), "\t"),
				prefixed(method.deprecated.as_deref()),
				self.owned(&method.ok),
				params.join(", ")
			));
			let c_params: Vec<String> = std::iter::once("void *data".to_string())
				.chain(
					method
						.params
						.iter()
						.enumerate()
						.map(|(k, p)| decl(abi, &abi.ty(&p.ty), &format!("p{k}"))),
				)
				.collect();
			let args: Vec<String> = method
				.params
				.iter()
				.enumerate()
				.map(|(k, p)| self.lift_lent(&p.ty, &format!("p{k}")))
				.collect();
			let call = format!(
				"static_cast<::{}::{name}{constant} *>(data)->{member}({})",
				self.ns,
				args.join(", ")
			);
			let ret = match &method.ok {
				Type::Unit => format!("{call};"),
				ok => format!("return {};", self.lower(ok, &call)),
			};
			trampolines.push(format!(
				"\t\tstatic {} m{n}({}) noexcept {{\n\t\t\t{ret}\n\t\t}}",
				decl(abi, &abi.ty(&method.ok), ""),
				c_params.join(", ")
			));
			table.push(format!("&t::m{n}"));
		}
		self.distinct(&name, &callback.path.join("::"), &members);
		self.line(format!(
			"{}class{} {name} {{\n{}\n}};\n",
			docs(text.trim(), ""),
			suffixed(callback.deprecated.as_deref()),
			body.join("\n")
		));
		self.def(format!(
			"namespace detail {{\n\ninline ::{c} lower(::std::unique_ptr<::{ns}::{name}> impl) {{\n\tif (!impl) {{\n\t\tfail(\"{c}: a null implementation\");\n\t}}\n\tstruct t {{\n{}\n\t}};\n\tstatic const ::{c_}_vtable vtable = {{{}}};\n\treturn ::{c}{{&vtable, impl.release()}};\n}}\n\n}} // namespace detail\n",
			trampolines.join("\n"),
			table.join(", "),
			ns = self.ns,
		));
	}

	/// A record's struct or a data enum's payloads, after the values they hold.
	fn value(&mut self, ty: &Type) {
		let abi = self.abi;
		let c = match ty {
			Type::Record(i) => abi.record(*i),
			Type::Enum(i) if !abi.api.enums[*i].plain() => abi.enumeration(*i),
			Type::Option(inner) | Type::Box(inner) | Type::Ref(_, inner) | Type::List(_, inner) => {
				return self.value(inner);
			}
			Type::Tuple(elements) => {
				for element in elements {
					self.value(element);
				}
				return;
			}
			_ => return,
		};
		let c_ = crate::abi::stem(&c);
		// Another crate's, which its header defines.
		let home = match ty {
			Type::Record(i) => &abi.api.records[*i].home,
			Type::Enum(i) => &abi.api.enums[*i].home,
			_ => unreachable!(),
		};
		if home.krate != self.krate || !self.defined.insert(c.clone()) {
			return;
		}
		match ty {
			Type::Record(i) => {
				let record = &abi.api.records[*i];
				for field in &record.fields {
					self.value(&field.ty);
				}
				let name = self.name(&c);
				let (fields, taken) = self.fields(*i);
				let mut body: Vec<String> = record
					.fields
					.iter()
					.zip(&fields)
					.map(|(f, field)| {
						format!(
							"{}\t{}{} {field}{};",
							docs(&f.docs, "\t"),
							prefixed(f.deprecated.as_deref()),
							self.owned(&f.ty),
							init(&f.ty)
						)
					})
					.collect();
				body.push(String::new());
				let mut members = fields.clone();
				let t = record.traits;
				if t.eq {
					// Friends, whose arguments, unlike a member's, shadow no field of any name.
					let (a, b) = self.operands();
					body.push(format!(
						"\tfriend bool operator==(const {name} &{a}, const {name} &{b});\n\tfriend bool operator!=(const {name} &{a}, const {name} &{b}) {{\n\t\treturn !({a} == {b});\n\t}}"
					));
					self.def(format!(
						"inline bool operator==(const {name} &{a}, const {name} &{b}) {{\n{}\tauto c_a_ = {};\n\tauto c_b_ = {};\n\treturn ::{c_}_eq(&c_a_, &c_b_{});\n}}\n",
						self.scope([ty]),
						self.lower(ty, &a),
						self.lower(ty, &b),
						self.value_status(ty),
					));
				}
				if t.ord != Order::None {
					let (a, b) = self.operands();
					let (scope, status) = (self.scope([ty]), self.value_status(ty));
					let (lower_a, lower_b) = (self.lower(ty, &a), self.lower(ty, &b));
					let ops = self.orders(t.ord, &format!("const {name} &{a}, const {name} &{b}"), &|suffix| {
						(
							format!("{scope}\tauto c_a_ = {lower_a};\n\tauto c_b_ = {lower_b};\n"),
							format!("::{c_}_{suffix}(&c_a_, &c_b_{status})"),
						)
					});
					// Declared friends, as `==` is, and defined after the class, but for C++20's
					// `<=>`: a class that differs between standards breaks the one-definition
					// rule, so it is the namespace's alone.
					for op in ops {
						if !op.cpp20 {
							body.push(format!("\tfriend {};", op.head));
						}
						self.def(format!("{}\n", op.define("inline ")));
					}
				}
				self.traits(&record.shown, &c, t, Err(ty), &mut body, &mut members);
				let owner = Ty::Struct(c.clone());
				// An opaque value's bytes are its private member `c_`.
				if record.opaque.is_some() {
					members.push("c_".to_string());
				}
				for (method, name) in record.methods.iter().zip(self.methods(&record.methods, &owner, &taken)) {
					let member = Member {
						name: &name,
						taken: &taken,
					};
					let (decl, member) = self.function(method, Some(&owner), Some(member));
					body.push(decl);
					members.push(member);
				}
				self.distinct(&name, &abi.api.rust(&record.path, &record.args), &members);
				while body.last().is_some_and(String::is_empty) {
					body.pop();
				}
				match record.opaque {
					// Its bytes, which only Rust makes or reads: C++ copies them, but makes
					// none, as it has no public constructor.
					Some(_) => {
						let text = format!(
							"{}\n\nIts bytes, which only Rust makes or reads: copy it, but never make one.",
							record.docs
						);
						self.line(format!(
							"{}class{} {name} {{\npublic:\n{}\n\nprivate:\n\tfriend struct {d}::access;\n\n\texplicit {name}(const ::{c} &c) noexcept : c_(c) {{}}\n\n\t::{c} c_;\n}};\n",
							docs(text.trim(), ""),
							suffixed(record.deprecated.as_deref()),
							body.join("\n"),
							d = self.d()
						));
					}
					None => self.line(format!(
						"{}struct{} {name} {{\n{}\n}};\n",
						docs(&record.docs, ""),
						suffixed(record.deprecated.as_deref()),
						body.join("\n")
					)),
				}
			}
			Type::Enum(i) => {
				let e = &abi.api.enums[*i];
				for variant in &e.variants {
					for field in &variant.fields {
						self.value(&field.ty);
					}
				}
				for variant in &e.variants {
					let payload = self.name(&abi::payload(&c, &variant.name));
					crate::lock::variant(&mut self.locked, &e.shown, &variant.name, &payload);
					let fields: Vec<String> = (variant.fields.iter())
						.zip(field_names(&variant.fields))
						.map(|(f, field)| format!("\t{} {field}{};", self.owned(&f.ty), init(&f.ty)))
						.collect();
					self.line(format!("struct {payload} {{\n{}\n}};\n", fields.join("\n")).replace("{\n\n}", "{}"));
				}
				self.enumeration(*i);
			}
			_ => unreachable!(),
		}
	}

	/// An enum's methods and std traits, as free functions taking it first: C++ enums
	/// hold no members.
	fn enumeration(&mut self, i: usize) {
		let abi = self.abi;
		let e = &abi.api.enums[i];
		let c = abi.enumeration(i);
		let c_ = crate::abi::stem(&c);
		let name = self.name(&c);
		let ns = self.ns.clone();
		let ty = Type::Enum(i);
		let param = self.param(&ty);
		let lifted = self.lift(&ty, "c_ret_");
		let (a, b) = self.operands();
		let lower = |e: &str| self.lower(&ty, e);
		let (lower_a, lower_b, this) = (lower(&a), lower(&b), lower("self"));
		let scope = self.scope([&ty]);
		let t = e.traits;
		let status = self.value_status(&ty);
		if t.eq {
			let (a_, b_) = (declare(&param, &a), declare(&param, &b));
			self.def(format!(
				"inline bool operator==({a_}, {b_}) {{\n{scope}\tauto c_a_ = {lower_a};\n\tauto c_b_ = {lower_b};\n\treturn ::{c_}_eq(&c_a_, &c_b_{status});\n}}\n\ninline bool operator!=({a_}, {b_}) {{\n\treturn !({a} == {b});\n}}\n"
			));
		}
		if t.ord != Order::None {
			let (a_, b_) = (declare(&param, &a), declare(&param, &b));
			let ops = self.orders(t.ord, &format!("{a_}, {b_}"), &|suffix| {
				(
					format!("{scope}\tauto c_a_ = {lower_a};\n\tauto c_b_ = {lower_b};\n"),
					format!("::{c_}_{suffix}(&c_a_, &c_b_{status})"),
				)
			});
			for op in ops {
				self.def(format!("{}\n", op.define("inline ")));
			}
		}
		for (on, key, trait_name) in [(t.display, "Display", "to_string"), (t.debug, "Debug", "debug")] {
			if on {
				let helper = self.name(&format!("{c_}_{trait_name}"));
				crate::lock::trait_member(&mut self.locked, &e.shown, key, &helper);
				self.def(format!(
					"inline ::{ns}::buf {helper}({}) {{\n{scope}\tauto c_self_ = {this};\n\treturn {d}::access::adopt(::{c_}_{trait_name}(&c_self_{status}));\n}}\n",
					declare(&param, "self"),
					d = self.d()
				));
			}
		}
		if t.default {
			let helper = self.name(&format!("{c_}_default"));
			crate::lock::trait_member(&mut self.locked, &e.shown, "Default", &helper);
			self.def(format!(
				"inline {name} {helper}() {{\n\tauto c_ret_ = ::{c_}_default();\n\treturn {lifted};\n}}\n"
			));
		}
		let owner = abi.ty(&ty);
		for method in &e.methods {
			self.function(method, Some(&owner), None);
		}
	}

	/// Each record's and data enum's conversions to and from C.
	fn conversions(&mut self) {
		let abi = self.abi;
		let api = abi.api;
		let ns = self.ns.clone();
		let home = |h: &crate::ir::Home| h.krate == self.krate;
		let records: Vec<usize> = (0..api.records.len()).filter(|&i| home(&api.records[i].home)).collect();
		let enums: Vec<usize> = (0..api.enums.len())
			.filter(|&i| !api.enums[i].plain() && home(&api.enums[i].home))
			.collect();
		let callbacks: Vec<usize> = (0..api.callbacks.len())
			.filter(|&i| home(&api.callbacks[i].home))
			.collect();
		if records.is_empty() && enums.is_empty() && callbacks.is_empty() {
			return;
		}
		self.line("namespace detail {\n");
		for &i in &records {
			let c = abi.record(i);
			let name = self.name(&c);
			let arena = self.arena_param(&Type::Record(i));
			self.line(format!("inline ::{c} lower(const ::{ns}::{name} &v{arena});"));
			self.line(format!("inline ::{ns}::{name} lift(const ::{c} &c);"));
		}
		for &i in &enums {
			let c = abi.enumeration(i);
			let name = self.name(&c);
			let arena = self.arena_param(&Type::Enum(i));
			self.line(format!("inline ::{c} lower(const ::{ns}::{name} &v{arena});"));
			self.line(format!("inline ::{ns}::{name} lift(const ::{c} &c);"));
		}
		for &i in &callbacks {
			let c = abi.callback(i);
			let name = self.name(&c);
			self.line(format!("inline ::{c} lower(::std::unique_ptr<::{ns}::{name}> impl);"));
		}
		self.line("");
		for &i in &records {
			let record = &api.records[i];
			let c = abi.record(i);
			let name = self.name(&c);
			if record.opaque.is_some() {
				let d = self.d();
				self.line(format!(
					"inline ::{c} lower(const ::{ns}::{name} &v) {{\n\treturn {d}::access::bits(v);\n}}\n"
				));
				self.line(format!(
					"inline ::{ns}::{name} lift(const ::{c} &c) {{\n\treturn {d}::access::make<::{ns}::{name}>(c);\n}}\n"
				));
				continue;
			}
			let lowered: Vec<String> = (record.fields.iter())
				.zip(self.fields(i).0)
				.map(|(f, field)| self.lower(&f.ty, &format!("v.{field}")))
				.collect();
			let lifted: Vec<String> = record
				.fields
				.iter()
				.map(|f| self.lift(&f.ty, &format!("c.{}", abi::escape(&f.name))))
				.collect();
			let (v, cc) = match record.fields.is_empty() {
				true => ("", ""),
				false => (" &v", " &c"),
			};
			let arena = self.arena_param(&Type::Record(i));
			self.line(format!(
				"inline ::{c} lower(const ::{ns}::{name}{v}{arena}) {{\n\treturn ::{c}{{{}}};\n}}\n",
				lowered.join(", ")
			));
			self.line(format!(
				"inline ::{ns}::{name} lift(const ::{c}{cc}) {{\n\treturn ::{ns}::{name}{{{}}};\n}}\n",
				lifted.join(", ")
			));
		}
		for &i in &enums {
			let e = &api.enums[i];
			let c = abi.enumeration(i);
			let name = self.name(&c);
			let upper = c.to_uppercase();
			let mut lower = Vec::new();
			let mut lift = Vec::new();
			for (n, variant) in e.variants.iter().enumerate() {
				let tag = abi::join(&upper, &abi::snake(&variant.name).to_uppercase());
				let payload = self.name(&abi::payload(&c, &variant.name));
				let member = abi::escape(&abi::snake(&variant.name));
				let mut set = vec![format!("c.tag = {tag};")];
				let mut fields = Vec::new();
				for (field, name) in variant.fields.iter().zip(field_names(&variant.fields)) {
					set.push(format!(
						"c.{member}.{} = {};",
						abi::escape(&field.name),
						self.lower(&field.ty, &format!("p.{name}"))
					));
					fields.push(self.lift(&field.ty, &format!("c.{member}.{}", abi::escape(&field.name))));
				}
				let bind = match variant.fields.is_empty() {
					true => String::new(),
					false => format!("const auto &p = *::std::get_if<{n}>(&v);\n\t\t"),
				};
				lower.push(format!(
					"\tcase {n}: {{\n\t\t{bind}{}\n\t\tbreak;\n\t}}",
					set.join("\n\t\t")
				));
				lift.push(format!(
					"\tcase {tag}:\n\t\treturn ::{ns}::{name}(::std::in_place_index<{n}>, ::{ns}::{payload}{{{}}});",
					fields.join(", ")
				));
			}
			let arena = self.arena_param(&Type::Enum(i));
			self.line(format!(
				"inline ::{c} lower(const ::{ns}::{name} &v{arena}) {{\n\t::{c} c{{}};\n\tswitch (v.index()) {{\n{}\n\t}}\n\treturn c;\n}}\n",
				lower.join("\n")
			));
			self.line(format!(
				"inline ::{ns}::{name} lift(const ::{c} &c) {{\n\tswitch (c.tag) {{\n{}\n\t}}\n\tfail(\"{c}: an invalid tag\");\n}}\n",
				lift.join("\n")
			));
		}
		self.line("} // namespace detail\n");
	}

	/// The return type a C++ function or future gives for `function`'s result.
	fn output(&self, function: &Function) -> String {
		let ok = self.owned(&function.ok);
		match &function.err {
			Some(err) => format!("::{}::result<{ok}, {}>", self.ns, self.owned(err)),
			None => ok,
		}
	}

	/// Declarations of the out values a C call writes through `outs`, and the C++ value they
	/// make, ok then error, for a result `output`.
	fn outs(&self, function: &Function, outs: &[(String, Ty)], output: &str) -> (Vec<String>, String, Option<String>) {
		let abi = self.abi;
		let declared: Vec<String> = outs
			.iter()
			.map(|(name, ty)| {
				let Ty::Out(inner) = ty else {
					unreachable!("an out parameter is a pointer")
				};
				format!("{}{{}};", decl(abi, inner, &format!("c_{name}_")))
			})
			.collect();
		let owned = strip_refs;
		let ok = match &function.ok {
			Type::Unit => String::new(),
			Type::Tuple(elements) => {
				let lifted: Vec<String> = elements
					.iter()
					.enumerate()
					.map(|(n, e)| self.lift(&owned(e), &format!("c_out{n}_")))
					.collect();
				format!("{}({})", self.owned(&function.ok), lifted.join(", "))
			}
			ok => self.lift(&owned(ok), "c_out_"),
		};
		let ok = match function.err.is_some() {
			true => format!("{output}::ok({ok})"),
			false => ok,
		};
		let err = function
			.err
			.as_ref()
			.map(|err| format!("{output}::err({})", self.lift(&owned(err), "c_err_")));
		(declared, ok, err)
	}

	/// `text`, a function's docs without its sections, then its `# Errors`, naming the
	/// error its `result` holds, and its `# Panics` as when it aborts.
	fn function_docs(&mut self, function: &Function, text: String) -> String {
		let sections = doc::sections(&function.docs);
		let mut text = text.trim().to_string();
		if let Some(errors) = &sections.errors {
			let what = match &function.err {
				Some(err) => format!("Its `result` holds a `{}` error: ", self.owned(err)),
				None => String::new(),
			};
			text.push_str(&format!("\n\n# Errors\n\n{what}{errors}"));
		}
		if let Some(panics) = &sections.panics {
			text.push_str(&format!("\n\n# Aborts\n\n{panics}"));
		}
		text.trim().to_string()
	}

	/// What Doxygen links for `target`, if C++ has it.
	fn link(&self, target: doc::Target<'_>) -> Option<String> {
		let abi = self.abi;
		let ns = &self.ns;
		let owner = |owner: doc::Owner| match owner {
			doc::Owner::Record(i) => self.name(&abi.record(i)),
			doc::Owner::Enum(i) => self.name(&abi.enumeration(i)),
			doc::Owner::Object(i) => self.name(&abi.object(i)),
			doc::Owner::Callback(i) => self.name(&abi.callback(i)),
		};
		Some(match target {
			doc::Target::Record(i) => format!("{ns}::{}", owner(doc::Owner::Record(i))),
			doc::Target::Enum(i) => format!("{ns}::{}", owner(doc::Owner::Enum(i))),
			doc::Target::Object(i) => format!("{ns}::{}", owner(doc::Owner::Object(i))),
			doc::Target::Callback(i) => format!("{ns}::{}", owner(doc::Owner::Callback(i))),
			doc::Target::Variant(i, v) => {
				let e = &abi.api.enums[i];
				let variant = &e.variants[v].name;
				match e.plain() {
					true => format!("{ns}::{}::{}", owner(doc::Owner::Enum(i)), self.values(i)[v]),
					false => format!("{ns}::{}", self.name(&abi::payload(&abi.enumeration(i), variant))),
				}
			}
			doc::Target::Field(i, n) => format!("{ns}::{}::{}", owner(doc::Owner::Record(i)), self.fields(i).0[n]),
			doc::Target::Constant(c) => format!("{ns}::{}", self.locked.get(&format!("const {}", c.path.join("::")))?),
			doc::Target::Function(of, f) => {
				let member = self.locked.get(&crate::lock::function_key(f))?;
				match of {
					// An enum's methods are free functions.
					Some(doc::Owner::Enum(_)) | None => format!("{ns}::{member}()"),
					Some(of) => format!("{ns}::{}::{member}()", owner(of)),
				}
			}
		})
	}

	/// `function`'s declaration in its class, and its member name, with its definition
	/// among the definitions; or as a free function if it has no class.
	fn function(&mut self, function: &Function, owner: Option<&Ty>, member: Option<Member<'_>>) -> (String, String) {
		let abi = self.abi;
		let export = abi.export(function, owner);
		let d = self.d();
		let owner_c = match owner {
			Some(Ty::Struct(n) | Ty::Enum(n) | Ty::Handle { name: n, .. }) => Some(n.clone()),
			_ => None,
		};
		let free = member.is_none();
		let (members, member) = match member {
			Some(Member { name, taken }) => (taken, name.to_string()),
			None => (&[][..], self.name(&export.call.name)),
		};
		crate::lock::function(&mut self.locked, function, &member);
		let ret = match &export.future {
			Some(future) => self.q(&future.handle),
			None => self.output(function),
		};
		let mut params: Vec<String> = Vec::new();
		let mut args: Vec<String> = Vec::new();
		let mut prelude = Vec::new();
		// What the call lowers, which may need an arena.
		let mut lowered: Vec<Type> = function.params.iter().map(|p| p.ty.clone()).collect();
		let receiver = function.receiver.filter(|_| owner.is_some());
		let mut c_params = export.call.params.iter();
		if receiver.is_some() {
			let (_, ty) = c_params.next().expect("a method has its receiver");
			match (ty, free) {
				// Moved out of, as a handle taken by value is.
				(Ty::Handle { name, .. }, _) if receiver == Some(Receiver::Owned) && !copy(abi, name) => {
					args.push("raw_.own()".to_string())
				}
				(Ty::Handle { .. }, _) => args.push("raw_.get()".to_string()),
				(_, false) => {
					let ty = record(abi, owner_c.as_deref().expect("a member method has its record"));
					prelude.push(format!("auto c_self_ = {};", self.lower(&ty, "*this")));
					args.push("&c_self_".to_string());
					lowered.push(ty);
				}
				(_, true) => {
					let ty = enumeration(abi, owner_c.as_deref().expect("a free method has its enum"));
					params.push(declare(&self.param(&ty), "self"));
					prelude.push(format!("auto c_self_ = {};", self.lower(&ty, "self")));
					args.push("&c_self_".to_string());
					lowered.push(ty);
				}
			}
		}
		if lowered.iter().any(|ty| self.arena(ty)) {
			prelude.insert(0, format!("{d}::arena a_;"));
		}
		let mut templates = Vec::new();
		// Whether an argument takes Rust's bytes from its parameter, and the arguments that
		// may throw, by allocating.
		let mut taken = false;
		let mut throwing = Vec::new();
		// The allocations among them that Rust takes (a kept closure, a future), each by its
		// argument, its type, its `new`, and what passes the pointer, which a hoisted one holds
		// in a `unique_ptr` until the call, so a later throw frees it.
		let mut owned: Vec<(usize, String, String, Option<String>)> = Vec::new();
		let rust: Vec<String> = function.params.iter().map(|p| p.name.clone()).collect();
		let names = named(&rust, |name| {
			self.globals.contains(name) || members.iter().any(|n| n == name) || generated(name)
		});
		for (param, name) in function.params.iter().zip(&names) {
			let (_, c_ty) = c_params.next().expect("each parameter has its C one");
			// Any future of nothing, such as a binding's `future` of `void`, moved in.
			if let Type::Future = param.ty {
				let f = format!("F{}", templates.len());
				params.push(format!("{f} {name}"));
				let new = format!("new {f}(::std::move({name}))");
				let wrap = format!("{d}::foreign_future");
				throwing.push(args.len());
				args.push(format!("{wrap}({new})"));
				owned.push((args.len() - 1, f.clone(), new, Some(wrap)));
				templates.push(format!("class {f}"));
				continue;
			}
			// Any reader, lent for the call.
			if let Type::Ref(_, inner) = &param.ty
				&& **inner == Type::Reader
			{
				let f = format!("F{}", templates.len());
				params.push(format!("{f} &{name}"));
				args.push(format!("{d}::foreign_reader({name})"));
				templates.push(format!("class {f}"));
				continue;
			}
			if let (Type::Closure(closure_), Ty::Fn(c_lent, c_ret)) = (&param.ty, c_ty) {
				c_params.next().expect("a closure has its data");
				let f = format!("F{}", templates.len());
				params.push(format!("{f} &&{name}"));
				// Named by its type, not its parameter, so no renamed parameter takes the name.
				let trampoline = format!("c_{}_", f.to_lowercase());
				prelude.push(self.closure(&f, &trampoline, closure_, (c_lent, c_ret)));
				args.push(trampoline);
				match closure_.kept {
					// Moved or copied into a callable Rust owns, which it destroys through `free`.
					true => {
						c_params.next().expect("a kept closure has its free");
						let ty = format!("::std::decay_t<{f}>");
						let new = format!("new {ty}(::std::forward<{f}>({name}))");
						throwing.push(args.len());
						owned.push((args.len(), ty, new.clone(), None));
						args.push(new);
						args.push(format!(
							"[](void *data_) noexcept {{ delete static_cast<::std::decay_t<{f}> *>(data_); }}"
						));
					}
					false => args.push(format!(
						"const_cast<void *>(static_cast<const void *>(::std::addressof({name})))"
					)),
				}
				templates.push(format!("class {f}"));
				continue;
			}
			params.push(declare(&self.param(&param.ty), name));
			if Self::takes(&param.ty) {
				taken = true;
			} else if self.arena(&param.ty) {
				throwing.push(args.len());
			}
			args.push(self.arg(&param.ty, name));
		}
		// Arguments evaluate in no set order, so those that may throw run first, and the call
		// itself throws nothing: Rust's bytes are taken, and an allocation released to Rust,
		// only once nothing left before the call can throw and leak them.
		if taken || throwing.len() > 1 {
			for &i in &throwing {
				match owned.iter().find(|(k, ..)| *k == i) {
					Some((_, ty, new, wrap)) => {
						prelude.push(format!("::std::unique_ptr<{ty}> c_arg{i}_({new});"));
						args[i] = match wrap {
							Some(wrap) => format!("{wrap}(c_arg{i}_.release())"),
							None => format!("c_arg{i}_.release()"),
						};
					}
					None => {
						prelude.push(format!("auto c_arg{i}_ = {};", args[i]));
						args[i] = format!("c_arg{i}_");
					}
				}
			}
		}
		let outs: Vec<(String, Ty)> = c_params.filter(|(_, ty)| !abi::is_status(ty)).cloned().collect();
		// Each value passed for `&mut` takes what Rust wrote back, on every path, as does a
		// value's own `&mut self`, which Rust wrote back in place.
		let mut writes: Vec<String> = match export.call.params.first() {
			Some((_, Ty::InOut(_))) => {
				let ty = record(abi, owner_c.as_deref().expect("a member method has its record"));
				vec![format!("*this = {};", self.lift(&ty, "c_self_"))]
			}
			_ => Vec::new(),
		};
		writes.extend(
			(function.params.iter().zip(&names))
				.filter(|(param, _)| param.ty.written())
				.map(|(param, name)| {
					let out = Abi::written(function, &abi::escape(&param.name));
					format!("{name} = {};", self.lift(param.ty.base(), &format!("c_{out}_")))
				}),
		);
		for (name, _) in &outs {
			args.push(format!("&c_{name}_"));
		}
		// A failed check aborts with its message, as C++ throws nothing.
		if export.call.status().is_some() {
			args.push("nullptr".to_string());
		}
		let call = format!("::{}({})", export.call.name, args.join(", "));
		let mut body = prelude;
		if let Some(abi::Future { handle, .. }) = &export.future {
			body.push(format!("return {d}::access::wrap<{}>({call});", self.q(handle)));
		} else {
			let (declared, ok, err) = self.outs(function, &outs, &ret);
			body.extend(declared);
			match (&function.err, &function.ok) {
				(Some(_), _) if !writes.is_empty() => {
					body.push(format!("bool c_ok_ = {call};"));
					body.extend(writes);
					body.push(format!("if (c_ok_) {{\n\t\treturn {ok};\n\t}}"));
					body.push(format!("return {};", err.expect("an error lifts")));
				}
				(Some(_), _) => {
					body.push(format!("if ({call}) {{\n\t\treturn {ok};\n\t}}"));
					body.push(format!("return {};", err.expect("an error lifts")));
				}
				(None, Type::Tuple(_)) => {
					body.push(format!("{call};"));
					body.extend(writes);
					body.push(format!("return {ok};"));
				}
				(None, Type::Unit) => {
					body.push(format!("{call};"));
					body.extend(writes);
				}
				(None, ok_ty) => {
					body.push(format!("auto c_out_ = {call};"));
					body.extend(writes);
					let lifted = self.lift(&strip_refs(ok_ty), "c_out_");
					body.push(format!("return {lifted};"));
				}
			}
		}
		let qualifier = match (receiver, free) {
			(_, true) | (None, _) => String::new(),
			(Some(Receiver::Ref), _) => " const".to_string(),
			(Some(Receiver::Mut), _) => String::new(),
			(Some(Receiver::Owned), _) => match owner {
				Some(Ty::Handle { name, .. }) if !copy(abi, name) => " &&".to_string(),
				_ => " const".to_string(),
			},
		};
		let is_static = receiver.is_none() && !free;
		let class_name = owner_c.as_deref().map(|c| self.name(c));
		let scope = match (&class_name, free) {
			(Some(class), false) => format!("{class}::"),
			_ => String::new(),
		};
		let template = match templates.is_empty() {
			true => String::new(),
			false => format!("template <{}>\n", templates.join(", ")),
		};
		let mut text = doc::sections(&function.docs).body;
		for (param, name) in function.params.iter().zip(&names) {
			if let Type::Future = param.ty {
				text.push_str(&format!(
					"\n\nRust polls `{name}` from any thread until it is ready, and destroys it once done with it."
				));
			}
			if let Type::Ref(_, inner) = &param.ty
				&& **inner == Type::Reader
			{
				text.push_str(&format!(
					"\n\nRust reads `{name}` only during the call, from any thread, one call at a time, through `read(uint8_t *, size_t)`, returning a `result` of how many bytes it read, at most the size it was given and none only at its end, and `seek(int64_t, std::ios_base::seekdir)`, returning one of the position from the start, or, from the start past `INT64_MAX`, `seek_start(uint64_t)`, which a reader without it fails. A failure's error is anything a `view` takes, which Rust raises as an `io::Error` of that message."
				));
			}
			if let Type::Closure(closure_) = &param.ty {
				let lent = &closure_.params;
				text.push_str(&match closure_.kept {
					true => format!(
						"\n\nRust keeps `{name}`, moved or copied, calls it from any thread, one call at a time, and destroys it once done with it, which may be before any call."
					),
					false => format!("\n\nRust calls `{name}` only during the call, from any thread."),
				});
				if closure_.err.is_some() {
					text.push_str(&format!(
						"\n`{name}` returns a `result`, or any type with its `operator bool`, `operator*`, and `error()`, such as `std::expected`, whose error is anything a `view` takes."
					));
				}
				if lent.iter().any(|p| matches!(p.base(), Type::Object(_))) {
					text.push_str(&format!(
						"\nAn object it lends lives only for that call of `{name}`: move none out of it."
					));
				}
			}
		}
		let text = self.function_docs(function, text);
		// Defined in the header of the crate it belongs to, which may follow its class's;
		// a free function's definition is its declaration, which carries its docs.
		if function.home.krate == self.krate {
			let docs = match scope.is_empty() {
				true => format!(
					"{}{template}{}",
					docs(&text, ""),
					prefixed(function.deprecated.as_deref())
				),
				false => template.clone(),
			};
			self.def(format!(
				"{docs}inline {ret} {scope}{member}({}){qualifier} {{\n\t{}\n}}\n",
				params.join(", "),
				body.join("\n\t")
			));
		}
		// An object's method is callable only before a move empties it, and one taking
		// `self` empties it.
		let prefix = match (is_static, free, owner) {
			(true, _, _) => "static ",
			(false, false, Some(Ty::Handle { .. })) if qualifier == " &&" => "RUST2_FFI_CONSUMES ",
			(false, false, Some(Ty::Handle { .. })) => "RUST2_FFI_CALLABLE ",
			_ => "",
		};
		let declaration = format!(
			"{}\t{}{}{prefix}{ret} {member}({}){qualifier};",
			docs(&text, "\t"),
			template.replace('\n', "\n\t"),
			prefixed(function.deprecated.as_deref()),
			params.join(", ")
		);
		(declaration, member)
	}

	/// The trampoline C calls for a closure parameter of type `f`, as the local `local`,
	/// which C declares as taking `c_lent` and returning `c_ret`: it lends each object as
	/// its class, and gives Rust what the closure returns, an error through its out.
	fn closure(&self, f: &str, local: &str, closure: &Closure, (c_lent, c_ret): (&[Ty], &Ty)) -> String {
		let abi = self.abi;
		let d = self.d();
		let c_params: Vec<String> = std::iter::once("void *data_".to_string())
			.chain(
				c_lent
					.iter()
					.enumerate()
					.map(|(k, ty)| decl(abi, ty, &format!("p{k}_"))),
			)
			.collect();
		// A kept closure is Rust's own copy, and one lent for the call the caller's.
		let data = match closure.kept {
			true => format!("::std::decay_t<{f}>"),
			false => format!("::std::remove_reference_t<{f}>"),
		};
		let mut body = vec![format!("auto &f_ = *static_cast<{data} *>(data_);")];
		let mut args = Vec::new();
		for (k, (ty, c_ty)) in closure.params.iter().zip(c_lent).enumerate() {
			let p = format!("p{k}_");
			match (ty, c_ty) {
				(Type::Ref(..) | Type::Object(_), Ty::Handle { name: c, constant }) => {
					let object = match ty {
						Type::Ref(_, object) => object,
						_ => ty,
					};
					let constant = match constant {
						true => "const ",
						false => "",
					};
					body.push(format!(
						"{d}::lent<{}, {constant}::{c}> l{k}_({p});",
						self.owned(object)
					));
					args.push(match ty {
						Type::Ref(crate::ir::Mutability::Mut, _) => {
							format!("::{}::borrow<{}>(l{k}_.get())", self.ns, self.owned(object))
						}
						// A `Copy` object Rust copied for the call, which its class reads in place.
						_ => format!("::std::as_const(l{k}_.get())"),
					});
				}
				_ => args.push(self.lift_lent(ty, &p)),
			}
		}
		let call = format!("f_({})", args.join(", "));
		match (&closure.err, &closure.ok) {
			(None, Type::Unit) => body.push(format!("{call};")),
			(None, ok) => {
				body.push(format!("auto r_ = {call};"));
				body.push(format!("return {};", self.returned(ok, "r_")));
			}
			(Some(_), ok) => {
				let err = format!("p{}_", c_lent.len() - 1);
				body.push(format!("auto r_ = {call};"));
				body.push(format!(
					"if (!r_) {{\n\t\t\t*{err} = {d}::copy_bytes(r_.error());\n\t\t\treturn false;\n\t\t}}"
				));
				if *ok != Type::Unit {
					body.push(format!("*p{}_ = {};", closure.params.len(), self.returned(ok, "(*r_)")));
				}
				body.push("return true;".to_string());
			}
		}
		format!(
			"auto {local} = []({}) noexcept -> {} {{\n\t\t{}\n\t}};",
			c_params.join(", "),
			decl(abi, c_ret, ""),
			body.join("\n\t\t")
		)
	}

	/// The C value a closure returns Rust of the C++ value `e` of type `ty`, which Rust then
	/// owns: bytes as a buffer `<prefix>_buf_copy` made. `e` is named, so it may be read more
	/// than once.
	fn returned(&self, ty: &Type, e: &str) -> String {
		let d = self.d();
		match ty {
			ty if Abi::is_buf(ty) => format!("{d}::copy_bytes({e})"),
			Type::Option(inner) if Abi::is_buf(inner) => format!("({e} ? {d}::copy_bytes(*{e}) : {d}::none())"),
			ty => self.lower(ty, e),
		}
	}

	/// The operations a future's type polls with.
	fn future(&mut self, function: &Function, owner: Option<&Ty>) {
		let abi = self.abi;
		let export = abi.export(function, owner);
		let Some(abi::Future { handle, poll, free, .. }) = &export.future else {
			return;
		};
		let name = self.name(handle);
		let output = self.output(function);
		let outs: Vec<(String, Ty)> = poll.params[2..]
			.iter()
			.filter(|(_, ty)| !abi::is_status(ty))
			.cloned()
			.collect();
		let (declared, ok, err) = self.outs(function, &outs, &output);
		let ok = match (&function.err, &function.ok) {
			(None, Type::Unit) => "out.emplace();".to_string(),
			_ => format!("out.emplace({ok});"),
		};
		let mut args = vec!["f".to_string(), "w".to_string()];
		args.extend(outs.iter().map(|(name, _)| format!("&c_{name}_")));
		// A poll on another thread than a future's that is not `Send` aborts, as C++ throws
		// nothing.
		if poll.status().is_some() {
			args.push("nullptr".to_string());
		}
		let upper = abi.prefix.to_uppercase();
		let mut cases = vec![format!("\t\tcase {upper}_POLL_READY:\n\t\t\t{ok}\n\t\t\tbreak;")];
		if let Some(err) = err {
			cases.push(format!(
				"\t\tcase {upper}_POLL_ERROR:\n\t\t\tout.emplace({err});\n\t\t\tbreak;"
			));
		}
		cases.push("\t\tdefault:\n\t\t\tbreak;".to_string());
		let declared: String = declared.iter().map(|d| format!("\t\t{d}\n")).collect();
		self.line(format!(
			"namespace detail {{\n\nstruct {name}_op {{\n\tusing raw = ::{handle};\n\tusing output = {output};\n\n\tstatic void free(raw *f) noexcept {{\n\t\t::{}(f);\n\t}}\n\n\tstatic void poll(raw *f, c_waker w, slot<output> &out) {{\n{declared}\t\tswitch (::{}({})) {{\n{}\n\t\t}}\n\t}}\n}};\n\n}} // namespace detail\n",
			free.name,
			poll.name,
			args.join(", "),
			cases.join("\n")
		));
	}
}

/// The aliases `rt.hpp` declares in the namespace for the runtime's C types, whatever the
/// API, each with its C name.
fn fixed_aliases(abi: &Abi<'_>) -> Vec<(String, String)> {
	let p = &abi.prefix;
	let mut aliases = vec![
		("u128".to_string(), format!("{p}_u128")),
		("duration".to_string(), format!("{p}_duration")),
	];
	for socket in [true, false] {
		for family in [Family::Any, Family::V4, Family::V6] {
			let name = crate::abi::addr(socket, family);
			aliases.push((name.to_string(), format!("{p}_{name}")));
		}
	}
	if abi.ranges.values().any(|ty| matches!(ty, Type::Bound(_))) {
		aliases.push(("bound".to_string(), abi.rt("bound")));
	}
	aliases
}

/// The names the runtime declares in the namespace, outside its `detail`, and `detail`.
fn runtime_names() -> impl Iterator<Item = &'static str> {
	let mut detail = false;
	let lines = RUNTIME.lines().filter(move |line| {
		match *line {
			"namespace detail {" => detail = true,
			"} // namespace detail" => detail = false,
			_ => {}
		}
		!detail
	});
	let names = lines.filter_map(|line| {
		let rest = ["class ", "struct ", "using "]
			.iter()
			.find_map(|kw| line.strip_prefix(kw))?;
		// Past the class's attributes, such as `[[nodiscard]]` and `RUST2_FFI_OWNER(T)`.
		let name = (rest.split(' ')).find(|word| !word.starts_with("[[") && !word.starts_with("RUST2_FFI_"))?;
		name.split(|c: char| !(c.is_ascii_alphanumeric() || c == '_')).next()
	});
	std::iter::once("detail").chain(names)
}

/// The member `bare` a type's trait exports, in the class `class`, with `_` appended where
/// it is the class's name, which C++ would read as a constructor.
fn apart(bare: &str, class: &str) -> String {
	match bare == class {
		true => format!("{bare}_"),
		false => bare.to_string(),
	}
}

/// The C++ names of `rust`, the names of one scope, kept apart from each other and from those
/// `taken` holds or a header defines as a macro, by [`abi::named`]'s rule with C++'s
/// [`abi::stepped`]: a name escaping leaves as it is keeps it while `taken` lacks it, and every
/// other, escaped, steps in Rust's order of names. So a field `new_` stays `new_` beside
/// `new`, which becomes `new_2`, and no order of the source moves one.
fn named(rust: &[String], taken: impl Fn(&str) -> bool) -> Vec<String> {
	let rust: Vec<&str> = rust.iter().map(String::as_str).collect();
	let spelled = rust.iter().map(|name| escape(name)).collect();
	let own = |rust: &str, name: &str| name == rust;
	abi::named(
		&rust,
		spelled,
		own,
		|_, name| taken(name) || abi::defined(name),
		abi::stepped,
	)
}

/// The C++ names of `fields`, a struct's with no other members: a data enum's payload's.
fn field_names(fields: &[crate::ir::Field]) -> Vec<String> {
	let rust: Vec<String> = fields.iter().map(|f| f.name.clone()).collect();
	named(&rust, |_| false)
}

/// The C++ name of the parameter `rust`, [`abi::stepped`] further while `taken` holds it, a header
/// defines it as a macro, or it is a name the header gives its own locals and members.
fn parameter(rust: &str, taken: impl Fn(&str) -> bool) -> String {
	let escaped = escape(rust);
	let mut name = escaped.clone();
	let mut n = 0;
	while taken(&name) || generated(&name) || abi::defined(&name) {
		n += 1;
		name = abi::stepped(&escaped, n);
	}
	name
}

/// Whether the header names its own locals and members `name`: the arena `a_`, a handle's
/// `raw_`, a C value or a closure's trampoline `c_…_`, and a trampoline's or lambda's `f_`,
/// `data_`, `p0_`, `l0_`, `x0_`, or `c0_`.
fn generated(name: &str) -> bool {
	let Some(stem) = name.strip_suffix('_') else {
		return false;
	};
	let numbered = |prefix: char| {
		stem.strip_prefix(prefix)
			.is_some_and(|n| !n.is_empty() && n.bytes().all(|b| b.is_ascii_digit()))
	};
	let c_value = stem.starts_with("c_") && !stem.ends_with('_');
	matches!(stem, "a" | "raw" | "f" | "data") || c_value || "xcpl".chars().any(numbered)
}

/// How many lists `ty` nests, which names the element of each list's lambda apart from
/// those of the lambdas around it.
fn lists(ty: &Type) -> usize {
	match ty {
		Type::List(_, inner) => 1 + lists(inner),
		Type::Option(inner) | Type::Box(inner) | Type::Ref(_, inner) => lists(inner),
		Type::Tuple(elements) => elements.iter().map(lists).max().unwrap_or(0),
		_ => 0,
	}
}

/// The enum named `c` in C.
fn enumeration(abi: &Abi<'_>, c: &str) -> Type {
	let i = (0..abi.api.enums.len()).find(|&i| abi.enumeration(i) == c);
	Type::Enum(i.expect("a free method's owner is an enum"))
}

/// The record named `c` in C.
fn record(abi: &Abi<'_>, c: &str) -> Type {
	let i = (0..abi.api.records.len()).find(|&i| abi.record(i) == c);
	Type::Record(i.expect("a member method's value is a record"))
}

/// Whether passing `ty` moves a handle out of a `Vec`, `Option`, or tuple, as a handle
/// taken by value is.
fn moves(abi: &Abi<'_>, ty: &Type) -> bool {
	match ty {
		Type::Object(i) => !abi.api.objects[*i].copy,
		Type::Option(inner) | Type::Box(inner) | Type::List(_, inner) => moves(abi, inner),
		Type::Tuple(elements) => elements.iter().any(|e| moves(abi, e)),
		_ => false,
	}
}

/// Whether the object named `c` in C is `Copy`.
fn copy(abi: &Abi<'_>, c: &str) -> bool {
	(0..abi.api.objects.len()).any(|i| abi.api.objects[i].copy && abi.object(i) == c)
}

/// `ty` returned: a borrow is returned as a clone, so owned.
fn strip_refs(ty: &Type) -> Type {
	match ty {
		Type::Ref(_, inner) => strip_refs(inner),
		Type::Option(inner) => Type::Option(Box::new(strip_refs(inner))),
		Type::Box(inner) => Type::Box(Box::new(strip_refs(inner))),
		ty => ty.clone(),
	}
}

/// A parameter of C++ type `ty` named `name`.
fn declare(ty: &str, name: &str) -> String {
	match ty.ends_with('&') || ty.ends_with('*') {
		true => format!("{ty}{name}"),
		false => format!("{ty} {name}"),
	}
}

/// A field's initializer, so a default-constructed struct holds no garbage: none for a
/// type with no default constructor.
fn init(ty: &Type) -> &'static str {
	match ty {
		Type::Object(_) | Type::Enum(_) | Type::Record(_) => "",
		Type::Box(inner) => init(inner),
		Type::Tuple(elements) if elements.iter().any(|e| init(e).is_empty()) => "",
		_ => "{}",
	}
}

/// `header` with no warning of a use of a deprecated item between its include guard's
/// lines: the generated code's own uses, which a consumer's are not.
fn quiet(header: &str) -> String {
	const PUSH: &str = "#if defined(__GNUC__) || defined(__clang__)\n#pragma GCC diagnostic push\n#pragma GCC diagnostic ignored \"-Wdeprecated-declarations\"\n#elif defined(_MSC_VER)\n#pragma warning(push)\n#pragma warning(disable : 4996)\n#endif\n";
	const POP: &str = "#if defined(__GNUC__) || defined(__clang__)\n#pragma GCC diagnostic pop\n#elif defined(_MSC_VER)\n#pragma warning(pop)\n#endif\n";
	let (Some(start), Some(end)) = (header.find("\n#define "), header.rfind("\n#endif")) else {
		return header.to_string();
	};
	let start = start + header[start + 1..].find('\n').map_or(0, |n| n + 1) + 1;
	format!(
		"{}{PUSH}{}{POP}{}",
		&header[..start],
		&header[start..end + 1],
		&header[end + 1..]
	)
}

/// `[[deprecated]]` with `deprecated`'s note, where it has one.
fn attribute(deprecated: Option<&str>) -> String {
	match deprecated {
		Some("") => "[[deprecated]]".to_string(),
		Some(note) => format!("[[deprecated({})]]", cpp_string(note)),
		None => String::new(),
	}
}

/// A C++ string literal of `text`, its control characters in octal, which ends after
/// three digits where a hex escape would take the next character as its own.
fn cpp_string(text: &str) -> String {
	let mut out = String::from("\"");
	for c in text.chars() {
		match c {
			'"' | '\\' => {
				out.push('\\');
				out.push(c);
			}
			'\n' => out.push_str("\\n"),
			'\t' => out.push_str("\\t"),
			c if c.is_ascii_control() => out.push_str(&format!("\\{:03o}", c as u32)),
			c => out.push(c),
		}
	}
	out.push('"');
	out
}

/// [`attribute`] before a declaration, which a space then follows.
fn prefixed(deprecated: Option<&str>) -> String {
	match deprecated {
		Some(_) => format!("{} ", attribute(deprecated)),
		None => String::new(),
	}
}

/// [`attribute`] after a class's or alias's key word or name, which a space then precedes.
fn suffixed(deprecated: Option<&str>) -> String {
	match deprecated {
		Some(_) => format!(" {}", attribute(deprecated)),
		None => String::new(),
	}
}

/// C++ comments holding `docs`, each line indented by `indent`, or nothing for none. A line
/// ending in a backslash, as a string continued in clap's examples does, would continue a
/// `///` comment onto the next line, so such docs are a block comment instead.
fn docs(docs: &str, indent: &str) -> String {
	if docs.lines().any(|line| line.ends_with('\\')) {
		let docs = docs.replace("*/", "* /").replace("/*", "/ *");
		let body: String = (docs.lines())
			.map(|line| match line.is_empty() {
				true => format!("{indent} *\n"),
				false => format!("{indent} * {line}\n"),
			})
			.collect();
		return format!("{indent}/**\n{body}{indent} */\n");
	}
	docs.lines()
		.map(|line| match line.is_empty() {
			true => format!("{indent}///\n"),
			false => format!("{indent}/// {line}\n"),
		})
		.collect()
}

fn cpp_int(int: Int) -> &'static str {
	match int {
		Int::U8 => "::std::uint8_t",
		Int::U16 => "::std::uint16_t",
		Int::U32 => "::std::uint32_t",
		Int::U64 => "::std::uint64_t",
		Int::Usize => "::std::size_t",
		Int::I8 => "::std::int8_t",
		Int::I16 => "::std::int16_t",
		Int::I32 => "::std::int32_t",
		Int::I64 => "::std::int64_t",
		Int::Isize => "::std::ptrdiff_t",
	}
}

/// A name C++ would read as a keyword with `_` appended, one starting with a digit with `_`
/// prepended, as C's are, and one a header defines as a macro [`abi::stepped`] past it. One C++
/// reserves for the implementation, holding `__` or starting with `_` and a capital
/// ([lex.name]), first drops its leading `_`s and keeps one `_` of each run, so `__all__` is
/// `all_`: appending keeps a reserved name reserved.
pub(crate) fn escape(name: &str) -> String {
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
		"restrict",
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
		// What the generated code names unqualified.
		"std",
		"self",
	];
	// A C name of an item named only `_`s is its prefix and `_` alone, which leaves `_`.
	let name = match abi::unreserved(name) {
		name if name.is_empty() => "_".to_string(),
		name => name,
	};
	match KEYWORDS.contains(&name.as_str()) {
		// One `_`, which makes no macro's name.
		true => format!("{name}_"),
		// The names of its stem that no macro holds keep their order, so `unix` and `unix_`
		// stay apart as `unix_` and `unix_2`. 32-bit x86's GNU dialects predefine `i386`,
		// which `macros.txt` lacks.
		false => abi::escaped(&name, |stem| stem == "i386"),
	}
}

/// Whether C++ reserves `name` for the implementation in every scope: one holding `__`, or
/// starting with `_` and a capital.
#[cfg(test)]
fn reserved(name: &str) -> bool {
	name.contains("__") || abi::reserved(name)
}

#[cfg(test)]
mod tests {
	use super::{Header, escape, named, parameter};
	use crate::ir::Traits;

	#[test]
	fn a_trailing_backslash_ends_no_line_comment() {
		assert_eq!(super::docs("a\nb", "\t"), "\t/// a\n\t/// b\n");
		assert_eq!(super::docs("x \\\n\ny */", ""), "/**\n * x \\\n *\n * y * /\n */\n");
	}

	#[test]
	fn methods_keep_apart_from_trait_members() {
		let traits = Traits {
			default: true,
			..Traits::default()
		};
		let taken = Header::made(traits, "Hasher");
		let method = |rust: &str| named(&[rust.to_string()], |n| taken.iter().any(|t| t == n)).remove(0);
		// rustc-hash's `FxHasher::default`, which C renames `_default_` beside `Default`'s
		// `_default`, while C++ escapes the keyword of both to `default_`.
		assert_eq!(method("default_"), "default_2");
		assert_eq!(method("default"), "default_2");
		assert_eq!(method("seed"), "seed");
	}

	#[test]
	fn names_escaping_alike_stay_apart() {
		let names = |rust: &[&str], taken: &[&str]| {
			let rust: Vec<String> = rust.iter().map(|n| n.to_string()).collect();
			let taken: Vec<String> = taken.iter().map(|n| n.to_string()).collect();
			named(&rust, |n| taken.iter().any(|t| t == n))
		};
		// A name escaping leaves as it is keeps it, in either order.
		assert_eq!(names(&["delete", "delete_"], &[]), ["delete_2", "delete_"]);
		assert_eq!(names(&["delete_", "delete"], &[]), ["delete_", "delete_2"]);
		assert_eq!(
			names(&["class", "class_", "class__"], &[]),
			["class_2", "class_", "class_3"]
		);
		// Unless something else holds it, such as a trait's member.
		assert_eq!(names(&["debug", "debug_"], &["debug"]), ["debug_2", "debug_"]);
		assert_eq!(names(&["default", "seed"], &["default_"]), ["default_2", "seed"]);
		// And two that neither keeps move by Rust's order, not the source's.
		assert_eq!(
			names(&["default", "default_"], &["default_"]),
			["default_2", "default_3"]
		);
		assert_eq!(
			names(&["default_", "default"], &["default_"]),
			["default_3", "default_2"]
		);
		// The macros' family escapes apart already.
		assert_eq!(names(&["unix", "unix_"], &[]), ["unix_", "unix_2"]);
		// A reserved name stays apart from what it escapes to.
		assert_eq!(names(&["__all__", "all_", "all"], &[]), ["all_2", "all_", "all"]);
		// A function's parameters also stay clear of the header's own locals, such as `a_`.
		let rust = ["a_".to_string(), "a".to_string(), "new_".to_string(), "new".to_string()];
		assert_eq!(named(&rust, super::generated), ["a_2", "a", "new_", "new_2"]);
	}

	#[test]
	fn macros_escape_apart() {
		// What a system header defines, whether a constant, a function, or a field.
		assert_eq!(escape("WEXITSTATUS"), "WEXITSTATUS_");
		assert_eq!(escape("NULL"), "NULL_");
		assert_eq!(escape("errno"), "errno_");
		assert_eq!(escape("h_addr"), "h_addr_");
		// Only C's `tgmath.h` defines `sin`, and only as a function-like macro.
		assert_eq!(escape("sin"), "sin");
		// A keyword C defines as a macro keeps its one escape, past which the rest move.
		assert_eq!(escape("bool"), "bool_");
		assert_eq!(escape("bool_"), "bool_2");
		// Predefined by GNU dialects.
		assert_eq!(escape("i386"), "i386_");
		assert_eq!(escape("unix"), "unix_");
		assert_eq!(escape("unix_"), "unix_2");
		assert_eq!(escape("unixes"), "unixes");
		assert_eq!(escape("offsetof_"), "offsetof_2");
		// A keyword keeps its single escape, so a field `type_` stays `type_`.
		assert_eq!(escape("type_"), "type_");
		// Past `ABDAY_2`, a macro `<langinfo.h>` defines.
		assert_eq!(escape("ABDAY"), "ABDAY");
		assert_eq!(escape("ABDAY_"), "ABDAY_");
		// A step moves past each of `ABDAY_2` to `ABDAY_7`.
		assert_eq!(
			named(&["ABDAY".into(), "ABDAY_".into()], |n| n == "ABDAY_"),
			["ABDAY", "ABDAY_8"]
		);
		// C's escapes, which keep each name apart with no scope to resolve them in, so a
		// keyword's own with `_` steps past its escape.
		assert_eq!(crate::abi::escape("bool"), "bool_");
		assert_eq!(crate::abi::escape("bool_"), "bool_2");
		assert_eq!(escape("bool_"), "bool_2");
		assert_eq!(crate::abi::escape("type_"), "type_");
		assert_eq!(crate::abi::escape("new_"), "new_2");
		assert_eq!(escape("new_"), "new_");
	}

	#[test]
	fn reserved_names_escape_unreserved() {
		// The api fixture's, and serde's derived `Deserialize::deserialize` parameter.
		assert_eq!(escape("__all__"), "all_");
		assert_eq!(escape("__deserializer"), "deserializer");
		assert_eq!(escape("_Bool"), "Bool");
		assert_eq!(escape("a__b"), "a_b");
		// Unreserved, it escapes as any other name, macros included.
		assert_eq!(escape("__class"), "class_");
		assert_eq!(escape("linux__"), "linux_2");
		assert_eq!(escape("EOF__"), "EOF_2");
		assert_eq!(escape("__cplusplus"), "cplusplus");
		assert_eq!(escape("__GNUC__"), "GNUC_");
		assert_eq!(escape("_STRING_H"), "STRING_H");
		assert_eq!(escape("_STRING_H_"), "STRING_H_");
		assert_eq!(escape("__1"), "_1");
		assert_eq!(escape("__"), "_");
		// C++ reserves a lowercase `_x` only in the global namespace, which holds no name of
		// the API.
		assert_eq!(escape("_bool"), "_bool");
	}

	/// Every identifier of up to five of `_`, `a`, `A`, and `1`, with the keywords and
	/// macros and their trailing `_`s, escapes to a name C++ leaves to programs, and a scope
	/// of all of them names each apart, still unreserved.
	#[test]
	fn no_name_is_reserved() {
		let mut rust: Vec<String> = vec![String::new()];
		let mut all = Vec::new();
		for _ in 0..5 {
			rust = (rust.iter())
				.flat_map(|name| "_aA1".chars().map(move |c| format!("{name}{c}")))
				.collect();
			all.extend(rust.iter().filter(|n| !n.starts_with('1') && *n != "_").cloned());
		}
		for stem in ["class", "new", "unix", "errno", "raw", "c_out"] {
			all.extend((0..3).map(|n| format!("{stem}{}", "_".repeat(n))));
		}
		all.sort();
		all.dedup();
		for name in &all {
			assert!(!super::reserved(&escape(name)), "{name} escapes to {}", escape(name));
			assert!(!super::reserved(&parameter(name, |_| false)), "{name} as a parameter");
		}
		let names = named(&all, super::generated);
		assert!(!names.iter().any(|n| super::reserved(n)), "a scope names one reserved");
		let distinct: std::collections::BTreeSet<&String> = names.iter().collect();
		assert_eq!(distinct.len(), names.len(), "a scope names two alike");
	}

	/// Each macro's stem, and it with one `_`, escape apart, to neither a macro nor a name C++
	/// reserves. More `_`s make a reserved name, which unreserves onto one of these two.
	#[test]
	fn escapes_keep_apart_from_every_macro() {
		let macros = include_str!("macros.txt").lines().filter(|line| !line.starts_with('#'));
		for stem in macros.map(|m| m.trim_end_matches('_')) {
			let escaped = [escape(stem), escape(&format!("{stem}_"))];
			// A keyword's one escape merges it with its own `_`, which each scope keeps apart.
			let keyword = |e: &str| e.strip_suffix('_').is_some_and(|k| escape(k) == e);
			assert!(
				escaped[0] != escaped[1] || keyword(&escaped[0]),
				"{stem}'s names escape alike"
			);
			for e in &escaped {
				assert!(!crate::abi::defined(e) && !super::reserved(e), "{stem} escapes to {e}");
			}
		}
	}

	/// Functions whose names escape alike stay apart in the namespace, as every language
	/// but Kotlin exports them.
	#[test]
	fn the_namespace_keeps_escapes_apart() {
		let api = crate::ir::Api {
			records: vec![],
			enums: vec![],
			objects: vec![],
			callbacks: vec![],
			functions: [
				"__all__",
				"all_",
				"all",
				"delete",
				"delete_",
				"__u128",
				"__duration",
				"a_b",
				"__",
				"___",
			]
			.map(crate::ir::Function::bare)
			.to_vec(),
			constants: ["__LIMIT", "LIMIT", "a_b"]
				.map(|name| crate::ir::Constant {
					path: vec!["ns".into(), name.into()],
					home: crate::ir::Home {
						krate: "ns".into(),
						outside: 1,
					},
					span: rustc_span::DUMMY_SP,
					on: crate::ir::On::ALL,
					docs: String::new(),
					deprecated: None,
					value: crate::ir::Literal::Bool(true),
				})
				.to_vec(),
			crates: vec!["ns".into()],
			targets: vec!["host".into()],
			links: Default::default(),
			reached: Vec::new(),
			profiles: Default::default(),
		};
		let rest = api.without_jni();
		let abi = crate::abi::Abi::new(&api, "ns");
		let abi = abi.without_jni(&rest);
		let output = super::headers(&abi, "ns").unwrap_or_else(|e| panic!("{e}"));
		let text: String = output.files.iter().map(|(_, text)| text.as_str()).collect();
		// C unreserves each name already, stepping past Rust's own spelling.
		for (cpp, c) in [
			("all_2", "ns_all_2"),
			("all_", "ns_all_"),
			("all", "ns_all"),
			("delete_2", "ns_delete"),
			("delete_", "ns_delete_"),
			// Past the runtime's.
			("u128_", "ns_u128_"),
			("duration_", "ns_duration_"),
			// Names of only `_`s, whose C names are the prefix and `_`, then stepped.
			("_", "ns_"),
			("_2", "ns_2"),
		] {
			let defined = format!("inline void {cpp}() {{\n\t::{c}();");
			assert!(text.contains(&defined), "{c} is not {cpp}:\n{text}");
		}
		// Constants, whose C names are in capitals, keep apart too.
		// A constant and a function of one C++ spelling, though C's case keeps them apart.
		assert!(text.contains("inline void a_b_() {\n\t::ns_a_b();"), "{text}");
		for (cpp, c) in [("LIMIT_", "NS_LIMIT_"), ("LIMIT", "NS_LIMIT"), ("a_b", "NS_A_B")] {
			let defined = format!("inline constexpr bool {cpp} = {c};");
			assert!(text.contains(&defined), "{c} is not {cpp}:\n{text}");
		}
	}

	#[test]
	fn a_namespace_escapes_as_a_name() {
		let api = crate::ir::Api::of(vec![crate::ir::Function::bare("f")]);
		for (lib, ns, c) in [("_class", "class_", "class_f"), ("unix", "unix_", "unix_f"), ("api", "api", "api_f")] {
			let rest = api.without_jni();
			let abi = crate::abi::Abi::new(&api, lib);
			let abi = abi.without_jni(&rest);
			let output = super::headers(&abi, "ns").unwrap_or_else(|e| panic!("{e}"));
			let text: String = output.files.iter().map(|(_, text)| text.as_str()).collect();
			assert!(text.contains(&format!("namespace {ns} {{")), "{lib}:\n{text}");
			assert!(text.contains(&format!("inline void f() {{\n\t::{c}();")), "{lib}:\n{text}");
		}
	}

	#[test]
	fn parameters_keep_apart_from_what_they_would_hide() {
		let taken = |name: &str| ["a", "size", "size_"].contains(&name);
		// Past a namespace name, and past the arena `a_` the rename would reach.
		assert_eq!(parameter("a", taken), "a_2");
		// Past a name already taken by the first rename.
		assert_eq!(parameter("size", taken), "size_2");
		// A keyword escapes as before; a name like a generated one moves out of its way.
		assert_eq!(parameter("new", taken), "new_");
		assert_eq!(parameter("raw_", taken), "raw_2");
		assert_eq!(parameter("c_out_", taken), "c_out_2");
		assert_eq!(parameter("x1_", taken), "x1_2");
		assert_eq!(parameter("x_", taken), "x_");
	}
}
