//! The export's `api.json`, as far as the transcript reads it: the exporter's IR and each
//! language's name for every item. Fields the transcript does not read are skipped, so the
//! IR may grow without touching this; a change to what it reads fails to parse, loudly.

use std::collections::BTreeMap;

/// The export's `api.json`.
#[derive(serde::Deserialize)]
pub(crate) struct Lock {
	/// Each language's name for each item, by the item's key, such as `fn api::divide`.
	pub(crate) names: BTreeMap<String, BTreeMap<String, String>>,
	pub(crate) api: Api,
}

#[derive(serde::Deserialize)]
pub(crate) struct Api {
	pub(crate) records: Vec<Record>,
	pub(crate) enums: Vec<Enum>,
	pub(crate) objects: Vec<Object>,
	pub(crate) callbacks: Vec<Callback>,
	pub(crate) functions: Vec<Function>,
	pub(crate) constants: Vec<Constant>,
	/// The targets the API serves, each a bit of an item's `on`.
	pub(crate) targets: Vec<String>,
}

impl Api {
	/// Whether an item `on` these targets is on every target the API serves.
	pub(crate) fn everywhere(&self, on: u64) -> bool {
		let all = u64::MAX >> (64 - self.targets.len());
		on & all == all
	}

	/// The objects a value of `ty` holds, its records' and enums' fields included.
	pub(crate) fn handles(&self, ty: &Type, out: &mut Vec<usize>, seen: &mut Vec<(bool, usize)>) {
		match ty {
			Type::Object(i) => out.push(*i),
			Type::Option(inner) | Type::Box(inner) | Type::Ref(_, inner) | Type::List(_, inner) => self.handles(inner, out, seen),
			Type::Tuple(elements) => elements.iter().for_each(|ty| self.handles(ty, out, seen)),
			Type::Record(i) if !seen.contains(&(false, *i)) => {
				seen.push((false, *i));
				(self.records[*i].fields.iter()).for_each(|f| self.handles(&f.ty, out, seen));
			}
			Type::Enum(i) if !seen.contains(&(true, *i)) => {
				seen.push((true, *i));
				(self.enums[*i].variants.iter().flat_map(|v| &v.fields)).for_each(|f| self.handles(&f.ty, out, seen));
			}
			_ => {}
		}
	}
}

/// The crate an item belongs to, whose package or module a binding puts it in.
#[derive(serde::Deserialize)]
pub(crate) struct Home {
	pub(crate) krate: String,
}

#[derive(serde::Deserialize)]
pub(crate) struct Record {
	pub(crate) path: Vec<String>,
	pub(crate) home: Home,
	/// The path, with any arguments, the API shows the type at, which its keys name it by.
	#[serde(default)]
	pub(crate) shown: Option<Vec<String>>,
	pub(crate) args: Vec<Type>,
	pub(crate) fields: Vec<Field>,
	pub(crate) tuple: bool,
	pub(crate) methods: Vec<Function>,
	pub(crate) traits: Traits,
	pub(crate) opaque: Option<serde_json::Value>,
	/// The targets it is on.
	pub(crate) on: u64,
}

#[derive(serde::Deserialize)]
pub(crate) struct Field {
	pub(crate) name: String,
	pub(crate) ty: Type,
}

#[derive(serde::Deserialize)]
pub(crate) struct Enum {
	pub(crate) path: Vec<String>,
	pub(crate) home: Home,
	/// The path, with any arguments, the API shows the type at, which its keys name it by.
	#[serde(default)]
	pub(crate) shown: Option<Vec<String>>,
	pub(crate) args: Vec<Type>,
	pub(crate) variants: Vec<Variant>,
	pub(crate) methods: Vec<Function>,
	pub(crate) traits: Traits,
	pub(crate) foreign: bool,
	/// The targets it is on.
	pub(crate) on: u64,
}

#[derive(serde::Deserialize)]
pub(crate) struct Variant {
	pub(crate) name: String,
	pub(crate) fields: Vec<Field>,
	pub(crate) tuple: bool,
}

#[derive(serde::Deserialize)]
pub(crate) struct Object {
	pub(crate) path: Vec<String>,
	pub(crate) home: Home,
	/// The path, with any arguments, the API shows the type at, which its keys name it by.
	#[serde(default)]
	pub(crate) shown: Option<Vec<String>>,
	pub(crate) args: Vec<Type>,
	#[serde(default = "yes")]
	pub(crate) send: bool,
	pub(crate) sync: bool,
	pub(crate) copy: bool,
	pub(crate) shared: bool,
	pub(crate) methods: Vec<Function>,
	pub(crate) traits: Traits,
	pub(crate) foreign: bool,
	/// The targets it is on.
	pub(crate) on: u64,
}

fn yes() -> bool {
	true
}

#[derive(serde::Deserialize)]
pub(crate) struct Callback {
	pub(crate) path: Vec<String>,
	pub(crate) home: Home,
	pub(crate) methods: Vec<Function>,
}

#[derive(Clone, Copy, Default, serde::Deserialize)]
pub(crate) struct Traits {
	pub(crate) clone: bool,
	pub(crate) eq: bool,
	pub(crate) display: bool,
	pub(crate) debug: bool,
	pub(crate) default: bool,
}

#[derive(serde::Deserialize)]
pub(crate) struct Function {
	pub(crate) path: Vec<String>,
	pub(crate) home: Home,
	/// The targets it is on, a bit each of the API's `targets`.
	pub(crate) on: u64,
	pub(crate) receiver: Option<Receiver>,
	pub(crate) params: Vec<Param>,
	pub(crate) ok: Type,
	pub(crate) err: Option<Type>,
	pub(crate) future: bool,
	pub(crate) collect: bool,
	pub(crate) lends: bool,
	#[serde(default)]
	pub(crate) lends_mut: bool,
	pub(crate) access: Access,
	pub(crate) instance: Option<Instance>,
	/// The targets it is on.
	pub(crate) on: u64,
}

#[derive(serde::Deserialize)]
pub(crate) struct Instance {
	pub(crate) named: Vec<Type>,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, serde::Deserialize)]
pub(crate) enum Access {
	Call,
	Field { shared: bool, set: bool },
	Const,
	Parse,
	Text,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, serde::Deserialize)]
pub(crate) enum Receiver {
	Ref,
	Mut,
	Owned,
}

#[derive(serde::Deserialize)]
pub(crate) struct Param {
	pub(crate) name: String,
	pub(crate) ty: Type,
}

#[derive(serde::Deserialize)]
pub(crate) struct Constant {
	pub(crate) path: Vec<String>,
	pub(crate) home: Home,
	pub(crate) value: Literal,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, serde::Deserialize)]
pub(crate) enum Literal {
	Bool(bool),
	Int(Int, i128),
}

/// The IR's `Type`, and a `Result` besides, which the IR keeps apart as a function's `err`.
#[derive(Clone, PartialEq, Eq, Debug, serde::Deserialize)]
pub(crate) enum Type {
	Unit,
	Bool,
	Int(Int),
	NonZero(Int),
	U128,
	F32,
	F64,
	Char,
	Str,
	String,
	Slice,
	Vec,
	Bytes,
	Duration,
	Array(usize),
	Path,
	PathBuf,
	SocketAddr(Family),
	IpAddr(Family),
	List(Collection, Box<Type>),
	Waker,
	Future,
	Reader,
	Jni(serde_json::Value),
	Option(Box<Type>),
	Box(Box<Type>),
	Ref(Mutability, Box<Type>),
	Record(usize),
	Enum(usize),
	Object(usize),
	Callback(usize, Dyn),
	Closure(Box<Closure>),
	Range(Int),
	Bound(Int),
	Bounds(Int),
	Tuple(Vec<Type>),
	Const(Literal),
	/// A call's value or its error: the transcript's own, never the IR's.
	#[serde(skip)]
	Result(Box<Type>, Box<Type>),
}

#[derive(Clone, PartialEq, Eq, Debug, serde::Deserialize)]
pub(crate) struct Closure {
	pub(crate) params: Vec<Type>,
	pub(crate) ok: Type,
	pub(crate) err: Option<Type>,
	pub(crate) kept: bool,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, serde::Deserialize)]
pub(crate) enum Collection {
	Vec,
	Slice,
	Box,
	BTreeSet,
	BTreeMap,
	JsonMap,
	Array(usize),
	Iter,
	IterRef,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, serde::Deserialize)]
pub(crate) enum Family {
	Any,
	V4,
	V6,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, serde::Deserialize)]
pub(crate) enum Mutability {
	Not,
	Mut,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, serde::Deserialize)]
pub(crate) enum Dyn {
	Impl,
	Box,
	Arc,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, serde::Deserialize)]
pub(crate) enum Int {
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
	pub(crate) fn rust(self) -> &'static str {
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

	pub(crate) fn signed(self) -> bool {
		matches!(self, Int::I8 | Int::I16 | Int::I32 | Int::I64 | Int::Isize)
	}

	/// Its width in bits, a pointer's taken as 64.
	pub(crate) fn bits(self) -> u32 {
		match self {
			Int::U8 | Int::I8 => 8,
			Int::U16 | Int::I16 => 16,
			Int::U32 | Int::I32 => 32,
			_ => 64,
		}
	}
}

impl Type {
	/// The type behind any references and boxes.
	pub(crate) fn base(&self) -> &Type {
		match self {
			Type::Ref(_, inner) | Type::Box(inner) => inner.base(),
			ty => ty,
		}
	}
}
