//! `Vec`'s and slices' editing and sorting methods, with closures that log what they are
//! called with, `Clone` and comparisons of vectors, tuples, boxes, and `Option`s that are
//! not `Copy`, `mem::take`, `replace`, and `swap`, called and as function values,
//! `mem::size_of`, `Ord`'s defaults on the crate's own types, and `str`'s byte searches.

use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::{BTreeSet, VecDeque};

use crate::fmt::put;
use crate::strs::text;

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
