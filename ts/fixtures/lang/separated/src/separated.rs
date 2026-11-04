//! A slice split by a predicate, by `split` and each of its forms, of bytes and of other
//! values, with no, some, and only separators: each call of the predicate logged as the
//! parts are read from the front, the back, and both in turn, with the size hints and
//! `as_slice` std gives as it goes; `splitn`'s counts; what is written through a mutable
//! part landing in the slice; and a predicate that owns what drops, dropped with its
//! iterator.

use std::cell::RefCell;

/// Puts what was logged since the last flush, after a marker.
fn flush(log: &RefCell<Vec<u8>>, out: &mut Vec<u8>) {
	out.push(0xfd);
	out.extend(log.borrow_mut().drain(..));
}

fn hint((lower, upper): (usize, Option<usize>), out: &mut Vec<u8>) {
	out.push(lower as u8);
	out.push(upper.map_or(0xfe, |u| u as u8));
}

/// A sum of a part, which tells parts of different lengths or contents apart.
fn sum(part: &[u8]) -> u8 {
	part.iter()
		.fold(part.len() as u8, |acc, &b| acc.wrapping_mul(31).wrapping_add(b))
}

fn sum_words(part: &[u16]) -> u8 {
	part.iter()
		.fold(part.len() as u8, |acc, &w| acc.wrapping_mul(31) ^ (w >> 3) as u8)
}

/// An element kept as an object, which a mutable part writes whole and by field.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Pair {
	a: u8,
	b: u16,
}

fn sum_pairs(part: &[Pair]) -> u8 {
	part.iter()
		.fold(part.len() as u8, |acc, p| acc.wrapping_mul(31) ^ p.a ^ (p.b >> 4) as u8)
}

fn sum_texts(part: &[String]) -> u8 {
	part.iter().fold(part.len() as u8, |acc, s| {
		acc.wrapping_mul(31).wrapping_add(s.len() as u8)
	})
}

fn sum_twos(part: &[[u8; 2]]) -> u8 {
	part.iter().fold(part.len() as u8, |acc, t| {
		acc.wrapping_mul(31) ^ t[0].wrapping_add(t[1])
	})
}

/// A separator test that logs its tag as it drops.
struct Below<'a> {
	tag: u8,
	bound: u16,
	log: &'a RefCell<Vec<u8>>,
}

impl Below<'_> {
	fn test(&self, b: u8) -> bool {
		self.log.borrow_mut().push(b);
		u16::from(b) < self.bound
	}
}

impl Drop for Below<'_> {
	fn drop(&mut self) {
		self.log.borrow_mut().push(self.tag);
	}
}

/// Each way of reading a double-ended iterator of parts `$make` makes anew, with the
/// predicate's calls flushed after each read: front to back, back to front, from both ends
/// as the bits of `$steps` pick, and by `nth`, `nth_back`, `last`, `count`, and `rev`.
macro_rules! ends {
	($make:expr, $sum:expr, $steps:expr, $log:expr, $out:expr) => {{
		let (log, out): (&RefCell<Vec<u8>>, &mut Vec<u8>) = ($log, $out);
		for back in [false, true] {
			let mut it = $make;
			hint(it.size_hint(), out);
			flush(log, out);
			while let Some(p) = if back { it.next_back() } else { it.next() } {
				out.push($sum(p));
				hint(it.size_hint(), out);
				flush(log, out);
			}
			// Fused, calling nothing more.
			out.push(it.next().is_none() as u8 | (it.next_back().is_none() as u8) << 1);
			flush(log, out);
		}
		let mut it = $make;
		for i in 0..12 {
			let p = match $steps >> (i % 8) & 1 {
				0 => it.next(),
				_ => it.next_back(),
			};
			out.push(p.map_or(0xfc, $sum));
			hint(it.size_hint(), out);
			flush(log, out);
		}
		out.extend($make.nth(1).map($sum));
		flush(log, out);
		out.extend($make.nth_back(1).map($sum));
		flush(log, out);
		out.extend($make.last().map($sum));
		flush(log, out);
		out.push($make.count() as u8);
		flush(log, out);
		out.extend($make.rev().skip(1).map($sum));
		flush(log, out);
	}};
}

/// `splitn` or `rsplitn` of `$data` by `$pred`, as `$method` names it, at each count up to
/// two past the most parts there can be, read with the predicate's calls flushed after each
/// part, then counted.
macro_rules! counted {
	($data:expr, $method:ident, $pred:expr, $sum:expr, $log:expr, $out:expr) => {{
		let (log, out): (&RefCell<Vec<u8>>, &mut Vec<u8>) = ($log, $out);
		for n in 0..$data.len() + 3 {
			let mut it = $data.$method(n, $pred);
			hint(it.size_hint(), out);
			while let Some(p) = it.next() {
				out.push($sum(p));
				hint(it.size_hint(), out);
				flush(log, out);
			}
			out.push(it.next().is_none() as u8);
			out.push($data.$method(n, $pred).count() as u8);
			flush(log, out);
		}
	}};
}

/// Writes through each mutable part `$data.$method(..)` makes, as `$scribble` writes the
/// `i`th: as each comes, then each of a pass that holds every part at once, back to front,
/// then `$data` whole.
macro_rules! write {
	($data:ident . $method:ident ( $($arg:expr),* ), $scribble:expr, $sum:expr, $log:expr, $out:expr) => {{
		let (log, out): (&RefCell<Vec<u8>>, &mut Vec<u8>) = ($log, $out);
		for (i, part) in $data.$method($($arg),*).enumerate() {
			$scribble(i, part);
		}
		flush(log, out);
		out.push($sum(&$data));
		let mut held: Vec<_> = $data.$method($($arg),*).collect();
		for (i, part) in held.iter_mut().enumerate().rev() {
			$scribble(i + 7, part);
		}
		out.extend(held.iter().map(|p| $sum(p)));
		flush(log, out);
		out.push($sum(&$data));
	}};
}

/// Writes through the mutable parts of a double-ended `$data.$method(..)`, taken from both
/// ends as the bits of `$steps` pick, then `$data` whole.
macro_rules! write_ends {
	($data:ident . $method:ident ( $($arg:expr),* ), $scribble:expr, $sum:expr, $steps:expr, $log:expr, $out:expr) => {{
		let (log, out): (&RefCell<Vec<u8>>, &mut Vec<u8>) = ($log, $out);
		let mut it = $data.$method($($arg),*);
		for i in 0..12 {
			let p = match $steps >> (i % 8) & 1 {
				0 => it.next(),
				_ => it.next_back(),
			};
			match p {
				Some(p) => $scribble(i, p),
				None => out.push(0xfc),
			}
			flush(log, out);
		}
		out.push($sum(&$data));
	}};
}

fn scribble(i: usize, part: &mut [u8]) {
	if let Some(first) = part.first_mut() {
		*first = first.wrapping_add(i as u8 + 1);
	}
	part.reverse();
	// A part of a part writes through to the slice too.
	for (j, inner) in part.split_mut(|&b| b % 3 == 0).enumerate() {
		if let Some(last) = inner.last_mut() {
			*last ^= j as u8 + 1;
		}
	}
}

fn scribble_words(i: usize, part: &mut [u16]) {
	if let Some(first) = part.first_mut() {
		*first = first.wrapping_add(i as u16 * 3 + 1);
	}
	part.reverse();
	for (j, inner) in part.split_inclusive_mut(|&w| w % 3 == 0).enumerate() {
		inner.fill(j as u16 | 0x100);
	}
}

fn scribble_pairs(i: usize, part: &mut [Pair]) {
	if let Some(first) = part.first_mut() {
		first.a = first.a.wrapping_add(i as u8 + 1);
	}
	if let Some(last) = part.last_mut() {
		*last = Pair {
			a: last.b as u8,
			b: i as u16,
		};
	}
	part.sort();
	for inner in part.rsplit_mut(|p| p.a % 2 == 0) {
		if let Some(p) = inner.first_mut() {
			p.b ^= 0x55;
		}
	}
}

/// A form of `split`, as `kind` picks, of `data` and of other values made of it, by a
/// predicate that logs each call, with each bound that makes no, some, or every element a
/// separator.
pub fn separated(kind: u8, data: &[u8], out: &mut Vec<u8>) {
	let log = RefCell::new(Vec::new());
	let log = &log;
	let steps = kind >> 3;
	for bound in [0u16, 1, 0x80, 0x100] {
		let mut bytes = data.to_vec();
		let mut words: Vec<u16> = data.iter().map(|&b| u16::from(b) * 257).collect();
		let mut pairs: Vec<Pair> = data
			.iter()
			.map(|&b| Pair {
				a: b,
				b: u16::from(b) * 3,
			})
			.collect();
		let texts: Vec<String> = data
			.iter()
			.map(|&b| ["", "a", "ab"][usize::from(b) % 3].to_string())
			.collect();
		let test = |&b: &u8| {
			log.borrow_mut().push(b);
			u16::from(b) < bound
		};
		let test_words = |&w: &u16| {
			log.borrow_mut().push(w as u8 ^ 0x5a);
			w >> 8 < bound
		};
		let test_pairs = |p: &Pair| {
			log.borrow_mut().push(p.a ^ 0x33);
			u16::from(p.a) < bound
		};
		let test_texts = |s: &String| {
			log.borrow_mut().push(s.len() as u8 | 0x80);
			s.len() < usize::from(bound) % 3
		};
		match kind % 8 {
			0 => {
				ends!(data.split(test), sum, steps, log, out);
				ends!(words.split(test_words), sum_words, steps, log, out);
				ends!(texts.split(test_texts), sum_texts, steps, log, out);
				// What is left to split, as parts are taken and once none is left, and a
				// clone, which reads on from where it was cloned.
				let mut it = data.split(test);
				for i in 0..4 {
					out.push(sum(it.as_slice()));
					match i % 2 {
						0 => out.extend(it.next().map(sum)),
						_ => out.extend(it.next_back().map(sum)),
					}
					flush(log, out);
				}
				let mut copy = it.clone();
				out.extend(copy.next_back().map(sum));
				out.extend(it.next().map(sum));
				out.push(sum(copy.as_slice()));
				out.push(sum(it.as_slice()));
				flush(log, out);
				let mut it = pairs.split(test_pairs);
				it.next_back();
				out.push(sum_pairs(it.as_slice()));
				flush(log, out);
			}
			1 => {
				ends!(data.rsplit(test), sum, steps, log, out);
				ends!(pairs.rsplit(test_pairs), sum_pairs, steps, log, out);
				ends!(texts.rsplit(test_texts), sum_texts, steps, log, out);
				let twos: Vec<[u8; 2]> = data.chunks_exact(2).map(|c| [c[0], c[1]]).collect();
				let test_twos = |t: &[u8; 2]| {
					log.borrow_mut().push(t[1]);
					u16::from(t[0]) < bound
				};
				ends!(twos.rsplit(test_twos), sum_twos, steps, log, out);
				ends!(twos.split(test_twos), sum_twos, steps, log, out);
			}
			2 => {
				ends!(data.split_inclusive(test), sum, steps, log, out);
				ends!(words.split_inclusive(test_words), sum_words, steps, log, out);
				ends!(pairs.split_inclusive(test_pairs), sum_pairs, steps, log, out);
			}
			3 => {
				counted!(data, splitn, test, sum, log, out);
				counted!(data, rsplitn, test, sum, log, out);
				counted!(words, splitn, test_words, sum_words, log, out);
				counted!(pairs, rsplitn, test_pairs, sum_pairs, log, out);
				counted!(texts, splitn, test_texts, sum_texts, log, out);
			}
			4 => {
				write!(bytes.split_mut(test), scribble, sum, log, out);
				write!(words.rsplit_mut(test_words), scribble_words, sum_words, log, out);
				write!(pairs.split_mut(test_pairs), scribble_pairs, sum_pairs, log, out);
				write_ends!(bytes.rsplit_mut(test), scribble, sum, steps, log, out);
				write_ends!(words.split_mut(test_words), scribble_words, sum_words, steps, log, out);
				write_ends!(pairs.rsplit_mut(test_pairs), scribble_pairs, sum_pairs, steps, log, out);
			}
			5 => {
				write!(bytes.split_inclusive_mut(test), scribble, sum, log, out);
				write!(
					words.split_inclusive_mut(test_words),
					scribble_words,
					sum_words,
					log,
					out
				);
				write!(
					pairs.split_inclusive_mut(test_pairs),
					scribble_pairs,
					sum_pairs,
					log,
					out
				);
				write_ends!(bytes.split_inclusive_mut(test), scribble, sum, steps, log, out);
				write_ends!(
					words.split_inclusive_mut(test_words),
					scribble_words,
					sum_words,
					steps,
					log,
					out
				);
				write_ends!(
					pairs.split_inclusive_mut(test_pairs),
					scribble_pairs,
					sum_pairs,
					steps,
					log,
					out
				);
			}
			6 => {
				let n = usize::from(steps) % (data.len() + 3);
				write!(bytes.splitn_mut(n, test), scribble, sum, log, out);
				write!(words.rsplitn_mut(n, test_words), scribble_words, sum_words, log, out);
				write!(pairs.splitn_mut(n, test_pairs), scribble_pairs, sum_pairs, log, out);
				write!(bytes.rsplitn_mut(n, test), scribble, sum, log, out);
			}
			// A predicate that owns what drops, which drops with its iterator: part way, as a
			// loop ends, and as a consumer that takes it ends.
			_ => {
				let below = |tag: u8| Below { tag, bound, log };
				let sep = below(0xe0);
				let mut it = data.split(move |&b| sep.test(b));
				out.extend(it.next().map(sum));
				flush(log, out);
				drop(it);
				flush(log, out);
				let sep = below(0xe1);
				for part in data.rsplit(move |&b| sep.test(b)) {
					out.push(sum(part));
					flush(log, out);
				}
				flush(log, out);
				let sep = below(0xe2);
				let all: Vec<&[u8]> = data.split_inclusive(move |&b| sep.test(b)).collect();
				flush(log, out);
				out.extend(all.iter().map(|p| sum(p)));
				let sep = below(0xe3);
				out.push(data.splitn(2, move |&b| sep.test(b)).count() as u8);
				flush(log, out);
				let sep = below(0xe4);
				for part in bytes.rsplitn_mut(3, move |&b| sep.test(b)) {
					scribble(1, part);
				}
				flush(log, out);
				out.push(sum(&bytes));
			}
		}
		flush(log, out);
	}
}
