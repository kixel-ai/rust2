//! `Cow<str>` and `Cow<[T]>`, borrowed and owned alike: made, read through `Deref`,
//! compared, ordered, hashed, printed, and held in a field and as a key, and changed
//! through `to_mut`, which leaves what it borrowed as it was. Each case runs once borrowed
//! and once owned.

use std::borrow::{Borrow, Cow};
use std::cell::Cell;
use std::cmp::Ordering;
use std::collections::{BTreeMap, HashSet};
use std::hash::{DefaultHasher, Hash, Hasher};
use std::sync::Arc;

use crate::harness::{put, text};

const FALLBACK: Cow<'static, str> = Cow::Borrowed("fallback");

/// A `Cow` in a field, with each derived impl reading it.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Named<'a> {
	name: Cow<'a, str>,
	id: u8,
}

impl<'a> Named<'a> {
	/// Borrowed unless it has to make the text, as moq-net's `Relative::new` is.
	fn new(s: &'a str, id: u8) -> Self {
		let trimmed = s.trim_end_matches('/');
		let name = match trimmed.is_empty() {
			true => Cow::Owned(format!("#{id}")),
			false => Cow::Borrowed(trimmed),
		};
		Self { name, id }
	}

	/// Through `to_mut` of the field.
	fn tag(&mut self, c: char) {
		self.name.to_mut().push(c);
	}

	fn into_owned(self) -> Named<'static> {
		Named {
			name: Cow::Owned(self.name.into_owned()),
			id: self.id,
		}
	}
}

/// `to_mut` of a `Result`'s payload, which the store rebuilds the `Result` around.
fn exclaimed(mut r: Result<Cow<'_, str>, u8>) -> Result<Cow<'_, str>, u8> {
	if let Ok(c) = &mut r {
		c.to_mut().push('!');
	}
	r
}

fn cow(s: &str, owned: bool) -> Cow<'_, str> {
	match owned {
		true => Cow::Owned(s.to_string()),
		false => s.into(),
	}
}

pub fn cows(kind: u8, data: &[u8], out: &mut Vec<u8>) {
	let s = text(data);
	let owned = kind & 0x80 != 0;
	let first = s.split('/').next().unwrap_or("");
	match kind % 6 {
		0 => {
			// What reads the text, and what makes one from it or out of it.
			let c = cow(&s, owned);
			out.push(c.len() as u8);
			out.push(c.is_empty() as u8);
			out.push(c.chars().count() as u8);
			put(out, &c);
			put(out, c.as_ref());
			put(out, &c.clone());
			put(out, &c.to_string());
			let held: String = c.into_owned();
			put(out, &held);
			let empty: Cow<'_, str> = Default::default();
			put(out, &empty);
			put(out, &FALLBACK);
			let from_string: Cow<'_, str> = Cow::from(s.clone());
			put(out, &String::from(from_string));
			let from_ref: Cow<'_, str> = Cow::from(&s);
			let shared: Arc<str> = Arc::from(from_ref);
			put(out, &shared);
			let part = s.split('\\').nth(1).map(Cow::Borrowed).unwrap_or(FALLBACK);
			put(out, &part);
			let into: String = cow(first, owned).into();
			put(out, &into);
			// The same conversions between a `str` and a `String`.
			let from_str = String::from(first);
			put(out, from_str.as_ref());
		}
		1 => {
			// `to_mut` of a local, twice through one reference, and of a vector's element.
			let mut c = cow(&s, owned);
			c.to_mut().push('!');
			let m = c.to_mut();
			m.push('?');
			m.push('\u{10000}');
			put(out, &c);
			put(out, &s);
			let mut v = vec![cow(first, owned), cow(&s, !owned)];
			v[1].to_mut().push('v');
			for c in &v {
				put(out, c);
			}
			put(out, &c.into_owned());
			let r = match kind & 0x40 {
				0 => Ok(cow(first, owned)),
				_ => Err(kind),
			};
			match exclaimed(r) {
				Ok(c) => put(out, &c),
				Err(e) => out.push(e),
			}
		}
		2 => {
			// Comparisons with each string type, either way around, and ordering.
			let a = cow(&s, owned);
			let b = cow(first, !owned);
			let bits = [
				a == *first,
				a == first,
				a == s,
				first == a,
				s == a,
				a == b,
				a != b,
				a.cmp(&b).is_lt(),
				a.cmp(&b).is_ge(),
				a.partial_cmp(&b) == Some(Ordering::Greater),
			];
			for bit in bits {
				out.push(bit as u8);
			}
			out.push((a.max(b.clone()) == b) as u8);
		}
		3 => {
			// A field: derived impls, `to_mut` through `&mut self`, and `into_owned`.
			let mut n = Named::new(first, kind);
			let m = n.clone();
			n.tag('x');
			put(out, &format!("{n:?} {m:?}"));
			out.push((n == m) as u8);
			out.push(n.cmp(&m).is_gt() as u8);
			out.push((n.partial_cmp(&m) == Some(Ordering::Less)) as u8);
			let o = n.into_owned();
			put(out, &o.name);
			put(out, &format!("{:?} {:?}", Named::default(), Named::new("", kind)));
		}
		4 => {
			// Keys: counted in a B-tree, found by a `&str`, and a field's hash in a set.
			let mut counts: BTreeMap<Cow<'_, str>, u8> = BTreeMap::new();
			for (i, part) in s.split('/').enumerate() {
				*counts.entry(cow(part, (i % 2 == 1) == owned)).or_insert(0) += 1;
			}
			for (k, v) in &counts {
				put(out, k);
				out.push(*v);
			}
			out.push(counts.get(first).copied().unwrap_or(0xff));
			let mut names = HashSet::new();
			for part in s.split('/') {
				names.insert(Named::new(part, 0));
			}
			out.push(names.len() as u8);
			out.push(names.contains(&Named::new(first, 0)) as u8);
		}
		_ => {
			// Formatting, with the specs `str`'s take.
			let c = cow(&s, owned);
			put(out, &format!("[{c}|{c:>6}|{c:<5}|{c:^7}|{c:*^3}|{c:?}]"));
			put(out, &format!("{:?}", Some(cow(first, owned))));
		}
	}
}

/// A slice of `u16`s read from `data`, two bytes each.
fn words(data: &[u8]) -> Vec<u16> {
	data.chunks(2)
		.map(|c| u16::from(c[0]) << 8 | u16::from(*c.last().unwrap()))
		.collect()
}

fn cow_of(v: &[u16], owned: bool) -> Cow<'_, [u16]> {
	match owned {
		true => Cow::Owned(v.to_vec()),
		false => Cow::Borrowed(v),
	}
}

fn put_words(out: &mut Vec<u8>, v: &[u16]) {
	out.push(v.len() as u8);
	for w in v {
		out.extend_from_slice(&w.to_be_bytes());
	}
}

fn hashed<T: Hash + ?Sized>(v: &T) -> u64 {
	let mut h = DefaultHasher::new();
	v.hash(&mut h);
	h.finish()
}

/// A `Cow<[T]>` in a field, with each derived impl reading it.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Run<'a> {
	words: Cow<'a, [u16]>,
}

impl Run<'_> {
	/// Through `to_mut` of the field.
	fn bump(&mut self) {
		for w in self.words.to_mut().iter_mut() {
			*w = w.wrapping_add(1);
		}
	}
}

/// An item whose `Clone` and `Drop` log, so what a `Cow` clones and drops shows.
struct Tally<'a>(u8, &'a Cell<u32>);

impl Clone for Tally<'_> {
	fn clone(&self) -> Self {
		self.1.set(self.1.get() * 7 + u32::from(self.0) + 1);
		Tally(self.0.wrapping_add(100), self.1)
	}
}

impl Drop for Tally<'_> {
	fn drop(&mut self) {
		self.1.set(self.1.get() * 3 + u32::from(self.0));
	}
}

/// `Cow<[u16]>`: made each way, collected, and read, changed through `to_mut`, compared,
/// ordered, and hashed against each sequence, matched on, and held in a field and as a key.
pub fn slices(kind: u8, data: &[u8], out: &mut Vec<u8>) {
	let v = words(data);
	let owned = kind & 0x80 != 0;
	let half = &v[..v.len() / 2];
	match kind % 6 {
		0 => {
			// What reads the slice, and what makes one from it or out of it.
			let c = cow_of(&v, owned);
			out.push(c.len() as u8);
			out.push(c.is_empty() as u8);
			out.extend_from_slice(&c.iter().fold(0u16, |a, w| a.wrapping_add(*w)).to_be_bytes());
			out.extend_from_slice(&c.first().copied().unwrap_or(0xffff).to_be_bytes());
			put_words(out, c.as_ref());
			put_words(out, c.borrow());
			put_words(out, &c.clone());
			put(out, &format!("{c:?}"));
			let held: Vec<u16> = c.into_owned();
			put_words(out, &held);
			let empty: Cow<'_, [u16]> = Default::default();
			put_words(out, &empty);
			let from_slice: Cow<'_, [u16]> = Cow::from(half);
			let from_vec: Cow<'_, [u16]> = Cow::from(v.clone());
			let from_ref: Cow<'_, [u16]> = Cow::from(&v);
			let from_array: Cow<'_, [u16]> = Cow::from(&[3u16, 1, 4]);
			let collected: Cow<'_, [u16]> = v.iter().rev().copied().collect();
			for c in [&from_slice, &from_vec, &from_ref, &from_array, &collected] {
				put_words(out, c);
			}
			put_words(out, &Vec::from(cow_of(half, owned)));
		}
		1 => {
			// `to_mut` of a local, twice, of a vector's element, and of a field, each of
			// which leaves the slice it borrowed as it was.
			let mut c = cow_of(&v, owned);
			c.to_mut().push(7);
			let m = c.to_mut();
			if let Some(w) = m.first_mut() {
				*w ^= 0xffff;
			}
			m.push(9);
			put_words(out, &c);
			put_words(out, &v);
			let mut cs = vec![cow_of(half, owned), cow_of(&v, !owned)];
			cs[1].to_mut().truncate(1);
			for c in &cs {
				put_words(out, c);
			}
			let mut run = Run {
				words: cow_of(half, owned),
			};
			let before = run.clone();
			run.bump();
			put(out, &format!("{run:?} {before:?}"));
			put_words(out, half);
			put_words(out, &c.into_owned());
		}
		2 => {
			// Comparisons with each sequence, either way around, and ordering and hashing.
			let a = cow_of(&v, owned);
			let b = cow_of(half, !owned);
			let array = [v.first().copied().unwrap_or(0); 2];
			let bits = [
				a == b,
				a != b,
				a == half,
				a == &v[..],
				a == v,
				a != half.to_vec(),
				half == b,
				v == a,
				&v[..] != a,
				*a == array,
				*b == array[..half.len().min(2)],
				a.cmp(&b).is_lt(),
				a.partial_cmp(&b) == Some(Ordering::Greater),
				a < b,
				a >= b,
				hashed(&a) == hashed(&v[..]),
				hashed(&b) == hashed(half),
				a.clone().max(b.clone()) == b,
			];
			for bit in bits {
				out.push(bit as u8);
			}
		}
		3 => {
			// Matched on, by reference, in place, and by value.
			let mut c = cow_of(&v, owned);
			match &c {
				Cow::Borrowed(s) => put_words(out, s),
				Cow::Owned(o) => {
					out.push(0xee);
					put_words(out, o);
				}
			}
			if let Cow::Owned(o) = &mut c {
				o.push(5);
			}
			put_words(out, &c);
			// A payload replaced whole, which leaves a clone as it was.
			let kept = c.clone();
			if let Cow::Borrowed(s) = &mut c {
				*s = half;
			}
			if let Cow::Owned(o) = &mut c {
				*o = half.iter().rev().copied().collect();
			}
			put_words(out, &c);
			put_words(out, &kept);
			// A payload passed on as a `&mut`, then replaced and read through the same
			// borrow, which leaves a clone as it was too.
			let mut e = cow_of(&v, !owned);
			let kept = e.clone();
			match &mut e {
				Cow::Borrowed(s) => {
					repoint(s, half);
					out.push(s.len() as u8);
					*s = &v[..v.len().min(1)];
					out.push(s.len() as u8);
				}
				Cow::Owned(o) => {
					o.truncate(1);
					*o = half.to_vec();
					out.push(o.len() as u8);
				}
			}
			put_words(out, &e);
			put_words(out, &kept);
			let made: Vec<u16> = match c {
				Cow::Borrowed(s) => s.iter().rev().copied().collect(),
				Cow::Owned(o) => o,
			};
			put_words(out, &made);
			let mut d = cow_of(half, !owned);
			d = match d {
				Cow::Borrowed(s) if s.len() > 1 => Cow::Owned(s[1..].to_vec()),
				d => d,
			};
			put_words(out, &d);
		}
		4 => {
			// Keys: counted in a B-tree, found by a `Cow`, and a field's hash in a set.
			let mut counts: BTreeMap<Cow<'_, [u16]>, u8> = BTreeMap::new();
			for (i, part) in v.chunks(2).enumerate() {
				*counts.entry(cow_of(part, (i % 2 == 1) == owned)).or_insert(0) += 1;
			}
			for (k, n) in &counts {
				put_words(out, k);
				out.push(*n);
			}
			out.push(counts.get(&cow_of(half, owned)).copied().unwrap_or(0xff));
			let mut runs = HashSet::new();
			for part in v.chunks(3) {
				runs.insert(Run {
					words: cow_of(part, owned),
				});
			}
			out.push(runs.len() as u8);
			out.push(runs.contains(&Run {
				words: cow_of(&v[..v.len().min(3)], !owned),
			}) as u8);
			put(out, &format!("{:?}", Run::default()));
		}
		_ => {
			// What a `Cow` of items whose `Clone` and `Drop` log clones and drops: a borrowed
			// one clones each item it changes or gives up, and an owned one drops them.
			let log = Cell::new(0);
			{
				let items: Vec<Tally> = data.iter().take(4).map(|&b| Tally(b, &log)).collect();
				let mut c = match owned {
					true => Cow::Owned(items.clone()),
					false => Cow::Borrowed(&items[..]),
				};
				out.extend_from_slice(&log.replace(0).to_be_bytes());
				let copy = c.clone();
				out.extend_from_slice(&log.replace(0).to_be_bytes());
				drop(copy);
				out.extend_from_slice(&log.replace(0).to_be_bytes());
				if let Cow::Borrowed(s) = &mut c {
					*s = &items[items.len().min(1)..];
				}
				out.push(c.len() as u8);
				c.to_mut().pop();
				out.extend_from_slice(&log.replace(0).to_be_bytes());
				c.to_mut().push(Tally(9, &log));
				out.extend_from_slice(&log.replace(0).to_be_bytes());
				// The `Vec` it owned drops as its payload is replaced.
				if let Cow::Owned(o) = &mut c {
					*o = vec![Tally(7, &log), Tally(8, &log)];
				}
				out.extend_from_slice(&log.replace(0).to_be_bytes());
				let kept = c.into_owned();
				out.extend_from_slice(&log.replace(0).to_be_bytes());
				out.extend(kept.iter().map(|t| t.0));
				drop(kept);
				out.extend_from_slice(&log.replace(0).to_be_bytes());
				let shared = Cow::Borrowed(&items[..]);
				let cloned = shared.into_owned();
				out.extend_from_slice(&log.replace(0).to_be_bytes());
				drop(cloned);
			}
			out.extend_from_slice(&log.get().to_be_bytes());
		}
	}
}

/// Point `s` at `to`, through the `&mut` a caller passes.
fn repoint<'a, T>(s: &mut &'a [T], to: &'a [T]) {
	*s = to;
}

fn made(s: &[u8], owned: bool) -> Cow<'_, [u8]> {
	match owned {
		true => Cow::Owned(s.to_vec()),
		false => Cow::Borrowed(s),
	}
}

/// `Cow<[u8]>` as bytes: made and collected, read, changed through `to_mut` without
/// touching what it borrowed, compared against each form of bytes, hashed, printed, and
/// matched on.
pub fn bytes(kind: u8, data: &[u8], out: &mut Vec<u8>) {
	let owned = kind & 0x80 != 0;
	let half = &data[..data.len() / 2];
	let mut c = made(data, owned);
	out.push(c.len() as u8);
	out.extend_from_slice(&c[..c.len().min(3)]);
	put(out, &format!("{c:?} {:?}", made(half, !owned)));
	let bits = [
		c == made(half, !owned),
		c == data,
		c == data.to_vec(),
		c != half,
		*c == [0u8; 2],
		data == c,
		data.to_vec() != c,
		c.cmp(&made(half, owned)).is_gt(),
		hashed(&c) == hashed(data),
		hashed(&c) == hashed(&data.to_vec()),
		matches!(c, Cow::Borrowed(_)),
	];
	for bit in bits {
		out.push(bit as u8);
	}
	// A payload replaced whole, which leaves a clone as it was.
	if kind & 0x40 != 0 {
		let kept = c.clone();
		match &mut c {
			Cow::Borrowed(s) => *s = half,
			Cow::Owned(o) => *o = half.to_vec(),
		}
		out.extend_from_slice(&kept);
		let mut e = made(data, !owned);
		match &mut e {
			Cow::Borrowed(s) => repoint(s, half),
			Cow::Owned(o) => o.truncate(1),
		}
		out.extend_from_slice(&e);
	}
	match kind % 3 {
		0 => {
			c.to_mut().push(kind);
			c.to_mut().extend_from_slice(half);
			out.extend_from_slice(&c);
		}
		1 => {
			if let Some(b) = c.to_mut().first_mut() {
				*b = b.wrapping_add(1);
			}
			let v: Vec<u8> = c.into_owned();
			out.extend_from_slice(&v);
		}
		_ => match c {
			Cow::Borrowed(s) => out.extend(s.iter().rev()),
			Cow::Owned(mut v) => {
				v.reverse();
				out.extend_from_slice(&v);
			}
		},
	}
	out.extend_from_slice(data);
	let empty: Cow<'_, [u8]> = Cow::default();
	out.push(empty.len() as u8);
	let held = data.to_vec();
	let from_ref: Cow<'_, [u8]> = Cow::from(&held);
	let from_array: Cow<'_, [u8]> = Cow::from(&[7u8, 8]);
	let collected: Cow<'_, [u8]> = data.iter().map(|b| b ^ 1).collect();
	for c in [&from_ref, &from_array, &collected] {
		out.extend_from_slice(c);
	}
}
