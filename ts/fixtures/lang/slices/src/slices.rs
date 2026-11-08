//! A slice's `windows` and each of its chunks, of bytes and of other values, read from
//! both ends, skipped through, and counted, at sizes past the slice's length, with what
//! is written through a mutable chunk landing in the slice; and `Vec::drain` of
//! each kind of range, read from both ends, dropped part way, and forgotten, of values
//! that log when they drop; and a slice's parts by each kind of range and by fixed size,
//! with what is written through a mutable one landing in the slice; a slice's edits in
//! place, of values that log as they are cloned and drop; and a slice's runs as arrays,
//! and a `Vec` cloned from a slice; and slices of arrays in each form they take: constants,
//! defaults, an `Option`'s, a `Cow`'s, a generic struct's, clones, and runs of references
//! and of wrapped arrays; and a mutable reference to part of a slice as an array, written
//! whole.

use std::cell::RefCell;

thread_local! {
	static LOG: RefCell<Vec<u8>> = const { RefCell::new(Vec::new()) };
}

/// Puts which items dropped since the last flush.
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
		LOG.with_borrow_mut(|log| log.push(self.tag));
	}
}

/// A digest of a part, which tells parts of different lengths or contents apart.
fn digest(part: &[u8]) -> u8 {
	part.iter()
		.fold(part.len() as u8, |acc, &b| acc.wrapping_mul(31).wrapping_add(b))
}

fn digest_words(part: &[u16]) -> u8 {
	part.iter()
		.fold(part.len() as u8, |acc, &w| acc.wrapping_mul(31) ^ (w >> 3) as u8)
}

/// Each way of reading `$parts`, an expression that makes the iterator anew.
macro_rules! read {
	($parts:expr, $digest:expr, $out:expr) => {{
		let out: &mut Vec<u8> = $out;
		out.extend($parts.map($digest));
		out.extend($parts.rev().map($digest));
		let mut it = $parts;
		out.push(it.len() as u8);
		out.extend(it.next().map($digest));
		out.extend(it.next_back().map($digest));
		out.extend(it.nth(1).map($digest));
		out.extend(it.nth_back(1).map($digest));
		out.push(it.len() as u8);
		out.extend(it.map($digest));
		out.extend($parts.last().map($digest));
		out.push($parts.count() as u8);
		out.extend($parts.skip(2).step_by(2).map($digest));
	}};
}

/// `windows` or `chunks`, as `kind` picks, of a size it picks too, which is 0 one time in
/// six, which panics.
pub fn parts(kind: u8, data: &[u8], out: &mut Vec<u8>) {
	let size = (kind / 2 % 6) as usize;
	let words: Vec<u16> = data.iter().map(|&b| b as u16 * 257).collect();
	match kind % 2 {
		0 => {
			read!(data.windows(size), digest, out);
			read!(words.windows(size), digest_words, out);
			// A window kept past the next, and compared with it.
			let mut steps = data.windows(size);
			if let (Some(a), Some(b)) = (steps.next(), steps.next()) {
				out.extend([(a == b) as u8, a[0], b[size - 1]]);
			}
		}
		_ => {
			read!(data.chunks(size), digest, out);
			read!(words.chunks(size), digest_words, out);
			// The remainder, which the last chunk is.
			out.extend(data.chunks(size).next_back().map(|c| c.len() as u8));
			let rows: Vec<u16> = words.chunks(size).map(|c| c.iter().sum::<u16>() >> 2).collect();
			out.extend(rows.iter().map(|&r| r as u8));
		}
	}
}

/// An element kept as an object, which a mutable chunk writes whole and by field.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Pair {
	a: u8,
	b: u16,
}

/// Writes through one mutable chunk, the `i`th, as each element type allows.
fn scribble(i: usize, chunk: &mut [u8]) {
	let n = chunk.len();
	chunk[0] = chunk[0].wrapping_add(i as u8);
	chunk.swap(0, n - 1);
	if i % 2 == 1 {
		chunk.reverse();
	}
	for b in chunk.iter_mut() {
		*b ^= 0x5a;
	}
}

fn scribble_words(i: usize, chunk: &mut [u16]) {
	let n = chunk.len();
	chunk[n - 1] = chunk[n - 1].wrapping_mul(3).wrapping_add(i as u16);
	chunk.swap(0, n / 2);
	if let Some(w) = chunk.first_mut() {
		*w ^= 0x1234;
	}
	if i % 3 == 2 {
		chunk.fill(i as u16);
	}
	// A chunk of a chunk writes through to the slice too.
	for (j, inner) in chunk.chunks_mut(2).enumerate() {
		inner[0] = inner[0].wrapping_add(j as u16);
	}
}

fn scribble_pairs(i: usize, chunk: &mut [Pair]) {
	let n = chunk.len();
	chunk[0].a = chunk[0].a.wrapping_add(i as u8);
	chunk[n - 1] = Pair {
		a: chunk[n - 1].b as u8,
		b: chunk[0].a as u16,
	};
	chunk.sort();
	chunk.swap(0, n / 2);
}

fn digest_pairs(part: &[Pair]) -> u8 {
	part.iter()
		.fold(part.len() as u8, |acc, p| acc.wrapping_mul(31) ^ p.a ^ (p.b >> 4) as u8)
}

/// Each mutable chunk of `$data`, written as `$scribble` writes the `i`th; then each of a
/// pass that keeps every chunk at once, written back to front; then `$data` whole.
macro_rules! write {
	($data:expr, $chunks:ident, $size:expr, $scribble:expr, $digest:expr, $out:expr) => {{
		let out: &mut Vec<u8> = $out;
		for (i, chunk) in $data.$chunks($size).enumerate() {
			$scribble(i, chunk);
		}
		out.push($digest(&$data));
		let mut held: Vec<_> = $data.$chunks($size).collect();
		for (i, chunk) in held.iter_mut().enumerate().rev() {
			$scribble(i + 7, chunk);
		}
		out.extend(held.iter().map(|c| $digest(c)));
		// From both ends and by `nth`, writing each as it comes.
		let mut it = $data.$chunks($size);
		out.push(it.len() as u8);
		if let Some(c) = it.next_back() {
			$scribble(1, c);
		}
		if let Some(c) = it.nth(1) {
			$scribble(2, c);
		}
		if let Some(c) = it.nth_back(1) {
			$scribble(3, c);
		}
		out.push(it.len() as u8);
		if let Some(c) = it.last() {
			$scribble(4, c);
		}
		out.push($data.$chunks($size).count() as u8);
		out.push($digest(&$data));
	}};
}

/// `chunks_exact`, `rchunks`, and `rchunks_exact`, and the mutable forms of those and
/// `chunks`, as `kind` picks, of a size it picks too, which is 0 one time in six, which
/// panics.
pub fn chunked(kind: u8, data: &[u8], out: &mut Vec<u8>) {
	let size = (kind / 8 % 6) as usize;
	let mut bytes = data.to_vec();
	let mut words: Vec<u16> = data.iter().map(|&b| b as u16 * 257).collect();
	let mut pairs: Vec<Pair> = data.iter().map(|&b| Pair { a: b, b: b as u16 * 3 }).collect();
	match kind % 8 {
		0 => {
			read!(data.chunks_exact(size), digest, out);
			read!(words.chunks_exact(size), digest_words, out);
			// The remainder, which no part holds, stays as the parts are taken.
			let mut it = data.chunks_exact(size);
			out.push(digest(it.remainder()));
			it.next();
			it.next_back();
			out.push(digest(it.remainder()));
			out.push(digest_words(words.chunks_exact(size).remainder()));
		}
		1 => {
			read!(data.rchunks(size), digest, out);
			read!(words.rchunks(size), digest_words, out);
			// The last part is the front of the slice, the short one.
			out.extend(data.rchunks(size).next_back().map(|c| c.len() as u8));
		}
		2 => {
			read!(data.rchunks_exact(size), digest, out);
			read!(words.rchunks_exact(size), digest_words, out);
			let mut it = data.rchunks_exact(size);
			out.push(digest(it.remainder()));
			it.nth(1);
			out.push(digest(it.remainder()));
			out.push(digest_words(words.rchunks_exact(size).remainder()));
		}
		3 => {
			write!(bytes, chunks_mut, size, scribble, digest, out);
			write!(words, chunks_mut, size, scribble_words, digest_words, out);
			write!(pairs, chunks_mut, size, scribble_pairs, digest_pairs, out);
		}
		4 => {
			write!(bytes, chunks_exact_mut, size, scribble, digest, out);
			write!(words, chunks_exact_mut, size, scribble_words, digest_words, out);
			write!(pairs, chunks_exact_mut, size, scribble_pairs, digest_pairs, out);
			// The remainder, written through after some parts are taken.
			let mut it = bytes.chunks_exact_mut(size);
			it.next();
			let rest = it.into_remainder();
			rest.reverse();
			let mut it = words.chunks_exact_mut(size);
			it.next_back();
			for w in it.into_remainder() {
				*w = w.wrapping_add(1);
			}
			if let Some(p) = pairs.chunks_exact_mut(size).into_remainder().first_mut() {
				*p = Pair { a: 0xee, b: 0xeeee };
			}
		}
		5 => {
			write!(bytes, rchunks_mut, size, scribble, digest, out);
			write!(words, rchunks_mut, size, scribble_words, digest_words, out);
			write!(pairs, rchunks_mut, size, scribble_pairs, digest_pairs, out);
		}
		6 => {
			write!(bytes, rchunks_exact_mut, size, scribble, digest, out);
			write!(words, rchunks_exact_mut, size, scribble_words, digest_words, out);
			write!(pairs, rchunks_exact_mut, size, scribble_pairs, digest_pairs, out);
			let rest = bytes.rchunks_exact_mut(size).into_remainder();
			if let Some(b) = rest.last_mut() {
				*b = 0xee;
			}
			let mut it = words.rchunks_exact_mut(size);
			it.next();
			it.into_remainder().fill(7);
			pairs.rchunks_exact_mut(size).into_remainder().reverse();
		}
		// Chunks of a mutable slice that is itself part of the vector.
		_ => {
			let tail = &mut bytes[data.len() / 3..];
			for (i, chunk) in tail.rchunks_mut(size).enumerate() {
				scribble(i, chunk);
			}
			for (i, chunk) in words.as_mut_slice().chunks_exact_mut(size).rev().enumerate() {
				scribble_words(i, chunk);
			}
		}
	}
	out.extend(&bytes);
	out.extend(words.iter().map(|&w| (w ^ w >> 8) as u8));
	out.push(digest_pairs(&pairs));
}

/// Writes through a mutable subslice, which may be empty, as each element type allows.
fn poke_words(i: usize, s: &mut [u16]) {
	let n = s.len();
	if let Some(w) = s.first_mut() {
		*w = w.wrapping_add(i as u16 + 1);
	}
	if let Some(w) = s.last_mut() {
		*w ^= 0x0f0f;
	}
	s.reverse();
	s.rotate_left(n / 3);
	for (j, w) in s.iter_mut().enumerate() {
		*w = w.wrapping_mul(3).wrapping_add(j as u16);
	}
	if n > 1 {
		s.swap(0, n - 1);
		s[n / 2] = s[n / 2].wrapping_add(0x0101);
	}
}

fn poke_pairs(i: usize, s: &mut [Pair]) {
	if let Some(p) = s.first_mut() {
		p.a = p.a.wrapping_add(i as u8);
	}
	if let Some(p) = s.last_mut() {
		*p = Pair {
			a: p.b as u8,
			b: p.a as u16 * 7,
		};
	}
	s.sort();
	if s.len() > 1 {
		s.swap(0, s.len() / 2);
	}
	for p in s.iter_mut() {
		p.b = p.b.wrapping_add(1);
	}
}

fn poke_array(s: &mut [u32]) {
	for (j, x) in s.iter_mut().enumerate() {
		*x = x.wrapping_mul(31).wrapping_add(j as u32);
	}
	s.reverse();
}

/// `&mut` of a range of a `Vec`, an array, and a slice, of other than bytes, nested, and
/// `split_at_mut`, `split_first_mut`, and `split_last_mut` of those and of bytes, at
/// bounds from the first two bytes, which may be out of bounds or backwards, which
/// panics; with what is written through each part landing in what it came from.
pub fn subslices(kind: u8, data: &[u8], out: &mut Vec<u8>) {
	let [a, b, data @ ..] = data else {
		return;
	};
	let reach = data.len() + 2;
	let (a, b) = (*a as usize % reach, *b as usize % reach);
	let mut bytes = data.to_vec();
	let mut words: Vec<u16> = data.iter().map(|&b| b as u16 * 257).collect();
	let mut pairs: Vec<Pair> = data.iter().map(|&b| Pair { a: b, b: b as u16 * 3 }).collect();
	let mut array = [0u32; 5];
	for (x, &b) in array.iter_mut().zip(data) {
		*x = b as u32 * 65599;
	}
	match kind % 9 {
		0 => {
			poke_words(0, &mut words[a..b]);
			poke_pairs(0, &mut pairs[a..b]);
			poke_array(&mut array[a.min(5)..b.min(5)]);
		}
		1 => {
			poke_words(1, &mut words[..b]);
			poke_words(2, &mut words[a..]);
			poke_pairs(1, &mut pairs[a..]);
			poke_pairs(2, &mut pairs[..]);
			let s = words.as_mut_slice();
			poke_words(3, &mut s[..b]);
			poke_array(&mut array[a..]);
		}
		// A part of a part, and writes to the outer part around the inner one's.
		2 => {
			let outer = &mut words[a / 2..];
			poke_words(4, &mut outer[..b / 2]);
			if let Some(w) = outer.last_mut() {
				*w = 0xbeef;
			}
			let outer = &mut pairs[..b];
			let inner = &mut outer[a / 2..];
			let n = inner.len();
			poke_pairs(3, &mut inner[1.min(n)..]);
			outer.reverse();
		}
		// Both halves written, then an element of one swapped with one of the other.
		3 => {
			let (l, r) = words.split_at_mut(a);
			poke_words(5, l);
			poke_words(6, r);
			if let (Some(x), Some(y)) = (l.last_mut(), r.first_mut()) {
				std::mem::swap(x, y);
			}
			let (l, r) = pairs.split_at_mut(b);
			poke_pairs(4, r);
			poke_pairs(5, l);
			if let (Some(x), Some(y)) = (l.first_mut(), r.last_mut()) {
				std::mem::swap(x, y);
			}
			let (l, r) = array.split_at_mut(a.min(5));
			poke_array(r);
			poke_array(l);
		}
		// At 0 and at the length, where one half is empty, and past it, checked.
		4 => {
			match words.split_at_mut_checked(a) {
				Some((l, r)) => {
					l.reverse();
					poke_words(7, r);
				}
				None => out.push(0xee),
			}
			let n = pairs.len();
			let (l, r) = pairs.split_at_mut(0);
			poke_pairs(6, r);
			out.push(l.len() as u8);
			let (l, r) = pairs.split_at_mut(n);
			poke_pairs(7, l);
			out.push(r.len() as u8);
			let n = words.len();
			let (l, r) = words.split_at_mut(n);
			poke_words(8, l);
			out.push(r.len() as u8);
			// A split of a split's half.
			let (l, r) = words.split_at_mut(a.min(n));
			let (rl, rr) = r.split_at_mut(b.min(n - a.min(n)));
			poke_words(9, rr);
			poke_words(10, l);
			poke_words(11, rl);
		}
		5 => {
			if let Some((first, rest)) = words.split_first_mut() {
				*first = first.wrapping_add(rest.len() as u16);
				poke_words(12, rest);
			}
			if let Some((last, rest)) = words.split_last_mut() {
				*last ^= 0x5a5a;
				if let Some((first, _)) = rest.split_first_mut() {
					*first = *last;
				}
			}
			if let Some((first, rest)) = pairs.split_first_mut() {
				first.b = rest.len() as u16;
				poke_pairs(8, rest);
				if let Some(p) = rest.first_mut() {
					*first = *p;
				}
			}
			if let Some((last, rest)) = pairs.split_last_mut() {
				*last = Pair { a: 1, b: 2 };
				poke_pairs(9, rest);
			}
			if let Some((last, rest)) = array.split_last_mut() {
				*last = rest.len() as u32;
				poke_array(rest);
			}
		}
		// A slice pattern's rest, of a slice and of an array.
		6 => {
			if let [first, mid @ .., last] = words.as_mut_slice() {
				std::mem::swap(first, last);
				poke_words(13, mid);
			}
			if let [rest @ .., _] = pairs.as_mut_slice() {
				poke_pairs(10, rest);
			}
			if let [_, rest @ ..] = &mut array {
				poke_array(rest);
				rest[3] = 7;
			}
			let [_, middle @ .., _] = &mut array;
			middle.swap(0, 2);
		}
		// A part's pointers, which reach the rest of what it is a part of, as one
		// allocation does.
		7 => {
			let n = words.len();
			let (l, r) = (a.min(b).min(n), a.max(b).min(n));
			let at = words[l..r].as_ptr();
			out.push(unsafe { at.offset_from(words.as_ptr()) } as u8);
			if l > 0 {
				out.push(unsafe { *at.sub(1) } as u8);
			}
			let p = words[l..r].as_mut_ptr();
			unsafe {
				if l > 0 {
					*p.sub(1) ^= 0x3c3c;
				}
				if r < n {
					*p.add(r - l) = 0x9999;
				}
			}
			let (front, back) = words.split_at_mut(l);
			poke_words(14, back);
			out.push(unsafe { back.as_ptr().offset_from(front.as_ptr()) } as u8);
		}
		_ => {
			let (l, r) = bytes.split_at_mut(a);
			l.reverse();
			if let Some(x) = r.first_mut() {
				*x ^= 0xff;
			}
			if let Some(x) = l.first_mut() {
				*x = x.wrapping_add(1);
			}
			if let Some((first, rest)) = bytes.split_first_mut() {
				*first = rest.len() as u8;
				rest.reverse();
			}
			if let Some((last, rest)) = bytes.split_last_mut() {
				*last = last.wrapping_mul(3);
				if let Some(x) = rest.last_mut() {
					*x = 0x11;
				}
			}
			match bytes.split_at_mut_checked(b) {
				Some((l, r)) => {
					r.reverse();
					out.push(l.len() as u8);
				}
				None => out.push(0xee),
			}
		}
	}
	out.extend(&bytes);
	out.extend(words.iter().map(|&w| (w ^ w >> 8) as u8));
	out.push(digest_pairs(&pairs));
	out.extend(array.iter().map(|&x| (x ^ x >> 13) as u8));
}

/// `Vec::drain` of a range from the first two bytes, which may be out of bounds or
/// backwards, which panics, of the rest as items, bytes, and integers: what it yields, and
/// what each drops and keeps as the drain drops, or as it is forgotten.
pub fn drains(kind: u8, data: &[u8], out: &mut Vec<u8>) {
	let [a, b, data @ ..] = data else {
		return;
	};
	let reach = data.len() + 2;
	let (a, b) = (*a as usize % reach, *b as usize % reach);
	let mut items: Vec<Item> = data.iter().map(|&tag| Item { tag }).collect();
	let mut bytes = data.to_vec();
	let mut words: Vec<u32> = data.iter().map(|&b| b as u32 + 1000).collect();
	match kind % 8 {
		0 => {
			for x in items.drain(a..b) {
				out.push(x.tag);
				flush(out);
			}
			out.extend(bytes.drain(a..b));
			out.extend(words.drain(a..b).map(|w| w as u8));
		}
		// The rest of the range drops front to back, before the tail moves back.
		1 => {
			let mut drain = items.drain(a..=b);
			out.extend(drain.next().map(|x| x.tag));
			out.extend(drain.next_back().map(|x| x.tag));
			out.push(drain.len() as u8);
			drop(drain);
			flush(out);
			let mut drain = bytes.drain(a..=b);
			out.extend(drain.next_back());
			drop(drain);
			let mut drain = words.drain(a..=b);
			out.extend(drain.next().map(|w| w as u8));
		}
		2 => {
			let taken: Vec<Item> = items.drain(..b).collect();
			out.extend(taken.iter().map(|x| x.tag));
			drop(taken);
			let taken: Vec<u8> = bytes.drain(..b).collect();
			out.extend(taken);
			words.drain(..b);
		}
		// What `nth` and `nth_back` skip drops as they pass it.
		3 => {
			let mut drain = items.drain(a..);
			out.extend(drain.nth(1).map(|x| x.tag));
			flush(out);
			out.extend(drain.nth_back(1).map(|x| x.tag));
			flush(out);
			out.push(drain.len() as u8);
			drop(drain);
			out.extend(bytes.drain(a..).nth(2));
			out.extend(words.drain(a..).rev().map(|w| w as u8));
		}
		4 => {
			out.extend(items.drain(..).last().map(|x| x.tag));
			out.extend(bytes.drain(..).last());
			out.push(words.drain(..).count() as u8);
		}
		// Forgotten, a drain leaves the vector what came before the range, and leaks the rest.
		5 => {
			let mut drain = items.drain(a..b);
			out.extend(drain.next().map(|x| x.tag));
			std::mem::forget(drain);
			let mut drain = bytes.drain(a..b);
			out.extend(drain.next());
			std::mem::forget(drain);
			std::mem::forget(words.drain(a..b));
		}
		6 => out.push(items.drain(a..b).count() as u8),
		_ => {
			let mut kept = vec![Item { tag: 0xee }];
			kept.extend(items.drain(a..b).filter(|x| x.tag % 2 == 0));
			flush(out);
			out.extend(kept.iter().map(|x| x.tag));
			let mut other = vec![0xee];
			other.extend(bytes.drain(a..b).rev());
			out.extend(other);
		}
	}
	flush(out);
	out.extend(items.iter().map(|x| x.tag));
	out.push(items.len() as u8);
	out.extend(&bytes);
	out.extend(words.iter().map(|&w| w as u8));
	items.push(Item { tag: 0xff });
	drop(items);
	flush(out);
}

/// What a part read by `$get` gives: its digest, or 0xee for `None`.
macro_rules! got {
	($get:expr, $digest:expr) => {
		match $get {
			Some(part) => $digest(part),
			None => 0xee,
		}
	};
}

/// A slice's parts by an inclusive range, by a pair of `Bound`s, by `get` and `get_mut` of
/// each kind of range, and by fixed size from either end, of bytes and other values, at
/// bounds from the first two bytes, 0xff being `usize::MAX`, which may be out of bounds or
/// backwards, which panics or gives `None`; with what is written through each mutable
/// part landing in what it came from.
pub fn ranges(kind: u8, data: &[u8], out: &mut Vec<u8>) {
	use std::ops::Bound::{self, Excluded, Included, Unbounded};
	let [a, b, data @ ..] = data else {
		return;
	};
	let reach = data.len() + 2;
	let at = |x: u8| if x == 0xff { usize::MAX } else { x as usize % reach };
	let (a, b) = (at(*a), at(*b));
	let mut bytes = data.to_vec();
	let mut words: Vec<u16> = data.iter().map(|&b| b as u16 * 257).collect();
	let mut pairs: Vec<Pair> = data.iter().map(|&b| Pair { a: b, b: b as u16 * 3 }).collect();
	let mut array = [0u32; 5];
	for (x, &b) in array.iter_mut().zip(data) {
		*x = b as u32 * 65599;
	}
	let digest_array = |s: &[u32]| {
		s.iter()
			.fold(s.len() as u8, |acc, &x| acc.wrapping_mul(31) ^ (x >> 5) as u8)
	};
	match kind % 8 {
		0 => {
			out.push(digest(&bytes[a..=b]));
			out.push(digest_words(&words[a..=b]));
			out.push(digest_pairs(&pairs[..=b]));
			out.push(digest_array(&array[a.min(4)..=b.min(4)]));
			out.push(digest(&data[..=b]));
		}
		1 => {
			let part = &mut bytes[a..=b];
			part.reverse();
			part[0] ^= 0x5a;
			poke_words(0, &mut words[..=b]);
			poke_pairs(1, &mut pairs[a..=b]);
			poke_array(&mut array[..=b.min(4)]);
			let s = words.as_mut_slice();
			poke_words(2, &mut s[a..=b]);
		}
		// Each kind of range by `get`, of bytes, other values, and an array.
		2 => {
			out.push(got!(bytes.get(a..b), digest));
			out.push(got!(bytes.get(a..=b), digest));
			out.push(got!(bytes.get(..b), digest));
			out.push(got!(bytes.get(..=b), digest));
			out.push(got!(bytes.get(a..), digest));
			out.push(got!(bytes.get(..), digest));
			out.push(got!(data.get(a..b), digest));
			out.push(got!(words.get(a..b), digest_words));
			out.push(got!(words.get(a..=b), digest_words));
			out.push(got!(words.get(..=b), digest_words));
			out.push(got!(pairs.get(a..), digest_pairs));
			out.push(got!(pairs.get(..b), digest_pairs));
			out.push(got!(array.get(a..=b), digest_array));
			out.push(got!(array.get(..), digest_array));
		}
		3 => {
			match bytes.get_mut(a..=b) {
				Some(part) => part.reverse(),
				None => out.push(0xee),
			}
			if let Some(part) = bytes.get_mut(..b) {
				part.fill(0x77);
			}
			match words.get_mut(a..b) {
				Some(part) => poke_words(3, part),
				None => out.push(0xee),
			}
			if let Some(part) = words.get_mut(..=b) {
				poke_words(4, part);
			}
			match pairs.get_mut(a..=b) {
				Some(part) => poke_pairs(5, part),
				None => out.push(0xee),
			}
			if let Some(part) = pairs.get_mut(a..) {
				poke_pairs(6, part);
			}
			if let Some(part) = array.get_mut(a..=b) {
				poke_array(part);
			}
		}
		// An inclusive range stepped past its end by `next`, and one a fold exhausts at its
		// end, which then index nothing past it.
		4 => {
			let mut stepped = a..=b;
			for _ in stepped.by_ref().take(8) {}
			let mut folded = a..=b.min(reach);
			let _: usize = folded.by_ref().sum();
			out.extend([stepped.is_empty() as u8, folded.is_empty() as u8]);
			out.push(got!(words.get(stepped.clone()), digest_words));
			out.push(got!(bytes.get(folded.clone()), digest));
			out.push(got!(pairs.get_mut(folded.clone()), |p: &mut [Pair]| digest_pairs(p)));
			poke_words(7, &mut words[folded.clone()]);
			out.push(digest(&bytes[stepped]));
			out.push(digest(&data[folded]));
		}
		// A pair of `Bound`s, each kind at either end.
		5 => {
			let bound = |x: usize, k: u8| -> Bound<usize> {
				match k % 3 {
					0 => Included(x),
					1 => Excluded(x),
					_ => Unbounded,
				}
			};
			let (lo, hi) = (bound(a, data.len() as u8), bound(b, kind / 8));
			out.push(got!(bytes.get((lo, hi)), digest));
			out.push(got!(pairs.get((lo, hi)), digest_pairs));
			if let Some(part) = words.get_mut((lo, hi)) {
				poke_words(8, part);
			}
			poke_pairs(9, &mut pairs[(lo, hi)]);
			out.push(digest(&bytes[(lo, hi)]));
		}
		// Fixed-size parts, of none, one, three, and four, from either end.
		6 => {
			out.extend(bytes.first_chunk::<0>().map(|c| digest(c)));
			out.extend(bytes.first_chunk::<3>().map(|c| digest(c)));
			out.extend(data.last_chunk::<1>().map(|c| digest(c)));
			out.extend(data.last_chunk::<4>().map(|c| digest(c)));
			out.extend(words.first_chunk::<4>().map(|c| digest_words(c)));
			out.extend(words.last_chunk::<0>().map(|c| digest_words(c)));
			out.extend(pairs.last_chunk::<3>().map(|c| digest_pairs(c)));
			if let Some((c, rest)) = bytes.split_first_chunk::<1>() {
				out.extend([digest(c), digest(rest)]);
			}
			if let Some((rest, c)) = bytes.split_last_chunk::<4>() {
				out.extend([digest(rest), digest(c)]);
			}
			if let Some((c, rest)) = words.split_first_chunk::<0>() {
				out.extend([digest_words(c), digest_words(rest)]);
			}
			if let Some((rest, c)) = pairs.split_last_chunk::<3>() {
				out.extend([digest_pairs(rest), digest_pairs(c)]);
			}
			if let Some((rest, c)) = array.split_last_chunk::<5>() {
				out.extend([digest_array(rest), digest_array(c)]);
			}
		}
		_ => {
			if let Some(c) = bytes.first_chunk_mut::<3>() {
				c[0] = c[2].wrapping_add(1);
				c.reverse();
			}
			if let Some(c) = bytes.last_chunk_mut::<1>() {
				c[0] ^= 0xff;
			}
			if let Some(c) = words.first_chunk_mut::<4>() {
				poke_words(10, c);
			}
			if let Some(c) = words.last_chunk_mut::<0>() {
				poke_words(11, c);
			}
			if let Some(c) = pairs.last_chunk_mut::<3>() {
				poke_pairs(12, c);
				c[0].b = 9;
			}
			if let Some((c, rest)) = bytes.split_first_chunk_mut::<1>() {
				c[0] = rest.len() as u8;
				rest.reverse();
			}
			if let Some((rest, c)) = words.split_last_chunk_mut::<4>() {
				poke_words(13, rest);
				c.swap(0, 3);
				c[1] = 0xabcd;
			}
			if let Some((c, rest)) = pairs.split_first_chunk_mut::<0>() {
				poke_pairs(14, c);
				poke_pairs(15, rest);
			}
			if let Some((c, rest)) = pairs.split_first_chunk_mut::<2>() {
				std::mem::swap(&mut c[0], &mut rest[0]);
				c[1].a = 3;
			}
			if let Some((rest, c)) = array.split_last_chunk_mut::<2>() {
				c[1] = rest.len() as u32;
				poke_array(rest);
			}
		}
	}
	out.extend(&bytes);
	out.extend(words.iter().map(|&w| (w ^ w >> 8) as u8));
	out.push(digest_pairs(&pairs));
	out.extend(array.iter().map(|&x| (x ^ x >> 13) as u8));
}

/// Logs its tag with the high bit set as it is cloned, into a clone one higher, and logs
/// its tag as it drops.
struct Noisy {
	tag: u8,
}

impl Clone for Noisy {
	fn clone(&self) -> Self {
		LOG.with_borrow_mut(|log| log.push(self.tag | 0x80));
		Noisy {
			tag: self.tag.wrapping_add(1),
		}
	}
}

impl Drop for Noisy {
	fn drop(&mut self) {
		LOG.with_borrow_mut(|log| log.push(self.tag));
	}
}

/// A `Copy` element whose own `Clone` logs, which `fill` and `clone_from_slice` call and
/// `copy_within` and `copy_from_slice` do not.
#[derive(Copy)]
struct Counted {
	n: u8,
}

impl Clone for Counted {
	fn clone(&self) -> Self {
		LOG.with_borrow_mut(|log| log.push(self.n ^ 0x40));
		*self
	}
}

/// A slice's `fill`, `fill_with`, `rotate_left`, `rotate_right`, `copy_within`,
/// `copy_from_slice`, and `clone_from_slice`, of bytes, other integers, objects that are
/// `Copy`, a `Copy` whose `Clone` logs, values that log as they are cloned and drop, and
/// options of them, on the whole slice and on mutable parts and chunks of it, at bounds
/// from the first three bytes, which may be out of bounds or backwards, which panics.
pub fn edits(kind: u8, data: &[u8], out: &mut Vec<u8>) {
	let [a, b, c, data @ ..] = data else {
		return;
	};
	let reach = data.len() + 2;
	let (a, b, c) = (*a as usize % reach, *b as usize % reach, *c as usize % reach);
	let size = c % 3 + 1;
	let mut bytes = data.to_vec();
	let mut words: Vec<u16> = data.iter().map(|&b| b as u16 * 257).collect();
	let mut pairs: Vec<Pair> = data.iter().map(|&b| Pair { a: b, b: b as u16 * 3 }).collect();
	let mut counted: Vec<Counted> = data.iter().map(|&n| Counted { n }).collect();
	let mut items: Vec<Noisy> = data.iter().map(|&tag| Noisy { tag }).collect();
	let mut array = [0u32; 5];
	for (x, &b) in array.iter_mut().zip(data) {
		*x = b as u32 * 65599;
	}
	match kind % 8 {
		// Each slot but the last a clone, the last the value, and each old one dropped
		// after its slot's clone; an empty slice drops the value.
		0 => {
			bytes.fill(c as u8);
			bytes[a..].fill(0x5a);
			words[..b].fill(c as u16 * 0x0101);
			pairs.fill(Pair { a: 1, b: 2 });
			if let Some(p) = pairs.first_mut() {
				p.a = 9;
			}
			// The value is the caller's, which the last slot is a copy of.
			let pair = Pair { a: 3, b: c as u16 };
			pairs[a..b].fill(pair);
			if let Some(p) = pairs.get_mut(b.wrapping_sub(1)) {
				p.b ^= 1;
			}
			out.push(pair.b as u8);
			counted[..b].fill(Counted { n: 0x11 });
			flush(out);
			items[a..].fill(Noisy { tag: 0x20 });
			flush(out);
			items[..0].fill(Noisy { tag: 0x30 });
			flush(out);
			items.fill(Noisy { tag: c as u8 });
			flush(out);
			array[a.min(5)..].fill(c as u32);
			let mut opts: Vec<Option<Noisy>> = data.iter().map(|&tag| Some(Noisy { tag })).collect();
			opts[a..].fill(None);
			flush(out);
			opts.fill(Some(Noisy { tag: 0x40 }));
			flush(out);
			out.extend(opts.iter().map(|o| o.as_ref().map_or(0xee, |x| x.tag)));
		}
		// In slot order, each value made before its slot's old one drops.
		1 => {
			let mut n = c as u8;
			bytes[..b].fill_with(|| {
				n = n.wrapping_add(3);
				n
			});
			words[a..].fill_with(|| {
				n = n.wrapping_mul(5);
				n as u16
			});
			pairs.fill_with(|| {
				n ^= 0x21;
				Pair { a: n, b: n as u16 }
			});
			counted[a..b].fill_with(|| Counted { n: 7 });
			flush(out);
			let mut tag = 0x60;
			items[a..].fill_with(|| {
				LOG.with_borrow_mut(|log| log.push(0xfe));
				tag += 1;
				Noisy { tag }
			});
			flush(out);
			// The function, holding a value that drops, drops once the slots are full.
			let held = Noisy { tag: 0x70 };
			items.fill_with(move || {
				let held = &held;
				Noisy { tag: held.tag ^ 1 }
			});
			flush(out);
			out.push(0x99);
			array.fill_with(Default::default);
		}
		2 => {
			bytes.rotate_left(c);
			words.rotate_right(a);
			pairs[a..].rotate_left(b);
			counted.rotate_right(c);
			items.rotate_left(c);
			flush(out);
			array.rotate_right(c);
			bytes[..b].rotate_right(a);
		}
		// Rotations of chunks, halves, and nothing, by 0 and by the length.
		3 => {
			for (i, chunk) in words.chunks_mut(size).enumerate() {
				chunk.rotate_left(i % (chunk.len() + 1));
			}
			for chunk in pairs.chunks_exact_mut(size) {
				chunk.rotate_right(1);
			}
			let (l, r) = items.split_at_mut(a);
			l.rotate_right(0);
			let n = r.len();
			r.rotate_left(n);
			r.rotate_right(b.min(n));
			flush(out);
			let (l, r) = bytes.split_at_mut(b.min(data.len()));
			l.rotate_left(l.len());
			r.rotate_right(r.len() / 2);
			array[1..].rotate_left(c);
		}
		// Overlapping either way, and of a `Copy` object, which each copy is its own of.
		4 => {
			bytes.copy_within(a..b, c);
			words.copy_within(a.., c);
			pairs.copy_within(..b, c);
			if let Some(p) = pairs.get_mut(c) {
				p.a ^= 0xff;
			}
			counted.copy_within(a..=b, c);
			flush(out);
			array.copy_within(a.min(5).., c.min(5));
			let part = &mut words[a / 2..];
			part.copy_within(b / 2.., 0);
			bytes.copy_within((std::ops::Bound::Excluded(a), std::ops::Bound::Unbounded), c);
		}
		// Of a part of the same slice, and of another, of a length that may not match.
		5 => {
			let (l, r) = words.split_at_mut(a);
			let n = l.len().min(r.len());
			l[..n].copy_from_slice(&r[..n]);
			let (l, r) = pairs.split_at_mut(b);
			let n = l.len().min(r.len());
			r[..n].copy_from_slice(&l[..n]);
			if let Some(p) = l.first_mut() {
				p.b = 0xbeef;
			}
			let other: Vec<Counted> = (0..c).map(|n| Counted { n: n as u8 }).collect();
			counted[a..].copy_from_slice(&other[..data.len() - a]);
			flush(out);
			let mut more = [0u32; 3];
			more.copy_from_slice(&array[..3]);
			array[2..].copy_from_slice(&more);
			let source = vec![Pair { a: 4, b: 5 }; c];
			pairs[..b].copy_from_slice(&source);
		}
		// Each slot's clone in order, each old one dropped after its clone; of a length
		// that may not match, which panics before any clone.
		6 => {
			let source: Vec<Noisy> = (0..c).map(|tag| Noisy { tag: tag as u8 | 0x50 }).collect();
			let n = data.len().min(c);
			items[..n].clone_from_slice(&source[..n]);
			flush(out);
			let (l, r) = counted.split_at_mut(a);
			let n = l.len().min(r.len());
			l[..n].clone_from_slice(&r[..n]);
			flush(out);
			let (l, r) = pairs.split_at_mut(a);
			let n = l.len().min(r.len());
			r[..n].clone_from_slice(&l[..n]);
			if let Some(p) = l.first_mut() {
				p.b = 0xbeef;
			}
			words[a..].clone_from_slice(&[7, 8, 9][..b.min(3)]);
			items[a..].clone_from_slice(&source[..b.min(c)]);
			flush(out);
			pairs[..c].clone_from_slice(&[Pair { a: 1, b: 1 }; 2]);
		}
		// Each edit on each mutable chunk, which lands in the slice.
		_ => {
			for (i, chunk) in pairs.chunks_mut(size).enumerate() {
				match i % 3 {
					0 => chunk.fill(Pair { a: i as u8, b: 0 }),
					1 => chunk.copy_within(1.., 0),
					_ => chunk.rotate_left(1),
				}
			}
			let mut it = items.rchunks_exact_mut(size);
			if let (Some(x), Some(y)) = (it.next(), it.next()) {
				x.clone_from_slice(y);
				y.rotate_right(1);
				y.fill(Noisy { tag: 0x0f });
			}
			flush(out);
			for chunk in words.chunks_exact_mut(size) {
				chunk.fill_with(|| a as u16);
			}
			if let Some((first, rest)) = counted.split_first_chunk_mut::<1>() {
				first.copy_from_slice(&rest[..1.min(rest.len())]);
			}
			flush(out);
		}
	}
	flush(out);
	out.extend(&bytes);
	out.extend(words.iter().map(|&w| (w ^ w >> 8) as u8));
	out.push(digest_pairs(&pairs));
	out.extend(counted.iter().map(|x| x.n));
	out.extend(items.iter().map(|x| x.tag));
	out.extend(array.iter().map(|&x| (x ^ x >> 13) as u8));
	drop(items);
	flush(out);
}

/// How many runs of `N` `data` holds, by `as_chunks`, which panics at none.
fn runs<const N: usize>(data: &[u8]) -> u8 {
	data.as_chunks::<N>().0.len() as u8
}

/// `as_chunks` and `as_rchunks` at one, two, and three elements, of bytes, of other
/// integers, and of values that log as they drop, of a slice, an empty one, a part, and a
/// chunk, and at none, which panics; and a `Vec` cloned from a shared and a mutable slice
/// and array, which later writes to what it cloned leave as they were.
pub fn arrays(kind: u8, data: &[u8], out: &mut Vec<u8>) {
	let mut bytes = data.to_vec();
	let mut words: Vec<u16> = data.iter().map(|&b| b as u16 * 257).collect();
	let mut items: Vec<Noisy> = data.iter().map(|&tag| Noisy { tag }).collect();
	let tags = |s: &[Noisy]| s.iter().fold(s.len() as u8, |acc, x| acc.wrapping_mul(31) ^ x.tag);
	match kind % 4 {
		0 => {
			let (chunks, rest) = data.as_chunks::<2>();
			out.extend(chunks.iter().map(|c| digest(c)));
			out.extend([chunks.len() as u8, digest(rest)]);
			let (rest, chunks) = data.as_rchunks::<3>();
			out.extend(chunks.iter().map(|c| c[0] ^ c[2]));
			out.extend([chunks.len() as u8, digest(rest)]);
			let (chunks, rest) = words.as_chunks::<3>();
			out.extend(chunks.iter().map(|c| digest_words(c)));
			out.extend([chunks.len() as u8, digest_words(rest)]);
			let (rest, chunks) = words.as_rchunks::<1>();
			out.extend(chunks.iter().map(|[w]| (w >> 4) as u8));
			out.push(rest.len() as u8);
			let (chunks, rest) = items.as_chunks::<3>();
			out.extend(chunks.iter().map(|c| tags(c)));
			out.extend([chunks.len() as u8, tags(rest)]);
			let (rest, chunks) = items.as_rchunks::<2>();
			out.extend(chunks.iter().map(|c| tags(c)));
			out.extend([chunks.len() as u8, tags(rest)]);
		}
		// An empty slice, a part, and a chunk of a chunk.
		1 => {
			let (none, rest) = data[..0].as_chunks::<4>();
			out.extend([none.len() as u8, rest.len() as u8]);
			let (rest, none) = items[..0].as_rchunks::<2>();
			out.extend([none.len() as u8, rest.len() as u8]);
			let start = data.len() / 3;
			let (chunks, rest) = data[start..].as_chunks::<3>();
			for c in chunks {
				let (pairs, last) = c.as_chunks::<2>();
				out.extend([pairs.len() as u8, pairs[0][1] ^ last[0]]);
			}
			out.push(digest(rest));
			let (rest, chunks) = words[start..].as_rchunks::<2>();
			out.extend(chunks.last().map(|c| digest_words(c)));
			out.push(digest_words(rest));
			// A run of bytes is where the slice holds it, and a copy of one is its own.
			let (a, _) = bytes.as_chunks::<2>();
			let (_, b) = bytes[1..].as_rchunks::<2>();
			let same = [
				a.first()
					.zip(bytes.first_chunk::<2>())
					.is_some_and(|(x, y)| std::ptr::eq(x, y)),
				a.first()
					.zip(bytes.as_chunks::<2>().0.first())
					.is_some_and(|(x, y)| std::ptr::eq(x, y)),
				b.last().zip(a.last()).is_some_and(|(x, y)| std::ptr::eq(x, y)),
			];
			out.extend(same.map(u8::from));
			let kept = a.first().copied();
			if let Some(x) = bytes.first_mut() {
				*x ^= 0xff;
			}
			out.extend(kept.into_iter().flatten());
			// A copy of a run of other values is its own too.
			let kept = words.as_chunks::<2>().0.first().copied();
			if let Some(x) = words.first_mut() {
				*x ^= 0xffff;
			}
			out.extend(kept.map(|c| digest_words(&c)));
		}
		2 => {
			let owned = Vec::from(data);
			let copied = Vec::from(&mut words[..]);
			let cloned = Vec::from(&items[..]);
			let more = Vec::from(&mut items[1.min(data.len())..]);
			let array = [Noisy { tag: 0x31 }, Noisy { tag: 0x32 }];
			let mut array_bytes = [data.len() as u8, 7];
			let from_array = (Vec::from(&array), Vec::from(&mut array_bytes));
			flush(out);
			bytes.reverse();
			words.fill(3);
			items.clear();
			array_bytes[0] = 0;
			flush(out);
			out.extend([digest(&owned), digest_words(&copied), tags(&cloned), tags(&more)]);
			out.extend([tags(&from_array.0), digest(&from_array.1), array_bytes[0]]);
			drop(cloned);
			drop(from_array);
			flush(out);
		}
		_ => out.push(runs::<0>(data)),
	}
	flush(out);
	out.extend(&bytes);
	out.extend(words.iter().map(|&w| (w ^ w >> 8) as u8));
	out.push(tags(&items));
	drop(items);
	flush(out);
}

/// How many runs of `N` `data` holds, by `as_chunks_mut`, which panics at none.
fn runs_mut<const N: usize>(data: &mut [u8]) -> u8 {
	data.as_chunks_mut::<N>().0.len() as u8
}

/// Whole writes into a slice's runs as arrays, each read by value before them: swapped,
/// reversed, rotated, sorted, copied within, filled, assigned, and replaced.
macro_rules! rewrite {
	($chunks:expr, $out:expr, $digest:expr, $one:expr, $two:expr) => {{
		let chunks = $chunks;
		let out: &mut Vec<u8> = $out;
		if let [first, .., last] = &*chunks {
			let (first, last) = (*first, *last);
			chunks.swap(0, chunks.len() - 1);
			chunks.reverse();
			let n = chunks.len();
			chunks[1..].reverse();
			chunks.rotate_left(1);
			chunks.rotate_right(n / 2);
			let kept = chunks[n - 1];
			chunks.copy_within(..n - 1, 1);
			out.extend([first, last, kept].iter().map($digest));
			chunks.sort();
			out.extend(chunks.iter().map($digest));
			chunks.sort_by_key(|c| std::cmp::Reverse(c[1]));
			chunks.sort_unstable();
			chunks.sort_by_cached_key(|c| std::cmp::Reverse(c[0]));
			let (head, tail) = chunks.split_at_mut(1);
			std::mem::swap(&mut head[0], &mut tail[n - 2]);
			let old = std::mem::replace(&mut chunks[0], $one);
			chunks[n - 1] = old;
			let kept = chunks[1];
			chunks[1] = chunks[0];
			chunks[0] = kept;
			if let Some(c) = chunks.last_mut() {
				*c = $two;
			}
			out.extend(chunks.iter().map($digest));
			chunks[n / 2..].fill($two);
		}
	}};
}

/// `as_chunks_mut` and `as_rchunks_mut` of bytes, of other integers, of values that log as
/// they drop, and of runs of bytes: written by element as moq writes TS packets, by enumerating, zipping, and
/// filtering them, and whole after a read of a run by value, which keeps what it read, as
/// does one the slice's own writes follow; and at none, which panics.
pub fn arrays_mut(kind: u8, data: &[u8], out: &mut Vec<u8>) {
	let mut bytes = data.to_vec();
	let mut words: Vec<u16> = data.iter().map(|&b| b as u16 * 257).collect();
	let mut items: Vec<Item> = data.iter().map(|&tag| Item { tag }).collect();
	let tags = |s: &[Item]| s.iter().fold(s.len() as u8, |acc, x| acc.wrapping_mul(31) ^ x.tag);
	match kind % 5 {
		0 => {
			let (chunks, rest) = bytes.as_chunks_mut::<3>();
			for (i, c) in chunks.iter_mut().enumerate() {
				c[0] ^= i as u8;
				let swapped = [c[2], c[1]];
				c[1..].copy_from_slice(&swapped);
			}
			for (c, &b) in chunks.iter_mut().rev().zip(data) {
				c[2] = c[2].wrapping_add(b);
			}
			let mut odd = chunks.iter_mut().filter(|c| c[0] & 1 != 0).peekable();
			if let Some(c) = odd.peek_mut() {
				c[1] |= 0x80;
			}
			for c in odd {
				c[0] = !c[0];
			}
			rest.fill(1);
			let (rest, chunks) = words.as_rchunks_mut::<2>();
			for (i, [a, b]) in chunks.iter_mut().enumerate() {
				*a ^= i as u16;
				*b = b.rotate_left(3);
			}
			rest.fill(1);
			let start = data.len() / 3;
			for c in bytes[start..].as_chunks_mut::<2>().0 {
				c.swap(0, 1);
			}
			// A read by value keeps what it read past the slice's own writes.
			let kept = bytes.as_chunks_mut::<2>().0.first().copied();
			let kept_words = words.as_chunks_mut::<2>().0.first().copied();
			bytes.fill(9);
			words.fill(9);
			out.extend(kept.into_iter().flatten());
			out.extend(kept_words.into_iter().flatten().map(|w| (w ^ w >> 8) as u8));
		}
		1 => rewrite!(bytes.as_chunks_mut::<2>().0, out, |c| digest(c), [7, 8], [9, 10]),
		2 => {
			rewrite!(words.as_rchunks_mut::<2>().1, out, |c| digest_words(c), [7, 8], [9, 10]);
			let (_, chunks) = bytes.as_rchunks_mut::<4>();
			rewrite!(chunks, out, |c| digest(c), [1, 2, 3, 4], [5; 4]);
			// Runs of runs, and of runs of runs, whose inner runs are views of the bytes too.
			let (pairs, _) = bytes.as_chunks_mut::<2>();
			let (quads, _) = pairs.as_chunks_mut::<2>();
			rewrite!(quads, out, |c| c[0][0] ^ c[1][1], [[1, 2], [3, 4]], [[5, 6], [7, 8]]);
			let (pairs, _) = bytes.as_chunks_mut::<2>();
			let (quads, _) = pairs.as_chunks_mut::<2>();
			let (octets, _) = quads.as_chunks_mut::<2>();
			let one = [[[1, 2], [3, 4]], [[5, 6], [7, 8]]];
			rewrite!(octets, out, |c| c[0][0][0] ^ c[1][1][1], one, [[[9; 2]; 2]; 2]);
			// Runs of runs of references, which a whole write moves and never copies.
			let mut backing = words.clone();
			let mut refs: Vec<&mut [u16]> = backing.chunks_mut(1).collect();
			let (pairs, _) = refs.as_chunks_mut::<2>();
			let (quads, _) = pairs.as_chunks_mut::<2>();
			if quads.len() >= 2 {
				quads.swap(0, 1);
			}
			for (i, r) in refs.iter_mut().enumerate() {
				r[0] ^= i as u16;
			}
			out.extend(backing.iter().map(|&w| (w ^ w >> 8) as u8));
			let (pairs, _) = words.as_chunks_mut::<2>();
			let (quads, _) = pairs.as_chunks_mut::<2>();
			rewrite!(
				quads,
				out,
				|c| (c[0][0] ^ c[1][1]) as u8,
				[[1, 2], [3, 4]],
				[[5, 6], [7, 8]]
			);
		}
		3 => {
			let (chunks, _) = items.as_chunks_mut::<2>();
			if chunks.len() >= 2 {
				let first = std::mem::replace(&mut chunks[0], [Item { tag: 0x41 }, Item { tag: 0x42 }]);
				flush(out);
				chunks.swap(0, 1);
				chunks.reverse();
				chunks.sort_by_key(|c| c[1].tag);
				chunks[1] = first;
				flush(out);
				chunks[0][1] = Item { tag: 0x43 };
				flush(out);
				let (head, tail) = chunks.split_at_mut(1);
				std::mem::swap(&mut head[0], &mut tail[0]);
				out.extend(chunks.iter().map(|c| tags(c)));
			}
		}
		_ => {
			let (none, rest) = bytes[..0].as_chunks_mut::<4>();
			out.extend([none.len() as u8, rest.len() as u8]);
			out.push(runs_mut::<0>(&mut bytes));
		}
	}
	flush(out);
	out.extend(&bytes);
	out.extend(words.iter().map(|&w| (w ^ w >> 8) as u8));
	out.push(tags(&items));
	drop(items);
	flush(out);
}

/// Arrays of two bytes a constant holds.
const PAIRS: &[[u8; 2]] = &[[1, 2], [3, 4]];

/// A generic struct's slice, instanced with arrays.
struct View<'a, T> {
	view: &'a [T],
}

impl<T: Copy> View<'_, T> {
	/// Its last element, if any.
	fn last(&self) -> Option<T> {
		self.view.last().copied()
	}
}

/// Puts each run's bytes.
fn put_runs<const N: usize>(runs: &[[u8; N]], out: &mut Vec<u8>) {
	out.push(runs.len() as u8);
	for run in runs {
		out.extend_from_slice(run);
	}
}

/// Slices of arrays in each form they take, each read after.
pub fn array_forms(kind: u8, data: &[u8], out: &mut Vec<u8>) {
	let byte = |at: usize| data.get(at).copied().unwrap_or(at as u8);
	let pairs: Vec<[u8; 2]> = data.chunks_exact(2).map(|c| [c[0], c[1]]).collect();
	match kind % 8 {
		// A reference copied out of an array of one, or out of runs of them, still points at
		// the cell it did.
		0 => {
			let cell = std::cell::Cell::new(byte(0));
			let other = std::cell::Cell::new(byte(1));
			let mut r = &cell;
			let a = std::array::from_mut(&mut r);
			let copy = *a;
			let runs = std::array::from_mut(a);
			let [[old]] = std::mem::replace(runs, [[&other]]);
			cell.set(byte(2));
			out.extend([copy[0].get(), old.get(), r.get()]);
			let mut refs = [[&cell], [&other]];
			refs.swap(0, 1);
			other.set(byte(3));
			out.extend([refs[0][0].get(), refs[1][0].get()]);
		}
		// Clones of slices of arrays, of values a clone copies and of others.
		1 => {
			let words: Vec<[String; 2]> = pairs.iter().map(|p| [p[0].to_string(), p[1].to_string()]).collect();
			let cloned = words[words.len().min(1)..].to_vec();
			out.push(cloned.len() as u8);
			for [a, b] in &cloned {
				out.extend(a.bytes().chain(b.bytes()));
			}
			let boxed: Box<[[u8; 2]]> = pairs.clone().into_boxed_slice();
			let copy = boxed.clone();
			put_runs(&copy, out);
			put_runs(&boxed.into_vec(), out);
		}
		// A constant's, a default, and an `Option`'s.
		2 => {
			put_runs(PAIRS, out);
			let empty: &[[u8; 2]] = Default::default();
			put_runs(empty, out);
			let some = pairs.first().copied();
			put_runs(some.as_slice(), out);
			put_runs(None::<[u8; 2]>.as_slice(), out);
		}
		// A generic struct's slice, of byte arrays and of others.
		3 => {
			let bytes = View { view: &pairs[..] };
			out.extend(bytes.last().unwrap_or_default());
			let words: Vec<[u16; 2]> = pairs.iter().map(|p| [p[0].into(), p[1].into()]).collect();
			let words = View { view: &words[..] };
			out.extend(words.last().unwrap_or_default().map(|w| w as u8));
		}
		// Arrays of wrapped arrays, which are elements of their own, copied through a borrow.
		4 => {
			let wrapped = [std::mem::MaybeUninit::new([byte(0), byte(1)])];
			let r = &wrapped;
			let copy = *r;
			// SAFETY: the one element is initialized.
			out.extend(unsafe { copy[0].assume_init() });
			let kept = [std::mem::ManuallyDrop::new([byte(2), byte(3)])];
			let r = &kept;
			let copy = *r;
			out.extend(*copy[0]);
		}
		// A `Cow`'s, borrowed and owned.
		5 => {
			let borrowed: std::borrow::Cow<[[u8; 2]]> = std::borrow::Cow::Borrowed(&pairs[..]);
			put_runs(&borrowed, out);
			let mut owned = borrowed.clone();
			if let Some(first) = owned.to_mut().first_mut() {
				first[0] ^= 0xff;
			}
			put_runs(&owned, out);
			put_runs(&borrowed, out);
			put_runs(&borrowed.into_owned(), out);
			let from: std::borrow::Cow<[[u8; 2]]> = pairs.clone().into();
			put_runs(&from, out);
		}
		// An `Option`'s array written through its slice, a borrowed `Vec`'s `Cow`, initialized
		// `MaybeUninit`s of arrays, shared, mutable, and boxed, and slices of arrays joined.
		6 => {
			let mut some = pairs.first().copied();
			if let Some(run) = some.as_mut_slice().first_mut() {
				run[1] ^= 0x5a;
			}
			put_runs(some.as_slice(), out);
			out.push(None::<[u8; 2]>.as_mut_slice().len() as u8);
			let cow: std::borrow::Cow<'_, [[u8; 2]]> = std::borrow::Cow::from(&pairs);
			put_runs(&cow, out);
			let mut wrapped: Vec<std::mem::MaybeUninit<[u8; 2]>> =
				pairs.iter().map(|&p| std::mem::MaybeUninit::new(p)).collect();
			// SAFETY: every element is initialized.
			put_runs(unsafe { wrapped.assume_init_ref() }, out);
			// SAFETY: every element is initialized.
			if let Some(run) = unsafe { wrapped.assume_init_mut() }.last_mut() {
				run[0] = run[0].wrapping_add(1);
			}
			// SAFETY: every element is initialized.
			put_runs(unsafe { wrapped.assume_init_ref() }, out);
			let mut boxed = Box::<[[u8; 2]]>::new_uninit_slice(pairs.len());
			for (slot, &p) in boxed.iter_mut().zip(&pairs) {
				slot.write(p);
			}
			// SAFETY: every element is written.
			put_runs(&unsafe { boxed.assume_init() }, out);
			let half = pairs.len() / 2;
			let joined = [&pairs[..half], &pairs[half..]].concat();
			put_runs(&joined, out);
			let cows = [
				std::borrow::Cow::Borrowed(&pairs[..half]),
				std::borrow::Cow::Owned(pairs[half..].to_vec()),
			];
			put_runs(&cows.concat(), out);
			put_runs(&[&pairs[..half], &pairs[half..]].join(&[0xee_u8, 0xff]), out);
			put_runs(
				&[&pairs[..half], &pairs[half..]].join(&[[0xdd_u8, 0xcc], [0xbb, 0xaa]][..]),
				out,
			);
		}
		// `mem::replace` and `Cell` of a run whose elements a write lands in.
		_ => {
			let mut item = vec![byte(0)];
			let a = std::array::from_mut(&mut item);
			let runs = std::slice::from_mut(a);
			let [old] = std::mem::replace(&mut runs[0], [vec![byte(1), byte(2)]]);
			out.extend(&old);
			let [[old]] = std::cell::Cell::from_mut(std::array::from_mut(std::array::from_mut(&mut item)))
				.replace([[vec![byte(3)]]]);
			out.extend(&old);
			out.extend(&item);
		}
	}
}

/// What logs, as it is cloned, whether it is the one its `home` points at.
struct Probe<'a> {
	tag: u8,
	home: &'a std::cell::Cell<Option<&'a Probe<'a>>>,
}

impl Clone for Probe<'_> {
	fn clone(&self) -> Self {
		let home = self.home.get().is_some_and(|home| std::ptr::eq(home, self));
		LOG.with_borrow_mut(|log| log.push(self.tag ^ home as u8));
		Probe {
			tag: self.tag,
			home: self.home,
		}
	}
}

/// Whether a callback std lends each row of `rows` gets that row itself, by `ptr::eq`.
fn lent_rows<const N: usize>(rows: &[[u8; N]], out: &mut Vec<u8>) {
	for target in rows {
		let mut lent = false;
		let found = rows.binary_search_by(|row| {
			lent |= std::ptr::eq(row, target);
			row.cmp(target)
		});
		out.extend([found.is_ok() as u8, lent as u8]);
	}
}

/// The same as [`lent_rows`], for rows of words and of arrays.
fn lent_words(rows: &[[u16; 2]], nested: &[[[u8; 2]; 2]], out: &mut Vec<u8>) {
	for target in rows {
		out.push(rows.iter().filter(|row| std::ptr::eq(*row, target)).count() as u8);
		out.push(rows.partition_point(|row| !std::ptr::eq(row, target) && row < target) as u8);
		let mut lent = false;
		let _ = rows.binary_search_by(|row| {
			lent |= std::ptr::eq(row, target);
			row.cmp(target)
		});
		out.push(lent as u8);
	}
	for target in nested {
		let mut lent = false;
		let _ = nested.binary_search_by(|row| {
			lent |= std::ptr::eq(row, target);
			row.cmp(target)
		});
		out.push(lent as u8);
	}
}

/// Rows of slices of arrays as std reads them: lent to a callback as themselves, and a
/// `join`'s separator cloned where it is.
pub fn array_identity(kind: u8, data: &[u8], out: &mut Vec<u8>) {
	let mut rows: Vec<[u8; 2]> = data.chunks_exact(2).map(|c| [c[0], c[1]]).collect();
	rows.sort();
	rows.dedup();
	match kind % 2 {
		// A callback's `ptr::eq` on a row std lends, of all the rows and of part of them.
		0 => {
			lent_rows(&rows, out);
			lent_rows(&rows[rows.len() / 2..], out);
			let words: Vec<[u16; 2]> = rows.iter().map(|r| [r[0].into(), r[1].into()]).collect();
			let nested: Vec<[[u8; 2]; 2]> = rows.windows(2).map(|w| [w[0], w[1]]).collect();
			lent_words(&words[words.len().min(1)..], &nested[nested.len() / 2..], out);
		}
		// `join`'s separator, an array, cloned by each element's `Clone` where it is.
		_ => {
			let away = std::cell::Cell::new(None);
			let probes = |at: usize| -> Vec<[Probe; 1]> {
				rows[at..]
					.iter()
					.map(|r| {
						[Probe {
							tag: r[0] & 0xfe,
							home: &away,
						}]
					})
					.collect()
			};
			let (a, b) = (probes(0), probes(rows.len() / 2));
			let home = std::cell::Cell::new(None);
			let sep = [Probe { tag: 0xee, home: &home }];
			home.set(Some(&sep[0]));
			let joined = [&a[..], &b[..], &a[..1.min(a.len())]].join(&sep);
			out.extend(joined.iter().map(|[p]| p.tag));
			flush(out);
		}
	}
}

/// Slices of arrays std's collections and pointers hand out, each read and written after.
pub fn array_sources(kind: u8, data: &[u8], out: &mut Vec<u8>) {
	let rows: Vec<[u8; 2]> = data.chunks_exact(2).map(|c| [c[0], c[1]]).collect();
	match kind % 5 {
		// A deque's, shared, mutable, and made contiguous, of bytes and of words.
		0 => {
			let mut q: std::collections::VecDeque<[u8; 2]> = rows.iter().copied().collect();
			if let Some(&first) = rows.first() {
				q.push_front(first);
				q.rotate_left(q.len() / 2);
			}
			let (a, b) = q.as_slices();
			out.extend([a.len() as u8, b.len() as u8]);
			out.extend(a.first().into_iter().chain(b.last()).flatten());
			let (a, b) = q.as_mut_slices();
			if let Some(row) = a.last_mut().or(b.first_mut()) {
				row[0] ^= 0x33;
			}
			out.extend(q.make_contiguous().iter().flatten());
			if let Some(row) = q.make_contiguous().first_mut() {
				row[1] ^= 0x44;
			}
			out.extend(q.iter().flatten());
			let mut words: std::collections::VecDeque<[u16; 2]> =
				rows.iter().map(|r| [r[0].into(), r[1].into()]).collect();
			words.rotate_right(words.len() / 3);
			let (a, b) = words.as_slices();
			out.extend(a.iter().chain(b).flatten().map(|&w| w as u8));
			out.extend(words.make_contiguous().iter().flatten().map(|&w| w as u8));
		}
		// A box's, through a raw pointer and back, and written through it.
		1 => {
			let boxed: Box<[[u8; 2]]> = rows.clone().into_boxed_slice();
			let raw = Box::into_raw(boxed);
			// SAFETY: `raw` is the box's, which nothing else holds.
			let shared = unsafe { &*raw };
			out.push(shared.len() as u8);
			// SAFETY: `raw` is the box's, which nothing else holds.
			if let Some(row) = unsafe { (*raw).first_mut() } {
				row[0] ^= 0x55;
			}
			// SAFETY: `raw` came from `Box::into_raw`.
			let back = unsafe { Box::from_raw(raw) };
			out.extend(back.iter().flatten());
			let mut owned = rows.clone();
			let p: *mut [[u8; 2]] = &mut owned[..];
			// SAFETY: `p` points at `owned`, which nothing else borrows.
			if let Some(row) = unsafe { (*p).last_mut() } {
				row[1] ^= 0x66;
			}
			out.extend(owned.iter().flatten());
		}
		// A heap's and a shared box's.
		2 => {
			let heap: std::collections::BinaryHeap<[u8; 2]> = rows.iter().copied().collect();
			out.extend(heap.as_slice().iter().flatten());
			let shared: std::rc::Rc<[[u8; 2]]> = rows.clone().into();
			out.extend(shared.iter().rev().flatten());
		}
		// A borrowed `Cow`'s, matched.
		3 => {
			let row = |at: usize| rows.get(at).copied().unwrap_or([at as u8; 2]);
			let grid = [row(0), row(1), row(2)];
			let cow: std::borrow::Cow<'_, [[u8; 2]]> = std::borrow::Cow::Borrowed(&grid[1..]);
			match cow {
				std::borrow::Cow::Borrowed(rows) => out.extend([rows.len() as u8, rows[0][0], rows[rows.len() - 1][1]]),
				std::borrow::Cow::Owned(rows) => out.push(rows.len() as u8),
			}
		}
		// A leaked `Vec`'s, written through.
		_ => {
			let leaked: &mut [[u8; 2]] = rows.clone().leak();
			if let Some(row) = leaked.first_mut() {
				row[0] = row[0].wrapping_add(1);
			}
			out.extend(leaked.iter().flatten());
		}
	}
}

/// Bumps the number it is given, which takes the item's place whole.
fn bump(x: &mut u32) {
	*x = x.wrapping_add(7);
}

/// Each item a `for` loop writes through `iter_mut`, of numbers, bytes, and `Option`s, in a
/// slice's part, a `Vec`, and a struct's field, left early, read again after the loop, and
/// passed on whole, which takes its place.
pub fn item_writes(kind: u8, data: &[u8], out: &mut Vec<u8>) {
	struct Held {
		words: Vec<u16>,
	}
	let mut wide: Vec<u64> = data.iter().map(|&b| u64::from(b) << 50 | u64::from(b)).collect();
	let mut bytes = data.to_vec();
	let mut words: Vec<u32> = data.iter().map(|&b| u32::from(b) * 0x0101_0101).collect();
	let mut options: Vec<Option<u16>> = data.iter().map(|&b| (b % 3 != 0).then_some(u16::from(b))).collect();
	let cut = usize::from(data.first().copied().unwrap_or(0)) % (data.len() + 1);
	match kind % 8 {
		// A part of a slice, as the codec bench's prefix writes are.
		0 => {
			let part = &mut wide[..cut];
			for x in part.iter_mut() {
				*x = x.wrapping_add(1);
			}
			for x in &mut wide[cut..] {
				*x ^= 0xff;
			}
		}
		// Bytes, by `&mut Vec` and from a struct's field, with a read before the write, and
		// a local array's items.
		1 => {
			let mut array = [cut as u32; 4];
			for (x, &b) in array.iter_mut().zip(data) {
				*x += u32::from(b);
			}
			for x in &mut array {
				*x = x.rotate_left(5);
			}
			out.extend(array.iter().flat_map(|x| x.to_le_bytes()));
			for b in &mut bytes {
				let before = *b;
				*b = before.rotate_left(3) ^ before;
			}
			let mut held = Held {
				words: words.iter().map(|&w| w as u16).collect(),
			};
			for w in held.words.iter_mut() {
				*w = w.wrapping_mul(3);
			}
			out.extend(held.words.iter().flat_map(|w| w.to_le_bytes()));
		}
		// Left at an item, by `break` and by `return`, which leaves the rest as it was.
		2 => {
			for w in words.iter_mut() {
				if *w > 0x8000_0000 {
					break;
				}
				*w += 1;
			}
			for b in bytes.iter_mut().rev() {
				*b = b.wrapping_sub(1);
			}
			for b in bytes.iter_mut() {
				if *b == 0 {
					out.extend(&bytes);
					return;
				}
				*b -= 1;
			}
		}
		// `Option`s, tested and replaced, and taken, which borrows the item whole.
		3 => {
			for o in options.iter_mut() {
				if let Some(v) = *o {
					*o = v.checked_mul(300);
				}
			}
			for o in options.iter_mut() {
				if o.take().is_none() {
					*o = Some(1);
				}
			}
		}
		// Passed on whole, and swapped, which take each item's place.
		4 => {
			for w in words.iter_mut() {
				bump(w);
			}
			let mut carry = 0u32;
			for w in words.iter_mut() {
				std::mem::swap(w, &mut carry);
			}
			out.extend(carry.to_le_bytes());
		}
		// Stepped by hand, then the rest by a loop.
		5 => {
			let mut it = words.iter_mut();
			if let Some(first) = it.next() {
				*first = 0;
			}
			for w in it {
				*w = w.wrapping_mul(5);
			}
		}
		// Nested, each row's items, and places kept past the loops that lent them: of
		// items, and of `Some`'s payloads.
		6 => {
			let mut rows: Vec<Vec<u8>> = data.chunks(3).map(<[u8]>::to_vec).collect();
			for row in rows.iter_mut() {
				for b in row.iter_mut() {
					*b = b.wrapping_add(row_len(cut));
				}
			}
			let mut kept: Vec<&mut u8> = Vec::new();
			for row in rows.iter_mut() {
				for b in row.iter_mut() {
					if *b % 2 == 0 {
						kept.push(b);
					}
				}
			}
			for b in kept {
				*b ^= 0x80;
			}
			out.extend(rows.into_iter().flatten());
			let mut grid: Vec<Vec<Option<u16>>> = options.chunks(2).map(<[Option<u16>]>::to_vec).collect();
			let mut payloads: Vec<&mut u16> = Vec::new();
			for row in grid.iter_mut() {
				for o in row.iter_mut() {
					if let Some(v) = o {
						payloads.push(v);
					}
				}
			}
			for v in payloads {
				*v = v.wrapping_add(1000);
			}
			out.extend(
				grid.into_iter()
					.flatten()
					.flat_map(|o| o.map_or([0xee, 0xee], u16::to_le_bytes)),
			);
		}
		// Overflow panics where Rust's does, part way through.
		_ => {
			for b in bytes.iter_mut() {
				*b += 100;
			}
		}
	}
	out.extend(wide.iter().flat_map(|x| x.to_le_bytes()));
	out.extend(&bytes);
	out.extend(words.iter().flat_map(|w| w.to_le_bytes()));
	out.extend(options.iter().flat_map(|o| o.map_or([0xff, 0xff], u16::to_le_bytes)));
}

/// A step that depends on the input.
fn row_len(cut: usize) -> u8 {
	cut as u8 | 1
}

/// Puts each word's low byte.
fn put_words(words: &[u32], out: &mut Vec<u8>) {
	out.extend(words.iter().map(|&w| w as u8));
}

/// A `&mut [T; N]` to part of a slice, of numbers, of strings, and of arrays, written whole:
/// by `mem::replace` and `mem::swap`, and through a `Cell` and an `UnsafeCell` of it, each
/// way a cell reads, writes, and points at it; a cell of arrays pointed at before a write,
/// and of pinned boxes, whose replacement keeps their allocations; and one std holds in an `Option` and a `Vec` and writes through.
pub fn array_parts(kind: u8, data: &[u8], out: &mut Vec<u8>) {
	use std::cell::{Cell, UnsafeCell};
	let mut words: Vec<u32> = data.iter().map(|&b| u32::from(b)).collect();
	words.resize(words.len().max(6), 9);
	let mut names: Vec<String> = data.iter().map(|b| b.to_string()).collect();
	names.resize(names.len().max(6), String::from("z"));
	let mut rows: Vec<[u32; 2]> = words.chunks(2).map(|c| [c[0], c[c.len() - 1]]).collect();
	let n = u32::from(kind);
	match kind % 4 {
		0 => {
			let (head, rest) = words.split_at_mut(1);
			let a: &mut [u32; 2] = rest.first_chunk_mut().unwrap();
			let old = std::mem::replace(a, [n, n + 1]);
			put_words(&old, out);
			let (x, y) = rest[2..].split_first_chunk_mut::<2>().unwrap();
			let y: &mut [u32; 2] = y.first_chunk_mut().unwrap();
			std::mem::swap(x, y);
			head[0] = n;
			let s: &mut [String; 2] = names[1..].first_chunk_mut().unwrap();
			let old = std::mem::replace(s, [String::from("a"), String::from("b")]);
			out.extend(old.iter().map(String::len).map(|l| l as u8));
		}
		1 => {
			let a: &mut [u32; 2] = words[1..].first_chunk_mut().unwrap();
			let cell = Cell::from_mut(a);
			let old = cell.replace([n, n + 2]);
			put_words(&old, out);
			put_words(&cell.get(), out);
			cell.set([n + 3, n + 4]);
			let p = cell.as_ptr();
			unsafe { (*p)[1] = n + 5 };
			cell.update(|[x, y]| [y, x]);
			put_words(&cell.take(), out);
			let s: &mut [String; 2] = names[2..].first_chunk_mut().unwrap();
			let old = Cell::from_mut(s).replace([String::from("abc"), String::new()]);
			out.extend(old.iter().map(String::len).map(|l| l as u8));
			let a: &mut [u32; 2] = words[2..].first_chunk_mut().unwrap();
			let cell = UnsafeCell::from_mut(a);
			unsafe { (*cell.get())[0] = n + 6 };
			cell.get_mut()[1] = n + 7;
		}
		2 => {
			let r: &mut [[u32; 2]; 2] = rows[1..].first_chunk_mut().unwrap();
			let cell = Cell::from_mut(r);
			let p = cell.as_ptr();
			unsafe { (*p)[0][1] = n + 6 };
			let old = cell.replace([[n, n + 7], [n + 8, n]]);
			out.extend(old.iter().flatten().map(|&w| w as u8));
			let r: &mut [[u32; 2]; 2] = rows[..2].first_chunk_mut().unwrap();
			let cell = UnsafeCell::from_mut(r);
			unsafe { (*cell.get())[1][0] = n + 9 };
			cell.get_mut()[0][0] = n + 10;
			let held = Cell::new([[n, 1], [2, n]]);
			let p = held.as_ptr();
			held.set([[3, n], [n, 4]]);
			out.extend(unsafe { *p }.iter().flatten().map(|&w| w as u8));
			let pinned = Box::pin((n as u8, 1));
			let at: *const (u8, u8) = &*pinned;
			let boxes = Cell::new([[pinned]]);
			let [[old]] = boxes.replace([[Box::pin((2, n as u8))]]);
			out.extend([u8::from(std::ptr::eq(at, &*old)), old.0]);
		}
		_ => {
			let (a, b) = words[1..].split_first_chunk_mut::<2>().unwrap();
			b[0] = n;
			let mut kept = Some(a);
			if let Some(a) = kept.as_mut() {
				a[0] = n + 1;
			}
			kept.map(|a| a[1] = n + 2);
			let mut v: Vec<&mut [u32; 2]> = Vec::new();
			let (d, rest) = words.split_at_mut(2);
			v.push(rest.first_chunk_mut().unwrap());
			v.push(d.first_chunk_mut().unwrap());
			for a in v {
				a.swap(0, 1);
			}
		}
	}
	put_words(&words, out);
	out.extend(names.iter().map(String::len).map(|l| l as u8));
	out.extend(rows.iter().flatten().map(|&w| w as u8));
}
