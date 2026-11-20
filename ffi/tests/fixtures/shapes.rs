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
	/// `IpAddr`, `Ipv4Addr`, or `Ipv6Addr`.
	IpAddr(&'static str),
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
	("IpAddr", "IpAddr, of each family"),
	("List", "Vec, Set, Map, and JsonMap"),
	("Waker", "none: a waker holds no value"),
	("Future", "none: a foreign future is of (), which holds no value"),
	("Reader", "none: a foreign reader is lent for a call and holds no value"),
	("Jni", "none: lent for a Kotlin call, which api.kt passes"),
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

/// Every leaf, once for each integer type and address family.
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
		Shape::IpAddr("IpAddr"),
		Shape::IpAddr("Ipv4Addr"),
		Shape::IpAddr("Ipv6Addr"),
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

/// The fixed set [`KNOWN`] checks: each leaf alone, each leaf in each wrapper, then `extra`
/// sampled to depth 3 from a fixed seed.
fn shapes(extra: usize) -> Vec<Shape> {
	let leaves = leaves();
	let mut shapes = leaves.clone();
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
	shapes.extend(drawn(0x5eed, extra));
	shapes
}

/// `n` shapes sampled to depth 3 from `seed`.
fn drawn(seed: u64, n: usize) -> Vec<Shape> {
	let leaves = leaves();
	let mut rng = Rng(seed);
	(0..n).map(|_| sample(&mut rng, &leaves, 3)).collect()
}

/// The seed the nightly draws its new shapes from: `RUST2_SHAPES_SEED`, to reproduce a run,
/// else the day's UTC date as the digits `YYYYMMDD`, so each night explores new shapes.
fn day_seed() -> u64 {
	if let Some(seed) = std::env::var_os("RUST2_SHAPES_SEED") {
		let seed = seed.to_str().and_then(|s| s.parse().ok());
		return seed.expect("RUST2_SHAPES_SEED is a u64");
	}
	let secs = std::time::SystemTime::now()
		.duration_since(std::time::UNIX_EPOCH)
		.expect("the clock is past 1970")
		.as_secs();
	let (year, month, day) = civil((secs / 86_400) as i64);
	(year * 10_000 + month * 100 + day) as u64
}

/// The proleptic Gregorian date of `days` since 1970-01-01, after Howard Hinnant's
/// `civil_from_days`.
fn civil(days: i64) -> (i64, i64, i64) {
	let z = days + 719_468;
	let era = z.div_euclid(146_097);
	let doe = z.rem_euclid(146_097);
	let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
	let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
	let mp = (5 * doy + 2) / 153;
	let day = doy - (153 * mp + 2) / 5 + 1;
	let month = if mp < 10 { mp + 3 } else { mp - 9 };
	(yoe + era * 400 + i64::from(month <= 2), month, day)
}

#[test]
fn civil_dates() {
	assert_eq!(civil(0), (1970, 1, 1));
	assert_eq!(civil(-1), (1969, 12, 31));
	// A leap day, and the day after one a century skips.
	assert_eq!(civil(11_016), (2000, 2, 29));
	assert_eq!(civil(47_541), (2100, 3, 1));
	assert_eq!(civil(20_776), (2026, 11, 19));
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
			Shape::PathBuf => ("std::path::PathBuf".into(), "r.path()".into()),
			Shape::SocketAddr(family) | Shape::IpAddr(family) => {
				(format!("std::net::{family}"), format!("r.{}()", family.to_lowercase()))
			}
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
				let fed: String = (0..types.len()).map(|i| format!(" self.f{i}.feed(h);")).collect();
				let _ = writeln!(
					self.items,
					"impl Feed for {name} {{\n\tfn feed(&self, h: &mut Fnv) {{\n\t\th.put(\"{name}{{\");{fed}\n\t\th.put(\"}}\");\n\t}}\n}}\n"
				);
				let fields: Vec<String> = makes.iter().enumerate().map(|(i, m)| format!("f{i}: {m}")).collect();
				(name.clone(), format!("{name} {{ {} }}", fields.join(", ")))
			}
			Shape::Enum(inner) => {
				let (ty, make) = self.shape(inner);
				let name = self.name("Enum");
				let _ = writeln!(
					self.items,
					"#[derive(Debug)]\npub enum {name} {{\n\tUnit,\n\tTuple({ty}),\n\tStruct {{ value: {ty} }},\n}}\n\n\
					impl Feed for {name} {{\n\tfn feed(&self, h: &mut Fnv) {{\n\t\tmatch self {{\n\t\t\tSelf::Unit => h.put(\"Unit;\"),\n\
					\t\t\tSelf::Tuple(v) => {{\n\t\t\t\th.put(\"Tuple(\");\n\t\t\t\tv.feed(h);\n\t\t\t\th.put(\")\");\n\t\t\t}}\n\
					\t\t\tSelf::Struct {{ value }} => {{\n\t\t\t\th.put(\"Struct{{\");\n\t\t\t\tvalue.feed(h);\n\t\t\t\th.put(\"}}\");\n\t\t\t}}\n\
					\t\t}}\n\t}}\n}}\n"
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
				let _ = writeln!(
					self.items,
					"#[derive(Debug)]\npub struct {name}({ty});\n\n\
					impl Feed for {name} {{\n\tfn feed(&self, h: &mut Fnv) {{\n\t\th.put(\"{name}(\");\n\t\tself.0.feed(h);\n\t\th.put(\")\");\n\t}}\n}}\n"
				);
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

	/// Any bits, a NaN's payload and sign among them: a signaling NaN, which a load that
	/// quiets it would change, and a quiet one. The gate's seed 1 draws the first, and the
	/// other seed `swap`, `pair`, and `held` make of 0xdead_beef the second.
	fn f32(&mut self) -> f32 {
		let (class, bits) = (self.below(4), self.next() as u32);
		match class {
			1 => f32::from_bits(bits & 1 << 31 | 0x7f80_0000 | (1 + bits % 0x3f_ffff)),
			2 => f32::from_bits(bits | 0x7fc0_0000),
			_ => f32::from_bits(self.int() as u32),
		}
	}

	fn f64(&mut self) -> f64 {
		let (class, bits) = (self.below(4), self.next());
		match class {
			1 => f64::from_bits(bits & 1 << 63 | 0x7ff0_0000_0000_0000 | (1 + bits % 0x7_ffff_ffff_ffff)),
			2 => f64::from_bits(bits | 0x7ff8_0000_0000_0000),
			_ => f64::from_bits(self.int()),
		}
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

	/// Any, the longest and those past what a nanosecond or millisecond `i64` holds
	/// among them.
	fn duration(&mut self) -> std::time::Duration {
		let nanos = match self.below(4) {
			0 => 0,
			1 => 999_999_999,
			_ => (self.next() % 1_000_000_000) as u32,
		};
		std::time::Duration::new(self.int(), nanos)
	}

	/// Text, or on Unix bytes that are not UTF-8, as a path there may hold.
	fn path(&mut self) -> std::path::PathBuf {
		#[cfg(unix)]
		if self.below(3) == 0 {
			use std::os::unix::ffi::OsStringExt;
			let mut bytes = self.string().into_bytes();
			bytes.push(0x80 | self.next() as u8 & 0x3f);
			return std::ffi::OsString::from_vec(bytes).into();
		}
		self.string().into()
	}

	fn socketaddrv4(&mut self) -> std::net::SocketAddrV4 {
		std::net::SocketAddrV4::new(self.ipv4addr(), self.next() as u16)
	}

	fn socketaddrv6(&mut self) -> std::net::SocketAddrV6 {
		std::net::SocketAddrV6::new(self.ipv6addr(), self.next() as u16, self.next() as u32, self.next() as u32)
	}

	fn ipv4addr(&mut self) -> std::net::Ipv4Addr {
		(self.next() as u32).into()
	}

	fn ipv6addr(&mut self) -> std::net::Ipv6Addr {
		((self.next() as u128) << 64 | self.next() as u128).into()
	}

	fn ipaddr(&mut self) -> std::net::IpAddr {
		match self.below(2) {
			0 => self.ipv4addr().into(),
			_ => self.ipv6addr().into(),
		}
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

/// FNV-1a of what a value feeds it.
struct Fnv(u64);

impl Fnv {
	fn put(&mut self, text: &str) {
		for b in text.bytes() {
			self.0 = (self.0 ^ b as u64).wrapping_mul(0x100_0000_01b3);
		}
	}
}

impl std::fmt::Write for Fnv {
	fn write_str(&mut self, text: &str) -> std::fmt::Result {
		self.put(text);
		Ok(())
	}
}

/// Every bit of a value: `Debug`'s, but a float's bits, since `Debug` prints any NaN alike.
trait Feed {
	fn feed(&self, h: &mut Fnv);
}

macro_rules! debug {
	($($ty:ty),*) => {$(
		impl Feed for $ty {
			fn feed(&self, h: &mut Fnv) {
				use std::fmt::Write;
				let _ = write!(h, "{self:?};");
			}
		}
	)*};
}

debug!(
	bool, u8, u16, u32, u64, usize, i8, i16, i32, i64, isize, u128, char, str, String,
	std::num::NonZero<u8>, std::num::NonZero<u16>, std::num::NonZero<u32>, std::num::NonZero<u64>,
	std::num::NonZero<usize>, std::num::NonZero<i8>, std::num::NonZero<i16>, std::num::NonZero<i32>,
	std::num::NonZero<i64>, std::num::NonZero<isize>, bytes::Bytes, std::time::Duration, std::path::Path,
	std::path::PathBuf, std::net::SocketAddr, std::net::SocketAddrV4, std::net::SocketAddrV6, std::net::IpAddr,
	std::net::Ipv4Addr, std::net::Ipv6Addr, std::ops::Range<u32>, std::ops::Range<i64>, std::ops::Bound<u8>,
	std::ops::Bound<isize>, serde_json::Value, serde_json::Map<String, serde_json::Value>, url::Url
);

impl Feed for f32 {
	fn feed(&self, h: &mut Fnv) {
		use std::fmt::Write;
		let _ = write!(h, "f32 {:#x};", self.to_bits());
		// The transcript marks a call that digests a NaN other than the one a float's
		// arithmetic makes: signaling, its quiet bit clear, or quiet with a payload or sign.
		#[cfg(feature = "transcript")]
		if self.is_nan() && self.to_bits() != f32::NAN.to_bits() {
			let class = if self.to_bits() & 1 << 22 == 0 { "signaling" } else { "quiet" };
			println!("nan {class} f32");
		}
	}
}

impl Feed for f64 {
	fn feed(&self, h: &mut Fnv) {
		use std::fmt::Write;
		let _ = write!(h, "f64 {:#x};", self.to_bits());
		#[cfg(feature = "transcript")]
		if self.is_nan() && self.to_bits() != f64::NAN.to_bits() {
			let class = if self.to_bits() & 1 << 51 == 0 { "signaling" } else { "quiet" };
			println!("nan {class} f64");
		}
	}
}

impl<T: Feed + ?Sized> Feed for &T {
	fn feed(&self, h: &mut Fnv) {
		(**self).feed(h);
	}
}

impl<T: Feed + ?Sized> Feed for Box<T> {
	fn feed(&self, h: &mut Fnv) {
		(**self).feed(h);
	}
}

impl<T: Feed> Feed for Option<T> {
	fn feed(&self, h: &mut Fnv) {
		match self {
			None => h.put("None;"),
			Some(v) => {
				h.put("Some(");
				v.feed(h);
				h.put(")");
			}
		}
	}
}

/// The items of a list, between `open` and `]`.
fn feed_all<'a, T: Feed + 'a>(h: &mut Fnv, open: &str, items: impl IntoIterator<Item = &'a T>) {
	h.put(open);
	for item in items {
		item.feed(h);
	}
	h.put("]");
}

impl<T: Feed> Feed for [T] {
	fn feed(&self, h: &mut Fnv) {
		feed_all(h, "[", self);
	}
}

impl<T: Feed> Feed for Vec<T> {
	fn feed(&self, h: &mut Fnv) {
		feed_all(h, "[", self);
	}
}

impl<T: Feed, const N: usize> Feed for [T; N] {
	fn feed(&self, h: &mut Fnv) {
		feed_all(h, "[", self);
	}
}

impl<T: Feed> Feed for std::collections::BTreeSet<T> {
	fn feed(&self, h: &mut Fnv) {
		feed_all(h, "{", self);
	}
}

impl<K: Feed, V: Feed> Feed for std::collections::BTreeMap<K, V> {
	fn feed(&self, h: &mut Fnv) {
		h.put("{");
		for (k, v) in self {
			k.feed(h);
			v.feed(h);
		}
		h.put("}");
	}
}

macro_rules! tuple {
	($($t:ident $i:tt),*) => {
		impl<$($t: Feed),*> Feed for ($($t,)*) {
			fn feed(&self, h: &mut Fnv) {
				h.put("(");
				$(self.$i.feed(h);)*
				h.put(")");
			}
		}
	};
}

tuple!(A 0);
tuple!(A 0, B 1);
tuple!(A 0, B 1, C 2);

/// FNV-1a of every bit of `value`.
fn digest<T: Feed + ?Sized>(value: &T) -> u64 {
	let mut h = Fnv(0xcbf2_9ce4_8422_2325);
	value.feed(&mut h);
	h.0
}
"#;

/// What each shape `N` exports, which the census may refuse one by one.
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
	/// `try_N(seed)`: the made value, `Ok` for an even seed and `Err` for an odd one.
	Try,
	/// `swap_N(&mut v, seed)`: the digest of `v`, which it replaces with the seed's value.
	Swap,
	/// `pair_N(a, seed, &b)`: a digest of all three.
	Pair,
	/// `maybe_N(v)`: the digest of an `Option` of a borrow.
	Maybe,
	/// `HolderN`: an object of the value, and its methods by each receiver.
	Holder,
	/// `sink_N(seed, s)`: the digest of what the foreign `SinkN::echo` returns of the
	/// made value.
	Sink,
}

impl Item {
	/// The paths the census names it by, any of which it may refuse.
	fn paths(self, n: usize) -> Vec<String> {
		let function = |name: &str| vec![format!("{name}_{n}")];
		match self {
			Item::Make => function("make"),
			Item::Echo => function("echo"),
			Item::Digest => function("digest"),
			Item::Lent => function("lent"),
			Item::Peek => function("peek"),
			Item::Made => function("made"),
			Item::Later => function("later"),
			Item::Try => function("try"),
			Item::Swap => function("swap"),
			Item::Pair => function("pair"),
			Item::Maybe => function("maybe"),
			Item::Holder => ["", "::new", "::digest", "::swap", "::into_inner"]
				.map(|method| format!("Holder{n}{method}"))
				.into(),
			Item::Sink => vec![format!("Sink{n}"), format!("sink_{n}")],
		}
	}

	/// The function's name, or the type's.
	fn name(self, n: usize) -> String {
		self.paths(n).swap_remove(0)
	}

	/// The Rust source of its items for shape `n` of type `ty`, which `digest_N` borrows
	/// as `lent`.
	fn source(self, n: usize, ty: &str, lent: &str, make: &str) -> String {
		match self {
			Item::Make => {
				format!("pub fn make_{n}(seed: u64) -> {ty} {{\n\tlet r = &mut Rng::new(seed);\n\t{make}\n}}\n")
			}
			Item::Echo => format!("pub fn echo_{n}(v: {ty}) -> {ty} {{\n\tv\n}}\n"),
			Item::Digest => format!("pub fn digest_{n}(v: {lent}) -> u64 {{\n\tdigest(&v)\n}}\n"),
			Item::Lent => {
				format!("pub fn lent_{n}(seed: u64, f: impl FnOnce({ty}) -> u64) -> u64 {{\n\tf(make_{n}(seed))\n}}\n")
			}
			Item::Peek => format!(
				"pub fn peek_{n}(seed: u64, f: impl FnOnce({lent}) -> u64) -> u64 {{\n\tf(&make_{n}(seed))\n}}\n"
			),
			Item::Made => {
				format!("pub fn made_{n}(seed: u64, f: impl FnOnce(u64) -> {ty}) -> u64 {{\n\tdigest(&f(seed))\n}}\n")
			}
			Item::Later => format!("pub async fn later_{n}(v: {ty}) -> {ty} {{\n\tv\n}}\n"),
			Item::Try => format!(
				"pub fn try_{n}(seed: u64) -> Result<{ty}, {ty}> {{\n\tlet v = make_{n}(seed);\n\tif seed % 2 == 0 {{ Ok(v) }} else {{ Err(v) }}\n}}\n"
			),
			Item::Swap => format!(
				"pub fn swap_{n}(v: &mut {ty}, seed: u64) -> u64 {{\n\tlet d = digest(&*v);\n\t*v = make_{n}(seed);\n\td\n}}\n"
			),
			Item::Pair => format!(
				"pub fn pair_{n}(a: {ty}, seed: u64, b: {lent}) -> u64 {{\n\tdigest(&(digest(&a), seed, digest(&b)))\n}}\n"
			),
			Item::Maybe => format!("pub fn maybe_{n}(v: Option<{lent}>) -> u64 {{\n\tdigest(&v)\n}}\n"),
			Item::Holder => format!(
				"pub struct Holder{n}({ty});\n\n\
				impl Holder{n} {{\n\
				\tpub fn new(v: {ty}) -> Self {{\n\t\tSelf(v)\n\t}}\n\n\
				\tpub fn digest(&self) -> u64 {{\n\t\tdigest(&self.0)\n\t}}\n\n\
				\tpub fn swap(&mut self, v: {ty}) -> {ty} {{\n\t\tstd::mem::replace(&mut self.0, v)\n\t}}\n\n\
				\tpub fn into_inner(self) -> {ty} {{\n\t\tself.0\n\t}}\n\
				}}\n"
			),
			Item::Sink => format!(
				"pub trait Sink{n}: Send + Sync {{\n\tfn echo(&self, v: {ty}) -> {ty};\n}}\n\n\
				pub fn sink_{n}(seed: u64, s: impl Sink{n}) -> u64 {{\n\tdigest(&s.echo(make_{n}(seed)))\n}}\n"
			),
		}
	}
}

/// What a line of the script calls, each the item it is named after, of its shape and seed.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Kind {
	/// `digest_N(echo_N(make_N(seed)))`
	Echo,
	/// `lent_N(seed, digest_N)`
	Lent,
	/// `peek_N(seed, digest_N)`
	Peek,
	/// `made_N(seed, make_N)`
	Made,
	/// `digest_N(await later_N(make_N(seed)))`
	Later,
	/// `try_N(seed)`, printing `ok` or `err` and the digest of its value.
	Try,
	/// `swap_N(&mut v, seed ^ 1)` of `v = make_N(seed)`, printing what it returns and
	/// `digest_N(v)` after.
	Swap,
	/// `pair_N(make_N(seed), seed, make_N(seed ^ 1))`
	Pair,
	/// `maybe_N(make_N(seed))` for an odd seed, else `maybe_N(None)`.
	Maybe,
	/// `HolderN::new(make_N(seed))`, `swap(make_N(seed ^ 1))`, printing the digests of
	/// what that returns, of `digest()`, and of `into_inner()`.
	Held,
	/// `sink_N(seed, s)` of an `s` whose `echo` returns what it takes, printing `echoed` in a
	/// foreign driver so its line shows the callback ran.
	Sink,
}

impl Kind {
	/// What the shape must export for the line, besides `make_N` and `digest_N`.
	fn item(self) -> Item {
		match self {
			Kind::Echo => Item::Echo,
			Kind::Lent => Item::Lent,
			Kind::Peek => Item::Peek,
			Kind::Made => Item::Made,
			Kind::Later => Item::Later,
			Kind::Try => Item::Try,
			Kind::Swap => Item::Swap,
			Kind::Pair => Item::Pair,
			Kind::Maybe => Item::Maybe,
			Kind::Held => Item::Holder,
			Kind::Sink => Item::Sink,
		}
	}
}

/// Every [`Kind`], in the order a shape's lines run.
const KINDS: [Kind; 11] = [
	Kind::Echo,
	Kind::Lent,
	Kind::Peek,
	Kind::Made,
	Kind::Later,
	Kind::Try,
	Kind::Swap,
	Kind::Pair,
	Kind::Maybe,
	Kind::Held,
	Kind::Sink,
];

/// Every [`Item`], in the order the crate declares them.
const ITEMS: [Item; 13] = [
	Item::Make,
	Item::Echo,
	Item::Digest,
	Item::Lent,
	Item::Peek,
	Item::Made,
	Item::Later,
	Item::Try,
	Item::Swap,
	Item::Pair,
	Item::Maybe,
	Item::Holder,
	Item::Sink,
];

/// A line of the script each driver runs, printing `kind n seed value`.
#[derive(Clone, Copy, Debug)]
struct Call {
	kind: Kind,
	/// Its shape's `N`.
	n: usize,
	seed: u64,
}

impl Call {
	fn label(self) -> String {
		let kind = format!("{:?}", self.kind).to_lowercase();
		format!("{kind} {} {}", self.n, self.seed)
	}

	/// The other seed, which `swap`, `pair`, and `held` make a second value from.
	fn other(self) -> u64 {
		self.seed ^ 1
	}
}

/// The seeds each shape is made from: even, which `try_N` returns `Ok` for, and odd.
const SEEDS: [u64; 3] = [0, 1, 0xdead_beef];

/// The shapes' crate, as the census left it: which shapes export, and the script.
struct Crate {
	dir: PathBuf,
	shapes: Vec<Shape>,
	/// How many of `shapes` are the fixed set, which [`KNOWN`] checks, and the seed the
	/// rest are drawn from, if any.
	drawn: Option<(usize, u64)>,
	/// Each shape's Rust type, by `N`.
	types: Vec<String>,
	calls: Vec<Call>,
	/// The digests the script prints natively.
	transcript: String,
}

/// Write the crate of `shapes` named `name` in the scratch directory, ask the exporter what
/// exports, and run its script natively.
fn emit(name: &str, shapes: &[Shape], drawn: Option<(usize, u64)>) -> Crate {
	let dir = scratch().join(name);
	let mut emit = Emit {
		items: String::new(),
		defined: 0,
	};
	let mut functions = String::new();
	let mut types = Vec::new();
	let mut lents = Vec::new();
	for (n, shape) in shapes.iter().enumerate() {
		let (ty, make) = emit.shape(shape);
		let lent = borrowed(shape, &ty);
		for item in ITEMS {
			let _ = writeln!(functions, "{}", item.source(n, &ty, &lent, &make));
		}
		types.push(ty);
		lents.push(lent);
	}
	let lib = format!(
		"//! Generated by `tests/fixtures/shapes.rs`: each shape's items.\n\
		#![allow(clippy::all, dead_code, reason = \"generated\")]\n{RUNTIME}\n{}{functions}",
		emit.items
	);
	let manifest = format!(
		"[package]\nname = \"{name}\"\nversion = \"0.0.0\"\nedition = \"2024\"\npublish = false\n\n\
		[dependencies]\nbytes = \"1\"\nserde_json = \"1\"\nurl = \"2\"\n\n\
		# Marks each call of the native script that digests a NaN of its own bits.\n[features]\ntranscript = []\n\n[workspace]\n"
	);
	super::write_all(
		&dir,
		&[("Cargo.toml", &manifest), ("src/lib.rs", &lib), ("ffi.toml", "")],
	);
	// The api fixture's versions, resolved offline.
	super::step::copy(&manifest_dir().join("Cargo.lock"), &dir.join("Cargo.lock"));

	let refused = refused(&dir);
	let exported = |item: Item, n: usize| item.paths(n).iter().all(|path| !refused.absent.contains(path));
	let mut excludes: Vec<&String> = refused.exclude.iter().collect();
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
			for kind in KINDS {
				if exported(kind.item(), n) {
					calls.push(Call { kind, n, seed });
				}
			}
		}
	}

	let mut sinks = String::new();
	let lines = by_shape(&calls, |call| {
		let Call { n, seed, .. } = call;
		let other = call.other();
		let value = match call.kind {
			Kind::Echo => format!("digest_{n}(&echo_{n}(make_{n}({seed})))"),
			Kind::Lent => format!("lent_{n}({seed}, |v| digest_{n}(&v))"),
			Kind::Peek => format!("peek_{n}({seed}, digest_{n})"),
			Kind::Made => format!("made_{n}({seed}, make_{n})"),
			Kind::Later => format!("digest_{n}(&later(later_{n}(make_{n}({seed}))))"),
			Kind::Try => format!(
				"match try_{n}({seed}) {{ Ok(v) => format!(\"ok {{}}\", digest_{n}(&v)), Err(e) => format!(\"err {{}}\", digest_{n}(&e)) }}"
			),
			Kind::Swap => format!(
				"{{ let mut v = make_{n}({seed}); let d = swap_{n}(&mut v, {other}); format!(\"{{d}} {{}}\", digest_{n}(&v)) }}"
			),
			Kind::Pair => format!("pair_{n}(make_{n}({seed}), {seed}, &make_{n}({other}))"),
			Kind::Maybe => match seed % 2 {
				1 => format!("maybe_{n}(Some::<{}>(&make_{n}({seed})))", lents[n]),
				_ => format!("maybe_{n}(None)"),
			},
			Kind::Held => format!(
				"{{ let mut h = Holder{n}::new(make_{n}({seed})); let old = h.swap(make_{n}({other})); \
				format!(\"{{}} {{}} {{}}\", digest_{n}(&old), h.digest(), digest_{n}(&h.into_inner())) }}"
			),
			Kind::Sink => {
				if seed == SEEDS[0] {
					let _ = writeln!(
						sinks,
						"struct Echo{n};\n\nimpl Sink{n} for Echo{n} {{\n\tfn echo(&self, v: {ty}) -> {ty} {{\n\t\tv\n\t}}\n}}\n",
						ty = types[n]
					);
				}
				format!("sink_{n}({seed}, Echo{n})")
			}
		};
		format!("\tprintln!(\"{} {{}}\", {value});", call.label())
	});
	// Each shape's calls in a function of its own, past whose locals wasm's limit no
	// function grows.
	let mut script = format!("//! The shapes' script, run natively.\n\nuse {name}::*;\n{NATIVE}");
	for (n, _) in &lines {
		let _ = writeln!(script, "\tshape_{n}();");
	}
	script.push_str("}\n\n");
	for (n, lines) in &lines {
		let _ = writeln!(script, "fn shape_{n}() {{\n{lines}}}\n");
	}
	script.push_str(&sinks);
	super::write_all(&dir, &[("examples/transcript.rs", &script)]);
	// Apart from the exporter's build, which cleans the crate each time.
	let transcript = run(cargo()
		.args(["run", "--quiet", "--example", "transcript", "--features", "transcript"])
		.env("CARGO_TARGET_DIR", dir.join("native"))
		.current_dir(&dir));
	Crate {
		dir,
		shapes: shapes.to_vec(),
		drawn,
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

/// What the census of the crate in `dir` says does not export: the config that exports
/// what maps, and what leaves itself out.
fn refused(dir: &Path) -> Refused {
	let census = dir.join("census");
	run(Command::new(env!("CARGO_BIN_EXE_rust2-ffi"))
		.arg("--config")
		.arg(dir.join("ffi.toml"))
		.arg("--census")
		.arg(&census)
		.env("CARGO_TARGET_DIR", dir.join("build")));
	let name = dir.file_name().unwrap().to_string_lossy();
	let report = std::fs::read_to_string(census.join(format!("{name}.md"))).unwrap();
	let exclude = match report.split_once("## The config that exports what maps") {
		Some((_, config)) => config
			.lines()
			.skip_while(|line| *line != "exclude = [")
			.skip(1)
			.take_while(|line| *line != "]")
			.map(|line| line.trim().trim_end_matches(',').trim_matches('"').to_string())
			.collect(),
		None => Vec::new(),
	};
	// The traits C cannot implement, and what C can never hold, leave themselves out.
	let mut absent = exclude.clone();
	for section in [
		"## Traits left out, since C cannot implement them",
		"## What C can never hold, left out",
	] {
		let Some((_, list)) = report.split_once(section) else {
			continue;
		};
		absent.extend(
			(list.lines().skip(1))
				.take_while(|line| !line.starts_with("## "))
				.filter_map(|line| Some(line.strip_prefix("- `")?.split_once('`')?.0.to_string())),
		);
	}
	Refused { exclude, absent }
}

/// What the census of the shapes' crate says does not export.
struct Refused {
	/// The paths its config leaves out.
	exclude: Vec<String>,
	/// Those, and what leaves itself out.
	absent: Vec<String>,
}

/// Each variant of the IR's `Type`, from its source.
fn ir_variants() -> Vec<String> {
	let ir = std::fs::read_to_string(manifest_dir().join("src").join("ir.rs")).unwrap();
	// The declaration whatever its visibility, which only the crate needs.
	let body = ir.split_once("enum Type {").expect("ir.rs declares Type").1;
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
	/// The web binding's module and native transcript, built for the web test alone.
	web: std::sync::OnceLock<(PathBuf, String)>,
}

/// Emit, census, export, and build the crate of `shapes` named `name`, of which `drawn`
/// says how many are the fixed set and the seed of the rest, if any.
fn build(name: &str, shapes: &[Shape], drawn: Option<(usize, u64)>) -> Built {
	let krate = emit(name, shapes, drawn);
	let out = krate.dir.join("out");
	run(Command::new(env!("CARGO_BIN_EXE_rust2-ffi"))
		.arg("--config")
		.arg(krate.dir.join("ffi.toml"))
		.arg("--out")
		.arg(&out)
		// One binding for the host and the target the web binding serves, as the api
		// fixture's.
		.args(["--target", super::host(), "--target", super::WEB])
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
	let lib = super::shared(name);
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
		web: std::sync::OnceLock::new(),
	}
}

impl Built {
	fn name(&self) -> String {
		self.krate.dir.file_name().unwrap().to_string_lossy().into_owned()
	}

	/// The module the export's `web` feature builds for wasm32-wasip1, and what the native
	/// script prints there, run under Node's WASI: its `usize` is 32 bits, and a path there
	/// is text.
	fn web(&self) -> &(PathBuf, String) {
		self.web.get_or_init(|| {
			let name = self.name();
			let glue = self.krate.dir.join("glue-web");
			run(cargo()
				.args(["build", "--quiet", "--locked", "--manifest-path"])
				.arg(self.out.join("Cargo.toml"))
				.arg("--target-dir")
				.arg(&glue)
				.args(["--target", super::WEB, "--features", "web"])
				.env("RUSTFLAGS", "-D warnings"));
			let native = self.krate.dir.join("native-web");
			run(cargo()
				.args([
					"build",
					"--quiet",
					"--example",
					"transcript",
					"--features",
					"transcript",
				])
				.args(["--target", super::WEB])
				.env("CARGO_TARGET_DIR", &native)
				.current_dir(&self.krate.dir));
			let wasm = native
				.join(super::WEB)
				.join("debug")
				.join("examples")
				.join("transcript.wasm");
			let transcript = run(Command::new("node")
				.args(["--no-warnings", "--input-type=module", "-e", super::WASI_MAIN])
				.arg(wasm));
			let module = glue.join(super::WEB).join("debug").join(format!("{name}.wasm"));
			(module, String::from_utf8_lossy(&transcript.stdout).into_owned())
		})
	}

	fn wasm_transcript(&self) -> &str {
		&self.web().1
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

	/// The calls `language`'s driver runs, each with the line native Rust printed for it in
	/// `transcript`: every call but those [`KNOWN`] says it gets wrong today, each entry of
	/// which must match one of the fixed set's.
	fn expected<'a>(&self, language: &str, transcript: &'a str) -> Vec<(Call, &'a str)> {
		// Each call's line, after the NaNs it digested, as `signaling f32` or `quiet f64`.
		let mut lines: Vec<(&str, Vec<&str>)> = Vec::new();
		let mut nans = Vec::new();
		for line in transcript.lines() {
			match line.strip_prefix("nan ") {
				Some(nan) => nans.push(nan),
				None => lines.push((line, std::mem::take(&mut nans))),
			}
		}
		assert_eq!(
			lines.len(),
			self.krate.calls.len(),
			"the native script printed a line per call"
		);
		let entries: Vec<&Known> = KNOWN.iter().filter(|k| k.language == language).collect();
		let calls: Vec<(Call, &str, &[&str])> = (self.krate.calls.iter().zip(&lines))
			.map(|(call, (line, nans))| (*call, *line, nans.as_slice()))
			.collect();
		let skipped =
			|(call, _, nans): &(Call, &str, &[&str]), k: &Known| (k.skips)(&self.krate.shapes[call.n], call.kind, nans);
		if let Some((fixed, _)) = self.krate.drawn {
			for k in &entries {
				// The gate's subset may hold none of an entry's calls; the nightly's fixed set
				// holds each.
				assert!(
					calls.iter().any(|c| c.0.n < fixed && skipped(c, k)),
					"KNOWN's {language} entry for {} ({}) matches no call: remove it",
					k.what,
					k.quest
				);
			}
		}
		(calls.iter())
			.filter(|c| !entries.iter().any(|k| skipped(c, k)))
			.map(|&(call, line, _)| (call, line))
			.collect()
	}

	/// Asserts `language`'s driver printed what native Rust did for each of `expected`,
	/// naming each line that differs by its shape's Rust type.
	fn check(&self, language: &str, expected: &[(Call, &str)], output: &std::process::Output) {
		let got = String::from_utf8_lossy(&output.stdout);
		let mut wrong = String::new();
		let lines: Vec<&str> = got.lines().collect();
		// A sink's foreign `echo` prints `echoed` once, before its call's line.
		let expected: Vec<(Call, &str)> = (expected.iter())
			.flat_map(|&(call, want)| {
				(call.kind == Kind::Sink)
					.then_some((call, "echoed"))
					.into_iter()
					.chain([(call, want)])
			})
			.collect();
		for (i, (call, want)) in expected.iter().enumerate() {
			let line = lines.get(i).copied().unwrap_or("<missing>");
			if line != *want {
				let _ = writeln!(wrong, "  {want} ({}), got {line}", self.krate.types[call.n]);
			}
		}
		let drawn = match self.krate.drawn {
			Some((_, seed)) => format!("; shapes past the fixed set drawn from RUST2_SHAPES_SEED={seed}"),
			None => String::new(),
		};
		assert!(
			wrong.is_empty() && lines.len() == expected.len(),
			"{language}'s round trips differ from native Rust's in {} of {} lines, {} printed{drawn}:\n{wrong}{}",
			wrong.lines().count(),
			expected.len(),
			lines.len(),
			String::from_utf8_lossy(&output.stderr)
		);
	}
}

/// A round trip a binding gets wrong today.
struct Known {
	language: &'static str,
	what: &'static str,
	/// Whether a call of a shape, which digested these NaNs natively, as `signaling f32` or
	/// `quiet f64`, does what it gets wrong, which the language's driver skips.
	skips: fn(&Shape, Kind, &[&str]) -> bool,
	/// The quest that fixes it.
	quest: &'static str,
}

/// The round trips a binding gets wrong today, each the quest's to fix: the language, what
/// it gets wrong, the calls it skips for it, and the quest. The nightly fails on an entry
/// matching no call of its fixed set, but its calls stay skipped after a fix, so the quest
/// that fixes one deletes its entry.
const KNOWN: &[Known] = &[
	Known {
		language: "python",
		what: "a signaling NaN of an f32, which a float holds quieted",
		skips: |shape, _, nans| nans.contains(&"signaling f32") && crosses(shape, &|s| matches!(s, Shape::F32)),
		quest: "quest/m1/ffi-export/signaling-nan.md",
	},
	Known {
		language: "dart",
		what: "a signaling NaN of an f32, which a double holds quieted",
		skips: |shape, _, nans| nans.contains(&"signaling f32") && crosses(shape, &|s| matches!(s, Shape::F32)),
		quest: "quest/m1/ffi-export/signaling-nan.md",
	},
	Known {
		language: "web",
		what: "a NaN's bits, which a number loses: quieted, and canonicalized once V8 optimizes",
		skips: |shape, _, nans| !nans.is_empty() && crosses(shape, &|s| matches!(s, Shape::F32 | Shape::F64)),
		quest: "quest/m1/ffi-export/signaling-nan.md",
	},
];

/// Whether a value of `shape` holds a `leaf` the binding converts, which one inside an
/// object does not: the object crosses as a handle, and only Rust reads its field.
fn crosses(shape: &Shape, leaf: &dyn Fn(&Shape) -> bool) -> bool {
	match shape {
		Shape::Object(_) => false,
		Shape::Option(s) | Shape::Vec(s) | Shape::Set(s) | Shape::Box(s) | Shape::Enum(s) => crosses(s, leaf),
		Shape::Map(k, v) => crosses(k, leaf) || crosses(v, leaf),
		Shape::Tuple(s) | Shape::Record(s) => s.iter().any(|s| crosses(s, leaf)),
		s => leaf(s),
	}
}

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
		let Some(Outs { decls, pointers, value }) = self.gather(ty, var) else {
			return format!("\t\t{ty} {var} = {function}({args}{});\n", self.tail());
		};
		let mut out = decls;
		let call = format!("{function}({args}, {pointers}{})", self.tail());
		let _ = match ready {
			Some(ready) => writeln!(out, "\t\tif ({call} != {ready}) abort();"),
			None => writeln!(out, "\t\t{call};"),
		};
		let _ = writeln!(out, "\t\t{ty} {var} = {value};");
		out
	}

	/// The out pointers it writes its output through, `out`, or `out0` on for a returned
	/// tuple's elements, declared as `var_K`, and their value as `ty`.
	fn gather(&self, ty: &str, var: &str) -> Option<Outs> {
		let is_out = |name: &str| name == "out" || name.strip_prefix("out").is_some_and(|k| k.parse::<usize>().is_ok());
		let outs: Vec<&(String, String)> = self.params.iter().filter(|(_, name)| is_out(name)).collect();
		if outs.is_empty() {
			return None;
		}
		let mut decls = String::new();
		let mut pointers = Vec::new();
		let mut values = Vec::new();
		for (k, (pointer, _)) in outs.iter().enumerate() {
			let _ = writeln!(decls, "\t\t{} {var}_{k};", pointer.strip_suffix('*').unwrap().trim());
			pointers.push(format!("&{var}_{k}"));
			values.push(format!("{var}_{k}"));
		}
		let value = match outs[0].1 == "out" {
			true => format!("{var}_0"),
			false => format!("({ty}){{{}}}", values.join(", ")),
		};
		Some(Outs {
			decls,
			pointers: pointers.join(", "),
			value,
		})
	}

	/// Whether it takes a trailing status, for a check one of its handles can fail.
	fn status(&self) -> bool {
		self.params.last().is_some_and(|(_, name)| name == "status")
	}

	/// What follows its other arguments: NULL for a status, aborting on a failed check.
	fn tail(&self) -> &'static str {
		match self.status() {
			true => ", NULL",
			false => "",
		}
	}

	/// What a function pointer parameter `(*name)` returns and takes after its data.
	fn pointer(&self, name: &str) -> (String, String) {
		let ty = self.param(name);
		let (ret, rest) = ty.split_once(&format!(" (*{name})(")).unwrap();
		let param = rest.strip_prefix("void *data, ").unwrap().strip_suffix(')').unwrap();
		(ret.to_string(), param.to_string())
	}
}

/// The type of the value an option struct `ty` holds, if `ty` is one.
fn option_value(header: &str, ty: &str) -> Option<String> {
	let body = header
		.split(&format!("typedef struct {ty} {{\n\tbool some;\n\t"))
		.nth(1)?;
	let field = body.lines().next()?;
	Some(field.strip_suffix(" value;")?.to_string())
}

/// What a C function writes its output through: the declarations of its out pointers, the
/// pointers to pass, and the value they gather.
struct Outs {
	decls: String,
	pointers: String,
	value: String,
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
		let Call { n, seed, .. } = *call;
		let make = proto(Item::Make, n);
		let digest = proto(Item::Digest, n);
		let digested = digest.param("v").to_string();
		let fname = |item: Item| format!("{name}_{}", item.name(n));
		// The digest of the value `v` holds.
		let digested_of = |v: &str| format!("{}({v}{})", fname(Item::Digest), digest.tail());
		let mut block = String::new();
		match call.kind {
			Kind::Try => {
				let tried = proto(Item::Try, n);
				let err = tried.param("err").strip_suffix('*').unwrap().trim().to_string();
				let Outs { decls, pointers, value } = tried.gather(&digested, "a").expect("try_N writes its value out");
				block += &decls;
				let _ = writeln!(
					block,
					"\t\t{err} e;\n\t\tif ({}({seed}u, {pointers}, &e)) {{\n\t\t\t{digested} v = {value};\n\
					\t\t\tprintf(\"{label} ok %\" PRIu64 \"\\n\", {});\n\t\t}} else {{\n\
					\t\t\tprintf(\"{label} err %\" PRIu64 \"\\n\", {});\n\t\t}}",
					fname(Item::Try),
					digested_of("v"),
					digested_of("e"),
				);
			}
			Kind::Swap => {
				let swap = proto(Item::Swap, n);
				block += &make.call(&fname(Item::Make), &[format!("{seed}u")], swap.param("v"), "a", None);
				let other = call.other();
				// A value is written back through `v_out`; an object's handle is borrowed.
				let after = match swap.params.iter().any(|(_, name)| name == "v_out") {
					true => {
						let ty = swap.param("v_out").strip_suffix('*').unwrap().trim().to_string();
						let _ = writeln!(
							block,
							"\t\t{ty} b;\n\t\tuint64_t d = {}(a, {other}u, &b{});",
							fname(Item::Swap),
							swap.tail()
						);
						"b"
					}
					false => {
						let _ = writeln!(
							block,
							"\t\tuint64_t d = {}(a, {other}u{});",
							fname(Item::Swap),
							swap.tail()
						);
						"a"
					}
				};
				let _ = writeln!(
					block,
					"\t\tprintf(\"{label} %\" PRIu64 \" %\" PRIu64 \"\\n\", d, {});",
					digested_of(after)
				);
			}
			Kind::Pair => {
				let pair = proto(Item::Pair, n);
				block += &make.call(&fname(Item::Make), &[format!("{seed}u")], pair.param("a"), "a", None);
				block += &make.call(
					&fname(Item::Make),
					&[format!("{}u", call.other())],
					pair.param("b"),
					"b",
					None,
				);
				let _ = writeln!(
					block,
					"\t\tprintf(\"{label} %\" PRIu64 \"\\n\", {}(a, {seed}u, b{}));",
					fname(Item::Pair),
					pair.tail()
				);
			}
			Kind::Maybe => {
				let maybe = proto(Item::Maybe, n);
				let ty = maybe.param("v").to_string();
				// An option of a value is a struct of it; of a buffer or handle, null is None.
				let option = option_value(&header, &ty);
				let v = match (seed % 2, &option) {
					(0, Some(_)) => format!("({ty}){{0}}"),
					(0, None) if ty.ends_with('*') => "NULL".into(),
					(0, None) => format!("({ty}){{0}}"),
					(_, Some(value)) => {
						block += &make.call(&fname(Item::Make), &[format!("{seed}u")], value, "a", None);
						format!("({ty}){{true, a}}")
					}
					(_, None) => {
						block += &make.call(&fname(Item::Make), &[format!("{seed}u")], &ty, "a", None);
						"a".into()
					}
				};
				let _ = writeln!(
					block,
					"\t\tprintf(\"{label} %\" PRIu64 \"\\n\", {}({v}{}));",
					fname(Item::Maybe),
					maybe.tail()
				);
			}
			Kind::Held => {
				let holder = format!("{name}_Holder{n}");
				let new = prototype(&header, &format!("{holder}_new"));
				let swap = prototype(&header, &format!("{holder}_swap"));
				let held = prototype(&header, &format!("{holder}_digest"));
				let inner = prototype(&header, &format!("{holder}_into_inner"));
				block += &make.call(&fname(Item::Make), &[format!("{seed}u")], new.param("v"), "a", None);
				block += &new.call(
					&format!("{holder}_new"),
					&["a".into()],
					&format!("{holder} *"),
					"h",
					None,
				);
				block += &make.call(
					&fname(Item::Make),
					&[format!("{}u", call.other())],
					swap.param("v"),
					"b",
					None,
				);
				block += &swap.call(
					&format!("{holder}_swap"),
					&["h".into(), "b".into()],
					&digested,
					"old",
					None,
				);
				let _ = writeln!(block, "\t\tuint64_t d = {holder}_digest(h{});", held.tail());
				block += &inner.call(&format!("{holder}_into_inner"), &["h".into()], &digested, "c", None);
				let _ = writeln!(
					block,
					"\t\t{holder}_free(h);\n\t\tprintf(\"{label} %\" PRIu64 \" %\" PRIu64 \" %\" PRIu64 \"\\n\", {}, d, {});",
					digested_of("old"),
					digested_of("c")
				);
			}
			Kind::Sink => {
				let sink = format!("{name}_Sink{n}");
				if defined.insert(format!("Sink{n}")) {
					let vtable = header
						.split(&format!("typedef struct {sink}_vtable {{"))
						.nth(1)
						.expect("the header declares the sink's vtable");
					let echo = vtable.lines().find(|line| line.contains("(*echo)")).unwrap();
					let echo = Proto {
						params: vec![(echo.trim().trim_end_matches(';').to_string(), "echo".into())],
					};
					let (ret, param) = echo.pointer("echo");
					// Its parameter, named in the vtable.
					let param = &param[..param.rfind([' ', '*']).unwrap() + 1];
					let _ = writeln!(
						closures,
						"static {ret} sink{n}_echo(void *data, {param}v) {{\n\t(void)data;\n\tprintf(\"echoed\\n\");\n\treturn v;\n}}\n\n\
						static void sink{n}_free(void *data) {{\n\t(void)data;\n}}\n\n\
						static const {sink}_vtable sink{n} = {{.free = sink{n}_free, .echo = sink{n}_echo}};\n"
					);
				}
				let _ = writeln!(
					block,
					"\t\tprintf(\"{label} %\" PRIu64 \"\\n\", {name}_sink_{n}({seed}u, ({sink}){{&sink{n}, NULL}}));"
				);
			}
			Kind::Echo => {
				let echo = proto(Item::Echo, n);
				block += &make.call(&fname(Item::Make), &[format!("{seed}u")], echo.param("v"), "a", None);
				block += &echo.call(&fname(Item::Echo), &["a".into()], &digested, "b", None);
				let _ = writeln!(
					block,
					"\t\tprintf(\"{label} %\" PRIu64 \"\\n\", {}(b{}));",
					fname(Item::Digest),
					digest.tail()
				);
			}
			Kind::Lent | Kind::Peek => {
				let item = if call.kind == Kind::Lent {
					Item::Lent
				} else {
					Item::Peek
				};
				if defined.insert(item.name(n)) {
					let (_, param) = proto(item, n).pointer("f");
					let _ = writeln!(
						closures,
						"static uint64_t {}(void *data, {param} v) {{\n\t(void)data;\n\treturn {}(v{});\n}}\n",
						item.name(n),
						fname(Item::Digest),
						digest.tail()
					);
				}
				let _ = writeln!(
					block,
					"\t\tprintf(\"{label} %\" PRIu64 \"\\n\", {}({seed}u, {}, NULL));",
					fname(item),
					item.name(n)
				);
			}
			Kind::Made => {
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
			Kind::Later => {
				let later = proto(Item::Later, n);
				let future = format!("{}_future", fname(Item::Later));
				let poll = prototype(&header, &format!("{}_poll", fname(Item::Later)));
				block += &make.call(&fname(Item::Make), &[format!("{seed}u")], later.param("v"), "a", None);
				let _ = writeln!(block, "\t\t{future} *f = {}(a{});", fname(Item::Later), later.tail());
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
					"\t\tprintf(\"{label} %\" PRIu64 \"\\n\", {}(b{}));",
					fname(Item::Digest),
					digest.tail()
				);
			}
		}
		let _ = writeln!(body, "\t{{\n{block}\t}}");
	}
	format!(
		"#include <inttypes.h>\n#include <stdio.h>\n#include <stdlib.h>\n\n#include \"{name}.h\"\n\n{closures}int main(void) {{\n{body}\treturn 0;\n}}\n"
	)
}

/// The C++ driver, over the header's own types and futures: each call a statement of its
/// own, since some declare what they print.
fn cpp(built: &Built, calls: &[Call]) -> String {
	let name = built.name();
	let header = std::fs::read_to_string(built.out.join("include").join(&name).join(format!("{name}.hpp"))).unwrap();
	let mut sinks = String::new();
	// Each sink's class, defined by its first call the driver runs.
	let mut echoes = std::collections::BTreeSet::new();
	let mut body = String::new();
	for call in calls {
		let Call { n, seed, .. } = *call;
		let other = call.other();
		let label = call.label();
		let make = |seed: u64| format!("{name}::make_{n}({seed}u)");
		let digest = |v: &str| format!("{name}::digest_{n}({v})");
		let print = |value: String| format!("\tstd::cout << \"{label} \" << {value} << '\\n';");
		let line = match call.kind {
			Kind::Echo => print(digest(&format!("{name}::echo_{n}({})", make(seed)))),
			Kind::Lent => print(format!(
				"{name}::lent_{n}({seed}u, [](auto v) {{ return {}; }})",
				digest("std::move(v)")
			)),
			Kind::Peek => print(format!(
				"{name}::peek_{n}({seed}u, [](auto &&v) {{ return {}; }})",
				digest("v")
			)),
			Kind::Made => print(format!(
				"{name}::made_{n}({seed}u, [](std::uint64_t s) {{ return {name}::make_{n}(s); }})"
			)),
			Kind::Later => print(digest(&format!("{name}::later_{n}({}).get()", make(seed)))),
			Kind::Try => format!(
				"\t{{\n\t\tauto r = {name}::try_{n}({seed}u);\n\
				\t\tif (r) std::cout << \"{label} ok \" << {} << '\\n';\n\
				\t\telse std::cout << \"{label} err \" << {} << '\\n';\n\t}}",
				digest("std::move(r.value())"),
				digest("std::move(r.error())")
			),
			Kind::Swap => format!(
				"\t{{\n\t\tauto v = {};\n\t\tauto d = {name}::swap_{n}(v, {other}u);\n\
				\t\tstd::cout << \"{label} \" << d << ' ' << {} << '\\n';\n\t}}",
				make(seed),
				digest("std::move(v)")
			),
			Kind::Pair => print(format!("{name}::pair_{n}({}, {seed}u, {})", make(seed), make(other))),
			Kind::Maybe => {
				// An object's option is a pointer to it; anything else's an optional.
				let pointer = declaration(&header, &format!("maybe_{n}(")).contains(" *v)");
				match (seed % 2, pointer) {
					(0, true) => print(format!("{name}::maybe_{n}(nullptr)")),
					(0, false) => print(format!("{name}::maybe_{n}(std::nullopt)")),
					(_, true) => format!(
						"\t{{\n\t\tauto v = {};\n\t\tstd::cout << \"{label} \" << {name}::maybe_{n}(&v) << '\\n';\n\t}}",
						make(seed)
					),
					(_, false) => print(format!("{name}::maybe_{n}({})", make(seed))),
				}
			}
			Kind::Held => format!(
				"\t{{\n\t\tauto h = {name}::Holder{n}::new_({});\n\t\tauto old = h.swap({});\n\
				\t\tauto d = h.digest();\n\t\tauto c = std::move(h).into_inner();\n\
				\t\tstd::cout << \"{label} \" << {} << ' ' << d << ' ' << {} << '\\n';\n\t}}",
				make(seed),
				make(other),
				digest("std::move(old)"),
				digest("std::move(c)")
			),
			Kind::Sink => {
				if echoes.insert(n) {
					let class = header
						.split(&format!("\nclass Sink{n} {{"))
						.nth(1)
						.expect("the header declares the sink");
					let echo = class.lines().find(|l| l.contains(" echo(")).unwrap().trim();
					let echo = echo.strip_prefix("virtual ").unwrap().strip_suffix(" = 0;").unwrap();
					let _ = writeln!(
						sinks,
						"struct Echo{n} final : {name}::Sink{n} {{\n\t{echo} override {{\n\t\tstd::cout << \"echoed\\n\";\n\t\treturn v;\n\t}}\n}};\n"
					);
				}
				// Its result first, since C++ streams the label before evaluating the next operand.
				format!(
					"\t{{\n\t\tauto d = {name}::sink_{n}({seed}u, std::make_unique<Echo{n}>());\n\t\tstd::cout << \"{label} \" << d << '\\n';\n\t}}"
				)
			}
		};
		let _ = writeln!(body, "{line}");
	}
	format!(
		"#include <cstdint>\n#include <iostream>\n#include <memory>\n#include <optional>\n#include <utility>\n\n\
		#include \"{name}.hpp\"\n\n{sinks}int main() {{\n{body}}}\n"
	)
}

/// The line of `source` that declares `function`, a prefix of its name and `(`.
fn declaration<'a>(source: &'a str, function: &str) -> &'a str {
	let at = source
		.match_indices(function)
		.map(|(at, _)| at)
		.find(|&at| at == 0 || source[..at].ends_with(['\n', '\t', ' ', ':']))
		.unwrap_or_else(|| panic!("the source declares {function}"));
	let start = source[..at].rfind('\n').map_or(0, |i| i + 1);
	source[start..].lines().next().unwrap()
}

/// Each shape's calls in a function of its own, so no function grows past a compiler's
/// limit: `(n, lines)`.
fn by_shape(calls: &[Call], mut line: impl FnMut(Call) -> String) -> Vec<(usize, String)> {
	let mut shapes: Vec<(usize, String)> = Vec::new();
	for call in calls {
		let n = call.n;
		if shapes.last().is_none_or(|(last, _)| *last != n) {
			shapes.push((n, String::new()));
		}
		let _ = writeln!(shapes.last_mut().unwrap().1, "{}", line(*call));
	}
	shapes
}

/// The Kotlin driver, on the JVM with kotlinx-coroutines.
fn kotlin(built: &Built, calls: &[Call]) -> String {
	let name = built.name();
	let source = std::fs::read_to_string(built.out.join("kotlin").join(format!("{name}.kt"))).unwrap();
	// Each exception a `Result`'s error is thrown as, by the type of its `error`.
	let thrown: std::collections::HashMap<&str, &str> = (source.lines())
		.filter_map(|line| {
			let rest = line.strip_prefix("class ")?;
			let (class, rest) = rest.split_once("(val error: ")?;
			Some((rest.strip_suffix(") : Exception() {")?, class))
		})
		.collect();
	let shapes = by_shape(calls, |call| {
		let Call { n, seed, .. } = call;
		let other = call.other();
		let value = match call.kind {
			Kind::Echo => format!("digest{n}(echo{n}(make{n}({seed}uL)))"),
			Kind::Lent => format!("lent{n}({seed}uL) {{ digest{n}(it) }}"),
			Kind::Peek => format!("peek{n}({seed}uL) {{ digest{n}(it) }}"),
			Kind::Made => format!("made{n}({seed}uL) {{ make{n}(it) }}"),
			Kind::Later => format!("digest{n}(later{n}(make{n}({seed}uL)))"),
			Kind::Try => {
				let declared = declaration(&source, &format!("fun try{n}("));
				let ty = declared.split_once("): ").unwrap().1.split_once(" = ").unwrap().0;
				let class = thrown.get(ty).unwrap_or_else(|| panic!("no exception holds a {ty}"));
				format!(
					"try {{ \"ok ${{digest{n}(try{n}({seed}uL))}}\" }} catch (e: {class}) {{ \"err ${{digest{n}(e.error)}}\" }}"
				)
			}
			// A value is passed in a `Mut`, which Rust writes back; an object's handle is borrowed.
			Kind::Swap => match declaration(&source, &format!("fun swap{n}(")).contains("v: _Mut<") {
				true => format!(
					"run {{ val v = Mut(make{n}({seed}uL)); val d = swap{n}(v, {other}uL); \"$d ${{digest{n}(v.value)}}\" }}"
				),
				false => format!(
					"run {{ val v = make{n}({seed}uL); val d = swap{n}(v, {other}uL); \"$d ${{digest{n}(v)}}\" }}"
				),
			},
			Kind::Pair => format!("pair{n}(make{n}({seed}uL), {seed}uL, make{n}({other}uL))"),
			Kind::Maybe => match seed % 2 {
				0 => format!("maybe{n}(null)"),
				_ => format!("maybe{n}(make{n}({seed}uL))"),
			},
			Kind::Held => format!(
				"Holder{n}.new(make{n}({seed}uL)).use {{ h -> val old = h.swap(make{n}({other}uL)); val d = h.digest(); \
				\"${{digest{n}(old)}} $d ${{digest{n}(h.intoInner())}}\" }}"
			),
			Kind::Sink => {
				let interface = source
					.split(&format!("\ninterface Sink{n} {{\n"))
					.nth(1)
					.expect("the Kotlin declares the sink");
				let echo = interface.lines().next().unwrap().trim();
				format!("sink{n}({seed}uL, object : Sink{n} {{ override {echo} {{ println(\"echoed\"); return v }} }})")
			}
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
	let source = std::fs::read_to_string(built.out.join("python").join(format!("{name}.py"))).unwrap();
	let mut sinks = String::new();
	// Each sink's class, defined by its first call the driver runs.
	let mut echoes = std::collections::BTreeSet::new();
	let mut body = String::new();
	for call in calls {
		let Call { n, seed, .. } = *call;
		let other = call.other();
		let value = match call.kind {
			Kind::Echo => format!("s.digest_{n}(s.echo_{n}(s.make_{n}({seed})))"),
			Kind::Lent => format!("s.lent_{n}({seed}, s.digest_{n})"),
			Kind::Peek => format!("s.peek_{n}({seed}, s.digest_{n})"),
			Kind::Made => format!("s.made_{n}({seed}, s.make_{n})"),
			Kind::Later => format!("s.digest_{n}(await s.later_{n}(s.make_{n}({seed})))"),
			Kind::Try => format!("tried(lambda: s.try_{n}({seed}), s.digest_{n})"),
			// A value is passed in a `Mut`, which Rust writes back; an object is borrowed.
			Kind::Swap => {
				let boxed = declaration(&source, &format!("def swap_{n}(")).contains("v: Mut[");
				let boxed = if boxed { "True" } else { "False" };
				format!("swapped(s.make_{n}, s.swap_{n}, s.digest_{n}, {seed}, {other}, {boxed})")
			}
			Kind::Pair => format!("s.pair_{n}(s.make_{n}({seed}), {seed}, s.make_{n}({other}))"),
			Kind::Maybe => match seed % 2 {
				0 => format!("s.maybe_{n}(None)"),
				_ => format!("s.maybe_{n}(s.make_{n}({seed}))"),
			},
			Kind::Held => format!("held(s.Holder{n}, s.make_{n}, s.digest_{n}, {seed}, {other})"),
			Kind::Sink => {
				if echoes.insert(n) {
					let _ = writeln!(
						sinks,
						"class Echo{n}(s.Sink{n}):\n\tdef echo(self, v: typing.Any) -> typing.Any:\n\t\tprint(\"echoed\")\n\t\treturn v\n\n"
					);
				}
				format!("s.sink_{n}({seed}, Echo{n}())")
			}
		};
		let _ = writeln!(body, "\tprint(f\"{} {{{value}}}\")", call.label());
	}
	format!(
		"import asyncio\nimport typing\n\nimport {name} as s\n\n\n{PYTHON_HELPERS}{sinks}async def main() -> None:\n{body}\n\nasyncio.run(main())\n"
	)
}

/// What the Python driver's lines call: `tried` digests a `Result`'s value or its error,
/// which only the binding's own exceptions hold; `swapped` a value before and after `&mut`;
/// `held` an object's values through each of its methods.
const PYTHON_HELPERS: &str = "def tried(f: typing.Any, digest: typing.Any) -> str:
	try:
		return f\"ok {digest(f())}\"
	except Exception as e:
		if type(e).__module__ != s.__name__:
			raise
		return f\"err {digest(typing.cast(typing.Any, e).error)}\"


def swapped(make: typing.Any, swap: typing.Any, digest: typing.Any, seed: int, other: int, boxed: bool) -> str:
	v = s.Mut(make(seed)) if boxed else make(seed)
	d = swap(v, other)
	return f\"{d} {digest(v.value if boxed else v)}\"


def held(holder: typing.Any, make: typing.Any, digest: typing.Any, seed: int, other: int) -> str:
	h = holder.new(make(seed))
	old = h.swap(make(other))
	d = h.digest()
	return f\"{digest(old)} {d} {digest(h.into_inner())}\"


";

/// The Swift driver, in the generated module's `main.swift`: each shape's calls in a
/// function of its own, since type-checking them as one body of top-level code takes
/// tens of gigabytes, and `lend`, which lends bytes or an array to a `digest_N` borrowing
/// a slice as the span it takes.
fn swift(calls: &[Call], source: &str) -> String {
	// Whether the generated `func` throws, as one taking a checked handle does.
	let throws = |function: &str| {
		let at = source
			.find(&format!("public func {function}("))
			.unwrap_or_else(|| panic!("the Swift declares {function}"));
		source[at..].lines().next().unwrap().contains(" throws ")
	};
	// `try` where an expression's call throws: a closure, which cannot, aborts instead.
	let tr = |functions: &[String]| match functions.iter().any(|f| throws(f)) {
		true => "try ",
		false => "",
	};
	let closed = |function: String| match throws(&function) {
		true => "try! ",
		false => "",
	};
	let mut sinks = String::new();
	// Each sink's class, defined by its first call the driver runs.
	let mut echoes = std::collections::BTreeSet::new();
	let shapes = by_shape(calls, |call| {
		let Call { n, seed, .. } = call;
		let other = call.other();
		let label = call.label();
		let make = |seed: u64| format!("{}make{n}({seed})", tr(&[format!("make{n}")]));
		// Whether `function` borrows its last argument, a slice, as a span.
		let spans = |function: &str| {
			let at = source.find(&format!("public func {function}(")).unwrap();
			source[at..].lines().next().unwrap().contains("borrowing Swift.Span<")
		};
		let lent = spans(&format!("digest{n}"));
		// `digest_N` of `v`, lent as a span where it borrows a slice.
		let digested = |v: &str| match lent {
			true => format!("lend({v}) {{ {}digest{n}($0) }}", tr(&[format!("digest{n}")])),
			false => format!("digest{n}({v})"),
		};
		let digest = |v: &str| format!("{}{}", tr(&[format!("digest{n}")]), digested(v));
		let value = match call.kind {
			Kind::Try => {
				// What it throws: an error holding the value, or the value, which is one.
				let at = source.find(&format!("public func try{n}(")).unwrap();
				let body = &source[at..at + source[at..].find("\n}\n").unwrap()];
				let thrown = body.split_once("\n\tthrow ").expect("try_N throws").1;
				let (thrown, error) = match thrown.split_once("(error: ") {
					Some((thrown, _)) if !thrown.contains([' ', '(']) => (thrown, "e.error"),
					_ => {
						let declared = body.lines().next().unwrap();
						(declared.split_once("throws -> ").unwrap().1.trim_end_matches(" {"), "e")
					}
				};
				return format!(
					"\tdo {{\n\t\tlet v = try try{n}({seed})\n\t\tprint(\"{label} ok \\({})\")\n\
					\t}} catch let e as {thrown} {{\n\t\tprint(\"{label} err \\({})\")\n\t}}",
					digest("v"),
					digest(error)
				);
			}
			Kind::Swap => {
				// A value is `inout`; an object's handle is borrowed.
				let at = source.find(&format!("public func swap{n}(")).unwrap();
				let (var, arg) = match source[at..].lines().next().unwrap().contains("inout") {
					true => ("var", "&v"),
					false => ("let", "v"),
				};
				return format!(
					"\tdo {{\n\t\t{var} v = {}\n\t\tlet d = {}swap{n}({arg}, {other})\n\t\tprint(\"{label} \\(d) \\({})\")\n\t}}",
					make(seed),
					tr(&[format!("swap{n}")]),
					digest("v")
				);
			}
			Kind::Pair if spans(&format!("pair{n}")) => format!(
				"{}lend({}) {{ {}pair{n}({}, {seed}, $0) }}",
				tr(&[format!("pair{n}"), format!("make{n}")]),
				make(other),
				tr(&[format!("pair{n}")]),
				make(seed)
			),
			Kind::Pair => format!(
				"{}pair{n}({}, {seed}, {})",
				tr(&[format!("pair{n}")]),
				make(seed),
				make(other)
			),
			Kind::Maybe => match seed % 2 {
				0 => format!("{}maybe{n}(nil)", tr(&[format!("maybe{n}")])),
				_ if spans(&format!("maybe{n}")) => format!(
					"{}lend({}) {{ {}maybe{n}($0) }}",
					tr(&[format!("maybe{n}"), format!("make{n}")]),
					make(seed),
					tr(&[format!("maybe{n}")])
				),
				_ => format!("{}maybe{n}({})", tr(&[format!("maybe{n}")]), make(seed)),
			},
			Kind::Held => {
				// Its methods, which throw where they take its checked handle; a struct's
				// `swap` is `mutating`.
				let at = source
					.find(&format!(" Holder{n}: "))
					.unwrap_or_else(|| panic!("the Swift declares Holder{n}"));
				let block = &source[at..at + source[at..].find("\n}\n").unwrap()];
				let method = |name: &str| {
					let line = block.lines().find(|l| l.contains(&format!(" func {name}("))).unwrap();
					(line.contains(" throws "), line.contains(" mutating "))
				};
				let t = |name: &str| if method(name).0 { "try " } else { "" };
				let var = if method("swap").1 { "var" } else { "let" };
				return format!(
					"\tdo {{\n\t\t{var} h = {}Holder{n}.new({})\n\t\tlet old = {}h.swap({})\n\t\tlet d = {}h.digest()\n\
					\t\tlet c = {}h.intoInner()\n\t\tprint(\"{label} \\({}) \\(d) \\({})\")\n\t}}",
					t("new"),
					make(seed),
					t("swap"),
					make(other),
					t("digest"),
					t("intoInner"),
					digest("old"),
					digest("c")
				);
			}
			Kind::Sink => {
				if echoes.insert(n) {
					let protocol = source
						.split(&format!("public protocol Sink{n}: Sendable {{\n"))
						.nth(1)
						.expect("the Swift declares the sink");
					let echo = protocol.lines().next().unwrap().trim();
					let _ = writeln!(
						sinks,
						"struct Echo{n}: Sink{n} {{\n\t{echo} {{\n\t\tprint(\"echoed\")\n\t\treturn v\n\t}}\n}}\n"
					);
				}
				format!("{}sink{n}({seed}, Echo{n}())", tr(&[format!("sink{n}")]))
			}
			Kind::Echo => format!(
				"{}{}",
				tr(&[format!("digest{n}"), format!("echo{n}"), format!("make{n}")]),
				digested(&format!("echo{n}(make{n}({seed}))"))
			),
			Kind::Lent => {
				format!(
					"{}lent{n}({seed}) {{ {}{} }}",
					tr(&[format!("lent{n}")]),
					closed(format!("digest{n}")),
					digested("$0")
				)
			}
			Kind::Peek => {
				format!(
					"{}peek{n}({seed}) {{ {}digest{n}($0) }}",
					tr(&[format!("peek{n}")]),
					closed(format!("digest{n}"))
				)
			}
			Kind::Made => {
				format!(
					"{}made{n}({seed}) {{ {}make{n}($0) }}",
					tr(&[format!("made{n}")]),
					closed(format!("make{n}"))
				)
			}
			// The future throws, so `try` always marks the call.
			Kind::Later => format!("try await {}", digested(&format!("later{n}(make{n}({seed}))"))),
		};
		format!("\tprint(\"{} \\({value})\")", call.label())
	});
	let mut out = String::from(
		"func lend<R>(_ bytes: Bytes, _ body: (Swift.Span<UInt8>) throws -> R) rethrows -> R {\n\ttry bytes.withSpan(body)\n}\n\n\
		func lend<T, R>(_ items: [T], _ body: (Swift.Span<T>) throws -> R) rethrows -> R {\n\tlet slice = items[...]\n\treturn try body(slice.span)\n}\n\n",
	);
	out.push_str(&sinks);
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
	// How the generated `func` returns: its values, and whether an error follows them. A
	// method is named after its receiver, as `(self *T) M`.
	let results = |function: &str| -> (usize, bool) {
		let at = source
			.find(&format!("\nfunc {function}("))
			.unwrap_or_else(|| panic!("the Go declares {function}"));
		let line = source[at + 1..].lines().next().unwrap();
		let mut depth = 0;
		let mut close = 0;
		for (i, c) in line.char_indices().skip(5 + function.len()) {
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
	// The parameter types of a `func`, after its name.
	let params = |function: &str| -> String {
		let at = source
			.find(&format!("\nfunc {function}("))
			.unwrap_or_else(|| panic!("the Go declares {function}"));
		let line = &source[at + 7 + function.len()..];
		line[..line.find(") ").unwrap()].to_string()
	};
	let mut sinks = String::new();
	// Each sink's class, defined by its first call the driver runs.
	let mut echoes = std::collections::BTreeSet::new();
	for call in calls {
		let Call { n, seed, .. } = *call;
		let other = call.other();
		let label = call.label();
		let made = |seed: u64| value(&format!("Make{n}"), format!("s.Make{n}({seed})"));
		let digest = |v: &str| format!("one(s.Digest{n}({v}))");
		// The parameter `v` of `function` takes a value where `Digest` takes it as is: an
		// object's handle, which it borrows.
		let handle = |function: &str| {
			let digested = params(&format!("Digest{n}"));
			params(function)
				.split(", ")
				.any(|p| p.starts_with("v ") && p == digested)
		};
		let printed = match call.kind {
			Kind::Try => {
				let (k, _) = results(&format!("Try{n}"));
				let (values, v) = match k {
					1 => ("v".to_string(), "v".to_string()),
					k => {
						let names: Vec<String> = (0..k).map(|i| format!("v{i}")).collect();
						(names.join(", "), format!("tuple{k}({})", names.join(", ")))
					}
				};
				let lift = if results(&format!("Digest{n}")).1 {
					"liftErr"
				} else {
					"lift"
				};
				let _ = writeln!(
					body,
					"\t{{\n\t\t{values}, err := s.Try{n}({seed})\n\t\tif err == nil {{\n\
					\t\t\tfmt.Println(\"{label} ok\", {})\n\t\t}} else {{\n\
					\t\t\tfmt.Println(\"{label} err\", failed({lift}(s.Digest{n}), err))\n\t\t}}\n\t}}",
					digest(&v)
				);
				continue;
			}
			Kind::Swap => {
				let arg = if handle(&format!("Swap{n}")) { "v" } else { "&v" };
				let _ = writeln!(
					body,
					"\t{{\n\t\tv := {}\n\t\td := one(s.Swap{n}({arg}, {other}))\n\t\tfmt.Println(\"{label}\", d, {})\n\t}}",
					made(seed),
					digest("v")
				);
				continue;
			}
			Kind::Pair => format!("one(s.Pair{n}({}, {seed}, {}))", made(seed), made(other)),
			Kind::Maybe => match (seed % 2, handle(&format!("Maybe{n}"))) {
				(0, _) => format!("one(s.Maybe{n}(nil))"),
				(_, true) => format!("one(s.Maybe{n}({}))", made(seed)),
				(_, false) => format!("one(s.Maybe{n}(ptr({})))", made(seed)),
			},
			Kind::Held => {
				let holder = format!("(self *Holder{n})");
				let swapped = value(&format!("{holder} Swap"), format!("h.Swap({})", made(other)));
				let inner = value(&format!("{holder} IntoInner"), "h.IntoInner()".into());
				let _ = writeln!(
					body,
					"\t{{\n\t\th := one(s.NewHolder{n}({}))\n\t\told := {swapped}\n\t\td := one(h.Digest())\n\
					\t\tc := {inner}\n\t\tif err := h.Close(); err != nil {{\n\t\t\tpanic(err)\n\t\t}}\n\
					\t\tfmt.Println(\"{label}\", {}, d, {})\n\t}}",
					made(seed),
					digest("old"),
					digest("c")
				);
				continue;
			}
			Kind::Sink => {
				if echoes.insert(n) {
					let interface = source
						.split(&format!("\ntype Sink{n} interface {{\n"))
						.nth(1)
						.expect("the Go declares the sink");
					let echo = interface.lines().next().unwrap().trim().replace("p_rt.", "rt.");
					let _ = writeln!(
						sinks,
						"type echo{n} struct{{}}\n\nfunc (echo{n}) {echo} {{\n\tfmt.Println(\"echoed\")\n\treturn v\n}}\n"
					);
				}
				format!("one(s.Sink{n}_({seed}, echo{n}{{}}))")
			}
			Kind::Echo => {
				let made = value(&format!("Make{n}"), format!("s.Make{n}({seed})"));
				let echoed = value(&format!("Echo{n}"), format!("s.Echo{n}({made})"));
				format!("one(s.Digest{n}({echoed}))")
			}
			Kind::Lent | Kind::Peek => {
				let item = if call.kind == Kind::Lent { "Lent" } else { "Peek" };
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
			Kind::Made => {
				let lift = if results(&format!("Make{n}")).1 {
					"liftErr"
				} else {
					"lift"
				};
				format!("s.Made{n}({seed}, {lift}(s.Make{n}))")
			}
			Kind::Later => {
				let made = value(&format!("Make{n}"), format!("s.Make{n}({seed})"));
				let later = value(&format!("Later{n}"), format!("s.Later{n}(ctx, {made})"));
				format!("one(s.Digest{n}({later}))")
			}
		};
		let _ = writeln!(body, "\tfmt.Println(\"{label}\", {printed})");
	}
	format!(
		"package main\n\nimport (\n\t\"context\"\n\t\"fmt\"\n\t\"reflect\"\n\t\"strings\"\n\n\ts \"{name}/{name}\"\n\t\"{name}/rt\"\n)\n\n\
		{sinks}{GO_HELPERS}\
		func one[T any](v T, err ...error) T {{\n\tif len(err) > 0 && err[0] != nil {{\n\t\tpanic(err[0])\n\t}}\n\treturn v\n}}\n\n\
		{tuples}func lift[A, B any](f func(A) B) func(A) B {{\n\treturn f\n}}\n\n\
		func liftErr[A, B any](f func(A) (B, error)) func(A) B {{\n\treturn func(a A) B {{ return one(f(a)) }}\n}}\n\n\
		func bytesIn[B any](f func(rt.Bytes) B) func([]byte) B {{\n\treturn func(v []byte) B {{ return f(rt.NewBytes(v)) }}\n}}\n\n\
		func bytesOptIn[B any](f func(*rt.Bytes) B) func(*[]byte) B {{\n\treturn func(v *[]byte) B {{\n\t\tif v == nil {{\n\t\t\treturn f(nil)\n\t\t}}\n\t\tb := rt.NewBytes(*v)\n\t\treturn f(&b)\n\t}}\n}}\n\n\
		func main() {{\n\tctx := context.Background()\n\t_ = ctx\n{body}}}\n",
		name = built.name(),
	)
}

/// What the Go driver's lines call besides: `ptr` points at a value, as an `Option<&T>`
/// takes it; `failed` is what `f` returns of the value a `Result`'s error holds, an object
/// that is the error or the `Value` of the runtime's error of it.
const GO_HELPERS: &str = "func ptr[T any](v T) *T {
	return &v
}

func failed[A, B any](f func(A) B, err error) B {
	if v, ok := err.(A); ok {
		return f(v)
	}
	e := reflect.ValueOf(err)
	if e.Kind() != reflect.Struct || !strings.HasSuffix(e.Type().Name(), \"Error\") {
		panic(err)
	}
	return f(e.FieldByName(\"Value\").Interface().(A))
}

";

/// The Dart driver, which prints each digest, a `U64`, unsigned. Each shape's calls are
/// a function of their own: in one `main` of the nightly set's 37,000 lines, the VM let
/// a `try350` exception its `on` clause names go uncaught, which alone it catches.
fn dart(built: &Built, calls: &[Call]) -> String {
	let name = built.name();
	let source = std::fs::read_to_string(built.out.join("dart").join(format!("{name}.dart"))).unwrap();
	let mut sinks = String::new();
	// Each sink's class, defined by its first call the driver runs.
	let mut echoes = std::collections::BTreeSet::new();
	let mut body = String::new();
	let mut shapes = Vec::new();
	for call in calls {
		if shapes.last() != Some(&call.n) {
			if !shapes.is_empty() {
				body.push_str("}\n\n");
			}
			let _ = writeln!(body, "Future<void> shape{}() async {{", call.n);
			shapes.push(call.n);
		}
		// A seed as the `int` of its bits.
		let u = |seed: u64| format!("s.U64.fromBits({})", seed as i64);
		let Call { n, seed, .. } = *call;
		let other = call.other();
		let label = call.label();
		let make = |seed: u64| format!("s.make{n}({})", u(seed));
		let digest = |v: &str| format!("${{s.digest{n}({v}).toRadixString(10)}}");
		let value = match call.kind {
			Kind::Echo => format!("s.digest{n}(s.echo{n}({}))", make(seed)),
			Kind::Lent => format!("s.lent{n}({}, s.digest{n})", u(seed)),
			Kind::Peek => format!("s.peek{n}({}, s.digest{n})", u(seed)),
			Kind::Made => format!("s.made{n}({}, s.make{n})", u(seed)),
			Kind::Later => format!("s.digest{n}(await s.later{n}({}))", make(seed)),
			Kind::Try => {
				// What it throws: an exception holding the value, or the value, which is one.
				let declared = declaration(&source, &format!("try{n}("));
				let ty = declared.split_once(" try").unwrap().0;
				let at = source.find(declared).unwrap();
				let body_ = &source[at..at + source[at..].find("\n}\n").unwrap()];
				let thrown = body_.split_once("throw ").expect("try_N throws").1;
				let thrown = &thrown[..thrown.find(['(', '.']).unwrap()];
				let wraps = source.contains(&format!("\nfinal class {thrown} implements Exception {{\n\tfinal "));
				let (thrown, error) = match wraps {
					true => (thrown, "e.error"),
					false => (ty, "e"),
				};
				let _ = writeln!(
					body,
					"  try {{\n    final v = s.try{n}({});\n    print('{label} ok {}');\n  }} on s.{thrown} catch (e) {{\n    print('{label} err {}');\n  }}",
					u(seed),
					digest("v"),
					digest(error)
				);
				continue;
			}
			Kind::Swap => {
				// A value is passed in a `Mut`, which Rust writes back; an object is borrowed.
				let boxed = declaration(&source, &format!("swap{n}(")).contains("(Mut<");
				let (v, after) = match boxed {
					true => (format!("s.Mut({})", make(seed)), "v.value"),
					false => (make(seed), "v"),
				};
				let _ = writeln!(
					body,
					"  {{\n    final v = {v};\n    final d = s.swap{n}(v, {});\n    print('{label} ${{d.toRadixString(10)}} {}');\n  }}",
					u(other),
					digest(after)
				);
				continue;
			}
			Kind::Pair => format!("s.pair{n}({}, {}, {})", make(seed), u(seed), make(other)),
			Kind::Maybe => match seed % 2 {
				0 => format!("s.maybe{n}(null)"),
				_ => format!("s.maybe{n}({})", make(seed)),
			},
			Kind::Held => {
				let _ = writeln!(
					body,
					"  {{\n    final h = s.Holder{n}({});\n    final old = h.swap({});\n    final d = h.digest();\n\
					\x20   final c = h.intoInner();\n    h.close();\n    print('{label} {} ${{d.toRadixString(10)}} {}');\n  }}",
					make(seed),
					make(other),
					digest("old"),
					digest("c")
				);
				continue;
			}
			Kind::Sink => {
				if echoes.insert(n) {
					let class = source
						.split(&format!("\nabstract class Sink{n} {{\n"))
						.nth(1)
						.expect("the Dart declares the sink");
					// Its types, the binding's own prefixed.
					let echo = class.lines().next().unwrap().trim().trim_end_matches(';');
					let mut prefixed = String::new();
					for (i, c) in echo.char_indices() {
						let starts = !echo[..i].ends_with(|p: char| p.is_alphanumeric() || p == '_' || p == '.');
						if starts && c.is_uppercase() && !echo[i..].starts_with("String") {
							prefixed.push_str("s.");
						}
						prefixed.push(c);
					}
					let echo = prefixed;
					let _ = writeln!(
						sinks,
						"final class Echo{n} extends s.Sink{n} {{\n  @override\n  {echo} {{\n    print('echoed');\n    return v;\n  }}\n}}\n"
					);
				}
				format!("s.sink{n}({}, Echo{n}())", u(seed))
			}
		};
		let _ = writeln!(body, "  print('{label} ${{({value}).toRadixString(10)}}');");
	}
	if !shapes.is_empty() {
		body.push_str("}\n\n");
	}
	let main: String = shapes.iter().map(|n| format!("  await shape{n}();\n")).collect();
	format!("import 'package:{name}/{name}.dart' as s;\n\n{sinks}{body}Future<void> main() async {{\n{main}}}\n")
}

/// The web driver, run by Node over the module the export's `web` feature builds, which it
/// loads from `RUST2_WASM`: each digest, a `u64`, in decimal.
fn web(built: &Built, calls: &[Call]) -> String {
	let source = std::fs::read_to_string(built.out.join("web").join("index.ts")).unwrap();
	let mut holders = std::collections::BTreeSet::new();
	let mut body = String::new();
	for call in calls {
		let Call { n, seed, .. } = *call;
		let other = call.other();
		let label = call.label();
		let u = |seed: u64| format!("rt.u64FromBig({seed}n)");
		let make = |seed: u64| format!("s.make{n}({})", u(seed));
		let digest = |v: &str| format!("${{dec(s.digest{n}({v}))}}");
		let value = match call.kind {
			Kind::Echo => format!("s.digest{n}(s.echo{n}({}))", make(seed)),
			Kind::Lent => format!("s.lent{n}({}, (v) => s.digest{n}(v))", u(seed)),
			Kind::Peek => format!("s.peek{n}({}, (v) => s.digest{n}(v))", u(seed)),
			Kind::Made => format!("s.made{n}({}, (seed) => s.make{n}(seed))", u(seed)),
			Kind::Later => format!("s.digest{n}(await s.later{n}({}))", make(seed)),
			Kind::Try => {
				let _ = writeln!(
					body,
					"{{\n\tconst r = s.try{n}({});\n\tprint(r.ok ? `{label} ok {}` : `{label} err {}`);\n}}",
					u(seed),
					digest("r.value"),
					digest("r.error")
				);
				continue;
			}
			Kind::Swap => {
				// A value is passed in a place, which Rust writes back; an object is borrowed.
				let declared = declaration(&source, &format!("swap{n}("));
				let (v, after) = match declared.contains("v: rt.Place<") {
					true => (format!("{{ v: plain({}) }}", make(seed)), "v.v"),
					false => (make(seed), "v"),
				};
				let _ = writeln!(
					body,
					"{{\n\tconst v = {v};\n\tconst d = s.swap{n}(v, {});\n\tprint(`{label} ${{dec(d)}} {}`);\n}}",
					u(other),
					digest(after)
				);
				continue;
			}
			Kind::Pair => format!("s.pair{n}({}, {}, {})", make(seed), u(seed), make(other)),
			Kind::Maybe => match seed % 2 {
				0 => format!("s.maybe{n}(undefined)"),
				_ => format!("s.maybe{n}({})", make(seed)),
			},
			Kind::Held => {
				holders.insert(n);
				let _ = writeln!(
					body,
					"{{\n\tconst h = Holder{n}.new({});\n\tconst old = Holder{n}.swap(h, {});\n\tconst d = Holder{n}.digest(h);\n\
					\tconst c = Holder{n}.intoInner(h);\n\th[Symbol.dispose]();\n\tprint(`{label} {} ${{dec(d)}} {}`);\n}}",
					make(seed),
					make(other),
					digest("old"),
					digest("c")
				);
				continue;
			}
			Kind::Sink => format!(
				"s.sink{n}({}, {{ echo: (v) => {{ print(\"echoed\"); return v; }} }})",
				u(seed)
			),
		};
		let _ = writeln!(body, "print(`{label} ${{dec({value})}}`);");
	}
	let imports: String = holders
		.iter()
		.map(|n| format!("import * as Holder{n} from \"./web/Holder{n}.ts\";\n"))
		.collect();
	format!(
		"{WEB_HEAD}{imports}\nawait s.load(process.getBuiltinModule(\"node:fs\").readFileSync(process.env.RUST2_WASM!));\n\n{body}"
	)
}

/// The web driver's head: what it imports, and Node's globals, which the binding's types
/// leave out.
const WEB_HEAD: &str = "import * as rt from \"./web/rt.ts\";
import * as s from \"./web/index.ts\";

declare const process: {
	readonly env: Record<string, string | undefined>;
	getBuiltinModule(id: \"node:fs\"): { readFileSync(path: string): Uint8Array<ArrayBuffer> };
};

const print = (line: string) => console.log(line);

/** A `u64` in decimal. */
const dec = (v: rt.u64) => (typeof v === \"number\" ? `${v}` : `${rt.u64ToBig(v)}`);

/** `v` as a place holds it: the bytes Rust returned copied into arrays. */
type Plain<T> = T extends rt.Bytes
	? Uint8Array<ArrayBuffer>
	: T extends readonly unknown[]
		? { -readonly [K in keyof T]: Plain<T[K]> }
		: T;

function plain<T>(v: T): Plain<T> {
	if (v instanceof rt.Bytes) return v.slice() as Plain<T>;
	if (Array.isArray(v)) return v.map(plain) as Plain<T>;
	return v as Plain<T>;
}

";

/// Build and run `language`'s driver of `built`, checking it against the native transcript.
fn round_trip(built: &Built, language: &str) {
	// The test that drew the shapes may have passed, its message captured: each failing one
	// names the seed again.
	if let Some((_, seed)) = built.krate.drawn {
		eprintln!("shapes past the fixed set drawn from RUST2_SHAPES_SEED={seed}");
	}
	let name = built.name();
	let dir = built.dir(language);
	let exe = dir.join(format!("shapes{}", std::env::consts::EXE_SUFFIX));
	let transcript = match language {
		"web" => built.wasm_transcript(),
		_ => &built.krate.transcript,
	};
	let expected = built.expected(language, transcript);
	let calls: Vec<Call> = expected.iter().map(|(call, _)| *call).collect();
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
				generated: None,
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
				// The nightly's binding is past what kotlinc's default heap of 512 MB compiles.
				Command::new("kotlinc")
					.args(["-J-Xmx4g", "-Werror", "-cp"])
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
			let source = std::fs::read_to_string(&generated).unwrap();
			step::write(&main, swift(calls, &source));
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
			super::go_library(&built.staticlib(), &export);
			let source = std::fs::read_to_string(export.join(&name).join(format!("{name}.go"))).unwrap();
			let rt = std::fs::read_to_string(export.join("rt").join("types.go")).unwrap_or_default();
			step::write(&module.join("main.go"), go(built, calls, &source, &rt));
			step::write(
				&module.join("go.mod"),
				format!("module shapestest\n\ngo 1.24\n\nrequire {name} v0.0.0\n\nreplace {name} => ./export\n"),
			);
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
					.env("GOFLAGS", "-buildvcs=false")
					.env("GOTOOLCHAIN", "local"),
			);
			run(&mut Command::new(&exe))
		}
		"dart" => {
			let main = dir.join("main.dart");
			step::write(&main, dart(built, calls));
			let lib = built.lib.join(super::shared(&name));
			let dill = super::dart_compile(&name, &dir, &built.out.join("dart"), &lib, &main, &[]);
			run(&mut super::dart_run(&dill, &[]))
		}
		"web" => {
			let (module, _) = built.web();
			let main = dir.join("main.ts");
			step::mirror(&built.out.join("web"), &dir.join("web"));
			step::write(&main, web(built, calls));
			// Type-checked as a caller's project would be, then run by Node.
			let config = dir.join("tsconfig.json");
			step::write(&config, super::TSCONFIG.replace("\"api.ts\"", "\"main.ts\""));
			let checked = dir.join("checked");
			let web = dir.join("web");
			let step = Step {
				stamp: &step::stamp(&checked),
				reads: &[&main, &web, &config],
				writes: &[],
			};
			step.run(Command::new("tsc").arg("-p").arg(&dir));
			run(Command::new("node")
				.arg("main.ts")
				.current_dir(&dir)
				.env("RUST2_WASM", module))
		}
		_ => unreachable!("no driver for {language}"),
	};
	built.check(language, &expected, &output);
}

/// The gate's subset of shapes, built once.
fn gated() -> &'static Built {
	static BUILT: std::sync::OnceLock<Built> = std::sync::OnceLock::new();
	BUILT.get_or_init(|| build("shapes", &gate(), None))
}

/// The nightly's shapes, built once: the fixed set, then as many again drawn from the
/// day's seed.
fn sampled() -> &'static Built {
	static BUILT: std::sync::OnceLock<Built> = std::sync::OnceLock::new();
	BUILT.get_or_init(|| {
		let mut all = shapes(SAMPLES);
		let fixed = all.len();
		let seed = day_seed();
		eprintln!("the nightly's new shapes are drawn from seed {seed}: RUST2_SHAPES_SEED={seed} reproduces them");
		all.extend(drawn(seed, SAMPLES));
		build("sampled", &all, Some((fixed, seed)))
	})
}

/// How many shapes the nightly samples from each seed.
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

languages!(c, cpp, kotlin, python, swift, go, dart, web);
