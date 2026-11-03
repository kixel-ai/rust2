//! Clones of std's iterators and adaptors, and of closures: each read on from where the
//! original was, independently of it, the two read in turns from either end after partial
//! reads, with what they own cloned and dropped as std's do. And clones of arrays, each
//! element by its own `Clone`.

use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque};
use std::rc::Rc;

/// Logs its id as it drops, and as it is cloned, the id with the high bit set: its clone's
/// id is one more.
struct Logged {
	id: u8,
	log: Rc<RefCell<Vec<u8>>>,
}

impl Logged {
	fn id(&self) -> u8 {
		self.id
	}
}

impl Clone for Logged {
	fn clone(&self) -> Self {
		self.log.borrow_mut().push(0x80 | self.id);
		Self {
			id: self.id.wrapping_add(1),
			log: self.log.clone(),
		}
	}
}

impl Drop for Logged {
	fn drop(&mut self) {
		self.log.borrow_mut().push(self.id);
	}
}

/// Puts what was logged since the last flush, after a marker.
fn flush(log: &RefCell<Vec<u8>>, out: &mut Vec<u8>) {
	out.push(0xfd);
	out.extend(log.borrow_mut().drain(..));
}

/// The crate's own iterator, which std's adaptors hold by value.
#[derive(Clone)]
struct Countdown(u8);

impl Iterator for Countdown {
	type Item = u8;

	fn next(&mut self) -> Option<u8> {
		self.0 = self.0.checked_sub(1)?;
		Some(self.0)
	}
}

/// What `turns` reads by: the case's bytes, each step's taken in order.
struct Plan<'a> {
	data: &'a [u8],
	at: usize,
}

impl Plan<'_> {
	fn next(&mut self) -> u8 {
		let b = self.data.get(self.at % self.data.len().max(1)).copied().unwrap_or(0);
		self.at += 1;
		b ^ (self.at as u8).wrapping_mul(29)
	}
}

/// `it` read up to three times from the front and then from the back by `back`, cloned, and
/// then the original and the clone read in turns, each from either end, as `plan` picks,
/// until both end: each item as `put` makes it, and each end as 0xff and which ended.
fn turns<I: Iterator + Clone>(
	mut it: I,
	back: impl Fn(&mut I) -> Option<I::Item>,
	plan: &mut Plan<'_>,
	put: impl Fn(I::Item) -> u8,
	out: &mut Vec<u8>,
) {
	for _ in 0..plan.next() % 4 {
		if let Some(item) = it.next() {
			out.push(put(item));
		}
	}
	for _ in 0..plan.next() % 4 {
		if let Some(item) = back(&mut it) {
			out.push(put(item));
		}
	}
	let mut other = it.clone();
	let hint = |it: &I| {
		let (lower, upper) = it.size_hint();
		[lower as u8, upper.map_or(0xfe, |n| n as u8)]
	};
	out.extend(hint(&it));
	out.extend(hint(&other));
	let (mut ended, mut other_ended) = (false, false);
	// Bounded, for an iterator that never ends.
	for _ in 0..64 {
		if ended && other_ended {
			break;
		}
		let step = plan.next();
		let first = step & 1 == 0 && !ended || other_ended;
		let item = match (first, step & 2 == 0) {
			(true, true) => it.next(),
			(true, false) => back(&mut it),
			(false, true) => other.next(),
			(false, false) => back(&mut other),
		};
		match item {
			Some(item) => out.push(put(item)),
			None => {
				match first {
					true => ended = true,
					false => other_ended = true,
				}
				out.extend([0xff, first as u8]);
			}
		}
	}
}

/// Each of std's iterators and adaptors, partly read and cloned, read in turns with its
/// clone.
pub fn iters(kind: u8, data: &[u8], out: &mut Vec<u8>) {
	let mut plan = Plan {
		data,
		at: usize::from(kind),
	};
	macro_rules! both {
		($it:expr, $put:expr) => {
			turns(
				$it,
				|it: &mut _| DoubleEndedIterator::next_back(it),
				&mut plan,
				$put,
				out,
			)
		};
	}
	macro_rules! front {
		($it:expr, $put:expr) => {
			turns($it, |it: &mut _| Iterator::next(it), &mut plan, $put, out)
		};
	}
	let id = |x: &u8| *x;
	let n = usize::from(data.first().map_or(1, |&b| b % 4 + 1));
	match kind % 16 {
		0 => {
			both!(data.iter(), id);
			both!(data.to_vec().into_iter(), |x| x);
			let mut array = [1u8, 2, 3, 4];
			for (a, &b) in array.iter_mut().zip(data) {
				*a = b;
			}
			both!(array.into_iter(), |x| x);
		}
		1 => {
			both!(data.windows(n), |w| w[0] ^ w[w.len() - 1]);
			both!(data.chunks(n), |c| c[0] ^ c.len() as u8);
			both!(data.chunks_exact(n), |c| c[0]);
			both!(data.rchunks(n), |c| c[0] ^ c.len() as u8);
			both!(data.rchunks_exact(n), |c| c[0]);
			// Arrays of bytes, which a borrow lends as slices of them.
			let arrays: Vec<[u8; 2]> = data.chunks_exact(2).map(|c| [c[0], c[1]]).collect();
			both!(arrays.iter(), |a| a[0] ^ a[1]);
		}
		2 => {
			let lo = data.first().copied().unwrap_or(0) % 8;
			let hi = data.get(1).copied().unwrap_or(9) % 16;
			both!(lo..hi, |x| x);
			both!(lo..=hi, |x| x);
			both!(u64::from(lo)..u64::from(hi), |x| x as u8);
			// `start..`, which never ends: bounded by `turns`' reads, short of `u8::MAX`.
			front!(lo.., |x| x);
			front!(u64::from(lo) << 53.., |x| (x >> 50) as u8 ^ x as u8);
			front!(char::from(b'a' + lo).., |c| c as u8);
			both!(data.first().into_iter(), id);
			both!(data.first().copied().into_iter(), |x| x);
			both!(std::iter::once(n as u8), |x| x);
			both!(std::iter::empty::<u8>(), |x| x);
		}
		// Text of one-, two-, three-, and four-byte chars, read mid-text.
		3 => {
			let text: String = data
				.iter()
				.map(|&b| ['a', 'é', '中', '🦀'][usize::from(b % 4)])
				.collect();
			both!(text.chars(), |c| (c as u32 % 251) as u8);
			both!(text.bytes(), |b| b);
			both!(text.split('é'), |s| s.len() as u8);
			front!(text.split("中"), |s| s.len() as u8);
			let mut seen = 0u8;
			both!(
				text.split(move |c: char| {
					seen = seen.wrapping_add(1);
					c == 'a' || seen % 3 == 0
				}),
				|s| s.len() as u8
			);
			both!(text.match_indices(['a', '🦀']), |(i, s)| i as u8 ^ s.len() as u8);
		}
		// Adaptors, from either end.
		4 => {
			both!(data.iter().map(|x| x ^ 1), |x| x);
			both!(data.iter().filter(|&&x| x % 3 != 0), id);
			both!(data.iter().filter_map(|&x| x.checked_sub(9)), |x| x);
			both!(data.iter().enumerate(), |(i, x)| i as u8 ^ x);
			both!(data.iter().rev(), id);
			both!(data.iter().skip(n), id);
			both!(data.iter().take(n * 2), id);
			both!(data.iter().step_by(n), id);
		}
		5 => {
			both!(data.iter().copied(), |x| x);
			both!(data.iter().cloned(), |x| x);
			both!(data.iter().fuse(), id);
			both!(data.iter().chain(data.iter().rev().skip(1)), id);
			both!(data.iter().peekable(), id);
			both!(data.chunks(n).flat_map(|c| c.iter()), id);
			both!(data.chunks(n).map(|c| c.to_vec()).flatten(), |x| x);
			both!(data.chunks(n).flatten(), id);
		}
		// Adaptors read only from the front.
		6 => {
			front!(data.iter().skip_while(|&&x| x % 4 != 0), id);
			front!(data.iter().take_while(|&&x| x % 5 != 0), id);
			front!(data.iter().map_while(|&x| x.checked_sub(3)), |x| x);
			front!(Countdown(n as u8 * 3).map(|x| x ^ 7), |x| x);
			front!(Countdown(n as u8 * 3).zip(data.iter()), |(a, b)| a ^ b);
			let mut left = n * 3;
			front!(
				std::iter::from_fn(move || {
					left = left.checked_sub(1)?;
					Some(left as u8)
				}),
				|x| x
			);
			front!(std::iter::successors(Some(n as u8 * 40), |&x| x.checked_sub(17)), |x| x);
		}
		// A zip, read past its shorter side before the clone, of iterators by index and not.
		7 => {
			let split = data.len() / 2;
			let (a, b) = data.split_at(split);
			let (a, b) = if kind & 32 == 0 { (a, b) } else { (b, a) };
			for skip in 0..2 {
				let mut zip = a.iter().zip(b.iter());
				for _ in 0..a.len().min(b.len()) + skip {
					zip.next();
				}
				both!(zip, |(x, y)| x ^ y);
				let mut zip = a.iter().filter(|_| true).zip(b.iter().filter(|_| true));
				for _ in 0..a.len().min(b.len()) + skip {
					zip.next();
				}
				front!(zip, |(x, y)| x ^ y);
			}
			both!(a.iter().zip(b.iter()), |(x, y)| x ^ y);
			front!(a.iter().zip(1..=7u8), |(x, y)| x ^ y);
			let log = RefCell::new(Vec::new());
			let mut zip = a.iter().map(|&x| {
				log.borrow_mut().push(x);
				x
			});
			let mut zip2 = zip.by_ref().zip(b.iter());
			for _ in 0..a.len().min(b.len()) + 1 {
				zip2.next();
			}
			drop(zip2);
			let clone = zip.clone();
			out.extend(clone.chain(zip).map(|x| x ^ 0x40));
			out.extend(log.take());
		}
		// A peeked item, and a peeked end.
		8 => {
			let mut it = data.iter().peekable();
			for _ in 0..n {
				out.push(it.peek().map_or(0xee, |&&x| x));
				it.next();
			}
			out.push(it.peek().map_or(0xee, |&&x| x));
			both!(it, id);
			let mut it = data.iter().copied().peekable();
			while it.next().is_some() {}
			out.push(it.peek().is_some() as u8);
			let mut clone = it.clone();
			out.push(clone.peek().is_some() as u8);
			both!(it, |x| x);
		}
		// B-tree iterators.
		9 => {
			let map: BTreeMap<u8, u8> = data.iter().map(|&b| (b % 32, b)).collect();
			let set: BTreeSet<u8> = data.iter().map(|&b| b % 32).collect();
			let lo = data.first().copied().unwrap_or(0) % 32;
			both!(map.iter(), |(k, v)| k ^ v);
			both!(map.keys(), id);
			both!(map.values(), id);
			both!(map.range(lo..), |(k, v)| k ^ v);
			both!(map.range(..=lo), |(k, v)| k ^ v);
			both!(set.iter(), id);
			both!(set.range(lo..), id);
		}
		// Hash iterators, whose order std leaves unspecified: each read the same as its clone.
		10 => {
			let map: HashMap<u8, u8> = data.iter().map(|&b| (b % 32, b)).collect();
			let set: HashSet<u8> = data.iter().map(|&b| b % 32).collect();
			unordered(map.iter().map(|(k, v)| k ^ v), &mut plan, out);
			unordered(map.keys().copied(), &mut plan, out);
			unordered(map.values().copied(), &mut plan, out);
			unordered(set.iter().copied(), &mut plan, out);
		}
		// Set operations, of sets small and large enough to look up rather than merge.
		11 => {
			let split = usize::from(data.first().copied().unwrap_or(0)) % (data.len() + 1);
			let a: BTreeSet<u8> = data[..split].iter().map(|&b| b % 64).collect();
			let b: BTreeSet<u8> = data[split..].iter().map(|&b| b % 64).collect();
			front!(a.union(&b), id);
			front!(a.intersection(&b), id);
			front!(b.intersection(&a), id);
			front!(a.difference(&b), id);
			front!(b.difference(&a), id);
			front!(a.symmetric_difference(&b), id);
			let a: HashSet<u8> = a.into_iter().collect();
			let b: HashSet<u8> = b.into_iter().collect();
			unordered(a.union(&b).copied(), &mut plan, out);
			unordered(a.intersection(&b).copied(), &mut plan, out);
			unordered(a.difference(&b).copied(), &mut plan, out);
			unordered(a.symmetric_difference(&b).copied(), &mut plan, out);
		}
		// Deques and heaps, borrowed and owned.
		12 => {
			// Split across the end of its buffer.
			let mut q = VecDeque::new();
			for &b in data {
				match b % 2 {
					0 => q.push_front(b),
					_ => q.push_back(b),
				}
			}
			both!(q.iter(), id);
			both!(q.clone().into_iter(), |x| x);
			let heap: BinaryHeap<u8> = data.iter().copied().collect();
			both!(heap.iter(), id);
			both!(heap.into_iter(), |x| x);
		}
		13 => owned(data, &mut plan, out),
		14 => closures(data, &mut plan, out),
		// Adaptors whose closures write their captures, so each clone counts on its own.
		_ => {
			let mut count = 0u8;
			both!(
				data.iter().map(move |&x| {
					count = count.wrapping_add(1);
					x ^ count
				}),
				|x| x
			);
			let mut seen = Vec::new();
			both!(
				data.iter().filter(move |&&x| {
					seen.push(x);
					seen.len() % 2 == 0
				}),
				id
			);
			let mut sum = 0u8;
			front!(
				data.iter().inspect(move |&&x| sum = sum.wrapping_add(x)).map(|&x| x),
				|x| x
			);
			let mut steps = 0u8;
			both!(
				data.chunks(n).flat_map(move |c| {
					steps += 1;
					c.iter().map(move |x| x ^ steps)
				}),
				|x| x
			);
		}
	}
}

/// `it` read up to three items, cloned, and both read to their ends: whether they read
/// the same, how many, and all it had, sorted, as std leaves a hash's order unspecified.
fn unordered<I: Iterator<Item = u8> + Clone>(mut it: I, plan: &mut Plan<'_>, out: &mut Vec<u8>) {
	let mut all: Vec<u8> = (0..plan.next() % 4).filter_map(|_| it.next()).collect();
	let other = it.clone();
	let (rest, other): (Vec<u8>, Vec<u8>) = (it.collect(), other.collect());
	out.extend([(rest == other) as u8, rest.len() as u8]);
	all.extend(rest);
	all.sort();
	out.extend(all);
}

/// Iterators that own what they yield, each item cloned by its own `Clone` as the iterator
/// is, and dropped as each iterator drops.
fn owned(data: &[u8], plan: &mut Plan<'_>, out: &mut Vec<u8>) {
	let log = Rc::new(RefCell::new(Vec::new()));
	let logged = |id: u8| Logged { id, log: log.clone() };
	let items: Vec<Logged> = data.iter().take(6).map(|&b| logged(b & 0x70)).collect();
	let mut it = items.into_iter();
	it.next();
	it.next_back();
	let clone = it.clone();
	flush(&log, out);
	drop(it);
	flush(&log, out);
	turns(clone, |it| it.next_back(), plan, |x| x.id(), out);
	flush(&log, out);

	let q: VecDeque<Logged> = data.iter().take(4).map(|&b| logged(b & 0x70)).collect();
	turns(q.into_iter(), |it| it.next_back(), plan, |x| x.id(), out);
	flush(&log, out);

	// Peeked, which clones after its iterator.
	let items: Vec<Logged> = data.iter().take(4).map(|&b| logged(b & 0x70)).collect();
	let mut it = items.into_iter().peekable();
	out.push(it.peek().map_or(0xee, Logged::id));
	let clone = it.clone();
	flush(&log, out);
	drop(clone);
	flush(&log, out);
	turns(it, |it| it.next_back(), plan, |x| x.id(), out);
	flush(&log, out);

	let it = std::iter::once(logged(1)).chain(Some(logged(2)));
	let clone = it.clone();
	flush(&log, out);
	drop(it);
	turns(clone, |it| it.next_back(), plan, |x| x.id(), out);
	flush(&log, out);

	// Each item of a flatten, and the inner iterator it holds at each end.
	let rows: Vec<Vec<Logged>> = data
		.chunks(3)
		.take(3)
		.map(|c| c.iter().map(|&b| logged(b & 0x70)).collect())
		.collect();
	let mut it = rows.into_iter().flatten();
	it.next();
	it.next_back();
	let clone = it.clone();
	flush(&log, out);
	drop(it);
	flush(&log, out);
	turns(clone, |it| it.next_back(), plan, |x| x.id(), out);
	flush(&log, out);

	let mut it = std::iter::successors(Some(logged(40)), |x| x.id().checked_sub(13).map(|id| logged(id)));
	it.next();
	let clone = it.clone();
	flush(&log, out);
	turns(clone, |it| it.next(), plan, |x| x.id(), out);
	drop(it);
	flush(&log, out);
}

/// Adaptors of closures that capture values that drop, which their clones clone and drop
/// with them, and closures cloned on their own.
fn closures(data: &[u8], plan: &mut Plan<'_>, out: &mut Vec<u8>) {
	let log = Rc::new(RefCell::new(Vec::new()));
	let logged = |id: u8| Logged { id, log: log.clone() };

	let (a, b) = (logged(1), logged(2));
	let it = data.iter().map(move |&x| x ^ a.id()).filter(move |&x| x != b.id());
	let clone = it.clone();
	flush(&log, out);
	drop(it);
	flush(&log, out);
	turns(clone, |it| it.next_back(), plan, |x| x, out);
	flush(&log, out);

	// Captured with state that changes and a value that drops, in that order.
	let (tag, mut count) = (logged(3), 0u8);
	let mut it = data.iter().map(move |&x| {
		count = count.wrapping_add(tag.id());
		x ^ count
	});
	it.next();
	let mut clone = it.clone();
	flush(&log, out);
	out.extend(it.next());
	out.extend(clone.next());
	out.extend(clone.next());
	out.extend(it.next());
	drop(clone);
	flush(&log, out);
	drop(it);
	flush(&log, out);

	// A closure that holds a clone of another, cloned on its own and called.
	let inner = logged(5);
	let mut total = 0u8;
	let mut add = move |x: u8| {
		total = total.wrapping_add(x ^ inner.id());
		total
	};
	out.push(add(1));
	let held = logged(6);
	let mut outer = move |x: u8| add(x) ^ held.id();
	let mut copy = outer.clone();
	flush(&log, out);
	out.extend([outer(2), copy(3), copy(4), outer(5)]);
	drop(outer);
	flush(&log, out);
	drop(copy);
	flush(&log, out);

	// A closure called once, which moves what it captured out, cloned first.
	let mut text = String::from("ab");
	let tag = logged(7);
	let once = move || {
		text.push('c');
		(text, tag)
	};
	let twice = once.clone();
	flush(&log, out);
	let (text, tag) = once();
	out.extend(text.bytes().chain([tag.id()]));
	drop(tag);
	let (text, tag) = twice();
	out.extend(text.bytes().chain([tag.id()]));
	drop(tag);
	flush(&log, out);

	// A closure that shares what it captured, each clone of which is the closure.
	let k = data.len() as u8;
	let shared = move |x: &u8| x.wrapping_mul(k);
	let it = data.iter().map(shared);
	let clone = it.clone().rev();
	out.extend(it.zip(clone).map(|(x, y)| x ^ y));
}

/// Clones of the bytes' ids, which panic, saying which, on one of them.
struct Fussy(u8);

impl Clone for Fussy {
	fn clone(&self) -> Self {
		if self.0 % 3 == 0 {
			panic!("cloning {}", self.0);
		}
		Fussy(self.0)
	}
}

/// A derived `Clone`, which clones each field in order.
#[derive(Clone)]
struct Pair {
	left: [Logged; 2],
	tag: String,
}

/// `x` cloned where only its bound says it clones.
fn cloned<T: Clone>(x: &T) -> T {
	x.clone()
}

/// Arrays of values that clone and drop themselves, each element cloned in order as std's
/// are: by `clone`, `to_owned`, and `clone_from` and `clone_into`, which clone each element
/// over its own and drop what that wrote over, of none, nested, and as an `Option`, a tuple,
/// a derived `Clone`, a `Vec`, and a generic function clone theirs.
pub fn arrays(kind: u8, data: &[u8], out: &mut Vec<u8>) {
	let log = Rc::new(RefCell::new(Vec::new()));
	let logged = |id: u8| Logged { id, log: log.clone() };
	let at = |i: usize| data.get(i).copied().unwrap_or((i as u8).wrapping_mul(0x30)) & 0x7e;
	let three = || [logged(at(0)), logged(at(1)), logged(at(2))];
	let ids = |a: &[Logged], out: &mut Vec<u8>| out.extend(a.iter().map(Logged::id));
	match kind % 8 {
		0 => {
			let a = three();
			let b = a.clone();
			flush(&log, out);
			ids(&b, out);
			drop(a);
			flush(&log, out);
			let c = b.to_owned();
			drop(b);
			flush(&log, out);
			let none: [Logged; 0] = [];
			let other = none.clone();
			ids(&c, out);
			drop((none, other, c));
			flush(&log, out);
		}
		1 => {
			let mut a = three();
			let b = [logged(at(3)), logged(at(4)), logged(at(5))];
			a.clone_from(&b);
			flush(&log, out);
			ids(&a, out);
			let mut c = [logged(at(6)), logged(at(7)), logged(at(8))];
			b.clone_into(&mut c);
			flush(&log, out);
			ids(&c, out);
			drop((a, b, c));
			flush(&log, out);
		}
		2 => {
			let some = Some([logged(at(0)), logged(at(1))]);
			let none: Option<[Logged; 2]> = None;
			let (x, y) = (some.clone(), none.clone());
			flush(&log, out);
			out.push(y.is_none() as u8);
			ids(x.as_ref().unwrap(), out);
			drop(some);
			flush(&log, out);
			drop(x);
			flush(&log, out);
		}
		3 => {
			let tuple = ([logged(at(0)), logged(at(1))], String::from("t"), logged(at(2)));
			let copy = tuple.clone();
			flush(&log, out);
			ids(&copy.0, out);
			drop(tuple);
			flush(&log, out);
			let pair = Pair {
				left: [logged(at(3)), logged(at(4))],
				tag: format!("{}", at(5)),
			};
			let other = pair.clone();
			flush(&log, out);
			ids(&other.left, out);
			out.extend(other.tag.bytes());
			drop((copy, pair));
			flush(&log, out);
			drop(other);
			flush(&log, out);
		}
		4 => {
			let grid = [[logged(at(0)), logged(at(1))], [logged(at(2)), logged(at(3))]];
			let copy = grid.clone();
			flush(&log, out);
			drop(grid);
			flush(&log, out);
			ids(&copy[1], out);
			let words = [String::from("a"), at(4).to_string(), String::new()];
			let vecs = [data.to_vec(), vec![at(5)]];
			let (words2, mut vecs2) = (words.clone(), vecs.clone());
			vecs2[0].push(1);
			out.extend(words2.concat().bytes());
			out.extend(vecs.iter().chain(&vecs2).flatten());
			drop(copy);
			flush(&log, out);
		}
		5 => {
			let rows = vec![[logged(at(0)), logged(at(1))], [logged(at(2)), logged(at(3))]];
			let copy = rows.clone();
			flush(&log, out);
			let generic = cloned(&copy[0]);
			flush(&log, out);
			let mut sum = 0u8;
			for row in rows.iter().cloned() {
				sum = sum.wrapping_add(row[0].id() ^ row[1].id());
			}
			out.push(sum);
			flush(&log, out);
			drop((rows, copy, generic));
			flush(&log, out);
		}
		// Panics on the first element whose clone does: those before cloned in order.
		_ => {
			let fussy = [Fussy(at(0)), Fussy(at(1)), Fussy(at(2))];
			let copy = fussy.clone();
			out.extend(copy.iter().map(|f| f.0));
		}
	}
}

/// Clones over itself in place, logging 0xfc and the id it takes, and keeps its own
/// `Logged`.
struct Kept(Logged);

impl Clone for Kept {
	fn clone(&self) -> Self {
		Kept(self.0.clone())
	}

	fn clone_from(&mut self, source: &Self) {
		self.0.log.borrow_mut().extend([0xfc, source.0.id]);
		self.0.id = source.0.id;
	}
}

/// A [`Kept`] or none, whose `clone_from` keeps a `Held` and otherwise clones.
enum Slot {
	Held(Kept),
	Empty,
}

impl Clone for Slot {
	fn clone(&self) -> Self {
		match self {
			Slot::Held(kept) => Slot::Held(kept.clone()),
			Slot::Empty => Slot::Empty,
		}
	}

	fn clone_from(&mut self, source: &Self) {
		match (self, source) {
			(Slot::Held(to), Slot::Held(from)) => to.clone_from(from),
			(to, from) => *to = from.clone(),
		}
	}
}

/// `clone_from` and `clone_into` of containers of values that drop, each in std's order:
/// an `Option`'s of two `Some`s is what it holds', a `Vec`'s truncates, clones over the
/// elements it keeps, and extends, and a slice's of arrays clones each over its own.
pub fn clone_froms(kind: u8, data: &[u8], out: &mut Vec<u8>) {
	let log = Rc::new(RefCell::new(Vec::new()));
	let logged = |id: u8| Logged { id, log: log.clone() };
	let at = |i: usize| data.get(i).copied().unwrap_or((i as u8).wrapping_mul(0x30)) & 0x7e;
	// Lengths of 0 to 3, unequal both ways.
	let len = |i: usize| usize::from(data.get(i).copied().unwrap_or(i as u8) % 4);
	let row = |from: usize, n: usize| (0..n).map(|i| logged(at(from + i))).collect::<Vec<_>>();
	let ids = |a: &[Logged], out: &mut Vec<u8>| out.extend(a.iter().map(Logged::id));
	match kind % 8 {
		0 => {
			let pick =
				|i: usize| (data.get(i).copied().unwrap_or(1) % 2 == 1).then(|| [logged(at(i)), logged(at(i + 1))]);
			let mut a = pick(0);
			let b = pick(2);
			flush(&log, out);
			a.clone_from(&b);
			flush(&log, out);
			out.push(a.is_some() as u8);
			if let Some(a) = &a {
				ids(a, out);
			}
			let mut c = pick(4);
			a.clone_into(&mut c);
			flush(&log, out);
			drop((a, b, c));
			flush(&log, out);
		}
		1 => {
			let pick = |i: usize| (data.get(i).copied().unwrap_or(1) % 5 != 0).then(|| row(i, len(i)));
			let mut a = pick(0);
			let b = pick(4);
			flush(&log, out);
			a.clone_from(&b);
			flush(&log, out);
			if let Some(a) = &a {
				ids(a, out);
			}
			drop((a, b));
			flush(&log, out);
		}
		2 => {
			let mut a = row(0, len(0));
			a.reserve(len(1));
			let b = row(4, len(4));
			flush(&log, out);
			a.clone_from(&b);
			flush(&log, out);
			ids(&a, out);
			out.push(a.capacity() as u8);
			let mut c = row(8, len(8));
			b.clone_into(&mut c);
			flush(&log, out);
			ids(&c, out);
			b[..].clone_into(&mut a);
			flush(&log, out);
			drop((a, b, c));
			flush(&log, out);
		}
		3 => {
			let mut grid = [[logged(at(0)), logged(at(1))], [logged(at(2)), logged(at(3))]];
			let other = [[logged(at(4)), logged(at(5))], [logged(at(6)), logged(at(7))]];
			flush(&log, out);
			grid.clone_from(&other);
			flush(&log, out);
			ids(&grid[1], out);
			other.clone_into(&mut grid);
			flush(&log, out);
			drop((grid, other));
			flush(&log, out);
		}
		4 => {
			let mut rows = vec![[logged(at(0)), logged(at(1))], [logged(at(2)), logged(at(3))]];
			let other = [[logged(at(4)), logged(at(5))], [logged(at(6)), logged(at(7))]];
			flush(&log, out);
			rows.clone_from_slice(&other);
			flush(&log, out);
			rows[..1].fill(other[1].clone());
			flush(&log, out);
			rows.fill([logged(at(8)), logged(at(9))]);
			flush(&log, out);
			ids(&rows[0], out);
			let mut none: [[Logged; 2]; 0] = [];
			none.fill([logged(at(10)), logged(at(11))]);
			flush(&log, out);
			drop((rows, other));
			flush(&log, out);
		}
		5 => {
			let mut rows = vec![row(0, len(0)), row(4, len(4))];
			let other = vec![row(8, len(8)), row(12, len(12)), row(16, len(16))];
			let mut cut: Vec<Vec<Logged>> = (0..4).map(|i| row(20 + i, len(20 + i))).collect();
			flush(&log, out);
			rows.clone_from(&other);
			flush(&log, out);
			cut.clone_from(&other);
			flush(&log, out);
			ids(&cut[0], out);
			rows.truncate(len(1));
			let n = rows.len();
			rows.clone_from_slice(&other[..n]);
			flush(&log, out);
			drop((rows, other, cut));
			flush(&log, out);
		}
		6 => {
			let mut a: Box<[Logged]> = row(0, len(0)).into();
			let b: Box<[Logged]> = row(4, len(4)).into();
			flush(&log, out);
			a.clone_from(&b);
			flush(&log, out);
			ids(&a, out);
			let ones = |from: usize, n: usize| (0..n).map(|i| [logged(at(from + i))]).collect::<Box<[_]>>();
			let mut c = ones(8, len(8));
			let d = ones(12, len(12));
			flush(&log, out);
			c.clone_from(&d);
			flush(&log, out);
			drop((a, b, c, d));
			flush(&log, out);
		}
		// Elements whose `clone_from` is their own, of an object and of a value a `&mut`
		// lends as a place.
		_ => {
			let kept = |i: usize| Kept(logged(at(i)));
			let mut a = vec![kept(0), kept(1)];
			a.truncate(len(0));
			let b: Vec<Kept> = (0..len(1)).map(|i| kept(2 + i)).collect();
			flush(&log, out);
			a.clone_from(&b);
			flush(&log, out);
			let mut grid = [[kept(5), kept(6)]];
			grid.clone_from(&[[kept(7), kept(8)]]);
			flush(&log, out);
			let mut some = Some(kept(9));
			some.clone_from(&Some(kept(10)));
			flush(&log, out);
			let slot = |i: usize| match data.get(i).copied().unwrap_or(0) % 3 {
				0 => Slot::Empty,
				_ => Slot::Held(kept(i)),
			};
			let mut slots = [slot(11), slot(12)];
			slots.clone_from(&[slot(13), slot(14)]);
			flush(&log, out);
			let mut list: LinkedList<Kept> = (0..len(15)).map(|i| kept(16 + i)).collect();
			let other: LinkedList<Kept> = (0..len(20)).map(|i| kept(21 + i)).collect();
			flush(&log, out);
			list.clone_from(&other);
			flush(&log, out);
			drop((a, b, grid, some, slots, list, other));
			flush(&log, out);
		}
	}
}
