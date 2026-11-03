//! Ranges as pairs of `Bound`s and as the crate's own `RangeBounds`: `Vec::drain` and
//! `VecDeque::drain` of values that log as they drop, and a B-tree's `range` and
//! `range_mut`, with each kind of bound at either end, empty, backwards, equal, and out of
//! bounds, which panic where std's do, and when std asks the crate's own range its bounds.

use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::ops::{Bound, RangeBounds};

thread_local! {
	static LOG: RefCell<Vec<u8>> = const { RefCell::new(Vec::new()) };
}

fn log(entry: u8) {
	LOG.with_borrow_mut(|log| log.push(entry));
}

/// Puts what was logged since the last flush.
fn flush(out: &mut Vec<u8>) {
	LOG.with_borrow_mut(|log| {
		out.extend_from_slice(log);
		log.clear();
	});
}

/// Logs its tag as it drops.
struct Item {
	tag: u8,
}

impl Drop for Item {
	fn drop(&mut self) {
		log(self.tag);
	}
}

/// The crate's own range, which logs each bound std asks it for: 0xa5 its start, 0xae its end.
struct Asked<T> {
	start: Bound<T>,
	end: Bound<T>,
}

impl<T> RangeBounds<T> for Asked<T> {
	fn start_bound(&self) -> Bound<&T> {
		log(0xa5);
		self.start.as_ref()
	}

	fn end_bound(&self) -> Bound<&T> {
		log(0xae);
		self.end.as_ref()
	}
}

/// A key the crate orders by its own `Ord`, high to low, as rangemap's keys are by theirs,
/// which logs each `==` and `partial_cmp`: 0xe9 and 0xc9.
#[derive(Eq)]
struct Key(u8);

impl PartialEq for Key {
	fn eq(&self, other: &Self) -> bool {
		log(0xe9);
		self.0 == other.0
	}
}

impl Ord for Key {
	fn cmp(&self, other: &Self) -> Ordering {
		other.0.cmp(&self.0)
	}
}

impl PartialOrd for Key {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		log(0xc9);
		Some(self.cmp(other))
	}
}

fn bound<T>(kind: u8, at: T) -> Bound<T> {
	match kind % 3 {
		0 => Bound::Included(at),
		1 => Bound::Excluded(at),
		_ => Bound::Unbounded,
	}
}

fn asked<T>(s: u8, e: u8, a: T, b: T) -> Asked<T> {
	Asked {
		start: bound(s, a),
		end: bound(e, b),
	}
}

/// The keys of `set` within `range`, through a generic function, as each instance has it.
fn within<R: RangeBounds<u8>>(set: &BTreeSet<u8>, range: R) -> usize {
	set.range(range).count()
}

/// `kind` picks the collection and how its range is given. The first two bytes pick the
/// kinds of the range's bounds, the next two their values, and the rest are the items.
pub fn bounds(kind: u8, data: &[u8], out: &mut Vec<u8>) {
	// What a case that panicked left, which a translation outlives, as native Rust's trap does not.
	LOG.with_borrow_mut(Vec::clear);
	let &[s, e, a, b, ref data @ ..] = data else {
		return;
	};
	let reach = data.len() + 2;
	let (i, j) = (a as usize % reach, b as usize % reach);
	let mut items: Vec<Item> = data.iter().map(|&tag| Item { tag }).collect();
	match kind % 6 {
		// A `Vec` drained by a pair of `Bound`s, of indices and of references to them.
		0 => {
			let range = (bound(s, i), bound(e, j));
			for x in items.drain(range) {
				out.push(x.tag);
				flush(out);
			}
			let mut bytes = data.to_vec();
			out.extend(bytes.drain((bound(s, &i), bound(e, &j))).rev());
			out.extend(&bytes);
			let mut words: Vec<u32> = data.iter().map(|&b| b as u32 + 1000).collect();
			let mut drain = words.drain(range);
			out.extend(drain.next_back().map(|w| w as u8));
			out.push(drain.len() as u8);
			drop(drain);
			out.extend(words.iter().map(|&w| w as u8));
		}
		// Asked its start, then its end, before the drain yields or drops anything.
		1 => {
			let mut drain = items.drain(asked(s, e, i, j));
			flush(out);
			out.extend(drain.next().map(|x| x.tag));
			flush(out);
			drop(drain);
			flush(out);
			let mut bytes = data.to_vec();
			out.push(bytes.drain(asked(s, e, i, j)).count() as u8);
			flush(out);
			out.extend(&bytes);
		}
		// A `VecDeque` whose items wrap around its buffer, drained by each.
		2 => {
			let mut queue: VecDeque<Item> = VecDeque::new();
			for (n, &tag) in data.iter().enumerate() {
				match n % 2 {
					0 => queue.push_back(Item { tag }),
					_ => queue.push_front(Item { tag }),
				}
			}
			let mut drain = queue.drain((bound(s, i), bound(e, j)));
			out.extend(drain.next_back().map(|x| x.tag));
			flush(out);
			drop(drain);
			flush(out);
			out.extend(queue.iter().map(|x| x.tag));
			let reach = queue.len() + 1;
			let drain = queue.drain(asked(s, e, a as usize % reach, b as usize % reach));
			flush(out);
			out.push(drain.count() as u8);
			flush(out);
			out.extend(queue.iter().map(|x| x.tag));
		}
		// A B-tree's ranges of keys, through `Bound`s and references to them.
		3 => {
			let mut map: BTreeMap<u8, u8> = data.iter().enumerate().map(|(n, &k)| (k, n as u8)).collect();
			let range = (bound(s, a), bound(e, b));
			for (k, v) in map.range(range) {
				out.push(*k);
				out.push(*v);
			}
			out.extend(map.range((bound(s, &a), bound(e, &b))).next_back().map(|(k, _)| *k));
			for (_, v) in map.range_mut(range) {
				*v = v.wrapping_add(100);
			}
			out.extend(map.values());
			let set: BTreeSet<u8> = data.iter().copied().collect();
			out.extend(set.range(range).rev());
			out.push(within(&set, range) as u8);
		}
		// Asked nothing of an empty map, and its start and then its end of any other.
		4 => {
			let empty: BTreeMap<u8, Item> = BTreeMap::new();
			out.push(empty.range(asked(s, e, a, b)).count() as u8);
			flush(out);
			let set: BTreeSet<u8> = data.iter().copied().collect();
			let mut range = set.range(asked(s, e, a, b));
			flush(out);
			out.extend(range.next());
			out.extend(range.next_back());
			out.push(within(&set, asked(s, e, a, b)) as u8);
			flush(out);
		}
		// Keys the crate orders, ranged by references to them, as rangemap ranges, whose ends
		// std checks by `==` and `>`.
		_ => {
			let (a, b) = (Key(a), Key(b));
			let mut set = BTreeSet::new();
			let mut map = BTreeMap::new();
			for (n, &k) in data.iter().enumerate() {
				set.insert(Key(k));
				map.insert(Key(k), n as u8);
			}
			out.extend(set.range((bound(s, &a), bound(e, &b))).map(|k| k.0));
			flush(out);
			for (_, v) in map.range_mut((bound(s, &a), bound(e, &b))) {
				*v ^= 0x80;
			}
			flush(out);
			out.extend(map.values());
			// Held in a std type, which compares by it.
			let mut pairs = BTreeSet::new();
			for &k in data {
				pairs.insert((k, Key(k)));
			}
			let (a, b) = ((a.0, a), (b.0, b));
			out.extend(pairs.range((bound(s, &a), bound(e, &b))).map(|(k, _)| *k));
			flush(out);
		}
	}
	flush(out);
	out.extend(items.iter().map(|x| x.tag));
	drop(items);
	flush(out);
}
