//! Keys a map compares, orders, and hashes through std's impls: `Option`s, which order
//! `None` first, and `Arc`s, which compare as what they hold; lookups through `Borrow`;
//! the references to values a map holds that JS copies, changed in place; and the `!=`,
//! `<`, `<=`, `>`, and `>=` a type's `PartialEq` and `PartialOrd` inherit or override.
//! Then bytes of each form, equal to one another and found by one another.
//! Then arrays of each element type as keys, found by the slices they borrow as.

use std::borrow::Borrow;
use std::cell::RefCell;
use std::cmp::{Ordering, Reverse};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque, btree_map, hash_map};
use std::hash::{DefaultHasher, Hash, Hasher};
use std::ops::Bound;
use std::sync::Arc;

/// A 64-bit hop, past 2^53 for some, where it is no longer a JS number.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Hop(u64);

/// A key whose derived impls read an `Option`, as moq-net's `Horizon` does.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Horizon {
	exclude: Option<Hop>,
	local: bool,
}

/// A fieldless enum, which an `Option` of compares through its own `PartialEq`.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Side {
	Left,
	Right,
}

/// Text in an `Arc`, whose derived impls compare and hash it as the text.
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Name {
	text: Arc<str>,
	rank: Option<u8>,
}

/// An `Eq` that is never equal, which std's `Arc` sees past when both are one `Arc`.
struct Liar;

impl PartialEq for Liar {
	fn eq(&self, _: &Self) -> bool {
		false
	}
}

impl Eq for Liar {}

/// A `PartialEq` that is never equal, and no `Eq`, so an `Arc` of it always asks it.
struct Never;

impl PartialEq for Never {
	fn eq(&self, _: &Self) -> bool {
		false
	}
}

/// An `Ord` by value whose `PartialOrd` disagrees: reversed, and unordered between odd
/// values, so a `partial_cmp` that calls `cmp` shows.
#[derive(PartialEq, Eq)]
struct Skew(u8);

impl Ord for Skew {
	fn cmp(&self, other: &Self) -> Ordering {
		self.0.cmp(&other.0)
	}
}

impl PartialOrd for Skew {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		match self.0 % 2 == 1 && other.0 % 2 == 1 {
			true => None,
			false => Some(other.0.cmp(&self.0)),
		}
	}
}

/// An `Ord` by value whose own `<`, `<=`, `>`, and `>=` each disagree with it: `<` and `>`
/// reversed, `<=` by quarter and reversed, and `>=` by the value mod 4, so a test that std
/// calls and the translation computes from `partial_cmp` shows. `<` is still a total
/// order, which sorts need.
#[derive(Clone, Copy, PartialEq, Eq, Ord)]
struct Lopsided(u8);

impl PartialOrd for Lopsided {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}

	fn lt(&self, other: &Self) -> bool {
		self.0 > other.0
	}

	fn le(&self, other: &Self) -> bool {
		self.0 / 4 >= other.0 / 4
	}

	fn gt(&self, other: &Self) -> bool {
		self.0 < other.0
	}

	fn ge(&self, other: &Self) -> bool {
		self.0 % 4 >= other.0 % 4
	}
}

/// A `ne` of its own that disagrees with `eq`, which an `Option`'s `!=` never calls.
struct Picky(u8);

/// A fieldless enum whose own `Eq` and `Hash` find `Up` and `Top` equal, which their JS
/// tags are not, through an `Option` in a key.
#[derive(Clone, Copy)]
enum Way {
	Up,
	Top,
	Down,
}

impl PartialEq for Way {
	fn eq(&self, other: &Self) -> bool {
		matches!(self, Way::Down) == matches!(other, Way::Down)
	}
}

impl Eq for Way {}

impl Hash for Way {
	fn hash<H: Hasher>(&self, state: &mut H) {
		matches!(self, Way::Down).hash(state);
	}
}

fn way(b: u8) -> Option<Way> {
	match b % 4 {
		0 => None,
		1 => Some(Way::Up),
		2 => Some(Way::Top),
		_ => Some(Way::Down),
	}
}

/// A key whose `Eq` and `Ord` panic, which a lookup that compares nothing never calls.
struct Touchy(u8);

impl Hash for Touchy {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.0.hash(state);
	}
}

impl PartialEq for Touchy {
	fn eq(&self, _: &Self) -> bool {
		panic!("compared {}", self.0)
	}
}

impl Eq for Touchy {}

impl PartialOrd for Touchy {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for Touchy {
	fn cmp(&self, _: &Self) -> Ordering {
		panic!("ordered {}", self.0)
	}
}

impl PartialEq for Picky {
	fn eq(&self, other: &Self) -> bool {
		self.0 == other.0
	}

	#[allow(clippy::partialeq_ne_impl)]
	fn ne(&self, other: &Self) -> bool {
		self.0 > other.0
	}
}

/// Borrows, whose derived impls compare, order, and hash what they point at through
/// std's impls for references.
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Seen<'a> {
	Hop(&'a Option<Hop>),
	Twice(&'a &'a Horizon),
	Text(&'a str),
	Name(&'a Name),
	Mut(&'a mut Hop),
}

/// A borrow of a `Skew`, whose derived `partial_cmp` reaches its own, not its `cmp`: a
/// derived `Ord` beside it would be called instead.
#[derive(PartialEq, PartialOrd)]
struct Skewed<'a>(&'a Skew);

/// An `eq` by value and a `partial_cmp` by quarter, unordered past 200: `!=` and `<=`
/// and `>=` all hold between some, and none of `<`, `<=`, `>`, and `>=` past 200.
#[derive(Clone, Copy)]
struct Rough(u8);

impl PartialEq for Rough {
	fn eq(&self, other: &Self) -> bool {
		self.0 == other.0
	}
}

impl PartialOrd for Rough {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		match self.0 > 200 || other.0 > 200 {
			true => None,
			false => (self.0 / 4).partial_cmp(&(other.0 / 4)),
		}
	}
}

/// Strings whose UTF-8 order differs from their UTF-16 order past U+FFFF.
const WORDS: [&str; 6] = ["", "a", "ab", "\u{e000}", "\u{ffff}", "\u{10000}"];

fn hop(b: u8) -> Option<Hop> {
	match b % 4 {
		0 => None,
		1 => Some(Hop(b as u64 >> 2)),
		2 => Some(Hop(u64::MAX - (b >> 2) as u64)),
		_ => Some(Hop((b as u64) << 50)),
	}
}

fn small(b: u8) -> Option<u8> {
	match b % 3 {
		0 => None,
		v => Some(v),
	}
}

fn side(v: u8) -> Side {
	match v {
		1 => Side::Left,
		_ => Side::Right,
	}
}

fn name(b: u8) -> Name {
	let word = WORDS[b as usize % WORDS.len()];
	let text: Arc<str> = match b % 2 {
		0 => Arc::from(word),
		_ => Arc::from(word.to_string()),
	};
	Name {
		text,
		rank: small(b / 64),
	}
}

fn hash_of<T: Hash>(value: &T) -> u64 {
	let mut h = DefaultHasher::new();
	value.hash(&mut h);
	h.finish()
}

fn order(o: Ordering) -> u8 {
	match o {
		Ordering::Less => 1,
		Ordering::Equal => 2,
		Ordering::Greater => 3,
	}
}

fn partial(o: Option<Ordering>) -> u8 {
	o.map_or(0, order)
}

/// The index of the first of `keys` that borrows as `q`.
fn find<K: Borrow<Q>, Q: PartialEq + ?Sized>(keys: &[K], q: &Q) -> u8 {
	for (i, k) in keys.iter().enumerate() {
		if k.borrow() == q {
			return i as u8;
		}
	}
	0xff
}

/// Run one case.
pub fn keys(kind: u8, data: &[u8], out: &mut Vec<u8>) {
	match kind % 5 {
		0 => options(data, out),
		1 => arcs(data, out),
		2 => borrows(data, out),
		3 => defaults(data, out),
		_ => places(data, out),
	}
}

/// `!=`, `<`, `<=`, `>`, and `>=` of each of `l` and `r` with the other, as bits.
fn compare<T: PartialOrd>(l: &T, r: &T) -> u8 {
	(l != r) as u8 | ((l < r) as u8) << 1 | ((l <= r) as u8) << 2 | ((l > r) as u8) << 3 | ((l >= r) as u8) << 4
}

/// The comparisons a type's `PartialEq` and `PartialOrd` inherit: `!=` as `!eq`, and
/// the orderings from `partial_cmp`, false when it is `None`, whatever `eq` says.
fn defaults(data: &[u8], out: &mut Vec<u8>) {
	for i in 1..data.len() {
		let (c, d) = (data[i - 1], data[i]);
		let (a, b) = (Rough(c), Rough(d));
		out.push(compare(&a, &b) | ((a != b) as u8) << 5 | ((a < b) as u8) << 6 | ((a >= b) as u8) << 7);
		out.push(compare(&Some(a), &Some(b)) | ((Some(a) <= None) as u8) << 5 | ((None < Some(b)) as u8) << 6);
		let (s, t) = (Skew(c % 4), Skew(d % 4));
		out.push(compare(&s, &t) | ((s <= t) as u8) << 5 | ((s > t) as u8) << 6);
		out.push(compare(&Skewed(&s), &Skewed(&t)));
		let (g, h) = (hop(c), hop(d));
		out.push(compare(&g, &h) | ((g.map(|h| h.0) > h.map(|h| h.0)) as u8) << 5);
		let (x, y) = (
			Horizon {
				exclude: g,
				local: c % 2 == 0,
			},
			Horizon {
				exclude: h,
				local: d % 2 == 0,
			},
		);
		out.push(compare(&x, &y) | ((&x < &y) as u8) << 5 | ((&&x >= &&y) as u8) << 6);
		let (m, n) = (name(c), name(d));
		out.push(compare(&m, &n) | ((m.text != n.text) as u8) << 5);
		// `Side`'s derived `eq` and `Way`'s own, which finds `Up` and `Top` equal.
		out.push(
			(side(c % 2) != side(d % 2)) as u8
				| ((way(c) != way(d)) as u8) << 1
				| ((way(c).map(|w| w != Way::Top) == Some(true)) as u8) << 2,
		);
		// Byte slices, through references and not.
		let (l, r) = (&data[..i], &data[i..]);
		let (p, q) = (&data[i - 1..i], &data[c as usize % i..c as usize % i + 1]);
		out.push(
			(l == r) as u8
				| ((l != r) as u8) << 1
				| ((p == q) as u8) << 2
				| ((*p != *q) as u8) << 3
				| ((&l[..0] == &r[..0]) as u8) << 4
				| ((data[..i] == data[data.len() - i..]) as u8) << 5,
		);
	}
}

/// The comparisons std makes through a type's own `<`, `<=`, `>`, and `>=`: directly, as
/// the last field of a tuple, in an `Arc`, and in a `Reverse`, and in `max`, `min`,
/// `clamp`, and sorts. A field before a tuple's last, and an `Option`, compare by
/// `partial_cmp`. The first three bytes clamp last, which panics unless `min <= max`.
pub fn overrides(_: u8, data: &[u8], out: &mut Vec<u8>) {
	for i in 1..data.len() {
		let (c, d) = (data[i - 1], data[i]);
		let (a, b) = (Lopsided(c), Lopsided(d));
		out.push(compare(&a, &b));
		out.push(compare(&(c % 2, a), &(d % 2, b)) | ((&(c % 2, &a) < &(d % 2, &b)) as u8) << 5);
		out.push(compare(&((c % 2, c % 3), a), &((d % 2, d % 3), b)));
		out.push(compare(&(a, c % 2), &(b, d % 2)));
		out.push(compare(&Arc::new(a), &Arc::new(b)));
		out.push(compare(&Reverse(a), &Reverse(b)));
		out.push(compare(&Some(a), &Some(b)));
		out.push(compare(&(c % 2, Reverse(Arc::new(a))), &(d % 2, Reverse(Arc::new(b)))));
		out.extend([a.max(b).0, a.min(b).0, (&a).max(&b).0]);
		let (m, n) = (
			(c % 2, Reverse(a)).max((d % 2, Reverse(b))),
			Arc::new(a).min(Arc::new(b)),
		);
		out.extend([m.0, m.1.0.0, n.0]);
		let x = Lopsided(c ^ d);
		if a <= b {
			out.push(x.clamp(a, b).0);
		}
	}
	let mut v: Vec<Lopsided> = data.iter().map(|&c| Lopsided(c)).collect();
	v.sort();
	out.extend(v.iter().map(|x| x.0));
	v.sort_unstable_by_key(|&x| (x.0 % 3, Reverse(x)));
	out.extend(v.iter().map(|x| x.0));
	let mut pairs: Vec<(u8, Lopsided)> = data.iter().map(|&c| (c % 4, Lopsided(c))).collect();
	pairs.sort_unstable();
	out.extend(pairs.iter().map(|p| p.1.0));
	if let [c, d, e, ..] = *data {
		out.push(Lopsided(c).clamp(Lopsided(d), Lopsided(e)).0);
	}
}

/// One of `Seen`'s variants, by `k`.
fn seen<'a>(k: u8, hop: &'a Option<Hop>, horizon: &'a &'a Horizon, name: &'a Name, mine: &'a mut Hop) -> Seen<'a> {
	match k % 5 {
		0 => Seen::Hop(hop),
		1 => Seen::Twice(horizon),
		2 => Seen::Text(WORDS[k as usize / 5 % WORDS.len()]),
		3 => Seen::Name(name),
		_ => Seen::Mut(mine),
	}
}

/// Borrows compared, ordered, and hashed as what they point at, through one reference
/// or two, shared or mutable.
fn borrows(data: &[u8], out: &mut Vec<u8>) {
	for i in 1..data.len() {
		let (c, d) = (data[i - 1], data[i]);
		let (a, b) = (hop(c), hop(d));
		let g = Horizon {
			exclude: a,
			local: c % 2 == 0,
		};
		let h = Horizon {
			exclude: b,
			local: d % 2 == 0,
		};
		let (gr, hr) = (&g, &h);
		let (m, n) = (name(c), name(d));
		let (mut x, mut y) = (Hop(c as u64 % 3), Hop(d as u64 % 3));
		let (l, r) = (seen(c, &a, &gr, &m, &mut x), seen(d, &b, &hr, &n, &mut y));
		out.push(
			(l == r) as u8
				| order(l.cmp(&r)) << 1
				| partial(l.partial_cmp(&r)) << 3
				| ((hash_of(&l) == hash_of(&r)) as u8) << 5,
		);
		let (s, t) = (Skew(c % 4), Skew(d % 4));
		let (s, t) = (Skewed(&s), Skewed(&t));
		out.push((s == t) as u8 | partial(s.partial_cmp(&t)) << 1);
		out.push(
			(&a == &b) as u8
				| ((&gr == &hr) as u8) << 1
				| ((&mut x == &mut y) as u8) << 2
				| ((&x == &mut y) as u8) << 3
				| ((&mut x == &y) as u8) << 4
				| ((hash_of(&&mut x) == hash_of(&y)) as u8) << 5,
		);
	}
}

/// `Option`s compared, ordered, and hashed, alone and in keys.
fn options(data: &[u8], out: &mut Vec<u8>) {
	let mut hashed: HashMap<(u8, Horizon), u8> = HashMap::new();
	let mut sorted: BTreeMap<(u8, Horizon), u8> = BTreeMap::new();
	let mut ways: HashMap<(Option<Way>, bool), u8> = HashMap::new();
	// Keyed by the `Option` itself, and through a `Reverse`, whose `Some` is the JS string
	// `Way`'s own `Eq` reads.
	let mut bare: HashMap<Option<Way>, u8> = HashMap::new();
	let mut deep: HashSet<Option<Reverse<Option<Way>>>> = HashSet::new();
	for i in 1..data.len() {
		let (c, d) = (data[i - 1], data[i]);
		let (a, b) = (hop(c), hop(d));
		out.push((a == b) as u8 | ((a != b) as u8) << 1 | order(a.cmp(&b)) << 2);
		out.push((a.max(b) == b) as u8 | ((a.min(b) == a) as u8) << 1);
		out.push((hash_of(&a) == hash_of(&b)) as u8);
		// `Option<u8>` compares as JS's `===` does, and `Option<Side>` through `Side`'s.
		let (x, y) = (small(c), small(d));
		let (s, t) = (x.map(side), y.map(side));
		out.push((x == y) as u8 | ((s == t) as u8) << 1 | ((s.as_ref() != t.as_ref()) as u8) << 2);
		let pairs = (a.map(|h| (h.0, d as u64)), b.map(|h| (h.0, c as u64)));
		out.push((pairs.0.max(pairs.1) == pairs.1) as u8);
		// `partial_cmp` through an `Option`, a tuple, and an `Arc` is the held type's.
		let (p, q) = (Some(Skew(c % 4)), Some(Skew(d % 4)));
		let (u, v) = ((Skew(c % 4), c), (Skew(d % 4), d));
		let (m, n) = (Arc::new(Skew(c % 4)), Arc::new(Skew(d % 4)));
		out.push(partial(p.partial_cmp(&q)) | partial(u.partial_cmp(&v)) << 2 | partial(m.partial_cmp(&n)) << 4);
		out.push(order(p.cmp(&q)) | order(u.cmp(&v)) << 2 | order(m.cmp(&n)) << 4);
		// An `Option`'s `!=` is `!eq`, under a tuple and an `Arc` too.
		let (g, h) = (Some(Picky(c % 3)), Some(Picky(d % 3)));
		let (j, k) = ((Some(Picky(c % 3)), c % 2), (Some(Picky(d % 3)), d % 2));
		let (r, s) = (Arc::new(Some(Picky(c % 3))), Arc::new(Some(Picky(d % 3))));
		out.push((g != h) as u8 | ((j != k) as u8) << 1 | ((r != s) as u8) << 2);
		let key = (
			c % 3,
			Horizon {
				exclude: a,
				local: d % 2 == 0,
			},
		);
		out.push(hashed.insert(key, i as u8).unwrap_or(0xcc));
		out.push(sorted.insert(key, i as u8).unwrap_or(0xcc));
		out.push(ways.insert((way(c), c < 128), i as u8).unwrap_or(0xcc));
		out.push(ways.get(&(way(d), d < 128)).copied().unwrap_or(0xdd));
		out.push(ways.len() as u8);
		out.push(bare.insert(way(c), i as u8).unwrap_or(0xcc));
		out.push(bare.get(&way(d)).copied().unwrap_or(0xdd));
		let nested = |b: u8| (b % 5 != 0).then(|| Reverse(way(b)));
		out.push(deep.insert(nested(c)) as u8 | (deep.contains(&nested(d)) as u8) << 1);
		out.extend([bare.len() as u8, deep.len() as u8]);
		out.push(hashed.contains_key(&(
			d % 3,
			Horizon {
				exclude: b,
				local: false,
			},
		)) as u8);
	}
	// `None` first, then each hop in order.
	for ((n, h), v) in &sorted {
		out.push(*n);
		out.extend_from_slice(&h.exclude.map_or(0, |h| h.0).to_be_bytes());
		out.push(h.exclude.is_some() as u8 | (h.local as u8) << 1);
		out.push(*v);
		out.push(hashed[&(*n, *h)]);
	}
}

/// `Arc`s compared, ordered, and hashed as what they hold, and borrowed as it.
fn arcs(data: &[u8], out: &mut Vec<u8>) {
	for i in 1..data.len() {
		let (a, b) = (name(data[i - 1]), name(data[i]));
		out.push(order(a.text.cmp(&b.text)) | order(a.cmp(&b)) << 2);
		// Of equal operands, `max` returns the second and `min` the first.
		let (max, min) = ((&a.text).max(&b.text), (&a.text).min(&b.text));
		out.push(Arc::ptr_eq(max, &b.text) as u8 | (Arc::ptr_eq(min, &a.text) as u8) << 1);
		out.push((a.text == b.text) as u8 | (a.eq(&b) as u8) << 1 | ((*max == b.text) as u8) << 2);
		out.push((hash_of(&a.text) == hash_of(&b.text)) as u8 | ((hash_of(&a) == hash_of(&b)) as u8) << 1);
		let text: &str = a.text.borrow();
		out.push((text == b.text.as_ref()) as u8);
		let word = WORDS[data[i] as usize % WORDS.len()];
		out.push(find(&[text.to_string(), word.to_string()], word));
		out.push(find(&[a.text, b.text], word));
	}
	if let Some(&b) = data.first() {
		out.push(find(data, &b));
	}
	// A `T: Eq` is equal to itself, so std's `Arc` takes one `Arc` as equal without asking it.
	let liar = Arc::new(Liar);
	let never = Arc::new(Never);
	out.push(
		(liar == liar.clone()) as u8 | ((never == never.clone()) as u8) << 1 | ((liar == Arc::new(Liar)) as u8) << 2,
	);
}

/// Changes a map's values JS copies through the references `get_mut` and the entry API
/// give: numbers, a wide number, and a tuple, by keys JS compares and by keys found
/// through their hash.
fn places(data: &[u8], out: &mut Vec<u8>) {
	let mut counts: BTreeMap<u8, usize> = BTreeMap::new();
	let mut wide: HashMap<u8, u64> = HashMap::new();
	let mut hops: HashMap<Hop, u16> = HashMap::new();
	let mut pairs: BTreeMap<Hop, (u8, u8)> = BTreeMap::new();
	for (i, &b) in data.iter().enumerate() {
		let k = b % 97;
		match b % 6 {
			0 => *counts.entry(k).or_default() += 1,
			1 => {
				if let Some(c) = counts.get_mut(&k) {
					*c -= 1;
					if *c == 0 {
						counts.remove(&k);
					}
				}
			}
			2 => match counts.entry(k) {
				btree_map::Entry::Occupied(mut e) if *e.get() > 1 => *e.get_mut() *= 3,
				btree_map::Entry::Occupied(e) => *e.into_mut() += 7,
				btree_map::Entry::Vacant(e) => *e.insert(i) += 2,
			},
			3 => {
				let c = counts.entry(k).or_insert_with(|| i * 2);
				*c ^= 5;
				out.push(*c as u8);
			}
			4 if counts.contains_key(&k) => *counts.get_mut(&k).expect("a counted key") += 4,
			4 => *counts.entry(k).or_insert(9) <<= 1,
			// A missing key panics.
			_ => *counts.get_mut(&k).unwrap() += 1,
		}
		let w = wide.entry(b % 7).or_insert(1 << 52);
		*w = w.wrapping_add(*w >> 1).wrapping_add(b as u64);
		let h = hops.entry(Hop(b as u64 % 6)).or_default();
		*h = h.wrapping_add(b as u16);
		if let Some(h) = hops.get_mut(&Hop(b as u64 % 4)) {
			*h ^= 0x5a5a;
		}
		match hops.entry(Hop(b as u64 % 5)) {
			hash_map::Entry::Occupied(mut e) if *e.get() > 0x1000 => *e.get_mut() -= 0x1000,
			hash_map::Entry::Occupied(e) => *e.into_mut() |= 1,
			hash_map::Entry::Vacant(e) => *e.insert(b as u16) <<= 4,
		}
		let p = pairs.entry(Hop(b as u64 % 3)).or_insert((0, 0));
		p.0 = p.0.wrapping_add(b);
		p.1 ^= p.0;
	}
	// More keys than a B-tree leaf holds, so an insert splits the leaf its place is in.
	let mut tree: BTreeMap<u16, u16> = BTreeMap::new();
	for j in 0..data.len() * 4 {
		let b = data[j % data.len()];
		let v = tree.entry((j as u16 * 37) ^ b as u16).or_insert(j as u16);
		*v = v.wrapping_add(b as u16);
	}
	for (k, v) in &tree {
		out.extend_from_slice(&k.to_be_bytes());
		out.extend_from_slice(&v.to_be_bytes());
	}
	// An entry of an empty map compares no keys, so its place is had without one.
	let mut touchy: BTreeMap<Touchy, u8> = BTreeMap::new();
	*touchy.entry(Touchy(data.len() as u8)).or_insert_with(|| 7) += 1;
	let mut hashy: HashMap<Touchy, u8> = HashMap::new();
	*hashy.entry(Touchy(data.len() as u8)).or_default() += 2;
	out.extend(touchy.values().chain(hashy.values()));
	// A reference into a map, written by a closure, swapped whole, and moved.
	let c = counts.entry(0).or_default();
	let mut bump = || *c += 1;
	bump();
	let mut spare = [data.len(), 8];
	let c = counts.entry(1).or_insert(3);
	std::mem::swap(c, &mut spare[0]);
	let d = counts.entry(2).or_default();
	let e = d;
	*e += spare[0];
	for (k, c) in &counts {
		out.push(*k);
		out.extend_from_slice(&(*c as u32).to_be_bytes());
	}
	for k in 0..7 {
		out.extend_from_slice(&wide.get(&k).copied().unwrap_or(0).to_be_bytes());
	}
	for k in 0..6 {
		out.extend_from_slice(&hops.get(&Hop(k)).copied().unwrap_or(0).to_be_bytes());
	}
	for (k, (a, b)) in &pairs {
		out.push(k.0 as u8);
		out.push(*a);
		out.push(*b);
	}
}

/// Bytes compared and found as keys: a `[u8; N]`, a `&[u8]`, or a `Vec<u8>`, equal to
/// another form holding the same bytes, alone or in a tuple, an `Option`, or an `Arc`,
/// and a key found by a `&[u8]` it borrows as, in hash and B-tree collections alike.
pub fn bytes(kind: u8, data: &[u8], out: &mut Vec<u8>) {
	let cut = data.len().min(6);
	for w in data.windows(2) {
		let (a, b) = ([w[0] % 3, w[1] % 3], [w[1] % 3, w[0] % 3]);
		out.extend([
			(a == b) as u8,
			(a != b) as u8,
			(&a == &b) as u8,
			(Some(a) == Some(b)) as u8,
		]);
		out.extend([((a, w[0]) == (b, w[1])) as u8, (a.to_vec() == b.to_vec()) as u8]);
		out.extend([
			(a[..] == b[..]) as u8,
			(a.to_vec() == a) as u8,
			(&a[..] == b.to_vec()) as u8,
		]);
		out.extend([
			(a.to_vec() == &b[..]) as u8,
			(a == b[..]) as u8,
			(Arc::new(a) == Arc::new(b)) as u8,
		]);
	}
	let mut arrays: HashMap<[u8; 2], usize> = HashMap::new();
	let mut vecs: HashMap<Vec<u8>, usize> = HashMap::new();
	let mut slices: HashMap<&[u8], usize> = HashMap::new();
	let mut nested: HashMap<(Option<[u8; 2]>, Vec<u8>), usize> = HashMap::new();
	let mut shared: HashMap<Arc<Vec<u8>>, usize> = HashMap::new();
	let mut arcs: HashMap<Arc<[u8; 2]>, usize> = HashMap::new();
	let mut boxed: HashMap<Box<[u8; 2]>, usize> = HashMap::new();
	let mut tree: BTreeMap<[u8; 2], usize> = BTreeMap::new();
	let mut ordered: BTreeSet<Vec<u8>> = BTreeSet::new();
	let pairs: Vec<[u8; 2]> = data.windows(2).map(|w| [w[0] % 4, w[1] % 4]).collect();
	let borrowed: HashSet<&[u8; 2]> = pairs.iter().collect();
	for (i, w) in data.windows(2).enumerate() {
		let key = [w[0] % 4, w[1] % 4];
		let s = &data[i..(i + kind as usize % 3).min(data.len())];
		out.push(arrays.insert(key, i).map_or(0xcc, |old| old as u8));
		out.push(vecs.insert(s.to_vec(), i).map_or(0xcc, |old| old as u8));
		out.push(slices.insert(s, i).map_or(0xcc, |old| old as u8));
		let opt = (w[0] % 3 != 0).then_some(key);
		out.push(nested.insert((opt, s.to_vec()), i).map_or(0xcc, |old| old as u8));
		out.push(shared.insert(Arc::new(s.to_vec()), i).map_or(0xcc, |old| old as u8));
		out.push(arcs.insert(Arc::new(key), i).map_or(0xcc, |old| old as u8));
		out.push(boxed.insert(Box::new(key), i).map_or(0xcc, |old| old as u8));
		out.push(tree.insert(key, i).map_or(0xcc, |old| old as u8));
		out.push(ordered.insert(s.to_vec()) as u8);
	}
	for i in 0..=cut {
		for j in i..=cut {
			let s = &data[i..j];
			out.push(vecs.get(s).map_or(0xdd, |v| *v as u8));
			out.push(slices.get(s).map_or(0xdd, |v| *v as u8));
			out.push(vecs.contains_key(&s.to_vec()) as u8);
			out.push(shared.get(&Arc::new(s.to_vec())).map_or(0xdd, |v| *v as u8));
			let key = [data.get(i).map_or(0, |b| b % 4), data.get(j).map_or(0, |b| b % 4)];
			out.push(arrays.get(&key).map_or(0xdd, |v| *v as u8));
			out.push(arcs.get(&key).map_or(0xdd, |v| *v as u8));
			out.push(boxed.get(&key).map_or(0xdd, |v| *v as u8));
			out.push(boxed.get(&Box::new(key)).map_or(0xdd, |v| *v as u8));
			out.push(tree.get(&key).map_or(0xdd, |v| *v as u8));
			out.push(borrowed.contains(&&key) as u8);
			out.push(ordered.contains(s) as u8);
			if s.len() >= 2 {
				let two = &s[..2];
				out.push(arrays.get(two).map_or(0xdd, |v| *v as u8));
				out.push(tree.get(two).map_or(0xdd, |v| *v as u8));
			}
			out.push(nested.get(&(Some(key), s.to_vec())).map_or(0xdd, |v| *v as u8));
			out.push(nested.get(&(None, s.to_vec())).map_or(0xdd, |v| *v as u8));
		}
	}
	out.push(vecs.remove(&[][..]).map_or(0xee, |v| v as u8));
	out.push(vecs.len() as u8);
	let set: HashSet<Vec<u8>> = data.chunks(2).map(<[u8]>::to_vec).collect();
	out.push(set.len() as u8);
	out.push(set.contains(&data[..data.len().min(2)]) as u8);
}

/// Keys that are `[u8; N]`, lent as std lends them by every map and set: read, compared
/// with one another and the keys they were made from, looked up by, and copied out and
/// written, which leaves the collection's own as they were. A hash collection's order is
/// the runtime's, so what it yields is sorted first.
pub fn lent_keys(kind: u8, data: &[u8], out: &mut Vec<u8>) {
	let pairs: Vec<[u8; 2]> = data.windows(2).map(|w| [w[0] % 4, w[1] % 4]).collect();
	let probe = [kind % 4, (kind / 4) % 4];
	let mut map: HashMap<[u8; 2], u8> = HashMap::new();
	let mut tree: BTreeMap<[u8; 2], u8> = BTreeMap::new();
	let mut set: HashSet<[u8; 2]> = HashSet::new();
	let mut ordered: BTreeSet<[u8; 2]> = BTreeSet::new();
	for (i, &pair) in pairs.iter().enumerate() {
		let mut key = pair;
		map.insert(key, i as u8);
		tree.insert(key, i as u8);
		set.insert(key);
		ordered.insert(key);
		// The collections hold copies, which a write to the key given them leaves alone.
		key[0] ^= 0x10;
		out.push(map.contains_key(&key) as u8);
	}
	// Each key lent, ordered among the others, compared, looked up by, and copied out and
	// written.
	let mut lent: Vec<&[u8; 2]> = map.keys().collect();
	lent.sort();
	for &k in &lent {
		out.extend([
			k[0],
			k[1],
			map[k],
			map.contains_key(k) as u8,
			(*k == probe) as u8,
			(k < &probe) as u8,
		]);
		out.push(tree.get(k).copied().unwrap_or(0xdd));
		out.push(set.get(k).map_or(0xdd, |s| (s == k) as u8));
		let mut copy = *k;
		copy[1] ^= 0x20;
		out.push(map.contains_key(&copy) as u8);
	}
	out.push(lent.iter().all(|k| map.contains_key(*k)) as u8);
	let mut entries: Vec<([u8; 2], u8)> = Vec::new();
	for (k, v) in map.iter() {
		entries.push((*k, *v));
	}
	entries.sort();
	for (k, v) in &entries {
		out.extend_from_slice(k);
		out.push(*v);
	}
	let mut sum = 0u8;
	for (k, v) in &mut map {
		*v = v.wrapping_add(k[0]);
		sum = sum.wrapping_add(k[1]).wrapping_add(*v);
	}
	for (k, v) in map.iter_mut() {
		*v ^= k[1];
	}
	for (k, v) in &map {
		sum = sum.wrapping_add(k[0] ^ *v);
	}
	out.push(sum);
	// A lookup lends the key the map holds, found by an equal one.
	out.push(map.get_key_value(&probe).map_or(0xdd, |(k, v)| k[0] ^ k[1] ^ *v));
	out.push(tree.get_key_value(&probe).map_or(0xdd, |(k, v)| k[0] ^ k[1] ^ *v));
	match map.entry(probe) {
		hash_map::Entry::Occupied(o) => out.extend([1, o.key()[0], o.key()[1], *o.get()]),
		hash_map::Entry::Vacant(v) => out.extend([0, v.key()[0], v.key()[1]]),
	}
	out.push(*map.entry(probe).or_insert_with_key(|k| k[0].wrapping_mul(3) ^ k[1]));
	out.push(map.entry([3, 3]).key()[1]);
	match tree.entry(probe) {
		btree_map::Entry::Occupied(o) => out.extend([1, o.key()[0], o.key()[1], *o.get()]),
		btree_map::Entry::Vacant(v) => out.extend([0, v.key()[0], v.key()[1]]),
	}
	out.push(*tree.entry(probe).or_insert_with_key(|k| k[1].wrapping_mul(5) ^ k[0]));
	if let Some(o) = tree.first_entry() {
		out.extend_from_slice(o.key());
	}
	if let Some(o) = tree.last_entry() {
		out.extend_from_slice(o.key());
	}
	// A set lends the key it holds, found by an equal array or a borrowed `&[u8]`.
	let mut members: Vec<[u8; 2]> = set.iter().copied().collect();
	members.sort();
	for m in &members {
		out.extend_from_slice(m);
	}
	for k in &set {
		out.push((ordered.get(k) == Some(k)) as u8);
	}
	let window = &data[..data.len().min(2)];
	out.push(set.get(window).map_or(0xdd, |k| k[0] ^ (k[1] << 2)));
	out.push(set.get(&probe[..]).map_or(0xdd, |k| k[0] ^ (k[1] << 2)));
	out.push(set.get(&probe).map_or(0xdd, |k| (k == &probe) as u8));
	out.push(ordered.get(&probe[..]).map_or(0xdd, |k| k[0] ^ (k[1] << 2)));
	// A B-tree lends its keys in order, from either end.
	for (k, v) in tree.iter().rev() {
		out.extend([k[0], k[1], *v]);
	}
	for k in tree.keys() {
		out.extend_from_slice(k);
	}
	for (k, v) in tree.range([1, 0]..[3, 0]) {
		out.extend([k[0], k[1], *v]);
	}
	for (k, v) in tree.range_mut(probe..) {
		*v = v.wrapping_add(k[0]);
	}
	for (k, v) in &tree {
		out.extend([k[1], *v]);
	}
	for (k, v) in &mut tree {
		*v ^= k[0];
	}
	out.push(tree.first_key_value().map_or(0xdd, |(k, v)| k[0] ^ *v));
	out.push(tree.last_key_value().map_or(0xdd, |(k, v)| k[1] ^ *v));
	for k in ordered.iter().rev() {
		out.extend_from_slice(k);
	}
	for k in ordered.range(..=probe) {
		out.extend_from_slice(k);
	}
	for k in &ordered {
		out.push(k[0] ^ k[1]);
	}
	out.extend(ordered.first().map_or([0xdd; 2], |k| *k));
	out.extend(ordered.last().map_or([0xdd; 2], |k| *k));
	// `retain` hands its closure each key.
	map.retain(|k, v| k[0] <= k[1] || *v % 2 == 0);
	tree.retain(|k, _| k[0] != k[1]);
	set.retain(|k| k[1] != probe[1]);
	ordered.retain(|k| k[0] < 2);
	out.extend([map.len() as u8, tree.len() as u8, set.len() as u8, ordered.len() as u8]);
	// What a copy of a lent key writes, the collection's own keys never see.
	if let Some(first) = ordered.first() {
		let mut copy = *first;
		copy[0] = 0xff;
		out.push(ordered.contains(&copy) as u8);
	}
	out.push(ordered.iter().all(|k| k[0] < 2) as u8);
	// Keys of bytes in other forms, lent as they are held.
	let vecs: HashMap<Vec<u8>, u8> = data.chunks(3).map(|c| (c.to_vec(), c.len() as u8)).collect();
	let mut lens: Vec<&Vec<u8>> = vecs.keys().collect();
	lens.sort();
	for k in lens {
		out.push(k.len() as u8);
		out.extend_from_slice(k);
	}
	let slices: HashSet<Vec<u8>> = data.chunks(2).map(<[u8]>::to_vec).collect();
	out.push(slices.get(window).map_or(0xdd, |k| k.len() as u8));
	let borrowed: BTreeSet<&[u8]> = data.chunks(2).collect();
	for k in &borrowed {
		out.extend_from_slice(k);
	}
	out.push(borrowed.get(window).map_or(0xdd, |k| k.len() as u8));
	// Empty collections lend nothing.
	let empty: BTreeMap<[u8; 2], u8> = BTreeMap::new();
	let none: HashSet<[u8; 2]> = HashSet::new();
	out.extend([
		empty.keys().count() as u8,
		empty.first_key_value().is_none() as u8,
		empty.range(probe..).count() as u8,
		none.iter().count() as u8,
		none.get(&probe).is_none() as u8,
	]);
}

/// Values that are `[u8; 2]`, and the items of a deque and a heap, lent as keys are: a
/// `&` reads them and a `&mut` writes where the collection holds them.
pub fn lent_values(kind: u8, data: &[u8], out: &mut Vec<u8>) {
	let mut map: HashMap<u8, [u8; 2]> = HashMap::new();
	let mut tree: BTreeMap<u8, [u8; 2]> = BTreeMap::new();
	let mut deque: VecDeque<[u8; 2]> = VecDeque::new();
	let mut heap: BinaryHeap<[u8; 2]> = BinaryHeap::new();
	for w in data.windows(2) {
		let value = [w[0], w[1] % 8];
		map.insert(w[0] % 8, value);
		tree.insert(w[1] % 8, value);
		deque.push_back(value);
		heap.push(value);
	}
	let at = kind % 8;
	out.extend(map.get(&at).map_or([0xdd; 2], |v| *v));
	out.extend(tree.get(&at).map_or([0xdd; 2], |v| *v));
	if let Some(v) = map.get_mut(&at) {
		v[0] ^= 0x40;
	}
	if let Some(v) = tree.get_mut(&at) {
		v[1] ^= 0x40;
	}
	if map.contains_key(&at) {
		out.extend(map[&at]);
	}
	if tree.contains_key(&at) {
		out.extend(tree[&at]);
	}
	map.entry(at).or_insert([at, at])[1] ^= 1;
	map.entry(at).and_modify(|v| v[0] = v[0].wrapping_add(1));
	tree.entry(at).or_insert_with(|| [at, 0])[0] ^= 2;
	for v in map.values_mut() {
		v[1] = v[1].wrapping_mul(3);
	}
	for (_, v) in tree.iter_mut() {
		v[0] = v[0].wrapping_add(v[1]);
	}
	let mut values: Vec<[u8; 2]> = map.values().copied().collect();
	values.sort();
	for v in &values {
		out.extend_from_slice(v);
	}
	for (k, v) in &tree {
		out.extend([*k, v[0], v[1]]);
	}
	// A deque's items, read and written from either end and by index.
	out.extend(deque.front().map_or([0xdd; 2], |v| *v));
	out.extend(deque.back().map_or([0xdd; 2], |v| *v));
	if let Some(v) = deque.front_mut() {
		v[0] ^= 0x80;
	}
	if let Some(v) = deque.get_mut(usize::from(kind) % 4) {
		v[1] ^= 0x80;
	}
	if !deque.is_empty() {
		let last = deque.len() - 1;
		deque[last][0] ^= 1;
		out.extend(deque[0]);
	}
	for v in deque.iter_mut() {
		v[1] = v[1].wrapping_add(1);
	}
	deque.retain(|v| v[0] % 3 != 0);
	for v in &deque {
		out.extend_from_slice(v);
	}
	out.push(deque.iter().filter(|v| v[1] > 4).count() as u8);
	// A heap's greatest item, and the rest in its own order.
	out.extend(heap.peek().map_or([0xdd; 2], |v| *v));
	heap.retain(|v| v[1] != kind % 8);
	for v in heap.iter() {
		out.extend_from_slice(v);
	}
	for v in &heap {
		out.push(v[0] ^ v[1]);
	}
	for v in heap.into_sorted_vec() {
		out.extend(v);
	}
}

/// Collections keyed by arrays of each element type, none at all, and many: found by an
/// equal array made apart and by the slice each borrows as, ordered as std orders arrays,
/// by their elements in turn, and ranged, walked, and edited through entries. A hash
/// collection's order is the runtime's, so what it yields is sorted first.
pub fn arrays(kind: u8, data: &[u8], out: &mut Vec<u8>) {
	match kind % 9 {
		0 => keyed(data, |b| [u16::from(b % 4) << 8, u16::from(b % 3)], out),
		1 => keyed(data, |b| [u64::MAX - u64::from(b % 4), 1 << (b % 3 * 30)], out),
		2 => keyed(data, |b| [i32::from(b as i8 % 3), -i32::from(b % 2)], out),
		3 => keyed(data, |b| ['\u{e000}', char::from(b % 4 + b'a')], out),
		4 => keyed(data, |b| [b % 2 == 0, b % 3 == 0], out),
		5 => keyed(data, |b| [small(b), small(b / 4)], out),
		6 => keyed(data, |b| [Hop(u64::from(b % 3) << 53), Hop(u64::from(b % 2))], out),
		7 => keyed(data, |_| [0u32; 0], out),
		_ => keyed(data, |b| [u32::from(b % 4); 275], out),
	}
}

/// `arrays` of keys `make` makes of each byte.
fn keyed<T: Ord + Hash + Clone, const N: usize>(data: &[u8], make: impl Fn(u8) -> [T; N], out: &mut Vec<u8>) {
	let mut map: HashMap<[T; N], u8> = HashMap::new();
	let mut tree: BTreeMap<[T; N], u8> = BTreeMap::new();
	let mut set: HashSet<[T; N]> = HashSet::new();
	let mut ordered: BTreeSet<[T; N]> = BTreeSet::new();
	let mut values: HashMap<u8, [T; N]> = HashMap::new();
	for (i, &b) in data.iter().enumerate() {
		let i = i as u8;
		out.push(map.insert(make(b), i).unwrap_or(0xcc));
		out.push(tree.insert(make(b), i).unwrap_or(0xcc));
		out.extend([set.insert(make(b)) as u8, ordered.insert(make(b)) as u8]);
		out.push(values.insert(b % 4, make(b)).map_or(0xcc, |old| (old == make(b)) as u8));
		*map.entry(make(b / 2)).or_insert(i) ^= 0x40;
		match tree.entry(make(b / 2)) {
			btree_map::Entry::Occupied(mut o) => *o.get_mut() ^= 0x40,
			btree_map::Entry::Vacant(v) => {
				out.push((*v.key() == make(b / 2)) as u8);
				v.insert(i);
			}
		}
	}
	// Each found by an equal array made apart, and by the slice it borrows as.
	for b in 0..8 {
		let key = make(b);
		out.extend([
			map.get(&key).copied().unwrap_or(0xdd),
			map.get(&key[..]).copied().unwrap_or(0xdd),
			tree.get(&key[..]).copied().unwrap_or(0xdd),
			set.contains(&key[..]) as u8,
			ordered.contains(&key[..]) as u8,
			map.get_key_value(&key[..]).map_or(0xdd, |(k, _)| (*k == key) as u8),
			tree.get_key_value(&key).map_or(0xdd, |(k, _)| (k[..] == key[..]) as u8),
			set.get(&key[..]).map_or(0xdd, |k| (k == &key) as u8),
			values.get(&b).map_or(0xdd, |v| (*v == key) as u8),
		]);
		// Ranges by an array and by a slice, which order by the elements in turn.
		out.push(tree.range(..key.clone()).count() as u8);
		out.push(
			tree.range::<[T], _>((Bound::Excluded(&key[..]), Bound::Unbounded))
				.count() as u8,
		);
		out.push(ordered.range::<[T; N], _>(&key..).map(|k| (k >= &key) as u8).sum());
		out.push(
			ordered
				.range::<[T], _>((Bound::Unbounded, Bound::Included(&key[..])))
				.count() as u8,
		);
	}
	// The order std gives arrays, read by the values each holds, and keys compared.
	out.extend(tree.values().copied());
	let keys: Vec<&[T; N]> = ordered.iter().collect();
	for w in keys.windows(2) {
		out.push((w[0] < w[1]) as u8 | ((w[0].cmp(w[1]) == w[0][..].cmp(&w[1][..])) as u8) << 1);
	}
	out.push(
		tree.first_key_value()
			.map_or(0xdd, |(k, v)| (Some(k) == keys.first().copied()) as u8 ^ v),
	);
	out.push(ordered.last().map_or(0xdd, |k| (Some(&k) == keys.last()) as u8));
	// Each removed by the slice it borrows as, then what is left.
	let probe = make(data.first().copied().unwrap_or(0));
	out.push(map.remove(&probe[..]).unwrap_or(0xee));
	out.push(map.remove_entry(&probe).map_or(0xee, |(k, v)| (k == probe) as u8 ^ v));
	out.push(tree.remove(&probe[..]).unwrap_or(0xee));
	out.extend([set.remove(&probe[..]) as u8, ordered.remove(&probe[..]) as u8]);
	out.push(ordered.pop_first().map_or(0xdd, |k| (k < probe) as u8));
	tree.retain(|k, v| k[..] >= probe[..] || *v % 2 == 0);
	out.extend(tree.values().copied());
	let mut held: Vec<u8> = map.values().copied().collect();
	held.sort();
	out.extend(held);
	out.extend([map.len(), tree.len(), set.len(), ordered.len()].map(|n| n as u8));
	// Collected, written through by a slice, split by one, and the keys lent sorted.
	let made: BTreeSet<[T; N]> = data.iter().map(|&b| make(b / 3)).collect();
	let mut counts: HashMap<[T; N], u8> = HashMap::from([(make(0), 1), (make(1), 2)]);
	for &b in data {
		out.push(counts.contains_key(&make(b)[..]) as u8);
		match counts.get_mut(&make(b)[..]) {
			Some(c) => *c = c.wrapping_mul(3),
			None => *counts.entry(make(b)).or_default() += 1,
		}
	}
	let mut lent: Vec<(&[T; N], &u8)> = counts.iter().collect();
	lent.sort();
	out.extend(lent.iter().map(|&(k, v)| (made.contains(k) as u8) << 7 | v));
	out.push(made.intersection(&ordered).count() as u8);
	out.push(made.is_subset(&ordered) as u8);
	let mut upper = made.clone();
	let lower = upper.split_off(&probe[..]);
	out.extend([upper.len() as u8, lower.len() as u8]);
	out.push(lower.iter().all(|k| k[..] >= probe[..]) as u8);
	// Keys that hold an array, and vectors found by the slice each borrows as.
	let mut pairs: BTreeMap<(u8, [T; N]), u8> = BTreeMap::new();
	let mut options: HashSet<Option<[T; N]>> = HashSet::new();
	let mut vecs: HashMap<Vec<T>, u8> = HashMap::new();
	let mut lists: BTreeSet<Vec<T>> = BTreeSet::new();
	for (i, &b) in data.iter().enumerate() {
		let pair = (b % 2, make(b));
		out.push(((&pair < &(1, make(2))) as u8) << 1 | (pair <= (0, make(3))) as u8);
		out.push(pairs.insert(pair, i as u8).unwrap_or(0xcc));
		out.push(options.insert((b % 3 != 0).then(|| make(b))) as u8);
		out.push(vecs.insert(make(b).to_vec(), i as u8).unwrap_or(0xcc));
		out.push(lists.insert(make(b / 2).to_vec()) as u8);
	}
	out.extend(pairs.values().copied());
	for b in 0..8 {
		let key = make(b);
		out.extend([
			options.contains(&Some(key.clone())) as u8,
			vecs.get(&key[..]).copied().unwrap_or(0xdd),
			lists.contains(&key[..]) as u8,
			lists
				.range::<[T], _>((Bound::Included(&key[..]), Bound::Unbounded))
				.count() as u8,
		]);
	}
	out.push(options.contains(&None) as u8);
	// Keys that are references to elements, found by what each points at.
	let firsts: Vec<T> = data.iter().filter_map(|&b| make(b).first().cloned()).collect();
	let refs: BTreeMap<&T, u8> = firsts.iter().zip(0..).collect();
	let hashed: HashSet<&T> = firsts.iter().collect();
	for b in 0..8 {
		if let Some(first) = make(b).first() {
			out.extend([refs.get(first).copied().unwrap_or(0xdd), hashed.contains(first) as u8]);
		}
	}
}

/// Arrays of bytes std takes by reference: copied into each collection `extend` fills from
/// references to them, lent to an adaptor's closure as std lends the items it holds, and
/// held as `&[u8; N]` keys beside `[u8; N]` values. Of two bytes, and of none.
pub fn byte_refs(kind: u8, data: &[u8], out: &mut Vec<u8>) {
	match kind % 2 {
		0 => taken(data, |w| [w[0] % 4, w[1] % 4], out),
		_ => taken(data, |_| [0u8; 0], out),
	}
}

/// Pushes its byte to the log it shares as it drops, and its clone another.
struct Dropped<'a>(u8, &'a RefCell<Vec<u8>>);

impl Dropped<'_> {
	/// Whether it has a byte, read through the whole value, so a closure captures it whole.
	fn live(&self) -> bool {
		self.0 != 0
	}
}

impl Clone for Dropped<'_> {
	fn clone(&self) -> Self {
		Dropped(self.0.wrapping_add(0x10), self.1)
	}
}

impl Drop for Dropped<'_> {
	fn drop(&mut self) {
		self.1.borrow_mut().push(self.0);
	}
}

/// `byte_refs` of the arrays `make` makes of each window of two bytes.
fn taken<const N: usize>(data: &[u8], make: impl Fn(&[u8]) -> [u8; N], out: &mut Vec<u8>) {
	let mut pairs: Vec<[u8; N]> = data.windows(2).map(make).collect();
	let mut vec: Vec<[u8; N]> = Vec::new();
	vec.extend(&pairs);
	let mut deque: VecDeque<[u8; N]> = VecDeque::new();
	deque.extend(pairs.iter());
	let mut heap: BinaryHeap<[u8; N]> = BinaryHeap::new();
	heap.extend(&pairs);
	let mut set: HashSet<[u8; N]> = HashSet::new();
	set.extend(&pairs);
	let mut ordered: BTreeSet<[u8; N]> = BTreeSet::new();
	ordered.extend(pairs.iter());
	let mut map: HashMap<[u8; N], [u8; N]> = HashMap::new();
	map.extend(pairs.iter().map(|p| (p, p)));
	let mut tree: BTreeMap<[u8; N], [u8; N]> = BTreeMap::new();
	tree.extend(pairs.iter().zip(&pairs));
	// Each holds its own copies: a write through one leaves the source and the others.
	for p in pairs.iter_mut() {
		if let Some(b) = p.first_mut() {
			*b ^= 0x10;
		}
	}
	for v in vec.iter_mut() {
		if let Some(b) = v.last_mut() {
			*b ^= 0x20;
		}
	}
	for v in deque.iter_mut() {
		if let Some(b) = v.first_mut() {
			*b ^= 0x40;
		}
	}
	for v in map.values_mut() {
		if let Some(b) = v.last_mut() {
			*b ^= 0x80;
		}
	}
	for v in tree.values_mut() {
		if let Some(b) = v.first_mut() {
			*b ^= 0x08;
		}
	}
	out.extend(pairs.iter().flatten());
	out.extend(vec.iter().flatten());
	out.extend(deque.iter().flatten());
	out.extend(heap.into_sorted_vec().iter().flatten());
	let mut members: Vec<[u8; N]> = set.iter().copied().collect();
	members.sort();
	out.extend(members.iter().flatten());
	out.extend(ordered.iter().flatten());
	let mut entries: Vec<([u8; N], [u8; N])> = map.iter().map(|(k, v)| (*k, *v)).collect();
	entries.sort();
	for (k, v) in &entries {
		out.extend(k.iter().chain(v));
	}
	for (k, v) in &tree {
		out.extend(k.iter().chain(v));
	}
	out.extend([vec.len(), deque.len(), set.len(), ordered.len(), map.len(), tree.len()].map(|n| n as u8));
	// std's adaptors lend their closures each array they hold.
	out.push(vec.clone().into_iter().filter(|p| p.first() != p.last()).count() as u8);
	let kept: Vec<[u8; N]> = pairs.iter().copied().filter(|p| p.iter().any(|&b| b < 2)).collect();
	out.extend(kept.iter().flatten());
	let mut rest = deque
		.clone()
		.into_iter()
		.skip_while(|p| p.first() == Some(&0x40))
		.peekable();
	if let Some(p) = rest.peek() {
		out.extend_from_slice(p);
	}
	if let Some(p) = rest.peek_mut()
		&& let Some(b) = p.first_mut()
	{
		*b = 0xaa;
	}
	out.extend(rest.next().iter().flatten());
	let mut seen = 0u8;
	let taken: Vec<[u8; N]> = (vec.iter().copied())
		.inspect(|p| seen = seen.wrapping_add(p.len() as u8))
		.take_while(|p| p.last() != Some(&0x23))
		.collect();
	out.extend([seen, taken.len() as u8]);
	out.extend(vec.iter().copied().max_by(|a, b| a.cmp(b)).iter().flatten());
	out.extend(
		vec.iter()
			.copied()
			.min_by_key(|p| p.iter().fold(0u8, |a, b| a ^ b))
			.iter()
			.flatten(),
	);
	out.extend(
		pairs
			.iter()
			.copied()
			.find(|p| p.iter().sum::<u8>() > 0x12)
			.iter()
			.flatten(),
	);
	out.push(vec.iter().copied().is_sorted_by(|a, b| a <= b) as u8);
	out.extend(deque.iter().copied().rfind(|p| p < &[0x50; N]).iter().flatten());
	// A closure std lends arrays to drops where std drops it, and a held one's clone is its
	// own, each logged as it drops.
	let log = RefCell::new(Vec::new());
	let first = Dropped(1, &log);
	let sorted = vec.iter().copied().is_sorted_by(move |a, b| first.live() && a <= b);
	log.borrow_mut().push(sorted as u8 | 0x80);
	let second = Dropped(2, &log);
	let filtered = (vec.clone().into_iter()).filter(move |p| second.live() && p.last() != Some(&0x21));
	let again = filtered.clone();
	let counted = again.count() as u8;
	log.borrow_mut().push(counted);
	let counted = filtered.count() as u8;
	log.borrow_mut().push(counted);
	out.extend(log.take());
	// A map's references to the arrays it holds, mapped and collected.
	let mut lent: Vec<(&[u8; N], &[u8; N])> = map.iter().map(|(k, v)| (k, v)).collect();
	lent.sort();
	for (k, v) in lent {
		out.extend(k.iter().chain(v));
	}
	let by_key: HashMap<&[u8; N], u8> = tree.iter().map(|(k, v)| (k, v.len() as u8)).collect();
	out.push(by_key.len() as u8);
	// Keys held as `&[u8; N]` beside values held as `[u8; N]`, each lent as its own slice.
	let mut refs: HashMap<&[u8; N], [u8; N]> = pairs.iter().map(|p| (p, *p)).collect();
	let mut sorted: BTreeMap<&[u8; N], [u8; N]> = pairs.iter().map(|p| (p, *p)).collect();
	for (k, v) in refs.iter_mut() {
		if let Some(b) = v.last_mut() {
			*b ^= k.first().copied().unwrap_or(1);
		}
	}
	for v in sorted.values_mut() {
		if let Some(b) = v.first_mut() {
			*b = b.wrapping_add(1);
		}
	}
	let probe: [u8; N] = pairs.first().map_or([0; N], |p| *p);
	out.extend(refs.get(&probe).into_iter().flatten());
	out.extend(refs.get(&&probe).into_iter().flatten());
	out.extend(sorted.get(&probe).into_iter().flatten());
	if let Some((k, v)) = refs.get_key_value(&probe) {
		out.extend(k.iter().chain(v));
	}
	out.extend(refs.entry(&probe).or_insert([0xee; N]).iter());
	let mut keys: Vec<&[u8; N]> = refs.keys().copied().collect();
	keys.sort();
	out.extend(keys.iter().copied().flatten());
	let mut values: Vec<[u8; N]> = refs.values().copied().collect();
	values.sort();
	out.extend(values.iter().flatten());
	for (k, v) in &sorted {
		out.extend(k.iter().chain(v));
	}
	refs.retain(|k, v| k.first() == v.first());
	sorted.retain(|k, v| k.last() != v.last());
	out.extend([refs.len() as u8, sorted.len() as u8]);
	out.extend(sorted.into_keys().next().iter().copied().flatten());
	out.extend(refs.into_values().map(|v| v.len() as u8));
}
