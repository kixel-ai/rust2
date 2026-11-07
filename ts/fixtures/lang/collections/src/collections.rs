//! Collections, each driven by a script of operations read from the input: keys JS
//! compares as Rust does, 64-bit keys past 2^53, strings, and structs, tuples, and enums
//! found through their translated `Hash` and `Eq` or ordered by their `Ord`. A hash
//! map's iteration order is unspecified, so only what does not depend on it is written.

use std::borrow::Cow;
use std::cmp::{Ordering, Reverse};
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque, btree_map, hash_map};
use std::hash::{DefaultHasher, Hash, Hasher};
use std::num::Wrapping;
use std::ops::Bound;

/// A key whose `Eq`, `Hash`, and `Ord` see only its low nibble, so two keys can be equal
/// while their bytes differ, and which sorts by that nibble, descending.
#[derive(Clone, Copy)]
struct Nibble(u8);

impl PartialEq for Nibble {
	fn eq(&self, other: &Self) -> bool {
		self.0 & 0xf == other.0 & 0xf
	}
}

impl Eq for Nibble {}

impl Hash for Nibble {
	fn hash<H: Hasher>(&self, state: &mut H) {
		(self.0 & 0xf).hash(state);
	}
}

impl Ord for Nibble {
	fn cmp(&self, other: &Self) -> Ordering {
		(other.0 & 0xf).cmp(&(self.0 & 0xf))
	}
}

impl PartialOrd for Nibble {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Pair {
	a: u8,
	b: u16,
}

/// A value changed in place through `get_mut` and `entry`: a JS number can't be.
#[derive(Clone, Copy, Default)]
struct Tally {
	n: u16,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Rank {
	Low,
	Mid(u8),
	High { x: u8, y: u8 },
}

/// A key whose `Hash` writes nothing, so every key collides.
#[derive(Clone, Copy, PartialEq, Eq)]
struct Clash(u8);

impl Hash for Clash {
	fn hash<H: Hasher>(&self, _: &mut H) {}
}

/// A fieldless enum whose own `Eq` and `Hash` find `Up` and `Top` equal, which their JS
/// tags are not.
#[derive(Clone, Copy)]
enum Dir {
	Up,
	Top,
	Down,
}

impl Dir {
	fn rising(self) -> bool {
		!matches!(self, Dir::Down)
	}
}

impl PartialEq for Dir {
	fn eq(&self, other: &Self) -> bool {
		self.rising() == other.rising()
	}
}

impl Eq for Dir {}

impl Hash for Dir {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.rising().hash(state);
	}
}

/// A value whose `clone` panics on a multiple of 7, saying which.
#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Fussy(u8);

impl Clone for Fussy {
	fn clone(&self) -> Self {
		if self.0 % 7 == 0 {
			panic!("cloning {}", self.0);
		}
		Fussy(self.0)
	}
}

/// Strings whose UTF-8 order differs from their UTF-16 order: past U+FFFF, a surrogate
/// pair sorts below U+E000 in UTF-16, and above it in UTF-8, as Rust sorts.
const WORDS: [&str; 8] = ["", "a", "ab", "b", "\u{e9}", "\u{e000}", "\u{ffff}", "\u{10000}"];

/// A 64-bit key: small, or past 2^53, where it is no longer a JS number.
fn wide(b: u8) -> u64 {
	match b % 3 {
		0 => b as u64,
		1 => (b as u64) << 50,
		_ => u64::MAX - b as u64,
	}
}

fn rank(b: u8, c: u8) -> Rank {
	match b % 3 {
		0 => Rank::Low,
		1 => Rank::Mid(c % 4),
		_ => Rank::High { x: c % 2, y: c % 3 },
	}
}

fn rank_byte(r: &Rank) -> u8 {
	match *r {
		Rank::Low => 0,
		Rank::Mid(m) => 0x10 | m,
		Rank::High { x, y } => 0x20 | x << 2 | y,
	}
}

/// A `HashMap` by each kind of key, through a script of inserts, lookups, and removals.
pub fn hash_maps(kind: u8, data: &[u8], out: &mut Vec<u8>) {
	match kind % 4 {
		0 => {
			let mut map: HashMap<u8, u16> = HashMap::new();
			for i in (0..data.len()).step_by(2) {
				let (k, v) = (data[i] % 16, data.get(i + 1).copied().unwrap_or(0) as u16);
				match v % 5 {
					0 => out.push(map.remove(&k).map_or(0xee, |old| old as u8)),
					1 => out.push(map.contains_key(&k) as u8),
					2 => out.push(map.get(&k).map_or(0xdd, |v| *v as u8)),
					_ => out.push(map.insert(k, v * 3).map_or(0xcc, |old| old as u8)),
				}
			}
			out.push(map.len() as u8);
			out.push(map.is_empty() as u8);
			let mut it = map.iter();
			out.push(it.by_ref().count() as u8);
			out.push(it.next().is_none() as u8);
			let total: u16 = map.values().map(|v| v % 256).sum();
			out.extend_from_slice(&total.to_be_bytes());
			let sorted: BTreeMap<u8, u16> = map.iter().map(|(k, v)| (*k, *v)).collect();
			for (k, v) in &sorted {
				out.push(*k);
				out.extend_from_slice(&v.to_be_bytes());
			}
			map.clear();
			out.push(map.len() as u8);
		}
		1 => {
			let mut map: HashMap<u64, u8> = HashMap::with_capacity(4);
			for (i, &b) in data.iter().enumerate() {
				let k = wide(b % 12);
				match b / 12 % 4 {
					0 => out.push(map.remove(&k).unwrap_or(0xee)),
					1 => out.push(map.get(&k).copied().unwrap_or(0xdd)),
					_ => out.push(map.insert(k, i as u8).unwrap_or(0xcc)),
				}
			}
			let keys: BTreeSet<u64> = map.keys().copied().collect();
			for k in &keys {
				out.extend_from_slice(&k.to_be_bytes());
				out.push(map[k]);
			}
			// A missing key panics.
			if let Some(&b) = data.first() {
				out.push(map[&wide(b % 13)]);
			}
		}
		2 => {
			// The key an insert keeps is the first equal one: Rust replaces only the value.
			let mut map: HashMap<Nibble, u8> = HashMap::new();
			let mut pairs: HashMap<Pair, Tally> = HashMap::new();
			let mut ranks: HashMap<Rank, Tally> = HashMap::new();
			let mut tuples: HashMap<(u8, &str), u8> = HashMap::new();
			for i in 1..data.len() {
				let (a, b) = (data[i - 1], data[i]);
				out.push(map.insert(Nibble(a), b).unwrap_or(0xcc));
				let pair = Pair {
					a: a % 4,
					b: b as u16 % 3,
				};
				let t = pairs.entry(pair).or_default();
				t.n += 1;
				let r = rank(a, b);
				if let Some(t) = ranks.get_mut(&r) {
					t.n += 1;
				} else {
					ranks.insert(r, Tally { n: i as u16 });
				}
				let t = (a % 3, WORDS[b as usize % WORDS.len()]);
				out.push(tuples.remove(&t).unwrap_or(0xbb));
				if a % 2 == 0 {
					tuples.insert(t, a);
				}
			}
			let raw: BTreeMap<u8, u8> = map.iter().map(|(k, v)| (k.0, *v)).collect();
			for (k, v) in raw {
				out.push(k);
				out.push(v);
			}
			// A stored value is its own copy, as is one read back out.
			let key = Pair { a: 9, b: 9 };
			let mut local = Tally { n: 7 };
			pairs.insert(key, local);
			local.n = 8;
			let before = pairs[&key];
			if let Some(t) = pairs.get_mut(&key) {
				t.n += 2;
			}
			out.push(local.n as u8);
			out.push(before.n as u8);
			out.push(pairs[&key].n as u8);
			let pairs: BTreeMap<Pair, Tally> = pairs.into_iter().collect();
			for (p, t) in pairs {
				out.push(p.a);
				out.push(p.b as u8);
				out.push(t.n as u8);
			}
			// The entry API, and a clone whose values change apart from its source's.
			for i in 1..data.len() {
				let (a, b) = (data[i - 1], data[i]);
				let t = ranks
					.entry(rank(b, a))
					.and_modify(|t| t.n ^= 3)
					.or_insert_with(|| Tally { n: b as u16 });
				t.n = t.n.wrapping_add(t.n);
				let e = ranks.entry(rank(data[i], 0));
				out.push(rank_byte(e.key()));
				out.push(e.or_insert_with_key(|k| Tally { n: rank_byte(k) as u16 }).n as u8);
			}
			let mut copy = ranks.clone();
			if let Some(t) = copy.get_mut(&Rank::Low) {
				t.n = 0x77;
			}
			out.push(ranks.get(&Rank::Low).map_or(0xff, |t| t.n as u8));
			out.push(copy.get(&Rank::Low).map_or(0xff, |t| t.n as u8));
			out.push(
				copy.remove_entry(&Rank::Mid(1))
					.map_or(0xff, |(k, t)| rank_byte(&k) ^ t.n as u8),
			);
			let bytes: HashMap<u8, Tally> = data.iter().map(|&b| (b % 8, Tally { n: b as u16 })).collect();
			let mut copy = bytes.clone();
			for t in copy.values_mut() {
				t.n += 1;
			}
			for k in 0..8 {
				out.push(bytes.get(&k).map_or(0xff, |t| t.n as u8));
				out.push(copy.get(&k).map_or(0xff, |t| t.n as u8));
			}
			for &b in data {
				out.push(claim(&mut copy, b % 8, b as u16));
			}
			// A capacity call still evaluates its receiver.
			let mut stamped: HashMap<u8, Tally> = HashMap::new();
			stamp(&mut stamped).shrink_to_fit();
			stamp(&mut stamped).reserve(2);
			out.push(stamped.len() as u8);
			// Extending by reference copies each value.
			let mut extended: HashMap<u8, Tally> = HashMap::new();
			extended.extend(&bytes);
			for t in extended.values_mut() {
				t.n ^= 0x11;
			}
			for k in 0..8 {
				out.push(bytes.get(&k).map_or(0xff, |t| t.n as u8));
				out.push(extended.get(&k).map_or(0xff, |t| t.n as u8));
			}
			// Equal keys hash equal through `DefaultHasher`, whose values are otherwise unspecified.
			let hash = |n: Nibble| {
				let mut h = DefaultHasher::new();
				n.hash(&mut h);
				h.finish()
			};
			let first = data.first().copied().unwrap_or(0);
			out.push((hash(Nibble(first)) == hash(Nibble(first ^ 0xf0))) as u8);
			// A hasher found by a call evaluates before the value written to it.
			let (mut h, mut counted) = (DefaultHasher::new(), Tally { n: first as u16 });
			counted_hasher(&mut h, &mut counted).write_u16(counted.n);
			let mut want = DefaultHasher::new();
			want.write_u16(first as u16 + 1);
			out.push((h.finish() == want.finish()) as u8);
			ranks.retain(|r, t| t.n % 2 == 0 || *r == Rank::Low);
			let ranks: BTreeMap<Rank, u16> = ranks.iter().map(|(r, t)| (*r, t.n)).collect();
			for (r, n) in &ranks {
				out.push(rank_byte(r));
				out.push(*n as u8);
			}
			out.push(tuples.len() as u8);
		}
		_ => {
			let mut words: HashMap<String, Vec<u8>> = HashMap::new();
			for &b in data {
				let word = WORDS[b as usize % WORDS.len()].to_string();
				words.entry(word).or_default().push(b);
			}
			let mut lens: BTreeMap<String, usize> = BTreeMap::new();
			for (w, bytes) in &words {
				lens.insert(w.clone(), bytes.len());
			}
			for (w, n) in &lens {
				out.extend_from_slice(w.as_bytes());
				out.push(*n as u8);
			}
			let drained: usize = words.drain().map(|(_, v)| v.len()).sum();
			out.push(drained as u8);
			out.push(words.is_empty() as u8);
			// Every key hashes alike, so each lookup walks the keys that collide.
			let mut clashes: HashMap<Clash, Tally> = HashMap::new();
			for (i, &b) in data.iter().enumerate() {
				let k = Clash(b % 8);
				match b / 8 % 4 {
					0 => out.push(clashes.remove(&k).map_or(0xee, |t| t.n as u8)),
					1 => out.push(clashes.get(&k).map_or(0xdd, |t| t.n as u8)),
					_ => out.push(clashes.insert(k, Tally { n: i as u16 }).map_or(0xcc, |t| t.n as u8)),
				}
			}
			clashes.retain(|k, t| k.0 % 3 != 0 || t.n % 2 == 0);
			let kept: BTreeSet<u8> = clashes.keys().map(|k| k.0).collect();
			out.extend(kept.iter());
			let copy = clashes.clone();
			out.push(copy.len() as u8);
			out.push(copy.contains_key(&Clash(1)) as u8);
		}
	}
}

/// Matching on an entry: add to an even tally, take out an odd one, or start one.
/// Counts a write, and gives back the hasher.
fn counted_hasher<'a>(h: &'a mut DefaultHasher, counted: &mut Tally) -> &'a mut DefaultHasher {
	counted.n += 1;
	h
}

/// Inserts a key, and gives back the map.
fn stamp(map: &mut HashMap<u8, Tally>) -> &mut HashMap<u8, Tally> {
	map.insert(map.len() as u8, Tally::default());
	map
}

fn claim(map: &mut HashMap<u8, Tally>, k: u8, n: u16) -> u8 {
	match map.entry(k) {
		hash_map::Entry::Occupied(mut e) if e.get().n % 2 == 0 => {
			let t = e.get_mut();
			t.n += n;
			1
		}
		hash_map::Entry::Occupied(e) => e.remove().n as u8,
		hash_map::Entry::Vacant(e) => {
			e.insert(Tally { n });
			0
		}
	}
}

/// `HashSet` and `BTreeSet`, each with the same script.
pub fn sets(kind: u8, data: &[u8], out: &mut Vec<u8>) {
	match kind % 3 {
		0 => {
			let mut hashed: HashSet<u16> = HashSet::new();
			let mut sorted: BTreeSet<u16> = BTreeSet::new();
			for &b in data {
				let v = b as u16 % 24;
				match b % 3 {
					0 => {
						out.push(hashed.remove(&v) as u8);
						out.push(sorted.remove(&v) as u8);
					}
					1 => {
						out.push(hashed.contains(&v) as u8);
						out.push(sorted.contains(&v) as u8);
					}
					_ => {
						out.push(hashed.insert(v) as u8);
						out.push(sorted.insert(v) as u8);
					}
				}
			}
			out.push(hashed.len() as u8);
			out.push(hashed.iter().filter(|v| **v % 2 == 1).count() as u8);
			for v in sorted.iter().rev() {
				out.push(*v as u8);
			}
			out.push(sorted.first().copied().unwrap_or(0xff) as u8);
			out.push(sorted.last().copied().unwrap_or(0xff) as u8);
			let probe = data.first().map_or(0, |&b| b as u16 % 24);
			for v in [
				hashed.replace(probe),
				sorted.replace(probe),
				sorted.get(&probe).copied(),
			] {
				out.push(v.unwrap_or(0xff) as u8);
			}
			for v in [hashed.take(&probe), sorted.take(&probe), sorted.take(&probe)] {
				out.push(v.unwrap_or(0xff) as u8);
			}
			let firsts: Vec<u16> = sorted.range(4..12).copied().collect();
			out.push(firsts.len() as u8);
			hashed.retain(|v| v % 3 != 0);
			sorted.retain(|v| v % 3 != 0);
			out.push(hashed.len() as u8);
			out.extend(sorted.iter().map(|v| *v as u8));
		}
		1 => {
			let mut nibbles: BTreeSet<Nibble> = BTreeSet::new();
			let mut seen: HashSet<Nibble> = HashSet::new();
			for &b in data {
				out.push(nibbles.insert(Nibble(b)) as u8);
				out.push(seen.insert(Nibble(b)) as u8);
			}
			// The equal key each holds apart from the one given: `replace` keeps the new one.
			let held = |n: Option<Nibble>| n.map_or(0xee, |n| n.0);
			let lent = |n: Option<&Nibble>| n.map_or(0xee, |n| n.0);
			for &b in data.iter().rev() {
				match b % 3 {
					0 => out.extend([
						held(nibbles.replace(Nibble(b ^ 0x30))),
						held(seen.replace(Nibble(b ^ 0x30))),
					]),
					1 => out.extend([
						held(nibbles.take(&Nibble(b ^ 0x50))),
						held(seen.take(&Nibble(b ^ 0x50))),
					]),
					_ => out.extend([lent(nibbles.get(&Nibble(b ^ 0x70))), lent(seen.get(&Nibble(b ^ 0x70)))]),
				}
			}
			out.extend(seen.get(&Nibble(0)).map(|n| n.0));
			for n in &nibbles {
				out.push(n.0);
			}
			while let Some(n) = nibbles.pop_last() {
				out.push(n.0);
				if nibbles.len() % 2 == 0 {
					out.push(nibbles.pop_first().map_or(0xee, |n| n.0));
				}
			}
			out.push(seen.len() as u8);
			let mut dirs: HashSet<Dir> = HashSet::new();
			for &b in data {
				let dir = [Dir::Up, Dir::Top, Dir::Down][b as usize % 3];
				out.push(dirs.insert(dir) as u8);
			}
			out.push(dirs.len() as u8);
		}
		_ => {
			let mut words: BTreeSet<&str> = BTreeSet::new();
			for &b in data {
				words.insert(WORDS[b as usize % WORDS.len()]);
			}
			for w in &words {
				out.extend_from_slice(w.as_bytes());
				out.push(0);
			}
			let wides: BTreeSet<u64> = data.iter().map(|&b| wide(b)).collect();
			for w in wides.iter().rev() {
				out.extend_from_slice(&w.to_be_bytes());
			}
		}
	}
}

/// `BTreeMap`: ordered by the key's `Ord`, through inserts, removals, and ranges.
pub fn btree_maps(kind: u8, data: &[u8], out: &mut Vec<u8>) {
	match kind % 4 {
		0 => {
			let mut map: BTreeMap<u8, u8> = BTreeMap::new();
			for i in (0..data.len()).step_by(2) {
				let (k, v) = (data[i] % 32, data.get(i + 1).copied().unwrap_or(0));
				match v % 6 {
					0 => out.push(map.remove(&k).unwrap_or(0xee)),
					1 => out.push(map.pop_first().map_or(0xdd, |(k, v)| k ^ v)),
					2 => out.push(map.pop_last().map_or(0xdc, |(k, v)| k ^ v)),
					3 => {
						if let Some(v) = map.get(&k).copied() {
							map.insert(k, v.wrapping_add(1));
						}
						out.push(map.get(&k).copied().unwrap_or(0xdb));
					}
					_ => out.push(map.insert(k, v).unwrap_or(0xcc)),
				}
			}
			out.push(map.first_key_value().map_or(0xff, |(k, _)| *k));
			out.push(map.last_key_value().map_or(0xff, |(k, _)| *k));
			for (k, v) in map.range(8..24) {
				out.push(*k);
				out.push(*v);
			}
			for (k, v) in map.range(10..).rev() {
				out.push(k ^ v);
			}
			for v in map.range(..6).map(|(_, v)| *v) {
				out.push(v);
			}
			out.extend(map.keys().copied());
			out.extend(map.values().copied());
			// Ends, which a B-tree's own iterators read without comparing.
			out.push(map.keys().max().map_or(0xff, |k| *k));
			out.push(map.iter().min().map_or(0xff, |(k, v)| k ^ v));
			out.push(map.range(4..).max().map_or(0xff, |(k, v)| k ^ v));
			out.push(map.values().max().map_or(0xff, |v| *v));
			map.remove(&3);
			out.push(map.len() as u8);
			out.push(map.contains_key(&3) as u8);
			// A range whose start is past its end panics.
			let lo = data.first().copied().unwrap_or(0) % 32;
			let hi = data.get(1).copied().unwrap_or(0) % 32;
			let within: Vec<u8> = match lo % 3 {
				0 => map.range(lo..hi).map(|(k, _)| *k).collect(),
				1 => map.range(lo..=hi).map(|(k, _)| *k).collect(),
				_ => map.range(lo..).map(|(k, _)| *k).collect(),
			};
			out.extend_from_slice(&within);
			out.extend(map.into_values());
		}
		1 => {
			let mut ranks: BTreeMap<Rank, Tally> = BTreeMap::new();
			let mut pairs: BTreeMap<(u8, u8), u8> = BTreeMap::new();
			let mut wides: BTreeMap<u64, u8> = BTreeMap::new();
			for i in 1..data.len() {
				let (a, b) = (data[i - 1], data[i]);
				let t = ranks.entry(rank(a, b)).or_default();
				t.n = t.n.wrapping_add(b as u16);
				pairs.insert((a % 4, b % 4), a);
				wides.entry(wide(a)).or_insert(b);
			}
			if let Some(t) = ranks.get_mut(&Rank::Low) {
				t.n ^= 0x5a5a;
			}
			for &b in data {
				let r = rank(b, b / 3);
				match ranks.entry(r) {
					btree_map::Entry::Vacant(e) => {
						out.push(rank_byte(e.key()));
						e.insert(Tally { n: b as u16 });
					}
					btree_map::Entry::Occupied(e) if e.get().n > 0x100 => out.push(rank_byte(&e.remove_entry().0)),
					btree_map::Entry::Occupied(mut e) => out.push(e.insert(Tally { n: 0x200 }).n as u8),
				}
			}
			for (r, t) in &ranks {
				out.push(rank_byte(r));
				out.extend_from_slice(&t.n.to_be_bytes());
			}
			for ((a, b), v) in &pairs {
				out.push(a << 4 | b);
				out.push(*v);
			}
			for (k, v) in wides.range(1u64 << 52..) {
				out.extend_from_slice(&k.to_be_bytes());
				out.push(*v);
			}
		}
		// `clone` clones each key and then its value, so which `Fussy` panics first is std's.
		2 => {
			let map: BTreeMap<Fussy, Fussy> = data.iter().map(|&b| (Fussy(b), Fussy(b ^ 0x5a))).collect();
			let copy = map.clone();
			for (k, v) in &copy {
				out.push(k.0 ^ v.0);
			}
		}
		_ => {
			let mut nibbles: BTreeMap<Nibble, u8> = BTreeMap::new();
			let mut words: BTreeMap<&str, Tally> = BTreeMap::new();
			for &b in data {
				out.push(nibbles.insert(Nibble(b), b).unwrap_or(0xcc));
				let t = words.entry(WORDS[b as usize % WORDS.len()]).or_default();
				t.n += 1;
			}
			for (k, v) in &nibbles {
				out.push(k.0);
				out.push(*v);
			}
			// `Ordering`'s methods, on an `Ord` that sorts descending.
			for i in 1..data.len() {
				let (a, b) = (Nibble(data[i - 1]), Nibble(data[i]));
				let (x, y) = (data[i - 1], data[i]);
				let o = a.cmp(&b).reverse().then_with(|| y.cmp(&x));
				out.push(
					o.is_lt() as u8 | (o.is_ge() as u8) << 1 | (a.cmp(&b).then(Ordering::Less).is_eq() as u8) << 2,
				);
				out.push(match o {
					Ordering::Less => 1,
					Ordering::Equal => 2,
					Ordering::Greater => 3,
				});
				out.push((o == Ordering::Greater) as u8);
			}
			// `collect` keeps the last of equal keys, and `extend` the first.
			let collected: BTreeMap<Nibble, u8> = data.iter().map(|&b| (Nibble(b), b)).collect();
			let mut keys: BTreeSet<Nibble> = data.iter().map(|&b| Nibble(b)).collect();
			keys.extend(data.iter().map(|&b| Nibble(b ^ 0x30)));
			for ((k, v), n) in collected.iter().zip(keys.iter()) {
				out.push(k.0);
				out.push(*v);
				out.push(n.0);
			}
			words.retain(|w, t| w.len() > 1 || t.n > 1);
			for (w, t) in &words {
				out.extend_from_slice(w.as_bytes());
				out.push(t.n as u8);
			}
		}
	}
}

/// Pushes an item, and gives back the deque.
fn touch(q: &mut VecDeque<u8>) -> &mut VecDeque<u8> {
	q.push_back(0x42);
	q
}

/// `VecDeque`: a ring buffer, pushed and popped at both ends past its capacity.
pub fn deques(data: &[u8], out: &mut Vec<u8>) {
	let mut q: VecDeque<u8> = VecDeque::new();
	let mut pairs: VecDeque<(u8, u8)> = VecDeque::with_capacity(2);
	for &b in data {
		match b % 7 {
			0 => out.push(q.pop_front().unwrap_or(0xee)),
			1 => out.push(q.pop_back().unwrap_or(0xed)),
			2 => q.push_front(b),
			3 => {
				out.push(q.front().copied().unwrap_or(0xec));
				out.push(q.back().copied().unwrap_or(0xeb));
				out.push(q.get(b as usize % 5).copied().unwrap_or(0xea));
			}
			_ => q.push_back(b),
		}
		pairs.push_back((b, q.len() as u8));
		if pairs.len() > 3 {
			pairs.pop_front();
		}
	}
	// The item at an index, or none past the end, the end's moved into its place.
	let at = data.last().map_or(0, |&b| b as usize % 6);
	out.push(q.swap_remove_back(at).unwrap_or(0xe8));
	out.push(q.swap_remove_front(at).unwrap_or(0xe7));
	touch(&mut q).shrink_to_fit();
	touch(&mut q).reserve(1);
	out.push(q.len() as u8);
	out.extend(q.iter().rev());
	if !q.is_empty() {
		out.push(q[q.len() / 2]);
	}
	q.retain(|b| b % 2 == 0);
	out.extend(q.iter());
	for (a, b) in &pairs {
		out.push(a ^ b);
	}
	let drained: Vec<u8> = q.drain(..).collect();
	out.extend_from_slice(&drained);
	out.push(q.is_empty() as u8);
	// A clone's items change apart from its source's.
	let tallies: VecDeque<Tally> = data.iter().map(|&b| Tally { n: b as u16 }).collect();
	let mut copy = tallies.clone();
	if let Some(t) = copy.front_mut() {
		t.n ^= 0xff;
	}
	out.extend(tallies.iter().map(|t| t.n as u8));
	out.extend(copy.iter().map(|t| t.n as u8));
	copy.extend(tallies.iter());
	if let Some(t) = copy.back_mut() {
		t.n ^= 0xff;
	}
	out.extend(tallies.iter().map(|t| t.n as u8));
	out.extend(copy.iter().map(|t| t.n as u8));
	for t in [copy.swap_remove_front(at), copy.swap_remove_back(at)] {
		out.push(t.map_or(0xe6, |t| t.n as u8));
	}
	out.extend(copy.iter().map(|t| t.n as u8));
	q.extend(data.iter().copied());
	q.insert(q.len() / 2, 0x42);
	out.push(q.remove(1).unwrap_or(0xe9));
	q.truncate(5);
	out.push(q.contains(&0x42) as u8);
	out.extend(q.iter().rev());
	// Draining or indexing past the end panics.
	let a = data.first().copied().unwrap_or(0) as usize % 8;
	let b = data.get(1).copied().unwrap_or(0) as usize % 8;
	let mut q: VecDeque<u8> = data.iter().copied().collect();
	let drained: Vec<u8> = match a % 4 {
		0 => q.drain(a..b).collect(),
		1 => q.drain(a..=b).collect(),
		2 => q.drain(..b).collect(),
		_ => q.drain(a..).collect(),
	};
	out.extend_from_slice(&drained);
	out.extend(q.iter());
	out.push(q[b]);
}

/// `VecDeque::range` by each form of range, over a ring buffer whose front items wrap past
/// its end, read from both ends and cloned midway, with `{:?}` of what is left after each
/// step, which shows the two slices std's splits it into; a range past the length or
/// backwards panics as std's does.
pub fn deque_ranges(kind: u8, data: &[u8], out: &mut Vec<u8>) {
	let (a, b, items) = match data {
		[a, b, items @ ..] => (*a as usize % 8, *b as usize % 8, items),
		_ => (0, 0, data),
	};
	let mut q: VecDeque<u8> = items.iter().copied().collect();
	// Pushed before the first item, which starts the buffer, they wrap to its end.
	for i in 0..kind / 8 % 4 {
		q.push_front(0xf0 | i);
	}
	let mut it = match kind % 8 {
		0 => q.range(a..b),
		1 => q.range(a..=b),
		2 => q.range(..b),
		3 => q.range(a..),
		4 => q.range(..),
		5 => q.range((Bound::Excluded(a), Bound::Included(b))),
		6 => q.range(..=b),
		_ => q.range(a..a),
	};
	out.push(it.len() as u8);
	out.extend_from_slice(format!("{it:?}").as_bytes());
	// Each step reads from the back by the kind's high bits, in turn.
	let mut i = 0;
	loop {
		let item = match kind >> (5 + i % 3) & 1 {
			1 => it.next_back(),
			_ => it.next(),
		};
		let Some(&item) = item else {
			break;
		};
		out.push(item);
		out.extend_from_slice(format!("{it:?}").as_bytes());
		if i == 1 {
			out.extend(it.clone().rev());
		}
		i += 1;
	}
	out.push(0xfd);
	out.extend(q.range(a.min(q.len())..).rev());
}

/// A value that counts how far it is cloned from the one `vec!` was given, which
/// `vec![x; n]` clones for every element but the last, which is `x` itself.
struct Cloned {
	depth: u8,
	name: String,
}

impl Clone for Cloned {
	fn clone(&self) -> Self {
		Cloned {
			depth: self.depth + 1,
			name: self.name.clone(),
		}
	}
}

/// `vec![a, b]` and `vec![x; n]`, of bytes, numbers, `Copy` tuples, and types that clone.
pub fn vecs(data: &[u8], out: &mut Vec<u8>) {
	let a = data.first().copied().unwrap_or(0);
	let b = data.get(1).copied().unwrap_or(0);
	let n = data.get(2).copied().unwrap_or(0) as usize % 5;
	let name = match String::from_utf8(data.to_vec()) {
		Ok(s) => s,
		Err(_) => "?".to_string(),
	};

	let bytes = vec![a, b, 0x42];
	out.extend_from_slice(&bytes);
	let mut pairs = vec![(a, b as u16)];
	pairs.push((b, a as u16));
	pairs[0].0 ^= 0xff;
	for (x, y) in &pairs {
		out.push(x ^ *y as u8);
	}
	let names = vec![name.clone(), "x".to_string(), name.clone()];
	out.push(names.len() as u8);
	let one = vec![Cloned {
		depth: a,
		name: name.clone(),
	}];
	out.push(one[0].depth);

	out.extend_from_slice(&vec![b; n]);
	let wide = vec![a as u16 * 0x101; n];
	out.extend(wide.iter().map(|w| (w >> 4) as u8));
	let mut tuples = vec![(a, b); n];
	if n > 0 {
		tuples[0].0 = 0xee;
	}
	out.extend(tuples.iter().map(|t| t.0 ^ t.1));
	let strings = vec![name.clone(); n];
	out.push(strings.len() as u8);
	let cloned = vec![Cloned { depth: 0, name }; n];
	out.extend(cloned.iter().map(|c| c.depth));
	out.push(cloned.iter().all(|c| c.name.len() == data.len()) as u8);
	// A `Copy` type's own `Clone` runs too.
	let bumped = vec![Bumped(b); n];
	out.extend(bumped.iter().map(|o| o.0));

	// Each element is a copy of what it was given, apart from it and from each other.
	let pair = (a, b);
	let held = Bumped(a);
	let mut listed = vec![pair, pair];
	listed[0].0 ^= 0xff;
	let mut repeated = vec![pair; n];
	let again = vec![pair; n];
	let mut bumps = vec![held; n];
	let kept = vec![held; n];
	if n > 0 {
		repeated[n - 1].1 ^= 0xff;
		bumps[n - 1].0 ^= 0xff;
	}
	out.extend_from_slice(&[pair.0, pair.1, listed[1].0, held.0]);
	out.extend(repeated.iter().chain(&again).map(|t| t.1));
	out.extend(bumps.iter().chain(&kept).map(|o| o.0));

	// A `Vec`'s slice is bytes only when its own elements are bytes.
	let mut grown = bytes.clone();
	grown.as_mut_slice()[0] ^= 0xff;
	out.extend_from_slice(grown.as_slice());
	let nested = vec![bytes, vec![b; n]];
	for v in nested.as_slice() {
		out.push(v.len() as u8);
		out.extend_from_slice(v);
	}
	let mut wrapped = vec![Wrap { value: a, count: b }; n];
	if n > 0 {
		wrapped.as_mut_slice()[n - 1].count ^= 0xff;
	}
	out.extend(wrapped.as_slice().iter().map(|w| w.value ^ w.count));
	out.extend(names.as_slice().iter().map(|s| s.len() as u8));
}

/// A `Copy` type whose `Clone` does more than copy.
#[derive(Copy)]
struct Bumped(u8);

impl Clone for Bumped {
	fn clone(&self) -> Self {
		Bumped(self.0.wrapping_add(1))
	}
}

/// A generic `Copy` struct.
#[derive(Clone, Copy)]
struct Wrap<T> {
	value: T,
	count: u8,
}

/// A struct that is not `Copy` holding a `Copy` aggregate by value.
struct Holder {
	pair: (u8, u8),
}

/// Stores `p` by value in `v`.
fn keep(v: &mut Vec<(u8, u8)>, p: (u8, u8)) {
	v.push(p);
}

/// Gives back what it was given.
fn same(p: (u8, u8)) -> (u8, u8) {
	p
}

/// Changes what it was given in place, which is its own.
fn bump(mut h: Holder) -> u8 {
	h.pair.0 ^= 1;
	h.pair.0
}

/// Holds `pair`, for its caller to change.
fn hold(pair: (u8, u8)) -> Holder {
	Holder { pair }
}

/// A `Copy` aggregate a std call stores by value is the container's own copy: changing
/// one stored element leaves the value it came from and the other elements as they were.
pub fn stores(data: &[u8], out: &mut Vec<u8>) {
	let a = data.first().copied().unwrap_or(0);
	let b = data.get(1).copied().unwrap_or(0);
	// Made by a call, so its fields are read from it, not from `a` and `b`.
	let pair = same((a, b));
	let mut changed = (b, a);
	changed.0 ^= 0x10;

	let mut v = Vec::new();
	v.push(pair);
	v.push(pair);
	v[0].0 ^= 1;
	// A local changed in place, before and after it is stored.
	v.push(changed);
	changed.1 ^= 0x20;
	v[2].0 ^= 4;
	// Through a parameter, and through a call's result.
	keep(&mut v, pair);
	v[3].1 ^= 8;
	v.push(same(pair));
	v[4].0 ^= 0x40;
	// Each element of an array std takes apart.
	v.extend([pair, changed]);
	v[5].1 ^= 0x80;
	v[6].0 ^= 0x80;
	let old = std::mem::replace(&mut v[0], pair);
	v[0].1 ^= 0x10;
	out.extend(v.iter().flat_map(|p| [p.0, p.1]));
	out.extend_from_slice(&[changed.0, changed.1, old.0, old.1]);

	let tally = Tally { n: a as u16 };
	let mut q = VecDeque::new();
	q.push_back(tally);
	q.push_front(tally);
	q.insert(1, tally);
	if let Some(t) = q.front_mut() {
		t.n ^= 1;
	}
	if let Some(t) = q.back_mut() {
		t.n ^= 2;
	}
	out.extend(q.iter().map(|t| t.n as u8));
	out.push(tally.n as u8);

	let mut hash = HashMap::new();
	hash.insert(0u8, pair);
	hash.insert(1, pair);
	if let Some(p) = hash.get_mut(&0) {
		p.0 ^= 1;
	}
	hash.entry(2).or_insert(pair).1 ^= 2;
	out.extend((0..3).flat_map(|k| hash.get(&k).map_or([0xff; 2], |p| [p.0, p.1])));
	let mut tree = BTreeMap::new();
	tree.insert(0u8, pair);
	tree.insert(1, pair);
	if let Some(p) = tree.get_mut(&1) {
		p.1 ^= 1;
	}
	out.extend(tree.values().flat_map(|p| [p.0, p.1]));

	let mut boxed = Box::new(pair);
	boxed.0 ^= 1;
	let mut holders = vec![Holder { pair }];
	let holder = Holder { pair };
	holders.push(holder);
	holders[1].pair.1 ^= 2;
	out.extend_from_slice(&[boxed.0, boxed.1, holders[0].pair.1, holders[1].pair.1]);
	out.push(bump(Holder { pair }));
	let mut held = hold(pair);
	held.pair.1 ^= 4;
	holders.push(hold(pair));
	holders[2].pair.0 ^= 8;
	out.extend_from_slice(&[held.pair.1, holders[2].pair.0]);
	out.extend_from_slice(&[pair.0, pair.1]);
}

/// What a script of operations on a map of `V`s reads, written as text: `None` apart from
/// `Some(None)` where `V` is an `Option`, and `Some(())` where it is `()`.
trait OptionMap<V> {
	fn get(&self, k: u8) -> Option<&V>;
	fn insert(&mut self, k: u8, v: V) -> Option<V>;
	fn remove(&mut self, k: u8) -> Option<V>;
	fn get_mut(&mut self, k: u8) -> Option<&mut V>;
	fn sorted(&self) -> Vec<(u8, V)>;
}

impl<V: Clone> OptionMap<V> for HashMap<u8, V> {
	fn get(&self, k: u8) -> Option<&V> {
		HashMap::get(self, &k)
	}
	fn insert(&mut self, k: u8, v: V) -> Option<V> {
		HashMap::insert(self, k, v)
	}
	fn remove(&mut self, k: u8) -> Option<V> {
		HashMap::remove(self, &k)
	}
	fn get_mut(&mut self, k: u8) -> Option<&mut V> {
		HashMap::get_mut(self, &k)
	}
	fn sorted(&self) -> Vec<(u8, V)> {
		let mut all: Vec<(u8, V)> = self.iter().map(|(k, v)| (*k, v.clone())).collect();
		all.sort_by_key(|(k, _)| *k);
		all
	}
}

impl<V: Clone> OptionMap<V> for BTreeMap<u8, V> {
	fn get(&self, k: u8) -> Option<&V> {
		BTreeMap::get(self, &k)
	}
	fn insert(&mut self, k: u8, v: V) -> Option<V> {
		BTreeMap::insert(self, k, v)
	}
	fn remove(&mut self, k: u8) -> Option<V> {
		BTreeMap::remove(self, &k)
	}
	fn get_mut(&mut self, k: u8) -> Option<&mut V> {
		BTreeMap::get_mut(self, &k)
	}
	fn sorted(&self) -> Vec<(u8, V)> {
		self.iter().map(|(k, v)| (*k, v.clone())).collect()
	}
}

/// Run `data` on `map`, each byte a key in its low nibble and an operation above it, and
/// `value` making what `insert` and `get_mut` put.
fn optional<V: Clone + std::fmt::Debug, M: OptionMap<V>>(
	mut map: M,
	data: &[u8],
	value: fn(u8) -> V,
	out: &mut Vec<u8>,
) {
	for &b in data {
		let k = b % 8;
		let read = match b / 8 % 4 {
			0 => format!("{:?} ", map.insert(k, value(b))),
			1 => format!("{:?} ", map.get(k)),
			2 => format!("{:?} ", map.remove(k)),
			_ => match map.get_mut(k) {
				Some(v) => {
					*v = value(b / 2);
					format!("={v:?} ")
				}
				None => "- ".into(),
			},
		};
		out.extend_from_slice(read.as_bytes());
	}
	out.extend_from_slice(format!("{:?}", map.sorted()).as_bytes());
}

/// `b`, unless a multiple of 3.
fn some(b: u8) -> Option<u8> {
	(b % 3 != 0).then_some(b)
}

/// What `Default` makes of `T`, as kio's `Shared<T>::default` makes its map.
fn made<T: Default>() -> T {
	T::default()
}

/// Maps whose values are `Option`s or `()`, whose `get`, `insert`, and `remove` return an
/// `Option` of one: by key, by hash of a key that is an object, and in order.
pub fn optionals(kind: u8, data: &[u8], out: &mut Vec<u8>) {
	match kind % 6 {
		0 => optional(HashMap::<u8, Option<u8>>::new(), data, some, out),
		1 => optional(made::<BTreeMap<u8, Option<u8>>>(), data, some, out),
		2 => optional(HashMap::<u8, ()>::new(), data, |_| (), out),
		3 => optional(BTreeMap::<u8, ()>::new(), data, |_| (), out),
		4 => optional(
			made::<HashMap<u8, Option<String>>>(),
			data,
			|b| some(b).map(|b| b.to_string()),
			out,
		),
		// Keys found by their hash, and moq-net's reads of one: a `Some(None)` there, cloned
		// and flattened, and the entry API.
		_ => {
			let mut map: HashMap<(u8, bool), Option<u8>> = HashMap::new();
			for &b in data {
				let k = (b % 4, b & 0x10 != 0);
				let read = match b / 32 % 4 {
					0 => map.insert(k, some(b)),
					1 => map.get(&k).cloned(),
					2 => map.remove(&k),
					_ => Some(*map.entry(k).or_insert(None)),
				};
				let s = format!("{read:?} {:?} {} ", read.flatten(), map.contains_key(&k));
				out.extend_from_slice(s.as_bytes());
			}
			let mut all: Vec<_> = map.into_iter().collect();
			all.sort();
			out.extend_from_slice(format!("{all:?}").as_bytes());
		}
	}
}

/// `union`, `intersection`, `difference`, and `symmetric_difference` of two sets each way,
/// by method and by operator, and whether one is a subset, superset, or disjoint from the
/// other: of sets that overlap, are equal, are empty, are disjoint, or are one key against
/// many, where std's B-tree searches the larger rather than merging. A B-tree's yield in
/// its order, and the sets its operators build are read from both ends; a hash set's yield
/// in the runtime's, so what each yields is sorted first. The `Nibble` an op yields shows
/// which set's key of two equal ones std's picks.
pub fn set_ops(kind: u8, data: &[u8], out: &mut Vec<u8>) {
	let (mut left, mut right) = data.split_at(data.len() / 2);
	let (evens, odds): (Vec<u8>, Vec<u8>);
	match kind / 5 % 6 {
		0 => {}
		1 => right = left,
		2 => left = &[],
		3 => (left, right) = data.split_at(data.len().min(1)),
		// Each map below keeps a byte's parity, so evens and odds never meet.
		4 => {
			evens = left.iter().map(|b| b & !1).collect();
			odds = right.iter().map(|b| b | 1).collect();
			(left, right) = (&evens, &odds);
		}
		// What an operator builds has its root even with no keys, so its range checks its
		// bounds, and panics.
		_ => {
			let all: BTreeSet<u8> = data.iter().copied().collect();
			let none = &all - &all;
			let k = data.first().copied().unwrap_or(0);
			out.push(none.range((Bound::Excluded(k), Bound::Excluded(k))).count() as u8);
			return;
		}
	}
	match kind % 5 {
		0 => set_ops_of(left, right, |b| b % 32, |v| *v, out),
		1 => set_ops_of(left, right, |b| [b % 4, b / 4 % 4], |k| k[0] << 4 | k[1], out),
		2 => set_ops_of(
			left,
			right,
			|b| (b % 32 != 0).then_some(b % 32),
			|v| v.unwrap_or(0xee),
			out,
		),
		3 => set_ops_of(left, right, Nibble, |n| n.0, out),
		// Where `None` is the greatest key.
		_ => set_ops_of(
			left,
			right,
			|b| Reverse((b % 32 != 0).then_some(b % 32)),
			|v| v.0.unwrap_or(0xee),
			out,
		),
	}
}

fn set_ops_of<T: Ord + Hash + Clone>(
	left: &[u8],
	right: &[u8],
	make: fn(u8) -> T,
	put: fn(&T) -> u8,
	out: &mut Vec<u8>,
) {
	let a: BTreeSet<T> = left.iter().map(|&b| make(b)).collect();
	let b: BTreeSet<T> = right.iter().map(|&b| make(b)).collect();
	for (x, y) in [(&a, &b), (&b, &a)] {
		put_all(x.union(y), put, out);
		put_all(x.intersection(y), put, out);
		put_all(x.difference(y), put, out);
		put_all(x.symmetric_difference(y), put, out);
		put_hints(x.union(y), 1, out);
		put_hints(x.intersection(y), 1, out);
		put_hints(x.difference(y), 1, out);
		put_hints(x.symmetric_difference(y), 1, out);
		// Only the first, where the merge stops.
		out.push(x.union(y).next().map_or(0xdd, put));
		out.push(x.intersection(y).next().map_or(0xdd, put));
		out.push(x.difference(y).next().map_or(0xdd, put));
		out.push(x.symmetric_difference(y).next().map_or(0xdd, put));
		out.extend([x.is_subset(y) as u8, x.is_superset(y) as u8, x.is_disjoint(y) as u8]);
		for built in [x | y, x & y, x - y, x ^ y] {
			put_all(built.iter(), put, out);
			put_all(built.iter().rev(), put, out);
		}
	}
	let a: HashSet<T> = left.iter().map(|&b| make(b)).collect();
	let b: HashSet<T> = right.iter().map(|&b| make(b)).collect();
	for (x, y) in [(&a, &b), (&b, &a)] {
		put_sorted(x.union(y).collect(), put, out);
		put_sorted(x.intersection(y).collect(), put, out);
		put_sorted(x.difference(y).collect(), put, out);
		put_sorted(x.symmetric_difference(y).collect(), put, out);
		// Hints of what is left past an item would hang on the runtime's order.
		put_hints(x.union(y), 0, out);
		put_hints(x.intersection(y), 0, out);
		put_hints(x.difference(y), 0, out);
		put_hints(x.symmetric_difference(y), 0, out);
		// Only the first, which is the runtime's pick, so only where it belongs is written.
		out.push(
			x.union(y)
				.next()
				.map_or(0xdd, |v| u8::from(x.contains(v) || y.contains(v))),
		);
		out.push(
			x.intersection(y)
				.next()
				.map_or(0xdd, |v| u8::from(x.contains(v) && y.contains(v))),
		);
		out.push(x.difference(y).next().map_or(0xdd, |v| u8::from(!y.contains(v))));
		out.push(
			x.symmetric_difference(y)
				.next()
				.map_or(0xdd, |v| u8::from(x.contains(v) != y.contains(v))),
		);
		out.extend([x.is_subset(y) as u8, x.is_superset(y) as u8, x.is_disjoint(y) as u8]);
		for built in [x | y, x & y, x - y, x ^ y] {
			put_sorted(built.iter().collect(), put, out);
		}
	}
}

/// How many `items` there are, then each.
fn put_all<'a, T: 'a>(items: impl Iterator<Item = &'a T>, put: fn(&T) -> u8, out: &mut Vec<u8>) {
	let at = out.len();
	out.push(0);
	for item in items {
		out.push(put(item));
	}
	out[at] = (out.len() - at - 1) as u8;
}

/// The size hint of `items`, and again after each of `read` more.
fn put_hints<'a, T: 'a>(mut items: impl Iterator<Item = &'a T>, read: usize, out: &mut Vec<u8>) {
	for i in 0..=read {
		if i > 0 {
			items.next();
		}
		let (lower, upper) = items.size_hint();
		out.extend([lower as u8, upper.map_or(0xff, |n| n as u8)]);
	}
}

/// [`put_all`] of `items` in order.
fn put_sorted<T: Ord>(mut items: Vec<&T>, put: fn(&T) -> u8, out: &mut Vec<u8>) {
	items.sort();
	put_all(items.into_iter(), put, out);
}

/// An item that equals a `Right` of the same remainder by 4, but whose `ne` sees the
/// remainder by 3, so what `!=` finds tells which std calls. Each call adds to the log,
/// in order.
struct Left<'a>(u16, &'a std::cell::Cell<u16>);

struct Right(u16);

impl Left<'_> {
	fn log(&self, by: u16) {
		self.1.set(self.1.get().wrapping_mul(31).wrapping_add(self.0 ^ by));
	}
}

impl PartialEq<Right> for Left<'_> {
	fn eq(&self, other: &Right) -> bool {
		self.log(0x100);
		self.0 % 4 == other.0 % 4
	}

	fn ne(&self, other: &Right) -> bool {
		self.log(0x200);
		self.0 % 3 != other.0 % 3
	}
}

/// Against a reference, by the remainder by 5, which no reference's impl forwards to.
impl PartialEq<&Right> for Left<'_> {
	fn eq(&self, other: &&Right) -> bool {
		self.log(0x800);
		self.0 % 5 == other.0 % 5
	}
}

/// Against a reference to another, by the remainder by 2.
impl<'a> PartialEq<&Left<'a>> for Left<'_> {
	fn eq(&self, other: &&Left<'a>) -> bool {
		self.log(0x1000);
		self.0 % 2 == other.0 % 2
	}
}

/// An item only a reference to compares, by the crate's impl of two references, which
/// std's of references would otherwise forward from.
#[derive(PartialEq)]
struct Odd(u16);

/// Ordered only through references, by the remainder by 3, though `Odd`s are equal by
/// value.
impl PartialOrd for &Odd {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		(self.0 % 3).partial_cmp(&(other.0 % 3))
	}
}

impl PartialEq<&Right> for &Odd {
	fn eq(&self, other: &&Right) -> bool {
		self.0 % 2 == other.0 % 2
	}
}

/// The other way, by the remainder by 4, with `ne`'s default.
impl<'a> PartialEq<Left<'a>> for Right {
	fn eq(&self, other: &Left<'a>) -> bool {
		other.log(0x400);
		self.0 % 4 == other.0 % 4
	}
}

/// A byte against a `Right`, read from a typed array.
impl PartialEq<Right> for u8 {
	fn eq(&self, other: &Right) -> bool {
		u16::from(*self) % 4 == other.0 % 4
	}
}

/// A pair against a `Right` by its sum, which reads the array through the reference `eq`
/// takes.
impl PartialEq<Right> for [u8; 2] {
	fn eq(&self, other: &Right) -> bool {
		(u16::from(self[0]) + u16::from(self[1])) % 4 == other.0 % 4
	}
}

/// A `Left` against a `Right` by the remainder by 4, as its `eq` finds them.
impl PartialOrd<Right> for Left<'_> {
	fn partial_cmp(&self, other: &Right) -> Option<std::cmp::Ordering> {
		self.log(0x2000);
		(self.0 % 4).partial_cmp(&(other.0 % 4))
	}
}

/// A pair against a `Right` by its sum, as its `eq` finds them.
impl PartialOrd<Right> for [u8; 2] {
	fn partial_cmp(&self, other: &Right) -> Option<std::cmp::Ordering> {
		((u16::from(self[0]) + u16::from(self[1])) % 4).partial_cmp(&(other.0 % 4))
	}
}

/// A `Right` against each of `data`, with the same remainders by 3 and 4 but at the
/// positions `kind` picks, where only `ne`'s or only `eq`'s is, and one fewer for bit 6.
fn rights(kind: u8, data: &[u8]) -> Vec<Right> {
	let (by_ne, by_eq) = (usize::from(kind & 7), usize::from(kind >> 3 & 7));
	let step = |i: usize| match i {
		_ if i == by_eq => 3,
		_ if i == by_ne => 4,
		_ => 12,
	};
	let mut rights: Vec<Right> = data
		.iter()
		.enumerate()
		.map(|(i, &b)| Right(u16::from(b) + step(i)))
		.collect();
	if kind & 0x40 != 0 {
		rights.pop();
	}
	rights
}

/// Whether a comparison found its sides equal, then the log of the calls it made.
fn put_eq(eq: bool, log: &std::cell::Cell<u16>, out: &mut Vec<u8>) {
	out.push(eq as u8);
	out.extend(log.replace(0).to_le_bytes());
}

/// `==` and `!=` of sequences whose items compare by their own `PartialEq<U>`, and of a
/// `VecDeque` against each other sequence: of unequal lengths, empty, with items `eq` and
/// `ne` disagree on at the positions `kind` picks, and a deque wrapped around its buffer.
/// Then `next_if_eq` of such sequences, matching and missing.
pub fn unlike_eqs(kind: u8, data: &[u8], out: &mut Vec<u8>) {
	let log = std::cell::Cell::new(0);
	let lefts = || data.iter().map(|&b| Left(b.into(), &log)).collect::<Vec<_>>();
	// Of the same remainders by 3 and 4 but where only `ne`'s or only `eq`'s is.
	let mut rights = rights(kind, data);
	let by_eq = usize::from(kind >> 3 & 7);
	let at = |i: usize| u16::from(data.get(i).copied().unwrap_or(0));
	let pair = || [Right(at(0) + 12), Right(at(1) + 12)];

	let mut mine = lefts();
	put_eq(mine == rights.as_slice(), &log, out);
	put_eq(mine != rights, &log, out);
	put_eq(mine[..] == rights[..], &log, out);
	put_eq(rights == mine, &log, out);
	put_eq(&mut mine[..] == rights, &log, out);
	put_eq(mine == pair(), &log, out);
	put_eq(mine == &pair(), &log, out);
	put_eq(mine.as_slice() == pair(), &log, out);
	let two = [Left(at(0), &log), Left(at(1), &log)];
	put_eq(two == pair(), &log, out);
	put_eq(two != pair(), &log, out);
	put_eq(two == rights[..], &log, out);
	put_eq(two == rights.as_slice(), &log, out);
	put_eq(Vec::<Left>::new() == Vec::<Right>::new(), &log, out);
	put_eq(Vec::<Left>::new() == rights, &log, out);
	put_eq(mine == [] as [Right; 0], &log, out);
	put_eq(data.to_vec() == rights, &log, out);
	put_eq(data == rights.as_slice(), &log, out);
	let pairs: Vec<[u8; 2]> = data.chunks_exact(2).map(|c| [c[0], c[1]]).collect();
	put_eq(pairs[..] == rights[..pairs.len().min(rights.len())], &log, out);
	put_eq(vec![lefts(), lefts()] == [rights.as_slice(), &rights[..]], &log, out);
	// By the impls against references, and through references of both, by `Right`'s.
	let refs: Vec<&Right> = rights.iter().collect();
	put_eq(mine == refs, &log, out);
	put_eq(mine.iter().collect::<Vec<_>>() == refs, &log, out);
	let others = lefts();
	put_eq(mine == others.iter().collect::<Vec<_>>(), &log, out);
	let odds: Vec<Odd> = data.iter().map(|&b| Odd(b.into())).collect();
	put_eq(odds.iter().collect::<Vec<_>>() == refs, &log, out);

	// Wrapped around: the first half pushed to the front backwards, the rest to the back.
	let half = data.len() / 2;
	let mut q = std::collections::VecDeque::new();
	for &b in data[..half].iter().rev() {
		q.push_front(Left(b.into(), &log));
	}
	for &b in &data[half..] {
		q.push_back(Left(b.into(), &log));
	}
	put_eq(q == rights.as_slice(), &log, out);
	put_eq(q == rights, &log, out);
	put_eq(q != rights.as_slice(), &log, out);
	put_eq(q == &mut rights[..], &log, out);
	put_eq(q == pair(), &log, out);
	put_eq(q == &pair(), &log, out);
	put_eq(q == &mut pair(), &log, out);
	let mut bytes = std::collections::VecDeque::new();
	for &b in data[..half].iter().rev() {
		bytes.push_front(b);
	}
	bytes.extend(&data[half..]);
	let stepped: Vec<u8> = data.iter().enumerate().map(|(i, &b)| b ^ (i == by_eq) as u8).collect();
	put_eq(bytes == stepped, &log, out);
	put_eq(bytes == data, &log, out);
	put_eq(bytes == [data.first().copied().unwrap_or(0), 0], &log, out);
	let wide: std::collections::VecDeque<u16> = bytes.iter().map(|&b| u16::from(b)).collect();
	let mut wrapped = std::collections::VecDeque::new();
	for &b in stepped.iter().rev() {
		wrapped.push_front(u16::from(b));
	}
	put_eq(wide == wrapped, &log, out);
	put_eq(vec![wide.clone()] == [wrapped], &log, out);

	let mut it = vec![lefts(), lefts(), lefts()].into_iter().peekable();
	let exact: Vec<Right> = data.iter().map(|&b| Right(b.into())).collect();
	put_eq(it.next_if_eq(exact.as_slice()).is_some(), &log, out);
	put_eq(it.next_if_eq(&rights.as_slice()).is_some(), &log, out);
	put_eq(it.next_if_eq(&exact[..half]).is_some(), &log, out);
	out.push(it.count() as u8);
	let mut it = vec![bytes.clone(), bytes].into_iter().peekable();
	put_eq(it.next_if_eq(&data.to_vec()).is_some(), &log, out);
	put_eq(it.next_if_eq(&stepped).is_some(), &log, out);
	out.push(it.count() as u8);
}

/// `v` in a deque wrapped around its buffer: the first half pushed to the front backwards,
/// the rest to the back.
fn wrapped<T: Clone>(v: &[T]) -> VecDeque<T> {
	let half = v.len() / 2;
	let mut q = VecDeque::new();
	for item in v[..half].iter().rev() {
		q.push_front(item.clone());
	}
	q.extend(v[half..].iter().cloned());
	q
}

/// `f` of each of `v`.
fn each<T, U>(v: &[T], f: impl Fn(&T) -> U) -> Vec<U> {
	v.iter().map(f).collect()
}

/// An item whose `ne` sees the remainder by 3 where its `eq` sees it by 4, so what `==` and
/// `!=` find tells which std calls. Each call adds to the log, in order.
#[derive(Clone)]
struct Picky<'a>(u16, &'a std::cell::Cell<u16>);

impl Picky<'_> {
	fn log(&self, other: &Self, by: u16) {
		self.1
			.set(self.1.get().wrapping_mul(31).wrapping_add(self.0 ^ other.0 << 4 ^ by));
	}
}

impl PartialEq for Picky<'_> {
	fn eq(&self, other: &Self) -> bool {
		self.log(other, 0x100);
		self.0 % 4 == other.0 % 4
	}

	fn ne(&self, other: &Self) -> bool {
		self.log(other, 0x200);
		self.0 % 3 != other.0 % 3
	}
}

/// Which an `Arc` or an `Rc` of one finds equal to itself before comparing.
impl Eq for Picky<'_> {}

/// `==` and `!=` of what holds a `Picky`, whose `ne` std's of tuples, `Arc`s, `Rc`s, and
/// `Box`es call, and std's `==` of slices and `Cow`s of them calls of each item, but which
/// an `Option`'s, a `Reverse`'s, and a `Wrapping`'s `ne`, as `!eq`, does not: of unequal
/// lengths, empty, nested, and with items `eq` and `ne` disagree on at the positions
/// `kind` picks.
pub fn own_nes(kind: u8, data: &[u8], out: &mut Vec<u8>) {
	use std::rc::Rc;
	use std::sync::Arc;

	let log = std::cell::Cell::new(0);
	let (by_ne, by_eq) = (usize::from(kind & 7), usize::from(kind >> 3 & 7));
	let step = |i: usize| match i {
		_ if i == by_eq => 3,
		_ if i == by_ne => 4,
		_ => 12,
	};
	let a: Vec<Picky> = data.iter().map(|&x| Picky(x.into(), &log)).collect();
	let mut b: Vec<Picky> = data
		.iter()
		.enumerate()
		.map(|(i, &x)| Picky(u16::from(x) + step(i), &log))
		.collect();
	if kind & 0x40 != 0 {
		b.pop();
	}
	let at = |v: &[Picky<'_>], i: usize| v.get(i).map_or(0, |p| p.0);
	let one = |v: &[Picky<'_>], i: usize| Picky(at(v, i), &log);

	put_eq(a == b, &log, out);
	put_eq(a != b, &log, out);
	put_eq(a[..] == b[..], &log, out);
	put_eq(&a[..] != &b[..], &log, out);
	put_eq(Vec::<Picky>::new() == Vec::new(), &log, out);
	put_eq(Vec::<Picky>::new() != b, &log, out);
	let (pa, pb) = ([one(&a, 0), one(&a, 1)], [one(&b, 0), one(&b, 1)]);
	put_eq(pa == pb, &log, out);
	put_eq(pa != pb, &log, out);
	put_eq(pa == b[..], &log, out);
	put_eq(vec![a.clone(), a.clone()] == [a.clone(), b.clone()], &log, out);
	put_eq(vec![a.clone(), b.clone()] != [a.clone(), a.clone()], &log, out);

	// A tuple's `!=` stops at the first field whose `ne` finds it unequal.
	let tuple = |v: &[Picky<'_>]| (one(v, 0), at(v, 2) as u8, one(v, 1));
	put_eq(tuple(&a) != tuple(&b), &log, out);
	put_eq(tuple(&a) == tuple(&b), &log, out);
	put_eq((tuple(&a), 1u8) != (tuple(&b), 1u8), &log, out);
	put_eq(
		each(&a, |p| (p.clone(), p.0 as u8)) == each(&b, |p| (p.clone(), p.0 as u8)),
		&log,
		out,
	);
	put_eq(
		&each(&a, |p| (p.clone(), p.0 as u8))[..] != &each(&b, |p| (p.clone(), p.0 as u8))[..],
		&log,
		out,
	);
	put_eq((Some(one(&a, 0)),) != (Some(one(&b, 0)),), &log, out);
	put_eq((Reverse(one(&a, 0)),) != (Reverse(one(&b, 0)),), &log, out);
	put_eq((Box::new(one(&a, 0)),) != (Box::new(one(&b, 0)),), &log, out);
	put_eq((&one(&a, 0),) != (&one(&b, 0),), &log, out);
	put_eq(
		each(&a, |p| Some(p.clone())) == each(&b, |p| Some(p.clone())),
		&log,
		out,
	);
	put_eq(
		each(&a, |p| Reverse(p.clone())) == each(&b, |p| Reverse(p.clone())),
		&log,
		out,
	);
	put_eq(
		each(&a, |p| Box::new(p.clone())) == each(&b, |p| Box::new(p.clone())),
		&log,
		out,
	);

	// An `Arc` of an `Eq` finds itself equal without comparing; of a tuple with a float,
	// which is no `Eq`, it compares even itself.
	let arc = Arc::new(one(&a, 0));
	put_eq(arc != arc.clone(), &log, out);
	put_eq(arc == arc.clone(), &log, out);
	put_eq(Arc::new(one(&a, 0)) != Arc::new(one(&b, 0)), &log, out);
	let floated = Arc::new((one(&a, 0), 0.5f32));
	put_eq(floated != floated.clone(), &log, out);
	put_eq(floated != Arc::new((one(&b, 0), 0.5)), &log, out);
	put_eq(Rc::new(one(&a, 1)) != Rc::new(one(&b, 1)), &log, out);
	let shared = each(&a, |p| Arc::new(p.clone()));
	put_eq(shared == shared.clone(), &log, out);
	put_eq(shared == each(&b, |p| Arc::new(p.clone())), &log, out);
	put_eq((arc.clone(), 0u8) != (Arc::new(one(&b, 0)), 0u8), &log, out);

	put_eq(wrapped(&a) == wrapped(&b), &log, out);
	put_eq(wrapped(&a) != wrapped(&b), &log, out);
	put_eq(wrapped(&a) == b, &log, out);
	put_eq(wrapped(&a) == &b[..], &log, out);

	// A `Wrapping`'s `ne` is `!eq`, and a `Cow<[T]>`, borrowed or owned, compares as its
	// slice, whose `==` and `!=` call each item's `ne`, against a `Cow`, a slice, a `Vec`,
	// and an array.
	put_eq((Wrapping(one(&a, 0)),) != (Wrapping(one(&b, 0)),), &log, out);
	put_eq(Wrapping(one(&a, 1)) == Wrapping(one(&b, 1)), &log, out);
	let (ca, cb) = (Cow::Borrowed(&a[..]), Cow::<[Picky]>::Owned(b.clone()));
	put_eq(ca == cb, &log, out);
	put_eq(ca != cb, &log, out);
	put_eq(ca == &b[..], &log, out);
	put_eq(cb != a, &log, out);
	put_eq(b == ca, &log, out);
	put_eq(&a[..] != cb, &log, out);
	put_eq(*cb == pa, &log, out);
	let owned: Vec<Cow<'_, [Picky]>> = each(&a, |p| Cow::Owned(vec![p.clone()]));
	let borrowed: Vec<Cow<'_, [Picky]>> = b.chunks(1).map(Cow::Borrowed).collect();
	put_eq(owned != borrowed, &log, out);
}

/// An item whose `ne` and `PartialOrd`'s tests see other remainders than its `eq` and
/// `partial_cmp`, and whose `cmp` yet another, so what each comparison finds tells which
/// std calls. Each call adds to the log, in order.
#[derive(Clone, Copy)]
struct Quirky<'a>(u16, &'a std::cell::Cell<u16>);

impl Quirky<'_> {
	fn log(&self, other: &Self, by: u16) {
		self.1
			.set(self.1.get().wrapping_mul(31).wrapping_add(self.0 ^ other.0 << 4 ^ by));
	}
}

impl PartialEq for Quirky<'_> {
	fn eq(&self, other: &Self) -> bool {
		self.log(other, 0x100);
		self.0 % 4 == other.0 % 4
	}

	fn ne(&self, other: &Self) -> bool {
		self.log(other, 0x200);
		self.0 % 3 != other.0 % 3
	}
}

impl Eq for Quirky<'_> {}

/// Unordered where either is a multiple of 7.
impl PartialOrd for Quirky<'_> {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		self.log(other, 0x300);
		(self.0 % 7 != 0 && other.0 % 7 != 0).then(|| (self.0 % 5).cmp(&(other.0 % 5)))
	}

	fn lt(&self, other: &Self) -> bool {
		self.log(other, 0x400);
		self.0 % 6 < other.0 % 6
	}

	fn le(&self, other: &Self) -> bool {
		self.log(other, 0x500);
		self.0 % 6 <= other.0 % 6
	}

	fn gt(&self, other: &Self) -> bool {
		self.log(other, 0x600);
		self.0 % 6 > other.0 % 6
	}

	fn ge(&self, other: &Self) -> bool {
		self.log(other, 0x700);
		self.0 % 6 >= other.0 % 6
	}
}

impl Ord for Quirky<'_> {
	fn cmp(&self, other: &Self) -> Ordering {
		self.log(other, 0x800);
		(self.0 % 8).cmp(&(other.0 % 8))
	}
}

fn put_order(order: Option<Ordering>, log: &std::cell::Cell<u16>, out: &mut Vec<u8>) {
	out.push(order.map_or(9, |o| o as i8 as u8));
	out.extend(log.replace(0).to_le_bytes());
}

fn put_cmp(order: Ordering, log: &std::cell::Cell<u16>, out: &mut Vec<u8>) {
	out.push(order as i8 as u8);
	out.extend(log.replace(0).to_le_bytes());
}

/// Each of `PartialEq`'s, `PartialOrd`'s, and `Ord`'s comparisons of `$a` and `$b`.
macro_rules! compared {
	($a:expr, $b:expr, $log:expr, $out:expr) => {{
		put_eq($a == $b, $log, $out);
		put_eq($a != $b, $log, $out);
		put_eq($a < $b, $log, $out);
		put_eq($a <= $b, $log, $out);
		put_eq($a > $b, $log, $out);
		put_eq($a >= $b, $log, $out);
		put_order($a.partial_cmp(&$b), $log, $out);
		put_cmp($a.cmp(&$b), $log, $out);
	}};
}

/// Comparisons of `Cell`s and `RefCell`s, which compare what they hold by its `eq`, its
/// tests, its `partial_cmp`, and its `cmp`, and whose `ne` is `!eq`, and `==` of one-time
/// cells: of a `Quirky`, numbers, and `Option`s, nested, held by what compares by `ne`, as
/// B-tree keys, and a `RefCell` against itself. Then, as `kind` picks, a `RefCell` compared
/// while mutably borrowed, which panics, or held by an `Arc` that finds itself equal first.
pub fn cell_eqs(kind: u8, data: &[u8], out: &mut Vec<u8>) {
	use std::cell::{Cell, OnceCell, RefCell};
	use std::sync::{Arc, OnceLock};

	let log = Cell::new(0);
	let at = |i: usize| u16::from(data.get(i).copied().unwrap_or(0));
	let (x, y) = (Quirky(at(0), &log), Quirky(at(1), &log));
	let maybe = |i: usize| (at(i) % 3 != 0).then_some(at(i) as u8);

	let (ca, cb) = (Cell::new(x), Cell::new(y));
	compared!(ca, cb, &log, out);
	let (ra, rb) = (RefCell::new(x), RefCell::new(y));
	compared!(ra, rb, &log, out);
	compared!(ra, ra, &log, out);
	let (na, nb) = (Cell::new(at(0) as u8), Cell::new(at(1) as u8));
	compared!(na, nb, &log, out);
	let (oa, ob) = (Cell::new(maybe(0)), Cell::new(maybe(1)));
	compared!(oa, ob, &log, out);
	let (sa, sb) = (RefCell::new(at(0).to_string()), RefCell::new(at(1).to_string()));
	compared!(sa, sb, &log, out);

	// Nested, and against itself while a shared borrow is held.
	let (rra, rrb) = (RefCell::new(RefCell::new(x)), RefCell::new(RefCell::new(y)));
	{
		let held = rra.borrow();
		let _inner = held.borrow();
		compared!(rra, rrb, &log, out);
		compared!(rra, rra, &log, out);
	}
	let (rca, rcb) = (RefCell::new(Cell::new(x)), RefCell::new(Cell::new(y)));
	compared!(rca, rcb, &log, out);
	compared!(Some(Cell::new(x)), Some(Cell::new(y)), &log, out);
	compared!((RefCell::new(x), x), (RefCell::new(y), y), &log, out);

	// A slice compares its items by `!=`, which a cell's is `!eq` of, but which a slice's
	// within a cell is still the items' own `ne`.
	let (va, vb) = (vec![x, y, x], vec![y, y, x]);
	put_eq(RefCell::new(va.clone()) == RefCell::new(vb.clone()), &log, out);
	put_eq(Cell::new(&va[..]) != Cell::new(&vb[..]), &log, out);
	put_eq(each(&va, |&p| Cell::new(p)) == each(&vb, |&p| Cell::new(p)), &log, out);
	put_eq(
		each(&va, |&p| RefCell::new(p)) != each(&vb, |&p| RefCell::new(p)),
		&log,
		out,
	);
	put_eq(
		(Cell::new(x), RefCell::new(y)) != (Cell::new(y), RefCell::new(x)),
		&log,
		out,
	);
	put_eq(Arc::new(RefCell::new(x)) != Arc::new(RefCell::new(y)), &log, out);

	// `Ord`'s defaults, by the cells' `lt` and `le`.
	put_eq(Cell::new(x).max(Cell::new(y)).get().0 == x.0, &log, out);
	put_eq(RefCell::new(x).min(RefCell::new(y)).borrow().0 == x.0, &log, out);
	// Within bounds that `le` finds in order, which `clamp` asserts.
	let (lo, hi) = if x.0 % 6 <= y.0 % 6 { (x, y) } else { (y, x) };
	let clamped = Cell::new(Quirky(at(2), &log)).clamp(Cell::new(lo), Cell::new(hi));
	out.push(clamped.get().0 as u8);
	out.extend(log.replace(0).to_le_bytes());

	// One-time cells, empty or set, by their `Option`'s `eq`, whose `ne` is `!eq`.
	let once = |i: usize, v| {
		let cell = OnceCell::new();
		if at(i) % 4 != 0 {
			_ = cell.set(v);
		}
		cell
	};
	put_eq(once(2, x) == once(3, y), &log, out);
	put_eq(once(2, x) != once(3, y), &log, out);
	put_eq(OnceCell::from(maybe(0)) == OnceCell::from(maybe(1)), &log, out);
	let lock = |i: usize, v| {
		let lock = OnceLock::new();
		if at(i) % 4 != 1 {
			_ = lock.set(v);
		}
		lock
	};
	put_eq(lock(2, x) != lock(3, y), &log, out);
	put_eq(
		OnceCell::from(RefCell::new(x)) != OnceCell::from(RefCell::new(y)),
		&log,
		out,
	);

	// As keys, ordered by what they hold.
	let set: BTreeSet<Cell<u8>> = data.iter().map(|&v| Cell::new(v % 8)).collect();
	out.push(set.len() as u8);
	out.push(set.contains(&Cell::new(at(0) as u8 % 8)) as u8);
	out.extend(set.iter().map(|c| c.get()));
	let map: BTreeMap<RefCell<u8>, usize> = data
		.iter()
		.enumerate()
		.map(|(i, &v)| (RefCell::new(v % 8), i))
		.collect();
	out.push(map.get(&RefCell::new(at(1) as u8 % 8)).map_or(0xff, |&i| i as u8));

	// What each compared left borrowed is released.
	*ra.borrow_mut() = y;
	*rra.borrow_mut().borrow_mut() = x;
	compared!(ra, *rra.borrow(), &log, out);

	let shared = Arc::new(RefCell::new(x));
	let _mutable = match kind % 6 {
		0 => return,
		1 => ra.borrow_mut(),
		2 => rb.borrow_mut(),
		_ => shared.borrow_mut(),
	};
	match kind % 6 {
		1 => put_eq(ra == rb, &log, out),
		2 => put_eq(ra < rb, &log, out),
		3 => put_eq(shared == shared.clone(), &log, out),
		4 => put_eq(shared != Arc::new(RefCell::new(y)), &log, out),
		_ => put_order(shared.partial_cmp(&shared.clone()), &log, out),
	}
	// Nested, with only the inner mutably borrowed.
	let held = rra.borrow();
	let _inner = held.borrow_mut();
	put_eq(rra == rrb, &log, out);
}

/// std's comparisons of iterators of items that compare by their own `PartialEq<U>` and
/// `PartialOrd<U>`: `eq` by `eq`, never `ne`, and `ne` as `!eq`, through references by
/// what they point at, against a reference by the impl against it, and of byte arrays,
/// both by value and through references.
pub fn unlike_compares(kind: u8, data: &[u8], out: &mut Vec<u8>) {
	let log = std::cell::Cell::new(0);
	let lefts = || data.iter().map(|&b| Left(b.into(), &log)).collect::<Vec<_>>();
	let rights = || rights(kind, data);
	let order = |o: Option<std::cmp::Ordering>, log: &std::cell::Cell<u16>, out: &mut Vec<u8>| {
		out.push(o.map_or(9, |o| o as u8));
		out.extend(log.replace(0).to_le_bytes());
	};
	let (mine, theirs) = (lefts(), rights());
	let refs: Vec<&Right> = theirs.iter().collect();
	put_eq(mine.iter().eq(&theirs), &log, out);
	put_eq(mine.iter().ne(theirs.iter()), &log, out);
	put_eq(lefts().into_iter().eq(rights()), &log, out);
	put_eq(lefts().into_iter().eq(refs.iter().copied()), &log, out);
	put_eq(lefts().into_iter().ne(mine.iter()), &log, out);
	let odds: Vec<Odd> = data.iter().map(|&b| Odd(b.into())).collect();
	put_eq(odds.iter().eq(refs.iter().copied()), &log, out);
	order(odds.iter().partial_cmp(odds.iter().rev()), &log, out);
	put_eq(data.iter().copied().eq(rights()), &log, out);
	let pairs: Vec<[u8; 2]> = data.chunks_exact(2).map(|c| [c[0], c[1]]).collect();
	put_eq(pairs.iter().copied().eq(rights()), &log, out);
	put_eq(pairs.iter().eq(&theirs), &log, out);
	order(mine.iter().partial_cmp(&theirs), &log, out);
	order(lefts().into_iter().partial_cmp(rights()), &log, out);
	put_eq(lefts().into_iter().lt(rights()), &log, out);
	put_eq(mine.iter().ge(theirs.iter()), &log, out);
	order(pairs.iter().copied().partial_cmp(rights()), &log, out);
	order(pairs.iter().partial_cmp(theirs.iter()), &log, out);
}

/// Compared, ordered, and hashed only through references: equal and hashed by the
/// remainder by 2, ordered by `PartialOrd` by the remainder by 3, and by `Ord` by the
/// remainder by 4, descending, so each composite shows which std calls.
struct ByRef(u16);

impl PartialEq for &ByRef {
	fn eq(&self, other: &Self) -> bool {
		self.0 % 2 == other.0 % 2
	}
}

impl Eq for &ByRef {}

impl PartialOrd for &ByRef {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		(self.0 % 3).partial_cmp(&(other.0 % 3))
	}
}

impl Ord for &ByRef {
	fn cmp(&self, other: &Self) -> Ordering {
		(other.0 % 4).cmp(&(self.0 % 4))
	}
}

impl Hash for &ByRef {
	fn hash<H: Hasher>(&self, state: &mut H) {
		(self.0 % 2).hash(state);
	}
}

/// `ByRef`'s impls on the value itself, which std's of references forward to.
struct ByVal(u16);

impl PartialEq for ByVal {
	fn eq(&self, other: &Self) -> bool {
		self.0 % 2 == other.0 % 2
	}
}

impl Eq for ByVal {}

impl PartialOrd for ByVal {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		(self.0 % 3).partial_cmp(&(other.0 % 3))
	}
}

/// With a `max` and `min` of its own, which std's `Ord` of a reference never calls.
impl Ord for ByVal {
	fn cmp(&self, other: &Self) -> Ordering {
		(other.0 % 4).cmp(&(self.0 % 4))
	}

	fn max(self, _: Self) -> Self {
		self
	}

	fn min(self, other: Self) -> Self {
		other
	}
}

/// With a `hash_slice` of its own, which std's `Hash` of a reference never calls.
impl Hash for ByVal {
	fn hash<H: Hasher>(&self, state: &mut H) {
		(self.0 % 2).hash(state);
	}

	fn hash_slice<H: Hasher>(data: &[Self], state: &mut H) {
		state.write_usize(data.len() * 7);
	}
}

/// Equal and ordered by `PartialOrd` by value, by the remainder by 5, which std's of a
/// reference forwards to, but ordered by `Ord` and hashed by a reference's own impls, by
/// the remainders by 3 and 5.
struct Mixed(u16);

impl PartialEq for Mixed {
	fn eq(&self, other: &Self) -> bool {
		self.0 % 5 == other.0 % 5
	}
}

impl PartialOrd for Mixed {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		(self.0 % 5).partial_cmp(&(other.0 % 5))
	}
}

impl Eq for &Mixed {}

impl Ord for &Mixed {
	fn cmp(&self, other: &Self) -> Ordering {
		(self.0 % 3).cmp(&(other.0 % 3))
	}
}

impl Hash for &Mixed {
	fn hash<H: Hasher>(&self, state: &mut H) {
		(self.0 % 5).hash(state);
	}
}

/// Equal by the remainder by 2 and ordered by the remainder by 3 only as a `Box`.
struct Boxed(u16);

impl PartialEq for Box<Boxed> {
	fn eq(&self, other: &Self) -> bool {
		self.0 % 2 == other.0 % 2
	}
}

impl PartialOrd for Box<Boxed> {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		(self.0 % 3).partial_cmp(&(other.0 % 3))
	}
}

/// std's comparisons, orderings, and hashes of composites holding a `T`, by each one's own
/// impls: `&ByRef`'s, `ByVal`'s through `&ByVal`, `Mixed`'s and `&Mixed`'s, and each
/// through `&&T`. Then `Box<Boxed>`'s.
pub fn ref_orders(_kind: u8, data: &[u8], out: &mut Vec<u8>) {
	use std::cell::{Cell, RefCell};

	fn hashed(value: impl Hash) -> [u8; 8] {
		let mut h = DefaultHasher::new();
		value.hash(&mut h);
		h.finish().to_le_bytes()
	}
	fn order_byte(o: Option<Ordering>) -> u8 {
		o.map_or(9, |o| o as i8 as u8)
	}
	fn cmp_byte(o: Ordering) -> u8 {
		o as i8 as u8
	}
	macro_rules! composites {
		($a:expr, $b:expr, $out:expr) => {{
			let (a, b, out): (_, _, &mut Vec<u8>) = ($a, $b, $out);
			out.extend([(a,) == (b,), (a,) != (b,), (a,) < (b,), (7u8, a) >= (7, b)].map(u8::from));
			out.push(order_byte((a,).partial_cmp(&(b,))));
			out.extend([(a,).cmp(&(b,)), (7u8, a).cmp(&(7, b))].map(cmp_byte));
			out.extend([
				order_byte(Some(a).partial_cmp(&Some(b))),
				cmp_byte(Some(a).cmp(&Some(b))),
			]);
			out.extend([Some(a) == Some(b), Some(a) < Some(b)].map(u8::from));
			out.push(u8::from(Ok::<_, u8>(a) == Ok(b)));
			out.push(order_byte([a, b][..].partial_cmp(&[b, a][..])));
			out.extend([[a, b].cmp(&[b, a]), vec![a].cmp(&vec![b])].map(cmp_byte));
			out.extend([[a, b] == [b, a], [a][..] < [b][..], vec![a, b] != vec![b, a]].map(u8::from));
			out.push(u8::from(VecDeque::from([a, b]) == VecDeque::from([b, a])));
			out.extend([
				order_byte(Reverse(a).partial_cmp(&Reverse(b))),
				cmp_byte(Reverse(a).cmp(&Reverse(b))),
			]);
			out.extend([
				order_byte(Cell::new(a).partial_cmp(&Cell::new(b))),
				cmp_byte(RefCell::new(a).cmp(&RefCell::new(b))),
			]);
			out.extend([Cell::new(a) == Cell::new(b), RefCell::new(a) < RefCell::new(b)].map(u8::from));
			out.extend([
				order_byte([a, b].iter().partial_cmp([b, a].iter())),
				cmp_byte([a].into_iter().cmp([b])),
			]);
			out.extend(
				[
					[a, b].into_iter().eq([b, a]),
					[a].into_iter().lt([b]),
					[a].iter().ge([b].iter()),
				]
				.map(u8::from),
			);
			out.extend([a == b, a != b, a < b, a >= b, &a == &b, &a > &b].map(u8::from));
			out.extend([a.partial_cmp(&b), (&a).partial_cmp(&&b)].map(order_byte));
			out.extend([a.cmp(&b), (&a).cmp(&&b)].map(cmp_byte));
			out.extend([a.max(b).0, a.min(b).0, (&a).max(&b).0, a.clamp(b, b).0].map(|v| v as u8));
			out.extend(hashed(a));
			out.extend(hashed(&a));
			for hash in [
				hashed((a,)),
				hashed(Some(a)),
				hashed([a, b]),
				hashed(&[a][..]),
				hashed(vec![b]),
			] {
				out.extend(hash);
			}
		}};
	}
	macro_rules! keys {
		($xs:expr, $out:expr) => {{
			let (xs, out): (Vec<_>, &mut Vec<u8>) = ($xs, $out);
			let mut sorted = xs.clone();
			sorted.sort();
			out.extend(sorted.iter().map(|x| x.0 as u8));
			// By `insert`, which compares only by `Ord`, as `collect` does not.
			let mut set = BTreeSet::new();
			let mut map = BTreeMap::new();
			for (i, &x) in xs.iter().enumerate() {
				out.push(u8::from(set.insert(x)));
				out.extend(map.insert((x,), i).map(|i| i as u8));
			}
			out.extend(set.iter().map(|x| x.0 as u8));
			out.extend(map.iter().map(|(k, &i)| [k.0.0 as u8, i as u8]).flatten());
			out.extend(set.get(&xs[0]).map(|x| x.0 as u8));
			let hashes: HashSet<_> = xs.iter().copied().collect();
			out.push(hashes.len() as u8);
			let tuples: HashSet<_> = xs.iter().map(|&x| (x, Some(x))).collect();
			out.push(tuples.len() as u8);
			out.extend(xs.iter().max().map(|x| x.0 as u8));
			out.extend(xs.iter().copied().min().map(|x| x.0 as u8));
			out.extend(xs.first().map(|first| u8::from(xs[1..].contains(first))));
			out.push(u8::from(xs.is_sorted()));
			out.push(u8::from(VecDeque::from(xs.clone()).contains(&xs[xs.len() / 2])));
		}};
	}
	let data = if data.is_empty() { &[0][..] } else { data };
	let by_ref: Vec<_> = data.iter().map(|&v| ByRef(v.into())).collect();
	let by_val: Vec<_> = data.iter().map(|&v| ByVal(v.into())).collect();
	let mixed: Vec<_> = data.iter().map(|&v| Mixed(v.into())).collect();
	for (i, j) in (0..data.len()).zip((0..data.len()).rev()) {
		composites!(&by_ref[i], &by_ref[j], out);
		composites!(&&by_ref[i], &&by_ref[j], out);
		composites!(&by_val[i], &by_val[j], out);
		composites!(&&by_val[i], &&by_val[j], out);
		let (a, b) = (&mixed[i], &mixed[j]);
		out.extend([(a,) < (b,), Some(a) <= Some(b), [a] == [b], vec![a] < vec![b]].map(u8::from));
		let orders = [
			(a,).cmp(&(b,)),
			Some(a).cmp(&Some(b)),
			[a].cmp(&[b]),
			Reverse(a).cmp(&Reverse(b)),
		];
		out.extend(orders.map(cmp_byte));
		out.extend([(&a,).cmp(&(&b,)), [a].into_iter().cmp([b]), [a].iter().cmp([b].iter())].map(cmp_byte));
		out.extend(hashed((a,)));
		out.extend(hashed([&a]));
		let (a, b) = (Box::new(Boxed(data[i].into())), Box::new(Boxed(data[j].into())));
		out.extend([(&a,) == (&b,), (&a,) < (&b,), Some(&a) >= Some(&b), [&a] != [&b]].map(u8::from));
		out.extend(
			[
				[&a].into_iter().eq([&b]),
				[&a].iter().ne([&b].iter()),
				[&a].into_iter().lt([&b]),
			]
			.map(u8::from),
		);
		out.push(order_byte([&a].iter().partial_cmp([&b].iter())));
	}
	keys!(by_ref.iter().collect(), out);
	keys!(by_val.iter().collect(), out);
	keys!(mixed.iter().collect(), out);
}

/// `extract_if` of hash maps and sets: what its closure takes out, by keys JS compares as
/// they are and keys found by their `Hash`, all colliding, with values changed in place as
/// each is read, and what stays, each sorted. Read part way, it takes out only what it
/// read, and the rest stays.
pub fn extracts(kind: u8, data: &[u8], out: &mut Vec<u8>) {
	let keep = data.first().map_or(2, |b| b % 3 + 2);
	match kind % 3 {
		0 => {
			let mut map: HashMap<u8, u16> = data.iter().map(|&b| (b % 16, u16::from(b))).collect();
			out.push(map.extract_if(|_, _| true).size_hint().1.map_or(0xfe, |n| n as u8));
			let taken: BTreeMap<u8, u16> = map
				.extract_if(|k, v| {
					*v += 1;
					k % keep == 0
				})
				.collect();
			let kept: BTreeMap<u8, u16> = map.iter().map(|(k, v)| (*k, *v)).collect();
			for (k, v) in taken.iter().chain([(&0xff, &0xffff)]).chain(&kept) {
				out.push(*k);
				out.extend(v.to_be_bytes());
			}
			let mut it = map.extract_if(|k, _| k % 2 == 0);
			let some = it.next().is_some();
			drop(it);
			out.extend([some as u8, map.len() as u8]);
			let mut words: HashMap<u8, Vec<u8>> = data.iter().map(|&b| (b % 8, vec![b])).collect();
			let taken: BTreeMap<u8, Vec<u8>> = words
				.extract_if(|k, v| {
					v.push(*k);
					v[0] % keep == 0
				})
				.collect();
			for (k, v) in &taken {
				out.push(*k);
				out.extend(v);
			}
			out.push(words.values().map(Vec::len).sum::<usize>() as u8);
		}
		1 => {
			let mut set: HashSet<Pair> = data
				.iter()
				.map(|&b| Pair {
					a: b % 8,
					b: u16::from(b % 3),
				})
				.collect();
			let taken: BTreeSet<Pair> = set.extract_if(|p| u16::from(p.a) % u16::from(keep) == p.b).collect();
			let kept: BTreeSet<Pair> = set.iter().copied().collect();
			for p in taken.iter().chain([&Pair { a: 0xff, b: 0 }]).chain(&kept) {
				out.extend([p.a, p.b as u8]);
			}
			let mut bytes: HashSet<u8> = data.iter().copied().collect();
			out.push(bytes.extract_if(|b| b % keep == 1).count() as u8);
			out.push(bytes.len() as u8);
			out.push(bytes.extract_if(|_| false).next().is_none() as u8);
		}
		_ => {
			// Keys that all collide, chained in one slot, and taken from any place in it.
			let mut map: HashMap<Clash, Tally> = data
				.iter()
				.map(|&b| (Clash(b % 8), Tally { n: u16::from(b) }))
				.collect();
			let taken: BTreeMap<u8, u16> = map
				.extract_if(|k, t| {
					t.n += u16::from(k.0);
					t.n % u16::from(keep) == 0
				})
				.map(|(k, t)| (k.0, t.n))
				.collect();
			let kept: BTreeMap<u8, u16> = map.iter().map(|(k, t)| (k.0, t.n)).collect();
			for (k, v) in taken.iter().chain([(&0xff, &0xffff)]).chain(&kept) {
				out.push(*k);
				out.extend(v.to_be_bytes());
			}
			for k in 0..8 {
				out.push(map.get(&Clash(k)).map_or(0xee, |t| t.n as u8));
			}
			let mut set: HashSet<Clash> = data.iter().map(|&b| Clash(b % 6)).collect();
			out.push(set.extract_if(|c| c.0 % keep == 0).count() as u8);
			let mut kept: Vec<u8> = set.iter().map(|c| c.0).collect();
			kept.sort();
			out.extend(kept);
			out.push(set.contains(&Clash(0)) as u8);
		}
	}
}

/// A B-tree key ordered by its number's low byte, whose `PartialOrd`'s `le` and `lt`, which
/// std checks a range's end by, see only that byte halved. Each of those and its drop add its
/// whole number to the log.
struct Logged<'a>(u16, &'a std::cell::RefCell<Vec<u16>>);

impl Drop for Logged<'_> {
	fn drop(&mut self) {
		self.1.borrow_mut().push(self.0);
	}
}

impl PartialEq for Logged<'_> {
	fn eq(&self, other: &Self) -> bool {
		self.0 & 0xff == other.0 & 0xff
	}
}

impl Eq for Logged<'_> {}

impl PartialOrd for Logged<'_> {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}

	fn le(&self, other: &Self) -> bool {
		self.1.borrow_mut().push(0x4000 | self.0);
		(self.0 & 0xff) / 2 <= (other.0 & 0xff) / 2
	}

	fn lt(&self, other: &Self) -> bool {
		self.1.borrow_mut().push(0x2000 | self.0);
		(self.0 & 0xff) / 2 < (other.0 & 0xff) / 2
	}
}

impl Ord for Logged<'_> {
	fn cmp(&self, other: &Self) -> Ordering {
		(self.0 & 0xff).cmp(&(other.0 & 0xff))
	}
}

/// A range of its own, which logs each read of either bound.
struct Window<'a> {
	lo: u16,
	hi: u16,
	log: &'a std::cell::RefCell<Vec<u16>>,
}

impl std::ops::RangeBounds<u16> for Window<'_> {
	fn start_bound(&self) -> Bound<&u16> {
		self.log.borrow_mut().push(0xa000);
		Bound::Included(&self.lo)
	}

	fn end_bound(&self) -> Bound<&u16> {
		self.log.borrow_mut().push(0xb000);
		Bound::Excluded(&self.hi)
	}
}

/// The range form `form` picks of `lo` and `hi`, which may be backwards or empty and
/// excluded at both ends, neither of which `extract_if` panics on.
fn form(form: u8, lo: u16, hi: u16) -> (Bound<u16>, Bound<u16>) {
	use Bound::*;
	match form % 8 {
		0 => (Unbounded, Unbounded),
		1 => (Included(lo), Excluded(hi)),
		2 => (Included(lo), Included(hi)),
		3 => (Included(lo), Unbounded),
		4 => (Unbounded, Excluded(hi)),
		5 => (Unbounded, Included(hi)),
		6 => (Excluded(lo), Excluded(hi)),
		_ => (Excluded(lo), Included(lo)),
	}
}

/// `extract_if` of B-tree maps and sets, large enough to merge and rebalance leaves as it
/// takes entries out: each form of range, with values changed in place as each is read,
/// read in full or part way, with what stays, the hint, and `Debug` of the iterator as it
/// reads, which peeks past the range's end until it stops there. Then keys and values that
/// log their drops and the `PartialOrd` tests std ends the range by, a closure and range
/// that drop with the iterator, and a range of its own, whose start std reads only of a
/// map with a root.
pub fn btree_extracts(kind: u8, data: &[u8], out: &mut Vec<u8>) {
	let at = |i: usize| data.get(i).copied().unwrap_or(0);
	let n = u16::from(at(0)) * 2;
	let (lo, hi) = (u16::from(at(1)) * 2, u16::from(at(2)) * 2);
	let (every, limit) = (u16::from(at(3) % 5 + 1), at(4));
	let range = form(at(5), lo, hi);
	match kind % 3 {
		0 => {
			let mut map: BTreeMap<u16, u16> = (0..n).map(|k| (k, k ^ 0x100)).collect();
			let mut it = map.extract_if(range, |k, v| {
				*v += 1;
				(*k ^ u16::from(limit)) % every == 0
			});
			crate::harness::put(out, &format!("{it:?} {:?}", it.size_hint()));
			let taken: Vec<(u16, u16)> = it.by_ref().take(usize::from(limit)).collect();
			crate::harness::put(out, &format!("{taken:?} {it:?} {:?}", it.size_hint()));
			if limit % 2 == 0 {
				crate::harness::put(out, &format!("{:?} {it:#?}", it.next()));
			}
			drop(it);
			crate::harness::put(
				out,
				&format!("{} {:?}", map.len(), map.iter().step_by(7).collect::<Vec<_>>()),
			);
			let taken: Vec<(u16, u16)> = map.extract_if(.., |_, v| *v % 3 == 0).collect();
			crate::harness::put(out, &format!("{taken:?} {map:?}"));
		}
		1 => {
			let mut set: BTreeSet<u16> = (0..n).map(|k| k * 3 % (n + 1)).collect();
			let mut it = set.extract_if(range, |k| k % every != 0);
			crate::harness::put(out, &format!("{it:?}"));
			let taken: Vec<u16> = it.by_ref().take(usize::from(limit)).collect();
			crate::harness::put(out, &format!("{taken:?} {it:?} {:?}", it.size_hint()));
			drop(it);
			crate::harness::put(out, &format!("{set:?}"));
			// Neither a set never added to nor one cleared has a root.
			let mut empty: BTreeSet<u16> = BTreeSet::new();
			crate::harness::put(out, &format!("{:?}", empty.extract_if(range, |_| true)));
			set.clear();
			crate::harness::put(out, &format!("{:?}", set.extract_if(.., |_| true)));
		}
		_ => {
			let log = std::cell::RefCell::new(Vec::new());
			let mut map: BTreeMap<Logged, Logged> = (0..n / 2)
				.map(|k| (Logged(k, &log), Logged(0x1000 | k, &log)))
				.collect();
			let guard = Logged(0x800, &log);
			let (start, end) = (Logged(0x900 | lo / 2, &log), Logged(0x900 | hi / 2, &log));
			let mut it = map.extract_if((Bound::Excluded(start), Bound::Included(end)), move |k, v| {
				let _ = &guard;
				v.0 ^= 0x200;
				k.0 % every == 0
			});
			let taken: Vec<(Logged, Logged)> = it.by_ref().take(usize::from(limit % 8)).collect();
			drop(it);
			out.extend(taken.iter().flat_map(|(k, v)| [k.0 as u8, v.0 as u8]));
			drop(taken);
			// Kept, each key in range is tested against the end, and the first past it.
			let mut it = map.extract_if(Logged(0xa00 | lo / 4, &log)..Logged(0xa00 | hi / 4, &log), |_, _| false);
			out.push(it.next().is_none() as u8);
			drop(it);
			drop(map);
			// A range of its own reads its start once, of a map with a root, and its end at each key.
			let mut bytes: BTreeMap<u16, u8> = BTreeMap::new();
			let window = |lo, hi| Window { lo, hi, log: &log };
			bytes.extract_if(window(0, 9), |_, _| true).for_each(drop);
			bytes.insert(1, 1);
			bytes.remove(&1);
			bytes.extract_if(window(0, 9), |_, _| true).for_each(drop);
			bytes.extend((0..n).map(|k| (k, k as u8)));
			let taken: Vec<(u16, u8)> = bytes
				.extract_if(window(lo / 2, hi / 2), |k, _| k % every == 0)
				.collect();
			out.extend(taken.iter().map(|(_, v)| *v));
			out.extend(log.take().iter().flat_map(|e| e.to_be_bytes()));
		}
	}
}

/// A B-tree key that logs each comparison std makes of it, in order: `cmp` and `eq` by its
/// low 10 bits, `le` and `gt` alike, but `lt`, which a set's `collect` sorts by, by those
/// bits with the low 2 flipped, and `ne`, which `collect` drops all but the last of a run of
/// equal keys by, by all 16. A collected tree is then out of `cmp`'s order, or holds keys
/// `cmp` finds equal.
#[derive(Clone, Copy)]
struct Probe<'a>(u16, &'a std::cell::Cell<u32>);

impl Probe<'_> {
	fn log(&self, other: &Self, by: u32) {
		let call = u32::from(self.0) << 16 ^ u32::from(other.0) ^ by;
		self.1.set(self.1.get().wrapping_mul(0x01000193).wrapping_add(call));
	}

	fn low(&self) -> u16 {
		self.0 & 0x3ff
	}
}

impl PartialEq for Probe<'_> {
	fn eq(&self, other: &Self) -> bool {
		self.log(other, 0x100);
		self.low() == other.low()
	}

	fn ne(&self, other: &Self) -> bool {
		self.log(other, 0x200);
		self.0 != other.0
	}
}

impl Eq for Probe<'_> {}

impl PartialOrd for Probe<'_> {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		self.log(other, 0x300);
		Some(self.low().cmp(&other.low()))
	}

	fn lt(&self, other: &Self) -> bool {
		self.log(other, 0x400);
		self.low() ^ 3 < other.low() ^ 3
	}

	fn le(&self, other: &Self) -> bool {
		self.log(other, 0x500);
		self.low() <= other.low()
	}

	fn gt(&self, other: &Self) -> bool {
		self.log(other, 0x600);
		self.low() > other.low()
	}
}

impl Ord for Probe<'_> {
	fn cmp(&self, other: &Self) -> Ordering {
		self.log(other, 0x800);
		self.low().cmp(&other.low())
	}
}

/// std's B-tree compares keys only as it searches its nodes, each from its first key on,
/// which `Probe` logs: of maps and sets grown by inserts that split nodes at every height,
/// then searched, removed from until nodes merge and take from their siblings, extended,
/// entered, ranged, popped, split, cloned, extracted from, and retained, and of the set
/// operations that search, of sets of sizes that make std look keys up rather than merge.
/// Then of a set and map `collect` leaves out of `cmp`'s order, which std searches as is.
/// Each answer is followed by the log.
pub fn btree_searches(kind: u8, data: &[u8], out: &mut Vec<u8>) {
	let log = std::cell::Cell::new(0u32);
	let at = |i: usize| data.get(i).copied().unwrap_or(0);
	let put = |out: &mut Vec<u8>, answer: u16| {
		out.extend(answer.to_le_bytes());
		out.extend(log.replace(0).to_le_bytes());
	};
	// Distinct keys spread over 0..1024 by an odd stride, so each lands anywhere in the tree.
	let n = usize::from(at(0)) + 1;
	let stride = usize::from(at(1)) * 2 + 1;
	let key = |i: usize| Probe((i * stride + usize::from(at(2))) as u16 & 0x3ff, &log);
	let probe = |b: u8| Probe(u16::from(b) * 4 + u16::from(b >> 6), &log);
	let some = |v: Option<&u16>| v.map_or(0xffff, |v| *v);
	match kind % 4 {
		0 => {
			let mut map: BTreeMap<Probe, u16> = BTreeMap::new();
			for i in 0..n {
				let old = map.insert(key(i), i as u16);
				put(out, old.unwrap_or(0xffff));
			}
			for &b in &data[data.len().min(3)..] {
				put(out, some(map.get(&probe(b))));
				put(out, map.contains_key(&key(usize::from(b))) as u16);
				put(out, map.remove(&key(usize::from(b) * 3)).unwrap_or(0xffff));
				*map.entry(probe(b)).or_insert(7) += 1;
				put(out, map.len() as u16);
			}
			map.extend((0..n / 2).map(|i| (key(i * 5), i as u16)));
			put(out, map.len() as u16);
			let (lo, hi) = (probe(at(3)), probe(at(4)));
			let (lo, hi) = if lo.low() <= hi.low() { (lo, hi) } else { (hi, lo) };
			let mut range = match at(5) % 3 {
				0 => map.range(lo..hi),
				1 => map.range(lo..=hi),
				_ => map.range((Bound::Excluded(lo), Bound::Unbounded)),
			};
			while let Some((k, _)) = if at(6) % 2 == 0 {
				range.next()
			} else {
				range.next_back()
			} {
				put(out, k.0);
			}
			put(out, map.pop_first().map_or(0xffff, |(k, _)| k.0));
			put(out, map.pop_last().map_or(0xffff, |(k, _)| k.0));
			let copy = map.clone();
			put(out, some(copy.get(&probe(at(7)))));
			let mut right = map.split_off(&probe(at(8)));
			put(out, (map.len() * 1000 + right.len()) as u16);
			put(out, some(right.get(&probe(at(9)))));
			let taken = right
				.extract_if(probe(at(10))..probe(at(11)), |k, _| k.0 % 3 == 0)
				.count();
			put(out, taken as u16);
			map.retain(|k, _| k.0 % 4 != 0);
			put(out, map.len() as u16);
			for i in (0..n).step_by(3) {
				put(out, map.remove(&key(i)).unwrap_or(0xffff));
			}
			put(out, map.len() as u16);
		}
		1 => {
			// One set at least 16 times the other's size, which std looks each key of up.
			let large: BTreeSet<Probe> = (0..n).map(key).collect();
			let mut small: BTreeSet<Probe> = BTreeSet::new();
			for &b in data {
				put(out, small.insert(probe(b)) as u16);
			}
			for (x, y) in [(&small, &large), (&large, &small)] {
				put(out, x.intersection(y).count() as u16);
				put(out, x.difference(y).count() as u16);
				put(out, x.union(y).count() as u16);
				put(out, x.symmetric_difference(y).count() as u16);
				put(out, x.is_subset(y) as u16);
				put(out, x.is_disjoint(y) as u16);
			}
			put(out, (&small & &large).len() as u16);
			for &b in data {
				put(out, large.contains(&probe(b)) as u16);
				put(out, large.get(&key(usize::from(b))).map_or(0xffff, |k| k.0));
			}
		}
		2 => {
			// Sorted by `lt` and deduplicated by `ne`, out of `cmp`'s order.
			let mut set: BTreeSet<Probe> = data.iter().map(|&b| probe(b)).chain((0..n).map(key)).collect();
			put(out, set.len() as u16);
			for &b in data {
				put(out, set.contains(&probe(b)) as u16);
				put(out, set.get(&key(usize::from(b))).map_or(0xffff, |k| k.0));
			}
			let (lo, hi) = (probe(at(3)), probe(at(4)));
			let (lo, hi) = if lo.low() <= hi.low() { (lo, hi) } else { (hi, lo) };
			for k in set.range(lo..=hi) {
				put(out, k.0);
			}
			let sorted: BTreeSet<Probe> = (0..n).map(key).collect();
			put(out, set.intersection(&sorted).count() as u16);
			put(out, set.difference(&sorted).count() as u16);
			put(out, set.is_subset(&sorted) as u16);
			for &b in data {
				put(out, set.insert(probe(b ^ 0x55)) as u16);
				put(out, set.remove(&probe(b)) as u16);
			}
			put(out, set.replace(probe(at(5))).map_or(0xffff, |k| k.0));
			put(out, set.take(&probe(at(6))).map_or(0xffff, |k| k.0));
			put(out, set.iter().map(|k| k.0).fold(0, u16::wrapping_add));
		}
		_ => {
			// Sorted by `cmp`, but keys it finds equal stay where `ne` tells them apart.
			let mut map: BTreeMap<Probe, u16> = data
				.iter()
				.map(|&b| (Probe(u16::from(b) << 8 | u16::from(b & 7), &log), u16::from(b)))
				.collect();
			put(out, map.len() as u16);
			for &b in data {
				put(out, some(map.get(&Probe(u16::from(b & 7), &log))));
				put(out, map.remove(&Probe(u16::from(b) & 7, &log)).unwrap_or(0xffff));
			}
			put(out, map.len() as u16);
		}
	}
}

/// A B-tree key ordered by its `start`, which lends its `end`, as rangemap's start wrapper
/// lends its end wrapper, and its `id`, a form of std's.
struct Lent<'a> {
	start: Probe<'a>,
	end: Probe<'a>,
	id: u16,
}

impl PartialEq for Lent<'_> {
	fn eq(&self, other: &Self) -> bool {
		self.start == other.start
	}
}

impl Eq for Lent<'_> {}

impl PartialOrd for Lent<'_> {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for Lent<'_> {
	fn cmp(&self, other: &Self) -> Ordering {
		self.start.cmp(&other.start)
	}
}

impl<'a> std::borrow::Borrow<Probe<'a>> for Lent<'a> {
	fn borrow(&self) -> &Probe<'a> {
		&self.end
	}
}

impl std::borrow::Borrow<u16> for Lent<'_> {
	fn borrow(&self) -> &u16 {
		&self.id
	}
}

/// std's B-tree searched by a form its keys lend by the crate's own `Borrow`, which it
/// orders by that form's `Ord` against what each key lends, as `Probe` logs: of a map and a
/// set whose keys' ends and ids rise with their starts, or of odd `kind`s fall, so a search
/// by one disagrees with the tree's order. Each answer is followed by the log.
pub fn btree_borrows(kind: u8, data: &[u8], out: &mut Vec<u8>) {
	let log = std::cell::Cell::new(0u32);
	let at = |i: usize| data.get(i).copied().unwrap_or(0);
	let put = |out: &mut Vec<u8>, answer: u16| {
		out.extend(answer.to_le_bytes());
		out.extend(log.replace(0).to_le_bytes());
	};
	let n = usize::from(at(0)) + 1;
	let falls = kind % 2 == 1;
	let lent = |i: usize| {
		let start = (i * 7) as u16 & 0x3ff;
		let end = if falls { 0x3ff - start } else { start + 3 };
		Lent {
			start: Probe(start, &log),
			end: Probe(end, &log),
			id: end ^ 0x100,
		}
	};
	let probe = |b: u8| Probe(u16::from(b) * 4 + u16::from(b >> 6), &log);
	let id = |b: u8| (u16::from(b) * 4) ^ 0x100;
	let some = |v: Option<&u16>| v.map_or(0xffff, |v| *v);
	let mut map: BTreeMap<Lent, u16> = BTreeMap::new();
	let mut set: BTreeSet<Lent> = BTreeSet::new();
	for i in 0..n {
		map.insert(lent(i), i as u16);
		set.insert(lent(i * 2));
	}
	put(out, map.len() as u16);
	for &b in &data[data.len().min(4)..] {
		put(out, some(map.get(&probe(b))));
		put(out, some(map.get(&id(b))));
		if let Some(v) = map.get_mut(&probe(b ^ 1)) {
			*v += 1;
		}
		put(out, map.get_key_value(&probe(b)).map_or(0xffff, |(k, v)| k.start.0 ^ v));
		put(out, map.contains_key(&id(b ^ 2)) as u16);
		if map.contains_key(&probe(b)) {
			put(out, map[&probe(b)]);
		}
		put(out, set.contains(&probe(b)) as u16);
		put(out, set.get(&id(b)).map_or(0xffff, |k| k.start.0));
		match b % 5 {
			0 => put(out, map.remove(&probe(b)).unwrap_or(0xffff)),
			1 => put(out, map.remove_entry(&id(b)).map_or(0xffff, |(k, v)| k.id ^ v)),
			2 => put(out, set.remove(&probe(b)) as u16),
			3 => put(out, set.take(&id(b)).map_or(0xffff, |k| k.id)),
			_ => put(out, map.len() as u16),
		}
	}
	// Backwards with `kind` 2, which std's checks by the form's `gt` and panics on.
	let (lo, hi) = match kind % 4 < 2 {
		true => (at(1).min(at(2)), at(1).max(at(2))),
		false => (at(1), at(2)),
	};
	let ranges: [(Bound<Probe>, Bound<Probe>); 4] = [
		(Bound::Excluded(probe(lo)), Bound::Unbounded),
		(Bound::Included(probe(lo)), Bound::Excluded(probe(hi))),
		(Bound::Unbounded, Bound::Included(probe(hi))),
		(Bound::Excluded(probe(lo)), Bound::Included(probe(hi))),
	];
	for range in ranges {
		for (k, v) in map.range::<Probe, _>(range) {
			put(out, k.start.0 ^ v);
		}
	}
	// rangemap's form: a bound of a reference.
	let sliver = probe(lo);
	for (k, v) in map.range::<Probe, (Bound<&Probe>, Bound<_>)>((Bound::Excluded(&sliver), Bound::Unbounded)) {
		put(out, k.start.0 ^ v);
	}
	for (_, v) in map.range_mut::<u16, _>(id(lo)..=id(hi)) {
		*v ^= 0x55;
	}
	for k in set.range::<u16, _>(id(lo)..id(hi)) {
		put(out, k.id);
	}
	for k in set.range::<Probe, _>(..probe(hi)).rev() {
		put(out, k.start.0);
	}
	let right = map.split_off(&probe(at(3)));
	put(out, (map.len() * 1000 + right.len()) as u16);
	let high = set.split_off(&id(at(3)));
	put(out, (set.len() * 1000 + high.len()) as u16);
	put(out, right.values().fold(0, |a, v| a ^ v));
}

/// A B-tree value, or a set's key, that logs its tag as it drops, ordered by it. A clone's
/// tag is 1000 past the original's.
struct Tagged<'a>(u16, &'a std::cell::RefCell<Vec<u16>>);

impl Drop for Tagged<'_> {
	fn drop(&mut self) {
		self.1.borrow_mut().push(self.0);
	}
}

impl Clone for Tagged<'_> {
	fn clone(&self) -> Self {
		Tagged(self.0 + 1000, self.1)
	}
}

impl PartialEq for Tagged<'_> {
	fn eq(&self, other: &Self) -> bool {
		self.0 == other.0
	}
}

impl Eq for Tagged<'_> {}

impl PartialOrd for Tagged<'_> {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for Tagged<'_> {
	fn cmp(&self, other: &Self) -> Ordering {
		self.0.cmp(&other.0)
	}
}

/// Whole B-trees whose entries log as they drop, which std drops node by node, each node's
/// entries in order after its children's: maps built by random, ascending, and descending
/// inserts and by `collect`, emptied in part by removals that merge nodes, cleared, assigned
/// over, retained, split, cloned, and nested, sets, and `IntoIter`, which drops what it still
/// holds in key order. The first byte sets how many entries, up to 297 so trees reach height
/// 2, and the rest seed the keys. Each drop is logged, with a mark between the steps.
pub fn btree_drops(kind: u8, data: &[u8], out: &mut Vec<u8>) {
	let log = std::cell::RefCell::new(Vec::new());
	let tag = |k: u16| Tagged(k, &log);
	let mark = |m: u16| log.borrow_mut().push(0xf000 | m);
	let n = u16::from(data.first().copied().unwrap_or(0)) * 7 / 6;
	// xorshift32, seeded by the input, so a short input scripts a tree of hundreds of entries.
	let mut state = data
		.iter()
		.fold(0x9e37_79b9u32, |s, &b| s.rotate_left(5) ^ u32::from(b))
		| 1;
	// Keys apart enough that a random script repeats some.
	let mut key = || {
		state ^= state << 13;
		state ^= state >> 17;
		state ^= state << 5;
		(state % 400) as u16
	};
	let mut random = |count: u16| {
		let mut map = BTreeMap::new();
		for _ in 0..count {
			let k = key();
			map.insert(k, tag(k));
		}
		map
	};
	match kind % 10 {
		// By random inserts, whose replaced values drop as they go.
		0 => {
			let map = random(n);
			mark(0);
			drop(map);
		}
		// In ascending order, then descending, which leave nodes of other sizes.
		1 => {
			drop((0..n).fold(BTreeMap::new(), |mut map, k| {
				map.insert(k, tag(k));
				map
			}));
			mark(0);
			drop((0..n).rev().fold(BTreeMap::new(), |mut map, k| {
				map.insert(k, tag(k));
				map
			}));
		}
		// By `collect`, std's bulk build from sorted entries, whose repeats drop as it builds.
		2 => {
			let keys: Vec<u16> = (0..n).map(|_| key()).collect();
			let map: BTreeMap<u16, Tagged> = keys.into_iter().map(|k| (k, tag(k))).collect();
			mark(0);
			drop(map);
		}
		// Removals after random inserts, which steal from and merge nodes, then pops.
		3 => {
			let mut map = random(n);
			for _ in 0..n {
				let k = key();
				if k % 3 != 0 {
					drop(map.remove(&k));
				}
			}
			for _ in 0..n / 8 {
				drop(map.pop_first());
				drop(map.pop_last());
			}
			mark(0);
			drop(map);
		}
		// A set, by inserts and removals.
		4 => {
			let mut set = BTreeSet::new();
			for _ in 0..n {
				set.insert(tag(key()));
			}
			for _ in 0..n / 2 {
				set.remove(&tag(key()));
			}
			mark(0);
			drop(set);
		}
		// Maps of maps, each of which drops its own nodes as its outer entry drops.
		5 => {
			let mut map: BTreeMap<u16, BTreeMap<u16, Tagged>> = BTreeMap::new();
			for _ in 0..n {
				let (k, j) = (key() % 24, key());
				map.entry(k).or_default().insert(j, tag(j));
			}
			mark(0);
			drop(map);
		}
		// `IntoIter`, read from both ends, drops what it still holds in key order.
		6 => {
			let mut iter = random(n).into_iter();
			for _ in 0..data.len() {
				drop(iter.next());
				drop(iter.next_back());
			}
			mark(0);
			drop(iter);
		}
		// `clear`, and a map assigned over.
		7 => {
			let mut map = random(n);
			mark(0);
			map.clear();
			mark(1);
			for k in 0..n {
				map.insert(k, tag(k));
			}
			map = BTreeMap::new();
			mark(map.len() as u16);
		}
		// `retain`, and a map split in two, each of which drops as its nodes are.
		8 => {
			let mut map = random(n);
			map.retain(|k, _| k % 5 != 0);
			mark(0);
			let high = map.split_off(&200);
			drop(map);
			mark(1);
			drop(high);
		}
		// A clone, which std builds node for node as the original.
		_ => {
			let mut map = random(n);
			for _ in 0..n / 2 {
				drop(map.remove(&key()));
			}
			mark(0);
			let copy = map.clone();
			drop(map);
			mark(1);
			drop(copy);
		}
	}
	out.extend(log.take().iter().flat_map(|e| e.to_be_bytes()));
}
