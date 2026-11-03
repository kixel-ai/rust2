//! Size hints: each of std's iterators' and adaptors' `size_hint`, as it reads, and the
//! `capacity overflow` std's collections panic with as they reserve by one.

use std::cell::Cell;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::hash::Hash;

/// Lengths at the edges a hint meets: none, a few, `isize::MAX` and past it, `usize::MAX`
/// and past it, and past 2^53.
const LENGTHS: [u64; 8] = [0, 1, 5, 0x7fff_ffff, 0x8000_0000, 0xffff_ffff, 0x1_0000_0000, 1 << 60];

/// A hint, as four bytes each, the upper bound after whether it has one.
fn put(out: &mut Vec<u8>, (lower, upper): (usize, Option<usize>)) {
	out.extend_from_slice(&(lower as u32).to_be_bytes());
	match upper {
		Some(upper) => {
			out.push(1);
			out.extend_from_slice(&(upper as u32).to_be_bytes());
		}
		None => out.push(0),
	}
}

/// `it`'s hint, and again after each of up to three items, read from the back where
/// `steps` has the item's bit set.
fn walk<I: DoubleEndedIterator>(mut it: I, steps: u8, out: &mut Vec<u8>) {
	put(out, it.size_hint());
	for k in 0..steps % 4 {
		match steps >> (2 + k) & 1 {
			0 => it.next(),
			_ => it.next_back(),
		};
		put(out, it.size_hint());
	}
}

/// As `walk`, of an iterator that reads only from the front.
fn walk_front<I: Iterator>(mut it: I, steps: u8, out: &mut Vec<u8>) {
	put(out, it.size_hint());
	for _ in 0..steps % 4 {
		it.next();
		put(out, it.size_hint());
	}
}

/// `n`, or `u32::MAX` past it.
fn u32_of(n: u64) -> u32 {
	u32::try_from(n).unwrap_or(u32::MAX)
}

/// An iterator of the crate's own, which keeps std's hint.
struct Countdown(u8);

/// An iterator of the crate's own that never ends.
struct Endless(u64);

impl Iterator for Endless {
	type Item = u64;

	fn next(&mut self) -> Option<u64> {
		Some(self.0)
	}
}

impl Iterator for Countdown {
	type Item = u8;

	fn next(&mut self) -> Option<u8> {
		self.0 = self.0.checked_sub(1)?;
		Some(self.0)
	}
}

/// The hints of the iterator `kind` and the top three bits of `data`'s third byte pick, of
/// lengths `data`'s first two bytes pick or of its fourth byte on, as each reads from the
/// front or the back by the third byte's other bits.
pub fn hints(kind: u8, data: &[u8], out: &mut Vec<u8>) {
	let at = |i: usize| data.get(i).copied().unwrap_or(0);
	let (n, m) = (LENGTHS[usize::from(at(0)) % 8], LENGTHS[usize::from(at(1)) % 8]);
	let steps = at(2);
	let arm = steps >> 5;
	let bytes = data.get(3..).unwrap_or(&[]);
	match kind % 24 {
		// Ranges of each width, past `usize::MAX` and 2^53, and empty ones.
		0 => match arm % 5 {
			0 => walk(0..u32_of(n), steps, out),
			1 => walk(u32_of(m)..u32_of(n), steps, out),
			2 => walk(n as u8..m as u8, steps, out),
			3 => walk(-(n as i32)..m as i32, steps, out),
			_ => walk(0..n as usize, steps, out),
		},
		1 => match arm % 4 {
			0 => walk(0..n, steps, out),
			1 => walk(1 << 63..(1 << 63) + n, steps, out),
			2 => walk(-(n as i64)..m as i64, steps, out),
			_ => walk(i64::MIN..i64::MIN + n as i64, steps, out),
		},
		2 => match arm % 3 {
			0 => walk(1u128 << 100..(1u128 << 100) + u128::from(n), steps, out),
			1 => walk(-(1i128 << 100)..i128::from(n) - (1i128 << 100), steps, out),
			_ => walk(i128::MIN..i128::MAX, steps, out),
		},
		3 => match arm % 5 {
			0 => walk(0..=u32_of(n), steps, out),
			1 => walk(0..=u32::MAX, steps, out),
			2 => walk(i8::MIN..=i8::MAX, steps, out),
			3 => walk(u64::MAX - n..=u64::MAX, steps, out),
			_ => walk(u32_of(n)..=u32_of(m), steps, out),
		},
		// Chains, whose upper bound overflows to `None`, and which hint one side once the
		// other ends.
		4 => match arm % 4 {
			0 => walk((0..u32_of(n.min(9))).chain(0..u32_of(m)), steps, out),
			1 => walk((0..n).chain(0..m), steps, out),
			2 => walk((0..u32_of(n)).chain(0..u32::MAX), steps, out),
			_ => walk_front(Endless(n).take(n as usize % 9).chain(0..m), steps, out),
		},
		// Zips: the shorter side's bounds, by index or not.
		5 => match arm % 4 {
			0 => walk_front((0..n).zip(0..m), steps, out),
			1 => walk_front(Endless(n).zip(0..u32_of(m)), steps, out),
			2 => walk(bytes.iter().zip(bytes.iter().skip(1)), steps, out),
			_ => walk_front((0..u32_of(n)).zip(Endless(n)), steps, out),
		},
		6 => match arm % 4 {
			0 => walk_front((0..n).skip(m as usize), steps, out),
			1 => walk((0..u32_of(n)).take(m as usize), steps, out),
			2 => walk_front(Endless(n).take(m as usize), steps, out),
			_ => walk_front(Endless(n).skip(m as usize % 9), steps, out),
		},
		// Steps over ranges the step specializes on and over others.
		7 => {
			let step = usize::from(at(1)) % 7 + 1;
			match arm % 4 {
				0 => walk((0..u32_of(n)).step_by(step), steps, out),
				1 => walk_front((0..u32_of(n)).step_by(step), steps, out),
				2 => walk_front((0..n).step_by(step), steps, out),
				_ => walk(bytes.iter().step_by(step), steps, out),
			}
		}
		8 => match arm % 6 {
			0 => walk((0..n).filter(|i| i % 2 == 0), steps, out),
			1 => walk((0..n).filter_map(|i| i.checked_sub(1)), steps, out),
			2 => walk_front((0..n).skip_while(|&i| i < 2), steps, out),
			3 => walk_front((0..n).take_while(|&i| i < 1), steps, out),
			4 => walk_front((0..n).map_while(|i| i.checked_sub(1)), steps, out),
			_ => walk_front(Endless(n).filter(|_| true), steps, out),
		},
		9 => match arm % 6 {
			0 => walk((0..n).map(|i| i + 1), steps, out),
			1 => walk((0..n).inspect(|_| {}), steps, out),
			2 => walk((0..u32_of(n)).enumerate(), steps, out),
			3 => walk((0..n).rev(), steps, out),
			4 => walk(bytes.iter().copied(), steps, out),
			_ => walk(bytes.iter().cloned(), steps, out),
		},
		10 => match arm % 4 {
			0 => walk_front(Endless(n).fuse(), steps, out),
			1 => walk_front(Endless(n).take(2).fuse(), steps, out),
			2 => walk((0..n).fuse(), steps, out),
			_ => walk_front(Countdown(n as u8).fuse(), steps, out),
		},
		// Peeked items, and a peeked end.
		11 => {
			let mut it = (0..n).chain(0..m).peekable();
			put(out, it.size_hint());
			for _ in 0..steps % 4 {
				it.peek();
				put(out, it.size_hint());
				it.next();
				put(out, it.size_hint());
			}
			let mut few = bytes.iter().peekable();
			while few.next().is_some() {}
			few.peek();
			put(out, few.size_hint());
		}
		// Flattens of arrays, whose length counts, empty ones too, of iterators that may
		// hold any number, and of ones that hold at most one, which std's hint specializes on.
		12 => match arm % 7 {
			0 => walk((0..n).map(|i| [i; 3]).flatten(), steps, out),
			1 => walk((0..n).flat_map(|i| [i]), steps, out),
			2 => walk(bytes.chunks(2).flatten(), steps, out),
			3 => walk((0..n.min(9)).map(|i| 0..i).flatten(), steps, out),
			// Reading one reads every empty array, so only a few, and the hint alone of more.
			5 => walk((0..n.min(9)).map(|_| [0u8; 0]).flatten(), steps, out),
			6 => put(out, (0..n).flat_map(|i| [i; 0]).size_hint()),
			_ => walk((0..n).flat_map(|i| [[i; 2]; 2].into_iter().flatten()), steps, out),
		},
		13 => match arm % 6 {
			0 => walk((0..n).map(|i| i.checked_sub(2)).flatten(), steps, out),
			1 => walk((0..n).flat_map(|i| (i % 2 == 0).then_some(i)), steps, out),
			2 => walk((0..n).flat_map(std::iter::once), steps, out),
			3 => walk(
				(0..n).flat_map(|i| Some(i).into_iter().filter(|i| i % 2 == 0)),
				steps,
				out,
			),
			4 => walk(bytes.iter().map(|&b| [b]).flatten(), steps, out),
			_ => {
				let arrays: Vec<[u8; 2]> = bytes.iter().map(|&b| [b, b]).collect();
				walk(arrays.iter().flatten(), steps, out);
			}
		},
		// Sources.
		14 => match arm % 6 {
			0 => walk(bytes.iter(), steps, out),
			1 => walk(bytes.to_vec().into_iter(), steps, out),
			2 => walk([at(3), at(4), at(5)].into_iter(), steps, out),
			3 => walk(bytes.first().into_iter(), steps, out),
			4 => walk(std::iter::once(n), steps, out),
			_ => walk(std::iter::empty::<u8>(), steps, out),
		},
		// `from_fn`'s of a closure that captures something, and of one that captures nothing.
		15 => match arm % 6 {
			5 => walk_front(std::iter::from_fn(|| None::<u8>), steps, out),
			0 => walk_front(std::iter::successors(Some(n % 4), |&i| i.checked_sub(1)), steps, out),
			1 => walk_front(std::iter::successors(None::<u8>, |&i| Some(i)), steps, out),
			2 => walk_front(Countdown(n as u8), steps, out),
			3 => walk_front(std::iter::from_fn(move || Some(n)), steps, out),
			_ => {
				let mut it = bytes.iter();
				walk(it.by_ref(), steps, out);
				put(out, it.size_hint());
			}
		},
		16 => {
			let s: String = bytes
				.iter()
				.map(|&b| ['a', 'é', '😀', '\u{800}'][usize::from(b % 4)])
				.collect();
			match arm % 4 {
				0 => walk(s.chars(), steps, out),
				1 => walk_front(s.split('é'), steps, out),
				2 => walk(s.bytes(), steps, out),
				_ => {
					let mut t = s.clone();
					let half: usize = bytes[..bytes.len() / 2]
						.iter()
						.map(|&b| [1, 2, 4, 3][usize::from(b % 4)])
						.sum();
					walk(t.drain(half..), steps, out);
				}
			}
		}
		17 => {
			let size = usize::from(at(1)) % 4 + 1;
			match arm % 5 {
				0 => walk(bytes.windows(size), steps, out),
				1 => walk(bytes.chunks(size), steps, out),
				2 => walk(bytes.chunks_exact(size), steps, out),
				3 => walk(bytes.rchunks(size), steps, out),
				_ => walk(bytes.rchunks_exact(size), steps, out),
			}
		}
		// Collections' iterators: each knows its length, but a B-tree's range.
		18 => {
			let map: HashMap<u8, u8> = bytes.iter().map(|&b| (b, b)).collect();
			let set: HashSet<u8> = bytes.iter().copied().collect();
			match arm % 5 {
				0 => walk_front(map.iter(), steps, out),
				1 => walk_front(map.keys(), steps, out),
				2 => walk_front(set.iter(), steps, out),
				3 => walk_front(map.clone().into_iter(), steps, out),
				_ => walk_front(map.clone().drain(), steps, out),
			}
		}
		19 => {
			let map: BTreeMap<u8, u8> = bytes.iter().map(|&b| (b, b)).collect();
			let set: BTreeSet<u8> = bytes.iter().copied().collect();
			match arm % 4 {
				0 => walk(map.iter(), steps, out),
				1 => walk(map.range(at(3)..), steps, out),
				2 => walk(set.iter(), steps, out),
				_ => walk(map.values(), steps, out),
			}
		}
		20 => {
			let mut deque: VecDeque<u8> = bytes.iter().copied().collect();
			let mut v = bytes.to_vec();
			let heap: BinaryHeap<u8> = bytes.iter().copied().collect();
			let half = bytes.len() / 2;
			match arm % 4 {
				0 => walk(deque.iter(), steps, out),
				1 => walk(deque.drain(half..), steps, out),
				2 => walk(v.drain(..half), steps, out),
				_ => walk(heap.iter(), steps, out),
			}
		}
		// Adaptors of adaptors.
		21 => match arm % 4 {
			0 => walk_front((0..n).chain(0..m).skip(3).take(u32_of(n) as usize), steps, out),
			1 => walk(
				(0..u32_of(n.min(9))).zip(0..u32_of(m.min(9))).rev().step_by(2),
				steps,
				out,
			),
			2 => walk((0..n).map(|i| [i; 2]).flatten().chain(0..m), steps, out),
			_ => walk_front((0..n).filter(|_| true).peekable(), steps, out),
		},
		// Through `&mut`, which hints as what it borrows.
		22 => {
			let mut it = (0..n).chain(0..m);
			walk(&mut it, steps, out);
			put(out, it.size_hint());
		}
		_ => match arm % 3 {
			0 => walk_front((0..n).zip(0..m).map(|(a, _)| a).chain(std::iter::once(0)), steps, out),
			1 => walk_front(
				(0..u32_of(n)).map(|i| [i; 2]).flatten().skip(m as usize % 9),
				steps,
				out,
			),
			_ => walk_front((0..n).flat_map(|i| 0..i).take(m as usize), steps, out),
		},
	}
}

/// The most of each element type std's collections hold on wasm32, past which they panic
/// with a capacity overflow: a `Vec` of it, a `Vec` of it paired with a `u32`, a hash set
/// of it, and a hash map of it to a `u32`.
#[derive(Clone, Copy)]
struct Most {
	vec: u64,
	pairs: u64,
	set: u64,
	map: u64,
}

/// What `reserves` collects into, by `kind`'s quotient.
const COLLECTIONS: u8 = 13;

/// Each collection `kind` picks, collected or extended by `0..=last` of `u32`s, mapped by
/// a closure that panics with `consumed` past the call `data`'s third byte picks, the way
/// `data`'s first byte picks, and as long as its second picks: a few, one past the most it
/// holds, `u32::MAX`, and `usize::MAX` and one more, whose upper bound is `None`. std's
/// reserves by its size hint first, which panics past the most before it reads an item of
/// a `TrustedLen` iterator, and after the first of another, which it reserves for again
/// whenever it fills.
pub fn reserves(kind: u8, data: &[u8], out: &mut Vec<u8>) {
	match kind / COLLECTIONS % 6 {
		0 => typed(
			kind,
			data,
			|i| i as u8,
			Most {
				vec: 0x7fff_ffff,
				pairs: 0x0fff_ffff,
				set: 0x1c00_0000,
				map: 0x0700_0000,
			},
			out,
		),
		1 => typed(
			kind,
			data,
			|i| i,
			Most {
				vec: 0x1fff_ffff,
				pairs: 0x0fff_ffff,
				set: 0x0e00_0000,
				map: 0x0700_0000,
			},
			out,
		),
		2 => typed(
			kind,
			data,
			|i| (i as u8, 0u8, 0u8),
			Most {
				vec: 0x2aaa_aaaa,
				pairs: 0x0fff_ffff,
				set: 0x0e00_0000,
				map: 0x0700_0000,
			},
			out,
		),
		3 => typed(
			kind,
			data,
			|i| (u64::from(i), i as u8),
			Most {
				vec: 0x07ff_ffff,
				pairs: 0x0555_5555,
				set: 0x0380_0000,
				map: 0x0380_0000,
			},
			out,
		),
		// Zero-sized, of which a `Vec` holds `usize::MAX`.
		5 => typed(
			kind,
			data,
			|_| Unit,
			Most {
				vec: 0xffff_ffff,
				pairs: 0x1fff_ffff,
				set: 0x1fff_ffff,
				map: 0x0e00_0000,
			},
			out,
		),
		// Over 1024 bytes, which a `Vec` grows by as few as one of.
		_ => typed(
			kind,
			data,
			|i| [i; 275],
			Most {
				vec: 1_952_257,
				pairs: 1_945_184,
				set: 917_504,
				map: 917_504,
			},
			out,
		),
	}
}

/// Zero-sized, whose derived `clone` assigns nothing.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Unit;

/// `reserves` of elements `make` makes.
fn typed<T: Ord + Hash + Clone, F: Fn(u32) -> T>(kind: u8, data: &[u8], make: F, most: Most, out: &mut Vec<u8>) {
	let at = |i: usize| data.get(i).copied().unwrap_or(0);
	let coll = kind % COLLECTIONS;
	let limit = [0u8, 1, 4, 5, 8][usize::from(at(2)) % 5];
	// What the collection holds before it extends, and the most it holds of the items.
	let (held, most) = match coll {
		0 | 1 | 3 | 5 | 11 | 12 => (0, most.vec),
		2 | 4 | 6 => (2, most.vec),
		7 => (0, most.vec),
		8 => (0, most.set),
		9 => (1, most.map),
		_ => (0, most.pairs),
	};
	// The non-empty hash map reserves half what it is given, rounded up.
	let past = match coll {
		9 => 2 * (most - held) + 1,
		_ => most - held + 1,
	};
	let shape = at(0) % 5;
	let count = match at(1) % 4 {
		0 => 3,
		// One more past a `skip(1)`, which hints one fewer.
		1 => past + u64::from(shape == 1),
		2 => u64::from(u32::MAX),
		_ => 1 << 32,
	};
	let last = u32::try_from(count.min(1 << 32) - 1).unwrap();
	let seed = make(7);
	let calls = Cell::new(0u8);
	let f = |i: u32| {
		calls.set(calls.get() + 1);
		assert!(calls.get() <= limit, "consumed");
		make(i)
	};
	match shape {
		0 => fill(coll, (0..=last).map(f), seed, out),
		1 => fill(coll, (0..=last).map(f).skip(1), seed, out),
		2 => fill(coll, (0..=last).map(f).filter(|_| true), seed, out),
		// Its lower bound grows once it reads into the last range, which the collection
		// reserves for at the first item, or after it grows by the second range's.
		3 => fill(coll, [0..=2, 0..=last].into_iter().flatten().map(f), seed, out),
		_ => fill(coll, [0..=2, 0..=2, 0..=last].into_iter().flatten().map(f), seed, out),
	}
	out.push(calls.get());
}

/// `items` collected into the collection `coll` picks, or extended into one holding `seed`.
fn fill<T: Ord + Hash + Clone, I: Iterator<Item = T>>(coll: u8, items: I, seed: T, out: &mut Vec<u8>) {
	let len = match coll {
		0 => items.collect::<Vec<T>>().len(),
		1 => {
			let mut v = Vec::new();
			v.extend(items);
			v.len()
		}
		2 => {
			let mut v = vec![seed.clone(); 2];
			v.extend(items);
			v.len()
		}
		3 => items.collect::<VecDeque<T>>().len(),
		4 => {
			let mut q = VecDeque::from(vec![seed.clone(); 2]);
			q.extend(items);
			q.len()
		}
		5 => items.collect::<BinaryHeap<T>>().len(),
		6 => {
			let mut h = BinaryHeap::from(vec![seed.clone(); 2]);
			h.extend(items);
			h.len()
		}
		7 => items.collect::<BTreeSet<T>>().len(),
		8 => items.collect::<HashSet<T>>().len(),
		9 => {
			let mut m = HashMap::from([(seed, 0u32)]);
			m.extend(items.map(|t| (t, 1)));
			m.len()
		}
		10 => items.map(|t| (t, 1u32)).collect::<BTreeMap<T, u32>>().len(),
		// Through a shunt, whose length std's never trusts.
		11 => items.map(Some).collect::<Option<Vec<T>>>().map_or(0xff, |v| v.len()),
		_ => match items
			.enumerate()
			.map(|(i, t)| if i < 3 { Ok(t) } else { Err(i) })
			.collect::<Result<Vec<T>, usize>>()
		{
			Ok(v) => v.len(),
			Err(i) => 0x100 + i,
		},
	};
	out.extend_from_slice(&(len as u32).to_be_bytes());
}
