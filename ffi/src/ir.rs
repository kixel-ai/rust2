//! The exported API, as each generator reads it: what the exported crates' `pub` items
//! became, with no language's names or calling convention decided yet.
//!
//! Every item keeps its Rust path, so a generator names it by its own rule, and its span,
//! so a generator refuses it with one.

use rustc_span::Span;

/// The exported crates' API.
pub struct Api {
	pub records: Vec<Record>,
	pub enums: Vec<Enum>,
	pub objects: Vec<Object>,
	pub callbacks: Vec<Callback>,
	pub functions: Vec<Function>,
	pub constants: Vec<Constant>,
	/// The crates items belong to, each after every crate it depends on.
	pub crates: Vec<String>,
}

impl Api {
	/// A type's Rust path with its instance's arguments, such as
	/// `moq_mux::catalog::Producer<moq_mux::catalog::hang::Extra>`.
	pub fn rust(&self, path: &Path, args: &[Type]) -> String {
		let path = path.join("::");
		match args {
			[] => path,
			args => {
				let args: Vec<String> = args.iter().map(|arg| self.arg(arg)).collect();
				format!("{path}<{}>", args.join(", "))
			}
		}
	}

	/// A type argument, as [`Self::rust`] spells it.
	fn arg(&self, ty: &Type) -> String {
		match ty {
			Type::Unit => "()".to_string(),
			Type::Bool => "bool".to_string(),
			Type::Int(int) => int.rust().to_string(),
			Type::F32 => "f32".to_string(),
			Type::F64 => "f64".to_string(),
			Type::Char => "char".to_string(),
			Type::String => "String".to_string(),
			Type::Bytes => "bytes::Bytes".to_string(),
			Type::Duration => "std::time::Duration".to_string(),
			Type::Record(i) => self.rust(&self.records[*i].path, &self.records[*i].args),
			Type::Enum(i) => self.rust(&self.enums[*i].path, &self.enums[*i].args),
			Type::Object(i) => self.rust(&self.objects[*i].path, &self.objects[*i].args),
			other => unreachable!("no type argument is {other:?}"),
		}
	}
}

/// A Rust path: its export's package name in snake case, unique among the exports, then
/// the path from that crate's root, such as `["moq_net", "track", "Producer", "append_group"]`.
pub type Path = Vec<String>;

/// The crate an item belongs to, and how much of its path lies outside that crate.
#[derive(Clone, Default, PartialEq, Eq, Debug)]
pub struct Home {
	/// The crate's name, followed by `#` and a number for a later crate of a name an
	/// earlier one has: the one defining the item, or for an instance, the last of its own
	/// and its arguments' crates in [`Api::crates`], so a crate's items name only types of
	/// crates before it.
	pub krate: String,
	/// How many leading segments of its `shown` path, the facade's root among them, name
	/// no module of that crate: the facade's own modules.
	pub outside: usize,
}

/// A struct whose fields are all `pub`: a value, copied across.
pub struct Record {
	pub path: Path,
	pub home: Home,
	/// Its path with its instance's arguments, which the config names it by, such as
	/// `moq_mux::catalog::Producer<moq_mux::catalog::hang::Extra>`.
	pub shown: Path,
	/// The type arguments of the instance it is, empty unless the type is generic.
	pub args: Vec<Type>,
	pub span: Span,
	pub docs: String,
	pub fields: Vec<Field>,
	/// Whether its fields are positional, named `0`, `1`, ...
	pub tuple: bool,
	pub methods: Vec<Function>,
	pub traits: Traits,
}

pub struct Field {
	/// Its Rust name: a number for a positional field.
	pub name: String,
	pub docs: String,
	pub ty: Type,
}

/// An enum: a value, copied across. Without fields in any variant, it is a plain
/// enumeration.
pub struct Enum {
	pub path: Path,
	pub home: Home,
	pub shown: Path,
	pub args: Vec<Type>,
	pub span: Span,
	pub docs: String,
	pub variants: Vec<Variant>,
	/// Whether another crate can match it without a wildcard arm.
	pub exhaustive: bool,
	pub methods: Vec<Function>,
	pub traits: Traits,
}

pub struct Variant {
	pub name: String,
	/// Its discriminant, for an enum without fields.
	pub discriminant: i128,
	pub fields: Vec<Field>,
	pub tuple: bool,
}

impl Enum {
	/// Whether no variant has fields.
	pub fn plain(&self) -> bool {
		self.variants.iter().all(|v| v.fields.is_empty())
	}
}

/// Any other type: a handle to a value the foreign side never sees inside, shared behind
/// an `Arc`, whose receivers borrow it as Rust's do, checked at run time.
pub struct Object {
	pub path: Path,
	pub home: Home,
	pub shown: Path,
	pub args: Vec<Type>,
	pub span: Span,
	pub docs: String,
	/// Whether it is `Sync`: if not, `&self` borrows it on one thread at a time.
	pub sync: bool,
	/// Whether it is `Copy`: taken by value, it is copied out of its handle, as Rust
	/// copies it, rather than moved out.
	pub copy: bool,
	/// Whether copies of its handle may share one value, since no call tells that apart
	/// from `Clone` but by an `Arc`'s or `Rc`'s count: it is `Clone` and `Sync`, its clone owns no state that changes
	/// behind `&` or counts clones, and no export borrows it mutably.
	pub shared: bool,
	/// How many lifetime parameters it has, each `'static` in the export.
	pub lifetimes: usize,
	pub methods: Vec<Function>,
	pub traits: Traits,
	/// Whether it is a foreign type, such as `url::Url`, that the facade maps rather than
	/// exports: its path is the type's in its own crate, not in the facade.
	pub foreign: bool,
	/// Whether it is a foreign type behind an `Arc`, held erased, which the glue never
	/// names: a handle with none of its API, which Rust takes back as the `Arc`.
	pub opaque: bool,
}

/// A `pub` trait: implemented by the foreign side, through a table of functions.
pub struct Callback {
	pub path: Path,
	pub home: Home,
	pub span: Span,
	pub docs: String,
	/// Its required methods, each taking `&self` or `&mut self`.
	pub methods: Vec<Function>,
}

/// The std traits a type's impls export.
#[derive(Default, Clone, Copy)]
pub struct Traits {
	pub clone: bool,
	pub eq: bool,
	pub display: bool,
	pub debug: bool,
	pub default: bool,
}

/// A function or method.
pub struct Function {
	/// Its path, ending in its own name: a method's goes through its type's.
	pub path: Path,
	/// Its path with the arguments of its type's instance and its own, which the config
	/// names it by: `moq_mux::catalog::Producer<moq_mux::catalog::hang::Extra>::stats`.
	pub shown: Path,
	pub span: Span,
	/// Its doc comment, as Markdown.
	pub docs: String,
	pub receiver: Option<Receiver>,
	pub params: Vec<Param>,
	/// What it returns, or for a future, what the future does.
	pub ok: Type,
	/// The error of a `Result` it returns: `String` for another crate's error, as its message.
	pub err: Option<Type>,
	/// Whether it returns a future: an `async fn`, or a value that implements `Future`.
	pub future: bool,
	/// For a future, whether the call itself runs code, as a function that is not
	/// `async` returning one does: the export calls it at once, and polls what it returns.
	pub eager: bool,
	/// The parameter it polls its task through, if any: C passes a waker for it or, for a
	/// future made of a returned `Poll`, the future's poll supplies it.
	pub task: Option<Task>,
	/// Whether it returns another crate's iterator, whose items the export collects into
	/// the array it returns.
	pub collect: bool,
	/// Whether it returns a borrow of an object that is not `Clone`, which C holds as a handle
	/// lent from its receiver's: shared, until the receiver's value is borrowed
	/// exclusively, moved out, or freed.
	pub lends: bool,
	/// What it reads: Rust's function, a field it reads or writes, or a constant named as it is.
	pub access: Access,
	/// The instance it is, if a type parameter exports at several.
	pub instance: Option<Instance>,
	/// Where it belongs: a method with its type, unless its instance names a later crate's
	/// type, which it then belongs with.
	pub home: Home,
}

/// An instance of a generic function or method, at one of the types a type parameter
/// exports at.
pub struct Instance {
	/// The arguments of its declared type parameters, which the call names, as Rust
	/// cannot infer one no argument names; its argument infers an `impl Trait` one, and
	/// one left `None`, which takes a lent closure or a foreign implementation.
	pub args: Vec<Option<Type>>,
	/// The arguments that tell it from its other instances, which name it.
	pub named: Vec<Type>,
}

/// What an exported function reads.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Access {
	/// It calls the function.
	Call,
	/// It reads a `pub` field of its receiver, or with `set` assigns it. `shared` when a
	/// method or constant of its type takes its name, which its path then names, so the
	/// config names the field `field@<path>`, as rustdoc does.
	Field { shared: bool, set: bool },
	/// It reads a constant that is not a `bool` or integer.
	Const,
	/// It parses its receiver's type from text, with `FromStr`, failing with the parse
	/// error's message: a foreign type exported as an object.
	Parse,
	/// It writes its receiver as text, with `Display`: a foreign type exported as an object.
	Text,
}

impl Function {
	pub fn name(&self) -> &str {
		self.path.last().expect("a function has a name")
	}
}

/// A parameter a function polls its task through.
pub struct Task {
	/// Its place among the parameters after `self`.
	pub at: usize,
	pub kind: TaskKind,
}

/// What a [`Task`] parameter is.
pub enum TaskKind {
	/// `&Context` or `&mut Context`: made from C's waker, or the future's own.
	Context(Mutability),
	/// A reference to an object made from a `Waker` by its associated function `make`, which
	/// the future makes from its waker at each poll and keeps until the next.
	Made { object: usize, make: String },
}

/// How a method takes `self`.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Receiver {
	/// `&self`
	Ref,
	/// `&mut self`
	Mut,
	/// `self`
	Owned,
}

pub struct Param {
	pub name: String,
	pub ty: Type,
	/// Whether it is a `&str` that must be `'static`, which the glue interns: it leaks each
	/// distinct string once and lends that copy to every later call with equal content.
	pub interned: bool,
}

/// A `pub const`.
pub struct Constant {
	pub path: Path,
	pub home: Home,
	pub span: Span,
	pub docs: String,
	pub value: Literal,
}

pub enum Literal {
	Bool(bool),
	Int(Int, i128),
}

/// A type as the API uses it, in the exact form the Rust side takes or returns.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum Type {
	Unit,
	Bool,
	Int(Int),
	F32,
	F64,
	Char,
	/// `str`, only behind a reference.
	Str,
	String,
	/// `[u8]`, only behind a reference.
	Slice,
	/// `Vec<u8>`
	Vec,
	/// `bytes::Bytes`
	Bytes,
	/// `std::time::Duration`
	Duration,
	/// `std::path::Path`, only behind a reference.
	Path,
	/// `std::path::PathBuf`
	PathBuf,
	/// `std::net::SocketAddr`, or `SocketAddrV4` or `SocketAddrV6` for one family.
	SocketAddr(Family),
	/// A collection of `T`, which C passes and returns as an array of its items: `Vec<T>` of
	/// any `T` but `u8`, which is [`Type::Vec`], and behind a reference, `[T]` too; a set;
	/// or a map, whose `T` is the tuple of a key and its value.
	List(Collection, Box<Type>),
	/// `std::task::Waker`, only as a parameter taken by value.
	Waker,
	Option(Box<Type>),
	/// `Box<T>`, of a non-trait type.
	Box(Box<Type>),
	Ref(Mutability, Box<Type>),
	Record(usize),
	Enum(usize),
	Object(usize),
	/// A foreign implementation of a callback trait, in the form the parameter takes.
	Callback(usize, Dyn),
	/// A closure the callee calls only during the call, as a foreign function: what it
	/// takes, then what it returns.
	Closure(Vec<Type>, Box<Type>),
	/// `std::ops::Range` of an integer.
	Range(Int),
	/// `std::ops::Bound` of an integer.
	Bound(Int),
	/// `(Bound<T>, Bound<T>)` of an integer, the form a `RangeBounds<T>` parameter takes.
	Bounds(Int),
	/// A tuple of one element or more.
	Tuple(Vec<Type>),
}

/// The Rust collection a [`Type::List`] is, which Rust builds from its items in order.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Collection {
	/// `Vec<T>`, or behind a reference, `[T]`.
	Vec,
	/// `std::collections::BTreeSet<T>`
	BTreeSet,
	/// `std::collections::BTreeMap<K, V>`
	BTreeMap,
	/// `serde_json::Map`, of `String` keys and `serde_json::Value`s.
	JsonMap,
}

/// The addresses a socket address type holds.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Family {
	/// IPv4 or IPv6: `SocketAddr`.
	Any,
	V4,
	V6,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Mutability {
	Not,
	Mut,
}

/// How a parameter takes a foreign implementation of a trait.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Dyn {
	/// A type parameter or `impl Trait`, given the foreign type.
	Impl,
	/// `Box<dyn Trait>`
	Box,
	/// `Arc<dyn Trait>`
	Arc,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum Int {
	U8,
	U16,
	U32,
	U64,
	Usize,
	I8,
	I16,
	I32,
	I64,
	Isize,
}

impl Int {
	/// Its Rust name.
	pub fn rust(self) -> &'static str {
		match self {
			Int::U8 => "u8",
			Int::U16 => "u16",
			Int::U32 => "u32",
			Int::U64 => "u64",
			Int::Usize => "usize",
			Int::I8 => "i8",
			Int::I16 => "i16",
			Int::I32 => "i32",
			Int::I64 => "i64",
			Int::Isize => "isize",
		}
	}
}

/// Call `f` on `ty` and on every type within it.
pub fn each(ty: &Type, f: &mut dyn FnMut(&Type)) {
	f(ty);
	match ty {
		Type::List(_, inner) | Type::Option(inner) | Type::Box(inner) | Type::Ref(_, inner) => each(inner, f),
		Type::Closure(params, ret) => {
			for param in params {
				each(param, f);
			}
			each(ret, f);
		}
		Type::Tuple(elements) => {
			for element in elements {
				each(element, f);
			}
		}
		_ => {}
	}
}

impl Type {
	/// Whether a parameter of this type is a value C passes in that Rust writes back
	/// through an out pointer: `&mut` of anything but an object, whose handle it borrows.
	pub fn written(&self) -> bool {
		matches!(self, Type::Ref(Mutability::Mut, inner) if !matches!(**inner, Type::Object(_)))
	}

	/// The type behind any reference or `Box`.
	pub fn base(&self) -> &Type {
		match self {
			Type::Ref(_, inner) | Type::Box(inner) => inner.base(),
			ty => ty,
		}
	}

	/// What the buffer C passes for this type holds, in whatever form Rust takes it: a
	/// string, bytes, or a path, or an `Option` of one, which C passes as a null buffer.
	pub fn buffer(&self) -> Option<Type> {
		match self.base() {
			Type::Str | Type::String => Some(Type::String),
			Type::Slice | Type::Vec | Type::Bytes => Some(Type::Bytes),
			Type::Path | Type::PathBuf => Some(Type::PathBuf),
			Type::Option(inner) => inner.buffer().map(|buffer| Type::Option(Box::new(buffer))),
			_ => None,
		}
	}
}
