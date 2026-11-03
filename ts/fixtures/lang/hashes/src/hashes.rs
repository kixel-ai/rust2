//! What `Hash` writes of std's types and the crate's into a `DefaultHasher` keyed as
//! `DefaultHasher::new` keys it, whose values must be native's byte for byte, and
//! `RandomState`, whose keys are random on both sides: its case prints only what holds
//! whatever they are, such as one state hashing equal values alike, and never a hash. A
//! `Hasher` of the crate's own logs each call `Hash` makes of it. Then hash maps and sets
//! that hold a `BuildHasher` of the crate's own, and their `==`, and `BuildHasherDefault`
//! of a `Hasher` of the crate's own.

use std::cell::{Cell, RefCell};
use std::cmp::Reverse;
use std::collections::{HashMap, HashSet};
use std::hash::{BuildHasher, BuildHasherDefault, DefaultHasher, Hash, Hasher, RandomState};
use std::num::NonZero;
use std::sync::Arc;
use std::time::Duration;

use crate::harness::{put, text};

#[derive(Hash)]
struct Packet {
	id: u64,
	flags: u8,
	name: String,
	tags: Vec<u16>,
	next: Option<i32>,
}

#[derive(Hash)]
enum Shape {
	Dot,
	Line(u32),
	Rect { w: i16, h: i16 },
}

/// A fieldless enum, which is a JS string, hashed by its `isize` discriminant.
#[derive(Clone, Copy, Hash)]
enum Dir {
	Up,
	Down,
}

/// A `u8` discriminant, which the derive writes as a byte.
#[derive(Clone, Copy, Hash)]
#[repr(u8)]
enum Small {
	Low = 3,
	High = 200,
}

/// A key whose `Hash` sees only its low nibble.
struct Nibble(u8);

impl Hash for Nibble {
	fn hash<H: Hasher>(&self, state: &mut H) {
		(self.0 & 0xf).hash(state);
	}
}

/// `DefaultHasher::new`'s hash of `value`, little-endian.
fn digest<T: Hash + ?Sized>(value: &T, out: &mut Vec<u8>) {
	let mut h = DefaultHasher::new();
	value.hash(&mut h);
	out.extend_from_slice(&h.finish().to_le_bytes());
}

/// The bytes of `data` from `at`, as many as fit, as one big-endian number.
fn wide(data: &[u8], at: usize, n: usize) -> u128 {
	let mut v = 0u128;
	for i in at..at + n {
		v = (v << 8) | data.get(i).copied().unwrap_or(0) as u128;
	}
	v
}

/// The hash of a value of each type `kind` picks, made of `data`.
pub fn values(kind: u8, data: &[u8], out: &mut Vec<u8>) {
	let w = |at: usize, n: usize| wide(data, at, n);
	match kind % 5 {
		0 => {
			let (a, b) = (w(0, 16), w(3, 8) as u64);
			digest(&(a as u8), out);
			digest(&(a as u16), out);
			digest(&(a as u32), out);
			digest(&b, out);
			// Past 2^53 or not, by the top byte.
			digest(&(b >> (data.len() % 64)), out);
			digest(&a, out);
			digest(&(a as i8), out);
			digest(&(a as i16), out);
			digest(&(a as i32), out);
			digest(&(b as i64), out);
			digest(&((b as i64) >> 11), out);
			digest(&(a as i128), out);
			digest(&((a as i128) >> 70), out);
			digest(&(a as usize), out);
			digest(&(a as isize), out);
			digest(&(a & 1 == 1), out);
			digest(&char::from_u32(a as u32 & 0x1fffff).unwrap_or('\u{10ffff}'), out);
		}
		1 => {
			let text = std::str::from_utf8(data).unwrap_or("é ✓ 😀");
			digest(text, out);
			digest(&text.to_string(), out);
			digest("", out);
			digest(&(text, text.len()), out);
			let arc: Arc<str> = Arc::from(text);
			digest(&arc, out);
		}
		2 => {
			let n = w(0, 4) as u32;
			digest(&Some(n), out);
			digest(&None::<u32>, out);
			digest(&Some(Some(n as u8)), out);
			digest(&(n as u8, "pair", w(4, 8) as u64), out);
			let words: Vec<u16> = data.iter().map(|&b| b as u16 * 257).collect();
			digest(&words, out);
			digest(&words[words.len() / 2..], out);
			digest(&[n, n ^ 1, n >> 3], out);
			digest(&(n as u8).cmp(&(w(4, 1) as u8)), out);
			digest(&Duration::new(w(0, 6) as u64, n % 1_000_000_000), out);
			digest(&Reverse(n as u16), out);
			digest(&NonZero::new(n | 1), out);
		}
		3 => {
			let n = w(0, 8) as u64;
			let packet = Packet {
				id: n,
				flags: data.first().copied().unwrap_or(0),
				name: std::str::from_utf8(data).unwrap_or("?").to_string(),
				tags: data.iter().map(|&b| b as u16).collect(),
				next: (n % 3 != 0).then_some(n as i32),
			};
			digest(&packet, out);
			let shape = match n % 3 {
				0 => Shape::Dot,
				1 => Shape::Line(n as u32),
				_ => Shape::Rect {
					w: n as i16,
					h: (n >> 16) as i16,
				},
			};
			digest(&shape, out);
			digest(&[Dir::Up, Dir::Down], out);
			digest(&(Small::Low, Small::High), out);
			digest(&Nibble(n as u8), out);
		}
		_ => {
			// Writes of each width and of bytes, which fill and split the hasher's 8-byte tail, and
			// `finish` between them, which leaves the hasher as it was.
			let mut h = DefaultHasher::new();
			for (i, &b) in data.iter().enumerate() {
				let v = w(i, 16);
				match b % 14 {
					0 => h.write_u8(v as u8),
					1 => h.write_u16(v as u16),
					2 => h.write_u32(v as u32),
					3 => h.write_u64(v as u64),
					4 => h.write_u128(v),
					5 => h.write_usize(v as usize),
					6 => h.write_i8(v as i8),
					7 => h.write_i16(v as i16),
					8 => h.write_i32(v as i32),
					9 => h.write_i64(v as i64),
					10 => h.write_i128(v as i128),
					11 => h.write_isize(v as isize),
					12 => h.write(&data[i..]),
					_ => out.extend_from_slice(&h.finish().to_le_bytes()),
				}
			}
			out.extend_from_slice(&h.finish().to_le_bytes());
		}
	}
}

/// A value that counts its drops, hashed as its byte.
struct Counted<'a>(u8, &'a Cell<u8>);

impl Hash for Counted<'_> {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.0.hash(state);
	}
}

impl Drop for Counted<'_> {
	fn drop(&mut self) {
		self.1.set(self.1.get() + 1);
	}
}

/// What holds of `RandomState` whatever its keys: one state, its clone, and the hasher it
/// builds hash a value alike, through a reference too, and values `Hash` sees as equal
/// alike, so ordering by its hashes leaves the same set. `hash_one` drops what it takes.
pub fn random(data: &[u8], out: &mut Vec<u8>) {
	#[derive(Default)]
	struct Priorities {
		state: RandomState,
	}
	let s = RandomState::new();
	let copy = s.clone();
	let other = Priorities::default();
	for (i, &b) in data.iter().enumerate() {
		let n = wide(data, i, 8) as u64;
		let text = std::str::from_utf8(&data[i..]).unwrap_or("✓");
		let mut built = s.build_hasher();
		n.hash(&mut built);
		out.push((s.hash_one(n) == built.finish()) as u8);
		out.push((s.hash_one(n) == copy.hash_one(n)) as u8);
		out.push((s.hash_one(n) == s.hash_one(&n)) as u8);
		out.push((other.state.hash_one(text) == other.state.hash_one(text.to_string())) as u8);
		out.push((s.hash_one((b, text)) == copy.hash_one(&(b, text))) as u8);
		out.push((s.hash_one(Nibble(b)) == s.hash_one(Nibble(b ^ 0xf0))) as u8);
		let arc: Arc<str> = Arc::from(text);
		out.push((s.hash_one(arc.clone()) == s.hash_one(&arc)) as u8);
		out.push(Arc::strong_count(&arc) as u8);
		let drops = Cell::new(0);
		let counted = Counted(b, &drops);
		out.push((s.hash_one(&counted) == s.hash_one(Counted(b, &drops))) as u8);
		out.push(drops.get());
		drop(counted);
		out.push(drops.get());
	}
	let mut keys: Vec<u64> = data.iter().map(|&b| b as u64 * 0x0101_0101_0101).collect();
	keys.sort_by_key(|k| other.state.hash_one(k));
	keys.sort();
	for k in keys {
		out.extend_from_slice(&k.to_le_bytes());
	}
}

/// What `Hash` writes of bytes, which is alike in each form, a `[u8; N]`, a `&[u8]`, or a
/// `Vec<u8>`: its length as a `usize`, then the bytes. Held in a tuple, an `Option`, and an
/// `Arc` too, and through `RandomState`'s `hash_one`, which hashes each form alike.
pub fn bytes(data: &[u8], out: &mut Vec<u8>) {
	let cut = data.len().min(5);
	for i in 0..=cut {
		let s = &data[..i];
		digest(s, out);
		digest(&s.to_vec(), out);
		digest(&(s, i as u8), out);
		digest(&Some(s.to_vec()), out);
		digest(&Arc::new(s.to_vec()), out);
	}
	let empty: [u8; 0] = [];
	digest(&empty, out);
	digest(&(empty, Some(empty)), out);
	let state = RandomState::new();
	for w in data.windows(3) {
		let a = [w[0], w[1], w[2]];
		let pair = [w[0] % 2, w[1] % 2];
		digest(&a, out);
		digest(&pair, out);
		digest(&(pair, a), out);
		digest(&Some(pair), out);
		digest(&Arc::new(a), out);
		out.push((state.hash_one(a) == state.hash_one(&w[..3])) as u8);
		out.push((state.hash_one(&a) == state.hash_one(w.to_vec())) as u8);
		out.push((state.hash_one((pair, 1u8)) == state.hash_one((&w[..2], 1u8))) as u8);
	}
	out.push((state.hash_one(empty) == state.hash_one(Vec::<u8>::new())) as u8);
}

/// Log a call of `tag`'s method, given `bytes`, as its tag, their count, and them.
fn record(log: &RefCell<Vec<u8>>, tag: u8, bytes: &[u8]) {
	let mut log = log.borrow_mut();
	log.push(tag);
	log.push(bytes.len() as u8);
	log.extend_from_slice(bytes);
}

/// A `Hasher` of the crate's own that writes every method itself, logging each call, and
/// its drop, as `finish` returns what it logged so far.
struct Full<'a>(&'a RefCell<Vec<u8>>);

impl Hasher for Full<'_> {
	fn finish(&self) -> u64 {
		record(self.0, b'f', &[]);
		self.0.borrow().len() as u64 * 0x9e37_79b9_7f4a_7c15
	}

	fn write(&mut self, bytes: &[u8]) {
		record(self.0, b'w', bytes);
	}

	fn write_u8(&mut self, i: u8) {
		record(self.0, 1, &i.to_le_bytes());
	}

	fn write_u16(&mut self, i: u16) {
		record(self.0, 2, &i.to_le_bytes());
	}

	fn write_u32(&mut self, i: u32) {
		record(self.0, 3, &i.to_le_bytes());
	}

	fn write_u64(&mut self, i: u64) {
		record(self.0, 4, &i.to_le_bytes());
	}

	fn write_u128(&mut self, i: u128) {
		record(self.0, 5, &i.to_le_bytes());
	}

	fn write_usize(&mut self, i: usize) {
		record(self.0, 6, &i.to_le_bytes());
	}

	fn write_i8(&mut self, i: i8) {
		record(self.0, 7, &i.to_le_bytes());
	}

	fn write_i16(&mut self, i: i16) {
		record(self.0, 8, &i.to_le_bytes());
	}

	fn write_i32(&mut self, i: i32) {
		record(self.0, 9, &i.to_le_bytes());
	}

	fn write_i64(&mut self, i: i64) {
		record(self.0, 10, &i.to_le_bytes());
	}

	fn write_i128(&mut self, i: i128) {
		record(self.0, 11, &i.to_le_bytes());
	}

	fn write_isize(&mut self, i: isize) {
		record(self.0, 12, &i.to_le_bytes());
	}

	fn write_length_prefix(&mut self, len: usize) {
		record(self.0, b'p', &len.to_le_bytes());
	}

	fn write_str(&mut self, s: &str) {
		record(self.0, b's', s.as_bytes());
	}
}

impl Drop for Full<'_> {
	fn drop(&mut self) {
		record(self.0, b'd', &[]);
	}
}

/// A `Hasher` of the crate's own that writes only `write` and `finish`, so every other
/// method is std's, which writes through them.
struct Bare<'a>(&'a RefCell<Vec<u8>>);

impl Hasher for Bare<'_> {
	fn finish(&self) -> u64 {
		self.0.borrow().len() as u64
	}

	fn write(&mut self, bytes: &[u8]) {
		record(self.0, b'w', bytes);
	}
}

/// A `Hasher` of the crate's own that writes some of the unsigned methods, through which
/// std's signed ones, length prefix, and `write_str` write.
struct Partial<'a>(&'a RefCell<Vec<u8>>);

impl Hasher for Partial<'_> {
	fn finish(&self) -> u64 {
		self.0.borrow().len() as u64
	}

	fn write(&mut self, bytes: &[u8]) {
		record(self.0, b'w', bytes);
	}

	fn write_u8(&mut self, i: u8) {
		record(self.0, 1, &[i]);
	}

	fn write_u64(&mut self, i: u64) {
		record(self.0, 4, &i.to_le_bytes());
	}

	fn write_usize(&mut self, i: usize) {
		record(self.0, 6, &i.to_le_bytes());
	}
}

/// What builds a [`Full`], logging that it did.
struct Build<'a>(&'a RefCell<Vec<u8>>);

impl<'a> BuildHasher for Build<'a> {
	type Hasher = Full<'a>;

	fn build_hasher(&self) -> Full<'a> {
		record(self.0, b'b', &[]);
		Full(self.0)
	}
}

/// A value whose `Hash` calls a method std provides of the hasher, and logs its drop.
struct Logged<'a>(i16, &'a RefCell<Vec<u8>>);

impl Hash for Logged<'_> {
	fn hash<H: Hasher>(&self, state: &mut H) {
		state.write_i16(self.0);
		state.write_length_prefix(2);
	}
}

impl Drop for Logged<'_> {
	fn drop(&mut self) {
		record(self.1, b'D', &[]);
	}
}

/// A `Hasher` of the crate's own that JS copies: an enum that logs each write until a
/// `write_u8` of a multiple of four switches it to summing what follows, so each write
/// through its `&mut` must land in the original, its variant too.
#[derive(Clone)]
enum Switch<'a> {
	Log(&'a RefCell<Vec<u8>>),
	Sum(&'a RefCell<Vec<u8>>, u64),
}

impl Hasher for Switch<'_> {
	fn finish(&self) -> u64 {
		match *self {
			Switch::Log(log) => {
				record(log, b'f', &[]);
				log.borrow().len() as u64
			}
			Switch::Sum(log, sum) => {
				record(log, b'F', &sum.to_le_bytes());
				sum
			}
		}
	}

	fn write(&mut self, bytes: &[u8]) {
		match self {
			Switch::Log(log) => record(log, b'w', bytes),
			Switch::Sum(_, sum) => {
				for &b in bytes {
					*sum = sum.wrapping_mul(31).wrapping_add(b as u64);
				}
			}
		}
	}

	fn write_u8(&mut self, i: u8) {
		match *self {
			Switch::Log(log) if i % 4 == 0 => *self = Switch::Sum(log, i as u64),
			Switch::Log(log) => record(log, 1, &[i]),
			Switch::Sum(_, ref mut sum) => *sum ^= (i as u64) << 56,
		}
	}
}

impl Drop for Switch<'_> {
	fn drop(&mut self) {
		let (Switch::Log(log) | Switch::Sum(log, _)) = *self;
		record(log, b'd', &[]);
	}
}

/// What builds a [`Switch`] by cloning the one it holds, logging that it did.
struct Seeded<'a>(Switch<'a>);

impl<'a> BuildHasher for Seeded<'a> {
	type Hasher = Switch<'a>;

	fn build_hasher(&self) -> Switch<'a> {
		let (Switch::Log(log) | Switch::Sum(log, _)) = self.0;
		record(log, b'b', &[]);
		self.0.clone()
	}
}

/// A [`Switch`] held in a field, hashed into through a `&mut` to the field.
struct Holder<'a> {
	tag: u8,
	h: Switch<'a>,
}

/// Hash values of std's types and the crate's, made of `data`, into `h`.
fn hash_into<H: Hasher>(h: &mut H, data: &[u8]) {
	let w = |at: usize, n: usize| wide(data, at, n);
	let (a, b) = (w(0, 16), w(3, 8) as u64);
	(a as u8).hash(h);
	(a as u16).hash(h);
	(a as u32).hash(h);
	b.hash(h);
	(b >> (data.len() % 64)).hash(h);
	a.hash(h);
	(a as i8).hash(h);
	(a as i16).hash(h);
	(a as i32).hash(h);
	(b as i64).hash(h);
	((b as i64) >> 11).hash(h);
	(a as i128).hash(h);
	((a as i128) >> 70).hash(h);
	(a as usize).hash(h);
	(a as isize).hash(h);
	(a & 1 == 1).hash(h);
	char::from_u32(a as u32 & 0x1fffff).unwrap_or('\u{10ffff}').hash(h);
	let text = std::str::from_utf8(data).unwrap_or("é ✓ 😀");
	text.hash(h);
	text.to_string().hash(h);
	().hash(h);
	(text, a as u8, Some(b)).hash(h);
	None::<u32>.hash(h);
	Arc::<str>::from(text).hash(h);
	data.hash(h);
	data.to_vec().hash(h);
	[a as u8, b as u8].hash(h);
	let words: Vec<u16> = data.iter().map(|&b| b as u16 * 257).collect();
	words.hash(h);
	words[words.len() / 2..].hash(h);
	[b, b << 40, u64::MAX - b].hash(h);
	[a as i128, -(a as i128)].hash(h);
	[a as isize, -1].hash(h);
	[a & 1 == 0, true].hash(h);
	['é', 'x'].hash(h);
	[&(a as u16), &3].hash(h);
	vec![(a as u8, "pair"), (0, "")].hash(h);
	(a as u8).cmp(&(b as u8)).hash(h);
	Duration::new(b, a as u32 % 1_000_000_000).hash(h);
	Reverse(a as u16).hash(h);
	NonZero::new(a as u32 | 1).hash(h);
	(a as u8..b as u8).hash(h);
	(a as u8..=b as u8).hash(h);
	Packet {
		id: b,
		flags: a as u8,
		name: text.to_string(),
		tags: words,
		next: (b % 3 != 0).then_some(b as i32),
	}
	.hash(h);
	Shape::Rect { w: a as i16, h: -1 }.hash(h);
	[Dir::Up, Dir::Down].hash(h);
	(Small::Low, Small::High).hash(h);
	Nibble(a as u8).hash(h);
	h.write_i32(a as i32);
	h.write_isize(-(a as isize));
	h.write_i64(-(b as i64));
}

/// What `Hash` writes of std's types and the crate's into a `Hasher` of the crate's own,
/// call by call: one that writes every method itself, one that writes only `write`, and
/// one that writes some, through which std's others write. Then `BuildHasher::hash_one`
/// of one, which builds its hasher, hashes, finishes, and drops the hasher, then the value.
pub fn own(data: &[u8], out: &mut Vec<u8>) {
	let log = RefCell::new(Vec::new());
	let mut full = Full(&log);
	hash_into(&mut full, data);
	let finish = full.finish();
	drop(full);
	let mut bare = Bare(&log);
	hash_into(&mut bare, data);
	let mut partial = Partial(&log);
	hash_into(&mut partial, data);
	let build = Build(&log);
	let text = std::str::from_utf8(data).unwrap_or("✓");
	let n = wide(data, 0, 2) as u16;
	let hashes = [
		finish,
		bare.finish(),
		partial.finish(),
		build.hash_one(&n),
		build.hash_one(text),
		build.hash_one(&Arc::<str>::from(text)),
		build.hash_one((n as u8, text)),
		build.hash_one(Logged(n as i16, &log)),
		build.hash_one(&Logged(-1, &log)),
	];
	for h in hashes {
		out.extend_from_slice(&h.to_le_bytes());
	}
	out.extend_from_slice(&log.into_inner());
}

/// What `Hash` writes into a `Hasher` of the crate's own that JS copies, call by call, and
/// what it then holds: one bound to a local, one held in a field, and one
/// `BuildHasher::hash_one` builds of one held in the state, which drops before the value.
pub fn copied(data: &[u8], out: &mut Vec<u8>) {
	let log = RefCell::new(Vec::new());
	let n = wide(data, 0, 2) as u16;
	let mut local = Switch::Log(&log);
	hash_into(&mut local, data);
	let mut holder = Holder {
		tag: n as u8,
		h: Switch::Log(&log),
	};
	holder.h.write_u8(n as u8 | 1);
	n.hash(&mut holder.h);
	hash_into(&mut holder.h, &data[data.len() / 2..]);
	let text = std::str::from_utf8(data).unwrap_or("✓");
	let seeded = Seeded(Switch::Log(&log));
	let summing = Seeded(Switch::Sum(&log, n as u64));
	let hashes = [
		local.finish(),
		holder.h.finish(),
		holder.tag as u64,
		seeded.hash_one(&n),
		seeded.hash_one(text),
		seeded.hash_one((n as u8, text)),
		seeded.hash_one(Logged(n as i16, &log)),
		summing.hash_one(&Logged(-1, &log)),
		summing.hash_one(data),
	];
	drop(local);
	drop(holder);
	drop((seeded, summing));
	for h in hashes {
		out.extend_from_slice(&h.to_le_bytes());
	}
	out.extend_from_slice(&log.into_inner());
}

thread_local! {
	/// What each [`Numbered`] and [`Noisy`] did, in order.
	static EVENTS: RefCell<Vec<u8>> = const { RefCell::new(Vec::new()) };
}

/// Log `what` happened to the value numbered `n`.
fn event(what: u8, n: u8) {
	EVENTS.with(|events| events.borrow_mut().extend([what, n]));
}

/// FNV-1a, a `Hasher` of the crate's own that logs nothing, so a map may call it as often
/// as it likes.
struct Fnv(u64);

impl Default for Fnv {
	fn default() -> Self {
		Fnv(0xcbf2_9ce4_8422_2325)
	}
}

impl Hasher for Fnv {
	fn finish(&self) -> u64 {
		self.0
	}

	fn write(&mut self, bytes: &[u8]) {
		for &b in bytes {
			self.0 = (self.0 ^ u64::from(b)).wrapping_mul(0x0100_0000_01b3);
		}
	}
}

/// A `BuildHasher` of the crate's own, numbered, which logs each one `Default` makes, each
/// clone, and each drop, and seeds its [`Fnv`] by its number.
struct Numbered(u8);

impl BuildHasher for Numbered {
	type Hasher = Fnv;

	fn build_hasher(&self) -> Fnv {
		Fnv(0xcbf2_9ce4_8422_2325 ^ u64::from(self.0))
	}
}

impl Default for Numbered {
	fn default() -> Self {
		event(b'n', 0);
		Numbered(0)
	}
}

impl Clone for Numbered {
	fn clone(&self) -> Self {
		event(b'c', self.0);
		Numbered(self.0.wrapping_add(1))
	}
}

impl Drop for Numbered {
	fn drop(&mut self) {
		event(b'x', self.0);
	}
}

/// A value that logs its clone and its drop.
struct Noisy(u8);

impl Clone for Noisy {
	fn clone(&self) -> Self {
		event(b'C', self.0);
		Noisy(self.0)
	}
}

impl Drop for Noisy {
	fn drop(&mut self) {
		event(b'D', self.0);
	}
}

/// A key that is a JS object.
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Key(u16, String);

/// Hash maps and sets with a `BuildHasher` of the crate's own, keyed by JS objects and by
/// numbers, which find their keys whatever it hashes them to. Each holds its own, which
/// `hasher` lends, made by `Default` where std's makes one, cloned before what the map
/// holds, and dropped before it, and at once by a consuming iterator. Only maps of one
/// entry say what they drop or clone, and only sorted what they hold, as std's order is
/// its own.
pub fn maps(data: &[u8], out: &mut Vec<u8>) {
	EVENTS.with(|events| events.borrow_mut().clear());
	let first = data.first().copied().unwrap_or(7);
	let mut keyed: HashMap<Key, u8, Numbered> = HashMap::with_capacity_and_hasher(data.len(), Numbered(first));
	let mut seen: HashSet<u8, Numbered> = HashSet::with_hasher(Numbered(1));
	for (i, &b) in data.iter().enumerate() {
		*keyed.entry(Key(u16::from(b % 8), text(&data[i..=i]))).or_default() += 1;
		out.push(seen.insert(b % 16) as u8);
		out.push(keyed.contains_key(&Key(u16::from(b % 8 ^ 1), text(&[b ^ 1]))) as u8);
	}
	let mut held: Vec<(&Key, &u8)> = keyed.iter().collect();
	held.sort();
	for (Key(n, text), count) in held {
		out.push(*n as u8);
		put(out, text);
		out.push(*count);
	}
	// What `hasher` lends is the map's own.
	let probe = Key(u16::from(first), text(&[first]));
	out.push(keyed.hasher().0);
	out.extend_from_slice(&keyed.hasher().hash_one(&probe).to_le_bytes());
	out.extend_from_slice(&seen.hasher().hash_one(first).to_le_bytes());
	let mut noisy: HashMap<u8, Noisy, Numbered> = HashMap::with_hasher(Numbered(5));
	noisy.insert(first, Noisy(first));
	let copy = noisy.clone();
	out.push(copy.hasher().0);
	drop(noisy);
	let mut entries = copy.into_iter();
	event(b'i', 0);
	drop(entries.next());
	drop(entries);
	let mut made: HashMap<u8, Noisy, Numbered> = HashMap::default();
	made.insert(2, Noisy(2));
	let taken = std::mem::take(&mut made);
	out.push(taken.len() as u8);
	out.push(made.len() as u8);
	let small: HashSet<u8, Numbered> = data.iter().map(|b| b % 4).collect();
	let union = &small | &seen;
	out.push(union.len() as u8);
	out.push(small.is_subset(&seen) as u8);
	seen.extend(small);
	out.push(seen.len() as u8);
	let mut keys: Vec<Key> = keyed.into_keys().collect();
	keys.sort();
	out.push(keys.len() as u8);
	for value in taken.into_values() {
		event(b'v', value.0);
	}
	drop(union);
	out.extend_from_slice(&EVENTS.with(|events| events.take()));
}

thread_local! {
	/// Each `Hash` and `eq` of a [`Sided`], in order.
	static CALLS: RefCell<Vec<u32>> = const { RefCell::new(Vec::new()) };
}

/// Log a call of `what` on the value of `side` holding `n`, given one of `other`.
fn call(what: u8, side: u8, other: u8, n: u8) {
	CALLS.with(|calls| calls.borrow_mut().push(u32::from_be_bytes([what, side, other, n])));
}

/// A key or value of a map's side that logs each `Hash` and `eq` of it, so the log says
/// whose key std hashes, and whose `eq` it calls with the other's as its argument.
#[derive(Clone)]
struct Sided {
	n: u8,
	side: u8,
}

impl Hash for Sided {
	fn hash<H: Hasher>(&self, state: &mut H) {
		call(b'h', self.side, 0, self.n);
		self.n.hash(state);
	}
}

impl PartialEq for Sided {
	fn eq(&self, other: &Self) -> bool {
		call(b'e', self.side, other.side, self.n);
		self.n == other.n
	}
}

impl Eq for Sided {}

/// A `Hasher` of the crate's own whose hash puts the low 7 bits of the last byte written
/// where hashbrown reads each slot's tag, on 32- and 64-bit targets, so a lookup among keys
/// that differ there calls `eq` only on the one it finds, as the runtime's does. The seed
/// moves the rest, and so where each key goes.
struct Spreader {
	seed: u64,
	last: u64,
}

impl Hasher for Spreader {
	fn finish(&self) -> u64 {
		let tag = self.last & 0x7f;
		(tag << 57) | (tag << 25) | ((self.last ^ self.seed).wrapping_mul(0x9e37_79b9) & 0x1ff_ffff)
	}

	fn write(&mut self, bytes: &[u8]) {
		if let Some(&b) = bytes.last() {
			self.last = u64::from(b);
		}
	}
}

/// What builds a [`Spreader`] of its seed.
#[derive(Clone, Default)]
struct Spread(u64);

impl BuildHasher for Spread {
	type Hasher = Spreader;

	fn build_hasher(&self) -> Spreader {
		Spreader { seed: self.0, last: 0 }
	}
}

/// A type whose derived `PartialEq` compares the maps it holds.
#[derive(PartialEq)]
struct Index {
	names: HashMap<u8, String>,
	seen: HashSet<(u8, bool)>,
}

/// Whether `a == b`, and what it logged: in the order it called each if it did so for one
/// entry at most, else sorted if it visited every entry, else nothing, as which entries it
/// visits before one differs follows std's order.
fn compared<T: PartialEq>(a: &T, b: &T, len: usize, out: &mut Vec<u8>) {
	CALLS.with(|calls| calls.borrow_mut().clear());
	let equal = a == b;
	out.push(equal as u8);
	out.push((a != b) as u8);
	let mut calls = CALLS.with(|calls| calls.take());
	if len > 1 && !equal {
		calls.clear();
	}
	// `!=` repeats `==`'s calls.
	calls.truncate(calls.len() / 2);
	if len > 1 {
		calls.sort();
	}
	out.push(calls.len() as u8);
	for c in calls {
		out.extend_from_slice(&c.to_be_bytes());
	}
}

/// `==` and `!=` of hash maps and sets, as std's: equal lengths, then each entry of the
/// left looked up in the right, which hashes the left's key by its own `BuildHasher` and
/// calls the left key's `eq` on the right's, then the left value's `eq` on the right's.
/// Each side holds its own `BuildHasher`, seeded apart, and is built of the same bytes in
/// another order, so a key given twice keeps the value it was given last; or of all but
/// the last, or of one byte flipped. Then maps of a `RandomState`, which compare their keys
/// as JS does, and maps in a `Vec` and in a type that derives `PartialEq`.
pub fn equal(data: &[u8], out: &mut Vec<u8>) {
	let first = data.first().copied().unwrap_or(0);
	let map = |side: u8, bytes: &[u8]| {
		let mut m = HashMap::with_hasher(Spread(u64::from(side) * 0x1_0001 + u64::from(first)));
		for &b in bytes {
			m.insert(
				Sided { n: b & 0x7f, side },
				Sided {
					n: b >> 6,
					side: side + 2,
				},
			);
		}
		m
	};
	let set = |side: u8, bytes: &[u8]| {
		let mut s = HashSet::with_hasher(Spread(u64::from(side)));
		for &b in bytes {
			s.insert(Sided { n: b & 0x7f, side });
		}
		s
	};
	let reversed: Vec<u8> = data.iter().rev().copied().collect();
	let flipped: Vec<u8> = data
		.iter()
		.enumerate()
		.map(|(i, &b)| if i == data.len() / 2 { b ^ 0x41 } else { b })
		.collect();
	let shorter = &data[..data.len().saturating_sub(1)];
	let left = map(1, data);
	for right in [map(2, &reversed), map(2, data), map(2, shorter), map(2, &flipped)] {
		compared(&left, &right, left.len(), out);
		compared(&right, &left, left.len(), out);
	}
	let left = set(1, data);
	for right in [set(2, &reversed), set(2, shorter), set(2, &flipped)] {
		compared(&left, &right, left.len(), out);
		compared(&right, &left, left.len(), out);
	}
	let empty: HashMap<Sided, Sided, Spread> = HashMap::default();
	compared(&empty, &HashMap::default(), 0, out);
	let counts = |bytes: &[u8]| {
		let mut m: HashMap<u8, u16> = HashMap::new();
		for &b in bytes {
			*m.entry(b % 8).or_default() += u16::from(b);
		}
		m
	};
	out.push((counts(data) == counts(&reversed)) as u8);
	out.push((counts(data) != counts(&flipped)) as u8);
	let strings = |bytes: &[u8]| bytes.iter().map(|&b| text(&[b % 4])).collect::<HashSet<String>>();
	out.push((strings(data) == strings(&reversed)) as u8);
	out.push((strings(data) == strings(shorter)) as u8);
	out.push((vec![strings(data), strings(&flipped)] == vec![strings(&reversed), strings(&flipped)]) as u8);
	let index = |bytes: &[u8]| Index {
		names: bytes.iter().map(|&b| (b % 8, text(&[b % 3]))).collect(),
		seen: bytes.iter().map(|&b| (b % 5, b % 2 == 0)).collect(),
	};
	out.push((index(data) == index(&reversed)) as u8);
	out.push((index(data) != index(&flipped)) as u8);
}

/// A `Hasher` of the crate's own that `BuildHasherDefault` builds, logging each one
/// `Default` makes, each byte written, each finish, and each drop.
struct Tally(u64);

impl Default for Tally {
	fn default() -> Self {
		event(b'n', 0);
		Tally(0)
	}
}

impl Hasher for Tally {
	fn finish(&self) -> u64 {
		event(b'f', self.0 as u8);
		self.0
	}

	fn write(&mut self, bytes: &[u8]) {
		for &b in bytes {
			event(b'w', b);
			self.0 = self.0.wrapping_mul(31).wrapping_add(u64::from(b));
		}
	}
}

impl Drop for Tally {
	fn drop(&mut self) {
		event(b'd', self.0 as u8);
	}
}

/// `BuildHasherDefault` held in a field, its impls derived.
#[derive(Default, Clone, Debug, PartialEq)]
struct Tallied {
	tag: u8,
	state: BuildHasherDefault<Tally>,
}

/// One made in a `const`.
const MADE: BuildHasherDefault<Tally> = BuildHasherDefault::new();

/// `BuildHasherDefault`, made by `new`, `Default`, and `Clone`, which all compare equal and
/// print alike whatever their `Hasher`, and each builds its hasher by the hasher's own
/// `Default`, once per hash. Then hash maps and sets that hold one, which find their keys
/// whatever it hashes them to, and say only sorted what they hold, as std's order is its own.
pub fn defaults(data: &[u8], out: &mut Vec<u8>) {
	EVENTS.with(|events| events.borrow_mut().clear());
	let first = data.first().copied().unwrap_or(7);
	let made = BuildHasherDefault::<Tally>::new();
	let copy = made.clone();
	let held = Tallied {
		tag: first,
		..Tallied::default()
	};
	let cloned = held.clone();
	out.push((made == copy) as u8);
	out.push((MADE != BuildHasherDefault::default()) as u8);
	out.push((held == cloned) as u8);
	out.push((held == Tallied::default()) as u8);
	put(out, &format!("{made:?} {:?} {cloned:?} {held:#?}", Some(&MADE)));
	let word = std::str::from_utf8(data).unwrap_or("✓");
	let n = wide(data, 0, 2) as u16;
	let mut built = made.build_hasher();
	n.hash(&mut built);
	let hashes = [
		built.finish(),
		made.hash_one(n),
		copy.hash_one(word),
		MADE.hash_one((n as u8, word)),
		cloned.state.hash_one(&Key(n, word.to_string())),
	];
	drop(built);
	for h in hashes {
		out.extend_from_slice(&h.to_le_bytes());
	}
	let mut keyed: HashMap<Key, u8, BuildHasherDefault<Fnv>> = HashMap::default();
	let mut seen: HashSet<u8, BuildHasherDefault<Fnv>> =
		HashSet::with_capacity_and_hasher(data.len(), BuildHasherDefault::new());
	for (i, &b) in data.iter().enumerate() {
		*keyed.entry(Key(u16::from(b % 8), text(&data[i..=i]))).or_default() += 1;
		out.push(seen.insert(b % 16) as u8);
		out.push(keyed.contains_key(&Key(u16::from(b % 8 ^ 1), text(&[b ^ 1]))) as u8);
	}
	let mut entries: Vec<(&Key, &u8)> = keyed.iter().collect();
	entries.sort();
	for (Key(n, text), count) in entries {
		out.push(*n as u8);
		put(out, text);
		out.push(*count);
	}
	let probe = Key(u16::from(first), text(&[first]));
	out.extend_from_slice(&keyed.hasher().hash_one(&probe).to_le_bytes());
	out.extend_from_slice(&seen.clone().hasher().hash_one(first).to_le_bytes());
	out.push((*keyed.hasher() == BuildHasherDefault::default()) as u8);
	out.push((keyed == keyed.clone()) as u8);
	put(out, &format!("{:?}", seen.hasher()));
	let small: HashSet<u8, BuildHasherDefault<Fnv>> = data.iter().map(|b| b % 4).collect();
	let union = &small | &seen;
	out.push(union.len() as u8);
	out.push(small.is_subset(&seen) as u8);
	out.extend_from_slice(&EVENTS.with(|events| events.take()));
}
