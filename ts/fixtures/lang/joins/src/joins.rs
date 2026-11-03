//! `concat` and `join` of slices of arrays, slices, vectors, and boxed slices, of bytes
//! and of other values, and of text: each item's elements cloned once, in order, with the
//! separator's between them, outer and inner slices empty or not, and totals past what the
//! result may hold, which panic as std's do.

use std::borrow::Cow;
use std::cell::RefCell;
use std::rc::Rc;

use crate::harness::{put, text};

/// Logs its id as it drops, and as it is cloned, the id with the high bit set: its clone's
/// id is one more.
struct Logged {
	id: u8,
	log: Rc<RefCell<Vec<u8>>>,
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

/// Panics as it is cloned, which shows whether std clones one.
struct Sentinel(u8);

impl Clone for Sentinel {
	fn clone(&self) -> Self {
		panic!("clone sentinel {}", self.0)
	}
}

/// Puts what was logged since the last flush, after a marker.
fn flush(log: &RefCell<Vec<u8>>, out: &mut Vec<u8>) {
	out.push(0xfd);
	out.extend(log.borrow_mut().drain(..));
}

/// Puts `v`'s length, then its bytes.
fn bytes(v: Vec<u8>, out: &mut Vec<u8>) {
	out.push(v.len() as u8);
	out.extend_from_slice(&v);
}

/// Puts `v`'s length, then each `u16`'s bytes.
fn wides(v: Vec<u16>, out: &mut Vec<u8>) {
	out.push(v.len() as u8);
	for x in v {
		out.extend(x.to_le_bytes());
	}
}

/// Puts the ids of `v`, then drops it.
fn ids(v: Vec<Logged>, log: &RefCell<Vec<u8>>, out: &mut Vec<u8>) {
	out.push(v.len() as u8);
	out.extend(v.iter().map(|l| l.id));
	drop(v);
	flush(log, out);
}

/// `concat` and `join` of what `kind` picks, each over the parts of `data` between its
/// zeros, as many as `kind`'s high bits pick, none and empty ones included.
pub fn joins(kind: u8, data: &[u8], out: &mut Vec<u8>) {
	let all: Vec<&[u8]> = data.split(|&b| b == 0).collect();
	let parts = &all[..usize::from(kind >> 3) % (all.len() + 1)];
	let sep = data.first().copied().unwrap_or(7);
	let seps = &data[..data.len().min(3)];
	match kind % 7 {
		// Byte slices and vectors of them.
		0 => {
			bytes(parts.concat(), out);
			bytes(parts.join(&sep), out);
			bytes(parts.join(seps), out);
			bytes(parts.join(&[][..]), out);
			let owned: Vec<Vec<u8>> = parts.iter().map(|p| p.to_vec()).collect();
			bytes(owned.concat(), out);
			bytes(owned.join(&sep), out);
			bytes(owned.join(seps), out);
			let boxed: Vec<Box<[u8]>> = parts.iter().map(|&p| Box::from(p)).collect();
			bytes(boxed.concat(), out);
			bytes(boxed.join(&sep), out);
			bytes(boxed.join(seps), out);
			let none: [&[u8]; 0] = [];
			bytes(none.concat(), out);
			bytes(none.join(&sep), out);
			bytes(none.join(seps), out);
		}
		// Byte arrays, of no bytes too.
		1 => {
			let at = |i: usize| data.get(i).copied().unwrap_or(i as u8);
			let hash_a: [u8; 8] = std::array::from_fn(at);
			let hash_b: [u8; 8] = std::array::from_fn(|i| at(i + 8));
			bytes([hash_a, hash_b].concat(), out);
			bytes([hash_a, hash_b].join(&sep), out);
			bytes([hash_a, hash_b].join(seps), out);
			let quads: Vec<[u8; 4]> = parts
				.iter()
				.map(|p| std::array::from_fn(|i| p.get(i).copied().unwrap_or(0)))
				.collect();
			bytes(quads.concat(), out);
			bytes(quads.join(&sep), out);
			bytes(quads.join(seps), out);
			let zeros: Vec<[u8; 0]> = parts.iter().map(|_| []).collect();
			bytes(zeros.concat(), out);
			bytes(zeros.join(&sep), out);
			bytes(zeros.join(seps), out);
			let none: [[u8; 8]; 0] = [];
			bytes(none.concat(), out);
			bytes(none.join(&sep), out);
		}
		// `u16`s, in vectors, arrays, and slices.
		2 => {
			let sep = u16::from(sep) << 8 | 1;
			let wide: Vec<Vec<u16>> = parts
				.iter()
				.map(|p| p.iter().map(|&b| u16::from(b) * 257).collect())
				.collect();
			let wide_seps: Vec<u16> = seps.iter().map(|&b| u16::from(b) | 0x100).collect();
			wides(wide.concat(), out);
			wides(wide.join(&sep), out);
			wides(wide.join(&wide_seps[..]), out);
			let slices: Vec<&[u16]> = wide.iter().map(|v| &v[..]).collect();
			wides(slices.concat(), out);
			wides(slices.join(&sep), out);
			wides(slices.join(&wide_seps[..]), out);
			let boxed: Vec<Box<[u16]>> = wide.iter().cloned().map(Vec::into_boxed_slice).collect();
			wides(boxed.concat(), out);
			wides(boxed.join(&sep), out);
			wides(boxed.join(&wide_seps[..]), out);
			let pairs: Vec<[u16; 2]> = slices
				.iter()
				.map(|s| [s.len() as u16, s.first().copied().unwrap_or(0)])
				.collect();
			wides(pairs.concat(), out);
			wides(pairs.join(&sep), out);
			wides(pairs.join(&wide_seps[..]), out);
			let zeros: Vec<[u16; 0]> = parts.iter().map(|_| []).collect();
			wides(zeros.concat(), out);
			wides(zeros.join(&sep), out);
			let none: Vec<Vec<u16>> = Vec::new();
			wides(none.concat(), out);
			wides(none.join(&sep), out);
			wides(none.join(&wide_seps[..]), out);
		}
		// Items whose clones log, each cloned once, in order, the separator's too.
		3 => {
			let log = Rc::new(RefCell::new(Vec::new()));
			let logged = |id: u8| Logged { id, log: log.clone() };
			let vecs: Vec<Vec<Logged>> = parts
				.iter()
				.map(|p| p.iter().map(|&b| logged(b & 0x7e)).collect())
				.collect();
			let sep = logged(0x50);
			let seps: Vec<Logged> = seps.iter().map(|&b| logged(b & 0x3e)).collect();
			ids(vecs.concat(), &log, out);
			ids(vecs.join(&sep), &log, out);
			ids(vecs.join(&seps[..]), &log, out);
			let slices: Vec<&[Logged]> = vecs.iter().map(|v| &v[..]).collect();
			ids(slices.concat(), &log, out);
			ids(slices.join(&sep), &log, out);
			ids(slices.join(&seps[..]), &log, out);
			let boxed: Vec<Box<[Logged]>> = slices.iter().map(|&s| Box::from(s)).collect();
			flush(&log, out);
			ids(boxed.concat(), &log, out);
			ids(boxed.join(&sep), &log, out);
			drop(boxed);
			flush(&log, out);
			let pairs = [[logged(2), logged(4)], [logged(6), logged(8)]];
			ids(pairs.concat(), &log, out);
			ids(pairs.join(&sep), &log, out);
			ids(pairs.join(&seps[..]), &log, out);
			drop(slices);
			drop((vecs, pairs, sep, seps));
			flush(&log, out);
		}
		// Text, owned and borrowed.
		4 => {
			let owned: Vec<String> = parts.iter().map(|p| text(p)).collect();
			let sep = text(seps);
			put(out, &owned.concat());
			put(out, &owned.join(&sep));
			let borrowed: Vec<&str> = owned.iter().map(String::as_str).collect();
			put(out, &borrowed.concat());
			put(out, &borrowed.join(&sep));
			let none: [String; 0] = [];
			put(out, &none.concat());
			put(out, &none.join(&sep));
		}
		// `Cow`s, borrowed and owned in turn.
		5 => {
			let cows: Vec<Cow<[u8]>> = parts
				.iter()
				.enumerate()
				.map(|(i, &p)| match i % 2 {
					0 => Cow::Borrowed(p),
					_ => Cow::Owned(p.to_vec()),
				})
				.collect();
			bytes(cows.concat(), out);
			bytes(cows.join(&sep), out);
			bytes(cows.join(seps), out);
			let wide: Vec<Vec<u16>> = parts
				.iter()
				.map(|p| p.iter().map(|&b| u16::from(b)).collect())
				.collect();
			let wide: Vec<Cow<[u16]>> = wide
				.iter()
				.enumerate()
				.map(|(i, v)| match i % 2 {
					0 => Cow::Borrowed(&v[..]),
					_ => Cow::Owned(v.clone()),
				})
				.collect();
			wides(wide.concat(), out);
			wides(wide.join(&0xffff), out);
			let texts: Vec<Cow<str>> = parts
				.iter()
				.enumerate()
				.map(|(i, p)| match i % 2 {
					0 => Cow::Borrowed("ab"),
					_ => Cow::Owned(text(p)),
				})
				.collect();
			put(out, &texts.concat());
			put(out, &texts.join("--"));
		}
		// Totals past what a `Vec` or a `String` may hold, of one part repeated, which panic
		// before anything is cloned, but where std's count wraps.
		_ => {
			let big = vec![1u8; 1 << 16];
			let wide = vec![1u16; 1 << 16];
			let text = text(&big);
			let refs = |n: usize| vec![&big[..]; n];
			match data.first().copied().unwrap_or(0) % 7 {
				// Bytes past `isize::MAX`.
				0 => bytes(refs((1 << 15) + 1).concat(), out),
				// Their count past `usize::MAX`.
				1 => bytes(refs((1 << 16) + 1).concat(), out),
				// Past `isize::MAX` only with the separators.
				2 => bytes(refs((1 << 14) + 1).join(&big[..]), out),
				// `u16`s past what a `Vec` of them may hold, half as many as bytes.
				3 => wides(vec![&wide[..]; (1 << 14) + 1].concat(), out),
				4 => put(out, &vec![text.as_str(); (1 << 15) + 1].concat()),
				5 => put(out, &vec![text.as_str(); (1 << 16) + 1].join("")),
				// A count of separators that wraps, as std's does, to what it reserves, so it
				// clones the first.
				_ => {
					let sep: Vec<Sentinel> = (0..1 << 16).map(|i| Sentinel(i as u8 ^ 0x5a)).collect();
					let none: &[Sentinel] = &[];
					drop(vec![none; (1 << 16) + 1].join(&sep[..]));
				}
			}
		}
	}
}
