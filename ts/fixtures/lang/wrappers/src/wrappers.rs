//! std's bounds and wrappers: `Bound` and `RangeBounds` of each range, `..=` as a
//! B-tree's range, `split_off`, `Reverse` as an order and a key, and `NonZero`, made, read,
//! matched as a constant, compared, and printed.

use std::cell::RefCell;
use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet};
use std::num::NonZero;
use std::ops::{Bound, RangeBounds};

use crate::harness::put;

/// A rate that is never 0, as a timescale is.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Scale(NonZero<u64>);

impl Scale {
	const ONE: Self = Self(NonZero::new(1).unwrap());
	const MILLI: Self = Self(NonZero::new(1000).unwrap());
	const WIDE: Self = Self(NonZero::new(u64::MAX).unwrap());
}

fn put_u64(out: &mut Vec<u8>, v: u64) {
	out.extend_from_slice(&v.to_be_bytes());
}

fn put_bound(out: &mut Vec<u8>, b: Bound<&u64>) {
	match b {
		Bound::Included(v) => {
			out.push(1);
			put_u64(out, *v);
		}
		Bound::Excluded(v) => {
			out.push(2);
			put_u64(out, *v);
		}
		Bound::Unbounded => out.push(3),
	}
}

/// The bounds of `r`, written, then the first sequence and the end past the last it holds,
/// as a range of groups reads them.
fn bounds(out: &mut Vec<u8>, r: impl RangeBounds<u64>) {
	put_bound(out, r.start_bound());
	put_bound(out, r.end_bound());
	let start = match r.start_bound().cloned() {
		Bound::Included(s) => Some(s),
		Bound::Excluded(s) => s.checked_add(1),
		Bound::Unbounded => Some(0),
	};
	let end = match r.end_bound().cloned() {
		Bound::Included(e) => e.checked_add(1),
		Bound::Excluded(e) => Some(e),
		Bound::Unbounded => None,
	};
	put_u64(out, start.unwrap_or(0));
	put_u64(out, end.unwrap_or(0));
	out.push(start.is_some() as u8 + 2 * end.is_some() as u8);
}

/// A sum that drops, as a fold's accumulator may.
struct Tally(u64);

impl Drop for Tally {
	fn drop(&mut self) {}
}

/// A bound of `a` picked by `kind`, held by value.
fn bound_of(kind: u8, a: u64) -> Bound<u64> {
	match kind % 3 {
		0 => Bound::Included(a),
		1 => Bound::Excluded(a),
		_ => Bound::Unbounded,
	}
}

pub fn wrappers(kind: u8, data: &[u8], out: &mut Vec<u8>) {
	let at = |i: usize| data.get(i).copied().unwrap_or(0);
	// Wide values next to the ends, so `checked_add` and the limbs past 2^53 show.
	let wide = |b: u8| match b % 4 {
		0 => b as u64,
		1 => u64::MAX - b as u64,
		2 => (1 << 53) + b as u64,
		_ => b as u64 * 7,
	};
	let (a, b) = (wide(at(0)), wide(at(1)));
	match kind % 5 {
		0 => {
			bounds(out, a..b);
			bounds(out, a..=b);
			// Drained, an inclusive range's start passes its end, and only wrapping exhausts it,
			// which leaves it excluding its start.
			let mut stepped = (a % 4)..=(b % 4);
			while stepped.next().is_some() {}
			bounds(out, stepped);
			let mut top = (u64::MAX - a % 2)..=u64::MAX;
			while top.next().is_some() {}
			bounds(out, top);
			let mut down = 0..=(b % 3);
			while down.next_back().is_some() {}
			bounds(out, down);
			// Drained by a fold, as std's `try_fold` takes it, `start` stops at `end`, exhausted.
			let (lo, hi) = (a % 4, b % 4);
			let mut found = lo..=hi;
			out.push(found.any(|x| x == hi) as u8);
			bounds(out, found);
			let mut rejected = lo..=hi;
			out.push(rejected.all(|x| x < hi) as u8);
			bounds(out, rejected.clone());
			out.push(rejected.find(|_| true).is_some() as u8);
			let mut placed = lo..=hi;
			out.push(placed.position(|x| x == lo + 1).map_or(0xee, |i| i as u8));
			bounds(out, placed);
			let mut each = lo..=hi;
			each.by_ref().for_each(|x| out.push(x as u8));
			bounds(out, each);
			let mut summed = lo..=hi;
			put_u64(out, summed.by_ref().sum());
			bounds(out, summed);
			let mut top = (u64::MAX - a % 3)..=u64::MAX;
			put_u64(out, top.by_ref().count() as u64);
			bounds(out, top);
			let mut last = lo..=hi;
			put_u64(out, last.by_ref().last().unwrap_or(0xee));
			bounds(out, last);
			// `max` reads its first by `next`, so one alone passes `end`, and folds the rest.
			let mut most = lo..=hi;
			put_u64(out, most.by_ref().max().unwrap_or(0xee));
			bounds(out, most);
			let mut collected = lo..=hi;
			out.push(collected.by_ref().collect::<Vec<_>>().len() as u8);
			bounds(out, collected);
			// Of an accumulator that drops, which takes the range's own `fold`.
			let mut gathered = lo..=hi;
			put_u64(out, gathered.by_ref().fold(Tally(0), |t, x| Tally(t.0 + x)).0);
			bounds(out, gathered);
			let mut keyed = lo..=hi;
			out.push(keyed.by_ref().collect::<BTreeSet<_>>().len() as u8);
			bounds(out, keyed);
			let mut queued = lo..=hi;
			out.push(queued.by_ref().collect::<std::collections::VecDeque<_>>().len() as u8);
			bounds(out, queued);
			let mut hashed = lo..=hi;
			out.push(hashed.by_ref().collect::<std::collections::HashSet<_>>().len() as u8);
			bounds(out, hashed);
			let mut extended = lo..=hi;
			let mut held = Vec::new();
			held.extend(extended.by_ref());
			out.push(held.len() as u8);
			bounds(out, extended);
			// Bytes, which collect into their own `Vec<u8>`.
			let mut narrow = (lo as u8)..=(hi as u8);
			out.extend(narrow.by_ref().collect::<Vec<u8>>());
			out.push(*narrow.start());
			out.push(narrow.is_empty() as u8);
			bounds(out, ..b);
			bounds(out, ..=b);
			bounds(out, a..);
			bounds(out, ..);
			bounds(out, (bound_of(at(2), a), bound_of(at(3), b)));
			let held = (bound_of(at(3), b), Bound::Unbounded);
			bounds(out, held);
			put_bound(out, held.0.as_ref());
			// Each one-sided range's field, read and through struct update syntax.
			let (to, from) = (
				..(at(2) as usize % (data.len() + 1)),
				(at(3) as usize % (data.len() + 1))..,
			);
			let through = ..=b;
			put_u64(out, (to.end + from.start) as u64);
			put_u64(out, through.end);
			out.push(data[std::ops::RangeTo { ..to }].len() as u8);
			out.push(data[std::ops::RangeFrom { ..from }].len() as u8);
		}
		1 => {
			let mut map = BTreeMap::new();
			for (i, &x) in data.iter().enumerate() {
				map.insert(x as u64, i as u64);
			}
			let (a, b) = (at(0) as u64, at(1) as u64);
			for (k, v) in map.range(..=a) {
				put_u64(out, *k);
				put_u64(out, *v);
			}
			out.push(0xfe);
			if let Some((k, _)) = map.range(..=b).next_back() {
				put_u64(out, *k);
			}
			out.push(map.range(..a).count() as u8);
			// Exhausted by wrapping either way, an inclusive range holds nothing.
			let mut top = (u64::MAX - a % 2)..=u64::MAX;
			while top.next().is_some() {}
			let mut bottom = 0..=(b % 2);
			while bottom.next_back().is_some() {}
			out.push(map.range(top).count() as u8);
			out.push(map.range(bottom).count() as u8);
			let above = map.split_off(&b);
			for (k, v) in map.iter().chain(above.iter()) {
				put_u64(out, *k);
				put_u64(out, *v);
			}
			out.push(map.len() as u8);
			out.push(above.len() as u8);
			let mut set: BTreeSet<u8> = data.iter().copied().collect();
			let high = set.split_off(&at(2));
			out.extend(set.iter().chain(high.iter()));
			out.push(high.len() as u8);
		}
		2 => {
			// Ordered the other way, alone and in tuples, as keys and as sort keys.
			let (x, y) = (at(0), at(1));
			out.push(Reverse(x).cmp(&Reverse(y)) as i8 as u8);
			out.push((Reverse(x) < Reverse(y)) as u8);
			out.push((Reverse(Some(x)) > Reverse(None)) as u8);
			out.push(Reverse(x).max(Reverse(y)).0);
			out.push(((Reverse(x), y) <= (Reverse(y), x)) as u8);
			let keyed: BTreeMap<Reverse<u8>, usize> = data.iter().enumerate().map(|(i, &d)| (Reverse(d), i)).collect();
			for (Reverse(k), i) in &keyed {
				out.push(*k);
				out.push(*i as u8);
			}
			let mut ranked: Vec<(u8, usize)> = data.iter().enumerate().map(|(i, &d)| (d, i)).collect();
			ranked.sort_by_key(|&(d, i)| (Reverse(d % 4), i));
			for (d, i) in ranked {
				out.push(d);
				out.push(i as u8);
			}
			let wrapped = Reverse(String::from("held"));
			put(out, &wrapped.0);
		}
		3 => {
			let n = NonZero::new(a);
			match n {
				Some(n) => {
					put_u64(out, n.get());
					put_u64(out, u64::from(n));
					put(out, &format!("{n} {n:?} {}", n == NonZero::new(b).unwrap_or(n)));
					out.push((n > Scale::ONE.0) as u8);
					out.push(n.cmp(&Scale::MILLI.0) as i8 as u8);
				}
				None => out.push(0),
			}
			let none: Option<NonZero<u64>> = NonZero::new(a & 1);
			out.push(none.is_some() as u8);
		}
		_ => {
			// Matched against constants, as derived `PartialEq` and patterns read them.
			let scale = match NonZero::new(a) {
				Some(n) if at(2) % 2 == 0 => Scale(n),
				_ => [Scale::ONE, Scale::MILLI, Scale::WIDE][at(3) as usize % 3],
			};
			let name = match scale {
				Scale::ONE => "one",
				Scale::MILLI => "milli",
				Scale::WIDE => "wide",
				Scale(_) => "other",
			};
			put(out, name);
			out.push((scale == Scale::MILLI) as u8);
			out.push((scale < Scale::WIDE) as u8);
			put_u64(out, scale.0.get());
		}
	}
}

/// Logs its id as it is cloned, and as it drops.
struct Logged<'a> {
	id: u8,
	log: &'a RefCell<Vec<u8>>,
}

impl Clone for Logged<'_> {
	fn clone(&self) -> Self {
		self.log.borrow_mut().push(self.id | 0x80);
		Logged {
			id: self.id.wrapping_add(1) & 0x7f,
			log: self.log,
		}
	}
}

impl Drop for Logged<'_> {
	fn drop(&mut self) {
		self.log.borrow_mut().push(self.id);
	}
}

/// `Reverse`'s `clone` and `clone_from`, each of what it wraps: a value JS copies, one
/// that logs as it is cloned and drops, and a `Vec`, which JS shares and whose
/// `clone_from` keeps its buffer.
pub fn reverses(kind: u8, data: &[u8], out: &mut Vec<u8>) {
	let at = |i: usize| data.get(i).copied().unwrap_or(i as u8);
	match kind % 3 {
		0 => {
			let a = Reverse(at(0));
			let mut b = a.clone();
			b.0 = b.0.wrapping_add(1);
			out.extend([a.0, b.0]);
			b.clone_from(&a);
			out.push(b.0);
		}
		1 => {
			let log = RefCell::new(Vec::new());
			{
				let a = Reverse(Logged {
					id: at(0) & 0x7f,
					log: &log,
				});
				let mut b = a.clone();
				log.borrow_mut().push(0xfe);
				b.clone_from(&Reverse(Logged {
					id: at(1) & 0x7f,
					log: &log,
				}));
				log.borrow_mut().push(0xfe);
				out.extend([a.0.id, b.0.id]);
			}
			out.extend(log.into_inner());
		}
		_ => {
			let a = Reverse((0..at(0) % 6).collect::<Vec<u8>>());
			let mut b = a.clone();
			b.0.push(0x55);
			out.extend(a.0.iter().chain(&b.0));
			out.extend([a.0.capacity() as u8, b.0.capacity() as u8]);
			let mut kept = Reverse(Vec::with_capacity(usize::from(at(1) % 16)));
			kept.0.push(at(2));
			kept.clone_from(&a);
			out.extend(kept.0.iter().copied());
			out.extend([kept.0.len() as u8, kept.0.capacity() as u8]);
		}
	}
}
