//! Generated type shapes, round-tripped through every language: a seeded sampler builds
//! value types from what the exporter's IR holds, nested to a bounded depth, and emits a
//! crate of `make_N(seed)`, `echo_N(v)`, and `digest_N(&v)` per shape, with the shape in a
//! closure's parameter and return and a future's output besides. The census of that crate
//! says which items export, so a shape the exporter refuses by design fails the export,
//! not the test; each language's driver is a generated flat list of calls whose printed
//! digests must match the crate's own, run natively.

use std::fmt::Write;
use std::path::{Path, PathBuf};
use std::process::Command;

use super::step::{self, Step};
use super::{cargo, manifest_dir, run, scratch};

/// The integer types of the IR's `Int`.
const INTS: [&str; 10] = ["u8", "u16", "u32", "u64", "usize", "i8", "i16", "i32", "i64", "isize"];

/// A value type the sampler builds, after the IR's `Type`.
#[derive(Clone, Debug)]
enum Shape {
	Bool,
	Int(&'static str),
	NonZero(&'static str),
	U128,
	F32,
	F64,
	Char,
	String,
	/// `Vec<u8>`
	Bytes,
	/// `bytes::Bytes`
	Buf,
	Duration,
	Array(usize),
	PathBuf,
	/// `SocketAddr`, `SocketAddrV4`, or `SocketAddrV6`.
	SocketAddr(&'static str),
	Range(&'static str),
	Bound(&'static str),
	/// `serde_json::Value`
	Json,
	/// `serde_json::Map<String, Value>`
	JsonMap,
	/// `url::Url`
	Url,
	Option(Box<Shape>),
	Vec(Box<Shape>),
	Set(Box<Shape>),
	Map(Box<Shape>, Box<Shape>),
	Box(Box<Shape>),
	Tuple(Vec<Shape>),
	/// A struct of `pub` fields: a record.
	Record(Vec<Shape>),
	/// An enum of a unit variant, a tuple variant, and a struct variant.
	Enum(Box<Shape>),
	/// A struct of a private field: an object.
	Object(Box<Shape>),
}

/// Each variant of the IR's `Type` (`src/ir.rs`), and how the shapes cover it: the
/// [`Shape`] that builds it, or why none does. A new variant fails [`ir_covered`] until it
/// is named here.
const COVERED: &[(&str, &str)] = &[
	("Unit", "none: no value, so a function returning it returns nothing"),
	("Bool", "Bool"),
	("Int", "Int, of each width"),
	("NonZero", "NonZero, of each width"),
	("U128", "U128"),
	("F32", "F32"),
	("F64", "F64"),
	("Char", "Char"),
	("Str", "String, which digest_N and peek_N's closure borrow as &str"),
	("String", "String"),
	("Slice", "Bytes, which digest_N and peek_N's closure borrow as &[u8]"),
	("Vec", "Bytes"),
	("Bytes", "Buf"),
	("Duration", "Duration"),
	("Array", "Array"),
	("Path", "PathBuf, which digest_N and peek_N's closure borrow as &Path"),
	("PathBuf", "PathBuf"),
	("SocketAddr", "SocketAddr, of each family"),
	("List", "Vec, Set, Map, and JsonMap"),
	("Waker", "none: a waker holds no value"),
	("Future", "none: a foreign future is of (), which holds no value"),
	("Reader", "none: a foreign reader is lent for a call and holds no value"),
	("Option", "Option"),
	("Box", "Box"),
	("Ref", "every shape, which digest_N and peek_N's closure borrow"),
	("Record", "Record"),
	("Enum", "Enum"),
	("Object", "Object, and Json and Url, which the export maps to objects"),
	("Callback", "none: a foreign trait implementation holds no value"),
	("Closure", "the positions lent_N, peek_N, and made_N"),
	("Range", "Range"),
	("Bound", "Bound"),
	(
		"Bounds",
		"none: the form of an impl RangeBounds parameter, which no value has",
	),
	("Tuple", "Tuple"),
	("Const", "none: a const argument, which no value has"),
];

/// Every leaf, once for each integer type and socket address family.
fn leaves() -> Vec<Shape> {
	let mut leaves = vec![Shape::Bool];
	leaves.extend(INTS.map(Shape::Int));
	leaves.extend(INTS.map(Shape::NonZero));
	leaves.extend([
		Shape::U128,
		Shape::F32,
		Shape::F64,
		Shape::Char,
		Shape::String,
		Shape::Bytes,
		Shape::Buf,
		Shape::Duration,
		Shape::Array(1),
		Shape::Array(16),
		Shape::PathBuf,
		Shape::SocketAddr("SocketAddr"),
		Shape::SocketAddr("SocketAddrV4"),
		Shape::SocketAddr("SocketAddrV6"),
		Shape::Range("u32"),
		Shape::Range("i64"),
		Shape::Bound("u8"),
		Shape::Bound("isize"),
		Shape::Json,
		Shape::JsonMap,
		Shape::Url,
	]);
	leaves
}

/// splitmix64: a fixed seed samples the same shapes and values on every host.
struct Rng(u64);

impl Rng {
	fn next(&mut self) -> u64 {
		self.0 = self.0.wrapping_add(0x9e37_79b9_7f4a_7c15);
		let mut z = self.0;
		z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
		z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
		z ^ (z >> 31)
	}

	fn below(&mut self, n: usize) -> usize {
		(self.next() % n as u64) as usize
	}
}

/// A shape of at most `depth` levels: a leaf, or a wrapper of shapes one level shallower.
fn sample(rng: &mut Rng, leaves: &[Shape], depth: u32) -> Shape {
	if depth == 0 || rng.below(3) == 0 {
		return leaves[rng.below(leaves.len())].clone();
	}
	let wrapper = rng.below(10);
	let mut inner = || Box::new(sample(rng, leaves, depth - 1));
	match wrapper {
		0 => Shape::Option(inner()),
		1 => Shape::Vec(inner()),
		2 => Shape::Box(inner()),
		3 => Shape::Enum(inner()),
		4 => Shape::Object(inner()),
		5 => {
			let element = ordered(rng, leaves);
			Shape::Set(Box::new(element))
		}
		6 => {
			let key = ordered(rng, leaves);
			Shape::Map(Box::new(key), Box::new(sample(rng, leaves, depth - 1)))
		}
		7 => {
			let n = 1 + rng.below(3);
			Shape::Tuple((0..n).map(|_| sample(rng, leaves, depth - 1)).collect())
		}
		_ => {
			let n = 1 + rng.below(3);
			Shape::Record((0..n).map(|_| sample(rng, leaves, depth - 1)).collect())
		}
	}
}

/// A leaf that is `Ord`, to key a set or a map.
fn ordered(rng: &mut Rng, leaves: &[Shape]) -> Shape {
	loop {
		let leaf = &leaves[rng.below(leaves.len())];
		if is_ord(leaf) {
			return leaf.clone();
		}
	}
}

fn is_ord(shape: &Shape) -> bool {
	!matches!(
		shape,
		Shape::F32 | Shape::F64 | Shape::Json | Shape::JsonMap | Shape::Bound(_) | Shape::Range(_)
	)
}

/// The shapes a run covers: each leaf alone, each leaf in each wrapper, then `extra`
/// sampled to depth 3 from a fixed seed.
fn shapes(extra: usize) -> Vec<Shape> {
	let leaves = leaves();
	let mut shapes = leaves.clone();
	let mut rng = Rng(0x5eed);
	for leaf in &leaves {
		let inner = || Box::new(leaf.clone());
		shapes.extend([
			Shape::Option(inner()),
			Shape::Vec(inner()),
			Shape::Box(inner()),
			Shape::Tuple(vec![leaf.clone(), Shape::Bool]),
			Shape::Record(vec![leaf.clone()]),
			Shape::Enum(inner()),
			Shape::Object(inner()),
		]);
		if is_ord(leaf) {
			shapes.push(Shape::Set(inner()));
			shapes.push(Shape::Map(inner(), Box::new(Shape::String)));
		}
	}
	shapes.extend((0..extra).map(|_| sample(&mut rng, &leaves, 3)));
	shapes
}

/// The gate's subset: each leaf alone, and each wrapper around a buffer, a
/// record field, and an object.
fn gate() -> Vec<Shape> {
	let mut shapes = leaves();
	for inner in [Shape::String, Shape::Int("i16"), Shape::Object(Box::new(Shape::U128))] {
		let boxed = || Box::new(inner.clone());
		shapes.extend([
			Shape::Option(boxed()),
			Shape::Vec(boxed()),
			Shape::Box(boxed()),
			Shape::Tuple(vec![inner.clone(), Shape::F64]),
			Shape::Record(vec![inner.clone(), Shape::Char]),
			Shape::Enum(boxed()),
		]);
	}
	shapes.push(Shape::Map(Box::new(Shape::String), Box::new(Shape::Bytes)));
	shapes.push(Shape::Set(Box::new(Shape::Int("u64"))));
	shapes
}

/// The Rust source of the shapes' crate.
struct Emit {
	/// Each record, enum, and object the shapes define.
	items: String,
	/// How many it has defined, which names the next.
	defined: usize,
}

impl Emit {
	/// The Rust type of `shape`, and an expression making one from `r`, a `&mut Rng`.
	fn shape(&mut self, shape: &Shape) -> (String, String) {
		match shape {
			Shape::Bool => ("bool".into(), "r.next() % 2 == 0".into()),
			Shape::Int(int) => (int.to_string(), format!("r.int() as {int}")),
			Shape::NonZero(int) => (
				format!("std::num::NonZero<{int}>"),
				format!("std::num::NonZero::new(r.int() as {int}).unwrap_or(std::num::NonZero::<{int}>::MAX)"),
			),
			Shape::U128 => ("u128".into(), "(r.int() as u128) << 64 | r.int() as u128".into()),
			Shape::F32 => ("f32".into(), "r.f32()".into()),
			Shape::F64 => ("f64".into(), "r.f64()".into()),
			Shape::Char => ("char".into(), "r.char()".into()),
			Shape::String => ("String".into(), "r.string()".into()),
			Shape::Bytes => ("Vec<u8>".into(), "r.bytes()".into()),
			Shape::Buf => ("bytes::Bytes".into(), "bytes::Bytes::from(r.bytes())".into()),
			Shape::Duration => ("std::time::Duration".into(), "r.duration()".into()),
			Shape::Array(n) => (format!("[u8; {n}]"), "std::array::from_fn(|_| r.next() as u8)".into()),
			Shape::PathBuf => (
				"std::path::PathBuf".into(),
				"std::path::PathBuf::from(r.string())".into(),
			),
			Shape::SocketAddr(family) => (format!("std::net::{family}"), format!("r.{}()", family.to_lowercase())),
			Shape::Range(int) => (
				format!("std::ops::Range<{int}>"),
				format!("(r.int() as {int})..(r.int() as {int})"),
			),
			Shape::Bound(int) => (
				format!("std::ops::Bound<{int}>"),
				format!(
					"match r.below(3) {{ 0 => std::ops::Bound::Included(r.int() as {int}), 1 => std::ops::Bound::Excluded(r.int() as {int}), _ => std::ops::Bound::Unbounded }}"
				),
			),
			Shape::Json => ("serde_json::Value".into(), "r.json(2)".into()),
			Shape::JsonMap => (
				"serde_json::Map<String, serde_json::Value>".into(),
				"(0..r.below(3)).map(|_| (r.string(), r.json(1))).collect()".into(),
			),
			Shape::Url => ("url::Url".into(), "r.url()".into()),
			Shape::Option(inner) => {
				let (ty, make) = self.shape(inner);
				(
					format!("Option<{ty}>"),
					format!("match r.below(4) {{ 0 => None, _ => Some({make}) }}"),
				)
			}
			Shape::Vec(inner) => {
				let (ty, make) = self.shape(inner);
				(
					format!("Vec<{ty}>"),
					format!("(0..r.below(4)).map(|_| {make}).collect()"),
				)
			}
			Shape::Set(inner) => {
				let (ty, make) = self.shape(inner);
				(
					format!("std::collections::BTreeSet<{ty}>"),
					format!("(0..r.below(4)).map(|_| {make}).collect()"),
				)
			}
			Shape::Map(key, value) => {
				let (key, make_key) = self.shape(key);
				let (value, make_value) = self.shape(value);
				(
					format!("std::collections::BTreeMap<{key}, {value}>"),
					format!("(0..r.below(4)).map(|_| ({make_key}, {make_value})).collect()"),
				)
			}
			Shape::Box(inner) => {
				let (ty, make) = self.shape(inner);
				(format!("Box<{ty}>"), format!("Box::new({make})"))
			}
			Shape::Tuple(elements) => {
				let (types, makes): (Vec<_>, Vec<_>) = elements.iter().map(|e| self.shape(e)).unzip();
				(format!("({},)", types.join(", ")), format!("({},)", makes.join(", ")))
			}
			Shape::Record(fields) => {
				let (types, makes): (Vec<_>, Vec<_>) = fields.iter().map(|f| self.shape(f)).unzip();
				let name = self.name("Record");
				let _ = writeln!(self.items, "#[derive(Debug)]\npub struct {name} {{");
				for (i, ty) in types.iter().enumerate() {
					let _ = writeln!(self.items, "\tpub f{i}: {ty},");
				}
				let _ = writeln!(self.items, "}}\n");
				let fields: Vec<String> = makes.iter().enumerate().map(|(i, m)| format!("f{i}: {m}")).collect();
				(name.clone(), format!("{name} {{ {} }}", fields.join(", ")))
			}
			Shape::Enum(inner) => {
				let (ty, make) = self.shape(inner);
				let name = self.name("Enum");
				let _ = writeln!(
					self.items,
					"#[derive(Debug)]\npub enum {name} {{\n\tUnit,\n\tTuple({ty}),\n\tStruct {{ value: {ty} }},\n}}\n"
				);
				(
					name.clone(),
					format!(
						"match r.below(3) {{ 0 => {name}::Unit, 1 => {name}::Tuple({make}), _ => {name}::Struct {{ value: {make} }} }}"
					),
				)
			}
			Shape::Object(inner) => {
				let (ty, make) = self.shape(inner);
				let name = self.name("Object");
				let _ = writeln!(self.items, "#[derive(Debug)]\npub struct {name}({ty});\n");
				(name.clone(), format!("{name}({make})"))
			}
		}
	}

	fn name(&mut self, kind: &str) -> String {
		self.defined += 1;
		format!("{kind}{}", self.defined)
	}
}

/// The borrowed form `digest_N` takes of a value of `ty`.
fn borrowed(shape: &Shape, ty: &str) -> String {
	match shape {
		Shape::String => "&str".into(),
		Shape::Bytes => "&[u8]".into(),
		Shape::PathBuf => "&std::path::Path".into(),
		Shape::Vec(_) => format!("&[{}]", &ty[4..ty.len() - 1]),
		_ => format!("&{ty}"),
	}
}

/// What `make_N` draws values from, and `digest`, which every `digest_N` calls.
const RUNTIME: &str = r#"
/// splitmix64, with edge values drawn often.
struct Rng(u64);

impl Rng {
	fn new(seed: u64) -> Self {
		Rng(seed)
	}

	fn next(&mut self) -> u64 {
		self.0 = self.0.wrapping_add(0x9e37_79b9_7f4a_7c15);
		let mut z = self.0;
		z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
		z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
		z ^ (z >> 31)
	}

	fn below(&mut self, n: u64) -> u64 {
		self.next() % n
	}

	/// Bits for an integer of any width, each edge of every width among them.
	fn int(&mut self) -> u64 {
		match self.below(8) {
			0 => 0,
			1 => u64::MAX,
			// The sign bit of a width, and the most a signed one holds.
			2 => 1 << ((8 << self.below(4)) - 1),
			3 => (1 << ((8 << self.below(4)) - 1)) - 1,
			_ => self.next(),
		}
	}

	fn f32(&mut self) -> f32 {
		let f = f32::from_bits(self.int() as u32);
		if f.is_nan() { -0.0 } else { f }
	}

	fn f64(&mut self) -> f64 {
		let f = f64::from_bits(self.int());
		if f.is_nan() { f64::INFINITY } else { f }
	}

	fn char(&mut self) -> char {
		char::from_u32(self.next() as u32 % 0x11_0000).unwrap_or('\u{10ffff}')
	}

	fn string(&mut self) -> String {
		const PARTS: [&str; 6] = ["a", "é", "😀", "\0", "Z", "語"];
		(0..self.below(5)).map(|_| PARTS[self.below(6) as usize]).collect()
	}

	fn bytes(&mut self) -> Vec<u8> {
		(0..self.below(6)).map(|_| self.next() as u8).collect()
	}

	/// Up to 2^33 seconds, past 136 years: Go's `time.Duration` holds 292.
	fn duration(&mut self) -> std::time::Duration {
		std::time::Duration::new(self.next() >> 31, (self.next() % 1_000_000_000) as u32)
	}

	fn socketaddrv4(&mut self) -> std::net::SocketAddrV4 {
		std::net::SocketAddrV4::new((self.next() as u32).into(), self.next() as u16)
	}

	fn socketaddrv6(&mut self) -> std::net::SocketAddrV6 {
		let ip = ((self.next() as u128) << 64 | self.next() as u128).into();
		std::net::SocketAddrV6::new(ip, self.next() as u16, self.next() as u32, self.next() as u32)
	}

	fn socketaddr(&mut self) -> std::net::SocketAddr {
		match self.below(2) {
			0 => self.socketaddrv4().into(),
			_ => self.socketaddrv6().into(),
		}
	}

	fn json(&mut self, depth: u32) -> serde_json::Value {
		match self.below(if depth == 0 { 4 } else { 6 }) {
			0 => serde_json::Value::Null,
			1 => (self.next() % 2 == 0).into(),
			2 => (self.int() as i64).into(),
			3 => self.string().into(),
			4 => (0..self.below(3)).map(|_| self.json(depth - 1)).collect(),
			_ => (0..self.below(3)).map(|_| (self.string(), self.json(depth - 1))).collect(),
		}
	}

	fn url(&mut self) -> url::Url {
		let text = format!("https://h{}.test:{}/p/{}?q={}", self.below(100), self.next() as u16, self.below(10), self.next());
		url::Url::parse(&text).unwrap()
	}
}

/// FNV-1a of the value's `Debug`, which prints every bit of each shape's value.
fn digest(value: &dyn std::fmt::Debug) -> u64 {
	format!("{value:?}").bytes().fold(0xcbf2_9ce4_8422_2325, |h, b| (h ^ b as u64).wrapping_mul(0x100_0000_01b3))
}
"#;

/// The functions each shape `N` exports, which the census may refuse one by one.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Item {
	Make,
	Echo,
	Digest,
	/// `lent_N(seed, f)`: calls `f` with the made value, by value.
	Lent,
	/// `peek_N(seed, f)`: calls `f` with the made value, borrowed as `digest_N` takes it.
	Peek,
	/// `made_N(seed, f)`: digests what `f` returns for the seed.
	Made,
	/// `later_N(v)`: a future of `v`.
	Later,
}

impl Item {
	fn name(self, n: usize) -> String {
		let item = match self {
			Item::Make => "make",
			Item::Echo => "echo",
			Item::Digest => "digest",
			Item::Lent => "lent",
			Item::Peek => "peek",
			Item::Made => "made",
			Item::Later => "later",
		};
		format!("{item}_{n}")
	}
}

/// A line of the script each driver runs, printing `label value`.
#[derive(Clone, Copy, Debug)]
enum Call {
	/// `digest_N(echo_N(make_N(seed)))`
	Echo(usize, u64),
	/// `lent_N(seed, digest_N)`
	Lent(usize, u64),
	/// `peek_N(seed, digest_N)`
	Peek(usize, u64),
	/// `made_N(seed, make_N)`
	Made(usize, u64),
	/// `digest_N(await later_N(make_N(seed)))`
	Later(usize, u64),
}

impl Call {
	/// The `N` of its shape.
	fn shape(self) -> usize {
		let (Call::Echo(n, _) | Call::Lent(n, _) | Call::Peek(n, _) | Call::Made(n, _) | Call::Later(n, _)) = self;
		n
	}

	fn label(self) -> String {
		match self {
			Call::Echo(n, seed) => format!("echo {n} {seed}"),
			Call::Lent(n, seed) => format!("lent {n} {seed}"),
			Call::Peek(n, seed) => format!("peek {n} {seed}"),
			Call::Made(n, seed) => format!("made {n} {seed}"),
			Call::Later(n, seed) => format!("later {n} {seed}"),
		}
	}
}

/// The seeds each shape is made from.
const SEEDS: [u64; 3] = [0, 1, 0xdead_beef];

/// The shapes' crate, as the census left it: which shapes export, and the script.
struct Crate {
	dir: PathBuf,
	shapes: Vec<Shape>,
	/// Each shape's Rust type, by `N`.
	types: Vec<String>,
	calls: Vec<Call>,
	/// The digests the script prints natively.
	transcript: String,
}

/// Write the crate of `shapes` named `name` in the scratch directory, ask the exporter what
/// exports, and run its script natively.
fn emit(name: &str, shapes: &[Shape]) -> Crate {
	let dir = scratch().join(name);
	let mut emit = Emit {
		items: String::new(),
		defined: 0,
	};
	let mut functions = String::new();
	let mut types = Vec::new();
	for (n, shape) in shapes.iter().enumerate() {
		let (ty, make) = emit.shape(shape);
		let lent = borrowed(shape, &ty);
		let _ = write!(
			functions,
			"pub fn make_{n}(seed: u64) -> {ty} {{\n\tlet r = &mut Rng::new(seed);\n\t{make}\n}}\n\n\
			pub fn echo_{n}(v: {ty}) -> {ty} {{\n\tv\n}}\n\n\
			pub fn digest_{n}(v: {lent}) -> u64 {{\n\tdigest(&v)\n}}\n\n\
			pub fn lent_{n}(seed: u64, f: impl FnOnce({ty}) -> u64) -> u64 {{\n\tf(make_{n}(seed))\n}}\n\n\
			pub fn peek_{n}(seed: u64, f: impl FnOnce({lent}) -> u64) -> u64 {{\n\tf(&make_{n}(seed))\n}}\n\n\
			pub fn made_{n}(seed: u64, f: impl FnOnce(u64) -> {ty}) -> u64 {{\n\tdigest(&f(seed))\n}}\n\n\
			pub async fn later_{n}(v: {ty}) -> {ty} {{\n\tv\n}}\n\n"
		);
		types.push(ty);
	}
	let lib = format!(
		"//! Generated by `tests/fixtures/shapes.rs`: each shape's `make`, `echo`, and `digest`.\n\
		#![allow(clippy::all, dead_code, reason = \"generated\")]\n{RUNTIME}\n{}{functions}",
		emit.items
	);
	let manifest = format!(
		"[package]\nname = \"{name}\"\nversion = \"0.0.0\"\nedition = \"2024\"\npublish = false\n\n\
		[dependencies]\nbytes = \"1\"\nserde_json = \"1\"\nurl = \"2\"\n\n[workspace]\n"
	);
	super::write_all(
		&dir,
		&[("Cargo.toml", &manifest), ("src/lib.rs", &lib), ("ffi.toml", "")],
	);
	// The api fixture's versions, resolved offline.
	super::step::copy(&manifest_dir().join("Cargo.lock"), &dir.join("Cargo.lock"));

	let refused = refused(&dir);
	let exported = |item: Item, n: usize| !refused.iter().any(|path| *path == item.name(n));
	let mut excludes: Vec<&String> = refused.iter().collect();
	excludes.sort();
	let config = excludes
		.iter()
		.map(|path| format!("    \"{path}\",\n"))
		.collect::<String>();
	super::step::write(&dir.join("ffi.toml"), format!("exclude = [\n{config}]\n"));

	let mut calls = Vec::new();
	for n in 0..shapes.len() {
		if ![Item::Make, Item::Digest].iter().all(|item| exported(*item, n)) {
			continue;
		}
		for seed in SEEDS {
			if exported(Item::Echo, n) {
				calls.push(Call::Echo(n, seed));
			}
			if exported(Item::Lent, n) {
				calls.push(Call::Lent(n, seed));
			}
			if exported(Item::Peek, n) {
				calls.push(Call::Peek(n, seed));
			}
			if exported(Item::Made, n) {
				calls.push(Call::Made(n, seed));
			}
			if exported(Item::Later, n) {
				calls.push(Call::Later(n, seed));
			}
		}
	}

	let mut script = format!("//! The shapes' script, run natively.\n\nuse {name}::*;\n{NATIVE}");
	for call in &calls {
		let value = match *call {
			Call::Echo(n, seed) => format!("digest_{n}(&echo_{n}(make_{n}({seed})))"),
			Call::Lent(n, seed) => format!("lent_{n}({seed}, |v| digest_{n}(&v))"),
			Call::Peek(n, seed) => format!("peek_{n}({seed}, digest_{n})"),
			Call::Made(n, seed) => format!("made_{n}({seed}, make_{n})"),
			Call::Later(n, seed) => format!("digest_{n}(&later(later_{n}(make_{n}({seed}))))"),
		};
		let _ = writeln!(script, "\tprintln!(\"{} {{}}\", {value});", call.label());
	}
	script.push_str("}\n");
	super::write_all(&dir, &[("examples/transcript.rs", &script)]);
	// Apart from the exporter's build, which cleans the crate each time.
	let transcript = run(cargo()
		.args(["run", "--quiet", "--example", "transcript"])
		.env("CARGO_TARGET_DIR", dir.join("native"))
		.current_dir(&dir));
	Crate {
		dir,
		shapes: shapes.to_vec(),
		types,
		calls,
		transcript: String::from_utf8_lossy(&transcript.stdout).into_owned(),
	}
}

/// The native script's head: `later` polls a future that is ready at once.
const NATIVE: &str = "
fn later<T>(f: impl Future<Output = T>) -> T {
	match std::pin::pin!(f).poll(&mut std::task::Context::from_waker(std::task::Waker::noop())) {
		std::task::Poll::Ready(v) => v,
		std::task::Poll::Pending => unreachable!(),
	}
}

fn main() {
";

/// The paths the census of the crate in `dir` refuses: its config that exports what maps.
fn refused(dir: &Path) -> Vec<String> {
	let census = dir.join("census");
	run(Command::new(env!("CARGO_BIN_EXE_rust2-ffi"))
		.arg("--config")
		.arg(dir.join("ffi.toml"))
		.arg("--census")
		.arg(&census)
		.env("CARGO_TARGET_DIR", dir.join("build")));
	let name = dir.file_name().unwrap().to_string_lossy();
	let report = std::fs::read_to_string(census.join(format!("{name}.md"))).unwrap();
	let Some((_, config)) = report.split_once("## The config that exports what maps") else {
		return Vec::new();
	};
	config
		.lines()
		.skip_while(|line| *line != "exclude = [")
		.skip(1)
		.take_while(|line| *line != "]")
		.map(|line| line.trim().trim_end_matches(',').trim_matches('"').to_string())
		.collect()
}

/// Each variant of the IR's `Type`, from its source.
fn ir_variants() -> Vec<String> {
	let ir = std::fs::read_to_string(manifest_dir().join("src").join("ir.rs")).unwrap();
	let body = ir.split_once("pub enum Type {").expect("ir.rs declares Type").1;
	let body = &body[..body.find("\n}").unwrap()];
	body.lines()
		.map(str::trim)
		.filter(|line| !line.starts_with("//") && !line.is_empty())
		.map(|line| line.split(|c: char| !c.is_alphanumeric()).next().unwrap().to_string())
		.collect()
}

/// Every variant of the IR's `Type` is sampled or named as unsampled, once.
#[test]
fn ir_covered() {
	let variants = ir_variants();
	let covered: Vec<&str> = COVERED.iter().map(|(name, _)| *name).collect();
	assert_eq!(
		variants, covered,
		"COVERED in tests/fixtures/shapes.rs names each variant of the IR's Type, in order"
	);
}

/// The shapes' crate, exported and built once for each language's test.
struct Built {
	krate: Crate,
	/// The export.
	out: PathBuf,
	/// Where cargo built the static library and the shared one Kotlin and Dart load.
	lib: PathBuf,
	/// What a program linking the static library links besides, as rustc prints it.
	native: Vec<String>,
}

/// The shared library of the crate `name`, which Python, Kotlin, and Dart load.
fn shared(name: &str) -> String {
	match (cfg!(windows), cfg!(target_os = "macos")) {
		(true, _) => format!("{name}.dll"),
		(_, true) => format!("lib{name}.dylib"),
		_ => format!("lib{name}.so"),
	}
}

/// Emit, census, export, and build the crate of `shapes` named `name`.
fn build(name: &str, shapes: &[Shape]) -> Built {
	let krate = emit(name, shapes);
	let out = krate.dir.join("out");
	run(Command::new(env!("CARGO_BIN_EXE_rust2-ffi"))
		.arg("--config")
		.arg(krate.dir.join("ffi.toml"))
		.arg("--out")
		.arg(&out)
		.env("CARGO_TARGET_DIR", krate.dir.join("build")));
	let target = krate.dir.join("glue");
	let build = |target: &Path, feature: &str| {
		run(cargo()
			.args(["build", "--quiet", "--locked", "--manifest-path"])
			.arg(out.join("Cargo.toml"))
			.arg("--target-dir")
			.arg(target)
			.args(["--features", feature])
			.env("RUSTFLAGS", "-D warnings --print=native-static-libs"))
	};
	// As the api fixture's: the CPython extension builds apart, in a target directory of its
	// own, and is copied beside the Python.
	let python = krate.dir.join("glue-python");
	build(&python, "python");
	let lib = shared(name);
	super::step::copy(&python.join("debug").join(&lib), &out.join("python").join(&lib));
	let built = build(&target, "kotlin,dart");
	let native = String::from_utf8_lossy(&built.stderr)
		.lines()
		.find_map(|line| line.strip_prefix("note: native-static-libs: "))
		.expect("rustc prints the static library's native libraries")
		.split_whitespace()
		.map(str::to_string)
		.collect();
	Built {
		krate,
		out,
		lib: target.join("debug"),
		native,
	}
}

impl Built {
	fn name(&self) -> String {
		self.krate.dir.file_name().unwrap().to_string_lossy().into_owned()
	}

	fn staticlib(&self) -> PathBuf {
		let name = self.name();
		self.lib.join(match cfg!(target_env = "msvc") {
			true => format!("{name}.lib"),
			false => format!("lib{name}.a"),
		})
	}

	/// A directory of its own for `language`'s driver.
	fn dir(&self, language: &str) -> PathBuf {
		self.krate.dir.join("drivers").join(language)
	}

	/// The exported shapes `language` gets wrong today, by [`KNOWN`], each entry of which must
	/// match one.
	fn known(&self, language: &str) -> Vec<usize> {
		let mut known = Vec::new();
		for Known { what, skips, quest, .. } in KNOWN.iter().filter(|k| k.language == language) {
			let matched: Vec<usize> = (0..self.krate.shapes.len())
				.filter(|&n| skips(&self.krate.shapes[n]) && self.krate.calls.iter().any(|call| call.shape() == n))
				.collect();
			// The gate's subset may hold none of an entry's shapes; the nightly's holds each.
			assert!(
				!matched.is_empty() || self.name() != "sampled",
				"KNOWN's {language} entry for {what} ({quest}) matches no shape: remove it"
			);
			known.extend(matched);
		}
		known
	}

	/// Asserts `language`'s driver printed the native transcript but for the shapes it gets
	/// wrong today, naming each line that differs by its shape's Rust type.
	fn check(&self, language: &str, known: &[usize], output: &std::process::Output) {
		let got = String::from_utf8_lossy(&output.stdout);
		let mut wrong = String::new();
		let expected: Vec<&str> = (self.krate.transcript.lines().zip(&self.krate.calls))
			.filter(|(_, call)| !known.contains(&call.shape()))
			.map(|(line, _)| line)
			.collect();
		let lines: Vec<&str> = got.lines().collect();
		for (i, want) in expected.iter().enumerate() {
			let line = lines.get(i).copied().unwrap_or("<missing>");
			if line != *want {
				let n: usize = want.split(' ').nth(1).unwrap().parse().unwrap();
				let _ = writeln!(wrong, "  {want} ({}), got {line}", self.krate.types[n]);
			}
		}
		assert!(
			wrong.is_empty() && lines.len() == expected.len(),
			"{language}'s round trips differ from native Rust's in {} of {} lines, {} printed:\n{wrong}{}",
			wrong.lines().count(),
			expected.len(),
			lines.len(),
			String::from_utf8_lossy(&output.stderr)
		);
	}
}

/// Whether `shape` or a shape within it meets `f`.
fn within(shape: &Shape, f: &dyn Fn(&Shape) -> bool) -> bool {
	f(shape)
		|| match shape {
			Shape::Option(inner)
			| Shape::Vec(inner)
			| Shape::Set(inner)
			| Shape::Box(inner)
			| Shape::Enum(inner)
			| Shape::Object(inner) => within(inner, f),
			Shape::Map(key, value) => within(key, f) || within(value, f),
			Shape::Tuple(elements) | Shape::Record(elements) => elements.iter().any(|e| within(e, f)),
			_ => false,
		}
}

fn duration(shape: &Shape) -> bool {
	within(shape, &|s| matches!(s, Shape::Duration))
}

/// A `Vec<u8>` or `Bytes`: a `Vec` of `u8` ints is one, as Rust types it alike.
fn is_bytes(shape: &Shape) -> bool {
	match shape {
		Shape::Bytes | Shape::Buf => true,
		Shape::Vec(inner) => matches!(**inner, Shape::Int("u8")),
		_ => false,
	}
}

fn ipv6(shape: &Shape) -> bool {
	within(shape, &|s| {
		matches!(s, Shape::SocketAddr("SocketAddr" | "SocketAddrV6"))
	})
}

/// A collection whose items are bytes or tuples holding them, which C++ takes as views.
fn viewed_items(shape: &Shape) -> bool {
	let buf = |s: &Shape| matches!(s, Shape::String | Shape::PathBuf) || is_bytes(s);
	let item = |s: &Shape| buf(s) || matches!(s, Shape::Tuple(elements) if elements.iter().any(buf));
	within(shape, &|s| match s {
		Shape::Vec(inner) | Shape::Set(inner) => item(inner),
		Shape::Map(key, value) => buf(key) || buf(value),
		_ => false,
	})
}

/// A tuple of one element, which Kotlin returns as its element but takes whole.
fn single(shape: &Shape) -> bool {
	matches!(shape, Shape::Tuple(elements) if elements.len() == 1)
}

/// A round trip a binding gets wrong today.
struct Known {
	language: &'static str,
	what: &'static str,
	/// Whether a shape holds what it gets wrong, which the language's driver skips.
	skips: fn(&Shape) -> bool,
	/// The quest that fixes it.
	quest: &'static str,
}

/// The round trips a binding gets wrong today, each the quest's to fix: the language, what
/// it gets wrong, the shapes it skips for it, and the quest. The nightly fails on an entry
/// matching no exported shape, but its shapes stay skipped after a fix, so the quest that
/// fixes one deletes its entry.
const KNOWN: &[Known] = &[
	Known {
		language: "python",
		what: "a Duration's nanoseconds",
		skips: duration,
		quest: "/quest/m1/ffi-export/lossless.md",
	},
	Known {
		language: "dart",
		what: "a Duration's nanoseconds",
		skips: duration,
		quest: "/quest/m1/ffi-export/lossless.md",
	},
	Known {
		language: "kotlin",
		what: "a Duration's nanoseconds past 146 years",
		skips: duration,
		quest: "/quest/m1/ffi-export/lossless.md",
	},
	Known {
		language: "kotlin",
		what: "an IPv6 flow label",
		skips: ipv6,
		quest: "/quest/m1/ffi-export/lossless.md",
	},
	Known {
		language: "kotlin",
		what: "a returned tuple of one passed back",
		skips: single,
		quest: "/quest/m1/ffi-export/round-trips.md",
	},
	Known {
		language: "go",
		what: "an IPv6 flow label",
		skips: ipv6,
		quest: "/quest/m1/ffi-export/lossless.md",
	},
	Known {
		language: "cpp",
		what: "a returned collection of bytes passed back",
		skips: viewed_items,
		quest: "/quest/m1/ffi-export/round-trips.md",
	},
];

/// A function the C header declares: each parameter's type and name.
struct Proto {
	params: Vec<(String, String)>,
}

/// The prototype of `function` in the C header.
fn prototype(header: &str, function: &str) -> Proto {
	let declared = format!("{function}(");
	let at = header
		.match_indices(&declared)
		.map(|(at, _)| at)
		.find(|&at| header[..at].ends_with([' ', '*']))
		.unwrap_or_else(|| panic!("the header declares {function}"));
	let rest = &header[at + declared.len()..];
	let list = &rest[..rest.find(");").unwrap()];
	// Split at the commas outside a function pointer's parentheses.
	let (mut params, mut depth, mut from) = (Vec::new(), 0, 0);
	for (i, c) in list.char_indices() {
		match c {
			'(' => depth += 1,
			')' => depth -= 1,
			',' if depth == 0 => {
				params.push(&list[from..i]);
				from = i + 1;
			}
			_ => {}
		}
	}
	params.push(&list[from..]);
	let params = params
		.into_iter()
		.map(|param| {
			let param = param.trim();
			match param.find("(*") {
				// A function pointer: `R (*f)(void *data, P)`, named `f`.
				Some(at) => (
					param.to_string(),
					param[at + 2..at + 2 + param[at + 2..].find(')').unwrap()].to_string(),
				),
				None => {
					let split = param.rfind([' ', '*']).unwrap() + 1;
					(param[..split].trim().to_string(), param[split..].to_string())
				}
			}
		})
		.collect();
	Proto { params }
}

impl Proto {
	/// The type of the parameter named `name`.
	fn param(&self, name: &str) -> &str {
		&self.params.iter().find(|(_, n)| n == name).unwrap().0
	}

	/// Statements calling it with `args` that declare `var` of type `ty`: its return, or
	/// what it writes through its out pointers, a returned tuple's gathered into the struct
	/// `ty`. A poll's call must return `ready`.
	fn call(&self, function: &str, args: &[String], ty: &str, var: &str, ready: Option<&str>) -> String {
		let args = args.join(", ");
		let outs = &self.params[self.params.len() - self.outs()..];
		if outs.is_empty() {
			return format!("\t\t{ty} {var} = {function}({args});\n");
		}
		let mut out = String::new();
		let mut pointers = Vec::new();
		let mut values = Vec::new();
		for (k, (pointer, _)) in outs.iter().enumerate() {
			let _ = writeln!(out, "\t\t{} {var}_{k};", pointer.strip_suffix('*').unwrap().trim());
			pointers.push(format!("&{var}_{k}"));
			values.push(format!("{var}_{k}"));
		}
		let call = format!("{function}({args}, {})", pointers.join(", "));
		let _ = match ready {
			Some(ready) => writeln!(out, "\t\tif ({call} != {ready}) abort();"),
			None => writeln!(out, "\t\t{call};"),
		};
		// `out` is the value; `out0` on are a returned tuple's elements, of one or more.
		let _ = match outs[0].1 == "out" {
			true => writeln!(out, "\t\t{ty} {var} = {var}_0;"),
			false => writeln!(out, "\t\t{ty} {var} = ({ty}){{{}}};", values.join(", ")),
		};
		out
	}

	/// How many out pointers it writes its output through: `out`, or `out0` on for a tuple.
	fn outs(&self) -> usize {
		let out = |name: &str| name == "out" || name.strip_prefix("out").is_some_and(|k| k.parse::<usize>().is_ok());
		self.params.iter().filter(|(_, name)| out(name)).count()
	}

	/// What a function pointer parameter `(*name)` returns and takes after its data.
	fn pointer(&self, name: &str) -> (String, String) {
		let ty = self.param(name);
		let (ret, rest) = ty.split_once(&format!(" (*{name})(")).unwrap();
		let param = rest.strip_prefix("void *data, ").unwrap().strip_suffix(')').unwrap();
		(ret.to_string(), param.to_string())
	}
}

/// The C driver: each call's value in a block of its own, typed by the header's prototypes.
fn c(built: &Built, calls: &[Call]) -> String {
	let name = built.name();
	let header = std::fs::read_to_string(built.out.join("include").join(format!("{name}.h"))).unwrap();
	let proto = |item: Item, n: usize| prototype(&header, &format!("{name}_{}", item.name(n)));
	let mut closures = String::new();
	let mut body = String::new();
	let mut defined = std::collections::BTreeSet::new();
	for call in calls {
		let label = call.label();
		let (Call::Echo(n, seed)
		| Call::Lent(n, seed)
		| Call::Peek(n, seed)
		| Call::Made(n, seed)
		| Call::Later(n, seed)) = *call;
		let make = proto(Item::Make, n);
		let digest = proto(Item::Digest, n);
		let digested = digest.param("v").to_string();
		let fname = |item: Item| format!("{name}_{}", item.name(n));
		let mut block = String::new();
		match *call {
			Call::Echo(..) => {
				let echo = proto(Item::Echo, n);
				block += &make.call(&fname(Item::Make), &[format!("{seed}u")], echo.param("v"), "a", None);
				block += &echo.call(&fname(Item::Echo), &["a".into()], &digested, "b", None);
				let _ = writeln!(
					block,
					"\t\tprintf(\"{label} %\" PRIu64 \"\\n\", {}(b));",
					fname(Item::Digest)
				);
			}
			Call::Lent(..) | Call::Peek(..) => {
				let item = if matches!(call, Call::Lent(..)) {
					Item::Lent
				} else {
					Item::Peek
				};
				if defined.insert(item.name(n)) {
					let (_, param) = proto(item, n).pointer("f");
					let _ = writeln!(
						closures,
						"static uint64_t {}(void *data, {param} v) {{\n\t(void)data;\n\treturn {}(v);\n}}\n",
						item.name(n),
						fname(Item::Digest)
					);
				}
				let _ = writeln!(
					block,
					"\t\tprintf(\"{label} %\" PRIu64 \"\\n\", {}({seed}u, {}, NULL));",
					fname(item),
					item.name(n)
				);
			}
			Call::Made(..) => {
				if defined.insert(Item::Made.name(n)) {
					let (ret, _) = proto(Item::Made, n).pointer("f");
					let made = make.call(&fname(Item::Make), &["seed".into()], &ret, "v", None);
					// A buffer C hands Rust is one buf_copy made.
					let value = match ret == format!("{name}_buf") {
						true => format!("{name}_buf_copy(v)"),
						false => "v".into(),
					};
					let _ = writeln!(
						closures,
						"static {ret} {}(void *data, uint64_t seed) {{\n\t(void)data;\n\t{{\n{made}\t\treturn {value};\n\t}}\n}}\n",
						Item::Made.name(n)
					);
				}
				let _ = writeln!(
					block,
					"\t\tprintf(\"{label} %\" PRIu64 \"\\n\", {}({seed}u, {}, NULL));",
					fname(Item::Made),
					Item::Made.name(n)
				);
			}
			Call::Later(..) => {
				let later = proto(Item::Later, n);
				let future = format!("{}_future", fname(Item::Later));
				let poll = prototype(&header, &format!("{}_poll", fname(Item::Later)));
				block += &make.call(&fname(Item::Make), &[format!("{seed}u")], later.param("v"), "a", None);
				let _ = writeln!(block, "\t\t{future} *f = {}(a);", fname(Item::Later));
				// Ready at once: an async fn that awaits nothing.
				let ready = format!("{}_POLL_READY", name.to_uppercase());
				let polled = poll.call(
					&format!("{}_poll", fname(Item::Later)),
					&["f".into(), format!("({name}_waker){{0}}")],
					&digested,
					"b",
					Some(&ready),
				);
				block += &polled;
				let _ = writeln!(block, "\t\t{future}_free(f);");
				let _ = writeln!(
					block,
					"\t\tprintf(\"{label} %\" PRIu64 \"\\n\", {}(b));",
					fname(Item::Digest)
				);
			}
		}
		let _ = writeln!(body, "\t{{\n{block}\t}}");
	}
	format!(
		"#include <inttypes.h>\n#include <stdio.h>\n#include <stdlib.h>\n\n#include \"{name}.h\"\n\n{closures}int main(void) {{\n{body}\treturn 0;\n}}\n"
	)
}

/// The C++ driver, over the header's own types and futures.
fn cpp(built: &Built, calls: &[Call]) -> String {
	let name = built.name();
	let mut body = String::new();
	for call in calls {
		let value = match *call {
			Call::Echo(n, seed) => format!("{name}::digest_{n}({name}::echo_{n}({name}::make_{n}({seed}u)))"),
			Call::Lent(n, seed) => {
				format!("{name}::lent_{n}({seed}u, [](auto v) {{ return {name}::digest_{n}(std::move(v)); }})")
			}
			Call::Peek(n, seed) => {
				format!("{name}::peek_{n}({seed}u, [](auto &&v) {{ return {name}::digest_{n}(v); }})")
			}
			Call::Made(n, seed) => {
				format!("{name}::made_{n}({seed}u, [](std::uint64_t s) {{ return {name}::make_{n}(s); }})")
			}
			Call::Later(n, seed) => format!("{name}::digest_{n}({name}::later_{n}({name}::make_{n}({seed}u)).get())"),
		};
		let _ = writeln!(body, "\tstd::cout << \"{} \" << {value} << '\\n';", call.label());
	}
	format!(
		"#include <cstdint>\n#include <iostream>\n#include <utility>\n\n#include \"{name}.hpp\"\n\nint main() {{\n{body}}}\n"
	)
}

/// Each shape's calls in a function of its own, so no function grows past a compiler's
/// limit: `(n, lines)`.
fn by_shape(calls: &[Call], line: impl Fn(Call) -> String) -> Vec<(usize, String)> {
	let mut shapes: Vec<(usize, String)> = Vec::new();
	for call in calls {
		let (Call::Echo(n, _) | Call::Lent(n, _) | Call::Peek(n, _) | Call::Made(n, _) | Call::Later(n, _)) = *call;
		if shapes.last().is_none_or(|(last, _)| *last != n) {
			shapes.push((n, String::new()));
		}
		let _ = writeln!(shapes.last_mut().unwrap().1, "{}", line(*call));
	}
	shapes
}

/// The Kotlin driver, on the JVM with kotlinx-coroutines.
fn kotlin(built: &Built, calls: &[Call]) -> String {
	let shapes = by_shape(calls, |call| {
		let value = match call {
			Call::Echo(n, seed) => format!("digest{n}(echo{n}(make{n}({seed}uL)))"),
			Call::Lent(n, seed) => format!("lent{n}({seed}uL) {{ digest{n}(it) }}"),
			Call::Peek(n, seed) => format!("peek{n}({seed}uL) {{ digest{n}(it) }}"),
			Call::Made(n, seed) => format!("made{n}({seed}uL) {{ make{n}(it) }}"),
			Call::Later(n, seed) => format!("digest{n}(later{n}(make{n}({seed}uL)))"),
		};
		format!("\tprintln(\"{} ${{{value}}}\")", call.label())
	});
	// A block of unsigned numbers is an experimental unsigned array, such as `UIntArray`.
	let mut out = format!(
		"@file:OptIn(ExperimentalUnsignedTypes::class)\n\nimport {}.*\nimport kotlinx.coroutines.runBlocking\n\n",
		built.name()
	);
	for (n, lines) in &shapes {
		let _ = writeln!(out, "suspend fun shape{n}() {{\n{lines}}}\n");
	}
	out.push_str("fun main() = runBlocking {\n");
	for (n, _) in &shapes {
		let _ = writeln!(out, "\tshape{n}()");
	}
	out.push_str("}\n");
	out
}

/// The Python driver, awaiting futures on asyncio.
fn python(built: &Built, calls: &[Call]) -> String {
	let name = built.name();
	let mut body = String::new();
	for call in calls {
		let value = match *call {
			Call::Echo(n, seed) => format!("s.digest_{n}(s.echo_{n}(s.make_{n}({seed})))"),
			Call::Lent(n, seed) => format!("s.lent_{n}({seed}, s.digest_{n})"),
			Call::Peek(n, seed) => format!("s.peek_{n}({seed}, s.digest_{n})"),
			Call::Made(n, seed) => format!("s.made_{n}({seed}, s.make_{n})"),
			Call::Later(n, seed) => format!("s.digest_{n}(await s.later_{n}(s.make_{n}({seed})))"),
		};
		let _ = writeln!(body, "\tprint(f\"{} {{{value}}}\")", call.label());
	}
	format!("import asyncio\n\nimport {name} as s\n\n\nasync def main() -> None:\n{body}\n\nasyncio.run(main())\n")
}

/// The Swift driver, in the generated module's `main.swift`: each shape's calls in a
/// function of its own, since type-checking them as one body of top-level code takes
/// tens of gigabytes.
fn swift(calls: &[Call]) -> String {
	let shapes = by_shape(calls, |call| {
		let value = match call {
			Call::Echo(n, seed) => format!("digest{n}(echo{n}(make{n}({seed})))"),
			Call::Lent(n, seed) => format!("lent{n}({seed}) {{ digest{n}($0) }}"),
			Call::Peek(n, seed) => format!("peek{n}({seed}) {{ digest{n}($0) }}"),
			Call::Made(n, seed) => format!("made{n}({seed}) {{ make{n}($0) }}"),
			Call::Later(n, seed) => format!("digest{n}(try await later{n}(make{n}({seed})))"),
		};
		format!("\tprint(\"{} \\({value})\")", call.label())
	});
	let mut out = String::new();
	for (n, lines) in &shapes {
		let _ = writeln!(out, "func shape{n}() async throws {{\n{lines}}}\n");
	}
	for (n, _) in &shapes {
		let _ = writeln!(out, "try await shape{n}()");
	}
	out
}

/// The Go driver: `one` takes the value of a call returning it alone or with an error,
/// `tupleK` gathers a returned tuple's values into the `rt.TupleK` Go takes, `lift` makes a
/// closure of a function of either form, and `bytesIn` and `bytesOptIn` adapt one taking an
/// `rt.Bytes` to the `[]byte` a callback is lent.
fn go(built: &Built, calls: &[Call], source: &str, rt: &str) -> String {
	let mut body = String::new();
	// How the generated `func` returns: its values, and whether an error follows them.
	let results = |function: &str| -> (usize, bool) {
		let at = source
			.find(&format!("\nfunc {function}("))
			.unwrap_or_else(|| panic!("the Go declares {function}"));
		let line = source[at + 1..].lines().next().unwrap();
		let mut depth = 0;
		let mut close = 0;
		for (i, c) in line.char_indices().skip(line.find('(').unwrap()) {
			match c {
				'(' | '[' => depth += 1,
				')' | ']' => depth -= 1,
				_ => {}
			}
			if depth == 0 {
				close = i;
				break;
			}
		}
		let result = line[close + 1..].trim().trim_end_matches('{').trim();
		let Some(list) = result.strip_prefix('(').and_then(|r| r.strip_suffix(')')) else {
			return (1, false);
		};
		let (mut types, mut depth) = (vec![String::new()], 0);
		for c in list.chars() {
			match c {
				'(' | '[' => depth += 1,
				')' | ']' => depth -= 1,
				',' if depth == 0 => {
					types.push(String::new());
					continue;
				}
				_ => {}
			}
			types.last_mut().unwrap().push(c);
		}
		let error = types.last().unwrap().trim() == "error";
		(types.len() - usize::from(error), error)
	};
	// The value of a call of `function`, in the form a parameter takes it.
	let value = |function: &str, call: String| match results(function) {
		(1, _) => format!("one({call})"),
		(k, false) => format!("tuple{k}({call})"),
		(k, true) => format!("tuple{k}e({call})"),
	};
	// A gatherer for each arity of tuple the runtime declares, which is each the API uses.
	let mut tuples = String::new();
	for k in 2..=9 {
		if !rt.contains(&format!("type Tuple{k}[")) {
			continue;
		}
		let params: Vec<String> = (0..k).map(|i| format!("T{i}")).collect();
		let types = params.join(", ");
		let args: Vec<String> = (0..k).map(|i| format!("v{i} T{i}")).collect();
		let fields: Vec<String> = (0..k).map(|i| format!("V{i}: v{i}")).collect();
		let values: Vec<String> = (0..k).map(|i| format!("v{i}")).collect();
		let _ = write!(
			tuples,
			"func tuple{k}[{types} any]({args}) rt.Tuple{k}[{types}] {{\n\treturn rt.Tuple{k}[{types}]{{{fields}}}\n}}\n\n\
			func tuple{k}e[{types} any]({args}, err error) rt.Tuple{k}[{types}] {{\n\treturn one(tuple{k}({values}), err)\n}}\n\n",
			args = args.join(", "),
			fields = fields.join(", "),
			values = values.join(", "),
		);
	}
	for call in calls {
		let n = call.shape();
		let printed = match *call {
			Call::Echo(n, seed) => {
				let made = value(&format!("Make{n}"), format!("s.Make{n}({seed})"));
				let echoed = value(&format!("Echo{n}"), format!("s.Echo{n}({made})"));
				format!("one(s.Digest{n}({echoed}))")
			}
			Call::Lent(_, seed) | Call::Peek(_, seed) => {
				let item = if matches!(call, Call::Lent(..)) { "Lent" } else { "Peek" };
				let lift = if results(&format!("Digest{n}")).1 {
					"liftErr"
				} else {
					"lift"
				};
				// A callback takes the bytes Rust lends it as a `[]byte`, which `Digest` takes
				// as an `rt.Bytes`.
				let at = source.find(&format!("\nfunc {item}{n}(")).unwrap();
				let line = source[at + 1..].lines().next().unwrap();
				let adapt = match () {
					_ if line.contains("func([]byte)") => "bytesIn",
					_ if line.contains("func(*[]byte)") => "bytesOptIn",
					_ => "",
				};
				format!("s.{item}{n}({seed}, {adapt}({lift}(s.Digest{n})))")
			}
			Call::Made(_, seed) => {
				let lift = if results(&format!("Make{n}")).1 {
					"liftErr"
				} else {
					"lift"
				};
				format!("s.Made{n}({seed}, {lift}(s.Make{n}))")
			}
			Call::Later(_, seed) => {
				let made = value(&format!("Make{n}"), format!("s.Make{n}({seed})"));
				let later = value(&format!("Later{n}"), format!("s.Later{n}(ctx, {made})"));
				format!("one(s.Digest{n}({later}))")
			}
		};
		let _ = writeln!(body, "\tfmt.Println(\"{}\", {printed})", call.label());
	}
	format!(
		"package main\n\nimport (\n\t\"context\"\n\t\"fmt\"\n\n\ts \"{name}/{name}\"\n\t\"{name}/rt\"\n)\n\n\
		func one[T any](v T, err ...error) T {{\n\tif len(err) > 0 && err[0] != nil {{\n\t\tpanic(err[0])\n\t}}\n\treturn v\n}}\n\n\
		{tuples}func lift[A, B any](f func(A) B) func(A) B {{\n\treturn f\n}}\n\n\
		func liftErr[A, B any](f func(A) (B, error)) func(A) B {{\n\treturn func(a A) B {{ return one(f(a)) }}\n}}\n\n\
		func bytesIn[B any](f func(rt.Bytes) B) func([]byte) B {{\n\treturn func(v []byte) B {{ return f(rt.NewBytes(v)) }}\n}}\n\n\
		func bytesOptIn[B any](f func(*rt.Bytes) B) func(*[]byte) B {{\n\treturn func(v *[]byte) B {{\n\t\tif v == nil {{\n\t\t\treturn f(nil)\n\t\t}}\n\t\tb := rt.NewBytes(*v)\n\t\treturn f(&b)\n\t}}\n}}\n\n\
		func main() {{\n\tctx := context.Background()\n\t_ = ctx\n{body}}}\n",
		name = built.name(),
	)
}

/// The Dart driver; Dart's `int` is Rust's `u64` in two's complement, printed unsigned.
fn dart(built: &Built, calls: &[Call]) -> String {
	let name = built.name();
	let mut body = String::new();
	for call in calls {
		let value = match *call {
			Call::Echo(n, seed) => format!("s.digest{n}(s.echo{n}(s.make{n}({seed})))"),
			Call::Lent(n, seed) => format!("s.lent{n}({seed}, s.digest{n})"),
			Call::Peek(n, seed) => format!("s.peek{n}({seed}, s.digest{n})"),
			Call::Made(n, seed) => format!("s.made{n}({seed}, s.make{n})"),
			Call::Later(n, seed) => format!("s.digest{n}(await s.later{n}(s.make{n}({seed})))"),
		};
		let _ = writeln!(body, "  print('{} ${{u({value})}}');", call.label());
	}
	format!(
		"import 'package:{name}/{name}.dart' as s;\n\nString u(int v) => BigInt.from(v).toUnsigned(64).toString();\n\n\
		Future<void> main() async {{\n{body}}}\n"
	)
}

/// Build and run `language`'s driver of `built`, checking it against the native transcript.
fn round_trip(built: &Built, language: &str) {
	let name = built.name();
	let dir = built.dir(language);
	let exe = dir.join(format!("shapes{}", std::env::consts::EXE_SUFFIX));
	let known = built.known(language);
	let calls: Vec<Call> = (built.krate.calls.iter().copied())
		.filter(|call| !known.contains(&call.shape()))
		.collect();
	let calls = &calls[..];
	let output = match language {
		"c" => {
			let main = dir.join("main.c");
			step::write(&main, c(built, calls));
			let cc = std::env::var_os("CC").unwrap_or_else(|| "cc".into());
			let include = built.out.join("include");
			let step = Step {
				stamp: &step::stamp(&exe),
				reads: &[&main, &include, &built.staticlib()],
				writes: &[&exe],
			};
			step.run(
				Command::new(cc)
					.args(["-std=c23", "-pedantic", "-Wall", "-Wextra", "-Werror", "-I"])
					.arg(&include)
					.arg(&main)
					.arg(built.staticlib())
					.args(&built.native)
					.arg("-o")
					.arg(&exe),
			);
			run(&mut Command::new(&exe))
		}
		"cpp" => {
			let main = dir.join("main.cpp");
			step::write(&main, cpp(built, calls));
			// The api fixture's build, so MSVC's runner builds it with cl.
			let link = super::Link {
				include: built.out.join("include"),
				staticlib: built.staticlib(),
				native: &built.native,
			};
			super::cxx(&link, 17, false, &main, &exe);
			run(&mut Command::new(&exe))
		}
		"kotlin" => {
			let main = dir.join("main.kt");
			step::write(&main, kotlin(built, calls));
			let classes = dir.join("classes");
			let generated = built.out.join("kotlin").join(format!("{name}.kt"));
			let jar = PathBuf::from(super::coroutines());
			let step = Step {
				stamp: &step::stamp(&classes),
				reads: &[&main, &generated, &jar],
				writes: &[&classes],
			};
			step.run(
				Command::new("kotlinc")
					.args(["-Werror", "-cp"])
					.arg(&jar)
					.arg(&generated)
					.arg(&main)
					.arg("-d")
					.arg(&classes),
			);
			let mut classpath = classes.into_os_string();
			classpath.push(if cfg!(windows) { ";" } else { ":" });
			classpath.push(super::coroutines());
			run(Command::new("kotlin")
				.arg(format!("-Djava.library.path={}", built.lib.display()))
				.arg("-cp")
				.arg(&classpath)
				.arg("MainKt"))
		}
		"python" => {
			let main = dir.join("main.py");
			step::write(&main, python(built, calls));
			run(
				Command::new(std::env::var_os("PYTHON").unwrap_or_else(|| "python3".into()))
					.args(["-P", "-X", "dev", "-W", "error"])
					.arg(&main)
					.env("PYTHONPATH", built.out.join("python")),
			)
		}
		"swift" => {
			let module = built.out.join("swift");
			let (generated, main) = (dir.join(format!("{name}.swift")), dir.join("main.swift"));
			step::copy(&module.join(format!("{name}.swift")), &generated);
			step::write(&main, swift(calls));
			let collections = super::swift_collections();
			let include = built.out.join("include");
			let step = Step {
				stamp: &step::stamp(&exe),
				reads: &[
					&generated,
					&main,
					&module,
					&include,
					&super::collections_dir(),
					&built.staticlib(),
				],
				writes: &[&exe],
			};
			step.run(
				super::swiftc(&module, 15)
					.args(["-swift-version", "6", "-module-name", "Shapes"])
					.arg(&generated)
					.arg(&main)
					.args(collections)
					.arg(built.staticlib())
					.args(&built.native)
					.arg("-o")
					.arg(&exe),
			);
			run(&mut Command::new(&exe))
		}
		"go" => {
			// The module apart from the program, which its build would otherwise read.
			let module = dir.join("module");
			let export = module.join("export");
			step::mirror(&built.out.join("go"), &export);
			let header = format!("{name}.h");
			step::copy(
				&built.out.join("include").join(&header),
				&module.join("include").join(&header),
			);
			let source = std::fs::read_to_string(export.join(&name).join(format!("{name}.go"))).unwrap();
			let rt = std::fs::read_to_string(export.join("rt").join("types.go")).unwrap_or_default();
			step::write(&module.join("main.go"), go(built, calls, &source, &rt));
			step::write(
				&module.join("go.mod"),
				format!("module shapestest\n\ngo 1.24\n\nrequire {name} v0.0.0\n\nreplace {name} => ./export\n"),
			);
			let mut ldflags = built.staticlib().into_os_string();
			for lib in &built.native {
				ldflags.push(" ");
				ldflags.push(lib);
			}
			let step = Step {
				stamp: &step::stamp(&exe),
				reads: &[&module, &built.staticlib()],
				writes: &[&exe],
			};
			step.run(
				Command::new("go")
					.args(["build", "-o"])
					.arg(&exe)
					.current_dir(&module)
					.env("CGO_ENABLED", "1")
					.env("CGO_LDFLAGS", ldflags)
					.env("GOFLAGS", "-buildvcs=false")
					.env("GOTOOLCHAIN", "local"),
			);
			run(&mut Command::new(&exe))
		}
		"dart" => {
			let main = dir.join("main.dart");
			step::write(&main, dart(built, calls));
			let lib = built.lib.join(shared(&name));
			let dill = super::dart_compile(&name, &dir, &built.out.join("dart"), &lib, &main);
			run(&mut super::dart_run(&dill, &[]))
		}
		_ => unreachable!("no driver for {language}"),
	};
	built.check(language, &known, &output);
}

/// The gate's subset of shapes, built once.
fn gated() -> &'static Built {
	static BUILT: std::sync::OnceLock<Built> = std::sync::OnceLock::new();
	BUILT.get_or_init(|| build("shapes", &gate()))
}

/// Every sampled shape, built once: the nightly's.
fn sampled() -> &'static Built {
	static BUILT: std::sync::OnceLock<Built> = std::sync::OnceLock::new();
	BUILT.get_or_init(|| build("sampled", &shapes(SAMPLES)))
}

/// How many shapes the nightly samples beyond each leaf in each wrapper.
const SAMPLES: usize = 64;

macro_rules! languages {
	($($language:ident),*) => {
		mod gate {
			$(
				#[test]
				fn $language() {
					super::round_trip(super::gated(), stringify!($language));
				}
			)*
		}

		mod nightly {
			$(
				#[test]
				#[ignore = "the nightly's: every sampled shape, just ffi shapes"]
				fn $language() {
					super::round_trip(super::sampled(), stringify!($language));
				}
			)*
		}
	};
}

languages!(c, cpp, kotlin, python, swift, go, dart);
