//! `char`: a Unicode scalar value, switched on, ordered, converted, and printed.

use std::collections::{BTreeSet, HashSet};

use crate::harness::put;

/// Values next to where a scalar's UTF-8 or UTF-16 length changes, the surrogates that
/// are no char, and past the last char.
const EDGES: [u32; 20] = [
	0,
	0x7f,
	0x80,
	0xff,
	0x100,
	0x7ff,
	0x800,
	0xd7ff,
	0xd800,
	0xdbff,
	0xdc00,
	0xdfff,
	0xe000,
	0xfffd,
	0xffff,
	0x10000,
	0x10fffd,
	0x10ffff,
	0x110000,
	u32::MAX,
];

/// Chars `Debug` escapes, or writes as they are. The halfwidth katakana marks extend
/// graphemes but print; the Hangul fillers are default-ignorable.
const SHOWN: [char; 18] = [
	'a', '\'', '"', '\\', '\n', '\0', '\u{7f}', 'é', '\u{301}', '\u{ad}', '\u{e000}', '😀', '\u{ff9e}', '\u{ff9f}',
	'\u{115f}', '\u{1160}', '\u{3164}', '\u{ffa0}',
];

/// The scalars to try: the edges, then each big-endian `u32` in `data`, zero-padded.
fn scalars(data: &[u8]) -> Vec<u32> {
	let mut all = EDGES.to_vec();
	let mut rest = data;
	while !rest.is_empty() {
		let mut buf = [0u8; 4];
		let n = rest.len().min(4);
		buf[..n].copy_from_slice(&rest[..n]);
		all.push(u32::from_be_bytes(buf));
		rest = &rest[n..];
	}
	all
}

/// A char's class, by a switch on its value and ranges of it.
fn class(c: char) -> u8 {
	match c {
		'\0' => 0,
		'a'..='z' => 1,
		'A'..='Z' | '_' => 2,
		'0'..='9' => 3,
		'/' | '\\' => 4,
		'\u{80}'..='\u{7ff}' => 5,
		'\u{e000}'..='\u{f8ff}' => 6,
		'\u{10000}'.. => 7,
		_ => 8,
	}
}

pub fn chars(kind: u8, data: &[u8], out: &mut Vec<u8>) {
	match kind % 4 {
		0 => {
			for n in scalars(data) {
				match char::from_u32(n) {
					Some(c) => {
						out.push(class(c));
						out.extend_from_slice(&u32::from(c).to_be_bytes());
					}
					None => out.push(0xff),
				}
			}
		}
		1 => {
			// Casts truncate, as `as` does, and widen.
			for n in scalars(data) {
				let c = char::from_u32(n).unwrap_or(char::from(n as u8));
				out.push(c as u8);
				out.extend_from_slice(&(c as u16).to_be_bytes());
				out.push(c as i8 as u8);
				out.extend_from_slice(&(c as i16).to_be_bytes());
				out.extend_from_slice(&(c as i32).to_be_bytes());
				out.extend_from_slice(&(c as u64).to_be_bytes());
				out.extend_from_slice(&(c as i128).to_be_bytes());
				out.push((n as u8 as char) as u8);
				// A cast of a cast, whose `as` must not read on into the mask.
				out.extend_from_slice(&(c as u32 as u8 as u16).to_be_bytes());
			}
		}
		2 => {
			// Ordered and equal as their values, and as keys.
			let cs: Vec<char> = scalars(data).into_iter().filter_map(char::from_u32).collect();
			let mut sorted = BTreeSet::new();
			let mut seen = HashSet::new();
			for (i, &c) in cs.iter().enumerate() {
				let d = cs[(i * 7 + 3) % cs.len()];
				out.push((c < d) as u8 | ((c == d) as u8) << 1 | ((c >= 'z') as u8) << 2);
				out.push(seen.insert(c) as u8);
				sorted.insert(c);
			}
			for c in sorted {
				out.extend_from_slice(&(c as u32).to_be_bytes());
			}
		}
		_ => {
			// `Display` writes the char, padded only with a width; `Debug` escapes it and
			// ignores the width.
			for n in scalars(data) {
				if let Some(c) = char::from_u32(n) {
					put(out, &format!("{c}|{c:>3}|{c:*^4}|{c:<2}"));
				}
			}
			for c in SHOWN {
				put(out, &format!("{c:?}|{c:>6?}|{c}"));
			}
		}
	}
}
