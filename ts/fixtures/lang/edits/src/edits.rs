//! `Vec`'s and slices' editing and sorting methods, with closures that log what they are
//! called with, `Clone` and comparisons of vectors, tuples, boxes, and `Option`s that are
//! not `Copy`, `mem::take`, `replace`, and `swap`, called and as function values,
//! `mem::size_of`, `Ord`'s defaults on the crate's own types, `str`'s byte searches, the
//! `Vec` and `VecDeque` methods that resize, split, and append one, and those that take
//! out, splice, flatten, leak, search, and rotate one.

use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::{BTreeSet, VecDeque};

use crate::harness::{put, text};

/// The crate's own order, whose `min`, `max`, and `clamp` are `Ord`'s defaults.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
struct Stamp(u8);

/// An order whose `partial_cmp` leans on its first operand, an odd one less than any, and
/// whose `cmp` is by value, so what `Ord`'s defaults call, with which operand first, shows.
#[derive(Clone, Copy, PartialEq, Eq)]
struct Lean(u8);

#[allow(clippy::non_canonical_partial_ord_impl)]
impl PartialOrd for Lean {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(match self.0 % 2 {
			1 => Ordering::Less,
			_ => self.0.cmp(&other.0),
		})
	}
}

impl Ord for Lean {
	fn cmp(&self, other: &Self) -> Ordering {
		self.0.cmp(&other.0)
	}
}

/// What `mem::take` leaves its own `Default` in place of.
struct Tally {
	n: u8,
	names: Vec<String>,
}

impl Default for Tally {
	fn default() -> Self {
		Self {
			n: 7,
			names: vec![String::from("fresh")],
		}
	}
}

/// An element that is not `Copy`, cloned and compared field by field.
#[derive(Clone, PartialEq, PartialOrd)]
struct Named {
	id: u16,
	name: String,
}

/// Into an `Option`, as a parameter that takes either.
fn optional(n: impl Into<Option<Named>>) -> Option<Named> {
	n.into()
}

fn put_u16s(out: &mut Vec<u8>, v: &[u16]) {
	for x in v {
		out.extend_from_slice(&x.to_be_bytes());
	}
	out.push(0xfe);
}

pub fn edits(kind: u8, data: &[u8], out: &mut Vec<u8>) {
	let words: Vec<u16> = data.iter().map(|&b| b as u16 * 3).collect();
	let at = |i: usize| data.get(i).copied().unwrap_or(0);
	match kind % 8 {
		0 => {
			// `retain` sees each element once, in order, and keeps what it returns true for.
			let mut v = words.clone();
			let mut seen = Vec::new();
			v.retain(|&x| {
				seen.push(x);
				x % 4 != 0
			});
			put_u16s(out, &seen);
			put_u16s(out, &v);
			let mut pairs: Vec<(u16, u16)> = words.iter().map(|&w| (w, 0)).collect();
			pairs.retain_mut(|p| {
				p.1 = p.0 / 2;
				p.0 % 3 == 0
			});
			for (a, b) in &pairs {
				put_u16s(out, &[*a, *b]);
			}
			let mut names: Vec<String> = data.iter().map(|&b| text(&[b])).collect();
			names.retain(|s| s.len() > 1);
			out.push(names.len() as u8);
			// `pop_if` lends the last element to its test, which keeps what it changed.
			let mut v = words.clone();
			while let Some(x) = v.pop_if(|x| {
				*x ^= 1;
				*x % 4 != 0
			}) {
				put_u16s(out, &[x]);
			}
			put_u16s(out, &v);
			while let Some(s) = names.pop_if(|s| {
				s.push('.');
				s.len() % 3 != 0
			}) {
				put(out, &s);
			}
			for s in &names {
				put(out, s);
			}
			// Panics on the element the last byte names, keeping what it decided.
			let stop = at(data.len().saturating_sub(1)) as u16 * 3;
			if kind & 0x80 != 0 {
				v.retain(|&x| {
					if x == stop {
						panic!("retain stopped at {x}");
					}
					true
				});
			}
		}
		1 => {
			// Runs of equal neighbors, by value, by a key, and by a test that logs what it
			// compares, current first.
			let mut v: Vec<u16> = data.iter().map(|&b| (b % 4) as u16).collect();
			v.dedup();
			put_u16s(out, &v);
			// By what each element's closure reads through its `&mut`.
			let named = |w: u16| Named {
				id: w,
				name: text(&[w as u8 / 16]),
			};
			let mut keyed: Vec<Named> = words.iter().map(|&w| named(w)).collect();
			keyed.dedup_by_key(|n| n.id / 24);
			for n in &keyed {
				put_u16s(out, &[n.id]);
			}
			let mut tested: Vec<Named> = words.iter().map(|&w| named(w)).collect();
			let mut seen = Vec::new();
			tested.dedup_by(|a, b| {
				seen.push(a.id);
				seen.push(b.id);
				a.name == b.name
			});
			put_u16s(out, &seen);
			for n in &tested {
				put(out, &n.name);
			}
			let mut names: Vec<String> = data.iter().map(|&b| text(&[b % 3])).collect();
			names.dedup();
			out.push(names.len() as u8);
		}
		2 => {
			let mut v = words.clone();
			v.sort();
			put_u16s(out, &v);
			let mut v = words.clone();
			v.sort_unstable_by(|a, b| b.cmp(a));
			put_u16s(out, &v);
			// Stable: equal keys keep their order.
			let mut pairs: Vec<(u16, u16)> = words.iter().enumerate().map(|(i, &w)| (w % 5, i as u16)).collect();
			pairs.sort_by_key(|p| p.0);
			for (a, b) in &pairs {
				put_u16s(out, &[*a, *b]);
			}
			pairs.sort_unstable_by_key(|&(a, b)| (b % 3, a, b));
			for (a, b) in &pairs {
				put_u16s(out, &[*a, *b]);
			}
			pairs.sort_by(|x, y| y.1.cmp(&x.1));
			for (a, b) in &pairs {
				put_u16s(out, &[*a, *b]);
			}
			// `None` first, as `Option`'s `Ord` puts it.
			let mut options: Vec<Option<u16>> =
				words.iter().map(|&w| if w % 2 == 0 { None } else { Some(w) }).collect();
			options.sort_unstable();
			for o in &options {
				put_u16s(out, &[o.map_or(0xffff, |o| o)]);
			}
			let mut names: Vec<String> = data.iter().map(|&b| text(&[b, b / 16])).collect();
			names.sort();
			for name in &names {
				put(out, name);
			}
		}
		3 => {
			let mut v = words.clone();
			put_u16s(out, &[v.last().copied().unwrap_or(0xffff)]);
			let mut runs: Vec<(u16, Option<u16>)> = words.iter().map(|&w| (w, None)).collect();
			if let Some(last) = runs.last_mut() {
				last.1 = Some(last.0);
			}
			if let Some(first) = runs.first_mut() {
				first.0 += 1;
			}
			for (a, b) in &runs {
				put_u16s(out, &[*a, b.unwrap_or(0xffff)]);
			}
			v.reverse();
			put_u16s(out, &v);
			let named: Vec<Named> = words
				.iter()
				.map(|&w| Named {
					id: w,
					name: text(&[w as u8]),
				})
				.collect();
			let probe = Named {
				id: at(1) as u16 * 3,
				name: text(&[at(1).wrapping_mul(3)]),
			};
			out.push(named.contains(&probe) as u8);
			out.push(words.contains(&(at(2) as u16 * 3)) as u8);
			out.push(data.contains(&at(3)) as u8);
			// Each panics with std's message past the end.
			let (i, j) = (at(0) as usize % 8, at(1) as usize % 8);
			let mut moved = words.clone();
			let taken = moved.swap_remove(i);
			put_u16s(out, &[taken]);
			put_u16s(out, &moved);
			moved.swap(i, j);
			put_u16s(out, &moved);
		}
		4 => {
			// Clones and comparisons of what is not `Copy`, element by element.
			let named: Vec<Named> = words
				.iter()
				.map(|&w| Named {
					id: w,
					name: text(&[w as u8]),
				})
				.collect();
			let cloned = named.clone();
			out.push((cloned == named) as u8);
			let mut changed = named.to_vec();
			if let Some(first) = changed.first_mut() {
				first.name.push('!');
			}
			out.push((changed == named) as u8);
			out.push((changed != named) as u8);
			out.push((changed < named) as u8);
			changed.extend_from_slice(&named[..named.len() / 2]);
			out.push(changed.len() as u8);
			let nested: Vec<Vec<u16>> = data
				.iter()
				.map(|&b| (0..b % 3).map(|i| i as u16 + b as u16).collect())
				.collect();
			out.push((nested.clone() == nested) as u8);
			let pair = (text(data), at(0));
			let copy = pair.clone();
			put(out, &copy.0);
			out.push(((copy.1, &copy.0) > (at(1), &pair.0)) as u8);
			out.push(((at(0), at(1)) < (at(1), at(0))) as u8);
			let mut grown = words.clone();
			grown.extend_from_slice(&words);
			put_u16s(out, &grown);
		}
		5 => {
			// Taken, leaving each type's own `Default`.
			let mut tally = Tally {
				n: at(0),
				names: vec![text(data)],
			};
			let taken = std::mem::take(&mut tally);
			out.push(taken.n);
			out.push(tally.n);
			put(out, &tally.names[0]);
			let mut queue: VecDeque<u16> = words.iter().copied().collect();
			let drained = std::mem::take(&mut queue);
			out.push(drained.len() as u8);
			out.push(queue.len() as u8);
			let mut set: BTreeSet<u16> = words.iter().copied().collect();
			out.push(std::mem::take(&mut set).len() as u8);
			out.push(set.is_empty() as u8);
			// `Ord`'s defaults of the crate's own type; `clamp` panics on bounds out of order.
			let (x, lo, hi) = (Stamp(at(0)), Stamp(at(1)), Stamp(at(2)));
			out.push(x.max(lo).0);
			out.push(x.min(lo).0);
			out.push(x.clamp(lo, hi).0);
			// By `<`, `<=`, and `>`, which call `partial_cmp`, `other` first in `max` and `min`.
			let (p, q, r) = (Lean(at(0)), Lean(at(1)), Lean(at(2)));
			out.push(p.max(q).0);
			out.push(p.min(q).0);
			out.push(p.clamp(q, r).0);
		}
		6 => {
			let named = |w: u16| Named {
				id: w,
				name: text(&[w as u8]),
			};
			let ids = |v: &[Named]| v.iter().map(|n| n.id).collect::<Vec<_>>();
			// Each panics with std's message past the end.
			let mut v: Vec<Named> = words.iter().map(|&w| named(w)).collect();
			v.insert(at(0) as usize % 8, named(0x1ff));
			put_u16s(out, &ids(&v));
			let removed = v.remove(at(1) as usize % 8);
			put_u16s(out, &[removed.id]);
			let kept: Vec<Named> = v.clone();
			v.truncate(at(2) as usize % 4);
			put_u16s(out, &ids(&v));
			v.clear();
			out.push(v.is_empty() as u8);
			// Cloned through the box, and in an `Option`.
			let boxed = Box::new(removed);
			let mut copy = boxed.clone();
			copy.name.push('!');
			put(out, &boxed.name);
			put(out, &copy.name);
			let held = Some(boxed).filter(|_| at(3) % 2 == 0);
			let again = held.clone();
			out.push((again.map(|b| b.id) == held.map(|b| b.id)) as u8);
			// `From` into an `Option` is `Some`, compared with what is not `Copy`.
			let some = Option::from(kept.clone());
			out.push((some == Some(kept.clone())) as u8);
			out.push((some == None) as u8);
			out.push((some != Some(Vec::new())) as u8);
			out.push((optional(named(5)) == optional(None)) as u8);
			out.push(optional(named(at(4) as u16)).map_or(0xffff, |n| n.id) as u8);
			// A slice of one, as `Option::map` makes it of what it holds.
			let first = kept.first();
			let one = first.map(std::slice::from_ref).unwrap_or(&kept);
			put_u16s(out, &ids(one));
			put_u16s(out, &ids(std::slice::from_ref(&named(7))));
			out.push(std::slice::from_ref(&at(5))[0]);
			let invalid = String::from_utf8(vec![b'a', 0xff, at(6)]).unwrap_err();
			put(out, &format!("{:?}", invalid.clone()));
			// Sizes as wasm32 lays them out.
			out.push(std::mem::size_of::<Named>() as u8);
			out.push(size_of::<(u8, u64, u16)>() as u8);
			out.push(size_of::<Option<Box<Named>>>() as u8);
			out.push(size_of::<[u16; 3]>() as u8);
			out.push(size_of::<usize>() as u8 * size_of::<Stamp>() as u8);
		}
		_ => {
			let s = text(data);
			let sep = ['/', 'a'];
			out.push(s.contains(sep) as u8);
			out.push(s.contains(&sep[..]) as u8);
			out.push(s.split(sep).count() as u8);
			out.push(s.ends_with("/") as u8);
			out.push(s.ends_with('😀') as u8);
			out.push(s.ends_with(|c: char| c.is_ascii_digit()) as u8);
			match s.split_once('/') {
				Some((head, tail)) => {
					put(out, head);
					put(out, tail);
				}
				None => out.push(0xfd),
			}
			match s.split_once("a0") {
				Some((head, tail)) => {
					put(out, head);
					put(out, tail);
				}
				None => out.push(0xfd),
			}
			for i in 0..=s.len() + 1 {
				out.push(s.is_char_boundary(i) as u8);
			}
			out.extend(s.bytes());
			out.extend(s.bytes().rev().take(3));
			out.push(s.bytes().len() as u8);
			out.push(s.bytes().filter(|&b| b >= 0x80).count() as u8);
		}
	}
}

/// [`edits`]'s methods of bytes, which are an `rt.ByteVec` or `rt.ByteSlice` rather than an
/// array, on every short prefix of the input and on a slice that starts past its storage's,
/// and those that lend a closure another element JS copies, as its place.
pub fn bytes(kind: u8, data: &[u8], out: &mut Vec<u8>) {
	let at = |i: usize| data.get(i).copied().unwrap_or(0);
	let words: Vec<u16> = data.iter().map(|&b| b as u16 * 3).collect();
	let prefixes = (0..data.len().min(3)).map(|n| &data[..n]).chain([data]);
	match kind % 4 {
		0 => {
			for data in prefixes {
				// `retain` sees each byte once, in order; `retain_mut` keeps what it changed.
				let mut v = data.to_vec();
				let mut seen = Vec::new();
				v.retain(|&b| {
					seen.push(b);
					b % 3 != 0
				});
				out.extend_from_slice(&seen);
				out.extend_from_slice(&v);
				let mut v = data.to_vec();
				v.retain_mut(|b| {
					*b = b.wrapping_add(1);
					*b % 4 != 0
				});
				out.extend_from_slice(&v);
				while let Some(b) = v.pop_if(|b| {
					*b ^= 0x80;
					*b % 3 != 0
				}) {
					out.push(b);
				}
				out.extend_from_slice(&v);
				out.push(0xfe);
			}
			// An element JS copies, changed through its place.
			let mut halves = words.clone();
			halves.retain_mut(|x| {
				*x /= 2;
				*x % 3 != 0
			});
			put_u16s(out, &halves);
		}
		1 => {
			for data in prefixes {
				let mut v: Vec<u8> = data.iter().map(|b| b % 4).collect();
				v.dedup();
				out.extend_from_slice(&v);
				// Current first, then the previous kept, each changed in place.
				let mut v = data.to_vec();
				let mut seen = Vec::new();
				v.dedup_by(|a, b| {
					seen.extend_from_slice(&[*a, *b]);
					*b = b.wrapping_add(1);
					*a / 32 == *b / 32
				});
				out.extend_from_slice(&seen);
				out.extend_from_slice(&v);
				let mut v = data.to_vec();
				v.dedup_by_key(|b| {
					*b ^= 1;
					*b / 64
				});
				out.extend_from_slice(&v);
				out.push(0xfe);
			}
			// Elements JS copies, changed through their places.
			let mut v: Vec<u16> = words.clone();
			v.dedup_by(|a, b| {
				*b += 1;
				*a / 8 == *b / 8
			});
			put_u16s(out, &v);
			let mut v: Vec<u16> = words.clone();
			v.dedup_by_key(|x| {
				*x ^= 1;
				*x / 16
			});
			put_u16s(out, &v);
		}
		2 => {
			for data in prefixes {
				let mut v = data.to_vec();
				v.sort();
				out.extend_from_slice(&v);
				let mut v = data.to_vec();
				v.sort_unstable();
				out.extend_from_slice(&v);
				let mut v = data.to_vec();
				v.sort_by(|a, b| b.cmp(a));
				out.extend_from_slice(&v);
				// Stable: bytes of a key keep their order.
				let mut v = data.to_vec();
				v.sort_by_key(|b| b % 5);
				out.extend_from_slice(&v);
				// Unstable, by keys no two bytes share, so std's order is the only one.
				let mut v = data.to_vec();
				v.sort_unstable_by_key(|b| b.wrapping_mul(13));
				out.extend_from_slice(&v);
				let mut v = data.to_vec();
				v.sort_unstable_by(|a, b| (b ^ 0x55).cmp(&(a ^ 0x55)));
				out.extend_from_slice(&v);
				// Past the slice's own start in what holds it.
				let mut v = data.to_vec();
				let n = v.len();
				let tail = &mut v[n.min(1)..];
				tail.sort();
				tail.reverse();
				out.extend_from_slice(&v);
				out.push(v.last().copied().unwrap_or(0xfd));
				out.push(v[v.len().min(1)..].last().copied().unwrap_or(0xfd));
				v.reverse();
				out.extend_from_slice(&v);
				out.push(0xfe);
			}
		}
		_ => {
			let (i, j) = (at(0) as usize % 8, at(1) as usize % 8);
			let mut v = data.to_vec();
			v.insert(i.min(v.len()), 0xaa);
			out.extend_from_slice(&v);
			out.push(v.swap_remove(j % v.len()));
			out.extend_from_slice(&v);
			if !v.is_empty() {
				out.push(v.remove(i % v.len()));
			}
			out.extend_from_slice(&v);
			// Past the slice's own start in what holds it.
			let n = v.len();
			let tail = &mut v[n.min(1)..];
			if !tail.is_empty() {
				tail.swap(i % tail.len(), j % tail.len());
			}
			out.extend_from_slice(&v);
			// Each panics with std's message past the end.
			match at(2) % 5 {
				0 => v.insert(i, 0xbb),
				1 => out.push(v.remove(i)),
				2 => out.push(v.swap_remove(j)),
				3 => v[i % 3..].swap(i, j),
				_ => v.swap(i, j),
			}
			out.extend_from_slice(&v);
		}
	}
}

/// Logs its id as it drops, so what `mem::replace` gives back and leaves shows by when each
/// drops.
#[derive(Default)]
struct Logged<'a> {
	id: u8,
	log: Option<&'a RefCell<Vec<u8>>>,
}

impl Drop for Logged<'_> {
	fn drop(&mut self) {
		if let Some(log) = self.log {
			log.borrow_mut().push(self.id);
		}
	}
}

/// The crate's own iterator, counting down to zero.
struct Down(u8);

impl Iterator for Down {
	type Item = u8;

	fn next(&mut self) -> Option<u8> {
		self.0 = self.0.checked_sub(1)?;
		Some(self.0)
	}
}

/// `f` of `place` and `value`, as a function value of two parameters is called.
fn apply<T, U, R>(f: impl FnOnce(&mut T, U) -> R, place: &mut T, value: U) -> R {
	f(place, value)
}

/// `mem::take`, `replace`, and `swap` passed as function values: to an iterator's `map`, to
/// a generic function, and as `fn` pointers, of bytes JS keeps in a place, of objects it
/// writes in place, and of values that drop; and other function values that a call
/// lowers differently, as `next`.
pub fn values(kind: u8, data: &[u8], out: &mut Vec<u8>) {
	let at = |i: usize| data.get(i).copied().unwrap_or(0);
	match kind % 4 {
		0 => {
			let mut bytes = data.to_vec();
			let taken: Vec<u8> = bytes.iter_mut().map(std::mem::take).collect();
			out.extend_from_slice(&taken);
			out.extend_from_slice(&bytes);
			let mut held: Vec<Vec<u8>> = data.chunks(2).map(<[u8]>::to_vec).collect();
			let taken: Vec<Vec<u8>> = held.iter_mut().map(std::mem::take).collect();
			out.extend(taken.iter().map(Vec::len).map(|n| n as u8));
			out.extend(held.iter().map(Vec::len).map(|n| n as u8));
			let mut texts: Vec<String> = data.iter().map(|&b| text(&[b])).collect();
			let taken: Vec<String> = texts.iter_mut().rev().map(std::mem::take).collect();
			put(out, &taken.concat());
			put(out, &texts.concat());
		}
		1 => {
			let mut n = at(0);
			out.push(apply(std::mem::replace, &mut n, at(1)));
			out.push(n);
			let replace: fn(&mut u8, u8) -> u8 = std::mem::replace;
			out.push(replace(&mut n, at(2)));
			out.push(n);
			let mut s = text(data);
			let old = apply(std::mem::replace, &mut s, text(&[at(0)]));
			put(out, &old);
			put(out, &s);
			let mut v = data.to_vec();
			let take: fn(&mut Vec<u8>) -> Vec<u8> = std::mem::take;
			let old = take(&mut v);
			out.push(old.len() as u8);
			out.push(v.len() as u8);
			// Others of std's that write what a `&mut` borrows, as values too.
			let mut held = (at(0) % 2 == 0).then_some(at(1));
			out.extend(apply(Option::replace, &mut held, at(2)));
			out.extend(held);
			apply(String::push_str, &mut s, "+");
			apply(String::push, &mut s, '!');
			apply(Clone::clone_from, &mut s, &old.len().to_string());
			put(out, &s);
			let mut bytes = data.to_vec();
			bytes.iter_mut().for_each(u8::make_ascii_uppercase);
			out.extend_from_slice(&bytes);
			// A value of std's `next` returns `None` past the end, as its call does.
			let next: fn(&mut std::vec::IntoIter<u8>) -> Option<u8> = Iterator::next;
			let mut it = bytes.into_iter();
			while let Some(b) = next(&mut it) {
				out.push(b);
			}
			out.push(next(&mut it).is_none() as u8);
			// And of the crate's own.
			let next: fn(&mut Down) -> Option<u8> = Down::next;
			let mut down = Down(at(0) % 4);
			while let Some(b) = next(&mut down) {
				out.push(b);
			}
			out.push(next(&mut down).is_none() as u8);
		}
		2 => {
			let (mut a, mut b) = (at(0), at(1));
			apply(std::mem::swap, &mut a, &mut b);
			out.extend([a, b]);
			let swap: fn(&mut Vec<u8>, &mut Vec<u8>) = std::mem::swap;
			let (mut x, mut y) = (data.to_vec(), vec![at(2)]);
			swap(&mut x, &mut y);
			out.extend_from_slice(&x);
			out.extend_from_slice(&y);
			let mut texts: Vec<String> = data.iter().map(|&b| text(&[b])).collect();
			let mut others: Vec<String> = data.iter().rev().map(|&b| text(&[b, b])).collect();
			texts
				.iter_mut()
				.zip(others.iter_mut())
				.for_each(|(t, o)| apply(std::mem::swap, t, o));
			put(out, &texts.concat());
			put(out, &others.concat());
		}
		_ => {
			// Each replaced value drops where its caller drops what it gave back, and each
			// left in place where its owner drops it.
			let log = RefCell::new(Vec::new());
			let logged = |id: u8| Logged { id, log: Some(&log) };
			{
				let mut held: Vec<Logged> = data.iter().map(|&b| logged(b)).collect();
				let taken: Vec<Logged> = held.iter_mut().map(std::mem::take).collect();
				log.borrow_mut().push(0xf0);
				drop(taken);
				log.borrow_mut().push(0xf1);
				let mut one = logged(at(0));
				drop(apply(std::mem::replace, &mut one, logged(at(1))));
				log.borrow_mut().push(0xf2);
				let replace: fn(&mut _, _) -> _ = std::mem::replace;
				let old = replace(&mut one, logged(at(2)));
				let mut other = logged(at(3));
				apply(std::mem::swap, &mut one, &mut other);
				log.borrow_mut().push(0xf3);
				drop(other);
				log.borrow_mut().push(0xf4);
				drop(old);
				log.borrow_mut().push(0xf5);
			}
			out.extend_from_slice(&log.borrow());
		}
	}
}

/// Logs `0xcc` and its id as it clones, and its id and how many clones from the first it is
/// as it drops, so which of a vector's values are clones, and when each drops, shows.
struct Tracked<'a> {
	id: u8,
	copies: u8,
	log: &'a RefCell<Vec<u8>>,
}

impl Clone for Tracked<'_> {
	fn clone(&self) -> Self {
		self.log.borrow_mut().extend([0xcc, self.id]);
		Tracked {
			id: self.id,
			copies: self.copies + 1,
			log: self.log,
		}
	}
}

impl Drop for Tracked<'_> {
	fn drop(&mut self) {
		self.log.borrow_mut().extend([self.id, self.copies]);
	}
}

/// By id, as a deque's searches compare the keys they make.
impl PartialEq for Tracked<'_> {
	fn eq(&self, other: &Self) -> bool {
		self.id == other.id
	}
}

impl Eq for Tracked<'_> {}

impl PartialOrd for Tracked<'_> {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for Tracked<'_> {
	fn cmp(&self, other: &Self) -> Ordering {
		self.id.cmp(&other.id)
	}
}

/// The range of `0..=len + 1` two bytes pick, which may be past `len` or end before it
/// starts.
fn picked(a: u8, b: u8, len: usize) -> std::ops::Range<usize> {
	usize::from(a) % (len + 2)..usize::from(b) % (len + 2)
}

/// `q` with its items moved from the front to the back `n % 8` times, which wraps them past
/// the end of its buffer.
fn wrap<T>(q: &mut VecDeque<T>, n: u8) {
	for _ in 0..n % 8 {
		if let Some(x) = q.pop_front() {
			q.push_back(x);
		}
	}
}

/// `Vec`'s and `VecDeque`'s `resize`, `resize_with`, `split_off`, `append`, and `Vec`'s
/// `extend_from_within`: which values clone, in order, and when each removed one drops,
/// against a closure that owns one; whether a value written after it is shared with
/// another, of objects and of arrays and tuples JS holds by reference; bytes, which are an
/// `rt.ByteVec`; `Vec::from` of a deque; and a deque's `range_mut` of items JS copies. Each
/// panics with std's message past the end.
pub fn resizes(kind: u8, data: &[u8], out: &mut Vec<u8>) {
	let at = |i: usize| data.get(i).copied().unwrap_or(0);
	let log = RefCell::new(Vec::new());
	let tracked = |id: u8| Tracked {
		id,
		copies: 0,
		log: &log,
	};
	let mark = |m: u8| log.borrow_mut().push(m);
	let ids = |v: &[Tracked]| v.iter().flat_map(|t| [t.id, t.copies]).collect::<Vec<u8>>();
	match kind % 6 {
		0 => {
			{
				let mut v: Vec<Tracked> = data.iter().take(6).map(|&b| tracked(b)).collect();
				v.resize(usize::from(at(0) % 10), tracked(0xa0));
				mark(0xf0);
				out.extend(ids(&v));
				let owned = tracked(0xe0);
				let mut n = 0;
				v.resize_with(usize::from(at(1) % 10), move || {
					n += 1;
					// The whole of `owned`, which drops with the closure.
					let owned = &owned;
					Tracked {
						id: owned.id + n,
						copies: 0,
						log: owned.log,
					}
				});
				mark(0xf1);
				out.extend(ids(&v));
				let mut tail = v.split_off(usize::from(at(2)) % (v.len() + 1));
				mark(0xf2);
				out.extend(ids(&tail));
				v.extend_from_within(picked(at(3), at(4), v.len()).start.min(v.len())..);
				mark(0xf3);
				v.append(&mut tail);
				out.push(tail.len() as u8);
				out.extend(ids(&v));
				mark(0xf4);
			}
			out.extend_from_slice(&log.borrow());
			// Each panics past the end, or with a range that ends before it starts.
			let mut v: Vec<u16> = data.iter().map(|&b| u16::from(b) << 4).collect();
			match kind >> 3 & 3 {
				0 => drop(v.split_off(usize::from(at(5)) % (v.len() + 2))),
				1 => v.extend_from_within(picked(at(5), at(6), v.len())),
				2 => v.extend_from_within(usize::from(at(5)) % (v.len() + 2)..=usize::from(at(6)) % (v.len() + 2)),
				_ => v.extend_from_within(..=usize::from(at(5)) % (v.len() + 2)),
			}
			for x in v {
				out.extend(x.to_be_bytes());
			}
		}
		1 => {
			// A value written once resized or extended within is its own, not its source's.
			let named = |w: u16| Named {
				id: w,
				name: text(&[w as u8]),
			};
			let mut v: Vec<Named> = data.iter().take(4).map(|&b| named(u16::from(b))).collect();
			v.resize(usize::from(at(0) % 8), named(0x1ff));
			v.extend_from_within(..);
			for (i, n) in v.iter_mut().enumerate().skip(1).step_by(2) {
				n.name.push('!');
				n.id += i as u16;
			}
			for n in &v {
				put(out, &n.name);
				out.extend(n.id.to_be_bytes());
			}
			let mut pairs: Vec<[u16; 2]> = data.iter().take(3).map(|&b| [u16::from(b), 0]).collect();
			pairs.resize(usize::from(at(1) % 7), [7, 7]);
			pairs.extend_from_within(usize::from(at(2)) % (pairs.len() + 1)..);
			let mut tuples: Vec<(u8, u16)> = data.iter().take(3).map(|&b| (b, 1)).collect();
			tuples.resize_with(usize::from(at(3) % 7), || (at(4), 2));
			tuples.extend_from_within(..tuples.len() / 2);
			let mut moved = pairs.split_off(pairs.len() / 2);
			for (i, p) in pairs.iter_mut().chain(moved.iter_mut()).enumerate() {
				p[i % 2] += i as u16;
			}
			for t in tuples.iter_mut().step_by(2) {
				t.1 += 10;
			}
			pairs.append(&mut moved);
			for p in pairs.iter().flatten() {
				out.extend(p.to_be_bytes());
			}
			for (a, b) in &tuples {
				out.push(*a);
				out.extend(b.to_be_bytes());
			}
			out.push(moved.len() as u8);
			let mut keys: Vec<[u8; 2]> = data.chunks_exact(2).take(3).map(|c| [c[0], c[1]]).collect();
			keys.resize(usize::from(at(5) % 6), [9, 9]);
			keys.extend_from_within(..);
			let mut rest = keys.split_off(keys.len() / 3);
			for (i, k) in keys.iter_mut().chain(rest.iter_mut()).enumerate().step_by(2) {
				k[i % 2] ^= 0xff;
			}
			keys.append(&mut rest);
			out.extend(keys.iter().flatten());
		}
		2 => {
			// Bytes, on every short prefix of the input.
			for data in (0..data.len().min(3)).map(|n| &data[..n]).chain([data]) {
				let at = |i: usize| data.get(i).copied().unwrap_or(0);
				let mut v = data.to_vec();
				v.resize(usize::from(at(0) % 9), 0xaa);
				out.extend_from_slice(&v);
				let mut n = 0;
				v.resize_with(usize::from(at(1) % 9), || {
					n += 3;
					n
				});
				out.extend_from_slice(&v);
				v.extend_from_within(usize::from(at(2)) % (v.len() + 1)..);
				out.extend_from_slice(&v);
				let mut tail = v.split_off(usize::from(at(3)) % (v.len() + 1));
				out.extend_from_slice(&tail);
				tail.push(0xbb);
				v.append(&mut tail);
				out.extend_from_slice(&v);
				out.push(tail.len() as u8);
				out.push(0xfe);
			}
			let mut v = data.to_vec();
			match kind >> 3 & 1 {
				0 => drop(v.split_off(usize::from(at(5)) % (v.len() + 2))),
				_ => v.extend_from_within(picked(at(5), at(6), v.len())),
			}
			out.extend_from_slice(&v);
		}
		3 => {
			{
				let mut q: VecDeque<Tracked> = data.iter().take(5).map(|&b| tracked(b)).collect();
				wrap(&mut q, at(0));
				q.resize(usize::from(at(1) % 9), tracked(0xa0));
				mark(0xf0);
				let owned = tracked(0xe0);
				let mut n = 0;
				q.resize_with(usize::from(at(2) % 9), move || {
					n += 1;
					// The whole of `owned`, which drops with the closure.
					let owned = &owned;
					Tracked {
						id: owned.id + n,
						copies: 0,
						log: owned.log,
					}
				});
				mark(0xf1);
				let mut tail = q.split_off(usize::from(at(3)) % (q.len() + 1));
				mark(0xf2);
				out.extend(tail.iter().flat_map(|t| [t.id, t.copies]));
				tail.push_front(tracked(0xd0));
				q.append(&mut tail);
				out.push(tail.len() as u8);
				out.extend(q.iter().flat_map(|t| [t.id, t.copies]));
				mark(0xf3);
				let v = Vec::from(q);
				out.extend(ids(&v));
				mark(0xf4);
			}
			out.extend_from_slice(&log.borrow());
			let mut q: VecDeque<u16> = data.iter().map(|&b| u16::from(b)).collect();
			drop(q.split_off(usize::from(at(5)) % (q.len() + 2)));
			out.push(q.len() as u8);
		}
		4 => {
			// `range_mut` writes each item JS copies in place, and an object it lends.
			let mut q: VecDeque<u16> = data.iter().map(|&b| u16::from(b)).collect();
			wrap(&mut q, at(0));
			let r = picked(at(1), at(2), q.len());
			let (lo, hi) = (r.start.min(q.len()), r.end.min(q.len()));
			for (i, x) in q.range_mut(lo.min(hi)..hi).enumerate() {
				*x += 0x100 * (i as u16 + 1);
			}
			let mut it = q.range_mut(lo.min(hi)..);
			if let Some(x) = it.next_back() {
				*x ^= 0xffff;
			}
			for x in &q {
				out.extend(x.to_be_bytes());
			}
			let mut named: VecDeque<Named> = q
				.iter()
				.map(|&w| Named {
					id: w,
					name: text(&[w as u8]),
				})
				.collect();
			for n in named.range_mut(lo.min(hi)..hi) {
				n.name.push('+');
			}
			for n in &named {
				put(out, &n.name);
			}
			let bytes: VecDeque<u8> = data.iter().copied().collect();
			let mut bytes = bytes;
			if let Some(b) = bytes.range_mut(..).next() {
				*b = b.wrapping_add(1);
			}
			out.extend_from_slice(&Vec::from(bytes));
			// Panics past the end, or with a range that ends before it starts.
			for x in q.range_mut(picked(at(3), at(4), q.len())) {
				*x += 1;
			}
			out.push(q.len() as u8);
		}
		_ => {
			// A deque of bytes, wrapped, and of values JS copies.
			let mut q: VecDeque<u8> = data.iter().copied().collect();
			wrap(&mut q, at(0));
			q.resize(usize::from(at(1) % 9), 0xaa);
			let mut n = 0;
			q.resize_with(usize::from(at(2) % 9), || {
				n += 3;
				n
			});
			let mut tail = q.split_off(usize::from(at(3)) % (q.len() + 1));
			out.extend(tail.iter());
			tail.push_front(0xdd);
			q.append(&mut tail);
			out.extend(q.iter());
			out.extend_from_slice(&Vec::from(q));
			let mut pairs: VecDeque<[u16; 2]> = data.iter().take(4).map(|&b| [u16::from(b), 0]).collect();
			pairs.resize(usize::from(at(4) % 7), [7, 7]);
			let mut moved = pairs.split_off(pairs.len() / 2);
			for (i, p) in pairs.iter_mut().chain(moved.iter_mut()).enumerate() {
				p[i % 2] += i as u16;
			}
			pairs.append(&mut moved);
			for p in Vec::from(pairs).iter().flatten() {
				out.extend(p.to_be_bytes());
			}
		}
	}
}

/// A range within `0..=len` two bytes pick.
fn within(a: u8, b: u8, len: usize) -> std::ops::Range<usize> {
	let start = usize::from(a) % (len + 1);
	start..start + usize::from(b) % (len - start + 1)
}

/// Items a counter makes up to `n`, logging each `size_hint` read of it, whose lower bound
/// is one at odd counts while any are left.
struct Hinted<'a> {
	n: u8,
	i: u8,
	log: &'a RefCell<Vec<u8>>,
}

impl<'a> Iterator for Hinted<'a> {
	type Item = Tracked<'a>;

	fn next(&mut self) -> Option<Tracked<'a>> {
		(self.i < self.n).then(|| {
			self.i += 1;
			Tracked {
				id: 0xd0 + self.i,
				copies: 0,
				log: self.log,
			}
		})
	}

	fn size_hint(&self) -> (usize, Option<usize>) {
		self.log.borrow_mut().push(0xd5);
		(usize::from((self.i % 2).min(self.n - self.i)), None)
	}
}

/// `n` units, logging each `size_hint` read of them, whose lower bound is none.
struct Units<'a> {
	n: u8,
	log: &'a RefCell<Vec<u8>>,
}

impl Iterator for Units<'_> {
	type Item = ();

	fn next(&mut self) -> Option<()> {
		self.n = self.n.checked_sub(1)?;
		self.log.borrow_mut().push(self.n);
		Some(())
	}

	fn size_hint(&self) -> (usize, Option<usize>) {
		self.log.borrow_mut().push(0xd5);
		(0, None)
	}
}

/// `v.splice(r, items)`, read `read % 4` times from alternate ends and then dropped, each
/// step marked.
fn spliced<'a>(
	v: &mut Vec<Tracked<'a>>,
	r: std::ops::Range<usize>,
	items: impl IntoIterator<Item = Tracked<'a>>,
	read: u8,
	log: &RefCell<Vec<u8>>,
) {
	let mut s = v.splice(r, items);
	for i in 0..read % 4 {
		let t = match i % 2 {
			0 => s.next(),
			_ => s.next_back(),
		};
		log.borrow_mut().push(0xf0);
		drop(t);
	}
	drop(s);
	log.borrow_mut().push(0xf1);
}

/// `Vec`'s `extract_if`, `splice`, `into_flattened`, and `leak`, and `VecDeque`'s
/// searches, rotations, `swap`, and `make_contiguous`: which values clone and drop, in
/// order, against closures and iterators that own one; `{:?}` of an `ExtractIf` and a
/// `Splice` as each is read; writes through the `&mut` each lends, of elements JS copies and of bytes; and a
/// splice by iterators whose lower bound is exact, short, or none. Each panics with std's
/// message past the end.
pub fn extracts(kind: u8, data: &[u8], out: &mut Vec<u8>) {
	let at = |i: usize| data.get(i).copied().unwrap_or(0);
	let log = RefCell::new(Vec::new());
	let tracked = |id: u8| Tracked {
		id,
		copies: 0,
		log: &log,
	};
	let mark = |m: u8| log.borrow_mut().push(m);
	let ids = |v: &[Tracked]| v.iter().flat_map(|t| [t.id, t.copies]).collect::<Vec<u8>>();
	match kind % 5 {
		0 => {
			{
				let mut v: Vec<Tracked> = data.iter().take(8).map(|&b| tracked(b)).collect();
				let r = within(at(0), at(1), v.len());
				let owned = tracked(0xe0);
				// Through the `&mut` each element lends, and the whole of `owned`, which
				// drops with the iterator.
				let mut it = v.extract_if(r, move |t| {
					let owned = &owned;
					owned.log.borrow_mut().extend([0xe1, t.id]);
					t.copies += 1;
					t.id % 3 == owned.id % 3
				});
				for _ in 0..at(2) % 4 {
					let t = it.next();
					mark(0xf0);
					drop(t);
				}
				drop(it);
				mark(0xf1);
				out.extend(ids(&v));
			}
			out.extend_from_slice(&log.borrow());
			let mut w: Vec<u16> = data.iter().map(|&b| u16::from(b)).collect();
			let r = within(at(3), at(4), w.len());
			let mut it = w.extract_if(r, |x| {
				*x += 0x100;
				*x & 1 == 1
			});
			for _ in 0..3 {
				out.extend(format!("{it:?}").as_bytes());
				it.next();
			}
			drop(it);
			for x in &w {
				out.extend(x.to_be_bytes());
			}
			let named: Vec<Named> = data
				.iter()
				.take(5)
				.map(|&b| Named {
					id: u16::from(b),
					name: text(&[b]),
				})
				.collect();
			let mut named = named;
			let taken: Vec<Named> = named
				.extract_if(.., |n| {
					n.name.push('+');
					n.id % 2 == 0
				})
				.collect();
			for n in named.iter().chain(&taken) {
				put(out, &n.name);
			}
			let mut bytes = data.to_vec();
			let mut it = bytes.extract_if(usize::from(at(5)) % (data.len() + 1).., |b| {
				*b = b.wrapping_add(1);
				*b % 3 == 0
			});
			let first = it.next();
			out.extend(format!("{it:?}{first:?}").as_bytes());
			let rest: Vec<u8> = it.collect();
			out.extend_from_slice(&rest);
			out.extend_from_slice(&bytes);
			// A leaked one leaves the vector empty, as std's holds none meanwhile.
			let mut it = w.extract_if(.., |x| *x % 2 == 0);
			it.next();
			std::mem::forget(it);
			out.push(w.len() as u8);
			let mut it = bytes.extract_if(.., |b| *b % 2 == 0);
			it.next();
			std::mem::forget(it);
			out.push(bytes.len() as u8);
			// Panics past the end, or with a range that ends before it starts.
			if kind & 0x80 != 0 {
				drop(w.extract_if(picked(at(6), at(7), w.len()), |_| true));
			}
		}
		1 => {
			{
				let mut v: Vec<Tracked> = data.iter().take(6).map(|&b| tracked(b)).collect();
				let mut r = within(at(0), at(1), v.len());
				if kind & 0x20 != 0 {
					r.end = v.len();
				}
				let owned = tracked(0xe0);
				let n = at(2) % 6;
				// The whole of `owned`, which drops with the iterator.
				let make = move |i: u8| {
					let owned = &owned;
					owned.log.borrow_mut().extend([0xe1, i]);
					Tracked {
						id: 0xb0 + i,
						copies: 0,
						log: owned.log,
					}
				};
				let read = at(3);
				match kind >> 3 & 3 {
					_ if kind & 0x40 != 0 => {
						drop(make);
						spliced(&mut v, r, Hinted { n, i: 0, log: &log }, read, &log);
					}
					0 => spliced(&mut v, r, (0..n).map(make), read, &log),
					1 => spliced(&mut v, r, (0..n).filter(|i| i % 2 == 0).map(make), read, &log),
					2 => {
						let first: Vec<Tracked> = (0..n % 3).map(|i| tracked(0xc0 + i)).collect();
						spliced(
							&mut v,
							r,
							first.into_iter().chain((0..n).filter(|i| i % 2 == 1).map(make)),
							read,
							&log,
						)
					}
					_ => {
						let items: Vec<Tracked> = (0..n).map(make).collect();
						spliced(&mut v, r, items, read, &log)
					}
				}
				out.extend(ids(&v));
			}
			out.extend_from_slice(&log.borrow());
			// Values JS copies, each its own once spliced in.
			let mut pairs: Vec<[u16; 2]> = data.iter().take(4).map(|&b| [u16::from(b), 0]).collect();
			let r = within(at(4), at(5), pairs.len());
			let fill = [7u16, 8];
			let mut s = pairs.splice(r, (0..at(6) % 4).map(|_| fill));
			let first = s.next();
			out.extend(format!("{s:?}{first:?}").as_bytes());
			let removed: Vec<[u16; 2]> = first.into_iter().chain(s).collect();
			for (i, p) in pairs.iter_mut().enumerate() {
				p[i % 2] += i as u16;
			}
			for p in pairs.iter().chain(&removed).flatten() {
				out.extend(p.to_be_bytes());
			}
			let mut bytes = data.to_vec();
			let r = within(at(7), at(8), bytes.len());
			let mut s = bytes.splice(
				r,
				data.iter()
					.rev()
					.take(usize::from(at(9) % 8))
					.copied()
					.filter(|b| b % 2 == 0),
			);
			let back = s.next_back();
			out.extend(format!("{s:?}").as_bytes());
			drop(s);
			out.extend(back);
			out.extend_from_slice(&bytes);
			let mut tail = data.to_vec();
			let start = usize::from(at(10)) % (tail.len() + 1);
			let s = tail.splice(start.., [0xaa, 0xbb]);
			out.extend(format!("{s:?}").as_bytes());
			out.extend(s.rev());
			out.extend_from_slice(&tail);
			// Zero-sized ones, collected as std's collects them.
			let hints = RefCell::new(Vec::new());
			let mut units = vec![(); usize::from(at(13) % 4)];
			let r = within(at(14), at(15), units.len());
			drop(units.splice(
				r,
				Units {
					n: at(16) % 5,
					log: &hints,
				},
			));
			out.push(units.len() as u8);
			out.extend_from_slice(&hints.borrow());
			// Panics past the end, or with a range that ends before it starts.
			if kind & 0x80 != 0 {
				drop(tail.splice(picked(at(11), at(12), tail.len()), []));
			}
		}
		2 => {
			{
				let pairs: Vec<[Tracked; 2]> = data
					.chunks_exact(2)
					.take(3)
					.map(|c| [tracked(c[0]), tracked(c[1])])
					.collect();
				let mut flat = pairs.into_flattened();
				mark(0xf0);
				flat.truncate(usize::from(at(0) % 4));
				mark(0xf1);
				out.extend(ids(&flat));
				// What a leaked vector holds never drops.
				let leaked = flat.leak();
				if let Some(t) = leaked.first_mut() {
					t.copies += 5;
				}
				out.extend(ids(leaked));
			}
			out.extend_from_slice(&log.borrow());
			let words: Vec<[u16; 3]> = data
				.chunks_exact(3)
				.map(|c| [u16::from(c[0]), u16::from(c[1]), u16::from(c[2])])
				.collect();
			let mut flat = words.into_flattened();
			for (i, x) in flat.iter_mut().enumerate().step_by(2) {
				*x += i as u16;
			}
			for x in &flat {
				out.extend(x.to_be_bytes());
			}
			let named: Vec<[Named; 2]> = data
				.iter()
				.take(4)
				.map(|&b| {
					let n = Named {
						id: u16::from(b),
						name: text(&[b]),
					};
					[n.clone(), n]
				})
				.collect();
			let mut flat = named.into_flattened();
			for n in flat.iter_mut().step_by(2) {
				n.name.push('!');
			}
			for n in &flat {
				put(out, &n.name);
			}
			let bytes: Vec<[u8; 2]> = data.chunks_exact(2).map(|c| [c[0], c[1]]).collect();
			let mut flat = bytes.into_flattened();
			flat.push(0xcc);
			out.extend_from_slice(&flat);
			let leaked: &mut [u16] = flat.iter().map(|&b| u16::from(b)).collect::<Vec<u16>>().leak();
			leaked.reverse();
			if let Some(x) = leaked.first_mut() {
				*x ^= 0xffff;
			}
			for x in leaked.iter() {
				out.extend(x.to_be_bytes());
			}
			let leaked: &mut [u8] = flat.leak();
			leaked.rotate_left(leaked.len() / 2);
			out.extend_from_slice(leaked);
		}
		3 => {
			{
				let mut q: VecDeque<Tracked> = data.iter().take(7).map(|&b| tracked(b)).collect();
				wrap(&mut q, at(0));
				let probe = tracked(at(1));
				// Each key it makes drops once compared.
				let found = q.binary_search_by_key(&probe, |t| {
					t.log.borrow_mut().extend([0xe1, t.id]);
					Tracked {
						id: t.id,
						copies: 9,
						log: t.log,
					}
				});
				mark(0xf0);
				out.extend(format!("{found:?}").as_bytes());
				let found = q.binary_search(&probe);
				out.extend(format!("{found:?}").as_bytes());
				let point = q.partition_point(|t| t.id < probe.id);
				out.push(point as u8);
				let n = q.len();
				q.rotate_left(usize::from(at(2)) % (n + 1));
				q.rotate_right(usize::from(at(3)) % (n + 1));
				if n > 0 {
					q.swap(usize::from(at(4)) % n, usize::from(at(5)) % n);
				}
				for t in q.make_contiguous() {
					t.copies += 1;
				}
				let (a, b) = q.as_slices();
				out.extend(ids(a));
				out.push(0xfe);
				out.extend(ids(b));
				mark(0xf1);
			}
			out.extend_from_slice(&log.borrow());
			// Panics with std's assertions.
			let mut q: VecDeque<u16> = data.iter().map(|&b| u16::from(b)).collect();
			let n = q.len();
			match kind >> 3 & 3 {
				_ if kind & 0x80 == 0 => {}
				0 => q.rotate_left(n + usize::from(at(6) % 2)),
				1 => q.rotate_right(n + usize::from(at(6) % 2)),
				2 => q.swap(n.min(usize::from(at(6))), 0),
				_ => q.swap(0, n.min(usize::from(at(6)))),
			}
			out.push(q.len() as u8);
		}
		_ => {
			// Writes through the run `make_contiguous` lends, of values JS copies and of
			// bytes, and objects whose searches take them.
			let mut q: VecDeque<u16> = data.iter().map(|&b| u16::from(b)).collect();
			wrap(&mut q, at(0));
			q.push_front(0x1ff);
			for (i, x) in q.make_contiguous().iter_mut().enumerate() {
				*x += i as u16;
			}
			q.rotate_right(q.len() / 3);
			for x in &q {
				out.extend(x.to_be_bytes());
			}
			let mut bytes: VecDeque<u8> = data.iter().copied().collect();
			wrap(&mut bytes, at(1));
			bytes.push_front(0xdd);
			bytes.make_contiguous().reverse();
			bytes.rotate_left(bytes.len() / 2);
			if bytes.len() > 1 {
				bytes.swap(0, bytes.len() - 1);
			}
			out.extend(bytes.iter());
			let (a, b) = bytes.as_slices();
			out.push(a.len() as u8);
			out.push(b.len() as u8);
			let mut named: VecDeque<Named> = data
				.iter()
				.take(6)
				.map(|&b| Named {
					id: u16::from(b),
					name: text(&[b]),
				})
				.collect();
			wrap(&mut named, at(2));
			named.make_contiguous().sort_by_key(|n| n.id);
			let probe = u16::from(at(3));
			let found = named.binary_search_by(|n| n.id.cmp(&probe));
			out.extend(format!("{found:?}").as_bytes());
			let found = named.binary_search_by_key(&text(&[at(3)]), |n| n.name.clone());
			out.extend(format!("{found:?}").as_bytes());
			out.push(named.partition_point(|n| n.id <= probe) as u8);
			let (Ok(i) | Err(i)) = found;
			if let Some(n) = named.get_mut(i) {
				n.name.push('?');
			}
			for n in &named {
				put(out, &n.name);
			}
		}
	}
}
