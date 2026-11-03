//! `char`: a Unicode scalar value, switched on, ordered, converted, and printed; its
//! Unicode classes and case maps over every char, by std's Unicode version; its escapes
//! and case maps as iterators; and its encodings, decodings, and conversions, with their
//! errors and panics.

use std::collections::{BTreeSet, HashSet};
use std::fmt::Debug;

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

/// Chars whose case maps to more than one char or none, crosses the BMP's edge, or is
/// title case, those each escape treats apart: quotes, controls, a grapheme extender, and
/// the unprintable, and the byte order mark, which a UTF-8 decoder may strip.
const SPECIAL: [char; 25] = [
	'a',
	'Z',
	'ß',
	'İ',
	'ı',
	'ǅ',
	'ΐ',
	'ﬃ',
	'ŉ',
	'\u{1e9e}',
	'\u{10428}',
	'\u{10400}',
	'Σ',
	'\'',
	'"',
	'\\',
	'\0',
	'\t',
	'\r',
	'\n',
	'\u{7f}',
	'\u{301}',
	'\u{e000}',
	'\u{feff}',
	'😀',
];

/// Every char of one 4096-scalar block, which the input picks: each Unicode class std
/// tests and both case maps, by method and as a function value.
pub fn unicode(kind: u8, data: &[u8], out: &mut Vec<u8>) {
	let block = u32::from(u16::from_be_bytes([
		data.first().copied().unwrap_or(0),
		data.get(1).copied().unwrap_or(0),
	])) % 0x110;
	let classes: [fn(char) -> bool; 6] = [
		char::is_alphabetic,
		char::is_lowercase,
		char::is_uppercase,
		char::is_alphanumeric,
		char::is_control,
		char::is_numeric,
	];
	for c in (block << 12..(block + 1) << 12).filter_map(char::from_u32) {
		let flags = match kind % 2 {
			0 => {
				c.is_alphabetic() as u8
					| (c.is_lowercase() as u8) << 1
					| (c.is_uppercase() as u8) << 2
					| (c.is_alphanumeric() as u8) << 3
					| (c.is_control() as u8) << 4
					| (c.is_numeric() as u8) << 5
					| (c.is_digit(36) as u8) << 6
			}
			_ => classes
				.iter()
				.enumerate()
				.fold(0, |flags, (i, class)| flags | (class(c) as u8) << i),
		};
		out.push(flags);
		for mapped in c.to_lowercase().chain(c.to_uppercase()) {
			out.extend_from_slice(&u32::from(mapped).to_be_bytes()[1..]);
		}
		out.push(c.to_lowercase().len() as u8 | (c.to_uppercase().len() as u8) << 2);
	}
}

/// What each iterator does from either end, sized, cloned, and printed partway.
fn walk<I>(out: &mut Vec<u8>, mut it: I)
where
	I: Iterator<Item = char> + ExactSizeIterator + Clone + std::fmt::Display + Debug,
{
	put(out, &format!("{it}|{it:>12}|{it:?}|{it:#?}"));
	let (low, high) = it.size_hint();
	out.extend([it.len() as u8, low as u8, high.map_or(0xff, |h| h as u8)]);
	let copy = it.clone();
	let first = it.next();
	put(out, &format!("{first:?}|{it}|{it:?}|{copy}|{}", it.len()));
	put(
		out,
		&format!(
			"{:?}|{}|{:?}",
			copy.clone().last(),
			copy.clone().count(),
			copy.clone().nth(1)
		),
	);
	let rest: String = it.collect();
	put(out, &format!("{rest}|{copy}"));
}

/// `char`'s case maps and escapes as iterators: what each yields from either end, its
/// length, its clone, and its `Display` and `Debug` before and partway through.
pub fn iters(kind: u8, data: &[u8], out: &mut Vec<u8>) {
	let cs = scalars(data).into_iter().filter_map(char::from_u32).chain(SPECIAL);
	for c in cs {
		match kind % 5 {
			0 => {
				walk(out, c.to_lowercase());
				put(out, &c.to_lowercase().rev().collect::<String>());
			}
			1 => {
				walk(out, c.to_uppercase());
				let mut up = c.to_uppercase();
				put(
					out,
					&format!("{:?}|{:?}|{up}|{up:?}|{}", up.next_back(), up.next(), up.len()),
				);
			}
			2 => walk(out, c.escape_unicode()),
			3 => walk(out, c.escape_debug()),
			_ => walk(out, c.escape_default()),
		}
	}
}

/// `len_utf8` and `len_utf16`, and `encode_utf8` and `encode_utf16` into buffers long
/// enough, written through after, or of a length the input picks, which panics when short.
pub fn encodes(kind: u8, data: &[u8], out: &mut Vec<u8>) {
	let cs = scalars(data).into_iter().filter_map(char::from_u32).chain(SPECIAL);
	for c in cs {
		out.extend([c.len_utf8() as u8, c.len_utf16() as u8]);
		match kind % 3 {
			0 => {
				let mut bytes = [0xaa; 4];
				let s = c.encode_utf8(&mut bytes);
				put(out, &format!("{s}|{}|{s:?}", s.len()));
				s.make_ascii_uppercase();
				out.extend_from_slice(&bytes);
				let mut units = [0xaaaa; 2];
				let u = c.encode_utf16(&mut units);
				out.push(u.len() as u8);
				u[0] ^= 1;
				for unit in units {
					out.extend_from_slice(&unit.to_be_bytes());
				}
			}
			1 => {
				// Into the middle of a longer buffer, which keeps what is past it.
				let mut bytes = [0x55u8; 7];
				let n = c.encode_utf8(&mut bytes[2..]).len();
				out.push(n as u8);
				out.extend_from_slice(&bytes);
				let mut units = vec![7u16; 4];
				out.push(c.encode_utf16(&mut units[1..]).len() as u8);
				for unit in units {
					out.extend_from_slice(&unit.to_be_bytes());
				}
			}
			_ => {
				let n = data.first().map_or(0, |&b| usize::from(b % 5));
				let mut bytes = vec![0u8; n];
				put(out, c.encode_utf8(&mut bytes));
				let mut units = vec![0u16; n % 3];
				out.push(c.encode_utf16(&mut units).len() as u8);
			}
		}
	}
}

/// `char::decode_utf16` of the input's big-endian units, as a method and as `char`'s
/// function: each char or unpaired surrogate, its error's `Display`, `Debug`, equality, and
/// clone, and its size hint as it goes.
pub fn decodes(kind: u8, data: &[u8], out: &mut Vec<u8>) {
	let units: Vec<u16> = data
		.chunks(2)
		.map(|p| u16::from_be_bytes([p[0], p.get(1).copied().unwrap_or(0)]))
		.collect();
	let mut it = match kind % 2 {
		0 => char::decode_utf16(units.iter().copied()),
		_ => std::char::decode_utf16(units.iter().copied()),
	};
	let mut last = None;
	loop {
		let (low, high) = it.size_hint();
		out.extend([low as u8, high.map_or(0xff, |h| h as u8)]);
		let ahead = it.clone().next();
		let Some(r) = it.next() else { break };
		out.push((ahead.as_ref() == Some(&r)) as u8);
		match &r {
			Ok(c) => out.extend_from_slice(&u32::from(*c).to_be_bytes()),
			Err(e) => {
				put(out, &format!("{e}|{e:>40}|{e:?}|{e:#?}|{r:?}"));
				out.extend_from_slice(&e.unpaired_surrogate().to_be_bytes());
				out.push((last.as_ref() == Some(e)) as u8 | ((e.clone() == *e) as u8) << 1);
				last = Some(e.clone());
			}
		}
	}
	// Of an array by value and of a `Vec`, each `IntoIterator`.
	let pair = [0xd83d, 0xde00];
	let s: String = char::decode_utf16(pair)
		.map(|r| r.unwrap_or(char::REPLACEMENT_CHARACTER))
		.collect();
	let lossy: String = char::decode_utf16(units).map(|r| r.unwrap_or('?')).collect();
	put(out, &format!("{s}|{lossy}"));
}

/// `char::from_digit`, `is_digit`, `char::try_from` a `u32`, and `u8` and `u16` `try_from`
/// a `char`, with their errors and the radix panics.
pub fn converts(kind: u8, data: &[u8], out: &mut Vec<u8>) {
	let radix = data.first().map_or(10, |&b| u32::from(b % 40));
	match kind % 3 {
		0 => {
			for n in [0, 1, 9, 10, 15, 35, 36, 40, u32::MAX] {
				put(out, &format!("{:?}", char::from_digit(n, radix)));
			}
		}
		1 => {
			for c in ['0', '9', 'a', 'z', 'A', 'Z', '/', ':', '@', '[', '`', '{', '٣', 'ａ'] {
				out.push(c.is_digit(radix) as u8);
			}
		}
		_ => {
			for n in scalars(data) {
				let r = char::try_from(n);
				put(out, &format!("{r:?}"));
				if let Err(e) = r {
					put(out, &format!("{e}|{e:>45}|{e:?}|{e:#?}"));
					out.push((e == e.clone()) as u8);
				}
				if let Some(c) = char::from_u32(n) {
					let (b, w) = (u8::try_from(c), u16::try_from(c));
					put(out, &format!("{b:?}|{w:?}|{}|{}", u64::from(c), u128::from(c)));
					if let Err(e) = w.and(b.map(u16::from)) {
						put(out, &format!("{e}|{e:<35}|{e:?}|{e:#?}"));
						out.push((e == e.clone()) as u8);
					}
				}
			}
		}
	}
}

/// The methods as function values, as a pattern, a map, or a test, over text of the
/// harness's alphabet and the special chars.
pub fn values(kind: u8, data: &[u8], out: &mut Vec<u8>) {
	let mut s = crate::harness::text(data);
	s.extend(SPECIAL);
	let s = s.as_str();
	match kind % 3 {
		0 => {
			let parts: Vec<&str> = s.split(char::is_alphabetic).collect();
			put(out, &format!("{parts:?}|{:?}", s.trim_matches(char::is_numeric)));
			put(out, &format!("{:?}", s.find(char::is_uppercase)));
			put(out, &format!("{:?}", s.rfind(char::is_control)));
		}
		1 => {
			let lens: usize = s.chars().map(char::len_utf8).sum();
			let units: usize = s.chars().map(char::len_utf16).sum();
			let upper: String = s.chars().flat_map(char::to_uppercase).collect();
			let lower: String = s.chars().flat_map(char::to_lowercase).collect();
			put(out, &format!("{lens}|{units}|{upper}|{lower}"));
		}
		_ => {
			let escaped: String = s.chars().flat_map(char::escape_default).collect();
			let debug: String = s.chars().map(char::escape_debug).map(|e| e.to_string()).collect();
			let unicode: Vec<usize> = s.chars().map(char::escape_unicode).map(|e| e.len()).collect();
			put(out, &format!("{escaped}|{debug}|{unicode:?}"));
			let tests = [
				s.chars().all(char::is_alphanumeric),
				s.chars().any(char::is_lowercase),
				s.chars().filter(|c| c.is_alphanumeric()).count() > 3,
			];
			out.extend(tests.map(u8::from));
		}
	}
}
