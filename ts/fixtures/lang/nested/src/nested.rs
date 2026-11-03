//! Options of values that can already be `undefined`, `Option<Option<u8>>` and
//! `Option<()>`, whose `None` is `rt.None` apart from each `Some`.

use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::hash::{DefaultHasher, Hash, Hasher};
use std::rc::Rc;
use std::sync::Arc;

/// Run case `kind % 8` over `data`, whose bytes pick each `None`, `Some(None)`, or
/// `Some(Some(_))`.
pub fn nested(kind: u8, data: &[u8], out: &mut Vec<u8>) {
	match kind % 8 {
		0 => made(data, out),
		1 => keys(data, out),
		2 => iters(data, out),
		3 => consumers(data, out),
		4 => panics(data, out),
		5 => closures(data, out),
		6 => elements(data, out),
		_ => units(data, out),
	}
}

/// `None` for a byte that is 0 mod 3, `Some(None)` for 1, and else `Some(Some(byte))`.
fn pick(b: u8) -> Option<Option<u8>> {
	match b % 3 {
		0 => None,
		1 => Some(None),
		_ => Some(Some(b)),
	}
}

/// `Some(())` for an even byte.
fn unit(b: u8) -> Option<()> {
	(b % 2 == 0).then_some(())
}

/// Which of the three `o` is, and its byte.
fn code(o: Option<Option<u8>>) -> u8 {
	match o {
		None => 0xe0,
		Some(None) => 0xe1,
		Some(Some(b)) => b,
	}
}

/// `?` on an `Option<()>` and on an `Option<Option<u8>>`, which return their `None`.
fn tried(a: Option<()>, b: Option<Option<u8>>) -> Option<Option<u8>> {
	a?;
	let inner = b?;
	Some(inner.map(|x| x / 2))
}

/// A generic type over a `T` that can be `undefined`, beside a field of no type parameter.
struct Tagged<T> {
	value: T,
	order: Ordering,
}

struct Slot {
	held: Option<Option<u8>>,
	flag: Option<()>,
}

fn made(data: &[u8], out: &mut Vec<u8>) {
	for &b in data {
		let o = pick(b);
		let u = unit(b);
		out.push(code(o));
		out.push(o.is_some() as u8 | (o.is_none() as u8) << 1 | (u.is_some() as u8) << 2 | (u.is_none() as u8) << 3);
		out.push(code(o.map(|x| x.map(|y| y ^ 0x55))));
		out.push(o.flatten().unwrap_or(0xf0));
		out.push(code(o.and_then(|x| x.map(|y| Some(y.wrapping_add(1))))));
		out.push(code(o.filter(|x| x.is_some())));
		out.push(code(o.or(Some(None))));
		out.push(code(o.xor(u.map(|()| None))));
		out.push(u.map_or(0xd0, |()| 0xd1));
		out.push(code(tried(u, o)));
		let copied = o.as_ref().copied();
		out.push((copied == o) as u8);
		let mut slot = Slot { held: o, flag: u };
		let taken = slot.held.take();
		let flag = std::mem::take(&mut slot.flag);
		out.push(code(taken) ^ code(slot.held));
		out.push(flag.is_some() as u8 | (slot.flag.is_some() as u8) << 1);
		let replaced = std::mem::replace(&mut slot.held, Some(Some(b)));
		out.push(code(replaced) ^ code(slot.held));
		let default: Option<Option<u8>> = Default::default();
		out.push(code(default));
		let tagged = Tagged {
			value: u,
			order: b.cmp(&0x80),
		};
		out.push(tagged.value.is_some() as u8 | order(tagged.order) << 1);
		out.extend_from_slice(format!("{o:?} {u:?}").as_bytes());
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

/// Compared, ordered, and hashed: `None` orders before `Some(None)`, which orders before
/// each `Some(Some(_))`.
fn keys(data: &[u8], out: &mut Vec<u8>) {
	let mut hashed: HashMap<(u8, Option<Option<u8>>), u8> = HashMap::new();
	let mut sorted: BTreeMap<(Option<Option<u8>>, u8), u8> = BTreeMap::new();
	for i in 1..data.len() {
		let (a, b) = (pick(data[i - 1]), pick(data[i]));
		out.push((a == b) as u8 | ((a != b) as u8) << 1 | ((a < b) as u8) << 2 | order(a.cmp(&b)) << 3);
		out.push((a.max(b) == b) as u8 | ((hash_of(&a) == hash_of(&b)) as u8) << 1);
		out.push(hashed.insert((data[i] % 2, a), i as u8).unwrap_or(0xcc));
		out.push(sorted.insert((a, data[i] % 2), i as u8).unwrap_or(0xcc));
		out.push(hashed.contains_key(&(data[i - 1] % 2, b)) as u8);
	}
	for ((o, n), v) in &sorted {
		out.push(code(*o));
		out.push(*n);
		out.push(*v);
	}
}

/// Counts down to 0, yielding each count as an `Option<u8>`: `None` for each odd one.
struct Halves(u8);

impl Iterator for Halves {
	type Item = Option<u8>;

	fn next(&mut self) -> Option<Option<u8>> {
		let n = self.0.checked_sub(1)?;
		self.0 = n;
		halve(n)
	}
}

fn halve(n: u8) -> Option<Option<u8>> {
	Some((n % 2 == 0).then_some(n / 2))
}

fn iters(data: &[u8], out: &mut Vec<u8>) {
	let items: Vec<Option<u8>> = data.iter().map(|&b| (b % 2 == 0).then_some(b)).collect();
	let at = data.first().map_or(0, |b| usize::from(*b) % 4);
	out.push(code(items.get(at).copied()));
	out.push(code(items.first().copied()));
	out.push(code((at > 1).then(|| items.get(at).copied().flatten())));
	let mut it = items.iter().copied();
	out.push(code(it.next()));
	out.push(code(it.next_back()));
	out.push(code(it.nth_back(1)));
	for item in it {
		out.push(item.unwrap_or(0xe1));
	}
	let mut own = Halves(data.len() as u8 % 8);
	let first = own.next();
	out.push(code(first));
	for item in own {
		out.push(item.map_or(0xe1, |x| x));
	}
	for b in data {
		let o = pick(*b);
		out.push(o.iter().count() as u8);
		o.into_iter().for_each(|x| out.push(x.unwrap_or(0xe1)));
		unit(*b).into_iter().for_each(|()| out.push(0xd1));
	}
}

/// Each consumer that returns an `Option` of the item, over items that are each `None`
/// for an odd byte: its `None` apart from its `Some(None)`.
fn consumers(data: &[u8], out: &mut Vec<u8>) {
	let items: Vec<Option<u8>> = data.iter().map(|&b| (b % 2 == 0).then_some(b)).collect();
	let n = data.first().map_or(0, |b| usize::from(*b) % 8);
	let it = || items.iter().copied();
	out.push(code(it().nth(n)));
	out.push(code(it().last()));
	out.push(code(it().rev().nth(n)));
	let mut tested = 0u8;
	out.push(code(it().find(|o| {
		tested += 1;
		o.is_none_or(|b| b > 0x80)
	})));
	out.push(tested);
	out.push(code(it().rfind(Option::is_none)));
	out.push(code(it().max()));
	out.push(code(it().min()));
	// By a key of ties, which std breaks to the last of the greatest and the first of the least.
	let key = |o: &Option<u8>| o.map(|b| b % 4);
	out.push(code(it().max_by_key(key)));
	out.push(code(it().min_by_key(key)));
	// By a key whose `None` is `rt.None` too.
	let nested = |o: &Option<u8>| o.map(|b| (b % 4 != 0).then_some(b % 3));
	out.push(code(it().max_by_key(nested)));
	out.push(code(it().min_by_key(nested)));
	out.push(code(items.iter().find(|o| o.is_none()).copied()));
	out.push(code(items.iter().max().copied()));
	let mut rest = it();
	out.push(code(rest.nth(n)));
	out.push(code(rest.next()));
	let mut peeked = it().peekable();
	out.push(code(peeked.peek().copied()));
	out.push(code(peeked.next_if(Option::is_none)));
	out.push(code(peeked.next()));
	// `None` is the least, so `max_by` keeps the last of the greatest and `min_by` the first `None`.
	out.push(code(it().max_by(|a, b| a.map(|b| b % 4).cmp(&b.map(|b| b % 4)))));
	out.push(code(it().min_by(Option::cmp)));
	out.push(code(it().max_by(|_, _| Ordering::Equal)));
	out.push(code(it().reduce(|a, b| a.xor(b))));
	let mut written = it().peekable();
	if let Some(o) = written.peek_mut() {
		*o = o.map_or(Some(0x42), |_| None);
	}
	out.push(code(written.next()));
	out.push(code(written.next_if_eq(&None)));
	out.push(code(written.next_if_eq(&Some(n as u8))));
	out.push(code(written.next()));
}

/// Each adaptor and source whose closure returns `pick`'s `Option<Option<u8>>` or `unit`'s
/// `Option<()>`: its `None` ends or skips, where its `Some(None)` is an item. Each count is
/// of the closure's calls.
fn closures(data: &[u8], out: &mut Vec<u8>) {
	let mut called = 0u8;
	let found = data.iter().find_map(|&b| {
		called += 1;
		pick(b)
	});
	out.extend([code(found), called]);
	out.push(data.iter().find_map(|&b| unit(b ^ 1)).map_or(0xd0, |()| 0xd1));
	called = 0;
	let kept: Vec<Option<u8>> = data
		.iter()
		.filter_map(|&b| {
			called += 1;
			pick(b)
		})
		.collect();
	out.extend(kept.iter().map(|&o| code(Some(o))));
	out.push(called);
	let back: Vec<Option<u8>> = data.iter().filter_map(|&b| pick(b)).rev().collect();
	out.extend(back.iter().map(|&o| code(Some(o))));
	// Through `fold`.
	let folded = data
		.iter()
		.filter_map(|&b| pick(b))
		.fold(0u8, |n, o| n.wrapping_mul(3).wrapping_add(code(Some(o))));
	out.push(folded);
	out.push(data.iter().filter_map(|&b| unit(b)).count() as u8);
	called = 0;
	let whiles: Vec<Option<u8>> = data
		.iter()
		.map_while(|&b| {
			called += 1;
			pick(b)
		})
		.collect();
	out.extend(whiles.iter().map(|&o| code(Some(o))));
	out.extend([whiles.len() as u8, called]);
	out.push(data.iter().map_while(|&b| unit(b)).count() as u8);
	// Not fused: `next` calls the closure again after its `None`.
	let mut rest = data.iter();
	called = 0;
	let mut made = std::iter::from_fn(|| {
		called += 1;
		pick(*rest.next()?)
	});
	let first: Vec<Option<u8>> = made.by_ref().collect();
	let after = made.next();
	out.extend(first.iter().map(|&o| code(Some(o))));
	out.extend([first.len() as u8, code(after), called]);
	let mut left = data.len();
	out.push(
		std::iter::from_fn(|| {
			left = left.checked_sub(1)?;
			unit(left as u8)
		})
		.count() as u8,
	);
	// From each `pick` of the first byte as the seed, each next from the bytes after.
	let mut seen = Vec::new();
	let mut rest = data.iter().skip(1);
	let walked: Vec<Option<u8>> = std::iter::successors(data.first().and_then(|&b| pick(b)), |&prev| {
		seen.push(code(Some(prev)));
		pick(*rest.next()?)
	})
	.collect();
	out.extend(walked.iter().map(|&o| code(Some(o))));
	out.extend(seen);
	let mut steps = 0u8;
	let units = std::iter::successors(Some(()), |()| {
		steps += 1;
		(steps < 3).then_some(())
	});
	out.extend([units.count() as u8, steps]);
	// `extend` reserves the size hint's lower bound, which is 1 while a `Some(())` or
	// `Some(None)` is pending: past `isize::MAX`, before it reads an item.
	if let [b @ 0xf0..=0xff] = *data {
		let mut s = String::new();
		let rest = (0..isize::MAX as usize).map(|_| -> char { panic!("read") });
		match b % 2 {
			0 => s.extend(std::iter::successors(Some(()), |()| None).map(|()| 'a').chain(rest)),
			_ => s.extend(
				std::iter::successors(Some(None::<u8>), |_| None)
					.map(|_| 'a')
					.chain(rest),
			),
		}
	}
}

/// Each accessor of an element of a `Vec`, a slice, a `VecDeque`, a `BinaryHeap`, a set, or
/// an `Arc` or `Rc`, over elements that are each `None` for an odd byte, and over `()`s,
/// then a map keyed by them: an empty collection's and an out-of-range `None` apart from a
/// `Some(None)`. The first byte is also the index.
fn elements(data: &[u8], out: &mut Vec<u8>) {
	let items: Vec<Option<u8>> = data.iter().map(|&b| (b % 2 == 0).then_some(b)).collect();
	let at = data.first().map_or(0, |b| usize::from(*b) % 8);
	let flip = |o: &mut Option<u8>| *o = o.map_or(Some(0x42), |_| None);
	let mut v = items.clone();
	out.extend([
		code(v.first().copied()),
		code(v.last().copied()),
		code(v.get(at).copied()),
	]);
	out.push(code(v[..].get(at..).and_then(|s| s.first()).copied()));
	v.first_mut().map(flip);
	v.last_mut().map(flip);
	v.get_mut(at).map(flip);
	out.extend([
		code(v.first().copied()),
		code(v.last().copied()),
		code(v.get(at).copied()),
	]);
	// `pop_if` lends the last element, changed in place whether or not its test takes it:
	// the first call keeps a `None` it made `Some`, which the second takes.
	for _ in 0..2 {
		out.push(code(v.pop_if(|o| {
			flip(o);
			o.is_none()
		})));
		out.push(code(v.last().copied()));
	}
	// Through each element, as `pop` reads past a `None` one.
	while let Some(o) = v.pop() {
		out.push(code(Some(o)));
	}
	out.extend([code(v.pop()), code(v.pop_if(|_| true))]);
	let mut units = vec![(); data.len()];
	out.extend([
		units.first().is_some() as u8,
		units.last().is_some() as u8,
		units.get(at).is_some() as u8,
	]);
	out.extend([
		units.first_mut().is_some() as u8,
		units.last_mut().is_some() as u8,
		units.get_mut(at).is_some() as u8,
	]);
	out.extend([units.pop_if(|()| at % 2 == 0).is_some() as u8, units.len() as u8]);
	let mut popped = 0u8;
	while let Some(()) = units.pop() {
		popped += 1;
	}
	out.push(popped);
	let mut q: VecDeque<Option<u8>> = VecDeque::new();
	for (i, &o) in items.iter().enumerate() {
		match i % 2 {
			0 => q.push_back(o),
			_ => q.push_front(o),
		}
	}
	out.extend([
		code(q.front().copied()),
		code(q.back().copied()),
		code(q.get(at).copied()),
	]);
	q.front_mut().map(flip);
	q.back_mut().map(flip);
	q.get_mut(at).map(flip);
	out.extend([
		code(q.front().copied()),
		code(q.back().copied()),
		code(q.get(at).copied()),
	]);
	out.extend([
		code(q.remove(at)),
		code(q.swap_remove_back(at)),
		code(q.swap_remove_front(at)),
	]);
	while let Some(o) = q.pop_front() {
		out.push(code(Some(o)));
		if let Some(o) = q.pop_back() {
			out.push(code(Some(o)));
		}
	}
	out.extend([
		code(q.pop_front()),
		code(q.pop_back()),
		code(q.front().copied()),
		code(q.remove(0)),
		code(q.swap_remove_back(0)),
		code(q.swap_remove_front(0)),
	]);
	let mut unit_q = VecDeque::from(vec![(); data.len() * 2]);
	out.extend([
		unit_q.front().is_some() as u8,
		unit_q.back().is_some() as u8,
		unit_q.get(at).is_some() as u8,
	]);
	out.extend([
		unit_q.pop_back().is_some() as u8,
		unit_q.remove(at).is_some() as u8,
		unit_q.swap_remove_back(at).is_some() as u8,
		unit_q.swap_remove_front(at).is_some() as u8,
		unit_q.pop_front().is_some() as u8,
		unit_q.len() as u8,
	]);
	// `None` is the least, so the heap yields each `None` last.
	let mut heap: BinaryHeap<Option<u8>> = items.iter().copied().collect();
	out.push(code(heap.peek().copied()));
	while let Some(o) = heap.pop() {
		out.push(code(Some(o)));
	}
	out.extend([code(heap.peek().copied()), code(heap.pop())]);
	let mut hashed: HashSet<Option<u8>> = HashSet::new();
	out.extend([code(hashed.get(&None).copied()), code(hashed.take(&None))]);
	out.extend([code(hashed.replace(None)), code(hashed.replace(None))]);
	hashed.extend(items.iter().copied());
	let probe = data.last().map(|&b| (b % 2 == 0).then_some(b));
	out.extend([
		code(hashed.get(&None).copied()),
		code(probe.and_then(|p| hashed.get(&p)).copied()),
	]);
	out.extend([
		code(hashed.take(&None)),
		code(hashed.take(&None)),
		code(hashed.get(&Some(1)).copied()),
	]);
	out.extend([code(hashed.replace(None)), code(probe.and_then(|p| hashed.replace(p)))]);
	out.push(hashed.len() as u8);
	let mut sorted: BTreeSet<Option<u8>> = items.iter().copied().collect();
	out.extend([code(sorted.first().copied()), code(sorted.last().copied())]);
	out.extend([
		code(sorted.get(&None).copied()),
		code(probe.and_then(|p| sorted.get(&p)).copied()),
		code(sorted.replace(None)),
		code(sorted.take(&None)),
		code(sorted.take(&None)),
		code(sorted.replace(None)),
	]);
	out.extend([code(sorted.pop_first()), code(sorted.pop_last())]);
	while let Some(o) = sorted.pop_last() {
		out.push(code(Some(o)));
	}
	out.extend([
		code(sorted.first().copied()),
		code(sorted.last().copied()),
		code(sorted.pop_first()),
		code(sorted.get(&None).copied()),
		code(sorted.take(&None)),
	]);
	// A shared one's value, from its last strong handle alone, which a `Weak` no longer
	// upgrades after.
	let shared = Arc::new(items.first().copied().unwrap_or(Some(0x33)));
	let other = Arc::clone(&shared);
	out.extend([code(Arc::into_inner(shared)), code(Arc::into_inner(other))]);
	let unit = Rc::new(());
	let weak = Rc::downgrade(&unit);
	let again = Rc::clone(&unit);
	out.extend([
		Rc::into_inner(unit).is_some() as u8,
		weak.upgrade().is_some() as u8,
		Rc::into_inner(again).is_some() as u8,
		weak.upgrade().is_some() as u8,
	]);
	// Keyed by them, through what a set shares with a map.
	let mut by: BTreeMap<Option<u8>, u8> = BTreeMap::new();
	let mut hash_by: HashMap<Option<u8>, u8> = HashMap::new();
	for (i, &o) in items.iter().enumerate() {
		*by.entry(o).or_default() += i as u8;
		hash_by.insert(o, i as u8);
	}
	for n in [by.get(&None), hash_by.get(&None), by.get(&Some(at as u8))] {
		out.push(n.copied().unwrap_or(0xcc));
	}
	out.push(hash_by.remove(&None).unwrap_or(0xcc));
	out.extend(
		by.first_key_value()
			.map(|(&k, &n)| [code(Some(k)), n])
			.unwrap_or([0xcc; 2]),
	);
	for (&k, &n) in &by {
		out.extend([code(Some(k)), n]);
	}
	out.push(hash_by.len() as u8);
}

fn panics(data: &[u8], out: &mut Vec<u8>) {
	for &b in data {
		out.push(pick(b).expect("picked none").unwrap_or(0xe1));
		out.push(code(Some(pick(b).unwrap())));
		unit(b ^ 1).unwrap();
	}
}

fn ignore(_: u8) {}

fn half(b: u8) -> u8 {
	b / 2
}

/// Each of `data` through `f`, read from its parameter: a function item too.
fn each<T>(data: &[u8], f: impl Fn(u8) -> T) -> Vec<T> {
	data.iter().copied().map(f).collect()
}

/// Each of `data` through a pointer.
fn pointed(data: &[u8], f: fn(u8)) -> Vec<()> {
	data.iter().copied().map(f).collect()
}

trait Made {
	type Out;
	fn made(&self, b: u8) -> Self::Out;
}

struct Unit;

impl Made for Unit {
	type Out = ();
	fn made(&self, _: u8) {}
}

fn made_all<M: Made>(m: &M, data: &[u8]) -> Vec<M::Out> {
	data.iter().map(|&b| m.made(b)).collect()
}

/// The `()` results of closures, functions, pointers, a `dyn Fn`, and a method returning an
/// associated type, as the items of a `Vec`, a `VecDeque`, an `Option`, and a `Result`.
fn units(data: &[u8], out: &mut Vec<u8>) {
	let units: Vec<()> = data.iter().map(|_| ()).collect();
	let q: VecDeque<()> = data.iter().rev().map(|_| ()).collect();
	let named: Vec<()> = data.iter().copied().map(ignore).collect();
	let f: fn(u8) = match data.first() {
		Some(b) if b % 2 == 0 => ignore,
		_ => |_| (),
	};
	let by_pointer = pointed(data, f);
	let generic = each(data, |_| ());
	let by_name = each(data, ignore);
	out.extend(each(data, half));
	let methods = made_all(&Unit, data);
	let g: &dyn Fn(u8) = &ignore;
	let dynamic: Vec<()> = data.iter().copied().map(g).collect();
	for v in [&units, &named, &by_pointer, &generic, &by_name, &methods, &dynamic] {
		out.extend([v.len() as u8, v.first().is_some() as u8, v.iter().count() as u8]);
	}
	out.extend([q.len() as u8, q.back().is_some() as u8]);
	let o: Option<()> = data.first().map(|_| ());
	let none: Option<()> = data.get(1).filter(|b| **b % 2 == 0).map(|_| ());
	let r: Result<(), u8> = data.first().copied().ok_or(0xee).map(|_| ());
	let e: Result<u8, ()> = data.first().copied().ok_or(()).map_err(|_| ());
	out.extend([
		o.is_some() as u8,
		none.is_some() as u8,
		r.is_ok() as u8,
		e.is_err() as u8,
	]);
	out.extend(format!("{units:?} {o:?} {none:?} {r:?} {e:?}").as_bytes());
}
