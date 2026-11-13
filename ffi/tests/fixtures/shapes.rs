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
	("Bounds", "none: the form of an impl RangeBounds parameter, which no value has"),
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
	let mut inner = || Box::new(sample(rng, leaves, depth - 1));
	match rng.below(10) {
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
pub fn shapes(extra: usize) -> Vec<Shape> {
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
pub fn gate() -> Vec<Shape> {
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
			Shape::PathBuf => ("std::path::PathBuf".into(), "std::path::PathBuf::from(r.string())".into()),
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
				(format!("Option<{ty}>"), format!("match r.below(4) {{ 0 => None, _ => Some({make}) }}"))
			}
			Shape::Vec(inner) => {
				let (ty, make) = self.shape(inner);
				(format!("Vec<{ty}>"), format!("(0..r.below(4)).map(|_| {make}).collect()"))
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
pub enum Item {
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
	pub fn name(self, n: usize) -> String {
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
pub enum Call {
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
	pub fn label(self) -> String {
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
pub struct Crate {
	pub dir: PathBuf,
	/// Each shape's Rust type, by `N`.
	pub types: Vec<String>,
	pub calls: Vec<Call>,
	/// The digests the script prints natively.
	pub transcript: String,
}

/// Write the crate of `shapes` named `name` in the scratch directory, ask the exporter what
/// exports, and run its script natively.
pub fn emit(name: &str, shapes: &[Shape]) -> Crate {
	let dir = scratch().join(name);
	let _ = std::fs::remove_dir_all(dir.join("src"));
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
		#![allow(clippy::all, reason = \"generated\")]\n{RUNTIME}\n{}{functions}",
		emit.items
	);
	let manifest = format!(
		"[package]\nname = \"{name}\"\nversion = \"0.0.0\"\nedition = \"2024\"\npublish = false\n\n\
		[dependencies]\nbytes = \"1\"\nserde_json = \"1\"\nurl = \"2\"\n\n[workspace]\n"
	);
	super::write_all(&dir, &[("Cargo.toml", &manifest), ("src/lib.rs", &lib), ("ffi.toml", "")]);
	// The api fixture's versions, resolved offline.
	std::fs::copy(manifest_dir().join("Cargo.lock"), dir.join("Cargo.lock")).unwrap();

	let refused = refused(&dir);
	let exported = |item: Item, n: usize| !refused.iter().any(|path| *path == item.name(n));
	let mut excludes: Vec<&String> = refused.iter().collect();
	excludes.sort();
	let config = excludes.iter().map(|path| format!("    \"{path}\",\n")).collect::<String>();
	std::fs::write(dir.join("ffi.toml"), format!("exclude = [\n{config}]\n")).unwrap();

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
	let transcript = run(cargo()
		.args(["run", "--quiet", "--example", "transcript"])
		.env("CARGO_TARGET_DIR", dir.join("build"))
		.current_dir(&dir));
	Crate {
		dir,
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

#[test]
fn probe() {
	let krate = emit("shapes", &gate());
	eprintln!("{}", krate.transcript.lines().count());
}
