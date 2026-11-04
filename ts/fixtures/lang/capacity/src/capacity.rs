//! The capacity std's `Vec`, `VecDeque`, and `BinaryHeap` hold as each step of a case
//! grows, shrinks, remakes, or reads one, of elements of each size its growth tells apart:
//! a byte, at most 1 KiB, past it, and none. Each it checks exactly past every step, as a
//! `Vec`'s decides when a push past a pop reallocates, and a `VecDeque`'s where its slices
//! split. A `HashMap`'s and a `HashSet`'s it checks only by std's guarantees.

use std::cell::RefCell;
use std::collections::{BinaryHeap, HashMap, HashSet, LinkedList, VecDeque};
use std::mem;

/// A length or capacity, as four bytes.
fn put(out: &mut Vec<u8>, n: usize) {
	out.extend_from_slice(&(n as u32).to_le_bytes());
}

/// A length and a capacity, as four bytes each.
fn sized(out: &mut Vec<u8>, len: usize, cap: usize) {
	put(out, len);
	put(out, cap);
}

/// How many to reserve by `arg`, beside `len` of `size`-byte elements: a few, or one to
/// eight past what a vector of them may hold, or within eight of `usize::MAX`, either of
/// which panics before it allocates.
fn amount(arg: u8, len: usize, size: usize) -> usize {
	match arg >> 5 {
		7 => usize::MAX - usize::from(arg & 7),
		6 if size > 0 => isize::MAX as usize / size - len + 1 + usize::from(arg & 7),
		_ => usize::from(arg & 0x1f),
	}
}

/// A test that keeps every other element it is asked about.
fn alternate() -> impl FnMut() -> bool {
	let mut kept = false;
	move || {
		kept = !kept;
		kept
	}
}

/// What the element type `kind` picks makes of a byte.
macro_rules! by_kind {
	($kind:expr, $f:ident, $data:expr, $out:expr) => {
		match $kind % 5 {
			0 => $f($data, |b| b, $out),
			1 => $f($data, u16::from, $out),
			2 => $f($data, |b| (u64::from(b), b), $out),
			3 => $f($data, |b| [u64::from(b); 160], $out),
			_ => $f($data, |_| (), $out),
		}
	};
}

/// A `Vec` that `data`'s first byte makes, by `with_capacity` if odd, and each pair after
/// it changes, its length and capacity after each.
pub fn vecs(kind: u8, data: &[u8], out: &mut Vec<u8>) {
	by_kind!(kind, vec_steps, data, out);
}

fn vec_steps<T: Clone + Ord>(data: &[u8], make: fn(u8) -> T, out: &mut Vec<u8>) {
	let size = size_of::<T>();
	let (first, steps) = data.split_first().map_or((0, &[][..]), |(&b, rest)| (b, rest));
	let mut v: Vec<T> = match first & 1 {
		1 => Vec::with_capacity(usize::from(first >> 3)),
		_ => Vec::new(),
	};
	put(out, v.capacity());
	for step in steps.chunks(2) {
		let (op, arg) = (step[0], step.get(1).copied().unwrap_or(0));
		match op % 41 {
			0 | 1 => v.push(make(arg)),
			2 => drop(v.pop()),
			3 | 4 => {
				let additional = amount(arg, v.len(), size);
				match op % 41 {
					3 => v.reserve(additional),
					_ => v.reserve_exact(additional),
				}
			}
			5 => v.shrink_to(usize::from(arg % 40)),
			6 => v.shrink_to_fit(),
			7 => v.truncate(usize::from(arg % 8)),
			8 => v.insert(usize::from(arg) % (v.len() + 1), make(arg)),
			9 if !v.is_empty() => drop(v.remove(usize::from(arg) % v.len())),
			// Not `TrustedLen`, so it reserves only once full, and a `TrustedLen` one.
			10 => v.extend((0..arg % 24).filter(|i| i % 3 != 0).map(make)),
			11 => v.extend((0..arg % 24).map(make)),
			// Room for the whole slice at once, which pushes would double to past it.
			12 => v.extend_from_slice(&vec![make(arg); usize::from(arg % 24)]),
			13 => v.clear(),
			14 => {
				let mut keep = alternate();
				v.retain(|_| keep());
			}
			15 => drop(v.drain(..v.len().min(usize::from(arg % 4)))),
			16 => v = v.clone(),
			17 => v = v.into_boxed_slice().into_vec(),
			// In place of its own, and of its own once read from either end.
			18 => {
				let mut keep = alternate();
				v = v.into_iter().filter(|_| keep()).collect();
			}
			19 => {
				let mut it = v.into_iter();
				for i in 0..arg % 8 {
					drop(if i % 3 == 2 { it.next_back() } else { it.next() });
				}
				v = it.collect();
			}
			20 => {
				let mut taken = mem::take(&mut v);
				taken.push(make(arg));
				sized(out, taken.len(), taken.capacity());
				v = mem::replace(&mut taken, Vec::with_capacity(usize::from(arg % 6)));
				sized(out, taken.len(), taken.capacity());
			}
			21 if !v.is_empty() => drop(v.swap_remove(usize::from(arg) % v.len())),
			22 => v.dedup(),
			// A clone of one read from the front, a new `Vec` of what was left.
			23 => {
				let mut it = v.into_iter();
				for _ in 0..arg % 4 {
					drop(it.next());
				}
				v = it.clone().collect();
				let rest = it.collect::<Vec<T>>();
				sized(out, rest.len(), rest.capacity());
			}
			24 => v = vec![make(arg); usize::from(arg % 6)],
			25 => drop(v.pop_if(|_| arg % 2 == 1)),
			// Joined, with room for exactly what it joins.
			26 => v = [v.clone(), v].concat(),
			// A boxed slice's `into_iter` reads a `Vec` with room for exactly its elements.
			27 => v = v.into_boxed_slice().into_iter().collect(),
			// Into one with room of its own, which std's keeps where it is enough.
			28 => {
				let mut own = Vec::with_capacity(usize::from(arg % 8));
				own.push(make(arg));
				own.clone_from(&v);
				v = own;
			}
			// Room for what it takes, leaving the other whatever it had.
			29 => {
				let mut other = Vec::with_capacity(usize::from(arg % 8));
				other.extend((0..arg % 12).map(make));
				v.append(&mut other);
				sized(out, other.len(), other.capacity());
			}
			// Room for the rest at once, by a few or past what it may hold, or cut.
			30 | 31 => {
				let new_len = match arg >> 5 {
					6 if size > 0 => isize::MAX as usize / size + 1 + usize::from(arg & 7),
					_ => usize::from(arg % 24),
				};
				match op % 41 {
					30 => v.resize(new_len, make(arg)),
					_ => v.resize_with(new_len, || make(arg)),
				}
			}
			// The rest in a new `Vec` with room for exactly them.
			32 => {
				let tail = v.split_off(usize::from(arg) % (v.len() + 1));
				sized(out, tail.len(), tail.capacity());
			}
			33 => v.extend_from_within(usize::from(arg) % (v.len() + 1)..),
			// Through a deque, which keeps its buffer both ways.
			34 => v = Vec::from(VecDeque::from(v)),
			// Room for the tail past what fills the range, reserved for the whole vector:
			// by a lower bound that is exact, short, or none, and by the rest once counted,
			// or by `extend` of a range to the end.
			35 => {
				let len = v.len();
				let start = usize::from(arg & 3) % (len + 1);
				let end = match arg & 0x80 {
					0 => start + usize::from(arg >> 2 & 1) % (len - start + 1),
					_ => len,
				};
				let n = [0, 2, 6, 17][usize::from(arg >> 5 & 3)];
				match arg >> 3 & 3 {
					0 => drop(v.splice(start..end, (0..n as u8).map(make))),
					1 => drop(v.splice(start..end, (0..n as u8).filter(|i| i % 4 != 1).map(make))),
					2 => drop(v.splice(
						start..end,
						(0..n as u8).chain((0..n as u8).filter(|i| i % 2 == 0)).map(make),
					)),
					_ => drop(v.splice(start..end, vec![make(arg); n])),
				}
			}
			36 => {
				let mut keep = alternate();
				let start = usize::from(arg) % (v.len() + 1);
				let _ = v.extract_if(start.., |_| keep()).take(usize::from(arg % 5)).count();
			}
			// Pairs, flattened into room for twice as many as there was for pairs.
			37 => {
				let mut pairs: Vec<[T; 2]> = Vec::with_capacity(usize::from(arg % 6));
				for c in v.chunks_exact(2) {
					pairs.push([c[0].clone(), c[1].clone()]);
				}
				let flat = pairs.into_flattened();
				sized(out, flat.len(), flat.capacity());
			}
			// A splice leaked before it drops keeps the vector's capacity.
			38 => {
				let start = usize::from(arg) % (v.len() + 1);
				std::mem::forget(v.splice(start.., []));
			}
			// Room as `reserve` makes it, or the error where it panics, which leaves the
			// capacity as it was.
			39 | 40 => {
				let additional = amount(arg, v.len(), size);
				let tried = match op % 41 {
					39 => v.try_reserve(additional),
					_ => v.try_reserve_exact(additional),
				};
				out.extend(format!("{tried:?}").bytes());
			}
			_ => {}
		}
		sized(out, v.len(), v.capacity());
	}
}

/// A struct whose derived `clone_from` clones.
#[derive(Clone)]
struct Holds<T>(Vec<T>);

/// The length and capacity of each `Vec` that `clone_from` writes over in each container
/// std's reaches it through: kept, grown only where it holds fewer, where std's clones
/// over each `Vec` it holds, and a clone's where a tuple's, a derived struct's, or a
/// deque's clones it anew.
pub fn clone_froms(kind: u8, data: &[u8], out: &mut Vec<u8>) {
	by_kind!(kind, clone_from_steps, data, out);
}

fn clone_from_steps<T: Clone + Ord>(data: &[u8], make: fn(u8) -> T, out: &mut Vec<u8>) {
	let at = |i: usize| {
		data.get(i)
			.copied()
			.unwrap_or((i as u8).wrapping_mul(0x35) ^ data.len() as u8)
	};
	// Room for more than it holds: by `with_capacity` if odd, else by pushes.
	let held = |i: usize| {
		let b = at(i);
		let mut v = match b & 1 {
			1 => Vec::with_capacity(usize::from(b >> 4)),
			_ => Vec::new(),
		};
		for j in 0..(b >> 1) % 6 {
			v.push(make(j));
		}
		v
	};
	let source = |i: usize| (0..at(i) % 10).map(make).collect::<Vec<T>>();
	let len = |i: usize| usize::from(at(i) % 4);
	let holds = |from: usize, n: usize| (0..n).map(|j| held(from + j)).collect::<Vec<_>>();
	let sources = |from: usize, n: usize| (0..n).map(|j| source(from + j)).collect::<Vec<_>>();
	let caps = |vs: &[Vec<T>], out: &mut Vec<u8>| {
		put(out, vs.len());
		for v in vs {
			put(out, v.len());
			put(out, v.capacity());
		}
	};
	// An array's and a slice's, each over its own, and `fill`'s, each but the last.
	let mut array = [held(0), held(1)];
	array.clone_from(&[source(2), source(3)]);
	caps(&array, out);
	let mut slice = [held(4), held(5), held(6)];
	slice[1..].clone_from_slice(&[source(7), source(8)]);
	caps(&slice, out);
	slice.fill(source(9));
	caps(&slice, out);
	// A `Vec`'s, over those it keeps, as a slice's `clone_into` one.
	let mut nested = holds(11, len(10));
	nested.clone_from(&sources(15, len(14)));
	caps(&nested, out);
	let mut into = holds(19, len(18));
	sources(23, len(22))[..].clone_into(&mut into);
	caps(&into, out);
	// A box's is what it holds', and a boxed slice's its elements' where the lengths match.
	let mut boxed = Box::new(held(26));
	boxed.clone_from(&Box::new(source(27)));
	caps(&[*boxed], out);
	let mut boxes: Box<[Vec<T>]> = holds(29, len(28)).into();
	boxes.clone_from(&sources(33, len(32)).into());
	caps(&boxes, out);
	// Of two `Some`s, and of two `Ok`s or two `Err`s, what they hold.
	let mut option = (at(36) % 3 != 0).then(|| held(37));
	option.clone_from(&(at(38) % 3 != 0).then(|| source(39)));
	caps(&option.into_iter().collect::<Vec<_>>(), out);
	let mut result: Result<Vec<T>, Vec<T>> = match at(40) % 2 {
		0 => Ok(held(41)),
		_ => Err(held(42)),
	};
	result.clone_from(&match at(43) % 2 {
		0 => Ok(source(44)),
		_ => Err(source(45)),
	});
	out.push(u8::from(result.is_ok()));
	caps(&[result.unwrap_or_else(|v| v)], out);
	let mut cell = RefCell::new(held(46));
	cell.clone_from(&RefCell::new(source(47)));
	caps(&[cell.into_inner()], out);
	let mut deep = Some([Box::new(held(52))]);
	deep.clone_from(&Some([Box::new(source(53))]));
	caps(&deep.map(|[b]| *b).into_iter().collect::<Vec<_>>(), out);
	// A list's, over each it keeps; a deque's, which clears and clones.
	let mut list: LinkedList<Vec<T>> = holds(55, len(54)).into_iter().collect();
	list.clone_from(&sources(59, len(58)).into_iter().collect());
	caps(&list.into_iter().collect::<Vec<_>>(), out);
	let mut deque: VecDeque<Vec<T>> = holds(63, len(62)).into();
	deque.clone_from(&sources(67, len(66)).into());
	caps(&Vec::from(deque), out);
	// A heap's is its vector's, and a deque's keeps its own buffer, each in an array too.
	let mut heap = BinaryHeap::from(held(70));
	heap.clone_from(&BinaryHeap::from(source(71)));
	caps(&[heap.into_vec()], out);
	let mut heaps = [BinaryHeap::from(held(72))];
	heaps.clone_from(&[BinaryHeap::from(source(73))]);
	put(out, heaps[0].capacity());
	let mut deques = [VecDeque::from(held(74))];
	deques.clone_from(&[VecDeque::from(source(75))]);
	put(out, deques[0].capacity());
	// A tuple's and a derived struct's clone.
	let mut pair = (held(76), 0u8);
	pair.clone_from(&(source(77), 1));
	let mut holder = Holds(held(78));
	holder.clone_from(&Holds(source(79)));
	caps(&[pair.0, holder.0], out);
}

/// A `VecDeque`, as `vecs` steps a `Vec`.
pub fn deques(kind: u8, data: &[u8], out: &mut Vec<u8>) {
	by_kind!(kind, deque_steps, data, out);
}

fn deque_steps<T: Clone + Ord>(data: &[u8], make: fn(u8) -> T, out: &mut Vec<u8>) {
	let size = size_of::<T>();
	let (first, steps) = data.split_first().map_or((0, &[][..]), |(&b, rest)| (b, rest));
	let mut q: VecDeque<T> = match first & 1 {
		1 => VecDeque::with_capacity(usize::from(first >> 3)),
		_ => VecDeque::new(),
	};
	put(out, q.capacity());
	for step in steps.chunks(2) {
		let (op, arg) = (step[0], step.get(1).copied().unwrap_or(0));
		match op % 28 {
			0 => q.push_back(make(arg)),
			1 => q.push_front(make(arg)),
			2 => drop(q.pop_front()),
			3 | 4 => {
				let additional = amount(arg, q.len(), size);
				match op % 28 {
					3 => q.reserve(additional),
					_ => q.reserve_exact(additional),
				}
			}
			5 => q.shrink_to(usize::from(arg % 40)),
			6 => q.shrink_to_fit(),
			7 => q.truncate(usize::from(arg % 8)),
			8 => q.insert(usize::from(arg) % (q.len() + 1), make(arg)),
			9 => drop(q.remove(usize::from(arg) % (q.len() + 1))),
			10 => q.extend((0..arg % 24).filter(|i| i % 3 != 0).map(make)),
			11 => q.extend((0..arg % 24).map(make)),
			12 => q.clear(),
			13 => {
				let mut keep = alternate();
				q.retain(|_| keep());
			}
			14 => drop(q.drain(..q.len().min(usize::from(arg % 4)))),
			15 => q = q.clone(),
			// Its own, and a `Vec`'s in place of one, and one it collects first.
			16 => q = q.into_iter().collect(),
			17 => q = VecDeque::from(q.into_iter().collect::<Vec<T>>()),
			18 => {
				let mut keep = alternate();
				q = q.into_iter().filter(|_| keep()).collect();
			}
			19 => {
				let mut it = q.into_iter();
				for _ in 0..arg % 4 {
					drop(it.next());
				}
				q = it.clone().collect();
				let rest = it.collect::<VecDeque<T>>();
				put(out, rest.len());
				put(out, rest.capacity());
			}
			20 => q = VecDeque::from([make(arg), make(arg), make(arg)]),
			21 => {
				let mut other = VecDeque::with_capacity(usize::from(arg % 8));
				other.extend((0..arg % 12).map(make));
				q.append(&mut other);
				put(out, other.len());
				put(out, other.capacity());
			}
			22 | 23 => {
				let new_len = match arg >> 5 {
					6 if size > 0 => isize::MAX as usize / size + 1 + usize::from(arg & 7),
					_ => usize::from(arg % 24),
				};
				match op % 28 {
					22 => q.resize(new_len, make(arg)),
					_ => q.resize_with(new_len, || make(arg)),
				}
			}
			24 => {
				let tail = q.split_off(usize::from(arg) % (q.len() + 1));
				put(out, tail.len());
				put(out, tail.capacity());
			}
			25 => q = VecDeque::from(Vec::from(q)),
			_ => {
				let additional = amount(arg, q.len(), size);
				let tried = match op % 28 {
					26 => q.try_reserve(additional),
					_ => q.try_reserve_exact(additional),
				};
				out.extend(format!("{tried:?}").bytes());
			}
		}
		put(out, q.len());
		put(out, q.capacity());
	}
}

/// A `BinaryHeap`, as `vecs` steps a `Vec`, and the `Vec` it gives up at the end.
pub fn heaps(kind: u8, data: &[u8], out: &mut Vec<u8>) {
	by_kind!(kind, heap_steps, data, out);
}

fn heap_steps<T: Clone + Ord>(data: &[u8], make: fn(u8) -> T, out: &mut Vec<u8>) {
	let size = size_of::<T>();
	let (first, steps) = data.split_first().map_or((0, &[][..]), |(&b, rest)| (b, rest));
	let mut h: BinaryHeap<T> = match first & 1 {
		1 => BinaryHeap::with_capacity(usize::from(first >> 3)),
		_ => BinaryHeap::new(),
	};
	put(out, h.capacity());
	let mut other = BinaryHeap::new();
	for step in steps.chunks(2) {
		let (op, arg) = (step[0], step.get(1).copied().unwrap_or(0));
		match op % 15 {
			0 | 1 => h.push(make(arg)),
			2 => drop(h.pop()),
			3 | 4 => {
				let additional = amount(arg, h.len(), size);
				match op % 15 {
					3 => h.reserve(additional),
					_ => h.reserve_exact(additional),
				}
			}
			5 => h.shrink_to(usize::from(arg % 40)),
			6 => h.shrink_to_fit(),
			7 => h.clear(),
			8 => h.extend((0..arg % 24).filter(|i| i % 3 != 0).map(make)),
			// The larger of the two keeps its elements, and the other's join them.
			9 => {
				other.push(make(arg));
				h.append(&mut other);
				sized(out, other.len(), other.capacity());
			}
			10 => drop(h.drain()),
			// A guard that hid all but the top, forgotten, which leaks the rest.
			11 => {
				if let Some(mut top) = h.peek_mut() {
					*top = make(arg);
					mem::forget(top);
				}
			}
			12 => h = BinaryHeap::from(h.into_vec()),
			_ => {
				let additional = amount(arg, h.len(), size);
				let tried = match op % 15 {
					13 => h.try_reserve(additional),
					_ => h.try_reserve_exact(additional),
				};
				out.extend(format!("{tried:?}").bytes());
			}
		}
		sized(out, h.len(), h.capacity());
	}
	let cloned = h.clone().into_vec();
	sized(out, cloned.len(), cloned.capacity());
	let sorted = h.into_sorted_vec();
	sized(out, sorted.len(), sorted.capacity());
}

/// What collections each way std's makes them from an iterator hold room for: in place of
/// a `Vec` of elements of another size, by a tuple's `Extend`, by `vec!`, and by
/// `with_capacity` past what one may hold, which panics.
pub fn collects(kind: u8, data: &[u8], out: &mut Vec<u8>) {
	let n = data.first().map_or(0, |&b| usize::from(b % 24));
	let pairs: Vec<(u32, u32)> = Vec::with_capacity(n + usize::from(kind % 4));
	let mut pairs = pairs;
	pairs.extend((0..n as u32).map(|i| (i, i)));
	match kind % 8 {
		// Two `u32`s to one in place, which fit twice as many.
		0 => {
			let halves: Vec<u32> = pairs.into_iter().map(|(a, _)| a).collect();
			sized(out, halves.len(), halves.capacity());
		}
		// Bytes in place of bytes, and wider items than bytes, which collect anew.
		1 => {
			let mut bytes = Vec::with_capacity(n + 3);
			bytes.extend_from_slice(&data[..n.min(data.len())]);
			sized(out, bytes.len(), bytes.capacity());
			let odd: Vec<u8> = bytes.clone().into_iter().filter(|b| b % 2 == 1).collect();
			sized(out, odd.len(), odd.capacity());
			let wide: Vec<u16> = bytes.into_iter().map(u16::from).collect();
			sized(out, wide.len(), wide.capacity());
		}
		2 => {
			let (a, b): (Vec<u32>, VecDeque<u32>) = pairs.iter().filter(|(a, _)| a % 2 == 0).copied().unzip();
			sized(out, a.len(), a.capacity());
			sized(out, b.len(), b.capacity());
			let (c, d): (Vec<u8>, BinaryHeap<u32>) = pairs.iter().map(|&(a, b)| (a as u8, b)).unzip();
			sized(out, c.len(), c.capacity());
			sized(out, d.len(), d.capacity());
		}
		3 => {
			let filled = vec![0u16; n];
			sized(out, filled.len(), filled.capacity());
			let units = vec![(); n];
			sized(out, units.len(), units.capacity());
		}
		4 => {
			let v: Vec<u64> = Vec::with_capacity(usize::MAX - n);
			put(out, v.capacity());
		}
		5 => {
			let q: VecDeque<(u64, u64)> = VecDeque::with_capacity(0x1000_0000 + n);
			put(out, q.capacity());
		}
		// A deque and a heap in place of a `Vec` of bytes, and the bytes a heap gives up.
		6 => {
			let mut bytes = Vec::with_capacity(n + 5);
			bytes.extend_from_slice(&data[..n.min(data.len())]);
			let q = VecDeque::from(bytes.clone());
			sized(out, q.len(), q.capacity());
			let heap = BinaryHeap::from(bytes);
			sized(out, heap.len(), heap.capacity());
			let v = heap.into_vec();
			sized(out, v.len(), v.capacity());
		}
		_ => {
			let bytes = vec![7u8; n];
			let mut grown = bytes.clone();
			grown.extend((0..n as u8).filter(|b| b % 2 == 0));
			sized(out, grown.len(), grown.capacity());
			grown.shrink_to(n / 2);
			sized(out, grown.len(), grown.capacity());
		}
	}
}

/// std's `HashMap` and `HashSet`, made by `with_capacity` if `kind` is odd, as each pair of
/// `data`'s bytes changes them: their lengths, and whether each holds at least its length,
/// what `with_capacity` and `reserve` asked for, and no less past `shrink_to`, `clear`, or
/// `drain` than it asked for of what there was. A `reserve` or `with_capacity` past what
/// one may hold panics, where `try_reserve` gives an error and keeps the capacity.
pub fn maps(kind: u8, data: &[u8], out: &mut Vec<u8>) {
	let made = data.first().map_or(0, |&b| usize::from(b % 24));
	let (mut m, mut s): (HashMap<u16, (u64, u8)>, HashSet<u8>) = match kind & 1 {
		1 => (HashMap::with_capacity(made), HashSet::with_capacity(made)),
		_ => (HashMap::new(), HashSet::new()),
	};
	let floor = made * usize::from(kind & 1);
	let checks = |out: &mut Vec<u8>, lens: [usize; 2], caps: [usize; 2], floors: [usize; 2]| {
		for i in 0..2 {
			put(out, lens[i]);
			out.push(u8::from(caps[i] >= lens[i].max(floors[i])));
		}
	};
	checks(out, [m.len(), s.len()], [m.capacity(), s.capacity()], [floor; 2]);
	for step in data.get(1..).unwrap_or_default().chunks(2) {
		let (op, arg) = (step[0], step.get(1).copied().unwrap_or(0));
		let before = [m.capacity(), s.capacity()];
		let lens = [m.len(), s.len()];
		let mut floors = [0; 2];
		match op % 11 {
			0..=2 => {
				m.insert(u16::from(arg), (u64::from(op), arg));
				s.insert(arg);
			}
			3 => {
				m.remove(&u16::from(arg));
				s.remove(&arg);
			}
			4 => {
				let (a, b) = (
					amount(arg, lens[0], size_of::<(u16, (u64, u8))>()),
					amount(arg, lens[1], 1),
				);
				m.reserve(a);
				s.reserve(b);
				floors = [lens[0] + a, lens[1] + b];
			}
			5 => {
				let min = usize::from(arg % 20);
				m.shrink_to(min);
				s.shrink_to(min);
				floors = [before[0].min(lens[0].max(min)), before[1].min(lens[1].max(min))];
			}
			6 => {
				m.shrink_to_fit();
				s.shrink_to_fit();
			}
			7 => {
				m.clear();
				s.clear();
				floors = before;
			}
			8 => {
				// By key, as which keys every other one would keep follows iteration order.
				m.retain(|k, _| k % 3 != u16::from(arg % 3));
				s.retain(|k| k % 3 != arg % 3);
				// std promises nothing of a clone's capacity.
				m = m.clone();
				s = s.clone();
			}
			9 => {
				drop(m.drain());
				drop(s.drain());
				floors = before;
			}
			// Room as `reserve` makes it, or the error where it panics, which leaves the
			// capacity as it was.
			_ => {
				let (a, b) = (
					amount(arg, lens[0], size_of::<(u16, (u64, u8))>()),
					amount(arg, lens[1], 1),
				);
				for (i, (tried, n)) in [(m.try_reserve(a), a), (s.try_reserve(b), b)].into_iter().enumerate() {
					out.extend(format!("{tried:?}").bytes());
					floors[i] = if tried.is_ok() { lens[i] + n } else { before[i] };
				}
			}
		}
		checks(out, [m.len(), s.len()], [m.capacity(), s.capacity()], floors);
	}
	if kind & 0x80 != 0 {
		let huge: HashSet<u64> = HashSet::with_capacity(usize::MAX >> (kind & 3));
		put(out, huge.len());
	}
}
