//! std's slice sorts, of items that log each test they make: by `PartialOrd::lt`, `Ord::cmp`,
//! a comparator, and a key, stable and not, at lengths across each of std's thresholds, of
//! element types whose size, `Freeze`, and `Copy` pick each of its small sorts and
//! partitions. Then sorts by a comparator that orders nothing, which std may answer with a
//! panic, and B-trees collected of the crate's own keys, which sort them.

use std::cell::Cell;
use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet};

const EMPTY: (u32, u32) = (0x811c_9dc5, 0);

thread_local! {
	/// A hash of every test since the last `put` or `begin`, and how many.
	static LOG: Cell<(u32, u32)> = const { Cell::new(EMPTY) };
}

/// Forgets what an earlier case logged, which its panic may have left.
fn begin() {
	LOG.with(|log| log.set(EMPTY));
}

fn note(what: u8, a: u16, b: u16) {
	LOG.with(|log| {
		let (mut hash, count) = log.get();
		for byte in [what, a as u8, (a >> 8) as u8, b as u8, (b >> 8) as u8] {
			hash = (hash ^ u32::from(byte)).wrapping_mul(0x0100_0193);
		}
		log.set((hash, count + 1));
	});
}

/// Puts the log's hash and count, then each tag in order, or a hash of them past 64.
fn put<P>(v: &[Item<P>], out: &mut Vec<u8>) {
	let (hash, count) = LOG.with(|log| log.replace(EMPTY));
	out.extend(hash.to_le_bytes());
	out.extend(count.to_le_bytes());
	match v.len() <= 64 {
		true => out.extend(v.iter().flat_map(|x| x.tag.to_le_bytes())),
		false => {
			let tags = v.iter().fold(0u32, |h, x| (h ^ u32::from(x.tag)).wrapping_mul(0x0100_0193));
			out.extend(tags.to_le_bytes());
		}
	}
}

/// Ordered by `key` alone, so `tag` tells equal ones apart, beside `pad`, which sizes it.
#[derive(Clone, Copy)]
struct Item<P> {
	key: u16,
	tag: u16,
	pad: P,
}

impl<P> PartialEq for Item<P> {
	fn eq(&self, other: &Self) -> bool {
		self.key == other.key
	}
}

impl<P> Eq for Item<P> {}

impl<P> PartialOrd for Item<P> {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}

	fn lt(&self, other: &Self) -> bool {
		note(b'<', self.tag, other.tag);
		self.key < other.key
	}
}

impl<P> Ord for Item<P> {
	fn cmp(&self, other: &Self) -> Ordering {
		note(b'c', self.tag, other.tag);
		self.key.cmp(&other.key)
	}
}

/// A pad that is not `Copy`.
struct Held([u8; 2]);

/// Lengths at and past each of std's thresholds: its insertion sorts', its small sorts' and
/// their networks', its pivots' pseudomedians', and its eager and square-root runs'.
const LENGTHS: [usize; 20] = [0, 1, 2, 9, 13, 16, 17, 18, 20, 21, 32, 33, 48, 64, 65, 100, 300, 2000, 4096, 4097];

/// Each sort of items of each kind, of the length `kind` picks, keyed as the first byte of
/// `data` picks from the rest.
pub fn sorts(kind: u8, data: &[u8], out: &mut Vec<u8>) {
	begin();
	let n = LENGTHS[usize::from(kind) % LENGTHS.len()];
	let keys = keys(n, data);
	// A `Copy` one std's network sorts, and one that isn't `Freeze`.
	sorted(&keys, || (), out);
	sorted(&keys, || Cell::new(0u32), out);
	// Past 300, what std picks by the type plays out as below it.
	if n <= 300 {
		// One that isn't `Copy`, those its general small sort takes 8 or 4 at once, and the
		// one that partitions by Lomuto's scheme past that, and by Hoare's past that.
		sorted(&keys, || Held([0; 2]), out);
		sorted(&keys, || [0u32; 10], out);
		sorted(&keys, || [0u32; 22], out);
		sorted(&keys, || [0u32; 30], out);
	}
	// One whose stable sort's scratch is less than the slice, which only that sort reads.
	if n == 2000 {
		let mut v = items(&keys, || [0u32; 1100]);
		v.sort();
		put(&v, out);
	}
}

/// `n` keys, random, of few values, ascending, strictly descending, or ascending with some
/// out of place, as the first byte of `data` picks, from the rest.
fn keys(n: usize, data: &[u8]) -> Vec<u16> {
	let (pick, seed) = data.split_first().map_or((0, &[][..]), |(&p, s)| (p, s));
	let mut state = seed.iter().fold(0x9e37_79b9u32, |h, &b| (h ^ u32::from(b)).wrapping_mul(0x0100_0193)) | 1;
	let mut random = move || {
		state ^= state << 13;
		state ^= state >> 17;
		state ^= state << 5;
		state
	};
	(0..n)
		.map(|i| {
			let r = random();
			match pick % 5 {
				0 => r as u16,
				1 => (r % 3) as u16,
				2 => i as u16,
				3 => (n - i) as u16,
				_ if r % 8 == 0 => (r % 64) as u16,
				_ => i as u16,
			}
		})
		.collect()
}

/// `keys` as items, each padded by `pad` and tagged by where it is.
fn items<P>(keys: &[u16], pad: impl Fn() -> P) -> Vec<Item<P>> {
	keys.iter()
		.enumerate()
		.map(|(i, &key)| Item {
			key,
			tag: i as u16,
			pad: pad(),
		})
		.collect()
}

/// Each sort of `keys` as items padded by `pad`, and what it tested and left.
fn sorted<P>(keys: &[u16], pad: impl Fn() -> P, out: &mut Vec<u8>) {
	let items = || items(keys, &pad);
	let mut v = items();
	v.sort();
	put(&v, out);
	let mut v = items();
	v.sort_unstable();
	put(&v, out);
	let mut v = items();
	v.sort_by(|a, b| {
		note(b'b', a.tag, b.tag);
		b.key.cmp(&a.key)
	});
	put(&v, out);
	let mut v = items();
	v.sort_unstable_by(Ord::cmp);
	put(&v, out);
	let mut v = items();
	v.sort_by_key(|x| {
		note(b'k', x.tag, 0);
		x.key / 4
	});
	put(&v, out);
	let mut v = items();
	v.sort_unstable_by_key(|x| {
		note(b'k', x.tag, 0);
		x.key % 3
	});
	put(&v, out);
}

/// One sort, stable or not, of items of the kind `kind` picks, by a comparator whose
/// answers follow `data` rather than any order, which std may panic at: what it tested
/// and left.
pub fn disorders(kind: u8, data: &[u8], out: &mut Vec<u8>) {
	begin();
	let n = LENGTHS[usize::from(kind) % LENGTHS.len()].min(300);
	let keys = keys(n, data);
	let stable = kind / 20 % 2 == 0;
	match kind / 40 % 4 {
		0 => disordered(&keys, stable, || (), data, out),
		1 => disordered(&keys, stable, || [0u32; 10], data, out),
		2 => disordered(&keys, stable, || [0u32; 30], data, out),
		_ => disordered(&keys, stable, || Cell::new(0u32), data, out),
	}
}

fn disordered<P>(keys: &[u16], stable: bool, pad: impl Fn() -> P, data: &[u8], out: &mut Vec<u8>) {
	let mut v = items(keys, pad);
	let calls = Cell::new(0usize);
	let by = |a: &Item<P>, b: &Item<P>| {
		note(b'?', a.tag, b.tag);
		let i = calls.get();
		calls.set(i + 1);
		let bits = data.get(i % data.len().max(1)).copied().unwrap_or(0) ^ (i as u8);
		match (a.key ^ b.key ^ u16::from(bits)) % 3 {
			0 => Ordering::Less,
			1 => Ordering::Equal,
			_ => Ordering::Greater,
		}
	};
	match stable {
		true => v.sort_by(by),
		false => v.sort_unstable_by(by),
	}
	put(&v, out);
}

/// B-trees collected of items, which sort them stably by `PartialOrd::lt` as a set's keys
/// and by `Ord::cmp` as a map's, before keeping the last of equal ones: which each tested
/// and kept.
pub fn btrees(kind: u8, data: &[u8], out: &mut Vec<u8>) {
	begin();
	let n = LENGTHS[usize::from(kind) % LENGTHS.len()].min(300);
	let keys = keys(n, data);
	let set: BTreeSet<Item<()>> = items(&keys, || ()).into_iter().collect();
	put(&set.into_iter().collect::<Vec<_>>(), out);
	let map: BTreeMap<Item<()>, u16> = items(&keys, || ()).into_iter().map(|x| (x, x.tag)).collect();
	put(&map.into_keys().collect::<Vec<_>>(), out);
	let map: BTreeMap<Item<[u32; 30]>, u16> = items(&keys, || [0; 30]).into_iter().map(|x| (x, x.tag)).collect();
	put(&map.into_keys().collect::<Vec<_>>(), out);
}
